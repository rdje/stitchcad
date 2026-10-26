"""Actual cleanup dispatch safety controls; no real project deletion."""
from pathlib import Path
import contextlib
import io
import json
import os
import runpy
import shutil
import ast
import hashlib
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/artifact_cleanup/cleanup.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
LEGACY = ROOT / 'scripts/check_no_background_jobs.sh'
CURRENT = ROOT / 'scripts/check_handoff.sh'
CENSUS = ROOT / 'scripts/handoff_census.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
WORK = ROOT / 'target/cleanup-entry-controls' / str(os.getpid())
AUDIT = ROOT / 'target/artifact_cleanup_audit' / ('guard-' + str(os.getpid()))
PS, LSOF = WORK / 'bin/ps', WORK / 'bin/lsof'
VICTIM = WORK / 'victim.log'
PLAN_FILE = AUDIT / 'plan.json'
BASE_PLAN = dict(sources=(Path(__file__), SOURCE, PROFILE_SOURCE, LEGACY, CURRENT, CENSUS),
                 directories=(WORK, PS.parent, AUDIT), output_files=(PS, LSOF, VICTIM, PLAN_FILE))
ENVIRONMENT = PROFILE['enter_producer'](ROOT, **BASE_PLAN)
ORIGINAL = SOURCE.read_bytes()
BEFORE = '80e9d330bcd1be431203881e380a4452c2b019b0'
OLD_SOURCE = subprocess.check_output(['git', 'show', BEFORE + ':' + SOURCE.relative_to(ROOT).as_posix()], cwd=ROOT)


def missing_evidence_controls():
    """Actual old/new shell entry points and the actual old apply body, with deletion trapped."""
    PROFILE['plan_producer'](ROOT, **BASE_PLAN)
    PS.write_text('#!/bin/bash\nif [ "$1" = "-Ao" ]; then printf "%s 1 00:01 census\\n" "$PPID"; else printf "1\\n"; fi\n')
    PROFILE['plan_producer'](ROOT, **BASE_PLAN)
    LSOF.write_text('#!/bin/bash\nexit 1\n')
    PROFILE['plan_producer'](ROOT, **BASE_PLAN); PS.chmod(0o755)
    PROFILE['plan_producer'](ROOT, **BASE_PLAN); LSOF.chmod(0o755)
    environment = dict(ENVIRONMENT, PATH=str(PS.parent) + os.pathsep + ENVIRONMENT['PATH'])
    results = []
    for entry in (LEGACY, CURRENT):
        PROFILE['plan_producer'](ROOT, **BASE_PLAN)
        result = subprocess.run(['bash', str(entry)], cwd=ROOT, env=environment, capture_output=True, text=True)
        results.append(result)
    assert results[0].returncode == 0 and 'handoff: OK' in results[0].stdout, results[0]
    assert results[1].returncode == 2 and 'lsof unavailable (exit 1)' in results[1].stderr, results[1]
    PROFILE['plan_producer'](ROOT, **BASE_PLAN)
    VICTIM.write_bytes(b'controlled generated artifact\n')
    saved = VICTIM.read_bytes()
    module = dict(__file__=str(SOURCE), __name__='cleanup_original_gap')
    exec(compile(OLD_SOURCE, str(SOURCE), 'exec'), module)
    row = module['inspect'](VICTIM.relative_to(ROOT).as_posix(), set())
    frozen = {'candidates': [row], 'tracked_sha256': module['tracked_digest'](set()), 'skipped': []}
    PROFILE['plan_producer'](ROOT, **BASE_PLAN)
    PLAN_FILE.write_text(json.dumps(frozen))
    namespace = module['apply'].__globals__
    namespace['census'] = lambda: frozen
    namespace['tracked'] = lambda: set()
    namespace['inspect'] = lambda name, tracked: row
    run, unlink = subprocess.run, Path.unlink
    calls = []
    def child(command, *args, **kwargs):
        calls.append(command)
        return run(command, *args, **dict(kwargs, env=environment, capture_output=True, text=True))
    def deletion(path, *args, **kwargs):
        assert path != VICTIM, 'actual cleanup reached deletion after unavailable process evidence'
        return unlink(path, *args, **kwargs)
    try:
        subprocess.run, Path.unlink = child, deletion
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                module['apply'](AUDIT)
            except AssertionError as error:
                assert str(error) == 'actual cleanup reached deletion after unavailable process evidence', error
            else:
                raise AssertionError('actual legacy cleanup safety gap was not reproduced')
        assert calls == [['bash', str(LEGACY)]], calls
    finally:
        subprocess.run, Path.unlink = run, unlink
    assert VICTIM.read_bytes() == saved and SOURCE.read_bytes() == ORIGINAL
    assert not (AUDIT / 'result.json').exists()
    print('cleanup process gap: actual legacy checker0/current checker2 on unavailable lsof; actual apply reached trapped deletion; victim/source exact; rc=0')


