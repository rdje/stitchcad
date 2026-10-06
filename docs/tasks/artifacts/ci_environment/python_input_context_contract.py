"""Actual input context/fixture/source-writer captures; numerical proof runs separately."""
from pathlib import Path
import ast
import contextlib
import inspect
import io
import os
import runpy
import shutil
import subprocess
import sys
import types

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
ENTRY = HERE / 'formula_input.py'
WRITER = HERE / 'formula_input_mutations.py'
DIAGNOSTIC = HERE / 'literal_diagnostic.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
BOOK = ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_input'
WRITER_WORK = ROOT / 'target/formula_input_mutations'
GOOD = runpy.run_path(str(PROFILE_SOURCE))
BOOT = (ENTRY, PROFILE_SOURCE, SOURCE, HERE / 'formula_structure.py', HERE / 'static_signature_contract.py')
REFERENCE = (*BOOT, *(BOOK / name for name in ('spec/formula-language.md',
    'spec/formula-language/grammar.md', 'spec/formula-language/examples.md',
    'spec/reference-skirt.md', 'spec/units-and-tolerances.md')))
DIAG_BOOT = (DIAGNOSTIC, PROFILE_SOURCE, ENTRY, BOOT[3], BOOT[4], SOURCE)
DIAG_REFERENCE = (DIAGNOSTIC, *REFERENCE)
WRITER_INPUTS = (WRITER, PROFILE_SOURCE, SOURCE, ENTRY, BOOT[3], BOOT[4], *REFERENCE[5:])
LABELS = ('missing_unit_gap', 'malformed_binding', 'keyword_binding')
SIGNATURES = ('unit follows it with exactly one space', 'violates lower-snake spelling',
              'is not an identifier here')
GOOD['directory'](ROOT, 'docs/book/src')
directories, files = [BOOK], []
for folder in directories:
    GOOD['directory'](ROOT, folder.relative_to(ROOT).as_posix())
    for path in sorted(folder.iterdir()):
        GOOD['require'](not path.is_symlink(), 'symlink control book entry')
        (directories if path.is_dir() else files).append(path)
BOOK_FILES = tuple(files)
WHOLE = set((*BOOT, WRITER, DIAGNOSTIC, *BOOK_FILES))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in WHOLE}
TEXTS = {path: FROZEN[path].decode() for path in (ENTRY, WRITER, DIAGNOSTIC)}
FILES = {WORK / label / 'src' / path.relative_to(BOOK) for label in LABELS for path in BOOK_FILES} | {
    WORK / (label + '.log') for label in LABELS}
WRITER_FILES = {WRITER_WORK / ('mutation-%d.log' % n) for n in range(1, 10)}
BEFORE = '4d3ec9f7fd6636d3edd968d2245074e6274e8592'


