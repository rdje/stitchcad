"""D278 red/green through native wc and the real checker; only ordinary target fixtures are written."""
from pathlib import Path
import argparse
import os
import runpy
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[4]
API = runpy.run_path(str(Path(__file__).with_name("maintained_reference.py")))
CHECK = ROOT / "scripts/check_live_doc_size.sh"


def require(value, detail):
    if not value:
        raise RuntimeError(detail)


def fixture(work, name, part="ok.md"):
    root = work / name
    (root / "reg").mkdir(parents=True)
    (root / "ok.md").write_text("normal owned body\n")
    fields = ["one", part, "file", "bounded_snapshot", "G1-SLICE.6.5.book.2", "COMMIT.md",
              "1", "18", "17", "-", "10", "200", "100", "20", "400", "200", "-", "-", "-", "-", "scope probe"]
    (root / "reg/surfaces.tsv").write_text("\t".join(fields) + "\n")
    (root / "reg/routes.tsv").write_text("")
    (root / "tracked.txt").write_text(part + "\n")
    return root


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", action="store_true", help="also reproduce the committed0dc4bc7 source in a target copy")
    parser.add_argument("--refusals-only", action="store_true", help="input guards before book-control bootstrap")
    arguments = parser.parse_args()
    base = ROOT / "target/scratch/measurement_scope_contract"
    base.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="scope-", dir=base) as temporary:
        work = Path(temporary)
        (work / "bin").mkdir()
        marker = work / "wc-called"
        wrapper = work / "bin/wc"
        wrapper.write_text('#!/usr/bin/env bash\nprintf "native-wc-called\\n" >> "$SCOPE_WC_MARKER"\nexec /usr/bin/wc "$@"\n')
        wrapper.chmod(0o700)
        def run(checker, root, *, extra=None, selftest=False):
            if marker.exists():
                marker.unlink()
            environment = dict(os.environ, LIVE_DOC_SIZE_ROOT=str(root), LIVE_DOC_SIZE_REGISTRY="reg",
                               LIVE_DOC_SIZE_TRACKED=str(root / "tracked.txt"), SCOPE_WC_MARKER=str(marker),
                               PATH=str(work / "bin") + os.pathsep + os.environ.get("PATH", ""))
            if extra:
                environment.update(extra)
            result = subprocess.run(["bash", str(checker), *(["--self-test"] if selftest else [])],
                                    cwd=ROOT, env=environment, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                    timeout=90, check=False)
            return result, marker.exists()
        leak = work / "leak.md"
        leak.write_text("Body must stay outside the numeric fixture.\n")
        escaped = fixture(work, "escape", "../leak.md")
        if arguments.baseline:
            saved = work / "original.sh"
            saved.write_bytes(API["git"](ROOT, ["show", "0dc4bc7:scripts/check_live_doc_size.sh"], 65536))
            result, called = run(saved, escaped)
            require(result.returncode == 0 and called, f"baseline did not reproduce owned-fixture escape: {result.stdout[-1200:]!r}")
            print("original0dc4bc7: escaped ordinary-fixture part reached native wc and passed rc=0")
        guarded = 0
        result, called = run(CHECK, escaped)
        require(result.returncode == 1 and not called and b"REFUSED input scope" in result.stdout,
                f"escaped part was measured: {result.stdout[-1200:]!r}")
        guarded += 1
        print("guard: escaped part refused before native wc rc=1")
        for name, kind in (("link", "link"), ("fifo", "fifo")):
            root = fixture(work, name, "part.md")
            if kind == "link":
                (root / "part.md").symlink_to(ROOT / "README.md")
            elif hasattr(os, "mkfifo"):
                os.mkfifo(root / "part.md")
            else:
                continue
            result, called = run(CHECK, root)
            require(result.returncode == 1 and not called, f"unsafe {name} part reached wc: {result.stdout[-1200:]!r}")
            guarded += 1
        marked = fixture(work, "marker")
        (marked / ".git").write_text("invalid marker-only directory; not a Git repository\n")
        result, called = run(CHECK, marked)
        require(result.returncode == 2 and not called and not (marked / "target").exists(),
                "marker-only workspace was entered or changed")
        guarded += 1
        outside = ROOT.parent / ("scope-refusal-" + work.name)
        require(not outside.exists(), "outside refusal sentinel already exists")
        result, called = run(CHECK, outside)
        require(result.returncode == 2 and not called and not outside.exists(), "outside workspace was accessed for writes")
        guarded += 1
        if arguments.refusals_only:
            print(f"scope: {guarded} pre-bootstrap refusal cases passed rc=0")
            return 0
        good = fixture(work, "good")
        result, called = run(CHECK, good)
        require(result.returncode == 0 and called and b"maintained-reference: OK" in result.stdout,
                f"valid ordinary fixture/full canonical book did not pass: {result.stdout[-1800:]!r}")
        guarded += 1
        print("guard: regular source measured; canonical book check also ran rc=0")
        # Redirected metadata belongs to this same repo; no other Git repository is initialized/amended.
        redirected = work / "redirected-worktree"
        redirected.mkdir()
        environment = dict(os.environ, GIT_DIR=str(ROOT / ".git"), GIT_WORK_TREE=str(redirected),
                           GIT_INDEX_FILE=str(work / "not-an-index"), GIT_CONFIG_COUNT="1",
                           GIT_CONFIG_KEY_0="invalid-no-section", GIT_CONFIG_VALUE_0="untrusted")
        for key in ("LIVE_DOC_SIZE_ROOT", "LIVE_DOC_SIZE_REGISTRY", "LIVE_DOC_SIZE_TRACKED"):
            environment.pop(key, None)
        result = subprocess.run(["bash", str(CHECK)], cwd=ROOT, env=environment,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=90, check=False)
        require(result.returncode == 0 and b"maintained-reference: OK" in result.stdout,
                f"canonical checker under caller Git overrides refused: {result.stdout[-1600:]!r}")
        require(not (redirected / "target").exists() and not (work / "not-an-index").exists(),
                "caller-selected metadata/worktree was written")
        guarded += 1
        print(f"scope: {guarded} guarded cases passed; isolated Git and native-wc before/after proof rc=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
