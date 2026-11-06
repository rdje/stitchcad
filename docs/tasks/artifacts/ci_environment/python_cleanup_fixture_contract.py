"""Actual cleanup fixture plans, effects, refusals and exception restoration."""
from pathlib import Path
import ast
import contextlib
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
ENTRY = ROOT / 'docs/tasks/artifacts/artifact_cleanup/cleanup_probes.py'
SOURCE = ENTRY.with_name('cleanup.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
CORE = (ENTRY, SOURCE, PROFILE_SOURCE, ROOT / 'scripts/check_handoff.sh',
        ROOT / 'scripts/handoff_census.py', ROOT / 'README.md')
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BEFORE = '0ab74212aa1b7cb3d3afed8e1c84b6b0e9fee600'
# Reviewed through0645:0315 plus0628 proof families,0640 platform and0645 legacy roots.
INSPECTOR_BASELINE = 'a6a2b356bd6fd4a6c3753ea491e6719ec27d2085'
CONTINUITY_BASELINE = '5bde28b2f17254a31b006c5a70ba832600a59fe4'
OLD = subprocess.check_output(['git', 'show', BEFORE + ':' + ENTRY.relative_to(ROOT).as_posix()], text=True)
WORK = ROOT / 'target/cleanup-fixture-controls' / str(os.getpid())
PROFILE['prepare_producer'](ROOT, sources=(*CORE, Path(__file__)), directories=(WORK,))
FROZEN = {path: path.read_bytes() for path in CORE}


def mirror(label):
    root = WORK / label
    files = tuple(root / path.relative_to(ROOT) for path in CORE)
    directories = tuple(sorted({root, *(path.parent for path in files)}))
    PROFILE['prepare_producer'](ROOT, sources=(*CORE, Path(__file__)),
                               directories=directories, output_files=files)
    for source, destination in zip(CORE, files):
        destination.write_bytes(FROZEN[source])
    saved = dict(os.environ)
    try:
        runpy.run_path(str(root / 'scripts/local_environment.py'))['prepare_producer'](
            root, sources=files)
    finally:
        os.environ.clear()
        os.environ.update(saved)
    return root


def execute(root, script, *, git_observer=None):
    """Run the actual fixture/inspector bodies; model only read-only Git ignore dispatch."""
    entry = root / ENTRY.relative_to(ROOT)
    namespace = dict(__file__=str(entry), __name__='cleanup_fixture_control')
    original = subprocess.run

    def child(argv, *args, **kwargs):
        if argv[:1] == ['git'] and 'check-ignore' in argv:
            assert argv[:5] == ['git', '-C', str(root), 'check-ignore', '-q']
            if git_observer:
                git_observer(argv, kwargs)
            return subprocess.CompletedProcess(argv, 1 if argv[-1] == 'README.md' else 0)
        return original(argv, *args, **kwargs)

    saved = dict(os.environ)
    try:
        subprocess.run = child
        with contextlib.redirect_stdout(io.StringIO()):
            exec(compile(script, str(entry), 'exec'), namespace)
    finally:
        subprocess.run = original
        os.environ.clear()
        os.environ.update(saved)
    return namespace


def before_control():
    root = mirror('before')
    module = execute(root, OLD)
    selected = module['BASE'] / 'callback-failure'

    def fail(path):
        raise RuntimeError('controlled setup failure')

    try:
        module['case']('callback-failure', fail, lambda m, p: None, 'unused')
    except RuntimeError as error:
        assert str(error) == 'controlled setup failure'
    else:
        raise AssertionError('original setup failure missing')
    assert selected.is_dir(), 'original setup failure did not strand its owned fixture'
    PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), directories=(selected,))
    assert not any(selected.iterdir()), 'unexpected contents in original failed fixture'
    shutil.rmtree(selected)
    assert not any(module['BASE'].iterdir())
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('original callback failure strands its owned directory; fifteen original cases/eleven reds pass; sources exact')


def before_alias_control():
    root = mirror('before-alias')
    source = root / SOURCE.relative_to(ROOT)
    PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), output_files=(source,))
    source.unlink()
    source.symlink_to(SOURCE)
    original = Path.mkdir
    real_base = ROOT / 'target/scratch/artifact_cleanup_probes'

    def mkdir(path, *args, **kwargs):
        assert path != real_base, 'source alias rebased fixture root before authority'
        return original(path, *args, **kwargs)

    try:
        Path.mkdir = mkdir
        try:
            execute(root, OLD)
        except AssertionError as error:
            assert str(error) == 'source alias rebased fixture root before authority', error
        else:
            raise AssertionError('original source alias did not reach the trapped outside-mirror creation')
    finally:
        Path.mkdir = original
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('original source alias reaches trapped project-root mkdir outside its mirror; no outside write; sources exact')


def before_linked_input_control():
    root = mirror('before-linked-input')
    readme, alias = root / 'README.md', root / 'target/readme-alias.data'
    PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), output_files=(alias,))
    os.link(readme, alias)
    before = readme.read_bytes()
    try:
        execute(root, OLD)
    except AssertionError as error:
        assert 'symlink-entry' in str(error) and 'hard-linked entry:' in str(error), error
    else:
        raise AssertionError('original link-guard interference was not reproduced')
    assert readme.read_bytes() == alias.read_bytes() == before and readme.stat().st_nlink == 2
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('original symlink fault is masked by the newer link guard on a valid linked README input; aliases/sources exact')


NAMES = ('tracked', 'symlink-entry', 'symlink-parent', 'git-boundary', 'git-descendant',
         'nonignored', 'protected-store', 'escaping', 'absolute', 'special-file',
         'plan-drift', 'record-locality', 'device-parent', 'device-entry', 'valid')
FILES = ('keep', 'link', 'pipe', 'output.log')