def capture(entry, mode, replacements=None, selectors=None):
    """Validate actual per-call plans, writes and dispatch; preserve borrowed state on errors."""
    texts = dict(TEXTS)
    texts.update(replacements or {})
    saved, argv = dict(os.environ), sys.argv
    custom = {name: str(ROOT / 'target/input-capture-stores' / suffix) for name, suffix in (
        ('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in custom.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    originals = (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text,
                 Path.write_bytes, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree, Path.mkdir)
    plans, frames, prepared, copies, removed, written, children = [], {}, [], [], [], [], []
    writer_bytes = [FROZEN[SOURCE]]
    child_checkpoint = [0]
    writer_checkpoint = [0]
    output = io.StringIO()

    def caller():
        frame = sys._getframe(2)
        while frame and Path(frame.f_globals.get('__file__', '')) not in TEXTS:
            frame = frame.f_back
        return frame

    def plan(root, **kwargs):
        assert root == ROOT, 'input borrowed root changed'
        frame = caller()
        assert frame is not None, 'input public caller missing'
        owner = Path(frame.f_globals['__file__'])
        sources = set(kwargs.get('sources', ()))
        if owner == WRITER:
            assert set(kwargs.get('mutable_sources', ())) == {SOURCE}, 'writer mutable authority changed'
            if sources == {SOURCE}:
                assert not kwargs.get('output_files') and not kwargs.get('directories'), 'writer restoration depends on unrelated outputs'
            else:
                assert sources == set(WRITER_INPUTS), 'writer source plan incomplete'
                assert set(kwargs.get('output_files', ())) == WRITER_FILES and kwargs.get('directories') == (WRITER_WORK,), 'writer output plan incomplete'
        elif owner == DIAGNOSTIC:
            assert not any(kwargs.get(name) for name in ('directories', 'output_files', 'mutable_sources')), 'diagnostic pure plan has effects'
            assert tuple(kwargs['sources']) in (DIAG_BOOT, DIAG_REFERENCE), 'diagnostic source plan incomplete'
        else:
            assert not kwargs.get('mutable_sources'), 'input tracked mutation authority'
        before = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == before, 'input pure plan changed environment'
        frames[frame] = kwargs
        plans.append(kwargs)
        return result

    def prepare(root, **kwargs):
        assert root == ROOT, 'input activation root changed'
        if caller().f_globals['__file__'] == str(WRITER):
            assert any(p == kwargs for p in plans), 'writer activation before pure plan'
        else:
            labels = caller().f_locals['labels']
            assert set(labels) <= set(LABELS), 'input unowned label'
            expected = {WORK / label / 'src' / p.relative_to(BOOK) for label in labels for p in BOOK_FILES} | {WORK / (label + '.log') for label in labels}
            assert set(kwargs.get('output_files', ())) == expected, 'input file plan incomplete'
            assert set(kwargs.get('sources', ())) >= set((*BOOT, *BOOK_FILES)), 'input producing source plan incomplete'
            expected_dirs = {WORK} | {WORK / label / 'src' / folder.relative_to(BOOK) for label in labels for folder in directories}
            assert set(kwargs.get('directories', ())) == expected_dirs, 'input directory plan incomplete'
            assert any(p == kwargs for p in plans), 'input activation before pure plan'
        prepared.append(kwargs)
        return GOOD['prepare_producer'](root, **kwargs)

    def effect(*args, **kwargs):
        raise AssertionError('input unexpected pure effect')

    def read(path, binary, *args, **kwargs):
        frame = caller()
        if frame is not None:
            owner = Path(frame.f_globals['__file__'])
            if owner == WRITER and path == SOURCE:
                assert frame in frames and prepared, 'writer read before producing plan'
                return writer_bytes[0] if binary else writer_bytes[0].decode()
            if path.is_relative_to(WORK) and '/src/' in str(path):
                base = WORK / path.relative_to(WORK).parts[0] / 'src'
                copied = {base / p.relative_to(BOOK) for p in BOOK_FILES}
                assert any(copied <= set(p.get('sources', ())) and copied <= set(p.get('output_files', ())) for p in plans), 'input copied source plan omitted'
            elif owner == ENTRY and frame.f_code.co_name in ('load_context', 'load_arithmetic_context'):
                assert set(frames.get(frame, {}).get('sources', ())) == set(REFERENCE), 'input read before fresh reference plan'
        return originals[2 if binary else 1](path, *args, **kwargs)

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, prepare_producer=prepare, enter_producer=effect)
        frame = caller()
        if frame is not None and Path(frame.f_globals['__file__']) == DIAGNOSTIC and path == ENTRY:
            assert tuple(frames.get(frame, {}).get('sources', ())) == DIAG_BOOT, 'diagnostic bootstrap omitted'
        if frame is not None and Path(frame.f_globals['__file__']) == ENTRY and path == BOOT[3]:
            assert tuple(frames.get(frame, {}).get('sources', ())) == REFERENCE, 'input delegation before fresh reference plan'
        if path in texts:
            namespace = dict(__file__=str(path), __name__='borrowed_input')
            exec(compile(texts[path], str(path), 'exec'), namespace)
            provider = namespace['load_context']
            def borrowed(*args, **kwargs):
                parent = caller()
                if parent and Path(parent.f_globals['__file__']) == DIAGNOSTIC:
                    assert tuple(frames.get(parent, {}).get('sources', ())) == DIAG_REFERENCE, 'diagnostic delegation plan omitted'
                return provider(*args, **kwargs)
            namespace['load_context'] = borrowed
            return namespace
        # Internal guards of borrowed providers use the unmodified shared profile. Our
        # entry's complete fresh plan is still required at the delegation boundary.
        runpy.run_path = originals[0]
        try:
            result = originals[0](file, *args, **kwargs)
        finally:
            runpy.run_path = load
        if path == BOOT[3]:
            provider = result['load_reference']
            def context(*args, **kwargs):
                parent = caller()
                assert tuple(frames.get(parent, {}).get('sources', ())) == REFERENCE, 'input borrowed reference plan omitted'
                if mode == 'fault':
                    raise RuntimeError('input injected context failure')
                return provider(*args, **kwargs)
            result['load_reference'] = context
        return result

    def authorized(path):
        assert any(path in p.get('output_files', ()) for p in prepared), 'input write before producing plan'
        assert path in FILES | WRITER_FILES, 'input unowned output'
        assert all(os.environ.get(k) == v for k, v in custom.items()), 'input stores lost'

    def write(path, data, binary, *args, **kwargs):
        if path == SOURCE:
            frame = caller()
            assert frame in frames and plans[-1] == frames[frame] and len(plans) > writer_checkpoint[0], 'writer write recheck omitted'
            writer_checkpoint[0] = len(plans)
            assert SOURCE in frames[frame].get('mutable_sources', ()), 'writer unowned tracked write'
            # Model only the already-declared source bytes; the original nine actual
            # failing children run separately against the real source, exclusively.
            writer_bytes[0] = data if binary else data.encode()
        else:
            authorized(path)
            if path in WRITER_FILES:
                assert plans[-1] == frames.get(caller()) and len(plans) > writer_checkpoint[0], 'writer log recheck omitted'
                writer_checkpoint[0] = len(plans)
            originals[4 if binary else 3](path, data, *args, **kwargs)
        if path in FILES and path.name == 'examples.md':
            child_checkpoint[0] = len(plans)
        written.append(path)
        return len(data)

    def mkdir(path, *args, **kwargs):
        path = Path(path)
        assert prepared and (path == WRITER_WORK or path == WORK or any(path.is_relative_to(WORK / label) for label in LABELS)), 'input mkdir before producing plan'
        return originals[5](path, *args, **kwargs)

    def remove(path, *args, **kwargs):
        path = Path(path)
        expected = {path / p.relative_to(BOOK) for p in BOOK_FILES} | {WORK / (path.parent.name + '.log')}
        assert set(plans[-1].get('output_files', ())) == expected, 'input delete recheck omitted'
        assert prepared, 'input delete before producing plan'
        removed.append(path)
        return originals[7](path, *args, **kwargs)

    def copyfile(source, destination):
        source, destination = Path(source), Path(destination)
        assert source in BOOK_FILES, 'input unplanned copy source'
        authorized(destination)
        GOOD['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        written.append(destination)
        return shutil.copy2(source, destination)

    def copytree(source, destination, *args, **kwargs):
        source, destination = Path(source), Path(destination)
        if source == BOOK:
            assert not args and not kwargs, 'input copy interface changed'
            copies.append(destination)
            return originals[8](source, destination, copy_function=copyfile)
        bound = inspect.signature(originals[8]).bind(source, destination, *args, **kwargs)
        bound.apply_defaults()
        assert bound.arguments['copy_function'] is copyfile and bound.arguments['ignore'] is None, 'input recursive copier changed'
        assert not any(bound.arguments[n] for n in ('symlinks', 'ignore_dangling_symlinks', 'dirs_exist_ok')), 'input recursive copy options changed'
        return originals[8](source, destination, *args, **kwargs)

    def child(arguments, *args, **kwargs):
        env = kwargs.get('env', {})
        assert not args and kwargs.get('cwd') == ROOT and kwargs.get('capture_output') is True, 'input child interface changed'
        assert all(env.get(k) == v for k, v in custom.items()) and env.get('MAKE_TMPDIR') == custom['TMPDIR'], 'input child stores lost'
        assert env.get('INPUT_UNRELATED') == 'preserved', 'input unrelated environment lost'
        if entry == WRITER:
            assert arguments == [sys.executable, '-I', '-B', ENTRY.relative_to(ROOT).as_posix(), '--contracts-only'], 'writer child argv changed'
            assert plans[-1] == frames.get(caller()) and len(plans) > writer_checkpoint[0], 'writer child recheck omitted'
            writer_checkpoint[0] = len(plans)
            assert writer_bytes[0] != FROZEN[SOURCE], 'writer source mutation missing'
            if mode == 'writer-fault':
                raise RuntimeError('input injected writer failure')
            result = types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'formula_input.py AssertionError\n')
        else:
            assert arguments == ['bash', str(SOURCE)] and kwargs.get('text') is True, 'input shell child changed'
            label = LABELS[len(children)]
            base = WORK / label / 'src'
            assert env.get('FORMULA_BOOK') == str(base), 'input copied book selector lost'
            assert all(env.get(k) == v for k, v in (selectors or {}).items()), 'input caller selectors lost'
            assert any(set((*BOOT, *BOOK_FILES)) <= set(p.get('sources', ())) for p in plans[child_checkpoint[0]:]), 'input child source recheck omitted'
            # Independently resolve the consumer's declared lexical inputs from the
            # effective copied-book paths; do not ask the producer for its expectation.
            parts = Path(env.get('FORMULA_PARTS_DIR') or base / 'spec/formula-language')
            selected = [Path(env.get(k) or default) for k, default in (
                ('FORMULA_CONTRACT', base / 'spec/formula-language.md'),
                ('FORMULA_GRAMMAR', parts / 'grammar.md'), ('FORMULA_EXAMPLES', parts / 'examples.md'),
                ('FORMULA_FIXTURE', base / 'spec/reference-skirt.md'))]
            selected = [p if p.is_absolute() else ROOT / p for p in selected]
            parts = parts if parts.is_absolute() else ROOT / parts
            references = []
            import re
            for path in selected[:3]:
                for _, target in re.findall(r'\[([^\]]*)\]\(([^)]+)\)', originals[1](path)):
                    if not target.startswith(('http://', 'https://', '#')):
                        references.append(Path(os.path.normpath(str(path.parent) + '/' + target.split('#')[0])))
            expected = set((*selected, *sorted(parts.glob('*.md')), *references))
            assert set(plans[-1].get('sources', ())) == expected, 'input child selector recheck omitted'
            result = types.SimpleNamespace(returncode=1, stdout='formula_parse '+SIGNATURES[len(children)]+'\n', stderr='')
            child_checkpoint[0] = len(plans)
        children.append(arguments)
        return result

    namespace = dict(__file__=str(entry), __name__='__main__' if mode == 'cli' else 'borrowed_input')
    expected_environment = dict(saved, **custom, MAKE_TMPDIR=custom['TMPDIR'], RUSTUP_AUTO_INSTALL='0',
                                INPUT_UNRELATED='preserved', RUSTUP_TOOLCHAIN='1.99.0')
    try:
        for key in ('FORMULA_ROOT','FORMULA_BOOK','FORMULA_CONTRACT','FORMULA_GRAMMAR','FORMULA_EXAMPLES','FORMULA_FIXTURE','FORMULA_PARTS_DIR'):
            expected_environment.pop(key, None)
        expected_environment.update(selectors or {})
        os.environ.clear(); os.environ.update(expected_environment)
        sys.argv = [str(entry), *(['--contracts-only'] if mode == 'contracts-cli' else [])]
        current_argv = sys.argv
        namespace['__name__'] = '__main__' if mode in ('cli','contracts-cli') else 'borrowed_input'
        runpy.run_path = load
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.write_text = lambda path, data, *args, **kwargs: write(path, data, False, *args, **kwargs)
        Path.write_bytes = lambda path, data, *args, **kwargs: write(path, data, True, *args, **kwargs)
        os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = mkdir, child, remove, copytree
        def path_mkdir(path, *args, **kwargs):
            assert prepared, 'input mkdir before producing plan'
            return originals[9](path, *args, **kwargs)
        Path.mkdir = path_mkdir
        with contextlib.redirect_stdout(output):
            try:
                exec(compile(texts[entry], str(entry), 'exec'), namespace)
                if entry == ENTRY and mode not in ('cli','contracts-cli','import'):
                    for unused in range(2 if mode in ('load','arithmetic','contracts') else 1):
                        if mode in ('load','fault'):
                            namespace['load_context']()
                        elif mode == 'arithmetic':
                            provider = namespace['load_context']
                            def arithmetic_context():
                                frame = caller()
                                assert tuple(frames.get(frame, {}).get('sources', ())) == REFERENCE, 'input arithmetic delegation plan omitted'
                                return provider()
                            namespace['load_context'] = arithmetic_context
                            try:
                                namespace['load_arithmetic_context']()
                            finally:
                                namespace['load_context'] = provider
                        else:
                            namespace['contracts' if mode == 'contracts' else 'end_to_end']()
                elif entry == WRITER and mode != 'cli':
                    namespace['main']()
            except RuntimeError as error:
                assert (mode, str(error)) in (('fault', 'input injected context failure'), ('writer-fault', 'input injected writer failure')), 'input unrelated execution failure'
        assert sys.argv is current_argv and dict(os.environ) == expected_environment, 'input normal/exception borrowed state changed'
        if entry == ENTRY:
            assert any(tuple(p.get('sources', ())) == BOOT for p in plans), 'input bootstrap omitted'
        if entry == WRITER:
            assert writer_bytes[0] == FROZEN[SOURCE], 'writer capture source restoration failed'
            assert (len(children), written.count(SOURCE)) == ((0, 2) if mode == 'writer-fault' else (9, 19)), 'writer capture mutation population changed'
        elif mode in ('cli','end-to-end') and entry == ENTRY:
            assert len(children) == 3 and len(copies) == len(removed) == 3 and set(written) == FILES, 'input copied dispatch coverage incomplete'
            assert len(written) == 3 * (len(BOOK_FILES) + 2), 'input copy/write population changed'
        else:
            assert not prepared and not written and not children, 'input pure call produced effects'
    finally:
        (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text,
         Path.write_bytes, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree, Path.mkdir) = originals
        sys.argv = argv; os.environ.clear(); os.environ.update(saved)
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'input tracked sources changed'
    print('actual input capture:', entry.name, mode, len(plans), 'plans;', len(written), 'writes;', len(children), 'intercepted children; no numerical verdict inferred')


def original_bodies():
    class Guards(ast.NodeTransformer):
        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and ast.unparse(node.value.func) == "PROFILE['plan_producer']":
                return None
            return self.generic_visit(node)
        def visit_Call(self, node):
            if ast.unparse(node.func) == 'subprocess.run':
                node.keywords = [kw for kw in node.keywords if kw.arg != 'env']
            return self.generic_visit(node)
    old = {path: subprocess.check_output(['git','show',BEFORE+':'+path.relative_to(ROOT).as_posix()], text=True) for path in TEXTS}
    previous = ast.parse(old[ENTRY]); current = Guards().visit(ast.parse(TEXTS[ENTRY]))
    for name in ('load_context','load_arithmetic_context','contracts'):
        pick = lambda tree: next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name==name)
        assert ast.dump(pick(previous)) == ast.dump(pick(current)), ('input original body changed',name)
    old_end = next(n for n in previous.body if isinstance(n,ast.FunctionDef) and n.name=='end_to_end')
    new_end = next(n for n in current.body if isinstance(n,ast.FunctionDef) and n.name=='end_to_end')
    assert ast.dump(old_end.body[1].iter) == ast.dump(new_end.body[2].iter), 'input original three fault cases changed'
    old_loop = Guards().visit(old_end.body[1]).body[3:]
    new_loop = [n for n in new_end.body[2].body[1:] if not (isinstance(n, ast.Expr) and isinstance(n.value, ast.Call) and ast.unparse(n.value.func) in ('source_plan', 'consumer_inputs'))]
    assert [ast.dump(n) for n in old_loop] == [ast.dump(n) for n in new_loop], 'input original copy-edit/child/assertion body changed'
    previous = ast.parse(old[WRITER]); current = Guards().visit(ast.parse(TEXTS[WRITER]))
    cases = lambda tree: next(n for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='CASES' for t in n.targets))
    assert ast.dump(cases(previous)) == ast.dump(cases(current)), 'writer original nine cases changed'
    old_body = [n for n in previous.body if isinstance(n,(ast.Try,ast.Assert)) or isinstance(n,ast.Expr)]
    new_body = next(n for n in current.body if isinstance(n,ast.FunctionDef) and n.name=='main').body[5:]
    assert [ast.dump(n) for n in old_body[2:]] == [ast.dump(n) for n in new_body], 'writer original assertions/restoration body changed'
    old_diag = ast.parse(old[DIAGNOSTIC]).body[5:]
    new_diag = next(n for n in ast.parse(TEXTS[DIAGNOSTIC]).body if isinstance(n,ast.Try)).body[5:]
    assert [ast.dump(n) for n in old_diag] == [ast.dump(n) for n in new_diag], 'diagnostic original observations changed'
    return old


