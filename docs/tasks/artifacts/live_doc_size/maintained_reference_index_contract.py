"""Finite staged-authority controls;native Git remains read-only,no temporary repository/index."""
from __future__ import annotations

import hashlib
import os
from pathlib import Path
import runpy
import subprocess
import sys
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[4]
BASE = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/live_doc_size/maintained_reference_contract.py'))
API, Refusal = BASE['API'], BASE['Refusal']


class Staged(BASE['BookFixture']):
    def staged(self, changes=None, drift=False):
        data = {p.relative_to(self.tree).as_posix(): p.read_bytes()
                for p in self.tree.rglob('*') if p.is_file()}
        for name, value in (changes or {}).items():
            if value is None:
                data.pop(name)
            else:
                data[name] = value
        objects = {hashlib.sha256(value).hexdigest(): value for value in data.values()}
        listing = b''.join(f'100644 {hashlib.sha256(value).hexdigest()} 0\t{name}\0'.encode()
                           for name, value in sorted(data.items()))
        native = API['git']
        lists = 0

        def git(root, arguments, maximum, **options):
            nonlocal lists
            if arguments[:2] == ['ls-files', '--stage']:
                lists += 1
                value = listing + (b'100644 ' + b'a' * 40 + b' 0\tdocs/tasks/other.md\0'
                                   if drift and lists > 1 else b'')
            elif arguments[:2] == ['cat-file', 'blob']:
                value = objects[arguments[2]]
            else:
                return native(root, arguments, maximum, **options)
            API['need'](len(value) <= maximum, 'budget', 'fixture native output bound')
            return value

        return patch.dict(API['check_index'].__globals__, git=git)

    def test_exact_staged_candidate_passes_without_touching_native_index(self):
        environment = dict(os.environ, GIT_OPTIONAL_LOCKS='0')
        before = subprocess.check_output(['git', '-C', str(ROOT), 'status', '--porcelain'], env=environment)
        with self.staged():
            result = API['check_index'](ROOT)
        self.assertEqual((result.mode, result.owner), ('index-candidate', BASE['OWNER']))
        self.assertEqual(before, subprocess.check_output(['git', '-C', str(ROOT), 'status', '--porcelain'], env=environment))

    def test_older_staged_control_cannot_be_licensed_by_fresh_working_authority(self):
        old = API['blob'](ROOT, self.head, API['CONTROL'], API['CONTROL_BYTES'])
        with self.staged({API['CONTROL']: old}), self.assertRaises(Refusal) as caught:
            API['check_index'](ROOT)
        self.assertEqual(caught.exception.code, 'target')

    def test_unstaged_completed_owner_does_not_complete_the_staged_task(self):
        task = 'docs/tasks/G1-SLICE.md'
        pending = (self.tree / task).read_bytes().replace(b'Status: `done`', b'Status: `pending`')
        with self.staged({task: pending}), self.assertRaises(Refusal) as caught:
            API['check_index'](ROOT)
        self.assertEqual(caught.exception.code, 'task')

    def test_missing_indexed_chapter_and_changed_staged_profile_refuse(self):
        with self.staged({API['SOURCE'] + '/introduction.md': None}), self.assertRaises(Refusal) as caught:
            API['check_index'](ROOT)
        self.assertEqual(caught.exception.code, 'navigation')
        registry = (self.tree / API['REGISTRY']).read_bytes().replace(b'700\t40960\t440', b'701\t40960\t440')
        with self.staged({API['REGISTRY']: registry}), self.assertRaises(Refusal) as caught:
            API['check_index'](ROOT)
        self.assertEqual(caught.exception.code, 'profile')

    def test_index_change_during_verification_refuses(self):
        with self.staged(drift=True), self.assertRaises(Refusal) as caught:
            API['check_index'](ROOT)
        self.assertEqual(caught.exception.code, 'changed')

    def test_staged_modes_stages_duplicates_and_native_shape_refuse(self):
        name = API['CONTROL'].encode()
        valid = b'100644 ' + b'a' * 40 + b' 0\t' + name + b'\0'
        self.assertEqual(API['staged_entries'](valid), {API['CONTROL']: 'a' * 40})
        cases = [valid.replace(b'100644', mode) for mode in (b'120000', b'160000')]
        cases += [valid.replace(b' 0\t', b' ' + stage + b'\t') for stage in (b'1', b'2', b'3')]
        cases += [valid + valid, valid[:-1], valid.replace(b'a' * 40, b'not-a-native-object')]
        for source in cases:
            with self.subTest(source=source), self.assertRaises(Refusal):
                API['staged_entries'](source)

    def test_index_cli_cannot_select_a_candidate_or_record_authority(self):
        control = (ROOT / API['CONTROL']).read_bytes()
        for arguments in [
            ['--candidate', 'target/readonly-index-probe'],
            ['record-change', '--task', BASE['OWNER'], '--reason', 'Read-only index control'],
        ]:
            with self.subTest(arguments=arguments):
                result = subprocess.run([sys.executable, '-I', '-B', str(BASE['MODULE']), '--index', *arguments],
                                        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, check=False)
                self.assertEqual(result.returncode, 2, result.stdout)
                self.assertIn(b'index mode cannot select', result.stdout)
        self.assertEqual((ROOT / API['CONTROL']).read_bytes(), control)
        self.assertFalse((ROOT / 'target/readonly-index-probe').exists())


if __name__ == '__main__':
    unittest.main(verbosity=2)
