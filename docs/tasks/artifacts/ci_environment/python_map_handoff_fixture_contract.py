"""Actual map output plans and handoff readonly/producing boundary controls."""
from pathlib import Path
import contextlib
import hashlib
import io
import os
import runpy
import subprocess
import sys
from types import SimpleNamespace

ROOT = Path(__file__).resolve().parents[4]
MAP = ROOT / 'docs/tasks/artifacts/formula_structure/knowledge_map_contract.py'
HANDOFF = ROOT / 'docs/tasks/artifacts/handoff/contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
CORE = (Path(__file__), PROFILE_SOURCE, MAP, HANDOFF)
WORK = ROOT / 'target/map-handoff-fixture-controls' / str(os.getpid())
PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(WORK,))
BEFORE = 'bc730f4'


def module(entry, script=None):
    namespace = dict(__file__=str(entry), __name__='map_handoff_fixture_control')
    with contextlib.redirect_stdout(io.StringIO()):
        exec(compile(entry.read_text() if script is None else script, str(entry), 'exec'), namespace)
    return namespace


def before_controls():
    old = {entry: subprocess.check_output(['git', 'show', BEFORE + ':' + entry.relative_to(ROOT).as_posix()], text=True)
           for entry in (MAP, HANDOFF)}
    original, saved, argv = (Path.mkdir, subprocess.run), dict(os.environ), sys.argv
    seen = []

    class Trapped(Exception):
        pass

    def mkdir(path, *args, **kwargs):
        seen.append('mkdir')
        raise Trapped

    def run(command, **kwargs):
        assert command[-1] == 'calibrate' and 'env' not in kwargs
        seen.append('calibrate')
        raise Trapped

    try:
        for name in PROFILE['STORES']:
            os.environ.pop(name, None)
        ns = module(MAP, old[MAP])
        Path.mkdir = mkdir
        try:
            ns['contracts']()
        except Trapped:
            pass
        else:
            raise AssertionError('old map write not trapped')
        Path.mkdir = original[0]
        sys.argv = [str(HANDOFF), '--real']
        subprocess.run = run
        if sys.platform != 'darwin':
            Path.mkdir = mkdir
        try:
            module(HANDOFF, old[HANDOFF])
        except Trapped:
            pass
        else:
            raise AssertionError('old handoff producer not trapped')
        assert seen == ['mkdir', 'calibrate' if sys.platform == 'darwin' else 'mkdir']
    finally:
        Path.mkdir, subprocess.run = original
        sys.argv = argv
        os.environ.clear(); os.environ.update(saved)
    print('original actual map mkdir/handoff native entry trapped with stores absent; no native effect')


def map_capture(script=None):
    ns = module(MAP, script)
    real = ns['PROFILE']
    saved = dict(os.environ)
    stores = {name: str(WORK / 'stores' / name.lower()) for name in PROFILE['STORES']}
    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=tuple(Path(value) for value in stores.values()))
    os.environ.update(stores, MAP_HANDOFF_CALLER='preserved', GIT_DIR=str(WORK / 'foreign-git'))
    outputs = {ns['WORK'] / ('header_%d.%s' % (index, suffix)) for index in range(5) for suffix in ('md', 'log')}
    inputs = {MAP, PROFILE_SOURCE, ns['SOURCE'], ROOT / 'knowledge-map/subsystems.md',
              *(ROOT / 'docs/tasks').glob('*.md'), *(ROOT / 'docs/decisions').glob('*.md')}
    frozen = {path: path.read_bytes() for path in inputs}
    original = Path.write_text, subprocess.run, Path.read_bytes
    plans, activations, writes, children = [], [], [], []
    latest, checkpoint = {}, 0

    def declared(kwargs):
        assert set(kwargs.get('sources', ())) == inputs, 'map complete input plan omitted'
        assert set(kwargs.get('output_files', ())) == outputs, 'map complete output plan omitted'
        assert set(kwargs.get('directories', ())) == {ns['WORK']}, 'map directory plan omitted'

    def plan(root, **kwargs):
        assert root == ROOT
        declared(kwargs); plans.append(kwargs); latest.clear(); latest.update(kwargs)
        return real['plan_producer'](root, **kwargs)

    def prepare(root, **kwargs):
        nonlocal checkpoint
        declared(kwargs); activations.append(True)
        checkpoint = len(plans)
        return real['prepare_producer'](root, **kwargs)

    def fresh():
        nonlocal checkpoint
        assert activations and len(plans) > checkpoint, 'map effect recheck omitted'
        checkpoint = len(plans)

    def write(path, *args, **kwargs):
        if sys._getframe(1).f_globals is ns:
            fresh(); assert path in latest['output_files']
            writes.append(path)
        return original[0](path, *args, **kwargs)

    def read(path, *args, **kwargs):
        if path == ns['SOURCE'] and sys._getframe(1).f_globals is ns:
            assert path in latest.get('sources', ()), 'map source read before authority'
        return original[2](path, *args, **kwargs)

    def run(command, **kwargs):
        fresh()
        assert command == ['bash'] and kwargs['cwd'] == ROOT and isinstance(kwargs['input'], str)
        environment = kwargs['env']
        assert all(environment[key] == value for key, value in stores.items()), 'map caller stores reset'
        assert environment['MAP_HANDOFF_CALLER'] == 'preserved' and 'GIT_DIR' not in environment
        children.append(command)
        return original[1](command, **kwargs)

    try:
        ns['PROFILE'] = dict(real, plan_producer=plan, prepare_producer=prepare)
        Path.write_text, subprocess.run, Path.read_bytes = write, run, read
        ns['contracts']()
        assert len(activations) == 1 and len(writes) == 10 and len(children) == 6
        assert all(os.environ[key] == value for key, value in stores.items())
        assert os.environ['MAP_HANDOFF_CALLER'] == 'preserved' and os.environ['GIT_DIR'] == str(WORK / 'foreign-git')
        assert all(path.read_bytes() == data for path, data in frozen.items())
    finally:
        Path.write_text, subprocess.run, Path.read_bytes = original
        os.environ.clear(); os.environ.update(saved)
    print('map capture: complete inputs/ten files, six actual Bash children/ten writes/fresh plans; caller/source exact')


