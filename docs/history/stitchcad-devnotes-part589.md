# Sealed engineering continuity — part589

- **Identity:** 83 lines, 7538 bytes, sha256:8cb718440dcca7603564d6855379b932ed978810eb809dbcd53c196fed970858
- **Coverage:** whole0437 task/lesson/ledger and0421–0422 completed task proofs from ca2537c9fd517f431c58c818c463987542e68750.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 37060–40387, 3327 bytes, sha256:6d8d2745e2fef4588cdefa7c0d9c040ff6dfc8d850109d67419a830a28908a0a
- Whole source interval: docs/tasks/G1-SLICE.md bytes 45264–47908, 2644 bytes, sha256:ce83fb7375956a66bd0edf0e920585dd88844650dba83ec6eba94e926df3216b
- Whole source interval: DEV_NOTES.md bytes 193–1105, 912 bytes, sha256:383aa72965f19b89b4edf8386da014221f39f55decada34064dc9ed4912f2db4
- Whole source interval: CHANGELOG.md bytes 207–862, 655 bytes, sha256:da7fd7468e9cbcc47360b63ff3d0b18b6c312e6fb3d4330b0ca5f1ddf208c09e
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.2.b.sources`
  Status: `done`
  Goal:0421 fix D281/D282; preserve whole graph/evidence and retire the completed journal for map bounds.
  - [x] REPRODUCE / ISSUE: task_source_contract.py --baseline expected1 error/1 failure; wrapper
    confirms both rc=0. Old7b refuses fresh sibling and accepts old done owner copied into root.
  - [x] ROOT CAUSE (WHY + WHERE): task_source_contract.py11 tests OK rc=0; native root lacks .5e.1.1
    while execution marks it done. Root-only lookup misses both directions; raw archive attrs were absent.
  - [x] FIX: task_source_contract.py11/newline_contract.py5 OK rc=0; bounded family/freshness/frozen
    sources, raw archive storage and scoped blank EOF; ordinary source whitespace still refuses.
  - [x] ADDRESSED: task_source_contract.py:11 tests OK rc=0; native partition retains98 nodes/every old id,
    exact294L/11636B/sha256:6a753dd7… body; late sibling changes preserve authority/caller edits.
  - [x] NO REGRESSION: maintained_reference_contract.py23/authoring8/newline5/AST/bash/diff rc=0;
    tree coverage10 lanes/13 trees/17 siblings/0 unowned/orphan/dead rc=0. Full product proof remains.
  - [x] LOCKSTEP: transition_receipts.py records570/571/572:4+2+1 exact Git spans rc=0; complete0420/0390/
    journal retained, seven current nodes/21 anchors preserved; map8185B. Retention/book/size/AST/bash/diff0.
- ID: `G1-SLICE.6.5.2.b.2`
  Status: `done`
  Goal:0422 actual private AssignMaterial candidate/inspect preview; complete current registry/source
  checks and exact Material diff, no publication or new entity allocation. Fresh-ID staging follows .b.3.
  - [x] REPRODUCE / ISSUE: git show b8693af:crates/sc-core/src/command/bus.rs shows only GetPieceList
    dispatch/HandlerUnavailable, rc=0; a detached Piece replacement cannot validate a full candidate.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-measure --test design_inputs_contract command_bus:
    10 passed/0 failed rc=0; real candidate needs actual whole registry/source validators and fresh authority.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings: finished rc=0;
    private actual candidate plus inspect authorization/binding, original typed causes and exact report.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus:10 passed rc=0;
    inspect-only order/no-op/reasons/missing Piece/Material/envelope/source/revocation; original owner unchanged.
  - [x] NO REGRESSION: cargo test -p sc-core --doc command::bus:2 passed rc=0; selected fmt --check rc=0;
    prior actual read5 also pass in command_bus10. Full integration remains .6.9, no publication claim.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part573.md:1 exact Git
    span rc=0; verify-retention OK rc=0. Book/README/live pointer/decision/lesson aligned; gates below.
  Book build/publication:80 chapters/159 scoped API rows/1461 source links/2435 rendered links rc=0; maintained-reference
  candidate81 files/17359 lines/1212157 bytes rc=0. Tree census10 lanes/13 trees/17 siblings/0 gaps rc=0.
  Checks use scripts/local_environment.py for Cargo. Initial indexing lint refusals corrected without
  lint suppression; selected strict lint now passes. New test module path fixed from rustfmt diagnostic.
- ID: `G1-SLICE.6.7.1.3`
  Status: `done`
  Goal:0437 bounded actual whole-model undo/redo, separate maximum-issued revision (D285), current
  operator authority, exact operation replay/append-only action audit and explicit reversibility loss.
  - [x] REPRODUCE / ISSUE: python3 -I -B docs/tasks/artifacts/command_layer/revision_branch_probe.py:
    pinned native counter/undo/equality model reuses1 and accepts old precondition, rc=0.
  - [x] ROOT CAUSE (WHY + WHERE): python3 -I -B docs/tasks/artifacts/command_layer/revision_branch_probe.py
    rc=0; restoring counters plus current.successor reuses
    snapshot identity. Whole actual alternatives and separate issuance/order preserve complete semantics.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished rc=0;
    bounded actual alternatives, separate original/current actors, finite action audit/shared keys,
    non-rewinding maximum-issued revisions and exact chronological authoring/history event sequences.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus:37 passed rc=0;
    prior30/history7 prove actual pointer/canonical roundtrip, human operator/agent author, scope/bounds/
    retry/conflict, final authority rollback, old1 refusal after undo0/new2 and next unused entity id.
    Actual Undo/new-branch witnesses retain audit2/3 order despite restored counters/repeated timestamps.
  - [x] NO REGRESSION: prior30 pass within37 rc=0; selected strict lint/fmt rc=0; cargo test -p
    sc-core --doc command::bus:7 passed rc=0, including direct history/action-journal erasure refusal.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part588.md:4 exact
    Git spans rc=0; whole0436/D285 diagnosis retained. README/book/status/defect/pointer/decision aligned.
  Native publication80 chapters/159 scoped API rows/1466 source links/2450 rendered links rc=0;
  retention1042/global72 fits rc=0; final core check after export/doc alignment rc=0.
  Cargo commands use scripts/local_environment.py. Promotion: indexed command-layer decision. D285
  runtime portion verified; storage/reopen .7 remains open.
  D286 P2 owned here: native ce5fce9 command module/book execution intros still label implemented
  metadata publication/replay/history pending. Scoped prior/current census3/0 rc=0; corrected module/
  book scope, actual native bus37 and rendered publication verify the fix. D286 closed; product code unchanged by wording. Payload/work/cancellation .6.8, deeper
  current/alternative source/allocation acceptance .6.5.5 and full .6.9 remain pending; no broader proof.
## _(2026-10-27 UTC)_ — bounded actual snapshots and non-rewinding issuance

0437 .6.7.1.3 moves whole actual models through bounded undo/redo alternatives, preserving original
hosts/semantics/IDs/counters. Current operator authority is separate from the accepted original author.
Finite action audit/exact retry share authoring key scope; drops/branches report lost reversibility
without erasing records. Maximum-issued revisions and global exact audit order never rewind.
Native bus37 (prior30/history7), privacy7/strict lint/fmt pass: D285 branch2 refuses old1 after undo0;
actual pointer/canonical roundtrip, scope/revocation/bounds/replay also pass. Promotion: command-layer
record. D285 storage/reopen remains .7; resource/full/source-allocation acceptance remains separately owned.

[Whole0436 and D285 diagnosis ledger](docs/history/stitchcad-devnotes-part588.md) retain exact native source evidence.

## STITCHCAD-G1-0437 — restore actual bounded snapshots without reissuing revisions

- Whole-model undo/redo retains original semantics/hosts/IDs/counters under fresh current-operator authority.
- Explicit finite depth/action audit; exact operation retry and shared actor-key conflicts preserve original records.
- Report undo eviction/redo-branch loss; separate maximum-issued revisions and chronological audit order never rewind.
- Native bus37/history7/privacy7/strict lint/fmt pass; D285 in-memory branch fixed, storage/reopen remains .7.
- Whole0436 and D285 diagnosis ledger retained588; resource/full/source-allocation acceptance remains owned.

