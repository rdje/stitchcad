"""Capture actual inline fixtures and nested writer boundaries; real verdicts run separately."""
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
HERE = ROOT / 'docs/tasks/artifacts/formula_language'
ENTRY = HERE / 'inline_context_contract.py'
WRITER = HERE / 'inline_context_mutations.py'
SOURCE = HERE / 'run_formula_language_census.sh'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
BOOK = ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_inline_context'
WRITER_WORK = ROOT / 'target/formula_inline_mutations'
GOOD = runpy.run_path(str(PROFILE_SOURCE))
BOOT = (ENTRY, PROFILE_SOURCE, SOURCE)
WRITER_BOOT = (WRITER, PROFILE_SOURCE, SOURCE, ENTRY)
directories, files = [BOOK], []
for folder in directories:
    GOOD['directory'](ROOT, folder.relative_to(ROOT).as_posix())
    for path in sorted(folder.iterdir()):
        GOOD['require'](not path.is_symlink(), 'symlink inline control book entry')
        (directories if path.is_dir() else files).append(path)
BOOK_FILES = tuple(files)
WHOLE = set((*WRITER_BOOT, *BOOK_FILES))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in WHOLE}
TEXTS = {path: FROZEN[path].decode() for path in (ENTRY, WRITER)}
BEFORE = '6df9f5ce4792ff46b607cc6dc4f2dbcad27c4362'
ORIGINALS = {path: subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], text=True)
             for path in TEXTS}


def cases(text):
    tree = ast.parse(text)
    return ast.literal_eval(next(node.value for node in tree.body if isinstance(node, ast.Assign)
                                and any(isinstance(target, ast.Name) and target.id == 'CASES' for target in node.targets)))


CASES = cases(ORIGINALS[ENTRY])
COPY_FILES = {WORK / 'src' / path.relative_to(BOOK) for path in BOOK_FILES}
COPY_DIRS = {WORK, *(WORK / 'src' / path.relative_to(BOOK) for path in directories)}
LOGS = {WRITER_WORK / ('mutation-' + str(n) + '.log') for n in range(1, 6)}