def capture(label, script, *, alias_profile=False):
    root = mirror(label)
    entry, source, profile_source = (root / path.relative_to(ROOT) for path in (ENTRY, SOURCE, PROFILE_SOURCE))
    base = root / 'target/scratch/artifact_cleanup_probes'
    core = {root / path.relative_to(ROOT) for path in CORE}
    fixtures = {base / name for name in NAMES}
    whole_dirs = {base, *fixtures, *(path / 'nested' for path in fixtures)}
    whole_files = {path / name for path in fixtures for name in FILES}
    real = runpy.run_path(str(profile_source))
    stores = {name: str(root / 'target/caller-stores' / name.lower()) for name in real['STORES']}
    saved = dict(os.environ)
    os.environ.update(stores, CLEANUP_FIXTURE_CALLER='preserved')
    real['prepare_producer'](root, sources=tuple(core))
    if alias_profile:
        PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), output_files=(profile_source,))
        profile_source.unlink()
        profile_source.symlink_to(PROFILE_SOURCE)
    originals = (runpy.run_path, Path.mkdir, Path.write_text, Path.symlink_to,
                 os.mkfifo, Path.rmdir, Path.rglob, Path.read_text, shutil.rmtree)
    plans, activations, effects, children = [], [], [], []
    latest, discovery, writes = {}, {}, {}

    def owner():
        frame = sys._getframe(1)
        while frame:
            filename = frame.f_globals.get('__file__')
            if filename in (str(entry), str(source)):
                return Path(filename), frame
            frame = frame.f_back
        return None, None

    def plan(selected_root, **kwargs):
        who, frame = owner()
        if who == entry:
            assert selected_root == root and set(kwargs.get('sources', ())) == core, 'cleanup fixture source plan incomplete'
            assert not kwargs.get('mutable_sources'), 'cleanup fixture tracked mutation authority'
            plans.append(kwargs)
            latest.clear(); latest.update(kwargs, generation=len(plans), frame=frame)
        return real['plan_producer'](selected_root, **kwargs)

    def environment(selected_root, variables, *, create=True):
        who, _ = owner()
        if who == entry and create:
            assert latest, 'cleanup fixture initial complete plan missing'
            dirs, files = set(latest.get('directories', ())), set(latest.get('output_files', ()))
            selected = dirs - {base}
            whole = dirs == whole_dirs and files == whole_files
            one = any(dirs == {base, fixture, fixture / 'nested'} and files == {fixture / name for name in FILES}
                      for fixture in {path.parent for path in files})
            assert selected and (whole or one), 'cleanup fixture activation plan incomplete'
            activations.append(latest['generation'])
        return real['environment_for'](selected_root, variables, create=create)

    def load(path, *args, **kwargs):
        selected = Path(path)
        if selected == profile_source:
            assert not selected.is_symlink() and selected.is_file(), 'cleanup fixture unsafe first helper delegation'
            assert not selected.parent.is_symlink() and not (selected.parent / '.git').exists(), 'cleanup fixture unsafe helper parent'
            return dict(real, plan_producer=plan, environment_for=environment)
        if selected == source and owner()[0] == entry:
            assert activations and set(latest.get('directories', ())) == whole_dirs, 'cleanup fixture provider load before whole plan'
        return originals[0](path, *args, **kwargs)

    def mkdir(path, *args, **kwargs):
        if owner()[0] == entry:
            assert latest and activations, 'cleanup fixture mkdir before complete activation'
            if path == base:
                assert set(latest.get('directories', ())) == whole_dirs, 'cleanup fixture base before whole plan'
            elif path in fixtures:
                assert activations[-1] == latest['generation'] and set(latest.get('directories', ())) == {base, path, path / 'nested'}, 'cleanup fixture mkdir before selected activation'
            elif path.name == '.git':
                assert path.parent in latest.get('directories', ()), 'cleanup fixture marker outside constructor plan'
            elif path.is_relative_to(base):
                assert path in latest.get('directories', ()), 'cleanup fixture directory outside constructor plan'
            else:
                assert any(path == Path(value) or Path(value).is_relative_to(path) for value in stores.values()), 'cleanup fixture mkdir outside stores'
            effects.append(('mkdir', path))
        return originals[1](path, *args, **kwargs)

    def output(path, kind):
        assert path in latest.get('output_files', ()), 'cleanup fixture file outside complete plan'
        assert all(os.environ.get(name) == value for name, value in stores.items()), 'cleanup fixture prepared stores lost'
        if kind == 'write':
            assert latest['generation'] > writes.get(path, 0), 'cleanup fixture write recheck omitted'
            writes[path] = latest['generation']
        effects.append((kind, path))

    def write(path, *args, **kwargs):
        if owner()[0] == entry:
            output(path, 'write')
        return originals[2](path, *args, **kwargs)

    def symlink(path, *args, **kwargs):
        if owner()[0] == entry:
            output(path, 'symlink')
        return originals[3](path, *args, **kwargs)

    def fifo(path, *args, **kwargs):
        if owner()[0] == entry:
            output(Path(path), 'fifo')
        return originals[4](path, *args, **kwargs)

    def discover(path, *args, **kwargs):
        if owner()[0] == entry and path.is_relative_to(base):
            discovery[path] = latest['generation']
        return originals[6](path, *args, **kwargs)

    def rmdir(path, *args, **kwargs):
        if owner()[0] == entry:
            fixture = next(fixture for fixture in fixtures if path.is_relative_to(fixture))
            assert set(latest.get('directories', ())) == {base} and latest['generation'] > discovery[fixture], 'cleanup fixture marker removal recheck omitted'
            effects.append(('marker-remove', path))
        return originals[5](path, *args, **kwargs)

    def read(path, *args, **kwargs):
        if path == source and owner()[0] == entry:
            assert not latest.get('directories') and not latest.get('output_files'), 'cleanup fixture source read recheck omitted'
        return originals[7](path, *args, **kwargs)

    def remove(path, *args, **kwargs):
        if owner()[0] == entry:
            assert set(latest.get('directories', ())) == {path} and not latest.get('output_files'), 'cleanup fixture tree removal recheck omitted'
            effects.append(('tree-remove', path))
        return originals[8](path, *args, **kwargs)

    remove.avoids_symlink_attacks = originals[8].avoids_symlink_attacks

    def child(argv, kwargs):
        assert kwargs.get('env') is not None and all(kwargs['env'].get(name) == value for name, value in stores.items()), 'cleanup fixture Git environment lost'
        assert kwargs['env'].get('CLEANUP_FIXTURE_CALLER') == 'preserved'
        children.append(argv)

    try:
        runpy.run_path, Path.mkdir, Path.write_text, Path.symlink_to = load, mkdir, write, symlink
        os.mkfifo, Path.rmdir, Path.rglob, Path.read_text, shutil.rmtree = fifo, rmdir, discover, read, remove
        module = execute(root, script, git_observer=child)
        assert module['PASS'] == 15 and len(activations) == 16, 'cleanup fixture original population changed'
        assert not module['OWNED'] and not any(base.iterdir()), 'cleanup fixture residue'
        assert all(path.read_bytes() == data for path, data in FROZEN.items())
        print('fixture capture:', len(plans), 'plans;', len(activations), 'activations;', len(effects), 'effect calls;', len(children), 'read-only Git dispatches; original15/11 and stores exact')
    finally:
        runpy.run_path, Path.mkdir, Path.write_text, Path.symlink_to, os.mkfifo, Path.rmdir, Path.rglob, Path.read_text, shutil.rmtree = originals
        os.environ.clear(); os.environ.update(saved)


