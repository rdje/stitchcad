"""Actual first-phase entry captures; intercepted dispatch is separate from native proof."""
from pathlib import Path
import ast
import contextlib
import io
import json
import os
import runpy
import subprocess
import sys
import types

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
ENTRY = HERE / 'first_phase_contract.py'
COUPLED = HERE / 'static_coupled_contract.py'
PROFILE = ROOT / 'scripts/local_environment.py'
GOOD = runpy.run_path(str(PROFILE))
WORK = ROOT / 'target/static_coupled_contract'
OUTPUT = WORK / 'first-phase'
OUTPUT_FILES = tuple(OUTPUT / name for name in (
    'cargo-artifacts.json', 'cargo.log', 'rustc.log', 'cases.hex.tsv', 'driver.tsv',
    'driver.log', 'competing.tsv', 'competing.log', 'driver'))
BEFORE_REVISION = '2358d4e800b1755e146818b73b3df0421fb06576'
PUBLIC_INPUTS = (
    ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh',
    HERE / 'static_coupled_driver.rs', HERE / 'canonical_worked_recipe_cases.tsv',
    *(HERE / name for name in ('dimension_payload_contract.py', 'static_review_contract.py',
                              'static_namespace_contract.py', 'static_signature_contract.py',
                              'checked_recipe_mutations.py')),
    *(ROOT / 'crates/sc-core/src/recipe' / name for name in
      ('checked.rs', 'checked_statement.rs', 'namespace/ordered.rs', 'checked_recipe.rs')),
    *(ROOT / 'docs/book/src' / name for name in
      ('spec/formula-language.md', 'spec/formula-language/grammar.md',
       'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md',
       'annexes/formula-static-validation.md', 'annexes/formula-wanted-signatures.md',
       'annexes/implementation-status.md')),
)


