"""Actual copied-book recipe dispatch controls; intercepted child results are separate from numerical proof."""
from pathlib import Path
import ast
import contextlib
import io
import inspect
import os
import runpy
import shutil
import subprocess
import sys
import types
ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'docs/tasks/artifacts/formula_structure/static_recipe_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
HERE = ENTRY.parent
BOOK = ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_structure/static_recipe'
GOOD = runpy.run_path(str(PROFILE_SOURCE))
BOOT = (ENTRY, PROFILE_SOURCE, SOURCE, HERE / 'static_namespace_contract.py', HERE / 'static_signature_contract.py')
GOOD['directory'](ROOT, 'docs/book/src')
def book_files():
    directories, files = [BOOK], []
    for directory in directories:
        GOOD['directory'](ROOT, directory.relative_to(ROOT).as_posix())
        for path in sorted(directory.iterdir()):
            GOOD['require'](not path.is_symlink(), 'symlink control book entry')
            (directories if path.is_dir() else files).append(path)
    return tuple(files)

BOOK_FILES = book_files()
WHOLE = set((*BOOT, *BOOK_FILES))
# Independently retained public L7 target population for the default and relative-selector captures.
CHILD_INPUTS = {BOOK / name for name in (
    'spec/formula-language.md',
    'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md',
    'spec/reference-skirt.md',
    'annexes/formula-syntax.md',
    'annexes/formula-literals.md',
    'annexes/formula-statements.md',
    'annexes/formula-recipe-inputs.md',
    'annexes/formula-static-validation.md',
    'annexes/formula-declarations.md',
    'annexes/formula-name-scopes.md',
    'annexes/formula-call-lookup.md',
    'spec/glossary.md',
    'spec/units-and-tolerances.md',
    'annexes/formula-runtime-validation.md',
    'spec/feature-matrix.md',
    'spec/i18n-architecture.md',
    'spec/size-sets.md',
    'governance.md',
    'annexes/formula-wanted-signatures.md',
    'annexes/formula-checked-recipes.md',
    'spec/ontology.md',
    'annexes/formula-rationals.md',
)}
LABELS = ('late_kind', 'late_name', 'per_recipe_measurement')
FILES = {WORK / label / 'src' / path.relative_to(BOOK) for label in LABELS for path in BOOK_FILES} | {WORK / (label + '.log') for label in LABELS} | {WORK / 'real.log'}
GOOD['plan_producer'](ROOT, sources=(*tuple(WHOLE), Path(__file__)))
FROZEN = {p: p.read_bytes() for p in WHOLE}
DATA = ENTRY.read_text()

