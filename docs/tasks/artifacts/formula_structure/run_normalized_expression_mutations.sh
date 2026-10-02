#!/usr/bin/env bash
# Run exclusively: actual Rust mutations, no overlapping build/probe/gate or handoff.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
mkdir -p target/cargo-home target/scratch
export CARGO_HOME="$ROOT/target/cargo-home" TMPDIR="$ROOT/target/scratch"
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_expression_mutations.py