def capture_controls():
    script = ENTRY.read_text()
    capture('capture', script)
    faults = (
        ('plan', "PROFILE['plan_producer'](ROOT, sources=SOURCES, directories=directories, output_files=output_files)", 'None', 'cleanup fixture initial complete plan missing'),
        ('sources', 'sources=SOURCES, directories=directories', 'sources=(Path(__file__),), directories=directories', 'cleanup fixture source plan incomplete'),
        ('initial', '    construction_plan(activate=True)', '    producer_plan(activate=True)', 'cleanup fixture activation plan incomplete'),
        ('case-activation', '    construction_plan(fixture, activate=True)\n    fixture.mkdir()', '    fixture.mkdir()', 'cleanup fixture mkdir before selected activation'),
        ('read', '            producer_plan()\n            source = SOURCE.read_text()', '            source = SOURCE.read_text()', 'cleanup fixture source read recheck omitted'),
        ('rewrite', "    construction_plan(fixture)\n    (fixture / 'output.log').write_text('changed output\\n')", "    (fixture / 'output.log').write_text('changed output\\n')", 'cleanup fixture write recheck omitted'),
        ('marker', "        removal_plan(fixture)\n        PROFILE['require']", "        PROFILE['require']", 'cleanup fixture marker removal recheck omitted'),
        ('remove', '    producer_plan(directories=(fixture,))\n    shutil.rmtree(fixture)', '    producer_plan(directories=(BASE,))\n    shutil.rmtree(fixture)', 'cleanup fixture tree removal recheck omitted'))
    for label, before, after, reason in faults:
        assert script.count(before) == 1, ('fixture omission anchor', label)
        try:
            capture('omission-' + label, script.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == reason, (label, error)
        else:
            raise AssertionError(('fixture omission passed', label))
    print('fixture plans: eight independent actual body omission reds')
    before = 'PROFILE_SOURCE.is_symlink() or not PROFILE_SOURCE.is_file()'
    assert script.count(before) == 1
    try:
        capture('helper-delegation-red', script.replace(before, 'not PROFILE_SOURCE.is_file()', 1), alias_profile=True)
    except AssertionError as error:
        assert str(error) == 'cleanup fixture unsafe first helper delegation', error
    else:
        raise AssertionError('unsafe profile delegation omission passed')
    print('fixture bootstrap: actual profile alias guard omission stopped before helper execution')


def snapshot(root):
    result = {}
    for directory, directories, files in os.walk(root, followlinks=False):
        for name in [*directories, *files]:
            path = Path(directory) / name
            key = path.relative_to(root).as_posix()
            if path.is_symlink():
                result[key] = ('link', os.readlink(path))
            elif path.is_file():
                result[key] = ('file', hashlib.sha256(path.read_bytes()).hexdigest())
            else:
                result[key] = ('other', path.lstat().st_mode)
    return result


def physical_controls():
    labels = ('source-missing', 'source-link', 'profile-missing', 'profile-link', 'profile-directory',
              'helper-parent-link', 'handoff-missing', 'readme-missing', 'base-link', 'base-marker',
              'late-file-link', 'late-file-hardlink', 'late-file-directory', 'late-file-fifo',
              'late-directory-marker', 'store-link', 'toolchain')
    for label in labels:
        root = mirror('physical-' + label)
        entry, source, profile_source = (root / path.relative_to(ROOT) for path in (ENTRY, SOURCE, PROFILE_SOURCE))
        base = root / 'target/scratch/artifact_cleanup_probes'
        files = [root / path.relative_to(ROOT) for path in CORE]
        directories = []
        if label.startswith('late-'):
            directories.extend((base, base / 'valid'))
        if label in ('base-link', 'base-marker'):
            directories.append(base.parent)
        if label == 'base-marker':
            directories.append(base)
        if label == 'store-link':
            directories.append(root / 'target/store-controls')
            files.append(root / 'target/store-controls/scratch')
        if label.startswith('late-file-'):
            files.append(base / 'valid/output.log')
        if label == 'base-link':
            files.append(base)
        PROFILE['prepare_producer'](ROOT, sources=(*CORE, Path(__file__)),
                                   directories=tuple(directories), output_files=tuple(files))
        environment = dict(os.environ)
        environment.update({name: str(root / relative) for name, relative in PROFILE['STORES'].items()})
        if label.startswith('source-'):
            source.unlink()
            if label.endswith('link'):
                source.symlink_to(SOURCE)
        elif label.startswith('profile-'):
            profile_source.unlink()
            if label.endswith('link'):
                profile_source.symlink_to(PROFILE_SOURCE)
            elif label.endswith('directory'):
                profile_source.mkdir()
        elif label == 'helper-parent-link':
            # Only copied files in an owned mirror are removed; target is read-only.
            for path in profile_source.parent.iterdir():
                path.unlink()
            profile_source.parent.rmdir()
            profile_source.parent.symlink_to(PROFILE_SOURCE.parent, target_is_directory=True)
        elif label == 'handoff-missing':
            (root / 'scripts/check_handoff.sh').unlink()
        elif label == 'readme-missing':
            (root / 'README.md').unlink()
        elif label == 'base-link':
            base.symlink_to(ROOT / 'target', target_is_directory=True)
        elif label == 'base-marker':
            (base / '.git').mkdir()
        elif label == 'late-directory-marker':
            (base / 'valid/.git').mkdir()
        elif label.startswith('late-file-'):
            path = base / 'valid/output.log'
            if label.endswith('link') and not label.endswith('hardlink'):
                path.symlink_to(ROOT / 'README.md')
            elif label.endswith('hardlink'):
                # Alias a copied, read-only source; never link a project input as mutable output.
                os.link(root / 'README.md', path)
            elif label.endswith('directory'):
                path.mkdir()
            else:
                os.mkfifo(path)
        elif label == 'store-link':
            path = root / 'target/store-controls/scratch'
            path.symlink_to(ROOT / 'target', target_is_directory=True)
            environment['TMPDIR'] = str(path)
        elif label == 'toolchain':
            environment['RUSTUP_TOOLCHAIN'] = 'bad/toolchain'
        before = snapshot(root)
        result = subprocess.run([sys.executable, '-I', '-B', str(entry)], cwd=root,
                                env=environment, capture_output=True, text=True)
        assert result.returncode == 2 and 'cleanup fixtures refused:' in result.stderr, (label, result)
        assert snapshot(root) == before, ('fixture physical refusal changed its mirror', label)
        assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('fixture physical controls:', len(labels), 'actual CLI refusals before effects; mirrored/project inputs exact')
    root = mirror('linked-read-only-inputs')
    sources = (root / 'README.md', root / 'scripts/local_environment.py')
    aliases = (root / 'target/readme-alias.data', root / 'target/profile-alias.data')
    PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), output_files=aliases)
    for source, alias in zip(sources, aliases):
        os.link(source, alias)
    identities = tuple(hashlib.sha256(path.read_bytes()).digest() for path in sources)
    module = execute(root, ENTRY.read_text())
    assert module['PASS'] == 15
    assert tuple(hashlib.sha256(path.read_bytes()).digest() for path in sources) == identities
    assert all(source.stat().st_nlink == alias.stat().st_nlink == 2 and source.read_bytes() == alias.read_bytes()
               for source, alias in zip(sources, aliases))
    print('fixture inputs: linked read-only profile/README positive; independent hashes, aliases and link counts exact')
    script = ENTRY.read_text()
    anchor = "            if name in ('symlink-entry', 'nonignored'):"
    assert script.count(anchor) == 1
    try:
        execute(root, script.replace(anchor, '            if False:', 1))
    except AssertionError as error:
        assert 'symlink-entry' in str(error) and 'hard-linked entry:' in str(error), error
    else:
        raise AssertionError('link isolation omission passed')
    assert tuple(hashlib.sha256(path.read_bytes()).digest() for path in sources) == identities
    print('fixture link isolation: actual omission recreates the original masked fault; real inspector/source and aliases exact')


