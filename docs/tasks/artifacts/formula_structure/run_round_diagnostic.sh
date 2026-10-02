#!/usr/bin/env bash
# Public endpoint diagnostic, not a green verdict; contracts judge its returned values.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
mkdir -p target/cargo-home target/scratch target/formula-round-diagnostic
export CARGO_HOME="$ROOT/target/cargo-home" TMPDIR="$ROOT/target/scratch"
cargo build -p sc-units --message-format=json > target/formula-round-diagnostic/build.json
SC_ROUND_DIAGNOSTIC_RLIB=$(python3 -I -B - <<'PY'
import json
from pathlib import Path
paths=[]
for line in Path('target/formula-round-diagnostic/build.json').read_text().splitlines():
    event=json.loads(line)
    if event.get('reason')=='compiler-artifact' and event['target']['name']=='sc_units':
        paths.extend(path for path in event['filenames'] if path.endswith('.rlib'))
assert len(paths)==1, ('ambiguous current library', paths)
print(paths[0])
PY
)
rustc --edition=2021 docs/tasks/artifacts/formula_structure/round_diagnostic.rs \
  --extern "sc_units=$SC_ROUND_DIAGNOSTIC_RLIB" -L dependency=target/debug/deps \
  -o target/formula-round-diagnostic/probe
target/formula-round-diagnostic/probe
