"""Arithmetic/angle entry plans: actual bodies, physical refusals and source-only restoration."""
from pathlib import Path
import ast
import contextlib
import hashlib
import io
import os
import re
import runpy
import shutil
import subprocess
import sys
import types

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
INPUT = HERE / 'formula_input.py'
HELPERS = (INPUT, HERE / 'formula_structure.py', HERE / 'static_signature_contract.py')
CHAPTERS = tuple(ROOT / ('docs/book/src/' + name) for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))
ENTRIES = tuple(HERE / name for name in ('arithmetic_contract.py', 'angle_contract.py', 'arithmetic_mutations.py', 'angle_mutations.py'))
ORACLE = HERE / 'angle_math_oracle.py'
GOOD = runpy.run_path(str(PROFILE_SOURCE))
ALL = (*ENTRIES, PROFILE_SOURCE, SOURCE, *HELPERS, *CHAPTERS, HERE / 'arithmetic_cases.tsv', HERE / 'angle_cases.tsv', ORACLE)
GOOD['plan_producer'](ROOT, sources=(*ALL, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in ALL}
BEFORE = '53b917e'
ORIGINALS = {path: subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], text=True) for path in ENTRIES}


def inputs(entry):
    index = 0 if entry in (ENTRIES[0], ENTRIES[2]) else 1
    paths = (entry, PROFILE_SOURCE, SOURCE, *HELPERS, *CHAPTERS,
             HERE / ('arithmetic_cases.tsv' if index == 0 else 'angle_cases.tsv'))
    return set((*paths, ENTRIES[index])) if entry in ENTRIES[2:] else set(paths)


def capture(entry, candidate=None):
    """Execute entry code while intercepting source writes and unsafe child dispatch."""
    saved, argv = dict(os.environ), sys.argv
    originals = (runpy.run_path, Path.read_bytes, Path.read_text, Path.write_bytes,
                 Path.write_text, Path.mkdir, subprocess.run)
    plans, writes, children = [], [], []
    environment, last = {}, [None]
    expected = inputs(entry)
    count = 9 if entry == ENTRIES[2] else 7
    work = ROOT / ('target/formula_arithmetic_mutations' if entry == ENTRIES[2] else 'target/formula_angle_mutations')
    outputs = {work / ('mutation-%d.log' % number) for number in range(1, count + 1)}

    def plan(root, **kwargs):
        assert root == ROOT, 'numeric root changed'
        state = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == state, 'numeric inspection changed environment'
        plans.append(kwargs); last[0] = kwargs
        return result

    def enter(root, **kwargs):
        assert set(kwargs.get('sources', ())) == expected, 'numeric source plan incomplete'
        assert set(kwargs.get('output_files', ())) == outputs, 'numeric output plan incomplete'
        assert kwargs.get('directories') == (work,), 'numeric directory plan incomplete'
        assert kwargs.get('mutable_sources') == (SOURCE,), 'numeric source authority omitted'
        plan(root, **kwargs)
        environment.update(GOOD['environment_for'](root, os.environ, create=False))
        os.environ.update(environment)
        return dict(environment)

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, enter_producer=enter)
        assert plans and path in last[0].get('sources', ()), 'numeric loader before bootstrap'
        namespace = originals[0](file, *args, **kwargs)
        if path == INPUT:
            name = 'load_arithmetic_context'
            real = namespace[name]
            def reference():
                assert set(last[0].get('sources', ())) == expected, 'numeric reference plan incomplete'
                return real()
            namespace[name] = reference
        return namespace

    def read(path, binary, *args, **kwargs):
        if path == SOURCE or path in (HERE / 'arithmetic_cases.tsv', HERE / 'angle_cases.tsv'):
            assert any(path in item.get('sources', ()) for item in plans), 'numeric read before plan'
        return originals[1 if binary else 2](path, *args, **kwargs)

    def write(path, data, *args, **kwargs):
        assert environment, 'numeric write before activation'
        assert last[0] is not None and len(plans) > (writes[-1][1] if writes else 1), 'numeric write recheck omitted'
        if path == SOURCE:
            assert SOURCE in last[0].get('mutable_sources', ()), 'numeric mutable recheck omitted'
        else:
            assert path in outputs and path in last[0].get('output_files', ()), 'numeric log recheck omitted'
            assert children and len(plans) > children[-1][1], 'numeric log recheck omitted'
        writes.append((path, len(plans)))
        return len(data)

    def mkdir(path, *args, **kwargs):
        assert environment and path == work, 'numeric mkdir before activation'

    def child(arguments, *args, **kwargs):
        assert last[0] is not None and len(plans) > writes[-1][1], 'numeric child recheck omitted'
        contract = ENTRIES[0 if entry == ENTRIES[2] else 1]
        assert arguments == [sys.executable, '-I', '-B', contract.relative_to(ROOT).as_posix()], 'numeric child arguments changed'
        assert not args and kwargs == dict(cwd=ROOT, env=environment, capture_output=True), 'numeric child environment changed'
        assert environment['NUMERIC_UNRELATED'] == 'preserved', 'numeric caller environment lost'
        assert set(last[0].get('sources', ())) == expected, 'numeric child sources incomplete'
        children.append((arguments, len(plans)))
        return types.SimpleNamespace(returncode=1, stdout=b'AssertionError arithmetic_contract.py angle_contract.py\n', stderr=b'')

    try:
        os.environ['NUMERIC_UNRELATED'] = 'preserved'
        sys.argv = [str(entry), 'unrelated']; current_argv = sys.argv
        runpy.run_path = load
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.write_bytes = Path.write_text = write
        Path.mkdir, subprocess.run = mkdir, child
        namespace = dict(__file__=str(entry), __name__='borrowed_numeric')
        with contextlib.redirect_stdout(io.StringIO()):
            exec(compile(candidate or FROZEN[entry].decode(), str(entry), 'exec'), namespace)
        assert sys.argv is current_argv, 'numeric argv changed'
        if entry in ENTRIES[:2]:
            assert not writes and not children and dict(os.environ) == dict(saved, NUMERIC_UNRELATED='preserved'), 'numeric pure call produced effects'
            assert namespace['checks'] == (162 if entry == ENTRIES[0] else 72), 'numeric authored checks changed'
        else:
            assert len(children) == count and len(writes) == count * 3 + 1, 'numeric mutation effects changed'
        return len(children), len(writes)
    finally:
        runpy.run_path, Path.read_bytes, Path.read_text, Path.write_bytes, Path.write_text, Path.mkdir, subprocess.run = originals
        os.environ.clear(); os.environ.update(saved); sys.argv = argv
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'numeric capture changed actual source'


