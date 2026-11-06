#!/usr/bin/env bash
# docs/tasks/artifacts/canvas_spike/run_canvas_spike.sh
# G1-SLICE.13 — measures ADR-0002's topologies against its declared corpus, and writes a results file the verdict reads.
#
# WHY: the decision record wrote the corpus, the gates and the rule down before any measurement, so that the outcome
# cannot be argued after the fact. This is the measurement. Correctness comes from sc-viewport, and fidelity is read back
# from the pixels the renderer actually drew, because a number computed beside the renderer would measure nothing.
#
# WHAT A FRAME TIME MEANS HERE: the time until one frame's whole picture can be read back, for every renderer. A real frame
# needs no read, so this overestimates all of them, and it is the one point where each renderer has provably finished. A
# headless browser's animation clock runs near 60 Hz whatever the load, so frame intervals could not tell them apart.
#
# WHAT MEMORY MEANS HERE: the system's physical footprint of every process, as `top` reports it, not resident size. Chrome's
# processes share most of their pages, and adding resident sizes counts those pages once per process: on this machine
# 1312 MB resident against 711 MB of footprint for the same browser. It has its own visit with no read-backs, because reading
# a whole frame back at every frame leaves garbage a real frame would not, which took one candidate past 1.7 GB.
#
# HONEST LIMITS: one machine and one OS per run, and a verdict is scoped to them. Peak memory counts the whole browser's
# processes, which the page's own share cannot be separated from. Headless Chrome paints offscreen.
#
# Usage:  bash docs/tasks/artifacts/canvas_spike/run_canvas_spike.sh browser
#         bash docs/tasks/artifacts/canvas_spike/run_canvas_spike.sh native     (needs the host in spikes/canvas-host)
# Output: one row per topology in target/canvas_spike/<profile>.tsv, in results.tsv's columns. Exit 0 when each row was measured.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
if ! python3 -I -B scripts/local_environment.py --verify >/dev/null 2>&1; then
  exec python3 -I -B scripts/local_environment.py -- bash docs/tasks/artifacts/canvas_spike/run_canvas_spike.sh "$@"
fi
PROFILE_NAME="${1:-browser}"
mkdir -p target/canvas_spike
cargo build --release --target wasm32-unknown-unknown -p sc-viewer > target/canvas_spike/build-wasm.log 2>&1 \
  || { echo "canvas spike: the module does not build; see target/canvas_spike/build-wasm.log" >&2; exit 1; }
if [ "$PROFILE_NAME" = native ]; then
  cargo build --release --manifest-path spikes/canvas-host/Cargo.toml > target/canvas_spike/build-host.log 2>&1 \
    || { echo "canvas spike: the native host does not build; see target/canvas_spike/build-host.log" >&2; exit 1; }
fi

exec python3 -I -B - "$PROFILE_NAME" <<'PY'
import http.server
import json
import os
import pathlib
import platform
import shutil
import signal
import socketserver
import subprocess
import sys
import threading
import time

PROFILE = sys.argv[1]
ROOT = pathlib.Path.cwd()
STORE = ROOT / "target/canvas_spike"
SITE = STORE / "site"
WASM = ROOT / "target/wasm32-unknown-unknown/release/sc_viewer.wasm"
PAGE = ROOT / "docs/tasks/artifacts/canvas_spike/page.html"
HOST = ROOT / "target/release/canvas-host"
TIMEOUT = 300
COLUMNS = ["topology", "profile", "capable", "pick_correct", "snap_exact", "fidelity_max_px", "fidelity_drift_px",
           "frame_p95_ms", "peak_mb", "unproven_deps", "frame_median_ms", "cold_start_ms", "bundle_kb"]
# A count of dependencies nobody has shipped in this role, with the reasoning in results.tsv's comment block.
UNPROVEN = {("dom-canvas", "browser"): 0, ("webgpu-webview", "browser"): 1,
            ("dom-canvas", "native"): 1, ("webgpu-webview", "native"): 2, ("native-wgpu-composited", "native"): 3}


