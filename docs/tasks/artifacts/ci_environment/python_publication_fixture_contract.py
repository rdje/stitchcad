"""Actual copied publication file/population plans and borrowed pure API controls."""
from pathlib import Path
import ast
import contextlib
import csv
import hashlib
import io
import json
import os
import re
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'docs/tasks/artifacts/book_publication/book_publication.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
MAP = ROOT / 'docs/tasks/artifacts/book_publication/status.tsv'
CORE = (ENTRY, PROFILE_SOURCE)
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
WORK = ROOT / 'target/publication-fixture-controls' / str(os.getpid())
PROFILE['prepare_producer'](ROOT, sources=(*CORE, MAP, Path(__file__)), directories=(WORK,))
BEFORE = 'f5f13dc8b62e673805fe1f9b48a7476696c77995'
OLD = subprocess.check_output(['git', 'show', BEFORE + ':' + ENTRY.relative_to(ROOT).as_posix()], text=True)


def module(script=None, filename=ENTRY):
    namespace = dict(__file__=str(filename), __name__='publication_fixture_control')
    exec(compile(ENTRY.read_text() if script is None else script, str(filename), 'exec'), namespace)
    return namespace


def population(root, destination):
    source, html = root / 'docs/book/src', root / 'docs/book/book'
    with (root / MAP.relative_to(ROOT)).open() as stream:
        rows = list(csv.reader((line for line in stream if not line.startswith('#')), delimiter='\t'))
    copied = {root / 'docs/book/book.toml', *source.rglob('*'), *(root / 'docs/tasks').glob('*.md'),
              *html.rglob('*.html'), root / 'ROADMAP.md', root / MAP.relative_to(ROOT),
              root / 'crates/sc-core/tests/formula_statement_contract.rs', *(root / row[2] for row in rows)}
    copied = {path for path in copied if path.is_file()}
    files = {destination / path.relative_to(root) for path in copied}
    directories = {destination, destination / 'docs/book', destination / 'docs/tasks', destination / 'docs/book/book',
                   destination / 'docs/book/src', *(destination / path.relative_to(root) for path in source.rglob('*') if path.is_dir())}
    for file in files:
        parent = file.parent
        while parent.is_relative_to(destination):
            directories.add(parent)
            if parent == destination:
                break
            parent = parent.parent
    return copied, files, directories


def exact_copied(root, destination):
    copied, files, _ = population(root, destination)
    assert all((destination / path.relative_to(root)).read_bytes() == path.read_bytes() for path in copied)
    assert {path for path in destination.rglob('*') if path.is_file()} == files


def before_control():
    old = module(OLD)
    target = ROOT / 'docs/book/book.toml'
    frozen = target.read_bytes()
    original = Path.write_text
    attempted = []

    def write(path, data, *args, **kwargs):
        assert path == target and data != frozen.decode()
        attempted.append(path)
        raise RuntimeError('trapped original unowned write')

    try:
        Path.write_text = write
        try:
            old['replace'](target, 'title = "StitchCAD"', 'title = "Incorrect"')
        except RuntimeError as error:
            assert str(error) == 'trapped original unowned write'
        else:
            raise AssertionError('original write not trapped')
    finally:
        Path.write_text = original
    assert attempted == [target] and target.read_bytes() == frozen
    print('original actual replace reaches trapped tracked-config write without authority; exact source bytes')


