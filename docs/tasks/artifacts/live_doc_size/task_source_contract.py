"""D281 task-family freshness controls in ordinary target candidates; native Git stays read-only."""
from pathlib import Path
import argparse
from collections import Counter
import hashlib
import re
import runpy
import sys
import types
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[4]
READER = Path(__file__).with_name("maintained_reference.py")
CONTROLS = runpy.run_path(str(READER.with_name("maintained_reference_contract.py")))
API, OWNER = CONTROLS["API"], CONTROLS["OWNER"]
CHECK_API = API
Refusal = API["Refusal"]


class TaskSources(CONTROLS["BookFixture"]):
    def linked(self):
        root = self.tree / "docs/tasks/G1-SLICE.md"
        sibling = self.tree / "docs/tasks/G1-SLICE-probe.md"
        original = root.read_bytes()
        sibling.write_bytes(original)
        root.write_bytes(b"# Candidate tree\n\n- Tree ID: `G1-SLICE`\n\n"
                         b"Canonical task sibling: [probe](G1-SLICE-probe.md).\n")
        return root, sibling, original

    def test_fresh_linked_sibling_owner_authorizes_the_exact_candidate(self):
        self.linked()
        self.assertEqual(CHECK_API["check"](ROOT, self.tree).owner, OWNER)
        self.assertEqual(API["record_change"](OWNER, "Exact sibling-owned proposal", ROOT, self.tree).owner, OWNER)

    def test_duplicate_owner_across_root_and_sibling_refuses(self):
        root, _sibling, original = self.linked()
        root.write_bytes(root.read_bytes() + original)
        self.refused("task")

    def test_unlinked_or_missing_sibling_refuses(self):
        root, sibling, _original = self.linked()
        saved = root.read_bytes()
        root.write_bytes(b"# Candidate root without a sibling link\n")
        self.refused("task")
        root.write_bytes(saved)
        sibling.unlink()
        self.refused("task")

    def test_unsafe_sibling_is_refused_before_opening_its_body(self):
        _root, sibling, _original = self.linked()
        sibling.unlink()
        sibling.symlink_to(ROOT / "README.md")
        self.refused("path")

    def test_task_family_file_bound_is_inclusive_and_excess_refuses(self):
        self.assertEqual(API["TASK_FILES"], 32, "the adopted profile is fixed, not fixture-derived")
        root, _sibling, _original = self.linked()
        for index in range(API["TASK_FILES"] - 2):
            name = f"G1-SLICE-part{index}.md"
            (root.parent / name).write_bytes(b"# Explicit task-family part\n")
            root.write_bytes(root.read_bytes() + f"\n[part]({name})\n".encode())
        self.assertEqual(len(API["current_tasks"](self.tree, OWNER)), API["TASK_FILES"])
        self.assertEqual(API["check"](ROOT, self.tree).owner, OWNER)
        (root.parent / "G1-SLICE-excess.md").write_bytes(b"# Excess task-family source\n")
        self.refused("budget")

    def test_late_sibling_body_change_preserves_authority_and_cleans_owned_temporary(self):
        _root, sibling, _original = self.linked()
        control = self.tree / API["CONTROL"]
        before = control.read_bytes()
        original = API["os"].fsync
        def change(descriptor):
            original(descriptor)
            sibling.write_bytes(sibling.read_bytes() + b"\nCaller changed source rationale.\n")
        with patch.object(API["os"], "fsync", change):
            with self.assertRaises(Refusal) as result:
                API["record_change"](OWNER, "Complete source freezing", ROOT, self.tree)
        self.assertEqual(result.exception.code, "changed")
        self.assertEqual(control.read_bytes(), before)
        self.assertIn(b"Caller changed source rationale", sibling.read_bytes())
        self.assertEqual(list((self.tree / "target/maintained-reference-authoring").iterdir()), [])

    def test_completed_historical_sibling_owner_cannot_be_reused(self):
        closed = "G1-SLICE.5e.1.1"
        # Independent native blobs establish the original gap: absent at the root, done in execution.
        main = API["blob"](ROOT, self.head, "docs/tasks/G1-SLICE.md", API["TASK_BYTES"])
        execution = API["blob"](ROOT, self.head, "docs/tasks/G1-SLICE-execution.md", API["TASK_BYTES"])
        self.assertNotIn(f'- ID: `{closed}`\n'.encode(), main)
        self.assertIn(f'- ID: `{closed}`\n  Status: `done`'.encode(), execution)
        self.assertEqual(API["owner_status"](API["baseline_tasks"](ROOT, self.head, closed), closed), "done")
        _root, sibling, _original = self.linked()
        sibling.write_bytes(f'# Candidate sibling\n\n- ID: `{closed}`\n  Status: `done`\n'.encode())
        self.document["references"][0]["authority"]["task"] = closed
        self.save()
        self.refused("task")

    def test_native_task_directory_failure_is_never_absence(self):
        with self.assertRaises(Refusal) as result:
            API["baseline_tasks"](ROOT, "0" * 40, OWNER)
        self.assertEqual(result.exception.code, "git")

    def test_completed_sibling_owner_cannot_be_laundered_into_the_current_root(self):
        closed = "G1-SLICE.5e.1.1"
        main = API["blob"](ROOT, self.head, "docs/tasks/G1-SLICE.md", API["TASK_BYTES"])
        execution = API["blob"](ROOT, self.head, "docs/tasks/G1-SLICE-execution.md", API["TASK_BYTES"])
        self.assertNotIn(f'- ID: `{closed}`\n'.encode(), main)
        self.assertIn(f'- ID: `{closed}`\n  Status: `done`'.encode(), execution)
        self.write("docs/tasks/G1-SLICE.md",
                   f'# Candidate root\n\n- Tree ID: `G1-SLICE`\n\n- ID: `{closed}`\n  Status: `done`\n'.encode())
        self.document["references"][0]["authority"]["task"] = closed
        self.save()
        with self.assertRaises(CHECK_API["Refusal"]) as result:
            CHECK_API["check"](ROOT, self.tree)
        self.assertEqual(result.exception.code, "task")

    def test_actual_completed_graph_is_an_exact_live_partition_of_native_history(self):
        API["storage_profile"](ROOT, {"docs/tasks/G1-SLICE-garment.md"})
        data = (ROOT / "docs/tasks/G1-SLICE-garment.md").read_bytes()
        header, body = data.split(b"\n---\n\n", 1)
        pattern = (rb"^Initial source: ([0-9a-f]{40}|[0-9a-f]{64}) bytes ([0-9]+)" + "–".encode() +
                   rb"([0-9]+); ([0-9]+) bytes; sha256:([0-9a-f]{64})$")
        match = re.search(pattern, header, re.M)
        self.assertIsNotNone(match)
        revision, begin, end, size, checksum = match.groups()
        original = API["blob"](ROOT, revision.decode(), "docs/tasks/G1-SLICE.md", API["TASK_BYTES"])
        self.assertEqual(body, original[int(begin):int(end)])
        self.assertEqual(len(body), int(size))
        self.assertEqual(hashlib.sha256(body).hexdigest().encode(), checksum)
        nodes = re.findall(rb"^- ID: `(G1-SLICE\.6\.4(?:\.[^`]+)?)`\n  Status: `([^`]+)`", body, re.M)
        expected = re.findall(rb"^- ID: `(G1-SLICE\.6\.4(?:\.[^`]+)?)`\n  Status: `([^`]+)`", original, re.M)
        self.assertEqual(nodes, expected)
        self.assertEqual(len(nodes), 98)
        self.assertEqual({state for _identity, state in nodes}, {b"done"})
        current = API["current_tasks"](ROOT, OWNER)
        self.assertIn("docs/tasks/G1-SLICE-garment.md", current)
        node = rb"^- ID: `G1-SLICE\.6\.4(?:\.|`)"
        self.assertIsNotNone(re.search(node, original, re.M), "native predecessor makes this control red")
        self.assertIsNone(re.search(node, current["docs/tasks/G1-SLICE.md"], re.M))
        old_ids = Counter(re.findall(rb"^- ID: `([^`]+)`", original, re.M))
        live_ids = Counter(identity for source in current.values()
                           for identity in re.findall(rb"^- ID: `([^`]+)`", source, re.M))
        self.assertFalse(old_ids - live_ids, "every original node remains in the live canonical family")

    def test_completed_journal_retirement_preserves_source_nodes_and_current_navigation(self):
        history = runpy.run_path(str(ROOT / "scripts/history_archive.py"))
        record = history["inventory"](ROOT)[0]["docs/history/stitchcad-devnotes-part572.md"]
        header, body = record.split(b"\n---\n\n", 1)
        revision = re.search(rb"from ([0-9a-f]{40}|[0-9a-f]{64})\.", header)
        self.assertIsNotNone(revision)
        before = API["blob"](ROOT, revision[1].decode(), "docs/tasks/G1-SLICE-journal.md", API["TASK_BYTES"])
        self.assertEqual(body, before)
        self.assertFalse((ROOT / "docs/tasks/G1-SLICE-journal.md").exists())
        nodes = re.findall(rb"^- ID: `([^`]+)`\n  Status: `([^`]+)`", before, re.M)
        self.assertEqual(len(nodes), 7)
        current = API["current_tasks"](ROOT, OWNER)
        for identity, _historical_status in nodes:
            states = [state for source in current.values()
                      if (state := API["task_status"](source, identity.decode())) is not None]
            self.assertTrue(states, identity)
            self.assertEqual(set(states), {"done"}, identity)
        old_links = 0
        for name in ("G1-SLICE-canonical.md", "G1-SLICE-formulas.md", "G1-SLICE-checked-recipes.md"):
            original = API["blob"](ROOT, revision[1].decode(), "docs/tasks/" + name, API["TASK_BYTES"])
            old_links += original.count(b"](G1-SLICE-journal.md")
        self.assertEqual(old_links, 21)
        for source in current.values():
            self.assertIsNone(re.search(rb"\]\(G1-SLICE-journal\.md(?:#[^()]*)?\)", source))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", action="store_true", help="run the fresh-sibling control against trusted committed0420 code")
    arguments = parser.parse_args()
    if arguments.baseline:
        module = types.ModuleType("stitchcad_d281_baseline")
        module.__file__ = str(READER.resolve())
        sys.modules[module.__name__] = module
        try:
            source = API["git"](ROOT, ["show", "7b3736f:docs/tasks/artifacts/live_doc_size/maintained_reference.py"], 65536)
            label = "<committed7b3736f:maintained_reference.py>"
            exec(compile(source, label, "exec"), module.__dict__)
        finally:
            del sys.modules[module.__name__]
        CHECK_API = module.__dict__
        suite = unittest.TestSuite(TaskSources(name) for name in (
            "test_fresh_linked_sibling_owner_authorizes_the_exact_candidate",
            "test_completed_sibling_owner_cannot_be_laundered_into_the_current_root"))
    else:
        suite = unittest.defaultTestLoader.loadTestsFromTestCase(TaskSources)
    raise SystemExit(not unittest.TextTestRunner(verbosity=2).run(suite).wasSuccessful())
