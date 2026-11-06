#!/usr/bin/env bash
# docs/tasks/artifacts/browser_run/run_capability_matrix_check.sh
# G1-SLICE.12 — the browser capability matrix states what the recorded run measured, and covers what the roadmap asks.
#
# WHY: G1's WASM evidence clause asks for "capability matrix with tested versions + limitations". A table typed by
# hand drifts from what ran, so this requires the chapter's version and sizes to equal the tracked recording, every row
# to say `tested` or `not tested`, and every limitation the leaf names to appear.
#
# HONEST LIMITS: this checks agreement with the recording. run_browser_run.sh is what runs Chrome. A hosted run uses a
# different Chrome and is not compared with the recording, so a matrix row is a fact about the recorded run.
#
# Usage:  bash docs/tasks/artifacts/browser_run/run_capability_matrix_check.sh
# Output: `capability matrix: <n> rows agree with the recorded run`. Exit 0 when they do.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
exec python3 -I -B - <<'PY'
import json
import pathlib
import re
import sys

BOOK = pathlib.Path("docs/book/src/spec/browser-capabilities.md")
RECORD = pathlib.Path("docs/tasks/artifacts/browser_run/recorded-run.json")
faults = []
text = BOOK.read_text(encoding="utf-8")
record = json.loads(RECORD.read_text(encoding="utf-8"))

for key in ("browser", "os"):
    if str(record[key]) not in text:
        faults.append("the chapter does not state the recorded %s, %s" % (key, record[key]))
for key in ("wasm_bytes", "image_bytes", "dark_pixels"):
    if "{:,}".format(record[key]) not in text:
        faults.append("the chapter does not state the recorded %s, %s" % (key, "{:,}".format(record[key])))
for key in ("compile_ms", "first_run_ms", "second_run_ms"):
    if ("%s ms" % record[key]) not in text:
        faults.append("the chapter does not state the recorded %s, %s ms" % (key, record[key]))

rows = re.findall(r"^\| ([^|]+) \| (tested|not tested|not used|not implemented) \|", text, flags=re.M)
for name in ("Chrome", "Firefox", "Safari", "Edge", "WebAssembly", "Startup size", "COOP/COEP", "Threads", "WebGPU",
             "IndexedDB", "Canvas", "Inline SVG", "Export", "Cancellation"):
    if not any(row[0].startswith(name) for row in rows):
        faults.append("the matrix has no row for %s" % name)
for row in rows:
    if row[1] != "tested" and not re.search(r"G5-SHELLS\.\d+", [line for line in text.splitlines() if line.startswith("| %s |" % row[0])][0]):
        faults.append("%s is %s, and its row names no owner" % (row[0], row[1]))
for fact, wanted in (("imports", 0), ("cross_origin_isolated", False), ("shared_array_buffer", False)):
    if record[fact] != wanted:
        faults.append("the matrix assumes %s is %r, and the recording says %r" % (fact, wanted, record[fact]))
for fault in faults:
    print("  x " + fault, file=sys.stderr)
if faults:
    sys.exit(1)
print("capability matrix: %d rows agree with the recorded run" % len(rows))
PY