def capture_fixture(label, script=None):
    ns = module(script)
    destination = WORK / label
    copied, files, directories = population(ROOT, destination)
    sources = {*CORE, *copied}
    identities = {path: hashlib.sha256(path.read_bytes()).digest() for path in sources}
    real = ns['PROFILE']
    saved = dict(os.environ)
    stores = {name: str(WORK / 'caller-stores' / name.lower()) for name in real['STORES']}
    os.environ.update(stores, PUBLICATION_CALLER='preserved')
    originals = (Path.mkdir, shutil.copy2, shutil.copytree)
    plans, activated, effects = [], [], []
    latest = {}
    checkpoint = 0

    def direct():
        return sys._getframe(2).f_globals is ns

    def plan(root, **kwargs):
        assert root == ROOT and not kwargs.get('mutable_sources'), 'publication copied mutable authority'
        declared = set(kwargs.get('sources', ()))
        outputs = set(kwargs.get('output_files', ()))
        folders = set(kwargs.get('directories', ()))
        assert {*CORE, MAP} <= declared <= sources, 'publication copied inputs omitted'
        assert outputs <= files and folders <= directories, 'publication copied output outside population'
        if not activated and (outputs or folders):
            assert declared == sources and outputs == files and folders == directories, 'publication full copy plan omitted'
        plans.append(kwargs); latest.clear(); latest.update(sources=declared, files=outputs, dirs=folders, generation=len(plans))
        return real['plan_producer'](root, **kwargs)

    def environment(root, variables, *, create=True):
        if create:
            assert latest.get('sources') == sources and latest.get('files') == files and latest.get('dirs') == directories, 'publication activation before full copy plan'
            activated.append(latest['generation'])
        return real['environment_for'](root, variables, create=create)

    def fresh():
        nonlocal checkpoint
        assert activated and latest['generation'] > checkpoint, 'publication copied effect recheck omitted'
        checkpoint = latest['generation']

    def mkdir(path, *args, **kwargs):
        if direct():
            fresh(); assert path in latest['dirs'], 'publication mkdir outside selected plan'
            effects.append(('mkdir', path))
        return originals[0](path, *args, **kwargs)

    def copy(source, target, *args, **kwargs):
        if direct():
            fresh(); assert Path(source) in latest['sources'] and Path(target) in latest['files'], 'publication copy outside selected plan'
            effects.append(('copy', Path(target)))
        return originals[1](source, target, *args, **kwargs)

    def tree(source, target, *args, **kwargs):
        if direct():
            fresh()
            expected = {path for path in copied if path.is_relative_to(Path(source))}
            expected_outputs = {Path(target) / path.relative_to(source) for path in expected}
            assert expected <= latest['sources'] and expected_outputs <= latest['files'], 'publication tree files omitted'
            effects.append(('tree', Path(target)))
        return originals[2](source, target, *args, **kwargs)

    try:
        ns['PROFILE'] = dict(real, plan_producer=plan, environment_for=environment)
        Path.mkdir, shutil.copy2, shutil.copytree = mkdir, copy, tree
        ns['fixture'](ROOT, destination)
        assert len(activated) == 1 and all(os.environ[name] == value for name, value in stores.items())
        assert os.environ['PUBLICATION_CALLER'] == 'preserved'
        exact_copied(ROOT, destination)
        assert all(hashlib.sha256(path.read_bytes()).digest() == digest for path, digest in identities.items())
        print('publication fixture capture:', len(plans), 'plans;', len(effects), 'direct effects;', len(copied), 'copied inputs;', len(files), 'declared files; source bytes/caller stores exact')
    finally:
        Path.mkdir, shutil.copy2, shutil.copytree = originals
        os.environ.clear(); os.environ.update(saved)
    return destination


def cases():
    names = ['unregistered chapter', 'missing indexed chapter', 'missing source target',
             'missing source anchor', 'missing rendered chapter', 'missing rendered anchor',
             'stale G0 status', 'missing public MTM API', 'missing formula proof boundary',
             'stale statement contract count', 'missing statement contract marker']
    ns = module()
    names += ['retired formula claim:%s:%d' % (chapter, ordinal)
              for chapter, claims in ns['FORMULA_SCOPE'].items() for ordinal in range(len(claims))]
    names += ['missing formula method:%s:%s' % (file, method)
              for file, methods in ns['FORMULA_METHODS'].items() for method in methods]
    return names


