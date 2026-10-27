"""Public archive reader: readonly purity, complete materialization and metadata authority."""
import ast
import contextlib
import io
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'scripts/history_archive.py'
GENERATIONS = ROOT / 'scripts/history_generations.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
WORK = ROOT / 'target/archive-reader-controls' / str(os.getpid())
LEGACY = ROOT / 'docs/tasks/artifacts/history_archive/legacy_refusals.py'
CORE = (Path(__file__), ENTRY, GENERATIONS, PROFILE_SOURCE, LEGACY)
BEFORE = '4027b36'


def module(source=None):
    ns = dict(__file__=str(ENTRY), __name__='archive_reader_control')
    exec(compile(ENTRY.read_text() if source is None else source, str(ENTRY), 'exec'), ns)
    return ns


class Output(io.StringIO):
    def __init__(self):
        super().__init__()
        self.buffer = io.BytesIO()


def cli(ns, root, mode, argument=None):
    argv, sys.argv = sys.argv, [str(ENTRY), '--root', str(root), mode]
    if argument is not None:
        sys.argv.append(argument)
    output = Output()
    try:
        with contextlib.redirect_stdout(output):
            ns['main']()
        return output.getvalue(), output.buffer.getvalue()
    finally:
        sys.argv = argv


@contextlib.contextmanager
def fixture(label, *, legacy=False):
    root = WORK / label
    files = []
    directories = {WORK, root}
    for base in (ROOT / '.doctrine/history_archive', ROOT / 'docs/history'):
        for folder, dirs, leaves in os.walk(base, followlinks=False):
            PROFILE['directory'](ROOT, Path(folder).relative_to(ROOT).as_posix())
            for name in dirs:
                PROFILE['directory'](ROOT, (Path(folder) / name).relative_to(ROOT).as_posix())
            files.extend(Path(folder) / name for name in leaves)
    contents = {path.relative_to(ROOT).as_posix(): path.read_bytes() for path in files}
    if legacy:
        PROFILE['plan_producer'](ROOT, sources=(*CORE, *files))
        reader = module()
        contents = runpy.run_path(str(LEGACY))['baseline'](reader, ROOT)
    destinations = {name: root / name for name in contents}
    for path in destinations.values():
        directories.update(parent for parent in path.parents if parent == root or parent.is_relative_to(root))
    plan = dict(sources=(*CORE, *files), directories=tuple(sorted(directories)),
                output_files=tuple(destinations.values()))
    PROFILE['require'](not root.exists() and not root.is_symlink(), 'archive reader fixture must be fresh')
    PROFILE['prepare_producer'](ROOT, **plan)
    try:
        for name, destination in destinations.items():
            PROFILE['plan_producer'](ROOT, **plan)
            destination.write_bytes(contents[name])
        yield root
    finally:
        # Materialization intentionally creates additional files. Inspect every actual entry before
        # removing this unique, owned fixture; never traverse a foreign marker or link.
        outputs, folders = [], [root]
        for folder in folders:
            PROFILE['directory'](ROOT, folder.relative_to(ROOT).as_posix())
            for path in sorted(folder.iterdir()):
                assert not path.is_symlink(), 'archive reader cleanup link'
                (folders if path.is_dir() else outputs).append(path)
        PROFILE['plan_producer'](ROOT, sources=CORE, directories=tuple(folders), output_files=tuple(outputs))
        assert shutil.rmtree.avoids_symlink_attacks
        shutil.rmtree(root)


def before_controls_at(authority):
    old = subprocess.check_output(['git', 'show', BEFORE + ':scripts/history_archive.py'], cwd=ROOT, text=True)
    ns = module(old)
    saved_env = dict(os.environ)
    mkdir, run = Path.mkdir, subprocess.run
    seen = []

    class Trapped(Exception):
        pass

    def trap(path, *args, **kwargs):
        seen.append(path)
        raise Trapped

    try:
        for name in PROFILE['STORES']:
            os.environ.pop(name, None)
        Path.mkdir = trap
        try:
            cli(ns, authority, 'materialize', 'target/archive-reader-before-' + str(os.getpid()))
        except Trapped:
            pass
        else:
            raise AssertionError('old archive materialization first effect escaped')
        assert len(seen) == 1 and seen[0].parent == authority / 'target'
        subprocess.run = lambda *args, **kwargs: subprocess.CompletedProcess(args[0], 128, b'', b'controlled unavailable HEAD')
        assert ns['guard_committed'](authority) is None, 'old metadata failure did not reproduce'
    finally:
        Path.mkdir, subprocess.run = mkdir, run
        os.environ.clear(); os.environ.update(saved_env)
    print('original materialization mkdir trapped without stores; unavailable Git HEAD incorrectly accepted')


