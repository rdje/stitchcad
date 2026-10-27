"""Permanent real transaction smoke on a fresh committed-HEAD image; never swaps live archive trees."""
import argparse
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).parent
READER = ROOT / 'scripts/history_archive.py'
GENERATIONS = ROOT / 'scripts/history_generations.py'
PROFILE = ROOT / 'scripts/local_environment.py'
PRODUCER = HERE / 'v2_migration.py'
INSTALLER = HERE / 'v2_install.py'
CONTRACT = HERE / 'v2_install_contract.py'
HELPER = ROOT / 'docs/tasks/artifacts/changelog/changelog_pointers.py'
OWNER = 'G1-SLICE.6.4.5.capacity.apply.accept.clients'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--identity-only', action='store_true', help='focused byte-identical install/recovery check')
    args = parser.parse_args()
    profile = runpy.run_path(str(PROFILE))
    api = runpy.run_path(str(READER))
    producer = runpy.run_path(str(PRODUCER))
    work = ROOT / 'target/scratch/history_transaction_smoke' / str(os.getpid())
    snapshot, prepared = work / 'source', work / 'prepared'
    sources = (Path(__file__), PRODUCER, INSTALLER, CONTRACT, HELPER, ROOT / 'CHANGELOG.md',
               *api['backing_sources'](ROOT, profile))
    profile['plan_producer'](ROOT, sources=sources, directories=(work,))
    assert not work.exists()
    frozen = {path: path.read_bytes() for path in sources}
    revision = api['_head_revision'](ROOT)
    profile['plan_producer'](ROOT, sources=sources)
    names = subprocess.check_output(['git', '-C', str(ROOT), 'ls-tree', '-r', '--name-only', revision,
                                     '--', 'docs/history', '.doctrine/history_archive'],
                                    env=api['metadata_environment'](ROOT)).decode().splitlines()
    contents = {name: api['_git_blob'](ROOT, revision, name, producer['source_bound'](name, api)) for name in names}
    profile['plan_producer'](ROOT, sources=sources)
    ledger = subprocess.check_output(['git', '-C', str(ROOT), 'show', revision + ':CHANGELOG.md'],
                                    env=api['metadata_environment'](ROOT))
    assert len(ledger) <= 160000
    contents['CHANGELOG.md'] = ledger
    outputs = tuple(snapshot / name for name in contents)
    directories = {work, snapshot}
    for path in outputs:
        directories.update(parent for parent in path.parents if parent.is_relative_to(work))
    plan = dict(sources=sources, directories=tuple(sorted(directories)), output_files=outputs)
    profile['enter_producer'](ROOT, **plan)
    for name, data in contents.items():
        profile['plan_producer'](ROOT, **plan)
        with (snapshot / name).open('xb') as stream:
            stream.write(data)
    raw = api['_git_paths'](ROOT, revision)
    original = {name: data for name, data in contents.items() if name != 'CHANGELOG.md'}

    def identity_candidate(destination):
        # Byte-identical candidates still exercise all four real directory renames and recovery
        # states: live/stage/backup locations differ. No capture or rollover claim is made here.
        records, pressure = api['inventory'](snapshot)
        inputs = (*sources, *api['backing_sources'](snapshot, profile))
        producer['write_prepared'](destination, profile, api, inputs, revision, pressure, original,
                                  original, records, api['backing_inventory'](snapshot),
                                  [['verify-retention']], len(api['flat_legacy_catalogs'](snapshot)),
                                  {'identity_transaction_smoke': True}, source_root=snapshot)

    if raw and not args.identity_only:
        producer['append'](prepared, OWNER, source_root=snapshot)
    else:
        identity_candidate(prepared)
    if not args.identity_only:
        audit = json.loads((prepared / 'source.json').read_bytes())
        inputs = (*sources, prepared / 'source.json', *(snapshot / name for name in contents),
                  *(prepared / 'candidate' / row['path'] for row in audit['candidate_files']))
        profile['plan_producer'](ROOT, sources=inputs)
        subprocess.run([sys.executable, '-I', '-B', str(CONTRACT), prepared.relative_to(ROOT).as_posix(),
                        '--source', snapshot.relative_to(ROOT).as_posix()], cwd=ROOT, check=True)
    if raw or args.identity_only:
        identity = prepared if args.identity_only else work / 'identity'
        if not args.identity_only:
            identity_candidate(identity)
        live = work / 'identity-live'
        outputs = tuple(live / name for name in original)
        directories = {work, live}
        for path in outputs:
            directories.update(parent for parent in path.parents if parent.is_relative_to(work))
        plan = dict(sources=(*sources, *(snapshot / name for name in contents)),
                    directories=tuple(sorted(directories)), output_files=outputs)
        profile['enter_producer'](ROOT, **plan)
        for name, data in original.items():
            profile['plan_producer'](ROOT, **plan)
            with (live / name).open('xb') as stream:
                stream.write(data)
        installer = runpy.run_path(str(INSTALLER))
        installer['install'](identity, live)
        installer['recover'](identity, live)
        assert all((live / name).read_bytes() == data for name, data in original.items())
        print('byte-identical transaction: real install/recovery; exact originals and distinct directory states; rc=0')
    assert api['_head_revision'](ROOT) == revision
    assert all(path.read_bytes() == data for path, data in frozen.items()), 'transaction smoke changed current inputs'
    assert all((snapshot / name).read_bytes() == data for name, data in contents.items()), 'committed snapshot changed'
    print('permanent transaction smoke: current committed HEAD/archive/ledger exact; live inputs unchanged; rc=0')


if __name__ == '__main__':
    main()
