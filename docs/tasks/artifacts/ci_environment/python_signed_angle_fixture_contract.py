"""Actual producing-on-load signed_angle entries and pure provider; real verdicts run separately."""
from pathlib import Path
import ast
import contextlib
import hashlib
import inspect
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
ENTRY = HERE / 'signed_angle_contract.py'
WRITER = HERE / 'signed_angle_mutations.py'
PROVIDER = HERE / 'signed_angle_fixture.py'
INPUT = HERE / 'formula_input.py'
STRUCTURE = HERE / 'formula_structure.py'
SIGNATURE = HERE / 'static_signature_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
BOOK = ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_signed_angle_replay'
WRITER_WORK = ROOT / 'target/formula_signed_angle_mutations'
GOOD = runpy.run_path(str(PROFILE_SOURCE))
ENTRY_BOOT = (ENTRY, PROFILE_SOURCE, PROVIDER, INPUT)
WRITER_BOOT = (WRITER, PROFILE_SOURCE, SOURCE, ENTRY, PROVIDER)
PROVIDER_BOOT = (PROVIDER, PROFILE_SOURCE, SOURCE, ENTRY, WRITER, INPUT, STRUCTURE, SIGNATURE)
directories, files = [BOOK], []
for folder in directories:
    GOOD['directory'](ROOT, folder.relative_to(ROOT).as_posix())
    for path in sorted(folder.iterdir()):
        GOOD['require'](not path.is_symlink(), 'symlink signed_angle control book entry')
        (directories if path.is_dir() else files).append(path)
BOOK_FILES = tuple(files)
WHOLE = set((*PROVIDER_BOOT, *BOOK_FILES))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in WHOLE}
SCRIPTS = {path: FROZEN[path].decode() for path in (ENTRY, WRITER, PROVIDER, INPUT, STRUCTURE, SIGNATURE)}
BEFORE = 'e45c21d383dcd281a7a25864e1b4e77d5898f03f'
ORIGINALS = {path: subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], text=True)
             for path in (ENTRY, WRITER)}
COPIES = {WORK / path.relative_to(BOOK) for path in BOOK_FILES}
COPY_LOG = WORK / 'replay.log'
REFUSAL_LOG = WORK / 'refusal.log'
FILES = COPIES | {COPY_LOG, REFUSAL_LOG}
DIRS = {WORK / path.relative_to(BOOK) for path in directories}
LOGS = {WRITER_WORK / ('mutation-%d.log' % n) for n in range(1, 16)}


