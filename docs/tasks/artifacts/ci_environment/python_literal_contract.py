"""Literal entry plans: actual bodies, physical refusals and source-only restoration."""
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
ENTRIES = tuple(HERE / name for name in (
    'literal_contract.py', 'canonical_literal_contract.py',
    'literal_mutations.py', 'canonical_literal_mutations.py'))
GOOD = runpy.run_path(str(PROFILE_SOURCE))
ALL = (*ENTRIES, PROFILE_SOURCE, SOURCE, *HELPERS, *CHAPTERS, HERE / 'literal_cases.tsv')
GOOD['plan_producer'](ROOT, sources=(*ALL, Path(__file__)))
FROZEN = {path: path.read_bytes() for path in ALL}
BEFORE = '665b973'
ORIGINALS = {path: subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], text=True) for path in ENTRIES}


def inputs(entry):
    contract = ENTRIES[0 if entry.name == 'literal_mutations.py' else 1]
    paths = (entry, PROFILE_SOURCE, SOURCE, *HELPERS, *CHAPTERS)
    if entry in ENTRIES[2:]:
        paths += (contract,)
    if entry in (ENTRIES[0], ENTRIES[2]):
        paths += (HERE / 'literal_cases.tsv',)
    return set(paths)


def capture(entry, candidate=None):
    """Execute entry code while intercepting source writes and unsafe child dispatch."""
    saved, argv = dict(os.environ), sys.argv
    originals = (runpy.run_path, Path.read_bytes, Path.read_text, Path.write_bytes,
                 Path.write_text, Path.mkdir, subprocess.run)
    plans, writes, children = [], [], []
    environment, last = {}, [None]
    expected = inputs(entry)
    count = 6 if entry == ENTRIES[2] else 12
    work = ROOT / ('target/formula_literal_mutations' if entry == ENTRIES[2] else 'target/formula_canonical_literal_mutations')
    outputs = {work / ('mutation-%d.log' % number) for number in range(1, count + 1)}

    def plan(root, **kwargs):
        assert root == ROOT, 'literal root changed'
        state = dict(os.environ)
        result = GOOD['plan_producer'](root, **kwargs)
        assert dict(os.environ) == state, 'literal inspection changed environment'
        plans.append(kwargs); last[0] = kwargs
        return result

    def enter(root, **kwargs):
        assert set(kwargs.get('sources', ())) == expected, 'literal source plan incomplete'
        assert set(kwargs.get('output_files', ())) == outputs, 'literal output plan incomplete'
        assert kwargs.get('directories') == (work,), 'literal directory plan incomplete'
        assert kwargs.get('mutable_sources') == (SOURCE,), 'literal source authority omitted'
        plan(root, **kwargs)
        environment.update(GOOD['environment_for'](root, os.environ, create=False))
        os.environ.update(environment)
        return dict(environment)

    def load(file, *args, **kwargs):
        path = Path(file)
        if path == PROFILE_SOURCE:
            return dict(GOOD, plan_producer=plan, enter_producer=enter)
        assert plans and path in last[0].get('sources', ()), 'literal loader before bootstrap'
        namespace = originals[0](file, *args, **kwargs)
        if path == INPUT:
            name = 'load_context' if entry == ENTRIES[0] else 'load_arithmetic_context'
            real = namespace[name]
            def reference():
                assert set(last[0].get('sources', ())) == expected, 'literal reference plan incomplete'
                return real()
            namespace[name] = reference
        return namespace

    def read(path, binary, *args, **kwargs):
        if path == SOURCE or path == HERE / 'literal_cases.tsv':
            assert any(path in item.get('sources', ()) for item in plans), 'literal read before plan'
        return originals[1 if binary else 2](path, *args, **kwargs)

    def write(path, data, *args, **kwargs):
        assert environment, 'literal write before activation'
        assert last[0] is not None and len(plans) > (writes[-1][1] if writes else 1), 'literal write recheck omitted'
        if path == SOURCE:
            assert SOURCE in last[0].get('mutable_sources', ()), 'literal mutable recheck omitted'
        else:
            assert path in outputs and path in last[0].get('output_files', ()), 'literal log recheck omitted'
            assert children and len(plans) > children[-1][1], 'literal log recheck omitted'
        writes.append((path, len(plans)))
        return len(data)

    def mkdir(path, *args, **kwargs):
        assert environment and path == work, 'literal mkdir before activation'

    def child(arguments, *args, **kwargs):
        assert last[0] is not None and len(plans) > writes[-1][1], 'literal child recheck omitted'
        contract = ENTRIES[0 if entry == ENTRIES[2] else 1]
        assert arguments == [sys.executable, '-I', '-B', contract.relative_to(ROOT).as_posix()], 'literal child arguments changed'
        assert not args and kwargs == dict(cwd=ROOT, env=environment, capture_output=True), 'literal child environment changed'
        assert environment['LITERAL_UNRELATED'] == 'preserved', 'literal caller environment lost'
        assert set(last[0].get('sources', ())) == expected, 'literal child sources incomplete'
        children.append((arguments, len(plans)))
        return types.SimpleNamespace(returncode=1, stdout=b'AssertionError literal_contract.py\n', stderr=b'')

    try:
        os.environ['LITERAL_UNRELATED'] = 'preserved'
        sys.argv = [str(entry), 'unrelated']; current_argv = sys.argv
        runpy.run_path = load
        Path.read_bytes = lambda path, *args, **kwargs: read(path, True, *args, **kwargs)
        Path.read_text = lambda path, *args, **kwargs: read(path, False, *args, **kwargs)
        Path.write_bytes = Path.write_text = write
        Path.mkdir, subprocess.run = mkdir, child
        namespace = dict(__file__=str(entry), __name__='borrowed_literal')
        with contextlib.redirect_stdout(io.StringIO()):
            exec(compile(candidate or FROZEN[entry].decode(), str(entry), 'exec'), namespace)
        assert sys.argv is current_argv, 'literal argv changed'
        if entry in ENTRIES[:2]:
            assert not writes and not children and dict(os.environ) == dict(saved, LITERAL_UNRELATED='preserved'), 'literal pure call produced effects'
            assert namespace['checks'] == (361 if entry == ENTRIES[0] else 146), 'literal authored checks changed'
        else:
            assert len(children) == count and len(writes) == count * 3 + 1, 'literal mutation effects changed'
        return len(children), len(writes)
    finally:
        runpy.run_path, Path.read_bytes, Path.read_text, Path.write_bytes, Path.write_text, Path.mkdir, subprocess.run = originals
        os.environ.clear(); os.environ.update(saved); sys.argv = argv
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'literal capture changed actual source'