def before_controls():
    with fixture('v1-before', legacy=True) as authority:
        before_controls_at(authority)


def pure_controls():
    ns = module()
    old = subprocess.check_output(['git', 'show', BEFORE + ':scripts/history_archive.py'], cwd=ROOT, text=True)
    definitions = lambda source: {node.name: ast.dump(node, include_attributes=False)
                                  for node in ast.parse(source).body if isinstance(node, ast.FunctionDef)}
    previous = definitions(old)
    for name, body in previous.items():
        if name in ('guard_committed', 'main'):
            continue
        target = {'master': '_master_v1', 'inventory': '_inventory_v1'}.get(name, name)
        node = next(node for node in ast.parse(ENTRY.read_text()).body
                    if isinstance(node, ast.FunctionDef) and node.name == target)
        node.name = name  # Original v1 bodies remain exact behind explicit version dispatch.
        assert ast.dump(node, include_attributes=False) == body, ('archive pure original body changed', name)
    records, _ = ns['inventory'](ROOT)
    name = sorted(records)[0]
    saved_env = dict(os.environ)
    original = Path.mkdir, Path.write_bytes, subprocess.run

    def no_effect(*args, **kwargs):
        raise AssertionError('archive readonly API activated effects')

    try:
        os.environ.update(TMPDIR='target/../unused-store', RUSTUP_TOOLCHAIN='invalid/channel')
        wanted = dict(os.environ)
        Path.mkdir = Path.write_bytes = subprocess.run = no_effect
        ns['load_profile'] = no_effect
        assert ns['inventory'](ROOT)[0] == records
        assert cli(ns, ROOT, 'read', name)[1] == records[name]
        assert cli(ns, ROOT, 'list')[0].splitlines() == sorted(records)
        assert 'archive: OK' in cli(ns, ROOT, 'verify')[0]
        assert dict(os.environ) == wanted
    finally:
        Path.mkdir, Path.write_bytes, subprocess.run = original
        os.environ.clear(); os.environ.update(saved_env)
    print('archive original pure AST/inventory/read/list/verify exact; unused invalid stores ignored; no activation/child/write')


def original_cli_controls():
    old = subprocess.check_output(['git', 'show', BEFORE + ':scripts/history_archive.py'], cwd=ROOT, text=True)
    old_tree, tree = ast.parse(old), ast.parse(ENTRY.read_text())
    functions = lambda value: {node.name: node for node in value.body if isinstance(node, ast.FunctionDef)}
    previous, current = functions(old_tree), functions(tree)
    old_main = previous['main']
    root_index = next(index for index, node in enumerate(old_main.body) if isinstance(node, ast.Assign)
                      and any(isinstance(target, ast.Name) and target.id == 'root' for target in node.targets))

    class Project(ast.NodeTransformer):
        def visit_If(self, node):
            if isinstance(node.test, ast.Call) and isinstance(node.test.func, ast.Name) and node.test.func.id in {
                    '_uses_v2', '_uses_v3_document', '_extended_cli'}:
                return None  # Added version/operation branches have their own native controls.
            return self.generic_visit(node)

        def visit_Assign(self, node):
            if any(isinstance(target, ast.Name) and target.id == 'profile' for target in node.targets):
                return None
            if isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name) and node.value.func.id == 'archive_root':
                return old_main.body[root_index:root_index + 3]
            return self.generic_visit(node)

        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name) and node.value.func.id in {
                    'materialization_plan', 'materialization_file_plan', '_guard_raw_records'}:
                return None
            return self.generic_visit(node)

        def visit_Call(self, node):
            self.generic_visit(node)
            node.keywords = [keyword for keyword in node.keywords if keyword.arg != 'env']
            return node

    assert ast.dump(Project().visit(current['main']), include_attributes=False) == ast.dump(old_main, include_attributes=False), 'archive original CLI body changed'
    # V3 adds closed typed metadata validation before the unchanged v1 successful comparison.
    # Remove only these exact additions, asserting their presence; never erase arbitrary predicates.
    additions = ast.parse("""
require(type(old) is dict and type(old.get('version')) is int and old['version'] in (1, 2, 3),
        'unsupported committed archive schema')
document, _ = load_json(root, MASTER, 8192)
require(old['version'] == 1, 'unsupported committed archive schema')
fields(old, 'version windows')
""").body
    additions = {ast.dump(node, include_attributes=False) for node in additions}
    tails = []
    for fn in (previous['guard_committed'], current['guard_committed']):
        index = next(index for index, node in enumerate(fn.body) if isinstance(node, ast.Assign)
                     and any(isinstance(target, ast.Name) and target.id == 'old' for target in node.targets))
        tail = ast.Module(body=fn.body[index:], type_ignores=[])
        if fn is current['guard_committed']:
            assert all(sum(ast.dump(node, include_attributes=False) == addition for node in tail.body) == 1
                       for addition in additions), 'declared typed metadata adapters changed'
            tail.body = [node for node in tail.body if ast.dump(node, include_attributes=False) not in additions]
        tails.append(ast.dump(Project().visit(tail), include_attributes=False))
    assert tails[0] == tails[1], 'archive successful committed comparison changed'
    print('archive complete original CLI argv/formats/order/output and successful immutability predicates exact through declared adapters')


