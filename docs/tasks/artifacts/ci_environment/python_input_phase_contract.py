"""Actual pure input/phase imports, borrowed calls, full plans and physical boundaries."""
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
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
STRUCTURE = HERE / 'formula_structure.py'
INPUT = HERE / 'formula_input.py'
INVENTORY = HERE / 'canonical_contract_inventory.py'
SIGNATURE = HERE / 'static_signature_contract.py'
NAMESPACE = HERE / 'static_namespace_contract.py'
BYTE = HERE / 'recipe_byte_contract.py'
SHELL = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
ENTRIES = (HERE / 'recipe_input_review_reference.py', HERE / 'whole_phase_contract.py')
PROVIDERS = dict(zip(ENTRIES, (BYTE, NAMESPACE)))
FIXTURES = dict(zip(ENTRIES, ((HERE / 'canonical_worked_recipe_cases.tsv', HERE / 'recipe_refusal_stage_cases.tsv'), ())))
CHAPTERS = tuple(ROOT / 'docs/book/src' / name for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md', 'spec/formula-language/examples.md',
    'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))
CANONICAL = (ROOT / 'docs/decisions/decision_canonical-expression-spelling.md', ROOT / 'docs/book/src/annexes/formula-literals.md')
BOOT = {ENTRIES[0]: {ENTRIES[0], PROFILE_SOURCE, BYTE, INPUT, STRUCTURE, SIGNATURE, INVENTORY, SHELL},
        ENTRIES[1]: {ENTRIES[1], PROFILE_SOURCE, NAMESPACE, SIGNATURE, SHELL}}
INPUTS = {ENTRIES[0]: BOOT[ENTRIES[0]] | set((*CHAPTERS, *CANONICAL, *FIXTURES[ENTRIES[0]],
           HERE / 'canonical_statement_cases.tsv', HERE / 'canonical_recipe_cases.tsv',
           ROOT / 'docs/decisions/decision_recipe-bytes.md')),
          ENTRIES[1]: BOOT[ENTRIES[1]] | set(CHAPTERS)}
WHOLE = set().union(*INPUTS.values())
GOOD = runpy.run_path(str(PROFILE_SOURCE))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in WHOLE}
SCRIPTS = {path: FROZEN[path].decode() for path in ENTRIES}
BEFORE = '5a70e3d302584e122a6b5c27df7165d40edaa733'
ORIGINALS = {path: subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT, text=True) for path in ENTRIES}


@contextlib.contextmanager
def blocked_effects():
    hooks = ((Path, 'mkdir'), (os, 'mkdir'), (Path, 'write_text'), (Path, 'write_bytes'), (subprocess, 'run'))
    originals = [getattr(module, name) for module, name in hooks]
    def blocked(*args, **kwargs):
        raise AssertionError('input/phase pure module produced effects')
    try:
        for module, name in hooks:
            setattr(module, name, blocked)
        yield
    finally:
        for (module, name), original in zip(hooks, originals):
            setattr(module, name, original)


