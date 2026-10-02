#!/usr/bin/env bash
# Reference structural-limit controls and copied-book refusals; no production mutation.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/formula_structure/formula_structure.py
python3 -I -B docs/tasks/artifacts/formula_structure/formula_input.py
