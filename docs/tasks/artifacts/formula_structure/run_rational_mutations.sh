#!/usr/bin/env bash
# Run alone: actual reference mutation and byte-identical restoration.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/formula_structure/rational_mutations.py
