# Sealed engineering continuity — part789

- **Identity:** 112 lines, 11433 bytes, sha256:e2beb9d5c52badca4498fcc2ec3d4bfd9be730644da7cf5b8b2d8020b65d0a6f
- **Coverage:** whole0629 task scope, lesson and changelog from 0ace3c44694d5b8dc8d1a9e396a4932cfe0447a9.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 46403–51611, 5208 bytes, sha256:f463007ff52b38452c37509866c22bed710bd6d3eea7662bfeed330ab0f71756
- Whole source interval: DEV_NOTES.md bytes 193–1426, 1233 bytes, sha256:3ac2bce2bc8bdb71ab24ffb9263d08b7a1b26013d4aa4b28c65a0d2ff1141928
- Whole source interval: CHANGELOG.md bytes 207–846, 639 bytes, sha256:d5391a6b4c9caad4a6a8b01a1335beacb51b3a560cf08e14fd9406642df2423b
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94929, 4353 bytes, sha256:0a6afd4e45c710d993a609e5511a89b3dabdd6828258fbf95ec8600b57efae0f
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.cleanup-20261030`
  Status: `done`
  Goal:daily safe cleanup after arc review;protected originals/stores intact;child receipts below.
- ID: `G1-SLICE.6.5.3.2.cleanup-20261030.proof-protection`
  Status: `done`
  Goal:D342 P1 prevent retained proof loss before any daily cleanup apply;protect current and
  future archive/search workspace families without changing ordinary artifact eligibility.
  - [x] REPRODUCE / ISSUE:actual seam-20261030 frozen plan35 proposed9 .log files inside4
    retained0586 search workspaces. Independent preservation review rejected overlap before any
    deletion. Proof logs/zero-byte markers are part of original capture identity,not generic strays.
  - [x] ROOT CAUSE (WHY + WHERE): python3 -I -B docs/tasks/artifacts/artifact_cleanup/cleanup.py plan
    target/artifact_cleanup_audit/seam-20261030 →35 candidates/1692 skipped,rc=0;independent
    protected-overlap assertion fails on9 retained search logs. The generic target suffix walk
    protected2 numerical stores but lacked archive/search workspace lifecycle protection.
    Cleanup code last changed0315;later retained search evidence had no equivalent family guard.
  - [x] FIX:add a bounded first-target-child predicate for history-capture-,history-generations-,
    history-proof-,history-review- and book_search_. Direct inspect refuses retained roots and
    census prunes them before traversal,including future identities. Keep old constants/bodies
    exact through only the two explicit additive guards. Pin all five families in policy controls.
    Existing2 numerical protections remain. Record rejected plan;keep prior cleanup date unchanged.
  - [x] ADDRESSED (verified): python3 -I -B docs/tasks/artifacts/artifact_cleanup/certificate_store_controls.py
    →protected7/ordinary2/actual compiled omission reds4/restored1;all data/source bytes unchanged,
    rc=0. Fresh fixtures cover2 numerical stores/five future family identities/nested ordinary
    lookalike. Actual numeric-constant,predicate,inspect and census-prune omissions each fail its
    independent assertion. Revised real read-only plan70 has proof overlap0;old9 fingerprints
    remain exact and no apply/result exists,rc=0. Added44 candidates are exactly victim logs from
    two owned entry-contract fixture runs,not newly selected retained proofs. No project deletion.
  - [x] NO REGRESSION: python3 -I -B docs/tasks/artifacts/ci_environment/python_cleanup_entry_contract.py
    →default/attested/complete captures41/9/1,14 physical refusals,5 actual omission reds,legacy
    body/constant projection exact plus2 certificates/5 explicit families,link/blocked1/clear0
    controls pass,rc=0. Producer349/34304 calls/73 heredocs/2892 embedded,history787 four exact
    Git spans,prior IDs/order/uniqueness plus two owned cleanup children,map/diff pass,rc=0.
    Book99/374/1659/2937 and glossary310/9/158/0 pass,rc=0. No actual deletion or changed latest date.
  - [x] LOCKSTEP:canonical cleanup policy/book/D342 ledger/task/index/MEM/DEV/CHANGE/LIVE align;
    whole prior native arc/ledger scope retained787. D342 fixed before deletion;next .apply owns
    a fresh post-commit plan,all protected/current proof preservation and immediate residue.
    Native numerical/product status unchanged;G1 6/18/G0 unapproved. Full/hosted79bd6f6a SHA-scoped.
    promotion: declined (existing immutable-proof preservation already requires this protection;the fix mechanizes retained workspace naming families and adds independent guards without new deletion authority).
  Commit:STITCHCAD-G1-0628;Verification:protected7/ordinary2/actual reds4/entry contracts/real read-only plan.
- ID: `G1-SLICE.6.5.3.2.cleanup-20261030.apply`
  Status: `done`
  - [x] REPRODUCE / ISSUE:daily cleanup due after2026-10-29 14:10UTC;old rejected plans unusable.
  - [x] ROOT CAUSE (WHY + WHERE): cleanup.py plan target/artifact_cleanup_audit/seam-20261030-safe
    →70 candidates/1700 unsafe skipped,HEAD c485e067;4 owned generated trees/66 strays,rc=0.
  - [x] FIX:copy/verify66 strays/26 receipts;hash14 complete proof roots;OS-idle apply;no-launcher residue.
  - [x] ADDRESSED (verified): python3 -I -B docs/tasks/artifacts/artifact_cleanup/cleanup.py apply
    target/artifact_cleanup_audit/seam-20261030-safe --idle-cua →handoff OK;204 files/16693061B
    removed,residue0/tracked_change false,rc=0. Frozen plan/result/apply.out in that audit.
  - [x] NO REGRESSION: python3 -I -B target/cleanup_20261030_guard.py after →70 absent/tracked exact;
    6496 entries/13380343714B across12 archive/search+2 numerical roots unchanged;66 copies/26
    receipts exact,rc=0. Store overlap0,not wholly hashed;1700 unsafe kept;Rust release/deps bin/log0,
    incremental2163 bins kept. Public archive1241/85,book99/374/1659/2937,glossary0,IDs/map/diff pass,rc=0.
  - [x] LOCKSTEP:cleanup record/book/live docs aligned;LIVE unchanged:G1 6/18/G0 unapproved.
    Next forward.retention owns active64;0628 stays live. No full/hosted verdict after79bd6f6a.
    promotion: declined (existing preservation/idle/residue contracts;no new authority).
  Commit:STITCHCAD-G1-0629;Verification:real apply/14-root independent residue/receipts/book.
## _(2026-10-30 UTC)_ — apply safe daily cleanup after proof-family repair

0629 freezes the actual post0628 plan,checks4 generated trees/66 strays against14 retained proof roots and protected stores,then copies/byte-verifies all66 strays and26 current verification receipts. OS-visible canonical handoff OK;204 files/16693061B removed,1700 unsafe entries retained.
Immediate no-launcher census proves all70 paths absent and tracked hash exact. All14 archive/search/numerical roots retain6496 entries/13380343714 regular B exactly. All66 copies/26 receipts match. Package/dependency stores excluded,not wholly hashed;2163 incremental bins retained with linked caches.
Regeneration follows the immediate proof;book99/374/1659/2937/glossary0/archive64/IDs/map/diff pass. 0628 guard scope stays live;active archive64 needs retention before growth. Original D342 rejected plans remain unapplied;protected0586 logs survive. Latest cleanup record now reflects actual removal.
Promotion declined:existing preservation/idle/residue contracts applied. LIVE reviewed unchanged:G1 6/18/G0 human unapproved;full/hosted79bd6f6a remains exact-SHA scoped. Native roots/arc reviewed;next forward retention,then coupled review. Full PNT continues.
## STITCHCAD-G1-0629 — clean regenerable artifacts and preserve original proofs

- Apply fresh post0628 daily plan after OS-visible idle admission:4 trees/66 strays/204 files/16693061B removed. Immediate independent census proves70 absent/tracked exact;6496 retained entries/13380343714 regular B unchanged across14 complete archive/search/numerical roots.
- All66 stray copies/26 current verification receipts exact;1700 unsafe entries retained. Book99/374/1659/2937/glossary0/archive64/IDs/map/diff pass. Latest cleanup record aligned. LIVE G1 6/18/G0 unapproved unchanged;full/hosted79bd6f6a SHA-scoped. Next coupled forward review.

## D298/D299 — verified glossary/index repair0484

Done;owner G1-SLICE.6.vocabulary.acceptance. [Whole causes/reproductions/repairs](../history/window23.md#stitchcad-devnotes-part639md)
retain obsolete secondary pointer,definition-only use and token/chapter-pair assumptions;focused4/full41 pass0.

## D300 — corrected execution scope0485

Done;owner .6.5.3.2.1. [Whole reproduction/source proof](../history/window23.md#stitchcad-devnotes-part642md) retains the exact original ledger.

## D301 — parent-aware ease conservation before paired effects

P1 paired prerequisite;owner G1-SLICE.6.5.3.2,in_progress before effects. Partition/lineage complete0574:47 native/21 doc scoped checkpoint. Next .5e.2.5.derived.3.3.3.4.2 owned sources. Real providers/physical joins .2.4/trusted atomic effects .2.5 remain pending before whole acceptance. Current repair/authority/runtime proof stays owned;local/hosted79bd6f6a stays SHA-scoped.
Actual native structural_child_graph_acceptance_is_not_parent_ease_conservation passes:copied10+10 gives20 for parent10;independently authored5+5 also passes structural validation with DeferredToG2AndG3. Root:per-span fields/validation lack an original-parent relation. No split result is published;handler unavailable.
Implement coupled parent-aware sources/quantization/provenance for explicit/formula/profile and distributions;standalone structural acceptance is not conservation proof.
Independent Fraction±1 diagnostic:rounded-child orders[0,1,0] versus[1,0,0];root boundaries[0,1,0],signed symmetry. Native .2 root-query watch passes with original domains/direction;nonuniform/context/stops/lineage/effects and full paired acceptance remain pending.

## D302 — aggregate count mistaken for active archive pressure

Done0490;owner G1-SLICE.6.5.3.2.retention. [Whole cause/reproduction/verified fix](../history/window23.md#stitchcad-devnotes-part659md).

## D303 — repeated retained-source interval claims

Done0492;owner G1-SLICE.6.5.3.2.3.2.3.2. [Whole red/fix/verification](../history/window23.md#stitchcad-devnotes-part659md).

## D304 — equal source content retained as multiple actual bodies

Done0492;owner G1-SLICE.6.5.3.2.3.2.3.2. [Whole red/fix/verification](../history/window23.md#stitchcad-devnotes-part659md).

## D305 — premature bounded intermediates in reference recomputation

P1 fixed0503;owner G1-SLICE.6.5.3.2.3.2.4.3.numeric. Independent Fraction14/public point+range native watch first failed1/rc101:nine representable final results falsely repaired. Split1/3 held1537228672809129301/4611686018427387902 now correctly resolves1/9223372036854775804.
Root:bounded differences/products before complete ratios in split-second/offset/merge point+range maps. Reuse exact complete relative progress;share one complete arc-length merge kernel. Reduce before narrowing;no reference width increase or rounding.
Focused range20/notch4 and oracle14 pass0;five genuinely wide final fractions retain original reference/range and actual numeric cause. Topology unit/scoped lint results belong owning leaf;projection/context and deferred broader paired acceptance remain separate.

## D306 — independent seam oracles absent from automated gates

Done0528;owner G1-SLICE.6.5.3.2.oracle-watch. Baseline9-producer fix retained687. Current gate10 producers/12 fixtures/9 finite guards pass;29 suites discovered. New stop oracle registered0529;missing/unknown/status/mismatch/wiring controls retained. Hosted verdict follows push400. Stop partition .3.3 continues.

## D307 — retained test block precedes later production items

Done0536;owner G1-SLICE.6.5.3.2.prepush-400.layout. Full make check at51e421cf rc2 exposed0471 production append after0470 tests. Existing test block moved to EOF;production/test content exact. Actual library-test Clippy/native boundary1/fmt pass0. D308 fixture lint and parent prepush-400 full gates/push/hosted verdict remain pending.

## D308–D342 — prior closures and retained-proof cleanup risk

D308–D341:[whole prior ledger](../history/stitchcad-devnotes-part787.md). D334 P2 after G1/before G0;D223/PLANNING.5 open. D342 P1 fixed0628:plan35 proposed9 retained search logs;no deletion. Owner G1-SLICE.6.5.3.2.cleanup-20261030.proof-protection adds5 future-workspace families;protected7/ordinary2/actual reds4/entry contracts pass;fresh apply remains owned.
