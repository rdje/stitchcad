"""Actual read-only loader plans, original bodies and isolated physical refusals."""
from pathlib import Path
import ast
import contextlib
import io
import hashlib
import json
import os
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
ENTRIES = tuple(HERE / ('static_' + name + '_contract.py')
                for name in ('signature', 'namespace', 'review'))
BOOK_INPUTS = tuple(ROOT / 'docs/book/src' / name for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))
BOOT = {ENTRIES[0]: (ENTRIES[0], PROFILE, SOURCE),
        ENTRIES[1]: (ENTRIES[1], PROFILE, SOURCE, ENTRIES[0]),
        ENTRIES[2]: (ENTRIES[2], PROFILE, SOURCE, ENTRIES[1], ENTRIES[0])}
INPUTS = {ENTRIES[0]: (*BOOT[ENTRIES[0]], *BOOK_INPUTS)}
INPUTS[ENTRIES[1]] = (ENTRIES[1], *INPUTS[ENTRIES[0]])
INPUTS[ENTRIES[2]] = (ENTRIES[2], *INPUTS[ENTRIES[1]])
GOOD = runpy.run_path(str(PROFILE))
GOOD['plan_producer'](ROOT, sources=(Path(__file__), PROFILE, SOURCE, *ENTRIES, *BOOK_INPUTS))
FROZEN = {path: path.read_bytes() for path in (PROFILE, SOURCE, *ENTRIES, *BOOK_INPUTS)}
TEXTS = {path: FROZEN[path].decode() for path in ENTRIES}
BEFORE = 'a265306d22dbe6c706d4524a93d534c81f5ad38c'


