"""Actual pure payload-library plans, preserved cases and isolated public refusals."""
from pathlib import Path
import ast
import contextlib
import io
import os
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
ENTRIES = tuple(HERE / (name + '_payload_contract.py') for name in ('dimension', 'ambiguity'))
SIGNATURE = HERE / 'static_signature_contract.py'
NAMESPACE = HERE / 'static_namespace_contract.py'
ALL_ENTRIES = (*ENTRIES, SIGNATURE, NAMESPACE)
BOOK_INPUTS = tuple(ROOT / 'docs/book/src' / name for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))
BOOT = {SIGNATURE: (SIGNATURE, PROFILE, SOURCE),
        NAMESPACE: (NAMESPACE, PROFILE, SOURCE, SIGNATURE),
        ENTRIES[0]: (ENTRIES[0], PROFILE, SOURCE, SIGNATURE),
        ENTRIES[1]: (ENTRIES[1], PROFILE, SOURCE, NAMESPACE, SIGNATURE)}
INPUTS = {SIGNATURE: (*BOOT[SIGNATURE], *BOOK_INPUTS)}
INPUTS[NAMESPACE] = (NAMESPACE, *INPUTS[SIGNATURE])
INPUTS[ENTRIES[0]] = (ENTRIES[0], *INPUTS[SIGNATURE])
INPUTS[ENTRIES[1]] = (ENTRIES[1], *INPUTS[NAMESPACE])
GOOD = runpy.run_path(str(PROFILE))
GOOD['plan_producer'](ROOT, sources=(Path(__file__), PROFILE, SOURCE, *ALL_ENTRIES, *BOOK_INPUTS))
FROZEN = {path: path.read_bytes() for path in (PROFILE, SOURCE, *ALL_ENTRIES, *BOOK_INPUTS)}
TEXTS = {path: FROZEN[path].decode() for path in ALL_ENTRIES}
BEFORE = 'ba8ae6ece39e8a64d80005211ab1a54b2955ce5a'


