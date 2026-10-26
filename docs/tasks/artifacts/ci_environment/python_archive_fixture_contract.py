"""Actual archive-refusal fixture plans, bounded effects and protected teardown controls."""
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
ENTRY = ROOT / 'docs/tasks/artifacts/history_archive/history_archive_probes.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
READER = ROOT / 'scripts/history_archive.py'
SHELL = ROOT / 'scripts/history_archive.sh'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
LEGACY = ROOT / 'docs/tasks/artifacts/history_archive/legacy_refusals.py'
CORE = (Path(__file__), ENTRY, PROFILE_SOURCE, READER, SHELL, LEGACY)
WORK = ROOT / 'target/archive-fixture-controls' / str(os.getpid())
BEFORE = '1b8fc23'


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
            master = reader['_master_v2'](ROOT)[0]
            manifests = [reader['load_json'](ROOT, name, 65536)[0] for name in master['windows']]
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


if __name__ == '__main__':
    assert sys.argv[1:] in ([], ['--before'], ['--population'], ['--deep'])
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
