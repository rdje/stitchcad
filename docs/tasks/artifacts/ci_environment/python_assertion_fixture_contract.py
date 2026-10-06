"""Actual assertion import/pure/copy plans; intercepted dispatch and real verdicts are distinct."""
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
ENTRY = ROOT / 'docs/tasks/artifacts/formula_structure/assertion_contract.py'
HELPER = ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
BOOK = ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_structure/assertion_refusal'
BASE = WORK / 'false_closure/src'
LOG = WORK / 'false_closure.log'
GOOD = runpy.run_path(str(PROFILE_SOURCE))
BOOT = (ENTRY, PROFILE_SOURCE, SOURCE, HELPER)
directories, files = [BOOK], []
for folder in directories:
    GOOD['directory'](ROOT, folder.relative_to(ROOT).as_posix())
    for path in sorted(folder.iterdir()):
        GOOD['require'](not path.is_symlink(), 'symlink assertion control book entry')
        (directories if path.is_dir() else files).append(path)
BOOK_FILES = tuple(files)
WHOLE = set((*BOOT, *BOOK_FILES))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in WHOLE}
TEXT = FROZEN[ENTRY].decode()
BEFORE = '117e1657e7f813663ba90df51b13a2b42a352ff3'
ORIGINAL = subprocess.check_output(['git', 'show', BEFORE + ':' + ENTRY.relative_to(ROOT).as_posix()], text=True)
COPIES = {BASE / path.relative_to(BOOK) for path in BOOK_FILES}
OUTPUTS = COPIES | {LOG}
DIRS = {WORK, BASE.parent, *(BASE / path.relative_to(BOOK) for path in directories)}
REFERENCES = set((*BOOT, *(BOOK / name for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))))


