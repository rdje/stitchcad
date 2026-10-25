# Sealed archive — completed binary quantity operator scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5c.2.3` on `2026-10-25` (UTC).

- **Sealed identity:** 24 lines, 1998 bytes, sha256:a19bc8c51e0eb3f6a12671dcaeb52c58c248381ed554d0e1e945f9ea0e2a6c26
- **Coverage:** whole committed source interval from 4e6803f79cdbc5131c69f8ea98ed9b98528c5667.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 24520–26518, 1998 bytes, sha256:a19bc8c51e0eb3f6a12671dcaeb52c58c248381ed554d0e1e945f9ea0e2a6c26
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.2.2`
  Status: `done`
  Goal: exact four-operator quantity scaling and per-completed-result scalar domains.
  Own0249: numeric.rs, rational.rs private scaled workspace and existing numeric/rational targets.
  Paths: crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/rational.rs;crates/sc-core/tests/formula_numeric_contract.rs
  Reuse normative kind signatures, ppm internal units and fused scaled products/quotients. Reduce
  the complete operator before128-bit width judgment; apply scalar domains before later rescue.
  Typed errors retain actual binary kinds/arc_length hint and scalar operation/value/bounds.
  One focused command for two affected native targets; broad/platform/mutation work .5c.4/pre-push.
  Retain completed numeric task/ledger320/221 and whole oldest journal321 if needed for fixed caps;
  sync book/live/current frontier, then commit before unary/comparison work.

- [x] REPRODUCE / ISSUE: fa6009e numeric values lack operator methods; source inspection rc=0.
- [x] ROOT CAUSE: judging unscaled ppm intermediates would falsely refuse valid results; focused
  cargo test --offline -p sc-core --test formula_numeric_contract --test formula_rational_contract
  proves scaled129-bit cancellation; output numeric3/0+rational4/0, rc=0.
- [x] FIX: four quantity methods/fused scaling, reused signatures and exact completed-result domains;
  actual dimension/scalar operation witnesses and arc_length hint, source inspection rc=0.
- [x] ADDRESSED: same focused two-target command proves scaling, kind/zero refusal and exact
  numerator/denominator/scalar witnesses; output7 passed/0 failed total, rc=0.
- [x] NO REGRESSION: same target command retains constructor/literal and34 independent Fraction
  rows,7/0 total, rc=0; broad/platform/mutation work stays .5c.4/pre-push.
- [x] LOCKSTEP: whole320/221 source/public intervals exact, rc=0; book/live/frontier aligned.
  promotion: declined (established numeric contract and focused-check policy).
