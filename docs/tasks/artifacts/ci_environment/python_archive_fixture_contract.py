"""Actual archive-refusal fixture plans, bounded effects and protected teardown controls."""
import ast
import contextlib
import copy
import hashlib
import io
import json
import os
from pathlib import Path
import runpy
import shutil
import stat
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'docs/tasks/artifacts/history_archive/history_archive_probes.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
READER = ROOT / 'scripts/history_archive.py'
GENERATIONS = ROOT / 'scripts/history_generations.py'
SHELL = ROOT / 'scripts/history_archive.sh'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
LEGACY = ROOT / 'docs/tasks/artifacts/history_archive/legacy_refusals.py'
CORE = (Path(__file__), ENTRY, PROFILE_SOURCE, READER, GENERATIONS, SHELL, LEGACY)
WORK = ROOT / 'target/archive-fixture-controls' / str(os.getpid())
BEFORE = '1b8fc23c7aab412d6a4602290dfcc26faceaeeb0'


def ast_shape(node):
    return ast.dump(node, include_attributes=False)


def one_function(tree, name):
    found = [node for node in ast.walk(tree) if isinstance(node, ast.FunctionDef) and node.name == name]
    assert len(found) == 1, ('archive body function anchor', name)
    return found[0]


def original_core(source):
    """Apply only the reviewed IO, pure-extraction, setup and HEAD-evidence adapters."""
    tree = ast.parse(source)
    main = next(node for node in tree.body if isinstance(node, ast.Try))
    pack = one_function(tree, 'pack')
    split = [i for i, node in enumerate(pack.body) if isinstance(node, ast.Expr)
             and isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name)
             and node.value.func.id == 'replace_payload']
    assert len(split) == 1, 'archive original pack extraction anchor'
    boundary = split[0]
    payload = copy.deepcopy(pack)
    payload.name = 'packed_payload'
    payload.args.args.pop(0)  # The pure prefix never uses the destination root.
    result = ast.parse('return manifest, raw').body[0]
    result.value.elts[1] = copy.deepcopy(pack.body[boundary].value.args[1])
    assert not any(isinstance(n, ast.Name) and n.id == 'root'
                   for node in pack.body[:boundary] for n in ast.walk(node)), 'archive pack prefix uses destination'
    payload.body = copy.deepcopy(pack.body[:boundary]) + [result]
    pack.body = ast.parse('manifest, raw = packed_payload(change, extra)\nreplace_payload(root, raw)').body + pack.body[boundary + 1:]
    tree.body.insert(tree.body.index(pack), payload)

    resident = one_function(tree, 'resident_fixture')
    begin = [i for i, node in enumerate(resident.body) if isinstance(node, ast.Assign)
             and any(isinstance(target, ast.Name) and target.id == 'content' for target in node.targets)]
    end = [i for i, node in enumerate(resident.body) if isinstance(node, ast.Assign)
           and any(isinstance(target, ast.Subscript) and isinstance(target.value, ast.Name)
                   and target.value.id == 'manifest' and isinstance(target.slice, ast.Constant)
                   and target.slice.value == 'payload' for target in node.targets)]
    assert len(begin) == len(end) == 1 and begin[0] <= end[0], 'archive original resident extraction anchor'
    resident_payload = copy.deepcopy(resident)
    resident_payload.name = 'resident_payload'
    resident_payload.args.args.pop(0)
    assert not any(isinstance(n, ast.Name) and n.id == 'p'
                   for node in resident.body[begin[0]:end[0] + 1] for n in ast.walk(node)), 'archive resident prefix uses destination'
    resident_payload.body = copy.deepcopy(resident.body[begin[0]:end[0] + 1]) + ast.parse('return manifest, raw, member').body
    resident.body = (resident.body[:begin[0]] + ast.parse('manifest, raw, member = resident_payload(padding)').body
                     + resident.body[end[0] + 1:])
    tree.body.insert(tree.body.index(pack) + 1, resident_payload)

    setup = [node for node in tree.body if isinstance(node, ast.Expr) and isinstance(node.value, ast.Call)
             and (ast_shape(node.value.func) == ast_shape(ast.parse('shutil.rmtree', mode='eval').body)
                  or ast_shape(node.value.func) == ast_shape(ast.parse('WORK.mkdir', mode='eval').body))]
    assert len(setup) == 2, 'archive original setup anchor'
    tree.body = [node for node in tree.body if node not in setup]
    main.body[:0] = setup  # The guarded producer protects the original setup with the same finally.
    tree.body = [node for node in tree.body if not (isinstance(node, ast.Assign)
                 and any(isinstance(target, ast.Name) and target.id == 'WORK' for target in node.targets))]

    class GuardedIO(ast.NodeTransformer):
        def visit_Call(self, node):
            self.generic_visit(node)
            if isinstance(node.func, ast.Attribute):
                method = node.func.attr
                if method in {'write_text', 'write_bytes', 'unlink', 'mkdir', 'symlink_to'}:
                    name = {'write_text': 'write_file', 'write_bytes': 'write_file',
                            'unlink': 'unlink_file', 'mkdir': 'make_directory', 'symlink_to': 'make_link'}[method]
                    node.args.insert(0, node.func.value)
                    node.func = ast.Name(id=name, ctx=ast.Load())
                elif ast_shape(node.func) == ast_shape(ast.parse('shutil.rmtree', mode='eval').body):
                    node.func = ast.Name(id='remove_tree', ctx=ast.Load())
                elif ast_shape(node.func) == ast_shape(ast.parse('subprocess.run', mode='eval').body):
                    assert not any(keyword.arg == 'env' for keyword in node.keywords)
                    node.keywords.append(ast.keyword(arg='env', value=ast.parse('child_environment()', mode='eval').body))
            return node

    tree = GuardedIO().visit(tree)
    prior = [node for node in ast.walk(main) if isinstance(node, ast.If)
             and ast_shape(node.test) == ast_shape(ast.parse('prior.returncode == 0', mode='eval').body)]
    assert len(prior) == 1, 'archive original HEAD anchor'
    evidence = ast.parse("""absence = subprocess.run(['git', 'ls-tree', '--name-only', 'HEAD', '--', history.MASTER],
                         cwd=ROOT, capture_output=True, check=True, env=child_environment())
PROFILE['require'](absence.stdout == b'', 'archive fixture HEAD metadata unavailable')
""").body
    prior[0].orelse[:0] = evidence
    return tree


