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
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_byte_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_input_review_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_byte_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_recipe_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_input_review_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/semantic_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/declaration_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/namespace_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/name_read_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/ordered_name_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/operator_signature_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/builtin_signature_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/wanted_signature_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/call_lookup_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/checked_expression_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/checked_statement_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/statement_owner_contract.py
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
python3 -I -B docs/tasks/artifacts/formula_structure/static_signature_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/reference_locator_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/static_namespace_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/header_dimension_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/ambiguity_payload_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/reserved_diagnostic_review.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/call_lookup_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/dimension_payload_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/geometry_argument_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/static_recipe_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/static_review_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/assertion_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/origin_value_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/provenance_contract.py --mutations
