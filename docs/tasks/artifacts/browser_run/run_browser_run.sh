#!/usr/bin/env bash
# docs/tasks/artifacts/browser_run/run_browser_run.sh
# G1-SLICE.12 — the real browser run: load a project, evaluate a recipe, solve one constraint, save and reopen, render,
# and export, in actual Chrome, across two separate browser processes.
#
# WHY: G1's exit asks for a real spike, "not compile-only". A compile proves nothing about IndexedDB, about what a
# page can actually read back from WebAssembly, or about whether a browser gives the same bytes as the native program.
#
# WHAT IT DOES, mechanically:
#   1. builds the wasm-viewer artifact and the native `stitchcad` program, then runs the two session files natively;
#   2. serves the artifact and a page from 127.0.0.1, so the page has an ordinary web origin, with no special headers;
#   3. starts Chrome and runs the first visit, which saves the project to IndexedDB;
#   4. ends that Chrome process and starts another on the same profile, which reopens from IndexedDB, renders to a real
#      canvas and an inline SVG, and exports through a Blob;
#   5. requires both visits' transcripts, the project image and the exported bytes to equal the native program's;
#   6. CONTROL: a third Chrome on a fresh profile must fail its reopen, or step 4 proved nothing about persistence;
#   7. writes the measurements to target/browser_run/latest.json, which docs/tasks/artifacts/browser_run/recorded-run.json
#      keeps a copy of for the book's capability matrix.
#
# HONEST LIMITS: one browser family on the machine that runs this. The drawing is a schematic of evaluated values,
# since drafting is G2. Headless Chrome paints to an offscreen canvas, so nobody has looked at the picture.
#
# Usage:  bash docs/tasks/artifacts/browser_run/run_browser_run.sh
# Output: one line per step, then `browser run: 6 steps passed in <browser>`. Exit 0 only when every step and the control hold.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
if ! python3 -I -B scripts/local_environment.py --verify >/dev/null 2>&1; then
  exec python3 -I -B scripts/local_environment.py -- bash docs/tasks/artifacts/browser_run/run_browser_run.sh "$@"
fi
mkdir -p target/browser_run
cargo build --release --target wasm32-unknown-unknown -p sc-viewer > target/browser_run/build-wasm.log 2>&1 \
  || { echo "browser run: the wasm-viewer artifact does not build; see target/browser_run/build-wasm.log" >&2; exit 1; }
cargo build -p sc-cli > target/browser_run/build-native.log 2>&1 \
  || { echo "browser run: the native program does not build; see target/browser_run/build-native.log" >&2; exit 1; }

exec python3 -I -B - <<'PY'
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

ROOT = pathlib.Path.cwd()
STORE = ROOT / "target/browser_run"
RUN = STORE / ("run-%d" % os.getpid())
SESSIONS = ROOT / "crates/sc-viewer/tests"
WASM = ROOT / "target/wasm32-unknown-unknown/release/sc_viewer.wasm"
PROGRAM = ROOT / "target/debug/stitchcad"
TIMEOUT = 120

