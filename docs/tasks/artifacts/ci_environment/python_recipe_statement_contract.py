"""Pure recipe/statement source plans and the exclusive nine-fault byte writer."""
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
import types

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
INPUT = HERE / 'formula_input.py'
INVENTORY = HERE / 'canonical_contract_inventory.py'
SOURCE = HERE / 'recipe_byte_contract.py'
WRITER = HERE / 'recipe_byte_mutations.py'
PURE = (SOURCE, HERE / 'statement_reference.py', HERE / 'recipe_reference.py')
ENTRIES = (*PURE, WRITER)
HELPERS = (INPUT, INVENTORY, HERE / 'formula_structure.py', HERE / 'static_signature_contract.py')
SHELL = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
CHAPTERS = tuple(ROOT / 'docs/book/src' / name for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md', 'spec/formula-language/examples.md',
    'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))
CANONICAL = (ROOT / 'docs/decisions/decision_canonical-expression-spelling.md',
             ROOT / 'docs/book/src/annexes/formula-literals.md')
FIXTURES = {
    SOURCE: (HERE / 'canonical_statement_cases.tsv', HERE / 'canonical_recipe_cases.tsv',
             ROOT / 'docs/decisions/decision_recipe-bytes.md'),
    PURE[1]: (HERE / 'statement_cases.tsv',),
    PURE[2]: (HERE / 'recipe_cases.tsv', HERE / 'recipe_coupled_cases.tsv'),
}
BOOT = {entry: {entry, PROFILE_SOURCE, *HELPERS, SHELL} for entry in PURE}
INPUTS = {entry: BOOT[entry] | set((*CHAPTERS, *CANONICAL, *FIXTURES[entry])) for entry in PURE}
INPUTS[WRITER] = INPUTS[SOURCE] | {WRITER}
WHOLE = set().union(*INPUTS.values())
WORK = ROOT / 'target/recipe_byte_mutations'
LOGS = {WORK / ('fault-%d.log' % number) for number in range(1, 10)}
GOOD = runpy.run_path(str(PROFILE_SOURCE))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in WHOLE}
SCRIPTS = {path: FROZEN[path].decode() for path in ENTRIES}
BEFORE = '03ebaa27c36fbaf0099317601c457fdab584ec9d'
ORIGINALS = {path: subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT, text=True)
             for path in ENTRIES}


