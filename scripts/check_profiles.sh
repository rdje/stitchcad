#!/usr/bin/env bash
# scripts/check_profiles.sh — the three runtime profiles of roadmap §7.3 build, each with its stated command.
#
# WHY: G1's first exit clause. The roadmap names `native-full`, `native-headless` and `wasm-viewer`, and a profile
# means nothing unless its command builds and its exclusions hold. This builds each one and checks what the book's
# capability matrix says it leaves out: the headless program pulls in no viewport or GUI, and the browser artifact
# asks its host for nothing, so it needs no JavaScript glue.
#
# HONEST LIMITS: compiling proves it compiles. The browser run, `docs/tasks/artifacts/browser_run/run_browser_run.py`,
# is what executes the wasm-viewer profile in a real browser. `native-full` holds what exists today: the roadmap's
# SQLite, wgpu viewport and Tauri parts do not exist yet, and the matrix names their owners.
#
# Usage:  bash scripts/check_profiles.sh
# Output: one line per profile, then `profiles: 3 built`. Exit 0 when all three hold.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
if ! python3 -I -B scripts/local_environment.py --verify >/dev/null 2>&1; then
  exec python3 -I -B scripts/local_environment.py -- bash scripts/check_profiles.sh "$@"
fi
BOOK=docs/book/src/spec/runtime-profiles.md
status=0
fail() { echo "profiles: $1" >&2; status=1; }

for target in profile-native-full profile-native-headless profile-wasm-viewer; do
  grep -q "^$target:" Makefile || fail "the Makefile has no $target target"
done
for name in native-full native-headless wasm-viewer; do
  grep -q "^| \`$name\` |" "$BOOK" || fail "the capability matrix has no $name row"
done

if cargo build --workspace > target/profile-native-full.log 2>&1; then
  echo "native-full: every crate that exists today builds natively"
else
  fail "native-full does not build; see target/profile-native-full.log"
fi

if cargo build -p sc-cli > target/profile-native-headless.log 2>&1 && [ -x target/debug/stitchcad ]; then
  # A headless build must not drag in anything that needs a display.
  excluded="$(cargo tree -p sc-cli -e normal,build --prefix none 2>&1 | grep -E '^(wgpu|winit|tauri|egui|iced|slint|rusqlite) ' || true)"
  if [ -n "$excluded" ]; then
    fail "native-headless pulls in: $excluded"
  else
    echo "native-headless: target/debug/stitchcad builds, with no viewport, GUI or database dependency"
  fi
else
  fail "native-headless does not build; see target/profile-native-headless.log"
fi

if cargo build --target wasm32-unknown-unknown -p sc-viewer > target/profile-wasm-viewer.log 2>&1; then
  python3 -I -B - target/wasm32-unknown-unknown/debug/sc_viewer.wasm <<'PY'
import pathlib
import sys


def leb(data, at):
    value, shift = 0, 0
    while True:
        byte = data[at]
        at += 1
        value |= (byte & 0x7f) << shift
        if not byte & 0x80:
            return value, at
        shift += 7


path = pathlib.Path(sys.argv[1])
data = path.read_bytes()
if data[:4] != b"\0asm":
    sys.exit("profiles: %s is not WebAssembly" % path)
at, imports, exports = 8, 0, []
while at < len(data):
    section, at = data[at], at + 1
    size, at = leb(data, at)
    body = at
    if section == 2:
        imports, _ = leb(data, body)
    if section == 7:
        count, cursor = leb(data, body)
        for _ in range(count):
            length, cursor = leb(data, cursor)
            exports.append(data[cursor:cursor + length].decode())
            cursor += length + 1
            _, cursor = leb(data, cursor)
    at = body + size
if imports:
    sys.exit("profiles: wasm-viewer asks its host for %d imports, and the matrix says none" % imports)
wanted = {"sc_input_clear", "sc_input_push", "sc_run", "sc_output_len", "sc_output_byte"}
missing = wanted - set(exports)
if missing:
    sys.exit("profiles: wasm-viewer does not export %s" % sorted(missing))
print("wasm-viewer: %s builds, with no host imports and the five exports" % path)
PY
  [ "$?" -eq 0 ] || status=1
else
  fail "wasm-viewer does not build; see target/profile-wasm-viewer.log"
fi

[ "$status" -eq 0 ] && echo "profiles: 3 built"
exit "$status"