def fault_controls():
    previous = original_bodies()
    for entry, mode, wanted in ((ENTRY, 'import', 'input bootstrap omitted'), (DIAGNOSTIC, 'cli', 'diagnostic bootstrap omitted'), (WRITER, 'cli', 'input mkdir before producing plan')):
        try:
            capture(entry, mode, {entry: previous[entry]})
        except AssertionError as error:
            assert str(error) == wanted, ('input unrelated original gap', entry.name, str(error))
        else:
            raise AssertionError(('input original gap escaped', entry.name))
    faults = [
        (ENTRY,'import',"PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)",'None','input bootstrap omitted'),
        (ENTRY,'load',"PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)",'None','input delegation before fresh reference plan'),
        (DIAGNOSTIC,'cli',"PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)",'None','diagnostic bootstrap omitted'),
        (DIAGNOSTIC,'cli',"PROFILE['plan_producer'](ROOT, sources=(Path(__file__), *INPUT['REFERENCE_INPUTS']))",'None','diagnostic delegation plan omitted'),
        (ENTRY,'end-to-end','output_files=outputs, sources=sources)','output_files=outputs[:-1], sources=sources)','input file plan incomplete'),
        (ENTRY,'end-to-end','directories=directories, output_files=outputs','directories=directories[:-1], output_files=outputs','input directory plan incomplete'),
        (ENTRY,'end-to-end','output_files=outputs, sources=sources)','output_files=outputs, sources=sources[:-1])','input producing source plan incomplete'),
        (ENTRY,'end-to-end',"return PROFILE['prepare_producer'](ROOT, **plan) if activate else None", "return PROFILE['environment_for'](ROOT, os.environ, create=False) if activate else None",'input mkdir before producing plan'),
        (ENTRY,'end-to-end','fixture_plan((label,), activate=False)','None','input delete recheck omitted'),
        (ENTRY,'end-to-end',"PROFILE['plan_producer'](ROOT, sources=(TOOL, *files), output_files=files)",'None','input copied source plan omitted'),
        (ENTRY,'end-to-end','        source_plan()\n        consumer_inputs','        consumer_inputs','input child source recheck omitted'),
        (ENTRY,'end-to-end','        consumer_inputs(dict(environment, FORMULA_BOOK=str(book)))\n','', 'input child selector recheck omitted'),
        (ENTRY,'end-to-end','env=dict(environment, FORMULA_BOOK=str(book))',"env=dict(environment, FORMULA_BOOK=str(book), TMPDIR=str(ROOT/'target/wrong-scratch'))",'input child stores lost'),
        (WRITER,'cli',"    PROFILE['plan_producer'](ROOT, **plan)\n    environment",'    environment','writer activation before pure plan'),
        (WRITER,'cli','env=environment','env={}','input child stores lost'),
    ]
    faults.extend((WRITER, 'cli', anchor, replacement, wanted) for anchor, replacement, wanted in (
        ("            PROFILE['plan_producer'](ROOT, **plan)\n            SOURCE.write_text", '            SOURCE.write_text', 'writer write recheck omitted'),
        ("            PROFILE['plan_producer'](ROOT, **plan)\n            result", '            result', 'writer child recheck omitted'),
        ("            PROFILE['plan_producer'](ROOT, **plan)\n            (WORK", '            (WORK', 'writer log recheck omitted'),
        ("            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))\n            SOURCE.write_bytes", '            SOURCE.write_bytes', 'writer write recheck omitted'),
        ("        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))\n        SOURCE.write_bytes", '        SOURCE.write_bytes', 'writer write recheck omitted')))
    arithmetic = next(n for n in ast.parse(TEXTS[ENTRY]).body if isinstance(n, ast.FunctionDef) and n.name == 'load_arithmetic_context')
    guard = arithmetic.body[1]
    lines = TEXTS[ENTRY].splitlines(keepends=True)
    disabled = ''.join(lines[:guard.lineno-1]) + '    None\n' + ''.join(lines[guard.end_lineno:])
    try:
        capture(ENTRY, 'arithmetic', {ENTRY: disabled})
    except AssertionError as error:
        assert str(error) == 'input arithmetic delegation plan omitted', str(error)
    else:
        raise AssertionError('input arithmetic guard omission escaped')
    for entry, mode, anchor, replacement, wanted in faults:
        assert TEXTS[entry].count(anchor) >= 1, ('input fault anchor changed',entry.name,anchor)
        try:
            capture(entry,mode,{entry:TEXTS[entry].replace(anchor,replacement,1)})
        except AssertionError as error:
            assert str(error)==wanted, ('input unrelated guard red',entry.name,anchor,str(error))
        else:
            raise AssertionError(('input actual guard omission escaped',entry.name,anchor))
    print('input original context/130 assertions/three input faults/nine mutation cases/literal observations exact;',len(faults)+1,'actual guard reds')