def capture(entry, candidate=None, *, classifier=False, failure=False):
    """Execute actual entries/helpers; keep mutable source bytes and child verdicts modeled."""
    originals = (runpy.run_path, Path.read_bytes, Path.read_text, Path.write_bytes,
                 Path.write_text, Path.mkdir, os.mkdir, subprocess.run)
    saved, argv = dict(os.environ), sys.argv
    plans, writes, children, latest = [], [], [], {}
    shadow, activation, checkpoint, read_checkpoint = [FROZEN[SOURCE]], [False], [0], [0]
    environment = {}

    def owner():
        frame = sys._getframe(1)
        while frame and Path(frame.f_globals.get('__file__', '')) not in (*ENTRIES, *HELPERS):
            frame = frame.f_back
        return Path(frame.f_globals['__file__']) if frame else None

    def plan(root, **kwargs):
        assert root == ROOT, 'recipe/statement root changed'
        state = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == state, 'recipe/statement pure plan changed environment'
        who = owner()
        if who in PURE:
            assert not any(kwargs.get(name) for name in ('directories', 'output_files', 'mutable_sources')), 'pure reference requested effects'
        if who == WRITER and kwargs.get('output_files'):
            assert set(kwargs['sources']) == INPUTS[WRITER], 'recipe writer sources incomplete'
            assert set(kwargs['output_files']) == LOGS, 'recipe writer files incomplete'
            assert kwargs.get('directories') == (WORK,), 'recipe writer directory incomplete'
            assert kwargs.get('mutable_sources') == (SOURCE,), 'recipe writer mutable authority omitted'
        plans.append((who, kwargs)); latest[who] = (kwargs, len(plans))
        return result

    def enter(root, **kwargs):
        plan(root, **kwargs)
        assert set(kwargs.get('sources', ())) == INPUTS[WRITER], 'recipe writer sources incomplete'
        assert set(kwargs.get('output_files', ())) == LOGS, 'recipe writer files incomplete'
        assert kwargs.get('directories') == (WORK,), 'recipe writer directory incomplete'
        assert kwargs.get('mutable_sources') == (SOURCE,), 'recipe writer mutable authority omitted'
        activation[0] = True; checkpoint[0] = len(plans)
        environment.update(GOOD['environment_for'](ROOT, os.environ, create=False))
        os.environ.update(environment)
        return dict(environment)

    def complete(who, message):
        assert set(latest.get(who, ({}, 0))[0].get('sources', ())) == INPUTS[who], message

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, enter_producer=enter)
        who = owner()
        assert path in latest.get(who, ({}, 0))[0].get('sources', ()), 'recipe/statement loader before bootstrap'
        if who in PURE and path == INVENTORY:
            complete(who, 'recipe/statement inventory before complete plan')
        namespace = originals[0](file, *args, **kwargs)
        if path == INPUT and who in PURE:
            real = namespace['load_context']
            def context():
                complete(who, 'recipe/statement context before complete plan')
                return real()
            namespace['load_context'] = context
        return namespace

    def read(path, binary, *args, **kwargs):
        who = owner()
        if who == WRITER and path == SOURCE:
            complete(who, 'recipe writer read before complete plan')
            return shadow[0] if binary else shadow[0].decode()
        if who in PURE and path in (*FIXTURES[who], CHAPTERS[1]):
            complete(who, 'recipe/statement fixture before complete plan')
            generation = latest[who][1]
            assert generation > read_checkpoint[0], 'recipe/statement fixture recheck omitted'
            read_checkpoint[0] = generation
        return originals[1 if binary else 2](path, *args, **kwargs)

    def fresh(message):
        assert latest.get(WRITER, ({}, 0))[1] > checkpoint[0], message
        checkpoint[0] = len(plans)

    def write(path, data, binary, *args, **kwargs):
        assert activation[0], 'recipe writer write before activation'
        fresh('recipe writer write recheck omitted' if path == SOURCE else 'recipe writer log recheck omitted')
        planned = latest[WRITER][0]
        if path == SOURCE:
            assert planned.get('mutable_sources') == (SOURCE,), 'recipe writer mutable recheck omitted'
            if binary:
                assert set(planned['sources']) == {SOURCE} and not planned.get('output_files'), 'recipe restoration depends on unrelated paths'
            shadow[0] = data if binary else data.encode()
        else:
            assert path in LOGS and path in planned.get('output_files', ()), 'recipe writer log unplanned'
            assert children and latest[WRITER][1] > children[-1][1], 'recipe writer log recheck omitted'
        writes.append((path, len(plans))); return len(data)

    def mkdir(path, *args, **kwargs):
        assert activation[0] and Path(path) == WORK, 'recipe writer mkdir before activation'

    def child(arguments, *args, **kwargs):
        assert activation[0] and not args, 'recipe writer child before activation'
        fresh('recipe writer child recheck omitted')
        assert arguments == [sys.executable, '-I', '-B', str(SOURCE)], 'recipe writer child argv changed'
        assert kwargs.get('cwd') == ROOT and kwargs.get('capture_output') is True, 'recipe writer child interface changed'
        assert kwargs.get('env') == environment, 'recipe writer child stores lost'
        assert shadow[0] != FROZEN[SOURCE], 'recipe writer actual edit absent'
        if failure:
            raise RuntimeError('recipe injected child failure')
        children.append((arguments, len(plans)))
        return types.SimpleNamespace(returncode=1, stdout=b'assert actual == expected\n',
                                     stderr=b"AssertionError: ('recipe byte contract', 'a', 'b', 'c')\n")

    output = io.StringIO()
    try:
        sys.argv = [str(entry), *(['--classifier-only'] if classifier else ['unrelated'] if entry in PURE else [])]
        current_argv = sys.argv
        runpy.run_path = load
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.write_bytes = lambda path, data, *args, **kwargs: write(path, data, True, *args, **kwargs)
        Path.write_text = lambda path, data, *args, **kwargs: write(path, data, False, *args, **kwargs)
        Path.mkdir = os.mkdir = mkdir
        subprocess.run = child
        with contextlib.redirect_stdout(output):
            try:
                exec(compile(candidate or SCRIPTS[entry], str(entry), 'exec'), dict(__file__=str(entry), __name__='borrowed_recipe'))
            except SystemExit as error:
                assert classifier and error.code == 0, error
            except RuntimeError as error:
                assert failure and str(error) == 'recipe injected child failure', error
        assert sys.argv is current_argv, 'recipe/statement borrowed argv changed'
        assert shadow[0] == FROZEN[SOURCE], 'recipe writer source stranded'
        if entry in PURE or classifier:
            assert not activation[0] and not writes and not children and dict(os.environ) == saved, 'pure reference/classifier produced effects'
        elif failure:
            assert len(writes) == 2 and not children, 'recipe child-failure restoration changed'
        else:
            assert len(children) == 9 and len(writes) == 28 and {path for path, _ in writes} == LOGS | {SOURCE}, 'recipe writer complete effects changed'
        return len(plans), len(writes), len(children), output.getvalue().splitlines()[-1:]
    finally:
        (runpy.run_path, Path.read_bytes, Path.read_text, Path.write_bytes,
         Path.write_text, Path.mkdir, os.mkdir, subprocess.run) = originals
        os.environ.clear(); os.environ.update(saved); sys.argv = argv
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'recipe capture changed original input'


