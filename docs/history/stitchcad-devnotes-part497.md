# Sealed archive — whole0345 Piece inventory and D248/D249

Immutable historical segment, sealed by leaf `G1-SLICE.6.4.2.a` on `2026-10-26` (UTC).

- **Sealed identity:** 63 lines, 5409 bytes, sha256:fab0f6ef882814c413eea0ef6ed7a8632ae44d24eed6a2a1b5334bd44db8902f
- **Coverage:** complete committed Piece inventory task/reports/lesson/ledger from cb561e3f63e46d7b9d716a622857aff277095d70.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 46136–48557, 2421 bytes, sha256:4293e58c4ab64e537d68c2e9381cc264330604998ca96377b33fcd0ff7ac82f5
- Whole source interval: docs/tasks/PLANNING.md bytes 84920–86118, 1198 bytes, sha256:6f755111b34494dff284e6d0dc451173007146e0a24528563d741d2ab029b076
- Whole source interval: DEV_NOTES.md bytes 193–1280, 1087 bytes, sha256:fe682cf4a27bd8673d563cfcc6cc92e71ddab7f597fdedd5b4297d54be95f39f
- Whole source interval: CHANGELOG.md bytes 207–865, 658 bytes, sha256:fd054359e1b82ea04c93ec29d22b8c6855bcb7ba3485bc7979cdb6aa18c4f981
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.1`
  Status: `done`
  Goal: immutable PieceInventory with authored order/identities, unique Piece ids and complete companion
  existence/paired-mode/reciprocity/opposite-hand/equal-quantity checks. Empty inventory stays legal;
  no IDs, labels, copies, geometry, materials or missing companions are inferred. No Design/release claim.
  Own source/public contract tests, selected lint/fmt and local vocabulary/book sync; D248/D249 docs
  correction in the same relevant records. Broader bus/geometry/platform verification remains later.
  D250 baseline glossary failures are owned by .6.vocabulary; prove exact before/after failure-row equality.
  Own complete prior subtree/report/lesson/ledger retirement496 needed for unchanged task caps.
  - [x] REPRODUCE / ISSUE: standalone reference-skirt member test passes1/0 while companion collection
    obligations remain explicit. New missing-target case constructs a valid Piece then refuses its inventory.
  - [x] ROOT CAUSE (WHY + WHERE): Piece::new takes only its definition/ledger and rejects self-companions;
    collection relations need the complete supplied Piece set. Baseline cargo test -p sc-core --test
    piece_contract the_reference_skirts_cut_once_pair_members_print_their_own_handedness passes1, rc=0.
  - [x] FIX: immutable authored PieceInventory and six typed relation/identity errors; canonical borrows,
    duplicate-before-relation/authored-order checks, explicit empty legality and no geometry/release claim.
  - [x] ADDRESSED: python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --test
    piece_inventory_contract gives8/0, rc=0, target/piece-inventory-native-0345.log; all error payloads exact.
  - [x] NO REGRESSION: Piece14/CutPlan9/privacy compile-fail1, selected strict Clippy and fmt pass0,
    target/piece-inventory-*-0345.log. Glossary132 current/baseline rows exactly equal; census rc=1,
    delta check rc=0. D250 owns baseline remediation after .6.9 before integration/pre-push, no full pass.
  - [x] LOCKSTEP: book/API row104/decisions/status/notes/index/pointer/D248-D249 aligned; complete
    prior subtree/reports/lesson/ledger496 matches4 Git source spans0. No ceilings raised; G1still6/18.
    Test syntax/indexing/helper-lint setup failures corrected/excluded. promotion: declined (existing pair decision).
    Commit0345; .6.4.2 next. Broader bus/geometry/platform/integration remains later.
## D248 — pair decision still says the settled D57 ruling awaits the director

The active decision_piece-pair-members-have-explicit-handedness.md ends with D57 remaining with the
director. The canonical physical-cut-copies decision records the director's ruling and implemented
CutPlan/SewingGraph APIs. Reproduce: rg D57 in both decisions and src/ontology/cut.rs/sewing.rs.
Impact: a current pairing reader is told a settled identity decision is pending. Owner G1-SLICE.6.4.1,
P2 fixed0345: active pair decision links the settled canonical ruling; pair inventory scope is explicit.

## D249 — executable copy guide still calls implemented sewing addressing the next slice

ontology-implementation.md's physical-copy example says the sewing-graph slice next integrates copy
addressing. SewingGraph::new, missing_copies and current copy-bound APIs already implement that scope;
G1-SLICE.3c.2b.2 is done. Reproduce: rg "slice next integrates" in that chapter and public methods in
ontology/sewing.rs. Impact: the book underreports current copy validation. Owner G1-SLICE.6.4.1,
P2 fixed0345: guide states supplied-copy validation at construction and missing targets; bus/revisions stay pending.

## _(2026-10-26 UTC)_ — complete Piece companion inventory

0345 closes the D56 collection obligation through an immutable authored PieceInventory. It checks unique
ids before companion existence/paired mode/reciprocity/opposite hands/equal quantities in authored order.
Labels imply no hand or partner; original Piece content/borrow identity survives, including explicit
unresolved materials. Empty inventories and equal odd member quantities are legal; no geometry or
Design/release readiness follows. Native8/privacy1/Piece14/CutPlan9/strict selected lint/fmt pass0.
D248/D249 stale decision/sewing prose corrected. The required glossary census exposes132 preexisting
C1 token/use failures: sourcec588e5e and current rows match exactly. D250 owns remediation after .6.9
before integration/pre-push; no full glossary/CI pass is claimed. New API names are locally declared.
Whole prior subtree/reports/lesson/ledger496 matches4 Git source spans; all10 prior nodes remain live.
Promotion: existing pair decision. G1 remains6/18; aggregate ownership/current references follow .6.4.2+.

## STITCHCAD-G1-0345 — validate complete Piece companion inventories

- Add immutable ordered PieceInventory: unique ids, existing reciprocal pair members, opposite hands/equal quantities.
- Preserve all authored fields/canonical borrows; empty inventories and odd per-member quantities remain legal.
- Native8/privacy1/Piece14/CutPlan9/selected strict lint/fmt and book/publication pass; D248/D249 corrected.
- D250 glossary census has the same132 baseline token/use failures; no new rows. Owned after .6.9 before integration/pre-push.
- Whole prior closure496 is Git-exact across4 source spans; caps unchanged. G1still6/18; aggregate state .6.4.2 next.

End of whole0345 Piece inventory continuity.
