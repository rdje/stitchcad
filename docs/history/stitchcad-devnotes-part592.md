# Sealed engineering continuity — part592

- **Identity:** 113 lines, 9458 bytes, sha256:f4d45166c1cf9eb4335142b081d0d5ec004c18cb88005a1834ada59dac7dd925
- **Coverage:** whole0440 scope/lesson/ledger, D287/D288 ledger and0432–0436 decision continuity from f7ecf9b455a8fa5eeb2ffd7686817d50c402c5dc.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 43435–46136, 2701 bytes, sha256:6e2a3e4b0828025078f88f32cbbd2e94800541873522d6fe1c8621d4670cb629
- Whole source interval: DEV_NOTES.md bytes 193–1081, 888 bytes, sha256:e6e18c31493436e9b2923dd0e90c5a21c11b7c486657a257b9ba317e16892fb4
- Whole source interval: CHANGELOG.md bytes 207–820, 613 bytes, sha256:0f6e38813986e527b91cebd4a3056058e3a616d1d9df98178b261248dd684c93
- Whole source interval: docs/tasks/PLANNING.md bytes 246–1335, 1089 bytes, sha256:dfaa5c0e12c781852b1687e2293877732ecfb5b827c6b517d6438b49807d2350
- Whole source interval: docs/decisions/decision_command-layer-contract-and-undo-granularity.md bytes 6686–10853, 4167 bytes, sha256:3f3fef82f008fc00fdfcb23efcafed5ac12bd0b3875b75a30268b7508fb2deb5
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.2.1`
  Status: `done`
  Goal:0440 D287 explicit cumulative Measurement evidence-reference admission; D288 package metadata sync.
  Fourth positive immutable host maximum counts original Known-state vectors/aliases without copying,
  deduplication or source lookup. Preserve exact excess context and failed keys; no production default.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-core --test command_control_contract
    known_measurement_evidence_cannot_bypass_the_explicit_admission_policy:0 passed/1 failed rc=101;
    native prior three maxima admit two distinct references with1 member/1 target/0 reason bytes.
  - [x] ROOT CAUSE (WHY + WHERE): transition_receipts.py --record docs/history/stitchcad-devnotes-part591.md:
    4 exact Git spans rc=0; prior whole0439 policy explicitly bounds only three dimensions. Known-state
    evidence vector is a separate submitted payload dimension, not a primary metadata target or reason.
  - [x] FIX: cargo clippy -p sc-core --test command_control_contract -- -D warnings:finished rc=0;
    explicit fourth constructor maximum, evidence demand/refusal/getters, remaining-room guard and
    exhaustive authored-state classification; all native call sites explicitly configure the new maximum.
  - [x] ADDRESSED: cargo test -p sc-core --test command_control_contract:6 passed rc=0; cargo test
    -p sc-measure --test design_inputs_contract command_bus:45 passed rc=0. Exact bound/cumulative alias
    excess retained; single/group preview, commit and history refuse before any authority read/progress.
    Original Piece pointer/revision remain; same authoring/history keys/capacities later succeed, sequence1.
  - [x] NO REGRESSION: selected sc-measure strict Clippy/fmt finished rc=0; cargo test -p sc-core --doc
    command::control:1 passed rc=0; native evidence case rechecked1 passed rc=0 after replacing a fixture
    indexing lint with nonempty first().expect, without suppression. Prior44 native bus cases still pass.
  - [x] LOCKSTEP: bash scripts/run_make.sh book and native publication:81 chapters/159 scoped API rows/
    1472 source links/2466 rendered links rc=0. Native D288 prior/current stale-description census1/0 rc=0;
    sc-core Cargo metadata now names the implemented metadata subset. Retention1045/global75 rc=0.
  Cargo commands use scripts/local_environment.py. Promotion: indexed command-layer decision.
  Whole0439/D285/D286 ledger retained591 unchanged. Admission acts on constructed typed input; raw
  decoding/initial structural construction, whole-model/resource/retained bytes remain .6.8.2.2 work;
  deeper source/allocation .6.5.5 and full .6.9 remain deferred. G1still6/18, no broader feature proof.
## _(2026-10-27 UTC)_ — explicit submitted evidence-reference admission

0440 .6.8.2.1 owns D287: native baseline admits two Known-state evidence references as1 member,
1 target/0 reason bytes; the desired regression fails101. Control6/bus45 pass0; selected lint/fmt/privacy1 pass0. Add a fourth explicit positive host maximum,
count cumulative original vectors/aliases before source/model work, retain exact first excess, and
leave refused keys/capacity unused. Fixture indexing lint corrected with first().expect. No copying or registry truth claim in admission. D288 corrects
stale sc-core package metadata that deferred already implemented mutations/history. Promotion:
command-layer record. Typed-input admission excludes raw decoding/construction and whole-model work;
remaining resource implementation .6.8.2.2, deeper/full acceptance .6.5.5/.6.9. Whole0439/D285/D286 proof591.

## STITCHCAD-G1-0440 — bound submitted Measurement evidence references

- D287: fourth explicit host maximum bounds cumulative Known-state references, including original aliases.
- Refuse before authority/model work on every costly metadata path; retain exact first excess and unused keys.
- Native control6/bus45, selected strict lint/fmt/control privacy1 and book/publication pass.
- D288: sc-core package metadata names implemented metadata operations/history rather than deferring mutations.
- Whole0439 and D285/D286 ledger retained591 with4 exact native Git spans; complete resource scope remains owned.

## D287 — submitted Known-state Measurement evidence bypasses input limits
P1 fixed0440, owner G1-SLICE.6.8.2.1: independent explicit evidence quota counts original vectors/aliases.
- [x] ROOT CAUSE: cargo test -p sc-core --test command_control_contract known_measurement_evidence:
  baseline0 passed/1 failed rc=101; prior native quotas admit two references as1 member/1 target/0 bytes.
- [x] ADDRESSED: cargo test -p sc-core --test command_control_contract:6 passed rc=0; native bus45
  passed rc=0; all four costly entries refuse evidence excess before authority/progress and leave keys unused.
- [x] NO REGRESSION: selected strict Clippy/fmt finished rc=0; control privacy1 passed rc=0.
D288 P2 fixed0440 same owner: native stale sc-core package-description census1/0 rc=0; actual subset named.
D285 runtime fixed0437; persistence/reopen issuance/order remains owned/pending in G1-SLICE.7.
[Whole D285/D286 diagnosis and verified runtime/claim closure](../history/stitchcad-devnotes-part591.md).
D283/D284 fixed; [whole correction/proofs](../history/stitchcad-devnotes-part581.md).

## Sequential metadata group preview0432

answers: "how do metadata groups stage actual models and retries?" · "how is the net group delta derived?"

G1-SLICE.6.5.4.2.2.3 uses one actual trusted authority snapshot/complete Inspect grant, then binds
actual Design/exact revision. Each first request consumes the preceding validated candidate and one
independent concrete reserve cursor. Per-member original guards and typed error sources remain;
a failure retains original group/index/command and discards every staged candidate/report.
Within one preview only, complete CommandMutation equality under a repeated key returns the first
report with replay_of; conflicting reuse refuses before reapplication. No persistent replay is claimed.
The final diff reads original/final actual models, permits only successful nonalias semantic footprints,
preserves complete other owner fields/order and omits restored values. Native mixed progression,
restoration/nonempty deltas, exact aliases/conflicts, later anchor/exhaustion, source refresh and
binding order pass in six group cases. Publication/audit/undo/evaluation remain their later owners.

## In-memory metadata publication0434

answers: "when is a metadata group published?" · "how does exact retry work after revisions advance?" · "what bounds the in-memory audit?"

G1-SLICE.6.6.1 requires explicit finite accepted-group capacity. Complete Commit authority and Design
binding precede journal lookup. Exact group id plus complete original group equality returns the first
immutable result after fresh authorization, before revision/capacity checks. Changed same-id groups
refuse; keys belong to this Design/complete-actor namespace, and cross-group overlap refuses rather
than replaying or publishing part. Journal entries are never discarded at capacity; new groups refuse.
New edits require exact current revision; original mismatch carries actual available advances/commands
and initial journal revision, with no fabricated earlier trail. Prepare/rebuild/current-check the actual
successor model and reserve/allocation preflight, then refresh complete Commit authority immediately
before infallible exclusive model/reserve/audit moves. Retain old host through the complete transition.
One accepted group adds one immutable original actor/request/result/grant/time entry and advances once,
including net no-op. Exact replay adds neither. Native commit6/previous24/diff5/lint/fmt are focused
proof. Persistence/reopen, bounded undo, byte/work/cancellation limits and full integration remain owned.

## Revision identity across undo branches — D285

answers: "how can undo restore counters without accepting stale branch requests?"

0435 .6.7.1.1 pins the native57767b0 contract/code and reproduces a counter model: restore0 after
snapshot1, then current.successor gives a distinct snapshot1 that accepts the old revision precondition.
Undo must preserve a separate maximum-issued counter; new authoring allocates above it, while undo/
redo restore original snapshots/counters. Original counter/identity restoration remains unchanged.
The model probe is diagnosis, not runtime proof. Runtime fix/native branch refusal is verified0437
under .6.7.1.3; storage/reopen .7 must still retain issuance independently of undo history.
The read-only model probe remains diagnosis. Native in-memory traversal/branch acceptance passes0437; storage/reopen proof remains pending.

## History intent and operator0436

answers: "how do current history operator and original author remain distinct?"

G1-SLICE.6.7.1.2 owns current operation context/direction and exact original CommandMutationGroup.
Explicit membership then Design must match; operator/revision may differ from original author/base.
Both complete actor/principal claims remain untrusted and exact; original payloads/keys/order remain.
Native schema3/privacy1/lint/fmt pass. Runtime must authenticate current operator for every original
command scope, bind the exact accepted record/current revision/key and enforce bounded atomic effects.
Input construction is no history capability; apply_history separately enforces traversal/action audit and D285 issuance.