def capture(entry, text=None, *, main=False, mutations=False):
    """Observe complete/fresh actual plans at helper loads, source reads and borrowed calls."""
    real_load, real_text, real_bytes = runpy.run_path, Path.read_text, Path.read_bytes
    state, argv = dict(os.environ), sys.argv
    expected = INPUTS[entry]
    plans, latest, checkpoint = [], {}, [0]
    def owner():
        frame = sys._getframe(1)
        while frame and Path(frame.f_globals.get('__file__', '')) not in (*ENTRIES, STRUCTURE, INPUT, INVENTORY, SIGNATURE, NAMESPACE, BYTE):
            frame = frame.f_back
        return Path(frame.f_globals['__file__']) if frame else None
    def plan(root, **kwargs):
        assert root == ROOT and not any(kwargs.get(name) for name in ('directories', 'output_files', 'mutable_sources')), 'input/phase pure plan requested effects'
        saved = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == saved, 'input/phase plan changed environment'
        plans.append((owner(), kwargs)); latest[owner()] = (kwargs, len(plans))
        return result
    def complete(message):
        assert set(latest.get(entry, ({}, 0))[0].get('sources', ())) == expected, message
    def fresh(message):
        complete(message)
        assert latest[entry][1] > checkpoint[0], message
        checkpoint[0] = latest[entry][1]
    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan)
        who = owner()
        assert path in latest.get(who, ({}, 0))[0].get('sources', ()), 'input/phase loader before bootstrap'
        if who == entry:
            complete('input/phase provider before complete plan')
        namespace = real_load(file, *args, **kwargs)
        if who == entry and path == PROVIDERS[entry]:
            checkpoint[0] = latest[entry][1]
            if entry == ENTRIES[1]:
                original = namespace['LOADER']['load_reference']
                def reference(*args, **kwargs):
                    fresh('input/phase borrowed call recheck omitted')
                    return original(*args, **kwargs)
                namespace['LOADER']['load_reference'] = reference
        return namespace
    def read(path, *args, **kwargs):
        if owner() == entry:
            fresh('input/phase read recheck omitted')
        return real_text(path, *args, **kwargs)
    def read_bytes(path, *args, **kwargs):
        if owner() == entry:
            fresh('input/phase source read recheck omitted')
        return real_bytes(path, *args, **kwargs)
    output = io.StringIO()
    try:
        sys.argv = [str(entry), *(['--mutations'] if mutations else [])]; current_argv = sys.argv
        runpy.run_path, Path.read_text, Path.read_bytes = load, read, read_bytes
        with blocked_effects(), contextlib.redirect_stdout(output):
            namespace = dict(__file__=str(entry), __name__='__main__' if main else 'borrowed_input')
            exec(compile(text or SCRIPTS[entry], str(entry), 'exec'), namespace)
            if entry == ENTRIES[1] and not main:
                for _ in range(2):
                    assert namespace['contracts'](verbose=False) == 584
        provider = namespace['context' if entry == ENTRIES[0] else 'ORACLE']
        assert plans and namespace['PROFILE'] is provider['PROFILE'], 'input/phase actual provider identity changed'
        assert dict(os.environ) == state and sys.argv is current_argv, 'input/phase borrowed state changed'
        return len(plans), output.getvalue().splitlines()[-2:]
    finally:
        runpy.run_path, Path.read_text, Path.read_bytes = real_load, real_text, real_bytes
        os.environ.clear(); os.environ.update(state); sys.argv = argv
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'input/phase capture changed original input'


def before_controls():
    for entry in ENTRIES:
        try:
            capture(entry, ORIGINALS[entry])
        except AssertionError as error:
            assert str(error) == 'input/phase loader before bootstrap', (entry.name, error)
        else:
            raise AssertionError('input/phase original gap escaped')
        print('input/phase original:', entry.name, 'loader before bootstrap; rc=0/no effects')


