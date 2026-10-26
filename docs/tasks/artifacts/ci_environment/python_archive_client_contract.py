"""D222: logical retained records survive packing without weakening source proof."""
import ast
import contextlib
import gzip
import io
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'docs/tasks/artifacts/handoff/d217_probe.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
READER = ROOT / 'scripts/history_archive.py'
SHELL = ROOT / 'scripts/history_archive.sh'
CORE = (Path(__file__), ENTRY, PROFILE_SOURCE, READER, SHELL)
WORK = ROOT / 'target/archive-client-controls' / str(os.getpid())
AUTHORITY = WORK / 'packed'
RECORDS = ('docs/history/stitchcad-devnotes-part303.md',
           'docs/history/stitchcad-devnotes-part304.md',
           'docs/history/stitchcad-changelog-part208.md')
BEFORE = '768aebe'


def module(source=None, root=ROOT):
    ns = dict(__file__=str(ENTRY), __name__='archive_client_control')
    exec(compile(ENTRY.read_text() if source is None else source, str(ENTRY), 'exec'), ns)
    ns['ROOT'] = root
    if root != ROOT:
        ns['__file__'] = str(root / ENTRY.relative_to(ROOT))
        ns['PROFILE_SOURCE'] = root / PROFILE_SOURCE.relative_to(ROOT)
    return ns


def inputs():
    result = []
    for parent in (ROOT / '.doctrine/history_archive', ROOT / 'docs/history'):
        for folder, dirs, files in os.walk(parent, followlinks=False):
            PROFILE['directory'](ROOT, Path(folder).relative_to(ROOT).as_posix())
            for name in dirs:
                PROFILE['directory'](ROOT, (Path(folder) / name).relative_to(ROOT).as_posix())
            result.extend(Path(folder) / name for name in files)
    PROFILE['plan_producer'](ROOT, sources=(*CORE, *result))
    return tuple(sorted(result))


def reference_records():
    inputs()
    records, _ = runpy.run_path(str(READER))['inventory'](ROOT)
    return {name: records[name] for name in RECORDS}


@contextlib.contextmanager
def packed_fixture():
    sources = inputs()
    data = {path.relative_to(ROOT).as_posix(): path.read_bytes() for path in sources}
    archive = io.BytesIO()
    members = []
    history = runpy.run_path(str(READER))
    reference = reference_records()
    with tarfile.open(fileobj=archive, mode='w', format=tarfile.PAX_FORMAT) as handle:
        for name in RECORDS:
            if name not in data:
                continue  # A later production window may already pack this record.
            content = reference[name]
            row = tarfile.TarInfo(name)
            row.size = len(content)
            handle.addfile(row, io.BytesIO(content))
            members.append(dict(path=name, lines=content.count(b'\n'), bytes=len(content),
                                sha256=history['digest'](content)))
    payload = gzip.compress(archive.getvalue(), mtime=0)
    sha = history['digest'](payload)
    master = json.loads(data['.doctrine/history_archive/windows.json'])
    window = 'window' + str(len(master['windows']) + 1)
    revision = subprocess.check_output(['git', 'rev-parse', BEFORE], cwd=ROOT, text=True).strip()
    manifest = dict(version=1, id=window, source_revision=revision,
                    created='2026-10-25', owner='G1-SLICE.5b.4c.h2.b.p.o.f.archive-client',
                    reason='Isolated D222 packed-record control; never a production window.',
                    payload='docs/history/payloads/' + sha + '.tar.gz', payload_sha256=sha,
                    catalog='docs/history/' + window + '.md', members=members)
    if members:
        control = '.doctrine/history_archive/' + window + '.json'
        master['windows'].append(control)
        data['.doctrine/history_archive/windows.json'] = (json.dumps(master) + '\n').encode()
        data[control] = (json.dumps(manifest) + '\n').encode()
        data[manifest['catalog']] = ('# D222 isolated packed fixture\n\n' + ''.join(
            '### ' + Path(row['path']).name + '\n' for row in members)).encode()
        data[manifest['payload']] = payload
        for row in members:
            del data[row['path']]
    for path in (ENTRY, PROFILE_SOURCE, READER, SHELL):
        data[path.relative_to(ROOT).as_posix()] = path.read_bytes()
    files = tuple(AUTHORITY / name for name in data)
    directories = {WORK, AUTHORITY}
    for path in files:
        directories.update(parent for parent in path.parents if parent == AUTHORITY or
                           parent.is_relative_to(AUTHORITY))
    plan = dict(sources=(*CORE, *sources), directories=tuple(sorted(directories)), output_files=files)
    PROFILE['require'](not WORK.exists() and not WORK.is_symlink(), 'archive client work must be fresh')
    PROFILE['prepare_producer'](ROOT, **plan)
    try:
        for name, content in data.items():
            PROFILE['plan_producer'](ROOT, **plan)
            (AUTHORITY / name).write_bytes(content)
        recovered, _ = history['inventory'](AUTHORITY)
        assert all(recovered[name] == reference[name] for name in RECORDS)
        assert all(not (AUTHORITY / name).exists() for name in RECORDS)
        yield data
    finally:
        PROFILE['plan_producer'](ROOT, **plan)
        assert {path for path in WORK.rglob('*') if path.is_file()} <= set(files)
        assert shutil.rmtree.avoids_symlink_attacks
        shutil.rmtree(WORK)


