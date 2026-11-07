#!/usr/bin/env bash
# scripts/dev_shell.sh — opens the dev shell on a session, in your own browser. It is a tool for looking at drafts, not the product.
#
# It builds the WebAssembly module, gathers the page, the module and the session into target/dev_shell/site, and serves them
# on 127.0.0.1 until you press Ctrl-C. Nothing is written outside target/, and saving stays in the page: closing it loses the project.
#
# Usage:  bash scripts/dev_shell.sh [session.json] [port]
# Default session: conformance/reference-skirt/session.json. Default port: 8765.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
if ! python3 -I -B scripts/local_environment.py --verify >/dev/null 2>&1; then
  exec python3 -I -B scripts/local_environment.py -- bash scripts/dev_shell.sh "$@"
fi
SESSION="${1:-conformance/reference-skirt/session.json}"
PORT="${2:-8765}"
[ -f "$SESSION" ] || { echo "dev shell: no session at $SESSION" >&2; exit 1; }
SITE=target/dev_shell/site
rm -rf "$SITE"
mkdir -p "$SITE"
cargo build --release --target wasm32-unknown-unknown -p sc-viewer > target/dev_shell/build.log 2>&1 \
  || { echo "dev shell: the module does not build; see target/dev_shell/build.log" >&2; exit 1; }
cp crates/sc-viewer/shell/shell.html "$SITE/index.html"
cp target/wasm32-unknown-unknown/release/sc_viewer.wasm "$SITE/sc_viewer.wasm"
cp "$SESSION" "$SITE/session.json"
echo "dev shell: open http://127.0.0.1:$PORT/ and press Ctrl-C here to stop"
exec python3 -I -B -m http.server "$PORT" --bind 127.0.0.1 --directory "$SITE"
