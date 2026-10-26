# Sealed archive — whole completed canonical input-adapter summaries

Immutable historical segment, sealed by leaf `G1-SLICE.4c.2.1` on `2026-10-26` (UTC).

- **Sealed identity:** 41 lines, 3225 bytes, sha256:d8d327a05bfaf40b8a644b56bed57461d4e36c084174de4619da4a0f8c1dad4d
- **Coverage:** whole committed source interval from 3c852040cef613d7b43d9046de58bfc15008df04.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 1007–4232, 3225 bytes, sha256:d8d327a05bfaf40b8a644b56bed57461d4e36c084174de4619da4a0f8c1dad4d
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.1.1`
  Status: `done`
  Goal: canonical length authored reads, exact source/state and25 pairs verified0261.
  Paths: crates/sc-core/src/recipe/input.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_length_input_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part339md).
- ID: `G1-SLICE.5e.1.2`
  Status: `done`
  Goal: canonical scalar state/value content and selected lint repair verified0262.
  Paths: crates/sc-core/src/value.rs;crates/sc-core/src/value/scalar.rs;crates/sc-core/tests/scalar_value_contract.rs;crates/sc-core/tests/formula_length_input_contract.rs
  Additional paths: crates/sc-core/src/recipe/input.rs;crates/sc-core/src/recipe/declaration.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/roots.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part341md).
- ID: `G1-SLICE.5e.1.3`
  Status: `done`
  Goal: sourced general scalar reads/actual target-kind checks verified0263, native5/3.
  Paths: crates/sc-core/src/recipe/input.rs;crates/sc-core/src/recipe/input/scalar.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/src/recipe/declaration.rs;crates/sc-core/tests/formula_scalar_input_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part343md).
- ID: `G1-SLICE.5e.1.4`
  Status: `done`
  Goal: intrinsic/explicit sourced optional tolerance reads verified0264, native5/0.
  Paths: crates/sc-core/src/recipe/context.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_tolerance_context_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part345md).
- ID: `G1-SLICE.5e.1.5`
  Status: `done`
  Goal: pinned authored-membership size_index/size_count/is_base_size, without labels or axes.
  Children .1 core range/source/value context; .2 actual sc-measure membership bridge. Axes remain .4c.2.
- ID: `G1-SLICE.5e.1.5.1`
  Status: `done`
  Goal: core size source/range/value frame and wide Count bridge verified0265.
  Paths: crates/sc-core/src/recipe/context.rs;crates/sc-core/src/recipe/context/size.rs;crates/sc-core/src/recipe/semantic.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/mod.rs
  Additional paths: crates/sc-core/tests/formula_size_context_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part347md).
- ID: `G1-SLICE.5e.1.5.2`
  Status: `done`
  Goal: actual pinned membership/selected-id size bridge verified0266, native5/0.
  Paths: crates/sc-measure/src/size_membership.rs;crates/sc-measure/src/size_membership/formula.rs;crates/sc-measure/src/lib.rs;crates/sc-measure/tests/formula_size_membership_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part348md).
- ID: `G1-SLICE.5e.1.6`
  Status: `done`
  Goal: exact canonical/stored scalar numeric bridges verified0267, native5/0.
  Paths: crates/sc-core/src/recipe/numeric.rs;crates/sc-core/tests/formula_value_read_bridge_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part349md).