def capture(mode, text=TEXT, selectors=None):
    """Execute actual entry/helper bodies; guard every modeled persistent effect."""
    saved, argv = dict(os.environ), sys.argv
    stores = {name: str(ROOT / 'target/assert-plan-capture-stores' / suffix) for name, suffix in (
        ('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in stores.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    originals = (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
                 Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree)
    plans, prepared, written, children, removed = [], [], [], [], []
    latest, edit_checkpoint, child_checkpoint, activation_checkpoint = {}, [0], [0], [0]

    def owner():
        frame = sys._getframe(1)
        while frame and Path(frame.f_globals.get('__file__', '')) not in (ENTRY, HELPER):
            frame = frame.f_back
        return Path(frame.f_globals['__file__']) if frame else None

    def plan(root, **kwargs):
        assert root == ROOT, 'assertion plan root changed'
        result = GOOD['plan_producer'](root, **kwargs)
        plans.append((owner(), kwargs)); latest[owner()] = (kwargs, len(plans))
        return result

    def prepare(root, **kwargs):
        assert latest.get(ENTRY, ({}, 0))[0] == kwargs, 'assertion activation before complete pure plan'
        assert set(kwargs.get('sources', ())) >= WHOLE, 'assertion producing sources incomplete'
        assert set(kwargs.get('output_files', ())) == OUTPUTS, 'assertion producing files incomplete'
        assert set(kwargs.get('directories', ())) == DIRS, 'assertion producing directories incomplete'
        prepared.append(kwargs); activation_checkpoint[0] = len(plans)
        return GOOD['prepare_producer'](root, **kwargs)

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, prepare_producer=prepare)
        assert path == HELPER and owner() == ENTRY, 'assertion unexpected loader'
        assert latest.get(ENTRY, ({}, 0))[0].get('sources') == BOOT, 'assertion loader before bootstrap'
        namespace = dict(__file__=str(HELPER), __name__='borrowed_signature')
        exec(compile(FROZEN[HELPER].decode(), str(HELPER), 'exec'), namespace)
        real = namespace['load_reference']

        def reference(*args, **kwargs):
            assert set(latest.get(ENTRY, ({}, 0))[0].get('sources', ())) == REFERENCES, 'assertion pure source recheck omitted'
            return real(*args, **kwargs)
        namespace['load_reference'] = reference
        return namespace

    def read(path, binary, *args, **kwargs):
        if path in COPIES:
            assert any(COPIES <= set(item.get('sources', ())) and COPIES <= set(item.get('output_files', ()))
                       for unused, item in plans), 'assertion copied source plan omitted'
        return originals[2 if binary else 1](path, *args, **kwargs)

    def authorized(path):
        assert prepared and path in OUTPUTS, 'assertion write before complete producing plan'
        assert all(os.environ.get(name) == value for name, value in stores.items()), 'assertion stores lost'

    def write(path, data, binary, *args, **kwargs):
        authorized(path)
        if path == LOG:
            assert latest.get(ENTRY, ({}, 0))[1] > child_checkpoint[0] and set(latest[ENTRY][0].get('output_files', ())) == OUTPUTS, 'assertion log recheck omitted'
        result = originals[4 if binary else 3](path, data, *args, **kwargs)
        written.append(path)
        if path in COPIES:
            edit_checkpoint[0] = len(plans)
        return result

    def path_mkdir(path, *args, **kwargs):
        assert prepared, 'assertion mkdir before producing plan'
        return originals[5](path, *args, **kwargs)

    def mkdir(path, *args, **kwargs):
        assert prepared and Path(path) in DIRS, 'assertion directory unplanned'
        return originals[6](path, *args, **kwargs)

    def remove(path, *args, **kwargs):
        assert len(plans) > activation_checkpoint[0] and Path(path) == BASE and set(plans[-1][1].get('output_files', ())) == OUTPUTS, 'assertion delete recheck omitted'
        assert prepared, 'assertion delete before producing plan'
        removed.append(Path(path)); return originals[8](path, *args, **kwargs)

    def copyfile(source, destination):
        source, destination = Path(source), Path(destination)
        assert source in BOOK_FILES, 'assertion copy source unplanned'
        authorized(destination); GOOD['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        written.append(destination); return shutil.copy2(source, destination)

    def copytree(source, destination, *args, **kwargs):
        if Path(source) == BOOK:
            assert len(plans) > activation_checkpoint[0], 'assertion delete recheck omitted'
            assert Path(destination) == BASE and not args and not kwargs, 'assertion copy interface changed'
            return originals[9](source, destination, copy_function=copyfile)
        bound = inspect.signature(originals[9]).bind(source, destination, *args, **kwargs); bound.apply_defaults()
        assert bound.arguments['copy_function'] is copyfile, 'assertion recursive copy changed'
        return originals[9](source, destination, *args, **kwargs)

    def child(arguments, *args, **kwargs):
        environment = kwargs.get('env', {})
        assert arguments == ['bash', str(SOURCE)] and not args and kwargs.get('cwd') == ROOT and kwargs.get('text') is True and kwargs.get('capture_output') is True, 'assertion child interface changed'
        assert all(environment.get(name) == value for name, value in stores.items()) and environment.get('MAKE_TMPDIR') == stores['TMPDIR'], 'assertion child stores lost'
        assert environment.get('ASSERTION_UNRELATED') == 'preserved', 'assertion unrelated environment lost'
        assert environment.get('FORMULA_BOOK') == str(BASE), 'assertion copied selector lost'
        assert all(environment.get(name) == value for name, value in (selectors or {}).items()), 'assertion caller selectors lost'
        assert any(WHOLE <= set(item.get('sources', ())) for unused, item in plans[edit_checkpoint[0]:]), 'assertion child source recheck omitted'
        parts = Path(environment.get('FORMULA_PARTS_DIR') or BASE / 'spec/formula-language')
        parts = parts if parts.is_absolute() else ROOT / parts
        selected = [Path(environment.get(name) or default) for name, default in (
            ('FORMULA_CONTRACT', BASE / 'spec/formula-language.md'), ('FORMULA_GRAMMAR', parts / 'grammar.md'),
            ('FORMULA_EXAMPLES', parts / 'examples.md'), ('FORMULA_FIXTURE', BASE / 'spec/reference-skirt.md'))]
        selected = [path if path.is_absolute() else ROOT / path for path in selected]
        references = [Path(os.path.normpath(str(path.parent) + '/' + target.split('#')[0]))
                      for path in selected[:3] for unused, target in re.findall(r'\[([^\]]*)\]\(([^)]+)\)', originals[1](path))
                      if not target.startswith(('http://', 'https://', '#'))]
        assert set(plans[-1][1].get('sources', ())) == set((*selected, *sorted(parts.glob('*.md')), *references)), 'assertion child selector recheck omitted'
        child_checkpoint[0] = len(plans); children.append(arguments)
        return types.SimpleNamespace(returncode=1, stdout='formula_assertion waistband_width_closure length:80000 != length:40000 `eps_num` (1)\n', stderr='')

    environment = dict(saved, **stores, MAKE_TMPDIR=stores['TMPDIR'], RUSTUP_AUTO_INSTALL='0', RUSTUP_TOOLCHAIN='1.99.0', ASSERTION_UNRELATED='preserved')
    for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE'):
        environment.pop(name, None)
    environment.update(selectors or {})
    try:
        os.environ.clear(); os.environ.update(environment); sys.argv = [str(ENTRY), '--mutations', 'unrelated']; current_argv = sys.argv
        runpy.run_path = load
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.write_text = lambda path, data, *args, **kwargs: write(path, data, False, *args, **kwargs)
        Path.write_bytes = lambda path, data, *args, **kwargs: write(path, data, True, *args, **kwargs)
        Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = path_mkdir, mkdir, child, remove, copytree
        namespace = dict(__file__=str(ENTRY), __name__='__main__' if mode == 'cli' else 'borrowed_assertion')
        with contextlib.redirect_stdout(io.StringIO()):
            exec(compile(text, str(ENTRY), 'exec'), namespace)
            if text == TEXT:
                assert namespace['PROFILE'] is namespace['HELPER']['PROFILE'], 'assertion provider identity changed'
            if mode == 'plan':
                result = namespace['fixture_plan'](activate=False)
                assert set(result['output_files']) == OUTPUTS and set(result['sources']) >= WHOLE and set(result['directories']) == DIRS, 'assertion pure plan incomplete'
            elif mode == 'contracts':
                assert namespace['contracts']() == 262, 'assertion authored population changed'
            elif mode == 'consumer':
                namespace['consumer']()
        assert sys.argv is current_argv and dict(os.environ) == environment, 'assertion borrowed state changed'
        if mode in ('consumer', 'cli'):
            assert len(children) == 1 and len(written) == len(BOOK_FILES) + 2 and set(written) == OUTPUTS, 'assertion output population changed'
        else:
            assert not prepared and not written and not children, 'assertion pure call has effects'
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'assertion capture changed sources'
        return len(plans), len(written), len(children)
    finally:
        runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes, Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = originals
        os.environ.clear(); os.environ.update(saved); sys.argv = argv


def original_controls():
    old, new = ast.parse(ORIGINAL), ast.parse(TEXT)
    assert sorted(ast.dump(node) for node in ast.walk(old) if isinstance(node, ast.Assert)) == sorted(ast.dump(node) for node in ast.walk(new) if isinstance(node, ast.Assert)), 'assertion authored assertions changed'
    for name in ('KINDS', 'CLASSES', 'FAULTS'):
        def value(tree):
            return next(node.value for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == name for target in node.targets))
        assert ast.dump(value(old)) == ast.dump(value(new)), ('assertion authored constants changed', name)
    def function(tree, name):
        return next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == name)
    body = function(new, 'contracts'); body.body = body.body[1:]
    assert ast.dump(body) == ast.dump(function(old, 'contracts')), 'assertion authored contracts changed'
    raw_consumer = ast.get_source_segment(ORIGINAL, function(old, 'consumer'))
    guarded_import_raw_consumer = TEXT.replace(ast.get_source_segment(TEXT, function(new, 'consumer')), raw_consumer, 1)
    for mode, candidate, wanted in (('import', ORIGINAL, 'assertion loader before bootstrap'),
                                     ('consumer', guarded_import_raw_consumer, 'assertion mkdir before producing plan')):
        try:
            capture(mode, candidate)
        except AssertionError as error:
            assert str(error) == wanted, ('assertion unrelated original gap', str(error))
        else:
            raise AssertionError(('assertion original gap escaped', mode))
    faults = (
        ('import', "    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'assertion loader before bootstrap'),
        ('contracts', "    PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)\n", '', 'assertion pure source recheck omitted'),
        ('consumer', "    PROFILE['plan_producer'](ROOT, **plan)\n    return", '    return', 'assertion activation before complete pure plan'),
        ('consumer', 'directories=directories, output_files=outputs, sources=sources', 'directories=directories, output_files=outputs[:-1], sources=sources', 'assertion producing files incomplete'),
        ('consumer', 'directories=directories, output_files=outputs, sources=sources', 'directories=directories[:-1], output_files=outputs, sources=sources', 'assertion producing directories incomplete'),
        ('consumer', 'directories=directories, output_files=outputs, sources=sources', 'directories=directories, output_files=outputs, sources=sources[:-1]', 'assertion producing sources incomplete'),
        ('consumer', "return PROFILE['prepare_producer'](ROOT, **plan) if activate else plan", "return PROFILE['environment_for'](ROOT, os.environ, create=False) if activate else plan", 'assertion mkdir before producing plan'),
        ('consumer', '    fixture_plan(activate=False)\n    book =', '    book =', 'assertion delete recheck omitted'),
        ('consumer', "PROFILE['plan_producer'](ROOT, sources=(SOURCE, *files), output_files=files)", 'None', 'assertion copied source plan omitted'),
        ('consumer', '    source_plan()\n    consumer_inputs', '    consumer_inputs', 'assertion child source recheck omitted'),
        ('consumer', '    consumer_inputs(dict(environment, FORMULA_BOOK=str(book)))\n', '', 'assertion child selector recheck omitted'),
        ('consumer', "env={**environment, 'FORMULA_BOOK': str(book)}", "env={**environment, 'FORMULA_BOOK': str(book), 'TMPDIR': str(ROOT/'target/wrong-assertion-store')}", 'assertion child stores lost'),
        ('consumer', "    fixture_plan(activate=False)\n    (WORK / 'false_closure.log')", "    (WORK / 'false_closure.log')", 'assertion log recheck omitted'),
    )
    for mode, anchor, replacement, wanted in faults:
        expected_count = 2 if mode == 'contracts' else 1
        assert TEXT.count(anchor) == expected_count, ('assertion fault anchor changed', anchor)
        try:
            capture(mode, TEXT.replace(anchor, replacement, 1))
        except AssertionError as error:
            assert str(error) == wanted, ('assertion unrelated guard red', anchor, str(error))
        else:
            raise AssertionError(('assertion actual guard omission escaped', anchor))
    print('assertion guards:', len(faults), 'actual reds/two original gaps; original262 cases/eight faults/body exact')


def physical_controls():
    """Refuse real filesystem faults in owned isolated copies before any producer effect."""
    cases = ('missing input', 'symlink input', 'symlink book directory', 'assertion is directory',
             'unexpected old fixture', 'symlink output', 'hardlinked output',
             'output directory is file', 'symlink copied input', 'outside selector',
             'assertion output alias', 'parts directory alias', 'symlink store', 'invalid toolchain', 'pure unused sidebar')
    def effect(*args, **kwargs):
        raise AssertionError('assertion physical effect before refusal')
    generation = ROOT / 'target/fixture-assertion-controls' / str(os.getpid())
    GOOD['directory'](ROOT, generation.parent.relative_to(ROOT).as_posix(), create=True)
    assert not generation.exists(), 'assertion fixture generation already exists'
    sandboxes = []
    for index, case in enumerate(cases):
        sandbox = generation / str(index)
        sandboxes.append(sandbox)
        mirrors = {path: sandbox / path.relative_to(ROOT) for path in WHOLE}
        directory_set = {sandbox}
        for path in mirrors.values():
            directory_set.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
        GOOD['prepare_producer'](ROOT, sources=tuple(WHOLE), directories=tuple(directory_set),
                                 output_files=tuple(mirrors.values()))
        for source, destination in mirrors.items():
            shutil.copy2(source, destination)
        work = sandbox / WORK.relative_to(ROOT)
        leaf = work / 'false_closure/src/spec/formula-language.md'
        log = work / 'false_closure.log'
        sidebar = sandbox / 'docs/book/src/annexes/formula-static-validation.md'
        input_file = sandbox / 'docs/book/src/spec/units-and-tolerances.md'
        extra = work / 'false_closure/src/unexpected.txt'
        scratch = sandbox / 'target/scratch'
        candidates = (leaf, log, extra, scratch)
        parents = {path.parent for path in candidates}
        GOOD['prepare_producer'](ROOT, directories=tuple(parents), output_files=candidates)
        selectors = {}
        expected = None
        if case == 'missing input':
            input_file.unlink(); expected = 'producer source is not a local regular file'
        elif case == 'symlink input':
            input_file.unlink(); input_file.symlink_to(mirrors[BOOT[0]])
            expected = 'symlink copied-book entry'
        elif case == 'symlink book directory':
            unused = sandbox / 'docs/book/src/unused-directory'
            GOOD['plan_producer'](ROOT, output_files=(unused,))
            unused.symlink_to(sandbox / 'docs/book/src/annexes', target_is_directory=True)
            expected = 'symlink copied-book entry'
        elif case == 'assertion is directory':
            input_file.unlink(); input_file.mkdir()
            expected = 'unexpected copied-book fixture entry'
            # A directory may be copied, but the compiled reference requires this exact file.
            selectors['FORMULA_CONTRACT'] = str(input_file)
            expected = 'producer source is not a local regular file'
        elif case == 'unexpected old fixture':
            extra.write_text('owned refusal sentinel\n'); expected = 'unexpected copied-book fixture entry'
        elif case == 'symlink output':
            leaf.symlink_to(input_file); expected = 'symlink copied-book entry'
        elif case == 'hardlinked output':
            os.link(input_file, leaf); expected = 'hard-linked producer output'
        elif case == 'output directory is file':
            base = work / 'false_closure/src'
            blocker = base / 'annexes'
            GOOD['plan_producer'](ROOT, output_files=(blocker,))
            blocker.write_text('owned directory refusal sentinel\n')
            expected = 'unexpected copied-book fixture entry'
        elif case == 'symlink copied input':
            linked = work / 'false_closure/src/spec/units-and-tolerances.md'
            GOOD['plan_producer'](ROOT, output_files=(linked,))
            linked.symlink_to(input_file); expected = 'symlink copied-book entry'
        elif case == 'outside selector':
            selectors['FORMULA_CONTRACT'] = str(ENTRY); expected = 'producer path outside workspace'
        elif case == 'assertion output alias':
            leaf.write_text('owned selected-input alias sentinel\n')
            selectors['FORMULA_CONTRACT'] = str(leaf)
            expected = 'copied-book output aliases selected input'
        elif case == 'parts directory alias':
            parts = work / 'false_closure/src/spec/formula-language'
            GOOD['directory'](ROOT, parts.relative_to(ROOT).as_posix(), create=True)
            selectors.update(FORMULA_PARTS_DIR=str(parts),
                             FORMULA_GRAMMAR=str(sandbox / 'docs/book/src/spec/formula-language/grammar.md'),
                             FORMULA_EXAMPLES=str(sandbox / 'docs/book/src/spec/formula-language/examples.md'))
            expected = 'copied-book output aliases selected input directory'
        elif case == 'symlink store':
            scratch.symlink_to(sandbox / 'docs/book/src', target_is_directory=True)
            expected = 'symlink store component'
        elif case == 'invalid toolchain':
            selectors['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'
            expected = 'toolchain override must be a name'
        elif case == 'pure unused sidebar':
            sidebar.unlink(); sidebar.symlink_to(input_file)
        saved = dict(os.environ)
        originals = (os.mkdir, shutil.rmtree, shutil.copytree, subprocess.run, Path.write_text, Path.write_bytes, Path.mkdir)
        stderr = io.StringIO()
        try:
            for key in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR',
                        'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'FORMULA_PARTS_DIR', 'RUSTUP_TOOLCHAIN'):
                os.environ.pop(key, None)
            os.environ.update({name: str(sandbox / suffix) for name, suffix in (
                ('CARGO_HOME', 'target/cargo-home'), ('RUSTUP_HOME', 'target/rustup'),
                ('CARGO_TARGET_DIR', 'target/build'), ('TMPDIR', 'target/scratch'))})
            os.environ.update(selectors)
            os.mkdir = shutil.rmtree = shutil.copytree = subprocess.run = Path.write_text = Path.write_bytes = Path.mkdir = effect
            namespace = {'__file__': str(mirrors[ENTRY]), '__name__': 'borrowed_input'}
            with contextlib.redirect_stderr(stderr):
                exec(compile(TEXT, str(mirrors[ENTRY]), 'exec'), namespace)
                if expected is None:
                    assert namespace['BOOT_INPUTS'] == tuple(mirrors[path] for path in BOOT)
                else:
                    try:
                        namespace['consumer']()
                    except namespace['PROFILE']['Refusal'] as error:
                        assert str(error) == expected, (case, 'unrelated physical refusal', str(error))
                        if case == 'parts directory alias':
                            guard = 'parts != base and not parts.is_relative_to(base)'
                            assert TEXT.count(guard) == 1
                            disabled = dict(__file__=str(mirrors[ENTRY]), __name__='directory_guard_disabled')
                            exec(compile(TEXT.replace(guard, 'True', 1), str(mirrors[ENTRY]), 'exec'), disabled)
                            disabled['fixture_plan'](activate=False)
                            assert not tuple(parts.iterdir()), 'D202 counterfactual changed directory'
                            print('Inherited D202 actual directory-alias guard omission admits same empty selected directory; no effects')

                    else:
                        raise AssertionError((case, 'assertion physical refusal escaped'))
        except SystemExit as error:
            assert error.code == 2 and expected and expected in stderr.getvalue(), (case, 'unrelated bootstrap refusal', stderr.getvalue())
        finally:
            os.mkdir, shutil.rmtree, shutil.copytree, subprocess.run, Path.write_text, Path.write_bytes, Path.mkdir = originals
            os.environ.clear(); os.environ.update(saved)
        print('assertion physical control:', case)
    # The same hardlinked log is refused by the current planner, then admitted only
    # after removing the actual profile planner body in the isolated mirror.
    sandbox = sandboxes[6]
    profile = sandbox / PROFILE_SOURCE.relative_to(ROOT)
    original = profile.read_text()
    tree = ast.parse(original)
    function = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'plan_producer')
    lines = original.splitlines(keepends=True)
    disabled = ''.join(lines[:function.body[0].lineno - 1]) + '    return ()\n' + ''.join(lines[function.end_lineno:])
    GOOD['plan_producer'](ROOT, output_files=(profile,))
    saved = dict(os.environ)
    try:
        for key in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR',
                    'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'FORMULA_PARTS_DIR'):
            os.environ.pop(key, None)
        for name in GOOD['STORES']:
            os.environ[name] = str(sandbox / GOOD['STORES'][name])
        profile.write_text(disabled)
        namespace = {'__file__': str(sandbox / ENTRY.relative_to(ROOT)), '__name__': 'disabled_input'}
        exec(compile(TEXT, namespace['__file__'], 'exec'), namespace)
        namespace['fixture_plan'](activate=False)
        assert (sandbox / 'target/formula_structure/assertion_refusal/false_closure/src/spec/formula-language.md').stat().st_nlink == 2
    finally:
        profile.write_text(original)
        os.environ.clear(); os.environ.update(saved)
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'assertion physical controls changed tracked sources'
    print('assertion guard counterfactual: actual profile planner disabled admits same hardlinked output; tracked sources exact')

def boundary_controls():
    """Named CLI refusals and repeated pure borrowed refusals share the real provider."""
    cases = ('missing', 'symlink', 'directory', 'nested-repo', 'symlink-store', 'invalid-toolchain',
             'missing-helper', 'symlink-helper', 'directory-helper', 'symlink-log', 'directory-log', 'hardlinked-log')
    generation = ROOT / 'target/assert-plan-boundaries' / str(os.getpid())
    borrowed = 0
    for index, case in enumerate(cases):
        sandbox = generation / str(index); assert not sandbox.exists()
        mirrors = {path: sandbox / path.relative_to(ROOT) for path in WHOLE}
        dirs = {sandbox}
        for path in mirrors.values():
            dirs.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
        GOOD['prepare_producer'](ROOT, sources=tuple(WHOLE), directories=tuple(dirs), output_files=tuple(mirrors.values()))
        for source, destination in mirrors.items():
            shutil.copy2(source, destination)
        victim = mirrors[HELPER if case.endswith('helper') else BOOK / 'spec/units-and-tolerances.md']
        expected = 'producer source is not a local regular file'
        extra = {}
        if case.startswith('missing'):
            victim.unlink()
        elif case.startswith('symlink') and case != 'symlink-store' and case != 'symlink-log':
            victim.unlink(); victim.symlink_to(mirrors[PROFILE_SOURCE]); expected = 'symlink producer source'
        elif case.startswith('directory') and case != 'directory-log':
            victim.unlink(); victim.mkdir()
        elif case == 'nested-repo':
            (victim.parent / '.git').mkdir(); expected = 'store crosses another Git repository'
        elif case == 'symlink-store':
            scratch = sandbox / 'target/scratch'; GOOD['directory'](ROOT, scratch.parent.relative_to(ROOT).as_posix(), create=True)
            scratch.symlink_to(victim.parent, target_is_directory=True); expected = 'symlink store component'
        elif case == 'invalid-toolchain':
            extra['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'; expected = 'toolchain override must be a name'
        else:
            log = sandbox / LOG.relative_to(ROOT)
            GOOD['prepare_producer'](ROOT, directories=(log.parent,), output_files=(log,))
            if case == 'symlink-log':
                log.symlink_to(victim); expected = 'symlink producer output'
            elif case == 'directory-log':
                log.mkdir(); expected = 'producer output is not a local regular file'
            else:
                os.link(victim, log); expected = 'hard-linked producer output'
        saved, argv = dict(os.environ), sys.argv
        originals = (Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run, shutil.copytree, shutil.rmtree)
        def effect(*args, **kwargs):
            raise AssertionError('assertion boundary refusal produced effects')
        stderr = io.StringIO()
        try:
            for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'RUSTUP_TOOLCHAIN'):
                os.environ.pop(name, None)
            os.environ.update({name: str(sandbox / suffix) for name, suffix in GOOD['STORES'].items()})
            os.environ.update(extra); sys.argv = [str(mirrors[ENTRY]), '--mutations', 'unrelated']
            Path.mkdir = os.mkdir = Path.write_text = Path.write_bytes = subprocess.run = shutil.copytree = shutil.rmtree = effect
            with contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                try:
                    runpy.run_path(str(mirrors[ENTRY]), run_name='__main__')
                except SystemExit as error:
                    assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + expected + '\n', (case, stderr.getvalue())
                else:
                    raise AssertionError(('assertion CLI fault accepted', case))
                if case in cases[:4]:
                    ns = runpy.run_path(str(mirrors[ENTRY]))
                    for unused in range(2):
                        try:
                            ns['contracts']()
                        except ns['PROFILE']['Refusal'] as error:
                            assert str(error) == expected, (case, 'assertion borrowed refusal changed', str(error))
                        else:
                            raise AssertionError(('assertion borrowed refusal escaped', case))
                        borrowed += 1
        finally:
            Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run, shutil.copytree, shutil.rmtree = originals
            os.environ.clear(); os.environ.update(saved); sys.argv = argv
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'assertion boundary controls changed sources'
    print('assertion boundaries:', len(cases), 'actual CLI refusals;', borrowed, 'borrowed pure refusals; no producing effects')



def provider_controls():
    """Use actual independent module loads, so a shared mock cannot hide class drift."""
    def check(text, fault=False):
        originals = (runpy.run_path, Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run)
        def effect(*args, **kwargs):
            raise AssertionError('assertion provider check produced effects')
        def load(file, *args, **kwargs):
            ns = originals[0](file, *args, **kwargs)
            if Path(file) == HELPER and fault:
                def fail(*args, **kwargs):
                    raise ns['PROFILE']['Refusal']('isolated delegated refusal')
                ns['load_reference'] = fail
            return ns
        stderr = io.StringIO(); saved, argv = dict(os.environ), sys.argv
        try:
            runpy.run_path = load
            Path.mkdir = os.mkdir = Path.write_text = Path.write_bytes = subprocess.run = effect
            ns = dict(__file__=str(ENTRY), __name__='__main__' if fault else 'borrowed_provider')
            with contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                if fault:
                    try:
                        exec(compile(text, str(ENTRY), 'exec'), ns)
                    except SystemExit as error:
                        assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — isolated delegated refusal\n', 'assertion delegated CLI refusal changed'
                    else:
                        raise AssertionError('assertion delegated CLI refusal escaped')
                else:
                    exec(compile(text, str(ENTRY), 'exec'), ns)
                    assert ns['PROFILE'] is ns['HELPER']['PROFILE'], 'assertion actual provider identity changed'
            assert dict(os.environ) == saved and sys.argv is argv, 'assertion provider call changed state'
        finally:
            runpy.run_path, Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run = originals
            os.environ.clear(); os.environ.update(saved); sys.argv = argv
    check(TEXT); check(TEXT, True)
    anchor = "PROFILE = HELPER['PROFILE']\n"
    assert TEXT.count(anchor) == 1
    try:
        check(TEXT.replace(anchor, '', 1))
    except AssertionError as error:
        assert str(error) == 'assertion actual provider identity changed'
    else:
        raise AssertionError('assertion provider adoption omission escaped')
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'assertion provider controls changed sources'
    print('assertion providers: two actual inert loads/delegated CLI refusal positives; one actual adoption red; state/source exact')


def main():
    for mode in ('import', 'plan', 'contracts', 'consumer', 'cli'):
        print('assertion capture:', mode, capture(mode))
    selectors = {name: str(file.relative_to(ROOT)) for name, file in (
        ('FORMULA_CONTRACT', BOOK / 'spec/formula-language.md'), ('FORMULA_GRAMMAR', BOOK / 'spec/formula-language/grammar.md'),
        ('FORMULA_EXAMPLES', BOOK / 'spec/formula-language/examples.md'), ('FORMULA_FIXTURE', BOOK / 'spec/reference-skirt.md'),
        ('FORMULA_PARTS_DIR', BOOK / 'spec/formula-language'))}
    selectors['FORMULA_ROOT'] = 'unused-root-preserved'
    capture('consumer', selectors=selectors)
    original_controls()
    provider_controls()
    physical_controls()
    boundary_controls()
    print('assertion controls: actual boundaries/original bodies/guard reds and physical refusals; real verdicts separate')


if __name__ == '__main__':
    main()
