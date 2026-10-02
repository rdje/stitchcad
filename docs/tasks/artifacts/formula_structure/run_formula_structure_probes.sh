#!/usr/bin/env bash
# Reference structural-limit controls and copied-book refusals; no production mutation.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/formula_structure/formula_structure.py
python3 -I -B docs/tasks/artifacts/formula_structure/formula_input.py
python3 -I -B docs/tasks/artifacts/formula_structure/formula_expression_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/round_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/unsigned_round_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/literal_normalization_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_expression_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/reduction_boundary_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_contract_inventory.py
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_expression_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/statement_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/statement_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/literal_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/arithmetic_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/angle_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/angle_math_oracle.py
python3 -I -B docs/tasks/artifacts/formula_structure/signed_angle_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/rational_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/scalar_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/binding_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/binding_replay_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_literal_contract.py
