"""Actual ledger fixture input/output plans, bounded teardown and dispatch controls."""
from pathlib import Path
import ast
import contextlib
import hashlib
import io
import os
import re
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'docs/tasks/artifacts/changelog/ledger_pointer_contract.py'
SHELL = ENTRY.with_name('run_changelog_ledger_probes.sh')
HELPER = ENTRY.with_name('changelog_pointers.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
READER = ROOT / 'scripts/history_archive.py'
LAUNCHER = ROOT / 'scripts/history_archive.sh'
CORE = (ENTRY, SHELL, HELPER, PROFILE_SOURCE, READER, LAUNCHER)
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
WORK = ROOT / 'target/ledger-fixture-controls' / str(os.getpid())
PROFILE['prepare_producer'](ROOT, sources=(*CORE, Path(__file__)), directories=(WORK,))
BEFORE = '5bde28b2f17254a31b006c5a70ba832600a59fe4'
OLD = subprocess.check_output(['git', 'show', BEFORE + ':' + ENTRY.relative_to(ROOT).as_posix()], text=True)
FROZEN = {path: path.read_bytes() for path in CORE}
folders, ARCHIVES = [ROOT / '.doctrine/history_archive', ROOT / 'docs/history'], []
for folder in folders:
    PROFILE['directory'](ROOT, folder.relative_to(ROOT).as_posix())
    for path in sorted(folder.iterdir()):
        PROFILE['require'](not path.is_symlink(), 'symlink ledger control archive input')
        (folders if path.is_dir() else ARCHIVES).append(path)
ARCHIVES = tuple(ARCHIVES)
PROFILE['plan_producer'](ROOT, sources=(*CORE, *ARCHIVES, Path(__file__)))
CONTENT = subprocess.check_output([sys.executable, '-I', '-B', str(READER), '--root', str(ROOT),
                                   'read', 'docs/history/stitchcad-changelog-part24.md'])


def mirror(label, *, archive=False):
    root = WORK / label
    inputs = (*CORE, *ARCHIVES) if archive else CORE
    files = tuple(root / path.relative_to(ROOT) for path in inputs)
    directories = tuple(sorted({root, *(path.parent for path in files)}))
    PROFILE['prepare_producer'](ROOT, sources=(*inputs, Path(__file__)),
                               directories=directories, output_files=files)
    for source, destination in zip(inputs, files):
        destination.write_bytes(source.read_bytes())
    return root


def before_control():
    root = mirror('before')
    entry = root / ENTRY.relative_to(ROOT)
    namespace = dict(__file__=str(entry), __name__='ledger_fixture_before')
    original = subprocess.check_output

    def child(argv, *args, **kwargs):
        assert argv == ['bash', 'scripts/history_archive.sh', 'read', 'docs/history/stitchcad-changelog-part24.md']
        assert kwargs['cwd'] == root
        raise subprocess.CalledProcessError(1, argv, output=b'controlled archive reader failure')

    try:
        subprocess.check_output = child
        try:
            exec(compile(OLD, str(entry), 'exec'), namespace)
        except subprocess.CalledProcessError as error:
            assert error.returncode == 1
        else:
            raise AssertionError('old reader failure not injected')
    finally:
        subprocess.check_output = original
    selected = root / 'target/scratch/ledger_pointer_contract'
    assert selected.is_dir() and not any(selected.iterdir()), 'old reader failure did not strand its new empty workspace'
    PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), directories=(selected,))
    shutil.rmtree(selected)
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('original actual archive-child failure strands its new empty workspace; isolated cleanup/source bytes exact')