def process_repair_controls():
    """Execute the repaired body and actual stale-dispatch fault against the same unavailable OS child."""
    environment = dict(ENVIRONMENT, PATH=str(PS.parent) + os.pathsep + ENVIRONMENT['PATH'])
    saved = VICTIM.read_bytes()
    text = ORIGINAL.decode()
    anchor = "HANDOFF = ROOT / 'scripts/check_handoff.sh'"
    assert text.count(anchor) == 1
    for faulty in (False, True):
        module = dict(__file__=str(SOURCE), __name__='cleanup_process_boundary')
        exec(compile(text.replace(anchor, "HANDOFF = ROOT / 'scripts/check_no_background_jobs.sh'", 1) if faulty else text,
                     str(SOURCE), 'exec'), module)
        frozen = json.loads(PLAN_FILE.read_text())
        namespace = module['apply'].__globals__
        namespace['census'] = lambda: frozen
        namespace['tracked'] = lambda: set()
        namespace['inspect'] = lambda name, tracked: frozen['candidates'][0]
        run, unlink = subprocess.run, Path.unlink
        calls = []
        state = dict(os.environ)
        def child(command, *args, **kwargs):
            calls.append(command)
            return run(command, *args, **dict(kwargs, env=environment, capture_output=True, text=True))
        def deletion(path, *args, **kwargs):
            assert path != VICTIM, 'actual cleanup stale dispatch admitted deletion'
            return unlink(path, *args, **kwargs)
        try:
            subprocess.run, Path.unlink = child, deletion
            try:
                module['apply'](AUDIT)
            except subprocess.CalledProcessError as error:
                assert not faulty and error.returncode == 2 and 'lsof unavailable (exit 1)' in error.stderr, error
            except AssertionError as error:
                assert faulty and str(error) == 'actual cleanup stale dispatch admitted deletion', error
            else:
                raise AssertionError('cleanup process control produced no expected verdict')
            assert calls == [['bash', str(LEGACY if faulty else CURRENT)]], calls
            assert dict(os.environ) == state, 'cleanup apply changed prepared caller state'
        finally:
            subprocess.run, Path.unlink = run, unlink
        assert VICTIM.read_bytes() == saved and SOURCE.read_bytes() == ORIGINAL
        assert not (AUDIT / 'result.json').exists()
    print('cleanup process repair: actual strict apply refused2 before deletion; actual stale-dispatch assertion red; victim/source/caller state exact; rc=0')


CORE = (SOURCE, PROFILE_SOURCE, CURRENT, CENSUS)


def mirror(label):
    """Copy only public cleanup/process entries; Git remains a controlled public CLI fixture."""
    sandbox = WORK / label
    assert not sandbox.exists(), 'cleanup mirror already exists'
    mapping = {path: sandbox / path.relative_to(ROOT) for path in CORE}
    readme = sandbox / 'README.md'
    victim = sandbox / 'target/victim.log'
    directories = {sandbox, victim.parent}
    for path in mapping.values():
        directories.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
    outputs = (*mapping.values(), readme, victim)
    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=tuple(directories), output_files=outputs)
    for path, destination in mapping.items():
        PROFILE['plan_producer'](ROOT, sources=CORE, directories=tuple(directories), output_files=outputs)
        shutil.copy2(path, destination)
    PROFILE['plan_producer'](ROOT, directories=tuple(directories), output_files=outputs)
    readme.write_text('controlled tracked input\n')
    PROFILE['plan_producer'](ROOT, directories=tuple(directories), output_files=outputs)
    victim.write_bytes(b'controlled disposable output\n')
    return sandbox, mapping, readme, victim