def body_controls():
    class Project(ast.NodeTransformer):
        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name) and node.value.func.id == 'source_plan':
                return None
            return self.generic_visit(node)
        def visit_Try(self, node):
            if any(isinstance(handler.type, ast.Tuple) and any(isinstance(item, ast.Subscript) for item in handler.type.elts) for handler in node.handlers):
                return [self.visit(item) for item in node.body if not (isinstance(item, ast.Expr) and isinstance(item.value, ast.Call) and isinstance(item.value.func, ast.Name) and item.value.func.id == 'source_plan')]
            return self.generic_visit(node)
    for entry, start in zip(ENTRIES, ('section =', 'def contracts')):
        old, new = ast.parse(ORIGINALS[entry]), ast.parse(SCRIPTS[entry])
        assertions = lambda tree: sorted(ast.dump(node) for node in ast.walk(tree) if isinstance(node, ast.Assert))
        assert assertions(old) == assertions(new), 'input/phase original assertions changed'
        suffix = lambda text: start + text.split(start, 1)[1]
        assert ast.dump(Project().visit(ast.parse(suffix(ORIGINALS[entry])))) == ast.dump(Project().visit(ast.parse(suffix(SCRIPTS[entry])))), 'input/phase complete original body/FAULTS/argv changed'
    class SelectedFixture(Project):
        def visit_Assign(self, node):
            if any(isinstance(target, ast.Name) and target.id == 'fixture_path' for target in node.targets):
                return None
            return self.generic_visit(node)
        def visit_Name(self, node):
            if node.id == 'fixture_path':
                return ast.BinOp(left=ast.Name(id='HERE', ctx=ast.Load()), op=ast.Div(), right=ast.Name(id='name', ctx=ast.Load()))
            return node
    function = lambda text: next(node for node in ast.parse(text).body if isinstance(node, ast.FunctionDef) and node.name == 'rows')
    assert ast.dump(SelectedFixture().visit(function(SCRIPTS[ENTRIES[0]]))) == ast.dump(function(ORIGINALS[ENTRIES[0]])), 'input/phase selected fixture renderer changed'
    faults = tuple((entry, before, after, wanted) for entry in ENTRIES for before, after, wanted in (
        ('    source_plan()\n    ' + ('context' if entry == ENTRIES[0] else 'ORACLE') + ' =',
         '    ' + ('context' if entry == ENTRIES[0] else 'ORACLE') + ' =', 'input/phase loader before bootstrap'),
        ("sources=(*REFERENCE_INPUTS, *extra)" if entry == ENTRIES[0] else 'sources=REFERENCE_INPUTS',
         'sources=BOOT_INPUTS', 'input/phase provider before complete plan')))
    faults += ((ENTRIES[0], "    source_plan()\n    statement_bytes", "    statement_bytes", 'input/phase read recheck omitted'),
               (ENTRIES[0], '    source_plan((fixture_path,))\n', '', 'input/phase read recheck omitted'),
               (ENTRIES[1], '    source_plan()\n    try:', '    try:', 'input/phase borrowed call recheck omitted'),
               (ENTRIES[1], '    source_plan()\n    original =', '    original =', 'input/phase source read recheck omitted'),
               (ENTRIES[1], '        source_plan()\n        assert SOURCE', '        assert SOURCE', 'input/phase source read recheck omitted'))
    for entry, anchor, replacement, wanted in faults:
        assert SCRIPTS[entry].count(anchor) == 1, ('input/phase guard anchor changed', anchor)
        try:
            capture(entry, SCRIPTS[entry].replace(anchor, replacement, 1), main=entry == ENTRIES[1], mutations='assert SOURCE' in anchor)
        except AssertionError as error:
            assert str(error) == wanted, ('input/phase unrelated guard red', anchor, str(error), wanted)
        else:
            raise AssertionError(('input/phase guard omission escaped', anchor))
    print('input/phase bodies: all original assertions/complete contract bodies/FAULTS/argv exact;', len(faults), 'actual guard reds; rc=0')


def mirror(label, index):
    sandbox = ROOT / ('target/input-phase-' + label) / str(os.getpid()) / str(index)
    assert not sandbox.exists(), 'input/phase mirror exists'
    paths = {path: sandbox / path.relative_to(ROOT) for path in WHOLE}
    directories = {sandbox}
    for path in paths.values():
        directories.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
    GOOD['prepare_producer'](ROOT, sources=tuple(WHOLE), directories=tuple(directories), output_files=tuple(paths.values()))
    for path, destination in paths.items():
        shutil.copy2(path, destination)
    return sandbox, paths


@contextlib.contextmanager
def isolated_environment(sandbox):
    state, argv = dict(os.environ), sys.argv
    try:
        for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT',
                     'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'RUSTUP_TOOLCHAIN'):
            os.environ.pop(name, None)
        os.environ.update({name: str(sandbox / suffix) for name, suffix in GOOD['STORES'].items()})
        yield
    finally:
        os.environ.clear(); os.environ.update(state); sys.argv = argv