def physical_controls():
    """Refuse real filesystem faults in owned isolated copies before any producer effect."""
    cases = ('missing input', 'symlink input', 'symlink book directory', 'input is directory',
             'unexpected old fixture', 'symlink output', 'hardlinked output',
             'output directory is file', 'symlink log', 'outside selector',
             'input output alias', 'parts directory alias', 'symlink store', 'invalid toolchain', 'pure unused sidebar')
    def effect(*args, **kwargs):
        raise AssertionError('input physical effect before refusal')
    generation = ROOT / 'target/fixture-input-controls' / str(os.getpid())
    GOOD['directory'](ROOT, generation.parent.relative_to(ROOT).as_posix(), create=True)
    assert not generation.exists(), 'input fixture generation already exists'
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
        leaf = work / 'missing_unit_gap/src/spec/formula-language.md'
        log = work / 'missing_unit_gap.log'
        sidebar = sandbox / 'docs/book/src/annexes/formula-static-validation.md'
        input_file = sandbox / 'docs/book/src/spec/units-and-tolerances.md'
        extra = work / 'missing_unit_gap/src/unexpected.txt'
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
            base = work / 'missing_unit_gap/src'
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
            parts = work / 'missing_unit_gap/src/spec/formula-language'
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
                    assert namespace['contracts']() is None
                else:
                    try:
                        namespace['end_to_end']()
                    except namespace['PROFILE']['Refusal'] as error:
                        assert str(error) == expected, (case, 'unrelated physical refusal', str(error))
                        if case == 'parts directory alias':
                            guard = 'not any(parts == base or parts.is_relative_to(base) for base in bases)'
                            assert TEXTS[ENTRY].count(guard) == 1
                            disabled = dict(__file__=str(mirrors[ENTRY]), __name__='directory_guard_disabled')
                            exec(compile(TEXTS[ENTRY].replace(guard, 'True', 1), str(mirrors[ENTRY]), 'exec'), disabled)
                            disabled['fixture_plan'](LABELS, activate=False)
                            assert not tuple(parts.iterdir()), 'D202 counterfactual changed directory'
                            print('Inherited D202 actual directory-alias guard omission admits same empty selected directory; no effects')

                    else:
                        raise AssertionError((case, 'input physical refusal escaped'))
        except SystemExit as error:
            assert error.code == 2 and expected and expected in stderr.getvalue(), (case, 'unrelated bootstrap refusal', stderr.getvalue())
        finally:
            os.mkdir, shutil.rmtree, shutil.copytree, subprocess.run, Path.write_text, Path.write_bytes, Path.mkdir = originals
            os.environ.clear(); os.environ.update(saved)
        print('input physical control:', case)
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
        namespace['fixture_plan'](LABELS, activate=False)
        assert (sandbox / 'target/formula_input/missing_unit_gap/src/spec/formula-language.md').stat().st_nlink == 2
    finally:
        profile.write_text(original)
        os.environ.clear(); os.environ.update(saved)
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'input physical controls changed tracked sources'
    print('input guard counterfactual: actual profile planner disabled admits same hardlinked output; tracked sources exact')




