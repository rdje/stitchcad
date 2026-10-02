#!/usr/bin/env bash
# Run alone: actual Rust production context mutations and exact restoration.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
mkdir -p target/cargo-home target/scratch
export CARGO_HOME="$ROOT/target/cargo-home" TMPDIR="$ROOT/target/scratch"
python3 -I -B docs/tasks/artifacts/formula_structure/domain_context_mutations.py