def failure_proof(root, script, *, valid_write=False):
    entry = root / ENTRY.relative_to(ROOT)
    selected = root / 'target/scratch/artifact_cleanup_probes/valid'
    if valid_write:
        original = Path.write_text

        def write(path, *args, **kwargs):
            if path == selected / 'output.log':
                raise RuntimeError('controlled first valid write failure')
            return original(path, *args, **kwargs)

        try:
            Path.write_text = write
            try:
                execute(root, script)
            except RuntimeError as error:
                assert str(error) == 'controlled first valid write failure'
            else:
                raise AssertionError('valid write failure not injected')
        finally:
            Path.write_text = original
    else:
        module = execute(root, script)
        selected = module['BASE'] / 'callback-failure'

        def setup(path):
            (path / 'output.log').write_text('owned before callback failure\n')
            (path / 'nested/.git').mkdir(parents=True)
            raise RuntimeError('controlled callback failure')

        saved = dict(os.environ)
        try:
            try:
                module['case']('callback-failure', setup, lambda m, p: None, 'unused')
            except RuntimeError as error:
                assert str(error) == 'controlled callback failure'
            else:
                raise AssertionError('callback failure not injected')
        finally:
            os.environ.clear(); os.environ.update(saved)
    assert not selected.exists(), 'cleanup fixture setup failure left owned residue'
    assert all(path.read_bytes() == data for path, data in FROZEN.items())


def restoration_controls():
    script = ENTRY.read_text()
    failure_proof(mirror('callback-restoration'), script)
    failure_proof(mirror('valid-restoration'), script, valid_write=True)
    faults = (
        ('callback', '    OWNED.add(fixture)\n    try:\n        setup(fixture)',
         '    OWNED.add(fixture)\n    setup(fixture)\n    try:', False),
        ('valid', "OWNED.add(fixture)\ntry:\n    (fixture / 'output.log').write_text('regenerable output\\n')",
         "OWNED.add(fixture)\n(fixture / 'output.log').write_text('regenerable output\\n')\ntry:", True))
    for label, before, after, valid in faults:
        assert script.count(before) == 1, ('restoration anchor', label)
        try:
            failure_proof(mirror('restoration-red-' + label), script.replace(before, after, 1), valid_write=valid)
        except AssertionError as error:
            assert str(error) == 'cleanup fixture setup failure left owned residue', (label, error)
        else:
            raise AssertionError(('fixture restoration fault passed', label))
    root = mirror('borrowed')
    module = execute(root, script)
    base = module['BASE']
    for name in ('../outside', str(ROOT / 'target/outside'), 'nested/child', '.git'):
        before = snapshot(root)
        try:
            module['case'](name, lambda path: None, lambda m, p: None, 'unused')
        except module['PROFILE']['Refusal']:
            pass
        else:
            raise AssertionError(('unsafe borrowed name accepted', name))
        assert snapshot(root) == before
    saved = dict(os.environ)
    try:
        module['case']('borrowed', lambda p: (p / 'output.log').write_text('owned'),
                       lambda m, p: m['validate_plan']({'a': 1}, {'a': 2}), 'plan drift')
        assert module['PASS'] == 16 and not module['OWNED'] and not any(base.iterdir())
    finally:
        os.environ.clear(); os.environ.update(saved)
    print('fixture restoration: two actual setup failures/two original-scope reds; four borrowed name refusals/valid borrowed case')


