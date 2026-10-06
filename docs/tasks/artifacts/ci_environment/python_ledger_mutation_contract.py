"""Independent actual ledger mutation plans, restoration and assertion-site controls."""
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
ENTRY = ROOT / 'docs/tasks/artifacts/changelog/ledger_pointer_mutations.py'
CONTRACT = ENTRY.with_name('ledger_pointer_contract.py')
SHELL = ENTRY.with_name('run_changelog_ledger_probes.sh')
HELPER = ENTRY.with_name('changelog_pointers.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
READER = ROOT / 'scripts/history_archive.py'
LAUNCHER = ROOT / 'scripts/history_archive.sh'
CORE = (ENTRY, CONTRACT, SHELL, HELPER, PROFILE_SOURCE, READER, LAUNCHER)
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BEFORE = '39861d115b7c83f1992ba993af2500837d93518b'
OLD = subprocess.check_output(['git', 'show', BEFORE + ':' + ENTRY.relative_to(ROOT).as_posix()], text=True)
WORK = ROOT / 'target/ledger-mutation-controls' / str(os.getpid())
PROFILE['prepare_producer'](ROOT, sources=(*CORE, Path(__file__)), directories=(WORK,))
FROZEN = {path: path.read_bytes() for path in CORE}
folders, ARCHIVES = [ROOT / '.doctrine/history_archive', ROOT / 'docs/history'], []
for folder in folders:
    PROFILE['directory'](ROOT, folder.relative_to(ROOT).as_posix())
    for path in sorted(folder.iterdir()):
        PROFILE['require'](not path.is_symlink(), 'symlink ledger mutation control input')
        (folders if path.is_dir() else ARCHIVES).append(path)
ARCHIVES = tuple(ARCHIVES)


def mirror(label, *, archives=False):
    root = WORK / label
    inputs = (*CORE, *ARCHIVES) if archives else CORE
    files = tuple(root / path.relative_to(ROOT) for path in inputs)
    PROFILE['prepare_producer'](ROOT, sources=(*inputs, Path(__file__)),
                               directories=tuple(sorted({root, *(path.parent for path in files)})),
                               output_files=files)
    for source, destination in zip(inputs, files):
        destination.write_bytes(source.read_bytes())
    return root


def before_control():
    root = mirror('before')
    entry = root / ENTRY.relative_to(ROOT)
    logs = tuple(root / 'target/ledger_pointer_mutations' / ('mutation-%d.log' % index) for index in range(1, 5))
    PROFILE['prepare_producer'](ROOT, sources=(*CORE, Path(__file__)),
                               directories=(logs[0].parent,), output_files=logs)
    originals = {root / path.relative_to(ROOT): path.read_bytes() for path in (SHELL, HELPER)}
    original = subprocess.run
    dispatch = []

    def child(command, *args, **kwargs):
        assert kwargs['cwd'] == root
        dispatch.append(command)
        if command[0] == 'bash' or kwargs.get('check'):
            return subprocess.CompletedProcess(command, 0, b'', b'')
        return subprocess.CompletedProcess(command, 1, b'', b'ledger_pointer_contract.py\nAssertionError: unrelated assertion\n')

    try:
        subprocess.run = child
        with contextlib.redirect_stdout(io.StringIO()) as output:
            exec(compile(OLD, str(entry), 'exec'), dict(__file__=str(entry), __name__='ledger_mutation_before'))
    finally:
        subprocess.run = original
    assert len(dispatch) == 9 and output.getvalue().count('actual contract assertion red') == 4
    assert all(path.read_bytes() == data for path, data in originals.items())
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('original actual classifier admits four unrelated assertion responses as pointer reds; exact sources restored')


NAMES = ('catalog target, short label', 'missing catalog',
         'valid pointer plus invalid target', 'fenced pseudo-index')


def response(index, kind=None):
    expected = index == 1
    stdout = 'POINTER FAIL controlled\n' if expected else 'POINTER PASS controlled\n'
    failures = ['POINTER FAIL controlled'] if expected else []
    evidence = (NAMES[index - 1], expected, failures, stdout, '')
    site = '    assert (not failures) == expected, (name, expected, failures, result.stdout, result.stderr)\n'
    if kind == 'site':
        site = '    assert unrelated_guard\n'
    elif kind == 'case':
        evidence = ('another case', *evidence[1:])
    elif kind == 'polarity':
        evidence = (evidence[0], not expected, *evidence[2:])
    elif kind == 'verdict':
        evidence = (evidence[0], expected, [], 'POINTER PASS controlled\n', '')
    elif kind == 'family':
        evidence = (evidence[0], expected, ['ORDER FAIL controlled'], 'ORDER FAIL controlled\n', '')
    tail = 'not a tuple' if kind == 'shape' else repr(evidence)
    return ('Traceback (most recent call last):\n  File "ledger_pointer_contract.py", line 1, in <module>\n' +
            site + 'AssertionError: ' + tail + '\n').encode()


def capture(label, script, *, bad=None, syntax_failure=False, write_failure=False, profile_alias=False):
    root = mirror(label, archives=True)
    entry, shell, helper, profile_source = (root / path.relative_to(ROOT) for path in (ENTRY, SHELL, HELPER, PROFILE_SOURCE))
    work = root / 'target/ledger_pointer_mutations'
    inputs = {root / path.relative_to(ROOT) for path in (*CORE, *ARCHIVES)}
    files = {work / ('mutation-%d.log' % index) for index in range(1, 5)}
    mutable = {shell, helper}
    real = runpy.run_path(str(profile_source))
    original_sources = {path: path.read_bytes() for path in mutable}
    saved = dict(os.environ)
    stores = {name: str(root / 'target/caller-stores' / name.lower()) for name in real['STORES']}
    os.environ.update(stores, LEDGER_MUTATION_CALLER='preserved')
    real['prepare_producer'](root, sources=tuple(inputs), mutable_sources=tuple(mutable))
    if profile_alias:
        PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), output_files=(profile_source,))
        profile_source.unlink(); profile_source.symlink_to(PROFILE_SOURCE)
    originals = (runpy.run_path, Path.mkdir, Path.read_bytes, Path.write_text, Path.write_bytes, subprocess.run)
    plans, effects, children, activated = [], [], [], []
    latest, checkpoint, case = {}, 0, 0

    def owner():
        frame = sys._getframe(1)
        while frame:
            if frame.f_globals.get('__file__') == str(entry):
                return frame
            frame = frame.f_back
        return None

    def plan(selected_root, **kwargs):
        if owner():
            assert selected_root == root and set(kwargs.get('sources', ())) == inputs, 'ledger mutation complete inputs omitted'
            assert set(kwargs.get('mutable_sources', ())) == mutable, 'ledger mutation mutable authority omitted'
            assert set(kwargs.get('directories', ())) == {work} and set(kwargs.get('output_files', ())) == files, 'ledger mutation complete outputs omitted'
            plans.append(kwargs); latest.clear(); latest.update(kwargs, generation=len(plans))
        return real['plan_producer'](selected_root, **kwargs)

    def environment(selected_root, variables, *, create=True):
        if owner() and create:
            assert latest, 'ledger mutation activation before complete plan'
            activated.append(latest['generation'])
        return real['environment_for'](selected_root, variables, create=create)

    def load(path, *args, **kwargs):
        if Path(path) == profile_source:
            assert not profile_source.is_symlink() and profile_source.is_file(), 'ledger mutation unsafe helper delegation'
            return dict(real, plan_producer=plan, environment_for=environment)
        return originals[0](path, *args, **kwargs)

    def fresh(reason):
        nonlocal checkpoint
        assert latest and latest['generation'] > checkpoint, reason
        checkpoint = latest['generation']

    def mkdir(path, *args, **kwargs):
        if owner():
            assert activated and path in latest.get('directories', ()), 'ledger mutation mkdir before complete plan'
            fresh('ledger mutation mkdir recheck omitted'); effects.append(('mkdir', path))
        return originals[1](path, *args, **kwargs)

    def read(path, *args, **kwargs):
        if owner() and path in mutable:
            assert latest, 'ledger mutation source read before pure plan'
            if activated:
                fresh('ledger mutation restored read recheck omitted')
            else:
                assert not effects, 'ledger mutation initial source read after effects'
        return originals[2](path, *args, **kwargs)

    def write(path, data, binary, *args, **kwargs):
        if owner():
            assert activated and path in (mutable | files), 'ledger mutation write outside complete plan'
            fresh('ledger mutation write recheck omitted'); effects.append(('write', path, binary))
        result = originals[4 if binary else 3](path, data, *args, **kwargs)
        if owner() and write_failure and not binary:
            raise RuntimeError('controlled post-write failure')
        return result

    def child(command, *args, **kwargs):
        nonlocal case
        if not owner():
            return originals[5](command, *args, **kwargs)
        environment = kwargs.get('env')
        assert environment and all(environment.get(name) == value for name, value in stores.items()), 'ledger mutation child environment omitted'
        assert environment.get('LEDGER_MUTATION_CALLER') == 'preserved' and kwargs['cwd'] == root
        fresh('ledger mutation child recheck omitted'); children.append(command)
        if command[0] == 'bash':
            assert command == ['bash', '-n', str(shell)] and kwargs.get('check') is True
            if syntax_failure:
                raise subprocess.CalledProcessError(1, command)
            return subprocess.CompletedProcess(command, 0, b'', b'')
        assert command == ['python3', '-I', '-B', 'docs/tasks/artifacts/changelog/ledger_pointer_contract.py']
        if kwargs.get('check'):
            assert all(originals[2](path) == data for path, data in original_sources.items()), 'ledger mutation baseline before restoration'
            return subprocess.CompletedProcess(command, 0, b'', b'')
        assert kwargs.get('capture_output') is True
        case += 1
        return subprocess.CompletedProcess(command, 1, b'', response(case, bad))

    try:
        runpy.run_path, Path.mkdir, Path.read_bytes = load, mkdir, read
        Path.write_text = lambda path, data, *args, **kwargs: write(path, data, False, *args, **kwargs)
        Path.write_bytes = lambda path, data, *args, **kwargs: write(path, data, True, *args, **kwargs)
        subprocess.run = child
        with contextlib.redirect_stdout(io.StringIO()):
            exec(compile(script, str(entry), 'exec'), dict(__file__=str(entry), __name__='ledger_mutation_capture'))
        assert not bad and not syntax_failure and not write_failure, 'ledger mutation refusal missing'
        assert len(children) == 9 and len(activated) == 1
        print('ledger mutation capture:', len(plans), 'complete plans;', len(effects), 'effect calls;', len(children), 'prepared children; sources exact')
    finally:
        runpy.run_path, Path.mkdir, Path.read_bytes, Path.write_text, Path.write_bytes, subprocess.run = originals
        os.environ.clear(); os.environ.update(saved)
        assert all(path.read_bytes() == data for path, data in FROZEN.items())
        restored = all(path.read_bytes() == data for path, data in original_sources.items())
        if not restored:
            PROFILE['plan_producer'](ROOT, sources=(*CORE, *original_sources, Path(__file__)),
                                     mutable_sources=tuple(original_sources))
            for path, data in original_sources.items():
                path.write_bytes(data)
            assert all(path.read_bytes() == data for path, data in original_sources.items())
        assert restored, 'ledger mutation capture restoration failed'