def before_control():
    original = subprocess.check_output(['git', 'show', BEFORE + ':' + ENTRY.relative_to(ROOT).as_posix()],
                                       cwd=ROOT, text=True)
    ns = module(original, AUTHORITY)
    saved = subprocess.check_output

    def no_child(*args, **kwargs):
        raise AssertionError('old retained client dispatched before missing physical input refusal')

    try:
        subprocess.check_output = no_child
        try:
            ns['retained'](PROFILE)
        except PROFILE['Refusal'] as error:
            assert 'producer source is not a local regular file' in str(error), str(error)
        else:
            raise AssertionError('original raw-only client accepted packed fixture')
    finally:
        subprocess.check_output = saved
    print('D222 original actual retained body refuses valid packed records with absent raw paths')


def capture(source=None, *, live=False, corrupt=None):
    ns = module(source)
    saved_env = dict(os.environ)
    expected = {ENTRY, PROFILE_SOURCE, SHELL, READER,
                *(AUTHORITY / name for name in inputs_relative())}
    plans, children = [], []
    original = subprocess.check_output, subprocess.run, Path.mkdir, Path.write_bytes, Path.read_bytes
    checkpoint = 0
    reference = reference_records()
    real_environment = PROFILE['environment_for'](ROOT, os.environ, create=False)
    os.environ.update(HISTORY_ARCHIVE_ROOT=str(AUTHORITY), ARCHIVE_CLIENT_CALLER='preserved',
                      GIT_DIR=str(WORK / 'foreign-git'))

    def plan(root, **kwargs):
        assert root == ROOT and set(kwargs.get('sources', ())) == expected, 'D222 complete backing plan omitted'
        assert not kwargs.get('directories') and not kwargs.get('output_files'), 'D222 readonly plan gained outputs'
        plans.append(kwargs)
        return PROFILE['plan_producer'](root, **kwargs)

    def child(command, **kwargs):
        nonlocal checkpoint
        assert len(plans) > checkpoint, 'D222 child input recheck omitted'
        checkpoint = len(plans)
        environment = kwargs['env']
        assert kwargs['cwd'] == ROOT and environment['HISTORY_ARCHIVE_ROOT'] == str(AUTHORITY)
        assert environment['ARCHIVE_CLIENT_CALLER'] == 'preserved'
        assert all(environment[key] == real_environment[key] for key in PROFILE['STORES']), 'D222 caller stores reset'
        assert 'GIT_DIR' not in environment and environment['GIT_CONFIG_NOSYSTEM'] == '1', 'D222 Git scope omitted'
        children.append(command)
        return kwargs

    def output(command, **kwargs):
        child(command, **kwargs)
        if command[:3] == ['bash', 'scripts/history_archive.sh', 'read'] and not live:
            content = reference[command[3]]
            if command[3] == corrupt:
                header, body = content.split(b'\n---\n\n', 1)
                content = header + b'\n---\n\n' + bytes([body[0] ^ 1]) + body[1:]
            return content
        return original[1](command, stdout=subprocess.PIPE, check=True, **kwargs).stdout

    def run(command, **kwargs):
        child(command, **kwargs)
        assert command == ['bash', 'scripts/history_archive.sh', 'prove-source', 'window11']
        return original[1](command, **kwargs) if live else subprocess.CompletedProcess(command, 0)

    def raw_fallback(path, *args, **kwargs):
        # Feed the deliberate raw-read fault exact logical bytes so the original child-dispatch
        # assertion, rather than FileNotFoundError after packing, remains the independent oracle.
        for name, content in reference.items():
            if path == ROOT / name:
                return content
        return original[4](path, *args, **kwargs)

    def no_write(*args, **kwargs):
        raise AssertionError('D222 readonly client created output')

    try:
        subprocess.check_output, subprocess.run = output, run
        Path.mkdir = Path.write_bytes = no_write
        Path.read_bytes = raw_fallback
        result = ns['retained'](dict(PROFILE, plan_producer=plan))
        assert result == dict(complete_source_intervals=5, bytes=4144, exact=True,
                              retained_window_source_exact=True, followup_bytes=8170, whole_ledger_bytes=3030), result
        assert [command[3] for command in children if command[:3] == ['bash', 'scripts/history_archive.sh', 'read']] == list(RECORDS), 'D222 logical read omitted'
        assert len(children) == 8, 'D222 original provenance child population changed'
        assert children[-1][-2:] == ['prove-source', 'window11']
        assert dict(os.environ) == dict(saved_env, HISTORY_ARCHIVE_ROOT=str(AUTHORITY),
                                        ARCHIVE_CLIENT_CALLER='preserved', GIT_DIR=str(WORK / 'foreign-git'))
    finally:
        subprocess.check_output, subprocess.run, Path.mkdir, Path.write_bytes, Path.read_bytes = original
        os.environ.clear(); os.environ.update(saved_env)
    return result


