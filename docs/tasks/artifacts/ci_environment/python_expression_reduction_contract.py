"""Actual pure expression/reduction imports, full plans, emissions and physical boundaries."""
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
SHELL = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
ENTRIES = tuple(HERE / name for name in ('formula_expression_reference.py', 'normalized_expression_reference.py',
                                        'canonical_expression_reference.py', 'reduction_boundary_reference.py'))
PROVIDERS = dict(zip(ENTRIES, (STRUCTURE, INPUT, INVENTORY, INPUT)))
FUNCTIONS = dict(zip(ENTRIES, ('load_reference', 'load_context', None, 'load_context')))
FIXTURES = dict(zip(ENTRIES, (
    (HERE / 'expression_shapes.tsv',), (HERE / 'normalized_expression_shapes.tsv',),
    (HERE / 'canonical_expression_cases.tsv', HERE / 'canonical_worked_cases.tsv'),
    (HERE / 'reduction_boundary_cases.tsv',))))
CHAPTERS = tuple(ROOT / 'docs/book/src' / name for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md', 'spec/formula-language/examples.md',
    'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))
CANONICAL = (ROOT / 'docs/decisions/decision_canonical-expression-spelling.md', ROOT / 'docs/book/src/annexes/formula-literals.md')
BOOT = {entry: {entry, PROFILE_SOURCE, STRUCTURE, SIGNATURE, SHELL,
                *(() if entry == ENTRIES[0] else (INPUT,)),
                *((INVENTORY,) if entry == ENTRIES[2] else ())} for entry in ENTRIES}
INPUTS = {entry: BOOT[entry] | set((*CHAPTERS, *FIXTURES[entry], *(CANONICAL if entry == ENTRIES[2] else ()))) for entry in ENTRIES}
WHOLE = set().union(*INPUTS.values())
GOOD = runpy.run_path(str(PROFILE_SOURCE))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in WHOLE}
SCRIPTS = {path: FROZEN[path].decode() for path in ENTRIES}
BOOK_LINK = '\n[Reference guards](formula-statements.md#reference-instrument-source-guards) preserve pure checks.\n'
BEFORE = '3d1be74b6297a45b5544df23cba3b9ed4694aed8'
ORIGINALS = {path: subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT, text=True) for path in ENTRIES}


@contextlib.contextmanager
def blocked_effects():
    hooks = ((Path, 'mkdir'), (os, 'mkdir'), (Path, 'write_text'), (Path, 'write_bytes'), (subprocess, 'run'))
    originals = [getattr(module, name) for module, name in hooks]
    def blocked(*args, **kwargs):
        raise AssertionError('expression/reduction pure module produced effects')
    try:
        for module, name in hooks:
            setattr(module, name, blocked)
        yield
    finally:
        for (module, name), original in zip(hooks, originals):
            setattr(module, name, original)