@contextlib.contextmanager
def isolated(sandbox):
    saved, argv = dict(os.environ), sys.argv
    try:
        os.environ.update({name: str(sandbox / suffix) for name, suffix in PROFILE['STORES'].items()})
        yield
    finally:
        os.environ.clear(); os.environ.update(saved); sys.argv = argv


def capture(label, *, text=None, mutation=None):
    """Execute actual plan/apply bodies with all writes/deletions observed and native children intercepted."""
    sandbox, mapping, readme, victim = mirror(label)
    entry = mapping[SOURCE]
    audit = sandbox / 'target/artifact_cleanup_audit/run'
    tracked = tuple(path.relative_to(sandbox).as_posix() for path in (*mapping.values(), readme))
    frozen = {path: path.read_bytes() for path in (*mapping.values(), readme)}
    real_load, real_git, real_run = runpy.run_path, subprocess.check_output, subprocess.run
    real_mkdir, real_open, real_unlink, real_rmtree = os.mkdir, Path.open, Path.unlink, shutil.rmtree
    plans, effects, children, audit_plans = [], [], [], []
    phase = ['plan']
    namespace = {}
    def owner():
        frame = sys._getframe(1)
        while frame and Path(frame.f_globals.get('__file__', '')) != entry:
            frame = frame.f_back
        return frame.f_globals if frame else None
    def complete_output(path, *, deletion=False):
        selected = audit / ('plan.json' if phase[0] == 'plan' else 'result.json')
        relevant = [kwargs for kwargs in plans if selected in kwargs.get('output_files', ())]
        assert relevant, 'cleanup effect before audit authority'
        if deletion or path not in (audit / 'plan.json', audit / 'result.json'):
            expected = {*mapping.values(), readme, *((audit / 'plan.json',) if phase[0] == 'apply' else ())}
            assert audit_plans and set(audit_plans[-1]['sources']) == expected, 'cleanup effect omitted complete input authority'
        elif path in (audit / 'plan.json', audit / 'result.json'):
            assert path in plans[-1].get('output_files', ()), 'cleanup audit write recheck omitted'
    def load(file, *args, **kwargs):
        value = real_load(file, *args, **kwargs)
        if Path(file) == mapping[PROFILE_SOURCE]:
            real_plan = value['plan_producer']
            def plan(root, **kwargs):
                answer = real_plan(root, **kwargs)
                assert root == sandbox and not kwargs.get('mutable_sources'), 'cleanup unexpected root/mutable authority'
                plans.append(kwargs)
                frame = sys._getframe(1).f_back
                if frame and frame.f_code.co_name == 'audit_plan':
                    audit_plans.append(kwargs)
                return answer
            value['plan_producer'] = plan
        return value
    def git(command, *args, **kwargs):
        assert command[:3] == ['git', '-C', str(sandbox)], ('cleanup unexpected Git command', command)
        assert kwargs.get('env') and kwargs['env']['TMPDIR'] == str(sandbox / 'target/scratch'), 'cleanup Git environment omitted'
        assert plans and set(plans[-1]['sources']) == set(mapping.values()), 'cleanup Git core authority omitted'
        if command[3:] == ['ls-files', '-z']:
            return ('\0'.join(tracked) + '\0').encode()
        assert command[3:] == ['rev-parse', 'HEAD'], command
        return b'controlled-cleanup-head\n'
    def child(command, *args, **kwargs):
        assert kwargs.get('env') and kwargs['env']['TMPDIR'] == str(sandbox / 'target/scratch'), 'cleanup child environment omitted'
        if command[:3] == ['git', '-C', str(sandbox)]:
            assert command[3:6] == ['check-ignore', '-q', '--'] and len(command) == 7, command
            return subprocess.CompletedProcess(command, 0 if command[-1].startswith('target/') else 1)
        assert command == ['bash', str(mapping[CURRENT])], 'cleanup dispatched stale process checker'
        complete_output(audit / 'result.json', deletion=True)
        children.append(command)
        return subprocess.CompletedProcess(command, 0)
    def mkdir(path, *args, **kwargs):
        if owner():
            complete_output(Path(path))
            effects.append(('mkdir', Path(path)))
        return real_mkdir(path, *args, **kwargs)
    def opened(path, mode='r', *args, **kwargs):
        if owner() and any(char in mode for char in 'wax+'):
            complete_output(path)
            assert mode == 'x' and path in (audit / 'plan.json', audit / 'result.json'), 'cleanup changed exclusive audit mode'
            effects.append(('write', path))
        return real_open(path, mode, *args, **kwargs)
    def deletion(path, *args, **kwargs):
        if owner():
            complete_output(path, deletion=True)
            assert children and path == victim, 'cleanup unowned file deletion'
            effects.append(('unlink', path))
        return real_unlink(path, *args, **kwargs)
    def tree(path, *args, **kwargs):
        if owner():
            complete_output(path, deletion=True)
            assert children and path == sandbox / 'target/scratch', 'cleanup unowned tree deletion'
            effects.append(('tree', path))
        return real_rmtree(path, *args, **kwargs)
    tree.avoids_symlink_attacks = real_rmtree.avoids_symlink_attacks
    try:
        with isolated(sandbox), contextlib.redirect_stdout(io.StringIO()):
            runpy.run_path, subprocess.check_output, subprocess.run = load, git, child
            os.mkdir, Path.open, Path.unlink, shutil.rmtree = mkdir, opened, deletion, tree
            namespace = dict(__file__=str(entry), __name__='borrowed_cleanup')
            source = text or ORIGINAL.decode()
            if mutation:
                before, after = mutation
                assert source.count(before) == 1, ('cleanup guard anchor changed', before)
                source = source.replace(before, after, 1)
            exec(compile(source, str(entry), 'exec'), namespace)
            sys.argv = [str(entry), 'plan', audit.relative_to(sandbox).as_posix()]
            namespace['main']()
            plan = json.loads((audit / 'plan.json').read_text())
            assert {row['path'] for row in plan['candidates']} == {'target/scratch', 'target/victim.log'}
            phase[0] = 'apply'
            sys.argv = [str(entry), 'apply', audit.relative_to(sandbox).as_posix()]
            namespace['main']()
            result = json.loads((audit / 'result.json').read_text())
            assert (result['removed_trees'], result['removed_strays'], result['removed_files'], result['removed_bytes']) == (1, 1, 1, 29)
            assert result['residue'] == 0 and result['tracked_change'] is False and result['skipped'] == []
            assert not victim.exists() and not (sandbox / 'target/scratch').exists(), 'cleanup selected residue changed'
            assert all(path.read_bytes() == data for path, data in frozen.items()), 'cleanup capture changed tracked fixture bytes'
            assert children == [['bash', str(mapping[CURRENT])]], 'cleanup process dispatch population changed'
            assert sum(kind == 'write' for kind, _ in effects) == 2, 'cleanup audit population changed'
        return len(plans), len(effects), len(children)
    finally:
        runpy.run_path, subprocess.check_output, subprocess.run = real_load, real_git, real_run
        os.mkdir, Path.open, Path.unlink, shutil.rmtree = real_mkdir, real_open, real_unlink, real_rmtree
        assert SOURCE.read_bytes() == ORIGINAL, 'cleanup capture changed actual producer'