def before_controls():
    for entry in ENTRIES:
        wanted = 'recipe writer read before complete plan' if entry == WRITER else 'recipe/statement loader before bootstrap'
        try:
            capture(entry, ORIGINALS[entry])
        except AssertionError as error:
            assert str(error) == wanted, (entry.name, error)
        else:
            raise AssertionError('recipe/statement original gap escaped')
        print('recipe/statement original gap:', entry.name, wanted, 'rc=0; no effects')


def body_controls():
    class Unguarded(ast.NodeTransformer):
        def __init__(self):
            self.fixture_path = None
        def visit_Assign(self, node):
            if len(node.targets) == 1 and isinstance(node.targets[0], ast.Name) and node.targets[0].id == 'fixture_path':
                self.fixture_path = node.value
                return None
            return self.generic_visit(node)
        def visit_Name(self, node):
            if node.id == 'fixture_path' and isinstance(node.ctx, ast.Load):
                assert self.fixture_path is not None
                return self.fixture_path
            return node
        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name) and node.value.func.id == 'source_plan':
                return None
            return self.generic_visit(node)
    for entry in ENTRIES:
        old, new = ast.parse(ORIGINALS[entry]), ast.parse(SCRIPTS[entry])
        assertions = lambda tree: sorted(ast.dump(node) for node in ast.walk(tree) if isinstance(node, ast.Assert))
        assert assertions(old) == assertions(new), 'recipe/statement original assertions changed'
        if entry in PURE:
            old_body = ORIGINALS[entry].split('reference.reserved =', 1)[1]
            new_body = SCRIPTS[entry].split('reference.reserved =', 1)[1]
            project = lambda text: ast.dump(Unguarded().visit(ast.parse('reference.reserved =' + text)))
            assert project(old_body) == project(new_body), 'recipe/statement complete original syntax body changed'
        else:
            cases = lambda tree: next(node.value for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'CASES' for target in node.targets))
            assert ast.dump(cases(old)) == ast.dump(cases(new)), 'recipe original nine fault cases changed'
            fn = lambda tree: next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
            assert ast.dump(fn(old)) == ast.dump(fn(new)), 'recipe original classifier changed'
    faults = []
    for entry in PURE:
        faults.extend((
            (entry, "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'recipe/statement loader before bootstrap'),
            (entry, "        PROFILE['plan_producer'](ROOT, sources=%s)" % ('REFERENCE_INPUTS' if entry == PURE[1] else '(*REFERENCE_INPUTS, *extra)'), '        None', 'recipe/statement context before complete plan'),
        ))
    faults.extend((
        (SOURCE, '    source_plan((fixture_path,))\n    return [line.split', '    return [line.split', 'recipe/statement fixture recheck omitted'),
        (PURE[2], '    source_plan((fixture_path,))\n    rows =', '    rows =', 'recipe/statement fixture recheck omitted'),
        (WRITER, 'source_plan()\nORIGINAL =', 'ORIGINAL =', 'recipe writer read before complete plan'),
        (WRITER, 'range(1, 10)', 'range(1, 9)', 'recipe writer files incomplete'),
        (WRITER, 'mutable_sources=(SOURCE,))', 'mutable_sources=())', 'recipe writer mutable authority omitted'),
        (WRITER, "            PROFILE['plan_producer'](ROOT, **PLAN)\n            SOURCE.write_text", '            SOURCE.write_text', 'recipe writer write recheck omitted'),
        (WRITER, "            PROFILE['plan_producer'](ROOT, **PLAN)\n            result =", '            result =', 'recipe writer child recheck omitted'),
        (WRITER, "            PROFILE['plan_producer'](ROOT, **PLAN)\n            (WORK /", '            (WORK /', 'recipe writer log recheck omitted'),
        (WRITER, 'env=environment, capture_output=True', 'capture_output=True', 'recipe writer child stores lost'),
    ))
    for entry, anchor, replacement, wanted in faults:
        assert SCRIPTS[entry].count(anchor) == (3 if anchor == 'mutable_sources=(SOURCE,))' else 1), ('recipe fault anchor changed', anchor)
        try:
            capture(entry, SCRIPTS[entry].replace(anchor, replacement, 1))
        except AssertionError as error:
            assert str(error) == wanted, ('recipe unrelated guard red', anchor, str(error), wanted)
        else:
            raise AssertionError(('recipe actual omission escaped', anchor))
    print('recipe/statement bodies: all original assertions/complete syntax bodies/classifier/nine CASES exact;', len(faults), 'actual guard reds; rc=0')


def mirror(label, index):
    sandbox = ROOT / ('target/recipe-statement-' + label) / str(os.getpid()) / str(index)
    assert not sandbox.exists(), 'recipe/statement mirror already exists'
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
    saved, argv = dict(os.environ), sys.argv
    try:
        for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT',
                     'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'RUSTUP_TOOLCHAIN'):
            os.environ.pop(name, None)
        os.environ.update({name: str(sandbox / suffix) for name, suffix in GOOD['STORES'].items()})
        yield
    finally:
        os.environ.clear(); os.environ.update(saved); sys.argv = argv


@contextlib.contextmanager
def blocked_effects():
    hooks = ((Path, 'mkdir'), (os, 'mkdir'), (Path, 'write_text'), (Path, 'write_bytes'), (subprocess, 'run'))
    originals = [getattr(module, name) for module, name in hooks]
    def blocked(*args, **kwargs):
        raise AssertionError('recipe/statement refusal or pure mode produced effects')
    try:
        for module, name in hooks:
            setattr(module, name, blocked)
        yield
    finally:
        for (module, name), original in zip(hooks, originals):
            setattr(module, name, original)


def physical_controls():
    cases = ('missing', 'symlink', 'directory', 'nested-repo', 'missing-helper', 'symlink-helper',
             'missing-inventory', 'symlink-inventory', 'hardlinked-source', 'symlink-store', 'invalid-toolchain')
    refusals = positives = borrowed = 0
    for entry in ENTRIES:
        for index, case in enumerate(cases):
            sandbox, paths = mirror('physical-' + entry.stem, index)
            victim = paths[INPUT if case.endswith('helper') else INVENTORY if case.endswith('inventory')
                           else SOURCE if entry == WRITER else FIXTURES[entry][0]]
            wanted = 'producer source is not a local regular file'
            positive = case == 'hardlinked-source' and entry in PURE
            if case.startswith('missing'):
                victim.unlink()
            elif case.startswith('symlink') and case != 'symlink-store':
                victim.unlink(); victim.symlink_to(paths[PROFILE_SOURCE]); wanted = 'symlink producer source'
            elif case == 'directory':
                victim.unlink(); victim.mkdir()
            elif case == 'nested-repo':
                (victim.parent / '.git').mkdir(); wanted = 'store crosses another Git repository'
            elif case == 'hardlinked-source':
                alias = sandbox / 'target/read-alias'
                GOOD['prepare_producer'](ROOT, directories=(alias.parent,), output_files=(alias,))
                os.link(victim, alias); wanted = 'hard-linked mutable producer source'
            elif case == 'symlink-store':
                scratch = sandbox / 'target/scratch'
                GOOD['directory'](ROOT, scratch.parent.relative_to(ROOT).as_posix(), create=True)
                scratch.symlink_to(victim.parent, target_is_directory=True); wanted = 'symlink store component'
            stderr = io.StringIO()
            with isolated_environment(sandbox), blocked_effects(), contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(stderr):
                if case == 'invalid-toolchain':
                    os.environ['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'; wanted = 'toolchain override must be a name'
                sys.argv = [str(paths[entry]), *([] if entry == WRITER else ['unrelated'])]
                current, current_argv = dict(os.environ), sys.argv
                try:
                    runpy.run_path(str(paths[entry]), run_name='__main__')
                except SystemExit as error:
                    assert not positive and error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', (entry.name, case, stderr.getvalue())
                    refusals += 1
                else:
                    assert positive, ('recipe/statement physical refusal escaped', entry.name, case)
                    positives += 1
                if case in ('missing', 'symlink', 'directory', 'nested-repo'):
                    for _ in range(2):
                        stderr.seek(0); stderr.truncate()
                        try:
                            runpy.run_path(str(paths[entry]), run_name='borrowed_recipe')
                        except SystemExit as error:
                            assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n'
                        else:
                            raise AssertionError('recipe/statement borrowed refusal escaped')
                        borrowed += 1
                assert dict(os.environ) == current and sys.argv is current_argv, 'recipe/statement refusal/pure mode changed state'
    # The public classifier-only mode has no destination authority, even with an unsafe log.
    sandbox, paths = mirror('classifier', 0)
    log = sandbox / (WORK / 'fault-9.log').relative_to(ROOT)
    GOOD['prepare_producer'](ROOT, directories=(log.parent,), output_files=(log,))
    log.symlink_to(paths[SOURCE])
    with isolated_environment(sandbox), blocked_effects(), contextlib.redirect_stdout(io.StringIO()):
        state = dict(os.environ); sys.argv = [str(paths[WRITER]), '--classifier-only']; current_argv = sys.argv
        try:
            runpy.run_path(str(paths[WRITER]), run_name='__main__')
        except SystemExit as error:
            assert error.code == 0
        else:
            raise AssertionError('recipe classifier-only exit changed')
        assert dict(os.environ) == state and sys.argv is current_argv and log.is_symlink() and paths[SOURCE].read_bytes() == FROZEN[SOURCE]
    # Full producing mode rejects the same final log before editing its original.
    with isolated_environment(sandbox), blocked_effects(), contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()) as stderr:
        sys.argv = [str(paths[WRITER])]
        try:
            runpy.run_path(str(paths[WRITER]), run_name='__main__')
        except SystemExit as error:
            assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — symlink producer output\n', stderr.getvalue()
        else:
            raise AssertionError('recipe final output alias escaped')
    print('recipe/statement physical:', refusals + 1, 'actual CLI refusals;', positives + 1,
          'pure positives;', borrowed, 'repeated borrowed refusals; classifier-only/final-log boundary exact; rc=0')


def boundary_controls():
    """Load independent real providers to discriminate Refusal-class adoption omissions."""
    def check(entry, text, provider):
        real = runpy.run_path; providers = []
        def load(file, *args, **kwargs):
            namespace = real(file, *args, **kwargs)
            if Path(file) == provider:
                providers.append(namespace)
                def refused(*args, **kwargs):
                    raise namespace['PROFILE']['Refusal']('isolated recipe/statement delegated refusal')
                if provider == INPUT:
                    namespace['load_context'] = refused
                else:
                    namespace['PROFILE']['plan_producer'] = refused
            return namespace
        state, argv = dict(os.environ), sys.argv
        stderr = io.StringIO(); namespace = dict(__file__=str(entry), __name__='borrowed_recipe')
        try:
            runpy.run_path = load
            with blocked_effects(), contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(stderr):
                try:
                    exec(compile(text, str(entry), 'exec'), namespace)
                except SystemExit as error:
                    assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — isolated recipe/statement delegated refusal\n', stderr.getvalue()
                except Exception as error:
                    assert providers and type(error) is providers[-1]['PROFILE']['Refusal'], ('recipe unrelated boundary failure', error)
                    raise AssertionError('recipe/statement delegated refusal escaped') from None
                else:
                    assert namespace['PROFILE'] is providers[-1]['PROFILE'], 'recipe/statement actual provider identity changed'
                    raise AssertionError('recipe/statement delegated refusal absent')
            assert dict(os.environ) == state and sys.argv is argv, 'recipe/statement provider control changed state'
        finally:
            runpy.run_path = real
    for entry in PURE:
        for provider, name in ((INPUT, 'context'), (INVENTORY, 'inventory')):
            check(entry, SCRIPTS[entry], provider)
            anchor = "    PROFILE = %s['PROFILE']\n" % name
            assert SCRIPTS[entry].count(anchor) == 1
            try:
                check(entry, SCRIPTS[entry].replace(anchor, '', 1), provider)
            except AssertionError as error:
                wanted = 'recipe/statement delegated refusal escaped' if provider == INPUT else 'recipe/statement actual provider identity changed'
                assert str(error) == wanted, ('recipe unrelated adoption red', error)
            else:
                raise AssertionError('recipe/statement provider adoption omission escaped')
    print('recipe/statement providers: six actual adoption omission reds/named delegated refusals; independent real identities; state exact; rc=0')


def caller_fixture_controls():
    """Exercise real exported readers with a safe borrowed fixture and unsafe selected paths."""
    verdicts = gaps = reds = repeated = 0
    for entry in (SOURCE, PURE[2]):
        fixture = FIXTURES[entry][0]
        def arguments(path):
            return (str(path),) if entry == SOURCE else (str(path), 9)
        function = 'rows' if entry == SOURCE else 'verify_fixture'
        expected = ([line.split('\t') for line in FROZEN[fixture].decode().splitlines()
                     if line and not line.startswith('#')] if entry == SOURCE else (14, 18))
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
    print('recipe caller fixtures:', verdicts, 'actual selected-path verdicts/', repeated,
          'repeated borrowed calls;', gaps, 'original gaps/', reds, 'actual guard reds; safe results/state/bytes exact; rc=0')


def late_restore_capture(text, index):
    sandbox, paths = mirror('late-restore', index)
    real = subprocess.run; log = sandbox / (WORK / 'fault-1.log').relative_to(ROOT)
    def child(arguments, **kwargs):
        assert arguments == [sys.executable, '-I', '-B', str(paths[SOURCE])] and kwargs['cwd'] == sandbox
        assert paths[SOURCE].read_bytes() != FROZEN[SOURCE], 'recipe late actual edit absent'
        GOOD['plan_producer'](ROOT, directories=(log,)); log.mkdir()
        return types.SimpleNamespace(returncode=1, stdout=b'assert actual == expected\n',
                                     stderr=b"AssertionError: ('recipe byte contract', 'a', 'b', 'c')\n")
    try:
        with isolated_environment(sandbox), contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()) as stderr:
            sys.argv = [str(paths[WRITER])]; subprocess.run = child
            try:
                exec(compile(text, str(paths[WRITER]), 'exec'), dict(__file__=str(paths[WRITER]), __name__='__main__'))
            except SystemExit as error:
                assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — producer output is not a local regular file\n', stderr.getvalue()
            else:
                raise AssertionError('recipe late unsafe-log refusal escaped')
            assert paths[SOURCE].read_bytes() == FROZEN[SOURCE], 'recipe late source stranded'
    finally:
        subprocess.run = real
        GOOD['plan_producer'](ROOT, sources=(paths[SOURCE],), mutable_sources=(paths[SOURCE],))
        paths[SOURCE].write_bytes(FROZEN[SOURCE])


