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
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part339.md).
- ID: `G1-SLICE.5e.1.2`
  Status: `done`
  Goal: canonical scalar state/value content and selected lint repair verified0262.
  Paths: crates/sc-core/src/value.rs;crates/sc-core/src/value/scalar.rs;crates/sc-core/tests/scalar_value_contract.rs;crates/sc-core/tests/formula_length_input_contract.rs
  Additional paths: crates/sc-core/src/recipe/input.rs;crates/sc-core/src/recipe/declaration.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/roots.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part341.md).
- ID: `G1-SLICE.5e.1.3`
  Status: `done`
  Goal: sourced general scalar reads/actual target-kind checks verified0263, native5/3.
  Paths: crates/sc-core/src/recipe/input.rs;crates/sc-core/src/recipe/input/scalar.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/src/recipe/declaration.rs;crates/sc-core/tests/formula_scalar_input_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part343.md).
- ID: `G1-SLICE.5e.1.4`
  Status: `done`
  Goal: intrinsic/explicit sourced optional tolerance reads verified0264, native5/0.
  Paths: crates/sc-core/src/recipe/context.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_tolerance_context_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part345.md).
- ID: `G1-SLICE.5e.1.5`
  Status: `done`
  Goal: pinned authored-membership size_index/size_count/is_base_size, without labels or axes.
  Children .1 core range/source/value context; .2 actual sc-measure membership bridge. D70 unchanged.
- ID: `G1-SLICE.5e.1.5.1`
  Status: `done`
  Goal: core size source/range/value frame and wide Count bridge verified0265.
  Paths: crates/sc-core/src/recipe/context.rs;crates/sc-core/src/recipe/context/size.rs;crates/sc-core/src/recipe/semantic.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/mod.rs
  Additional paths: crates/sc-core/tests/formula_size_context_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part347.md).
- ID: `G1-SLICE.5e.1.5.2`
  Status: `done`
  Goal: actual pinned membership/selected-id size bridge verified0266, native5/0.
  Paths: crates/sc-measure/src/size_membership.rs;crates/sc-measure/src/size_membership/formula.rs;crates/sc-measure/src/lib.rs;crates/sc-measure/tests/formula_size_membership_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part348.md).
- ID: `G1-SLICE.5e.1.6`
  Status: `done`
  Goal: exact canonical/stored scalar numeric bridges verified0267, native5/0.
  Paths: crates/sc-core/src/recipe/numeric.rs;crates/sc-core/tests/formula_value_read_bridge_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part349.md).
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
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part363.md).
- ID: `G1-SLICE.5e.2.2`
  Status: `done`
  Goal: checked node/expression/statement/atomic execution sequence recorded0274.
  Paths: docs/book/src/annexes/formula-execution-plan.md;docs/book/src/SUMMARY.md
  Acceptance: [whole scope/receipts/partition proof](../history/stitchcad-devnotes-part365.md).
- ID: `G1-SLICE.5e.2.3`
  Status: `done`
  Goal: immutable per-node kind/symbolic-role certificates in actual checked-expression owners.
  Preserve original spans/arena, all static lookup/error ordering and typed wanted alternatives.
  Runtime conditionals consume actual branch proofs, not supplied declarations as complete proof.
  Preserve actual privacy/lifetime compile-fail intent, not incidental missing-field failures.
  Focused static target only; no provider/value reads or forged foreign-node proof.
  Work unit0275.
  Paths: crates/sc-core/src/recipe/checked.rs;crates/sc-core/src/recipe/checked/nodes.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_checked_node_contract.rs
  Keep root API/dependencies/error precedence; add pinned typed node/form/argument views with exact
  resolved name sources and static builtin identities. No value/provider calls; complete proofs only.
  Focused new/existing expression targets, selected lint and relevant privacy/lifetime doc examples.
  Retain365 whole0274 task,366 oldest full built-in lesson,247 ledger; bounded new book guide/live sync.
  Book paths: docs/book/src/annexes/formula-checked-nodes.md;docs/book/src/SUMMARY.md and existing scope/status pages.
  Defect owned/fixed here, P2: implementation-status source rows after blank lines/prose render
  as plain <p>| paragraphs. Reproduce generated implementation-status.html canonical scalar rows;
  impact: formula availability map unreadable as a table. Join all original rows under one header,
  preserve every cell, verify actual rendered row families/cell counts; no broad publication campaign.
- [x] REPRODUCE:0275 de0bc89 retains only the root kind in its checked owner; actual field/source
  inspection rc=0. Status-map HTML has literal pipe paragraphs after blank table breaks, rc=0.
- [x] ROOT CAUSE:0275 consumed intermediate kinds need pinned certificates, not fresh namespace
  queries; closed arena traversal resolves every actual node/source/callee. Native6+7 pass, rc=0.
- [x] FIX: per-node certificates and private typed child/argument views; root API/error order held.
  Status map joins every original row under one header, preserving cell bytes/order.
- [x] ADDRESSED:0275 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_checked_node_contract --test formula_checked_expression_contract:6+7 pass/0 fail, rc=0.
- [x] NO REGRESSION:0275 cargo clippy --offline -p sc-core --test formula_checked_node_contract
  --test formula_checked_expression_contract -- -D warnings and cargo test --offline -p sc-core
  --doc recipe::checked:: pass0 (8 scoped docs); native source/role/limit/whole-statement controls rc=0.
  Broad .5g/pre-push remains owned. Book render and actual HTMLParser four-cell row/order check rc=0.
- [x] LOCKSTEP:365/366/247 whole Git/hash/public reads rc=0; book/availability/plan/live/frontier align.
  Promotion declined: existing static-owner, source role and metadata-only contracts; no runtime claim.
- ID: `G1-SLICE.5e.2.h13`
  Status: `pending`
  Goal: before .2.4 history growth, capture whole eligible records in archive window13 at fixed64 cap.
  Preserve prior12 windows, exact source/reader/materialization and all logical identities; no cap bump.
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
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part352.md).
- ID: `G1-SLICE.5e.3.2`
  Status: `done`
  Goal: sourced numeric/Boolean operators/bindings verified0269, native5/0.
  Paths: crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_evaluated_scalar_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part354.md).
- ID: `G1-SLICE.5e.3.3`
  Status: `done`
  Goal: sourced implemented numeric call families verified0270, native5/0.
  Paths: crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/evaluated/calls.rs;crates/sc-core/tests/formula_evaluated_calls_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part356.md).
- ID: `G1-SLICE.5e.3.4`
  Status: `done`
  Goal: exact sourced named tolerance comparisons verified0271, native5/0.
  Paths: crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/evaluated/tolerance.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_tolerance_comparison_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part358.md).
- ID: `G1-SLICE.5e.3.5`
  Status: `done`
  Goal: actual named assertion success/refusal verified0272, native4/0.
  Paths: crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/evaluated/assertion.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_assertion_value_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part361.md).
- ID: `G1-SLICE.5e.4`
  Status: `pending`
  Goal: atomic replay and dependency validation, including corrupted persisted cycle evidence.
  Acceptance: failure yields no accepted partial evaluation, repeat/from-statement1 byte equality,
  invisible-cache equivalence, cycle names and unbound/rebinding/ambiguity context. Loader/storage
  .7 consumes graph validation; canonical text alone is not a loader or authoritative graph.
  Verification: `pending`; Commit: `pending`.