def materialization_capture(label, source=None):
    with fixture(label) as root:
        ns = module(source)
        records, _ = ns['inventory'](root)
        destination = root / 'target/copy'
        files = {destination / name for name in records}
        folders = {destination, *(parent for parent in destination.parents if parent.is_relative_to(ROOT / 'target'))}
        for path in files:
            folders.update(parent for parent in path.parents if parent == destination or parent.is_relative_to(destination))
        inputs = {ENTRY, PROFILE_SOURCE, *(path for folder in (root / '.doctrine/history_archive', root / 'docs/history')
                                        for path in folder.rglob('*') if path.is_file())}
        if ns['load_json'](root, ns['MASTER'], 8192)[0]['version'] == 3:
            inputs.add(GENERATIONS)
        saved_env = dict(os.environ)
        stores = {name: str(root / 'stores' / name.lower()) for name in PROFILE['STORES']}
        PROFILE['prepare_producer'](ROOT, sources=CORE, directories=tuple(Path(value) for value in stores.values()))
        os.environ.update(stores, ARCHIVE_READER_CALLER='preserved')
        plans, activations, writes = [], [], []
        checkpoint = 0
        original = Path.mkdir, Path.open

        def plan(project, **kwargs):
            assert project == ROOT
            if set(kwargs.get('sources', ())) == {ENTRY, PROFILE_SOURCE}:
                assert len(kwargs.get('output_files', ())) == 1, 'archive file plan omitted'
                output = kwargs['output_files'][0]
                assert output in files and set(kwargs['directories']) == {destination, output.parent}
            else:
                assert set(kwargs.get('sources', ())) == inputs, 'archive complete sources omitted'
                assert set(kwargs.get('output_files', ())) == files, 'archive complete files omitted'
                assert set(kwargs.get('directories', ())) == folders, 'archive complete directories omitted'
            plans.append(kwargs)
            return PROFILE['plan_producer'](project, **kwargs)

        def environment(project, values, *, create=True):
            assert all(values.get(name) == value for name, value in stores.items()), 'archive caller stores reset'
            assert values['ARCHIVE_READER_CALLER'] == 'preserved'
            if create:
                assert plans and set(plans[-1].get('output_files', ())) == files, 'archive activation before complete plan'
                activations.append(True)
            return PROFILE['environment_for'](project, values, create=create)

        def fresh(path, *, opening=False):
            nonlocal checkpoint
            assert activations, 'archive activation omitted'
            assert len(plans) > checkpoint, 'archive effect recheck omitted'
            checkpoint = len(plans)
            assert path in (plans[-1]['output_files'] if opening else plans[-1]['directories']), 'archive effect outside latest plan'

        def mkdir(path, *args, **kwargs):
            if path.is_relative_to(destination) or path == root / 'target':
                assert path in folders, 'archive implicit parent outside complete plan'
                if sys._getframe(1).f_globals is ns:
                    fresh(path)
            return original[0](path, *args, **kwargs)

        def open_file(path, mode='r', *args, **kwargs):
            if 'x' in mode and sys._getframe(1).f_globals is ns:
                fresh(path, opening=True)
                writes.append(path)
            return original[1](path, mode, *args, **kwargs)

        try:
            ns['load_profile'] = lambda: dict(PROFILE, plan_producer=plan, environment_for=environment)
            Path.mkdir, Path.open = mkdir, open_file
            text, _ = cli(ns, root, 'materialize', 'target/copy')
            assert 'materialized %d records' % len(records) in text
            assert len(activations) == 1 and writes == sorted(files)
            assert {path.relative_to(destination).as_posix(): path.read_bytes() for path in destination.rglob('*.md')} == records
            assert all(os.environ[name] == value for name, value in stores.items())
            assert os.environ['ARCHIVE_READER_CALLER'] == 'preserved'
        finally:
            Path.mkdir, Path.open = original
            os.environ.clear(); os.environ.update(saved_env)
        return len(files)


