# Sealed engineering continuity — part788

- **Identity:** 92 lines, 9801 bytes, sha256:f69729be288e937c54ed8ed7964a715c0ac86c6dce9f73effd043d33e4e2ab01
- **Coverage:** whole0628 task scope, lesson and changelog from c485e067ff972cef433ee2c989732f78bc1c20ae.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 46948–50462, 3514 bytes, sha256:94597ba042742a405ea18a140126eccdc3e195c30e1a6436f44803d8c6d21692
- Whole source interval: DEV_NOTES.md bytes 193–1499, 1306 bytes, sha256:f484d630ddec59559c115250a130a1573dba0cef5bdb7f3dbabcbbd8a87edc1c
- Whole source interval: CHANGELOG.md bytes 207–835, 628 bytes, sha256:b1bfd15207adcc64b27d8826050db143a0616c428bfb865fafd3ecd82c156647
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94929, 4353 bytes, sha256:0a6afd4e45c710d993a609e5511a89b3dabdd6828258fbf95ec8600b57efae0f
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

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
## _(2026-10-30 UTC)_ — prevent retained proof workspace deletion before daily cleanup

0628 fixes D342. The actual35-candidate plan proposed9 retained0586 search logs;preservation review stopped before deletion. Generic suffix handling lacked archive/search lifecycle guards. The new first-target-child predicate protects five workspace families in direct inspect and prunes the census before traversal.
Protected7/ordinary2/actual omission reds4/restored1 pass with bytes exact. Future family names and a nested ordinary lookalike are independent fixtures. Entry contracts retain41/9/1 captures,14 physical refusals,5 omission reds,link/process controls and old policy bodies/constants through exact additive adapters.
Revised read-only plan70 excludes proofs;old9 fingerprints remain exact,no apply. Extra44 are two owned contract runs' victim logs. Producer349/34304calls/history787/IDs/map/diff/book99/374/1659/2937/glossary0 pass. [Whole0627/ledger](docs/history/stitchcad-devnotes-part787.md) retained.
D342 fixed;actual daily cleanup waits for this commit and a fresh plan. Last completed cleanup date stays2026-10-29 14:10UTC. Product/native root/arc status,G1 6/18/G0 unapproved unchanged. Preserve all original proof workspaces/current verification before deletion;then resume forward review.
## STITCHCAD-G1-0628 — prevent retained proof workspace deletion before daily cleanup

- Fix D342 before removal:the original plan proposed9 retained search-proof logs. Protect/prune five archive/search workspace families,including future identities,while keeping numerical stores and old policies. Project cleanup has not run;retained proofs remain intact.
- Protected7/ordinary2/actual reds4/entry contracts/real revised plan pass;old9 fingerprints exact. Book99/374/1659/2937/glossary0/history787/IDs/map/diff and producer census pass. Latest cleanup date unchanged;next fresh plan/apply. G1 6/18/G0 unapproved unchanged.

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
