"""D156: planned Python entry paths, effective activation and actual standalone prefix captures."""
from pathlib import Path
import ast
import contextlib
import io
import os
import runpy
import subprocess
import tempfile
import types

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'scripts/local_environment.py'
ENTRY = ROOT / 'docs/tasks/artifacts/size_membership/size_membership_mutations.py'
EXPECTED = {'CARGO_HOME': 'target/cargo-home', 'RUSTUP_HOME': 'target/cargo-home/rustup-ci',
            'CARGO_TARGET_DIR': 'target', 'TMPDIR': 'target/scratch', 'MAKE_TMPDIR': 'target/scratch'}
GOOD = runpy.run_path(str(SOURCE))


def controls(scope):
    work = GOOD['directory'](ROOT, 'target/scratch/python_producer_contract', create=True)
    saved = dict(os.environ)
    count = 0
    try:
        with tempfile.TemporaryDirectory(dir=work) as directory:
            root = Path(directory)
            source = root / 'input'
            source.write_text('owned source')
            output = root / 'target/output'
            wanted = {name: str(root / relative) for name, relative in EXPECTED.items()}
            os.environ['RUSTUP_TOOLCHAIN'] = '1.99.0'
            os.environ['D156_RETAINED'] = 'opaque value'
            original_mkdir, observations = Path.mkdir, []

            def observe(path, *args, **kwargs):
                if path == output:
                    observations.append({name: os.environ.get(name) for name in EXPECTED})
                return original_mkdir(path, *args, **kwargs)

            Path.mkdir = observe
            try:
                scope['prepare_producer'](root, directories=('target/output',), sources=('input',))
            finally:
                Path.mkdir = original_mkdir
            assert observations == [wanted], 'D156 Python output before activation'
            assert all(os.environ[name] == value for name, value in wanted.items()), 'D156 Python effective stores'
            assert os.environ['RUSTUP_TOOLCHAIN'] == '1.99.0' and os.environ['D156_RETAINED'] == 'opaque value', 'D156 Python retained environment'
            assert source.read_text() == 'owned source' and output.is_dir(), 'D156 Python owned paths'
            count += 4
        for kind in ('external', 'source-output', 'escape', 'bytes', 'type', 'missing-source',
                     'source-link', 'source-parent-link', 'source-git', 'late-source', 'source-directory', 'source-device'):
            with tempfile.TemporaryDirectory(dir=work) as directory:
                root = Path(directory)
                (root / 'input').write_text('owned source')
                outputs, inputs = ['target/output'], ['input']
                message = None
                if kind == 'external':
                    outputs = [str(ROOT.parent / 'never-created')]
                    message = 'producer path outside workspace'
                elif kind == 'source-output':
                    outputs = ['unowned-output']
                    message = 'producer output must be under target'
                elif kind == 'escape':
                    outputs = ['target/../escape']
                    message = 'invalid producer path components'
                elif kind == 'bytes':
                    inputs = ['input\n']
                    message = 'invalid producer path bytes'
                elif kind == 'type':
                    outputs = [None]
                    message = 'invalid producer path type'
                elif kind in ('missing-source', 'late-source'):
                    inputs = ['absent'] if kind == 'missing-source' else ['input', 'absent']
                    message = 'producer source is not a local regular file'
                elif kind == 'source-directory':
                    inputs = ['data']
                    (root / 'data').mkdir()
                    message = 'producer source is not a local regular file'
                elif kind == 'source-device':
                    message = 'producer source is not a local regular file'
                elif kind == 'source-link':
                    (root / 'alias').symlink_to(root / 'input')
                    inputs = ['alias']
                    message = 'symlink producer source'
                else:
                    (root / 'data').mkdir()
                    (root / 'data/input').write_text('owned source')
                    if kind == 'source-parent-link':
                        (root / 'alias').symlink_to(root / 'data', target_is_directory=True)
                        inputs = ['alias/input']
                        message = 'symlink store component'
                    else:
                        (root / 'data/.git').write_text('synthetic boundary marker, no Git database')
                        inputs = ['data/input']
                        message = 'store crosses another Git repository'
                before = dict(os.environ)
                original_stat = Path.stat

                def changed_device(path, *args, **kwargs):
                    metadata = original_stat(path, *args, **kwargs)
                    if kind == 'source-device' and path == root / 'input':
                        return types.SimpleNamespace(st_mode=metadata.st_mode, st_dev=metadata.st_dev + 1)
                    return metadata

                try:
                    Path.stat = changed_device
                    try:
                        scope['prepare_producer'](root, directories=outputs, sources=inputs)
                    except scope['Refusal'] as error:
                        assert str(error) == message, ('D156 Python refusal identity', kind, str(error))
                    else:
                        raise AssertionError('D156 Python accepted ' + kind)
                finally:
                    Path.stat = original_stat
                assert dict(os.environ) == before and not (root / 'target').exists(), 'D156 Python refusal created stores or activated environment'
                count += 1
                marker = root / 'data/.git'
                if marker.exists():
                    marker.unlink()
        with tempfile.TemporaryDirectory(dir=work) as directory:
            output = io.StringIO()
            with contextlib.redirect_stderr(output):
                try:
                    scope['enter_producer'](Path(directory), directories=('source',))
                except SystemExit as error:
                    assert error.code == 2, 'D156 Python CLI status'
                else:
                    raise AssertionError('D156 Python CLI accepted invalid output')
            assert output.getvalue() == 'local-environment: REFUSED — producer output must be under target\n', 'D156 Python CLI message'
            count += 1
    finally:
        os.environ.clear()
        os.environ.update(saved)
    return count