def capture_mutation(root, name, script=None, *, relative=False):
    ns = module(script)
    target = ns['mutation_target'](root, name)
    frozen = {path: path.read_bytes() for path in root.rglob('*') if path.is_file()}
    identities = {path: hashlib.sha256(path.read_bytes()).digest() for path in CORE}
    real = ns['PROFILE']
    saved, cwd = dict(os.environ), Path.cwd()
    stores = {key: str(WORK / 'mutation-stores' / key.lower()) for key in real['STORES']}
    os.environ.update(stores, PUBLICATION_CALLER='preserved')
    original = (Path.write_text, Path.open, Path.unlink)
    plans, activated, effects = [], [], []
    latest = {}

    def direct():
        return sys._getframe(2).f_globals is ns

    def plan(selected_root, **kwargs):
        assert selected_root == ROOT
        sources = {*CORE, target} if target.exists() else set(CORE)
        assert set(kwargs.get('sources', ())) == sources, 'publication mutation inputs omitted'
        assert set(kwargs.get('mutable_sources', ())) == ({target} if target.exists() else set()), 'publication mutation mutable authority omitted'
        assert set(kwargs.get('output_files', ())) == {target}, 'publication mutation output omitted'
        assert not kwargs.get('directories'), 'publication mutation unexpected directories'
        plans.append(kwargs); latest.clear(); latest.update(kwargs)
        return real['plan_producer'](selected_root, **kwargs)

    def environment(selected_root, variables, *, create=True):
        if create:
            assert latest, 'publication mutation activation before plan'
            activated.append(len(plans))
        return real['environment_for'](selected_root, variables, create=create)

    def effect(path):
        assert Path(path).absolute() == target and latest, 'publication mutation effect before plan'
        assert activated and activated[-1] == len(plans), 'publication mutation effect before fresh activation'
        effects.append(Path(path).absolute())

    def write(path, *args, **kwargs):
        if direct():
            effect(path)
        return original[0](path, *args, **kwargs)

    def opening(path, mode='r', *args, **kwargs):
        if direct() and any(char in mode for char in 'wax+'):
            effect(path)
        return original[1](path, mode, *args, **kwargs)

    def unlink(path, *args, **kwargs):
        if direct():
            effect(path)
        return original[2](path, *args, **kwargs)

    try:
        ns['PROFILE'] = dict(real, plan_producer=plan, environment_for=environment)
        Path.write_text, Path.open, Path.unlink = write, opening, unlink
        if relative:
            os.chdir(ROOT)
        ns['mutate'](root.relative_to(ROOT) if relative else root, name)
        assert effects == [target] and len(activated) == 1
        assert all(os.environ[key] == value for key, value in stores.items())
        assert os.environ['PUBLICATION_CALLER'] == 'preserved'
        changed = {path for path, data in frozen.items() if not path.exists() or path.read_bytes() != data}
        changed.update(path for path in root.rglob('*') if path.is_file() and path not in frozen)
        assert changed == {target}, ('publication mutation changed other files', name, changed)
    finally:
        os.chdir(cwd)
        Path.write_text, Path.open, Path.unlink = original
        os.environ.clear(); os.environ.update(saved)
        if target in frozen:
            PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), output_files=(target,))
            target.write_bytes(frozen[target])
        elif target.exists():
            PROFILE['plan_producer'](ROOT, sources=(*CORE, target, Path(__file__)), mutable_sources=(target,), output_files=(target,))
            target.unlink()
        assert all(path.read_bytes() == data for path, data in frozen.items())
        assert all(hashlib.sha256(path.read_bytes()).digest() == digest for path, digest in identities.items())


def expect_red(action, reason):
    try:
        action()
    except AssertionError as error:
        assert str(error) == reason, (reason, error)
    else:
        raise AssertionError(('publication omission escaped', reason))