def inputs_relative():
    return tuple(path.relative_to(AUTHORITY).as_posix()
                 for folder in (AUTHORITY / '.doctrine/history_archive', AUTHORITY / 'docs/history')
                 for path in folder.rglob('*') if path.is_file())


def original_provenance(source=None):
    """Compare every old assertion, Git argv and extraction expression independently."""
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + ENTRY.relative_to(ROOT).as_posix()], cwd=ROOT, text=True)
    current = ENTRY.read_text() if source is None else source
    bodies = []
    for source in (old, current):
        function = next(node for node in ast.parse(source).body if isinstance(node, ast.FunctionDef) and node.name == 'retained')
        assertions = [ast.dump(node, include_attributes=False) for node in ast.walk(function) if isinstance(node, ast.Assert)]
        # The body after the original first Git read is identical apart from the three record acquisitions
        # and the new child recheck. Comparing source retains slices, order, extraction and return fields.
        body = source.split("    original = subprocess.check_output(['git', 'show', 'cdc6a77:' + tree]", 1)[1].split('\n\ndef runtime(', 1)[0]
        body = body.replace("archive.read_bytes()", 'logical[0]').replace('followup.read_bytes()', 'logical[1]').replace('ledger.read_bytes()', 'logical[2]')
        body = body.replace('    environment = retained_plan(profile)[1]\n', '')
        bodies.append((assertions, body))
    assert bodies[0] == bodies[1], 'D222 original provenance changed'


def physical_refusals():
    saved_env = dict(os.environ)
    count = 0
    for case in ('input-link', 'input-directory', 'input-git', 'profile-link',
                 'authority-escape', 'authority-outside-target', 'toolchain'):
        ns = module()
        path = AUTHORITY / '.doctrine/history_archive/windows.json'
        content = path.read_bytes()
        marker = path.parent / '.git'
        os.environ['HISTORY_ARCHIVE_ROOT'] = str(AUTHORITY)
        original = Path.mkdir, Path.write_bytes, subprocess.check_output, subprocess.run

        def no_effect(*args, **kwargs):
            raise AssertionError('D222 invalid input reached an effect')

        try:
            if case == 'input-link':
                path.unlink(); path.symlink_to(ROOT / '.doctrine/history_archive/windows.json')
            elif case == 'input-directory':
                path.unlink(); path.mkdir()
            elif case == 'input-git':
                marker.mkdir()
            elif case == 'profile-link':
                path = AUTHORITY / 'scripts/local_environment.py'
                content = path.read_bytes()
                path.unlink(); path.symlink_to(PROFILE_SOURCE)
                ns['PROFILE_SOURCE'] = path
            elif case == 'authority-escape':
                os.environ['HISTORY_ARCHIVE_ROOT'] = 'target/../docs'
            elif case == 'authority-outside-target':
                os.environ['HISTORY_ARCHIVE_ROOT'] = 'docs/history'
            else:
                os.environ['RUSTUP_TOOLCHAIN'] = 'invalid/channel'
            Path.mkdir = Path.write_bytes = subprocess.check_output = subprocess.run = no_effect
            try:
                ns['retained'](PROFILE)
            except (ValueError, OSError):
                count += 1
            except AssertionError as error:
                raise AssertionError((case, str(error))) from error
            else:
                raise AssertionError('D222 physical refusal missing: ' + case)
        finally:
            Path.mkdir, Path.write_bytes, subprocess.check_output, subprocess.run = original
            if path.is_symlink():
                path.unlink()
            elif path.is_dir():
                path.rmdir()
            if marker.exists():
                marker.rmdir()
            path.write_bytes(content)
            os.environ.clear(); os.environ.update(saved_env)
    assert count == 7
    print('D222 seven physical input/profile/authority/toolchain refusals before effects')