def handoff_readonly():
    saved, argv = dict(os.environ), sys.argv
    original = Path.mkdir, Path.write_text, subprocess.run, subprocess.Popen

    def no_effect(*args, **kwargs):
        raise AssertionError('readonly handoff activated a producer')

    try:
        os.environ.update(TMPDIR='target/../unused', RUSTUP_TOOLCHAIN='invalid/channel')
        expected = dict(os.environ)
        # Importing a fixture library never inherits a host program's --real request.
        sys.argv = ['host-program', '--real']
        Path.mkdir = Path.write_text = subprocess.run = subprocess.Popen = no_effect
        ns = module(HANDOFF)
        ns['PROFILE'] = {name: no_effect for name in ns['PROFILE']}
        ns['verify'](ns['ns'])
        assert dict(os.environ) == expected
    finally:
        Path.mkdir, Path.write_text, subprocess.run, subprocess.Popen = original
        sys.argv = argv
        os.environ.clear(); os.environ.update(saved)
    print('handoff readonly: actual49/17, imported --real ignored; pure verify; no store inspection/activation/writes/children')


def handoff_capture(script=None):
    ns = module(HANDOFF, script)
    real, saved = ns['PROFILE'], dict(os.environ)
    inputs = {HANDOFF, PROFILE_SOURCE, ns['SOURCE'], ns['HANDOFF'], ns['PROBE']}
    frozen = {path: path.read_bytes() for path in inputs}
    stores = {name: str(WORK / 'real-stores' / name.lower()) for name in PROFILE['STORES']}
    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=tuple(Path(value) for value in stores.values()))
    os.environ.update(stores, MAP_HANDOFF_CALLER='preserved')
    original = subprocess.run, subprocess.Popen
    plans, activation, calls, children = [], [], [], []
    checkpoint = 0

    def declared(kwargs):
        assert set(kwargs.get('sources', ())) == inputs, 'handoff complete sources omitted'
        assert set(kwargs.get('output_files', ())) == {ns['HELD_FILE']}, 'handoff held file plan omitted'
        assert set(kwargs.get('directories', ())) == {ROOT / 'target'}, 'handoff directory plan omitted'

    def plan(root, **kwargs):
        declared(kwargs); plans.append(kwargs)
        return real['plan_producer'](root, **kwargs)

    def prepare(root, **kwargs):
        declared(kwargs); activation.append(True)
        return real['prepare_producer'](root, **kwargs)

    def ready(kwargs):
        nonlocal checkpoint
        assert activation and len(plans) > checkpoint, 'handoff child recheck omitted'
        checkpoint = len(plans)
        environment = kwargs['env']
        assert all(environment[key] == value for key, value in stores.items()), 'handoff caller stores reset'
        assert environment['MAP_HANDOFF_CALLER'] == 'preserved' and kwargs['cwd'] == ROOT

    def run(command, **kwargs):
        ready(kwargs)
        if command[-1] == 'calibrate':
            assert command == [sys.executable, '-I', '-B', str(ns['PROBE']), 'calibrate'] and kwargs['check']
            calls.append('calibrate')
            return SimpleNamespace(returncode=0)
        assert command == ['bash', 'scripts/check_handoff.sh', '--idle-cua']
        ordinal = len([call for call in calls if call != 'calibrate'])
        calls.append('census')
        return SimpleNamespace(returncode=1 if ordinal == 1 else 0,
                               stdout='PID 600001: repo handle' if ordinal == 1 else 'handoff: OK', stderr='')

    def popen(command, **kwargs):
        ready(kwargs)
        assert command[:4] == [sys.executable, '-I', '-B', '-c']
        assert command[4] == "import os,sys; f=open('target/handoff-real-handle.txt','w'); print(os.getpid(),flush=True); sys.stdin.read()"
        assert kwargs['stdin'] == kwargs['stdout'] == subprocess.PIPE and kwargs['text'] is True
        child = SimpleNamespace(stdin=io.StringIO(), stdout=io.StringIO('600001\n'), returncode=None)

        def wait(timeout):
            assert timeout == 10 and child.stdin.closed
            child.returncode = 0

        child.wait = wait; children.append(child)
        return child

    try:
        ns['PROFILE'] = dict(real, plan_producer=plan, prepare_producer=prepare)
        subprocess.run, subprocess.Popen = run, popen
        ns['real_contract']()
        assert len(activation) == 1 and len(children) == 1 and calls.count('census') == 3
        assert calls.count('calibrate') == (sys.platform == 'darwin')
        assert children[0].stdin.closed and children[0].stdout.closed and children[0].returncode == 0
        assert all(path.read_bytes() == data for path, data in frozen.items())
        assert all(os.environ[key] == value for key, value in stores.items())
    finally:
        subprocess.run, subprocess.Popen = original
        os.environ.clear(); os.environ.update(saved)
    print('handoff real capture: complete sources/held file/store plans; exact argv; intercepted calibration/three censuses/owned child closure')