def capture(entry, mode, replacements=None, selectors=None):
    """Check actual plans and effects while intercepting only child execution/source bytes."""
    scripts = dict(TEXTS); scripts.update(replacements or {})
    saved, argv = dict(os.environ), sys.argv
    custom = {name: str(ROOT / 'target/inline-capture-stores' / suffix) for name, suffix in (
        ('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in custom.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    originals = (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
                 Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree)
    plans, latest, prepared, written, children, removed, copies = [], {}, [], [], [], [], []
    shadow, checkpoint, child_checkpoint = [FROZEN[SOURCE]], [0], [0]

    def caller():
        frame = sys._getframe(2)
        while frame and Path(frame.f_globals.get('__file__', '')) not in TEXTS:
            frame = frame.f_back
        return frame

    def owner():
        return Path(caller().f_globals['__file__'])

    def plan(root, **kwargs):
        who = owner(); assert root == ROOT, 'inline plan root changed'
        sources = set(kwargs.get('sources', ()))
        if who == WRITER:
            if sources == set(WRITER_BOOT):
                assert not any(kwargs.get(key) for key in ('directories', 'output_files', 'mutable_sources')), 'inline bootstrap has effects'
            elif sources == {SOURCE}:
                assert set(kwargs.get('mutable_sources', ())) == {SOURCE}, 'inline restoration authority changed'
                assert not kwargs.get('output_files') and not kwargs.get('directories'), 'inline restoration depends on outputs'
            else:
                assert sources >= WHOLE and set(kwargs.get('mutable_sources', ())) == {SOURCE}, 'inline writer sources incomplete'
                assert set(kwargs.get('output_files', ())) == COPY_FILES | LOGS, 'inline nested writer files incomplete'
                assert set(kwargs.get('directories', ())) == COPY_DIRS | {WRITER_WORK}, 'inline nested writer directories incomplete'
        else:
            assert not kwargs.get('mutable_sources'), 'inline unowned tracked mutation'
        before = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == before, 'inline pure plan changed environment'
        plans.append((who, kwargs)); latest[who] = (kwargs, len(plans))
        return result

    def prepare(root, **kwargs):
        who = owner()
        assert who in latest and latest[who][0] == kwargs, 'inline activation before complete pure plan'
        expected_files = COPY_FILES | (LOGS if who == WRITER else set())
        expected_dirs = COPY_DIRS | ({WRITER_WORK} if who == WRITER else set())
        assert set(kwargs.get('output_files', ())) == expected_files, 'inline producing files incomplete'
        assert set(kwargs.get('directories', ())) == expected_dirs, 'inline producing directories incomplete'
        assert set(kwargs.get('sources', ())) >= set((*BOOT, *BOOK_FILES)), 'inline producing sources incomplete'
        prepared.append(kwargs)
        return GOOD['prepare_producer'](root, **kwargs)

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, prepare_producer=prepare)
        assert path == ENTRY and owner() == WRITER, 'inline unexpected delegated module'
        assert latest.get(WRITER, ({}, 0))[0].get('sources') == WRITER_BOOT, 'inline writer bootstrap omitted'
        namespace = dict(__file__=str(ENTRY), __name__='borrowed_inline')
        exec(compile(scripts[ENTRY], str(ENTRY), 'exec'), namespace)
        return namespace

    def fresh_writer(wanted):
        assert WRITER in latest and plans[-1][0] == WRITER and latest[WRITER][1] > checkpoint[0], wanted
        checkpoint[0] = latest[WRITER][1]

    def authorized(path):
        assert prepared and any(path in item.get('output_files', ()) for item in prepared), 'inline output before producing plan'
        assert path in COPY_FILES | LOGS, 'inline unowned output'
        assert all(os.environ.get(name) == value for name, value in custom.items()), 'inline stores lost'

    def read(path, binary, *args, **kwargs):
        if path == SOURCE and owner() == WRITER:
            assert prepared and latest[WRITER][0].get('mutable_sources') == (SOURCE,), 'inline writer read before complete plan'
            return shadow[0] if binary else shadow[0].decode()
        if path in COPY_FILES:
            assert any(COPY_FILES <= set(item.get('sources', ())) and COPY_FILES <= set(item.get('output_files', ()))
                       for unused, item in plans), 'inline copied source plan omitted'
        return originals[2 if binary else 1](path, *args, **kwargs)

    def write(path, data, binary, *args, **kwargs):
        if path == SOURCE:
            fresh_writer('inline writer write recheck omitted')
            assert SOURCE in latest[WRITER][0].get('mutable_sources', ()), 'inline writer authority omitted'
            if binary:
                assert set(latest[WRITER][0].get('sources', ())) == {SOURCE}, 'inline restoration depends on unrelated inputs'
            shadow[0] = data if binary else data.encode()
        else:
            authorized(path)
            if path in LOGS:
                fresh_writer('inline writer log recheck omitted')
            originals[4 if binary else 3](path, data, *args, **kwargs)
            if path in COPY_FILES:
                child_checkpoint[0] = len(plans)
        written.append(path)
        return len(data)

    def path_mkdir(path, *args, **kwargs):
        assert prepared, 'inline mkdir before producing plan'
        return originals[5](path, *args, **kwargs)

    def mkdir(path, *args, **kwargs):
        assert prepared and Path(path) in COPY_DIRS | {WRITER_WORK}, 'inline unplanned directory'
        return originals[6](path, *args, **kwargs)

    def remove(path, *args, **kwargs):
        assert Path(path) == WORK / 'src' and set(plans[-1][1].get('output_files', ())) == COPY_FILES, 'inline delete recheck omitted'
        assert prepared, 'inline delete before producing plan'
        removed.append(Path(path)); return originals[8](path, *args, **kwargs)

    def copyfile(source, destination):
        source, destination = Path(source), Path(destination)
        assert source in BOOK_FILES, 'inline unplanned copied source'
        authorized(destination)
        GOOD['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        written.append(destination); return shutil.copy2(source, destination)

    def copytree(source, destination, *args, **kwargs):
        if Path(source) == BOOK:
            assert not args and not kwargs, 'inline copier interface changed'
            copies.append(Path(destination)); return originals[9](source, destination, copy_function=copyfile)
        bound = inspect.signature(originals[9]).bind(source, destination, *args, **kwargs)
        bound.apply_defaults()
        assert bound.arguments['copy_function'] is copyfile and bound.arguments['ignore'] is None, 'inline recursive copier changed'
        assert not any(bound.arguments[name] for name in ('symlinks', 'ignore_dangling_symlinks', 'dirs_exist_ok')), 'inline copy options changed'
        return originals[9](source, destination, *args, **kwargs)

    def child(arguments, *args, **kwargs):
        environment = kwargs.get('env', {})
        assert not args and kwargs.get('cwd') == ROOT and kwargs.get('capture_output') is True, 'inline child interface changed'
        assert all(environment.get(name) == value for name, value in custom.items()) and environment.get('MAKE_TMPDIR') == custom['TMPDIR'], 'inline child stores lost'
        assert environment.get('INLINE_UNRELATED') == 'preserved', 'inline unrelated environment lost'
        if entry == WRITER:
            assert arguments == [sys.executable, '-I', '-B', ENTRY.relative_to(ROOT).as_posix()], 'inline writer child argv changed'
            fresh_writer('inline writer child recheck omitted')
            assert shadow[0] != FROZEN[SOURCE], 'inline classifier mutation missing'
            if mode == 'writer-fault':
                raise RuntimeError('inline injected child failure')
            result = types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'inline_context_contract.py AssertionError\n')
        else:
            assert arguments == ['bash', str(SOURCE)] and kwargs.get('text') is True, 'inline shell interface changed'
            assert environment.get('FORMULA_BOOK') == str(WORK / 'src'), 'inline copied selector lost'
            assert all(environment.get(name) == value for name, value in (selectors or {}).items()), 'inline caller selectors lost'
            assert any(set((*BOOT, *BOOK_FILES)) <= set(item.get('sources', ()))
                       for unused, item in plans[child_checkpoint[0]:]), 'inline child source recheck omitted'
            parts = Path(environment.get('FORMULA_PARTS_DIR') or WORK / 'src/spec/formula-language')
            parts = parts if parts.is_absolute() else ROOT / parts
            selected = [Path(environment.get(name) or default) for name, default in (
                ('FORMULA_CONTRACT', WORK / 'src/spec/formula-language.md'),
                ('FORMULA_GRAMMAR', parts / 'grammar.md'), ('FORMULA_EXAMPLES', parts / 'examples.md'),
                ('FORMULA_FIXTURE', WORK / 'src/spec/reference-skirt.md'))]
            selected = [path if path.is_absolute() else ROOT / path for path in selected]
            references = [Path(os.path.normpath(str(path.parent) + '/' + target.split('#')[0]))
                          for path in selected[:3] for unused, target in re.findall(r'\[([^\]]*)\]\(([^)]+)\)', originals[1](path))
                          if not target.startswith(('http://', 'https://', '#'))]
            assert set(plans[-1][1].get('sources', ())) == set((*selected, *sorted(parts.glob('*.md')), *references)), 'inline child selector recheck omitted'
            unused, unused_path, unused_snippet, verdict, reason = CASES[len(children)]
            result = types.SimpleNamespace(returncode=verdict, stdout=reason + '\n', stderr='')
        children.append(arguments); return result

    expected = dict(saved, **custom, MAKE_TMPDIR=custom['TMPDIR'], RUSTUP_AUTO_INSTALL='0',
                    RUSTUP_TOOLCHAIN='1.99.0', INLINE_UNRELATED='preserved')
    for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE'):
        expected.pop(name, None)
    expected.update(selectors or {})
    namespace = dict(__file__=str(entry), __name__='__main__' if mode == 'cli' else 'borrowed_inline')
    try:
        os.environ.clear(); os.environ.update(expected)
        sys.argv = [str(entry), 'unrelated-argument']; current_argv = sys.argv
        runpy.run_path = load
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.write_text = lambda path, data, *args, **kwargs: write(path, data, False, *args, **kwargs)
        Path.write_bytes = lambda path, data, *args, **kwargs: write(path, data, True, *args, **kwargs)
        Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = path_mkdir, mkdir, child, remove, copytree
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                exec(compile(scripts[entry], str(entry), 'exec'), namespace)
                if mode == 'plan':
                    namespace['fixture_plan'](activate=False)
                elif mode not in ('cli', 'import'):
                    namespace['contracts' if entry == ENTRY else 'main']()
            except RuntimeError as error:
                assert mode == 'writer-fault' and str(error) == 'inline injected child failure', 'inline unrelated exception'
        assert sys.argv is current_argv and dict(os.environ) == expected, 'inline borrowed normal/exception state changed'
        if entry == WRITER and mode != 'import':
            assert shadow[0] == FROZEN[SOURCE], 'inline modeled restoration failed'
            assert (len(children), written.count(SOURCE), len(written)) == ((0, 2, 2) if mode == 'writer-fault' else (5, 11, 16)), 'inline writer population changed'
        elif entry == ENTRY and mode in ('cli', 'borrowed'):
            assert len(children) == len(copies) == len(removed) == 13 and len(written) == 13 * (len(BOOK_FILES) + 1) and set(written) == COPY_FILES, 'inline copied population changed'
        else:
            assert not prepared and not written and not children, 'inline pure call produced effects'
        assert any(who == ENTRY and item.get('sources') == BOOT for who, item in plans), 'inline bootstrap omitted'
    finally:
        (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
         Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree) = originals
        sys.argv = argv; os.environ.clear(); os.environ.update(saved)
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'inline tracked inputs changed'
    print('actual inline capture:', entry.name, mode, len(plans), 'plans;', len(written),
          'writes;', len(children), 'intercepted children; actual verdicts separate')


def original_bodies():
    class Guards(ast.NodeTransformer):
        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and ast.unparse(node.value.func) in (
                    "PROFILE['plan_producer']", 'producer_plan', 'source_plan', 'consumer_inputs'):
                return None
            return self.generic_visit(node)
        def visit_Call(self, node):
            if ast.unparse(node.func) == 'subprocess.run':
                node.keywords = [keyword for keyword in node.keywords if keyword.arg != 'env']
            return self.generic_visit(node)
    pick = lambda tree, name: next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == name)
    for path in TEXTS:
        assert cases(TEXTS[path]) == cases(ORIGINALS[path]), 'inline original cases changed'
    before = Guards().visit(ast.parse(ORIGINALS[ENTRY]))
    after = Guards().visit(ast.parse(TEXTS[ENTRY]))
    old, new = pick(before, 'contracts'), pick(after, 'contracts')
    assert ast.dump(old.body[0]) == ast.dump(new.body[1]), 'inline original directory call changed'
    assert [ast.dump(node) for node in old.body[1].body[3:]] == [ast.dump(node) for node in new.body[2].body[1:]], 'inline original edit/child/assertion body changed'
    assert ast.dump(old.body[2]) == ast.dump(new.body[3]), 'inline original completion changed'
    before = Guards().visit(ast.parse(ORIGINALS[WRITER]))
    after = Guards().visit(ast.parse(TEXTS[WRITER]))
    old = [node for node in before.body if isinstance(node, (ast.Try, ast.Assert)) or
           isinstance(node, ast.Expr) and isinstance(node.value, ast.Call) and ast.unparse(node.value.func) == 'print']
    assert [ast.dump(node) for node in old] == [ast.dump(node) for node in pick(after, 'main').body[5:]], 'inline original writer assertions/restoration changed'
    print('inline original13/five authored cases and assertion bodies exact against source HEAD')


