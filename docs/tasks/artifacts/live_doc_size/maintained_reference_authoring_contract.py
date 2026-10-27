"""Guarded authority publication in ordinary target candidates; native Git remains read-only."""
from __future__ import annotations

import json
import os
from pathlib import Path
import runpy
import subprocess
import sys
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[4]
READER = ROOT / "docs/tasks/artifacts/live_doc_size/maintained_reference.py"
CONTROLS = runpy.run_path(str(READER.with_name("maintained_reference_contract.py")))
API, OWNER = CONTROLS["API"], CONTROLS["OWNER"]
Refusal = API["Refusal"]


class Authoring(CONTROLS["BookFixture"]):
    def publish(self):
        return API["record_change"](OWNER, "Exact owned authoring contract", ROOT, self.tree)

    def canonical(self):
        return (self.tree / API["CONTROL"]).read_bytes()

    def temporary_files(self):
        base = self.tree / "target/maintained-reference-authoring"
        return list(base.iterdir()) if base.exists() else []

    def snapshot(self):
        return {p.relative_to(self.tree).as_posix(): p.read_bytes()
                for p in self.tree.rglob("*") if p.is_file()}

    def test_actual_publication_changes_only_exact_control_and_leaves_no_temporary_file(self):
        before = self.snapshot()
        result = self.publish()
        after = self.snapshot()
        self.assertEqual({name for name in before if before[name] != after[name]}, {API["CONTROL"]})
        self.assertEqual(set(before), set(after))
        self.assertEqual(result, API["check"](ROOT, self.tree))
        record = API["parse_control"](self.canonical())
        self.assertEqual(record["authority"]["task"], OWNER)
        self.assertEqual(record["authority"]["reason"], "Exact owned authoring contract")
        self.assertEqual(self.temporary_files(), [])
        saved = self.canonical()
        self.publish()
        self.assertEqual(self.canonical(), saved, "repeat preparation within this fresh task is deterministic")

    def test_cli_publishes_only_an_explicit_ordinary_project_candidate(self):
        result = subprocess.run([sys.executable, "-I", "-B", str(READER), "--candidate",
                                 self.tree.relative_to(ROOT).as_posix(), "record-change", "--task", OWNER,
                                 "--reason", "CLI exact proposal"], cwd=ROOT, stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE, check=False)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(API["parse_control"](self.canonical())["authority"]["reason"], "CLI exact proposal")
        self.assertEqual(self.temporary_files(), [])

    def test_malformed_or_conflicting_input_refuses_before_any_owned_write(self):
        good = self.canonical()
        cases = []
        cases.append(b"not JSONL\n")
        wrong = json.loads(json.dumps(self.document))
        wrong["references"][0]["authority"]["task"] = "G1-SLICE.someoneElse"
        header = {key: value for key, value in wrong.items() if key != "references"}
        cases.append((json.dumps(header) + "\n" + json.dumps(wrong["references"][0]) + "\n").encode())
        for raw in cases:
            with self.subTest(raw=raw[:20]):
                self.write(API["CONTROL"], raw)
                before = self.snapshot()
                with self.assertRaises(Refusal):
                    self.publish()
                self.assertEqual(self.snapshot(), before)
                self.assertEqual(self.temporary_files(), [])
        self.write(API["CONTROL"], good)
        before = self.snapshot()
        with self.assertRaises(Refusal):
            API["record_change"](OWNER, "", ROOT, self.tree)
        self.assertEqual(self.snapshot(), before)
        with self.assertRaises(Refusal) as result:
            API["record_change"](OWNER, "No alternate repository baseline", self.tree)
        self.assertEqual(result.exception.code, "path")
        self.assertEqual(self.snapshot(), before)

    def test_missing_control_is_created_only_after_complete_validation(self):
        (self.tree / API["CONTROL"]).unlink()
        result = self.publish()
        self.assertEqual(result, API["check"](ROOT, self.tree))
        self.assertEqual(self.temporary_files(), [])

    def test_native_replace_failure_preserves_previous_control_and_cleans_owned_temporary(self):
        before = self.snapshot()
        with patch.object(API["os"], "replace", side_effect=OSError("controlled rename refusal")):
            with self.assertRaises(Refusal) as result:
                self.publish()
        self.assertEqual(result.exception.code, "write")
        self.assertEqual(self.snapshot(), before)
        self.assertEqual(self.temporary_files(), [])

    def test_late_source_change_refuses_publication_without_undoing_the_callers_edit(self):
        old = self.canonical()
        original = API["os"].fsync
        source = self.tree / API["SOURCE"] / "introduction.md"
        def change(descriptor):
            original(descriptor)
            source.write_bytes(source.read_bytes() + b"\nExplicit concurrent author edit.\n")
        with patch.object(API["os"], "fsync", change):
            with self.assertRaises(Refusal) as result:
                self.publish()
        self.assertEqual(result.exception.code, "changed")
        self.assertEqual(self.canonical(), old)
        self.assertIn(b"Explicit concurrent author edit", source.read_bytes())
        self.assertEqual(self.temporary_files(), [])

    def test_late_destination_change_is_preserved_and_never_overwritten(self):
        original = API["os"].fsync
        path = self.tree / API["CONTROL"]
        replacement = b"Explicit external authority edit.\n"
        def change(descriptor):
            original(descriptor)
            path.write_bytes(replacement)
        with patch.object(API["os"], "fsync", change):
            with self.assertRaises(Refusal):
                self.publish()
        self.assertEqual(path.read_bytes(), replacement)
        self.assertEqual(self.temporary_files(), [])

    def test_unfinished_owner_and_unsafe_local_store_refuse_before_temporary_creation(self):
        task = self.tree / "docs/tasks/G1-SLICE.md"
        original = task.read_bytes()
        task.write_bytes(original.replace(f'- ID: `{OWNER}`\n  Status: `done`'.encode(),
                                          f'- ID: `{OWNER}`\n  Status: `in_progress`'.encode()))
        before = self.snapshot()
        with self.assertRaises(Refusal) as result:
            self.publish()
        self.assertEqual(result.exception.code, "task")
        self.assertEqual(before, self.snapshot())
        task.write_bytes(original)
        # A project-local store alias is invalid; no external directory or Git repository is touched.
        alias = self.tree / "unsafe-store"
        alias.symlink_to(ROOT / "target", target_is_directory=True)
        before = self.canonical()
        with patch.dict(os.environ, {"TMPDIR": str(alias)}):
            with self.assertRaises(Refusal) as result:
                self.publish()
        self.assertEqual(result.exception.code, "plan")
        self.assertEqual(self.canonical(), before)
        self.assertEqual(self.temporary_files(), [])


if __name__ == "__main__":
    unittest.main(verbosity=2)