def metadata_cases(source=None):
    ns = module(source)
    old_env = dict(os.environ)
    saved = subprocess.run, Path.mkdir
    commands = []
    situation = 'unavailable'

    def no_write(*args, **kwargs):
        raise AssertionError('archive metadata check activated stores')

    def run(command, **kwargs):
        environment = kwargs['env']
        assert 'GIT_DIR' not in environment and environment['GIT_CONFIG_NOSYSTEM'] == '1', 'archive metadata Git scope omitted'
        assert environment['ARCHIVE_READER_CALLER'] == 'preserved'
        assert environment['TMPDIR'] == 'target/../unused' and environment['RUSTUP_TOOLCHAIN'] == 'invalid/channel'
        commands.append(command)
        if situation == 'unavailable':
            return subprocess.CompletedProcess(command, 128, b'', b'controlled unavailable HEAD')
        if command[3] == 'show':
            return subprocess.CompletedProcess(command, 128, b'', b'controlled unreadable object')
        assert command == ['git', '-C', str(ROOT), 'ls-tree', '--full-tree', '--name-only', 'HEAD', '--', ns['MASTER']]
        return subprocess.CompletedProcess(command, 0, b'' if situation == 'absent' else ns['MASTER'].encode() + b'\n')

    try:
        os.environ.update(GIT_DIR=str(WORK / 'foreign-git'), TMPDIR='target/../unused',
                          RUSTUP_TOOLCHAIN='invalid/channel', ARCHIVE_READER_CALLER='preserved')
        wanted = dict(os.environ)
        Path.mkdir, subprocess.run = no_write, run
        for situation in ('unavailable', 'present', 'absent'):
            try:
                ns['guard_committed'](ROOT)
            except ns['Refusal'] as error:
                assert situation != 'absent' and str(error) == 'committed HEAD metadata unavailable', (situation, error)
            else:
                assert situation == 'absent', 'archive metadata failure admitted'
        assert len(commands) == 6 and dict(os.environ) == wanted
    finally:
        subprocess.run, Path.mkdir = saved
        os.environ.clear(); os.environ.update(old_env)
    print('archive metadata: unavailable/present-unreadable refused; observed first-transition absence accepted; scoped readonly children')


