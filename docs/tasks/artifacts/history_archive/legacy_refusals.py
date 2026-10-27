"""Run the original v1 refusal producer against exact retained v1 bytes after repacking.

The producer and reader sources are copied unchanged. Only Git HEAD arguments in this
process are scoped to the original v1 capture; no Git object or repository is written.
"""
import argparse
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = Path(__file__).with_name('history_archive_probes.py')
READER = ROOT / 'scripts/history_archive.py'
PROFILE = ROOT / 'scripts/local_environment.py'
MASTER = '.doctrine/history_archive/windows.json'


def baseline(reader, root):
    backings = reader['backing_inventory'](root)
    master = json.loads(backings[MASTER])
    packed = {row['path'] for path in master['windows'] for row in json.loads(backings[path])['members']}
    records = reader['inventory'](root)[0]
    if reader['_uses_v2'](root):
        manifests = list(reader['flat_snapshot'](root)[3].values())
        revision = next(value['source_revision'] for value in manifests
                        if any(row['kind'] == 'backing' for row in value['members']))
        cohort = {row['path'] for value in manifests if value['source_revision'] == revision
                  for row in value['members'] if row['kind'] == 'record'}
        records = {path: records[path] for path in cohort}
    catalogs = {json.loads(backings[path])['catalog'] for path in master['windows']}
    return {**backings, **{path: records[path] for path in records if path not in packed},
            **{path: (root / path).read_bytes() for path in catalogs}}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=ROOT)
    args = parser.parse_args()
    root = args.root
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), ENTRY, READER, PROFILE))
    reader = runpy.run_path(str(READER))
    root = reader['archive_root'](root)
    if root == ROOT and not reader['_uses_v2'](root):
        runpy.run_path(str(ENTRY), run_name='__main__')
        return
    work = ROOT / 'target/scratch/legacy_archive_refusals' / str(os.getpid())
    reader_sources = reader['backing_sources'](root, profile)
    files = baseline(reader, root)
    for path in (ENTRY, READER, PROFILE, ROOT / 'scripts/history_archive.sh'):
        files[path.relative_to(ROOT).as_posix()] = path.read_bytes()
    sources = (Path(__file__), ENTRY, READER, PROFILE, ROOT / 'scripts/history_archive.sh', *reader_sources)
    outputs = tuple(work / name for name in files)
    directories = {work}
    for path in outputs:
        directories.update(parent for parent in path.parents if parent.is_relative_to(work))
    plan = dict(sources=sources, directories=tuple(sorted(directories)), output_files=outputs)
    profile['plan_producer'](ROOT, **plan)
    assert not work.exists(), 'fresh legacy refusal fixture required'
    frozen = {path: path.read_bytes() for path in sources}
    profile['enter_producer'](ROOT, **plan)
    for name, data in files.items():
        profile['plan_producer'](ROOT, **plan)
        with (work / name).open('xb') as stream:
            stream.write(data)
    manifests = reader['flat_snapshot'](root)[3].values()
    revision = next(value['source_revision'] for value in manifests
                    if any(row['kind'] == 'backing' for row in value['members']))
    assert reader['_git_blob'](ROOT, revision, MASTER, 8192) == files[MASTER]
    original_run = subprocess.run
    original_environment = dict(os.environ)

    def pinned(command, *args, **kwargs):
        if command and command[0] == 'git':
            command = [revision if value == 'HEAD' else revision + value[4:]
                       if isinstance(value, str) and value.startswith('HEAD:') else value for value in command]
        return original_run(command, *args, **kwargs)

    try:
        subprocess.run = pinned
        os.environ['HISTORY_ARCHIVE_ROOT'] = str(work)
        runpy.run_path(str(work / ENTRY.relative_to(ROOT)), run_name='__main__')
        assert all(path.read_bytes() == data for path, data in frozen.items()), 'legacy refusal source drift'
        print('legacy v1 refusals: original producer/source bytes unchanged; Git capture explicitly pinned')
    finally:
        subprocess.run = original_run
        os.environ.clear()
        os.environ.update(original_environment)
    # Retain this complete owned fixture for inspection; ordinary artifact cleanup owns it.


if __name__ == '__main__':
    main()