def omission_controls(root):
    script = ENTRY.read_text()
    faults = (
        ('copy-input', 'inputs = {*CORE, *copied}', 'inputs = {*CORE, *copied} - {root / "ROADMAP.md"}', 'publication full copy plan omitted'),
        ('copy-file', 'output_files=tuple(sorted(outputs)))', 'output_files=())', 'publication full copy plan omitted'),
        ('copy-dir', 'directories=tuple(sorted(directories)),\n                                 output_files=', 'directories=(),\n                                 output_files=', 'publication full copy plan omitted'),
        ('copy-recheck', '    fixture_effect(plan, sources=(root / "docs/book/book.toml",), files=(destination / "docs/book/book.toml",))\n', '', 'publication copied effect recheck omitted'),
    )
    for label, before, after, reason in faults:
        assert script.count(before) == 1, ('publication omission anchor', label)
        expect_red(lambda: capture_fixture('red-' + label, script.replace(before, after, 1)), reason)
    faults = (
        ('sources=sources, mutable_sources=mutable, output_files=(file,)', 'sources=CORE, mutable_sources=(), output_files=(file,)', 'publication mutation inputs omitted'),
        ('mutable_sources=mutable, output_files=(file,)', 'mutable_sources=(), output_files=(file,)', 'publication mutation mutable authority omitted'),
        ('mutable_sources=mutable, output_files=(file,)', 'mutable_sources=mutable, output_files=()', 'publication mutation output omitted'),
        ('    mutation_plan(file, activate=True)\n', '', 'publication mutation effect before fresh activation'),
    )
    for before, after, reason in faults:
        assert script.count(before) == 1
        expect_red(lambda: capture_mutation(root, 'stale G0 status', script.replace(before, after, 1)), reason)
    for name, before, after in (
        ('retired formula claim:syntax:0', '        mutation_plan(path, activate=True)\n        path.write_text(intro +', '        path.write_text(intro +'),
        ('missing source target', '        mutation_plan(selected, activate=True)\n        with (source / "introduction.md").open("a") as stream:\n            stream.write("\\n[Missing](missing-chapter.md)\\n")', '        with (source / "introduction.md").open("a") as stream:\n            stream.write("\\n[Missing](missing-chapter.md)\\n")'),
        ('missing rendered chapter', '        mutation_plan(selected, activate=True)\n        (root / HTML / "learn/design-to-pattern.html").unlink()', '        (root / HTML / "learn/design-to-pattern.html").unlink()'),
    ):
        assert script.count(before) == 1
        expect_red(lambda: capture_mutation(root, name, script.replace(before, after, 1)), 'publication mutation effect before fresh activation')
    print('publication omission controls: eleven actual input/file/directory/fresh-effect/mutable/write/append/delete/activation assertion reds; source bytes exact')


def snapshot(root):
    result = {}
    for folder, directories, files in os.walk(root, followlinks=False):
        for name in [*directories, *files]:
            path = Path(folder) / name
            result[path.relative_to(root).as_posix()] = (('link', os.readlink(path)) if path.is_symlink()
                else ('file', path.read_bytes()) if path.is_file() else ('other', path.lstat().st_mode))
    return result


