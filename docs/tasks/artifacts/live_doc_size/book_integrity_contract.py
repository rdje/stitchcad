"""Book growth is unrestricted; original structural/path/staged correctness checks still discriminate."""
from pathlib import Path
import runpy
import shutil
import unittest
import uuid

ROOT = Path(__file__).resolve().parents[4]
MODULE = Path(__file__).with_name("maintained_reference.py")
API = runpy.run_path(str(MODULE))
PROFILE = runpy.run_path(str(ROOT / "scripts/local_environment.py"))


class BookIntegrity(unittest.TestCase):
    def setUp(self):
        self.tree = ROOT / "target/book-integrity-contract" / uuid.uuid4().hex

    def tearDown(self):
        if self.tree.exists():
            shutil.rmtree(self.tree)

    def write(self, parts):
        files = {self.tree / API["SOURCE"] / name: body for name, body in parts.items()}
        dirs = {path.parent for path in files}
        PROFILE["prepare_producer"](ROOT, sources=(Path(__file__), MODULE),
                                    directories=tuple(dirs), output_files=tuple(files))
        for path, body in files.items():
            path.write_bytes(body)

    def test_growth_exceeds_every_former_book_limit_without_authority(self):
        # More chapters/index lines, deeper navigation, wider lines and larger parts/aggregate
        # than the retired policy. No receipt, baseline, registry or task is present in this tree.
        parts = {}
        index = []
        for n in range(150):
            indent = "  " * min(n, 5)
            index.append(f"{indent}- [{'chapter ' + 'x' * 300}](part{n}.md)\n")
            parts[f"part{n}.md"] = b"# Chapter\n" + b"x" * 500 + b"\n" + b"body\n" * 10000
        parts["SUMMARY.md"] = "".join(index).encode()
        self.write(parts)
        result = API["check"](tree=self.tree)
        self.assertEqual(result.files, 151)
        self.assertEqual(result.bytes, sum(map(len, parts.values())))
        self.assertGreater(result.bytes, 1200000)
        self.assertGreater(result.lines, 20000)
        first = self.tree / API["SOURCE"] / "part0.md"
        original = first.read_bytes()
        first.write_bytes(original.replace(b"body", b"edit", 1))
        self.assertEqual(API["check"](tree=self.tree).bytes, result.bytes,
                         "equal-size edits also need no growth authorization")

    def test_missing_unindexed_duplicate_and_escaping_chapters_refuse(self):
        self.write({"SUMMARY.md": b"[Chapter](one.md)\n", "one.md": b"# One\n"})
        base = self.tree / API["SOURCE"]
        for summary in (b"[Missing](absent.md)\n", b"[One](one.md)\n[Again](one.md)\n",
                        b"[Escape](../outside.md)\n", b"[Self](SUMMARY.md)\n"):
            with self.subTest(summary=summary):
                (base / "SUMMARY.md").write_bytes(summary)
                with self.assertRaises(API["Refusal"]):
                    API["check"](tree=self.tree)
        (base / "SUMMARY.md").write_bytes(b"[Chapter](one.md)\n")
        extra = base / "extra.md"
        PROFILE["plan_producer"](ROOT, sources=(Path(__file__), MODULE), output_files=(extra,))
        extra.write_text("Unindexed chapter\n")
        with self.assertRaises(API["Refusal"]):
            API["check"](tree=self.tree)

    def test_source_alias_and_non_utf8_refuse(self):
        self.write({"SUMMARY.md": b"[Chapter](one.md)\n", "one.md": b"\xff"})
        with self.assertRaises(API["Refusal"]) as error:
            API["check"](tree=self.tree)
        self.assertEqual(error.exception.code, "encoding")
        chapter = self.tree / API["SOURCE"] / "one.md"
        chapter.unlink()
        chapter.symlink_to(ROOT / "README.md")
        with self.assertRaises(API["Refusal"]) as error:
            API["check"](tree=self.tree)
        self.assertEqual(error.exception.code, "path")

    def test_staged_index_checks_original_objects_without_authority(self):
        actual = API["check_index"]()
        self.assertGreater(actual.files, 1)
        path = (API["SOURCE"] + "/one.md").encode()
        oid = b"0" * 40
        regular = b"100644 " + oid + b" 0\t" + path + b"\0"
        self.assertEqual(API["staged_entries"](regular), {path.decode(): oid.decode()})
        for listing in (regular + regular, regular.replace(b" 0\t", b" 2\t"),
                        regular.replace(b"100644", b"120000"), regular[:-1]):
            with self.subTest(listing=listing):
                with self.assertRaises(API["Refusal"]):
                    API["staged_entries"](listing)


if __name__ == "__main__":
    unittest.main(verbosity=2)