def real_plan():
    """Actual read-only project census and exclusive audit production; never apply it."""
    run = ROOT / 'target/artifact_cleanup_audit' / ('entry-plan-' + str(os.getpid()))
    inputs = tuple(ROOT / name for name in subprocess.check_output(['git', 'ls-files'], cwd=ROOT, text=True).splitlines())
    PROFILE['plan_producer'](ROOT, sources=inputs)
    frozen = {path: path.read_bytes() for path in inputs}
    plan = dict(sources=(*CORE, Path(__file__)), directories=(run,), output_files=(run / 'plan.json',))
    PROFILE['plan_producer'](ROOT, **plan)
    result = subprocess.run([sys.executable, '-I', '-B', str(SOURCE), 'plan', run.relative_to(ROOT).as_posix()],
                            cwd=ROOT, env=ENVIRONMENT, capture_output=True, text=True)
    assert result.returncode == 0, (result.stdout, result.stderr)
    actual = json.loads((run / 'plan.json').read_text())
    assert actual['producer_sha256'] == __import__('hashlib').sha256(ORIGINAL).hexdigest()
    assert actual['head'] == subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    assert all(path.read_bytes() == data for path, data in frozen.items()), 'cleanup real plan changed tracked bytes'
    assert not (run / 'result.json').exists(), 'cleanup real plan applied a deletion'
    print('cleanup real plan:', len(inputs), 'tracked inputs exact;', result.stdout.strip(), 'rc=0; no real deletion')


