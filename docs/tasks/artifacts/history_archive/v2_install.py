"""Journaled v1-to-v2 directory swaps; recovery refuses any unrecognized bytes.

Only individual directory renames are atomic. Readers must be idle during installation;
an interrupted transaction is completed by explicit recovery before ordinary archive use.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
READER = ROOT / 'scripts/history_archive.py'
PROFILE = ROOT / 'scripts/local_environment.py'
PRODUCER = Path(__file__).with_name('v2_migration.py')
TREES = ('.doctrine/history_archive', 'docs/history')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def tree(root, relative):
    """Closed regular-file population; never traverse links or nested repositories."""
    base = root / relative
    if not base.exists() and not base.is_symlink():
        return None
    require(base.is_dir() and not base.is_symlink(), 'unsafe transaction tree')
    result, seen_directories = {}, set()
    for parent, directories, files in os.walk(base, followlinks=False):
        folder = Path(parent)
        seen_directories.add(folder.relative_to(base).as_posix())
        require(folder.stat().st_dev == ROOT.stat().st_dev, 'foreign transaction volume')
        require('.git' not in directories and '.git' not in files, 'nested transaction repository')
        for name in directories:
            require(not (folder / name).is_symlink(), 'transaction directory link')
        for name in files:
            path = folder / name
            require(not path.is_symlink() and path.is_file() and path.stat().st_nlink == 1,
                    'unsafe transaction file')
            require(path.stat().st_dev == ROOT.stat().st_dev and path.stat().st_size <= 1048576,
                    'transaction file bound')
            require(len(result) < 128, 'transaction file count bound')
            data = path.read_bytes()
            result[path.relative_to(base).as_posix()] = (len(data), digest(data))
    required_directories = {'.'}
    for name in result:
        required_directories.update(parent.as_posix() for parent in Path(name).parents)
    require(seen_directories == required_directories, 'unrecognized transaction directory')
    return result


def expected(rows, relative):
    prefix = relative + '/'
    return {row['path'][len(prefix):]: (row['bytes'], row['sha256'])
            for row in rows if row['path'].startswith(prefix)}


def context(work, destination):
    require(destination == ROOT or destination.is_relative_to(ROOT / 'target'), 'unsafe migration destination')
    profile = runpy.run_path(str(PROFILE))
    profile['workspace'](destination)
    profile['plan_producer'](ROOT, sources=(Path(__file__), PROFILE, READER, PRODUCER, work / 'source.json'))
    reader = runpy.run_path(str(READER))
    raw = reader['bounded_read'](work / 'source.json', 131072)
    audit = json.loads(raw, object_pairs_hook=reader['no_duplicate_keys'])
    require(reader['_head_revision'](ROOT) == audit['source_revision'], 'migration HEAD drift')
    for path, key in ((PRODUCER, 'producer_sha256'), (READER, 'reader_sha256'), (PROFILE, 'profile_sha256')):
        require(digest(path.read_bytes()) == audit[key], 'migration producer drift')
    for relative in TREES:
        require(tree(work / 'candidate', relative) == expected(audit['candidate_files'], relative),
                'migration candidate drift')
    sources = (Path(__file__), PROFILE, READER, PRODUCER, work / 'source.json',
               *(work / 'candidate' / row['path'] for row in audit['candidate_files']))
    transaction = work / 'installation'
    directories = {transaction, transaction / 'stage', transaction / 'backup'}
    outputs = {transaction / 'journal.json'}
    prefixes = [transaction / 'stage', transaction / 'backup']
    if destination != ROOT:
        prefixes.append(destination)
    for prefix in prefixes:
        for row in [*audit['source_files'], *audit['candidate_files']]:
            path = prefix / row['path']
            outputs.add(path)
            directories.update(parent for parent in path.parents
                               if parent != ROOT and parent.is_relative_to(ROOT))
    # Parent creation must not create the stage/backup trees before a rename. Validate the
    # complete plan read-only, then create only the individual required parents at each effect.
    plan = dict(sources=sources, directories=tuple(sorted(directories)), output_files=tuple(sorted(outputs)))
    profile['plan_producer'](ROOT, **plan)
    for relative in TREES:
        profile['directory'](ROOT, (destination / relative).parent.relative_to(ROOT).as_posix())
    return profile, reader, audit, transaction, plan


def validate_states(destination, transaction, audit):
    states = []
    for relative in TREES:
        original = expected(audit['source_files'], relative)
        candidate = expected(audit['candidate_files'], relative)
        live, stage, backup = (tree(base, relative) for base in
                               (destination, transaction / 'stage', transaction / 'backup'))
        shapes = ((original, candidate, None), (None, candidate, original),
                  (candidate, None, original))
        require((live, stage, backup) in shapes, 'unrecognized transaction bytes: ' + relative)
        states.append(shapes.index((live, stage, backup)))
    return states


def recover(work, destination=ROOT):
    profile, reader, audit, transaction, plan = context(work, destination)
    journal = transaction / 'journal.json'
    require(journal.read_bytes() == journal_bytes(audit, destination), 'transaction journal drift')
    states = validate_states(destination, transaction, audit)
    for relative, state in reversed(list(zip(TREES, states))):
        live, stage, backup = (base / relative for base in
                               (destination, transaction / 'stage', transaction / 'backup'))
        if state == 2:
            profile['plan_producer'](ROOT, **plan)
            live.rename(stage)
        if state >= 1:
            profile['plan_producer'](ROOT, **plan)
            backup.rename(live)
    require(validate_states(destination, transaction, audit) == [0, 0], 'rollback incomplete')
    reader['guard_committed'](destination)
    print('v2 recovery: exact original trees restored; candidate and journal retained')


def journal_bytes(audit, destination):
    return (json.dumps({'source_revision': audit['source_revision'],
                        'destination': destination.relative_to(ROOT).as_posix()}, sort_keys=True) + '\n').encode()


def install(work, destination=ROOT, *, checkpoint=lambda _label: None):
    profile, reader, audit, transaction, plan = context(work, destination)
    require(not transaction.exists(), 'fresh installation transaction required')
    for relative in TREES:
        require(tree(destination, relative) == expected(audit['source_files'], relative), 'migration source drift')
    records = reader['inventory'](destination)[0]
    require(reader['inventory'](work / 'candidate')[0] == records, 'candidate logical mismatch')
    reader['guard_committed'](work / 'candidate')
    initial = tuple(destination / row['path'] for row in audit['source_files'])
    profile['plan_producer'](ROOT, **dict(plan, sources=(*plan['sources'], *initial), mutable_sources=initial))
    # Store activation happens after every source/candidate/HEAD proof, before the first write.
    profile['plan_producer'](ROOT, **plan)
    os.environ.update(profile['environment_for'](ROOT, os.environ, create=True))
    for parent in (transaction, transaction / 'stage', transaction / 'backup'):
        profile['plan_producer'](ROOT, **plan)
        parent.mkdir()
    for relative in TREES:
        for prefix in (transaction / 'stage', transaction / 'backup'):
            parent = (prefix / relative).parent
            profile['plan_producer'](ROOT, **plan)
            parent.mkdir(parents=True, exist_ok=True)
        for row in audit['candidate_files']:
            if not row['path'].startswith(relative + '/'):
                continue
            path = transaction / 'stage' / row['path']
            profile['plan_producer'](ROOT, **plan)
            path.parent.mkdir(parents=True, exist_ok=True)
            profile['plan_producer'](ROOT, **plan)
            with path.open('xb') as stream:
                stream.write((work / 'candidate' / row['path']).read_bytes())
    require(validate_states(destination, transaction, audit) == [0, 0], 'staged candidate mismatch')
    profile['plan_producer'](ROOT, **plan)
    with (transaction / 'journal.json').open('xb') as stream:
        stream.write(journal_bytes(audit, destination))
        stream.flush()
        os.fsync(stream.fileno())
    try:
        for relative in TREES:
            profile['plan_producer'](ROOT, **plan)
            (destination / relative).rename(transaction / 'backup' / relative)
            checkpoint('saved ' + relative)
            profile['plan_producer'](ROOT, **plan)
            (transaction / 'stage' / relative).rename(destination / relative)
            checkpoint('installed ' + relative)
        require(validate_states(destination, transaction, audit) == [2, 2], 'installation incomplete')
        require(reader['inventory'](destination)[0] == records, 'installed logical mismatch')
        reader['guard_committed'](destination)
    except BaseException:
        recover(work, destination)
        raise
    print('v2 installation: exact logical/source retention verified; original trees retained for recovery')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('operation', choices=('install', 'recover'))
    parser.add_argument('workspace', help='prepared repository-relative target/ directory')
    args = parser.parse_args()
    require(args.workspace.startswith('target/') and '..' not in Path(args.workspace).parts,
            'workspace must be repository-relative target/')
    (install if args.operation == 'install' else recover)(ROOT / args.workspace)


if __name__ == '__main__':
    main()
