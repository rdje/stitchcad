#!/usr/bin/env bash
# Run alone: actual Rust production mutation and byte-identical restoration.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
mkdir -p target/cargo-home target/scratch
export CARGO_HOME="$ROOT/target/cargo-home" TMPDIR="$ROOT/target/scratch"
python3 -I -B docs/tasks/artifacts/formula_structure/length_operator_mutations.py