def fail(message):
    print("canvas spike: " + message, file=sys.stderr)
    sys.exit(1)


class Results:
    def __init__(self):
        self.lock = threading.Condition()
        self.value = None

    def post(self, value):
        with self.lock:
            self.value = value
            self.lock.notify_all()

    def wait(self):
        with self.lock:
            self.lock.wait_for(lambda: self.value is not None, timeout=TIMEOUT)
            value, self.value = self.value, None
            return value


RESULTS = Results()


class Handler(http.server.BaseHTTPRequestHandler):
    def log_message(self, *arguments):
        pass

    def do_GET(self):
        name = self.path.split("?")[0].lstrip("/") or "page.html"
        target = SITE / name
        if not target.is_file():
            self.send_error(404)
            return
        body = target.read_bytes()
        self.send_response(200)
        kinds = {".wasm": "application/wasm", ".html": "text/html; charset=utf-8"}
        self.send_header("Content-Type", kinds.get(target.suffix, "application/octet-stream"))
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_POST(self):
        length = int(self.headers.get("Content-Length", "0"))
        RESULTS.post(json.loads(self.rfile.read(length).decode("utf-8")))
        self.send_response(204)
        self.end_headers()


class Server(socketserver.ThreadingMixIn, http.server.HTTPServer):
    daemon_threads = True