def physical_controls():
    labels = ('source-link', 'source-missing', 'source-marker', 'destination-link',
              'destination-marker', 'output-link', 'output-hardlink', 'output-directory',
              'mutation-hardlink', 'mutation-tracked', 'toolchain', 'profile-link')
    for label in labels:
        root = WORK / ('physical-' + label)
        module()['fixture'](ROOT, root)
        for source in CORE:
            target = root / source.relative_to(ROOT)
            PROFILE['prepare_producer'](ROOT, sources=(*CORE, Path(__file__)), directories=(target.parent,), output_files=(target,))
            target.write_bytes(source.read_bytes())
        entry, profile = (root / path.relative_to(ROOT) for path in CORE)
        destination = root / 'target/copied'
        selected = root / 'docs/book/book.toml'
        output = destination / selected.relative_to(root)
        PROFILE['prepare_producer'](ROOT, sources=(*CORE, Path(__file__)), directories=(output.parent,), output_files=(output,))
        if label == 'source-link':
            selected.unlink(); selected.symlink_to(ROOT / selected.relative_to(root))
        elif label == 'source-missing':
            selected.unlink()
        elif label == 'source-marker':
            (root / 'docs/book/src/.git').mkdir()
        elif label == 'destination-link':
            shutil.rmtree(destination); destination.symlink_to(ROOT / 'target', target_is_directory=True)
        elif label == 'destination-marker':
            (destination / '.git').mkdir()
        elif label == 'output-link':
            output.symlink_to(selected)
        elif label == 'output-hardlink':
            os.link(selected, output)
        elif label == 'output-directory':
            output.mkdir()
        elif label == 'mutation-hardlink':
            os.link(selected, root / 'target/alias')
        elif label == 'profile-link':
            profile.unlink(); profile.symlink_to(PROFILE_SOURCE)
        saved = dict(os.environ)
        os.environ.update({key: str(root / relative) for key, relative in PROFILE['STORES'].items()})
        if label == 'toolchain':
            os.environ['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'
        frozen = snapshot(root)
        try:
            try:
                ns = module(filename=entry)
                if label.startswith('mutation-'):
                    ns['replace'](selected if label == 'mutation-hardlink' else ROOT / 'docs/book/book.toml', 'title = "StitchCAD"', 'title = "Incorrect"')
                else:
                    ns['fixture'](root, destination)
            except Exception as error:
                assert type(error).__name__ == 'PublicationError' and str(error).startswith('FIXTURE_AUTHORITY:'), (label, error)
            else:
                raise AssertionError(('publication physical refusal missing', label))
            assert snapshot(root) == frozen, ('publication physical refusal changed files', label)
        finally:
            os.environ.clear(); os.environ.update(saved)
    print('publication physical controls: twelve actual source/profile/output/mutable/store/marker refusals; mirrors exact')


def borrowed_controls(root):
    ns = module()
    expected = ns['check'](ROOT)
    frozen, environment = snapshot(root), dict(os.environ)

    def refuse(*args, **kwargs):
        raise AssertionError('borrowed pure API activated a producer')

    ns['PROFILE'] = {key: refuse for key in ns['PROFILE']}
    assert ns['check'](root) == expected
    ns['rows'](root); ns['formula_scope'](root)
    assert snapshot(root) == frozen and dict(os.environ) == environment
    print('publication borrowed pure APIs: check/rows/formula_scope exact; zero profile calls/writes/environment changes')


def original_body_controls():
    old, current = ast.parse(OLD), ast.parse(ENTRY.read_text())
    added = {'load_profile', 'source_tree', 'fixture_plan', 'fixture_effect', 'mutation_plan', 'mutation_target',
             'temporary_root', 'build_inputs', 'publication_environment', 'publication_plan', 'build', 'main_inputs'}

    class Project(ast.NodeTransformer):
        def visit_FunctionDef(self, node):
            return None if node.name in added else self.generic_visit(node)

        def visit_Import(self, node):
            return None if len(node.names) == 1 and node.names[0].name in {'runpy', 'hashlib', 'json', 'uuid', 'tempfile'} else node

        def visit_ImportFrom(self, node):
            return None if node.module == 'contextlib' else node

        def visit_Call(self, node):
            if isinstance(node.func, ast.Name) and node.func.id == 'temporary_root':
                node.func = ast.Attribute(value=ast.Name(id='tempfile', ctx=ast.Load()), attr='TemporaryDirectory', ctx=ast.Load())
                node.keywords = [keyword for keyword in node.keywords if keyword.arg != 'fixture_root']
            return self.generic_visit(node)

        def visit_Assign(self, node):
            if any(isinstance(target, ast.Name) and target.id in {'PROFILE_SOURCE', 'PROFILE', 'CORE', 'plan', 'selected', 'current_dirs', 'current_files'} for target in node.targets):
                return None
            if any(isinstance(target, ast.Tuple) and all(isinstance(value, ast.Name) for value in target.elts) and
                   [value.id for value in target.elts] in (['root', 'destination'], ['current_dirs', 'current_files']) for target in node.targets):
                return None
            return self.generic_visit(node)

        def visit_Expr(self, node):
            if (isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Attribute) and
                    isinstance(node.value.func.value, ast.Name) and node.value.func.value.id == 'scratch' and
                    node.value.func.attr == 'mkdir'):
                return None
            if isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name):
                if node.value.func.id in {'fixture_effect', 'mutation_plan', 'main_inputs'}:
                    return None
                if node.value.func.id == 'require' and len(node.value.args) > 1 and isinstance(node.value.args[1], ast.Constant) and node.value.args[1].value == 'FIXTURE_AUTHORITY':
                    return None
            return self.generic_visit(node)

        def visit_Subscript(self, node):
            if isinstance(node.value, ast.Name) and node.value.id == 'plan':
                assert isinstance(node.slice, ast.Constant) and node.slice.value == 'rows'
                return ast.Call(func=ast.Name(id='rows', ctx=ast.Load()), args=[ast.Name(id='root', ctx=ast.Load())], keywords=[])
            return self.generic_visit(node)

    assert ast.dump(Project().visit(current), include_attributes=False) == ast.dump(Project().visit(old), include_attributes=False), 'publication original whole AST changed'
    print('publication original AST: pure/copier/mutation/main semantics exact through declared guards/owned temporary context; builder separately captured')


