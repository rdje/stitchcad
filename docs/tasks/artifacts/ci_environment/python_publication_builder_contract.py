"""Actual publication builder, dynamic output authority and temporary lifecycle controls."""
from pathlib import Path
import ast
import contextlib
import csv
import hashlib
import io
import json
import os
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'docs/tasks/artifacts/book_publication/book_publication.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
CORE = (ENTRY, PROFILE_SOURCE, Path(__file__))
WORK = ROOT / 'target/publication-builder-controls' / str(os.getpid())
PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(WORK,))


def module(script=None):
    namespace = dict(__file__=str(ENTRY), __name__='publication_builder_control')
    exec(compile(ENTRY.read_text() if script is None else script, str(ENTRY), 'exec'), namespace)
    return namespace


def before_control():
    """Trap the real old dispatcher before mdBook can write anything."""
    source = subprocess.check_output(['git', 'show', '8e497de:' + ENTRY.relative_to(ROOT).as_posix()], text=True)
    node = next(node for node in ast.parse(source).body if isinstance(node, ast.FunctionDef) and node.name == 'build')
    namespace = module()
    exec(compile(ast.Module(body=[node], type_ignores=[]), str(ENTRY), 'exec'), namespace)
    original, saved = subprocess.run, dict(os.environ)
    seen = []

    def child(argv, **kwargs):
        assert argv == ['mdbook', 'build', 'docs/book'] and kwargs['cwd'] == ROOT
        assert not any(name in kwargs['env'] for name in PROFILE['STORES'])
        assert kwargs['env']['MDBOOK_LOG'] == kwargs['env']['RUST_LOG'] == 'warn'
        seen.append(argv)
        raise RuntimeError('trapped ambient mdBook child')

    try:
        for name in PROFILE['STORES']:
            os.environ.pop(name, None)
        subprocess.run = child
        try:
            namespace['build'](ROOT)
        except RuntimeError as error:
            assert str(error) == 'trapped ambient mdBook child'
        else:
            raise AssertionError('publication original dispatcher not trapped')
        assert len(seen) == 1
    finally:
        subprocess.run = original
        os.environ.clear(); os.environ.update(saved)
    print('original actual mdBook argv/cwd/warnings retained; dispatcher lacks all four local stores; child trapped before effects')


def book(label):
    root = WORK / label
    config, summary, chapter, stale = (root / path for path in (
        'docs/book/book.toml', 'docs/book/src/SUMMARY.md', 'docs/book/src/intro.md', 'docs/book/book/stale.css'))
    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=tuple({path.parent for path in (config, summary, chapter, stale)}),
                                output_files=(config, summary, chapter, stale))
    config.write_bytes((ROOT / 'docs/book/book.toml').read_bytes())
    summary.write_text('# Summary\n\n- [Introduction](intro.md)\n')
    chapter.write_text('# Introduction\n\nA controlled publication.\n')
    stale.write_text('old output\n')
    return root


def snapshot(root):
    return {path.relative_to(root).as_posix(): ('link', os.readlink(path)) if path.is_symlink()
            else ('file', path.read_bytes()) if path.is_file() else ('directory',)
            for path in root.rglob('*')}