def browser():
    for name in ("google-chrome", "chromium", "chrome"):
        found = shutil.which(name)
        if found:
            return found
    darwin = pathlib.Path("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    if platform.system() == "Darwin" and darwin.is_file():
        return str(darwin)
    fail("no Chrome is installed here")


def kilobytes(text):
    text = text.rstrip("+-")
    units = {"K": 1, "M": 1024, "G": 1024 * 1024, "B": 0}
    try:
        return int(float(text[:-1]) * units[text[-1]]) if text[-1] in units else int(text) // 1024
    except (ValueError, IndexError):
        return 0


# Physical footprint of a process group and, for a webview, its system-owned helper processes.
def resident_kb(group, helpers):
    if platform.system() == "Darwin":
        done = subprocess.run(["top", "-l", "1", "-stats", "pgrp,mem,command"], capture_output=True, text=True)
        rows = [line.split(None, 2) for line in done.stdout.splitlines()]
        return sum(kilobytes(parts[1]) for parts in rows if len(parts) == 3
                   and (parts[0] == str(group) or (helpers and "WebKit" in parts[2])))
    done = subprocess.run(["ps", "-axo", "pgid=,rss="], capture_output=True, text=True)
    rows = [line.split() for line in done.stdout.splitlines()]
    return sum(int(parts[1]) for parts in rows if len(parts) == 2 and parts[0] == str(group))


def memory_of(arguments, helpers):
    _, peak = measure(arguments, helpers)
    return peak


def measure(arguments, helpers):
    baseline = resident_kb(-1, helpers) if helpers else 0
    process = subprocess.Popen(arguments, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
    peak = 0
    stop = threading.Event()

    def sample():
        nonlocal peak
        while not stop.wait(0.2):
            peak = max(peak, resident_kb(process.pid, helpers) - baseline)

    thread = threading.Thread(target=sample, daemon=True)
    thread.start()
    try:
        return RESULTS.wait(), peak
    finally:
        stop.set()
        thread.join()
        # A process that already exited leaves only a zombie, and macOS answers that with a permission error.
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except (ProcessLookupError, PermissionError):
            pass
        process.wait()


def row(topology, result, peak_kb, bundle_bytes):
    if result is None or "failure" in result:
        reason = "no report within the time limit" if result is None else result["failure"]
        print("%s/%s: not capable: %s" % (topology, PROFILE, reason))
        return {"topology": topology, "profile": PROFILE, "capable": "no", "pick_correct": "-", "snap_exact": "-",
                "fidelity_max_px": "-", "fidelity_drift_px": "-", "frame_p95_ms": "-", "peak_mb": "-",
                "unproven_deps": UNPROVEN[(topology, PROFILE)], "frame_median_ms": "-", "cold_start_ms": "-",
                "bundle_kb": "-"}
    measured = {"topology": topology, "profile": PROFILE, "capable": result["capable"],
                "pick_correct": result["pick_correct"], "snap_exact": result["snap_exact"],
                "fidelity_max_px": round(result["fidelity_max_px"], 4), "fidelity_drift_px": round(result["fidelity_drift_px"], 6),
                "frame_p95_ms": round(result["frame_p95_ms"], 2), "peak_mb": round(peak_kb / 1024),
                "unproven_deps": UNPROVEN[(topology, PROFILE)], "frame_median_ms": round(result["frame_median_ms"], 2),
                "cold_start_ms": round(result["cold_start_ms"], 1), "bundle_kb": round(bundle_bytes / 1024)}
    print("%s/%s: picks %s, snaps %s, fidelity %.4f px, drift %s, p95 %.2f ms, median %.2f ms, peak %d MB%s"
          % (topology, PROFILE, result.get("pick_right"), result.get("snap_right"), result["fidelity_max_px"],
             result["fidelity_drift_px"], result["frame_p95_ms"], result["frame_median_ms"], peak_kb / 1024,
             (", adapter " + result["adapter"]) if result.get("adapter") else ""))
    return measured


def main():
    if SITE.exists():
        shutil.rmtree(SITE)
    SITE.mkdir(parents=True)
    shutil.copy(WASM, SITE / "sc_viewer.wasm")
    shutil.copy(PAGE, SITE / "page.html")
    bundle = WASM.stat().st_size + PAGE.stat().st_size
    server = Server(("127.0.0.1", 0), Handler)
    port = server.server_address[1]
    threading.Thread(target=server.serve_forever, daemon=True).start()
    rows = []
    if PROFILE == "browser":
        executable = browser()
        version = subprocess.run([executable, "--version"], capture_output=True, text=True, timeout=30).stdout.strip()
        print("browser: " + version)
        for topology in ("dom-canvas", "webgpu-webview"):
            profile = STORE / ("profile-" + topology)
            if profile.exists():
                shutil.rmtree(profile)
            profile.mkdir()
            def arguments(mode):
                listed = [executable, "--headless=new", "--no-first-run", "--no-default-browser-check",
                          "--disable-background-networking", "--disable-component-update", "--disable-sync",
                          "--disable-crash-reporter", "--disable-breakpad", "--enable-unsafe-webgpu",
                          "--user-data-dir=" + str(profile), "--disk-cache-dir=" + str(profile) + "-cache",
                          "http://127.0.0.1:%d/page.html?topology=%s&mode=%s" % (port, topology, mode)]
                if os.environ.get("CI"):
                    listed.insert(1, "--no-sandbox")
                return listed
            result, _ = measure(arguments("timing"), False)
            peak = memory_of(arguments("memory"), False)
            rows.append(row(topology, result, peak, bundle))
    else:
        for topology in ("dom-canvas", "webgpu-webview", "native-wgpu-composited"):
            helpers = topology != "native-wgpu-composited"
            base = "http://127.0.0.1:%d" % port
            result, _ = measure([str(HOST), topology, base, "timing"], helpers)
            peak = memory_of([str(HOST), topology, base, "memory"], helpers)
            rows.append(row(topology, result, peak, HOST.stat().st_size))
    server.shutdown()
    lines = ["\t".join(str(item[column]) for column in COLUMNS) for item in rows]
    (STORE / (PROFILE + ".tsv")).write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("canvas spike: %d rows measured for the %s profile" % (len(rows), PROFILE))


main()
PY