def physical_controls():
    refused = positives = borrowed = 0
    for entry in ENTRIES:
        for index, case in enumerate(('missing', 'symlink', 'directory', 'nested-repo', 'missing-helper', 'symlink-helper', 'hardlink', 'symlink-store', 'toolchain')):
            sandbox, paths = mirror('physical-' + entry.stem, index)
            victim = paths[PROVIDERS[entry] if case.endswith('helper') else (FIXTURES[entry][-1] if FIXTURES[entry] else SHELL)]
            wanted = 'producer source is not a local regular file'
            if case.startswith('missing'):
                victim.unlink()
            elif case in ('symlink', 'symlink-helper'):
                victim.unlink(); victim.symlink_to(paths[PROFILE_SOURCE]); wanted = 'symlink producer source'
            elif case == 'directory':
                victim.unlink(); victim.mkdir()
            elif case == 'nested-repo':
                (victim.parent / '.git').mkdir(); wanted = 'store crosses another Git repository'
            elif case == 'hardlink':
                alias = sandbox / 'target/read-alias'
                GOOD['prepare_producer'](ROOT, directories=(alias.parent,), output_files=(alias,)); os.link(victim, alias)
            elif case == 'symlink-store':
                scratch = sandbox / 'target/scratch'
                GOOD['directory'](ROOT, scratch.parent.relative_to(ROOT).as_posix(), create=True)
                scratch.symlink_to(victim.parent, target_is_directory=True); wanted = 'symlink store component'
            stderr, output = io.StringIO(), io.StringIO()
            with isolated_environment(sandbox), blocked_effects(), contextlib.redirect_stdout(output), contextlib.redirect_stderr(stderr):
                if case == 'toolchain':
                    os.environ['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'; wanted = 'toolchain override must be a name'
                sys.argv = [str(paths[entry])]; state, argv = dict(os.environ), sys.argv
                try:
                    runpy.run_path(str(paths[entry]), run_name='__main__')
                except SystemExit as error:
                    assert case != 'hardlink' and error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', (entry.name, case, stderr.getvalue())
                    refused += 1
                else:
                    assert case == 'hardlink', 'input/phase physical refusal escaped'
                    positives += 1
                if case in ('missing', 'symlink', 'directory', 'nested-repo'):
                    for _ in range(2):
                        stderr.seek(0); stderr.truncate()
                        try:
                            runpy.run_path(str(paths[entry]), run_name='borrowed_input')
                        except SystemExit as error:
                            assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n'
                        else:
                            raise AssertionError('input/phase borrowed refusal escaped')
                        borrowed += 1
                assert dict(os.environ) == state and sys.argv is argv, 'input/phase physical state changed'
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'input/phase physical fixtures changed originals'
    print('input/phase physical:', refused, 'CLI refusals/', positives,
          'read-only hardlink positives/', borrowed, 'repeated borrowed refusals; rc=0')


def caller_fixture_controls():
    """Exercise real exported readers with a safe borrowed fixture and unsafe selected paths."""
    verdicts = gaps = reds = repeated = 0
    for entry in ENTRIES[:1]:
        fixture = FIXTURES[entry][0]
        def arguments(path):
            return (str(path),)
        function = 'rows'
        expected = [line.split('\t') for line in FROZEN[fixture].decode().splitlines()
                    if line and not line.startswith('#')]
        def invoke(namespace, selected, *, observe=False, refusal=None):
            real = Path.read_text; stderr = io.StringIO()
            def read(path, *args, **kwargs):
                assert path != selected, 'caller selected read before validation'
                return real(path, *args, **kwargs)
            state, argv = dict(os.environ), sys.argv
            try:
                if observe:
                    Path.read_text = read
                with blocked_effects(), contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(stderr):
                    try:
                        actual = namespace[function](*arguments(selected))
                    except SystemExit as error:
                        assert refusal and error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + refusal + '\n', stderr.getvalue()
                    else:
                        assert refusal is None and actual == expected, 'borrowed fixture result changed'
                assert dict(os.environ) == state and sys.argv is argv, 'borrowed fixture call changed state'
            finally:
                Path.read_text = real
        for index, case in enumerate(('safe', 'missing', 'symlink', 'directory', 'nested-repo', 'outside')):
            sandbox, paths = mirror('caller-' + entry.stem, index)
            selected = sandbox / 'target/caller-input/caller.tsv'
            GOOD['prepare_producer'](ROOT, directories=(selected.parent,), output_files=(selected,))
            selected.write_bytes(FROZEN[fixture])
            wanted = None
            if case == 'missing':
                selected.unlink(); wanted = 'producer source is not a local regular file'
            elif case == 'symlink':
                selected.unlink(); selected.symlink_to(paths[fixture]); wanted = 'symlink producer source'
            elif case == 'directory':
                selected.unlink(); selected.mkdir(); wanted = 'producer source is not a local regular file'
            elif case == 'nested-repo':
                (selected.parent / '.git').mkdir(); wanted = 'store crosses another Git repository'
            elif case == 'outside':
                selected = fixture; wanted = 'producer path outside workspace'
            with isolated_environment(sandbox), blocked_effects(), contextlib.redirect_stdout(io.StringIO()):
                namespace = runpy.run_path(str(paths[entry]))
                for _ in range(2):
                    invoke(namespace, selected, observe=case != 'safe', refusal=wanted)
                    repeated += 1
                if case == 'safe':
                    assert selected.read_bytes() == FROZEN[fixture]
                if case == 'outside':
                    for label, text in (('original', ORIGINALS[entry]), ('omission', SCRIPTS[entry].replace('    source_plan((fixture_path,))', '    None', 1))):
                        alternate = dict(__file__=str(paths[entry]), __name__='borrowed_fixture_gap')
                        exec(compile(text, str(paths[entry]), 'exec'), alternate)
                        try:
                            invoke(alternate, selected, observe=True)
                        except AssertionError as error:
                            assert str(error) == 'caller selected read before validation', ('caller unrelated gap/red', label, error)
                        else:
                            raise AssertionError('caller fixture original/omission gap escaped')
                        if label == 'original':
                            gaps += 1
                        else:
                            reds += 1
            verdicts += 1
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'caller fixture controls changed originals'
    print('input/phase caller fixtures:', verdicts, 'actual selected-path verdicts/', repeated,
          'repeated borrowed calls;', gaps, 'original gaps/', reds, 'actual guard reds; safe results/state/bytes exact; rc=0')


def borrowed_late_controls():
    """Refuse drift of each imported module's own source before any later delegated operation."""
    repeated = 0
    for entry in ENTRIES:
        for index, case in enumerate(('missing', 'symlink', 'directory', 'nested-repo')):
            sandbox, paths = mirror('late-' + entry.stem, index)
            with isolated_environment(sandbox), blocked_effects(), contextlib.redirect_stdout(io.StringIO()):
                namespace = runpy.run_path(str(paths[entry]))
            victim = paths[entry]
            wanted = 'producer source is not a local regular file'
            if case == 'missing':
                victim.unlink()
            elif case == 'symlink':
                victim.unlink(); victim.symlink_to(paths[PROFILE_SOURCE]); wanted = 'symlink producer source'
            elif case == 'directory':
                victim.unlink(); victim.mkdir()
            else:
                (victim.parent / '.git').mkdir(); wanted = 'store crosses another Git repository'
            with isolated_environment(sandbox), blocked_effects(), contextlib.redirect_stdout(io.StringIO()):
                state, argv = dict(os.environ), sys.argv
                for _ in range(2):
                    stderr = io.StringIO()
                    with contextlib.redirect_stderr(stderr):
                        try:
                            if entry == ENTRIES[0]:
                                namespace['rows']('canonical_worked_recipe_cases.tsv')
                            else:
                                namespace['contracts'](verbose=False)
                        except SystemExit as error:
                            assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n'
                        else:
                            raise AssertionError('input/phase late borrowed source drift escaped')
                    assert dict(os.environ) == state and sys.argv is argv, 'input/phase late borrowed state changed'
                    repeated += 1
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'input/phase borrowed controls changed originals'
    print('input/phase late:', repeated, 'repeated borrowed source-drift refusals; state/source bytes exact; rc=0')


def boundary_controls():
    """Load independent real providers to discriminate Refusal-class adoption omissions."""
    def check(entry, text, provider):
        real = runpy.run_path; providers = []
        def load(file, *args, **kwargs):
            namespace = real(file, *args, **kwargs)
            if Path(file) == provider:
                providers.append(namespace)
                def refused(*args, **kwargs):
                    raise namespace['PROFILE']['Refusal']('isolated input/phase delegated refusal')
                if entry == ENTRIES[1]:
                    namespace['LOADER']['load_reference'] = refused
                else:
                    namespace['PROFILE']['plan_producer'] = refused
            return namespace
        state, argv = dict(os.environ), sys.argv
        stderr = io.StringIO(); namespace = dict(__file__=str(entry), __name__='borrowed_input')
        try:
            runpy.run_path = load
            with blocked_effects(), contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(stderr):
                try:
                    exec(compile(text, str(entry), 'exec'), namespace)
                    if entry == ENTRIES[1]:
                        namespace['contracts'](verbose=False)
                except SystemExit as error:
                    assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — isolated input/phase delegated refusal\n', stderr.getvalue()
                except Exception as error:
                    assert providers and type(error) is providers[-1]['PROFILE']['Refusal'], ('input/phase unrelated boundary failure', error)
                    raise AssertionError('input/phase delegated refusal escaped') from None
                else:
                    assert namespace['PROFILE'] is providers[-1]['PROFILE'], 'input/phase actual provider identity changed'
                    raise AssertionError('input/phase delegated refusal absent')
            assert dict(os.environ) == state and sys.argv is argv, 'input/phase provider control changed state'
        finally:
            runpy.run_path = real
    for entry in ENTRIES:
        provider = PROVIDERS[entry]
        check(entry, SCRIPTS[entry], provider)
        anchor = "    PROFILE = " + ("context['PROFILE']\n" if entry == ENTRIES[0] else "ORACLE['PROFILE']\n")
        assert SCRIPTS[entry].count(anchor) == 1
        try:
            check(entry, SCRIPTS[entry].replace(anchor, '', 1), provider)
        except AssertionError as error:
            wanted = 'input/phase actual provider identity changed' if entry == ENTRIES[0] else 'input/phase delegated refusal escaped'
            assert str(error) == wanted, ('input/phase unrelated adoption red', error)
        else:
            raise AssertionError('input/phase provider adoption omission escaped')
    print('input/phase providers: two actual adoption omission reds/named delegated refusals; independent identities/state exact; rc=0')


def continuity_controls():
    """Reconstruct retained whole intervals from Git and verify prior immutable inputs."""
    reader = ROOT / 'scripts/history_archive.py'
    records = tuple(ROOT / ('docs/history/' + name) for name in (
        'stitchcad-devnotes-part290.md', 'stitchcad-changelog-part201.md'))
    for record in records:
        data = record.read_bytes()
        header, content = data.split(b'\n---\n\n', 1)
        text = header.decode()
        revision = re.search(r'complete intervals from ([0-9a-f]{40})\.', text).group(1)
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', text)
        assert (content.count(b'\n'), len(content), hashlib.sha256(content).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3]), 'input/phase retained identity drift'
        chunks = []
        for path, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', text):
            source = subprocess.check_output(['git', 'show', revision + ':' + path], cwd=ROOT)
            chunk = source[int(start):int(stop)]
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest, 'input/phase retained source interval drift'
            chunks.append(chunk)
        assert chunks and b''.join(chunks).rstrip(b'\n') + b'\n' == content, 'input/phase whole retention incomplete'
        result = subprocess.run([sys.executable, '-I', '-B', str(reader), '--root', str(ROOT), 'read', record.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
        assert result.returncode == 0 and result.stdout == data, 'input/phase public retained read differs'
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT), 'input/phase prior archive changed'
    for path in WHOLE - set(ENTRIES):
        assert path.read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT), 'input/phase original reference input changed'
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    current = (ROOT / tree).read_text()
    headers = lambda value: re.findall(r'^- ID: `([^`]+)`$', value, re.M)
    assert headers(current) == headers(old), 'input/phase canonical task graph changed'
    print('input/phase continuity: two complete retained records/public reads; prior archive/reference bytes and task graph exact')