def capture(label, script=None, *, result='success'):
    root, ns = book(label), module(script)
    html = root / 'docs/book/book'
    if result == 'shape-change':
        folder, file = html / 'intro.html', html / 'css'
        PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(folder,), output_files=(folder / 'old.txt', file))
        (folder / 'old.txt').write_text('old directory output')
        file.write_text('old file output')
    inputs = {root / 'docs/book/book.toml', root / 'docs/book/src/SUMMARY.md', root / 'docs/book/src/intro.md'}
    frozen = {path: path.read_bytes() for path in (*CORE, *inputs)}
    before = snapshot(html)
    originals = subprocess.run, shutil.rmtree, shutil.copytree, Path.mkdir
    real_profile = ns['PROFILE']
    saved = dict(os.environ)
    stores = {name: str(WORK / 'caller-stores' / name.lower()) for name in PROFILE['STORES']}
    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=tuple(Path(value) for value in stores.values()))
    git_values = dict(GIT_DIR=str(WORK / 'unrelated-git'), GIT_INDEX_FILE=str(WORK / 'unrelated-index'),
                      GIT_CONFIG_COUNT='1', GIT_CONFIG_KEY_0='core.worktree', GIT_CONFIG_VALUE_0=str(WORK))
    os.environ.update(stores, PUBLICATION_BUILDER_CALLER='preserved', **git_values)
    planned, activated, rendered, published, removed = [], [], [], [], []
    declared_files, declared_inputs, declared_dirs = set(), set(), set()
    latest, checkpoint = {}, 0
    stages = set()

    def direct():
        return sys._getframe(2).f_globals is ns

    def plan(selected_root, **kwargs):
        assert selected_root == ROOT
        declared_inputs.update(kwargs.get('sources', ()))
        declared_dirs.update(kwargs.get('directories', ()))
        declared_files.update(kwargs.get('output_files', ()))
        planned.append(kwargs)
        latest.clear(); latest.update(kwargs)
        return real_profile['plan_producer'](selected_root, **kwargs)

    def environment(selected_root, variables, *, create=True):
        if create:
            assert inputs <= declared_inputs, 'publication builder inputs omitted before activation'
            activated.append(True)
        return real_profile['environment_for'](selected_root, variables, create=create)

    def child(argv, **kwargs):
        if argv[:1] != ['mdbook']:
            environment = kwargs['env']
            assert not set(git_values) & set(environment), 'publication index query inherited unrelated Git authority'
            assert environment['GIT_CONFIG_NOSYSTEM'] == '1' and environment['GIT_CONFIG_GLOBAL'] == os.devnull
            assert environment['GIT_OPTIONAL_LOCKS'] == '0' and environment['GIT_LITERAL_PATHSPECS'] == '1'
            return originals[0](argv, **kwargs)
        assert argv == ['mdbook', 'build', 'docs/book'] and kwargs['cwd'] == root
        assert inputs <= declared_inputs and activated, 'publication builder unprepared dispatch'
        environment = kwargs['env']
        assert not set(git_values) & set(environment), 'publication renderer inherited unrelated Git authority'
        assert all(environment.get(key) == value for key, value in stores.items()), 'publication builder caller stores reset'
        assert environment['PUBLICATION_BUILDER_CALLER'] == 'preserved'
        assert environment['MDBOOK_LOG'] == environment['RUST_LOG'] == 'warn'
        assert environment.get('MDBOOK_BUILD__CREATE_MISSING') == 'false', 'publication builder source creation enabled'
        stage = Path(json.loads(environment['MDBOOK_BUILD__BUILD_DIR']))
        assert stage.is_relative_to(ROOT / 'target/scratch') and stage in stages
        assert stage in declared_dirs, 'publication stage outside directory plan'
        files = stage / 'intro.html', stage / 'css/generated.css'
        PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(files[1].parent,), output_files=files)
        if result != 'missing-output':
            files[0].write_text('<h1 id="intro">Controlled</h1>')
        files[1].write_text('body { color: black; }')
        if result == 'input-change':
            selected = root / 'docs/book/src/intro.md'
            PROFILE['plan_producer'](ROOT, sources=(*CORE, selected), mutable_sources=(selected,), output_files=(selected,))
            selected.write_text('controlled changed source\n')
        if result == 'topology-change':
            PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(root / 'docs/book/src/unexpected-empty',))
        rendered.append(stage)
        return subprocess.CompletedProcess(argv, 1 if result == 'failure' else 0,
                                           'controlled builder failure' if result == 'failure' else '',
                                           '\x1b[33m WARN malformed generic\x1b[0m\n' if result == 'warning' else '')

    def mkdir(path, *args, **kwargs):
        if direct():
            assert path in declared_dirs and activated, 'publication temporary creation without plan'
            stages.add(path)
        return originals[3](path, *args, **kwargs)

    def ready(stage):
        nonlocal checkpoint
        generated = {path for path in stage.rglob('*') if path.is_file()}
        assert generated <= set(latest.get('output_files', ())), 'publication generated file plan omitted'
        assert generated <= set(latest.get('sources', ())), 'publication generated input plan omitted'
        assert len(planned) > checkpoint, 'publication effect recheck omitted'
        checkpoint = len(planned)
        assert inputs <= declared_inputs

    def remove(path, *args, **kwargs):
        if direct():
            if path == html:
                assert len(rendered) == 1
                ready(rendered[0]); removed.append(path)
            else:
                assert path in stages, 'publication teardown outside temporary ownership'
                ready(path)
        return originals[1](path, *args, **kwargs)

    def copy(source, destination, *args, **kwargs):
        if direct():
            assert Path(source) in stages and Path(destination) == html
            ready(Path(source)); published.append(Path(destination))
        return originals[2](source, destination, *args, **kwargs)

    try:
        ns['PROFILE'] = dict(real_profile, plan_producer=plan, environment_for=environment)
        subprocess.run, shutil.rmtree, shutil.copytree, Path.mkdir = child, remove, copy, mkdir
        if result in ('success', 'shape-change'):
            assert ns['build'](root) == ''
            assert published == removed == [html]
            assert {path.relative_to(html).as_posix() for path in html.rglob('*') if path.is_file()} == {'intro.html', 'css/generated.css'}
        else:
            try:
                ns['build'](root)
            except ns['PublicationError'] as error:
                expected = ('BOOK_BUILD_WARNING:' if result == 'warning' else
                            'BOOK_AUTHORITY:' if result in ('input-change', 'topology-change') else 'BOOK_BUILD:')
                assert str(error).startswith(expected), error
            else:
                raise AssertionError('publication builder failure accepted')
            assert not published and not removed and snapshot(html) == before
            if result == 'input-change':
                selected = root / 'docs/book/src/intro.md'
                PROFILE['plan_producer'](ROOT, sources=(*CORE, selected), mutable_sources=(selected,), output_files=(selected,))
                selected.write_bytes(frozen[selected])
        assert rendered and all(not stage.exists() for stage in stages)
        assert all(path.read_bytes() == data for path, data in frozen.items())
        assert all(os.environ[key] == value for key, value in stores.items())
        assert os.environ['PUBLICATION_BUILDER_CALLER'] == 'preserved'
        assert all(os.environ[key] == value for key, value in git_values.items())
    finally:
        subprocess.run, shutil.rmtree, shutil.copytree, Path.mkdir = originals
        os.environ.clear(); os.environ.update(saved)
        if result == 'input-change':
            selected = root / 'docs/book/src/intro.md'
            if selected.read_bytes() != frozen[selected]:
                PROFILE['plan_producer'](ROOT, sources=(*CORE, selected), mutable_sources=(selected,), output_files=(selected,))
                selected.write_bytes(frozen[selected])
        extra = root / 'docs/book/src/unexpected-empty'
        if result == 'topology-change' and extra.exists():
            PROFILE['plan_producer'](ROOT, sources=CORE, directories=(extra,))
            extra.rmdir()
        for stage in stages:
            if stage.exists():
                folders, files = ns['source_tree'](stage)
                PROFILE['plan_producer'](ROOT, sources=(*CORE, *files), directories=folders, output_files=files)
                shutil.rmtree(stage)
        assert all(path.read_bytes() == data for path, data in frozen.items() if path in CORE)
    print('publication builder capture:', result, len(planned), 'plans; original argv/cwd/warnings; complete inputs/generated files; exact source/store bytes; temporary cleanup')