def fault_controls():
    original_bodies()
    for entry, mode, wanted in ((ENTRY, 'import', 'inline bootstrap omitted'),
                               (WRITER, 'import', 'inline mkdir before producing plan')):
        try:
            capture(entry, mode, {entry: ORIGINALS[entry]})
        except AssertionError as error:
            assert str(error) == wanted, ('inline unrelated original gap', str(error))
        else:
            raise AssertionError('inline original gap escaped')
    faults = [
        (ENTRY, 'import', "PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", 'None', 'inline bootstrap omitted'),
        (WRITER, 'import', "PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", 'None', 'inline writer bootstrap omitted'),
        (ENTRY, 'cli', 'directories=directories, output_files=outputs, sources=sources', 'directories=directories, output_files=outputs[:-1], sources=sources', 'inline producing files incomplete'),
        (ENTRY, 'cli', 'directories=directories, output_files=outputs, sources=sources', 'directories=directories[:-1], output_files=outputs, sources=sources', 'inline producing directories incomplete'),
        (ENTRY, 'cli', 'directories=directories, output_files=outputs, sources=sources', 'directories=directories, output_files=outputs, sources=sources[:-1]', 'inline producing sources incomplete'),
        (ENTRY, 'cli', "return PROFILE['prepare_producer'](ROOT, **plan) if activate else plan", "return PROFILE['environment_for'](ROOT, os.environ, create=False) if activate else plan", 'inline mkdir before producing plan'),
        (ENTRY, 'cli', '    fixture_plan(activate=False)', '    None', 'inline delete recheck omitted'),
        (ENTRY, 'cli', "PROFILE['plan_producer'](ROOT, sources=(TOOL, *files), output_files=files)", 'None', 'inline copied source plan omitted'),
        (ENTRY, 'cli', '        source_plan()\n        consumer_inputs', '        consumer_inputs', 'inline child source recheck omitted'),
        (ENTRY, 'cli', '        consumer_inputs(dict(environment, FORMULA_BOOK=str(book)))\n', '', 'inline child selector recheck omitted'),
        (ENTRY, 'cli', "env={**environment, 'FORMULA_BOOK': str(book)}", "env={**environment, 'FORMULA_BOOK': str(book), 'TMPDIR': str(ROOT/'target/wrong-inline-store')}", 'inline child stores lost'),
        (WRITER, 'cli', "    PROFILE['plan_producer'](ROOT, **plan)\n    return plan", '    return plan', 'inline activation before complete pure plan'),
        (WRITER, 'cli', "output_files=(*child['output_files'],", 'output_files=(*(),', 'inline nested writer files incomplete'),
        (WRITER, 'cli', "directories=(WORK, *child['directories'])", 'directories=(WORK,)', 'inline nested writer directories incomplete'),
        (WRITER, 'cli', 'env=environment', 'env={}', 'inline child stores lost'),
        (WRITER, 'cli', '            producer_plan()\n            SOURCE.write_text', '            SOURCE.write_text', 'inline writer write recheck omitted'),
        (WRITER, 'cli', '            producer_plan()\n            result', '            result', 'inline writer child recheck omitted'),
        (WRITER, 'cli', '            producer_plan()\n            (WORK', '            (WORK', 'inline writer log recheck omitted'),
        (WRITER, 'cli', "            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))\n            SOURCE.write_bytes", '            SOURCE.write_bytes', 'inline writer write recheck omitted'),
        (WRITER, 'cli', "        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))\n        SOURCE.write_bytes", '        SOURCE.write_bytes', 'inline writer write recheck omitted'),
    ]
    for entry, mode, anchor, replacement, wanted in faults:
        assert TEXTS[entry].count(anchor) == 1, ('inline fault anchor changed', anchor)
        try:
            capture(entry, mode, {entry: TEXTS[entry].replace(anchor, replacement, 1)})
        except AssertionError as error:
            assert str(error) == wanted, ('inline unrelated guard red', anchor, str(error))
        else:
            raise AssertionError(('inline actual guard omission escaped', anchor))
    print('inline guard controls:', len(faults), 'actual guard reds/two original gaps; original cases and bodies exact')


