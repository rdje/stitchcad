# G1 formula execution frontier

Canonical task/evidence sibling of [G1-SLICE](G1-SLICE.md), split by .5e.2.2 before growth.
The original working execution block was copied byte-for-byte; later entries remain live.

### Execution, uncertainty and replay
- ID: `G1-SLICE.5e.1`
  Status: `in_progress`
  D122/D127/D128 reference child .5e.1a verified; product input adapters below remain pending.
  Goal: explicit adapters for measurement/Ease/design/profile/material/size inputs and five
  authored states, preserving canonical identity, kind, source and provenance requirements.
  Acceptance: unknown has no value, derived reads follow their dependencies, assumed/preference
  remain labelled; all six bindable kinds supported, optional tolerance context explicit.
  Source truth/artifact blocking belongs G4; no invented policy or duplicated authored values.
  Full SizeSet/axis integration waits .4c.2 under confirmed D70, preserving size-context acceptance.
  Verification: `pending`; Commit: `pending`.
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
- ID: `G1-SLICE.5e.2`
  Status: `in_progress`
  Goal: single-pass declaration-order evaluator over statically validated graphs using .5c/.5d.
  Acceptance: always-visible versus prior names, stored numeric/Boolean reads, both-branch static
  validation but taken-only computation, unknown untaken succeeds/taken refuses with full context.
  Numeric domain/division/width errors occur at the actual completed operation; no reordering.
  Verification: `pending`; Commit: `pending`.
- ID: `G1-SLICE.5e.2.1`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part416.md).
- ID: `G1-SLICE.5e.2.2`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part416.md).
- ID: `G1-SLICE.5e.2.3`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part416.md).
- ID: `G1-SLICE.5e.2.h13`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part416.md).
  Retention0295 owned .6.2.2.b.2.a:
  - [x] ROOT CAUSE0295: git showe95fa7a whole four execution-prerequisite summaries retained416; Python byte/hash comparison rc=0.
  - [x] ADDRESSED0295: whole source/hash exact in416, Python comparison rc=0.
  - [x] NO REGRESSION0295: scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part416.md whole public file exact, Python comparison rc=0; no reader change.
- ID: `G1-SLICE.5e.2.4`
  Status: `pending`
  Goal: bounded source-order expression execution consuming actual node proofs and sourced values.
  Actual errors retain failing node/owner/canonical identity; taken-only if, all evaluated call
  inputs, exact numeric kernels, explicit source adapters. Trig/geometry remain actual dependencies.
  No absent implemented family or caller placeholder becomes a language refusal/canonical result.
- ID: `G1-SLICE.5e.2.5`
  Status: `pending`
  Goal: whole checked recipe declaration-order execution and real let/assert statement context.
  Publish a binding only after successful expression/storage; retain original sources/ordinals.
  Complete accepted result/atomic failure/replay must integrate .5e.4 before whole runtime closure.
- ID: `G1-SLICE.5e.3`
  Status: `in_progress`
  Goal: within/assertion execution and tolerance/irrational-result provenance propagation.
  D121 reference provenance .5e.3b verified; D125 reference assertions .3a verified.
  Independent reference D125 repair .5e.3a is verified; D124 recognition closes at .5b.1c.2.
  Acceptance: all five named classes, context-supplied missing values, T2-or-looser when an
  irrational result contributes (including subsequent binding/reads), exact bare comparisons,
  assertion failure values/class/name and no substituted value/geometry after failure.
  Verification: `pending`; Commit: `pending`.
- ID: `G1-SLICE.5e.3.1`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part413.md).
- ID: `G1-SLICE.5e.3.2`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part413.md).
- ID: `G1-SLICE.5e.3.3`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part413.md).
- ID: `G1-SLICE.5e.3.4`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part413.md).
- ID: `G1-SLICE.5e.3.5`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part413.md).
  Retention0294 owned .6.2.2.b.1:
  - [x] ROOT CAUSE0294: git show4e98649 whole five completed adapter summaries retained413; Python byte/hash comparison rc=0.
  - [x] ADDRESSED0294: whole source/hash exact in413, Python comparison rc=0.
  - [x] NO REGRESSION0294: scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part413.md whole public file exact, Python comparison rc=0; no reader change.
- ID: `G1-SLICE.5e.4`
  Status: `pending`
  Goal: atomic replay and dependency validation, including corrupted persisted cycle evidence.
  Acceptance: failure yields no accepted partial evaluation, repeat/from-statement1 byte equality,
  invisible-cache equivalence, cycle names and unbound/rebinding/ambiguity context. Loader/storage
  .7 consumes graph validation; canonical text alone is not a loader or authoritative graph.
  Verification: `pending`; Commit: `pending`.

- ID: `G1-SLICE.5e.3.6`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part407.md).
- ID: `G1-SLICE.5e.3.7`
  Status: `done`
  Goal: verified; [whole scope/provenance](../history/stitchcad-devnotes-part407.md).