def body_controls():
    reds = 0
    for entry in ENTRIES:
        try:
            capture(entry, ORIGINALS[entry])
        except AssertionError as error:
            assert str(error) in ('literal loader before bootstrap', 'literal mkdir before activation'), error
            reds += 1
        else:
            raise AssertionError('literal original gap escaped')
        new, old = ast.parse(FROZEN[entry].decode()), ast.parse(ORIGINALS[entry])
        def assignment(tree, name):
            return next(node.value for node in tree.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == name for target in node.targets))
        if entry in ENTRIES[2:]:
            assert ast.dump(assignment(new, 'CASES')) == ast.dump(assignment(old, 'CASES')), 'literal authored faults changed'
            faults = (
                ("mutable_sources=(SOURCE,))\nenvironment", "mutable_sources=())\nenvironment", 'literal source authority omitted'),
                ("output_files=OUTPUT_FILES, sources=SOURCES", "output_files=OUTPUT_FILES[:-1], sources=SOURCES", 'literal output plan incomplete'),
                ("output_files=OUTPUT_FILES, sources=SOURCES", "output_files=OUTPUT_FILES, sources=SOURCES[:-1]", 'literal source plan incomplete'),
                ("            PROFILE['plan_producer'](ROOT, **PLAN)\n            SOURCE.write_text", "            SOURCE.write_text", 'literal write recheck omitted'),
                ("            PROFILE['plan_producer'](ROOT, **PLAN)\n            result = subprocess.run", "            result = subprocess.run", 'literal child recheck omitted'),
                ("            PROFILE['plan_producer'](ROOT, **PLAN)\n            (WORK", "            (WORK", 'literal log recheck omitted'),
                ('cwd=ROOT, env=environment, capture_output=True', 'cwd=ROOT, capture_output=True', 'literal child environment changed'),
            )
        else:
            faults = (
                ("    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)", '    None', 'literal loader before bootstrap'),
                ("    PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)", '    None', 'literal reference plan incomplete'),
            )
            # Numerical helpers and the entire authored assertion suffix remain exact.
            marker = '# Authored independently' if entry == ENTRIES[0] else 'MAX128, LOW64, HIGH64'
            assert FROZEN[entry].decode().split(marker, 1)[1] == ORIGINALS[entry].split(marker, 1)[1], 'literal numeric body changed'
        for anchor, replacement, wanted in faults:
            text = FROZEN[entry].decode(); assert text.count(anchor) == 1, anchor
            try:
                capture(entry, text.replace(anchor, replacement, 1))
            except AssertionError as error:
                assert str(error) == wanted, (anchor, error, wanted)
                reds += 1
            else:
                raise AssertionError(('literal omission escaped', anchor))
    print('literal body controls: %d actual reds; original gaps/numeric bodies/fault populations verified' % reds)