def capture_controls():
    script = ENTRY.read_text()
    capture('positive', script)
    for label, options, exception, reason in (
            ('syntax-failure', {'syntax_failure': True}, subprocess.CalledProcessError, None),
            ('write-failure', {'write_failure': True}, RuntimeError, 'controlled post-write failure'),
            ('site', {'bad': 'site'}, AssertionError, 'pointer red wrong assertion site'),
            ('case', {'bad': 'case'}, AssertionError, 'pointer red wrong case'),
            ('polarity', {'bad': 'polarity'}, AssertionError, 'pointer red invalid verdict'),
            ('shape', {'bad': 'shape'}, AssertionError, 'pointer red malformed evidence'),
            ('verdict', {'bad': 'verdict'}, AssertionError, 'pointer red invalid verdict'),
            ('family', {'bad': 'family'}, AssertionError, 'pointer red invalid verdict')):
        try:
            capture('failure-' + label, script, **options)
        except exception as error:
            assert reason is None or str(error) == reason, (label, error)
        else:
            raise AssertionError(('ledger mutation refusal missing', label))
    faults = (
        ('inputs', 'sources=(*SOURCES, *backing_inputs()),', 'sources=SOURCES,', 'ledger mutation complete inputs omitted'),
        ('mutable', 'mutable_sources=(shell, helper),', 'mutable_sources=(),', 'ledger mutation mutable authority omitted'),
        ('files', 'output_files=FILES)', 'output_files=())', 'ledger mutation complete outputs omitted'),
        ('pure-read', 'try:\n    producer_plan()\nexcept', 'try:\n    pass\nexcept', 'ledger mutation source read before pure plan'),
        ('write', '        producer_plan()\n        path.write_text(text)', '        path.write_text(text)', 'ledger mutation write recheck omitted'),
        ('restore', '        producer_plan()\n        path.write_bytes(original[path])', '        path.write_bytes(original[path])', 'ledger mutation write recheck omitted'),
        ('child', 'cwd=ROOT, env=environment, capture_output=True)', 'cwd=ROOT, capture_output=True)', 'ledger mutation child environment omitted'))
    for label, before, after, reason in faults:
        assert script.count(before) == 1, ('ledger mutation omission anchor', label)
        try:
            capture('red-' + label, script.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == reason, (label, error)
        else:
            raise AssertionError(('ledger mutation omission escaped', label))
    omitted = script.replace('        assert_pointer_red(index, output)\n', '', 1)
    try:
        capture('red-classifier', omitted, bad='site')
    except AssertionError as error:
        assert str(error) == 'ledger mutation refusal missing', error
    else:
        raise AssertionError('ledger mutation classifier omission escaped')
    before = '''finally:
    for path, content in original.items():
        producer_plan()
        path.write_bytes(content)
'''
    assert script.count(before) == 1
    try:
        capture('red-finally', script.replace(before, 'finally:\n    pass\n', 1), bad='site')
    except AssertionError as error:
        assert str(error) == 'ledger mutation capture restoration failed', error
    else:
        raise AssertionError('ledger mutation protected restoration omission escaped')
    try:
        capture('profile-alias', script, profile_alias=True)
    except SystemExit as error:
        assert error.code == 2
    else:
        raise AssertionError('ledger mutation profile alias admitted')
    start, stop = script.index('def load_profile():'), script.index('\n\ntry:\n    PROFILE = load_profile()')
    fault = script[:start] + "def load_profile():\n    return runpy.run_path(str(PROFILE_SOURCE))" + script[stop:]
    try:
        capture('red-profile-alias', fault, profile_alias=True)
    except AssertionError as error:
        assert str(error) == 'ledger mutation unsafe helper delegation', error
    else:
        raise AssertionError('ledger mutation bootstrap omission escaped')
    print('ledger mutation controls: two protected restoration failures/six unrelated-red refusals/ten actual omission reds/profile alias refusal; project bytes exact')


def snapshot(root):
    result = {}
    for directory, directories, files in os.walk(root, followlinks=False):
        for name in [*directories, *files]:
            path = Path(directory) / name
            value = ('link', os.readlink(path)) if path.is_symlink() else ('file', path.read_bytes()) if path.is_file() else ('other', path.lstat().st_mode)
            result[path.relative_to(root).as_posix()] = value
    return result


def physical_controls():
    labels = ('shell-hardlink', 'helper-hardlink', 'source-link', 'source-missing',
              'profile-link', 'late-input-link', 'work-link', 'work-marker',
              'last-file-link', 'last-file-hardlink', 'last-file-directory', 'toolchain', 'archive-overlap')
    for label in labels:
        root = mirror('physical-' + label, archives=True)
        entry, shell, helper, profile, contract = (root / path.relative_to(ROOT) for path in (ENTRY, SHELL, HELPER, PROFILE_SOURCE, CONTRACT))
        work = root / 'target/ledger_pointer_mutations'; last = work / 'mutation-4.log'
        selected = root / ARCHIVES[-1].relative_to(ROOT)
        PROFILE['prepare_producer'](ROOT, sources=(*CORE, *ARCHIVES, Path(__file__)),
                                   directories=(work.parent,), output_files=(last, profile, shell, helper, contract, selected))
        environment = dict(os.environ)
        environment.update({name: str(root / relative) for name, relative in PROFILE['STORES'].items()})
        if label.endswith('hardlink') and not label.startswith('last-'):
            path = shell if label.startswith('shell-') else helper
            linked = work.parent / ('linked-' + label)
            PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), output_files=(linked,))
            os.link(path, linked)
        elif label.startswith('source-'):
            contract.unlink()
            if label.endswith('link'):
                contract.symlink_to(CONTRACT)
        elif label == 'profile-link':
            profile.unlink(); profile.symlink_to(PROFILE_SOURCE)
        elif label == 'late-input-link':
            selected.unlink(); selected.symlink_to(ARCHIVES[-1])
        elif label == 'work-link':
            work.symlink_to(ROOT / 'target', target_is_directory=True)
        elif label == 'work-marker':
            work.mkdir(); (work / '.git').mkdir()
        elif label.startswith('last-file-'):
            work.mkdir()
            if label.endswith('link') and not label.endswith('hardlink'):
                last.symlink_to(SHELL)
            elif label.endswith('hardlink'):
                os.link(shell, last)
            else:
                last.mkdir()
        elif label == 'toolchain':
            environment['RUSTUP_TOOLCHAIN'] = 'bad/toolchain'
        elif label == 'archive-overlap':
            work.mkdir(); environment['HISTORY_ARCHIVE_ROOT'] = str(work)
        before = snapshot(root)
        result = subprocess.run([sys.executable, '-I', '-B', str(entry)], cwd=root, env=environment, capture_output=True, text=True)
        assert result.returncode == 2 and 'ledger mutations refused:' in result.stderr, (label, result)
        assert snapshot(root) == before, ('ledger mutation physical refusal changed mirror', label)
        assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('ledger mutation physical controls: thirteen actual CLI input/mutable/output/store/marker/overlap refusals; mirrors and project bytes exact')


