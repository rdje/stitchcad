#!/usr/bin/env bash
# Run alone: mutates actual reference temporarily; no overlapping probes/builds/gates/handoff.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/formula_structure/literal_mutations.py