def capture(entry, mode, texts=None):
    """Run actual bodies with per-call plans; no output, child or store activation permitted."""
    scripts = dict(TEXTS)
    scripts.update(texts or {})
    saved, argv = dict(os.environ), sys.argv
    custom = {name: str(ROOT / 'target/payload-capture-stores' / suffix) for name, suffix in (
        ('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in custom.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    expected_environment = dict(saved, **custom, MAKE_TMPDIR=custom['TMPDIR'], RUSTUP_AUTO_INSTALL='0',
                                PAYLOAD_UNRELATED='preserved', RUSTUP_TOOLCHAIN='1.99.0')
    original = (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text,
                Path.write_bytes, os.mkdir, subprocess.run)
    plans, reads, frames = [], [], {}
    output = io.StringIO()

    def effect(*args, **kwargs):
        raise AssertionError('payload unexpected effect or activation')

    def caller():
        frame = sys._getframe(2)
        while frame and Path(frame.f_globals.get('__file__', '')) not in ALL_ENTRIES:
            frame = frame.f_back
        assert frame is not None, 'payload public caller missing'
        return frame, Path(frame.f_globals['__file__'])

    def plan(root, *, directories=(), output_files=(), sources=(), mutable_sources=()):
        frame, owner = caller()
        assert root == ROOT and not (directories or output_files or mutable_sources), 'payload pure plan has effects'
        expected = (BOOT[owner], INPUTS[owner]) if frame.f_code.co_name == '<module>' else (INPUTS[owner],)
        assert sources in expected, 'payload source plan incomplete'
        before = dict(os.environ)
        result = GOOD['plan_producer'](root, sources=sources)
        assert dict(os.environ) == before, 'payload pure plan changed stores'
        frames[frame] = sources
        plans.append((owner, sources))
        return result

    def read(path, binary, *args, **kwargs):
        frame, owner = caller()
        assert frames.get(frame) == INPUTS[owner], 'payload reference read before complete fresh plan'
        assert path in INPUTS[owner] and path in FROZEN, 'payload undeclared input read'
        reads.append(path)
        return original[2 if binary else 1](path, *args, **kwargs)

    def load(file, *args, **kwargs):
        path = Path(file)
        assert not args and not kwargs, 'payload helper interface changed'
        if path == PROFILE:
            return dict(GOOD, plan_producer=plan, prepare_producer=effect, enter_producer=effect)
        frame, owner = caller()
        assert path in BOOT[owner] and frames.get(frame) == BOOT[owner], 'payload helper before bootstrap plan'
        namespace = dict(__file__=str(path), __name__='<run_path>')
        exec(compile(scripts[path], str(path), 'exec'), namespace)
        if path == SIGNATURE:
            provider = namespace['load_reference']
            def borrowed(*args, **kwargs):
                frame, owner = caller()
                assert frames.get(frame) == INPUTS[owner], 'payload delegation before complete fresh plan'
                return provider(*args, **kwargs)
            namespace['load_reference'] = borrowed
        return namespace

    namespace = dict(__file__=str(entry), __name__='__main__' if mode.startswith('cli') else 'borrowed_loader')
    try:
        sys.argv = [str(entry), *(['--mutations'] if mode == 'cli-mutations' else [])]
        current_argv = sys.argv
        os.environ.update(expected_environment)
        runpy.run_path = load
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.write_text = Path.write_bytes = os.mkdir = subprocess.run = effect
        with contextlib.redirect_stdout(output):
            exec(compile(scripts[entry], str(entry), 'exec'), namespace)
            if mode == 'contracts':
                for _ in range(2):
                    result = namespace['contracts'](verbose=False)
                    if entry == ENTRIES[0]:
                        groups, refusals = result
                        assert sum(groups.values()) == 4023 and refusals == 3814, 'payload dimension population drift'
                    else:
                        assert result == 4192, 'payload ambiguity population drift'
            elif mode == 'fault-argv':
                try:
                    namespace['contracts'](('import pathlib, re, sys', "raise RuntimeError('payload injected execution failure')"), verbose=False)
                except RuntimeError as error:
                    assert str(error) == 'payload injected execution failure'
                else:
                    raise AssertionError('payload injected failure escaped')
                assert sys.argv is current_argv, 'payload exception lost borrowed argv'
        assert (entry, BOOT[entry]) in plans, 'payload bootstrap plan omitted'
        if mode not in ('import', 'fault-argv'):
            assert set(BOOK_INPUTS) <= set(reads), 'payload actual book reads incomplete'
        assert sys.argv is current_argv, 'payload normal call lost argv identity'
        assert dict(os.environ) == expected_environment, 'payload environment changed'
    finally:
        (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text,
         Path.write_bytes, os.mkdir, subprocess.run) = original
        sys.argv = argv
        os.environ.clear(); os.environ.update(saved)
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'payload tracked inputs changed'
    return output.getvalue(), len(plans), len(reads)


def original_bodies():
    """Prior committed function/case/fault bodies are the independent semantic oracle."""
    class Guards(ast.NodeTransformer):
        def visit_Expr(self, node):
            if isinstance(node.value, ast.Call) and ast.unparse(node.value.func) == "PROFILE['plan_producer']":
                return None
            return self.generic_visit(node)
    previous = {}
    for entry in ENTRIES:
        old = subprocess.check_output(['git', 'show', BEFORE + ':' + entry.relative_to(ROOT).as_posix()]).decode()
        previous[entry] = old
        original, current = ast.parse(old), Guards().visit(ast.parse(TEXTS[entry]))
        for node in original.body:
            if isinstance(node, (ast.FunctionDef, ast.ClassDef)):
                actual = next(n for n in current.body if type(n) is type(node) and n.name == node.name)
                assert ast.dump(actual) == ast.dump(node), ('payload original body changed', entry.name, node.name)
            elif isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id in
                    ('FAULTS', 'UNARY', 'BINARY') for t in node.targets):
                assert any(ast.dump(n) == ast.dump(node) for n in current.body), 'payload authored population changed'
        old_main = next(node for node in original.body if isinstance(node, ast.If)
                        and ast.unparse(node.test) == "__name__ == '__main__'")
        new_main = next(node for node in current.body if isinstance(node, ast.If)
                        and ast.unparse(node.test) == "__name__ == '__main__'")
        assert len(new_main.body) == 1 and isinstance(new_main.body[0], ast.Try), 'payload CLI refusal wrapper changed'
        assert [ast.dump(node) for node in new_main.body[0].body] == [ast.dump(node) for node in old_main.body], 'payload original CLI body changed'
    return previous


def physical_controls():
    """Real missing/link/directory/boundary faults in fresh repository-volume copies."""
    generation = ROOT / 'target/payload-physical' / str(os.getpid())
    cases = ('missing', 'symlink', 'directory', 'nested-repo', 'helper-missing', 'helper-symlink',
             'helper-directory', 'helper-nested-repo', 'symlink-store', 'invalid-toolchain')
    total = 0
    for index, (entry, case) in enumerate((entry, case) for entry in ENTRIES for case in cases):
        sandbox = generation / str(index)
        assert not sandbox.exists(), 'payload physical fixture already exists'
        originals = INPUTS[entry]
        mirrors = {path: sandbox / path.relative_to(ROOT) for path in originals}
        directories = {sandbox}
        for path in mirrors.values():
            directories.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
        GOOD['prepare_producer'](ROOT, sources=originals, directories=tuple(sorted(directories)), output_files=tuple(mirrors.values()))
        for source, destination in mirrors.items():
            shutil.copy2(source, destination)
        victim = mirrors[(SIGNATURE if entry == ENTRIES[0] else NAMESPACE) if case.startswith('helper-') else BOOK_INPUTS[-1]]
        fault = case.removeprefix('helper-')
        wanted = 'producer source is not a local regular file'
        extra = {}
        if fault == 'missing':
            victim.unlink()
        elif fault == 'symlink':
            victim.unlink(); victim.symlink_to(mirrors[SOURCE]); wanted = 'symlink producer source'
        elif fault == 'directory':
            victim.unlink(); victim.mkdir()
        elif fault == 'nested-repo':
            marker = victim.parent / '.git'
            GOOD['directory'](ROOT, marker.parent.relative_to(ROOT).as_posix())
            marker.mkdir(); wanted = 'store crosses another Git repository'
        elif case == 'symlink-store':
            store = sandbox / 'target/scratch'
            GOOD['directory'](ROOT, store.parent.relative_to(ROOT).as_posix(), create=True)
            store.symlink_to(victim.parent, target_is_directory=True); wanted = 'symlink store component'
        else:
            extra['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'; wanted = 'toolchain override must be a name'
        saved, argv = dict(os.environ), sys.argv
        originals_api = (os.mkdir, Path.write_text, Path.write_bytes, subprocess.run)
        stderr = io.StringIO()
        def effect(*args, **kwargs):
            raise AssertionError('payload physical refusal produced effect')
        try:
            os.environ.update({name: str(sandbox / suffix) for name, suffix in (
                ('CARGO_HOME', 'target/cargo-home'), ('RUSTUP_HOME', 'target/rustup'),
                ('CARGO_TARGET_DIR', 'target/build'), ('TMPDIR', 'target/scratch'))})
            os.environ.pop('RUSTUP_TOOLCHAIN', None); os.environ.update(extra)
            sys.argv = [str(mirrors[entry])]
            os.mkdir = Path.write_text = Path.write_bytes = subprocess.run = effect
            with contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                try:
                    runpy.run_path(str(mirrors[entry]), run_name='__main__')
                except SystemExit as error:
                    assert error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', (entry.name, case, stderr.getvalue())
                else:
                    raise AssertionError(('payload physical fault accepted', entry.name, case))
                if case in ('missing', 'symlink', 'directory', 'nested-repo'):
                    namespace = runpy.run_path(str(mirrors[entry]))
                    try:
                        namespace['contracts'](verbose=False)
                    except namespace['PROFILE']['Refusal'] as error:
                        assert str(error) == wanted, 'payload borrowed refusal identity changed'
                    else:
                        raise AssertionError('payload borrowed physical refusal escaped')
        finally:
            os.mkdir, Path.write_text, Path.write_bytes, subprocess.run = originals_api
            sys.argv = argv; os.environ.clear(); os.environ.update(saved)
        if case == 'invalid-toolchain':
            # Remove the actual shared planner body solely in this owned mirror. The same
            # otherwise-valid read-only entry then admits the invalid store/toolchain plan.
            profile = mirrors[PROFILE]
            GOOD['plan_producer'](ROOT, sources=(profile,), mutable_sources=(profile,))
            original_profile = profile.read_text()
            function = next(node for node in ast.parse(original_profile).body
                            if isinstance(node, ast.FunctionDef) and node.name == 'plan_producer')
            lines = original_profile.splitlines(keepends=True)
            disabled = ''.join(lines[:function.body[0].lineno - 1]) + '    return ()\n' + ''.join(lines[function.end_lineno:])
            profile.write_text(disabled)
            saved, argv = dict(os.environ), sys.argv
            originals_api = (os.mkdir, Path.write_text, Path.write_bytes, subprocess.run)
            try:
                os.environ.update({name: str(sandbox / suffix) for name, suffix in (
                    ('CARGO_HOME', 'target/cargo-home'), ('RUSTUP_HOME', 'target/rustup'),
                    ('CARGO_TARGET_DIR', 'target/build'), ('TMPDIR', 'target/scratch'))})
                os.environ.update(RUSTUP_TOOLCHAIN='invalid/toolchain')
                sys.argv = [str(mirrors[entry])]
                os.mkdir = Path.write_text = Path.write_bytes = subprocess.run = effect
                with contextlib.redirect_stdout(io.StringIO()):
                    runpy.run_path(str(mirrors[entry]), run_name='__main__')
            finally:
                os.mkdir, Path.write_text, Path.write_bytes, subprocess.run = originals_api
                os.environ.clear(); os.environ.update(saved); sys.argv = argv
                profile.write_text(original_profile)
            assert profile.read_text() == original_profile
        total += 1
    return total


def main():
    previous = original_bodies()
    captures = reds = 0
    for entry in ENTRIES:
        try:
            capture(entry, 'import', previous)
        except AssertionError as error:
            assert str(error) in ('payload bootstrap plan omitted', 'payload helper before bootstrap plan'), 'payload unrelated original red'
        else:
            raise AssertionError('payload original omission escaped')
        for mode in ('import', 'contracts', 'cli', 'cli-mutations', 'fault-argv'):
            _, plans, reads = capture(entry, mode)
            print('actual payload capture:', entry.name, mode, plans, 'plans', reads, 'reads')
            captures += 1
        script = TEXTS[entry]
        for anchor, modes in (("PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", ('import',)),
                              ("PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)", ('contracts', 'cli'))):
            assert script.count(anchor) == len(modes), 'payload guard population changed'
            for index, mode in enumerate(modes):
                parts = script.split(anchor)
                fault = anchor.join(parts[:index + 1]) + 'None' + anchor.join(parts[index + 1:])
                try:
                    capture(entry, mode, {entry: fault})
                except AssertionError as error:
                    assert str(error) in ('payload bootstrap plan omitted', 'payload helper before bootstrap plan',
                                         'payload reference read before complete fresh plan', 'payload delegation before complete fresh plan'), (entry.name, index, mode, str(error))
                else:
                    raise AssertionError(('payload actual guard omission escaped', entry.name, index, mode))
                reds += 1
    physical = physical_controls()
    print('payload library controls:', captures, 'actual pure captures;', reds, 'actual guard reds;', physical,
          'physical CLI refusals/8 borrowed refusals/2 actual planner-disable admissions; original bodies/populations/source/argv/prepared stores exact; no activation')


if __name__ == '__main__':
    assert sys.argv[1:] == [], 'payload control arguments'
    main()