def borrowed_physical_controls():
    """Actual CLI and repeated borrowed-context refusals, plus mutable-source link safety."""
    generation = ROOT / 'target/input-boundary-controls' / str(os.getpid())
    total = borrowed = 0
    cases = ('missing','symlink','directory','nested-repo','symlink-store','invalid-toolchain')
    selected = [(entry,case) for entry in TEXTS for case in cases] + [(WRITER,'hardlinked-source')]
    for index,(entry,case) in enumerate(selected):
        sandbox = generation / str(index)
        assert not sandbox.exists(), 'input boundary fixture already exists'
        inputs = REFERENCE if entry == ENTRY else DIAG_REFERENCE if entry == DIAGNOSTIC else WRITER_INPUTS
        mirrors = {path:sandbox/path.relative_to(ROOT) for path in inputs}
        dirs={sandbox}
        for path in mirrors.values():
            dirs.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
        GOOD['prepare_producer'](ROOT,sources=inputs,directories=tuple(dirs),output_files=tuple(mirrors.values()))
        for source,destination in mirrors.items():
            shutil.copy2(source,destination)
        victim = mirrors[SOURCE if entry == WRITER else REFERENCE[-1]]
        wanted='producer source is not a local regular file'
        extra={}
        if case=='missing': victim.unlink()
        elif case=='symlink':
            victim.unlink(); victim.symlink_to(mirrors[PROFILE_SOURCE]); wanted='symlink producer source'
        elif case=='directory':
            victim.unlink(); victim.mkdir()
        elif case=='nested-repo':
            (victim.parent/'.git').mkdir(); wanted='store crosses another Git repository'
        elif case=='symlink-store':
            scratch=sandbox/'target/scratch'
            GOOD['directory'](ROOT,scratch.parent.relative_to(ROOT).as_posix(),create=True)
            scratch.symlink_to(victim.parent,target_is_directory=True); wanted='symlink store component'
        elif case=='hardlinked-source':
            alias=sandbox/'target/source-alias'
            GOOD['prepare_producer'](ROOT,directories=(alias.parent,),output_files=(alias,))
            os.link(victim,alias); wanted='hard-linked mutable producer source'
        else:
            extra['RUSTUP_TOOLCHAIN']='invalid/toolchain'; wanted='toolchain override must be a name'
        saved,argv=dict(os.environ),sys.argv
        originals=(os.mkdir,Path.write_text,Path.write_bytes,subprocess.run,Path.mkdir)
        def effect(*args,**kwargs): raise AssertionError('input boundary refusal produced an effect')
        stderr=io.StringIO()
        try:
            os.environ.update({name:str(sandbox/suffix) for name,suffix in (
                ('CARGO_HOME','target/cargo-home'),('RUSTUP_HOME','target/rustup'),
                ('CARGO_TARGET_DIR','target/build'),('TMPDIR','target/scratch'))})
            os.environ.pop('RUSTUP_TOOLCHAIN',None); os.environ.update(extra)
            sys.argv=[str(mirrors[entry]),'--contracts-only']
            os.mkdir=Path.write_text=Path.write_bytes=subprocess.run=Path.mkdir=effect
            with contextlib.redirect_stderr(stderr),contextlib.redirect_stdout(io.StringIO()):
                try:
                    runpy.run_path(str(mirrors[entry]),run_name='__main__')
                except SystemExit as error:
                    assert error.code==2 and stderr.getvalue()=='local-environment: REFUSED — '+wanted+'\n', (entry.name,case,stderr.getvalue())
                else: raise AssertionError(('input boundary fault accepted',entry.name,case))
                if entry==ENTRY and case in cases[:4]:
                    namespace=runpy.run_path(str(mirrors[entry]))
                    for name in ('load_context','load_arithmetic_context'):
                        try: namespace[name]()
                        except namespace['PROFILE']['Refusal'] as error:
                            assert str(error)==wanted, 'input borrowed refusal identity changed'
                        else: raise AssertionError(('input borrowed refusal escaped',name,case))
                        borrowed+=1
        finally:
            os.mkdir,Path.write_text,Path.write_bytes,subprocess.run,Path.mkdir=originals
            os.environ.clear(); os.environ.update(saved); sys.argv=argv
        total+=1
    print('input boundaries:',total,'actual CLI refusals;',borrowed,'borrowed context refusals; no writes/activation/child dispatch')