def physical_controls():
    """Refuse real filesystem faults in owned isolated copies before any producer effect."""
    cases = ('missing input', 'symlink input', 'symlink book directory', 'inline is directory',
             'unexpected old fixture', 'symlink output', 'hardlinked output',
             'output directory is file', 'symlink copied input', 'outside selector',
             'inline output alias', 'parts directory alias', 'symlink store', 'invalid toolchain', 'pure unused sidebar')
    def effect(*args, **kwargs):
        raise AssertionError('inline physical effect before refusal')
    generation = ROOT / 'target/fixture-inline-controls' / str(os.getpid())
    GOOD['directory'](ROOT, generation.parent.relative_to(ROOT).as_posix(), create=True)
    assert not generation.exists(), 'inline fixture generation already exists'
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
        leaf = work / 'src/spec/formula-language.md'
        log = work / 'unused-control.log'
        sidebar = sandbox / 'docs/book/src/annexes/formula-static-validation.md'
        input_file = sandbox / 'docs/book/src/spec/units-and-tolerances.md'
        extra = work / 'src/unexpected.txt'
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
        elif case == 'inline is directory':
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
            base = work / 'src'
            blocker = base / 'annexes'
            GOOD['plan_producer'](ROOT, output_files=(blocker,))
            blocker.write_text('owned directory refusal sentinel\n')
            expected = 'unexpected copied-book fixture entry'
        elif case == 'symlink copied input':
            linked = work / 'src/spec/units-and-tolerances.md'
            GOOD['plan_producer'](ROOT, output_files=(linked,))
            linked.symlink_to(input_file); expected = 'symlink copied-book entry'
        elif case == 'outside selector':
            selectors['FORMULA_CONTRACT'] = str(ENTRY); expected = 'producer path outside workspace'
        elif case == 'inline output alias':
            leaf.write_text('owned selected-input alias sentinel\n')
            selectors['FORMULA_CONTRACT'] = str(leaf)
            expected = 'copied-book output aliases selected input'
        elif case == 'parts directory alias':
            parts = work / 'src/spec/formula-language'
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
                exec(compile(TEXTS[ENTRY], str(mirrors[ENTRY]), 'exec'), namespace)
                if expected is None:
                    assert namespace['BOOT_INPUTS'] == tuple(mirrors[path] for path in BOOT)
                else:
                    try:
                        namespace['contracts']()
                    except namespace['PROFILE']['Refusal'] as error:
                        assert str(error) == expected, (case, 'unrelated physical refusal', str(error))
                        if case == 'parts directory alias':
                            guard = 'parts != base and not parts.is_relative_to(base)'
                            assert TEXTS[ENTRY].count(guard) == 1
                            disabled = dict(__file__=str(mirrors[ENTRY]), __name__='directory_guard_disabled')
                            exec(compile(TEXTS[ENTRY].replace(guard, 'True', 1), str(mirrors[ENTRY]), 'exec'), disabled)
                            disabled['fixture_plan'](activate=False)
                            assert not tuple(parts.iterdir()), 'D202 counterfactual changed directory'
                            print('Inherited D202 actual directory-alias guard omission admits same empty selected directory; no effects')

                    else:
                        raise AssertionError((case, 'inline physical refusal escaped'))
        except SystemExit as error:
            assert error.code == 2 and expected and expected in stderr.getvalue(), (case, 'unrelated bootstrap refusal', stderr.getvalue())
        finally:
            os.mkdir, shutil.rmtree, shutil.copytree, subprocess.run, Path.write_text, Path.write_bytes, Path.mkdir = originals
            os.environ.clear(); os.environ.update(saved)
        print('inline physical control:', case)
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
        exec(compile(TEXTS[ENTRY], namespace['__file__'], 'exec'), namespace)
        namespace['fixture_plan'](activate=False)
        assert (sandbox / 'target/formula_inline_context/src/spec/formula-language.md').stat().st_nlink == 2
    finally:
        profile.write_text(original)
        os.environ.clear(); os.environ.update(saved)
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'inline physical controls changed tracked sources'
    print('inline guard counterfactual: actual profile planner disabled admits same hardlinked output; tracked sources exact')


