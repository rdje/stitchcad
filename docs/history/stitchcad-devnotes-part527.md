# Sealed engineering continuity — part527

- **Identity:** 63 lines, 4891 bytes, sha256:0978063c1a1e58afcac3b286f313deee16165073237cda3f1eb805b44f00b4e7
- **Coverage:** complete original anchor-cause task/lesson/ledger and D258/D259 reports from 46ce9ef4c3607f7128ddf9d92d1b440ae2679524.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 50809–52340, 1531 bytes, sha256:a732bd403b3e5ce4f5de2ea16f7f15b95f757f82f54b6c9c97de7b9a9c312311
- Whole source interval: DEV_NOTES.md bytes 193–1224, 1031 bytes, sha256:97d948edea16ebe3c32a6edf64be3a1d1cf73f4b154f664f3b673e4c537b721d
- Whole source interval: CHANGELOG.md bytes 207–672, 465 bytes, sha256:cc6d11ef88e4ca451b9120f134e221f3faf8061e3ce13c24ca298f7b83b78f0e
- Whole source interval: docs/tasks/PLANNING.md bytes 87488–89272, 1784 bytes, sha256:7174d1544d6c0ba0d044bdba74567869e1748d0d6d99581e9d9d93eacf33e280
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.5.1.a`
  Status: `done`
  Goal: D258/D259 original nested AnchorError sources for Pocket/NotionPlacement; focused red/green,
  preserve all validation behavior and data-only evidence; book/census/whole526 continuity.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-core --test pocket_contract
    pocket_position_error_keeps_original_anchor_cause -- --exact:0/1 rc=101; notion equivalent0/1 rc=101.
  - [x] ROOT CAUSE (WHY + WHERE): both exact native red controls fail at source() None;
    original PocketError/NotionPlacementError empty Error impls drop nested cause chains, rc=101 each.
  - [x] FIX: cargo clippy -p sc-core --test pocket_contract --test notion_contract -- -D warnings:rc=0;
    expose actual AnchorError by borrow/as_ref, preserving concrete type and original data.
  - [x] ADDRESSED: cargo test -p sc-core --test pocket_contract
    pocket_position_error_keeps_original_anchor_cause -- --exact:1/0 rc=0; notion equivalent1/0 rc=0.
  - [x] NO REGRESSION: python3 -I -B Git-source comparison: all non-Error-impl bytes unchanged
    in both production modules rc=0; fmt0, glossary132 failures rc=1, exact D250 rows compare rc=0.
  - [x] LOCKSTEP: bash scripts/run_make.sh book:rc=0; scoped publication79/137/1454/2401 rc=0;
    transition_receipts.py --record docs/history/stitchcad-devnotes-part526.md:5 exact spans rc=0.
    promotion: declined (local standard Error repairs; canonical D258/D259).
    Completed STITCHCAD-G1-0375; .1.b Pocket collection/current components next, broad CI later.
## _(2026-10-27 UTC)_ — original garment anchor error causes

0375 fixes D258/D259: PocketError::Position(Box<AnchorError>) and NotionPlacementError::Anchor
retained real nested failures, but empty standard Error impls returned no source. Actual deleted-anchor
controls each red0/1 rc101, then green1/0 rc0 after borrowing the exact original AnchorError; both
concrete downcast and pointer identity are preserved, rather than exposing Box as the cause type.
All production bytes outside the two Error impls are identical to4d5a4a4. Placement validation/data
and range evidence behavior stay unchanged; no invented cause wraps data-only RangeResolution.
Strict affected lint/fmt/book/publication79/137 pass0; D250132 exact prior rows.526 preserves whole
closed identity/sewing task scopes,0374 and historical recipe-phase proof in5 exact Git spans0.
Promotion declined: localized standard Error contract repairs; canonical reports close D258/D259.
Pocket collection/current components next; full integration/CI remains deferred.

## STITCHCAD-G1-0375 — preserve original garment anchor error causes

- Fix D258/D259 Pocket/NotionPlacement standard error chains to expose exact concrete AnchorError.
- Each actual deleted-anchor control red101/green0; all non-Error-impl production bytes unchanged.
- Affected lint/fmt/book/publication pass0; D250132 unchanged. Whole identity/sewing/0374/phase proof526.
- Pocket collection/current component scope next; full integration/CI remains separate.

## D258 — Pocket position error hid its original anchor cause (fixed0375)

Before repair, PocketError::Position retained Box<AnchorError> but the empty Error impl returned None.
Repro: cargo test -p sc-core --test pocket_contract pocket_position_error_keeps_original_anchor_cause
-- --exact: red0/1 rc=101, green1/0 rc=0 via local_environment.py; target/garment-anchor-pocket-*-0375.log.
Fix: source() exposes the original concrete AnchorError, same pointer/data, rather than Box type.
Impact was lost standard diagnostic chaining; enum data/validation remained intact. P2; owner
G1-SLICE.6.4.5.1.a. Closed with focused check; full CI remains integration/pre-push.

## D259 — Notion placement error hid its original anchor cause (fixed0375)

Before repair, NotionPlacementError::Anchor held AnchorError but the empty Error impl returned None.
Repro: cargo test -p sc-core --test notion_contract notion_anchor_error_keeps_original_anchor_cause
-- --exact: red0/1 rc=101, green1/0 rc=0; target/garment-anchor-notion-*-0375.log. source() now returns
that exact original concrete cause. Diagnostic chaining is repaired without rewriting placement data
or treating RangeResolution as an error. P2; owner G1-SLICE.6.4.5.1.a. Closed with focused check.

0375 shared verification (D258/D259):
- [x] ROOT CAUSE: both exact cargo test pocket/notion anchor-source controls returned0/1 rc=101;
  source() returned None despite the original nested AnchorError.
- [x] ADDRESSED: the same two exact cargo test controls returned1/0 rc=0 each after repair;
  original concrete type/pointer/data retained, Box cause refused.
- [x] NO REGRESSION: cargo clippy -p sc-core --test pocket_contract --test notion_contract
  -- -D warnings rc=0; production non-Error-impl Git-source bytes identical rc=0.
End of whole0375 original garment anchor causes and complete D258/D259 reports.