def continuity_controls(*, observe=None):
    reader = ROOT / 'scripts/history_archive.py'
    # These are logical records: later windows may retire their physical source files.
    archives = tuple(path for directory in (ROOT / 'docs/history', ROOT / '.doctrine/history_archive')
                     for path in directory.rglob('*') if path.is_file())
    PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__), reader, *archives))

    def read(path):
        if path.startswith('docs/history/stitchcad-'):
            result = subprocess.run([sys.executable, '-I', '-B', str(reader), '--root', str(ROOT),
                                     'read', path], cwd=ROOT, capture_output=True)
            assert result.returncode == 0, ('publication archive read failed', path, result.stderr)
            data = result.stdout
        else:
            data = (ROOT / path).read_bytes()
        return observe(path, data) if observe else data

    boundary = '7dfd6474cf1e974aa51163e5b68f727806cd3c1b'
    captures = [(name, BEFORE) for name in ('stitchcad-devnotes-part298.md', 'stitchcad-changelog-part206.md',
                                          'stitchcad-devnotes-part299.md')]
    captures += [(name, boundary) for name in ('stitchcad-devnotes-part305.md', 'stitchcad-changelog-part210.md',
                                             'stitchcad-devnotes-part306.md')]
    captures += [(name, '8e497de58f772a74c4d5e62c4489ced1569e83df') for name in ("stitchcad-devnotes-part307.md", "stitchcad-changelog-part211.md")]
    captures += [(name, 'bc730f49d3050531f6b39f3747b266669bf6eec1') for name in ("stitchcad-devnotes-part308.md", "stitchcad-changelog-part212.md")]
    for name, revision in captures:
        path = 'docs/history/' + name
        data = read(path); header, body = data.split(b'\n---\n\n', 1)
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', header.decode())
        assert (body.count(b'\n'), len(body), hashlib.sha256(body).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3]), 'publication sealed record changed'
        chunks = []
        for source, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', header.decode()):
            chunk = subprocess.check_output(['git', 'show', revision + ':' + source], cwd=ROOT)[int(start):int(stop)]
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest
            chunks.append(chunk)
        assert b''.join(chunks).rstrip(b'\n') + b'\n' == body, 'publication original source interval changed'
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        previous = subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT)
        current = read(path)
        if path == '.doctrine/history_archive/windows.json':
            old, new = json.loads(previous), json.loads(current)
            assert set(new) == set(old) and new['version'] == old['version'], 'publication archive master schema changed'
            assert new['windows'][:len(old['windows'])] == old['windows'], 'publication prior windows changed'
            assert len(set(new['windows'])) == len(new['windows']), 'publication duplicate window'
        else:
            assert current == previous, ('publication prior archive bytes changed', path)
    tree = 'docs/tasks/G1-SLICE-producers.md'
    previous = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    headers = lambda text: re.findall(r'^- ID: `([^`]+)`$', text, re.M)
    expected = [value for old in headers(previous)
        for value in ([old, old + '.1', old + '.2'] if old.endswith('a23.3') else
                      [old, old[:-1] + '3'] if old.endswith('f.cleanup2') else [old])]
    current = headers(read(tree).decode())
    assert [value for value in current if value in expected] == expected, 'publication predecessor task graph changed'
    assert len(current) == len(set(current)), 'publication duplicate task node'
    print(f'publication continuity: {len(captures)} whole captures/public reads/prior logical archives exact; predecessor task/window order preserved with later additions')


