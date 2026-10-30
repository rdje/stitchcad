"""Actual retained-certificate/workspace census and compiled protection-omission controls.

Fresh owned fixtures contain no Git repository. Project cleanup apply is never invoked.
"""
from pathlib import Path
import ast
import hashlib
import os
import runpy
import shutil
import subprocess
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[4]
PROFILE_PATH = ROOT / 'scripts/local_environment.py'
SOURCE = Path(__file__).with_name('cleanup.py')
BASE = ROOT / 'target/cleanup_certificate_controls' / str(os.getpid())
INPUTS = (Path(__file__), SOURCE, PROFILE_PATH, ROOT / 'scripts/check_handoff.sh', ROOT / 'scripts/handoff_census.py')
CERTIFICATES = ('target/trig_forward_certificate', 'target/inverse_threshold_certificate',
                'target/irrational_platform_certificate')
WORKSPACES = ('target/history-capture-future', 'target/history-generations-future',
    'target/history-proof-future', 'target/history-review-future', 'target/book_search_future')
STORES = (*CERTIFICATES, *WORKSPACES)
ORDINARY = ('target/ordinary.log', 'target/other/book_search_future/ordinary.log')
NAMES = (*(store + '/retained.bin' for store in STORES), *ORDINARY)


def main():
    profile = runpy.run_path(str(PROFILE_PATH))
    mapping = {path: BASE / path.relative_to(ROOT) for path in INPUTS if path != Path(__file__)}
    files = (*mapping.values(), *(BASE / name for name in NAMES))
    directories = {BASE, *(path.parent for path in files)}
    for path in files:
        directories.update(parent for parent in path.parents if parent.is_relative_to(BASE))
    profile['plan_producer'](ROOT, sources=INPUTS, directories=tuple(directories), output_files=files)
    assert not BASE.exists(), 'stale owned certificate fixture'
    try:
        profile['enter_producer'](ROOT, sources=INPUTS, directories=tuple(directories), output_files=files)
        for source, destination in mapping.items():
            profile['plan_producer'](ROOT, output_files=(destination,))
            shutil.copyfile(source, destination)
        for name in NAMES:
            path = BASE / name
            profile['plan_producer'](ROOT, output_files=(path,))
            path.write_bytes(('owned fixture: ' + name + '\n').encode())
        saved = {name: hashlib.sha256((BASE / name).read_bytes()).hexdigest() for name in NAMES}
        # Read-only cleanup planning does not create stores. Prepare this mirrored root's
        # stores first, so Git receives an existing local TMPDIR rather than a missing path.
        profile['prepare_producer'](BASE, sources=tuple(mapping.values()))
        copied = mapping[SOURCE]
        module = runpy.run_path(str(copied))
        actual_run = subprocess.run

        def local_git(command, *args, **kwargs):
            if command[0] == 'git':
                temporary = Path(kwargs['env']['TMPDIR'])
                assert temporary.is_dir() and temporary.is_relative_to(BASE)
                assert temporary.stat().st_dev == ROOT.stat().st_dev
                kwargs['stderr'] = subprocess.PIPE
                result = actual_run(command, *args, **kwargs)
                assert not result.stderr, ('unexpected Git startup warning', result.stderr)
                return result
            return actual_run(command, *args, **kwargs)

        def require_protected(selected):
            for name in NAMES[:len(STORES)]:
                try:
                    selected['inspect'](name, set())
                except selected['Refusal'] as error:
                    assert 'protected output/store' in str(error) or 'protected proof workspace' in str(error)
                else:
                    raise AssertionError('retained proof deletion candidate admitted')
            census = selected['census']()
            assert {row['path'] for row in census['candidates']} == {*ORDINARY, 'target/scratch'}, 'certificate census was not pruned or ordinary cleanup changed'
            assert not any(row['path'] == store or row['path'].startswith(store + '/')
                for row in census['skipped'] for store in STORES), 'retained workspace traversed by census'
            scratch = next(row for row in census['candidates'] if row['path'] == 'target/scratch')
            # Git's platform launcher may create SDK cache data in the prepared local scratch.
            # That ordinary regenerable tree remains eligible; do not assume it stays empty.
            assert scratch['kind'] == 'tree'

        with patch.object(subprocess, 'run', local_git):
            require_protected(module)
        tree = ast.parse(copied.read_text())
        assignment = next(node for node in tree.body if isinstance(node, ast.Assign)
                          and isinstance(node.targets[0], ast.Name) and node.targets[0].id == 'PROTECTED')
        before = len(assignment.value.elts)
        assignment.value.elts = [item for item in assignment.value.elts
                                if not (isinstance(item, ast.Constant) and item.value in CERTIFICATES)]
        assert before - len(assignment.value.elts) == 3, 'fault did not remove exactly three certificate protections'
        faulty = {'__file__': str(copied), '__name__': 'certificate_protection_omitted'}
        exec(compile(ast.fix_missing_locations(tree), str(copied), 'exec'), faulty)
        with patch.object(subprocess, 'run', local_git):
            admitted = faulty['census']()['candidates']
            assert {row['path'] for row in admitted} == {*(name + '/retained.bin' for name in CERTIFICATES), *ORDINARY, 'target/scratch'}, 'actual omission failed to expose all three proof files'
            try:
                require_protected(faulty)
            except AssertionError as error:
                assert str(error) == 'retained proof deletion candidate admitted'
            else:
                raise AssertionError('omission did not fail the protection assertion')
            require_protected(module)
        family_tree = ast.parse(copied.read_text())
        function = next(node for node in family_tree.body
                        if isinstance(node, ast.FunctionDef) and node.name == 'retained_proof')
        function.body = [ast.Return(value=ast.Constant(value=False))]
        family_fault = {'__file__': str(copied), '__name__': 'proof_family_protection_omitted'}
        exec(compile(ast.fix_missing_locations(family_tree), str(copied), 'exec'), family_fault)
        with patch.object(subprocess, 'run', local_git):
            admitted = family_fault['census']()['candidates']
            assert {row['path'] for row in admitted} == {
                *(name + '/retained.bin' for name in WORKSPACES), *ORDINARY, 'target/scratch'}, \
                'actual family omission failed to expose future proof identities'
            try:
                require_protected(family_fault)
            except AssertionError as error:
                assert str(error) == 'retained proof deletion candidate admitted'
            else:
                raise AssertionError('family omission survived the actual protection assertion')
            require_protected(module)
        # Each new interface guard must matter independently: direct inspect and pruned census.
        for operation in ('inspect', 'census'):
            guarded_tree = ast.parse(copied.read_text())
            function = next(node for node in guarded_tree.body
                            if isinstance(node, ast.FunctionDef) and node.name == operation)
            if operation == 'inspect':
                guards = [node for node in function.body if isinstance(node, ast.Expr)
                    and isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Name)
                    and node.value.func.id == 'require' and node.value.args
                    and isinstance(node.value.args[0], ast.UnaryOp)
                    and isinstance(node.value.args[0].operand, ast.Call)
                    and isinstance(node.value.args[0].operand.func, ast.Name)
                    and node.value.args[0].operand.func.id == 'retained_proof']
                assert len(guards) == 1
                function.body.remove(guards[0])
            else:
                guards = [node for node in ast.walk(function) if isinstance(node, ast.If)
                    and isinstance(node.test, ast.BoolOp) and isinstance(node.test.op, ast.Or)
                    and isinstance(node.test.values[0], ast.Call)
                    and isinstance(node.test.values[0].func, ast.Name)
                    and node.test.values[0].func.id == 'retained_proof']
                assert len(guards) == 1 and len(guards[0].test.values) == 2
                guards[0].test = guards[0].test.values[1]
            omitted = {'__file__': str(copied), '__name__': 'proof_' + operation + '_guard_omitted'}
            exec(compile(ast.fix_missing_locations(guarded_tree), str(copied), 'exec'), omitted)
            with patch.object(subprocess, 'run', local_git):
                try:
                    require_protected(omitted)
                except AssertionError as error:
                    wanted = 'retained proof deletion candidate admitted' if operation == 'inspect' else 'retained workspace traversed by census'
                    assert str(error) == wanted, ('unrelated omitted proof guard', operation, error)
                else:
                    raise AssertionError(('actual omitted proof guard survived', operation))
                require_protected(module)
        assert {name: hashlib.sha256((BASE / name).read_bytes()).hexdigest() for name in NAMES} == saved
        assert all(source.read_bytes() == destination.read_bytes() for source, destination in mapping.items())
        print('retained cleanup: protected8/ordinary-strays2/actual omission-reds4/restored1; all data/source bytes unchanged, rc=0')
    finally:
        if BASE.exists() or BASE.is_symlink():
            profile['plan_producer'](ROOT, directories=(BASE,))
            assert BASE.is_dir() and not BASE.is_symlink() and BASE.stat().st_dev == ROOT.stat().st_dev
            assert shutil.rmtree.avoids_symlink_attacks
            assert all(not path.is_symlink() and path.name != '.git' and path.stat().st_dev == ROOT.stat().st_dev
                       and (path.is_file() or path.is_dir())
                       and (not path.is_file() or path.stat().st_nlink == 1)
                       for path in BASE.rglob('*')), 'unsafe owned certificate fixture; preserved'
            shutil.rmtree(BASE)


if __name__ == '__main__':
    main()