def capture(text, mutations=False, borrowed=None, selectors=None):
    """Guard all actual fixture copies/writes/deletions; intercept the sole shell child."""
    saved = dict(os.environ)
    argv = sys.argv
    custom = {name: str(ROOT / 'target/scratch/recipe_capture' / suffix) for name, suffix in (('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in custom.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    plans = []
    prepared = []
    copies = []
    removed = []
    written = []
    children = []
    child_checkpoint = [0]
    loaded = []
    real_run_path = runpy.run_path
    real_run = subprocess.run
    real_rmtree = shutil.rmtree
    real_copytree = shutil.copytree
    real_copy2 = shutil.copy2
    real_read_bytes = Path.read_bytes
    real_read = Path.read_text
    real_write = Path.write_text
    real_write_bytes = Path.write_bytes
    real_mkdir = os.mkdir

    def plan(root, **kwargs):
        assert root == ROOT and (not kwargs.get('mutable_sources')), 'recipe tracked mutation authority'
        result = GOOD['plan_producer'](root, **kwargs)
        plans.append(kwargs)
        return result

    def prepare(root, **kwargs):
        assert root == ROOT and (not kwargs.get('mutable_sources')), 'recipe tracked mutation authority'
        assert WHOLE <= set(kwargs.get('sources', ())), 'recipe producing source plan incomplete'
        output = set(kwargs.get('output_files', ()))
        assert output and output <= FILES, 'recipe producing files changed'
        frame = sys._getframe(1).f_locals
        labels, real = frame['labels'], frame['real']
        assert set(labels) <= set(LABELS), 'recipe unowned labels'
        expected = {WORK / label / 'src' / q.relative_to(BOOK) for label in labels for q in BOOK_FILES} | {WORK / (label + '.log') for label in labels} | ({WORK / 'real.log'} if real else set())
        assert output == expected, 'recipe producing file plan incomplete'
        assert any(set(p.get('output_files', ())) >= output and WHOLE <= set(p.get('sources', ())) for p in plans), 'recipe activation before pure plan'
        expected_dirs = {WORK} | {WORK / label / 'src' / directory.relative_to(BOOK) for label in labels for directory in (BOOK, *(p for p in BOOK.rglob('*') if p.is_dir()))}
        assert set(kwargs.get('directories', ())) == expected_dirs, 'recipe producing directory plan incomplete'
        prepared.append(kwargs)
        return GOOD['prepare_producer'](root, **kwargs)

    def authorized(path):
        assert any((path in set(p.get('output_files', ())) for p in prepared)), 'recipe output before complete producing plan'
        assert path in FILES and path not in WHOLE, 'recipe unowned output'
        assert all((os.environ.get(k) == v for k, v in custom.items())), 'recipe stores lost'

    def mkdir(path, *args, **kwargs):
        path = Path(path)
        assert any((path == WORK or path.is_relative_to(WORK / label) for label in LABELS)), 'recipe directory outside fixture plan'
        assert prepared, 'recipe directory before producing plan'
        return real_mkdir(path, *args, **kwargs)

    def remove(path, *args, **kwargs):
        path = Path(path)
        assert path in {WORK / label / 'src' for label in LABELS}, 'recipe delete outside fixture plan'
        expected = {path / q.relative_to(BOOK) for q in BOOK_FILES}
        assert any((expected <= set(p.get('output_files', ())) for p in prepared)), 'recipe deletion before complete file plan'
        assert set(plans[-1].get('output_files', ())) == expected | {WORK / (path.parent.name + '.log')} and WHOLE <= set(plans[-1].get('sources', ())), 'recipe deletion recheck omitted'
        removed.append(path)
        return real_rmtree(path, *args, **kwargs)

    def copyfile(source, destination):
        source, destination = (Path(source), Path(destination))
        assert source in BOOK_FILES, 'recipe unplanned copy input'
        authorized(destination)
        GOOD['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        written.append(destination)
        return real_copy2(source, destination)

    def copytree(source, destination, *args, **kwargs):
        source, destination = (Path(source), Path(destination))
        if source == BOOK:
            assert not args and (not kwargs), 'recipe copy interface changed'
            assert destination in {WORK / label / 'src' for label in LABELS}, 'recipe unowned copy tree'
            copies.append(destination)
            return real_copytree(source, destination, copy_function=copyfile)
        assert source.is_relative_to(BOOK) and destination in {WORK / label / 'src' / source.relative_to(BOOK) for label in LABELS}, 'recipe recursive copy outside plan'
        bound = inspect.signature(real_copytree).bind(source, destination, *args, **kwargs)
        bound.apply_defaults()
        options = bound.arguments
        assert options['copy_function'] is copyfile and options['ignore'] is None, 'recipe recursive copier changed'
        assert not any(options[name] for name in ('symlinks', 'ignore_dangling_symlinks', 'dirs_exist_ok')), 'recipe recursive copy options changed'
        return real_copytree(source, destination, *args, **kwargs)

    def readbytes(path, *args, **kwargs):
        if path == SOURCE and borrowed is None and not prepared:
            assert any(FILES <= set(p.get('output_files', ())) and WHOLE <= set(p.get('sources', ())) for p in plans), 'recipe initial full plan omitted'
        return real_read_bytes(path, *args, **kwargs)

    def read(path, *args, **kwargs):
        if path.is_relative_to(WORK) and '/src/' in str(path):
            base = WORK / path.relative_to(WORK).parts[0] / 'src'
            copied = {base / q.relative_to(BOOK) for q in BOOK_FILES}
            assert any(copied <= set(p.get('sources', ())) and copied <= set(p.get('output_files', ())) for p in plans), 'recipe copied consumer source plan omitted'
        return real_read(path, *args, **kwargs)

    def write(path, data, *args, **kwargs):
        authorized(path)
        written.append(path)
        if path == WORK / 'late_name.log':
            child_checkpoint[0] = len(plans)
        return real_write(path, data, *args, **kwargs)

    def writebytes(path, data, *args, **kwargs):
        raise AssertionError('recipe unexpected byte write')

    def load(file, *args, **kwargs):
        if Path(file) == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, prepare_producer=prepare)
        if Path(file) == BOOT[3]:
            assert any((tuple(p.get('sources', ())) == BOOT for p in plans)), 'recipe bootstrap plan omitted'
            loaded.append(Path(file))
        result = real_run_path(file, *args, **kwargs)
        if Path(file) == BOOT[3]:
            loader = dict(result['LOADER'])
            original_loader = loader['load_reference']
            def reference_loader(*args, **kwargs):
                expected = set(BOOT) | {BOOK / p for p in ('spec/formula-language.md', 'spec/formula-language/grammar.md', 'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md')}
                assert set(plans[-1].get('sources', ())) == expected, 'recipe pure reference plan incomplete'
                return original_loader(*args, **kwargs)
            loader['load_reference'] = reference_loader
            result['LOADER'] = loader
        return result

    def child(arguments, *args, **kwargs):
        assert arguments == ['bash', str(SOURCE)] and (not args) and (kwargs.get('cwd') == ROOT), 'recipe child interface changed'
        assert kwargs.get('capture_output') is True and kwargs.get('text') is True, 'recipe child capture changed'
        env = kwargs.get('env', {})
        assert all((env.get(k) == v for k, v in custom.items())) and env.get('MAKE_TMPDIR') == custom['TMPDIR'], 'recipe child stores lost'
        assert env.get('FORMULA_BOOK') == str(BOOK) and env.get('RECIPE_UNRELATED') == 'preserved', 'recipe child selectors lost'
        assert all(env.get(k) == v for k, v in (selectors or {}).items()), 'recipe caller selectors changed'
        assert any(WHOLE <= set(p.get('sources', ())) for p in plans[child_checkpoint[0]:]), 'recipe child source recheck omitted'
        assert set(plans[-1].get('sources', ())) == CHILD_INPUTS, 'recipe child selector recheck omitted'
        children.append(arguments)
        return types.SimpleNamespace(returncode=0, stdout='statically accepted statements: 21\n17 bindings / 4 assertions / 13 refusals / 0 mismatch(es)\n', stderr='')
    try:
        for key in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'FORMULA_PARTS_DIR'):
            os.environ.pop(key, None)
        os.environ.update(custom, RECIPE_UNRELATED='preserved', RUSTUP_TOOLCHAIN='1.99.0')
        os.environ.update(selectors or {})
        sys.argv = [str(ENTRY), *(['--mutations'] if mutations else [])]
        runpy.run_path = load
        subprocess.run = child
        shutil.rmtree = remove
        shutil.copytree = copytree
        Path.read_bytes = readbytes
        Path.read_text = read
        Path.write_text = write
        Path.write_bytes = writebytes
        os.mkdir = mkdir
        namespace = {'__file__': str(ENTRY), '__name__': '__main__' if borrowed is None else 'borrowed_recipe'}
        with contextlib.redirect_stdout(io.StringIO()):
            exec(compile(text, str(ENTRY), 'exec'), namespace)
            if borrowed == 'pure':
                assert namespace['contracts'](verbose=False) == 196
            elif borrowed == 'consumer':
                assert namespace['consumer_contracts'](verbose=False) == 2
            elif borrowed == 'measurement':
                namespace['measurement_contract']()
    finally:
        runpy.run_path = real_run_path
        subprocess.run = real_run
        shutil.rmtree = real_rmtree
        shutil.copytree = real_copytree
        Path.read_bytes = real_read_bytes
        Path.read_text = real_read
        Path.write_text = real_write
        Path.write_bytes = real_write_bytes
        os.mkdir = real_mkdir
        sys.argv = argv
        os.environ.clear()
        os.environ.update(saved)
        assert all((p.read_bytes() == data for p, data in FROZEN.items())), 'recipe tracked input changed'
    labels = () if borrowed == 'pure' else ('late_kind', 'late_name') if borrowed == 'consumer' else ('per_recipe_measurement',) if borrowed == 'measurement' else LABELS
    expected_files = {WORK / label / 'src' / q.relative_to(BOOK) for label in labels for q in BOOK_FILES} | {WORK / (label + '.log') for label in labels} | ({WORK / 'real.log'} if borrowed is None else set())
    assert len(children) == (1 if borrowed is None else 0) and set(written) == expected_files and loaded == [BOOT[3]], 'recipe full dispatch coverage'
    assert len(copies) == len(removed) == (6 if mutations else len(labels)), 'recipe copy/delete population changed'
    assert len(written) == (6 * len(BOOK_FILES) + 11 if mutations else len(labels) * (len(BOOK_FILES) + 2) + (1 if borrowed is None else 0)), 'recipe total write population changed'
    if borrowed == 'pure':
        expected = set(BOOT) | {BOOK / p for p in ('spec/formula-language.md', 'spec/formula-language/grammar.md', 'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md')}
        signature_inputs = {BOOT[4], PROFILE_SOURCE, SOURCE} | (expected - set(BOOT))
        assert any(set(p.get('sources', ())) == expected for p in plans), 'recipe pure reference plan changed'
        assert not prepared and not copies and not written and not children and set(plans[-1]['sources']) == signature_inputs, 'recipe pure reference plan changed'
    print('actual recipe captured', borrowed or mutations, len(copies), len(written), len(children), 'tracked inputs exact; no numerical result inferred')



def physical_controls():
    """Refuse real filesystem faults in owned isolated copies before any producer effect."""
    cases = ('missing input', 'symlink input', 'symlink book directory', 'input is directory',
             'unexpected old fixture', 'symlink output', 'hardlinked output',
             'output directory is file', 'symlink log', 'outside selector',
             'input output alias', 'parts directory alias', 'symlink store', 'invalid toolchain', 'pure unused sidebar')
    def effect(*args, **kwargs):
        raise AssertionError('recipe physical effect before refusal')
    generation = ROOT / 'target/fixture-recipe-controls' / str(os.getpid())
    GOOD['directory'](ROOT, generation.parent.relative_to(ROOT).as_posix(), create=True)
    assert not generation.exists(), 'recipe fixture generation already exists'
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
        leaf = work / 'late_kind/src/spec/formula-language.md'
        log = work / 'real.log'
        sidebar = sandbox / 'docs/book/src/annexes/formula-static-validation.md'
        input_file = sandbox / 'docs/book/src/spec/units-and-tolerances.md'
        extra = work / 'late_kind/src/unexpected.txt'
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
        elif case == 'input is directory':
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
            base = work / 'late_kind/src'
            blocker = base / 'annexes'
            GOOD['plan_producer'](ROOT, output_files=(blocker,))
            blocker.write_text('owned directory refusal sentinel\n')
            expected = 'unexpected copied-book fixture entry'
        elif case == 'symlink log':
            log.symlink_to(input_file); expected = 'symlink producer output'
        elif case == 'outside selector':
            selectors['FORMULA_CONTRACT'] = str(ENTRY); expected = 'producer path outside workspace'
        elif case == 'input output alias':
            leaf.write_text('owned selected-input alias sentinel\n')
            selectors['FORMULA_CONTRACT'] = str(leaf)
            expected = 'copied-book output aliases selected input'
        elif case == 'parts directory alias':
            parts = work / 'late_kind/src/spec/formula-language'
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
        originals = (os.mkdir, shutil.rmtree, shutil.copytree, subprocess.run, Path.write_text, Path.write_bytes)
        stderr = io.StringIO()
        try:
            for key in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR',
                        'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'FORMULA_PARTS_DIR', 'RUSTUP_TOOLCHAIN'):
                os.environ.pop(key, None)
            os.environ.update({name: str(sandbox / suffix) for name, suffix in (
                ('CARGO_HOME', 'target/cargo-home'), ('RUSTUP_HOME', 'target/rustup'),
                ('CARGO_TARGET_DIR', 'target/build'), ('TMPDIR', 'target/scratch'))})
            os.environ.update(selectors)
            os.mkdir = shutil.rmtree = shutil.copytree = subprocess.run = Path.write_text = Path.write_bytes = effect
            namespace = {'__file__': str(mirrors[ENTRY]), '__name__': 'borrowed_recipe'}
            with contextlib.redirect_stderr(stderr):
                exec(compile(DATA, str(mirrors[ENTRY]), 'exec'), namespace)
                if expected is None:
                    assert namespace['contracts'](verbose=False) == 196
                else:
                    try:
                        namespace['main']()
                    except namespace['PROFILE']['Refusal'] as error:
                        assert str(error) == expected, (case, 'unrelated physical refusal', str(error))
                        if case == 'parts directory alias':
                            guard = 'not any(parts == base or parts.is_relative_to(base) for base in bases)'
                            assert DATA.count(guard) == 1
                            disabled = dict(__file__=str(mirrors[ENTRY]), __name__='directory_guard_disabled')
                            exec(compile(DATA.replace(guard, 'True', 1), str(mirrors[ENTRY]), 'exec'), disabled)
                            disabled['fixture_plan'](('late_kind', 'late_name'), real=True, activate=False)
                            assert not tuple(parts.iterdir()), 'D202 counterfactual changed directory'
                            print('D202 actual directory-alias guard omission admits same empty selected directory; no effects')

                    else:
                        raise AssertionError((case, 'recipe physical refusal escaped'))
        except SystemExit as error:
            assert error.code == 2 and expected and expected in stderr.getvalue(), (case, 'unrelated bootstrap refusal', stderr.getvalue())
        finally:
            os.mkdir, shutil.rmtree, shutil.copytree, subprocess.run, Path.write_text, Path.write_bytes = originals
            os.environ.clear(); os.environ.update(saved)
        print('recipe physical control:', case)
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
        namespace = {'__file__': str(sandbox / ENTRY.relative_to(ROOT)), '__name__': 'disabled_recipe'}
        exec(compile(DATA, namespace['__file__'], 'exec'), namespace)
        namespace['fixture_plan'](LABELS, real=True, activate=False)
        assert (sandbox / 'target/formula_structure/static_recipe/late_kind/src/spec/formula-language.md').stat().st_nlink == 2
    finally:
        profile.write_text(original)
        os.environ.clear(); os.environ.update(saved)
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'recipe physical controls changed tracked sources'
    print('recipe guard counterfactual: actual profile planner disabled admits same hardlinked output; tracked sources exact')


def fault_controls():
    original = subprocess.check_output(['git', 'show', 'aea883c78b36ff3b5828da981c385966b9474618:' + ENTRY.relative_to(ROOT).as_posix()], text=True)
    try:
        capture(original)
    except AssertionError as error:
        assert str(error) == 'recipe bootstrap plan omitted', ('unrelated original recipe red', str(error))
    else:
        raise AssertionError('original unguarded recipe escaped')
    old_tree, new_tree = ast.parse(original), ast.parse(DATA)
    old_contract = next(n for n in old_tree.body if isinstance(n, ast.FunctionDef) and n.name == 'contracts')
    new_contract = next(n for n in new_tree.body if isinstance(n, ast.FunctionDef) and n.name == 'contracts')
    assert ast.dump(ast.Module(body=old_contract.body, type_ignores=[])) == ast.dump(ast.Module(body=new_contract.body[1:], type_ignores=[])), 'original 196 recipe body changed'
    assignments = lambda tree: next(n.value for n in tree.body if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'FAULTS' for t in n.targets))
    assert ast.dump(assignments(old_tree)) == ast.dump(assignments(new_tree)), 'original eleven recipe faults changed'
    faults = (
        ('initial plan', "fixture_plan(('late_kind', 'late_name', 'per_recipe_measurement'), real=True, activate=False)", 'None', 'recipe initial full plan omitted'),
        ('pure plan', "PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)", 'None', 'recipe pure reference plan incomplete'),
        ('child selectors', '        consumer_inputs(environment)\n        result = subprocess.run', '        result = subprocess.run', 'recipe child selector recheck omitted'),
        ('bootstrap', "PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", 'None', 'recipe bootstrap plan omitted'),
        ('producing sources', 'plan = dict(directories=directories, output_files=outputs, sources=sources)',
         'plan = dict(directories=directories, output_files=outputs, sources=sources[:-1])', 'recipe producing source plan incomplete'),
        ('producing files', 'plan = dict(directories=directories, output_files=outputs, sources=sources)',
         'plan = dict(directories=directories, output_files=outputs[:-1], sources=sources)', 'recipe producing file plan incomplete'),
        ('producing directories', 'plan = dict(directories=directories, output_files=outputs, sources=sources)',
         'plan = dict(directories=directories[:-1], output_files=outputs, sources=sources)', 'recipe producing directory plan incomplete'),
        ('activation', "return PROFILE['prepare_producer'](ROOT, **plan) if activate else None",
         "return PROFILE['environment_for'](ROOT, os.environ, create=False) if activate else None", 'recipe deletion before complete file plan'),
        ('delete recheck', 'fixture_plan((label,), activate=False)', 'None', 'recipe deletion recheck omitted'),
        ('copied inputs', "PROFILE['plan_producer'](ROOT, sources=(SOURCE, *files), output_files=files)", 'None', 'recipe copied consumer source plan omitted'),
        ('child recheck', '        source_plan()\n        consumer_inputs(environment)', '        consumer_inputs(environment)', 'recipe child source recheck omitted'),
        ('child stores', 'env=dict(environment, FORMULA_BOOK=str(BOOK))',
         "env=dict(environment, FORMULA_BOOK=str(BOOK), TMPDIR=str(ROOT / 'target/wrong-scratch'))", 'recipe child stores lost'),
    )
    for label, anchor, replacement, diagnostic in faults:
        assert DATA.count(anchor) == 1, ('recipe actual fault anchor changed', label)
        try:
            borrowed = 'consumer' if label in ('producing sources', 'producing files', 'producing directories', 'activation', 'delete recheck', 'copied inputs') else None
            capture(DATA.replace(anchor, replacement, 1), borrowed=borrowed)
        except AssertionError as error:
            assert str(error) == diagnostic, (label, 'unrelated actual recipe fault', str(error))
        else:
            raise AssertionError(('recipe actual fault escaped', label))
    print('recipe original gap red / original196 body and11 fault AST exact / %d actual guard body reds' % len(faults))


def main():
    for mode in (False, True):
        capture(DATA, mode)
    for borrowed in ('pure', 'consumer', 'measurement'):
        capture(DATA, borrowed=borrowed)
    selectors = {name: str(path.relative_to(ROOT)) for name, path in (
        ('FORMULA_CONTRACT', BOOK / 'spec/formula-language.md'),
        ('FORMULA_GRAMMAR', BOOK / 'spec/formula-language/grammar.md'),
        ('FORMULA_EXAMPLES', BOOK / 'spec/formula-language/examples.md'),
        ('FORMULA_FIXTURE', BOOK / 'spec/reference-skirt.md'),
        ('FORMULA_PARTS_DIR', BOOK / 'spec/formula-language'))}
    selectors['FORMULA_ROOT'] = 'unused-root-preserved'
    capture(DATA, selectors=selectors)
    fault_controls()
    physical_controls()
    print('recipe fixture controls: six actual dispatches / borrowed pure196 / twelve actual guard reds / fourteen physical refusals / pure unused sidebar / actual guard-disable admission; intercepted child metrics are authored, numerical proof separate')

if __name__ == '__main__':
    main()