def ownership_controls():
    script = ENTRY.read_text()
    anchor = "    PROFILE['require'](fixture in OWNED, 'fixture removal is not owned')"
    assert script.count(anchor) == 1
    for fault in (False, True):
        root = mirror('ownership-' + str(fault))
        module = execute(root, script.replace(anchor, '    pass', 1) if fault else script)
        selected = module['BASE'] / 'unowned'
        output = selected / 'output.log'
        PROFILE['prepare_producer'](ROOT, sources=(*CORE, Path(__file__)), directories=(selected,), output_files=(output,))
        output.write_bytes(b'caller-owned sentinel\n')
        original = shutil.rmtree

        def remove(path, *args, **kwargs):
            assert path != selected, 'unowned fixture removal reached deletion'
            return original(path, *args, **kwargs)

        remove.avoids_symlink_attacks = original.avoids_symlink_attacks
        try:
            shutil.rmtree = remove
            try:
                module['remove_fixture'](selected)
            except module['PROFILE']['Refusal'] as error:
                assert not fault and 'fixture removal is not owned' in str(error)
            except AssertionError as error:
                assert fault and str(error) == 'unowned fixture removal reached deletion', error
            else:
                raise AssertionError('unowned fixture removal accepted')
        finally:
            shutil.rmtree = original
        assert output.read_bytes() == b'caller-owned sentinel\n'
    root = mirror('nonempty-marker')
    module = execute(root, script)
    selected = module['BASE'] / 'nonempty-marker'

    def setup(path):
        nested = path / 'nested'
        PROFILE['prepare_producer'](ROOT, sources=(*CORE, Path(__file__)),
                                   directories=(nested,), output_files=(nested / 'keep',))
        (nested / 'keep').write_bytes(b'owned test data; never a repository\n')
        nested.rename(path / '.git')
        raise RuntimeError('controlled nonempty marker setup')

    original, calls, saved = shutil.rmtree, [], dict(os.environ)

    def remove(path, *args, **kwargs):
        calls.append(path)
        return original(path, *args, **kwargs)

    remove.avoids_symlink_attacks = original.avoids_symlink_attacks
    try:
        shutil.rmtree = remove
        try:
            module['case']('nonempty-marker', setup, lambda m, p: None, 'unused')
        except OSError:
            assert selected in module['OWNED'] and (selected / '.git').is_dir() and not calls
        else:
            raise AssertionError('nonempty controlled marker did not stop teardown')
    finally:
        shutil.rmtree = original
        os.environ.clear(); os.environ.update(saved)
    # Remove the simulated boundary by renaming the already owned data directory;
    # no operation wrote into a Git marker and no Git repository was initialized.
    PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), directories=(module['BASE'],))
    (selected / '.git').rename(selected / 'nested')
    module['remove_fixture'](selected)
    assert not selected.exists() and not module['OWNED']
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('fixture teardown: actual unowned refusal/omission red preserve sentinel; actual nonempty fake marker stops before tree deletion')


def protected_store_controls():
    """Trap package-store traversal; test actual prefix restoration on every outcome."""
    script = ENTRY.read_text()
    root = mirror('protected-store-isolation')
    store = root / 'target/cargo-home'
    source = root / SOURCE.relative_to(ROOT)
    original_iterdir = Path.iterdir

    def iterdir(path):
        caller = sys._getframe(1)
        if path == store and caller.f_code.co_name == 'inspect' and caller.f_code.co_filename == str(source):
            raise AssertionError('protected-store mutant reached package-store traversal')
        return original_iterdir(path)

    old_operation = "     protected_store, 'protected output/store',"
    assert script.count(old_operation) == 1
    try:
        Path.iterdir = iterdir
        module = execute(root, script)
        faulty = script.replace(old_operation,
                                "     lambda m, p: m['inspect']('target/cargo-home', set()), 'protected output/store',", 1)
        try:
            execute(root, faulty)
        except AssertionError as error:
            assert 'protected-store mutant reached package-store traversal' in str(error), error
        else:
            raise AssertionError('original store-based mutation did not reach the traversal trap')
    finally:
        Path.iterdir = original_iterdir

    namespace = module['GOOD']['inspect'].__globals__
    original_prefixes = namespace['PROTECTED']
    expected = runpy.run_path(str(source))['PROTECTED']
    assert original_prefixes == expected, 'protected fixture prefix leaked after expected refusal'
    original_require = namespace['require']

    def abort(*args):
        raise RuntimeError('controlled inspector failure')

    try:
        namespace['require'] = abort
        try:
            module['protected_store'](module['GOOD'], module['BASE'] / 'protected-exception')
        except RuntimeError as error:
            assert str(error) == 'controlled inspector failure', error
        else:
            raise AssertionError('inspector failure was not injected')
    finally:
        namespace['require'] = original_require
    assert namespace['PROTECTED'] is original_prefixes, 'protected prefix not restored after exception'

    restore = "        namespace['PROTECTED'] = original"
    assert script.count(restore) == 1
    faulty = execute(mirror('protected-store-restoration-red'), script.replace(restore, '        pass', 1))
    try:
        assert faulty['GOOD']['inspect'].__globals__['PROTECTED'] == expected, 'protected fixture prefix leaked'
    except AssertionError as error:
        assert str(error) == 'protected fixture prefix leaked', error
    else:
        raise AssertionError('prefix restoration omission passed')
    assert not any(module['BASE'].iterdir()) and not any(faulty['BASE'].iterdir())
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('protected-store controls: real refusal; empty-fixture isolation/traversal-trap red; expected/unexpected restoration; actual restore-body assertion red; sources exact')