def current_core(source):
    """Remove declared guard scaffolding; keep every original semantic body and argument."""
    tree = ast.parse(source)
    bindings = {'READER': "ROOT / 'scripts/history_archive.py'", 'SHELL': "ROOT / 'scripts/history_archive.sh'",
                'PROFILE_SOURCE': "ROOT / 'scripts/local_environment.py'",
                'WORK': "ROOT / 'target/scratch/history_archive_probes'"}
    for name, value in bindings.items():
        found = [node for node in tree.body if isinstance(node, ast.Assign)
                 and any(isinstance(target, ast.Name) and target.id == name for target in node.targets)]
        assert len(found) == 1 and ast_shape(found[0].value) == ast_shape(ast.parse(value, mode='eval').body), \
            'archive body binding changed: ' + name
    helpers = {'backing_inputs', 'fixture_population', 'work_plan', 'child_environment',
               'remove_tree', 'make_directory', 'write_file', 'unlink_file', 'make_link'}
    added = {*bindings, 'PROFILE', 'BACKING', 'INPUT_DIRECTORIES', 'SOURCES', 'FROZEN',
             'DIRECTORIES', 'FILES', 'OWNED_LINK'}
    kept, preflight = [], 0
    for node in tree.body:
        if isinstance(node, ast.Import) and len(node.names) == 1 and node.names[0].name in {'os', 'runpy'}:
            continue
        if isinstance(node, ast.FunctionDef) and node.name in helpers:
            continue
        if isinstance(node, ast.Assign):
            names = {child.id for target in node.targets for child in ast.walk(target) if isinstance(child, ast.Name)}
            if names & added:
                assert names <= added, 'archive body mixed guard binding'
                continue
        if isinstance(node, ast.If) and any(isinstance(child, ast.Raise) and isinstance(child.exc, ast.Call)
                and isinstance(child.exc.func, ast.Name) and child.exc.func.id == 'ValueError'
                and child.exc.args and isinstance(child.exc.args[0], ast.Constant)
                and child.exc.args[0].value == 'unsafe archive fixture profile' for child in ast.walk(node)):
            preflight += 1
            continue
        kept.append(node)
    assert preflight == 1, 'archive body profile preflight anchor'
    tree.body = kept

    class Scaffolding(ast.NodeTransformer):
        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name) and node.value.func.id == 'work_plan':
                return None
            return self.generic_visit(node)

        def visit_Name(self, node):
            if node.id == 'READER' and isinstance(node.ctx, ast.Load):
                return ast.parse(bindings['READER'], mode='eval').body
            return node

    return Scaffolding().visit(tree)


def check_original_body(source, before):
    assert ast_shape(current_core(source)) == ast_shape(original_core(before)), 'archive original semantic bodies changed'


