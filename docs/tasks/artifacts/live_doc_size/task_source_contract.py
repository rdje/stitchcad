"""Non-book canonical task/history preservation, independent of retired book growth authority."""
from pathlib import Path
from collections import Counter
import hashlib
import re
import runpy
import unittest

ROOT = Path(__file__).resolve().parents[4]
API = runpy.run_path(str(Path(__file__).with_name("maintained_reference.py")))
OWNER = "G1-SLICE.5f.2.dependencies.geometry.bind.book-capacity"

def blob(root, revision, relative, maximum):
    API["portable"](relative)
    API["need"](re.fullmatch(r"[0-9a-f]{40}|[0-9a-f]{64}", revision) is not None,
                "git", "historical revision identity")
    return API["git"](root, ["show", revision + ":" + relative], maximum)

API["blob"] = blob

class TaskHistory(unittest.TestCase):
    def test_paired_branch_retains_exact_native_source_and_unique_live_nodes(self):
        history = runpy.run_path(str(ROOT / "scripts/history_archive.py"))
        record = history["inventory"](ROOT)[0]["docs/history/stitchcad-devnotes-part650.md"]
        header, body = record.split(b"\n---\n\n", 1)
        revision = re.search(rb"from ([0-9a-f]{40}|[0-9a-f]{64})\.", header)
        extent = re.search(rb"Whole source interval: docs/tasks/G1-SLICE\.md bytes ([0-9]+)" +
                           "–".encode() + rb"([0-9]+), ([0-9]+) bytes, sha256:([0-9a-f]{64})", header)
        self.assertIsNotNone(revision)
        self.assertIsNotNone(extent)
        original = API["blob"](ROOT, revision[1].decode(), "docs/tasks/G1-SLICE.md", API["TASK_BYTES"])
        begin, end, size, checksum = extent.groups()
        self.assertEqual(body, original[int(begin):int(end)])
        self.assertEqual(len(body), int(size))
        self.assertEqual(hashlib.sha256(body).hexdigest().encode(), checksum)
        current = API["current_tasks"](ROOT, OWNER)
        self.assertIn("docs/tasks/G1-SLICE-execution.md", current)
        prefix = rb"^- ID: `G1-SLICE\.6\.5\.3\.2(?:\.|`)"
        self.assertIsNotNone(re.search(prefix, original, re.M), "native predecessor makes absence control red")
        self.assertIsNone(re.search(prefix, current["docs/tasks/G1-SLICE.md"], re.M))
        old_ids = Counter(re.findall(rb"^- ID: `([^`]+)`", original, re.M))
        live_ids = Counter(identity for source in current.values()
                           for identity in re.findall(rb"^- ID: `([^`]+)`", source, re.M))
        self.assertFalse(old_ids - live_ids, "every original root node remains live")
        nodes = re.findall(rb"^- ID: `([^`]+)`\n  Status: `([^`]+)`", body, re.M)
        self.assertTrue(nodes)
        for identity, old_state in nodes:
            found = [state for source in current.values()
                     if (state := API["task_status"](source, identity.decode())) is not None]
            self.assertEqual(len(found), 1, identity)
            if old_state == b"done":
                self.assertEqual(found, ["done"], identity)
        live = Counter(identity for source in current.values()
                       for identity in re.findall(rb"^- ID: `(G1-SLICE\.6\.5\.3\.2(?:\.[^`]+)?)`", source, re.M))
        self.assertTrue(live)
        self.assertEqual(set(live.values()), {1}, "canonical paired identities never duplicate")

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
    unittest.main(verbosity=2)