def capture(text, *, mutations=False, late_source=False, output_alias=False, early_refusal=None, coupled_text=None):
    """Compile actual bodies; all entry writes/children intercepted after guarded local setup."""
    sources = (ENTRY, COUPLED, *PUBLIC_INPUTS)
    original = {path: path.read_bytes() for path in (*sources, PROFILE)}
    saved, argv = dict(os.environ), sys.argv
    base = 'target/scratch/python_bridge_contract/stores'
    custom = {name: str(ROOT / base / suffix) for name, suffix in
              (('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'),
               ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    # Setup is local and separate from captured effects. Actual entries below may only
    # request these validated existing directories; they never create fixtures or artifacts.
    for value in custom.values():
        GOOD['directory'](ROOT, Path(value).relative_to(ROOT).as_posix(), create=True)
    GOOD['directory'](ROOT, OUTPUT.relative_to(ROOT).as_posix(), create=True)
    library = Path(custom['CARGO_TARGET_DIR']) / 'deps/libsc_core_dispatch_fixture.rlib'
    dependency = library.parent / 'libdependency_dispatch_fixture.rmeta'
    GOOD['directory'](ROOT, library.parent.relative_to(ROOT).as_posix(), create=True)
    GOOD['plan_producer'](ROOT, output_files=(library, dependency))
    library.write_bytes(b'authored bridge library fixture; never compiled')
    dependency.write_bytes(b'authored bridge metadata fixture; never compiled')
    metadata = '\n'.join(json.dumps(message) for message in (
        {'reason': 'compiler-artifact', 'target': {'name': 'dependency', 'kind': ['lib']},
         'filenames': [str(dependency)]},
        {'reason': 'compiler-artifact', 'target': {'name': 'sc_core', 'kind': ['lib']},
         'filenames': [str(library)]})) + '\n'
    build_filenames = ('cargo-artifacts.json', 'cargo.log', 'rustc.log', 'cases.hex.tsv',
                       'driver.tsv', 'driver.log', 'driver')
    baseline_files = tuple(WORK / 'baseline' / name for name in build_filenames)
    sig = HERE / 'static_signature_contract.py'
    ns = HERE / 'static_namespace_contract.py'
    review = HERE / 'static_review_contract.py'
    profile_source = ROOT / 'scripts/local_environment.py'
    reference_source = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
    book_sources = tuple(ROOT / 'docs/book/src' / name for name in (
        'spec/formula-language.md', 'spec/formula-language/grammar.md',
        'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))
    signature_boot = (sig, profile_source, reference_source)
    namespace_boot = (ns, profile_source, reference_source, sig)
    review_boot = (review, profile_source, reference_source, ns, sig)
    readonly_expected = (signature_boot, review_boot, namespace_boot, signature_boot,
                         (review, ns, *signature_boot, *book_sources))
    readonly_plans = []
    post_native_reference_plans = []
    build_plans, library_plans, library_directories = [], [], []
    scope = {'__file__': str(ENTRY), '__name__': '__main__'}
    loaded, planned, entered, calls, writes, directories = [], [], [], [], {}, []
    coupled = {}
    original_run_path, original_run = runpy.run_path, subprocess.run
    mkdir, write_text, write_bytes, read_text, read_bytes, is_file, is_symlink = (
        Path.mkdir, Path.write_text, Path.write_bytes, Path.read_text, Path.read_bytes,
        Path.is_file, Path.is_symlink)

    def environment(values):
        assert all(values.get(name) == value for name, value in custom.items()), 'D156 bridge child stores lost'
        assert values.get('MAKE_TMPDIR') == custom['TMPDIR'], 'D156 bridge scratch alias lost'
        assert values.get('RUSTUP_TOOLCHAIN') == '1.99.0' and values.get('RUSTUP_AUTO_INSTALL') == '0', \
            'D156 bridge toolchain changed'
        assert values.get('D156_BRIDGE_UNRELATED') == 'preserved', 'D156 bridge unrelated value lost'

    def plan(root, *, directories=(), output_files=(), sources=(), mutable_sources=()):
        assert root == ROOT and not mutable_sources, 'D200 bridge pure plan has mutable authority'
        before = dict(os.environ)
        result = GOOD['plan_producer'](root, directories=directories, output_files=output_files, sources=sources)
        if not entered:
            assert not loaded and not calls and not writes, 'D156 bridge planning produced effects'
            assert directories == (OUTPUT,), 'D156 bridge pre-output plan changed'
            assert sources == (ENTRY, COUPLED), 'D156 bridge pre-source plan changed'
            assert output_files == OUTPUT_FILES, 'D156 bridge pre-file plan changed'
            assert not planned, 'D200 bridge duplicate module plan'
            planned.append(True)
        elif not directories and not output_files and sources == (*signature_boot, *book_sources):
            assert entered == ['coupled', 'bridge'] and len(calls) == 4 and len(writes) == 8, 'loader bridge reference plan was early'
            post_native_reference_plans.append(sources)
        elif not directories and not output_files and sources in readonly_expected:
            assert entered == ['coupled'] and not calls and not writes, 'loader bridge readonly plan was late'
            assert sources == readonly_expected[len(readonly_plans)], 'loader bridge readonly sequence changed'
            readonly_plans.append(sources)
        elif directories:
            assert entered == ['coupled', 'bridge'], 'D156 bridge directory before guard'
            assert not calls and not writes and not build_plans, 'D200 bridge build plan was late'
            assert directories == (OUTPUT,), 'D200 bridge build directory plan changed'
            assert output_files == tuple(OUTPUT / name for name in build_filenames), 'D200 bridge build file plan changed'
            assert sources == PUBLIC_INPUTS, 'D200 bridge build source plan changed'
            build_plans.append(True)
        else:
            assert entered == ['coupled', 'bridge'] and len(calls) == 1 and not library_plans, 'D200 bridge library plan was late'
            assert not output_files and sources == (library, dependency), 'D200 bridge library input plan changed'
            library_plans.append(True)
        assert dict(os.environ) == before, 'D200 bridge pure plan changed stores'
        return result

    def library_directory(root, relative, *, create=False, missing_ok=False):
        assert root == ROOT and not create and not missing_ok, 'D200 bridge library directory plan has effects'
        assert library_plans == [True], 'D200 bridge library plan omitted'
        assert len(calls) == 1 and not library_directories, 'D200 bridge library directory plan was late'
        assert ROOT / relative == library.parent, 'D200 bridge library directory changed'
        library_directories.append(True)
        return GOOD['directory'](root, relative)

    def enter(root, *, directories=(), output_files=(), sources=(), mutable_sources=()):
        assert root == ROOT and not mutable_sources, 'D200 bridge borrowed entry has mutable authority'
        if directories == (WORK,):
            assert planned == [True] and not entered and not calls and not writes, 'D156 bridge delegated guard order'
            assert sources == PUBLIC_INPUTS, 'D156 bridge delegated input plan changed'
            assert output_files == baseline_files, 'D200 bridge delegated file contract changed'
            role = 'coupled'
        else:
            assert root == ROOT and directories == (OUTPUT,), 'D156 bridge producing output plan changed'
            assert sources == (ENTRY, COUPLED, *PUBLIC_INPUTS), 'D156 bridge producing input plan changed'
            assert output_files == OUTPUT_FILES, 'D156 bridge producing file plan changed'
            assert entered == ['coupled'] and not calls and not writes, 'D156 bridge producing guard order'
            role = 'bridge'
        result = GOOD['enter_producer'](root, directories=directories, output_files=output_files, sources=sources,
                                         mutable_sources=mutable_sources)
        entered.append(role)
        environment(result)
        return result

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE:
            return dict(GOOD, plan_producer=plan, enter_producer=enter, directory=library_directory)
        if path == COUPLED:
            assert planned == [True], 'D156 bridge pre-plan omitted'
        else:
            assert entered == ['coupled'] and path in PUBLIC_INPUTS, 'D156 bridge unplanned helper load'
        loaded.append(path)
        if path == COUPLED and coupled_text is not None:
            result = {'__file__': str(COUPLED), '__name__': '<run_path>'}
            exec(compile(coupled_text, str(COUPLED), 'exec'), result)
        else:
            result = original_run_path(file, *args, **kwargs)
        if path == COUPLED:
            coupled.update(result)
        return result

    def directory(path, *args, **kwargs):
        assert path in (WORK, OUTPUT), 'D156 bridge unplanned directory'
        environment(os.environ)
        assert entered == (['coupled'] if path == WORK else ['coupled', 'bridge']), \
            'D156 bridge directory before guard'
        if path == OUTPUT:
            assert build_plans == [True], 'D200 bridge build plan omitted'
        directories.append(path)

    def write(path, content, *args, **kwargs):
        assert path.parent == OUTPUT and entered == ['coupled', 'bridge'], 'D156 bridge unplanned output'
        environment(os.environ)
        assert path not in original, 'D156 bridge source write'
        assert path not in writes, 'D156 bridge duplicate output'
        writes[path] = content
        return len(content)

    def read(path, binary, *args, **kwargs):
        assert path in original and entered, 'D156 bridge unplanned source read'
        return (read_bytes if binary else read_text)(path, *args, **kwargs)

    def run(arguments, *args, **kwargs):
        assert entered == ['coupled', 'bridge'], 'D156 bridge native before guard'
        assert not args and kwargs.get('cwd') == ROOT, 'D156 bridge native cwd changed'
        environment(kwargs.get('env', {}))
        assert kwargs.get('text') is True and kwargs.get('capture_output') is True, 'D156 bridge native capture changed'
        stage = len(calls)
        assert build_plans == [True], 'D200 bridge build plan omitted'
        if stage:
            assert library_plans == [True], 'D200 bridge library plan omitted'
            assert library_directories == [True], 'D200 bridge library directory plan omitted'
        calls.append(arguments)
        if stage == 0:
            assert arguments == ['cargo', 'build', '-p', 'sc-core', '--message-format=json'], \
                'D156 bridge Cargo argv changed'
            output = metadata
        elif stage == 1:
            assert arguments == ['rustc', '--edition=2021', '-Dwarnings', '--extern',
                                 'sc_core=' + str(library), '-L', 'dependency=' + str(library.parent),
                                 str(HERE / 'static_coupled_driver.rs'), '-o', str(OUTPUT / 'driver')], \
                'D156 bridge Rustc argv changed'
            output = ''
        elif stage in (2, 3):
            assert arguments == [str(OUTPUT / 'driver')], 'D156 bridge driver argv changed'
            population = coupled['CASES'] if stage == 2 else scope['CASES']
            mode_sources = [(mode, source) for mode, source, _ in population] if stage == 2 else [
                ('R', source) for source, _, _, _ in population]
            expected_input = ';'.join(':'.join(row) for row in coupled['CATALOG']) + '\n'
            expected_input += ''.join('%d\t%s\t%s\n' % (index, mode, source.encode().hex())
                                      for index, (mode, source) in enumerate(mode_sources))
            assert kwargs.get('input') == expected_input, 'D156 bridge input population changed'
            # Authored public metadata/results simulate dispatch only; real consumers prove outcomes.
            outcomes = [expected for _, _, expected in population] if stage == 2 else [
                ('ERR', token, 'STAGE:%s:%d' % (phase, ordinal)) for _, token, phase, ordinal in population]
            output = ''.join(str(index) + '\t' + '\t'.join(row) + '\n'
                             for index, row in enumerate(outcomes))
        else:
            raise AssertionError('D156 bridge unexpected native child')
        return types.SimpleNamespace(returncode=0, stdout=output, stderr='')

    def file(path):
        return False if late_source and path == PUBLIC_INPUTS[-1] else is_file(path)

    diagnostic = io.StringIO()
    try:
        os.environ.update(custom, RUSTUP_TOOLCHAIN='1.99.0', D156_BRIDGE_UNRELATED='preserved')
        for name in ('MAKE_TMPDIR', 'RUSTUP_AUTO_INSTALL'):
            os.environ.pop(name, None)
        sys.argv = [str(ENTRY), *(['--mutations'] if mutations else [])]
        runpy.run_path, subprocess.run = load, run
        Path.mkdir, Path.write_text, Path.write_bytes = directory, write, write
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.is_file = file
        Path.is_symlink = lambda path: (output_alias and path == OUTPUT / 'competing.log') or is_symlink(path)
        with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(diagnostic):
            try:
                exec(compile(text, str(ENTRY), 'exec'), scope)
            except SystemExit as error:
                assert error.code == 2 and (late_source or output_alias or early_refusal), 'D156 bridge unrelated entry refusal'
            else:
                assert not late_source and not output_alias and not early_refusal, 'D156 bridge accepted invalid plan'
    finally:
        runpy.run_path, subprocess.run = original_run_path, original_run
        Path.mkdir, Path.write_text, Path.write_bytes = mkdir, write_text, write_bytes
        Path.read_text, Path.read_bytes, Path.is_file = read_text, read_bytes, is_file
        Path.is_symlink = is_symlink
        sys.argv = argv
        os.environ.clear()
        os.environ.update(saved)
        assert all(path.read_bytes() == data for path, data in original.items()), 'D156 bridge tracked source changed'
    if late_source or output_alias or early_refusal:
        wanted = early_refusal or ('symlink producer output' if output_alias else 'producer source is not a local regular file')
        assert diagnostic.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', 'D156 bridge refusal identity'
        assert not calls and not writes and not directories and not entered, 'D156 bridge refused after effects'
    else:
        assert planned == [True] and entered == ['coupled', 'bridge'] and len(calls) == 4, 'D156 bridge guard/dispatch coverage'
        assert build_plans == library_plans == library_directories == [True], 'D200 bridge complete plan coverage'
        assert tuple(readonly_plans) == readonly_expected, 'loader bridge readonly coverage omitted'
        assert len(post_native_reference_plans) == (11 if mutations else 1), 'loader bridge reference coverage omitted'
        assert set(path.name for path in writes) == {
            'cargo-artifacts.json', 'cargo.log', 'rustc.log', 'cases.hex.tsv',
            'driver.tsv', 'driver.log', 'competing.tsv', 'competing.log'}, 'D156 bridge output coverage'
    return len(calls), len(writes)


def main():
    GOOD['enter_producer'](ROOT, sources=(Path(__file__), ENTRY, COUPLED, PROFILE, *PUBLIC_INPUTS))
    text = ENTRY.read_text()
    before = subprocess.check_output(['git', 'show', BEFORE_REVISION + ':' + ENTRY.relative_to(ROOT).as_posix()]).decode()
    try:
        capture(before)
    except AssertionError as error:
        assert str(error) == 'D156 bridge pre-plan omitted', 'D156 bridge unrelated original refusal'
    else:
        raise AssertionError('D156 bridge original gap not reproduced')
    for mode in (False, True):
        assert capture(text, mutations=mode) == (4, 8)
    capture(text, late_source=True)
    capture(text, output_alias=True)
    anchor = "WORK = ROOT / 'target/static_coupled_contract/first-phase'"
    assert text.count(anchor) == 1
    capture(text.replace(anchor, "WORK = ROOT.parent / 'unowned-bridge-output'", 1),
            early_refusal='producer path outside workspace')
    faults = (
        ('module plan omitted', "PROFILE['plan_producer'](ROOT, directories=(WORK,), output_files=OUTPUT_FILES,\n                             sources=(Path(__file__), COUPLED_SOURCE))", 'None', 'D156 bridge pre-plan omitted'),
        ('module output omitted', 'directories=(WORK,), output_files=OUTPUT_FILES,\n', 'directories=(), output_files=OUTPUT_FILES,\n', 'D156 bridge pre-output plan changed'),
        ('module source omitted', 'sources=(Path(__file__), COUPLED_SOURCE)', 'sources=(Path(__file__),)', 'D156 bridge pre-source plan changed'),
        ('module files omitted', 'output_files=OUTPUT_FILES,\n', 'output_files=(),\n', 'D156 bridge pre-file plan changed'),
        ('producing plan omitted', "PROFILE['enter_producer'](ROOT, directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES)", 'None', 'D156 bridge directory before guard'),
        ('producing source omitted', 'sources=SOURCES)', 'sources=SOURCES[:-1])', 'D156 bridge producing input plan changed'),
        ('producing output omitted', 'directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES)', 'directories=(), output_files=OUTPUT_FILES, sources=SOURCES)', 'D156 bridge producing output plan changed'),
        ('producing files omitted', 'output_files=OUTPUT_FILES, sources=SOURCES)', 'output_files=(), sources=SOURCES)', 'D156 bridge producing file plan changed'),
        ('prepared stores reset', 'env=dict(os.environ)', "env=dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'))", 'D156 bridge child stores lost'),
    )
    for label, anchor, replacement, message in faults:
        assert text.count(anchor) == 1, ('D156 actual bridge anchor', label)
        try:
            capture(text.replace(anchor, replacement, 1))
        except AssertionError as error:
            assert str(error) == message, (label, 'D156 unrelated bridge body fault', str(error))
        else:
            raise AssertionError(('D156 bridge body fault survived', label))
    coupled_text = COUPLED.read_text()
    tree = ast.parse(coupled_text)
    build = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'build_and_check')
    call = next(node for node in ast.walk(build) if isinstance(node, ast.Call)
                and ast.unparse(node.func) == "PROFILE['plan_producer']"
                and any(keyword.arg == 'directories' for keyword in node.keywords))
    build_plan = ast.get_source_segment(coupled_text, call)
    delegated_faults = [
        ('borrowed mutable authority', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=MUTATED',
         'D200 bridge borrowed entry has mutable authority'),
        ('borrowed build plan', build_plan, 'None', 'D200 bridge build plan omitted'),
        ('borrowed build sources', 'sources=SOURCES)', 'sources=())', 'D200 bridge build source plan changed'),
        ('current library plan', "PROFILE['plan_producer'](ROOT, sources=(library, *sorted(library_files - {library})))",
         'None', 'D200 bridge library plan omitted'),
        ('complete current libraries', 'sources=(library, *sorted(library_files - {library}))',
         'sources=(library,)', 'D200 bridge library input plan changed'),
        ('current library directory', "PROFILE['directory'](ROOT, path.relative_to(ROOT).as_posix())",
         'None', 'D200 bridge library directory plan omitted'),
    ]
    delegated_faults.extend(('baseline file omitted %d' % index, 'output_files=OUTPUT_FILES',
                             'output_files=OUTPUT_FILES[:%d] + OUTPUT_FILES[%d:]' % (index, index + 1),
                             'D200 bridge delegated file contract changed') for index in range(7))
    delegated_faults.extend(('borrowed build file omitted %d' % index,
                             'output_files=tuple(output_dir / name for name in BUILD_FILENAMES)',
                             'output_files=tuple(output_dir / name for name in BUILD_FILENAMES[:%d] + BUILD_FILENAMES[%d:])' % (index, index + 1),
                             'D200 bridge build file plan changed') for index in range(7))
    delegated_faults.extend(('coupled source omitted %d' % index, 'sources=SOURCES,',
                             'sources=SOURCES[:%d] + SOURCES[%d:],' % (index, index + 1),
                             'D156 bridge delegated input plan changed') for index in range(20))
    for label, before, after, message in delegated_faults:
        assert coupled_text.count(before) == 1, (label, 'D200 actual coupled anchor')
        try:
            capture(text, coupled_text=coupled_text.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == message, (label, 'D200 unrelated coupled body fault', str(error))
        else:
            raise AssertionError((label, 'D200 coupled body fault survived'))
    print('D200 delegated controls: %d actual coupled body reds; ordinary7/readonly20/per-build7/current library+metadata/directory plans; borrowed mutation argv has no mutable authority' % len(delegated_faults))
    print('bridge controls: original omitted plan red / two complete four-child eight-output captures / '
          'early output/file-alias and late delegated-source refusals / nine actual body reds; '
          'all nine file destinations planned; sources exact; native proof separate')


if __name__ == '__main__':
    main()