def late_restore_controls():
    guard = "PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))"
    assert SCRIPTS[WRITER].count(guard) == 2
    try:
        late_restore_capture(SCRIPTS[WRITER].replace(guard, "PROFILE['plan_producer'](ROOT, **PLAN)"), 0)
    except AssertionError as error:
        assert str(error) == 'recipe late source stranded', ('recipe unrelated late red', error)
    else:
        raise AssertionError('recipe late restoration coupling escaped')
    late_restore_capture(SCRIPTS[WRITER], 1)
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'recipe physical fixtures changed originals'
    print('recipe late restoration: actual mirrored edit/unsafe log/coupling red and source-only exact green; rc=0')


def continuity_controls():
    """Reconstruct retained whole intervals from Git and verify prior immutable inputs."""
    reader = ROOT / 'scripts/history_archive.py'
    records = tuple(ROOT / ('docs/history/' + name) for name in (
        'stitchcad-devnotes-part288.md', 'stitchcad-changelog-part199.md'))
    for record in records:
        data = record.read_bytes()
        header, content = data.split(b'\n---\n\n', 1)
        text = header.decode()
        revision = re.search(r'complete intervals from ([0-9a-f]{40})\.', text).group(1)
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', text)
        assert (content.count(b'\n'), len(content), hashlib.sha256(content).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3]), 'recipe/statement retained identity drift'
        chunks = []
        for path, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', text):
            source = subprocess.check_output(['git', 'show', revision + ':' + path], cwd=ROOT)
            chunk = source[int(start):int(stop)]
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest, 'recipe/statement retained source interval drift'
            chunks.append(chunk)
        assert chunks and b''.join(chunks).rstrip(b'\n') + b'\n' == content, 'recipe/statement whole retention incomplete'
        result = subprocess.run([sys.executable, '-I', '-B', str(reader), '--root', str(ROOT), 'read', record.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
        assert result.returncode == 0 and result.stdout == data, 'recipe/statement public retained read differs'
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT), 'recipe/statement prior archive changed'
    for path in WHOLE - set(ENTRIES):
        assert path.read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT), 'recipe/statement original reference input changed'
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    current = (ROOT / tree).read_text()
    headers = lambda value: re.findall(r'^- ID: `([^`]+)`$', value, re.M)
    assert headers(current) == headers(old), 'recipe/statement canonical task graph changed'
    print('recipe/statement continuity: two complete retained records/public reads; prior archive/reference bytes and task graph exact')



