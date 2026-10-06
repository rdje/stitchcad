"""Actual structural/inventory boundaries; intercepted dispatch is not numerical proof."""
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
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
SIGNATURE = HERE / 'static_signature_contract.py'
INPUT = HERE / 'formula_input.py'
STRUCTURE = HERE / 'formula_structure.py'
INVENTORY = HERE / 'canonical_contract_inventory.py'
WRITERS = (HERE / 'formula_structure_mutations.py', HERE / 'canonical_contract_inventory_mutations.py')
ENTRIES = (STRUCTURE, INVENTORY, *WRITERS)
BOOK = ROOT / 'docs/book/src'
DECISION = ROOT / 'docs/decisions/decision_canonical-expression-spelling.md'
BOOK_INPUTS = tuple(BOOK / name for name in ('spec/formula-language.md',
    'spec/formula-language/grammar.md', 'spec/formula-language/examples.md',
    'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))
BOOT = {STRUCTURE: (STRUCTURE, PROFILE_SOURCE, SOURCE, SIGNATURE),
        INVENTORY: (INVENTORY, PROFILE_SOURCE, INPUT, STRUCTURE, SIGNATURE, SOURCE)}
REFERENCE = {STRUCTURE: (*BOOT[STRUCTURE], *BOOK_INPUTS),
             INVENTORY: (INVENTORY, INPUT, PROFILE_SOURCE, SOURCE, STRUCTURE, SIGNATURE,
                         *BOOK_INPUTS, DECISION, BOOK / 'annexes/formula-literals.md')}
WRITER_INPUTS = {WRITERS[0]: (WRITERS[0], PROFILE_SOURCE, SOURCE, STRUCTURE, SIGNATURE, *BOOK_INPUTS),
                WRITERS[1]: (WRITERS[1], PROFILE_SOURCE, INVENTORY, INPUT, STRUCTURE, SIGNATURE,
                            SOURCE, *BOOK_INPUTS, DECISION, BOOK / 'annexes/formula-literals.md')}
MUTABLE = dict(zip(WRITERS, (SOURCE, INVENTORY)))
WRITER_WORK = dict(zip(WRITERS, (ROOT / 'target/formula_structure_mutations',
                                ROOT / 'target/canonical_contract_inventory_mutations')))
WRITER_FILES = {entry: {WRITER_WORK[entry] / (prefix + str(n) + '.log') for n in range(1, 5)}
                for entry, prefix in zip(WRITERS, ('mutation-', 'fault-'))}
WORK = ROOT / 'target/formula_structure'
LABELS = ('call_nodes', 'call_if_depth')
SIGNATURES = ('expression has 258 nodes, the limit is 256', 'conditional depth is 17, the limit is 16')
GOOD = runpy.run_path(str(PROFILE_SOURCE))
directories, book_files = [BOOK], []
for folder in directories:
    GOOD['directory'](ROOT, folder.relative_to(ROOT).as_posix())
    for path in sorted(folder.iterdir()):
        GOOD['require'](not path.is_symlink(), 'symlink structural control book entry')
        (directories if path.is_dir() else book_files).append(path)
BOOK_FILES = tuple(book_files)
WHOLE = set((*ENTRIES, PROFILE_SOURCE, SOURCE, SIGNATURE, INPUT, DECISION, *BOOK_FILES))
GOOD['plan_producer'](ROOT, sources=(*WHOLE, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in WHOLE}
TEXTS = {path: FROZEN[path].decode() for path in ENTRIES}
FILES = {WORK / label / 'src' / path.relative_to(BOOK) for label in LABELS for path in BOOK_FILES} | {
    WORK / (label + '.log') for label in LABELS}
BEFORE = '8d1721d5699642600991eacfdd312caf3e7c67f4'


def capture(entry, mode, replacements=None, selectors=None):
    scripts = dict(TEXTS); scripts.update(replacements or {})
    saved, argv = dict(os.environ), sys.argv
    custom = {name: str(ROOT / 'target/structure-capture-stores' / suffix) for name, suffix in (
        ('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in custom.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    originals = (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
                 Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree)
    plans, frames, generations, prepared, written, children, copies, removed = [], {}, {}, [], [], [], [], []
    checkpoint, writer_checkpoint, inventory_checkpoint = [0], [0], [0]
    shadow = {path: FROZEN[path] for path in MUTABLE.values()}
    output = io.StringIO()

    def caller():
        frame = sys._getframe(2)
        while frame and Path(frame.f_globals.get('__file__', '')) not in ENTRIES:
            frame = frame.f_back
        return frame

    def plan(root, **kwargs):
        frame = caller(); assert root == ROOT and frame is not None, 'structure plan root/caller changed'
        owner = Path(frame.f_globals['__file__']); sources = set(kwargs.get('sources', ()))
        if owner in WRITERS:
            assert set(kwargs.get('mutable_sources', ())) == {MUTABLE[owner]}, 'structure writer authority changed'
            if sources == {MUTABLE[owner]}:
                assert not kwargs.get('output_files') and not kwargs.get('directories'), 'structure restore depends on unrelated outputs'
            else:
                assert sources == set(WRITER_INPUTS[owner]), 'structure writer inputs incomplete'
                assert set(kwargs.get('output_files', ())) == WRITER_FILES[owner] and kwargs.get('directories') == (WRITER_WORK[owner],), 'structure writer outputs incomplete'
        elif owner == INVENTORY:
            assert not any(kwargs.get(k) for k in ('directories', 'output_files', 'mutable_sources')), 'inventory pure plan has effects'
            assert sources in (set(BOOT[owner]), set(REFERENCE[owner])), 'inventory inputs incomplete'
        else:
            assert not kwargs.get('mutable_sources'), 'structure unowned tracked mutation'
        before = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == before, 'structure pure plan changed environment'
        frames[frame] = kwargs; plans.append(kwargs)
        # A nested provider plan cannot refresh the inventory caller’s boundary.
        generations[frame] = len(plans)
        return result

    def prepare(root, **kwargs):
        frame = caller(); owner = Path(frame.f_globals['__file__'])
        assert any(p == kwargs for p in plans), 'structure activation before pure plan'
        if owner == STRUCTURE:
            labels = frame.f_locals['labels']
            assert set(labels) <= set(LABELS), 'structure unowned labels'
            expected = {WORK / label / 'src' / p.relative_to(BOOK) for label in labels for p in BOOK_FILES} | {WORK / (label + '.log') for label in labels}
            assert set(kwargs.get('output_files', ())) == expected, 'structure file plan incomplete'
            assert set(kwargs.get('sources', ())) >= set((*BOOT[STRUCTURE], *BOOK_FILES)), 'structure producing inputs incomplete'
            expected_dirs = {WORK} | {WORK / label / 'src' / folder.relative_to(BOOK) for label in labels for folder in directories}
            assert set(kwargs.get('directories', ())) == expected_dirs, 'structure directory plan incomplete'
        prepared.append(kwargs)
        return GOOD['prepare_producer'](root, **kwargs)

    def read(path, binary, *args, **kwargs):
        frame = caller()
        if frame:
            owner = Path(frame.f_globals['__file__'])
            sources = set(frames.get(frame, {}).get('sources', ()))
            if owner in WRITERS and path == MUTABLE[owner]:
                assert prepared and frame in frames, 'structure writer read before producing plan'
                return shadow[path] if binary else shadow[path].decode()
            if owner == INVENTORY:
                assert sources == set(REFERENCE[owner]) and path in sources, 'inventory read before fresh complete plan'
                direct = sys._getframe(2)
                if direct.f_code.co_filename == str(INVENTORY):
                    assert generations[frame] > inventory_checkpoint[0], 'inventory direct read recheck omitted'
                    inventory_checkpoint[0] = generations[frame]
            elif owner == STRUCTURE and frame.f_code.co_name == 'load_reference':
                assert sources == set(REFERENCE[owner]) and path in sources, 'structure read before fresh reference plan'
            elif path.is_relative_to(WORK) and '/src/' in str(path):
                base = WORK / path.relative_to(WORK).parts[0] / 'src'
                copied = {base / p.relative_to(BOOK) for p in BOOK_FILES}
                assert any(copied <= set(p.get('sources', ())) and copied <= set(p.get('output_files', ())) for p in plans), 'structure copied source plan omitted'
        return originals[2 if binary else 1](path, *args, **kwargs)

    def unexpected(*args, **kwargs):
        raise AssertionError('structure unexpected pure effect')

    def load(file, *args, **kwargs):
        path = Path(file); frame = caller()
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, prepare_producer=prepare, enter_producer=unexpected)
        if frame and Path(frame.f_globals['__file__']) == INVENTORY and path == INPUT:
            assert set(frames.get(frame, {}).get('sources', ())) == set(BOOT[INVENTORY]), 'inventory bootstrap omitted'
        if frame and Path(frame.f_globals['__file__']) == STRUCTURE and path == SIGNATURE:
            assert set(frames.get(frame, {}).get('sources', ())) == set(REFERENCE[STRUCTURE]), 'structure delegation before fresh reference plan'
        if path in scripts:
            namespace = dict(__file__=str(path), __name__='borrowed_structure')
            exec(compile(scripts[path], str(path), 'exec'), namespace)
            return namespace
        runpy.run_path = originals[0]
        try:
            namespace = originals[0](file, *args, **kwargs)
        finally:
            runpy.run_path = load
        if path == INPUT:
            # The inventory deliberately adopts its provider's exported profile. Keep
            # that public identity instrumented; do not alter the borrowed provider body.
            namespace['PROFILE'] = dict(GOOD, plan_producer=plan, prepare_producer=prepare, enter_producer=unexpected)
        if path in (INPUT, SIGNATURE):
            name = 'load_context' if path == INPUT else 'load_reference'
            provider = namespace[name]
            def borrowed(*args, **kwargs):
                parent = caller(); owner = Path(parent.f_globals['__file__'])
                assert set(frames.get(parent, {}).get('sources', ())) == set(REFERENCE[owner]), 'structure borrowed delegation plan omitted'
                if owner == INVENTORY:
                    assert generations[parent] > inventory_checkpoint[0], 'inventory delegation recheck omitted'
                    inventory_checkpoint[0] = generations[parent]
                if mode == 'fault': raise RuntimeError('structure injected reference failure')
                return provider(*args, **kwargs)
            namespace[name] = borrowed
        return namespace

    def authorized(path):
        assert any(path in p.get('output_files', ()) for p in prepared), 'structure output before producing plan'
        assert path in FILES | set().union(*WRITER_FILES.values()), 'structure unowned output'
        assert all(os.environ.get(k) == v for k, v in custom.items()), 'structure stores lost'

    def fresh_writer(frame, wanted):
        assert frame in frames and plans[-1] == frames[frame] and len(plans) > writer_checkpoint[0], wanted
        writer_checkpoint[0] = len(plans)

    def write(path, data, binary, *args, **kwargs):
        frame = caller()
        if path in shadow:
            fresh_writer(frame, 'structure writer write recheck omitted')
            assert path in frames[frame].get('mutable_sources', ()), 'structure unowned source write'
            if binary:
                assert set(frames[frame].get('sources', ())) == {path}, 'structure restore depends on unrelated inputs'
            shadow[path] = data if binary else data.encode()
        else:
            authorized(path)
            if path in set().union(*WRITER_FILES.values()): fresh_writer(frame, 'structure writer log recheck omitted')
            originals[4 if binary else 3](path, data, *args, **kwargs)
            if path.name == 'examples.md': checkpoint[0] = len(plans)
        written.append(path)
        return len(data)

    def mkdir(path, *args, **kwargs):
        assert prepared, 'structure mkdir before producing plan'
        path = Path(path)
        assert path == WORK or any(path.is_relative_to(WORK / label) for label in LABELS) or path in WRITER_WORK.values(), 'structure unowned directory'
        return originals[6](path, *args, **kwargs)

    def path_mkdir(path, *args, **kwargs):
        assert prepared, 'structure mkdir before producing plan'
        return originals[5](path, *args, **kwargs)

    def remove(path, *args, **kwargs):
        path = Path(path)
        expected = {path / p.relative_to(BOOK) for p in BOOK_FILES} | {WORK / (path.parent.name + '.log')}
        assert set(plans[-1].get('output_files', ())) == expected, 'structure delete recheck omitted'
        assert prepared, 'structure delete before producing plan'
        removed.append(path)
        return originals[8](path, *args, **kwargs)

    def copyfile(source, destination):
        source, destination = Path(source), Path(destination)
        assert source in BOOK_FILES, 'structure unplanned copy input'; authorized(destination)
        GOOD['plan_producer'](ROOT, sources=(source,), output_files=(destination,))
        written.append(destination)
        return shutil.copy2(source, destination)

    def copytree(source, destination, *args, **kwargs):
        source, destination = Path(source), Path(destination)
        if source == BOOK:
            assert not args and not kwargs, 'structure copy interface changed'
            copies.append(destination)
            return originals[9](source, destination, copy_function=copyfile)
        bound = inspect.signature(originals[9]).bind(source, destination, *args, **kwargs); bound.apply_defaults()
        assert bound.arguments['copy_function'] is copyfile and bound.arguments['ignore'] is None, 'structure recursive copier changed'
        assert not any(bound.arguments[n] for n in ('symlinks', 'ignore_dangling_symlinks', 'dirs_exist_ok')), 'structure copy options changed'
        return originals[9](source, destination, *args, **kwargs)

    def child(arguments, *args, **kwargs):
        assert not args and kwargs.get('cwd') == ROOT and kwargs.get('capture_output') is True, 'structure child interface changed'
        environment = kwargs.get('env', {})
        assert all(environment.get(k) == v for k, v in custom.items()) and environment.get('MAKE_TMPDIR') == custom['TMPDIR'], 'structure child stores lost'
        assert environment.get('STRUCTURE_UNRELATED') == 'preserved', 'structure unrelated child environment lost'
        if entry in WRITERS:
            expected = [sys.executable, '-I', '-B', STRUCTURE.relative_to(ROOT).as_posix(), '--contracts-only'] if entry == WRITERS[0] else [sys.executable, '-I', '-B', str(INVENTORY)]
            assert arguments == expected, 'structure writer child argv changed'
            fresh_writer(caller(), 'structure writer child recheck omitted')
            assert shadow[MUTABLE[entry]] != FROZEN[MUTABLE[entry]], 'structure source mutation missing'
            if mode == 'writer-fault': raise RuntimeError('structure injected writer failure')
            result = types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'formula_structure.py AssertionError assert actual == expected\n')
        else:
            assert arguments == ['bash', str(SOURCE)] and kwargs.get('text') is True, 'structure shell child changed'
            base = WORK / LABELS[len(children)] / 'src'
            assert environment.get('FORMULA_BOOK') == str(base), 'structure copied selector lost'
            assert all(environment.get(k) == v for k, v in (selectors or {}).items()), 'structure caller selectors lost'
            assert any(set((*BOOT[STRUCTURE], *BOOK_FILES)) <= set(p.get('sources', ())) for p in plans[checkpoint[0]:]), 'structure child source recheck omitted'
            parts = Path(environment.get('FORMULA_PARTS_DIR') or base / 'spec/formula-language')
            selected = [Path(environment.get(k) or default) for k, default in (
                ('FORMULA_CONTRACT', base / 'spec/formula-language.md'), ('FORMULA_GRAMMAR', parts / 'grammar.md'),
                ('FORMULA_EXAMPLES', parts / 'examples.md'), ('FORMULA_FIXTURE', base / 'spec/reference-skirt.md'))]
            selected = [p if p.is_absolute() else ROOT / p for p in selected]; parts = parts if parts.is_absolute() else ROOT / parts
            import re
            references = [Path(os.path.normpath(str(path.parent) + '/' + target.split('#')[0]))
                          for path in selected[:3] for unused, target in re.findall(r'\[([^\]]*)\]\(([^)]+)\)', originals[1](path))
                          if not target.startswith(('http://', 'https://', '#'))]
            assert set(plans[-1].get('sources', ())) == set((*selected, *sorted(parts.glob('*.md')), *references)), 'structure child selector recheck omitted'
            result = types.SimpleNamespace(returncode=1, stdout=SIGNATURES[len(children)]+'\n', stderr='')
        children.append(arguments)
        return result

    namespace = dict(__file__=str(entry), __name__='__main__' if mode in ('cli', 'contracts-cli') else 'borrowed_structure')
    expected_environment = dict(saved, **custom, MAKE_TMPDIR=custom['TMPDIR'], RUSTUP_AUTO_INSTALL='0',
                                STRUCTURE_UNRELATED='preserved', RUSTUP_TOOLCHAIN='1.99.0')
    for name in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_PARTS_DIR', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR', 'FORMULA_EXAMPLES', 'FORMULA_FIXTURE'):
        expected_environment.pop(name, None)
    expected_environment.update(selectors or {})
    try:
        os.environ.clear(); os.environ.update(expected_environment)
        sys.argv = [str(entry), *(['--contracts-only'] if mode == 'contracts-cli' else [])]; current_argv = sys.argv
        runpy.run_path = load
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.write_text = lambda path, data, *args, **kwargs: write(path, data, False, *args, **kwargs)
        Path.write_bytes = lambda path, data, *args, **kwargs: write(path, data, True, *args, **kwargs)
        Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree = path_mkdir, mkdir, child, remove, copytree
        with contextlib.redirect_stdout(output):
            try:
                exec(compile(scripts[entry], str(entry), 'exec'), namespace)
                if entry == STRUCTURE and mode not in ('cli', 'contracts-cli', 'import'):
                    for unused in range(2 if mode in ('load', 'contracts') else 1):
                        namespace['load_reference' if mode in ('load', 'fault') else 'contracts' if mode == 'contracts' else 'end_to_end']()
                elif entry in WRITERS and mode not in ('cli', 'import'): namespace['main']()
            except RuntimeError as error:
                assert (mode, str(error)) in (('fault', 'structure injected reference failure'), ('writer-fault', 'structure injected writer failure')), 'structure unrelated execution failure'
        assert sys.argv is current_argv and dict(os.environ) == expected_environment, 'structure borrowed normal/exception state changed'
        if entry in WRITERS and mode != 'import':
            assert all(shadow[path] == FROZEN[path] for path in shadow), 'structure modeled source restoration failed'
            assert (len(children), written.count(MUTABLE[entry])) == ((0, 2) if mode == 'writer-fault' else (4, 9)), 'structure writer population changed'
        elif entry == STRUCTURE and mode in ('cli', 'end-to-end'):
            assert len(children) == len(copies) == len(removed) == 2 and set(written) == FILES and len(written) == 2*(len(BOOK_FILES)+2), 'structure full copied dispatch coverage changed'
        else:
            assert not prepared and not written and not children, 'structure pure call produced effects'
        if entry == STRUCTURE: assert any(set(p.get('sources', ())) == set(BOOT[entry]) for p in plans), 'structure bootstrap omitted'
    finally:
        (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
         Path.mkdir, os.mkdir, subprocess.run, shutil.rmtree, shutil.copytree) = originals
        sys.argv = argv; os.environ.clear(); os.environ.update(saved)
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'structure tracked inputs changed'
    print('actual structural capture:', entry.name, mode, len(plans), 'plans;', len(written), 'writes;', len(children), 'intercepted children; numerical verdict separate')


def original_bodies():
    class Guards(ast.NodeTransformer):
        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and ast.unparse(node.value.func) == "PROFILE['plan_producer']": return None
            return self.generic_visit(node)
        def visit_Call(self, node):
            if ast.unparse(node.func) == 'subprocess.run': node.keywords = [kw for kw in node.keywords if kw.arg != 'env']
            return self.generic_visit(node)
    old = {entry: subprocess.check_output(['git', 'show', BEFORE+':'+entry.relative_to(ROOT).as_posix()], text=True) for entry in ENTRIES}
    pick = lambda tree, name: next(n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == name)
    previous, current = ast.parse(old[STRUCTURE]), Guards().visit(ast.parse(TEXTS[STRUCTURE]))
    for name in ('load_reference', 'conditional', 'node_expression', 'contracts'):
        assert ast.dump(pick(previous, name)) == ast.dump(pick(current, name)), ('structure original body changed', name)
    before, after = pick(previous, 'end_to_end'), pick(current, 'end_to_end')
    assert [ast.dump(n) for n in before.body[1:3]] == [ast.dump(n) for n in after.body[2:4]], 'structure two authored refusal cases changed'
    old_loop, new_loop = Guards().visit(before.body[3]), after.body[4]
    relevant = [n for n in new_loop.body[1:] if not (isinstance(n, ast.Expr) and isinstance(n.value, ast.Call) and ast.unparse(n.value.func) in ('source_plan','consumer_inputs'))]
    old_relevant = [n for n in old_loop.body[3:] if not (isinstance(n, ast.Assign) and any(isinstance(t,ast.Name) and t.id=='environment' for t in n.targets))]
    assert [ast.dump(n) for n in old_relevant] == [ast.dump(n) for n in relevant], 'structure original edit/child/assertion body changed'
    previous = Guards().visit(ast.parse(old[INVENTORY])); current = Guards().visit(ast.parse(TEXTS[INVENTORY]))
    body = next(n for n in current.body if isinstance(n,ast.Try)).body[4:]
    assert [ast.dump(n) for n in previous.body[7:]] == [ast.dump(n) for n in body], 'inventory original cases/renderer/assertions changed'
    for writer in WRITERS:
        previous = Guards().visit(ast.parse(old[writer])); current = Guards().visit(ast.parse(TEXTS[writer]))
        cases = lambda tree: next(n for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='CASES' for t in n.targets))
        assert ast.dump(cases(previous)) == ast.dump(cases(current)), 'structure original four mutation cases changed'
        before = [n for n in previous.body if isinstance(n,(ast.Try,ast.Assert)) or isinstance(n,ast.Expr) and isinstance(n.value,ast.Call) and ast.unparse(n.value.func)=='print']
        after = pick(current,'main').body[4 if writer == WRITERS[1] else 5:]
        assert [ast.dump(n) for n in before] == [ast.dump(n) for n in after], 'structure original writer assertions/restoration changed'
    return old



def fault_controls():
    previous = original_bodies()
    for entry, mode, wanted in ((STRUCTURE,'import','structure bootstrap omitted'),
                               (INVENTORY,'cli','inventory bootstrap omitted'),
                               (WRITERS[0],'cli','structure mkdir before producing plan'),
                               (WRITERS[1],'cli','structure writer read before producing plan')):
        try: capture(entry,mode,{entry:previous[entry]})
        except AssertionError as error:
            assert str(error)==wanted, ('structure unrelated original gap',entry.name,str(error))
        else: raise AssertionError(('structure original gap escaped',entry.name))
    faults = [
        (STRUCTURE,'import',"PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)",'None','structure bootstrap omitted'),
        (STRUCTURE,'load',"PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)",'None','structure delegation before fresh reference plan'),
        (INVENTORY,'cli',"PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)",'None','inventory bootstrap omitted'),
        (STRUCTURE,'end-to-end','output_files=outputs, sources=sources)','output_files=outputs[:-1], sources=sources)','structure file plan incomplete'),
        (STRUCTURE,'end-to-end','directories=directories, output_files=outputs','directories=directories[:-1], output_files=outputs','structure directory plan incomplete'),
        (STRUCTURE,'end-to-end','output_files=outputs, sources=sources)','output_files=outputs, sources=sources[:-1])','structure producing inputs incomplete'),
        (STRUCTURE,'end-to-end',"return PROFILE['prepare_producer'](ROOT, **plan) if activate else None", "return PROFILE['environment_for'](ROOT, os.environ, create=False) if activate else None",'structure mkdir before producing plan'),
        (STRUCTURE,'end-to-end','fixture_plan((label,), activate=False)','None','structure delete recheck omitted'),
        (STRUCTURE,'end-to-end',"PROFILE['plan_producer'](ROOT, sources=(TOOL, *files), output_files=files)",'None','structure copied source plan omitted'),
        (STRUCTURE,'end-to-end','        source_plan()\n        consumer_inputs','        consumer_inputs','structure child source recheck omitted'),
        (STRUCTURE,'end-to-end','        consumer_inputs(dict(environment, FORMULA_BOOK=str(book)))\n','','structure child selector recheck omitted'),
        (STRUCTURE,'end-to-end','env=dict(environment, FORMULA_BOOK=str(book))',"env=dict(environment, FORMULA_BOOK=str(book), TMPDIR=str(ROOT/'target/wrong-scratch'))",'structure child stores lost'),
    ]
    for writer in WRITERS:
        faults.extend((writer,'cli',anchor,replacement,wanted) for anchor,replacement,wanted in (
            ("    PROFILE['plan_producer'](ROOT, **plan)\n    environment",'    environment','structure activation before pure plan'),
            ('env=environment','env={}','structure child stores lost'),
            ("            PROFILE['plan_producer'](ROOT, **plan)\n            SOURCE.write_text",'            SOURCE.write_text','structure writer write recheck omitted'),
            ("            PROFILE['plan_producer'](ROOT, **plan)\n            result",'            result','structure writer child recheck omitted'),
            ("            PROFILE['plan_producer'](ROOT, **plan)\n            (WORK",'            (WORK','structure writer log recheck omitted'),
            ("            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))\n            SOURCE.write_bytes",'            SOURCE.write_bytes','structure writer write recheck omitted'),
            ("        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))\n        SOURCE.write_bytes",'        SOURCE.write_bytes','structure writer write recheck omitted')))
    for entry,mode,anchor,replacement,wanted in faults:
        assert TEXTS[entry].count(anchor)==1,('structure guard anchor changed',entry.name,anchor)
        try: capture(entry,mode,{entry:TEXTS[entry].replace(anchor,replacement,1)})
        except AssertionError as error:
            assert str(error)==wanted,('structure unrelated guard red',entry.name,anchor,str(error))
        else: raise AssertionError(('structure actual guard omission escaped',entry.name,anchor))
    # Each of the inventory's four reference plans has a distinct fresh boundary.
    anchor="PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)"
    assert TEXTS[INVENTORY].count(anchor)==4
    for index in range(4):
        parts=TEXTS[INVENTORY].split(anchor)
        disabled=anchor.join(parts[:index+1])+'None'+anchor.join(parts[index+1:])
        try: capture(INVENTORY,'cli',{INVENTORY:disabled})
        except AssertionError as error:
            wanted='structure borrowed delegation plan omitted' if index==0 else 'inventory direct read recheck omitted'
            assert str(error)==wanted,('inventory unrelated fresh-guard red',index,str(error))
        else: raise AssertionError(('inventory stale source plan admitted',index))
    print('structure guard controls:',len(faults)+4,'actual guard reds/four original gaps; original bodies and mutation populations exact')


def physical_controls():
    """Refuse real filesystem faults in owned isolated copies before any producer effect."""
    cases = ('missing input', 'symlink input', 'symlink book directory', 'structure is directory',
             'unexpected old fixture', 'symlink output', 'hardlinked output',
             'output directory is file', 'symlink log', 'outside selector',
             'structure output alias', 'parts directory alias', 'symlink store', 'invalid toolchain', 'pure unused sidebar')
    def effect(*args, **kwargs):
        raise AssertionError('structure physical effect before refusal')
    generation = ROOT / 'target/fixture-structure-controls' / str(os.getpid())
    GOOD['directory'](ROOT, generation.parent.relative_to(ROOT).as_posix(), create=True)
    assert not generation.exists(), 'structure fixture generation already exists'
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
        leaf = work / 'call_nodes/src/spec/formula-language.md'
        log = work / 'call_nodes.log'
        sidebar = sandbox / 'docs/book/src/annexes/formula-static-validation.md'
        input_file = sandbox / 'docs/book/src/spec/units-and-tolerances.md'
        extra = work / 'call_nodes/src/unexpected.txt'
        scratch = sandbox / 'target/scratch'
        candidates = (leaf, log, extra, scratch)
        parents = {path.parent for path in candidates}
        GOOD['prepare_producer'](ROOT, directories=tuple(parents), output_files=candidates)
        selectors = {}
        expected = None
        if case == 'missing input':
            input_file.unlink(); expected = 'producer source is not a local regular file'
        elif case == 'symlink input':
            input_file.unlink(); input_file.symlink_to(mirrors[BOOT[STRUCTURE][0]])
            expected = 'symlink copied-book entry'
        elif case == 'symlink book directory':
            unused = sandbox / 'docs/book/src/unused-directory'
            GOOD['plan_producer'](ROOT, output_files=(unused,))
            unused.symlink_to(sandbox / 'docs/book/src/annexes', target_is_directory=True)
            expected = 'symlink copied-book entry'
        elif case == 'structure is directory':
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
            base = work / 'call_nodes/src'
            blocker = base / 'annexes'
            GOOD['plan_producer'](ROOT, output_files=(blocker,))
            blocker.write_text('owned directory refusal sentinel\n')
            expected = 'unexpected copied-book fixture entry'
        elif case == 'symlink log':
            log.symlink_to(input_file); expected = 'symlink producer output'
        elif case == 'outside selector':
            selectors['FORMULA_CONTRACT'] = str(STRUCTURE); expected = 'producer path outside workspace'
        elif case == 'structure output alias':
            leaf.write_text('owned selected-input alias sentinel\n')
            selectors['FORMULA_CONTRACT'] = str(leaf)
            expected = 'copied-book output aliases selected input'
        elif case == 'parts directory alias':
            parts = work / 'call_nodes/src/spec/formula-language'
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
            namespace = {'__file__': str(mirrors[STRUCTURE]), '__name__': 'borrowed_structure'}
            with contextlib.redirect_stderr(stderr):
                exec(compile(TEXTS[STRUCTURE], str(mirrors[STRUCTURE]), 'exec'), namespace)
                if expected is None:
                    assert namespace['contracts']() is None
                else:
                    try:
                        namespace['end_to_end']()
                    except namespace['PROFILE']['Refusal'] as error:
                        assert str(error) == expected, (case, 'unrelated physical refusal', str(error))
                        if case == 'parts directory alias':
                            guard = 'not any(parts == base or parts.is_relative_to(base) for base in bases)'
                            assert TEXTS[STRUCTURE].count(guard) == 1
                            disabled = dict(__file__=str(mirrors[STRUCTURE]), __name__='directory_guard_disabled')
                            exec(compile(TEXTS[STRUCTURE].replace(guard, 'True', 1), str(mirrors[STRUCTURE]), 'exec'), disabled)
                            disabled['fixture_plan'](LABELS, activate=False)
                            assert not tuple(parts.iterdir()), 'D202 counterfactual changed directory'
                            print('Inherited D202 actual directory-alias guard omission admits same empty selected directory; no effects')

                    else:
                        raise AssertionError((case, 'structure physical refusal escaped'))
        except SystemExit as error:
            assert error.code == 2 and expected and expected in stderr.getvalue(), (case, 'unrelated bootstrap refusal', stderr.getvalue())
        finally:
            os.mkdir, shutil.rmtree, shutil.copytree, subprocess.run, Path.write_text, Path.write_bytes, Path.mkdir = originals
            os.environ.clear(); os.environ.update(saved)
        print('structure physical control:', case)
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
        namespace = {'__file__': str(sandbox / STRUCTURE.relative_to(ROOT)), '__name__': 'disabled_structure'}
        exec(compile(TEXTS[STRUCTURE], namespace['__file__'], 'exec'), namespace)
        namespace['fixture_plan'](LABELS, activate=False)
        assert (sandbox / 'target/formula_structure/call_nodes/src/spec/formula-language.md').stat().st_nlink == 2
    finally:
        profile.write_text(original)
        os.environ.clear(); os.environ.update(saved)
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'structure physical controls changed tracked sources'
    print('structure guard counterfactual: actual profile planner disabled admits same hardlinked output; tracked sources exact')



def borrowed_physical_controls():
    """Exercise actual CLI refusals and repeat the borrowed reference boundary."""
    generation = ROOT / 'target/structure-boundary-controls' / str(os.getpid())
    total = borrowed = 0
    cases = ('missing', 'symlink', 'directory', 'nested-repo', 'symlink-store', 'invalid-toolchain')
    selected = [(entry, case) for entry in ENTRIES for case in cases]
    selected.extend((writer, 'hardlinked-source') for writer in WRITERS)
    for index, (entry, case) in enumerate(selected):
        sandbox = generation / str(index)
        assert not sandbox.exists(), 'structure boundary fixture already exists'
        inputs = REFERENCE.get(entry, WRITER_INPUTS.get(entry))
        mirrors = {path: sandbox / path.relative_to(ROOT) for path in inputs}
        dirs = {sandbox}
        for path in mirrors.values():
            dirs.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
        GOOD['prepare_producer'](ROOT, sources=inputs, directories=tuple(dirs),
                                 output_files=tuple(mirrors.values()))
        for source, destination in mirrors.items():
            shutil.copy2(source, destination)
        victim = mirrors[MUTABLE[entry] if entry in WRITERS else REFERENCE[entry][-1]]
        wanted = 'producer source is not a local regular file'
        extra = {}
        if case == 'missing':
            victim.unlink()
        elif case == 'symlink':
            victim.unlink(); victim.symlink_to(mirrors[PROFILE_SOURCE])
            wanted = 'symlink producer source'
        elif case == 'directory':
            victim.unlink(); victim.mkdir()
        elif case == 'nested-repo':
            (victim.parent / '.git').mkdir(); wanted = 'store crosses another Git repository'
        elif case == 'symlink-store':
            scratch = sandbox / 'target/scratch'
            GOOD['directory'](ROOT, scratch.parent.relative_to(ROOT).as_posix(), create=True)
            scratch.symlink_to(victim.parent, target_is_directory=True)
            wanted = 'symlink store component'
        elif case == 'hardlinked-source':
            alias = sandbox / 'target/source-alias'
            GOOD['prepare_producer'](ROOT, directories=(alias.parent,), output_files=(alias,))
            os.link(victim, alias); wanted = 'hard-linked mutable producer source'
        else:
            extra['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'
            wanted = 'toolchain override must be a name'
        saved, argv = dict(os.environ), sys.argv
        originals = (os.mkdir, Path.write_text, Path.write_bytes, subprocess.run, Path.mkdir)
        def effect(*args, **kwargs):
            raise AssertionError('structure boundary refusal produced an effect')
        stderr = io.StringIO()
        try:
            for key in ('FORMULA_ROOT', 'FORMULA_BOOK', 'FORMULA_CONTRACT', 'FORMULA_GRAMMAR',
                        'FORMULA_EXAMPLES', 'FORMULA_FIXTURE', 'FORMULA_PARTS_DIR', 'RUSTUP_TOOLCHAIN'):
                os.environ.pop(key, None)
            os.environ.update({name: str(sandbox / suffix) for name, suffix in (
                ('CARGO_HOME', 'target/cargo-home'), ('RUSTUP_HOME', 'target/rustup'),
                ('CARGO_TARGET_DIR', 'target/build'), ('TMPDIR', 'target/scratch'))})
            os.environ.update(extra)
            sys.argv = [str(mirrors[entry]), '--contracts-only']
            os.mkdir = Path.write_text = Path.write_bytes = subprocess.run = Path.mkdir = effect
            with contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                try:
                    runpy.run_path(str(mirrors[entry]), run_name='__main__')
                except SystemExit as error:
                    assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', (entry.name, case, stderr.getvalue())
                else:
                    raise AssertionError(('structure boundary fault accepted', entry.name, case))
                if entry == STRUCTURE and case in cases[:4]:
                    namespace = runpy.run_path(str(mirrors[entry]))
                    for unused in range(2):
                        try:
                            namespace['load_reference']()
                        except namespace['PROFILE']['Refusal'] as error:
                            assert str(error) == wanted, 'structure borrowed refusal identity changed'
                        else:
                            raise AssertionError(('structure borrowed refusal escaped', case))
                        borrowed += 1
        finally:
            os.mkdir, Path.write_text, Path.write_bytes, subprocess.run, Path.mkdir = originals
            os.environ.clear(); os.environ.update(saved); sys.argv = argv
        total += 1
    print('structure boundaries:', total, 'actual CLI refusals;', borrowed,
          'repeated borrowed reference refusals; no writes/activation/child dispatch')


def late_restore_capture(writer, text):
    """A real mirrored mutation must survive an unrelated late log refusal (D207)."""
    sandbox = ROOT / 'target/structure-late-restore-controls' / str(os.getpid()) / str(late_restore_capture.calls)
    late_restore_capture.calls += 1
    assert not sandbox.exists(), 'structure late restoration fixture already exists'
    inputs = WRITER_INPUTS[writer]
    mirrors = {path: sandbox / path.relative_to(ROOT) for path in inputs}
    dirs = {sandbox}
    for path in mirrors.values():
        dirs.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
    GOOD['prepare_producer'](ROOT, sources=inputs, directories=tuple(dirs), output_files=tuple(mirrors.values()))
    for path, destination in mirrors.items():
        shutil.copy2(path, destination)
    source = MUTABLE[writer]
    original = FROZEN[source]
    saved, argv = dict(os.environ), sys.argv
    real_run = subprocess.run
    stderr = io.StringIO()
    log = sandbox / sorted(WRITER_FILES[writer])[0].relative_to(ROOT)

    def child(arguments, **kwargs):
        assert arguments[:3] == [sys.executable, '-I', '-B'], 'structure late child interface changed'
        assert mirrors[source].read_bytes() != original, 'structure late actual source was not mutated'
        GOOD['plan_producer'](ROOT, directories=(log,))
        log.mkdir()
        return types.SimpleNamespace(returncode=1, stdout=b'', stderr=b'formula_structure.py AssertionError assert actual == expected\n')

    try:
        for name, suffix in GOOD['STORES'].items():
            os.environ[name] = str(sandbox / suffix)
        os.environ.pop('RUSTUP_TOOLCHAIN', None)
        sys.argv = [str(mirrors[writer])]
        subprocess.run = child
        with contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
            try:
                exec(compile(text, str(mirrors[writer]), 'exec'),
                     dict(__file__=str(mirrors[writer]), __name__='__main__'))
            except SystemExit as error:
                assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — producer output is not a local regular file\n', ('structure unrelated late refusal', stderr.getvalue())
            else:
                raise AssertionError('structure late log refusal escaped')
        assert mirrors[source].read_bytes() == original, 'structure actual late source not restored'
    finally:
        subprocess.run = real_run; sys.argv = argv
        os.environ.clear(); os.environ.update(saved)
        GOOD['plan_producer'](ROOT, sources=(mirrors[source],), mutable_sources=(mirrors[source],))
        mirrors[source].write_bytes(original)
        assert mirrors[source].read_bytes() == original and source.read_bytes() == original


late_restore_capture.calls = 0


def late_restore_controls():
    guard = "PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))"
    for writer in WRITERS:
        assert TEXTS[writer].count(guard) == 2, 'structure restoration guards changed'
        broken = TEXTS[writer].replace(guard, "PROFILE['plan_producer'](ROOT, **plan)")
        try:
            late_restore_capture(writer, broken)
        except AssertionError as error:
            assert str(error) == 'structure actual late source not restored', ('structure unrelated restore red', str(error))
        else:
            raise AssertionError('structure restored-output coupling escaped')
        late_restore_capture(writer, TEXTS[writer])
    print('D207 both actual source restoration assertions red/green; late-log CLI rc=2; mirrors and tracked sources exact')


def main():
    outputs = tuple(WORK / label / 'src' / p.relative_to(BOOK) for label in LABELS for p in BOOK_FILES)
    dirs = tuple(WORK / label / 'src' / folder.relative_to(BOOK) for label in LABELS for folder in directories)
    GOOD['prepare_producer'](ROOT, sources=tuple(WHOLE), directories=dirs, output_files=outputs)
    for label in LABELS:
        for source in BOOK_FILES: shutil.copy2(source, WORK / label / 'src' / source.relative_to(BOOK))
    for mode in ('import','load','contracts','contracts-cli','fault','end-to-end','cli'): capture(STRUCTURE,mode)
    capture(INVENTORY,'cli')
    for writer in WRITERS:
        for mode in ('import','cli','borrowed','writer-fault'): capture(writer,mode)
    selectors = {name: str(file.relative_to(ROOT)) for name, file in (
        ('FORMULA_CONTRACT', BOOK / 'spec/formula-language.md'),
        ('FORMULA_GRAMMAR', BOOK / 'spec/formula-language/grammar.md'),
        ('FORMULA_EXAMPLES', BOOK / 'spec/formula-language/examples.md'),
        ('FORMULA_FIXTURE', BOOK / 'spec/reference-skirt.md'),
        ('FORMULA_PARTS_DIR', BOOK / 'spec/formula-language'))}
    selectors['FORMULA_ROOT'] = 'unused-root-preserved'
    capture(STRUCTURE, 'end-to-end', selectors=selectors)
    fault_controls()
    physical_controls()
    borrowed_physical_controls()
    late_restore_controls()
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'structure final sources changed'
    print('structural controls: actual pure/copy/source-writer boundaries and guard counterfactuals; real grammar verdicts separate')


if __name__ == '__main__':
    main()
