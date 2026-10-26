"""Guarded native inverse chunk build, exact Python row/parity checks and bounded cost probe."""
from fractions import Fraction as F
from pathlib import Path
import hashlib
import json
import runpy
import subprocess
import time

ROOT = Path(__file__).resolve().parents[4]
PROFILE = ROOT / 'scripts/local_environment.py'
NATIVE = ROOT / 'crates/sc-core/examples/inverse_threshold_chunk.rs'
HELPER = Path(__file__).with_name('inverse_threshold_feasibility.py')
FORWARD = Path(__file__).with_name('finite_threshold_feasibility.py')
PI = ROOT / 'crates/sc-core/tests/fixtures/pi640_certificate.tsv'
RUN = ROOT / 'target/native_inverse_probe/0283'
LOG = RUN / 'build.log'
PAYLOAD = RUN / 'prefix.bin'
REPORT = RUN / 'report.json'
S = 1 << 640
Q = ((1 << 128) - 1) ** 2
ROW_BYTES = 288


def verify_row(row):
    assert len(row) == ROW_BYTES
    low, high = int.from_bytes(row[:80], 'big'), int.from_bytes(row[80:160], 'big')
    a, b, c, d = (int.from_bytes(row[i:i + 32], 'big') for i in range(160, ROW_BYTES, 32))
    assert 0 < low <= high < S
    assert 0 < b <= Q and 0 < d <= Q and b + d > Q
    assert c * b - a * d == 1
    assert a * S < low * b and high * d < c * S
    return F(low, S), F(high, S)


def main():
    profile = runpy.run_path(str(PROFILE))
    initial = (Path(__file__), PROFILE, NATIVE, HELPER, FORWARD, PI, ROOT / 'Cargo.toml', ROOT / 'Cargo.lock')
    outputs = (LOG, PAYLOAD, REPORT)
    profile['plan_producer'](ROOT, sources=initial, directories=(RUN, ROOT / 'target'), output_files=outputs)
    # Include the actual tracked workspace source/manifests before Cargo dispatch; new example is explicit.
    inventory = subprocess.run(['git', 'ls-files', '*.rs', '**/Cargo.toml'], cwd=ROOT,
                               check=True, capture_output=True, text=True).stdout.splitlines()
    sources = tuple(sorted(set(initial) | {ROOT / name for name in inventory}))
    profile['plan_producer'](ROOT, sources=sources, directories=(RUN, ROOT / 'target'), output_files=outputs)
    hashes = {p.relative_to(ROOT).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}
    inverse, forward = runpy.run_path(str(HELPER)), runpy.run_path(str(FORWARD))
    pi_bounds = forward['load_pi']()
    assert F(1, 720_000_000) > inverse['BAND']
    inverse['controls'](pi_bounds, forward['certify'])
    protocol = ('%x\n%x\n' % pi_bounds).encode()
    profile['enter_producer'](ROOT, sources=sources, directories=(RUN, ROOT / 'target'), output_files=outputs)
    build = subprocess.run(['cargo', 'build', '--offline', '-p', 'sc-core', '--release',
                            '--example', 'inverse_threshold_chunk', '--message-format=json'],
                           cwd=ROOT, capture_output=True, text=True)
    profile['plan_producer'](ROOT, output_files=(LOG,))
    LOG.write_text((build.stdout + build.stderr).replace(str(ROOT), '.'))
    assert build.returncode == 0, 'native build failed; see target/native_inverse_probe/0283/build.log'
    binaries = [Path(item['executable']) for line in build.stdout.splitlines()
                if (item := json.loads(line)).get('reason') == 'compiler-artifact'
                and item.get('target', {}).get('name') == 'inverse_threshold_chunk' and item.get('executable')]
    assert len(binaries) == 1, 'actual Cargo artifact identity'
    binary = binaries[0]
    assert binary.relative_to(ROOT).parts[0] == 'target'
    profile['plan_producer'](ROOT, sources=(binary,))
    binary_hash = hashlib.sha256(binary.read_bytes()).hexdigest()

    def invoke(start, count, payload=protocol):
        profile['plan_producer'](ROOT, sources=(binary,))
        return subprocess.run([str(binary), str(start), str(count)], input=payload, cwd=ROOT, capture_output=True)

    start = time.perf_counter_ns()
    result = invoke(0, 10_000)
    elapsed = time.perf_counter_ns() - start
    assert result.returncode == 0, result.stderr.decode()
    assert len(result.stdout) == 10_000 * ROW_BYTES
    for offset in range(0, len(result.stdout), ROW_BYTES):
        verify_row(result.stdout[offset:offset + ROW_BYTES])
    profile['plan_producer'](ROOT, output_files=(PAYLOAD,))
    PAYLOAD.write_bytes(result.stdout)
    total = inverse['POPULATION']
    cohort = sorted({0, 1, 2, total // 2 - 1, total // 2, total - 3, total - 2, total - 1,
                     *(i * (total - 1) // 31 for i in range(32)), *range(32)})
    native_rows = 10_000
    for index in cohort:
        if index < 10_000:
            row = result.stdout[index * ROW_BYTES:(index + 1) * ROW_BYTES]
        else:
            extra = invoke(index, 1)
            assert extra.returncode == 0, extra.stderr.decode()
            row = extra.stdout
            native_rows += 1
        low, high = verify_row(row)
        expected = inverse['slope_band'](F(2 * index + 1, 720_000_000), pi_bounds)
        assert low <= expected[0] <= expected[1] <= high, ('Python band not enclosed', index)
    for args, payload in [((0, 0), protocol), ((total, 1), protocol),
                          ((0, 100_001), protocol), ((0, 1), b'not-pi\n'),
                          ((0, 1), protocol + b'extra\n')]:
        refusal = invoke(*args, payload)
        assert refusal.returncode != 0 and not refusal.stdout, 'native invalid input was not refused before output'
    assert hashlib.sha256(binary.read_bytes()).hexdigest() == binary_hash
    assert {p.relative_to(ROOT).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest() for p in sources} == hashes
    report = {'scope': 'native_inverse_chunk_probe_only', 'native_rows_verified': native_rows,
              'Python_band_parity': len(cohort), 'invalid_input_refusals': 5,
              'prefix_count': 10_000, 'prefix_wall_ns': elapsed,
              'payload_sha256': hashlib.sha256(result.stdout).hexdigest(),
              'row_bytes': ROW_BYTES, 'binary': binary.relative_to(ROOT).as_posix(),
              'binary_sha256': binary_hash, 'sources': hashes}
    profile['plan_producer'](ROOT, output_files=(REPORT,))
    REPORT.write_text(json.dumps(report, indent=2) + '\n')
    print('native inverse probe: rows%d/Python-band%d/invalid-input5 pass, rc=0' % (native_rows, len(cohort)))
    print('native10000_wall_ns=%d; row_bytes288; full45M/native inverse API NOT verified' % elapsed)


if __name__ == '__main__':
    main()
