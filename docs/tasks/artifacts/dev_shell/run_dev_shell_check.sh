#!/usr/bin/env bash
# docs/tasks/artifacts/dev_shell/run_dev_shell_check.sh
# G2-2D.9 — the dev shell in a real browser, driven by real events, and its recording replayed by the real program.
#
# WHY: G2's exit asks for a minimal viewer and editor over the same commands. Tests of its logic cannot say whether the page wires a
# button to the right action or whether anything reaches the screen. So this loads the real page in Chrome, sends real events to the real
# controls, reads pixels back from what the renderer drew, then hands what the page recorded to the native program.
#
# THE PROOF OF "THE SAME COMMANDS": replaying the page's recording with `stitchcad` must give the exact bytes the page exported. A page
# with any other way to change the Design could not pass, because the program has only the session's steps.
#
# HONEST LIMITS: one browser family on the machine that runs this. Headless Chrome paints offscreen, so frames reach no display.
# On a machine with no GPU the page falls back to a 2D canvas, and the printed renderer says so.
#
# Usage:  bash docs/tasks/artifacts/dev_shell/run_dev_shell_check.sh
# Output: one line per check, then `dev shell: N checks held in <browser>, drawn with <renderer>`. Exit 0 only when all hold.
# target/dev_shell_check/picture.png keeps what the renderer drew.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
if ! python3 -I -B scripts/local_environment.py --verify >/dev/null 2>&1; then
  exec python3 -I -B scripts/local_environment.py -- bash docs/tasks/artifacts/dev_shell/run_dev_shell_check.sh "$@"
fi
STORE=target/dev_shell_check
rm -rf "$STORE"
mkdir -p "$STORE/site"
cargo build --release --target wasm32-unknown-unknown -p sc-viewer > "$STORE/build-wasm.log" 2>&1 \
  || { echo "dev shell: the module does not build; see $STORE/build-wasm.log" >&2; exit 1; }
cargo build -p sc-cli > "$STORE/build-native.log" 2>&1 \
  || { echo "dev shell: the program does not build; see $STORE/build-native.log" >&2; exit 1; }
cp target/wasm32-unknown-unknown/release/sc_viewer.wasm "$STORE/site/"
cp conformance/reference-skirt/session.json "$STORE/site/session.json"
cp docs/tasks/artifacts/dev_shell/drive.js "$STORE/site/drive.js"
{ cat crates/sc-viewer/shell/shell.html; printf '<script src="drive.js"></script>\n'; } > "$STORE/site/page.html"

exec python3 -I -B - <<'PY'
import base64
import shlex
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

STORE = pathlib.Path("target/dev_shell_check").resolve()
SITE = STORE / "site"
PROGRAM = pathlib.Path("target/debug/stitchcad").resolve()
TIMEOUT = 180
KINDS = {".wasm": "application/wasm", ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".json": "application/json"}


def fail(message):
    print("dev shell: " + message, file=sys.stderr)
    sys.exit(1)


class Report:
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
            return self.value


REPORT = Report()


class Handler(http.server.BaseHTTPRequestHandler):
    def log_message(self, *arguments):
        pass

    def do_GET(self):
        target = SITE / (self.path.split("?")[0].lstrip("/") or "page.html")
        if not target.is_file():
            self.send_error(404)
            return
        body = target.read_bytes()
        self.send_response(200)
        self.send_header("Content-Type", KINDS.get(target.suffix, "application/octet-stream"))
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_POST(self):
        length = int(self.headers.get("Content-Length", "0"))
        REPORT.post(json.loads(self.rfile.read(length).decode("utf-8")))
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


def main():
    executable = browser()
    version = subprocess.run([executable, "--version"], capture_output=True, text=True, timeout=30).stdout.strip()
    server = Server(("127.0.0.1", 0), Handler)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    profile = STORE / "profile"
    profile.mkdir()
    arguments = [executable, "--headless=new", "--no-first-run", "--no-default-browser-check", "--disable-background-networking",
                 "--disable-component-update", "--disable-sync", "--disable-crash-reporter", "--disable-breakpad", "--enable-unsafe-webgpu",
                 "--window-size=1500,950", "--user-data-dir=" + str(profile), "--disk-cache-dir=" + str(profile) + "-cache",
                 "http://127.0.0.1:%d/page.html" % server.server_address[1]]
    if os.environ.get("CI"):
        arguments.insert(1, "--no-sandbox")
    # D418: a diagnostic hook to put this machine on another machine's rendering path, such as SwiftShader's.
    for flag in reversed(shlex.split(os.environ.get("DEV_SHELL_CHROME_FLAGS", ""))):
        arguments.insert(1, flag)
    process = subprocess.Popen(arguments, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
    try:
        report = REPORT.wait()
    finally:
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except (ProcessLookupError, PermissionError):
            pass
        process.wait()
        server.shutdown()
    if report is None:
        fail("the page reported nothing within %d seconds" % TIMEOUT)
    if "failure" in report:
        fail("the page failed: " + report["failure"])
    bad = 0
    for item in report["results"]:
        print("%s %s%s" % ("held  " if item["held"] else "FAILED", item["name"], (": " + item["detail"]) if item["detail"] else ""))
        bad += 0 if item["held"] else 1
    (STORE / "picture.png").write_bytes(base64.b64decode(report["picture"].split(",", 1)[1]))
    # The proof: the native program, given only what the page recorded, writes the exact bytes the page exported.
    (STORE / "recording.json").write_text(report["recording"], encoding="utf-8")
    project = STORE / "replayed"
    done = subprocess.run([str(PROGRAM), "replay", str(STORE / "recording.json"), str(project)], capture_output=True, text=True)
    if done.returncode:
        fail("the program refused what the page recorded: " + done.stdout[:400] + done.stderr[:400])
    if len(report["files"]) < 4:
        fail("only %d files came back from the page" % len(report["files"]))
    for path, content in sorted(report["files"].items()):
        written = (project / path).read_bytes()
        same = written == bytes.fromhex(content)
        print("%s the program replays %s to the page's %d bytes" % ("held  " if same else "FAILED", path, len(written)))
        bad += 0 if same else 1
    if bad:
        fail("%d checks failed in %s, drawn with %s" % (bad, version, report["renderer"]))
    total = len(report["results"]) + len(report["files"])
    print("dev shell: %d checks held in %s, drawn with %s" % (total, version, report["renderer"]))


main()
PY