def physical_controls():
    """Actual mode bodies refuse unsafe late audit files before creation and protected-source reads."""
    cases = ('helper-missing', 'helper-symlink', 'helper-directory', 'tracked-missing',
             'output-symlink', 'output-hardlink', 'output-directory', 'run-symlink', 'run-repository',
             'input-missing', 'input-symlink', 'input-directory', 'store-symlink', 'toolchain')
    refused = 0
    for index, case in enumerate(cases):
        sandbox, mapping, readme, victim = mirror('physical-' + str(index))
        audit = sandbox / 'target/artifact_cleanup_audit/run'
        mode = 'apply' if case.startswith('input-') else 'plan'
        output = audit / ('plan.json' if mode == 'plan' else 'result.json')
        fixture_paths = (output, audit / 'plan.json', audit.parent / 'link-target', sandbox / 'target/scratch')
        fixture_directories = (audit, sandbox / 'target')
        PROFILE['prepare_producer'](ROOT, directories=fixture_directories, output_files=fixture_paths)
        wanted = 'producer source is not a local regular file'
        if case.startswith('helper-'):
            selected = mapping[CENSUS]; selected.unlink()
            if case.endswith('symlink'):
                selected.symlink_to(readme); wanted = 'symlink producer source'
            elif case.endswith('directory'):
                selected.mkdir()
        elif case == 'tracked-missing':
            readme.unlink()
        elif case.startswith('output-'):
            if case.endswith('symlink'):
                output.symlink_to(readme); wanted = 'symlink producer output'
            elif case.endswith('hardlink'):
                os.link(readme, output); wanted = 'hard-linked producer output'
            else:
                output.mkdir(); wanted = 'producer output is not a local regular file'
        elif case == 'run-symlink':
            audit.rmdir(); audit.symlink_to(sandbox / 'target', target_is_directory=True); wanted = 'symlink ancestor:'
        elif case == 'run-repository':
            (audit / '.git').mkdir(); wanted = 'nested repository:'
        elif case.startswith('input-'):
            if case.endswith('symlink'):
                (audit / 'plan.json').symlink_to(readme); wanted = 'symlink producer source'
            elif case.endswith('directory'):
                (audit / 'plan.json').mkdir()
        elif case == 'store-symlink':
            (sandbox / 'target/scratch').symlink_to(sandbox / 'target', target_is_directory=True); wanted = 'symlink store component'
        else:
            wanted = 'toolchain override must be a name'
        real_git, real_run, real_mkdir, real_open = subprocess.check_output, subprocess.run, os.mkdir, Path.open
        tracked = tuple(path.relative_to(sandbox).as_posix() for path in (*mapping.values(), readme))
        def git(command, *args, **kwargs):
            assert command == ['git', '-C', str(sandbox), 'ls-files', '-z'], ('cleanup refusal unexpectedly delegated', command)
            assert kwargs.get('env') and kwargs['env']['TMPDIR'] == str(sandbox / 'target/scratch')
            return ('\0'.join(tracked) + '\0').encode()
        def blocked(*args, **kwargs):
            raise AssertionError('cleanup physical refusal produced an effect/child')
        def opened(path, open_mode='r', *args, **kwargs):
            assert not any(char in open_mode for char in 'wax+'), 'cleanup physical refusal wrote a file'
            return real_open(path, open_mode, *args, **kwargs)
        stderr = io.StringIO()
        try:
            with isolated(sandbox), contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(stderr):
                if case == 'toolchain':
                    os.environ['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'
                sys.argv = [str(mapping[SOURCE]), mode, audit.relative_to(sandbox).as_posix()]
                state, argv = dict(os.environ), sys.argv
                subprocess.check_output, subprocess.run, os.mkdir, Path.open = git, blocked, blocked, opened
                try:
                    runpy.run_path(str(mapping[SOURCE]), run_name='__main__')
                except SystemExit as error:
                    assert error.code == 2 and stderr.getvalue().startswith('cleanup refused: ') and wanted in stderr.getvalue(), (case, stderr.getvalue())
                else:
                    raise AssertionError(('cleanup physical unsafe mode accepted', case))
                assert dict(os.environ) == state and sys.argv is argv, 'cleanup refusal changed caller state'
                refused += 1
        finally:
            subprocess.check_output, subprocess.run, os.mkdir, Path.open = real_git, real_run, real_mkdir, real_open
        assert victim.read_bytes() == b'controlled disposable output\n'
    assert SOURCE.read_bytes() == ORIGINAL
    print('cleanup physical:', refused, 'actual CLI refusals; no creation/write/deletion/process child; caller/victim/source exact; rc=0')


def omission_controls():
    faults = (
        ("        PROFILE['plan_producer'](ROOT, **plan)\n", '', 'cleanup Git core authority omitted'),
        ('sources=inputs, directories=(directory,)', 'sources=plan_input, directories=(directory,)', 'cleanup effect omitted complete input authority'),
        ('    source_plan(directories=(path.parent,), output_files=(path,))\n', '', 'cleanup audit write recheck omitted'),
        ('environment = source_plan()\n    return subprocess.check_output', 'environment = {}\n    return subprocess.check_output', 'cleanup Git environment omitted'),
        ("subprocess.run(['bash', str(HANDOFF)],", "subprocess.run(['bash', str(ROOT / 'scripts/handoff_census.py')],", 'cleanup dispatched stale process checker'),
    )
    for index, (before, after, wanted) in enumerate(faults):
        try:
            capture('omission-' + str(index), mutation=(before, after))
        except AssertionError as error:
            assert str(error) == wanted, ('cleanup unrelated omission red', before, error, wanted)
        else:
            raise AssertionError(('cleanup omission escaped', before))
    print('cleanup omissions:', len(faults), 'actual complete-plan/save/environment/process dispatch reds; rc=0')


def original_body_controls():
    """Project only the added planning/environment/strict-dispatch adapters; preserve original policies."""
    class Project(ast.NodeTransformer):
        function = None
        def visit_FunctionDef(self, node):
            self.function = node.name
            return self.generic_visit(node)
        def visit_Assign(self, node):
            if (len(node.targets) == 1 and isinstance(node.targets[0], ast.Name)
                    and node.targets[0].id == 'environment' and isinstance(node.value, ast.Call)
                    and isinstance(node.value.func, ast.Name) and node.value.func.id in ('source_plan', 'audit_plan')):
                return None
            return self.generic_visit(node)
        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name):
                if (node.value.func.id == 'require' and node.value.args
                        and isinstance(node.value.args[0], ast.Compare)
                        and isinstance(node.value.args[0].left, ast.Attribute)
                        and node.value.args[0].left.attr == 'st_nlink'):
                    return None
                if node.value.func.id == 'source_plan':
                    return None
                if node.value.func.id == 'audit_plan':
                    return (ast.parse('directory.mkdir(parents=True, exist_ok=True)').body[0]
                            if self.function == 'main' else None)
            return self.generic_visit(node)
        def visit_Call(self, node):
            node = self.generic_visit(node)
            node.keywords = [keyword for keyword in node.keywords if keyword.arg != 'env']
            if (isinstance(node.func, ast.Attribute) and isinstance(node.func.value, ast.Name)
                    and node.func.value.id == 'subprocess' and node.func.attr == 'run'
                    and node.args and isinstance(node.args[0], ast.List)
                    and isinstance(node.args[0].elts[0], ast.Constant) and node.args[0].elts[0].value == 'bash'):
                node.args[0] = ast.parse("['bash', str(ROOT / 'scripts/check_no_background_jobs.sh')]", mode='eval').body
            return node
    old, new = ast.parse(OLD_SOURCE), ast.parse(ORIGINAL)
    functions = lambda tree: {node.name: node for node in tree.body if isinstance(node, ast.FunctionDef)}
    old_functions, new_functions = functions(old), functions(new)
    for name, original in old_functions.items():
        projected = Project().visit(new_functions[name])
        assert ast.dump(projected) == ast.dump(original), ('cleanup original complete policy body changed', name)
    constants = lambda tree: {node.targets[0].id: ast.dump(node.value) for node in tree.body
                              if isinstance(node, ast.Assign) and isinstance(node.targets[0], ast.Name)}
    for name, value in constants(old).items():
        if name == 'PROTECTED':
            baseline = next(node.value for node in old.body if isinstance(node, ast.Assign)
                            and isinstance(node.targets[0], ast.Name) and node.targets[0].id == name)
            expected = ast.Tuple(elts=[*baseline.elts,
                ast.Constant(value='target/trig_forward_certificate'),
                ast.Constant(value='target/inverse_threshold_certificate')], ctx=ast.Load())
            assert constants(new)[name] == ast.dump(expected), 'cleanup protected-store delta differs from the two admitted certificates'
        else:
            assert constants(new)[name] == value, ('cleanup original policy constant changed', name)
    print('cleanup original bodies/constants exact through declared adapters, plus exactly2 certificate stores; rc=0')
    certificate = ROOT / 'docs/tasks/artifacts/artifact_cleanup/certificate_store_controls.py'
    PROFILE['plan_producer'](ROOT, sources=(certificate, SOURCE, PROFILE_SOURCE, CURRENT, CENSUS))
    subprocess.run(['python3', '-I', '-B', str(certificate)], cwd=ROOT, env=ENVIRONMENT, check=True)


