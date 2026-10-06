"""Actual producing-on-load binding entries and pure provider; real verdicts run separately."""
from pathlib import Path
import ast
import contextlib
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
ENTRY = HERE / 'binding_replay_contract.py'
WRITER = HERE / 'binding_replay_mutations.py'
PROVIDER = HERE / 'binding_replay_fixture.py'
INPUT = HERE / 'formula_input.py'
STRUCTURE = HERE / 'formula_structure.py'
SIGNATURE = HERE / 'static_signature_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
BOOK = ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_binding_replay'
WRITER_WORK = ROOT / 'target/formula_binding_replay_mutations'
GOOD = runpy.run_path(str(PROFILE_SOURCE))
ENTRY_BOOT = (ENTRY, PROFILE_SOURCE, PROVIDER, INPUT)
WRITER_BOOT = (WRITER, PROFILE_SOURCE, SOURCE, ENTRY, PROVIDER)
PROVIDER_BOOT = (PROVIDER, PROFILE_SOURCE, SOURCE, ENTRY, WRITER, INPUT, STRUCTURE, SIGNATURE)
directories, files = [BOOK], []
for folder in directories:
    GOOD['directory'](ROOT, folder.relative_to(ROOT).as_posix())
    for path in sorted(folder.iterdir()):
        GOOD['require'](not path.is_symlink(), 'symlink binding control book entry')
        (directories if path.is_dir() else files).append(path)
BOOK_FILES = tuple(files)
WHOLE = set((*PROVIDER_BOOT, *BOOK_FILES))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in WHOLE}
SCRIPTS = {path: FROZEN[path].decode() for path in (ENTRY, WRITER, PROVIDER, INPUT, STRUCTURE, SIGNATURE)}
BEFORE = '78ebaa7f88b5401128818fb4adef184767e4788f'
ORIGINALS = {path: subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], text=True)
             for path in (ENTRY, WRITER)}
COPIES = {WORK / path.relative_to(BOOK) for path in BOOK_FILES}
COPY_LOG = WORK / 'replay.log'
FILES = COPIES | {COPY_LOG}
DIRS = {WORK / path.relative_to(BOOK) for path in directories}
LOGS = {WRITER_WORK / ('mutation-%d.log' % n) for n in range(1, 10)}