PAGE = r"""<!doctype html>
<meta charset="utf-8">
<title>StitchCAD browser run</title>
<canvas id="canvas" width="600" height="900"></canvas>
<div id="drawing"></div>
<script>
const open_database = () => new Promise((resolve, reject) => {
  const request = indexedDB.open("stitchcad", 1);
  request.onupgradeneeded = () => request.result.createObjectStore("images");
  request.onsuccess = () => resolve(request.result);
  request.onerror = () => reject(request.error);
});
const read_image = (database) => new Promise((resolve, reject) => {
  const request = database.transaction("images").objectStore("images").get("project");
  request.onsuccess = () => resolve(request.result === undefined ? "" : request.result);
  request.onerror = () => reject(request.error);
});
// One transaction, so the browser's copy is the old image or the new one, never a mix.
const write_image = (database, image) => new Promise((resolve, reject) => {
  const transaction = database.transaction("images", "readwrite");
  transaction.objectStore("images").put(image, "project");
  transaction.oncomplete = () => resolve();
  transaction.onerror = () => reject(transaction.error);
});
const report = (body) => fetch("/result", {method: "POST", body: JSON.stringify(body)});

function paint(svg_text) {
  const holder = document.getElementById("drawing");
  holder.innerHTML = svg_text;
  const polygon = holder.querySelector("polygon");
  const box = holder.querySelector("svg").viewBox.baseVal;
  const canvas = document.getElementById("canvas");
  const scale = Math.min(canvas.width / box.width, canvas.height / box.height) * 0.9;
  const context = canvas.getContext("2d", {willReadFrequently: true});
  context.fillStyle = "white";
  context.fillRect(0, 0, canvas.width, canvas.height);
  context.beginPath();
  const points = polygon.getAttribute("points").trim().split(/\s+/).map((pair) => pair.split(",").map(Number));
  points.forEach(([x, y], at) => at === 0 ? context.moveTo(x * scale + 10, y * scale + 10) : context.lineTo(x * scale + 10, y * scale + 10));
  context.closePath();
  context.strokeStyle = "black";
  context.lineWidth = 2;
  context.stroke();
  const pixels = context.getImageData(0, 0, canvas.width, canvas.height).data;
  let dark = 0;
  for (let at = 0; at < pixels.length; at += 4) { if (pixels[at] < 128) dark += 1; }
  return {dark_pixels: dark, points: points.length};
}

async function main() {
  const visit = new URLSearchParams(location.search).get("visit");
  const started = performance.now();
  const wasm_bytes = await (await fetch("sc_viewer.wasm")).arrayBuffer();
  const session = await (await fetch(visit + ".session.json")).text();
  const fetched = performance.now();
  const module = await WebAssembly.compile(wasm_bytes);
  const imports = WebAssembly.Module.imports(module).length;
  const instance = await WebAssembly.instantiate(module, {});
  const instantiated = performance.now();
  const database = await open_database();
  const stored = await read_image(database);
  const exports = instance.exports;
  const feed = (slot, text) => {
    exports.sc_input_clear(slot);
    for (const byte of new TextEncoder().encode(text)) exports.sc_input_push(slot, byte);
  };
  feed(0, session);
  feed(1, stored);
  const ran_from = performance.now();
  exports.sc_run();
  const output_bytes = new Uint8Array(exports.sc_output_len());
  for (let at = 0; at < output_bytes.length; at += 1) output_bytes[at] = exports.sc_output_byte(at);
  const output = new TextDecoder().decode(output_bytes);
  const ran = performance.now();
  const result = JSON.parse(output);
  const facts = {
    visit, output, stored_bytes: stored.length, imports, wasm_bytes: wasm_bytes.byteLength,
    user_agent: navigator.userAgent, secure_context: isSecureContext, cross_origin_isolated: crossOriginIsolated,
    webgpu_present: Boolean(navigator.gpu), shared_array_buffer: typeof SharedArrayBuffer !== "undefined",
    fetch_ms: fetched - started, compile_ms: instantiated - fetched, run_ms: ran - ran_from,
  };
  if (result.failed === undefined) {
    await write_image(database, result.image);
    const drawn = result.transcript.find((step) => step.do === "render");
    if (drawn !== undefined) {
      facts.painted = paint(drawn.result);
      const blob = new Blob([drawn.result], {type: "image/svg+xml"});
      const link = document.createElement("a");
      link.download = "front-quadrant.svg";
      link.href = URL.createObjectURL(blob);
      document.body.append(link);
      facts.exported = await blob.text();
      facts.export_link = link.href.startsWith("blob:");
    }
  }
  await report(facts);
}
main().catch((error) => report({visit: "error", output: "", message: String(error)}));
</script>
"""


def fail(message):
    print("browser run: " + message, file=sys.stderr)
    sys.exit(1)


