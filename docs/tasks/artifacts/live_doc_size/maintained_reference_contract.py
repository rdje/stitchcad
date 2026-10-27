"""Focused schema/navigation/fresh-candidate controls; native Git is read-only in this repository.

No temporary Git repository is initialized. Candidate trees are ordinary owned directories under
this project's target/. Committed-mode integration is deliberately reserved for atomic adoption.
"""
from __future__ import annotations

from dataclasses import asdict
import hashlib
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[4]
MODULE = ROOT / "docs/tasks/artifacts/live_doc_size/maintained_reference.py"
API = runpy.run_path(str(MODULE))
Refusal = API["Refusal"]
OWNER = "G1-SLICE.probe0416"


class BookFixture(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.head = subprocess.check_output(["git", "-C", str(ROOT), "rev-parse", "HEAD"], text=True).strip()
        cls.baseline = API["git_snapshot"](ROOT, cls.head)
        # Refuse an unsafe source before copying any Markdown into ordinary candidate directories.
        API["filesystem_snapshot"](ROOT)
        cls.parts = {p.relative_to(ROOT).as_posix(): p.read_bytes()
                     for p in (ROOT / API["SOURCE"]).rglob("*.md")}
        cls.registry = (ROOT / API["REGISTRY"]).read_text()
        cls.tasks = (ROOT / "docs/tasks/G1-SLICE.md").read_text()
        need_absent = f'- ID: `{OWNER}`'
        if need_absent in cls.tasks:
            raise RuntimeError("fixture-only task id unexpectedly entered the maintained task tree")

    def setUp(self):
        parent = ROOT / "target/scratch/maintained_reference"
        parent.mkdir(parents=True, exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(prefix="reader-", dir=parent)
        self.addCleanup(self.temporary.cleanup)
        self.tree = Path(self.temporary.name)
        for name, data in self.parts.items():
            self.write(name, data)
        rows = []
        for line in self.registry.splitlines():
            if line.startswith("book_collection\t"):
                fields = line.split("\t")
                fields[3] = "maintained_reference"
                fields[16:19] = ["-", "-", "-"]
                line = "\t".join(fields)
            rows.append(line)
        self.write(API["REGISTRY"], ("\n".join(rows) + "\n").encode())
        task_fixture = (f"# Candidate task tree\n\n- Tree ID: `G1-SLICE`\n\n"
                        f"- ID: `{OWNER}`\n  Status: `done`\n  Goal: fixture-only fresh owner.\n"
                        "- ID: `G1-SLICE.6.4.6`\n  Status: `done`\n  Goal: already closed baseline owner.\n")
        self.write("docs/tasks/G1-SLICE.md", task_fixture.encode())
        intro = API["SOURCE"] + "/introduction.md"
        self.write(intro, self.parts[intro] + b"\nReader candidate contract probe.\n")
        target = API["filesystem_snapshot"](self.tree)
        self.document = {
            "schema_version": 1, "max_records": 1, "max_bytes": API["CONTROL_BYTES"],
            "max_record_bytes": API["RECORD_BYTES"], "references": [{
                "surface": "book_collection", "audience": "Book readers and authors",
                "rationale": "Unique maintained product reference; semantic parts and complete index",
                "navigation_depth": 3, "authority": {
                    "task": OWNER, "reason": "Focused fresh-candidate contract fixture",
                    "base_commit": self.head, "baseline": asdict(self.baseline), "target": asdict(target),
                    "delta": {key: getattr(target, key) - getattr(self.baseline, key)
                              for key in ("files", "lines", "bytes")},
                },
            }],
        }
        self.save()

    def write(self, name: str, data: bytes):
        path = self.tree / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)

    def encoded(self):
        header = {key: value for key, value in self.document.items() if key != "references"}
        return ("\n".join(json.dumps(value, sort_keys=True) for value in
                         [header, *self.document["references"]]) + "\n").encode()

    def save(self):
        self.write(API["CONTROL"], self.encoded())

    def refused(self, code: str):
        with self.assertRaises(Refusal) as result:
            API["check"](ROOT, self.tree)
        self.assertEqual(result.exception.code, code, str(result.exception))



class Candidate(BookFixture):
    def test_candidate_matches_native_counts_and_reader_is_read_only(self):
        paths = sorted(self.tree.rglob("*"))
        before = {p.relative_to(self.tree).as_posix(): (p.stat().st_mtime_ns, hashlib.sha256(p.read_bytes()).digest())
                  for p in paths if p.is_file()}
        status = subprocess.check_output(["git", "-C", str(ROOT), "status", "--porcelain"],
                                         env=dict(os.environ, GIT_OPTIONAL_LOCKS="0"))
        result = API["check"](ROOT, self.tree)
        self.assertEqual(result.mode, "candidate")
        self.assertEqual(result.owner, OWNER)
        native_lines = native_bytes = files = 0
        for p in paths:
            if p.is_file() and p.suffix == ".md" and API["SOURCE"] in p.relative_to(self.tree).as_posix():
                data = p.read_bytes()
                native_lines += int(subprocess.check_output(["wc", "-l"], input=data))
                native_bytes += int(subprocess.check_output(["wc", "-c"], input=data))
                files += 1
        self.assertEqual((result.snapshot.files, result.snapshot.lines, result.snapshot.bytes),
                         (files, native_lines, native_bytes))
        after = {p.relative_to(self.tree).as_posix(): (p.stat().st_mtime_ns, hashlib.sha256(p.read_bytes()).digest())
                 for p in self.tree.rglob("*") if p.is_file()}
        self.assertEqual(before, after)
        self.assertEqual(status, subprocess.check_output(["git", "-C", str(ROOT), "status", "--porcelain"],
                                                        env=dict(os.environ, GIT_OPTIONAL_LOCKS="0")))

    def test_caller_git_context_cannot_redirect_native_authority_or_change_parent_environment(self):
        injected = {
            "GIT_DIR": str(self.tree / "not-a-git-directory"),
            "GIT_WORK_TREE": str(self.tree),
            "GIT_INDEX_FILE": str(self.tree / "not-an-index"),
            "GIT_OBJECT_DIRECTORY": str(self.tree / "not-an-object-store"),
            "GIT_NAMESPACE": "untrusted-probe-namespace",
            "GIT_CONFIG_COUNT": "1", "GIT_CONFIG_KEY_0": "invalid-no-section",
            "GIT_CONFIG_VALUE_0": "must not reach Git",
        }
        with patch.dict(os.environ, injected):
            before = dict(os.environ)
            self.assertEqual(API["check"](ROOT, self.tree).mode, "candidate")
            self.assertEqual(dict(os.environ), before)
        self.assertFalse((self.tree / "not-an-index").exists())
        self.assertFalse((self.tree / "not-a-git-directory").exists())
        self.assertFalse((self.tree / "not-an-object-store").exists())

    def test_cli_accepts_only_the_explicit_project_local_candidate(self):
        command = [sys.executable, "-I", "-B", str(MODULE), "--candidate"]
        good = subprocess.run(command + [self.tree.relative_to(ROOT).as_posix()], cwd=ROOT,
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
        self.assertEqual(good.returncode, 0, good.stdout + good.stderr)
        self.assertIn(b"maintained-reference: OK", good.stdout)
        for path in ("../outside", str(self.tree), "target/../outside"):
            with self.subTest(path=path):
                bad = subprocess.run(command + [path], cwd=ROOT, stdout=subprocess.PIPE,
                                     stderr=subprocess.PIPE, check=False)
                self.assertEqual(bad.returncode, 2, bad.stdout + bad.stderr)
                self.assertIn(b"maintained-reference: path:", bad.stdout)

    def test_closed_schema_rejects_boolean_numbers_unknown_fields_and_excess_capacity(self):
        original = json.loads(json.dumps(self.document))
        changes = [("max_records", 0), ("max_records", 2), ("max_bytes", 8193),
                   ("max_record_bytes", 6145), ("schema_version", True), ("unknown", 1)]
        for field, value in changes:
            with self.subTest(field=field, value=value):
                self.document = json.loads(json.dumps(original))
                self.document[field] = value
                self.save()
                self.refused("schema")
        for value in ([], [original["references"][0], original["references"][0]]):
            self.document = json.loads(json.dumps(original))
            self.document["references"] = value
            self.save()
            self.refused("schema")

    def test_duplicate_json_and_malformed_encoding_are_refused(self):
        good = self.encoded()
        for bad in (b'{"schema_version":1,' + good[1:], b"\xff", b"[" * 3000):
            with self.subTest(prefix=bad[:20]):
                self.write(API["CONTROL"], bad)
                self.refused("schema")

    def test_control_and_record_string_budgets_are_not_decorative(self):
        self.write(API["CONTROL"], b" " * (API["CONTROL_BYTES"] + 1))
        self.refused("budget")
        self.document["references"][0]["audience"] = "x" * 257
        self.save()
        self.refused("schema")
        self.document["references"][0]["audience"] = "\ud800"
        self.save()
        self.refused("schema")

    def test_frozen_parts_index_and_lifecycle_cannot_be_reclassified_without_guard(self):
        path = self.tree / API["REGISTRY"]
        good = path.read_bytes()
        for row, field, value in (("book_collection", 13, "701"), ("book_summary", 14, "8193"),
                                  ("book_collection", 3, "partitioned_canonical"),
                                  ("book_collection", 16, "80"), ("book_collection", 18, "1200000")):
            with self.subTest(row=row, field=field):
                output = []
                changed = 0
                for line in good.decode().splitlines():
                    fields = line.split("\t")
                    if fields[0] == row:
                        fields[field] = value
                        line = "\t".join(fields)
                        changed += 1
                    output.append(line)
                self.assertEqual(changed, 1)
                path.write_text("\n".join(output) + "\n")
                self.refused("profile")
        path.write_bytes(good)
        self.document["references"][0]["navigation_depth"] = 4
        self.save()
        self.refused("schema")

    def test_raw_record_bytes_are_bounded_before_json_normalization(self):
        header, record, _ = self.encoded().split(b"\n")
        padded = record + b" " * (API["RECORD_BYTES"] + 1 - len(record))
        self.write(API["CONTROL"], header + b"\n" + padded + b"\n")
        self.refused("budget")
        # Inclusive raw-record boundary: padding is data-plane whitespace, never future headroom.
        self.write(API["CONTROL"], header + b"\n" + padded[:-1] + b"\n")
        self.assertEqual(API["check"](ROOT, self.tree).mode, "candidate")

    def replace_book(self, count: int, body: bytes):
        shutil.rmtree(self.tree / API["SOURCE"])
        index = "# Summary\n\n" + "".join(f"- [Part {n}](p{n:03}.md)\n" for n in range(count))
        self.write(API["INDEX"], index.encode())
        for number in range(count):
            self.write(f"{API['SOURCE']}/p{number:03}.md", body)
        target = API["filesystem_snapshot"](self.tree)
        authority = self.document["references"][0]["authority"]
        authority["target"] = asdict(target)
        authority["delta"] = {key: getattr(target, key) - getattr(self.baseline, key)
                              for key in ("files", "lines", "bytes")}
        self.save()
        return target

    def test_exact_fresh_reference_can_exceed_legacy_aggregate_lines_and_bytes(self):
        target = self.replace_book(32, (b"a" * 56 + b"\n") * 700)
        self.assertGreater(target.lines, 20000)
        self.assertGreater(target.bytes, 1200000)
        self.assertEqual(API["check"](ROOT, self.tree).snapshot, target)

    def test_complete_bounded_index_can_authorize_more_than_legacy_eighty_files(self):
        target = self.replace_book(90, b"# Fixed chapter\n")
        self.assertEqual(target.files, 91)
        self.assertEqual(API["check"](ROOT, self.tree).snapshot, target)

    def test_signed_deltas_target_counts_and_content_must_all_match(self):
        authority = self.document["references"][0]["authority"]
        authority["delta"]["bytes"] += 1
        self.save()
        self.refused("delta")
        authority["target"]["bytes"] += 1
        self.save()
        self.refused("target")

    def test_equal_size_substitution_does_not_reuse_authority(self):
        path = self.tree / API["SOURCE"] / "introduction.md"
        original = path.read_bytes()
        self.assertIn(b"Reader candidate", original)
        path.write_bytes(original.replace(b"Reader candidate", b"reader candidate", 1))
        self.assertEqual(path.stat().st_size, len(original))
        self.refused("target")

    def test_banked_headroom_and_native_baseline_substitution_are_refused(self):
        authority = self.document["references"][0]["authority"]
        authority["target"]["bytes"] += 100
        authority["delta"]["bytes"] += 100
        self.save()
        self.refused("target")
        authority["target"]["bytes"] -= 100
        authority["delta"]["bytes"] -= 100
        authority["baseline"]["tree_sha256"] = "0" * 64
        self.save()
        self.refused("baseline")

    def test_stale_baseline_and_previously_closed_owner_cannot_authorize_new_content(self):
        authority = self.document["references"][0]["authority"]
        authority["base_commit"] = subprocess.check_output(["git", "-C", str(ROOT), "rev-parse", "HEAD^"], text=True).strip()
        self.save()
        self.refused("freshness")
        authority["base_commit"] = self.head
        authority["task"] = "G1-SLICE.6.4.6"
        self.save()
        self.refused("task")

    def test_unfinished_missing_and_duplicate_resulting_owners_refuse(self):
        path = self.tree / "docs/tasks/G1-SLICE.md"
        good = path.read_text()
        marker = f'- ID: `{OWNER}`\n  Status: `done`'
        for changed in (good.replace(marker, marker.replace("done", "in_progress")),
                        good.replace(marker, ""), good + "\n" + marker + "\n"):
            with self.subTest(change=len(changed)):
                path.write_text(changed)
                self.refused("task")

    def test_complete_navigation_rejects_extra_missing_duplicate_and_escaped_chapters(self):
        index = self.tree / API["INDEX"]
        good = index.read_text()
        for changed in (good + "\n- [Duplicate](introduction.md)\n",
                        good.replace("(introduction.md)", "(../outside.md)", 1),
                        good.replace("(introduction.md)", "(https://example.test/x.md)", 1),
                        good.replace("(introduction.md)", "(intro%2ed.md)", 1),
                        good + "\n      - [Too deep](deep.md)\n",
                        good.replace("[Introduction](introduction.md)", "")):
            with self.subTest(change=len(changed)):
                index.write_text(changed)
                with self.assertRaises(Refusal) as result:
                    API["check"](ROOT, self.tree)
                self.assertIn(result.exception.code, ("navigation", "path"))
        index.write_text(good)
        extra = self.tree / API["SOURCE"] / "unindexed.md"
        extra.write_text("# Unindexed\n")
        self.refused("navigation")
        extra.unlink()
        (self.tree / API["SOURCE"] / "introduction.md").unlink()
        self.refused("navigation")

    def test_oversized_parts_and_index_refuse_before_content_authority(self):
        index = self.tree / API["INDEX"]
        original = index.read_bytes()
        index.write_bytes(b" " * (API["INDEX_BOUNDS"][1] + 1))
        self.refused("budget")
        index.write_bytes(original)
        path = self.tree / API["SOURCE"] / "introduction.md"
        path.write_bytes(b"x" * (API["PART_BOUNDS"][1] + 1))
        self.refused("budget")
        path.write_bytes(b"x" * (API["PART_BOUNDS"][2] + 1) + b"\n")
        self.refused("part")
        path.write_bytes(b"\n" * (API["PART_BOUNDS"][0] + 1))
        self.refused("part")

    def test_links_shared_files_and_special_sources_refuse_without_following(self):
        path = self.tree / API["SOURCE"] / "introduction.md"
        original = path.read_bytes()
        path.unlink()
        path.symlink_to(ROOT / API["SOURCE"] / "introduction.md")
        self.refused("path")
        path.unlink()
        path.write_bytes(original)
        hold = self.tree / API["SOURCE"] / "held-data"
        os.link(path, hold)
        self.refused("path")
        hold.unlink()
        if hasattr(os, "mkfifo"):
            fifo = self.tree / API["SOURCE"] / "pipe.md"
            os.mkfifo(fifo)
            self.refused("path")

    def test_marker_only_candidate_boundary_is_refused_without_initializing_git(self):
        # A marker-only ordinary directory is not a Git repository; never run git init/commit here.
        self.write(".git", b"invalid marker-only boundary probe\n")
        self.refused("path")


class NativeContract(unittest.TestCase):
    def test_absent_optional_path_is_distinct_from_a_native_git_failure(self):
        head = subprocess.check_output(["git", "-C", str(ROOT), "rev-parse", "HEAD"], text=True).strip()
        self.assertIsNone(API["blob"](ROOT, head, "docs/book/src/probe_absent_0416.md", 64, absent=True))
        with self.assertRaises(Refusal) as result:
            API["blob"](ROOT, "0" * len(head), API["INDEX"], 8192, absent=True)
        self.assertEqual(result.exception.code, "git")
        with self.assertRaises(Refusal) as result:
            API["git"](ROOT, ["show", head + ":" + API["INDEX"]], 1)
        self.assertEqual(result.exception.code, "budget")
        parent = API["git"](ROOT, ["rev-parse", "--verify", head + "^1"], 128).decode().strip()
        expected = subprocess.check_output(["git", "-C", str(ROOT), "rev-list", "--parents", "-n", "1", head], text=True).split()[1]
        self.assertEqual(parent, expected)

    def test_watchdog_start_failure_refuses_and_reaps_the_actual_native_child(self):
        children = []
        native = API["subprocess"].Popen

        def capture(*arguments, **keywords):
            child = native(*arguments, **keywords)
            children.append(child)
            return child

        class BrokenTimer:
            def __init__(self, *_arguments):
                pass

            def start(self):
                raise RuntimeError("controlled watchdog startup failure")

            def cancel(self):
                pass

        with patch.object(API["subprocess"], "Popen", capture), patch.object(API["threading"], "Timer", BrokenTimer):
            with self.assertRaises(Refusal) as result:
                API["git"](ROOT, ["rev-parse", "HEAD"], 128)
            self.assertEqual(result.exception.code, "git")
        self.assertEqual(len(children), 1)
        self.assertIsNotNone(children[0].poll(), "actual owned native child must already be reaped")

    def test_native_manifest_rejects_duplicate_paths_links_and_submodule_entries(self):
        regular = b"100644 blob " + b"a" * 40 + b"\tdocs/book/src/intro.md\0"
        self.assertEqual(API["git_entries"](regular), ["docs/book/src/intro.md"])
        for record in (regular * 2, regular.replace(b"100644", b"120000"),
                       regular.replace(b"100644 blob", b"160000 commit"), b"malformed\0"):
            with self.subTest(record=record[:25]):
                with self.assertRaises(Refusal):
                    API["git_entries"](record)

    def test_fixed_hash_framing_matches_independent_native_sha256(self):
        parts = {"docs/book/src/SUMMARY.md": b"[Intro](intro.md)\n",
                 "docs/book/src/intro.md": b"# Intro\n"}
        # Hand-framed schema-1 vector; not generated through summarize or its framing helpers.
        wire = (b"StitchCAD-maintained-book-v1\0"
                b"24:docs/book/src/SUMMARY.md\0" b"18:[Intro](intro.md)\n\0"
                b"22:docs/book/src/intro.md\0" b"8:# Intro\n\0")
        if shutil.which("shasum"):
            command = ["shasum", "-a", "256"]
        else:
            self.assertIsNotNone(shutil.which("sha256sum"), "native SHA-256 oracle required")
            command = ["sha256sum"]
        expected = subprocess.check_output(command, input=wire).decode().split()[0]
        actual = API["summarize"](parts)
        self.assertEqual((actual.files, actual.lines, actual.bytes), (2, 2, 26))
        self.assertEqual(actual.tree_sha256, expected)


if __name__ == "__main__":
    unittest.main(verbosity=2)
