"""Source-bound feasibility for a finite generation archive; never changes live archive bytes."""
import argparse
import json
from pathlib import Path
import runpy
import subprocess

ROOT = Path(__file__).resolve().parents[4]
READER = ROOT / 'scripts/history_archive.py'
PROFILE = ROOT / 'scripts/local_environment.py'
GENERATION_BYTES = 4_000_000
GENERATION_LINES = 60_000
ROOT_BYTES = 8192
RECORD_BYTES = 160000
CATALOG_HEADER = 512
CATALOG_MEMBER = 208
PLANNED_SLICES = 100


def slots(retained, slices):
    """Round the source plus a declared one-segment-per-slice planning envelope to binary fanout."""
    if type(retained) is not int or not 0 <= retained <= GENERATION_BYTES:
        raise ValueError('one valid source generation required')
    if type(slices) is not int or not 1 <= slices <= PLANNED_SLICES:
        raise ValueError('bounded planning window required')
    needed = retained + slices * (RECORD_BYTES + CATALOG_HEADER + CATALOG_MEMBER)
    minimum = (needed + GENERATION_BYTES - 1) // GENERATION_BYTES
    return needed, 1 << (minimum - 1).bit_length()


def limits(count):
    if type(count) is not int or not 1 <= count <= 8:
        raise ValueError('generation count must be in1..8')
    return {
        'generations': count,
        'decoded_bytes': count * GENERATION_BYTES,
        'decoded_lines': count * GENERATION_LINES,
        'resident_bytes': count * GENERATION_BYTES + ROOT_BYTES,
        'control_bytes': count * 262144 + ROOT_BYTES,
        'control_files': count * 17 + 1,
        'payload_files': count * 16,
        # Sealed groups have only catalogs; the first also keeps the16 original catalogs.
        'working_markdown': (count - 1) * 16 + 16 + 64,
    }


def controls():
    checked = 0
    for retained in (0, 1, GENERATION_BYTES - 1, GENERATION_BYTES):
        for slices in (1, 25, 99, PLANNED_SLICES):
            needed, count = slots(retained, slices)
            # Independent iterative capacity oracle, avoiding the production ceiling/bit arithmetic.
            oracle = 1
            while oracle * GENERATION_BYTES < needed:
                oracle *= 2
            assert oracle == count and needed <= limits(count)['decoded_bytes']
            assert count == 1 or needed > limits(count // 2)['decoded_bytes']
            checked += 1
    for retained, slices in ((True, 1), (-1, 1), (GENERATION_BYTES + 1, 1),
                             (0, False), (0, 0), (0, PLANNED_SLICES + 1)):
        try:
            slots(retained, slices)
        except ValueError:
            checked += 1
        else:
            raise AssertionError('invalid planning input admitted')
    for count in (False, 0, 9):
        try:
            limits(count)
        except ValueError:
            checked += 1
        else:
            raise AssertionError('invalid fanout admitted')
    assert limits(8) == dict(generations=8, decoded_bytes=32000000, decoded_lines=480000,
                            resident_bytes=32008192, control_bytes=2105344,
                            control_files=137, payload_files=128, working_markdown=192)
    print('generation budget: %d boundary/oracle/refusal controls; rc=0' % checked)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    if args.self_test:
        controls()
        return
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), PROFILE, READER))
    reader = runpy.run_path(str(READER))
    sources = (Path(__file__), *reader['backing_sources'](ROOT, profile))
    profile['plan_producer'](ROOT, sources=sources)
    frozen = {path: path.read_bytes() for path in sources}
    records, pressure = reader['inventory'](ROOT)
    needed, count = slots(pressure[2], PLANNED_SLICES)
    revision = reader['_head_revision'](ROOT)
    # wc is an independent OS byte oracle for all physical source files, not the archive decoder.
    archive = [path for path in sources if path.is_relative_to(ROOT / 'docs/history')
               or path.is_relative_to(ROOT / '.doctrine/history_archive')]
    profile['plan_producer'](ROOT, sources=sources)
    measured = subprocess.check_output(['wc', '-c', *(str(path) for path in archive)], cwd=ROOT)
    assert int(measured.decode().splitlines()[-1].split()[0]) == sum(len(frozen[path]) for path in archive)
    assert all(path.read_bytes() == data for path, data in frozen.items()), 'source drift'
    print(json.dumps(dict(source_revision=revision, working_source_pressure=pressure,
                          logical_records=len(records), planning_slices=PLANNED_SLICES,
                          required_decoded_bytes=needed, limits=limits(count),
                          physical_source_bytes=sum(len(frozen[path]) for path in archive),
                          assumptions='one full-size segment and one bounded compact catalog per slice',
                          live_installed=False), sort_keys=True))


if __name__ == '__main__':
    main()
