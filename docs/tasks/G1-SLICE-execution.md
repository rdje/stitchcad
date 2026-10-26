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
  Goal:0274 checked-AST execution/interface sequence before joining value primitives.
  Paths: docs/book/src/annexes/formula-execution-plan.md;docs/book/src/SUMMARY.md
  Review actual checked/normalized/statement/recipe and value interfaces; keep source/identity,
  static-before-value, selected-only execution and remaining trig/geometry/input obligations explicit.
  Own bounded implementation children below; no new numerical backend or runtime API adopted here.
  Retain363 whole0273 task,364 oldest complete unary/comparison lesson,246 ledger; book/live sync.
  Own whole execution-frontier split to docs/tasks/G1-SLICE-execution.md before further growth;
  paths/index/map updated with byte-exact current-block verification and fixed caps.
- [x] REPRODUCE:0274 actual checked.rs proof fields retain root kind/dependencies; normalized
  and statement/recipe APIs preserve owners/ordinals. Focused source inspection rc=0.
- [x] ROOT CAUSE:0274 runtime must consume actual child proofs, not supplied metadata as AST
  acceptance; documented node/expression/statement/atomic sequence follows those interfaces, rc=0.
- [x] FIX: bounded next children and explicit existing numerical/source/geometry integration owners.
- [x] ADDRESSED:0274 bash scripts/run_make.sh book builds new execution sequence warning-free, rc=0.
- [x] NO REGRESSION:0274 no Rust/API/backend/limit change; whole frontier copy9773B/133L equals
  captured working block sha256:03c65d647e279c6397d60fc0acbac0d6b226865917343be2880189d272cb2f87, rc=0.
  Existing static/value/runtime integration and broad .5g/pre-push checks retain their owners.
- [x] LOCKSTEP:363/364/246 Git/hash/public reads rc=0; book/main task/index/map/live aligned.
  Promotion declined: existing checked-owner, static-before-value, numerical and atomic contracts.
- ID: `G1-SLICE.5e.2.3`
  Status: `pending`
  Goal: immutable per-node kind/symbolic-role certificates in actual checked-expression owners.
  Preserve original spans/arena, all static lookup/error ordering and typed wanted alternatives.
  Runtime conditionals consume actual branch proofs, not supplied declarations as complete proof.
  Preserve actual privacy/lifetime compile-fail intent, not incidental missing-field failures.
  Focused static target only; no provider/value reads or forged foreign-node proof.
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