def original_body_controls():
    old, current = ast.parse(OLD), ast.parse(ENTRY.read_text())
    cases = next(node for node in old.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'CASES' for target in node.targets))
    old.body.remove(cases)
    position = next(index for index, node in enumerate(old.body) if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'original' for target in node.targets))
    old.body.insert(position, cases)
    mkdir = next(node for node in old.body if isinstance(node, ast.Expr) and isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Attribute) and node.value.func.attr == 'mkdir')
    old.body.remove(mkdir); old.body.insert(next(index for index, node in enumerate(old.body) if isinstance(node, ast.Try)), mkdir)

    class Project(ast.NodeTransformer):
        def visit_FunctionDef(self, node):
            return None if node.name in {'load_profile', 'backing_inputs', 'producer_plan', 'assert_pointer_red'} else self.generic_visit(node)

        def visit_Import(self, node):
            return None if [name.name for name in node.names] in (['os'], ['runpy'], ['sys']) else node

        def visit_Assign(self, node):
            if any(isinstance(target, ast.Name) and target.id in {'PROFILE_SOURCE', 'PROFILE', 'SOURCES', 'FILES', 'EXPECTED', 'CONTRACT', 'environment'} for target in node.targets):
                return None
            return self.generic_visit(node)

        def visit_Try(self, node):
            return None if node.handlers else self.generic_visit(node)

        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name) and node.value.func.id in {'producer_plan', 'assert_pointer_red'}:
                return None
            return self.generic_visit(node)

        def visit_Call(self, node):
            self.generic_visit(node)
            node.keywords = [keyword for keyword in node.keywords if keyword.arg != 'env']
            return node

    assert ast.dump(Project().visit(current), include_attributes=False) == ast.dump(old, include_attributes=False), 'ledger mutation original complete bodies changed'
    print('ledger mutation original AST: all four edits/anchors/parsing/checks/argv/logs/restoration/baseline exact through declared plan/environment/classifier adapters')