def physical_controls():
    cases = (
        ('output-link', 'symlink path refused'), ('output-hardlink', 'destination exists'),
        ('output-directory', 'destination exists'), ('parent-link', 'symlink path refused'),
        ('parent-git', 'Git repository boundary'), ('parent-device', 'off-volume path'),
        ('profile-link', 'unsafe archive profile source'), ('entry-link', 'local regular file'),
        ('entry-missing', 'local regular file'), ('backing-link', 'symlink path refused'),
        ('backing-hardlink', 'unsafe archive file'),
        ('store-invalid', 'invalid store components'), ('store-in-output', 'contains a store'),
        ('store-in-backing', 'contains a store'), ('destination-escape', 'unsafe path'),
    )
    for label, reason in cases:
        with fixture('physical-' + label) as root:
            ns = module()
            output, parent = root / 'target/copy', root / 'target'
            victim = root / 'docs/history/window1.md'
            content = victim.read_bytes()
            alias, marker = root / 'alias.py', parent / '.git'
            saved_env = dict(os.environ)
            selected = 'target/copy'
            if label in ('output-link', 'output-hardlink', 'output-directory', 'parent-git', 'parent-device', 'backing-hardlink'):
                PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(parent,))
            if label == 'output-link':
                output.symlink_to(victim)
            elif label == 'output-hardlink':
                # Keep the archive valid: output existence and source hard-link safety are
                # independent predicates. Linking the source would trip the latter first.
                alias.write_bytes(content)
                os.link(alias, output)
            elif label == 'output-directory':
                output.mkdir()
            elif label == 'parent-link':
                parent.symlink_to(root / 'docs', target_is_directory=True)
            elif label == 'parent-git':
                marker.mkdir()
            elif label in ('profile-link', 'entry-link'):
                alias.symlink_to(PROFILE_SOURCE if label == 'profile-link' else ENTRY)
                ns['PROFILE_SOURCE' if label == 'profile-link' else '__file__'] = alias
            elif label == 'entry-missing':
                ns['__file__'] = str(alias)
            elif label == 'backing-link':
                victim.unlink(); victim.symlink_to(ROOT / 'docs/history/window1.md')
            elif label == 'backing-hardlink':
                os.link(victim, alias)
            elif label == 'store-invalid':
                os.environ['TMPDIR'] = 'target/../invalid'
            elif label == 'store-in-output':
                os.environ['TMPDIR'] = str(output / 'store')
            elif label == 'store-in-backing':
                os.environ['TMPDIR'] = str(root / 'docs/history/store')
            elif label == 'destination-escape':
                selected = 'target/../escape'
            original = Path.mkdir, Path.open, Path.stat, subprocess.run

            def no_effect(*args, **kwargs):
                raise AssertionError('archive physical refusal reached effect: ' + label)

            def open_file(path, mode='r', *args, **kwargs):
                assert not any(flag in mode for flag in 'wax+'), 'archive physical refusal wrote output'
                return original[1](path, mode, *args, **kwargs)

            def stat(path, *args, **kwargs):
                value = original[2](path, *args, **kwargs)
                if path == parent and label == 'parent-device':
                    fields = list(value); fields[2] += 1
                    return os.stat_result(fields)
                return value

            try:
                Path.mkdir, Path.open, Path.stat, subprocess.run = no_effect, open_file, stat, no_effect
                try:
                    cli(ns, root, 'materialize', selected)
                except (ValueError, OSError) as error:
                    assert reason in str(error), (label, str(error), reason)
                else:
                    raise AssertionError('archive physical refusal missing: ' + label)
            finally:
                Path.mkdir, Path.open, Path.stat, subprocess.run = original
                os.environ.clear(); os.environ.update(saved_env)
                if alias.is_symlink():
                    alias.unlink()
                elif label in ('output-hardlink', 'backing-hardlink') and alias.is_file():
                    alias.unlink()
                if victim.is_symlink():
                    victim.unlink(); victim.write_bytes(content)
                if parent.is_symlink():
                    parent.unlink()
                if marker.exists():
                    marker.rmdir()
                if output.is_symlink() or output.is_file():
                    output.unlink()
            assert victim.read_bytes() == content, label
    print('archive fifteen physical source/profile/output/store/parent refusals before effects; device observation simulated')


