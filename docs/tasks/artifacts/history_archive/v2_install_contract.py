"""Real directory-swap interruptions, explicit recovery, public bytes and legacy consumers."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
INSTALLER = Path(__file__).with_name('v2_install.py')
READER = ROOT / 'scripts/history_archive.py'
PROFILE = ROOT / 'scripts/local_environment.py'
PRODUCER = Path(__file__).with_name('v2_migration.py')
GENERATIONS = ROOT / 'scripts/history_generations.py'
HELPER = ROOT / 'docs/tasks/artifacts/changelog/changelog_pointers.py'
CHANGELOG = ROOT / 'CHANGELOG.md'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('workspace')
    args = parser.parse_args()
    assert args.workspace.startswith('target/') and '..' not in Path(args.workspace).parts
    prepared = ROOT / args.workspace
    work = ROOT / 'target/scratch/v2_install_contract' / str(os.getpid())
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), INSTALLER, PRODUCER, READER, GENERATIONS,
                                          PROFILE, HELPER, CHANGELOG, prepared / 'source.json'), directories=(work,))
    reader = runpy.run_path(str(READER))
    install = runpy.run_path(str(INSTALLER))
    audit = json.loads((prepared / 'source.json').read_bytes())
    candidates = {row['path']: (prepared / 'candidate' / row['path']).read_bytes() for row in audit['candidate_files']}
    original = {row['path']: (ROOT / row['path']).read_bytes() for row in audit['source_files']}
    sources = (Path(__file__), INSTALLER, PRODUCER, READER, GENERATIONS, PROFILE,
               HELPER, CHANGELOG, prepared / 'source.json',
               *(prepared / 'candidate' / name for name in candidates), *(ROOT / name for name in original))
    frozen = {path: path.read_bytes() for path in sources}
    profile['plan_producer'](ROOT, sources=sources, directories=(work,))
    assert not work.exists()
    checks = 0

    def fixture(label):
        base = work / label
        target = base / 'live'
        files = {base / 'source.json': (prepared / 'source.json').read_bytes(),
                 **{base / 'candidate' / name: data for name, data in candidates.items()},
                 **{target / name: data for name, data in original.items()}}
        directories = {work, base, target}
        for path in files:
            directories.update(parent for parent in path.parents if parent.is_relative_to(work))
        plan = dict(sources=sources, directories=tuple(sorted(directories)), output_files=tuple(files))
        profile['enter_producer'](ROOT, **plan)
        for path, data in files.items():
            profile['plan_producer'](ROOT, **plan)
            with path.open('xb') as stream:
                stream.write(data)
        return base, target

    def exact(target, rows):
        for relative in install['TREES']:
            assert install['tree'](target, relative) == install['expected'](rows, relative)

    # Real effects: every rename interruption must restore both original trees byte-for-byte.
    labels = ['saved .doctrine/history_archive', 'installed .doctrine/history_archive',
              'saved docs/history', 'installed docs/history']
    for index, label in enumerate(labels):
        base, target = fixture('interrupt%d' % index)
        def interrupt(current):
            if current == label:
                raise OSError('controlled interruption: ' + label)
        try:
            install['install'](base, target, checkpoint=interrupt)
        except OSError as error:
            assert str(error) == 'controlled interruption: ' + label
        else:
            raise AssertionError('interruption not reached')
        exact(target, audit['source_files'])
        assert reader['inventory'](target)[0] == reader['inventory'](ROOT)[0]
        checks += 1
    base, target = fixture('abrupt')
    child = ("import os,runpy; from pathlib import Path; "
             "entry=runpy.run_path(%r); entry['install'](Path(%r),Path(%r),"
             "checkpoint=lambda label: os._exit(73) if label=='saved docs/history' else None)"
             % (str(INSTALLER), str(base), str(target)))
    profile['plan_producer'](ROOT, sources=(*sources, base / 'source.json',
                             *(base / 'candidate' / name for name in candidates),
                             *reader['backing_sources'](target, profile)))
    assert subprocess.run([sys.executable, '-I', '-B', '-c', child], cwd=ROOT).returncode == 73
    assert not (target / 'docs/history').exists()
    install['recover'](base, target)
    exact(target, audit['source_files'])
    checks += 1
    base, target = fixture('success')
    install['install'](base, target)
    exact(target, audit['candidate_files'])
    checks += 1
    records = reader['inventory'](ROOT)[0]
    command = [sys.executable, '-I', '-B', str(READER), '--root', str(target)]
    current_sources = (*sources, *reader['backing_sources'](target, profile))
    profile['plan_producer'](ROOT, sources=current_sources)
    assert subprocess.check_output([*command, 'list'], cwd=ROOT).decode().splitlines() == sorted(records)
    recovered = target / 'target/review'
    outputs = tuple(recovered / name for name in records)
    directories = {recovered}
    for output in outputs:
        directories.update(parent for parent in output.parents if parent.is_relative_to(recovered))
    profile['plan_producer'](ROOT, sources=current_sources, directories=tuple(sorted(directories)), output_files=outputs)
    subprocess.run([*command, 'materialize', 'target/review'], cwd=ROOT, check=True, capture_output=True)
    assert {path.relative_to(recovered).as_posix(): path.read_bytes() for path in recovered.rglob('*.md')} == records
    assert reader['backing_inventory'](target) == reader['backing_inventory'](ROOT)
    checks += 3
    # Independent OS tar lists and emits concatenated first-level bytes, without the writer/reader.
    expected = {**records, **reader['backing_inventory'](ROOT)}
    candidate_master = json.loads(candidates[reader['MASTER']])
    if candidate_master['version'] == 3:
        windows = [path for row in candidate_master['generations']
                   for path in json.loads(candidates[row['path']])['windows']]
    else:
        windows = candidate_master['windows']
    for name in windows:
        manifest = json.loads(candidates[name])
        payload = target / manifest['payload']
        names = [row['path'] for row in manifest['members']]
        profile['plan_producer'](ROOT, sources=current_sources)
        assert subprocess.check_output(['tar', '-tzf', str(payload)]).decode().splitlines() == names
        profile['plan_producer'](ROOT, sources=current_sources)
        assert subprocess.check_output(['tar', '-xOzf', str(payload)]) == b''.join(expected[name] for name in names)
        checks += 1
    profile['plan_producer'](ROOT, sources=sources)
    current = subprocess.check_output([sys.executable, '-I', '-B', str(HELPER), str(ROOT), str(ROOT)])
    profile['plan_producer'](ROOT, sources=(*sources, *(target / name for name in candidates)))
    migrated = subprocess.check_output([sys.executable, '-I', '-B', str(HELPER), str(ROOT), str(target)])
    assert current == migrated
    checks += 1
    # A successful install can also be explicitly rolled back. Only the archive trees are owned;
    # review outputs reside in the separate target/ tree and cannot affect recovery decisions.
    install['recover'](base, target)
    exact(target, audit['source_files'])
    checks += 1
    # Recovery preflights *both* trees before touching either: ambiguous data stays untouched.
    path = target / next(row['path'] for row in audit['source_files'] if row['path'].endswith('.md'))
    before = path.read_bytes()
    profile['plan_producer'](ROOT, sources=sources, output_files=(path,))
    path.write_bytes(before + b'controlled drift\n')
    try:
        install['recover'](base, target)
    except ValueError as error:
        assert 'unrecognized transaction bytes' in str(error)
    else:
        raise AssertionError('recovery admitted source drift')
    assert path.read_bytes() == before + b'controlled drift\n'
    checks += 1
    profile['plan_producer'](ROOT, sources=sources, output_files=(path,))
    path.write_bytes(before)
    # Hash/audit drift must refuse before a transaction directory or either tree is changed.
    for key in ('generation_reader_sha256', 'installer_sha256', 'producer_sha256', 'reader_sha256', 'profile_sha256'):
        base, target = fixture('drift-' + key)
        changed = dict(audit, **{key: '0' * 64})
        file = base / 'source.json'
        profile['plan_producer'](ROOT, sources=sources, output_files=(file,))
        file.write_bytes((json.dumps(changed) + '\n').encode())
        try:
            install['install'](base, target)
        except ValueError as error:
            assert 'migration producer drift' in str(error)
        else:
            raise AssertionError('producer drift admitted: ' + key)
        assert not (base / 'installation').exists()
        exact(target, audit['source_files'])
        checks += 1
    # Calibrate D264 against the actual installer body with only its self-hash arm omitted.
    original_body = INSTALLER.read_text()
    arm = "(Path(__file__), 'installer_sha256'), "
    assert original_body.count(arm) == 1
    namespace = dict(__file__=str(INSTALLER), __name__='installer_self_hash_omission')
    exec(compile(original_body.replace(arm, '', 1), str(INSTALLER), 'exec'), namespace)
    base, target = fixture('calibrated-installer-self-hash-omission')
    changed = dict(audit, installer_sha256='0' * 64)
    path = base / 'source.json'
    profile['plan_producer'](ROOT, sources=sources, output_files=(path,))
    path.write_bytes((json.dumps(changed) + '\n').encode())
    namespace['install'](base, target)
    exact(target, audit['candidate_files'])
    assert (base / 'installation/journal.json').is_file(), 'omission did not reach real effects'
    checks += 1
    # The flushed journal binds exact audit bytes, including whitespace and both identity lists.
    base, target = fixture('audit-journal-binding')
    install['install'](base, target)
    before = {relative: install['tree'](target, relative) for relative in install['TREES']}
    path = base / 'source.json'
    profile['plan_producer'](ROOT, sources=sources, output_files=(path,))
    path.write_bytes(path.read_bytes() + b' ')
    try:
        install['recover'](base, target)
    except ValueError as error:
        assert 'transaction journal drift' in str(error)
    else:
        raise AssertionError('changed audit bytes admitted by recovery')
    assert {relative: install['tree'](target, relative) for relative in install['TREES']} == before
    checks += 1
    # Calibrate the binding predicate with the actual journal body minus only its audit hash.
    arm = "                        'source_audit_sha256': audit_hash,\n"
    assert original_body.count(arm) == 1
    namespace = dict(__file__=str(INSTALLER), __name__='journal_audit_hash_omission')
    exec(compile(original_body.replace(arm, '', 1), str(INSTALLER), 'exec'), namespace)
    base, target = fixture('calibrated-journal-audit-hash-omission')
    namespace['install'](base, target)
    path = base / 'source.json'
    profile['plan_producer'](ROOT, sources=sources, output_files=(path,))
    path.write_bytes(path.read_bytes() + b' ')
    namespace['recover'](base, target)
    exact(target, audit['source_files'])
    checks += 1
    base, target = fixture('journal-read-bound')
    install['install'](base, target)
    before = {relative: install['tree'](target, relative) for relative in install['TREES']}
    path = base / 'installation/journal.json'
    profile['plan_producer'](ROOT, sources=sources, output_files=(path,))
    path.write_bytes(b'x' * 8193)
    try:
        install['recover'](base, target)
    except ValueError as error:
        assert 'byte bound exceeded' in str(error)
    else:
        raise AssertionError('unbounded transaction journal admitted')
    assert {relative: install['tree'](target, relative) for relative in install['TREES']} == before
    checks += 1
    assert all(path.read_bytes() == data for path, data in frozen.items()), 'original input drift'
    print('v2 install contract: %d controls/0 failures; four real rename interruptions, explicit rollback, offline materialization, native tar and ledger routes exact' % checks)
    print('owned fixtures retained: ' + work.relative_to(ROOT).as_posix())


if __name__ == '__main__':
    main()
