"""Native measurement/storage controls; all fixtures stay in this repo and Git is read-only."""
from pathlib import Path
import argparse
from contextlib import nullcontext
import os
import re
import runpy
import subprocess
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[4]
API = runpy.run_path(str(Path(__file__).with_name("maintained_reference.py")))
BASELINE = False


class Newlines(unittest.TestCase):
    def test_actual_awk_excludes_only_one_terminal_cr_and_wc_keeps_raw_bytes(self):
        script = (API["git"](ROOT, ["show", "03460b0:scripts/check_live_doc_size.sh"], 65536)
                  if BASELINE else (ROOT / "scripts/check_live_doc_size.sh").read_bytes())
        match = re.search(rb"m=\$\(awk '([^']+)' \"\$ROOT/\$p\"\)", script)
        self.assertIsNotNone(match, "measure the actual checker program, never a copied predicate")
        parent = ROOT / "target/scratch/newline_contract"
        parent.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix="native-", dir=parent) as temporary:
            source = Path(temporary) / "part.md"
            for body, expected in ((b"", 0), (b"x" * 440 + b"\n", 440),
                                   (b"x" * 440 + b"\r\n", 440),
                                   (b"x\ry\r\r\n", 4)):
                with self.subTest(body=body[-10:]):
                    source.write_bytes(body)
                    result = subprocess.run(["awk", match[1].decode(), str(source)],
                                            env=dict(os.environ, LC_ALL="C"), capture_output=True)
                    self.assertEqual(result.returncode, 0, result.stderr)
                    self.assertEqual(int(result.stdout), expected)
                    count = subprocess.check_output(["wc", "-c", str(source)])
                    self.assertEqual(int(count.split()[0]), len(body))

    def test_native_git_preserves_exact_governed_bytes_under_newline_settings(self):
        environment = {key: value for key, value in os.environ.items() if not key.startswith("GIT_")}
        environment.update(GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull,
                           GIT_CONFIG_SYSTEM=os.devnull, GIT_OPTIONAL_LOCKS="0")
        parent = ROOT / "target/scratch/newline_contract"
        parent.mkdir(parents=True, exist_ok=True)
        # Reproduce the original absent-attributes policy in an ordinary target directory, using
        # this same repository's Git metadata read-only. Never initialize or amend another repo.
        context = tempfile.TemporaryDirectory(prefix="old-storage-", dir=parent) if BASELINE else nullcontext(None)
        with context as baseline:
            for relative in (API["SOURCE"] + "/introduction.md", API["CONTROL"]):
                for conversion in ("false", "true", "input"):
                    for body in (b"Exact storage\n", b"Exact storage\r\n"):
                        with self.subTest(path=relative, conversion=conversion, body=body):
                            prefix = ["git", "-C", str(ROOT), "-c", "core.autocrlf=" + conversion,
                                      "-c", "core.eol=crlf"]
                            if baseline:
                                # hash-object resolves --path from its cwd, unlike check-attr's
                                # worktree-relative lookup. Keep both cwd and worktree in the fixture.
                                prefix += ["-C", baseline, "--git-dir", str(ROOT / ".git"),
                                           "--work-tree", baseline]
                            raw = subprocess.check_output(prefix + ["hash-object", "--stdin", "--no-filters"],
                                                          input=body, env=environment)
                            stored = subprocess.check_output(prefix + ["hash-object", "--stdin", "--path", relative],
                                                             input=body, env=environment, stderr=subprocess.PIPE)
                            self.assertEqual(raw, stored, "Git storage must match the authority's raw bytes")

    def test_effective_attributes_refuse_transformations_and_malformed_native_output(self):
        paths = {API["SOURCE"] + "/introduction.md", API["CONTROL"]}
        output = API["git"](ROOT, ["check-attr", "-z", "text", "filter", "ident",
                                   "working-tree-encoding", "--", *sorted(paths)], API["LISTING_BYTES"])
        API["storage_profile"](ROOT, paths)
        for changed in (output.replace(b"\0unset\0", b"\0set\0", 1), output[:-1], output + output):
            with self.subTest(changed=changed[:80]):
                with patch.dict(API["storage_profile"].__globals__, git=lambda *args, **kwargs: changed):
                    with self.assertRaises(API["Refusal"]) as result:
                        API["storage_profile"](ROOT, paths)
                    self.assertEqual(result.exception.code, "storage")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", action="store_true", help="reproduce committed03460b0 measurement")
    arguments = parser.parse_args()
    BASELINE = arguments.baseline
    # Select only original native controls for the before-state measurement.
    suite = (unittest.TestSuite(Newlines(name) for name in (
        "test_actual_awk_excludes_only_one_terminal_cr_and_wc_keeps_raw_bytes",
        "test_native_git_preserves_exact_governed_bytes_under_newline_settings"))
        if BASELINE else unittest.defaultTestLoader.loadTestsFromTestCase(Newlines))
    raise SystemExit(not unittest.TextTestRunner(verbosity=2).run(suite).wasSuccessful())