def inspector_preserved(data):
    expected = subprocess.check_output(['git', 'show', INSPECTOR_BASELINE + ':' + SOURCE.relative_to(ROOT).as_posix()])
    assert data == expected, 'fixture inspector differs from reviewed0645'


def original_body_controls():
    """Compare complete original AST after the declared plan/root/protected-setup adapters."""
    old, current = ast.parse(OLD), ast.parse(ENTRY.read_text())
    original_case = next(node for node in old.body if isinstance(node, ast.FunctionDef) and node.name == 'case')
    original_finally = next(node for node in original_case.body if isinstance(node, ast.Try)).finalbody
    original_store = next(node for node in old.body if isinstance(node, ast.Expr)
                          and isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name)
                          and node.value.func.id == 'case' and node.value.args[0].value == 'protected-store')
    real_store = ast.parse("refusal(GOOD, lambda m: m['inspect']('target/cargo-home', set()), 'protected output/store')").body[0]
    helpers = {'load_profile', 'producer_plan', 'fixture_path', 'construction_plan', 'removal_plan', 'remove_fixture',
               'protected_store'}
    added = {'PROFILE_SOURCE', 'PROFILE', 'SOURCES', 'NAMES', 'FILE_NAMES', 'OWNED'}

    class Project(ast.NodeTransformer):
        def visit_Expr(self, node):
            value = node.value
            if isinstance(value, ast.Call):
                if ast.dump(node, include_attributes=False) == ast.dump(real_store, include_attributes=False):
                    return None
                if isinstance(value.func, ast.Name) and value.func.id == 'case' and value.args[0].value == 'protected-store':
                    assert ast.dump(value.args[2], include_attributes=False) == ast.dump(ast.Name(id='protected_store', ctx=ast.Load()), include_attributes=False)
                    value.args[2] = original_store.value.args[2]
                if isinstance(value.func, ast.Name) and value.func.id in {'producer_plan', 'construction_plan'}:
                    return None
                if isinstance(value.func, ast.Attribute) and isinstance(value.func.value, ast.Name) and value.func.value.id == 'OWNED':
                    return None
            return self.generic_visit(node)

        def visit_If(self, node):
            if ast.dump(node.test, include_attributes=False) == ast.dump(ast.parse("name in ('symlink-entry', 'nonignored')", mode='eval').body, include_attributes=False):
                return None
            return self.generic_visit(node)

        def visit_FunctionDef(self, node):
            if node.name in helpers:
                return None
            self.generic_visit(node)
            if node.name == 'case':
                protected = next(item for item in node.body if isinstance(item, ast.Try))
                protected.finalbody = original_finally
            return node

        def visit_Assign(self, node):
            if any(isinstance(target, ast.Name) and target.id in added for target in node.targets):
                return None
            if any(isinstance(target, ast.Name) and target.id == 'ROOT' for target in node.targets):
                node.value = ast.parse('SOURCE.resolve().parents[4]', mode='eval').body
            return self.generic_visit(node)

        def visit_Import(self, node):
            return None if [name.name for name in node.names] == ['sys'] else node

        def visit_Try(self, node):
            if any(isinstance(item, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'PROFILE' for target in item.targets) for item in node.body):
                return None
            if any(isinstance(item, ast.Expr) and isinstance(item.value, ast.Call) and isinstance(item.value.func, ast.Name) and item.value.func.id == 'construction_plan' for item in node.body) and node.handlers:
                return None
            self.generic_visit(node)
            if node.finalbody and any(isinstance(item, ast.Expr) and isinstance(item.value, ast.Call) and isinstance(item.value.func, ast.Name) and item.value.func.id == 'remove_fixture' for item in node.finalbody):
                node.finalbody = ast.parse('shutil.rmtree(fixture)').body
            return node

    # Protect the same actual setup expressions in the old AST before comparison.
    setup = next(item for item in original_case.body if isinstance(item, ast.Expr) and isinstance(item.value, ast.Call) and isinstance(item.value.func, ast.Name) and item.value.func.id == 'setup')
    original_case.body.remove(setup)
    next(item for item in original_case.body if isinstance(item, ast.Try)).body.insert(0, setup)
    protected = next(item for item in old.body if isinstance(item, ast.Try))
    position = old.body.index(protected)
    protected.body.insert(0, old.body.pop(position - 1))
    projected = Project().visit(current)
    assert ast.dump(projected, include_attributes=False) == ast.dump(old, include_attributes=False), 'cleanup fixture original complete bodies/constants/signatures/argv changed'
    inspector_preserved(SOURCE.read_bytes())
    anchor = b"    require(not retained_proof(name), 'protected proof workspace: ' + name)\n"
    assert SOURCE.read_bytes().count(anchor) == 1
    try:
        inspector_preserved(SOURCE.read_bytes().replace(anchor, b'', 1))
    except AssertionError as error:
        assert str(error) == 'fixture inspector differs from reviewed0645'
    else:
        raise AssertionError('fixture inspector guard omission escaped exact pin')
    print('fixture original AST: complete bodies/constants/signatures/cases/argv exact through declared adapters; inspector exactly reviewed0645; actual proof-guard omission refused')
    root = mirror('root-selector')
    source = root / SOURCE.relative_to(ROOT)
    PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)), output_files=(source,))
    source.unlink(); source.symlink_to(SOURCE)
    prefix = []
    for node in ast.parse(ENTRY.read_text()).body:
        prefix.append(node)
        if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'ROOT' for target in node.targets):
            break
    for fault in (False, True):
        if fault:
            prefix[-1].value = ast.parse('SOURCE.resolve().parents[4]', mode='eval').body
        program = ast.fix_missing_locations(ast.Module(body=prefix, type_ignores=[]))
        namespace = dict(__file__=str(root / ENTRY.relative_to(ROOT)))
        exec(compile(program, namespace['__file__'], 'exec'), namespace)
        assert (namespace['ROOT'] == root) is not fault, 'fixture root selector did not distinguish unchecked dependency alias'
    print('fixture root selector: actual metadata-only entry prefix/intact and old-expression red; alias never redirects the repaired root')