def late_restore_capture(text):
    """D207: a real mirrored write must be restored despite an unrelated late log fault."""
    sandbox = ROOT / 'target/input-late-restore-controls' / str(os.getpid()) / str(late_restore_capture.calls)
    late_restore_capture.calls += 1
    assert not sandbox.exists(), 'D207 fixture already exists'
    mirrors = {path: sandbox / path.relative_to(ROOT) for path in WRITER_INPUTS}
    dirs = {sandbox}
    for path in mirrors.values():
        dirs.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
    GOOD['prepare_producer'](ROOT, sources=WRITER_INPUTS, directories=tuple(dirs), output_files=tuple(mirrors.values()))
    for path, destination in mirrors.items():
        shutil.copy2(path, destination)
    original = FROZEN[SOURCE]
    saved, argv = dict(os.environ), sys.argv
    real_run = subprocess.run
    stderr = io.StringIO()
    log = sandbox / 'target/formula_input_mutations/mutation-1.log'

    def child(arguments, **kwargs):
        assert arguments[:3] == [sys.executable, '-I', '-B'], 'D207 child interface changed'
        assert mirrors[SOURCE].read_bytes() != original, 'D207 actual source was not mutated'
        GOOD['plan_producer'](ROOT, directories=(log,))
        log.mkdir()
        return types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'formula_input.py AssertionError\n')

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


