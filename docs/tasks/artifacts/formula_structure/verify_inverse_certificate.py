"""Independent exact reader of the inverse certificate; partial coverage never means complete.

Frozen build inputs retain artifact provenance. Only the actual math/coverage basis must still
match live sources, so unrelated feature changes do not invalidate historical numeric evidence.
"""
from pathlib import Path
import argparse
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[4]
PROFILE = ROOT / 'scripts/local_environment.py'
RUN = ROOT / 'target/inverse_threshold_certificate/0284'
MANIFEST = RUN / 'manifest.json'
S = 1 << 640
Q = ((1 << 128) - 1) ** 2
ROWS = 100_000
CHUNKS = 450
ROW_BYTES = 288
PARAMETERS = {'fraction_bits': 640, 'turn_band_bits': 600, 'population': 45_000_000,
              'chunk_size': ROWS, 'chunk_count': CHUNKS, 'row_bytes': ROW_BYTES,
              'slope_padding_scaled': str(1 << 44)}
CRITICAL = (
    'crates/sc-core/examples/inverse_threshold_chunk.rs',
    'crates/sc-core/tests/fixtures/pi640_certificate.tsv',
    'docs/tasks/artifacts/formula_structure/inverse_threshold_certificate.py',
    'docs/tasks/artifacts/formula_structure/verify_inverse_certificate.py',
)


def digest(path):
    result = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1 << 20), b''):
            result.update(block)
    return result.hexdigest()


def verify_row(row):
    assert len(row) == ROW_BYTES, 'row size'
    low, high = int.from_bytes(row[:80], 'big'), int.from_bytes(row[80:160], 'big')
    a, b, c, d = (int.from_bytes(row[i:i + 32], 'big') for i in range(160, ROW_BYTES, 32))
    assert 0 < low <= high < S, 'slope range'
    assert 0 < b <= Q and 0 < d <= Q and b + d > Q, 'denominators'
    assert c * b - a * d == 1, 'determinant'
    left, right = low * b - a * S, c * S - high * d
    assert left > 0 and right > 0, 'strict containment'
    return low, high, min(left // b, right // d)


def inspect_chunk(path):
    assert path.stat().st_size == ROWS * ROW_BYTES, 'chunk size'
    checksum = hashlib.sha256()
    first = previous = None
    minimum = None
    with path.open('rb') as stream:
        for _ in range(ROWS):
            row = stream.read(ROW_BYTES)
            checksum.update(row)
            low, high, margin = verify_row(row)
            assert previous is None or previous < low, 'row order'
            first = low if first is None else first
            previous = high
            minimum = margin if minimum is None else min(minimum, margin)
        assert not stream.read(1), 'trailing data'
    return {'sha256': checksum.hexdigest(), 'minimum_margin_scaled': minimum}, first, previous


def verify_manifest(manifest):
    assert manifest['schema'] == 1 and manifest['scope'] == 'inverse_threshold_population'
    assert manifest['parameters'] == PARAMETERS, 'certificate profile'
    assert type(manifest['complete']) is bool
    assert manifest['producer_file'] in ('producer', 'producer.exe')
    assert set(CRITICAL) <= set(manifest['frozen_sources']), 'critical source inventory'
    for name in manifest['frozen_sources']:
        path = Path(name)
        assert not path.is_absolute() and '..' not in path.parts and path.as_posix() == name
    indices = []
    for chunk in manifest['chunks']:
        index = chunk['index']
        assert type(index) is int and 0 <= index < CHUNKS
        assert chunk['file'] == 'chunk-%03d.bin' % index
        assert chunk['start'] == index * ROWS and chunk['count'] == ROWS
        indices.append(index)
    assert indices == sorted(set(indices)), 'duplicate/out-of-order chunks'
    assert manifest['complete'] == (indices == list(range(CHUNKS))), 'complete coverage'


def verify_sources(manifest, profile):
    frozen = [RUN / 'sources' / name for name in manifest['frozen_sources']]
    live = [ROOT / name for name in CRITICAL]
    binary = RUN / manifest['producer_file']
    profile['plan_producer'](ROOT, sources=(*frozen, *live, binary))
    for name, expected in manifest['frozen_sources'].items():
        assert digest(RUN / 'sources' / name) == expected, ('frozen source drift', name)
    for name in CRITICAL:
        assert digest(ROOT / name) == manifest['frozen_sources'][name], ('critical live source drift', name)
    assert digest(binary) == manifest['producer_sha256'], 'frozen producer drift'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--allow-partial', action='store_true')
    args = parser.parse_args()
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), PROFILE, MANIFEST))
    assert MANIFEST.stat().st_size <= 1_000_000, 'manifest size'
    manifest = json.loads(MANIFEST.read_text())
    verify_manifest(manifest)
    assert args.allow_partial or manifest['complete'], 'partial population is not complete evidence'
    verify_sources(manifest, profile)
    chunks = [RUN / chunk['file'] for chunk in manifest['chunks']]
    profile['plan_producer'](ROOT, sources=chunks)
    previous = None
    minimum = None
    for number, (path, expected) in enumerate(zip(chunks, manifest['chunks']), 1):
        actual, first, last = inspect_chunk(path)
        assert all(actual[key] == expected[key] for key in actual), 'chunk hash/margin'
        assert previous is None or previous < first, 'chunk order'
        previous = last
        minimum = actual['minimum_margin_scaled'] if minimum is None else min(minimum, actual['minimum_margin_scaled'])
        if number % 10 == 0:
            print('inverse reader: %d/%d chunks checked' % (number, len(chunks)), flush=True)
    verify_sources(manifest, profile)
    print('inverse reader: %s; rows%d/chunks%d exact gaps/hash/coverage pass, rc=0' %
          ('COMPLETE' if manifest['complete'] else 'PARTIAL', len(chunks) * ROWS, len(chunks)))
    print('minimum scaled slope margin=%s; interval construction remains separate producer/derivation evidence' % minimum)


if __name__ == '__main__':
    main()