def omission_controls():
    source = ENTRY.read_text()
    cases = (
        ('sources', 'sources=sources, directories=tuple(sorted(folders)), output_files=files',
         'sources=(Path(__file__), PROFILE_SOURCE), directories=tuple(sorted(folders)), output_files=files', 'archive file plan omitted'),
        ('files', 'directories=tuple(sorted(folders)), output_files=files',
         'directories=tuple(sorted(folders)), output_files=()', 'archive complete files omitted'),
        ('directories', 'directories=tuple(sorted(folders)), output_files=files',
         'directories=(), output_files=files', 'archive complete directories omitted'),
        ('activation', '        materialization_plan(profile, root, destination, records, activate=True)\n', '', 'archive activation omitted'),
        ('file-recheck', "            materialization_file_plan(profile, destination, output)\n            with output.open('xb')",
         "            with output.open('xb')", 'archive effect recheck omitted'),
        ('stores', "profile['environment_for'](PROJECT, os.environ, create=True)",
         "profile['environment_for'](PROJECT, {}, create=True)", 'archive caller stores reset'),
    )
    for label, before, after, reason in cases:
        assert source.count(before) == 1, (label, 'actual archive body anchor')
        try:
            materialization_capture('fault-' + label, source.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == reason, (label, str(error), reason)
        else:
            raise AssertionError('archive body fault survived: ' + label)
    cases = (
        ("require(absence.returncode == 0 and absence.stdout == b'', 'committed HEAD metadata unavailable')", 'pass', 'archive metadata failure admitted'),
        ("if not name.startswith('GIT_')", 'if True', 'archive metadata Git scope omitted'),
    )
    for before, after, reason in cases:
        assert source.count(before) == 1, 'archive metadata body anchor'
        try:
            metadata_cases(source.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == reason, str(error)
        else:
            raise AssertionError('archive metadata fault survived')
    print('archive eight compiled actual plan/activation/recheck/store/HEAD/Git assertion reds')


def native_controls():
    with fixture('native') as root:
        ns = module()
        records, _ = ns['inventory'](root)
        destination = root / 'target/native'
        files = tuple(destination / name for name in records)
        directories = {destination, root / 'target'}
        for path in files:
            directories.update(parent for parent in path.parents if parent == destination or parent.is_relative_to(destination))
        inputs = (*CORE, *(path for folder in (root / '.doctrine/history_archive', root / 'docs/history')
                          for path in folder.rglob('*') if path.is_file()))
        frozen = {path: path.read_bytes() for path in inputs}
        environment = PROFILE['environment_for'](ROOT, os.environ, create=False)

        def command(mode, argument=None):
            PROFILE['plan_producer'](ROOT, sources=inputs, directories=tuple(directories), output_files=files)
            argv = [sys.executable, '-I', '-B', str(ENTRY), '--root', str(root), mode]
            if argument is not None:
                argv.append(argument)
            return subprocess.run(argv, cwd=ROOT, env=environment, capture_output=True)

        result = command('materialize', 'target/native')
        assert result.returncode == 0, result
        assert {path.relative_to(destination).as_posix(): path.read_bytes() for path in destination.rglob('*.md')} == records
        result = command('materialize', 'target/native')
        assert result.returncode == 1 and b'destination exists; overwrite refused' in result.stderr, result
        assert {path.relative_to(destination).as_posix(): path.read_bytes() for path in destination.rglob('*.md')} == records
        for mode, argument in (('verify-retention', None), ('prove-source', 'window11')):
            result = command(mode, argument)
            assert result.returncode == 0, result
        assert all(path.read_bytes() == content for path, content in frozen.items())
    print('archive native public CLI: all logical bytes exact, overwrite refused, committed retention/window11 source proof; sources/backing exact')


def retention_controls():
    ns = module()
    logical, _ = ns['inventory'](ROOT)
    captures = (('stitchcad-changelog-part214.md', 'CHANGELOG.md'),
                ('stitchcad-devnotes-part310.md', 'docs/tasks/G1-SLICE-producers.md'),
                ('stitchcad-devnotes-part311.md', 'docs/tasks/G1-SLICE-producers.md'))
    for name, source in captures:
        data = subprocess.check_output([sys.executable, '-I', '-B', str(ENTRY), '--root', str(ROOT),
                                        'read', 'docs/history/' + name], cwd=ROOT)
        header, body = data.split(b'\n---\n\n', 1)
        old = subprocess.check_output(['git', 'show', BEFORE + ':' + source], cwd=ROOT)
        assert old.count(body) == 1 and body.endswith(b'\n') and not body.endswith(b'\n\n')
        start = old.index(body)
        assert '%s bytes %d–%d, %d bytes, sha256:%s' % (
            source, start, start + len(body), len(body), ns['digest'](body)) in header.decode()
    backings = ns['backing_inventory'](ROOT)
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--',
                                    'docs/history', '.doctrine/history_archive'], cwd=ROOT).decode().splitlines()
    for name in paths:
        old = subprocess.check_output(['git', 'show', BEFORE + ':' + name], cwd=ROOT)
        if name == ns['MASTER']:
            windows = ns['json'].loads(old)['windows']
            assert ns['json'].loads(backings[name])['windows'][:len(windows)] == windows
        elif ns['SEGMENT'].fullmatch(name):
            assert logical[name] == old, name
        else:
            assert (backings[name] if name in backings else (ROOT / name).read_bytes()) == old, name
    print('archive exact whole predecessor task/ledgers and public reads; all previous logical/control/payload bytes preserved')


if __name__ == '__main__':
    assert sys.argv[1:] in ([], ['--before'], ['--real'])
    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(WORK,))
    frozen = {path: path.read_bytes() for path in CORE}
    before_controls()
    if sys.argv[1:] != ['--before']:
        pure_controls()
        original_cli_controls()
        print('archive complete capture:', materialization_capture('positive'), 'exact logical files')
        metadata_cases()
        physical_controls()
        omission_controls()
        if sys.argv[1:] == ['--real']:
            native_controls()
        retention_controls()
    assert all(path.read_bytes() == content for path, content in frozen.items()), 'archive reader control sources changed'
    assert not list(WORK.iterdir()), 'archive reader fixture residue'
    PROFILE['plan_producer'](ROOT, sources=CORE, directories=(WORK,))
    WORK.rmdir()
