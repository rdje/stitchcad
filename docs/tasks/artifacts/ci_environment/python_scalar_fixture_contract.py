"""Scalar producer plans: actual imports, complete copied files and two-source restoration."""
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
import textwrap

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
ENTRY = HERE / 'scalar_contract.py'
WRITER = HERE / 'scalar_mutations.py'
PROVIDER = HERE / 'scalar_fixture.py'
INPUT = HERE / 'formula_input.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
BOOK = ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_scalar_source'
WRITER_WORK = ROOT / 'target/formula_scalar_mutations'
GOOD = runpy.run_path(str(PROFILE_SOURCE))
ENTRY_BOOT = (ENTRY, PROFILE_SOURCE, PROVIDER, INPUT)
WRITER_BOOT = (WRITER, PROFILE_SOURCE, SOURCE, INPUT, ENTRY, PROVIDER)
PROVIDER_BOOT = (PROVIDER, PROFILE_SOURCE, SOURCE, ENTRY, WRITER, INPUT,
                 HERE / 'formula_structure.py', HERE / 'static_signature_contract.py')
BOOK_DIRS, book_files = [BOOK], []
for folder in BOOK_DIRS:
    GOOD['directory'](ROOT, folder.relative_to(ROOT).as_posix())
    for path in sorted(folder.iterdir()):
        GOOD['require'](not path.is_symlink(), 'symlink scalar control book entry')
        (BOOK_DIRS if path.is_dir() else book_files).append(path)
BOOK_FILES = tuple(book_files)
WHOLE = set((*PROVIDER_BOOT, *BOOK_FILES))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in WHOLE}
SCRIPTS = {path: FROZEN[path].decode() for path in PROVIDER_BOOT if path.suffix == '.py'}
BEFORE = 'da61ee1edf25b5559dde2c2c8de2220119e00f6b'
ORIGINALS = {path: subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], text=True)
             for path in (ENTRY, WRITER)}
COPIES = {WORK / path.relative_to(BOOK) for path in BOOK_FILES}
DIRS = {WORK / path.relative_to(BOOK) for path in BOOK_DIRS}
LOGS = {WRITER_WORK / ('mutation-%d.log' % number) for number in range(1, 12)}


def before_controls():
    """Stop original code at its first unplanned effect, without touching an original."""
    original_load, original_mkdir = runpy.run_path, Path.mkdir
    def load(*args, **kwargs):
        raise AssertionError('scalar loader before bootstrap')
    def mkdir(*args, **kwargs):
        raise AssertionError('scalar mkdir before activation')
    try:
        runpy.run_path, Path.mkdir = load, mkdir
        for entry, wanted in ((ENTRY, 'scalar loader before bootstrap'), (WRITER, 'scalar mkdir before activation')):
            try:
                exec(compile(ORIGINALS[entry], str(entry), 'exec'), dict(__file__=str(entry), __name__='borrowed_scalar'))
            except AssertionError as error:
                assert str(error) == wanted, error
                print('scalar original gap:', entry.name, wanted, 'rc=0; no effects')
            else:
                raise AssertionError('scalar original gap escaped')
    finally:
        runpy.run_path, Path.mkdir = original_load, original_mkdir
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'scalar original capture changed source'