def capture(label, script, *, masked=False, reader_failure=False):
    root = mirror(label, archive=True)
    entry, profile_source, shell = (root / path.relative_to(ROOT) for path in (ENTRY, PROFILE_SOURCE, SHELL))
    work = root / 'target/scratch/ledger_pointer_contract'
    dirs = {work.parent, work, *(work / str(index) / suffix for index in range(13) for suffix in ('', 'docs', 'docs/history'))}
    files = {work / str(index) / suffix for index in range(13) for suffix in (
        'docs/history/stitchcad-changelog-part24.md', 'CHANGELOG.md', 'order.txt', 'verdict.log')}
    inputs = {root / path.relative_to(ROOT) for path in (*CORE, *ARCHIVES)}
    real = runpy.run_path(str(profile_source))
    saved, argv = dict(os.environ), sys.argv
    stores = {name: str(root / 'target/caller-stores' / name.lower()) for name in real['STORES']}
    os.environ.update(stores, LEDGER_FIXTURE_CALLER='preserved')
    real['prepare_producer'](root, sources=tuple(inputs))
    originals = (runpy.run_path, Path.mkdir, Path.read_text, Path.write_text, Path.write_bytes,
                 shutil.rmtree, subprocess.run, subprocess.check_output)
    plans, activated, effects, children = [], [], [], []
    latest, written = {}, {}

    def owner():
        frame = sys._getframe(1)
        while frame:
            if frame.f_globals.get('__file__') == str(entry):
                return frame
            frame = frame.f_back
        return None

    def plan(selected_root, **kwargs):
        if owner():
            assert selected_root == root and set(kwargs.get('sources', ())) == inputs, 'ledger fixture complete inputs omitted'
            assert set(kwargs.get('directories', ())) == dirs and set(kwargs.get('output_files', ())) == files, 'ledger fixture complete destinations omitted'
            assert not kwargs.get('mutable_sources'), 'ledger fixture unowned source mutation'
            plans.append(kwargs); latest.clear(); latest.update(kwargs, generation=len(plans))
        return real['plan_producer'](selected_root, **kwargs)

    def environment(selected_root, variables, *, create=True):
        if owner() and create:
            assert latest, 'ledger fixture activation before complete plan'
            activated.append(latest['generation'])
        return real['environment_for'](selected_root, variables, create=create)

    def load(path, *args, **kwargs):
        if Path(path) == profile_source:
            assert not profile_source.is_symlink() and profile_source.is_file(), 'ledger fixture unsafe helper delegation'
            return dict(real, plan_producer=plan, environment_for=environment)
        return originals[0](path, *args, **kwargs)

    def mkdir(path, *args, **kwargs):
        if owner():
            assert activated and path in latest.get('directories', ()), 'ledger fixture mkdir before complete plan'
            effects.append(('mkdir', path))
        return originals[1](path, *args, **kwargs)

    def read(path, *args, **kwargs):
        if path == shell and owner():
            assert latest and not activated, 'ledger fixture source read before pure plan'
        return originals[2](path, *args, **kwargs)

    def write(path, data, binary, *args, **kwargs):
        if owner():
            assert activated and path in latest.get('output_files', ()), 'ledger fixture write outside complete plan'
            assert latest['generation'] > written.get('checkpoint', 0), 'ledger fixture write recheck omitted'
            written['checkpoint'] = latest['generation']; effects.append(('write', path))
        return originals[4 if binary else 3](path, data, *args, **kwargs)

    def remove(path, *args, **kwargs):
        if owner():
            assert path == work and latest and latest['generation'] > written.get('checkpoint', 0), 'ledger fixture removal recheck omitted'
            effects.append(('remove', path))
        return originals[5](path, *args, **kwargs)

    remove.avoids_symlink_attacks = originals[5].avoids_symlink_attacks

    def child_environment(kwargs):
        environment = kwargs.get('env')
        assert environment and all(environment.get(name) == value for name, value in stores.items()), 'ledger fixture child environment omitted'
        assert environment.get('LEDGER_FIXTURE_CALLER') == 'preserved'

    def reader(command, *args, **kwargs):
        if owner():
            child_environment(kwargs)
            assert command == ['bash', 'scripts/history_archive.sh', 'read', 'docs/history/stitchcad-changelog-part24.md'] and kwargs['cwd'] == root
            children.append(command)
            if reader_failure:
                raise subprocess.CalledProcessError(1, command, output=b'controlled reader failure')
            return CONTENT
        return originals[7](command, *args, **kwargs)

    def run(command, *args, **kwargs):
        if owner():
            child_environment(kwargs)
            assert command[:2] == ['bash', '-c'] and command[3:5] == ['ledger-pointer', str(root)] and kwargs['cwd'] == root
            fixture = Path(command[5]); assert Path(command[6]) == fixture / 'order.txt'
            text = (fixture / 'CHANGELOG.md').read_text()
            # Authored label-independent expected positives; simulated shell output proves dispatch only.
            positive = any(token in text for token in ('[Earlier](docs/history/', '[other.md](docs/history/window2.md#stitchcad-changelog-part24md)', '[stitchcad-changelog-part24.md](docs/history/window2.md#stitchcad-changelog-part24md)'))
            positive = positive and not any(token in text for token in ('introduction.md', '```', '<!--'))
            children.append(command)
            return subprocess.CompletedProcess(command, 0, 'POINTER PASS controlled\n' if positive else 'POINTER FAIL controlled\n', '')
        return originals[6](command, *args, **kwargs)

    namespace = dict(__file__=str(entry), __name__='ledger_fixture_capture')
    try:
        runpy.run_path, Path.mkdir, Path.read_text = load, mkdir, read
        Path.write_text = lambda path, data, *args, **kwargs: write(path, data, False, *args, **kwargs)
        Path.write_bytes = lambda path, data, *args, **kwargs: write(path, data, True, *args, **kwargs)
        shutil.rmtree, subprocess.run, subprocess.check_output = remove, run, reader
        sys.argv = [str(entry)] + (['--mask-first'] if masked else [])
        try:
            with contextlib.redirect_stdout(io.StringIO()):
                exec(compile(script, str(entry), 'exec'), namespace)
        except subprocess.CalledProcessError:
            assert reader_failure
        else:
            assert not reader_failure
        assert not work.exists(), 'ledger fixture reader failure left residue'
        assert len(activated) == 1 and len(children) == (1 if reader_failure else 14)
        assert all(path.read_bytes() == data for path, data in FROZEN.items())
        print('ledger capture:', 'reader failure' if reader_failure else 'masked' if masked else 'default', len(plans), 'plans;', len(effects), 'effect calls;', len(children), 'guarded children; sources/residue exact')
    finally:
        runpy.run_path, Path.mkdir, Path.read_text, Path.write_text, Path.write_bytes, shutil.rmtree, subprocess.run, subprocess.check_output = originals
        sys.argv = argv; os.environ.clear(); os.environ.update(saved)


