# Sealed archive — canonical formula-rational construction scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5c.1.2.1` on `2026-10-25` (UTC).

- **Sealed identity:** 21 lines, 1810 bytes, sha256:2d50f2130d699af871dbe1aa4076a9843a758e958f81b34d4267d6ca51f3c13a
- **Coverage:** whole .5c.1.1 scope and verification from 67e0a416fd7b5194aaa1f38e9d5abadf7524d334; ownership remains live.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 22290–24100, 1810 bytes, sha256:2d50f2130d699af871dbe1aa4076a9843a758e958f81b34d4267d6ca51f3c13a
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.1.1`
  Status: `done`
  Goal: STITCHCAD-G1-0244 implements privately canonical signed128-magnitude formula rational values.
  Paths: crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_rational_contract.rs
  Acceptance: exact gcd reduction, positive denominator, full unsigned magnitudes with independent sign,
  canonical positive0/1, typed zero-denominator refusal, immutable inspectors and signed-integer input.
  No rounding, allocation, dependencies, kind/domain/storage/evaluator authority. One focused native
  contract target; broad numerical/platform verification belongs to .5c.1.3. Update book/status map,
  live docs and commit before arithmetic. Own whole0243 task/ledger retention313/216 under unchanged caps.

- [x] REPRODUCE / ISSUE: recipe source inventory has literal/ontology rationals but no formula value
  primitive; source/mod.rs inspection, rc=0.
- [x] ROOT CAUSE: edge Rational is i64-bounded; literal magnitudes are input integers. New focused
  formula_rational_contract tests full u128 components and independent sign, one pass, rc=0.
- [x] FIX: private reduced value, canonical0/1, typed zero-denominator error and exact i128 input.
  The focused native public contract passes1/0, rc=0; no rounding/dependency/domain/evaluator added.
- [x] ADDRESSED: primary local profile cargo test -p sc-core --test formula_rational_contract:
  canonical_values_preserve_sign_zero_and_full_width ok;1 passed/0 failed, rc=0.
- [x] NO REGRESSION: focused target compiles sc-core/passes, rc=0; broader/platform checks owned by .5c.1.3.
- [x] LOCKSTEP: whole0243 source/public task/ledger bytes match, rc=0; book/status/live/frontier
  updated. promotion: declined (existing numeric contract/focused policy). Commit hooks required.
