# Sealed archive — whole five completed sourced-adapter summaries

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.b.1` on `2026-10-26` (UTC).

- **Sealed identity:** 25 lines, 1922 bytes, sha256:04e96d580b6b5411e36a8aaec23c445ffb675ac2628443a6bf799e9908cbd1c8
- **Coverage:** whole committed source interval from 4e986498404e8464c5361eeebcf37a3281f0cc06.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 7769–9691, 1922 bytes, sha256:04e96d580b6b5411e36a8aaec23c445ffb675ac2628443a6bf799e9908cbd1c8
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.3.1`
  Status: `done`
  Goal: immutable approximation-source sets/T1 guard verified0268, native4/0.
  Paths: crates/sc-core/src/recipe/provenance.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_approximation_sources_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part352md).
- ID: `G1-SLICE.5e.3.2`
  Status: `done`
  Goal: sourced numeric/Boolean operators/bindings verified0269, native5/0.
  Paths: crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_evaluated_scalar_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part354md).
- ID: `G1-SLICE.5e.3.3`
  Status: `done`
  Goal: sourced implemented numeric call families verified0270, native5/0.
  Paths: crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/evaluated/calls.rs;crates/sc-core/tests/formula_evaluated_calls_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part356md).
- ID: `G1-SLICE.5e.3.4`
  Status: `done`
  Goal: exact sourced named tolerance comparisons verified0271, native5/0.
  Paths: crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/evaluated/tolerance.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_tolerance_comparison_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part358md).
- ID: `G1-SLICE.5e.3.5`
  Status: `done`
  Goal: actual named assertion success/refusal verified0272, native4/0.
  Paths: crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/evaluated/assertion.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_assertion_value_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part361md).
