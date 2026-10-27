# Sealed engineering continuity — part587

- **Identity:** 36 lines, 2907 bytes, sha256:6efd580df225f3a2930ba9a2e105cda12da5ad3937e14766b2a276ae0d31ae2f
- **Coverage:** whole0435 D285 diagnosis task/lesson/ledger from 3b4f464060eed8ac95b8efd2c7dc1de3d425dcf9.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 44970–46579, 1609 bytes, sha256:ee93277fe3bda50aa9bf781a0e50a6c849bd6cbc8ed2d967aa174c34b5d199cb
- Whole source interval: DEV_NOTES.md bytes 193–1020, 827 bytes, sha256:8c7ba5301219daf91ddef9c0ca20b436c87e4780c5b6279bd6017e7b58bda2a0
- Whole source interval: CHANGELOG.md bytes 207–678, 471 bytes, sha256:f3d8a93b350095e0df433e3bfe15bdb30b166948d216612467de6ec2b001268b
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.7.1.1`
  Status: `done`
  Goal:0435 reproduce/own D285 revision-reuse risk before undo implementation; specify separate
  maximum-issued authoring counter, with runtime fix/native branch/reopen acceptance explicitly pending.
  - [x] REPRODUCE / ISSUE: python3 -I -B docs/tasks/artifacts/command_layer/revision_branch_probe.py:
    pinned native contract model reuses1 for distinct content and stale precondition matches, rc=0.
  - [x] ROOT CAUSE (WHY + WHERE): revision_branch_probe.py rc=0 pins native57767b0 counter successor,
    current-revision commit allocation, equality precondition and required undo counter restoration.
  - [x] FIX: diagnosis/specification scope only; revision_branch_probe.py shows separate maximum-issued
    counter gives fresh2 and refuses old1, rc=0. D285 implementation remains priority .6.7.1.3, not fixed.
  - [x] ADDRESSED: tracked producer rc=0; durable P1 owner, new branch/storage obligations and task
    decomposition recorded. No current native undo execution or completed fix follows from this model.
  - [x] NO REGRESSION: producer AST parsed rc=0; git diff --quiet HEAD -- crates rc=0; product unchanged.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part586.md:4 exact
    Git spans rc=0; whole0434/D283/D284 continuity retained. Book/decision/defect/pointer/frontier aligned.
  Native publication80 chapters/159 scoped API rows/1466 source links/2448 rendered links rc=0;
  retention1040/global70 fits rc=0.
  Promotion: indexed command-layer decision; complete runtime undo feature and verification follow.
## _(2026-10-27 UTC)_ — D285 revision restoration and branch identity

0435 .6.7.1.1 pins the actual counter/equality/commit allocation/undo clauses and models two distinct
snapshots both named1 after restore-and-successor. Old optimistic preconditions would falsely match.
Separate maximum-issued authoring counter yields2 while undo/redo preserve original snapshot counters.
Tracked read-only probe/AST pass; product unchanged. This is a contract counterexample, not runtime
undo proof. P1 D285 remains owned .6.7.1.3, with native branch/refusal and reopen .7 acceptance pending.
Promotion: command-layer decision. Typed current-operator/original-group history inputs .1.2 next.

[Whole0434 scope/lesson/ledger and retained D283/D284 continuity](docs/history/stitchcad-devnotes-part586.md) preserve exact source evidence.

## STITCHCAD-G1-0435 — own revision reuse before undo branching

- D285 P1: pinned native contract model proves restored-counter successor can name different content.
- Specify separate maximum-issued authoring revision while preserving undo/redo snapshot counters.
- Track immediate runtime fix/native branch proof and storage/reopen obligation; no runtime fix claimed.
- Read-only model probe/AST pass; product unchanged. Whole0434/D283/D284 continuity retained586.