def original_body_controls():
    PROFILE['plan_producer'](ROOT, sources=CORE)
    reader = runpy.run_path(str(READER))
    before = subprocess.check_output(['git', 'show', BEFORE + ':' + ENTRY.relative_to(ROOT).as_posix()],
                                     cwd=ROOT, env=reader['metadata_environment'](ROOT)).decode()
    original = ENTRY.read_bytes()
    source = original.decode()
    check_original_body(source, before)
    check_original_body(ast.unparse(ast.parse(source)), before)
    print('archive original bodies: complete original predicates/packer/resident/CLI and signatures match frozen source through declared adapters; reformat positive; rc=0')

    reds = 0

    def fault(label, select, change, expected='archive original semantic bodies changed'):
        nonlocal reds
        tree = ast.parse(source)
        matches = select(tree)
        assert len(matches) == 1, ('archive actual body fault anchor', label)
        change(matches[0])
        try:
            check_original_body(ast.unparse(ast.fix_missing_locations(tree)), before)
        except AssertionError as error:
            assert str(error) == expected, (label, error)
            reds += 1
        else:
            raise AssertionError(('archive body fault escaped', label))

    fault('packer format', lambda t: [n for n in ast.walk(one_function(t, 'packed_payload'))
          if isinstance(n, ast.keyword) and n.arg == 'format'],
          lambda n: setattr(n, 'value', ast.parse('tarfile.GNU_FORMAT', mode='eval').body))
    fault('resident compression', lambda t: [n for n in ast.walk(one_function(t, 'resident_payload'))
          if isinstance(n, ast.keyword) and n.arg == 'mtime'], lambda n: setattr(n, 'value', ast.Constant(value=1)))
    fault('refusal discrimination', lambda t: [n for n in ast.walk(one_function(t, 'arm'))
          if isinstance(n, ast.Assert) and isinstance(n.test, ast.BoolOp)],
          lambda n: setattr(n, 'test', ast.Constant(value=True)))
    fault('CLI predicate', lambda t: [n for n in ast.walk(t) if isinstance(n, ast.Assert)
          and any(isinstance(a, ast.Attribute) and a.attr == 'returncode' and isinstance(a.value, ast.Name)
                  and a.value.id == 'result' for a in ast.walk(n.test))],
          lambda n: setattr(n, 'test', ast.Constant(value=True)))
    fault('resident population', lambda t: [n for n in ast.walk(one_function(t, 'resident_fixture'))
          if isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id == 'range'],
          lambda n: setattr(n.args[0], 'value', 21))
    fault('public pack signature', lambda t: [one_function(t, 'pack')],
          lambda n: setattr(n.args.args[1], 'arg', 'ignored_change'))
    fault('Boolean integer fixture', lambda t: [n for n in ast.walk(one_function(t, 'bool_integer'))
          if isinstance(n, ast.Constant) and n.value is True], lambda n: setattr(n, 'value', 1))
    fault('expected refusal', lambda t: [n for n in ast.walk(t) if isinstance(n, ast.Constant)
          and n.value == 'payload digest mismatch'], lambda n: setattr(n, 'value', 'unrelated refusal'))

    fault('reader adapter binding', lambda t: [n for n in t.body if isinstance(n, ast.Assign)
          and any(isinstance(a, ast.Name) and a.id == 'READER' for a in n.targets)],
          lambda n: setattr(n, 'value', ast.parse("ROOT / 'scripts/wrong_reader.py'", mode='eval').body),
          'archive body binding changed: READER')

    tree = ast.parse(source)
    main = next(node for node in tree.body if isinstance(node, ast.Try))
    removed = [node for node in main.body if isinstance(node, ast.Expr) and isinstance(node.value, ast.Call)
               and isinstance(node.value.func, ast.Name) and node.value.func.id == 'arm'
               and isinstance(node.value.args[0], ast.Constant) and node.value.args[0].value == 'RED catalog membership']
    assert len(removed) == 1, 'archive actual removed-case anchor'
    main.body.remove(removed[0])
    try:
        check_original_body(ast.unparse(ast.fix_missing_locations(tree)), before)
    except AssertionError as error:
        assert str(error) == 'archive original semantic bodies changed', error
        reds += 1
    else:
        raise AssertionError('archive missing original case escaped')
    assert ENTRY.read_bytes() == original, 'archive body controls changed producer'
    print('archive original body faults:%d actual AST predicate/interface/population/algorithm/adapter reds; source unchanged; no fixture/build execution; rc=0' % reds)


def before_control():
    source = subprocess.check_output(['git', 'show', BEFORE + ':' + ENTRY.relative_to(ROOT).as_posix()], cwd=ROOT, text=True)
    original, saved = shutil.rmtree, dict(os.environ)
    seen = []

    class Trapped(Exception):
        pass

    def trap(path, **kwargs):
        assert path == ROOT / 'target/scratch/history_archive_probes' and kwargs == dict(ignore_errors=True)
        seen.append(path)
        raise Trapped

    try:
        for name in PROFILE['STORES']:
            os.environ.pop(name, None)
        shutil.rmtree = trap
        try:
            exec(compile(source, str(ENTRY), 'exec'), dict(__file__=str(ENTRY), __name__='before_archive_fixture'))
        except Trapped:
            pass
        else:
            raise AssertionError('old archive fixture removal escaped')
        assert len(seen) == 1
    finally:
        shutil.rmtree = original
        os.environ.clear(); os.environ.update(saved)
    print('original actual archive fixture removal trapped with stores absent; no native effect')


def bootstrap(source=None, entry=ENTRY):
    source = ENTRY.read_text() if source is None else source
    tree = ast.parse(source)
    index = next(index for index, node in enumerate(tree.body) if isinstance(node, ast.Try))
    previous = tree.body[index - 1]
    if isinstance(previous, ast.Expr) and isinstance(previous.value, ast.Call) and isinstance(previous.value.func, ast.Name) and previous.value.func.id == 'work_plan':
        index -= 1
    ns = dict(__file__=str(entry), __name__='archive_fixture_control')
    exec(compile(ast.Module(body=tree.body[:index], type_ignores=[]), str(entry), 'exec'), ns)
    return ns, ast.Module(body=tree.body[index:], type_ignores=[])