def content_refusals(source=None):
    messages = ('whole committed task records differ', 'whole prior diagnostic records differ',
                'whole prior continuity ledgers differ')
    for record, message in zip(RECORDS, messages):
        try:
            capture(source, corrupt=record)
        except AssertionError as error:
            assert str(error) == message, ('unrelated D222 source red', str(error), message)
        else:
            raise AssertionError('D222 provenance admission')


def omission_controls():
    source = ENTRY.read_text()
    faults = (
        ('backing', 'sources=tuple(sources)', 'sources=tuple(sources[:4])', 'D222 complete backing plan omitted'),
        ('recheck', "    environment = retained_plan(profile)[1]\n    original = subprocess.check_output",
         '    original = subprocess.check_output', 'D222 child input recheck omitted'),
        ('Git-scope', "if not name.startswith('GIT_')", 'if True', 'D222 Git scope omitted'),
        ('logical-read', "        logical.append(subprocess.check_output(['bash', 'scripts/history_archive.sh', 'read', record],\n                                               cwd=ROOT, env=environment))",
         '        logical.append((ROOT / record).read_bytes())', 'D222 logical read omitted'),
    )
    for label, before, after, reason in faults:
        assert source.count(before) == 1, (label, 'actual body anchor')
        try:
            capture(source.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == reason, (label, 'unrelated omission red', str(error))
        else:
            raise AssertionError('D222 omission survived: ' + label)
    checks = ("assert body == b'\\n'.join(records)", 'assert retained_followup == expected',
              "assert retained_ledgers == b'\\n'.join(whole_ledgers)")
    content_refusals()
    for check in checks:
        assert source.count(check) == 1, 'D222 provenance body anchor'
        try:
            content_refusals(source.replace(check, 'assert True', 1))
        except AssertionError as error:
            assert str(error) == 'D222 provenance admission', str(error)
        else:
            raise AssertionError('D222 provenance omission survived')
    print('D222 seven compiled actual plan/recheck/Git/read/provenance assertion reds; three independent content refusals')


def retention():
    captures = (('stitchcad-changelog-part213.md', 'CHANGELOG.md', '## STITCHCAD-G1-0239 —'),
                ('stitchcad-devnotes-part309.md', 'docs/tasks/G1-SLICE-producers.md',
                 '- ID: `G1-SLICE.5b.4c.h2.b.p.o.a24`'))
    for name, source, heading in captures:
        data = subprocess.check_output(['bash', 'scripts/history_archive.sh', 'read', 'docs/history/' + name], cwd=ROOT)
        body = data.split(b'\n---\n\n', 1)[1]
        prior = subprocess.check_output(['git', 'show', BEFORE + ':' + source], cwd=ROOT)
        assert prior.count(body) == 1 and body.startswith(heading.encode())
        assert body.endswith(b'\n') and not body.endswith(b'\n\n')
        metadata = data.split(b'\n---\n\n', 1)[0].decode()
        start = prior.index(body)
        sha = runpy.run_path(str(READER))['digest'](body)
        assert '%s bytes %d–%d, %d bytes, sha256:%s' % (source, start, start + len(body), len(body), sha) in metadata
    earlier = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--',
                                       'docs/history', '.doctrine/history_archive'], cwd=ROOT).decode().splitlines()
    history = runpy.run_path(str(READER))
    logical, _ = history['inventory'](ROOT)
    backings = history['backing_inventory'](ROOT)
    for name in earlier:
        old = subprocess.check_output(['git', 'show', BEFORE + ':' + name], cwd=ROOT)
        if name == history['MASTER']:
            windows = json.loads(old)['windows']
            assert json.loads(backings[name])['windows'][:len(windows)] == windows
        elif history['SEGMENT'].fullmatch(name):
            assert logical[name] == old, name
        else:
            assert (backings[name] if name in backings else (ROOT / name).read_bytes()) == old, name
    print('D222 whole0239/a24 retained/public-read exact; predecessor logical/control/payload bytes unchanged')


if __name__ == '__main__':
    assert sys.argv[1:] in ([], ['--before'], ['--real'])
    frozen = {path: path.read_bytes() for path in (*CORE, *inputs())}
    with packed_fixture() as fixture:
        before_control()
        if sys.argv[1:] != ['--before']:
            original_provenance()
            result = capture(live=sys.argv[1:] == ['--real'])
            print('D222 complete readonly backing/8-child capture and exact original provenance:', json.dumps(result, sort_keys=True))
            physical_refusals()
            omission_controls()
        assert all((AUTHORITY / name).read_bytes() == content for name, content in fixture.items()), 'packed fixture changed'
    assert all(path.read_bytes() == content for path, content in frozen.items()), 'archive client source/backing changed'
    assert not WORK.exists(), 'archive client workspace residue'
    if sys.argv[1:] != ['--before']:
        retention()