def capture(entry, *, fault=False, scripts=None, selectors=None, cli=False):
    """Execute actual modules, intercept child dispatch and keep the writable shell modeled."""
    scripts = dict(SCRIPTS) | (scripts or {})
    saved, argv = dict(os.environ), sys.argv
    stores = {name: str(ROOT / 'target/signed-plan-capture-stores' / suffix) for name, suffix in (
        ('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in stores.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    originals = (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
                 Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree)
    latest, plans, prepared, writes, children = {}, [], [], [], []
    shadow, checkpoint, edit_checkpoint, child_checkpoint, read_checkpoint, activation, tree_checkpoint = [FROZEN[SOURCE]], [0], [0], [0], [0], [0], [0]

    def owner():
        frame = sys._getframe(1)
        while frame and Path(frame.f_globals.get('__file__', '')) not in (ENTRY, WRITER, PROVIDER):
            frame = frame.f_back
        return Path(frame.f_globals['__file__']) if frame else None

    def plan(root, **kwargs):
        assert root == ROOT, 'signed_angle plan root changed'
        who = owner()
        if who == WRITER:
            sources = set(kwargs.get('sources', ()))
            if sources == set(WRITER_BOOT) and not kwargs.get('output_files'):
                assert not any(kwargs.get(key) for key in ('directories', 'output_files', 'mutable_sources')), 'signed_angle bootstrap has effects'
            elif sources == {SOURCE}:
                assert kwargs.get('mutable_sources') == (SOURCE,) and not kwargs.get('output_files') and not kwargs.get('directories'), 'signed_angle restoration authority changed'
            else:
                assert sources >= WHOLE, 'signed_angle writer sources incomplete'
                assert set(kwargs.get('output_files', ())) == FILES | LOGS, 'signed_angle nested writer files incomplete'
                assert set(kwargs.get('directories', ())) == DIRS | {WRITER_WORK}, 'signed_angle nested writer directories incomplete'
                assert kwargs.get('mutable_sources') == (SOURCE,), 'signed_angle writer authority omitted'
        before = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == before, 'signed_angle pure plan changed environment'
        plans.append((who, kwargs)); latest[who] = (kwargs, len(plans))
        return result

    def prepare(root, **kwargs):
        who = owner()
        assert latest.get(who, ({}, 0))[0] == kwargs, 'signed_angle activation before complete pure plan'
        assert set(kwargs.get('sources', ())) >= WHOLE, 'signed_angle producing sources incomplete'
        assert set(kwargs.get('output_files', ())) == FILES | (LOGS if who == WRITER else set()), 'signed_angle producing files incomplete'
        assert set(kwargs.get('directories', ())) == DIRS | ({WRITER_WORK} if who == WRITER else set()), 'signed_angle producing directories incomplete'
        assert set(kwargs.get('mutable_sources', ())) == ({SOURCE} if who == WRITER else set()), 'signed_angle authority changed'
        prepared.append(kwargs); activation[0] = len(plans)
        return GOOD['prepare_producer'](root, **kwargs)

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, prepare_producer=prepare)
        who = owner()
        assert path in scripts, 'signed_angle unexpected loader'
        if path == PROVIDER:
            boot = ENTRY_BOOT if who == ENTRY else WRITER_BOOT
            assert latest.get(who, ({}, 0))[0].get('sources') == boot, 'signed_angle loader before bootstrap'
        elif path == INPUT and who == PROVIDER:
            assert latest.get(PROVIDER, ({}, 0))[0].get('sources') == PROVIDER_BOOT, 'signed_angle context before provider bootstrap'
        else:
            assert path in latest.get(who, ({}, 0))[0].get('sources', ()), 'signed_angle delegated source unchecked'
        ns = dict(__file__=str(path), __name__='borrowed_signed_angle')
        exec(compile(scripts[path], str(path), 'exec'), ns)
        return ns

    def fresh_writer(message):
        assert latest.get(WRITER, ({}, 0))[1] > checkpoint[0] and plans[-1][0] == WRITER, message
        checkpoint[0] = len(plans)

    def read(path, binary, *args, **kwargs):
        if path == SOURCE:
            if owner() == WRITER:
                assert prepared and latest[WRITER][0].get('mutable_sources') == (SOURCE,), 'signed_angle writer read before complete plan'
            return shadow[0] if binary else shadow[0].decode()
        if path in COPIES:
            planned, generation = latest.get(PROVIDER, ({}, 0))
            assert path in planned.get('sources', ()) and generation > edit_checkpoint[0], 'signed_angle copied input recheck omitted'
            if planned.get('output_files'):
                assert COPIES <= set(planned['sources']) and COPIES <= set(planned['output_files']), 'signed_angle copied source plan omitted'
            read_checkpoint[0] = generation
        return originals[2 if binary else 1](path, *args, **kwargs)

    def authorized(path):
        assert prepared and any(path in item.get('output_files', ()) for item in prepared), 'signed_angle output before complete producing plan'
        assert path in FILES | LOGS, 'signed_angle unowned output'
        assert all(os.environ.get(name) == value for name, value in stores.items()), 'signed_angle stores lost'

    def write(path, data, binary, *args, **kwargs):
        if path == SOURCE:
            fresh_writer('signed_angle writer write recheck omitted')
            assert SOURCE in latest[WRITER][0].get('mutable_sources', ()), 'signed_angle writer authority omitted'
            if binary:
                assert set(latest[WRITER][0].get('sources', ())) == {SOURCE} and not latest[WRITER][0].get('output_files'), 'signed_angle restoration depends on unrelated paths'
            shadow[0] = data if binary else data.encode()
        else:
            authorized(path)
            if path in LOGS:
                fresh_writer('signed_angle writer log recheck omitted')
            elif path in (COPY_LOG, REFUSAL_LOG):
                planned, generation = latest.get(PROVIDER, ({}, 0))
                assert set(planned.get('output_files', ())) == FILES and generation > child_checkpoint[0], 'signed_angle copy log recheck omitted'
            else:
                planned, generation = latest.get(PROVIDER, ({}, 0))
                assert COPIES <= set(planned.get('sources', ())) and generation > max(edit_checkpoint[0], read_checkpoint[0]), 'signed_angle copied edit recheck omitted'
            originals[4 if binary else 3](path, data, *args, **kwargs)
            if path in COPIES:
                edit_checkpoint[0] = len(plans)
        writes.append(path); return len(data)

    def path_mkdir(path, *args, **kwargs):
        assert prepared, 'signed_angle mkdir before producing plan'
        return originals[5](path, *args, **kwargs)

    def mkdir(path, *args, **kwargs):
        assert prepared and Path(path) in DIRS | {WRITER_WORK}, 'signed_angle unplanned directory'
        return originals[6](path, *args, **kwargs)

    def remove(path, *args, **kwargs):
        assert prepared, 'signed_angle delete before producing plan'
        kwargs_plan, generation = latest.get(PROVIDER, ({}, 0))
        assert Path(path) == WORK and generation > activation[0] and set(kwargs_plan.get('output_files', ())) == FILES, 'signed_angle delete recheck omitted'
        tree_checkpoint[0] = generation
        return originals[8](path, *args, **kwargs)

    def copyfile(source, destination):
        source, destination = Path(source), Path(destination)
        assert source in BOOK_FILES, 'signed_angle unplanned copy source'
        authorized(destination); GOOD['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        writes.append(destination); return shutil.copy2(source, destination)

    def copytree(source, destination, *args, **kwargs):
        if Path(source) == BOOK:
            assert Path(destination) == WORK and not args and not kwargs, 'signed_angle copy interface changed'
            assert latest.get(PROVIDER, ({}, 0))[1] > max(activation[0], tree_checkpoint[0]), 'signed_angle copy recheck omitted'
            return originals[9](source, destination, copy_function=copyfile)
        bound = inspect.signature(originals[9]).bind(source, destination, *args, **kwargs); bound.apply_defaults()
        assert bound.arguments['copy_function'] is copyfile, 'signed_angle recursive copier changed'
        return originals[9](source, destination, *args, **kwargs)

    def child(arguments, *args, **kwargs):
        environment = kwargs.get('env', {})
        assert not args and kwargs.get('cwd') == ROOT and kwargs.get('capture_output') is True, 'signed_angle child interface changed'
        assert all(environment.get(name) == value for name, value in stores.items()) and environment.get('MAKE_TMPDIR') == stores['TMPDIR'], 'signed_angle child stores lost'
        assert environment.get('SIGNED_ANGLE_UNRELATED') == 'preserved', 'signed_angle unrelated environment lost'
        if entry == WRITER:
            assert arguments == [sys.executable, '-I', '-B', ENTRY.relative_to(ROOT).as_posix()], 'signed_angle writer child argv changed'
            fresh_writer('signed_angle writer child recheck omitted')
            assert shadow[0] != FROZEN[SOURCE], 'signed_angle actual source mutation absent'
            if fault:
                raise RuntimeError('signed_angle injected child failure')
            result = types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'signed_angle_contract.py AssertionError\n')
        else:
            assert arguments == ['bash', SOURCE.relative_to(ROOT).as_posix()] and kwargs.get('text') is True, 'signed_angle consumer argv changed'
            assert environment.get('FORMULA_BOOK') == str(WORK), 'signed_angle copied selector lost'
            assert all(environment.get(name) == value for name, value in (selectors or {}).items()), 'signed_angle caller selectors lost'
            assert any(who == PROVIDER and WHOLE <= set(item.get('sources', ())) for who, item in plans[edit_checkpoint[0]:]), 'signed_angle child source recheck omitted'
            parts = Path(environment.get('FORMULA_PARTS_DIR') or WORK / 'spec/formula-language')
            parts = parts if parts.is_absolute() else ROOT / parts
            selected = [Path(environment.get(name) or default) for name, default in (
                ('FORMULA_CONTRACT', WORK / 'spec/formula-language.md'), ('FORMULA_GRAMMAR', parts / 'grammar.md'),
                ('FORMULA_EXAMPLES', parts / 'examples.md'), ('FORMULA_FIXTURE', WORK / 'spec/reference-skirt.md'))]
            selected = [path if path.is_absolute() else ROOT / path for path in selected]
            references = [Path(os.path.normpath(str(path.parent) + '/' + target.split('#')[0]))
                          for path in selected[:3] for unused, target in re.findall(r'\[([^\]]*)\]\(([^)]+)\)', originals[1](path))
                          if not target.startswith(('http://', 'https://', '#'))]
            assert set(latest.get(PROVIDER, ({}, 0))[0].get('sources', ())) == set((*selected, *sorted(parts.glob('*.md')), *references)), 'signed_angle child selector recheck omitted'
            child_checkpoint[0] = len(plans)
            result = types.SimpleNamespace(returncode=0 if not children else 1, stdout='bindings: 25 · mismatches: 0\n' if not children else 'computes to -45.0, the chapter publishes 315.0\n', stderr='')
        children.append(arguments); return result

    environment = dict(saved, **stores, MAKE_TMPDIR=stores['TMPDIR'], RUSTUP_AUTO_INSTALL='0', RUSTUP_TOOLCHAIN='1.99.0', SIGNED_ANGLE_UNRELATED='preserved')
    for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE'):
        environment.pop(name, None)
    environment.update(selectors or {})
    try:
        os.environ.clear(); os.environ.update(environment); sys.argv = [str(entry), 'unrelated']; current_argv = sys.argv
        runpy.run_path = load
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.write_text = lambda path, data, *args, **kwargs: write(path, data, False, *args, **kwargs)
        Path.write_bytes = lambda path, data, *args, **kwargs: write(path, data, True, *args, **kwargs)
        Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = path_mkdir, mkdir, child, remove, copytree
        ns = dict(__file__=str(entry), __name__='__main__' if cli else 'borrowed_signed_angle')
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                exec(compile(scripts[entry], str(entry), 'exec'), ns)
                if entry == PROVIDER:
                    plan_result = ns['signed_angle_plan']()
                    assert set(plan_result['output_files']) == FILES and set(plan_result['sources']) >= WHOLE and set(plan_result['directories']) == DIRS, 'signed_angle pure plan incomplete'
            except RuntimeError as error:
                assert entry == WRITER and fault and str(error) == 'signed_angle injected child failure', 'signed_angle unrelated exception'
        assert sys.argv is current_argv and dict(os.environ) == environment, 'signed_angle borrowed state changed'
        if entry == PROVIDER:
            assert not prepared and not writes and not children, 'signed_angle pure provider produced effects'
        elif entry == ENTRY:
            assert ns['checks'] == 90 and len(children) == 2 and len(writes) == len(BOOK_FILES) + 4 and set(writes) == FILES, 'signed_angle authored entry population changed'
        else:
            assert shadow[0] == FROZEN[SOURCE], 'signed_angle modeled restoration failed'
            assert (len(children), writes.count(SOURCE), len(writes)) == ((0, 2, 2) if fault else (15, 31, 46)), 'signed_angle writer population changed'
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'signed_angle capture changed tracked sources'
        return len(plans), len(writes), len(children)
    finally:
        runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes, Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = originals
        os.environ.clear(); os.environ.update(saved); sys.argv = argv


def original_controls():
    for path in (ENTRY, WRITER):
        old, new = ast.parse(ORIGINALS[path]), ast.parse(SCRIPTS[path])
        assert sorted(ast.dump(node) for node in ast.walk(old) if isinstance(node, ast.Assert)) == sorted(ast.dump(node) for node in ast.walk(new) if isinstance(node, ast.Assert)), 'signed_angle authored assertions changed'
        if path == ENTRY:
            for node in old.body:
                if isinstance(node, ast.FunctionDef) and node.name != 'replay':
                    actual = next(item for item in ast.walk(new) if isinstance(item, ast.FunctionDef) and item.name == node.name)
                    assert ast.dump(actual) == ast.dump(node), ('signed_angle authored function changed', node.name)
        else:
            def cases(tree):
                return next(node.value for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'CASES' for target in node.targets))
            assert ast.dump(cases(old)) == ast.dump(cases(new)), 'signed_angle fifteen cases changed'
    old = ast.parse(ORIGINALS[ENTRY]).body
    new = next(node.body for node in ast.parse(SCRIPTS[ENTRY]).body if isinstance(node, ast.Try) and any(isinstance(item, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'environment' for target in item.targets) for item in node.body))
    def numerical_body(nodes):
        start = next(index for index, node in enumerate(nodes) if isinstance(node, ast.Expr) and isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Attribute) and node.value.func.attr == 'update')
        end = next(index for index, node in enumerate(nodes) if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'work' for target in node.targets))
        return [ast.dump(node) for node in nodes[start:end]]
    assert numerical_body(old) == numerical_body(new), 'signed_angle complete authored numerical body changed'
    for path, wanted in ((ENTRY, 'signed_angle delegated source unchecked'), (WRITER, 'signed_angle mkdir before producing plan')):
        try:
            capture(path, scripts={path: ORIGINALS[path]})
        except AssertionError as error:
            assert str(error) == wanted, ('signed_angle unrelated original gap', str(error))
        else:
            raise AssertionError(('signed_angle original gap escaped', path.name))
    faults = (
        (ENTRY, ENTRY, "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'signed_angle loader before bootstrap'),
        (PROVIDER, ENTRY, "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'signed_angle context before provider bootstrap'),
        (PROVIDER, ENTRY, "    PROFILE['plan_producer'](ROOT, **plan)\n    return", '    return', 'signed_angle activation before complete pure plan'),
        (PROVIDER, ENTRY, 'directories=directories, output_files=outputs, sources=sources', 'directories=directories, output_files=outputs[:-1], sources=sources', 'signed_angle producing files incomplete'),
        (PROVIDER, ENTRY, 'directories=directories, output_files=outputs, sources=sources', 'directories=directories[:-1], output_files=outputs, sources=sources', 'signed_angle producing directories incomplete'),
        (PROVIDER, ENTRY, 'directories=directories, output_files=outputs, sources=sources', 'directories=directories, output_files=outputs, sources=sources[:-1]', 'signed_angle producing sources incomplete'),
        (PROVIDER, ENTRY, "return PROFILE['prepare_producer'](ROOT, **plan) if activate else plan", "return PROFILE['environment_for'](ROOT, os.environ, create=False) if activate else plan", 'signed_angle delete before producing plan'),
        (PROVIDER, ENTRY, '    signed_angle_plan()\n    if WORK.exists()', '    if WORK.exists()', 'signed_angle delete recheck omitted'),
        (PROVIDER, ENTRY, '    signed_angle_plan()\n    shutil.copytree', '    shutil.copytree', 'signed_angle copy recheck omitted'),
        (PROVIDER, ENTRY, "    PROFILE['plan_producer'](ROOT, sources=(SOURCE, *files), output_files=files)", '    None', 'signed_angle copied input recheck omitted'),
        (ENTRY, ENTRY, "        FIXTURE['source_plan']()\n        FIXTURE['consumer_inputs']", "        FIXTURE['consumer_inputs']", 'signed_angle child source recheck omitted'),
        (ENTRY, ENTRY, "        FIXTURE['consumer_inputs'](dict(environment, FORMULA_BOOK=str(work)))\n", '', 'signed_angle child selector recheck omitted'),
        (ENTRY, ENTRY, 'env=dict(environment, FORMULA_BOOK=str(work))', "env=dict(environment, FORMULA_BOOK=str(work), TMPDIR=str(ROOT/'target/wrong-signed_angle-store'))", 'signed_angle child stores lost'),
        (ENTRY, ENTRY, "        FIXTURE['signed_angle_plan']()\n        (work / (", "        (work / (", 'signed_angle copy log recheck omitted'),
        (WRITER, WRITER, "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'signed_angle loader before bootstrap'),
        (WRITER, WRITER, "sources=tuple(dict.fromkeys((*BOOT_INPUTS, *child['sources'])))", 'sources=BOOT_INPUTS', 'signed_angle writer sources incomplete'),
        (WRITER, WRITER, "output_files=(*child['output_files'],", 'output_files=(*(),', 'signed_angle nested writer files incomplete'),
        (WRITER, WRITER, "directories=(WORK, *child['directories'])", 'directories=(WORK,)', 'signed_angle nested writer directories incomplete'),
        (WRITER, WRITER, 'mutable_sources=(SOURCE,))', 'mutable_sources=())', 'signed_angle writer authority omitted'),
        (WRITER, WRITER, "            producer_plan()\n            SOURCE.write_text", '            SOURCE.write_text', 'signed_angle writer write recheck omitted'),
        (WRITER, WRITER, '            producer_plan()\n            result =', '            result =', 'signed_angle writer child recheck omitted'),
        (WRITER, WRITER, "            producer_plan()\n            (WORK /", '            (WORK /', 'signed_angle writer log recheck omitted'),
    )
    for path, entry, anchor, replacement, wanted in faults:
        expected = 3 if anchor == 'mutable_sources=(SOURCE,))' else 1
        assert SCRIPTS[path].count(anchor) == expected, ('signed_angle guard anchor changed', anchor)
        try:
            capture(entry, scripts={path: SCRIPTS[path].replace(anchor, replacement, 1)})
        except AssertionError as error:
            assert str(error) == wanted, ('signed_angle unrelated guard red', anchor, str(error))
        else:
            raise AssertionError(('signed_angle actual guard omission escaped', anchor))
    print('signed_angle guards:', len(faults), 'actual reds/two original gaps; original assertion/function bodies/fifteen cases exact')


def mirror(label, index):
    sandbox = ROOT / ('target/signed-plan-' + label) / str(os.getpid()) / str(index)
    assert not sandbox.exists(), 'signed_angle mirror generation already exists'
    mirrors = {path: sandbox / path.relative_to(ROOT) for path in WHOLE}
    directories = {sandbox}
    for path in mirrors.values():
        directories.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
    GOOD['prepare_producer'](ROOT, sources=tuple(WHOLE), directories=tuple(directories), output_files=tuple(mirrors.values()))
    for path, destination in mirrors.items():
        shutil.copy2(path, destination)
    return sandbox, mirrors


@contextlib.contextmanager
def isolated_environment(sandbox, extra=None):
    saved, argv = dict(os.environ), sys.argv
    try:
        for key in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'RUSTUP_TOOLCHAIN'):
            os.environ.pop(key, None)
        os.environ.update({name: str(sandbox / suffix) for name, suffix in GOOD['STORES'].items()})
        os.environ.update(extra or {})
        yield
    finally:
        os.environ.clear(); os.environ.update(saved); sys.argv = argv


@contextlib.contextmanager
def blocked_effects():
    hooks = ((Path, 'mkdir'), (os, 'mkdir'), (Path, 'write_text'), (Path, 'write_bytes'),
             (subprocess, 'run'), (shutil, 'rmtree'), (shutil, 'copytree'))
    originals = [getattr(module, name) for module, name in hooks]
    def effect(*args, **kwargs):
        raise AssertionError('signed_angle physical refusal produced effects')
    try:
        for module, name in hooks:
            setattr(module, name, effect)
        yield
    finally:
        for (module, name), original in zip(hooks, originals):
            setattr(module, name, original)


def physical_controls():
    cases = ('missing', 'symlink', 'directory', 'nested-repo', 'missing-helper', 'symlink-helper',
             'symlink-book-directory', 'unexpected-old-entry', 'symlink-output', 'hardlinked-output',
             'directory-output', 'outside-selector', 'file-alias', 'directory-alias', 'symlink-store',
             'invalid-toolchain', 'hardlinked-source')
    total = borrowed = 0
    sandboxes = {}
    for entry in (ENTRY, WRITER):
        for case in cases:
            sandbox, mirrors = mirror('physical', total)
            sandboxes[entry, case] = (sandbox, mirrors)
            victim = mirrors[PROVIDER if case.endswith('helper') else SOURCE if entry == WRITER else BOOK / 'spec/units-and-tolerances.md']
            log = sandbox / COPY_LOG.relative_to(ROOT)
            extra, wanted = {}, 'producer source is not a local regular file'
            if case.startswith('missing'):
                victim.unlink()
            elif case in ('symlink', 'symlink-helper'):
                victim.unlink(); victim.symlink_to(mirrors[PROFILE_SOURCE])
                wanted = 'symlink copied-book entry' if entry == ENTRY and case == 'symlink' else 'symlink producer source'
            elif case == 'directory':
                victim.unlink(); victim.mkdir()
            elif case == 'nested-repo':
                (victim.parent / '.git').mkdir(); wanted = 'store crosses another Git repository'
            elif case == 'symlink-book-directory':
                link = sandbox / 'docs/book/src/unused-directory'
                GOOD['plan_producer'](ROOT, output_files=(link,))
                link.symlink_to(sandbox / 'docs/book/src/annexes', target_is_directory=True); wanted = 'symlink copied-book entry'
            elif case == 'unexpected-old-entry':
                sentinel = sandbox / WORK.relative_to(ROOT) / 'unexpected.txt'
                GOOD['prepare_producer'](ROOT, directories=(sentinel.parent,), output_files=(sentinel,))
                sentinel.write_text('owned signed_angle preflight sentinel\n'); wanted = 'unexpected copied-book fixture entry'
            elif case in ('symlink-output', 'hardlinked-output', 'directory-output'):
                GOOD['prepare_producer'](ROOT, directories=(log.parent,), output_files=(log,))
                if case == 'symlink-output':
                    log.symlink_to(victim); wanted = 'symlink copied-book entry'
                elif case == 'hardlinked-output':
                    os.link(victim, log); wanted = 'hard-linked producer output'
                else:
                    log.mkdir(); wanted = 'unexpected copied-book fixture entry'
            elif case == 'outside-selector':
                extra['FORMULA_CONTRACT'] = str(ENTRY); wanted = 'producer path outside workspace'
            elif case == 'file-alias':
                leaf = sandbox / WORK.relative_to(ROOT) / 'spec/formula-language.md'
                GOOD['prepare_producer'](ROOT, directories=(leaf.parent,), output_files=(leaf,))
                leaf.write_text('owned signed_angle selected input sentinel\n')
                extra['FORMULA_CONTRACT'] = str(leaf); wanted = 'copied-book output aliases selected input'
            elif case == 'directory-alias':
                parts = sandbox / WORK.relative_to(ROOT) / 'spec/formula-language'
                GOOD['directory'](ROOT, parts.relative_to(ROOT).as_posix(), create=True)
                extra.update(FORMULA_PARTS_DIR=str(parts), FORMULA_GRAMMAR=str(mirrors[BOOK / 'spec/formula-language/grammar.md']),
                             FORMULA_EXAMPLES=str(mirrors[BOOK / 'spec/formula-language/examples.md']))
                wanted = 'copied-book output aliases selected input directory'
            elif case == 'symlink-store':
                scratch = sandbox / 'target/scratch'
                GOOD['directory'](ROOT, scratch.parent.relative_to(ROOT).as_posix(), create=True)
                scratch.symlink_to(victim.parent, target_is_directory=True); wanted = 'symlink store component'
            elif case == 'invalid-toolchain':
                extra['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'; wanted = 'toolchain override must be a name'
            elif case == 'hardlinked-source':
                # Only the writer requests authority over the shell; a reader may share its inode.
                alias = sandbox / 'target/source-alias'
                GOOD['prepare_producer'](ROOT, directories=(alias.parent,), output_files=(alias,))
                os.link(mirrors[SOURCE], alias)
                if entry == ENTRY:
                    with isolated_environment(sandbox), blocked_effects():
                        namespace = runpy.run_path(str(mirrors[PROVIDER])); namespace['signed_angle_plan']()
                    total += 1
                    continue
                wanted = 'hard-linked mutable producer source'
            stderr = io.StringIO()
            with isolated_environment(sandbox, extra), blocked_effects(), contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                sys.argv = [str(mirrors[entry]), 'unrelated']
                unchanged, current_argv = dict(os.environ), sys.argv
                try:
                    runpy.run_path(str(mirrors[entry]), run_name='__main__')
                except SystemExit as error:
                    assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', (entry.name, case, stderr.getvalue())
                else:
                    raise AssertionError(('signed_angle physical refusal escaped', entry.name, case))
                if case in ('missing', 'symlink', 'directory', 'nested-repo'):
                    for unused in range(2):
                        stderr.seek(0); stderr.truncate(0)
                        try:
                            runpy.run_path(str(mirrors[entry]))
                        except SystemExit as error:
                            assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', 'signed_angle borrowed boundary changed'
                        else:
                            raise AssertionError('signed_angle producing borrowed refusal escaped')
                        borrowed += 1
                assert dict(os.environ) == unchanged and sys.argv is current_argv, 'signed_angle refused load changed state'
            total += 1
    # D202 is independently discriminated from output-file alias checks.
    sandbox, mirrors = sandboxes[WRITER, 'directory-alias']
    parts = sandbox / WORK.relative_to(ROOT) / 'spec/formula-language'
    extra = dict(FORMULA_PARTS_DIR=str(parts), FORMULA_GRAMMAR=str(mirrors[BOOK / 'spec/formula-language/grammar.md']),
                 FORMULA_EXAMPLES=str(mirrors[BOOK / 'spec/formula-language/examples.md']))
    guard = 'parts != WORK and not parts.is_relative_to(WORK)'
    assert SCRIPTS[PROVIDER].count(guard) == 1
    with isolated_environment(sandbox, extra), blocked_effects():
        ns = dict(__file__=str(mirrors[PROVIDER]), __name__='disabled_directory_guard')
        exec(compile(SCRIPTS[PROVIDER].replace(guard, 'True', 1), ns['__file__'], 'exec'), ns)
        ns['signed_angle_plan'](); assert not tuple(parts.iterdir()), 'D202 omission produced effects'
    sandbox, mirrors = sandboxes[WRITER, 'hardlinked-output']
    profile = mirrors[PROFILE_SOURCE]; text = profile.read_text()
    node = next(node for node in ast.parse(text).body if isinstance(node, ast.FunctionDef) and node.name == 'plan_producer')
    lines = text.splitlines(keepends=True)
    disabled = ''.join(lines[:node.body[0].lineno - 1]) + '    return ()\n' + ''.join(lines[node.end_lineno:])
    GOOD['plan_producer'](ROOT, sources=(profile,), mutable_sources=(profile,))
    try:
        profile.write_text(disabled)
        with isolated_environment(sandbox), blocked_effects():
            ns = runpy.run_path(str(mirrors[PROVIDER])); ns['signed_angle_plan']()
            assert (sandbox / COPY_LOG.relative_to(ROOT)).stat().st_nlink == 2
    finally:
        profile.write_text(text)
    sandbox, mirrors = mirror('pure-unused-sidebar', 0)
    sidebar = mirrors[BOOK / 'annexes/formula-static-validation.md']
    sidebar.unlink(); sidebar.symlink_to(mirrors[BOOK / 'spec/units-and-tolerances.md'])
    with isolated_environment(sandbox), blocked_effects():
        ns = runpy.run_path(str(mirrors[PROVIDER]))
        assert ns['PROFILE'] is ns['CONTEXT']['PROFILE'], 'signed_angle pure unused-sidebar provider drift'
    print('signed_angle pure provider: unused sidebar is not consumed on import; no effects')
    print('signed_angle physical:', total - 1, 'actual CLI refusals/one read-only hardlink positive;', borrowed, 'repeated producing borrowed refusals; actual D202/planner omission admissions')


def boundary_controls():
    """Independent real module loads expose Refusal-class drift that shared mocks hide."""
    def check(entry, text):
        real = runpy.run_path
        provider = []
        def load(file, *args, **kwargs):
            ns = real(file, *args, **kwargs)
            if Path(file) == PROVIDER:
                provider.append(ns)
                def fail(*args, **kwargs):
                    raise ns['PROFILE']['Refusal']('isolated delegated signed_angle refusal')
                ns['signed_angle_plan'] = fail
            return ns
        stderr, saved, argv = io.StringIO(), dict(os.environ), sys.argv
        try:
            runpy.run_path = load
            with blocked_effects(), contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                ns = dict(__file__=str(entry), __name__='__main__')
                try:
                    exec(compile(text, str(entry), 'exec'), ns)
                except SystemExit as error:
                    assert entry != PROVIDER and error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — isolated delegated signed_angle refusal\n', 'signed_angle delegated CLI refusal changed'
                except Exception as error:
                    assert provider and type(error) is provider[0]['PROFILE']['Refusal'], ('signed_angle unrelated provider failure', error)
                    raise AssertionError('signed_angle delegated refusal escaped') from None
                else:
                    assert entry == PROVIDER, 'signed_angle delegated refusal absent'
                assert ns['PROFILE'] is ns['CONTEXT' if entry == PROVIDER else 'FIXTURE']['PROFILE'], 'signed_angle actual provider identity changed'
            assert dict(os.environ) == saved and sys.argv is argv, 'signed_angle provider control changed state'
        finally:
            runpy.run_path = real; os.environ.clear(); os.environ.update(saved); sys.argv = argv
    for entry in (PROVIDER, ENTRY, WRITER):
        check(entry, SCRIPTS[entry])
        anchor = "PROFILE = %s['PROFILE']\n" % ('CONTEXT' if entry == PROVIDER else 'FIXTURE')
        assert SCRIPTS[entry].count(anchor) == 1
        try:
            check(entry, SCRIPTS[entry].replace(anchor, '', 1))
        except AssertionError as error:
            assert str(error) == ('signed_angle actual provider identity changed' if entry == PROVIDER else 'signed_angle delegated refusal escaped')
        else:
            raise AssertionError('signed_angle actual provider adoption omission escaped')
    print('signed_angle providers: actual independently loaded identities/delegated named refusals; three adoption omission reds; state exact')


def late_restore_capture(text, index):
    sandbox, mirrors = mirror('late-restore', index)
    original = FROZEN[SOURCE]
    real = subprocess.run; stderr = io.StringIO()
    log = sandbox / (WRITER_WORK / 'mutation-1.log').relative_to(ROOT)
    def child(arguments, **kwargs):
        assert arguments == [sys.executable, '-I', '-B', ENTRY.relative_to(ROOT).as_posix()], 'D207 signed_angle child interface changed'
        assert kwargs['cwd'] == sandbox and mirrors[SOURCE].read_bytes() != original, 'D207 signed_angle actual mutation absent'
        GOOD['plan_producer'](ROOT, directories=(log,)); log.mkdir()
        return types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'signed_angle_contract.py AssertionError\n')
    try:
        with isolated_environment(sandbox), contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
            subprocess.run = child
            try:
                exec(compile(text, str(mirrors[WRITER]), 'exec'), dict(__file__=str(mirrors[WRITER]), __name__='__main__'))
            except SystemExit as error:
                assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — producer output is not a local regular file\n', ('D207 unrelated signed_angle refusal', stderr.getvalue())
            else:
                raise AssertionError('D207 signed_angle late log refusal escaped')
            assert mirrors[SOURCE].read_bytes() == original, 'D207 signed_angle actual source not restored'
    finally:
        subprocess.run = real
        GOOD['plan_producer'](ROOT, sources=(mirrors[SOURCE],), mutable_sources=(mirrors[SOURCE],))
        mirrors[SOURCE].write_bytes(original)
        assert mirrors[SOURCE].read_bytes() == original and SOURCE.read_bytes() == original


def late_restore_controls():
    guard = "PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))"
    assert SCRIPTS[WRITER].count(guard) == 2
    try:
        late_restore_capture(SCRIPTS[WRITER].replace(guard, "PROFILE['plan_producer'](ROOT, **plan)"), 0)
    except AssertionError as error:
        assert str(error) == 'D207 signed_angle actual source not restored', ('D207 unrelated signed_angle red', str(error))
    else:
        raise AssertionError('D207 signed_angle restoration coupling escaped')
    late_restore_capture(SCRIPTS[WRITER], 1)
    sandbox, mirrors = mirror('nested-preflight', 0)
    sentinel = sandbox / WORK.relative_to(ROOT) / 'unexpected.txt'
    GOOD['prepare_producer'](ROOT, directories=(sentinel.parent,), output_files=(sentinel,))
    sentinel.write_text('owned signed_angle nested preflight sentinel\n')
    node = next(node for node in ast.walk(ast.parse(SCRIPTS[PROVIDER])) if isinstance(node, ast.Expr) and isinstance(node.value, ast.Call)
                and any(isinstance(argument, ast.Constant) and argument.value == 'unexpected copied-book fixture entry' for argument in node.value.args))
    lines = SCRIPTS[PROVIDER].splitlines(keepends=True)
    disabled = ''.join(lines[:node.lineno - 1]) + "    PROFILE['require'](True, 'unexpected copied-book fixture entry')\n" + ''.join(lines[node.end_lineno:])
    provider = mirrors[PROVIDER]
    GOOD['plan_producer'](ROOT, sources=(provider,), mutable_sources=(provider,))
    try:
        with isolated_environment(sandbox), blocked_effects():
            ns = runpy.run_path(str(provider))
            try:
                ns['signed_angle_plan']()
            except ns['PROFILE']['Refusal'] as error:
                assert str(error) == 'unexpected copied-book fixture entry', 'signed_angle nested unrelated refusal'
            else:
                raise AssertionError('signed_angle nested pure preflight admitted unexpected entry')
        provider.write_text(disabled)
        with isolated_environment(sandbox), blocked_effects():
            ns = runpy.run_path(str(provider)); ns['signed_angle_plan']()
            assert sentinel.read_text() == 'owned signed_angle nested preflight sentinel\n' and mirrors[SOURCE].read_bytes() == FROZEN[SOURCE]
    finally:
        provider.write_text(SCRIPTS[PROVIDER])
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'signed_angle physical controls changed tracked sources'
    print('signed_angle D207: actual mirrored source restoration red/green after late-log refusal; nested child old-entry guard refusal/omission admission; all tracked sources exact')


def continuity_controls():
    """Reconstruct retained whole intervals from Git and verify prior immutable inputs."""
    reader = ROOT / 'scripts/history_archive.py'
    records = tuple(ROOT / ('docs/history/' + name) for name in (
        'stitchcad-devnotes-part286.md', 'stitchcad-devnotes-part287.md', 'stitchcad-changelog-part198.md'))
    for record in records:
        data = record.read_bytes()
        header, content = data.split(b'\n---\n\n', 1)
        text = header.decode()
        revision = re.search(r'complete intervals from ([0-9a-f]{40})\.', text).group(1)
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', text)
        assert (content.count(b'\n'), len(content), hashlib.sha256(content).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3]), 'signed_angle retained identity drift'
        chunks = []
        for path, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', text):
            source = subprocess.check_output(['git', 'show', revision + ':' + path], cwd=ROOT)
            chunk = source[int(start):int(stop)]
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest, 'signed_angle retained source interval drift'
            chunks.append(chunk)
        assert chunks and b''.join(chunks).rstrip(b'\n') + b'\n' == content, 'signed_angle whole retention incomplete'
        result = subprocess.run([sys.executable, '-I', '-B', str(reader), '--root', str(ROOT), 'read', record.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
        assert result.returncode == 0 and result.stdout == data, 'signed_angle public retained read differs'
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT), 'signed_angle prior archive changed'
    for path in (PROFILE_SOURCE, SOURCE, INPUT, HERE / 'formula_structure.py', HERE / 'static_signature_contract.py', *(BOOK / name for name in ('spec/formula-language.md', 'spec/formula-language/grammar.md', 'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))):
        assert path.read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT), 'signed_angle original reference input changed'
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    current = (ROOT / tree).read_text()
    headers = lambda value: re.findall(r'^- ID: `([^`]+)`$', value, re.M)
    assert headers(current) == headers(old), 'signed_angle canonical task graph changed'
    print('signed_angle continuity: three complete retained records/public reads; prior archive/reference bytes and task graph exact')


def real_controls():
    """Run exclusively; observe real numerical assertions, then compare every scoped input."""
    work = ROOT / 'target/signed-plan-real'
    outputs = tuple(work / (entry.stem + '.log') for entry in (ENTRY, WRITER))
    plan = dict(sources=(*WHOLE, Path(__file__)), directories=(work,), output_files=outputs,
                mutable_sources=(SOURCE,))
    environment = GOOD['enter_producer'](ROOT, **plan)
    for entry, output in zip((ENTRY, WRITER), outputs):
        GOOD['plan_producer'](ROOT, **plan)
        result = subprocess.run([sys.executable, '-I', '-B', str(entry)], cwd=ROOT, env=environment, capture_output=True)
        GOOD['plan_producer'](ROOT, **plan)
        output.write_bytes(result.stdout + result.stderr)
        assert result.returncode == 0, (entry.name, result.returncode, result.stderr.decode())
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'signed_angle real run stranded source'
        print('signed_angle real:', entry.name, result.stdout.decode().splitlines()[-1], 'rc=0;', len(FROZEN), 'scoped inputs exact')



def main():
    for entry in (PROVIDER, ENTRY, WRITER):
        print('signed_angle capture:', entry.name, capture(entry))
    print('signed_angle child-failure capture:', capture(WRITER, fault=True))
    for entry in (ENTRY, WRITER):
        print('signed_angle CLI capture:', entry.name, capture(entry, cli=True))
    selectors = {name: str(path.relative_to(ROOT)) for name, path in (
        ('FORMULA_CONTRACT', BOOK / 'spec/formula-language.md'), ('FORMULA_GRAMMAR', BOOK / 'spec/formula-language/grammar.md'),
        ('FORMULA_EXAMPLES', BOOK / 'spec/formula-language/examples.md'), ('FORMULA_FIXTURE', BOOK / 'spec/reference-skirt.md'),
        ('FORMULA_PARTS_DIR', BOOK / 'spec/formula-language'))}
    selectors['FORMULA_ROOT'] = 'unused-root-preserved'
    print('signed_angle selector capture:', capture(ENTRY, selectors=selectors))
    original_controls()
    physical_controls()
    boundary_controls()
    late_restore_controls()
    print('signed_angle controls: actual producing imports/pure provider/original assertions/guard reds and physical refusals; real verdicts separate')


if __name__ == '__main__':
    if '--before' in sys.argv:
        for entry, wanted in ((ENTRY, 'signed_angle delegated source unchecked'), (WRITER, 'signed_angle mkdir before producing plan')):
            try:
                capture(entry, scripts={entry: ORIGINALS[entry]})
            except AssertionError as error:
                assert str(error) == wanted, error
                print('signed-angle original gap:', entry.name, wanted, 'rc=0; no original effects')
            else:
                raise AssertionError('signed-angle original gap escaped')
    elif '--real' in sys.argv:
        real_controls()
    elif '--continuity' in sys.argv:
        continuity_controls()
    else:
        main()
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'signed_angle controls changed tracked source'