def real_controls():
    """Run the real instruments exclusively and verify all original inputs after each child."""
    work = ROOT / 'target/recipe-statement-real'
    outputs = tuple(work / (entry.stem + '.log') for entry in ENTRIES)
    plan = dict(sources=(*WHOLE, Path(__file__)), directories=(work,), output_files=outputs, mutable_sources=(SOURCE,))
    environment = GOOD['enter_producer'](ROOT, **plan)
    for entry, output in zip(ENTRIES, outputs):
        GOOD['plan_producer'](ROOT, **plan)
        result = subprocess.run([sys.executable, '-I', '-B', str(entry)], cwd=ROOT, env=environment, capture_output=True)
        GOOD['plan_producer'](ROOT, **plan); output.write_bytes(result.stdout + result.stderr)
        assert result.returncode == 0, (entry.name, result.returncode, result.stderr.decode())
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'recipe/statement real run stranded source'
        print('recipe/statement real:', entry.name, result.stdout.decode().splitlines()[-1], 'rc=0;', len(FROZEN), 'scoped inputs exact')


if __name__ == '__main__':
    if '--before' in sys.argv:
        before_controls()
    elif '--real' in sys.argv:
        real_controls()
    elif '--continuity' in sys.argv:
        continuity_controls()
    else:
        before_controls()
        body_controls()
        for entry in ENTRIES:
            print('recipe/statement capture:', entry.name, capture(entry))
        print('recipe classifier-only capture:', capture(WRITER, classifier=True))
        print('recipe child-failure capture:', capture(WRITER, failure=True))
        physical_controls()
        boundary_controls()
        caller_fixture_controls()
        late_restore_controls()
