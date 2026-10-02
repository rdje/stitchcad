#!/usr/bin/env bash
# Run alone: actual classifier mutation and exact restoration.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/formula_language/inline_context_mutations.py