def hardlink_controls():
    """Actual legacy admission, repaired refusal and unique-link guard omission; no deletion."""
    victim = WORK / 'linked.log'
    alias = WORK / 'linked.dat'
    PROFILE['plan_producer'](ROOT, output_files=(victim, alias))
    alias.write_bytes(b'controlled shared artifact\n')
    PROFILE['plan_producer'](ROOT, output_files=(victim, alias)); os.link(alias, victim)
    anchor = "            require(entry.stat().st_nlink == 1, 'hard-linked entry: ' + rel)\n"
    assert ORIGINAL.decode().count(anchor) == 1
    for label, source in (('original', OLD_SOURCE), ('intact', ORIGINAL),
                          ('omission', ORIGINAL.decode().replace(anchor, '', 1))):
        module = dict(__file__=str(SOURCE), __name__='cleanup_link_guard')
        exec(compile(source, str(SOURCE), 'exec'), module)
        def must_refuse():
            try:
                row = module['inspect'](victim.relative_to(ROOT).as_posix(), set())
            except module['Refusal'] as error:
                assert 'hard-linked entry:' in str(error), error
            else:
                assert row['files'] == 1
                raise AssertionError('cleanup nonunique candidate admitted')
        if label == 'intact':
            must_refuse()
            relative = alias.relative_to(ROOT).as_posix()
            import hashlib
            wanted = hashlib.sha256(relative.encode() + b'\0' + alias.read_bytes()).hexdigest()
            assert module['tracked_digest']({relative}) == wanted, 'cleanup read-only linked input changed'
        else:
            try:
                must_refuse()
            except AssertionError as error:
                assert str(error) == 'cleanup nonunique candidate admitted', (label, error)
            else:
                raise AssertionError(('cleanup linked original/fault unexpectedly refused', label))
        assert victim.stat().st_nlink == alias.stat().st_nlink == 2
        assert victim.read_bytes() == alias.read_bytes() == b'controlled shared artifact\n'
    assert SOURCE.read_bytes() == ORIGINAL
    print('cleanup links: actual original admission/unique-link refusal/actual guard omission assertion red/read-only alias positive; both controlled aliases intact; rc=0')


