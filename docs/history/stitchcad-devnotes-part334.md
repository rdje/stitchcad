# Sealed archive — completed exact numeric and binding scope group

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.h` on `2026-10-25` (UTC).

- **Sealed identity:** 52 lines, 3397 bytes, sha256:a26446082cdc2bdb5614c82fbc5754ccff3b80b0fcaec094aef19b8b757f643d
- **Coverage:** whole committed source interval from f168eec4e43cc794dc3e989706215a61f44cdec4.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 22808–26205, 3397 bytes, sha256:a26446082cdc2bdb5614c82fbc5754ccff3b80b0fcaec094aef19b8b757f643d
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.2`
  Status: `done`
  Goal: all typed rational operators and abs/min/max/clamp without implicit rounding.
  Acceptance: dimension/scaling tables, exact fractional Count, signed length/area per-result
  domains, literal input boundary distinct from result boundary, zero division and invalid clamp;
  later cancellation never rescues an invalid completed intermediate. Raw formula turns preserved.
  Children: .1 typed completed values/domains; .2 binary arithmetic/scaling; .3 unary/comparison;
  .4 abs/min/max/clamp. Verification remains focused per feature; broad integration .5c.4.

- ID: `G1-SLICE.5c.2.1`
  Status: `done`
  Goal: immutable completed numeric values/domains and normalized literal bridge verified0248.
  Paths: crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_numeric_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part320.md).

- ID: `G1-SLICE.5c.2.2`
  Status: `done`
  Goal: four exact quantity operators/fused ppm scaling and result domains verified0249.
  Paths: crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/rational.rs;crates/sc-core/tests/formula_numeric_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part321.md).

- ID: `G1-SLICE.5c.2.3`
  Status: `done`
  Goal: exact unary operators/total rational ordering/six same-kind comparisons verified0250.
  Paths: crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/tests/formula_rational_contract.rs;crates/sc-core/tests/formula_numeric_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part322.md).

- ID: `G1-SLICE.5c.2.4`
  Status: `done`
  Goal: exact abs/min/max/clamp with typed arity/kind/bound refusals verified0251.
  Paths: crates/sc-core/src/recipe/numeric.rs;crates/sc-core/tests/formula_numeric_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part323.md).

- ID: `G1-SLICE.5c.3`
  Status: `done`
  Goal: explicit round_to and one binding conversion for all five numeric kinds plus Boolean.
  Acceptance: exact ties away at authored step/binding, full i64 endpoints including MIN, wider
  exact literal/temporary permitted, Count nonnegative and scalar bounds checked; later reads
  use the returned stored integer. No Angle direction modulo for formula sweeps/equality.
  Diagnostics retain binding name/kind/bounds/rounded measured result, without a silent clamp.
  Verification: `pending`; Commit: `pending`.
- ID: `G1-SLICE.5c.3.1`
  Status: `done`
  Goal: exact explicit round_to with signed/fractional steps and final width/domain judgment verified0252.
  Paths: crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/tests/formula_numeric_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part324.md).

- ID: `G1-SLICE.5c.3.2`
  Status: `done`
  Goal: one scalar binding conversion, both i64 endpoints/Boolean/exact refusal context verified0253.
  Paths: crates/sc-core/src/recipe/bound.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/lib.rs;crates/sc-core/Cargo.toml;crates/sc-core/tests/formula_binding_value_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part326.md).
