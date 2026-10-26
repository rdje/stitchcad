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
  Full SizeSet/axis integration waits .4c.2 D70, without dropping size-context acceptance.
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
  Children .1 core range/source/value context; .2 actual sc-measure membership bridge. D70 unchanged.
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
  Acceptance: [whole scope/six proof boxes/book-map repair](../history/stitchcad-devnotes-part367.md).
- ID: `G1-SLICE.5e.2.h13`
  Status: `done`
  Goal:0276 window13 packs52 whole records; prior12/source/read/materialization exact.
  Paths: docs/history/;.doctrine/history_archive/;docs/tasks/artifacts/history_archive/capture_window13.py
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part370.md).
  Retention0277, owned .5d.1.2.2.2.a:
  - [x] ROOT CAUSE0277: git show3d97348:docs/tasks/G1-SLICE-execution.md interval5838:8312 is whole2474B, Python hash comparison rc=0.
  - [x] ADDRESSED0277: retained370 body/source/hash exact, Python comparison rc=0.
  - [x] NO REGRESSION0277: scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part370.md returns whole file byte-exact, Python comparison rc=0; no reader change.
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
- ID: `G1-SLICE.5e.4`
  Status: `pending`
  Goal: atomic replay and dependency validation, including corrupted persisted cycle evidence.
  Acceptance: failure yields no accepted partial evaluation, repeat/from-statement1 byte equality,
  invisible-cache equivalence, cycle names and unbound/rebinding/ambiguity context. Loader/storage
  .7 consumes graph validation; canonical text alone is not a loader or authoritative graph.
  Verification: `pending`; Commit: `pending`.

- ID: `G1-SLICE.5e.3.6`
  Status: `done`
  Goal:0281 sourced sin/cos exact-axis/storage/T1 propagation native6/0/lint0.
  Paths: crates/sc-core/src/recipe/evaluated/calls.rs;crates/sc-core/tests/formula_evaluated_calls_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part381.md).
  Retention0282, owned .5d.1.2.2.2.b.2.a:
  - [x] ROOT CAUSE0282: git show83de197 whole sourced sincos node retained381; Python source/hash comparison rc=0.
  - [x] ADDRESSED0282: complete node381/source/hash byte equality, Python comparison rc=0.
  - [x] NO REGRESSION0282: scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part381.md whole file exact, Python comparison rc=0; no reader change.

- ID: `G1-SLICE.5e.3.7`
  Status: `done`
  Goal:0289 sourced inverse exact axes/both operand sources/storage/T1/kind priority verified8/0/lint0.
  Paths: crates/sc-core/src/recipe/evaluated/calls.rs;crates/sc-core/tests/formula_evaluated_calls_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part398.md).
  Retention0290 owned .5d.1.2.2.2.d.1:
  - [x] ROOT CAUSE0290: git showcbbfb7c whole0289 task retained398; Python source/hash comparison rc=0.
  - [x] ADDRESSED0290: whole node/source/hash exact in398, Python comparison rc=0.
  - [x] NO REGRESSION0290: scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part398.md exact whole file, Python comparison rc=0; no reader change.
