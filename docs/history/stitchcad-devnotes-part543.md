# Sealed engineering continuity — part543

- **Identity:** 49 lines, 4080 bytes, sha256:a54d712a00421dbd3a82ec3f7133d06bf7acaf7b5141d8d2d663a2e712ae353d
- **Coverage:** whole0393 authoring task, lesson, ledger and D268 report from 8a0fa96ca4af639c672356aa3360efb216b85910.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 49060–50458, 1398 bytes, sha256:6caa4feb36f2e79a24af6486dbcfc2c3e64c80d650aad403d87b75ebf8c0fac7
- Whole source interval: DEV_NOTES.md bytes 193–1380, 1187 bytes, sha256:7d1266b2c35af03f97403df52e407ef393fa33d6eb08709c646a4663a3bfc029
- Whole source interval: CHANGELOG.md bytes 207–716, 509 bytes, sha256:811f1c4d4e99068582886c98aec87954c8f5eb45746aaf9396d0c99546aa756a
- Whole source interval: docs/tasks/PLANNING.md bytes 89550–90536, 986 bytes, sha256:f2b61698ed7946f93fb57149779cbc609cf6230b847dbe85f5ff3ee8850d969b
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.4.5.capacity.apply.accept.authoring`
  Status: `done`
  Goal:0393 bounded complete-snapshot append, immutable seals/rollover and multi-window source proof.
  - [x] REPRODUCE / ISSUE: history_archive.sh verify-retention after b951604:refused1 invalid source path;
    append target/history-authoring-red-0393 refused1 invalid backing path; no workspace created.
  - [x] ROOT CAUSE (WHY + WHERE): Git/source diagnostic rc=0: committed-v3 generation paths absent
    from _git_blob/source bounds; pre-commit old=v2 followed the transition branch. D268 owned/corrected.
  - [x] FIX: v3_authoring_contract.py:10 physical source/candidate controls and4 early pure refusals0;
    actual HEAD full preparation/source proof, cross-group multi-window cohort and native tar bytes exact.
  - [x] ADDRESSED: history_archive.sh verify-retention:996 records/4013358 decodedB rc=0;
    existing seals/indexed bytes preserved; payload/decoded/line/window/control rollover, slot9 refusal.
  - [x] NO REGRESSION: v3_reader_contract.py:46 physical controls rc=0; generation_budget.py --self-test:
    25 budget+4 runtime0; AST comparison:6 old decoder/2 formatter bodies exact0. No Rust changes.
  - [x] LOCKSTEP: transition_receipts.py --record542:4 exact Git spans0; book/publication/size rc=0.
    Promotion: decision_history-generations.md. Current-client/permanent smoke/resource proof remains.
## _(2026-10-27 UTC)_ — bounded generation authoring and committed-v3 evidence

0393 captures all committed raw records, splitting payloads and filling bounded groups before ordered
rollover. Explicit seal opens an active group before retirement; unsealed drafts retain exact bytes.
Existing indexed files/sealed descriptors never move or change. Slot9 and empty seals refuse before
workspace activation. Complete same-revision source proof spans all windows/groups. One record works
in v3; original v2 formatter/two-record/report rules remain. Native10 source/candidate controls and4
pure refusals pass0, including actual GitHEAD full preparation and independent OS tar byte oracles.
D268 post-commit v3 retention failed closed: generation paths were missing from Git/source bounds;
old=v2 pre-commit proof exercised only transition. Exact generation1..8 rules now pass native committed
HEAD proof. Reader46, budget25/runtime4, old6 decoder/2 formatter AST and4-span542 pass0. No Root
archive install in this slice. Promotion: history-generations decision. Client/permanent transaction
smoke and full-budget resources remain immediate; full CI/platform integration remains deferred.

## STITCHCAD-G1-0393 — author bounded append and ordered generation seals

- Complete raw snapshots split across bounded payloads/groups; explicit seal carries unsealed drafts.
- Exact old indexed bytes/seals persist; slot9/empty seal refuse before effects. Cohort source proof spans groups.
- D268 committed-v3 Git/source path rules corrected; native10/pure4, reader46, budget25/runtime4 pass0.
- Whole0392 evidence542 has4 exact Git spans0; client/permanent transaction/resource acceptance remains open.

## D268 — committed-v3 Git control paths

Owner G1-SLICE.6.4.5.capacity.apply.accept.authoring; immediate. After b951604, native
history_archive.sh verify-retention refuses1: invalid source path. Pre-commit old=v2 did not exercise
committed-v3 generation controls. _git_blob allowlist and preparer source_bound omitted generation
paths; ordinary retrieval unaffected. Corrected exact generation1..8 rules and native HEAD fixture.

- [x] ROOT CAUSE: history_archive.sh verify-retention after b951604 refused1 invalid source path;
  Git/source diagnostic0 identifies both missing rules and old=v2 pre-commit coverage.
- [x] ADDRESSED: same command:996 records/4013358 decodedB rc=0; v3_authoring_contract.py:10 native
  controls/4 pure refusals0, actual committed-HEAD preparation and exact Git/native tar bytes.
- [x] NO REGRESSION: v3_reader_contract.py:46/0; generation_budget.py --self-test:25+4/0;
  legacy6 decoder/2 formatter AST exact0; book/publication/size and4-span542 rc=0.
