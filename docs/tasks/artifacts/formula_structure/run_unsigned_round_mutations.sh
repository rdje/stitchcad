#!/usr/bin/env bash
# Run exclusively; production source mutation and byte-identical restoration.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/formula_structure/unsigned_round_mutations.py
