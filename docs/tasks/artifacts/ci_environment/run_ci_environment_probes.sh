#!/usr/bin/env bash
# G1-SLICE.5b.3c.2b.h1.r2 — effective paths and actual guard counterfactuals.
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/ci_environment_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/local_environment_contract.py"