def capture(entry, text=None, *, emit=False):
    """Run actual scripts/helpers with independent completeness and freshness observations."""
    real_load, real_read = runpy.run_path, Path.read_text
    state, argv = dict(os.environ), sys.argv
    expected = INPUTS[entry] - (set(FIXTURES[entry]) if emit else set())
    plans, latest, checkpoint = [], {}, [0]
    def owner():
        frame = sys._getframe(1)
        while frame and Path(frame.f_globals.get('__file__', '')) not in (*ENTRIES, STRUCTURE, INPUT, INVENTORY, SIGNATURE):
            frame = frame.f_back
        return Path(frame.f_globals['__file__']) if frame else None
    def plan(root, **kwargs):
        assert root == ROOT and not any(kwargs.get(name) for name in ('directories', 'output_files', 'mutable_sources')), 'expression/reduction pure plan requested effects'
        saved = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == saved, 'expression/reduction plan changed environment'
        plans.append((owner(), kwargs)); latest[owner()] = (kwargs, len(plans))
        return result
    def complete(message):
        assert set(latest.get(entry, ({}, 0))[0].get('sources', ())) == expected, message
    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan)
        who = owner()
        assert path in latest.get(who, ({}, 0))[0].get('sources', ()), 'expression/reduction loader before bootstrap'
        if who == entry and path == INVENTORY:
            complete('canonical provider before complete plan')
        namespace = real_load(file, *args, **kwargs)
        if who == entry and path == PROVIDERS[entry] and FUNCTIONS[entry]:
            name = FUNCTIONS[entry]; real = namespace[name]
            def context():
                complete('expression/reduction context before complete plan')
                result = real()
                checkpoint[0] = latest[entry][1]
                return result
            namespace[name] = context
        return namespace
    def read(path, *args, **kwargs):
        if owner() == entry and path in (*FIXTURES[entry], CHAPTERS[2]):
            complete('expression/reduction read before complete plan')
            assert latest[entry][1] > checkpoint[0], 'expression/reduction read recheck omitted'
            checkpoint[0] = latest[entry][1]
        return real_read(path, *args, **kwargs)
    output = io.StringIO()
    try:
        sys.argv = [str(entry), *(['--emit'] if emit else [])]; current_argv = sys.argv
        runpy.run_path, Path.read_text = load, read
        with blocked_effects(), contextlib.redirect_stdout(output):
            namespace = dict(__file__=str(entry), __name__='borrowed_expression')
            exec(compile(text or SCRIPTS[entry], str(entry), 'exec'), namespace)
        assert plans and namespace['PROFILE'] is namespace['CONTEXT']['PROFILE'], 'expression/reduction actual provider identity changed'
        assert dict(os.environ) == state and sys.argv is current_argv, 'expression/reduction borrowed state changed'
        if emit:
            assert output.getvalue().encode() == FROZEN[FIXTURES[entry][0]], 'reduction emitted population differs'
        return len(plans), (['176 rows; fixture bytes exact'] if emit else output.getvalue().splitlines()[-1:])
    finally:
        runpy.run_path, Path.read_text = real_load, real_read
        os.environ.clear(); os.environ.update(state); sys.argv = argv
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'expression/reduction capture changed original input'


def before_controls():
    for entry in ENTRIES:
        try:
            capture(entry, ORIGINALS[entry])
        except AssertionError as error:
            assert str(error) == 'expression/reduction loader before bootstrap', (entry.name, error)
        else:
            raise AssertionError('expression/reduction original gap escaped')
        print('expression/reduction original:', entry.name, 'loader before bootstrap; rc=0/no effects')


def body_controls():
    class Project(ast.NodeTransformer):
        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name) and node.value.func.id == 'source_plan':
                return None
            return self.generic_visit(node)
    starts = dict(zip(ENTRIES, ('OPERATORS', 'OPERATORS', 'fixture = ROOT', 'assert reference.units')))
    for entry in ENTRIES:
        old, new = ast.parse(ORIGINALS[entry]), ast.parse(SCRIPTS[entry])
        assertions = lambda tree: sorted(ast.dump(node) for node in ast.walk(tree) if isinstance(node, ast.Assert))
        assert assertions(old) == assertions(new), 'expression/reduction original assertions changed'
        suffix = lambda text: starts[entry] + text.split(starts[entry], 1)[1]
        assert ast.dump(Project().visit(ast.parse(suffix(ORIGINALS[entry])))) == ast.dump(Project().visit(ast.parse(suffix(SCRIPTS[entry])))), 'expression/reduction complete original body changed'
        if entry == ENTRIES[3]:
            factor = lambda tree: next(node.value for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'FACTORS' for target in node.targets))
            assert ast.dump(factor(old)) == ast.dump(factor(new)), 'reduction independent factors changed'
    faults = []
    for entry in ENTRIES:
        faults.extend(((entry, '    source_plan()\n    CONTEXT =', '    CONTEXT =', 'expression/reduction loader before bootstrap'),
                       (entry, "        PROFILE['plan_producer'](ROOT, sources=SOURCE_INPUTS)", "        PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", 'canonical provider before complete plan' if entry == ENTRIES[2] else 'expression/reduction context before complete plan')))
    faults.extend(((ENTRIES[0], 'source_plan()\nfor row', 'for row', 'expression/reduction read recheck omitted'),
                   (ENTRIES[1], 'source_plan()\nrows=', 'rows=', 'expression/reduction read recheck omitted'),
                   (ENTRIES[2], 'source_plan()\nchapter =', 'chapter =', 'expression/reduction read recheck omitted'),
                   (ENTRIES[2], 'source_plan()\nworked_rows =', 'worked_rows =', 'expression/reduction read recheck omitted'),
                   (ENTRIES[3], '    source_plan()\n    actual=', '    actual=', 'expression/reduction read recheck omitted')))
    for entry, anchor, replacement, wanted in faults:
        assert SCRIPTS[entry].count(anchor) == 1, ('expression guard anchor changed', anchor)
        try:
            capture(entry, SCRIPTS[entry].replace(anchor, replacement, 1))
        except AssertionError as error:
            assert str(error) == wanted, ('expression unrelated guard red', anchor, str(error), wanted)
        else:
            raise AssertionError(('expression/reduction guard omission escaped', anchor))
    print('expression/reduction bodies: complete original syntax/oracle bodies/assertions/factors exact;', len(faults), 'actual guard reds; rc=0')