def capture(entry, *, fault=False, scripts=None, selectors=None, cli=False):
    """Execute actual modules, intercept child dispatch and keep the writable shell modeled."""
    scripts = dict(SCRIPTS) | (scripts or {})
    saved, argv = dict(os.environ), sys.argv
    stores = {name: str(ROOT / 'target/replay-plan-capture-stores' / suffix) for name, suffix in (
        ('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in stores.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    originals = (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
                 Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree)
    latest, plans, prepared, writes, children = {}, [], [], [], []
    shadow, checkpoint, edit_checkpoint, child_checkpoint, activation = [FROZEN[SOURCE]], [0], [0], [0], [0]

    def owner():
        frame = sys._getframe(1)
        while frame and Path(frame.f_globals.get('__file__', '')) not in (ENTRY, WRITER, PROVIDER):
            frame = frame.f_back
        return Path(frame.f_globals['__file__']) if frame else None

    def plan(root, **kwargs):
        assert root == ROOT, 'binding replay plan root changed'
        who = owner()
        if who == WRITER:
            sources = set(kwargs.get('sources', ()))
            if sources == set(WRITER_BOOT) and not kwargs.get('output_files'):
                assert not any(kwargs.get(key) for key in ('directories', 'output_files', 'mutable_sources')), 'binding replay bootstrap has effects'
            elif sources == {SOURCE}:
                assert kwargs.get('mutable_sources') == (SOURCE,) and not kwargs.get('output_files') and not kwargs.get('directories'), 'binding replay restoration authority changed'
            else:
                assert sources >= WHOLE, 'binding replay writer sources incomplete'
                assert set(kwargs.get('output_files', ())) == FILES | LOGS, 'binding replay nested writer files incomplete'
                assert set(kwargs.get('directories', ())) == DIRS | {WRITER_WORK}, 'binding replay nested writer directories incomplete'
                assert kwargs.get('mutable_sources') == (SOURCE,), 'binding replay writer authority omitted'
        before = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == before, 'binding replay pure plan changed environment'
        plans.append((who, kwargs)); latest[who] = (kwargs, len(plans))
        return result

    def prepare(root, **kwargs):
        who = owner()
        assert latest.get(who, ({}, 0))[0] == kwargs, 'binding replay activation before complete pure plan'
        assert set(kwargs.get('sources', ())) >= WHOLE, 'binding replay producing sources incomplete'
        assert set(kwargs.get('output_files', ())) == FILES | (LOGS if who == WRITER else set()), 'binding replay producing files incomplete'
        assert set(kwargs.get('directories', ())) == DIRS | ({WRITER_WORK} if who == WRITER else set()), 'binding replay producing directories incomplete'
        assert set(kwargs.get('mutable_sources', ())) == ({SOURCE} if who == WRITER else set()), 'binding replay authority changed'
        prepared.append(kwargs); activation[0] = len(plans)
        return GOOD['prepare_producer'](root, **kwargs)

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, prepare_producer=prepare)
        who = owner()
        assert path in scripts, 'binding replay unexpected loader'
        if path == PROVIDER:
            boot = ENTRY_BOOT if who == ENTRY else WRITER_BOOT
            assert latest.get(who, ({}, 0))[0].get('sources') == boot, 'binding replay loader before bootstrap'
        elif path == INPUT and who == PROVIDER:
            assert latest.get(PROVIDER, ({}, 0))[0].get('sources') == PROVIDER_BOOT, 'binding replay context before provider bootstrap'
        else:
            assert path in latest.get(who, ({}, 0))[0].get('sources', ()), 'binding replay delegated source unchecked'
        ns = dict(__file__=str(path), __name__='borrowed_binding')
        exec(compile(scripts[path], str(path), 'exec'), ns)
        return ns

    def fresh_writer(message):
        assert latest.get(WRITER, ({}, 0))[1] > checkpoint[0] and plans[-1][0] == WRITER, message
        checkpoint[0] = len(plans)

    def read(path, binary, *args, **kwargs):
        if path == SOURCE:
            if owner() == WRITER:
                assert prepared and latest[WRITER][0].get('mutable_sources') == (SOURCE,), 'binding replay writer read before complete plan'
            return shadow[0] if binary else shadow[0].decode()
        if path in COPIES:
            planned, generation = latest.get(PROVIDER, ({}, 0))
            assert path in planned.get('sources', ()) and generation > edit_checkpoint[0], 'binding replay copied input recheck omitted'
            if planned.get('output_files'):
                assert COPIES <= set(planned['sources']) and COPIES <= set(planned['output_files']), 'binding replay copied source plan omitted'
        return originals[2 if binary else 1](path, *args, **kwargs)

    def authorized(path):
        assert prepared and any(path in item.get('output_files', ()) for item in prepared), 'binding replay output before complete producing plan'
        assert path in FILES | LOGS, 'binding replay unowned output'
        assert all(os.environ.get(name) == value for name, value in stores.items()), 'binding replay stores lost'

    def write(path, data, binary, *args, **kwargs):
        if path == SOURCE:
            fresh_writer('binding replay writer write recheck omitted')
            assert SOURCE in latest[WRITER][0].get('mutable_sources', ()), 'binding replay writer authority omitted'
            if binary:
                assert set(latest[WRITER][0].get('sources', ())) == {SOURCE} and not latest[WRITER][0].get('output_files'), 'binding replay restoration depends on unrelated paths'
            shadow[0] = data if binary else data.encode()
        else:
            authorized(path)
            if path in LOGS:
                fresh_writer('binding replay writer log recheck omitted')
            elif path == COPY_LOG:
                planned, generation = latest.get(PROVIDER, ({}, 0))
                assert set(planned.get('output_files', ())) == FILES and generation > child_checkpoint[0], 'binding replay copy log recheck omitted'
            else:
                planned, generation = latest.get(PROVIDER, ({}, 0))
                assert COPIES <= set(planned.get('sources', ())) and generation > edit_checkpoint[0], 'binding replay copied edit recheck omitted'
            originals[4 if binary else 3](path, data, *args, **kwargs)
            if path in COPIES:
                edit_checkpoint[0] = len(plans)
        writes.append(path); return len(data)

    def path_mkdir(path, *args, **kwargs):
        assert prepared, 'binding replay mkdir before producing plan'
        return originals[5](path, *args, **kwargs)

    def mkdir(path, *args, **kwargs):
        assert prepared and Path(path) in DIRS | {WRITER_WORK}, 'binding replay unplanned directory'
        return originals[6](path, *args, **kwargs)

    def remove(path, *args, **kwargs):
        assert prepared, 'binding replay delete before producing plan'
        kwargs_plan, generation = latest.get(PROVIDER, ({}, 0))
        assert Path(path) == WORK and generation > activation[0] and set(kwargs_plan.get('output_files', ())) == FILES, 'binding replay delete recheck omitted'
        return originals[8](path, *args, **kwargs)

    def copyfile(source, destination):
        source, destination = Path(source), Path(destination)
        assert source in BOOK_FILES, 'binding replay unplanned copy source'
        authorized(destination); GOOD['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        writes.append(destination); return shutil.copy2(source, destination)

    def copytree(source, destination, *args, **kwargs):
        if Path(source) == BOOK:
            assert Path(destination) == WORK and not args and not kwargs, 'binding replay copy interface changed'
            assert latest.get(PROVIDER, ({}, 0))[1] > activation[0], 'binding replay delete recheck omitted'
            return originals[9](source, destination, copy_function=copyfile)
        bound = inspect.signature(originals[9]).bind(source, destination, *args, **kwargs); bound.apply_defaults()
        assert bound.arguments['copy_function'] is copyfile, 'binding replay recursive copier changed'
        return originals[9](source, destination, *args, **kwargs)

    def child(arguments, *args, **kwargs):
        environment = kwargs.get('env', {})
        assert not args and kwargs.get('cwd') == ROOT and kwargs.get('capture_output') is True, 'binding replay child interface changed'
        assert all(environment.get(name) == value for name, value in stores.items()) and environment.get('MAKE_TMPDIR') == stores['TMPDIR'], 'binding replay child stores lost'
        assert environment.get('BINDING_UNRELATED') == 'preserved', 'binding replay unrelated environment lost'
        if entry == WRITER:
            assert arguments == [sys.executable, '-I', '-B', ENTRY.relative_to(ROOT).as_posix()], 'binding replay writer child argv changed'
            fresh_writer('binding replay writer child recheck omitted')
            assert shadow[0] != FROZEN[SOURCE], 'binding replay actual source mutation absent'
            if fault:
                raise RuntimeError('binding replay injected child failure')
            result = types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'binding_replay_contract.py AssertionError\n')
        else:
            assert arguments == ['bash', SOURCE.relative_to(ROOT).as_posix()] and kwargs.get('text') is True, 'binding replay consumer argv changed'
            assert environment.get('FORMULA_BOOK') == str(WORK), 'binding replay copied selector lost'
            assert all(environment.get(name) == value for name, value in (selectors or {}).items()), 'binding replay caller selectors lost'
            assert any(who == PROVIDER and WHOLE <= set(item.get('sources', ())) for who, item in plans[edit_checkpoint[0]:]), 'binding replay child source recheck omitted'
            parts = Path(environment.get('FORMULA_PARTS_DIR') or WORK / 'spec/formula-language')
            parts = parts if parts.is_absolute() else ROOT / parts
            selected = [Path(environment.get(name) or default) for name, default in (
                ('FORMULA_CONTRACT', WORK / 'spec/formula-language.md'), ('FORMULA_GRAMMAR', parts / 'grammar.md'),
                ('FORMULA_EXAMPLES', parts / 'examples.md'), ('FORMULA_FIXTURE', WORK / 'spec/reference-skirt.md'))]
            selected = [path if path.is_absolute() else ROOT / path for path in selected]
            references = [Path(os.path.normpath(str(path.parent) + '/' + target.split('#')[0]))
                          for path in selected[:3] for unused, target in re.findall(r'\[([^\]]*)\]\(([^)]+)\)', originals[1](path))
                          if not target.startswith(('http://', 'https://', '#'))]
            assert set(latest.get(PROVIDER, ({}, 0))[0].get('sources', ())) == set((*selected, *sorted(parts.glob('*.md')), *references)), 'binding replay child selector recheck omitted'
            child_checkpoint[0] = len(plans)
            frame = sys._getframe(1)
            while frame and not (Path(frame.f_globals.get('__file__', '')) == ENTRY and frame.f_code.co_name == 'replay'):
                frame = frame.f_back
            assert frame is not None, 'replay child lost authored caller'
            rows, refusal = frame.f_locals['rows'], frame.f_locals['refusal']
            stdout = ('bindings: %d · mismatches: 0\n' % (17 + len(rows))) if refusal is None else refusal + '\n'
            result = types.SimpleNamespace(returncode=0 if refusal is None else 1, stdout=stdout, stderr='')
        children.append(arguments); return result

    environment = dict(saved, **stores, MAKE_TMPDIR=stores['TMPDIR'], RUSTUP_AUTO_INSTALL='0', RUSTUP_TOOLCHAIN='1.99.0', BINDING_UNRELATED='preserved')
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
        ns = dict(__file__=str(entry), __name__='__main__' if cli else 'borrowed_binding')
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                exec(compile(scripts[entry], str(entry), 'exec'), ns)
                if entry == PROVIDER:
                    plan_result = ns['fixture_plan']()
                    assert set(plan_result['output_files']) == FILES and set(plan_result['sources']) >= WHOLE and set(plan_result['directories']) == DIRS, 'binding replay pure plan incomplete'
            except RuntimeError as error:
                assert entry == WRITER and fault and str(error) == 'binding replay injected child failure', 'binding replay unrelated exception'
        assert sys.argv is current_argv and dict(os.environ) == environment, 'binding replay borrowed state changed'
        if entry == PROVIDER:
            assert not prepared and not writes and not children, 'binding replay pure provider produced effects'
        elif entry == ENTRY:
            assert ns['checks'] == 19 and len(children) == 17 and len(writes) == 17 * (len(BOOK_FILES) + 2) + 1 and set(writes) == FILES, 'binding replay authored entry population changed'
        else:
            assert shadow[0] == FROZEN[SOURCE], 'binding replay modeled restoration failed'
            assert (len(children), writes.count(SOURCE), len(writes)) == ((0, 2, 2) if fault else (9, 19, 28)), 'binding replay writer population changed'
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'binding replay capture changed tracked sources'
        return len(plans), len(writes), len(children)
    finally:
        runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes, Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = originals
        os.environ.clear(); os.environ.update(saved); sys.argv = argv


def original_controls():
    for path in (ENTRY, WRITER):
        old, new = ast.parse(ORIGINALS[path]), ast.parse(SCRIPTS[path])
        assert sorted(ast.dump(node) for node in ast.walk(old) if isinstance(node, ast.Assert)) == sorted(ast.dump(node) for node in ast.walk(new) if isinstance(node, ast.Assert)), 'binding replay authored assertions changed'
        if path == WRITER:
            def cases(tree):
                return next(node.value for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'CASES' for target in node.targets))
            assert ast.dump(cases(old)) == ast.dump(cases(new)), 'replay nine cases changed'
    def authored(tree):
        return sorted(ast.dump(node.value) for node in ast.walk(tree) if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'valid' for target in node.targets)), sorted(ast.dump(node.iter) for node in ast.walk(tree) if isinstance(node, ast.For))
    old, new = ast.parse(ORIGINALS[ENTRY]), ast.parse(SCRIPTS[ENTRY])
    assert authored(old) == authored(new), 'replay authored positive/refusal/formatter population changed'
    for path, wanted in ((ENTRY, 'binding replay delete before producing plan'), (WRITER, 'binding replay mkdir before producing plan')):
        try:
            capture(path, scripts={path: ORIGINALS[path]})
        except AssertionError as error:
            assert str(error) == wanted, ('binding replay unrelated original gap', str(error))
        else:
            raise AssertionError(('binding replay original gap escaped', path.name))
    faults = (
        (ENTRY, ENTRY, "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'binding replay loader before bootstrap'),
        (PROVIDER, ENTRY, "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'binding replay context before provider bootstrap'),
        (PROVIDER, ENTRY, "    PROFILE['plan_producer'](ROOT, **plan)\n    return", '    return', 'binding replay activation before complete pure plan'),
        (PROVIDER, ENTRY, 'directories=directories, output_files=outputs, sources=sources', 'directories=directories, output_files=outputs[:-1], sources=sources', 'binding replay producing files incomplete'),
        (PROVIDER, ENTRY, 'directories=directories, output_files=outputs, sources=sources', 'directories=directories[:-1], output_files=outputs, sources=sources', 'binding replay producing directories incomplete'),
        (PROVIDER, ENTRY, 'directories=directories, output_files=outputs, sources=sources', 'directories=directories, output_files=outputs, sources=sources[:-1]', 'binding replay producing sources incomplete'),
        (PROVIDER, ENTRY, "return PROFILE['prepare_producer'](ROOT, **plan) if activate else plan", "return PROFILE['environment_for'](ROOT, os.environ, create=False) if activate else plan", 'binding replay delete before producing plan'),
        (PROVIDER, ENTRY, '    fixture_plan()\n    if WORK.exists()', '    if WORK.exists()', 'binding replay delete recheck omitted'),
        (PROVIDER, ENTRY, "    PROFILE['plan_producer'](ROOT, sources=(SOURCE, *files), output_files=files)", '    None', 'binding replay copied input recheck omitted'),
        (ENTRY, ENTRY, "        FIXTURE['source_plan']()\n        FIXTURE['consumer_inputs']", "        FIXTURE['consumer_inputs']", 'binding replay child source recheck omitted'),
        (ENTRY, ENTRY, "        FIXTURE['consumer_inputs'](dict(environment, FORMULA_BOOK=str(WORK)))\n", '', 'binding replay child selector recheck omitted'),
        (ENTRY, ENTRY, 'env=dict(environment, FORMULA_BOOK=str(WORK))', "env=dict(environment, FORMULA_BOOK=str(WORK), TMPDIR=str(ROOT/'target/wrong-binding-store'))", 'binding replay child stores lost'),
        (ENTRY, ENTRY, "        FIXTURE['fixture_plan']()\n        (WORK / 'replay.log')", "        (WORK / 'replay.log')", 'binding replay copy log recheck omitted'),
        (WRITER, WRITER, "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'binding replay loader before bootstrap'),
        (WRITER, WRITER, "sources=tuple(dict.fromkeys((*BOOT_INPUTS, *child['sources'])))", 'sources=BOOT_INPUTS', 'binding replay writer sources incomplete'),
        (WRITER, WRITER, "output_files=(*child['output_files'],", 'output_files=(*(),', 'binding replay nested writer files incomplete'),
        (WRITER, WRITER, "directories=(WORK, *child['directories'])", 'directories=(WORK,)', 'binding replay nested writer directories incomplete'),
        (WRITER, WRITER, 'mutable_sources=(SOURCE,))', 'mutable_sources=())', 'binding replay writer authority omitted'),
        (WRITER, WRITER, "            producer_plan()\n            SOURCE.write_text", '            SOURCE.write_text', 'binding replay writer write recheck omitted'),
        (WRITER, WRITER, '            producer_plan()\n            result =', '            result =', 'binding replay writer child recheck omitted'),
        (WRITER, WRITER, "            producer_plan()\n            (WORK /", '            (WORK /', 'binding replay writer log recheck omitted'),
    )
    for path, entry, anchor, replacement, wanted in faults:
        expected = 3 if anchor == 'mutable_sources=(SOURCE,))' else 1
        assert SCRIPTS[path].count(anchor) == expected, ('binding replay guard anchor changed', anchor)
        try:
            capture(entry, scripts={path: SCRIPTS[path].replace(anchor, replacement, 1)})
        except AssertionError as error:
            assert str(error) == wanted, ('binding replay unrelated guard red', anchor, str(error))
        else:
            raise AssertionError(('binding replay actual guard omission escaped', anchor))
    print('binding replay guards:', len(faults), 'actual reds/two original gaps; original assertion bodies/authored rows/nine cases exact')


def mirror(label, index):
    sandbox = ROOT / ('target/replay-plan-' + label) / str(os.getpid()) / str(index)
    assert not sandbox.exists(), 'binding replay mirror generation already exists'
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
        raise AssertionError('binding replay physical refusal produced effects')
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
                sentinel.write_text('owned binding preflight sentinel\n'); wanted = 'unexpected copied-book fixture entry'
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
                leaf.write_text('owned binding selected input sentinel\n')
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
                        namespace = runpy.run_path(str(mirrors[PROVIDER])); namespace['fixture_plan']()
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
                    raise AssertionError(('binding replay physical refusal escaped', entry.name, case))
                if case in ('missing', 'symlink', 'directory', 'nested-repo'):
                    for unused in range(2):
                        stderr.seek(0); stderr.truncate(0)
                        try:
                            runpy.run_path(str(mirrors[entry]))
                        except SystemExit as error:
                            assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', 'binding replay borrowed boundary changed'
                        else:
                            raise AssertionError('binding replay producing borrowed refusal escaped')
                        borrowed += 1
                assert dict(os.environ) == unchanged and sys.argv is current_argv, 'binding replay refused load changed state'
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
        ns['fixture_plan'](); assert not tuple(parts.iterdir()), 'D202 omission produced effects'
    sandbox, mirrors = sandboxes[WRITER, 'hardlinked-output']
    profile = mirrors[PROFILE_SOURCE]; text = profile.read_text()
    node = next(node for node in ast.parse(text).body if isinstance(node, ast.FunctionDef) and node.name == 'plan_producer')
    lines = text.splitlines(keepends=True)
    disabled = ''.join(lines[:node.body[0].lineno - 1]) + '    return ()\n' + ''.join(lines[node.end_lineno:])
    GOOD['plan_producer'](ROOT, sources=(profile,), mutable_sources=(profile,))
    try:
        profile.write_text(disabled)
        with isolated_environment(sandbox), blocked_effects():
            ns = runpy.run_path(str(mirrors[PROVIDER])); ns['fixture_plan']()
            assert (sandbox / COPY_LOG.relative_to(ROOT)).stat().st_nlink == 2
    finally:
        profile.write_text(text)
    sandbox, mirrors = mirror('pure-unused-sidebar', 0)
    sidebar = mirrors[BOOK / 'annexes/formula-static-validation.md']
    sidebar.unlink(); sidebar.symlink_to(mirrors[BOOK / 'spec/units-and-tolerances.md'])
    with isolated_environment(sandbox), blocked_effects():
        ns = runpy.run_path(str(mirrors[PROVIDER]))
        assert ns['PROFILE'] is ns['CONTEXT']['PROFILE'], 'binding replay pure unused-sidebar provider drift'
    print('binding replay pure provider: unused sidebar is not consumed on import; no effects')
    print('binding replay physical:', total - 1, 'actual CLI refusals/one read-only hardlink positive;', borrowed, 'repeated producing borrowed refusals; actual D202/planner omission admissions')


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
                    raise ns['PROFILE']['Refusal']('isolated delegated binding refusal')
                ns['fixture_plan'] = fail
            return ns
        stderr, saved, argv = io.StringIO(), dict(os.environ), sys.argv
        try:
            runpy.run_path = load
            with blocked_effects(), contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                ns = dict(__file__=str(entry), __name__='__main__')
                try:
                    exec(compile(text, str(entry), 'exec'), ns)
                except SystemExit as error:
                    assert entry != PROVIDER and error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — isolated delegated binding refusal\n', 'binding replay delegated CLI refusal changed'
                except Exception as error:
                    assert provider and type(error) is provider[0]['PROFILE']['Refusal'], ('binding replay unrelated provider failure', error)
                    raise AssertionError('binding replay delegated refusal escaped') from None
                else:
                    assert entry == PROVIDER, 'binding replay delegated refusal absent'
                assert ns['PROFILE'] is ns['CONTEXT' if entry == PROVIDER else 'FIXTURE']['PROFILE'], 'binding replay actual provider identity changed'
            assert dict(os.environ) == saved and sys.argv is argv, 'binding replay provider control changed state'
        finally:
            runpy.run_path = real; os.environ.clear(); os.environ.update(saved); sys.argv = argv
    for entry in (PROVIDER, ENTRY, WRITER):
        check(entry, SCRIPTS[entry])
        anchor = "PROFILE = %s['PROFILE']\n" % ('CONTEXT' if entry == PROVIDER else 'FIXTURE')
        assert SCRIPTS[entry].count(anchor) == 1
        try:
            check(entry, SCRIPTS[entry].replace(anchor, '', 1))
        except AssertionError as error:
            assert str(error) == ('binding replay actual provider identity changed' if entry == PROVIDER else 'binding replay delegated refusal escaped')
        else:
            raise AssertionError('binding replay actual provider adoption omission escaped')
    print('binding replay providers: actual independently loaded identities/delegated named refusals; three adoption omission reds; state exact')


def late_restore_capture(text, index):
    sandbox, mirrors = mirror('late-restore', index)
    original = FROZEN[SOURCE]
    real = subprocess.run; stderr = io.StringIO()
    log = sandbox / (WRITER_WORK / 'mutation-1.log').relative_to(ROOT)
    def child(arguments, **kwargs):
        assert arguments == [sys.executable, '-I', '-B', ENTRY.relative_to(ROOT).as_posix()], 'D207 binding replay child interface changed'
        assert kwargs['cwd'] == sandbox and mirrors[SOURCE].read_bytes() != original, 'D207 binding replay actual mutation absent'
        GOOD['plan_producer'](ROOT, directories=(log,)); log.mkdir()
        return types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'binding_replay_contract.py AssertionError\n')
    try:
        with isolated_environment(sandbox), contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
            subprocess.run = child
            try:
                exec(compile(text, str(mirrors[WRITER]), 'exec'), dict(__file__=str(mirrors[WRITER]), __name__='__main__'))
            except SystemExit as error:
                assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — producer output is not a local regular file\n', ('D207 unrelated binding refusal', stderr.getvalue())
            else:
                raise AssertionError('D207 binding replay late log refusal escaped')
            assert mirrors[SOURCE].read_bytes() == original, 'D207 binding replay actual source not restored'
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
        assert str(error) == 'D207 binding replay actual source not restored', ('D207 unrelated binding red', str(error))
    else:
        raise AssertionError('D207 binding replay restoration coupling escaped')
    late_restore_capture(SCRIPTS[WRITER], 1)
    sandbox, mirrors = mirror('nested-preflight', 0)
    sentinel = sandbox / WORK.relative_to(ROOT) / 'unexpected.txt'
    GOOD['prepare_producer'](ROOT, directories=(sentinel.parent,), output_files=(sentinel,))
    sentinel.write_text('owned binding nested preflight sentinel\n')
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
                ns['fixture_plan']()
            except ns['PROFILE']['Refusal'] as error:
                assert str(error) == 'unexpected copied-book fixture entry', 'binding replay nested unrelated refusal'
            else:
                raise AssertionError('binding replay nested pure preflight admitted unexpected entry')
        provider.write_text(disabled)
        with isolated_environment(sandbox), blocked_effects():
            ns = runpy.run_path(str(provider)); ns['fixture_plan']()
            assert sentinel.read_text() == 'owned binding nested preflight sentinel\n' and mirrors[SOURCE].read_bytes() == FROZEN[SOURCE]
    finally:
        provider.write_text(SCRIPTS[PROVIDER])
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'binding replay physical controls changed tracked sources'
    print('binding replay D207: actual mirrored source restoration red/green after late-log refusal; nested child old-entry guard refusal/omission admission; all tracked sources exact')


def main():
    for entry in (PROVIDER, ENTRY, WRITER):
        print('binding replay capture:', entry.name, capture(entry))
    print('binding replay child-failure capture:', capture(WRITER, fault=True))
    for entry in (ENTRY, WRITER):
        print('binding replay CLI capture:', entry.name, capture(entry, cli=True))
    selectors = {name: str(path.relative_to(ROOT)) for name, path in (
        ('FORMULA_CONTRACT', BOOK / 'spec/formula-language.md'), ('FORMULA_GRAMMAR', BOOK / 'spec/formula-language/grammar.md'),
        ('FORMULA_EXAMPLES', BOOK / 'spec/formula-language/examples.md'), ('FORMULA_FIXTURE', BOOK / 'spec/reference-skirt.md'),
        ('FORMULA_PARTS_DIR', BOOK / 'spec/formula-language'))}
    selectors['FORMULA_ROOT'] = 'unused-root-preserved'
    print('binding replay selector capture:', capture(ENTRY, selectors=selectors))
    original_controls()
    physical_controls()
    boundary_controls()
    late_restore_controls()
    print('binding replay controls: actual producing imports/pure provider/original assertions/guard reds and physical refusals; real verdicts separate')


if __name__ == '__main__':
    main()
