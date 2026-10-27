# Sealed engineering continuity — part596

- **Identity:** 125 lines, 9886 bytes, sha256:bfda602727b7691ae8c88a823d4085180b419ee18833320e42a15d54d62788a1
- **Coverage:** whole0444 canonical-resource scope/lesson/ledger and0437–0442 decision continuity from 04a4dbb9ec3c16d348f9d4c9615796a843ea6095.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 45193–48148, 2955 bytes, sha256:24bbbd9820ce991a5562b44a08dda43e6f504506043ee378e698681dd1f65851
- Whole source interval: DEV_NOTES.md bytes 193–973, 780 bytes, sha256:40b4e15036b767fc6641fcd8bd010176e34e0db22368f857cd1aa96fbbcaacfc
- Whole source interval: CHANGELOG.md bytes 207–725, 518 bytes, sha256:2cdf2506b9dc4404b385ac962be3198e5926e0d7fcde062fc4e4bbda140723b9
- Whole source interval: docs/decisions/decision_command-layer-contract-and-undo-granularity.md bytes 6895–12528, 5633 bytes, sha256:7d3608e6110c4e912ce3db9aec2228c5d6ce7df96db86262a0bd59bba879bf89
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.2.2.2.2.2.1`
  Status: `done`
  Goal:0444 actual canonical Length/Scalar inventory and Known evidence-vector resource producer.
  Exhaustive internal owner view and declaration/state patterns preserve future-field obligations;
  fixed value payloads must remain Copy. Borrow only, precharge before traversal, no numeric availability,
  registry lookup, copying, normalization or changed declarations/source/state/value/provenance/order.
  - [x] REPRODUCE / ISSUE: python3 -I -B native HEAD canonical/resource assertions print actual
    length/scalar ownership and earlier meter-only module rc=0; this leaf adds the explicit canonical
    producer to that module rather than treating declared demo charges as an actual model walk.
  - [x] ROOT CAUSE (WHY + WHERE): transition_receipts.py --record docs/history/stitchcad-devnotes-part595.md:
    3 exact Git spans rc=0; whole0443 scope/lesson/ledger preserves its actual-visitor-pending boundary.
    Canonical vectors and per-Known evidence are logical dynamic entries; fixed primitive values are not.
  - [x] FIX: cargo clippy -p sc-core --test design_resource_contract -- -D warnings:finished rc=0;
    exhaustive borrowed owner view, complete original state matches, Copy-value constraints and length-
    then-scalar precharge preserve exact first resource location/index without model/authority fabrication.
  - [x] ADDRESSED: cargo test -p sc-core --test design_resource_contract:10 passed rc=0. Four actual
    canonical cases verify five Length/all authored states and six Scalars/computed Area, demand15/0/0,
    reused evidence entries counted twice, max14 scalar-evidence13+2/index0, whole-vector max4 refusal,
    max6 length-evidence refusal, unchanged input, actual empty scope and prior-error composition.
  - [x] NO REGRESSION: selected cargo fmt check rc=0; strict selected lint rc=0; git diff --quiet HEAD
    -- crates/sc-core/src/command.rs crates/sc-core/src/command rc=0. Original six meter cases still pass;
    runtime policy/enforcement, metadata candidate/source/authority/cancellation paths are unchanged.
  - [x] LOCKSTEP: whole0443 scope/lesson/ledger595 retained unchanged with3 exact Git spans rc=0;
    book/pointer/frontier/lesson/indexed decision aligned. Remaining core components .2–.5, source-host
    .3 and runtime admission .4 stay owned; deeper .6.5.5 and full .6.9 acceptance remain deferred.
  Cargo invocations use scripts/local_environment.py. Promotion: indexed command-layer decision.
  Native book/publication81 chapters/159 scoped API rows/1475 source links/2473 rendered links rc=0;
  maintained-reference82 files/18204 lines/1270093 bytes owner .6.8.2.2.2.2.2.1 rc=0.
  Retention1049 logical/79 aggregate working Markdown rc=0; active generation fits64/group.
  This is actual canonical-component accounting, not complete model/source traversal or runtime
  enforcement, evidence truth, allocator RSS/time or publication authority.
## _(2026-10-27 UTC)_ — actual canonical dynamic-content producer

0444 .6.8.2.2.2.2.2.1 borrows actual canonical Length/Scalar inventories and Known evidence vectors,
precharges exact lengths and retains authored field/index order. Exhaustive owner/state patterns
and Copy-value constraints prevent silently omitting new owned payloads. No numeric availability,
registry, cloning or changed state/provenance. Resource10 (prior6/actual4)/strict selected lint/fmt pass;
actual mixed-state demand15/0/0, first refusals, empty/prior-error composition and unchanged owner proven.
Promotion: command-layer record. Other model/source/runtime work remains owned; no whole-model/RSS/
time/evidence certificate. Whole0443 contract scope/lesson/ledger retained595 with3 exact Git spans.

## STITCHCAD-G1-0444 — account for actual canonical input resources

- Borrow actual length/scalar inventories and Known evidence vectors; precharge in original authored order.
- Exhaustive owner/state views and Copy-value constraints; retain first field/index refusal, no numeric/source lookup.
- Resource10 (prior6/actual4), strict selected lint/fmt pass; actual state/provenance/order unchanged, bus unchanged.
- Book scopes canonical-component proof; whole0443 logical-contract scope/lesson/ledger retained595.

## Actual bounded snapshot history0437

answers: "how does undo preserve complete original semantics?" · "how are history effects authorized and ordered?"

G1-SLICE.6.7.1.3 moves actual predecessor/current models through bounded alternatives; no inverse is
reconstructed. Explicit finite depth/action limits, exact operation replay and shared complete-actor
key scope preserve append-only original/action records. Bind the complete submitted original to its
accepted record and selected stack top, authenticate current operator across every original command
scope, validate actual current/alternative models, preflight allocations and refresh authority at effects.
Original author remains distinct from current operator. Report oldest undo eviction/discarded redo.
Model counters restore, while maximum-issued authoring revision/entity ids never rewind. Global exact
audit sequence orders authoring/Undo/Redo independently of timestamps and restored counters; stale
witnesses share actual event ordering. Native bus37 (prior30/history7), privacy7/lint/fmt cover this
metadata subset, including D285 undo0/new2/old1 refusal and pointer/canonical roundtrip. Storage/reopen
must persist issuance/order; work/cancellation and deeper/full acceptance remain separately owned.

## Explicit metadata admission/progress contracts0438

answers: "what do initial operation limits measure?" · "does progress data itself cancel or authorize work?"

G1-SLICE.6.8.1.1 admits original submitted members, primary metadata selections and cumulative proposed
UTF-8 assignment-reason bytes under explicit positive host limits. Aliases still count. Member excess
precedes per-member target/byte excess; guard remaining room before arithmetic and retain exact factors.
Typed phase-local fractions require positive total/completed<=total. Data-only observer returns Continue/
Cancel and carries no model/authority capability. Native control4/privacy1/lint/fmt pass; runtime bus is
unchanged. Immutable installation/actual emission/atomic cancellation follow .6.8.1.2. These inputs do
not prove CPU time, total model memory, allocation peaks, numeric/file/decompression or solver bounds.

## Actual immutable operation control0439

answers: "where does metadata cancellation become effective?" · "may a request widen installed limits?"

G1-SLICE.6.8.1.2 installs policy exactly once with no default or request override. Admit original typed
metadata input before private source/model work, then emit actual phase-local data-only checkpoints.
Cancel discards private work without changing semantic model/reserve/audit/history/issuance/keys;
internal reserved capacity may grow. Prepare closed audit/result copies before ReadyToPublish and
fresh final complete effect authority. No observer/large preparation follows that authority check.
Exact replay refreshes entry authority and emits Replaying without new effects. Cheap authorized Piece
reads require no policy. Native bus44/privacy8/lint/fmt pass, including observer revocation at Ready.
Input counts and cooperative checkpoints do not prove total model/work/memory or wall-clock bounds;
.6.8.2 owns the remaining resource implementation, .6.5.5/.6.9 deeper/full acceptance.

## Measurement evidence admission0440

answers: "are submitted Measurement evidence vectors covered by target counts?"

D287 native baseline shows they are independent: a Known replacement carries two references while
member/target/reason demand stays1/1/0. G1-SLICE.6.8.2.1 adds a fourth explicit positive host maximum
and cumulative evidence-reference demand/refusal, including exact aliases. Preserve original vectors,
counts and first member/index/command; compare remaining room before adding, without source lookup.
All costly metadata entry paths share this admission before authority/model work. Refused keys remain
unused. No evidence truth/registry grant is implied. This bounds already typed input, not raw decoding,
initial construction, whole-model validation/cloning or retained-state memory; those remain separately owned.

## Bound source-host reconstruction copies0441

answers: "why consume copied inner Design owners instead of cloning them again?"

D289 native real-source instrumentation measured individual H copies3/3/2 in Measurement/Material/Notch
previews. G1-SLICE.6.8.2.2.1 consumes copied Structure/Inputs/Sewing definitions privately and rebuilds
through unchanged validators. Bus reconstruction now requests one H copy per non-alias candidate and
one extra for a new revision; replay and actual Undo/Redo request none. Explicit member policy bounds
candidate count. Native bus46 and privacy3/4/4 verify copy counts and no public extraction. This does
not bound trusted host-internal bytes/time, all core collection work or retained memory; .6.8.2.2.2 owns those.

## Shared reserve publication0442

answers: "does committing metadata require copying all unused reserved identities?"

D290 native source/caller census shows prior after_consuming recopied the entire unused Vec even for
zero creation. G1-SLICE.6.8.2.2.2.1 retains one immutable shared backing and publishes a checked forward
prefix. Independent borrowed cursors preserve local consumption/exhaustion; publication shares the
backing without copying unused IDs, and history never rewinds issuance. Cursor1/native bus46 and
selected lint/fmt pass. Initial backing including consumed IDs remains allocated for bus lifetime;
this trades finite initial retention for removal of per-commit backing copies, not a total resource,
initial-conversion or timing certificate. D291's standalone preview explicitly installs operation policy.