def mirror(label, index):
    sandbox = ROOT / ('target/expression-reduction-' + label) / str(os.getpid()) / str(index)
    assert not sandbox.exists(), 'expression/reduction mirror exists'
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
            victim = paths[PROVIDERS[entry] if case.endswith('helper') else FIXTURES[entry][-1]]
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
                    assert case == 'hardlink', 'expression/reduction physical refusal escaped'
                    positives += 1
                if case in ('missing', 'symlink', 'directory', 'nested-repo'):
                    for _ in range(2):
                        stderr.seek(0); stderr.truncate()
                        try:
                            runpy.run_path(str(paths[entry]), run_name='borrowed_expression')
                        except SystemExit as error:
                            assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n'
                        else:
                            raise AssertionError('expression/reduction borrowed refusal escaped')
                        borrowed += 1
                assert dict(os.environ) == state and sys.argv is argv, 'expression/reduction physical state changed'
    entry = ENTRIES[3]
    for index, case in enumerate(('missing', 'symlink', 'directory')):
        sandbox, paths = mirror('emit-unused', index); fixture = paths[FIXTURES[entry][0]]
        fixture.unlink()
        if case == 'symlink':
            fixture.symlink_to(paths[PROFILE_SOURCE])
        elif case == 'directory':
            fixture.mkdir()
        output = io.StringIO()
        with isolated_environment(sandbox), blocked_effects(), contextlib.redirect_stdout(output):
            sys.argv = [str(paths[entry]), '--emit']; state, argv = dict(os.environ), sys.argv
            runpy.run_path(str(paths[entry]), run_name='__main__')
            assert output.getvalue().encode() == FROZEN[FIXTURES[entry][0]], 'reduction unused fixture changed emitted bytes'
            assert dict(os.environ) == state and sys.argv is argv, 'reduction emitter changed state'
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'expression/reduction physical fixtures changed originals'
    print('expression/reduction physical:', refused, 'CLI refusals/', positives,
          'read-only hardlink positives/', borrowed, 'repeated borrowed refusals; three unsafe-unused emit positives/exact176 rows; rc=0')


def boundary_controls():
    """Load independent real providers to discriminate Refusal-class adoption omissions."""
    def check(entry, text, provider):
        real = runpy.run_path; providers = []
        def load(file, *args, **kwargs):
            namespace = real(file, *args, **kwargs)
            if Path(file) == provider:
                providers.append(namespace)
                def refused(*args, **kwargs):
                    raise namespace['PROFILE']['Refusal']('isolated expression/reduction delegated refusal')
                if FUNCTIONS[entry]:
                    namespace[FUNCTIONS[entry]] = refused
                else:
                    namespace['PROFILE']['plan_producer'] = refused
            return namespace
        state, argv = dict(os.environ), sys.argv
        stderr = io.StringIO(); namespace = dict(__file__=str(entry), __name__='borrowed_expression')
        try:
            runpy.run_path = load
            with blocked_effects(), contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(stderr):
                try:
                    exec(compile(text, str(entry), 'exec'), namespace)
                except SystemExit as error:
                    assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — isolated expression/reduction delegated refusal\n', stderr.getvalue()
                except Exception as error:
                    assert providers and type(error) is providers[-1]['PROFILE']['Refusal'], ('expression unrelated boundary failure', error)
                    raise AssertionError('expression/reduction delegated refusal escaped') from None
                else:
                    assert namespace['PROFILE'] is providers[-1]['PROFILE'], 'expression/reduction actual provider identity changed'
                    raise AssertionError('expression/reduction delegated refusal absent')
            assert dict(os.environ) == state and sys.argv is argv, 'expression/reduction provider control changed state'
        finally:
            runpy.run_path = real
    for entry in ENTRIES:
        provider = PROVIDERS[entry]
        check(entry, SCRIPTS[entry], provider)
        anchor = "    PROFILE = CONTEXT['PROFILE']\n"
        assert SCRIPTS[entry].count(anchor) == 1
        try:
            check(entry, SCRIPTS[entry].replace(anchor, '', 1), provider)
        except AssertionError as error:
            wanted = 'expression/reduction delegated refusal escaped' if FUNCTIONS[entry] else 'expression/reduction actual provider identity changed'
            assert str(error) == wanted, ('expression unrelated adoption red', error)
        else:
            raise AssertionError('expression/reduction provider adoption omission escaped')
    print('expression/reduction providers: four actual adoption omission reds/named delegated refusals; independent identities/state exact; rc=0')


