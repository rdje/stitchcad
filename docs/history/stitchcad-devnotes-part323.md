# Sealed archive — completed exact numeric built-in scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5c.3.1` on `2026-10-25` (UTC).

- **Sealed identity:** 24 lines, 1898 bytes, sha256:7a5116e64def0804cbe0039514e92674e37dd55dbafc9bcc99ac66f4b6ce16a3
- **Coverage:** whole committed source interval from dda64a5d565b4b0b6b9fbf4cfa4c7ad7f3145f37.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 25270–27168, 1898 bytes, sha256:7a5116e64def0804cbe0039514e92674e37dd55dbafc9bcc99ac66f4b6ce16a3
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.2.4`
  Status: `done`
  Goal: exact abs/min/max/clamp with typed kind and bound refusals.
  Own0251: numeric.rs and existing formula_numeric_contract native target; no evaluator scaffolding.
  Paths: crates/sc-core/src/recipe/numeric.rs;crates/sc-core/tests/formula_numeric_contract.rs
  Absolute value preserves exact signs/zero/domains. Min/max accept one or more homogeneous numeric
  values and return an existing exact operand; empty calls refuse formula_parse. Clamp requires
  matching kinds and exact low<=high, returns the appropriate admitted operand without rounding.
  Errors retain builtin, actual first mismatch/index/wanted kind or exact inverted bounds.
  One affected native target; broad/platform/mutation work stays .5c.4/pre-push. Retain completed
  unary task/ledger322/223 under fixed caps; book/live/frontier sync and commit before bindings.


- [x] REPRODUCE / ISSUE:1b114c8 numeric values lack abs/min/max/clamp methods; source inspection0.
- [x] ROOT CAUSE: exact homogeneous built-ins must preserve fractions and return admitted operands;
  cargo test --offline -p sc-core --test formula_numeric_contract proves all outcomes,5/0, rc=0.
- [x] FIX: total abs/checked min/max/clamp, first actual-kind/index mismatch, empty-call and exact
  inverted-bound refusals; no rounding or evaluator dependency, source inspection rc=0.
- [x] ADDRESSED: same native target covers full magnitudes, singleton/variadic/empty/mixed operands,
  near-equal fractions and inclusive/inverted clamp bounds,5 passed/0 failed, rc=0.
- [x] NO REGRESSION: same target retains constructor/literal/binary/unary/comparison cases,5/0,
  rc=0; primitive unchanged, broad/platform/mutation checks remain .5c.4/pre-push.
- [x] LOCKSTEP: whole322/223 source/public reads exact, rc=0; book/live/frontier aligned.
  promotion: declined (established numeric contract/focused-check policy).
