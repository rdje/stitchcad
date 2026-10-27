#!/usr/bin/env python3
"""D274: hold one real checker at native wc while another finishes in the same fixture root."""
from pathlib import Path
import argparse
import os
import signal
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[4]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checker", type=Path, default=ROOT / "scripts/check_live_doc_size.sh")
    checker = parser.parse_args().checker.resolve(strict=True)
    base = ROOT / "target/doctrine_scratch"
    base.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="live-size-overlap-", dir=base))
    (work / "registry").mkdir()
    (work / "barrier").mkdir()
    (work / "bin").mkdir()
    (work / "ok.md").write_text("tracked file\n", encoding="utf-8")
    (work / "tracked.txt").write_text("ok.md\n", encoding="utf-8")
    row = ["one", "ok.md", "file", "bounded_snapshot", "G1-SLICE.6.4.5.4.e.2",
           "COMMIT.md", "1", "13", "12", "-", "10", "200", "100", "20", "400",
           "200", "-", "-", "-", "-", "overlap contract"]
    (work / "registry/surfaces.tsv").write_text("\t".join(row) + "\n", encoding="utf-8")
    (work / "registry/routes.tsv").write_text("", encoding="utf-8")
    # This wrapper changes scheduling only. Both checks still use the OS byte/line oracle.
    wrapper = work / "bin/wc"
    wrapper.write_text('''#!/usr/bin/env bash
set -eu
if mkdir "$SIZE_OVERLAP_BARRIER/ready.$SIZE_OVERLAP_TOKEN" 2>/dev/null; then
  count=0
  while [ ! -f "$SIZE_OVERLAP_BARRIER/release.$SIZE_OVERLAP_TOKEN" ]; do
    count=$((count+1)); [ "$count" -lt 1500 ] || exit 124
    sleep 0.02
  done
fi
exec /usr/bin/wc "$@"
''', encoding="utf-8")
    wrapper.chmod(0o700)
    processes = []

    def start(token: str) -> subprocess.Popen:
        env = dict(os.environ, LIVE_DOC_SIZE_ROOT=str(work), LIVE_DOC_SIZE_REGISTRY="registry",
                   LIVE_DOC_SIZE_TRACKED=str(work / "tracked.txt"),
                   SIZE_OVERLAP_BARRIER=str(work / "barrier"), SIZE_OVERLAP_TOKEN=token,
                   PATH=str(work / "bin") + os.pathsep + os.environ.get("PATH", ""))
        process = subprocess.Popen(["bash", str(checker)],
                                   cwd=ROOT, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                   text=True, start_new_session=True)
        processes.append(process)
        deadline = time.monotonic() + 30
        while not (work / f"barrier/ready.{token}").is_dir():
            if process.poll() is not None or time.monotonic() >= deadline:
                raise RuntimeError(f"checker {token} failed to reach its scheduling barrier")
            time.sleep(0.02)
        return process

    try:
        first = start("first")
        second = start("second")
        (work / "barrier/release.second").touch()
        second_output, _ = second.communicate(timeout=30)
        (work / "barrier/release.first").touch()
        first_output, _ = first.communicate(timeout=30)
        for label, process, output in [("first", first, first_output), ("second", second, second_output)]:
            (work / f"{label}.log").write_text(output, encoding="utf-8")
        expected = "live-doc-size: OK — 1 surfaces, 0 routes, 1 files measured, 0 warning(s)"
        for label, process, output in [("first", first, first_output), ("second", second, second_output)]:
            if process.returncode != 0 or expected not in output:
                print(f"overlap: FAIL {label} rc={process.returncode}; evidence {work.relative_to(ROOT)}")
                print(output[-1800:])
                return 1
        leftovers = list((work / "target/doctrine_scratch").iterdir())
        if leftovers:
            raise RuntimeError("checker left invocation scratch after exit")
        print("overlap: 2 checker processes passed; exact1-file native measurements, owned scratch absent; rc=0")
        return 0
    finally:
        for process in processes:
            if process.poll() is None:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()


if __name__ == "__main__":
    raise SystemExit(main())
