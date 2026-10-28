# Sealed engineering continuity — part629

- **Identity:** 104 lines, 9551 bytes, sha256:7b4af79fb2b2dfeda4bd9bcfeabf8e3c4ad64ae637108b90a17b335ee4bb5a60
- **Coverage:** whole0474 transaction scope, lesson, changelog and0471–0474 decisions from 4f9976e18d29494f8318d3cfb12a0f619991716f.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 40192–41978, 1786 bytes, sha256:238d3359d5c979618c827fbb9dfe46bca83aa9bec503e817c1bc4f84688e953e
- Whole source interval: DEV_NOTES.md bytes 193–1409, 1216 bytes, sha256:f6cc26fc2f118502bfea0757e7bab7c437733b2fa564deb462863aaaffcc4a31
- Whole source interval: CHANGELOG.md bytes 207–974, 767 bytes, sha256:257a660bc55d2780bee4da32e01688fa2d9cbfd8421db3910ff12a670b367217
- Whole source interval: docs/decisions/decision_command-layer-contract-and-undo-granularity.md bytes 8925–14707, 5782 bytes, sha256:c2312799031f4ed0ff46556088f5151909f15210541386d5444f5219978704b6
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.5`
  Status: `done`
  Goal: scoped actual transaction/failure-context/rollback acceptance; full bus review .6.9 later.
  Prove every observed pre-effect checkpoint cancellation on a mixed real metadata group, exact
  later-member native error context, same-key/identity/audit-slot retry and original snapshot Undo.
  - [x] REPRODUCE / ISSUE: actual mixed reference trace emits37 pre-effect events across9 required
    phases;native effect-boundary check confirms sole final authority/effect ordering,core unchanged rc=0.
  - [x] ROOT CAUSE: python3 -I -B scoped source check rc=0:private candidates/cursor stage before
    final grant;publication moves are infallible. New actual fixture exercises all observed checkpoints.
  - [x] FIX: add public-interface transaction acceptance over real complete sources/models, three
    handlers/five members, exact alias, finite two-ID/one-slot ownership and independent native error.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus::transaction_acceptance::
    →2 passed rc=0;all37 cancel/retry/replay/Undo cases and member3 original Notch error/corrected retry pass.
  - [x] NO REGRESSION: cargo test selected group_preview/group_commit/control →6/6/8 passed rc=0;
    cargo clippy measure contract -D warnings and cargo fmt -p sc-measure --check pass0;production unchanged.
  - [x] LOCKSTEP: book publication85 chapters/159 API rows/1544 source/2599 rendered links rc=0;
    live docs/decision/index updated,LIVE transaction note,README unchanged;whole0473/D294 retained628.
  Verification: above focused checks pass0;retention1082/2415074residentB rc=0;full bus .6.9 next.
  Commit: STITCHCAD-G1-0474;no allocation exhaustion/panic/persistence/browser/other-handler certificate.
## _(2026-10-28 UTC)_ — mixed transaction cancellation and rollback acceptance

0474 .6.5.5 uses real Notch/Material/Measurement/Notch plus exact alias, two reserved IDs and sole authoring/undo/action slots. Actual reference37 pre-effect events span9 required phases; cancelling each retains original event context and exact trace prefix.
All37 cancellations preserve model/address/revision, same-key retry/full reference report, both IDs, capacity replay and original-address Undo. Separate member3 Notch failure after all3 handlers preserves the original error chain and corrected same-key retry.
New2/group-preview6/group-commit6/control8, strict selected lint/fmt, native final-grant/infallible-effect source check pass0. No production source change; allocation exhaustion/panic recovery/persistence/browser/other-handler proof is not claimed.
Promotion:indexed command decision;book85/159/1544/2599,retention1082/2415074B pass0. Whole0473/D294 in628(4 exact Git spans);0473 resource252 counts historical. Next full bus .6.9;LIVE transaction note,G1still6/18;README unchanged.
[Whole0473 resource-acceptance lesson](docs/history/stitchcad-devnotes-part628.md) retains actual integrated proof and D294 closure.
## STITCHCAD-G1-0474 — verify mixed transaction cancellation and rollback

- Cancel each of37 observed pre-effect events across9 required phases in a real mixed metadata/alias group; preserve exact error context, original model and trace prefix.
- Same group/keys/two-ID/one-slot retry matches the full reference commit; capacity replay and original-address Undo pass. Later native failure at original member3 retains its complete cause and corrected retry.
- New2/group-preview6/group-commit6/control8/strict lint/fmt/native effect-boundary/book checks pass; production sources unchanged.
- Whole0473/D294 retained628; historical resource checkpoint counts labeled. Next broader bus acceptance .6.9; no allocator-fault/panic/persistence/browser or release proof.

## Exact prospective history0471 and source-clone defect D293

answers: "what changes in retained state when Undo or Redo publishes?" · "can semantic Clone equality certify charged source footprint?"

Borrow the already validated actual target/request/accepted record. Restore target as current; omit
its actual source-stack back, retain the other alternatives in order and append previous current to
the reciprocal stack. Preserve journal initial/maximum and accepted entries; forecast only sequence
and the new action, including effective commands, original author request/aliases and current operator.
Precharge whole vectors before walks; no clones/collection/counter construction occurs in projection.
RetainedPublication0/1 precedes publication copies/reserves and the final fresh grant/effect.
History4/current4/resource-admission4/lint/fmt pass0; real oracles and max1797/action642bits are in the book.

P0 D293 is confirmed by a plain adapter over real DesignSources plus an honestly charged String cache.
Each Clone leaves its source immutable and returns semantic-equal data with cache1→2→3. Native diagnostic1
passes0 by demonstrating forecast907B accepted but actual retained908B, followed by current882+26 refusal.
[Rust Clone/PartialEq](https://doc.rust-lang.org/std/clone/trait.Clone.html#clone-and-partialeqeq) requires
semantic equality, not identical cache footprint (that implication is our inference, tested natively).
Thus projection logic is focused-verified but generic publication bounds remain defective. Owned P0
.4.3.4.3 freezes actual source ownership before .4.4 acceptance; a diagnostic pass is not a fix verdict.

## Actual immutable source ownership0472 closes D293

answers: "why share the source instance rather than trust H Clone shape?" · "what compatibility changes accompany frozen snapshot sources?"

Private DesignMetadata stores Arc<H> and manually clones that handle plus the captured census Vec.
No H.clone callback participates in model snapshots. Semantic equality alone cannot certify an honest
cache's content/size; a post-clone rejection would occur after the publication copy and leave the
assumption in place. Sharing the exact immutable source removes it at the source-owner boundary.
Core model collections still copy; each logical model charges full source/census without deduplication.
The source integration obligation remains immutable/honest/exhaustive; this is not hostile-code isolation.

host() still lends &H and exposes no Arc/mutator; it becomes an ordinary method instead of const fn.
Cross-thread ownership now requires H:Send+Sync; single-thread H remains supported. This follows the
[standard Arc contract](https://doc.rust-lang.org/std/sync/struct.Arc.html); no unsafe trait override is used.
Native actual DesignGarment<DesignSources> satisfies Send+Sync. Metadata Clone itself requires no H:Clone;
existing model/command generic bounds stay as before in this focused repair.

Two regressions fail against the old inline owner and pass after:source address remains identical,
census vectors differ, two models charge exactly twice the content, explicit foreign Clone grows its
own cache,905B refuses original880+26,906B/old907B publish and pass subsequent current admission.
The independent clone telemetry control increments1; all actual preview/group/commit/replay/Undo/Redo
foreign clones are0. This count is no longer a proxy for core model copies; affected tests/book say so.
Affected command25/metadata8/doctest4/lint/fmt pass0. D293 closes; coupled .4.4/deeper/full proof stays owned.

## Coupled native logical-resource acceptance0473

answers: "which integrated resource checks passed after feature completion?" · "does native resource acceptance certify the whole command layer?"

After current/prospective models, retained state and D293 source ownership are implemented, the actual
Design-input158(including93 bus)/source33/core resource47/control6/metadata8 suites all pass:252 cases.
Five private model/audit forecast, exact bit/entry chunk and reserve-cursor units plus16 bus/resource/
control privacy doctests pass. Strict selected test lint also passes. Book evidence links the native
commands and limits. Broader transaction .6.5.5/full bus .6.9 follow; no full CI/browser/persistence/
ingress/allocator-time/physical/release verdict follows from this scoped acceptance.
D294 exposed stale component-only boundaries in the model/source resource chapters. Native precise
pending-clause census model11→0/source3→0 and the rendered publication check pass0; actual component
proofs remain scoped and historical counts stay labeled. Whole0472/D293 closure is retained627.

## Actual mixed transaction acceptance0474

answers: "how are all actual pre-effect cancellation points checked?" · "can failed mixed preparation consume identities or its sole audit slot?"

Trace one successful actual five-member Notch/Material/Measurement/Notch/alias transaction. Require all9
public pre-effect phases; the fixture emits37 events. Cancel each actual event on a fresh owner, compare
its exact original Cancelled cause and trace prefix, then commit the unchanged original group/keys.
The whole actual commit equals the reference, both original IDs issue, capacity replay works and Undo
restores the original Piece address. A separate fourth-member native Notch failure after all3 handlers
retains group/index/command and the full typed source chain; correcting the same keys gives the reference.
New2/group-preview6/group-commit6/control8/strict lint/fmt/native effect-boundary checks pass0. Production
source stays unchanged. Allocation exhaustion/panic recovery/persistence/browser/other handlers remain
outside this proof. Resource checkpoint0473 counts stay historical; full bus .6.9 follows.