def capture(entry, mode, texts=None):
    """Run actual bodies with per-call plans; no output, child or store activation permitted."""
    scripts = dict(TEXTS)
    scripts.update(texts or {})
    saved, argv = dict(os.environ), sys.argv
    custom = {name: str(ROOT / 'target/loader-capture-stores' / suffix) for name, suffix in (
        ('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'), ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for path in custom.values():
        GOOD['directory'](ROOT, Path(path).relative_to(ROOT).as_posix(), create=True)
    expected_environment = dict(saved, **custom, MAKE_TMPDIR=custom['TMPDIR'], RUSTUP_AUTO_INSTALL='0',
                                LOADER_UNRELATED='preserved', RUSTUP_TOOLCHAIN='1.99.0')
    original = (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text,
                Path.write_bytes, os.mkdir, subprocess.run)
    plans, reads, frames = [], [], {}
    output = io.StringIO()

    def effect(*args, **kwargs):
        raise AssertionError('loader unexpected effect or activation')

    def caller():
        frame = sys._getframe(2)
        while frame and Path(frame.f_globals.get('__file__', '')) not in ENTRIES:
            frame = frame.f_back
        assert frame is not None, 'loader public caller missing'
        return frame, Path(frame.f_globals['__file__'])

    def plan(root, *, directories=(), output_files=(), sources=(), mutable_sources=()):
        frame, owner = caller()
        assert root == ROOT and not (directories or output_files or mutable_sources), 'loader pure plan has effects'
        expected = (BOOT[owner], INPUTS[owner]) if frame.f_code.co_name == '<module>' else (INPUTS[owner],)
        assert sources in expected, 'loader source plan incomplete'
        before = dict(os.environ)
        result = GOOD['plan_producer'](root, sources=sources)
        assert dict(os.environ) == before, 'loader pure plan changed stores'
        frames[frame] = sources
        plans.append((owner, sources))
        return result

    def read(path, binary, *args, **kwargs):
        frame, owner = caller()
        assert frames.get(frame) == INPUTS[owner], 'loader reference read before complete fresh plan'
        assert path in INPUTS[owner] and path in FROZEN, 'loader undeclared input read'
        reads.append(path)
        return original[2 if binary else 1](path, *args, **kwargs)

    def load(file, *args, **kwargs):
        path = Path(file)
        assert not args and not kwargs, 'loader helper interface changed'
        if path == PROFILE:
            return dict(GOOD, plan_producer=plan, prepare_producer=effect, enter_producer=effect)
        frame, owner = caller()
        assert path in BOOT[owner] and frames.get(frame) == BOOT[owner], 'loader helper before bootstrap plan'
        namespace = dict(__file__=str(path), __name__='<run_path>')
        exec(compile(scripts[path], str(path), 'exec'), namespace)
        if path == ENTRIES[0]:
            provider = namespace['load_reference']
            def borrowed(*args, **kwargs):
                frame, owner = caller()
                assert frames.get(frame) == INPUTS[owner], 'loader delegation before complete fresh plan'
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
                assert namespace['contracts'](verbose=False) == {ENTRIES[0]: 4032, ENTRIES[1]: 1139, ENTRIES[2]: 100}[entry]
            elif mode == 'mutations':
                namespace['mutations']()
            elif mode == 'load':
                namespace['load_reference' if entry == ENTRIES[0] else 'load']()
            elif mode == 'quantum':
                namespace['parameter_quantum_contract']()
            elif mode == 'fault-argv':
                loader = namespace['load_reference' if entry == ENTRIES[0] else 'load']
                current_argv = sys.argv
                try:
                    loader(('import pathlib, re, sys', "raise RuntimeError('loader injected execution failure')"))
                except RuntimeError as error:
                    assert str(error) == 'loader injected execution failure'
                else:
                    raise AssertionError('loader injected failure escaped')
                assert sys.argv is current_argv, 'loader exception lost borrowed argv'
        assert (entry, BOOT[entry]) in plans, 'loader bootstrap plan omitted'
        if mode not in ('import', 'fault-argv'):
            assert set(BOOK_INPUTS) <= set(reads), 'loader actual book reads incomplete'
        assert sys.argv is current_argv, 'loader normal call lost argv identity'
        assert dict(os.environ) == expected_environment, 'loader environment changed'
    finally:
        (runpy.run_path, Path.read_text, Path.read_bytes, Path.write_text,
         Path.write_bytes, os.mkdir, subprocess.run) = original
        sys.argv = argv
        os.environ.clear(); os.environ.update(saved)
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'loader tracked inputs changed'
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
                assert ast.dump(actual) == ast.dump(node), ('loader original body changed', entry.name, node.name)
            elif isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id in
                    ('FAULTS', 'WORKED', 'ASSERTIONS', 'REFUSALS', 'SIGNATURES', 'RESERVED') for t in node.targets):
                assert any(ast.dump(n) == ast.dump(node) for n in current.body), 'loader authored population changed'
        old_main = next(node for node in original.body if isinstance(node, ast.If)
                        and ast.unparse(node.test) == "__name__ == '__main__'")
        new_main = next(node for node in current.body if isinstance(node, ast.If)
                        and ast.unparse(node.test) == "__name__ == '__main__'")
        assert len(new_main.body) == 1 and isinstance(new_main.body[0], ast.Try), 'loader CLI refusal wrapper changed'
        assert [ast.dump(node) for node in new_main.body[0].body] == [ast.dump(node) for node in old_main.body], 'loader original CLI body changed'
    return previous


def physical_controls():
    """Real missing/link/directory/boundary faults in fresh repository-volume copies."""
    generation = ROOT / 'target/loader-physical' / str(os.getpid())
    cases = ('missing', 'symlink', 'directory', 'nested-repo', 'symlink-store', 'invalid-toolchain')
    total = 0
    for index, (entry, case) in enumerate((entry, case) for entry in ENTRIES for case in cases):
        sandbox = generation / str(index)
        assert not sandbox.exists(), 'loader physical fixture already exists'
        originals = INPUTS[entry]
        mirrors = {path: sandbox / path.relative_to(ROOT) for path in originals}
        directories = {sandbox}
        for path in mirrors.values():
            directories.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
        GOOD['prepare_producer'](ROOT, sources=originals, directories=tuple(sorted(directories)), output_files=tuple(mirrors.values()))
        for source, destination in mirrors.items():
            shutil.copy2(source, destination)
        victim = mirrors[BOOK_INPUTS[-1]]
        wanted = 'producer source is not a local regular file'
        extra = {}
        if case == 'missing':
            victim.unlink()
        elif case == 'symlink':
            victim.unlink(); victim.symlink_to(mirrors[SOURCE]); wanted = 'symlink producer source'
        elif case == 'directory':
            victim.unlink(); victim.mkdir()
        elif case == 'nested-repo':
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
            raise AssertionError('loader physical refusal produced effect')
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
                    raise AssertionError(('loader physical fault accepted', entry.name, case))
                if case in ('missing', 'symlink', 'directory', 'nested-repo'):
                    namespace = runpy.run_path(str(mirrors[entry]))
                    try:
                        namespace['contracts'](verbose=False)
                    except namespace['PROFILE']['Refusal'] as error:
                        assert str(error) == wanted, 'loader borrowed refusal identity changed'
                    else:
                        raise AssertionError('loader borrowed physical refusal escaped')
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
            assert str(error) in ('loader bootstrap plan omitted', 'loader helper before bootstrap plan'), 'loader unrelated original red'
        else:
            raise AssertionError('loader original omission escaped')
        for mode in ('import', 'contracts', 'cli', 'cli-mutations'):
            output, plans, reads = capture(entry, mode)
            print('actual loader capture:', entry.name, mode, plans, 'plans', reads, 'reads')
            captures += 1
        if entry != ENTRIES[1]:
            for mode in ('load', 'fault-argv'):
                capture(entry, mode); captures += 1
        if entry == ENTRIES[2]:
            capture(entry, 'quantum'); captures += 1
        script = TEXTS[entry]
        bootstrap = "PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)"
        reference = "PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)"
        for anchor, mode in ((bootstrap, 'import'), (reference, 'contracts')):
            if anchor == reference and entry == ENTRIES[0]:
                mode = 'load'
            assert script.count(anchor) >= 1
            # Each occurrence is an actual guard; run the public call that reaches it.
            for index in range(script.count(anchor)):
                parts = script.split(anchor)
                fault = anchor.join(parts[:index + 1]) + 'None' + anchor.join(parts[index + 1:])
                selected = mode
                if anchor == reference and index:
                    selected = 'cli-mutations' if entry != ENTRIES[2] or index == 2 else 'quantum'
                if anchor == reference and entry == ENTRIES[0] and index == 1:
                    selected = 'mutations'
                try:
                    capture(entry, selected, {entry: fault})
                except AssertionError as error:
                    assert str(error) in ('loader bootstrap plan omitted', 'loader helper before bootstrap plan',
                                         'loader reference read before complete fresh plan', 'loader delegation before complete fresh plan'), (entry.name, index, selected, str(error))
                else:
                    raise AssertionError(('loader actual guard omission escaped', entry.name, index, selected))
                reds += 1
    physical = physical_controls()
    print('static loader controls:', captures, 'actual pure captures;', reds, 'actual guard reds;', physical,
          'physical CLI refusals/12 borrowed refusals/3 actual planner-disable admissions; original bodies/populations/source/argv/prepared stores exact; no activation')


def native():
    """Watched real consumers and full input restoration; never source-mutation dispatch."""
    paths = subprocess.check_output(['git', 'ls-files', '-z'], text=True).split(chr(0))
    rust = tuple(ROOT / path for path in paths if path.endswith('.rs'))
    helpers = tuple(HERE / name for name in (
        'dimension_payload_contract.py', 'static_review_contract.py', 'static_namespace_contract.py',
        'static_signature_contract.py', 'checked_recipe_mutations.py', 'static_coupled_contract.py',
        'first_phase_contract.py', 'static_coupled_driver.rs', 'canonical_worked_recipe_cases.tsv',
        'ambiguity_payload_contract.py', 'geometry_argument_contract.py', 'call_lookup_contract.py'))
    folders, book = [ROOT / 'docs/book/src'], []
    for folder in folders:
        GOOD['directory'](ROOT, folder.relative_to(ROOT).as_posix())
        for path in sorted(folder.iterdir()):
            GOOD['require'](not path.is_symlink(), 'symlink native control book entry')
            (folders if path.is_dir() else book).append(path)
    sources = tuple(dict.fromkeys((Path(__file__),
        Path(__file__).with_name('python_payload_library_contract.py'),
        PROFILE, SOURCE, *helpers, *book, *rust)))
    outputs = tuple(ROOT / 'target' / name for name in (
        'loader-native-first-phase.log', 'loader-native-coupled.log',
        'loader-native-frozen.json', 'loader-library.json',
        'loader-native-dimension.log', 'loader-native-ambiguity.log',
        'loader-native-geometry.log', 'loader-native-call.log'))
    GOOD['enter_producer'](ROOT, sources=sources, output_files=outputs)
    frozen = {path.relative_to(ROOT).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sources}
    for path in rust:
        assert path.read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()]), 'native Rust source changed'
    for entry, arguments, log, markers in (
        ('dimension_payload_contract.py', ['--mutations'], outputs[4],
         ('dimension payloads:4023 actual cases /3814', 'dimension payload faults:15')),
        ('ambiguity_payload_contract.py', ['--mutations'], outputs[5],
         ('D147 ambiguity contract: 4192', 'D147 faults: 16')),
        ('geometry_argument_contract.py', ['--mutations'], outputs[6],
         ('geometry argument contracts: 97', 'geometry argument faults: 12')),
        ('call_lookup_contract.py', ['--mutations'], outputs[7],
         ('call payloads:166', 'call payload faults:12', 'curve documentation:three')),
        ('first_phase_contract.py', ['--mutations'], outputs[0],
         ('D150 public:263', 'D150 reference:263', 'D150 faults:10')),
        ('static_coupled_contract.py', [], outputs[1], ('coupled static review: 26723 shared cases',)),
    ):
        result = subprocess.run([sys.executable, '-I', '-B', 'scripts/local_environment.py', '--',
                                 sys.executable, '-I', '-B', str(HERE / entry), *arguments],
                                cwd=ROOT, text=True, capture_output=True, env=dict(os.environ))
        log.write_text(result.stdout + result.stderr)
        assert result.returncode == 0 and all(marker in result.stdout for marker in markers), (entry, result.returncode, result.stderr)
        assert all(hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest for path, digest in frozen.items()), 'native input restoration failed'
        print('actual native consumer:', entry, 'terminal0; inputs exact', flush=True)
    metadata = ROOT / 'target/static_coupled_contract/baseline/cargo-artifacts.json'
    GOOD['plan_producer'](ROOT, sources=(metadata,))
    messages = [json.loads(line) for line in metadata.read_text().splitlines()]
    libraries = {Path(name) for row in messages if row.get('reason') == 'compiler-artifact'
                 and row.get('target', {}).get('name') == 'sc_core'
                 and 'lib' in row.get('target', {}).get('kind', [])
                 for name in row.get('filenames', []) if name.endswith('.rlib')}
    assert len(libraries) == 1, 'native current library population'
    library = libraries.pop()
    GOOD['plan_producer'](ROOT, sources=(library,))
    artifact = dict(path=library.relative_to(ROOT).as_posix(), bytes=library.stat().st_size,
                    sha256=hashlib.sha256(library.read_bytes()).hexdigest())
    outputs[2].write_text(json.dumps(frozen, indent=2) + '\n')
    outputs[3].write_text(json.dumps(artifact, indent=2) + '\n')
    print('native loader proof:', len(frozen), 'inputs;', len(rust), 'tracked Rust files;',
          sum('/src/' in path.relative_to(ROOT).as_posix() for path in rust),
          'production sources exact; 4023/3814/15 dimension;4192/16 ambiguity;97/12 geometry;166/12+3 call;263/10 competing;26723 coupled; current library frozen; rc=0')


if __name__ == '__main__':
    assert sys.argv[1:] in ([], ['--native']), 'static loader control arguments'
    native() if sys.argv[1:] else main()