def real_controls():
    PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__)))
    result = subprocess.run([sys.executable, '-I', '-B', str(ENTRY)], cwd=ROOT,
                            env=dict(os.environ), capture_output=True, text=True)
    assert result.returncode == 0 and 'cleanup probes: 15 pass / 0 fail; 11 actual compiled assertion reds; source unchanged' in result.stdout, result
    assert not any((ROOT / 'target/scratch/artifact_cleanup_probes').iterdir())
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('fixture real consumer: original15 cases/11 in-memory guard reds pass; six source inputs exact; fixture residue zero; no project cleanup apply')


def continuity_controls(*, observe=None):
    reader = ROOT / 'scripts/history_archive.py'
    task_reader = ROOT / 'docs/tasks/artifacts/live_doc_size/maintained_reference.py'
    archives = tuple(path for directory in (ROOT / 'docs/history', ROOT / '.doctrine/history_archive')
                     for path in directory.rglob('*') if path.is_file())
    PROFILE['plan_producer'](ROOT, sources=(*CORE, Path(__file__), reader, task_reader,
        ROOT / 'docs/TASK_TREE.md', ROOT / 'MEMORY.md', *archives))

    def read(path):
        if path.startswith(('docs/history/stitchcad-', '.doctrine/history_archive/', 'docs/history/payloads/')):
            operation = 'read' if path.startswith('docs/history/stitchcad-') else 'read-backing'
            result = subprocess.run([sys.executable, '-I', '-B', str(reader), '--root', str(ROOT),
                                     operation, path], cwd=ROOT, capture_output=True)
            assert result.returncode == 0, ('fixture archive read failed', path, result.stderr)
            data = result.stdout
        else:
            data = (ROOT / path).read_bytes()
        return observe(path, data) if observe else data

    for name in ('stitchcad-devnotes-part293.md', 'stitchcad-changelog-part203.md', 'stitchcad-defects-part133.md'):
        path = 'docs/history/' + name
        data = read(path)
        assert data == subprocess.check_output(['git', 'show', CONTINUITY_BASELINE + ':' + path], cwd=ROOT), 'fixture sealed record changed'
        header, body = data.split(b'\n---\n\n', 1)
        text = header.decode()
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', text)
        assert (body.count(b'\n'), len(body), hashlib.sha256(body).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3])
        chunks = []
        for source, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', text):
            original = subprocess.check_output(['git', 'show', BEFORE + ':' + source], cwd=ROOT)
            chunk = original[int(start):int(stop)]
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest
            chunks.append(chunk)
        if chunks:
            assert b''.join(chunks).rstrip(b'\n') + b'\n' == body, 'fixture whole source intervals changed'
        else:
            assert name == 'stitchcad-defects-part133.md' and all(body.count(('- **D%d**' % number).encode()) == 1 for number in (212, 213, 214))
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        previous = subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT)
        current = read(path)
        if path == '.doctrine/history_archive/windows.json':
            old, new = json.loads(previous), json.loads(current)
            assert set(new) == set(old) and new['version'] == old['version'], 'fixture archive master schema changed'
            assert new['windows'][:len(old['windows'])] == old['windows'], 'fixture prior windows changed'
            assert len(set(new['windows'])) == len(new['windows']), 'fixture duplicate window'
        else:
            assert current == previous, ('fixture prior archive bytes changed', path)
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    headers = lambda text: re.findall(r'^- ID: `([^`]+)`$', text, re.M)
    expected, current = headers(old), headers(read(tree).decode())
    assert len(current) == len(set(current)), 'fixture duplicate task node'
    assert [value for value in current if value in expected] == expected, 'fixture predecessor task graph changed'
    pointer = re.search(r'^- Next ([A-Za-z0-9_.-]+)', read('MEMORY.md').decode(), re.M)
    assert pointer, 'fixture next frontier pointer missing'
    frontier = pointer[1]
    family = frontier.split('.', 1)[0]
    index = read('docs/TASK_TREE.md').decode()
    row = next(line for line in index.splitlines() if line.startswith('| [`' + family + '`]'))
    suffix = re.search(r'`(\.[^`]+)`', row)
    assert suffix, 'fixture indexed frontier missing'
    indexed = family + suffix[1]
    assert frontier == indexed, 'fixture next frontier pointers disagree'
    tasks = runpy.run_path(str(task_reader))
    sources = tasks['current_tasks'](ROOT, frontier)
    PROFILE['plan_producer'](ROOT, sources=tuple(ROOT / name for name in sources))
    if observe:
        sources = {name: observe(name, data) for name, data in sources.items()}
    assert tasks['owner_status'](sources, frontier) is not None, 'fixture current canonical owner missing'
    old_index = subprocess.check_output(['git', 'show', BEFORE + ':docs/TASK_TREE.md'], cwd=ROOT, text=True)
    old_activity = subprocess.check_output(['git', 'show', BEFORE + ':docs/tasks/G1-SLICE.md'], cwd=ROOT, text=True)
    assert '`.5b.4c.h2.b.p.o.a21`' in next(line for line in old_index.splitlines() if line.startswith('| [`G1-SLICE`]'))
    assert '`G1-SLICE.5b.4c.h2.b.p.o.a22.2`' in next(line for line in old_activity.splitlines() if line.startswith('| current |'))
    print('fixture continuity:three exact0224 records/source intervals/public reads; prior logical bytes/windows and predecessor task order preserved')
    print('fixture frontier:current index/MEMORY/unique linked canonical owner agree; historical D34 index.a21/tree.a22.2 retained; broader PLANNING.5 stays open')


