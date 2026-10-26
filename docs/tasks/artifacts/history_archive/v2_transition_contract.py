"""Source-pinned transition/immutability controls; only owned project-local copies are mutated."""
import argparse
import contextlib
import hashlib
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
READER = ROOT / 'scripts/history_archive.py'
PROFILE = ROOT / 'scripts/local_environment.py'
MASTER = '.doctrine/history_archive/windows.json'
BEFORE = '072af62dc5bf4b0b6c2da5d1aed40c64303f96d8'


def module(source):
    result = dict(__file__=str(READER), __name__='v2_transition_contract')
    exec(compile(source, str(READER), 'exec'), result)
    return result


@contextlib.contextmanager
def git_snapshot(revision, *, unreadable_master=False):
    original = subprocess.run
    def run(command, *args, **kwargs):
        selected = list(command)
        if unreadable_master and selected[3:5] == ['show', 'HEAD:' + MASTER]:
            return subprocess.CompletedProcess(command, 128, b'', b'controlled unreadable master')
        if selected and selected[0] == 'git':
            selected = [revision if value == 'HEAD' else revision + value[4:] if value.startswith('HEAD:') else value
                        for value in selected]
        return original(selected, *args, **kwargs)
    subprocess.run = run
    try:
        yield
    finally:
        subprocess.run = original


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('workspace', help='prepared repository-relative target/ workspace')
    args = parser.parse_args()
    assert args.workspace.startswith('target/') and '..' not in Path(args.workspace).parts
    prepared = ROOT / args.workspace
    work = ROOT / 'target/scratch/v2_transition_contract' / str(os.getpid())
    v1, v2 = work / 'v1', work / 'v2'
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), READER, PROFILE, prepared / 'source.json'), directories=(work,))
    source = READER.read_text()
    ns = module(source)
    candidate = prepared / 'candidate'
    inputs = ns['backing_sources'](candidate, profile)
    sources = (Path(__file__), prepared / 'source.json', *inputs)
    profile['plan_producer'](ROOT, sources=sources, directories=(work,))
    assert not work.exists()
    audit = json.loads((prepared / 'source.json').read_bytes())
    revision = audit['source_revision']
    original = {}
    for row in audit['source_files']:
        maximum = 160000 if row['path'].endswith('.md') else ns['_backing_limit'](row['path'])
        data = ns['_git_blob'](ROOT, revision, row['path'], maximum)
        assert len(data) == row['bytes'] and hashlib.sha256(data).hexdigest() == row['sha256']
        original[row['path']] = data
    copied = {path.relative_to(candidate).as_posix(): path.read_bytes() for path in inputs
              if path.is_relative_to(candidate / '.doctrine/history_archive') or path.is_relative_to(candidate / 'docs/history')}
    before = subprocess.check_output(['git', '-C', str(ROOT), 'show', BEFORE + ':scripts/history_archive.py'],
                                     env=ns['metadata_environment'](ROOT)).decode()
    old = module(before)
    dummy_name = 'docs/history/stitchcad-devnotes-part99999.md'
    outputs = [v1 / name for name in original] + [v2 / name for name in copied] + [v2 / dummy_name]
    directories = {work, v1, v2}
    for path in outputs:
        directories.update(parent for parent in path.parents if parent.is_relative_to(work))
    plan = dict(sources=sources, directories=tuple(sorted(directories)), output_files=tuple(outputs))
    frozen = {path: path.read_bytes() for path in sources}
    profile['enter_producer'](ROOT, **plan)
    for base, files in [(v1, original), (v2, copied)]:
        for name, data in files.items():
            profile['plan_producer'](ROOT, **plan)
            with (base / name).open('xb') as stream:
                stream.write(data)
    checks = 0

    def refusal(label, operation, expected):
        nonlocal checks
        try:
            operation()
        except ns['Refusal'] as error:
            assert expected in str(error), (label, str(error), expected)
        else:
            raise AssertionError(label + ' admitted')
        checks += 1

    def rewrite(base, path, data):
        profile['plan_producer'](ROOT, **plan)
        (base / path).write_bytes(data)

    try:
        with git_snapshot(revision):
            ns['guard_committed'](v1)
            ns['guard_committed'](v2)
        checks += 2
        with git_snapshot(revision, unreadable_master=True):
            assert old['guard_committed'](v1) is None
            refusal('D244 present fixture master', lambda: ns['guard_committed'](v1), 'committed HEAD metadata unavailable')
        needle = "'ls-tree', '--full-tree', '--name-only', 'HEAD'"
        assert source.count(needle) == 1
        broken = module(source.replace(needle, "'ls-tree', '--name-only', 'HEAD'", 1))
        with git_snapshot(revision, unreadable_master=True):
            try:
                refusal('D244 compiled root scope omission', lambda: broken['guard_committed'](v1), 'committed HEAD metadata unavailable')
            except AssertionError as error:
                assert str(error) == 'D244 compiled root scope omission admitted'
            else:
                raise AssertionError('D244 source omission survived')
        checks += 1
        master1 = json.loads(original[MASTER])
        packed_names = {row['path'] for name in master1['windows'] for row in json.loads(original[name])['members']}
        raw = sorted(name for name in original if ns['SEGMENT'].fullmatch(name) and name not in packed_names)[-1]
        rewrite(v1, raw, original[raw] + b'\nOwned raw edit control.\n')
        with git_snapshot(revision):
            assert old['guard_committed'](v1) is None  # Old guard's declared scope was retained windows.
            refusal('committed raw edit', lambda: ns['guard_committed'](v1), 'committed raw record changed')
        rewrite(v1, raw, original[raw])
        # All opaque bytes are available through the public CLI, without source-Git operations.
        backings = ns['backing_inventory'](v2)
        command = [sys.executable, '-I', '-B', str(READER), '--root', str(v2), 'read-backing']
        for name, data in backings.items():
            assert subprocess.check_output([*command, name], cwd=ROOT) == data
        checks += len(backings)
        result = subprocess.run([*command, '../escape'], cwd=ROOT, capture_output=True)
        assert result.returncode == 1 and b'unknown backing identity' in result.stderr
        checks += 1
        records = ns['inventory'](v2)[0]
        master2 = json.loads(copied[MASTER])
        active = master2['windows'][0]
        manifest = json.loads(copied[active])
        dummy = b'Owned future raw record.\n'
        rewrite(v2, dummy_name, dummy)
        golden = {**copied, **records, dummy_name: dummy}
        current_master, git_blob, git_paths, head = (ns[name] for name in ('_committed_master', '_git_blob', '_git_paths', '_head_revision'))
        ns['_committed_master'] = lambda _root: subprocess.CompletedProcess([], 0, copied[MASTER], b'')
        ns['_git_blob'] = lambda _root, _revision, path, maximum: golden[path]
        ns['_git_paths'] = lambda _root, _revision: {raw, dummy_name}
        ns['_head_revision'] = lambda _root: revision
        try:
            ns['guard_committed'](v2)
            checks += 1
            changed = dict(manifest, reason='Controlled valid metadata edit')
            rewrite(v2, active, (json.dumps(changed, indent=2) + '\n').encode())
            assert ns['inventory'](v2)[0][dummy_name] == dummy
            refusal('future manifest bytes', lambda: ns['guard_committed'](v2), 'committed manifest changed')
            rewrite(v2, active, copied[active])
            catalog = manifest['catalog']
            rewrite(v2, catalog, copied[catalog] + b'\nControlled prose edit.\n')
            assert ns['inventory'](v2)[0][dummy_name] == dummy
            refusal('future catalog bytes', lambda: ns['guard_committed'](v2), 'committed retained file changed')
            rewrite(v2, catalog, copied[catalog])
            row = master2['legacy_catalogs'][0]
            data = copied[row['path']] + b'\nControlled legacy edit.\n'
            altered = json.loads(copied[MASTER])
            altered['legacy_catalogs'][0].update(lines=data.count(b'\n'), bytes=len(data), sha256=hashlib.sha256(data).hexdigest())
            rewrite(v2, row['path'], data)
            rewrite(v2, MASTER, (json.dumps(altered, indent=2) + '\n').encode())
            ns['inventory'](v2)
            refusal('future legacy descriptor', lambda: ns['guard_committed'](v2), 'committed legacy catalog descriptors changed')
            rewrite(v2, row['path'], copied[row['path']])
            rewrite(v2, MASTER, copied[MASTER])
            rewrite(v2, dummy_name, dummy + b'changed\n')
            refusal('future raw bytes', lambda: ns['guard_committed'](v2), 'committed raw record changed')
            rewrite(v2, dummy_name, dummy)
            ns['guard_committed'](v2)
            checks += 1
        finally:
            ns['_committed_master'], ns['_git_blob'], ns['_git_paths'], ns['_head_revision'] = current_master, git_blob, git_paths, head
            profile['plan_producer'](ROOT, **plan)
            (v2 / dummy_name).unlink()
        with git_snapshot(revision):
            ns['guard_committed'](v2)
        assert all(path.read_bytes() == data for path, data in frozen.items()), 'source drift'
        print('v2 transition: %d controls/0 failures; D244 actual scope and compiled omission; %d public backings exact; original inputs unchanged' % (checks, len(backings)))
    finally:
        profile['plan_producer'](ROOT, **plan)
        entries = list(work.rglob('*'))
        assert all(not path.is_symlink() and not (path.is_dir() and (path / '.git').exists()) for path in entries)
        assert {path for path in entries if path.is_file()} <= set(outputs), 'unowned cleanup entry'
        for path in entries:
            if path.is_file(): path.unlink()
        for path in sorted((path for path in entries if path.is_dir()), key=lambda path: len(path.parts), reverse=True):
            path.rmdir()
        work.rmdir()


if __name__ == '__main__':
    main()