def borrowed_physical_controls():
    """Actual CLI and repeated borrowed-context refusals, plus mutable-source link safety."""
    generation = ROOT / 'target/inline-boundary-controls' / str(os.getpid())
    total = borrowed = 0
    cases = ('missing','symlink','directory','nested-repo','symlink-store','invalid-toolchain')
    selected = [(entry,case) for entry in (ENTRY, WRITER) for case in cases] + [(WRITER,'hardlinked-source'), (WRITER,'nested-child-extra')]
    for index,(entry,case) in enumerate(selected):
        sandbox = generation / str(index)
        assert not sandbox.exists(), 'inline boundary fixture already exists'
        inputs = tuple(WHOLE)
        mirrors = {path:sandbox/path.relative_to(ROOT) for path in inputs}
        dirs={sandbox}
        for path in mirrors.values():
            dirs.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
        GOOD['prepare_producer'](ROOT,sources=inputs,directories=tuple(dirs),output_files=tuple(mirrors.values()))
        for source,destination in mirrors.items():
            shutil.copy2(source,destination)
        victim = mirrors[SOURCE if entry == WRITER else BOOK / 'spec/units-and-tolerances.md']
        wanted='producer source is not a local regular file'
        extra={}
        if case=='missing': victim.unlink()
        elif case=='symlink':
            victim.unlink(); victim.symlink_to(mirrors[PROFILE_SOURCE]); wanted='symlink copied-book entry' if entry==ENTRY else 'symlink producer source'
        elif case=='directory':
            victim.unlink(); victim.mkdir()
        elif case=='nested-repo':
            (victim.parent/'.git').mkdir(); wanted='store crosses another Git repository'
        elif case=='symlink-store':
            scratch=sandbox/'target/scratch'
            GOOD['directory'](ROOT,scratch.parent.relative_to(ROOT).as_posix(),create=True)
            scratch.symlink_to(victim.parent,target_is_directory=True); wanted='symlink store component'
        elif case=='nested-child-extra':
            sentinel=sandbox/'target/formula_inline_context/src/unexpected.txt'
            GOOD['prepare_producer'](ROOT,directories=(sentinel.parent,),output_files=(sentinel,))
            sentinel.write_text('owned nested child refusal sentinel\n')
            wanted='unexpected copied-book fixture entry'
        elif case=='hardlinked-source':
            alias=sandbox/'target/source-alias'
            GOOD['prepare_producer'](ROOT,directories=(alias.parent,),output_files=(alias,))
            os.link(victim,alias); wanted='hard-linked mutable producer source'
        else:
            extra['RUSTUP_TOOLCHAIN']='invalid/toolchain'; wanted='toolchain override must be a name'
        saved,argv=dict(os.environ),sys.argv
        originals=(os.mkdir,Path.write_text,Path.write_bytes,subprocess.run,Path.mkdir)
        def effect(*args,**kwargs): raise AssertionError('inline boundary refusal produced an effect')
        stderr=io.StringIO()
        try:
            os.environ.update({name:str(sandbox/suffix) for name,suffix in (
                ('CARGO_HOME','target/cargo-home'),('RUSTUP_HOME','target/rustup'),
                ('CARGO_TARGET_DIR','target/build'),('TMPDIR','target/scratch'))})
            os.environ.pop('RUSTUP_TOOLCHAIN',None); os.environ.update(extra)
            sys.argv=[str(mirrors[entry]),'unrelated-argument']
            os.mkdir=Path.write_text=Path.write_bytes=subprocess.run=Path.mkdir=effect
            with contextlib.redirect_stderr(stderr),contextlib.redirect_stdout(io.StringIO()):
                try:
                    runpy.run_path(str(mirrors[entry]),run_name='__main__')
                except SystemExit as error:
                    assert error.code==2 and stderr.getvalue()=='local-environment: REFUSED — '+wanted+'\n', (entry.name,case,stderr.getvalue())
                else: raise AssertionError(('inline boundary fault accepted',entry.name,case))
                if entry==ENTRY and case in cases[:4]:
                    namespace=runpy.run_path(str(mirrors[entry]))
                    for unused in range(2):
                        name = 'fixture_plan'
                        try: namespace[name](activate=False)
                        except namespace['PROFILE']['Refusal'] as error:
                            assert str(error)==wanted, 'inline borrowed refusal identity changed'
                        else: raise AssertionError(('inline borrowed refusal escaped',name,case))
                        borrowed+=1
        finally:
            os.mkdir,Path.write_text,Path.write_bytes,subprocess.run,Path.mkdir=originals
            os.environ.clear(); os.environ.update(saved); sys.argv=argv
        total+=1
    print('inline boundaries:',total,'actual CLI refusals;',borrowed,'borrowed context refusals; no writes/activation/child dispatch')


