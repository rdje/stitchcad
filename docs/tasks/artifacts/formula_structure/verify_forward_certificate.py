"""Independent exact reader of forward threshold gap payloads; no interval construction.

Coverage binds implicit row order to threshold indices through the source-locked producer.
This reader checks every encoded gap, size, digest and strict inequality, not asin accuracy.
"""
from pathlib import Path
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[4]
PROFILE = ROOT / 'scripts/local_environment.py'
RUN = ROOT / 'target/trig_forward_certificate/0278'
M = 1_000_000
S = 1 << 512
Q = 360 * M * ((1 << 128) - 1)
ROW_BYTES = 208  # two 64-byte phase endpoints, four 20-byte Farey components


def verify_row(row):
    assert len(row) == ROW_BYTES, 'row length'
    low, high = int.from_bytes(row[:64], 'big'), int.from_bytes(row[64:128], 'big')
    a, b, c, d = (int.from_bytes(row[offset:offset + 20], 'big')
                  for offset in range(128, ROW_BYTES, 20))
    assert 0 < low <= high < S // 4, 'phase range'
    assert 0 < b <= Q and 0 < d <= Q, 'denominator bound'
    assert c * b - a * d == 1 and b + d > Q, 'Farey gap'
    left_margin = low * b - a * S
    right_margin = c * S - high * d
    assert left_margin > 0 and right_margin > 0, 'strict containment'
    return low, high, min(left_margin // b, right_margin // d)


def verify_chunk(path, expected):
    assert path.stat().st_size == expected['count'] * ROW_BYTES, 'chunk size'
    digest = hashlib.sha256()
    minimum = None
    previous = 0
    with path.open('rb') as stream:
        for _ in range(expected['count']):
            row = stream.read(ROW_BYTES)
            digest.update(row)
            low, high, margin = verify_row(row)
            assert previous < low, 'row order'
            previous = high
            minimum = margin if minimum is None else min(minimum, margin)
        assert not stream.read(1), 'trailing payload'
    assert digest.hexdigest() == expected['sha256'], 'chunk hash'
    assert minimum == expected['minimum_margin_scaled'], 'chunk margin'
    return minimum


def verify_manifest(manifest):
    assert manifest['schema'] == 1 and manifest['scope'] == 'forward_threshold_population'
    assert manifest['parameters'] == {
        'fraction_bits': 512, 'value_band_bits': 384, 'population': M,
        'chunk_count': 100, 'chunk_size': 10_000, 'row_bytes': ROW_BYTES,
    }, 'certificate profile'
    assert type(manifest['complete']) is bool
    assert set(manifest['sources']) == {
        'scripts/local_environment.py',
        'crates/sc-core/tests/fixtures/pi640_certificate.tsv',
        'docs/tasks/artifacts/formula_structure/finite_threshold_feasibility.py',
        'docs/tasks/artifacts/formula_structure/forward_threshold_certificate.py',
        'docs/tasks/artifacts/formula_structure/verify_forward_certificate.py',
    }, 'source inventory'
    assert len(manifest['chunks']) <= 100
    for index, chunk in enumerate(manifest['chunks']):
        assert chunk['file'] == 'chunk-%03d.bin' % index, 'chunk name/order'
        assert chunk['start'] == index * 10_000 and chunk['count'] == 10_000, 'coverage'
    assert manifest['complete'] == (len(manifest['chunks']) == 100), 'completion/coverage'


def main():
    profile = runpy.run_path(str(PROFILE))
    manifest_path = RUN / 'manifest.json'
    profile['plan_producer'](ROOT, sources=(Path(__file__), PROFILE, manifest_path))
    manifest = json.loads(manifest_path.read_text())
    verify_manifest(manifest)
    assert manifest['complete'], 'partial population is not complete evidence'
    sources = [ROOT / name for name in manifest['sources']]
    chunks = [RUN / chunk['file'] for chunk in manifest['chunks']]
    profile['plan_producer'](ROOT, sources=(*sources, *chunks))
    for path in sources:
        assert hashlib.sha256(path.read_bytes()).hexdigest() == manifest['sources'][path.relative_to(ROOT).as_posix()], 'source drift'
    margins = [verify_chunk(path, chunk) for path, chunk in zip(chunks, manifest['chunks'])]
    print('forward certificate reader: 1000000 rows/100 contiguous chunks; exact gaps/hash/strict margins pass, rc=0')
    print('minimum scaled phase margin=%d; interval construction remains producer/derivation evidence' % min(margins))


if __name__ == '__main__':
    main()