def continuity_controls():
    """Reconstruct retained whole intervals from Git and verify prior immutable inputs."""
    reader = ROOT / 'scripts/history_archive.py'
    records = tuple(ROOT / ('docs/history/' + name) for name in (
        'stitchcad-devnotes-part289.md', 'stitchcad-changelog-part200.md'))
    for record in records:
        data = record.read_bytes()
        header, content = data.split(b'\n---\n\n', 1)
        text = header.decode()
        revision = re.search(r'complete intervals from ([0-9a-f]{40})\.', text).group(1)
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', text)
        assert (content.count(b'\n'), len(content), hashlib.sha256(content).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3]), 'expression/reduction retained identity drift'
        chunks = []
        for path, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', text):
            source = subprocess.check_output(['git', 'show', revision + ':' + path], cwd=ROOT)
            chunk = source[int(start):int(stop)]
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest, 'expression/reduction retained source interval drift'
            chunks.append(chunk)
        assert chunks and b''.join(chunks).rstrip(b'\n') + b'\n' == content, 'expression/reduction whole retention incomplete'
        result = subprocess.run([sys.executable, '-I', '-B', str(reader), '--root', str(ROOT), 'read', record.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
        assert result.returncode == 0 and result.stdout == data, 'expression/reduction public retained read differs'
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT), 'expression/reduction prior archive changed'
    for path in WHOLE - set(ENTRIES) - {CANONICAL[1]}:
        assert path.read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT), 'expression/reduction original reference input changed'
    before_annex = subprocess.check_output(['git', 'show', BEFORE + ':' + CANONICAL[1].relative_to(ROOT).as_posix()], cwd=ROOT)
    assert CANONICAL[1].read_bytes().replace(BOOK_LINK.encode(), b'', 1) == before_annex, 'expression/reduction annex changed beyond the guarded reference link'
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    current = (ROOT / tree).read_text()
    headers = lambda value: re.findall(r'^- ID: `([^`]+)`$', value, re.M)
    assert headers(current) == headers(old), 'expression/reduction canonical task graph changed'
    print('expression/reduction continuity: two complete retained records/public reads; prior archive/reference bytes and task graph exact')



def real_controls():
    """Run actual pure children and compare the full scoped input population afterward."""
    work = ROOT / 'target/expression-reduction-real'
    cases = tuple((entry, ()) for entry in ENTRIES) + ((ENTRIES[3], ('--emit',)),)
    outputs = tuple(work / (entry.stem + ('-emit' if arguments else '') + '.log') for entry, arguments in cases)
    plan = dict(sources=(*WHOLE, Path(__file__)), directories=(work,), output_files=outputs)
    environment = GOOD['enter_producer'](ROOT, **plan)
    for (entry, arguments), output in zip(cases, outputs):
        GOOD['plan_producer'](ROOT, **plan)
        result = subprocess.run([sys.executable, '-I', '-B', str(entry), *arguments], cwd=ROOT, env=environment, capture_output=True)
        GOOD['plan_producer'](ROOT, **plan); output.write_bytes(result.stdout + result.stderr)
        assert result.returncode == 0, (entry.name, result.returncode, result.stderr.decode())
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'expression/reduction real run changed inputs'
        if arguments:
            assert result.stdout == FROZEN[FIXTURES[entry][0]] and not result.stderr, 'reduction actual emitted bytes differ'
        print('expression/reduction real:', entry.name, '176 emitted rows/fixture bytes exact' if arguments else result.stdout.decode().splitlines()[-1], 'rc=0;', len(FROZEN), 'inputs exact')


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
            print('expression/reduction capture:', entry.name, capture(entry))
        print('reduction emit capture:', capture(ENTRIES[3], emit=True))
        physical_controls()
        boundary_controls()