def late_restore_capture(text):
    """D207: a real mirrored write must be restored despite an unrelated late log fault."""
    sandbox = ROOT / 'target/inline-late-restore-controls' / str(os.getpid()) / str(late_restore_capture.calls)
    late_restore_capture.calls += 1
    assert not sandbox.exists(), 'D207 fixture already exists'
    mirrors = {path: sandbox / path.relative_to(ROOT) for path in tuple(WHOLE)}
    dirs = {sandbox}
    for path in mirrors.values():
        dirs.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
    GOOD['prepare_producer'](ROOT, sources=tuple(WHOLE), directories=tuple(dirs), output_files=tuple(mirrors.values()))
    for path, destination in mirrors.items():
        shutil.copy2(path, destination)
    original = FROZEN[SOURCE]
    saved, argv = dict(os.environ), sys.argv
    real_run = subprocess.run
    stderr = io.StringIO()
    log = sandbox / 'target/formula_inline_mutations/mutation-1.log'

    def child(arguments, **kwargs):
        assert arguments[:3] == [sys.executable, '-I', '-B'], 'D207 child interface changed'
        assert mirrors[SOURCE].read_bytes() != original, 'D207 actual source was not mutated'
        GOOD['plan_producer'](ROOT, directories=(log,))
        log.mkdir()
        return types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'inline_context_contract.py AssertionError\n')

    try:
        sys.argv = [str(mirrors[WRITER])]
        subprocess.run = child
        with contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
            try:
                exec(compile(text, str(mirrors[WRITER]), 'exec'),
                     dict(__file__=str(mirrors[WRITER]), __name__='__main__'))
            except SystemExit as error:
                assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — producer output is not a local regular file\n', ('D207 unrelated late refusal', stderr.getvalue())
            else:
                raise AssertionError('D207 late log refusal escaped')
        assert mirrors[SOURCE].read_bytes() == original, 'D207 actual source not restored'
    finally:
        subprocess.run = real_run
        sys.argv = argv
        os.environ.clear(); os.environ.update(saved)
        # The intentionally bad producer variant leaves only this owned mirror dirty.
        GOOD['plan_producer'](ROOT, sources=(mirrors[SOURCE],), mutable_sources=(mirrors[SOURCE],))
        mirrors[SOURCE].write_bytes(original)
        assert mirrors[SOURCE].read_bytes() == original and SOURCE.read_bytes() == original


