#!/usr/bin/env bash
# Actual reference input mutations: no overlapping reference/probe/build/gate or handoff.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/formula_structure/formula_input_mutations.py
