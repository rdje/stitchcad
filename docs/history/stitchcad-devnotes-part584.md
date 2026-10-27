# Sealed engineering continuity — part584

- **Identity:** 42 lines, 3641 bytes, sha256:fd216e2b89b6d22cc1038d44fa987dd0bedfb047adc1a5d31c53e48848855842
- **Coverage:** whole0432 sequential metadata-group task/lesson/ledger from ffda59c36923b8f70cd6a0ad3e4c1886ff19f698.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 43303–45412, 2109 bytes, sha256:2e7fd6c5ae71a86d3db1cb37baf25042ee461cb7e50c9c855bc934754127d8c5
- Whole source interval: DEV_NOTES.md bytes 193–1125, 932 bytes, sha256:1857144b4e86453e1f64e8cfe5ae9c2bc5d37398067a5f2bbff88de3b5e68f8c
- Whole source interval: CHANGELOG.md bytes 207–807, 600 bytes, sha256:edf4eb59e9eb416d9d4f08e00eae457eda361eb6ae5062f2d4d54d284d1d4ab4
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.4.2.2.3`
  Status: `done`
  Goal:0432 actual sequential metadata-group preview, final semantic delta and original failure context;
  exact within-group retry aliases, original model/revision/reserve unchanged. No persisted replay.
  - [x] REPRODUCE / ISSUE: git show HEAD:crates/sc-core/src/command/bus.rs | rg -n 'pub fn preview':
    only single preview printed rc=0; group schema/authority alone do not process actual successive models.
  - [x] ROOT CAUSE (WHY + WHERE): native HEAD source rc=0 and command_bus24 passed rc=0; each member
    must use preceding actual candidate/shared cursor, while a report list alone misstates restored deltas.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished rc=0;
    private sequential candidates, exact ephemeral aliases/conflicts and actual final-model semantic guard.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus:24 passed rc=0;
    original18/group6 cover mixed progression/net restoration/nonempty ordered delta/two IDs/retry/failure.
  - [x] NO REGRESSION: original18 pass within24 rc=0; selected strict Clippy and fmt --check rc=0.
    Revocation/auth-before-binding and later native anchor/exhaustion causes retain original owners/reserve.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part583.md:4 exact
    Git spans rc=0; whole0431 and0291–0430 decision continuity retained. Book/pointer/frontier/decision aligned.
  Native publication80 chapters/159 scoped API rows/1465 source links/2446 rendered links rc=0;
  maintained-reference81 files/17656 lines/1232356 bytes rc=0; retention1037/global67 fits rc=0.
  Cargo commands use scripts/local_environment.py. Initial17 missing error-field docs refused lint;
  every field now documents its actual semantics without suppression. Initial live-status row330B
  exceeded320B; tightened the summary, preserving feature/proof limits. Promotion: indexed command-layer
  decision. No publication, persisted replay/audit/undo, evaluator or full integration claim follows.
## _(2026-10-27 UTC)_ — sequential actual metadata-group preview

0432 .6.5.4.2.2.3 prepares each first request from the preceding validated actual model, sharing one
independent reserve cursor. One fresh snapshot/complete Inspect grant precedes actual Design/revision.
Exact group-local retry aliases return the first report; different same-key requests fail with context.
Net diff compares actual original/final models and preserves all other content; restoration omits net
changes. Later failure discards every candidate/report and leaves original identity allocation intact.
Native bus24 (original18/group6), strict selected Clippy/fmt pass; initial missing field docs corrected.
Promotion: existing command-layer decision records staging/retry/net-delta semantics and proof limits.

[Whole0431 base/cursor lesson and0291–0430 decision continuity](docs/history/stitchcad-devnotes-part583.md) retain exact original sources.

## STITCHCAD-G1-0432 — preview sequential actual metadata groups

- Fresh whole-group authority/binding; actual successive candidates share one independent identity cursor.
- Ordered step reports retain explicit exact-retry aliases; conflicting reuse and later failures refuse whole.
- Actual final semantic delta omits restorations and preserves inventory/owner context; no publication effects.
- Native bus24 (original18/group6), strict selected lint/fmt pass; full integration remains .6.9.
- Whole0431 and prior command decision continuity retained583 with four exact native source intervals.