def mirror(entry, number):
    sandbox = ROOT / 'target/literal-plan-physical' / str(os.getpid()) / str(number)
    assert not sandbox.exists(), 'literal mirror exists'
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
            work = sandbox / ('target/formula_literal_mutations' if entry in (ENTRIES[0], ENTRIES[2]) else 'target/formula_canonical_literal_mutations')
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
                raise AssertionError('literal physical refusal produced effects')
            def child(*args, **kwargs):
                log.symlink_to(copies[PROFILE_SOURCE])
                return types.SimpleNamespace(returncode=1, stdout=b'AssertionError literal_contract.py\n', stderr=b'')
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
                        assert positive, ('literal physical refusal escaped', case)
                        positives += 1
                assert sys.argv is current_argv, 'literal physical argv changed'
                if case != 'late-log':
                    assert dict(os.environ) == current, 'literal refusal/pure call changed environment'
                else:
                    assert source.read_bytes() == original and log.is_symlink(), 'literal late log stranded mutable source'
                    restores += 1
            finally:
                Path.mkdir, os.mkdir, Path.write_text, Path.write_bytes, subprocess.run = originals
                os.environ.clear(); os.environ.update(saved); sys.argv = argv
    print('literal physical controls: %d fixtures/%d pure positives/%d late-log exact restorations' % (count, positives, restores))


def real_controls():
    """Run genuine reference children exclusively and verify every scoped input afterward."""
    work = ROOT / 'target/literal-plan-real'
    outputs = tuple(work / (entry.stem + '.log') for entry in ENTRIES)
    plan = dict(sources=(*ALL, Path(__file__)), directories=(work,),
                output_files=outputs, mutable_sources=(SOURCE,))
    environment = GOOD['enter_producer'](ROOT, **plan)
    for entry, output in zip(ENTRIES, outputs):
        GOOD['plan_producer'](ROOT, **plan)
        result = subprocess.run([sys.executable, '-I', '-B', str(entry)], cwd=ROOT,
                                env=environment, capture_output=True)
        GOOD['plan_producer'](ROOT, **plan)
        output.write_bytes(result.stdout + result.stderr)
        assert result.returncode == 0, (entry.name, result.returncode, result.stderr.decode())
        assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'literal real run stranded source'
        print('literal real:', entry.name, result.stdout.decode().splitlines()[-1], 'rc=0; scoped inputs exact')


def continuity_controls():
    """Reconstruct retained whole intervals from Git and verify prior immutable inputs."""
    reader = ROOT / 'scripts/history_archive.py'
    records = tuple(ROOT / ('docs/history/' + name) for name in (
        'stitchcad-devnotes-part282.md', 'stitchcad-changelog-part193.md'))
    for record in records:
        data = record.read_bytes()
        header, content = data.split(b'\n---\n\n', 1)
        text = header.decode()
        revision = re.search(r'complete intervals from ([0-9a-f]{40})\.', text).group(1)
        identity = re.search(r'Sealed identity:\*\* ([0-9]+) lines, ([0-9]+) bytes, sha256:([0-9a-f]{64})', text)
        assert (content.count(b'\n'), len(content), hashlib.sha256(content).hexdigest()) == (int(identity[1]), int(identity[2]), identity[3]), 'literal retained identity drift'
        chunks = []
        for path, start, stop, size, digest in re.findall(r'Whole source interval: ([^ ]+) bytes ([0-9]+)–([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})', text):
            source = subprocess.check_output(['git', 'show', revision + ':' + path], cwd=ROOT)
            chunk = source[int(start):int(stop)]
            assert len(chunk) == int(size) and hashlib.sha256(chunk).hexdigest() == digest, 'literal retained source interval drift'
            chunks.append(chunk)
        assert chunks and b''.join(chunks).rstrip(b'\n') + b'\n' == content, 'literal whole retention incomplete'
        result = subprocess.run([sys.executable, '-I', '-B', str(reader), '--root', str(ROOT), 'read', record.relative_to(ROOT).as_posix()], cwd=ROOT, capture_output=True)
        assert result.returncode == 0 and result.stdout == data, 'literal public retained read differs'
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BEFORE, '--', 'docs/history', '.doctrine/history_archive'], cwd=ROOT, text=True).splitlines()
    for path in paths:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path], cwd=ROOT), 'literal prior archive changed'
    for path in (PROFILE_SOURCE, SOURCE, *HELPERS, *CHAPTERS, HERE / 'literal_cases.tsv'):
        assert path.read_bytes() == subprocess.check_output(['git', 'show', BEFORE + ':' + path.relative_to(ROOT).as_posix()], cwd=ROOT), 'literal original reference input changed'
    tree = 'docs/tasks/G1-SLICE-producers.md'
    old = subprocess.check_output(['git', 'show', BEFORE + ':' + tree], cwd=ROOT, text=True)
    current = (ROOT / tree).read_text()
    headers = lambda value: re.findall(r'^- ID: `([^`]+)`$', value, re.M)
    assert headers(current) == headers(old), 'literal canonical task graph changed'
    print('literal continuity: two complete retained records/public reads; prior archive/reference bytes and task graph exact')


if __name__ == '__main__':
    if '--real' in sys.argv:
        real_controls()
    elif '--continuity' in sys.argv:
        continuity_controls()
    else:
        body_controls()
        for entry in ENTRIES:
            print('literal actual capture:', entry.name, capture(entry))
        physical_controls()
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'literal controls changed source'