def headerless_current(script=None):
    ns = module(MAP, script)
    curated = ROOT / 'knowledge-map/subsystems.md'
    original = Path.read_text
    text = curated.read_text()
    assert text.startswith('<!--') and '-->\n' in text
    body = text.split('-->\n', 1)[1]

    def read(path, *args, **kwargs):
        value = original(path, *args, **kwargs)
        return body if path == curated and sys._getframe(1).f_globals is ns else value

    try:
        Path.read_text = read
        ns['contracts']()
    finally:
        Path.read_text = original
    print('map current-source metamorphic control: removing a header preserves its complete existing body; no source edit')


def reference_headers():
    ns = module(MAP)
    for header in ns['HEADERS']:
        expected = ('\n' if header.startswith('\n') else '') + ns['BODY']
        assert ns['curated_body'](header + ns['BODY']) == expected
    print('map current-body reference: all five independently specified header shapes')


def physical_controls():
    files = (MAP, HANDOFF, PROFILE_SOURCE, ROOT / 'scripts/handoff_census.py', ROOT / 'scripts/check_handoff.sh',
             ROOT / 'docs/tasks/artifacts/handoff/d217_probe.py', ROOT / 'knowledge-map/scripts/gen_knowledge_map.sh',
             ROOT / 'knowledge-map/subsystems.md')
    cases = ('profile-link', 'source-link', 'source-missing', 'source-marker', 'output-link', 'output-hardlink',
             'output-directory', 'output-parent-link', 'output-device', 'toolchain')
    for family, entry in (('map', MAP), ('handoff', HANDOFF)):
        for label in cases:
            root = WORK / ('physical-' + family + '-' + label)
            destinations = tuple(root / path.relative_to(ROOT) for path in files)
            directories = {path.parent for path in destinations} | {root / 'docs/tasks', root / 'docs/decisions', root / 'target'}
            PROFILE['prepare_producer'](ROOT, sources=(*CORE, *files), directories=tuple(directories), output_files=destinations)
            for source, target in zip(files, destinations):
                target.write_bytes(source.read_bytes())
            selected_entry = root / entry.relative_to(ROOT)
            ns = dict(__file__=str(selected_entry), __name__='physical_map_handoff')
            code = selected_entry.read_text()
            with contextlib.redirect_stdout(io.StringIO()):
                exec(compile(code, str(selected_entry), 'exec'), ns)
            if family == 'map':
                ns['plan']()
            output = root / ('target/knowledge_map_contract/header_4.log' if family == 'map' else 'target/handoff-real-handle.txt')
            victim = root / 'target/victim.txt'
            PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(output.parent,), output_files=(victim,))
            victim.write_text('preserve this owned fixture')
            source = root / ('knowledge-map/scripts/gen_knowledge_map.sh' if family == 'map' else 'scripts/handoff_census.py')
            if label == 'profile-link':
                selected = root / 'scripts/local_environment.py'; selected.unlink(); selected.symlink_to(PROFILE_SOURCE)
            elif label == 'source-link':
                original_source = ROOT / source.relative_to(root)
                source.unlink(); source.symlink_to(original_source)
            elif label == 'source-missing':
                source.unlink()
            elif label == 'source-marker':
                (source.parent / '.git').mkdir()
            elif label == 'output-link':
                output.symlink_to(victim)
            elif label == 'output-hardlink':
                os.link(victim, output)
            elif label == 'output-directory':
                output.mkdir()
            elif label == 'output-parent-link':
                # For handoff this selects target itself; move the known fixture first.
                other = root / 'other-outputs'
                other.mkdir()
                if output.parent == root / 'target':
                    victim.unlink()
                output.parent.rmdir(); output.parent.symlink_to(other, target_is_directory=True)
            elif label == 'output-device':
                output.write_text('controlled device observation')
            saved = dict(os.environ)
            if label == 'toolchain':
                os.environ['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'
            before = tree_snapshot(root)
            originals = Path.mkdir, Path.write_text, Path.stat, subprocess.run, subprocess.Popen

            def no_effect(*args, **kwargs):
                raise AssertionError('map/handoff refusal reached an effect')

            def stat(path, *args, **kwargs):
                value = originals[2](path, *args, **kwargs)
                if path == output:
                    parts = list(value); parts[2] += 1
                    return os.stat_result(parts)
                return value

            try:
                Path.mkdir = Path.write_text = subprocess.run = subprocess.Popen = no_effect
                if label == 'output-device':
                    Path.stat = stat
                with contextlib.redirect_stdout(io.StringIO()):
                    try:
                        ns = dict(__file__=str(selected_entry), __name__='physical_map_handoff')
                        exec(compile(code, str(selected_entry), 'exec'), ns)
                        ns['contracts']() if family == 'map' else ns['real_contract']()
                    except (ValueError, OSError, SystemExit):
                        pass
                    else:
                        raise AssertionError(('map/handoff physical refusal missing', family, label))
                assert tree_snapshot(root) == before, (family, label, 'refusal changed fixture')
            finally:
                Path.mkdir, Path.write_text, Path.stat, subprocess.run, subprocess.Popen = originals
                os.environ.clear(); os.environ.update(saved)
    print('map/handoff physical: twenty source/profile/output/toolchain refusals before effects; device observations simulated; mirrors exact')


def tree_snapshot(root):
    return {path.relative_to(root).as_posix(): ('link', os.readlink(path)) if path.is_symlink()
            else ('file', path.read_bytes()) if path.is_file() else ('directory',)
            for path in root.rglob('*')}


def omission_controls():
    source = MAP.read_text()
    faults = (
        ('read-source', '    plan()\n    original = SOURCE.read_bytes()', '    original = SOURCE.read_bytes()', 'map source read before authority'),
        ('sources', 'sources=inputs, directories=(WORK,), output_files=outputs',
         'sources=(Path(__file__), PROFILE_SOURCE), directories=(WORK,), output_files=outputs', 'map complete input plan omitted'),
        ('files', 'sources=inputs, directories=(WORK,), output_files=outputs',
         'sources=inputs, directories=(WORK,), output_files=()', 'map complete output plan omitted'),
        ('directory', 'directories=(WORK,), output_files=outputs',
         'directories=(), output_files=outputs', 'map directory plan omitted'),
        ('write-recheck', '        plan()\n        path.write_text', '        path.write_text', 'map effect recheck omitted'),
    )
    for label, before, after, reason in faults:
        assert source.count(before) == 1, label
        try:
            map_capture(source.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == reason, (label, error)
        else:
            raise AssertionError(('map actual omission escaped', label))
    source = HANDOFF.read_text()
    faults = (
        ('sources', 'sources=sources, directories=(ROOT / \'target\',), output_files=(HELD_FILE,)',
         'sources=(Path(__file__), PROFILE_SOURCE), directories=(ROOT / \'target\',), output_files=(HELD_FILE,)', 'handoff complete sources omitted'),
        ('file', 'output_files=(HELD_FILE,)', 'output_files=()', 'handoff held file plan omitted'),
        ('directory', 'directories=(ROOT / \'target\',),', 'directories=(),', 'handoff directory plan omitted'),
        ('held-recheck', '    environment = real_plan()\n    child = subprocess.Popen', '    child = subprocess.Popen', 'handoff child recheck omitted'),
    )
    for label, before, after, reason in faults:
        assert source.count(before) == 1, label
        try:
            handoff_capture(source.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == reason, (label, error)
        else:
            raise AssertionError(('handoff actual omission escaped', label))
    print('map/handoff: nine compiled actual source/file/directory/read/recheck omission assertion reds; no native fault')


if __name__ == '__main__':
    if sys.argv[1:] == ['--before']:
        before_controls()
    elif sys.argv[1:] == ['--headerless']:
        headerless_current()
    else:
        assert not sys.argv[1:]
        before_controls()
        map_capture()
        reference_headers()
        headerless_current()
        handoff_readonly()
        handoff_capture()
        physical_controls()
        omission_controls()