def capture_controls():
    script = ENTRY.read_text()
    capture('default', script)
    capture('masked', script, masked=True)
    capture('reader-failure', script, reader_failure=True)
    faults = (
        ('inputs', 'sources=(*SOURCES, *backing_inputs()),', 'sources=SOURCES,', 'ledger fixture complete inputs omitted'),
        ('files', 'output_files=tuple(sorted(files)))', 'output_files=())', 'ledger fixture complete destinations omitted'),
        ('pure-read', 'try:\n    work_plan()\nexcept', 'try:\n    pass\nexcept', 'ledger fixture source read before pure plan'),
        ('removal', 'def remove_work():\n    work_plan()', 'def remove_work():\n    pass', 'ledger fixture removal recheck omitted'),
        ('write', "        work_plan()\n        order.write_text('')", "        order.write_text('')", 'ledger fixture write recheck omitted'),
        ('child', 'cwd=ROOT, env=environment, capture_output=True, text=True)', 'cwd=ROOT, capture_output=True, text=True)', 'ledger fixture child environment omitted'))
    for label, before, after, reason in faults:
        assert script.count(before) == 1, ('ledger omission anchor', label)
        try:
            capture('red-' + label, script.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == reason, (label, error)
        else:
            raise AssertionError(('ledger omission passed', label))
    start = script.index('try:\n    if WORK.exists():')
    stop = script.index('    for index, (name, row, expected)', start)
    setup = script[start + len('try:\n'):stop]
    fault = script[:start] + ''.join(line[4:] if line.startswith('    ') else line for line in setup.splitlines(keepends=True)) + 'try:\n' + script[stop:]
    try:
        capture('red-reader-scope', fault, reader_failure=True)
    except AssertionError as error:
        assert str(error) == 'ledger fixture reader failure left residue', error
    else:
        raise AssertionError('ledger setup scope omission passed')
    print('ledger actual omissions: full inputs/files/pure read/removal/write/child environment and original reader-scope reds')


def snapshot(root):
    result = {}
    for directory, directories, files in os.walk(root, followlinks=False):
        for name in [*directories, *files]:
            path = Path(directory) / name
            if path.is_symlink():
                value = ('link', os.readlink(path))
            elif path.is_file():
                value = ('file', path.read_bytes())
            else:
                value = ('other', path.lstat().st_mode)
            result[path.relative_to(root).as_posix()] = value
    return result


def physical_controls():
    labels = ('source-missing', 'source-link', 'profile-link', 'late-input-link',
              'work-link', 'work-marker', 'late-file-link', 'late-file-hardlink',
              'late-file-directory', 'unknown-work-entry', 'toolchain', 'archive-overlap')
    for label in labels:
        root = mirror('physical-' + label, archive=True)
        entry, shell, profile = (root / path.relative_to(ROOT) for path in (ENTRY, SHELL, PROFILE_SOURCE))
        work = root / 'target/scratch/ledger_pointer_contract'
        last = work / '12/verdict.log'
        selected = root / ARCHIVES[-1].relative_to(ROOT)
        directories = [work.parent]
        if label in ('work-marker', 'unknown-work-entry', 'archive-overlap'):
            directories.append(work)
        if label.startswith('late-file-'):
            directories.append(last.parent)
        files = tuple(root / path.relative_to(ROOT) for path in (*CORE, *ARCHIVES))
        PROFILE['prepare_producer'](ROOT, sources=(*CORE, *ARCHIVES, Path(__file__)),
                                   directories=tuple(directories), output_files=(*files, last))
        environment = dict(os.environ)
        environment.update({name: str(root / relative) for name, relative in PROFILE['STORES'].items()})
        if label.startswith('source-'):
            shell.unlink()
            if label.endswith('link'):
                shell.symlink_to(SHELL)
        elif label == 'profile-link':
            profile.unlink(); profile.symlink_to(PROFILE_SOURCE)
        elif label == 'late-input-link':
            selected.unlink(); selected.symlink_to(ARCHIVES[-1])
        elif label == 'work-link':
            work.symlink_to(ROOT / 'target', target_is_directory=True)
        elif label == 'work-marker':
            (work / '.git').mkdir()
        elif label == 'unknown-work-entry':
            sentinel = work / 'unknown.data'
            PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), output_files=(sentinel,))
            sentinel.write_bytes(b'caller-owned sentinel\n')
        elif label == 'late-file-link':
            last.symlink_to(SHELL)
        elif label == 'late-file-hardlink':
            os.link(shell, last)
        elif label == 'late-file-directory':
            last.mkdir()
        elif label == 'toolchain':
            environment['RUSTUP_TOOLCHAIN'] = 'bad/toolchain'
        elif label == 'archive-overlap':
            environment['HISTORY_ARCHIVE_ROOT'] = str(work)
        before = snapshot(root)
        result = subprocess.run([sys.executable, '-I', '-B', str(entry)], cwd=root, env=environment,
                                capture_output=True, text=True)
        assert result.returncode == 2 and 'ledger fixtures refused:' in result.stderr, (label, result)
        assert snapshot(root) == before, ('ledger physical refusal changed fixture', label)
        assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('ledger physical controls: twelve actual CLI early/late input/file/store/marker/unknown/overlap refusals; snapshots and sources exact')