@contextlib.contextmanager
def mirror(label):
    root = WORK / label
    inputs, folders = list(CORE), [ROOT / '.doctrine/history_archive', ROOT / 'docs/history']
    for folder in folders:
        PROFILE['directory'](ROOT, folder.relative_to(ROOT).as_posix())
        for path in sorted(folder.iterdir()):
            assert not path.is_symlink()
            (folders if path.is_dir() else inputs).append(path)
    inputs = tuple(dict.fromkeys(inputs))
    PROFILE['plan_producer'](ROOT, sources=inputs)
    reader = runpy.run_path(str(READER))
    contents = runpy.run_path(str(LEGACY))['baseline'](reader, ROOT)
    contents.update({path.relative_to(ROOT).as_posix(): path.read_bytes() for path in CORE})
    files = tuple(root / name for name in contents)
    directories = {WORK, root}
    for path in files:
        directories.update(parent for parent in path.parents if parent == root or parent.is_relative_to(root))
    plan = dict(sources=inputs, directories=tuple(sorted(directories)), output_files=files)
    assert not root.exists() and not root.is_symlink()
    PROFILE['prepare_producer'](ROOT, **plan)
    try:
        for name, output in zip(contents, files):
            PROFILE['plan_producer'](ROOT, **plan)
            output.write_bytes(contents[name])
        yield root
    finally:
        files, folders = [], [root]
        for folder in folders:
            PROFILE['directory'](ROOT, folder.relative_to(ROOT).as_posix())
            for path in sorted(folder.iterdir()):
                assert not path.is_symlink(), 'archive control cleanup link'
                (folders if path.is_dir() else files).append(path)
        PROFILE['plan_producer'](ROOT, sources=CORE, directories=tuple(folders), output_files=tuple(files))
        assert shutil.rmtree.avoids_symlink_attacks
        shutil.rmtree(root)