def original_controls():
    before_controls()
    for path in (ENTRY, WRITER):
        old, new = ast.parse(ORIGINALS[path]), ast.parse(SCRIPTS[path])
        assertions = lambda tree: sorted(ast.dump(node) for node in ast.walk(tree) if isinstance(node, ast.Assert))
        assert assertions(new) == assertions(old), 'scalar original assertions changed'
        if path == ENTRY:
            for node in old.body:
                if isinstance(node, ast.FunctionDef):
                    actual = next(item for item in ast.walk(new) if isinstance(item, ast.FunctionDef) and item.name == node.name)
                    assert ast.dump(actual) == ast.dump(node), ('scalar numeric function changed', node.name)
            marker = '# Read an independently altered normative bound'
            expected = ORIGINALS[path].split('checks = 1', 1)[1].split(marker, 1)[0]
            actual = textwrap.dedent('    checks = 1' + SCRIPTS[path].split('    checks = 1', 1)[1].split(marker, 1)[0]).split('checks = 1', 1)[1]
            assert actual.rstrip() == expected.rstrip(), 'scalar complete numerical body changed'
        else:
            cases = lambda tree: next(node.value for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'CASES' for target in node.targets))
            assert ast.dump(cases(old)) == ast.dump(cases(new)), 'scalar original eleven faults changed'
    faults = (
        (ENTRY, ENTRY, "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'scalar loader before bootstrap'),
        (PROVIDER, ENTRY, "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'scalar context before provider bootstrap'),
        (PROVIDER, ENTRY, "    PROFILE['plan_producer'](ROOT, **plan)\n    return", '    return', 'scalar activation before complete pure plan'),
        (PROVIDER, ENTRY, 'directories=directories, output_files=outputs, sources=sources', 'directories=directories, output_files=outputs[:-1], sources=sources', 'scalar producing files incomplete'),
        (PROVIDER, ENTRY, 'directories=directories, output_files=outputs, sources=sources', 'directories=directories[:-1], output_files=outputs, sources=sources', 'scalar producing directories incomplete'),
        (PROVIDER, ENTRY, 'directories=directories, output_files=outputs, sources=sources', 'directories=directories, output_files=outputs, sources=sources[:-1]', 'scalar producing sources incomplete'),
        (PROVIDER, ENTRY, "return PROFILE['prepare_producer'](ROOT, **plan) if activate else plan", "return PROFILE['environment_for'](ROOT, __import__('os').environ, create=False) if activate else plan", 'scalar delete before producing plan'),
        (PROVIDER, ENTRY, '    scalar_plan()\n    if WORK.exists()', '    if WORK.exists()', 'scalar delete recheck omitted'),
        (PROVIDER, ENTRY, '    scalar_plan()\n    shutil.copytree', '    shutil.copytree', 'scalar copy recheck omitted'),
        (PROVIDER, ENTRY, "    PROFILE['plan_producer'](ROOT, sources=(SOURCE, *files), output_files=files)", '    None', 'scalar copied input recheck omitted'),
        (ENTRY, ENTRY, "    FIXTURE['copied_plan']()\n    p.write_text(text.replace", '    p.write_text(text.replace', 'scalar copied edit recheck omitted'),
        (WRITER, WRITER, "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'scalar loader before bootstrap'),
        (WRITER, WRITER, "sources=tuple(dict.fromkeys((*BOOT_INPUTS, *child['sources'])))", 'sources=BOOT_INPUTS', 'scalar writer sources incomplete'),
        (WRITER, WRITER, "output_files=(*child['output_files'],", 'output_files=(*(),', 'scalar nested writer files incomplete'),
        (WRITER, WRITER, "directories=(WORK, *child['directories'])", 'directories=(WORK,)', 'scalar nested writer directories incomplete'),
        (WRITER, WRITER, 'range(1, 12)', 'range(1, 11)', 'scalar nested writer files incomplete'),
        (WRITER, WRITER, 'mutable_sources=(SOURCE, LOADER))', 'mutable_sources=(SOURCE,))', 'scalar writer authority omitted'),
        (WRITER, WRITER, '            producer_plan()\n            target.write_text', '            target.write_text', 'scalar writer write recheck omitted'),
        (WRITER, WRITER, '            producer_plan()\n            result =', '            result =', 'scalar writer child recheck omitted'),
        (WRITER, WRITER, '            producer_plan()\n            (WORK /', '            (WORK /', 'scalar writer log recheck omitted'),
        (WRITER, WRITER, 'cwd=ROOT, env=environment, capture_output=True', 'cwd=ROOT, capture_output=True', 'scalar child stores lost'),
    )
    for path, entry, anchor, replacement, wanted in faults:
        assert SCRIPTS[path].count(anchor) == 1, ('scalar guard anchor changed', anchor)
        try:
            capture(entry, scripts={path: SCRIPTS[path].replace(anchor, replacement, 1)})
        except AssertionError as error:
            assert str(error) == wanted, ('scalar unrelated guard red', anchor, str(error), wanted)
        else:
            raise AssertionError(('scalar actual guard omission escaped', anchor))
    print('scalar body controls:', len(faults), 'actual omission reds/two original gaps; complete numerical body/assertions/functions/eleven faults exact')


def capture(entry, *, fault=False, scripts=None, selectors=None, cli=False):
    """Execute actual modules, intercept child dispatch and keep the writable shell modeled."""
    scripts = dict(SCRIPTS) | (scripts or {})
    saved, argv = dict(os.environ), sys.argv
    stores = {name: str(ROOT / 'target/scalar-plan-capture-stores' / suffix) for name, suffix in (
        ('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in stores.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    originals = (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
                 Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree)
    latest, plans, prepared, writes, children = {}, [], [], [], []
    shadow = {path: FROZEN[path] for path in (SOURCE, INPUT)}
    checkpoint, edit_checkpoint, read_checkpoint, activation, tree_checkpoint = [0], [0], [0], [0], [0]

    def owner():
        frame = sys._getframe(1)
        while frame and Path(frame.f_globals.get('__file__', '')) not in (ENTRY, WRITER, PROVIDER):
            frame = frame.f_back
        return Path(frame.f_globals['__file__']) if frame else None

    def plan(root, **kwargs):
        assert root == ROOT, 'scalar plan root changed'
        who = owner()
        if who == WRITER:
            sources = set(kwargs.get('sources', ()))
            if sources == set(WRITER_BOOT) and not kwargs.get('output_files'):
                assert not any(kwargs.get(key) for key in ('directories', 'output_files', 'mutable_sources')), 'scalar bootstrap has effects'
            elif len(sources) == 1 and sources <= {SOURCE, INPUT}:
                assert set(kwargs.get('mutable_sources', ())) == sources and not kwargs.get('output_files') and not kwargs.get('directories'), 'scalar restoration authority changed'
            else:
                assert sources >= WHOLE, 'scalar writer sources incomplete'
                assert set(kwargs.get('output_files', ())) == COPIES | LOGS, 'scalar nested writer files incomplete'
                assert set(kwargs.get('directories', ())) == DIRS | {WRITER_WORK}, 'scalar nested writer directories incomplete'
                assert kwargs.get('mutable_sources') == (SOURCE, INPUT), 'scalar writer authority omitted'
        before = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == before, 'scalar pure plan changed environment'
        plans.append((who, kwargs)); latest[who] = (kwargs, len(plans))
        return result

    def prepare(root, **kwargs):
        who = owner()
        assert latest.get(who, ({}, 0))[0] == kwargs, 'scalar activation before complete pure plan'
        assert set(kwargs.get('sources', ())) >= WHOLE, 'scalar producing sources incomplete'
        assert set(kwargs.get('output_files', ())) == COPIES | (LOGS if who == WRITER else set()), 'scalar producing files incomplete'
        assert set(kwargs.get('directories', ())) == DIRS | ({WRITER_WORK} if who == WRITER else set()), 'scalar producing directories incomplete'
        assert set(kwargs.get('mutable_sources', ())) == ({SOURCE, INPUT} if who == WRITER else set()), 'scalar authority changed'
        prepared.append(kwargs); activation[0] = len(plans)
        return GOOD['prepare_producer'](root, **kwargs)

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, prepare_producer=prepare)
        who = owner()
        assert path in scripts, 'scalar unexpected loader'
        if path == PROVIDER:
            boot = ENTRY_BOOT if who == ENTRY else WRITER_BOOT
            assert latest.get(who, ({}, 0))[0].get('sources') == boot, 'scalar loader before bootstrap'
        elif path == INPUT and who == PROVIDER:
            assert latest.get(PROVIDER, ({}, 0))[0].get('sources') == PROVIDER_BOOT, 'scalar context before provider bootstrap'
        else:
            assert path in latest.get(who, ({}, 0))[0].get('sources', ()), 'scalar delegated source unchecked'
        ns = dict(__file__=str(path), __name__='borrowed_scalar')
        exec(compile(scripts[path], str(path), 'exec'), ns)
        return ns

    def fresh_writer(message):
        assert latest.get(WRITER, ({}, 0))[1] > checkpoint[0] and plans[-1][0] == WRITER, message
        checkpoint[0] = len(plans)

    def read(path, binary, *args, **kwargs):
        if path in (SOURCE, INPUT):
            if owner() == WRITER:
                assert any(item.get('mutable_sources') == (SOURCE, INPUT) for item in prepared), 'scalar writer read before complete plan'
            return shadow[path] if binary else shadow[path].decode()
        if path in COPIES:
            planned, generation = latest.get(PROVIDER, ({}, 0))
            assert path in planned.get('sources', ()) and generation > edit_checkpoint[0], 'scalar copied input recheck omitted'
            if planned.get('output_files'):
                assert COPIES <= set(planned['sources']) and COPIES <= set(planned['output_files']), 'scalar copied source plan omitted'
            read_checkpoint[0] = generation
        return originals[2 if binary else 1](path, *args, **kwargs)

    def authorized(path):
        assert prepared and any(path in item.get('output_files', ()) for item in prepared), 'scalar output before complete producing plan'
        assert path in COPIES | LOGS, 'scalar unowned output'
        assert all(os.environ.get(name) == value for name, value in stores.items()), 'scalar stores lost'

    def write(path, data, binary, *args, **kwargs):
        if path in (SOURCE, INPUT):
            fresh_writer('scalar writer write recheck omitted')
            assert path in latest[WRITER][0].get('mutable_sources', ()), 'scalar writer authority omitted'
            if binary:
                assert set(latest[WRITER][0].get('sources', ())) == {path} and not latest[WRITER][0].get('output_files'), 'scalar restoration depends on unrelated paths'
            shadow[path] = data if binary else data.encode()
        else:
            authorized(path)
            if path in LOGS:
                fresh_writer('scalar writer log recheck omitted')
            else:
                planned, generation = latest.get(PROVIDER, ({}, 0))
                assert COPIES <= set(planned.get('sources', ())) and generation > max(edit_checkpoint[0], read_checkpoint[0]), 'scalar copied edit recheck omitted'
            originals[4 if binary else 3](path, data, *args, **kwargs)
            if path in COPIES:
                edit_checkpoint[0] = len(plans)
        writes.append(path); return len(data)

    def path_mkdir(path, *args, **kwargs):
        assert prepared, 'scalar mkdir before producing plan'
        return originals[5](path, *args, **kwargs)

    def mkdir(path, *args, **kwargs):
        assert prepared and Path(path) in DIRS | {WRITER_WORK}, 'scalar unplanned directory'
        return originals[6](path, *args, **kwargs)

    def remove(path, *args, **kwargs):
        assert prepared, 'scalar delete before producing plan'
        kwargs_plan, generation = latest.get(PROVIDER, ({}, 0))
        assert Path(path) == WORK and generation > activation[0] and set(kwargs_plan.get('output_files', ())) == COPIES, 'scalar delete recheck omitted'
        tree_checkpoint[0] = generation
        return originals[8](path, *args, **kwargs)

    def copyfile(source, destination):
        source, destination = Path(source), Path(destination)
        assert source in BOOK_FILES, 'scalar unplanned copy source'
        authorized(destination); GOOD['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        writes.append(destination); return shutil.copy2(source, destination)

    def copytree(source, destination, *args, **kwargs):
        if Path(source) == BOOK:
            assert Path(destination) == WORK and not args and not kwargs, 'scalar copy interface changed'
            assert latest.get(PROVIDER, ({}, 0))[1] > max(activation[0], tree_checkpoint[0]), 'scalar copy recheck omitted'
            return originals[9](source, destination, copy_function=copyfile)
        bound = inspect.signature(originals[9]).bind(source, destination, *args, **kwargs); bound.apply_defaults()
        assert bound.arguments['copy_function'] is copyfile, 'scalar recursive copier changed'
        return originals[9](source, destination, *args, **kwargs)

    def child(arguments, *args, **kwargs):
        environment = kwargs.get('env', {})
        assert not args and kwargs.get('cwd') == ROOT and kwargs.get('capture_output') is True, 'scalar child interface changed'
        assert all(environment.get(name) == value for name, value in stores.items()) and environment.get('MAKE_TMPDIR') == stores['TMPDIR'], 'scalar child stores lost'
        assert environment.get('SCALAR_UNRELATED') == 'preserved', 'scalar unrelated environment lost'
        assert entry == WRITER and arguments == [sys.executable, '-I', '-B', ENTRY.relative_to(ROOT).as_posix()], 'scalar writer child argv changed'
        fresh_writer('scalar writer child recheck omitted')
        assert any(shadow[path] != FROZEN[path] for path in (SOURCE, INPUT)), 'scalar actual source mutation absent'
        if fault:
            raise RuntimeError('scalar injected child failure')
        result = types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'scalar_contract.py AssertionError\n')
        children.append(arguments); return result

    environment = dict(saved, **stores, MAKE_TMPDIR=stores['TMPDIR'], RUSTUP_AUTO_INSTALL='0', RUSTUP_TOOLCHAIN='1.99.0', SCALAR_UNRELATED='preserved')
    environment.update(selectors or {})
    try:
        os.environ.clear(); os.environ.update(environment); sys.argv = [str(entry), 'unrelated']; current_argv = sys.argv
        runpy.run_path = load
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.write_text = lambda path, data, *args, **kwargs: write(path, data, False, *args, **kwargs)
        Path.write_bytes = lambda path, data, *args, **kwargs: write(path, data, True, *args, **kwargs)
        Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = path_mkdir, mkdir, child, remove, copytree
        ns = dict(__file__=str(entry), __name__='__main__' if cli else 'borrowed_scalar')
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                exec(compile(scripts[entry], str(entry), 'exec'), ns)
                if entry == PROVIDER:
                    plan_result = ns['scalar_plan']()
                    assert set(plan_result['output_files']) == COPIES and set(plan_result['sources']) >= WHOLE and set(plan_result['directories']) == DIRS, 'scalar pure plan incomplete'
            except RuntimeError as error:
                assert entry == WRITER and fault and str(error) == 'scalar injected child failure', 'scalar unrelated exception'
        assert sys.argv is current_argv and dict(os.environ) == environment, 'scalar borrowed state changed'
        if entry == PROVIDER:
            assert not prepared and not writes and not children, 'scalar pure provider produced effects'
        elif entry == ENTRY:
            assert ns['checks'] == 57 and not children and len(writes) == len(BOOK_FILES) + 2 and set(writes) == COPIES, 'scalar authored entry population changed'
        else:
            assert all(shadow[path] == FROZEN[path] for path in (SOURCE, INPUT)), 'scalar modeled restoration failed'
            assert (len(children), len(writes)) == ((0, 3) if fault else (11, 35)), 'scalar writer population changed'
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'scalar capture changed tracked sources'
        return len(plans), len(writes), len(children)
    finally:
        runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes, Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = originals
        os.environ.clear(); os.environ.update(saved); sys.argv = argv



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
                    raise ns['PROFILE']['Refusal']('isolated delegated scalar refusal')
                ns['scalar_plan'] = fail
            return ns
        stderr, saved, argv = io.StringIO(), dict(os.environ), sys.argv
        try:
            runpy.run_path = load
            with blocked_effects(), contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                ns = dict(__file__=str(entry), __name__='__main__')
                try:
                    exec(compile(text, str(entry), 'exec'), ns)
                except SystemExit as error:
                    assert entry != PROVIDER and error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — isolated delegated scalar refusal\n', 'scalar delegated CLI refusal changed'
                except Exception as error:
                    assert provider and type(error) is provider[0]['PROFILE']['Refusal'], ('scalar unrelated provider failure', error)
                    raise AssertionError('scalar delegated refusal escaped') from None
                else:
                    assert entry == PROVIDER, 'scalar delegated refusal absent'
                assert ns['PROFILE'] is ns['CONTEXT' if entry == PROVIDER else 'FIXTURE']['PROFILE'], 'scalar actual provider identity changed'
            assert dict(os.environ) == saved and sys.argv is argv, 'scalar provider control changed state'
        finally:
            runpy.run_path = real; os.environ.clear(); os.environ.update(saved); sys.argv = argv
    for entry in (PROVIDER, ENTRY, WRITER):
        check(entry, SCRIPTS[entry])
        anchor = "PROFILE = %s['PROFILE']\n" % ('CONTEXT' if entry == PROVIDER else 'FIXTURE')
        assert SCRIPTS[entry].count(anchor) == 1
        try:
            check(entry, SCRIPTS[entry].replace(anchor, '', 1))
        except AssertionError as error:
            assert str(error) == ('scalar actual provider identity changed' if entry == PROVIDER else 'scalar delegated refusal escaped')
        else:
            raise AssertionError('scalar actual provider adoption omission escaped')
    print('scalar providers: actual independently loaded identities/delegated named refusals; three adoption omission reds; state exact')



def mirror(label, index):
    sandbox = ROOT / ('target/scalar-plan-' + label) / str(os.getpid()) / str(index)
    assert not sandbox.exists(), 'scalar mirror generation already exists'
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
        raise AssertionError('scalar physical refusal produced effects')
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
             'directory-output', 'symlink-store',
             'invalid-toolchain', 'hardlinked-source', 'hardlinked-loader')
    total = borrowed = 0
    for entry in (ENTRY, WRITER):
        current_cases = (*cases, *('linked-mutation-log', 'hardlinked-mutation-log', 'directory-mutation-log')) if entry == WRITER else cases
        for case in current_cases:
            sandbox, mirrors = mirror('physical', total)
            victim = mirrors[PROVIDER if case.endswith('helper') else SOURCE if entry == WRITER else BOOK / 'spec/units-and-tolerances.md']
            log = sandbox / (WORK / 'spec/units-and-tolerances.md').relative_to(ROOT)
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
                sentinel.write_text('owned scalar preflight sentinel\n'); wanted = 'unexpected copied-book fixture entry'
            elif case in ('symlink-output', 'hardlinked-output', 'directory-output'):
                GOOD['prepare_producer'](ROOT, directories=tuple(parent for parent in log.parents if parent.is_relative_to(sandbox)), output_files=(log,))
                if case == 'symlink-output':
                    log.symlink_to(victim); wanted = 'symlink copied-book entry'
                elif case == 'hardlinked-output':
                    os.link(victim, log); wanted = 'hard-linked producer output'
                else:
                    log.mkdir(); wanted = 'unexpected copied-book fixture entry'
            elif case.endswith('mutation-log'):
                log = sandbox / (WRITER_WORK / 'mutation-11.log').relative_to(ROOT)
                victim = mirrors[BOOK / 'spec/units-and-tolerances.md']
                GOOD['prepare_producer'](ROOT, directories=(log.parent,), output_files=(log,))
                if case == 'linked-mutation-log':
                    log.symlink_to(victim); wanted = 'symlink producer output'
                elif case == 'hardlinked-mutation-log':
                    os.link(victim, log); wanted = 'hard-linked producer output'
                else:
                    log.mkdir(); wanted = 'producer output is not a local regular file'
            elif case == 'symlink-store':
                scratch = sandbox / 'target/scratch'
                GOOD['directory'](ROOT, scratch.parent.relative_to(ROOT).as_posix(), create=True)
                scratch.symlink_to(victim.parent, target_is_directory=True); wanted = 'symlink store component'
            elif case == 'invalid-toolchain':
                extra['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'; wanted = 'toolchain override must be a name'
            elif case in ('hardlinked-source', 'hardlinked-loader'):
                # Only the writer requests authority over the shell; a reader may share its inode.
                alias = sandbox / 'target/source-alias'
                GOOD['prepare_producer'](ROOT, directories=(alias.parent,), output_files=(alias,))
                os.link(mirrors[INPUT if case == 'hardlinked-loader' else SOURCE], alias)
                if entry == ENTRY:
                    with isolated_environment(sandbox), blocked_effects():
                        namespace = runpy.run_path(str(mirrors[PROVIDER])); namespace['scalar_plan']()
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
                    raise AssertionError(('scalar physical refusal escaped', entry.name, case))
                if case in ('missing', 'symlink', 'directory', 'nested-repo'):
                    for unused in range(2):
                        stderr.seek(0); stderr.truncate(0)
                        try:
                            runpy.run_path(str(mirrors[entry]))
                        except SystemExit as error:
                            assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', 'scalar borrowed boundary changed'
                        else:
                            raise AssertionError('scalar producing borrowed refusal escaped')
                        borrowed += 1
                assert dict(os.environ) == unchanged and sys.argv is current_argv, 'scalar refused load changed state'
            total += 1
    print('scalar physical:', total - 2, 'actual CLI refusals/two read-only source positives;', borrowed, 'repeated borrowed refusals; no effects')


def continuity_controls():
    """Reconstruct retained whole intervals from Git and verify prior immutable inputs."""
    reader = ROOT / 'scripts/history_archive.py'
    records = tuple(ROOT / ('docs/history/' + name) for name in (
        'stitchcad-devnotes-part285.md', 'stitchcad-changelog-part196.md'))
    for record in records:
        data = record.read_bytes()
        header, content = data.split(b'\n---\n\n', 1)
        text = header.decode()
        revision = re.search(r'complete intervals from ([0-9a-f]{40})\.', text).group(1)
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', text)
        assert (content.count(b'\n'), len(content), hashlib.sha256(content).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3]), 'scalar retained identity drift'
        chunks = []
        for path, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', text):
            source = subprocess.check_output(['git', 'show', revision + ':' + path], cwd=ROOT)
            chunk = source[int(start):int(stop)]
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest, 'scalar retained source interval drift'
            chunks.append(chunk)
        assert chunks and b''.join(chunks).rstrip(b'\n') + b'\n' == content, 'scalar whole retention incomplete'
        result = subprocess.run([sys.executable, '-I', '-B', str(reader), '--root', str(ROOT), 'read', record.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
        assert result.returncode == 0 and result.stdout == data, 'scalar public retained read differs'
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT), 'scalar prior archive changed'
    for path in (PROFILE_SOURCE, SOURCE, INPUT, HERE / 'formula_structure.py', HERE / 'static_signature_contract.py', *(BOOK / name for name in ('spec/formula-language.md', 'spec/formula-language/grammar.md', 'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))):
        assert path.read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT), 'scalar original reference input changed'
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    current = (ROOT / tree).read_text()
    headers = lambda value: re.findall(r'^- ID: `([^`]+)`$', value, re.M)
    assert headers(current) == headers(old), 'scalar canonical task graph changed'
    print('scalar continuity: two complete retained records/public reads; prior archive/reference bytes and task graph exact')


def late_restore_capture(text, index, number):
    """Create an unsafe log after a genuine mirrored edit of each mutable original."""
    sandbox, mirrors = mirror('late-restore', index)
    original_run = subprocess.run
    stderr = io.StringIO()
    target = INPUT if number == 10 else SOURCE
    calls = [0]
    log = sandbox / (WRITER_WORK / ('mutation-%d.log' % number)).relative_to(ROOT)
    def child(arguments, **kwargs):
        calls[0] += 1
        assert arguments == [sys.executable, '-I', '-B', ENTRY.relative_to(ROOT).as_posix()], 'scalar late child interface changed'
        assert kwargs['cwd'] == sandbox, 'scalar late child root changed'
        if calls[0] == number:
            assert mirrors[target].read_bytes() != FROZEN[target], 'scalar late actual mutation absent'
            GOOD['plan_producer'](ROOT, directories=(log,)); log.mkdir()
        return types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'scalar_contract.py AssertionError\n')
    try:
        with isolated_environment(sandbox), contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
            subprocess.run = child
            try:
                exec(compile(text, str(mirrors[WRITER]), 'exec'), dict(__file__=str(mirrors[WRITER]), __name__='__main__'))
            except SystemExit as error:
                assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — producer output is not a local regular file\n', ('scalar unrelated late refusal', stderr.getvalue())
            else:
                raise AssertionError('scalar late refusal escaped')
            assert all(mirrors[path].read_bytes() == FROZEN[path] for path in (SOURCE, INPUT)), 'scalar actual original not restored'
    finally:
        subprocess.run = original_run
        for path in (SOURCE, INPUT):
            GOOD['plan_producer'](ROOT, sources=(mirrors[path],), mutable_sources=(mirrors[path],))
            mirrors[path].write_bytes(FROZEN[path])
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'scalar late fixture changed actual source'


def late_restore_controls():
    guards = tuple("PROFILE['plan_producer'](ROOT, sources=(%s,), mutable_sources=(%s,))" % (name, name) for name in ('target', 'path'))
    disabled = SCRIPTS[WRITER]
    for guard in guards:
        assert disabled.count(guard) == 1
        disabled = disabled.replace(guard, 'producer_plan()', 1)
    for index, number in enumerate((1, 10)):
        try:
            late_restore_capture(disabled, index * 2, number)
        except AssertionError as error:
            assert str(error) == 'scalar actual original not restored', ('scalar unrelated restore red', error)
        else:
            raise AssertionError('scalar restore coupling escaped')
        late_restore_capture(SCRIPTS[WRITER], index * 2 + 1, number)
    print('scalar late restoration: two actual coupling reds/two exact greens; shell and quiet loader both restored after late-log refusals')


def real_controls():
    """Run exclusively; observe real numerical assertions, then compare every scoped input."""
    work = ROOT / 'target/scalar-plan-real'
    outputs = tuple(work / (entry.stem + '.log') for entry in (ENTRY, WRITER))
    plan = dict(sources=(*WHOLE, Path(__file__)), directories=(work,), output_files=outputs,
                mutable_sources=(SOURCE, INPUT))
    environment = GOOD['enter_producer'](ROOT, **plan)
    for entry, output in zip((ENTRY, WRITER), outputs):
        GOOD['plan_producer'](ROOT, **plan)
        result = subprocess.run([sys.executable, '-I', '-B', str(entry)], cwd=ROOT, env=environment, capture_output=True)
        GOOD['plan_producer'](ROOT, **plan)
        output.write_bytes(result.stdout + result.stderr)
        assert result.returncode == 0, (entry.name, result.returncode, result.stderr.decode())
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'scalar real run stranded source'
        print('scalar real:', entry.name, result.stdout.decode().splitlines()[-1], 'rc=0;', len(FROZEN), 'scoped inputs exact')


if __name__ == '__main__':
    if '--before' in sys.argv:
        before_controls()
    elif '--real' in sys.argv:
        real_controls()
    elif '--continuity' in sys.argv:
        continuity_controls()
    else:
        for entry in (PROVIDER, ENTRY, WRITER):
            print('scalar capture:', entry.name, capture(entry))
        print('scalar child-failure capture:', capture(WRITER, fault=True))
        for entry in (ENTRY, WRITER):
            print('scalar CLI capture:', entry.name, capture(entry, cli=True))
        print('scalar selector preservation capture:', capture(ENTRY, selectors={'FORMULA_BOOK': 'unused-scalar-selector', 'FORMULA_ROOT': 'unused-root'}))
        original_controls()
        physical_controls()
        boundary_controls()
        late_restore_controls()
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'scalar controls changed source'