def original_body_controls():
    old, current = ast.parse(OLD), ast.parse(ENTRY.read_text())
    cases = next(node for node in old.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'CASES' for target in node.targets))
    mask = next(node for node in old.body if isinstance(node, ast.If) and isinstance(node.test, ast.Compare))
    old.body.remove(cases); old.body.remove(mask)
    position = next(index for index, node in enumerate(old.body) if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'source' for target in node.targets))
    old.body[position:position] = [cases, mask]
    setup_index = next(index for index, node in enumerate(old.body) if isinstance(node, ast.If) and isinstance(node.test, ast.Call))
    setup = old.body[setup_index:setup_index + 3]
    del old.body[setup_index:setup_index + 3]
    protected = next(node for node in old.body if isinstance(node, ast.Try))
    protected.body[:0] = setup

    class Project(ast.NodeTransformer):
        def visit_FunctionDef(self, node):
            return None if node.name in {'load_profile', 'backing_inputs', 'fixture_paths', 'work_plan', 'remove_work'} else self.generic_visit(node)

        def visit_Import(self, node):
            return None if [name.name for name in node.names] in (['os'], ['runpy']) else node

        def visit_Assign(self, node):
            if any(isinstance(target, ast.Name) and target.id in {'PROFILE_SOURCE', 'PROFILE', 'SOURCES', 'environment'} for target in node.targets):
                return None
            return self.generic_visit(node)

        def visit_Try(self, node):
            return None if node.handlers else self.generic_visit(node)

        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name):
                if node.value.func.id == 'work_plan':
                    return None
                if node.value.func.id == 'remove_work':
                    return ast.parse('shutil.rmtree(WORK)').body[0]
            return self.generic_visit(node)

        def visit_Call(self, node):
            self.generic_visit(node)
            node.keywords = [keyword for keyword in node.keywords if keyword.arg != 'env']
            return node

    assert ast.dump(Project().visit(current), include_attributes=False) == ast.dump(old, include_attributes=False), 'ledger original complete bodies/cases/argv/logs changed'
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('ledger original AST: complete thirteen cases/permutation/assertions/extraction/argv/logs exact through planning/environment/protected-reader adapters')