def process_status_controls():
    """Actual canonical checker blocked/clear statuses, controlled OS evidence and owned artifact only."""
    for blocked in (True, False):
        audit = ROOT / 'target/artifact_cleanup_audit' / ('status-' + str(os.getpid()) + ('-blocked' if blocked else '-clear'))
        files = (audit / 'plan.json', audit / 'result.json', PS, LSOF)
        PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(audit,), output_files=files)
        ps_text = '#!/bin/bash\nprintf "%s 1 00:01 census\\n" "$PPID"\n'
        if blocked:
            ps_text += 'printf "777 1 00:01 worker %s\\n" "$CONTROLLED_ROOT"\n'
        PROFILE['plan_producer'](ROOT, output_files=files); PS.write_text(ps_text)
        PROFILE['plan_producer'](ROOT, output_files=files)
        LSOF.write_text('#!/bin/bash\nprintf "p%s\\nf0\\ntPIPE\\nn\\n" "$PPID"\n')
        environment = dict(ENVIRONMENT, PATH=str(PS.parent) + os.pathsep + ENVIRONMENT['PATH'], CONTROLLED_ROOT=str(ROOT))
        module = runpy.run_path(str(SOURCE))
        row = module['inspect'](VICTIM.relative_to(ROOT).as_posix(), set())
        frozen = {'candidates': [row], 'tracked_sha256': module['tracked_digest'](set()), 'skipped': []}
        PROFILE['plan_producer'](ROOT, output_files=files); (audit / 'plan.json').write_text(json.dumps(frozen))
        namespace = module['apply'].__globals__
        namespace['census'] = lambda: frozen
        namespace['tracked'] = lambda: set()
        namespace['inspect'] = lambda name, tracked: row
        run = subprocess.run
        def child(command, *args, **kwargs):
            assert command == ['bash', str(CURRENT)]
            return run(command, *args, **dict(kwargs, env=environment, capture_output=True, text=True))
        saved = VICTIM.read_bytes()
        try:
            subprocess.run = child
            try:
                result = module['apply'](audit)
            except subprocess.CalledProcessError as error:
                assert blocked and error.returncode == 1 and 'handoff: BLOCKED' in error.stdout, error
                assert VICTIM.read_bytes() == saved and not (audit / 'result.json').exists()
            else:
                assert not blocked and result['removed_strays'] == 1 and result['removed_bytes'] == len(saved)
                assert not VICTIM.exists() and result['residue'] == 0 and result['tracked_change'] is False
        finally:
            subprocess.run = run
        assert SOURCE.read_bytes() == ORIGINAL
    print('cleanup process statuses: actual canonical blocked1 preserves fixture; clear0 removes only owned controlled artifact/result; producer exact; rc=0')