def physical_controls():
    cases = ('source-link', 'source-missing', 'source-marker', 'source-device', 'output-link', 'output-hardlink', 'output-device',
             'output-parent-link', 'output-marker', 'output-directory-file', 'config-output',
             'config-command', 'environment-output', 'environment-renderer', 'theme', 'include', 'missing-chapter', 'toolchain')
    for label in cases:
        root, ns = book('physical-' + label), module()
        source, output = root / 'docs/book/src/intro.md', root / 'docs/book/book/stale.css'
        config = root / 'docs/book/book.toml'
        saved = dict(os.environ)
        if label == 'source-link':
            source.unlink(); source.symlink_to(ROOT / 'docs/book/src/introduction.md')
        elif label == 'source-missing':
            source.unlink()
        elif label == 'source-marker':
            (source.parent / '.git').mkdir()
        elif label == 'output-link':
            output.unlink(); output.symlink_to(source)
        elif label == 'output-hardlink':
            output.unlink(); os.link(source, output)
        elif label == 'output-parent-link':
            shutil.rmtree(output.parent); output.parent.symlink_to(root / 'docs/book/src', target_is_directory=True)
        elif label == 'output-marker':
            (output.parent / '.git').mkdir()
        elif label == 'output-directory-file':
            shutil.rmtree(output.parent); output.parent.write_text('not a directory')
        elif label == 'config-output':
            config.write_text(config.read_text() + '\n[build]\nbuild-dir = "../../escape"\n')
        elif label == 'config-command':
            config.write_text(config.read_text() + '\n[preprocessor.foreign]\ncommand = "unowned-child"\n')
        elif label == 'environment-output':
            os.environ['MDBOOK_BUILD__BUILD_DIR'] = str(WORK / 'escape')
        elif label == 'environment-renderer':
            os.environ['MDBOOK_OUTPUT'] = '{"foreign":{"command":"unowned-child"}}'
        elif label == 'theme':
            (root / 'docs/book/theme').mkdir()
        elif label == 'include':
            source.write_text('{{#include ../../../../../foreign-input}}\n')
        elif label == 'missing-chapter':
            summary = source.parent / 'SUMMARY.md'
            summary.write_text(summary.read_text() + '\n- [Missing](missing.md)\n')
        elif label == 'toolchain':
            os.environ['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'
        before, original, configured = snapshot(root), subprocess.run, dict(os.environ)
        original_stat = Path.stat
        real_environment = ns['PROFILE']['environment_for']

        def no_activation(selected_root, variables, *, create=True):
            assert not create, 'publication refused input activated stores'
            return real_environment(selected_root, variables, create=False)

        def no_child(*args, **kwargs):
            raise AssertionError('publication refused input reached a child')

        def stat(path, *args, **kwargs):
            result = original_stat(path, *args, **kwargs)
            if path == (source if label == 'source-device' else output):
                fields = list(result); fields[2] += 1
                return os.stat_result(fields)
            return result

        try:
            ns['PROFILE'] = dict(ns['PROFILE'], environment_for=no_activation)
            if label.endswith('-device'):
                Path.stat = stat
            subprocess.run = no_child
            try:
                ns['build'](root)
            except ns['PublicationError'] as error:
                assert str(error).startswith('BOOK_AUTHORITY:'), (label, error)
            else:
                raise AssertionError(('publication physical guard missing', label))
            assert snapshot(root) == before, ('publication refusal changed root', label)
            assert dict(os.environ) == configured, ('publication refusal changed environment', label)
        finally:
            subprocess.run = original
            Path.stat = original_stat
            os.environ.clear(); os.environ.update(saved)
    print(f'publication builder physical: {len(cases)} actual source/output/device/config/environment/include/theme/toolchain refusals before activation/children; roots exact')


def temporary_controls(script=None):
    ns = module(script)
    population = ns['fixture_plan'](ROOT, WORK / 'unused-temporary-plan')['inputs']
    real = ns['PROFILE']
    declared, roots = set(), []

    def plan(root, **kwargs):
        declared.update(kwargs.get('sources', ()))
        return real['plan_producer'](root, **kwargs)

    def environment(root, variables, *, create=True):
        assert not create or population <= declared, 'publication temporary fixture plan omitted before activation'
        return real['environment_for'](root, variables, create=create)

    ns['PROFILE'] = dict(real, plan_producer=plan, environment_for=environment)
    for failure in (None, RuntimeError, OSError):
        try:
            with ns['temporary_root'](prefix='publication-lifecycle-', dir=WORK, fixture_root=ROOT) as root:
                roots.append(root)
                output = root / 'proof.txt'
                PROFILE['plan_producer'](ROOT, sources=CORE, output_files=(output,))
                output.write_text('owned temporary input\n')
                if failure:
                    raise failure('controlled temporary body failure')
        except (RuntimeError, OSError) as error:
            assert type(error) is failure and str(error) == 'controlled temporary body failure'
        assert not roots[-1].exists()
    print('publication temporary lifecycle: complete fixture plan before activation/creation; ordinary and exceptional cleanup pass')


def index_controls(script=None):
    ns, root = module(script), book('index-guard')
    real_environment = ns['PROFILE']['environment_for']
    original = subprocess.check_output
    before, saved = snapshot(root), dict(os.environ)

    def environment(root, variables, *, create=True):
        assert not create, 'publication tracked-output guard omitted before activation'
        return real_environment(root, variables, create=False)

    ns['PROFILE'] = dict(ns['PROFILE'], environment_for=environment)
    try:
        for failure in (False, True):
            def index(argv, **kwargs):
                assert argv[:4] == ['git', 'ls-files', '-z', '--'] and kwargs['cwd'] == ROOT
                if failure:
                    raise subprocess.CalledProcessError(128, argv)
                return b'controlled-tracked-output\0'

            subprocess.check_output = index
            try:
                ns['build'](root)
            except ns['PublicationError'] as error:
                assert str(error).startswith('BOOK_AUTHORITY:'), error
                assert failure or 'tracked data' in str(error)
            else:
                raise AssertionError('publication index uncertainty accepted')
            assert snapshot(root) == before and dict(os.environ) == saved
    finally:
        subprocess.check_output = original
    print('publication index: controlled tracked-data/read-failure refusals before activation; no index mutation or native child')


def main_write_controls(script=None):
    """Compile the actual two tail contexts; the complete real main corpus is checked separately."""
    source = ENTRY.read_text() if script is None else script
    ns = module(source)
    main = next(node for node in ast.parse(source).body if isinstance(node, ast.FunctionDef) and node.name == 'main')
    contexts = [node for node in main.body if isinstance(node, ast.With) and
                any(keyword.arg == 'prefix' and isinstance(keyword.value, ast.Constant) and
                    keyword.value.value in ('static-currency-', 'book-warning-')
                    for keyword in node.items[0].context_expr.keywords)]
    assert len(contexts) == 2
    writes = [node for node in ast.walk(main) if isinstance(node, ast.Call) and
              isinstance(node.func, ast.Attribute) and node.func.attr == 'write_text']
    assert len(writes) == 2, 'publication direct main write population changed'
    function = ast.FunctionDef(name='_main_tail', args=ast.arguments(posonlyargs=[], args=[], kwonlyargs=[], kw_defaults=[], defaults=[]),
                               body=contexts, decorator_list=[])
    exec(compile(ast.fix_missing_locations(ast.Module(body=[function], type_ignores=[])), str(ENTRY), 'exec'), ns)
    ns.update(scratch=ROOT / 'target/scratch', result=ns['check'](ROOT))
    original_write, original_plan, saved = Path.write_text, ns['mutation_plan'], dict(os.environ)
    latest, observed = [], []
    identities = {file: hashlib.sha256(file.read_bytes()).digest() for file in CORE}

    def plan(file, **kwargs):
        result = original_plan(file, **kwargs)
        if kwargs.get('activate'):
            latest[:] = [Path(file).absolute()]
        return result

    def write(path, *args, **kwargs):
        caller = sys._getframe(1)
        if caller.f_globals is ns and caller.f_code.co_name == '_main_tail':
            assert latest == [path.absolute()], 'publication main write lacks fresh mutable authority'
            observed.append(path.relative_to(caller.f_locals['root']).as_posix())
            latest.clear()
        return original_write(path, *args, **kwargs)

    def builder(root):
        text = (root / 'docs/book/src/introduction.md').read_text()
        if '\nPublication control: Checked<UnclosedType>\n' in text:
            raise ns['PublicationError']('BOOK_BUILD_WARNING: unclosed generic')
        path = root / 'docs/book/book/introduction.html'
        PROFILE['plan_producer'](ROOT, sources=(*CORE, path), mutable_sources=(path,), output_files=(path,))
        path.write_text(path.read_text() + '<code>Checked&lt;UnclosedType&gt;</code>')
        return ''

    try:
        Path.write_text, ns['mutation_plan'], ns['build'] = write, plan, builder
        with contextlib.redirect_stdout(io.StringIO()):
            ns['_main_tail']()
        assert observed == ['docs/book/src/annexes/formula-static-validation.md', 'docs/book/src/introduction.md']
    finally:
        Path.write_text = original_write
        os.environ.clear(); os.environ.update(saved)
        assert all(hashlib.sha256(file.read_bytes()).digest() == digest for file, digest in identities.items())
    print('publication main writes: actual two contexts/full copied fixtures; fresh mutable authority; tracked sources exact; renderer stub only')


def main_input_controls(script=None):
    """Independent non-rendered reader population, trapped before the first main effect."""
    ns = module(script)
    mapping = ROOT / 'docs/tasks/artifacts/book_publication/status.tsv'
    PROFILE['plan_producer'](ROOT, sources=(*CORE, mapping))
    with mapping.open() as stream:
        entries = list(csv.reader((line for line in stream if not line.startswith('#')), delimiter='\t'))
    expected = {ENTRY, PROFILE_SOURCE, mapping, ROOT / 'ROADMAP.md', ROOT / 'docs/book/book.toml',
                ROOT / 'crates/sc-core/tests/formula_statement_contract.rs',
                *(path for path in (ROOT / 'docs/book/src').rglob('*') if path.is_file()),
                *(ROOT / 'docs/tasks').glob('*.md'), *(ROOT / row[2] for row in entries),
                *(ROOT / 'crates/sc-core/src/recipe' / file for file in ('checked.rs', 'checked_recipe.rs', 'namespace/ordered.rs'))}
    real, planned, seen = ns['PROFILE'], set(), []
    saved = dict(os.environ)
    original_open = Path.open

    class EffectReached(Exception):
        pass

    def plan(root, **kwargs):
        planned.update(kwargs.get('sources', ()))
        return real['plan_producer'](root, **kwargs)

    def first_effect(root):
        assert root == ROOT and expected <= planned, 'publication main reader inputs omitted before first effect'
        seen.append(True)
        raise EffectReached

    def opening(path, *args, **kwargs):
        if path == mapping and sys._getframe(1).f_globals is ns:
            assert mapping in planned, 'publication main map read before authority'
        return original_open(path, *args, **kwargs)

    try:
        ns['PROFILE'] = dict(real, plan_producer=plan)
        ns['build'] = first_effect
        Path.open = opening
        try:
            ns['main']()
        except EffectReached:
            pass
        assert seen and dict(os.environ) == saved
    finally:
        Path.open = original_open
        os.environ.clear(); os.environ.update(saved)
    print('publication main inputs: complete independently enumerated book/map/roadmap/tasks/Rust sources before first effect; child trapped')


def omission_controls():
    source = ENTRY.read_text()
    reds = 0
    anchor = '    main_inputs(ROOT)\n'
    assert source.count(anchor) == 1
    try:
        main_input_controls(source.replace(anchor, '', 1))
    except AssertionError as error:
        assert str(error) == 'publication main reader inputs omitted before first effect'
    else:
        raise AssertionError('publication main input omission escaped')
    reds += 1
    function = next(node for node in ast.parse(source).body if isinstance(node, ast.FunctionDef) and node.name == 'main_inputs')
    original = ast.get_source_segment(source, function)
    anchor = '        PROFILE["plan_producer"](ROOT, sources=(*CORE, root / MAP))\n'
    assert original.count(anchor) == 1
    try:
        main_input_controls(source.replace(original, original.replace(anchor, '', 1), 1))
    except AssertionError as error:
        assert str(error) == 'publication main map read before authority'
    else:
        raise AssertionError('publication main map source guard omission escaped')
    reds += 1
    faults = (
        ('inputs', 'sources=(*CORE, config, *files)', 'sources=(*CORE, config)',
         'publication builder inputs omitted before activation'),
        ('stage-files', 'sources=(*CORE, *generated_files), output_files=generated_files',
         'sources=(*CORE, *generated_files), output_files=()', 'publication generated file plan omitted'),
        ('stage-inputs', 'sources=(*CORE, *generated_files), output_files=generated_files',
         'sources=CORE, output_files=generated_files', 'publication generated input plan omitted'),
        ('source-creation', 'MDBOOK_BUILD__CREATE_MISSING="false"', 'MDBOOK_BUILD__CREATE_MISSING="true"',
         'publication builder source creation enabled'),
        ('stores', 'dict(publication_environment(create=True), MDBOOK_LOG=',
         'dict({key: value for key, value in publication_environment(create=True).items() if key not in PROFILE["STORES"]}, MDBOOK_LOG=',
         'publication builder caller stores reset'),
    )
    for label, before, after, reason in faults:
        assert source.count(before) == 1, label
        try:
            capture('red-' + label, source.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == reason, (label, error)
        else:
            raise AssertionError(('publication builder omission escaped', label))
        reds += 1
    print('publication builder: five actual input/generated-input/file/source-creation/store guard assertion reds; no native child')
    before = '                   if not key.upper().startswith("GIT_")'
    assert source.count(before) == 1
    try:
        capture('red-git-context', source.replace(before, '', 1))
    except AssertionError as error:
        assert str(error) == 'publication index query inherited unrelated Git authority'
    else:
        raise AssertionError('publication Git context omission escaped')
    reds += 1
    print('publication child Git context: actual foreign-context filter omission assertion red before dispatch')
    before = '    require(not tracked, "BOOK_AUTHORITY", "public output contains tracked data")\n'
    assert source.count(before) == 1
    try:
        index_controls(source.replace(before, '', 1))
    except AssertionError as error:
        assert str(error) == 'publication tracked-output guard omitted before activation'
    else:
        raise AssertionError('publication tracked-output guard omission escaped')
    reds += 1
    print('publication index: actual tracked-data guard omission assertion red before activation')

    for label, before, after, result, reason in (
        ('warning', '            require(not warnings, "BOOK_BUILD_WARNING", "\\n".join(warnings))\n', '', 'warning',
         'publication builder failure accepted'),
        ('exit-status', '            require(result.returncode == 0, "BOOK_BUILD", diagnostic.strip())\n', '', 'failure',
         'publication builder failure accepted'),
        ('publish-recheck', '            publication_plan(root, stage)\n            shutil.copytree',
         '            shutil.copytree', 'success', 'publication effect recheck omitted'),
        ('teardown-plan', '            PROFILE["plan_producer"](ROOT, sources=(*CORE, *files), directories=folders,\n                                     output_files=files)\n',
         '', 'success', 'publication effect recheck omitted'),
    ):
        assert source.count(before) == 1, label
        try:
            capture('red-' + label, source.replace(before, after, 1), result=result)
        except AssertionError as error:
            assert str(error) == reason, (label, error)
        else:
            raise AssertionError(('publication lifecycle/status omission escaped', label))
        reds += 1
    print('publication builder: four actual warning/status/publication-recheck/teardown guard assertion reds')
    for label, before, after, result in (
        ('input-dirs', 'build_inputs(root) == (input_dirs, inputs)', 'build_inputs(root)[1] == inputs', 'topology-change'),
        ('input-bytes', 'hashlib.sha256(file.read_bytes()).digest() == digest', 'True', 'input-change'),
        ('chapter-output', '        require(required <= set(generated_files), "BOOK_BUILD", "renderer omitted a summary chapter")\n', '', 'missing-output'),
    ):
        assert source.count(before) == 1, label
        try:
            capture('red-' + label, source.replace(before, after, 1), result=result)
        except AssertionError as error:
            assert str(error) == 'publication builder failure accepted', (label, error)
        else:
            raise AssertionError(('publication input/chapter omission escaped', label))
        reds += 1
    print('publication builder: three actual input-topology/input-byte/complete-chapter assertion reds')
    before = '            fixture_plan(fixture_root, destination)'
    assert source.count(before) == 1
    try:
        temporary_controls(source.replace(before, '            pass', 1))
    except AssertionError as error:
        assert str(error) == 'publication temporary fixture plan omitted before activation'
    else:
        raise AssertionError('publication temporary preflight omission escaped')
    reds += 1
    print('publication temporary lifecycle: actual fixture-preflight omission assertion red')
    for label, before in (
        ('static', '        mutation_plan(chapter, activate=True)\n        chapter.write_text(chapter.read_text() + "\\nComplete checking:'),
        ('warning', '        mutation_plan(chapter, activate=True)\n        chapter.write_text(chapter.read_text() + marker)'),
    ):
        assert source.count(before) == 1, label
        after = before.replace('        mutation_plan(chapter, activate=True)\n', '')
        try:
            main_write_controls(source.replace(before, after, 1))
        except AssertionError as error:
            assert str(error) == 'publication main write lacks fresh mutable authority', (label, error)
        else:
            raise AssertionError(('publication main authority omission escaped', label))
        reds += 1
    print('publication main writes: two actual mutable-authority omission assertion reds before writes')
    print(f'publication builder omissions: {reds} compiled actual-body assertion reds; source bytes exact')


if __name__ == '__main__':
    if sys.argv[1:] == ['--before']:
        before_control()
    elif sys.argv[1:] == ['--shape']:
        capture('shape-change', result='shape-change')
    elif sys.argv[1:] == ['--main-plan']:
        main_input_controls()
    else:
        assert not sys.argv[1:]
        before_control()
        capture('success')
        capture('shape-change', result='shape-change')
        capture('warning', result='warning')
        capture('failure', result='failure')
        capture('input-change', result='input-change')
        capture('topology-change', result='topology-change')
        capture('missing-output', result='missing-output')
        physical_controls()
        temporary_controls()
        index_controls()
        main_write_controls()
        main_input_controls()
        omission_controls()