def main():
    # The actual existing copies are deliberately primed, so every replacement/delete
    # boundary is exercised. Plan every source and destination before the setup effect.
    outputs = tuple(WORK / label / 'src' / p.relative_to(BOOK) for label in LABELS for p in BOOK_FILES)
    dirs = tuple(WORK / label / 'src' / folder.relative_to(BOOK) for label in LABELS for folder in directories)
    GOOD['prepare_producer'](ROOT,sources=tuple(WHOLE),directories=dirs,output_files=outputs)
    for label in LABELS:
        for source in BOOK_FILES:
            shutil.copy2(source,WORK/label/'src'/source.relative_to(BOOK))
    for mode in ('import','load','arithmetic','contracts','contracts-cli','fault','end-to-end','cli'):
        capture(ENTRY,mode)
    capture(DIAGNOSTIC,'cli')
    capture(WRITER,'cli')
    capture(WRITER,'borrowed')
    capture(WRITER,'writer-fault')
    selectors={name:str(path.relative_to(ROOT)) for name,path in (
        ('FORMULA_CONTRACT',BOOK/'spec/formula-language.md'),
        ('FORMULA_GRAMMAR',BOOK/'spec/formula-language/grammar.md'),
        ('FORMULA_EXAMPLES',BOOK/'spec/formula-language/examples.md'),
        ('FORMULA_FIXTURE',BOOK/'spec/reference-skirt.md'),
        ('FORMULA_PARTS_DIR',BOOK/'spec/formula-language'))}
    selectors['FORMULA_ROOT']='unused-root-preserved'
    capture(ENTRY,'end-to-end',selectors=selectors)
    fault_controls()
    physical_controls()
    borrowed_physical_controls()
    late_restore_controls()
    assert all(path.read_bytes()==data for path,data in FROZEN.items()), 'input final sources changed'
    print('input context controls: actual pure/copy/source-writer captures and guard counterfactuals; real grammar verdicts remain separate')


if __name__=='__main__':
    main()
