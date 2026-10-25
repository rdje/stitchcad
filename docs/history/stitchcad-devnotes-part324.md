# Sealed archive — completed explicit quantization scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5c.3.2` on `2026-10-25` (UTC).

- **Sealed identity:** 20 lines, 1537 bytes, sha256:0941fd51101dfc5a67086966435ddaea785806fe173e7f22f5a56f67fcd465c6
- **Coverage:** whole committed source interval from a9d86c4015fe5ecda988a7ee0be121e189ef63e6.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 26138–27675, 1537 bytes, sha256:0941fd51101dfc5a67086966435ddaea785806fe173e7f22f5a56f67fcd465c6
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.3.1`
  Status: `done`
  Goal:0252 exact round_to at authored same-kind step without intermediate width/storage refusals.
  Paths: crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/tests/formula_numeric_contract.rs
  Own bounded wide quotient/half-away magnitude rounding and final reduced result. Zero steps
  refuse formula_division, negative steps preserve the same multiples where their kind admits them.
  Preserve signed ties/canonical zero/fractional steps, no ppm scaling of the step quotient; final
  width/scalar checks occur after complete quantization. No source evaluation/provenance claim.
  One affected numeric native target; broad/platform .5c.4/pre-push. Own completed task/ledger
  retention323/224 and book/live/frontier sync; commit before binding conversion.


- [x] REPRODUCE / ISSUE:dda64a5 has no round_to value method, source inspection rc=0.
- [x] ROOT CAUSE:256-bit quotient may yield valid final multiple; cargo test --offline -p sc-core
  --test formula_numeric_contract:6 passed/0 failed, rc=0.
- [x] FIX: bounded wide rounding/final reduction, signed steps, canonical zero and exact refusals.
- [x] ADDRESSED: same native target proves ties/steps/final widths/domains,6/0, rc=0.
- [x] NO REGRESSION: same target retains earlier quantity/built-in cases,6/0, rc=0; broad work .5c.4.
- [x] LOCKSTEP: whole323/224 Git/public bytes exact, rc=0; book/live/frontier aligned.
  promotion: declined (established half-away/numeric contract/focused policy).
