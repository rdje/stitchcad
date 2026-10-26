"""Actual read-only certificate/stray census and compiled protection-omission control.

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
STORES = ('target/trig_forward_certificate', 'target/inverse_threshold_certificate')
NAMES = (*(store + '/retained.bin' for store in STORES), 'target/ordinary.log')


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
            for name in NAMES[:2]:
                try:
                    selected['inspect'](name, set())
                except selected['Refusal'] as error:
                    assert 'protected output/store' in str(error)
                else:
                    raise AssertionError('certificate deletion candidate admitted')
            census = selected['census']()
            assert {row['path'] for row in census['candidates']} == {NAMES[2], 'target/scratch'}, 'certificate census was not pruned or ordinary cleanup changed'
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
                                if not (isinstance(item, ast.Constant) and item.value in STORES)]
        assert before - len(assignment.value.elts) == 2, 'fault did not remove exactly both new protections'
        faulty = {'__file__': str(copied), '__name__': 'certificate_protection_omitted'}
        exec(compile(ast.fix_missing_locations(tree), str(copied), 'exec'), faulty)
        with patch.object(subprocess, 'run', local_git):
            admitted = faulty['census']()['candidates']
            assert {row['path'] for row in admitted} == {*NAMES, 'target/scratch'}, 'actual omission failed to expose both proof files'
            try:
                require_protected(faulty)
            except AssertionError as error:
                assert str(error) == 'certificate deletion candidate admitted'
            else:
                raise AssertionError('omission did not fail the protection assertion')
            require_protected(module)
        assert {name: hashlib.sha256((BASE / name).read_bytes()).hexdigest() for name in NAMES} == saved
        assert all(source.read_bytes() == destination.read_bytes() for source, destination in mapping.items())
        print('certificate cleanup: protected2/ordinary-stray1/actual omission-red1/restored1; all data/source bytes unchanged, rc=0')
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