def capture(label, source=None, *, metadata_mode=None, interrupt=False):
    with mirror(label) as root:
        entry = root / ENTRY.relative_to(ROOT)
        saved_env = dict(os.environ)
        stores = {name: str(root / 'target/prepared' / name.lower()) for name in PROFILE['STORES']}
        PROFILE['prepare_producer'](ROOT, sources=CORE, directories=tuple(Path(value) for value in stores.values()))
        os.environ.update(stores, HISTORY_ARCHIVE_ROOT=str(root), ARCHIVE_FIXTURE_CALLER='preserved',
                          GIT_DIR=str(root / 'foreign-git'))
        expected_sources = {entry, root / PROFILE_SOURCE.relative_to(ROOT), root / READER.relative_to(ROOT),
                            root / SHELL.relative_to(ROOT),
                            *(path for folder in (root / '.doctrine/history_archive', root / 'docs/history')
                              for path in folder.rglob('*') if path.is_file())}
        frozen = {path: path.read_bytes() for path in expected_sources}
        # Keep Git as the independent comparison authority. Under v2, the original v1
        # capture revision belongs to the immutable cohort containing backing members.
        reader = runpy.run_path(str(READER))
        revision = 'HEAD'
        if reader['_uses_v2'](ROOT):
            manifests = reader['flat_snapshot'](ROOT)[3].values()
            revision = next(value['source_revision'] for value in manifests
                            if any(row['kind'] == 'backing' for row in value['members']))
        names = subprocess.check_output(['git', 'ls-tree', '--full-tree', '-r', '--name-only', revision, '--',
                                         '.doctrine/history_archive', 'docs/history'], cwd=ROOT,
                                        env=reader['metadata_environment'](ROOT)).decode().splitlines()
        git = {name: reader['_git_blob'](ROOT, revision, name,
                                       160000 if name.endswith('.md') else reader['_backing_limit'](name))
               for name in names}
        population = {path.relative_to(root).as_posix() for folder in
                      (root / '.doctrine/history_archive', root / 'docs/history')
                      for path in folder.rglob('*') if path.is_file()}
        assert population == set(git), 'retained v1 source membership mismatch'
        assert all((root / name).read_bytes() == data for name, data in git.items()), 'retained v1 source mismatch'
        originals = (runpy.run_path, Path.mkdir, Path.write_bytes, Path.write_text, Path.unlink,
                     Path.symlink_to, shutil.rmtree, shutil.copytree, subprocess.run)
        namespace, latest = {}, {}
        plans, effects, children, activation = [], [], [], []
        checkpoint = 0

        def plan(project, **kwargs):
            assert project == root
            assert set(kwargs.get('sources', ())) == expected_sources, 'archive fixture complete sources omitted'
            if kwargs.get('directories') or kwargs.get('output_files'):
                assert namespace and set(kwargs.get('directories', ())) == namespace['DIRECTORIES'], 'archive fixture complete directories omitted'
                links = {namespace['OWNED_LINK']} if namespace['OWNED_LINK'].is_symlink() else set()
                assert set(kwargs.get('output_files', ())) == namespace['FILES'] - links, 'archive fixture complete files omitted'
            plans.append(kwargs)
            latest.clear(); latest.update(kwargs)
            return PROFILE['plan_producer'](project, **kwargs)

        def environment(project, values, *, create=True):
            assert project == root
            assert all(values.get(name) == value for name, value in stores.items()), 'archive fixture caller stores reset'
            assert values['ARCHIVE_FIXTURE_CALLER'] == 'preserved'
            if create:
                assert latest.get('output_files') and set(latest['output_files']) == namespace['FILES'], 'archive fixture activation before complete plan'
                activation.append(True)
            return PROFILE['environment_for'](project, values, create=create)

        observed = dict(PROFILE, plan_producer=plan, environment_for=environment)

        def load(path, *args, **kwargs):
            return observed if Path(path) == root / PROFILE_SOURCE.relative_to(ROOT) else originals[0](path, *args, **kwargs)

        def fresh(kind, path=None):
            nonlocal checkpoint
            assert activation, 'archive fixture activation omitted'
            assert len(plans) > checkpoint, 'archive fixture effect recheck omitted'
            checkpoint = len(plans)
            assert latest.get('directories') and latest.get('output_files'), 'archive fixture incomplete effect plan'
            if path is not None:
                assert path in namespace['DIRECTORIES'] or path in namespace['FILES'], 'archive fixture effect outside closed population'
            effects.append((kind, path))

        def mkdir(path, *args, **kwargs):
            if sys._getframe(1).f_globals is namespace:
                fresh('mkdir', path)
            return originals[1](path, *args, **kwargs)

        def write_bytes(path, *args, **kwargs):
            if sys._getframe(1).f_globals is namespace:
                fresh('bytes', path)
            return originals[2](path, *args, **kwargs)

        def write_text(path, *args, **kwargs):
            if sys._getframe(1).f_globals is namespace:
                fresh('text', path)
            return originals[3](path, *args, **kwargs)

        def unlink(path, *args, **kwargs):
            if sys._getframe(1).f_globals is namespace:
                fresh('unlink', path)
            return originals[4](path, *args, **kwargs)

        def symlink(path, *args, **kwargs):
            if sys._getframe(1).f_globals is namespace:
                fresh('symlink', path)
                assert path == namespace['OWNED_LINK'] and args == (root / namespace['MANIFEST']['payload'],)
            return originals[5](path, *args, **kwargs)

        def remove(path, *args, **kwargs):
            if sys._getframe(1).f_globals is namespace:
                fresh('rmtree', path)
            return originals[6](path, *args, **kwargs)
        remove.avoids_symlink_attacks = originals[6].avoids_symlink_attacks

        def copy(source_path, destination, *args, **kwargs):
            if sys._getframe(1).f_globals is namespace:
                fresh('copytree', destination)
                if interrupt:
                    raise OSError('controlled archive fixture copy failure')
            return originals[7](source_path, destination, *args, **kwargs)

        def run(command, **kwargs):
            if sys._getframe(1).f_globals is namespace:
                fresh('child')
            environment = kwargs['env']
            assert 'GIT_DIR' not in environment and environment['GIT_CONFIG_NOSYSTEM'] == '1', 'archive fixture child Git scope omitted'
            assert environment['ARCHIVE_FIXTURE_CALLER'] == 'preserved'
            assert all(environment[name] == value for name, value in stores.items()), 'archive fixture child stores reset'
            children.append(command)
            if command[0] == 'bash':
                assert command[:2] == ['bash', str(root / SHELL.relative_to(ROOT))] and kwargs['cwd'] == root
                mode, argument = command[2:]
                if mode == 'materialize' and argument == 'target/scratch/history_archive_probes/materialized' and len(children) == 1:
                    destination = root / argument
                    for name, content in namespace['SOURCE'].items():
                        output = destination / name
                        assert output in namespace['FILES']
                        output.parent.mkdir(parents=True, exist_ok=True)
                        output.write_bytes(content)
                    return subprocess.CompletedProcess(command, 0, b'', b'')
                reason = 'unknown logical identity' if mode == 'read' else 'destination must be under target/' if argument == '../escape' else 'destination exists'
                return subprocess.CompletedProcess(command, 1, b'', reason.encode())
            if command[:2] == ['git', 'show']:
                assert command == ['git', 'show', 'HEAD:' + namespace['history'].MASTER]
                return subprocess.CompletedProcess(command, 128 if metadata_mode else 0,
                                                   b'' if metadata_mode else git[namespace['history'].MASTER], b'')
            if command[:2] == ['git', 'ls-tree']:
                assert metadata_mode
                if metadata_mode == 'unavailable':
                    if kwargs.get('check'):
                        raise subprocess.CalledProcessError(128, command, b'', b'controlled unavailable tree')
                    return subprocess.CompletedProcess(command, 128, b'', b'controlled unavailable tree')
                return subprocess.CompletedProcess(command, 0, b'' if metadata_mode == 'absent' else
                                                   namespace['history'].MASTER.encode() + b'\n', b'')
            assert command[:3] == ['git', '-C', str(namespace['WORK'] / 'fixture')]
            assert command[3] == 'show' and command[4].startswith('HEAD:')
            return subprocess.CompletedProcess(command, 0, git[command[4][5:]], b'')

        try:
            runpy.run_path = load
            # Bootstrap performs only source inspection and pure population construction.
            namespace, body = bootstrap(source, entry)
            Path.mkdir, Path.write_bytes, Path.write_text = mkdir, write_bytes, write_text
            Path.unlink, Path.symlink_to = unlink, symlink
            shutil.rmtree, shutil.copytree, subprocess.run = remove, copy, run
            with contextlib.redirect_stdout(io.StringIO()) as output:
                try:
                    exec(compile(body, str(entry), 'exec'), namespace)
                except OSError as error:
                    assert interrupt and str(error) == 'controlled archive fixture copy failure', error
                else:
                    assert not interrupt, 'archive fixture interrupted copy did not run'
            assert not namespace['WORK'].exists(), 'archive fixture protected teardown omitted'
            if interrupt:
                assert all(path.read_bytes() == content for path, content in frozen.items())
                return len(plans), len(effects), len(children)
            assert metadata_mode not in ('present', 'unavailable'), 'archive fixture unavailable HEAD admitted'
            count = 27 if metadata_mode == 'absent' else 28
            assert namespace['PASS'] == count and 'archive probes: %d pass / 0 fail' % count in output.getvalue()
            assert len(activation) == 1 and sum(kind == 'symlink' for kind, _ in effects) == 1
            assert not namespace['WORK'].exists(), 'archive fixture residue'
            assert all(path.read_bytes() == content for path, content in frozen.items()), 'archive fixture input changed'
            assert all(os.environ[name] == value for name, value in stores.items())
            return len(plans), len(effects), len(children)
        finally:
            (runpy.run_path, Path.mkdir, Path.write_bytes, Path.write_text, Path.unlink,
             Path.symlink_to, shutil.rmtree, shutil.copytree, subprocess.run) = originals
            os.environ.clear(); os.environ.update(saved_env)