def real_controls():
    identities = {path: hashlib.sha256(path.read_bytes()).digest() for path in (*CORE, *ARCHIVES)}
    PROFILE['plan_producer'](ROOT, sources=(*CORE, *ARCHIVES, Path(__file__)), mutable_sources=(SHELL, HELPER))
    result = subprocess.run([sys.executable, '-I', '-B', str(ENTRY)], cwd=ROOT, env=dict(os.environ), capture_output=True, text=True)
    assert result.returncode == 0 and result.stdout.count('actual contract assertion red') == 4, result
    assert 'ledger pointer mutations: four syntax-checked actual reds; sources restored byte-identically' in result.stdout
    assert 'ledger pointer contracts: 13 independent actual verdicts / 0 fail' in result.stdout
    assert all(hashlib.sha256(path.read_bytes()).digest() == digest for path, digest in identities.items())
    assert not (ROOT / 'target/scratch/ledger_pointer_contract').exists()
    print('ledger mutation real: four current expected case/site/verdict reds, native13 restored baseline; all core/archive bytes exact; zero contract residue')


def continuity_controls():
    for name in ('stitchcad-devnotes-part296.md', 'stitchcad-changelog-part205.md', 'stitchcad-devnotes-part297.md'):
        path = ROOT / 'docs/history' / name
        data = path.read_bytes(); header, body = data.split(b'\n---\n\n', 1)
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', header.decode())
        assert (body.count(b'\n'), len(body), hashlib.sha256(body).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3])
        chunks = []
        for source, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', header.decode()):
            original = subprocess.check_output(['git', 'show', BEFORE + ':' + source], cwd=ROOT)
            chunk = original[int(start):int(stop)]; chunks.append(chunk)
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest
        unfolded = body
        if name.endswith('297.md'):
            for before, after in ((b'log. Guarded fixture source79', b'log.\n  Guarded fixture source79'),
                                  (b'owned next .f.h.v. Master old', b'owned next .f.h.v.\n  Master old')):
                assert unfolded.count(after) == 1
                unfolded = unfolded.replace(after, before)
            assert max(map(len, body.splitlines())) <= 400 and max(map(len, unfolded.splitlines())) == 404
        assert b''.join(chunks).rstrip(b'\n') + b'\n' == unfolded
        result = subprocess.run([sys.executable, '-I', '-B', str(READER), '--root', str(ROOT), 'read', path.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
        assert result.returncode == 0 and result.stdout == data
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT)
    defect = ROOT / 'docs/history/stitchcad-defects-part135.md'
    data = defect.read_bytes(); header, body = data.split(b'\n---\n\n', 1)
    identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', header.decode())
    assert (body.count(b'\n'), len(body), hashlib.sha256(body).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3])
    assert identity[3] == '7783424017815de2433cf8a90db6716a053911ef9677176f20bd45c5876f5f76'
    assert body.count(b'- **D216**') == 1 and b'Closed; publication producer remains next' in body
    result = subprocess.run([sys.executable, '-I', '-B', str(READER), '--root', str(ROOT), 'read', defect.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
    assert result.returncode == 0 and result.stdout == data
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    headers = lambda text: re.findall(r'^- ID: `([^`]+)`$', text, re.M)
    assert headers((ROOT / tree).read_text()) == headers(old)
    print('ledger mutation continuity: three whole committed records/two exact receipt folds/D216 authored repair/public reads/prior archives/task graph exact')


if __name__ == '__main__':
    if sys.argv[1:] == ['--before']:
        before_control()
    elif sys.argv[1:] == ['--real']:
        real_controls()
    elif sys.argv[1:] == ['--continuity']:
        continuity_controls()
    else:
        assert not sys.argv[1:]
        before_control()
        capture_controls()
        physical_controls()
        original_body_controls()