def browser():
    for name in ("google-chrome", "chromium", "chrome"):
        found = shutil.which(name)
        if found:
            return found
    darwin = pathlib.Path("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    if platform.system() == "Darwin" and darwin.is_file():
        return str(darwin)
    fail("no Chrome is installed here")


class Results:
    def __init__(self):
        self.lock = threading.Condition()
        self.latest = None

    def post(self, value):
        with self.lock:
            self.latest = value
            self.lock.notify_all()

    def wait(self):
        with self.lock:
            self.lock.wait_for(lambda: self.latest is not None, timeout=TIMEOUT)
            value, self.latest = self.latest, None
            return value


RESULTS = Results()


class Handler(http.server.BaseHTTPRequestHandler):
    def log_message(self, *arguments):
        pass

    def do_GET(self):
        name = self.path.split("?")[0].lstrip("/") or "index.html"
        target = RUN / "site" / name
        if not target.is_file():
            self.send_error(404)
            return
        body = target.read_bytes()
        self.send_response(200)
        kinds = {".wasm": "application/wasm", ".html": "text/html; charset=utf-8", ".json": "application/json"}
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


def visit(executable, profile, port, name):
    arguments = [executable, "--headless=new", "--no-first-run", "--no-default-browser-check",
                 "--disable-background-networking", "--disable-component-update", "--disable-sync",
                 "--disable-crash-reporter", "--disable-breakpad",
                 "--user-data-dir=" + str(profile), "--disk-cache-dir=" + str(profile.parent / (profile.name + "-cache")),
                 "http://127.0.0.1:%d/index.html?visit=%s" % (port, name)]
    # Hosted runners restrict the sandbox's namespaces, so CI runs without it. This page reads only its own origin.
    if os.environ.get("CI"):
        arguments.insert(1, "--no-sandbox")
    process = subprocess.Popen(arguments, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
    try:
        return RESULTS.wait()
    finally:
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        process.wait()
        # A killed browser can leave its profile lock behind, and the next visit must still open that profile.
        for name in ("SingletonLock", "SingletonSocket", "SingletonCookie"):
            (profile / name).unlink(missing_ok=True)


def native(name, project):
    done = subprocess.run([str(PROGRAM), "replay", str(SESSIONS / (name + ".session.json")), str(project)],
                          capture_output=True, text=True, timeout=TIMEOUT)
    if done.returncode != 0:
        fail("the native program failed on %s: %s" % (name, done.stdout))
    return json.loads(done.stdout)


def files_of(directory):
    return {path.relative_to(directory).as_posix(): path.read_bytes()
            for path in sorted(directory.rglob("*")) if path.is_file()}


def decoded(image_text):
    return {path: bytes.fromhex(text) for path, text in json.loads(image_text).items()}


def main():
    if RUN.exists():
        shutil.rmtree(RUN)
    (RUN / "site").mkdir(parents=True)
    (RUN / "site/index.html").write_text(PAGE, encoding="utf-8")
    shutil.copy(WASM, RUN / "site/sc_viewer.wasm")
    for name in ("first-visit", "second-visit"):
        shutil.copy(SESSIONS / (name + ".session.json"), RUN / "site" / (name + ".session.json"))

    project = RUN / "native-project"
    native_first = native("first-visit", project)
    first_files = files_of(project)
    native_second = native("second-visit", project)
    second_files = files_of(project)

    server = Server(("127.0.0.1", 0), Handler)
    port = server.server_address[1]
    threading.Thread(target=server.serve_forever, daemon=True).start()
    executable = browser()
    version = subprocess.run([executable, "--version"], capture_output=True, text=True, timeout=30).stdout.strip()
    profile = RUN / "profile"
    profile.mkdir()

    first = visit(executable, profile, port, "first-visit")
    if first is None or first.get("visit") != "first-visit":
        fail("the first visit did not report: %r" % (first,))
    result = json.loads(first["output"])
    if "failed" in result:
        fail("the first visit failed: %s" % first["output"])
    if first["imports"] != 0:
        fail("the page's module asks for %d imports" % first["imports"])
    if first["stored_bytes"] != 0:
        fail("a fresh profile already held a project")
    print("load: real Chrome compiled and instantiated the module, with no imports")
    if result["transcript"] != native_first:
        fail("the first visit's transcript differs from the native program's")
    steps = {step["do"]: step for step in result["transcript"]}
    print("evaluate: the recipe's values equal the native program's")
    if steps["solve"]["result"]["outcome"] != "satisfied":
        fail("the constraint did not solve")
    print("constraint: the constraint kernel solved in the browser, equal to native")
    if decoded(result["image"]) != first_files:
        fail("the saved project image differs from the native program's files")

    second = visit(executable, profile, port, "second-visit")
    if second is None or second.get("visit") != "second-visit":
        fail("the second visit did not report: %r" % (second,))
    result = json.loads(second["output"])
    if "failed" in result:
        fail("the second visit failed: %s" % second["output"])
    if second["stored_bytes"] == 0:
        fail("a second browser process found nothing in IndexedDB")
    if result["transcript"] != native_second:
        fail("the second visit's transcript differs from the native program's")
    steps = {step["do"]: step for step in result["transcript"]}
    waist = steps["evaluate"]["result"][0]
    if (waist["name"], waist["value"]) != ("garment_waist", "780000"):
        fail("the reopened project does not hold the first visit's edit: %r" % (waist,))
    print("save and reopen: a second browser process reopened the edit from IndexedDB")
    painted = second.get("painted") or {}
    if painted.get("points") != 5 or painted.get("dark_pixels", 0) < 200:
        fail("nothing was painted: %r" % (painted,))
    print("render: %d dark pixels on a real canvas, from the 5 points of the inline SVG" % painted["dark_pixels"])
    exported = second.get("exported", "").encode("utf-8")
    if not second.get("export_link") or exported != second_files["exports/front-quadrant.svg"]:
        fail("the browser's export differs from the native program's file")
    if decoded(result["image"]) != second_files:
        fail("the saved project image differs from the native program's files")
    print("export: the Blob's bytes equal the native program's exported file")

    # The control: on a fresh profile the same page finds nothing to reopen, so IndexedDB really did carry it.
    fresh = RUN / "fresh-profile"
    fresh.mkdir()
    control = visit(executable, fresh, port, "second-visit")
    if control is None or json.loads(control.get("output") or "{}").get("failed") != "reopen":
        fail("the control did not fail its reopen, so the run does not show that IndexedDB carried the project")
    print("control: a fresh profile has nothing to reopen, so IndexedDB carried the project")

    server.shutdown()
    record = {
        "browser": version,
        "user_agent": second["user_agent"],
        "os": platform.system() + " " + platform.machine(),
        "wasm_bytes": first["wasm_bytes"],
        "imports": first["imports"],
        "secure_context": first["secure_context"],
        "cross_origin_isolated": first["cross_origin_isolated"],
        "webgpu_present": first["webgpu_present"],
        "shared_array_buffer": first["shared_array_buffer"],
        "compile_ms": round(first["compile_ms"], 1),
        "first_run_ms": round(first["run_ms"], 1),
        "second_run_ms": round(second["run_ms"], 1),
        "image_bytes": second["stored_bytes"],
        "dark_pixels": painted["dark_pixels"],
    }
    (STORE / "latest.json").write_text(json.dumps(record, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    shutil.rmtree(RUN)
    print("browser run: 6 steps passed in " + version)


main()
PY
