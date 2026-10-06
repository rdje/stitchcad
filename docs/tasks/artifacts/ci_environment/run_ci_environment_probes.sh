#!/usr/bin/env bash
# G1-SLICE.5b.3c.2b.h1.r2 — effective paths and actual guard counterfactuals.
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/ci_environment_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/local_environment_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/producer_census_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/producer_census.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/python_producer_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/python_bridge_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/python_recipe_fixture_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/python_input_context_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/python_structure_fixture_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/python_inline_context_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/python_static_loader_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/python_payload_library_contract.py"
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/python_static_loader_contract.py" --native
python3 -I -B "$ROOT/docs/tasks/artifacts/ci_environment/push_cadence_contract.py"