def real_controls():
    identities = {path: hashlib.sha256(path.read_bytes()).digest() for path in (*CORE, *ARCHIVES)}
    for arguments in ([], ['--mask-first']):
        PROFILE['plan_producer'](ROOT, sources=(*CORE, *ARCHIVES, Path(__file__)))
        result = subprocess.run([sys.executable, '-I', '-B', str(ENTRY), *arguments], cwd=ROOT,
                                env=dict(os.environ), capture_output=True, text=True)
        assert result.returncode == 0 and 'ledger pointer contracts: 13 independent actual verdicts / 0 fail' in result.stdout, result
        assert not (ROOT / 'target/scratch/ledger_pointer_contract').exists()
        assert all(hashlib.sha256(path.read_bytes()).digest() == digest for path, digest in identities.items())
    print('ledger real consumers: default/masked thirteen actual verdicts each; all core/backing inputs exact; zero workspace residue')


def continuity_controls():
    for name in ('stitchcad-devnotes-part294.md', 'stitchcad-changelog-part204.md', 'stitchcad-devnotes-part295.md'):
        path = ROOT / 'docs/history' / name
        data = path.read_bytes(); header, body = data.split(b'\n---\n\n', 1)
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', header.decode())
        assert (body.count(b'\n'), len(body), hashlib.sha256(body).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3])
        chunks = []
        for source, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', header.decode()):
            original = subprocess.check_output(['git', 'show', BEFORE + ':' + source], cwd=ROOT)
            chunk = original[int(start):int(stop)]; chunks.append(chunk)
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest
        assert b''.join(chunks).rstrip(b'\n') + b'\n' == body
        result = subprocess.run([sys.executable, '-I', '-B', str(READER), '--root', str(ROOT), 'read', path.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
        assert result.returncode == 0 and result.stdout == data
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT)
    defect = ROOT / 'docs/history/stitchcad-defects-part134.md'
    data = defect.read_bytes(); header, body = data.split(b'\n---\n\n', 1)
    identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', header.decode())
    assert (body.count(b'\n'), len(body), hashlib.sha256(body).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3])
    assert identity[3] == '890e114145c49c8db424505004ceb32d8e80c4c46966992c9d8d5411e735db4f'
    assert body.count(b'- **D215**') == 1 and b'Closed; mutation producer remains separate.' in body
    result = subprocess.run([sys.executable, '-I', '-B', str(READER), '--root', str(ROOT), 'read', defect.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
    assert result.returncode == 0 and result.stdout == data
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    headers = lambda text: re.findall(r'^- ID: `([^`]+)`$', text, re.M)
    expected_new = {'G1-SLICE.5b.4c.h2.b.p.o.a23.' + str(index) for index in (1, 2, 3)}
    assert [identity for identity in headers((ROOT / tree).read_text()) if identity not in expected_new] == headers(old)
    print('ledger continuity: three whole committed records/D215 authored repair/public reads and prior archives exact; task graph changes only three owned children')


if __name__ == '__main__':
    if sys.argv[1:] == ['--before']:
        before_control()
    elif sys.argv[1:] == ['--real']:
        real_controls()
    elif sys.argv[1:] == ['--continuity']:
        continuity_controls()
    else:
        assert not sys.argv[1:], 'control mode not yet implemented'
        before_control()
        capture_controls()
        physical_controls()
        original_body_controls()
