"""Read-only validated v3 per-generation pressure, cross-checked by a physical catalog census.

The public verify-retention total is aggregate. Never compare it with one group's limit.
No writes, environment activation, subprocesses or archive installation occur here.
"""
import argparse
import json
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]


def working_headroom(groups):
    """Active-generation working slots only; other byte/line/window limits still apply."""
    if not groups or any(type(value) is not int or not 0 <= value <= 64 for value in groups):
        raise ValueError('each validated generation must have 0..64 working Markdown files')
    if sum(groups) > 192:
        raise ValueError('aggregate working Markdown exceeds192')
    return 64 - groups[-1]


def controls():
    # Independent named examples distinguish aggregate and active pressure in both directions.
    assert working_headroom([21, 43]) == 21  # Aggregate64 does not require packing.
    assert working_headroom([21, 64]) == 0   # Aggregate85 still has active pressure.
    assert working_headroom([21, 44]) == 20  # One more raw record fits the working-file limit.
    print('working-slot controls: aggregate64/active43, aggregate85/active64, next active44; rc=0')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    if args.self_test:
        controls()
        return
    reader = runpy.run_path(str(ROOT / 'scripts/history_archive.py'))
    _records, aggregate = reader['inventory'](ROOT)  # Complete validation, including orphan checks.
    engine = reader['generation_reader']()
    _document, _data, groups = engine['describe'](ROOT, reader)
    actual = engine['physical_files'](ROOT, reader)
    raw = {path for path in actual if reader['SEGMENT'].fullmatch(path)}
    used, measured = set(), []
    for index, group in enumerate(groups, 1):
        last = index == len(groups)
        _found, pressure, _backings, _windows, _files = engine['decode_group'](
            ROOT, reader, group, raw if last else set(), used)
        # Independent JSON + physical directory census, not decoder return counters.
        control = json.loads((ROOT / group[0]['path']).read_bytes())
        catalogs = {row['path'] for row in control['legacy_catalogs']}
        catalogs.update(json.loads((ROOT / path).read_bytes())['catalog'] for path in control['windows'])
        census = len(catalogs) + (len(raw) if last else 0)
        assert catalogs <= actual and census == pressure[0], 'per-generation physical count mismatch'
        measured.append(pressure)
        print('generation%d: working=%d/64 decoded_lines=%d/60000 decoded_bytes=%d/4000000 '
              'resident_bytes=%d/4000000 raw=%d windows=%d' %
              (index, *pressure, len(raw) if last else 0, len(control['windows'])))
    assert tuple(sum(row[column] for row in measured) for column in range(3)) == aggregate[:3]
    physical_md = sum(path.suffix == '.md' for path in (ROOT / 'docs/history').iterdir())
    assert physical_md == aggregate[0], 'independent aggregate physical count mismatch'
    headroom = working_headroom([row[0] for row in measured])
    print('aggregate: working=%d/192; active working-slot headroom=%d; '
          'independent physical/catalog census agrees; rc=0' % (aggregate[0], headroom))


if __name__ == '__main__':
    main()
