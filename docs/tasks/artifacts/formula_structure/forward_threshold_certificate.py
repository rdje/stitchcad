"""Source-locked resumable full forward-band certificate. No native runtime admission."""
from contextlib import contextmanager
from fractions import Fraction
from pathlib import Path
import argparse
import hashlib
import json
import os
import runpy
import time

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__)
PROFILE = ROOT / 'scripts/local_environment.py'
HELPER = HERE.with_name('finite_threshold_feasibility.py')
READER = HERE.with_name('verify_forward_certificate.py')
PI = ROOT / 'crates/sc-core/tests/fixtures/pi640_certificate.tsv'
RUN = ROOT / 'target/trig_forward_certificate/0278'
MANIFEST = RUN / 'manifest.json'
PENDING = RUN / 'manifest.pending'
LOCK = RUN / 'active.lock'
SIZE = 10_000
TOTAL = 100


def source_hashes():
    return {p.relative_to(ROOT).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in (HERE, PROFILE, HELPER, READER, PI)}


def payload_path(index):
    return RUN / ('chunk-%03d.bin' % index)


def encode(low, high, gap):
    fields = [(low.numerator * (1 << 512) // low.denominator, 64),
              (high.numerator * (1 << 512) // high.denominator, 64),
              (gap[0].numerator, 20), (gap[0].denominator, 20),
              (gap[1].numerator, 20), (gap[1].denominator, 20)]
    # Helpers return exact scaled integers; never silently round a payload endpoint.
    assert Fraction(fields[0][0], 1 << 512) == low
    assert Fraction(fields[1][0], 1 << 512) == high
    return b''.join(value.to_bytes(width, 'big') for value, width in fields)


@contextmanager
def exclusive_run():
    with LOCK.open('x') as stream:
        stream.write(str(os.getpid()) + '\n')
    try:
        yield
    finally:
        LOCK.unlink()


def publish(profile, manifest):
    assert source_hashes() == manifest['sources'], 'source drift before checkpoint'
    profile['plan_producer'](ROOT, output_files=(PENDING, MANIFEST))
    PENDING.write_text(json.dumps(manifest, indent=2) + '\n')
    PENDING.replace(MANIFEST)


def controls(helper, reader, pi_bounds):
    helper['controls'](pi_bounds)
    low, high = helper['band'](Fraction(1, 2_000_000), pi_bounds)
    gap = helper['certify'](low, high, reader['Q'])
    assert gap is not None
    valid = encode(low, high, gap)
    reader['verify_row'](valid)
    invalid = [valid[:-1], valid[:148] + bytes(20) + valid[168:],
               valid[:168] + valid[128:148] + valid[188:]]
    for row in invalid:
        try:
            reader['verify_row'](row)
        except AssertionError:
            continue
        raise AssertionError('reader accepted corrupted gap payload')
    print('forward controls: cohort oracles/negative gaps and reader corruption3 refused, rc=0', flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--chunks', type=int, default=TOTAL,
                        help='maximum new chunks this invocation; partial never means complete')
    args = parser.parse_args()
    assert 1 <= args.chunks <= TOTAL
    profile = runpy.run_path(str(PROFILE))
    sources = (HERE, PROFILE, HELPER, READER, PI)
    outputs = (MANIFEST, PENDING, LOCK, *(payload_path(i) for i in range(TOTAL)))
    profile['plan_producer'](ROOT, sources=sources, directories=(RUN,), output_files=outputs)
    hashes = source_hashes()
    helper = runpy.run_path(str(HELPER))
    reader = runpy.run_path(str(READER))
    assert helper['BITS'] == 512 and helper['BAND'] == Fraction(1, 1 << 384)
    assert helper['M'] == 1_000_000 and reader['ROW_BYTES'] == 208
    pi_bounds = helper['load_pi']()
    controls(helper, reader, pi_bounds)
    parameters = {'fraction_bits': 512, 'value_band_bits': 384, 'population': 1_000_000,
                  'chunk_count': TOTAL, 'chunk_size': SIZE, 'row_bytes': 208}
    profile['enter_producer'](ROOT, sources=sources, directories=(RUN,), output_files=outputs)
    with exclusive_run():
        if MANIFEST.exists():
            manifest = json.loads(MANIFEST.read_text())
            reader['verify_manifest'](manifest)
            assert manifest['sources'] == hashes and manifest['parameters'] == parameters, 'stale checkpoint'
            for chunk in manifest['chunks']:
                reader['verify_chunk'](RUN / chunk['file'], chunk)
            print('resume: %d previously verified chunks' % len(manifest['chunks']), flush=True)
        else:
            manifest = {'schema': 1, 'scope': 'forward_threshold_population', 'complete': False,
                        'parameters': parameters, 'sources': hashes, 'chunks': []}
        # A crash may leave exactly the next unpublished payload or manifest.pending.
        # It is recomputed; published chunks are never overwritten or trusted without verification.
        allowed = {'manifest.json', 'manifest.pending', 'active.lock'}
        allowed.update('chunk-%03d.bin' % i for i in range(min(TOTAL, len(manifest['chunks']) + 1)))
        assert {p.name for p in RUN.iterdir()} <= allowed, 'unexpected run artifact'
        for index in range(len(manifest['chunks']), min(TOTAL, len(manifest['chunks']) + args.chunks)):
            assert source_hashes() == hashes, 'source drift before chunk'
            path = payload_path(index)
            profile['plan_producer'](ROOT, output_files=(path,))
            digest = hashlib.sha256()
            minimum = None
            started = time.perf_counter_ns()
            with path.open('wb') as stream:
                for offset in range(SIZE):
                    j = index * SIZE + offset
                    low, high = helper['band'](Fraction(2 * j + 1, 2_000_000), pi_bounds)
                    gap = helper['certify'](low, high, reader['Q'])
                    assert gap is not None, ('uncertified forward threshold', j)
                    row = encode(low, high, gap)
                    _, _, margin = reader['verify_row'](row)
                    minimum = margin if minimum is None else min(minimum, margin)
                    digest.update(row)
                    stream.write(row)
            elapsed = time.perf_counter_ns() - started
            chunk = {'file': path.name, 'start': index * SIZE, 'count': SIZE,
                     'sha256': digest.hexdigest(), 'minimum_margin_scaled': minimum,
                     'elapsed_ns': elapsed}
            reader['verify_chunk'](path, chunk)
            manifest['chunks'].append(chunk)
            manifest['complete'] = len(manifest['chunks']) == TOTAL
            publish(profile, manifest)
            print('forward coverage: %d/1000000; chunk_ns=%d; complete=%s' %
                  ((index + 1) * SIZE, elapsed, manifest['complete']), flush=True)
        reader['verify_manifest'](manifest)
        assert source_hashes() == hashes, 'final source drift'
        print('forward result: %s; chunks=%d; rc=0' %
              ('COMPLETE' if manifest['complete'] else 'PARTIAL', len(manifest['chunks'])), flush=True)


if __name__ == '__main__':
    main()