def metadata_controls(source=None):
    for mode in ('present', 'unavailable', 'absent'):
        try:
            capture('metadata-' + mode, source, metadata_mode=mode)
        except ValueError as error:
            assert mode == 'present' and str(error) == 'archive fixture HEAD metadata unavailable', (mode, error)
        except subprocess.CalledProcessError as error:
            assert mode == 'unavailable' and error.returncode == 128, (mode, error)
        else:
            assert mode == 'absent', 'archive fixture unavailable HEAD admitted'
    print('fixture HEAD evidence: present-unreadable/unavailable refuse; genuine first transition keeps original27 cases')


def omission_controls():
    source = ENTRY.read_text()
    faults = (
        ('sources', 'sources=(Path(__file__), PROFILE_SOURCE, READER, SHELL, *files)',
         'sources=(Path(__file__), PROFILE_SOURCE, SHELL, *files)', 'archive fixture complete sources omitted'),
        ('files', 'output_files=tuple(sorted(FILES - links))', 'output_files=()', 'archive fixture complete files omitted'),
        ('directories', 'directories=tuple(sorted(DIRECTORIES))', 'directories=()', 'archive fixture complete directories omitted'),
        ('activation', 'work_plan(activate=True, owned_link=True)\n', '', 'archive fixture activation omitted'),
        ('write', 'def write_file(path, content):\n    work_plan()\n', 'def write_file(path, content):\n', 'archive fixture effect recheck omitted'),
        ('stores', "PROFILE['environment_for'](ROOT, os.environ, create=True)",
         "PROFILE['environment_for'](ROOT, {}, create=True)", 'archive fixture caller stores reset'),
        ('Git', "if not name.startswith('GIT_')", 'if True', 'archive fixture child Git scope omitted'),
    )
    for label, before, after, reason in faults:
        assert source.count(before) == 1, (label, 'fixture actual anchor')
        try:
            capture('fault-' + label, source.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == reason, (label, str(error), reason)
        else:
            raise AssertionError('archive fixture omission survived: ' + label)
    before = 'finally:\n    remove_tree(WORK)'
    assert source.count(before) == 1
    try:
        capture('fault-teardown', source.replace(before, 'finally:\n    pass', 1), interrupt=True)
    except AssertionError as error:
        assert str(error) == 'archive fixture protected teardown omitted', str(error)
    else:
        raise AssertionError('archive fixture teardown fault survived')
    faults = (
        ("PROFILE['require'](absence.stdout == b'', 'archive fixture HEAD metadata unavailable')", 'pass'),
        ('cwd=ROOT, capture_output=True, check=True, env=child_environment()',
         'cwd=ROOT, capture_output=True, env=child_environment()'),
    )
    for index, (before, after) in enumerate(faults):
        assert source.count(before) == 1, ('fixture metadata anchor', index)
        try:
            metadata_controls(source.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == 'archive fixture unavailable HEAD admitted', str(error)
        else:
            raise AssertionError('archive fixture metadata omission survived')
    print('fixture ten compiled actual source/file/directory/activation/write/store/Git/teardown/HEAD assertion reds')


# This child executes the unmodified entry. Its independent audit hook stops effects
# before dispatch, including a FIFO read that would otherwise hang a broken guard.
ENTRY_SENTRY = r'''
import copy, gzip, hashlib, importlib.util, io, json, os, pathlib, runpy, shutil, stat, subprocess, sys, tarfile
entry, action = sys.argv[1:]
sys.argv = [entry]
class EffectAttempt(BaseException):
    pass
def audit(event, args):
    if event == 'open':
        path, mode, flags = args
        if (isinstance(mode, str) and any(c in mode for c in 'wax+')) or flags & (os.O_WRONLY | os.O_RDWR | os.O_CREAT | os.O_TRUNC | os.O_APPEND):
            raise EffectAttempt(event)
        if isinstance(path, (str, bytes)):
            try:
                fifo = stat.S_ISFIFO(os.lstat(path).st_mode)
            except FileNotFoundError:
                fifo = False
            if fifo:
                raise EffectAttempt('open FIFO')
    if event in {'os.mkdir', 'os.remove', 'os.rmdir', 'os.rename', 'os.symlink', 'os.link',
                 'os.chmod', 'os.chown', 'os.truncate', 'os.utime', 'os.putenv', 'os.unsetenv',
                 'os.system', 'os.exec', 'os.posix_spawn', 'os.fork', 'os.forkpty',
                 'subprocess.Popen', 'shutil.rmtree', 'shutil.copyfile', 'shutil.copytree'}:
        raise EffectAttempt(event)
sys.addaudithook(audit)
try:
    if action == 'write':
        with open(entry + '.sentry-output', 'wb') as stream:
            stream.write(b'forbidden')
    elif action == 'mkdir':
        os.mkdir(entry + '.sentry-directory')
    elif action == 'child':
        subprocess.run([sys.executable, '-I', '-B', '-c', 'pass'], check=True)
    elif action == 'entry':
        runpy.run_path(entry, run_name='__main__')
    else:
        raise AssertionError('unknown sentry action')
except EffectAttempt as error:
    result = dict(kind='effect', event=str(error))
except BaseException as error:
    result = dict(kind='refusal', type=type(error).__name__, message=str(error))
else:
    result = dict(kind='accepted')
print('ARCHIVE_ENTRY_RESULT=' + json.dumps(result, sort_keys=True), flush=True)
'''


def physical_snapshot(root, *, metadata=True):
    """No alias traversal or special-file opens; access time is deliberately excluded."""
    result, pending = {}, [root]
    while pending:
        path = pending.pop()
        info = path.lstat()
        row = (stat.S_IFMT(info.st_mode), stat.S_IMODE(info.st_mode), info.st_nlink)
        if metadata:
            row += (info.st_dev, info.st_ino, info.st_size, info.st_mtime_ns, info.st_ctime_ns)
        if stat.S_ISDIR(info.st_mode):
            pending.extend(sorted(path.iterdir()))
        elif stat.S_ISREG(info.st_mode):
            row += (hashlib.sha256(path.read_bytes()).hexdigest(),)
        elif stat.S_ISLNK(info.st_mode):
            row += (os.readlink(path),)
        result[path.relative_to(root).as_posix()] = row
    return result


@contextlib.contextmanager
def physical_fault(root, path, kind, content=None):
    """Create one exact owned fault; undo it lexically before guarded mirror cleanup."""
    assert path.is_relative_to(root) and path != root
    backup = path.with_name(path.name + '.physical-original')
    assert not backup.exists() and not backup.is_symlink()
    existed = path.exists()
    # The ordinary parent and replacement/backup paths are admitted before fault
    # creation. A fake .git marker is an empty controlled fault, never a repository.
    if path.name == '.git':
        PROFILE['plan_producer'](ROOT, sources=CORE, directories=(path.parent,))
    elif path.is_dir() or kind == 'directory':
        PROFILE['plan_producer'](ROOT, sources=CORE, directories=(path, backup))
    else:
        PROFILE['plan_producer'](ROOT, sources=CORE, directories=(path.parent,), output_files=(path, backup))
    if existed:
        path.rename(backup)
    try:
        if kind == 'symlink':
            path.symlink_to(os.path.relpath(backup if existed else content, path.parent))
        elif kind == 'directory':
            path.mkdir()
        elif kind == 'bytes':
            path.write_bytes(content)
        elif kind == 'hardlink':
            os.link(content, path)
        elif kind == 'fifo':
            os.mkfifo(path)
        else:
            assert kind == 'missing'
        yield
    finally:
        if kind == 'directory':
            path.rmdir()  # Refuse unexpected children; never recursively traverse a fault.
        elif kind != 'missing':
            path.unlink()  # Unlink the exact alias/FIFO without following it.
        if existed:
            backup.rename(path)


def physical_controls():
    originals = {path: path.read_bytes() for path in CORE}
    archives = {name: physical_snapshot(ROOT / name) for name in ('.doctrine/history_archive', 'docs/history')}
    with mirror('physical') as root:
        entry = root / ENTRY.relative_to(ROOT)
        work = root / 'target/scratch/history_archive_probes'
        output = work / 'fixture/.doctrine/history_archive/orphan.json'
        sentinel = root / 'physical-sentinel'
        destination = root / 'physical-destination'
        stores = {name: str(root / 'target/prepared' / name.lower()) for name in PROFILE['STORES']}
        directories = (destination, output.parent, *(Path(value) for value in stores.values()))
        PROFILE['prepare_producer'](ROOT, sources=CORE, directories=directories, output_files=(sentinel,))
        sentinel.write_bytes(b'owned alias target must stay byte exact\n')
        pristine = physical_snapshot(root, metadata=False)
        env = dict(os.environ, **stores)
        env.pop('RUSTUP_TOOLCHAIN', None)
        count = 0

        def invoke(label, expected, *, overrides=None, action='entry'):
            nonlocal count
            before = physical_snapshot(root)
            result = subprocess.run([sys.executable, '-I', '-B', '-c', ENTRY_SENTRY, str(entry), action],
                                    cwd=root, env=dict(env, **(overrides or {})), capture_output=True,
                                    text=True, timeout=60, check=True)
            assert physical_snapshot(root) == before, ('archive physical mirror changed', label)
            lines = result.stdout.splitlines()
            assert len(lines) == 1 and lines[0].startswith('ARCHIVE_ENTRY_RESULT='), (label, result)
            actual = json.loads(lines[0].partition('=')[2])
            assert actual == expected, (label, actual, expected, result.stderr)
            count += 1
            print('archive physical ' + label + ': ' + json.dumps(actual, sort_keys=True), flush=True)

        def refusal(message, type='Refusal'):
            return dict(kind='refusal', type=type, message=message)

        # The independent sentry is calibrated separately from producer refusals.
        for action, event in (('write', 'open'), ('mkdir', 'os.mkdir'), ('child', 'subprocess.Popen')):
            invoke('sentry-' + action, dict(kind='effect', event=event), action=action)
        invoke('valid-prefix', dict(kind='effect', event='os.putenv'))
        cases = (
            ('profile-link', root / PROFILE_SOURCE.relative_to(ROOT), 'symlink', None, refusal('unsafe archive fixture profile', 'ValueError')),
            ('profile-parent-link', root / 'scripts', 'symlink', None, refusal('unsafe archive fixture profile', 'ValueError')),
            ('profile-git-marker', root / 'scripts/.git', 'directory', None, refusal('unsafe archive fixture profile', 'ValueError')),
            ('entry-link', entry, 'symlink', None, refusal('symlink producer source')),
            ('reader-link', root / READER.relative_to(ROOT), 'symlink', None, refusal('symlink producer source')),
            ('shell-link', root / SHELL.relative_to(ROOT), 'symlink', None, refusal('symlink producer source')),
            ('reader-missing', root / READER.relative_to(ROOT), 'missing', None, refusal('producer source is not a local regular file')),
            ('archive-control-link', root / '.doctrine/history_archive/window1.json', 'symlink', None, refusal('symlink archive fixture input')),
            ('workspace-link', work, 'symlink', None, refusal('symlink store component')),
            ('workspace-git-marker', work / '.git', 'directory', None, refusal('store crosses another Git repository')),
            ('workspace-unknown', work / 'unknown', 'bytes', b'owned fault\n', refusal('unknown archive fixture entry')),
            ('output-link', output, 'symlink', sentinel, refusal('unexpected archive fixture link')),
            ('output-hardlink', output, 'hardlink', sentinel, refusal('hard-linked producer output')),
            ('output-fifo', output, 'fifo', None, refusal('producer output is not a local regular file')),
            ('store-link', Path(stores['CARGO_HOME']), 'symlink', None, refusal('symlink store component')),
        )
        for label, path, kind, content, expected in cases:
            with physical_fault(root, path, kind, content):
                invoke(label, expected)
            assert physical_snapshot(root, metadata=False) == pristine, ('archive physical fault restoration', label)
        for label, overrides, message in (
            ('store-source-tree', dict(CARGO_HOME='crates/cache'), 'local store must be under target'),
            ('store-traversal', dict(TMPDIR='target/../scratch'), 'invalid store components'),
            ('store-overlap', dict(TMPDIR=str(work)), 'archive fixture workspace contains a store'),
            ('toolchain-path', dict(RUSTUP_TOOLCHAIN='bad/toolchain'), 'toolchain override must be a name'),
        ):
            invoke(label, refusal(message), overrides=overrides)
        assert physical_snapshot(root, metadata=False) == pristine, 'archive physical final restoration'
    assert all(path.read_bytes() == data for path, data in originals.items()), 'archive physical canonical source changed'
    assert all(physical_snapshot(ROOT / name) == data for name, data in archives.items()), 'archive physical canonical archive changed'
    assert count == 23, count
    print('archive physical:19 actual entry refusals; valid-prefix activation and3 independent sentry controls; '
          'all23 children reaped; faulted mirror metadata/bytes and canonical inputs exact; owned faults restored; rc=0', flush=True)


if __name__ == '__main__':
    assert sys.argv[1:] in ([], ['--before'], ['--population'], ['--deep'], ['--body'], ['--physical'])
    if sys.argv[1:] == ['--body']:
        original_body_controls()
    elif sys.argv[1:] == ['--physical']:
        PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(WORK,))
        physical_controls()
    else:
        PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(WORK,))
        before_control()
        if sys.argv[1:] != ['--before']:
            with mirror('population') as root:
                ns, _ = bootstrap(entry=root / ENTRY.relative_to(ROOT))
                assert not ns['WORK'].exists(), 'native fixture workspace must remain absent'
                assert ns['OWNED_LINK'] in ns['FILES']
                print('closed pure population:', len(ns['DIRECTORIES']), 'directories /', len(ns['FILES']),
                      'files; no effects during bootstrap; retained v1 fixture')
            if sys.argv[1:] in ([], ['--deep']):
                print('actual archive fixture capture:', capture('positive'))
            if sys.argv[1:] == ['--deep']:
                print('protected interrupted copy:', capture('interrupted', interrupt=True))
                metadata_controls()
                omission_controls()