def real_controls():
    """Actual pure CLI modes, including all eleven original in-memory phase faults."""
    work = ROOT / 'target/input-phase-real'
    cases = ((ENTRIES[0], ()), (ENTRIES[1], ()), (ENTRIES[1], ('--mutations',)))
    outputs = tuple(work / (entry.stem + ('-mutations' if arguments else '') + '.log') for entry, arguments in cases)
    plan = dict(sources=(*WHOLE, Path(__file__)), directories=(work,), output_files=outputs)
    environment = GOOD['enter_producer'](ROOT, **plan)
    for (entry, arguments), output in zip(cases, outputs):
        GOOD['plan_producer'](ROOT, **plan)
        result = subprocess.run([sys.executable, '-I', '-B', str(entry), *arguments], cwd=ROOT, env=environment, capture_output=True)
        GOOD['plan_producer'](ROOT, **plan); output.write_bytes(result.stdout + result.stderr)
        assert result.returncode == 0, (entry.name, result.returncode, result.stderr.decode())
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'input/phase real run changed inputs'
        print('input/phase real:', entry.name, arguments, result.stdout.decode().splitlines()[-1], 'rc=0;', len(FROZEN), 'inputs exact')


if __name__ == '__main__':
    if '--before' in sys.argv:
        before_controls()
    elif '--continuity' in sys.argv:
        continuity_controls()
    elif '--real' in sys.argv:
        real_controls()
    else:
        before_controls()
        body_controls()
        for entry in ENTRIES:
            print('input/phase capture:', entry.name, capture(entry))
        print('input/phase CLI mutation capture:', capture(ENTRIES[1], main=True, mutations=True))
        physical_controls()
        caller_fixture_controls()
        borrowed_late_controls()
        boundary_controls()
