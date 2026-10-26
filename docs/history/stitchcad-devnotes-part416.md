# Sealed archive — whole completed execution-prerequisite summaries

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.b.2.a` on `2026-10-26` (UTC).

- **Sealed identity:** 22 lines, 1663 bytes, sha256:cc984863b63601684092ffb7c8fb89a567750d36d251197fe92186216a4b8f1f
- **Coverage:** whole committed source interval from e95fa7af29b543fb96d28e3bd691058011f2cbc5.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 4698–6361, 1663 bytes, sha256:cc984863b63601684092ffb7c8fb89a567750d36d251197fe92186216a4b8f1f
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.2.1`
  Status: `done`
  Goal: lazy conditional value kinds/selected errors/sources verified0273, native4/0.
  Paths: crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/evaluated/conditional.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_conditional_value_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part363md).
- ID: `G1-SLICE.5e.2.2`
  Status: `done`
  Goal: checked node/expression/statement/atomic execution sequence recorded0274.
  Paths: docs/book/src/annexes/formula-execution-plan.md;docs/book/src/SUMMARY.md
  Acceptance: [whole scope/receipts/partition proof](../history/window13.md#stitchcad-devnotes-part365md).
- ID: `G1-SLICE.5e.2.3`
  Status: `done`
  Goal: actual per-node kind/role/source/callee certificates verified0275, native6+7/docs8.
  Paths: crates/sc-core/src/recipe/checked.rs;crates/sc-core/src/recipe/checked/nodes.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_checked_node_contract.rs
  Acceptance: [whole scope/six proof boxes/book-map repair](../history/window14.md#stitchcad-devnotes-part367md).
- ID: `G1-SLICE.5e.2.h13`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part408.md).
  Retention0292 owned .6.2.1:
  - [x] ROOT CAUSE0292: git show6662298 whole sourced-call/archive13 records retained407/408; Python byte/hash comparison rc=0.
  - [x] ADDRESSED0292: whole source/hash exact in407/408, Python comparison rc=0.
  - [x] NO REGRESSION0292: scripts/history_archive.py --root . read407/408 whole public files exact, Python comparison rc=0; no reader change.