def body_controls():
    reds = 0
    for entry in ENTRIES:
        try:
            capture(entry, ORIGINALS[entry])
        except AssertionError as error:
            assert str(error) in ('numeric loader before bootstrap', 'numeric mkdir before activation'), error
            reds += 1
        else:
            raise AssertionError('numeric original gap escaped')
        new, old = ast.parse(FROZEN[entry].decode()), ast.parse(ORIGINALS[entry])
        def assignment(tree, name):
            return next(node.value for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == name for target in node.targets))
        if entry in ENTRIES[2:]:
            assert ast.dump(assignment(new, 'CASES')) == ast.dump(assignment(old, 'CASES')), 'numeric authored faults changed'
            faults = (
                ("mutable_sources=(SOURCE,))\nenvironment", "mutable_sources=())\nenvironment", 'numeric source authority omitted'),
                ("output_files=OUTPUT_FILES, sources=SOURCES", "output_files=OUTPUT_FILES[:-1], sources=SOURCES", 'numeric output plan incomplete'),
                ("output_files=OUTPUT_FILES, sources=SOURCES", "output_files=OUTPUT_FILES, sources=SOURCES[:-1]", 'numeric source plan incomplete'),
                ("            PROFILE['plan_producer'](ROOT, **PLAN)\n            SOURCE.write_text", "            SOURCE.write_text", 'numeric write recheck omitted'),
                ("            PROFILE['plan_producer'](ROOT, **PLAN)\n            result = subprocess.run", "            result = subprocess.run", 'numeric child recheck omitted'),
                ("            PROFILE['plan_producer'](ROOT, **PLAN)\n            (WORK", "            (WORK", 'numeric log recheck omitted'),
                ('cwd=ROOT, env=environment, capture_output=True', 'cwd=ROOT, capture_output=True', 'numeric child environment changed'),
            )
        else:
            faults = (
                ("    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'numeric loader before bootstrap'),
                ("    PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)", '    None', 'numeric reference plan incomplete'),
            )
            # Numerical helpers and the entire authored assertion suffix remain exact.
            marker = 'checks = 0' if entry == ENTRIES[0] else 'reference.sigs.update'
            assert FROZEN[entry].decode().split(marker, 1)[1] == ORIGINALS[entry].split(marker, 1)[1], 'numeric assertion body changed'
        for anchor, replacement, wanted in faults:
            text = FROZEN[entry].decode(); assert text.count(anchor) == 1, anchor
            try:
                capture(entry, text.replace(anchor, replacement, 1))
            except AssertionError as error:
                assert str(error) == wanted, (anchor, error, wanted)
                reds += 1
            else:
                raise AssertionError(('numeric omission escaped', anchor))
    print('numeric body controls: %d actual reds; original gaps/numeric bodies/fault populations verified' % reds)


def mirror(entry, number):
    sandbox = ROOT / 'target/numeric-plan-physical' / str(os.getpid()) / str(number)
    assert not sandbox.exists(), 'numeric mirror exists'
    sources = inputs(entry)
    copies = {path: sandbox / path.relative_to(ROOT) for path in sources}
    dirs = {sandbox}
    for path in copies.values():
        dirs.update(parent for parent in path.parents if parent.is_relative_to(sandbox))
    GOOD['prepare_producer'](ROOT, sources=tuple(sources), directories=tuple(dirs), output_files=tuple(copies.values()))
    for source, destination in copies.items():
        shutil.copy2(source, destination)
    return sandbox, copies


def physical_controls():
    count, positives, restores = 0, 0, 0
    for entry in ENTRIES:
        cases = ['missing-source', 'linked-source', 'directory-source', 'nested-source', 'linked-store', 'invalid-toolchain']
        if entry in ENTRIES[2:]:
            cases += ['hardlinked-source', 'linked-log', 'hardlinked-log', 'directory-log', 'late-log']
        else:
            cases += ['unused-log', 'readonly-source']
        for case in cases:
            sandbox, copies = mirror(entry, count); count += 1
            source = copies[SOURCE]; original = source.read_bytes()
            work = sandbox / ('target/formula_arithmetic_mutations' if entry in (ENTRIES[0], ENTRIES[2]) else 'target/formula_angle_mutations')
            log = work / 'mutation-1.log'; alias = sandbox / 'target/alias'
            GOOD['prepare_producer'](ROOT, directories=(work,), output_files=(log, alias))
            expected, positive = None, case in ('unused-log', 'readonly-source')
            if case in ('missing-source', 'directory-source'):
                source.unlink()
                if case == 'directory-source':
                    source.mkdir()
                expected = 'producer source is not a local regular file'
            elif case == 'linked-source':
                source.unlink(); source.symlink_to(copies[PROFILE_SOURCE]); expected = 'symlink producer source'
            elif case == 'nested-source':
                (source.parent / '.git').mkdir(); expected = 'store crosses another Git repository'
            elif case in ('hardlinked-source', 'readonly-source'):
                os.link(source, alias); expected = 'hard-linked mutable producer source'
            elif case in ('linked-log', 'unused-log'):
                log.symlink_to(source); expected = 'symlink producer output'
            elif case == 'hardlinked-log':
                os.link(source, log); expected = 'hard-linked producer output'
            elif case == 'directory-log':
                log.mkdir(); expected = 'producer output is not a local regular file'
            elif case == 'linked-store':
                (sandbox / 'target/scratch').symlink_to(source.parent, target_is_directory=True); expected = 'symlink store component'
            saved, argv = dict(os.environ), sys.argv
            originals = (Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run)
            def effect(*args, **kwargs):
                raise AssertionError('numeric physical refusal produced effects')
            def child(*args, **kwargs):
                log.symlink_to(copies[PROFILE_SOURCE])
                return types.SimpleNamespace(returncode=1, stdout=b'AssertionError arithmetic_contract.py angle_contract.py\n', stderr=b'')
            stderr = io.StringIO()
            try:
                os.environ.update({name: str(sandbox / suffix) for name, suffix in GOOD['STORES'].items()})
                os.environ.pop('RUSTUP_TOOLCHAIN', None)
                if case == 'invalid-toolchain':
                    os.environ['RUSTUP_TOOLCHAIN'] = 'invalid/toolchain'; expected = 'toolchain override must be a name'
                current = dict(os.environ); sys.argv = [str(copies[entry]), 'unrelated']; current_argv = sys.argv
                if case == 'late-log':
                    subprocess.run = child; expected = 'symlink producer output'
                else:
                    Path.mkdir = os.mkdir = Path.write_text = Path.write_bytes = subprocess.run = effect
                with contextlib.redirect_stderr(stderr), contextlib.redirect_stdout(io.StringIO()):
                    try:
                        runpy.run_path(str(copies[entry]))
                    except SystemExit as error:
                        assert not positive and error.code == 2 and stderr.getvalue() == 'local-environment: REFUSED — ' + expected + '\n', (case, error, stderr.getvalue())
                    else:
                        assert positive, ('numeric physical refusal escaped', case)
                        positives += 1
                assert sys.argv is current_argv, 'numeric physical argv changed'
                if case != 'late-log':
                    assert dict(os.environ) == current, 'numeric refusal/pure call changed environment'
                else:
                    assert source.read_bytes() == original and log.is_symlink(), 'numeric late log stranded mutable source'
                    restores += 1
            finally:
                Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run = originals
                os.environ.clear(); os.environ.update(saved); sys.argv = argv
    print('numeric physical controls: %d fixtures/%d pure positives/%d late-log exact restorations' % (count, positives, restores))


def real_controls():
    """Run genuine reference children exclusively and verify every scoped input afterward."""
    work = ROOT / 'target/numeric-plan-real'
    entries = (*ENTRIES, ORACLE)
    outputs = tuple(work / (entry.stem + '.log') for entry in entries)
    plan = dict(sources=(*ALL, Path(__file__)), directories=(work,),
                output_files=outputs, mutable_sources=(SOURCE,))
    environment = GOOD['enter_producer'](ROOT, **plan)
    for entry, output in zip(entries, outputs):
        GOOD['plan_producer'](ROOT, **plan)
        result = subprocess.run([sys.executable, '-I', '-B', str(entry)], cwd=ROOT,
                                env=environment, capture_output=True)
        GOOD['plan_producer'](ROOT, **plan)
        output.write_bytes(result.stdout + result.stderr)
        assert result.returncode == 0, (entry.name, result.returncode, result.stderr.decode())
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'numeric real run stranded source'
        print('numeric real:', entry.name, result.stdout.decode().splitlines()[-1], 'rc=0; scoped inputs exact')


def continuity_controls():
    """Reconstruct retained whole intervals from Git and verify prior immutable inputs."""
    reader = ROOT / 'scripts/history_archive.py'
    records = tuple(ROOT / ('docs/history/' + name) for name in (
        'stitchcad-devnotes-part284.md', 'stitchcad-changelog-part195.md'))
    for record in records:
        data = record.read_bytes()
        header, content = data.split(b'\n---\n\n', 1)
        text = header.decode()
        revision = re.search(r'complete intervals from ([0-9a-f]{40})\.', text).group(1)
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', text)
        assert (content.count(b'\n'), len(content), hashlib.sha256(content).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3]), 'numeric retained identity drift'
        chunks = []
        for path, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', text):
            source = subprocess.check_output(['git', 'show', revision + ':' + path], cwd=ROOT)
            chunk = source[int(start):int(stop)]
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest, 'numeric retained source interval drift'
            chunks.append(chunk)
        assert chunks and b''.join(chunks).rstrip(b'\n') + b'\n' == content, 'numeric whole retention incomplete'
        result = subprocess.run([sys.executable, '-I', '-B', str(reader), '--root', str(ROOT), 'read', record.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
        assert result.returncode == 0 and result.stdout == data, 'numeric public retained read differs'
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT), 'numeric prior archive changed'
    for path in (PROFILE_SOURCE, SOURCE, *HELPERS, *CHAPTERS, HERE / 'arithmetic_cases.tsv', HERE / 'angle_cases.tsv', ORACLE):
        assert path.read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT), 'numeric original reference input changed'
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    current = (ROOT / tree).read_text()
    headers = lambda value: re.findall(r'^- ID: `([^`]+)`$', value, re.M)
    assert headers(current) == headers(old), 'numeric canonical task graph changed'
    print('numeric continuity: two complete retained records/public reads; prior archive/reference bytes and task graph exact')


if __name__ == '__main__':
    if '--before' in sys.argv:
        for entry in ENTRIES:
            try:
                capture(entry, ORIGINALS[entry])
            except AssertionError as error:
                assert str(error) in ('numeric loader before bootstrap', 'numeric mkdir before activation'), error
                print('numeric original gap:', entry.name, str(error))
            else:
                raise AssertionError('numeric original gap escaped')
    elif '--real' in sys.argv:
        real_controls()
    elif '--continuity' in sys.argv:
        continuity_controls()
    else:
        body_controls()
        for entry in ENTRIES:
            print('numeric actual capture:', entry.name, capture(entry))
        physical_controls()
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'numeric controls changed source'