late_restore_capture.calls = 0

def late_restore_controls():
    guard = "PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))"
    assert TEXTS[WRITER].count(guard) == 2, 'D207 restoration guards changed'
    broken = TEXTS[WRITER].replace(guard, "PROFILE['plan_producer'](ROOT, **plan)")
    try:
        late_restore_capture(broken)
    except AssertionError as error:
        assert str(error) == 'D207 actual source not restored', ('D207 unrelated counterfactual', str(error))
    else:
        raise AssertionError('D207 restored-output coupling escaped')
    late_restore_capture(TEXTS[WRITER])
    print('D207 actual source restoration assertion red/green; late-log CLI rc=2 retained; both mirrored sources and actual tracked source exact')



def nested_preflight_control():
    """Removing the child's actual old-entry guard admits the same unexpected fixture."""
    sandbox = ROOT / 'target/inline-nested-preflight-controls' / str(os.getpid())
    assert not sandbox.exists(), 'inline nested fixture already exists'
    mirrors = {path: sandbox / path.relative_to(ROOT) for path in WHOLE}
    dirs = {sandbox}
    for destination in mirrors.values():
        dirs.update(parent for parent in destination.parents if parent.is_relative_to(sandbox))
    sentinel = sandbox / 'target/formula_inline_context/src/unexpected.txt'
    dirs.add(sentinel.parent)
    GOOD['prepare_producer'](ROOT, sources=tuple(WHOLE), directories=tuple(dirs),
                             output_files=(*mirrors.values(), sentinel))
    for source, destination in mirrors.items():
        shutil.copy2(source, destination)
    sentinel.write_text('owned nested preflight sentinel\n')
    original = FROZEN[ENTRY].decode()
    node = next(node for node in ast.walk(ast.parse(original)) if isinstance(node, ast.Expr)
                and isinstance(node.value, ast.Call) and any(isinstance(argument, ast.Constant)
                and argument.value == 'unexpected copied-book fixture entry' for argument in node.value.args))
    lines = original.splitlines(keepends=True)
    disabled = ''.join(lines[:node.lineno - 1]) + "    PROFILE['require'](True, 'unexpected copied-book fixture entry')\n" + ''.join(lines[node.end_lineno:])
    saved, argv = dict(os.environ), sys.argv
    # Only the public pure planner runs; activation and source mutation are forbidden.
    hooks = (os.mkdir, Path.mkdir, Path.write_text, Path.write_bytes, subprocess.run, shutil.rmtree, shutil.copytree)
    def effect(*args, **kwargs):
        raise AssertionError('inline nested pure preflight produced an effect')
    try:
        for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT',
                     'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'RUSTUP_TOOLCHAIN'):
            os.environ.pop(name, None)
        for name, suffix in GOOD['STORES'].items():
            os.environ[name] = str(sandbox / suffix)
        namespace = runpy.run_path(str(mirrors[WRITER]))
        os.mkdir = Path.mkdir = Path.write_text = Path.write_bytes = subprocess.run = shutil.rmtree = shutil.copytree = effect
        try:
            namespace['producer_plan']()
        except namespace['PROFILE']['Refusal'] as error:
            assert str(error) == 'unexpected copied-book fixture entry', 'inline nested unrelated refusal'
        else:
            raise AssertionError('inline nested current planner admitted unexpected entry')
        os.mkdir, Path.mkdir, Path.write_text, Path.write_bytes, subprocess.run, shutil.rmtree, shutil.copytree = hooks
        GOOD['plan_producer'](ROOT, sources=(mirrors[ENTRY],), mutable_sources=(mirrors[ENTRY],))
        mirrors[ENTRY].write_text(disabled)
        namespace = runpy.run_path(str(mirrors[WRITER]))
        os.mkdir = Path.mkdir = Path.write_text = Path.write_bytes = subprocess.run = shutil.rmtree = shutil.copytree = effect
        namespace['producer_plan']()
        assert sentinel.read_text() == 'owned nested preflight sentinel\n'
        assert mirrors[SOURCE].read_bytes() == FROZEN[SOURCE] and sys.argv is argv
    finally:
        os.mkdir, Path.mkdir, Path.write_text, Path.write_bytes, subprocess.run, shutil.rmtree, shutil.copytree = hooks
        os.environ.clear(); os.environ.update(saved); sys.argv = argv
        GOOD['plan_producer'](ROOT, sources=(mirrors[ENTRY],), mutable_sources=(mirrors[ENTRY],))
        mirrors[ENTRY].write_text(original)
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'inline nested tracked input drift'
    print('inline nested writer preflight: actual child old-entry guard refusal/omission admission; no effects; exact sources')


