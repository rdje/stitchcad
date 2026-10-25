# Sealed archive — completed numeric domain scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5c.2.2` on `2026-10-25` (UTC).

- **Sealed identity:** 23 lines, 1828 bytes, sha256:7b3ec947e7292fc50084a136965b3751d581caeabc85e309cd17e7825f87d80d
- **Coverage:** whole committed source interval from fa6009e1cb47d2e28ac4ce04549aa5d3a38fb4cb.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 24168–25996, 1828 bytes, sha256:7b3ec947e7292fc50084a136965b3751d581caeabc85e309cd17e7825f87d80d
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.2.1`
  Status: `done`
  Goal: immutable completed numeric values with exact scalar-domain validation and normalized input bridge.
  Acceptance: only five numeric kinds; length/area exact signed limits, nonnegative fractional Count,
  Angle/Ratio retain full rational range until binding; reject before rounding or later cancellation.
  Own0248: numeric.rs, recipe exports, new focused native formula_numeric_contract target; no evaluator
  or binding. Typed errors retain actual kind/exact value/bounds without invented source context.
  Paths: crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_numeric_contract.rs
  Book/status-map/live/frontier sync; completed oracle task/ledger retained319/220 under fixed caps.


- [x] REPRODUCE / ISSUE: predecessor6b1de3d has kind metadata and untyped rationals only;
  numeric value source absent, source inspection rc=0.
- [x] ROOT CAUSE: exact completed domains differ from rounded input and signed storage bounds;
  cargo test --offline -p sc-core --test formula_numeric_contract:2 passed/0 failed, rc=0.
- [x] FIX: immutable numeric value validates exact signed length/area and nonnegative Count;
  normalized input bridge preserves internal integers, typed actual-kind/value/bound refusals.
- [x] ADDRESSED: same focused native target checks inclusive bounds/fractional excess, all kinds,
  full-width temporaries/input rounding/raw turns,2 passed/0 failed, rc=0.
- [x] NO REGRESSION: constructor bridge exercises existing literal conversion,2/0, rc=0;
  primitive/operator syntax unchanged; broad/platform checks remain .5c.4/pre-push.
- [x] LOCKSTEP: source/public retained319/220 exact, rc=0; API map/book/live/frontier aligned.
  promotion: declined (existing exact numeric contract and focused-check policy).