def continuity_refusals():
    """Replay actual reader/Git observations; corrupt one boundary without editing retained data."""
    originals = subprocess.run, subprocess.check_output
    run_cache, git_cache = {}, {}

    def run(argv, **kwargs):
        key = (*argv, kwargs.get('text', False))
        if key not in run_cache:
            run_cache[key] = originals[0](argv, **kwargs)
        return run_cache[key]

    def git(argv, **kwargs):
        key = (tuple(argv), kwargs.get('text', False))
        if key not in git_cache:
            git_cache[key] = originals[1](argv, **kwargs)
        return git_cache[key]

    record = 'docs/history/stitchcad-devnotes-part298.md'
    master, tree = '.doctrine/history_archive/windows.json', 'docs/tasks/G1-SLICE-producers.md'

    def catalog(data, mode):
        value = json.loads(data)
        if mode == 'remove':
            value['windows'].pop(0)
        elif mode == 'reorder':
            value['windows'][:2] = reversed(value['windows'][:2])
        elif mode == 'duplicate':
            value['windows'].append(value['windows'][0])
        elif mode == 'schema':
            value['unexpected'] = True
        return json.dumps(value).encode()

    def nodes(data, mode):
        text = data.decode()
        markers = re.findall(r'^- ID: `[^`]+`$', text, re.M)
        if mode == 'remove':
            text = text.replace(markers[0], '', 1)
        elif mode == 'reorder':
            text = text.replace(markers[0], 'CONTROL_NODE', 1).replace(markers[1], markers[0], 1).replace('CONTROL_NODE', markers[1], 1)
        elif mode == 'duplicate':
            text += '\n' + markers[-1] + '\n'
        return text.encode()

    cases = [
        (record, lambda data: data + b'corrupt', 'publication sealed record changed'),
        ('docs/history/stitchcad-changelog-part188.md', lambda data: data + b'corrupt',
         ('publication prior archive bytes changed', 'docs/history/stitchcad-changelog-part188.md')),
        *[(master, lambda data, mode=mode: catalog(data, mode), reason) for mode, reason in (
            ('remove', 'publication prior windows changed'), ('reorder', 'publication prior windows changed'),
            ('duplicate', 'publication duplicate window'), ('schema', 'publication archive master schema changed'))],
        *[(tree, lambda data, mode=mode: nodes(data, mode), reason) for mode, reason in (
            ('remove', 'publication predecessor task graph changed'), ('reorder', 'publication predecessor task graph changed'),
            ('duplicate', 'publication duplicate task node'))],
    ]

    def refuses(checker, path, change, reason):
        try:
            with contextlib.redirect_stdout(io.StringIO()):
                checker(observe=lambda selected, data: change(data) if selected == path else data)
        except AssertionError as error:
            assert error.args == (reason,), (reason, error)
        else:
            raise AssertionError(('publication continuity corruption escaped', reason))

    try:
        subprocess.run, subprocess.check_output = run, git
        with contextlib.redirect_stdout(io.StringIO()):
            continuity_controls()
        for path, change, reason in cases:
            refuses(continuity_controls, path, change, reason)
        selected = next(key for key in run_cache if key[-2] == record)
        result = run_cache[selected]
        run_cache[selected] = subprocess.CompletedProcess(result.args, 2, b'', b'controlled unavailable archive')
        try:
            continuity_controls()
        except AssertionError as error:
            assert error.args[0][:2] == ('publication archive read failed', record), error
        else:
            raise AssertionError('publication failed reader accepted')
        run_cache[selected] = result

        source = Path(__file__).read_text()
        function = next(node for node in ast.parse(source).body if isinstance(node, ast.FunctionDef) and node.name == 'continuity_controls')
        body = ast.get_source_segment(source, function)
        faults = (
            ("assert current == previous, ('publication prior archive bytes changed', path)", 'pass', 1),
            ("assert new['windows'][:len(old['windows'])] == old['windows'], 'publication prior windows changed'", 'pass', 2),
            ("assert [value for value in current if value in expected] == expected, 'publication predecessor task graph changed'", 'pass', 6),
        )
        for anchor, replacement, case in faults:
            assert body.count(anchor) == 1
            namespace = dict(globals())
            exec(compile(body.replace(anchor, replacement, 1), __file__, 'exec'), namespace)
            path, change, reason = cases[case]
            try:
                refuses(namespace['continuity_controls'], path, change, reason)
            except AssertionError as error:
                assert error.args[0] == ('publication continuity corruption escaped', reason), error
            else:
                raise AssertionError('publication continuity body fault missed')
    finally:
        subprocess.run, subprocess.check_output = originals
    print('publication continuity: ten independent refusals / three compiled guard-omission assertion reds; retained files untouched')