def continuity_refusals():
    """Corrupt actual public observations in memory; retained files are never amended."""
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
    record, master, tree = ('docs/history/stitchcad-devnotes-part293.md',
                           '.doctrine/history_archive/windows.json', 'docs/tasks/G1-SLICE-producers.md')
    def catalog(data, mode):
        value = json.loads(data)
        if mode == 'remove': value['windows'].pop(0)
        elif mode == 'reorder': value['windows'][:2] = reversed(value['windows'][:2])
        elif mode == 'duplicate': value['windows'].append(value['windows'][0])
        elif mode == 'schema': value['unexpected'] = True
        return json.dumps(value).encode()
    def nodes(data, mode):
        text = data.decode()
        markers = re.findall(r'^- ID: `[^`]+`$', text, re.M)
        if mode == 'remove': text = text.replace(markers[0], '', 1)
        elif mode == 'reorder':
            text = text.replace(markers[0], 'CONTROL_NODE', 1).replace(markers[1], markers[0], 1).replace('CONTROL_NODE', markers[1], 1)
        elif mode == 'duplicate': text += '\n' + markers[-1] + '\n'
        return text.encode()
    def refuses(checker, path, change, reason):
        try:
            with contextlib.redirect_stdout(io.StringIO()):
                checker(observe=lambda selected, data: change(data) if selected == path else data)
        except AssertionError as error:
            assert error.args == (reason,), (reason, error)
        else:
            raise AssertionError(('fixture continuity corruption escaped', reason))
    try:
        subprocess.run, subprocess.check_output = run, git
        continuity_controls()
        paths = git(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
        prior = next(path for path in paths if path.startswith('docs/history/stitchcad-'))
        index = (ROOT / 'docs/TASK_TREE.md').read_text()
        frontier = re.search(r'^- Next ([A-Za-z0-9_.-]+)', (ROOT / 'MEMORY.md').read_text(), re.M)[1]
        suffix = frontier[len(frontier.split('.', 1)[0]):]
        anchor = '`' + suffix + '`'
        # Only the frontier family's row: another family's frontier may share this suffix, as every gate's `.1` does.
        family_row = next(line for line in index.splitlines() if line.startswith('| [`' + frontier.split('.', 1)[0] + '`]'))
        assert family_row.count(anchor) == 1
        cases = [
            (record, lambda data: data + b'corrupt', 'fixture sealed record changed'),
            (prior, lambda data: data + b'corrupt', ('fixture prior archive bytes changed', prior)),
            *[(master, lambda data, mode=mode: catalog(data, mode), reason) for mode, reason in (
                ('remove', 'fixture prior windows changed'), ('reorder', 'fixture prior windows changed'),
                ('duplicate', 'fixture duplicate window'), ('schema', 'fixture archive master schema changed'))],
            *[(tree, lambda data, mode=mode: nodes(data, mode), reason) for mode, reason in (
                ('remove', 'fixture predecessor task graph changed'), ('reorder', 'fixture predecessor task graph changed'),
                ('duplicate', 'fixture duplicate task node'))],
            ('docs/TASK_TREE.md', lambda data: data.replace(family_row.encode(), family_row.replace(anchor, '`' + suffix + '.fault`', 1).encode(), 1),
             'fixture next frontier pointers disagree'),
        ]
        for path, change, reason in cases:
            refuses(continuity_controls, path, change, reason)
        for path in (record, master):
            selected = next(key for key in run_cache if key[-2] == path)
            result = run_cache[selected]
            run_cache[selected] = subprocess.CompletedProcess(result.args, 2, b'', b'controlled unavailable archive')
            try:
                continuity_controls()
            except AssertionError as error:
                assert error.args[0][:2] == ('fixture archive read failed', path), error
            else:
                raise AssertionError('fixture unavailable archive accepted')
            finally:
                run_cache[selected] = result
        source = Path(__file__).read_text()
        function = next(node for node in ast.parse(source).body if isinstance(node, ast.FunctionDef) and node.name == 'continuity_controls')
        body = ast.get_source_segment(source, function)
        for anchor, case in (
            ("assert current == previous, ('fixture prior archive bytes changed', path)", 1),
            ("assert new['windows'][:len(old['windows'])] == old['windows'], 'fixture prior windows changed'", 2),
            ("assert [value for value in current if value in expected] == expected, 'fixture predecessor task graph changed'", 6),
            ("assert frontier == indexed, 'fixture next frontier pointers disagree'", 9),
        ):
            assert body.count(anchor) == 1
            namespace = dict(globals())
            exec(compile(body.replace(anchor, 'pass', 1), __file__, 'exec'), namespace)
            path, change, reason = cases[case]
            try:
                refuses(namespace['continuity_controls'], path, change, reason)
            except AssertionError as error:
                assert error.args[0] == ('fixture continuity corruption escaped', reason), error
            else:
                raise AssertionError('fixture continuity guard omission missed')
    finally:
        subprocess.run, subprocess.check_output = originals
    print('fixture continuity:12 independent refusals/four compiled guard-omission assertion reds; retained data unchanged')


if __name__ == '__main__':
    if sys.argv[1:] == ['--before']:
        before_control()
        before_alias_control()
        before_linked_input_control()
    elif sys.argv[1:] == ['--real']:
        real_controls()
    elif sys.argv[1:] == ['--continuity']:
        continuity_refusals()
    else:
        assert not sys.argv[1:], 'control mode not yet implemented'
        before_control()
        before_alias_control()
        before_linked_input_control()
        capture_controls()
        protected_store_controls()
        physical_controls()
        restoration_controls()
        ownership_controls()
        original_body_controls()
        continuity_refusals()