def continuity_controls():
    """Reconstruct retained whole intervals from Git and verify prior immutable inputs."""
    reader = ROOT / 'scripts/history_archive.py'
    records = tuple(ROOT / ('docs/history/' + name) for name in (
        'stitchcad-devnotes-part291.md', 'stitchcad-changelog-part202.md', 'stitchcad-defects-part132.md', 'stitchcad-devnotes-part292.md'))
    for record in records:
        data = record.read_bytes()
        header, content = data.split(b'\n---\n\n', 1)
        text = header.decode()
        match = re.search(r'complete intervals from ([0-9a-f]{40})\.', text)
        revision = match.group(1) if match else None
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', text)
        assert (content.count(b'\n'), len(content), hashlib.sha256(content).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3]), 'cleanup retained identity drift'
        chunks = []
        for path, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', text):
            source = subprocess.check_output(['git', 'show', revision + ':' + path], cwd=ROOT)
            chunk = source[int(start):int(stop)]
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest, 'cleanup retained source interval drift'
            chunks.append(chunk)
        if revision:
            assert chunks and b''.join(chunks).rstrip(b'\n') + b'\n' == content, 'cleanup whole retention incomplete'
        else:
            assert record.name == 'stitchcad-defects-part132.md' and content.count(b'- **D210**') == content.count(b'- **D211**') == 1
        result = subprocess.run([sys.executable, '-I', '-B', str(reader), '--root', str(ROOT), 'read', record.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
        assert result.returncode == 0 and result.stdout == data, 'cleanup public retained read differs'
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT), 'cleanup prior archive changed'
    for path in set(CORE) - {SOURCE}:
        assert path.read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT), 'cleanup original reference input changed'
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    current = (ROOT / tree).read_text()
    headers = lambda value: re.findall(r'^- ID: `([^`]+)`$', value, re.M)
    assert [identity for identity in headers(current) if identity not in ('G1-SLICE.5b.4c.h2.b.p.o.a22.1', 'G1-SLICE.5b.4c.h2.b.p.o.a22.2')] == headers(old), 'cleanup canonical task graph changed'
    print('cleanup continuity: four complete retained records/public reads; prior archive/reference bytes and task graph exact')



if __name__ == '__main__':
    if '--continuity' in sys.argv:
        continuity_controls()
    elif '--real' in sys.argv:
        real_plan()
    else:
        missing_evidence_controls()
    if not any(option in sys.argv for option in ('--before', '--real', '--continuity')):
        process_repair_controls()
        print('cleanup complete mode capture:', capture('complete'))
        physical_controls()
        omission_controls()
        original_body_controls()
        hardlink_controls()
        process_status_controls()