def capture(text, entry_path=ENTRY, *, work_name=None, refusal=False):
    """Execute the actual prefix through its first mkdir, intercepting every dispatch/write."""
    tree = ast.parse(text)
    boundary = next(node.end_lineno for node in tree.body if isinstance(node, ast.Expr) and
                    isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Attribute) and
                    ast.unparse(node.value.func) == 'WORK.mkdir')
    prefix = '\n'.join(text.splitlines()[:boundary]) + '\n'
    saved, mkdir, run = dict(os.environ), Path.mkdir, subprocess.run
    observations = []
    diagnostic = io.StringIO()

    class Stopped(Exception):
        pass

    def observe(path, *args, **kwargs):
        expected_work = ROOT / 'target' / (work_name or entry_path.stem)
        assert path == expected_work, 'D156 Python unexpected prefix write'
        assert all(os.environ.get(name) == str(ROOT / value) for name, value in EXPECTED.items()), 'D156 Python standalone activation omitted'
        assert os.environ.get('RUSTUP_TOOLCHAIN') == '1.99.0', 'D156 Python standalone channel lost'
        observations.append(path)
        raise Stopped

    def no_child(*args, **kwargs):
        raise AssertionError('D156 Python unexpected prefix child')

    try:
        for name in EXPECTED:
            os.environ.pop(name, None)
        os.environ['RUSTUP_TOOLCHAIN'] = '1.99.0'
        Path.mkdir, subprocess.run = observe, no_child
        try:
            with contextlib.redirect_stderr(diagnostic):
                exec(compile(prefix, str(entry_path), 'exec'), {'__file__': str(entry_path), '__name__': 'entry_capture'})
        except Stopped:
            assert not refusal, 'D156 Python late-source refusal reached a write'
            pass
        except SystemExit as error:
            assert refusal and error.code == 2, 'D156 Python actual entry refusal status'
            assert diagnostic.getvalue() == 'local-environment: REFUSED — producer source is not a local regular file\n', 'D156 Python actual entry refusal message'
    finally:
        Path.mkdir, subprocess.run = mkdir, run
        os.environ.clear()
        os.environ.update(saved)
    assert len(observations) == (0 if refusal else 1), 'D156 Python standalone capture absent'


def main():
    original, entry = SOURCE.read_bytes(), ENTRY.read_bytes()
    count = controls(GOOD)
    faults = (
        ('output area', "require(Path(relative).parts[:1] == ('target',), 'producer output must be under target')", 'pass'),
        ('source link', "require(not path.is_symlink(), 'symlink producer source')", 'pass'),
        ('source parent', 'directory(root, Path(relative).parent.as_posix())', 'pass'),
        ('source device', 'path.is_file() and path.stat().st_dev == root.stat().st_dev', 'path.is_file()'),
        ('early stores', 'environment_for(root, os.environ, create=False)', 'environment_for(root, os.environ)'),
        ('activation', 'os.environ.update(environment)', 'pass'),
    )
    red = 0
    for name, before, after in faults:
        assert original.decode().count(before) == 1, (name, 'actual anchor')
        scope = {'__file__': str(SOURCE), '__name__': 'producer_fault'}
        exec(compile(original.decode().replace(before, after, 1), str(SOURCE), 'exec'), scope)
        try:
            controls(scope)
        except AssertionError as error:
            assert str(error).startswith('D156 Python'), (name, 'unrelated failure', str(error))
            red += 1
        else:
            raise AssertionError((name, 'body fault survived'))
    call = "runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](\n    ROOT, directories=(WORK,), sources=(SOURCE,))\n"
    adopters = (
        (ENTRY, 'size_membership_mutations'),
        (ROOT / 'docs/tasks/artifacts/ease/ease_mutations.py', 'ease_mutations'),
        (ROOT / 'docs/tasks/artifacts/ease/ease_set_mutations.py', 'ease_set_mutations'),
        (ROOT / 'docs/tasks/artifacts/measurement_table/table_mutations.py', 'measurement_table_mutations'),
        (ROOT / 'docs/tasks/artifacts/size_chart/size_chart_mutations.py', 'size_chart_mutations'),
        (ROOT / 'docs/tasks/artifacts/size_chart_collection/size_chart_collection_mutations.py', 'size_chart_collection_mutations'),
        (ROOT / 'docs/tasks/artifacts/mtm_chart/mtm_chart_mutations.py', 'mtm_chart_mutations'),
    )
    originals = {path: path.read_bytes() for path, _ in adopters}
    for path, work_name in adopters:
        text = path.read_text()
        capture(text, path, work_name=work_name)
        assert not (ROOT / 'target/scratch/python_producer_contract/absent-source').exists()
        assert text.count('sources=(SOURCE,)') == 1
        capture(text.replace('sources=(SOURCE,)',
                             "sources=(SOURCE, ROOT / 'target/scratch/python_producer_contract/absent-source')", 1),
                path, work_name=work_name, refusal=True)
        assert text.count(call) == 1
        try:
            capture(text.replace(call, '', 1), path, work_name=work_name)
        except AssertionError as error:
            assert str(error) == 'D156 Python standalone activation omitted'
            red += 1
        else:
            raise AssertionError('D156 Python standalone body fault survived')
    assert SOURCE.read_bytes() == original and ENTRY.read_bytes() == entry, 'D156 Python source changed'
    assert all(path.read_bytes() == data for path, data in originals.items()), 'D156 Python adopter source changed'
    print('Python producer controls: ' + str(count) + ' runtime cases / ' + str(red) +
          ' actual body reds / ' + str(len(adopters)) + ' actual standalone pre-write captures / ' +
          str(len(adopters)) + ' actual late-source refusals / source unchanged')


if __name__ == '__main__':
    main()