def real_controls():
    inputs, _, _ = population(ROOT, WORK / 'real-unused')
    identities = {path: hashlib.sha256(path.read_bytes()).digest() for path in {*CORE, *inputs}
                  if not path.is_relative_to(ROOT / 'docs/book/book')}
    result = subprocess.run([sys.executable, '-I', '-B', str(ENTRY)], cwd=ROOT,
                            env=dict(os.environ), capture_output=True, text=True)
    assert result.returncode == 0 and 'publication probes: 45 pass / 0 fail' in result.stdout, result
    assert 'actual malformed generic (BOOK_BUILD_WARNING); repaired rendering pass' in result.stdout
    changed = [path.relative_to(ROOT).as_posix() for path, digest in identities.items()
               if hashlib.sha256(path.read_bytes()).digest() != digest]
    assert not changed, ('publication real inputs changed during verification', changed)
    print(result.stdout, end='')
    print('publication real: original45 current cases/rendered repair/source bytes exact; actual builder/main authority remains owned by .2')


if __name__ == '__main__':
    if sys.argv[1:] == ['--before']:
        before_control()
    elif sys.argv[1:] == ['--continuity']:
        continuity_controls()
        continuity_refusals()
    elif sys.argv[1:] == ['--real']:
        real_controls()
    else:
        assert not sys.argv[1:]
        before_control()
        root = capture_fixture('positive')
        for name in cases():
            capture_mutation(root, name)
        capture_mutation(root, 'stale G0 status', relative=True)
        print('publication mutation captures:', len(cases()), 'branches; one relative-root positive; complete inputs/mutable/files/stores; exact restoration')
        omission_controls(root)
        physical_controls()
        borrowed_controls(root)
        original_body_controls()
        continuity_controls()
        continuity_refusals()
