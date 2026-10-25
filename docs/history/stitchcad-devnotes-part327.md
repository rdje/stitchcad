# Sealed archive — completed exact root-family plan and fixture proof

Immutable historical segment, sealed by leaf `G1-SLICE.5d.2.1` on `2026-10-25` (UTC).

- **Sealed identity:** 15 lines, 1088 bytes, sha256:ffa5feca428839894105c53985c1ee4f2cc94ac3b5836653f4a80815c7e0b12f
- **Coverage:** whole committed source interval from 5fae1cb00c5f3a53ea9dd57e785c7b1f2c29a00e.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 27626–28714, 1088 bytes, sha256:ffa5feca428839894105c53985c1ee4f2cc94ac3b5836653f4a80815c7e0b12f
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.1.1`
  Status: `done`
  Goal:0254 exact root plan and independent fixtures before code.
  Paths: docs/tasks/artifacts/formula_structure/root_value_oracle.py;crates/sc-core/tests/fixtures/formula_roots.tsv
  Own decision/book/fixtures/retention326/226 and true-value float clarification.
  Acceptance: [whole algorithm/bounds/oracle plan](../decisions/decision_numerical-contract-fixed-point.md).
  Production roots remain .5d.2; transcendental algorithms .5d.1.2, broad/platform .5d.4/pre-push.

- [x] REPRODUCE / ISSUE: root functions lack a certified algorithm; source inspection0.
- [x] ROOT CAUSE: exact midpoint/bounds required; pure oracle CLI25 rows/authored checks, rc=0.
- [x] FIX: integer plan and independent isqrt/Fraction fixtures, no production claim.
- [x] ADDRESSED: oracle CLI25 expected rows/authored checks pass, rc=0.
- [x] NO REGRESSION: same CLI25 rows, rc=0; production unchanged, broad checks .5d.4.
- [x] LOCKSTEP:326/226 Git/public bytes exact, rc=0; book/pointers aligned.
  promotion: numerical decision gains root-family proof/oracle plan.