def main():
    GOOD['prepare_producer'](ROOT, sources=tuple(WHOLE), directories=tuple(COPY_DIRS), output_files=tuple(COPY_FILES))
    for source in BOOK_FILES:
        shutil.copy2(source, WORK / 'src' / source.relative_to(BOOK))
    for mode in ('import', 'plan', 'cli', 'borrowed'):
        capture(ENTRY, mode)
    for mode in ('import', 'cli', 'borrowed', 'writer-fault'):
        capture(WRITER, mode)
    selectors = {name: str(file.relative_to(ROOT)) for name, file in (
        ('FORMULA_CONTRACT', BOOK / 'spec/formula-language.md'),
        ('FORMULA_GRAMMAR', BOOK / 'spec/formula-language/grammar.md'),
        ('FORMULA_EXAMPLES', BOOK / 'spec/formula-language/examples.md'),
        ('FORMULA_FIXTURE', BOOK / 'spec/reference-skirt.md'),
        ('FORMULA_PARTS_DIR', BOOK / 'spec/formula-language'))}
    selectors['FORMULA_ROOT'] = 'unused-root-preserved'
    capture(ENTRY, 'borrowed', selectors=selectors)
    fault_controls()
    physical_controls()
    borrowed_physical_controls()
    late_restore_controls()
    nested_preflight_control()
    print('inline controls: actual boundaries/original bodies/guard reds and physical refusals; real verdicts separate')


if __name__ == '__main__':
    main()
