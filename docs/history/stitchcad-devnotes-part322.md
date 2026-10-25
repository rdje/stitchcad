# Sealed archive — completed unary operator and comparison scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5c.2.4` on `2026-10-25` (UTC).

- **Sealed identity:** 24 lines, 1958 bytes, sha256:c0e398c3666b0fd9474bfd4f3c73790baa1472e42d33c372e96732aff8e2d02c
- **Coverage:** whole committed source interval from 1b114c8df74333202d7bf3496e301723a5f22980.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 24871–26829, 1958 bytes, sha256:c0e398c3666b0fd9474bfd4f3c73790baa1472e42d33c372e96732aff8e2d02c
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.2.3`
  Status: `done`
  Goal: exact unary negation/square and same-kind numeric comparisons.
  Own0250: rational sign negation/total ordering, numeric unary/comparison methods and affected targets.
  Paths: crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/tests/formula_rational_contract.rs;crates/sc-core/tests/formula_numeric_contract.rs
  Reuse unary/kind signatures: Count negation refuses even zero, square has only declared kinds,
  ratio square keeps fused ppm scaling. Comparisons retain full signed turns and fractional values;
  bounded wide cross-products cannot falsely refuse because a difference would exceed width128.
  One focused two-target command; broad/platform/mutation work remains .5c.4/pre-push.
  Retain completed binary task/ledger321/222 under fixed caps; sync book/live before built-ins.

- [x] REPRODUCE / ISSUE:4e6803f lacks rational ordering and numeric unary/comparison methods;
  source inspection confirms, rc=0.
- [x] ROOT CAUSE: exact comparison needs wider cross-products without width-limited subtraction;
  cargo test --offline -p sc-core --test formula_numeric_contract --test formula_rational_contract
  distinguishes adjacent full-width fractions, output numeric4/0+rational5/0, rc=0.
- [x] FIX: exact ordering/sign reversal plus signature-backed negation/square and six comparisons;
  Count negation and undeclared square kinds refuse, source inspection rc=0.
- [x] ADDRESSED: same focused two-target command proves signs/raw turns/fractions/scaled squares/
  kind refusals/exact ordering,9 passed/0 failed total, rc=0.
- [x] NO REGRESSION: same target command retains constructor/binary cases and34 Fraction rows,
 9/0 total, rc=0; broad/platform/mutation work stays .5c.4/pre-push.
- [x] LOCKSTEP: whole321/222 source/public reads exact, rc=0; book/live/frontier aligned.
  promotion: declined (established exact numeric contract/focused-check policy).
