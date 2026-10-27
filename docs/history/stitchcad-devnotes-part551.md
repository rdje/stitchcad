# Sealed engineering continuity — part551

- **Identity:** 55 lines, 4393 bytes, sha256:d3afdf4f671b0d226a379502e7fa3a30c7eee160bd20412336b81feefddc783d
- **Coverage:** whole0401 current construction subtree, lesson, ledger and D273/retirement proof from bcb6b43184e97457d6216927d644aa722c340a4a.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 49033–50860, 1827 bytes, sha256:ce31440ac1c29b4fcd6134564e3fcc77b8aea3ea73094b8fd29fd897f4cd445c
- Whole source interval: DEV_NOTES.md bytes 193–1247, 1054 bytes, sha256:ff385b8c328b06c3d42d9890a34e6e03b3834ce280b5622d73113669387b0e7b
- Whole source interval: CHANGELOG.md bytes 207–716, 509 bytes, sha256:5cd1b70a7524cf897f447afeef97d68fa98de21ba37555098b0f64e4ec91381e
- Whole source interval: docs/tasks/PLANNING.md bytes 90009–91012, 1003 bytes, sha256:7ea824d7e4686fdc259e870a188afb9987f5f52708fb87240b2f02224948a822
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.4.5.4.d`
  Status: `done`
  Goal: six-family ownership/current scope; .1 immutable orders/ids; .2 graph/copy/Piece/sources.
- ID: `G1-SLICE.6.4.5.4.d.1`
  Status: `done`
  Goal:0400 exact construction ownership; [whole proof](../history/stitchcad-devnotes-part550.md).
- ID: `G1-SLICE.6.4.5.4.d.2`
  Status: `done`
  Goal:0401 complete current context/sources and original typed chains.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-core --test sewing_contract
    current_cut_plan_refusal_preserves_the_original_standard_error_cause:0/1 rc=101, cause None.
    D273 owned P2/fixed here; complete context required even empty.
  - [x] ROOT CAUSE (WHY + WHERE): source review and native red rc=101:sewing.rs empty Error impl
    hid InvalidCutPlan's original cause; complete graph preflight must precede actual-family selection.
  - [x] FIX: cargo test -p sc-core --test construction_inventory_contract --test sewing_contract:
    9+22 passed rc=0; graph-first current traversal and original contextual typed causes implemented.
  - [x] ADDRESSED: same31 native checks rc=0:empty context, unselected graph source, duplicate Pieces,
    stale quantities, missing owner, later index1 in all6 families, historical split/reversal and no cache;
    exact cause equality/downcasts include Dart AnchorError and Graph/Sewing/CutPlan.
  - [x] NO REGRESSION: old cases remain in31 native checks rc=0; affected strict Clippy/fmt and
    cargo test -p sc-core --doc ontology::construction_inventory:3 compile-fail contracts rc=0.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part550.md:4 spans rc=0;
    retention/book/publication/size rc=0; book facts retained, repetition tightened.
    Promotion: ontology structural decision. Declarations .e/global joins .5/component gate .f remain.
## _(2026-10-27 UTC)_ — whole current construction scope and original causes

0401 preflights the actual whole graph/copy/Piece/landmark/distribution context, even with empty
ownership, then checks every original Piece/source in fixed family/authored order. Missing owners
never retarget; contextual errors preserve actual id/index and original concrete raw evidence.
Later index1 damage in all6 families refuses without partial success or content changes. Historical
split/reversal remains held; subsequent damage is rechecked without caching. D273 red0/1 rc101
showed SewingError's empty standard cause chain hiding an existing exact CutPlanError; fixed and
verified through nested aggregate downcasts. Native9+22/compile-fail3/strict lint/fmt pass0.
Whole0400 scopes550 retain4 exact Git spans; retention/book/publication/size0. Repeated book wording
tightened with original semantics retained. Promotion: ontology structural decision. Declaration
roles .e, focused component acceptance .f and global Design/garment joins .5 remain; full CI later.

## STITCHCAD-G1-0401 — validate complete current construction context

- Preflight the whole graph/copy/Piece context before every family source, even with empty ownership.
- Preserve original id/family/index and concrete causes/raw repairs; no partial success, retarget or cached proof.
- Fix D273 standard SewingError/CutPlanError chain with native red/green and nested aggregate controls.
- Native9+22/compile-fail3/strict lint/fmt pass0; book current scope/example and whole0400 proof550 synchronized.

## D271/D272 — current causes and derived census
[Whole corrected reports0397](../history/stitchcad-devnotes-part547.md); [whole retirement proof0400](../history/stitchcad-devnotes-part550.md).
## D273 — sewing standard cause chain
Owned P2 .6.4.5.4.d.2, fixed0401. Current stale-quantity reproduction:0/1 rc=101, then1/0 rc=0.
Empty SewingError impl hid exact InvalidCutPlan cause. Native aggregate downcasts retain Graph →
SewingError → CutPlanError; concrete leaf errors remain terminal. Existing typed refusal unchanged.
- [x] ROOT CAUSE: transition_receipts.py --record docs/history/stitchcad-devnotes-part550.md:4 exact
  Git spans rc=0; whole0400 scopes retained before live retirement; D273 native red0/1 rc=101.
- [x] ADDRESSED: native sewing22/construction9 rc=0; D273 exact original cause preserved, no retarget.
- [x] NO REGRESSION: compile-fail3/strict affected Clippy/fmt/book/publication/retention/size rc=0;
  broader component verification remains .4.f, full integration later.
