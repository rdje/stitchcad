# Sealed archive — complete producer records before the D217 investigation

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.cleanup4.hunt` on `2026-10-25` (UTC).

- **Sealed identity:** 52 lines, 4144 bytes, sha256:e667f091717b2a24ea6535575c51e4f81d981341969aac6b4a1bf3026571a5d8
- **Coverage:** five whole completed producer records from cdc6a776ada1e9c79b70f289309f744e412f67b9; canonical IDs and source ownership remain live.
- Source interval: docs/tasks/G1-SLICE-producers.md bytes 53735–55044; sha256:6b7214551cf230bba7390c568deafee9815c8226d36be44467d85f132e79ca07
- Source interval: docs/tasks/G1-SLICE-producers.md bytes 6784–7478; sha256:f65d032acc971f00f55f0d6c737e6fc92df2c7b6ff60f28b99bcb8008a926341
- Source interval: docs/tasks/G1-SLICE-producers.md bytes 7479–8254; sha256:db9bcc719ab00f9c21c0d780d549da3db11b66926f55aaef48b201cb381eca4b
- Source interval: docs/tasks/G1-SLICE-producers.md bytes 8255–9008; sha256:b2caece03a51e487fd1d1028647c69195630e08f3082412f420e7f400be8e38f
- Source interval: docs/tasks/G1-SLICE-producers.md bytes 20188–20797; sha256:ca817c8789e20b6b103580d3fbe830b5a3da89ac54ee34bd24d5602cdaa564d4
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5b.4c.h2.b.p.o.f.cleanup4.fix`
  Status: `done`
  Goal: D217 diagnosis/actionable refusal, strict gate preserved; host recovery remains blocked.
  Paths: scripts/handoff_census.py;docs/tasks/artifacts/handoff/contract.py;live/book records.

- [x] REPRODUCE / ISSUE: bash scripts/check_handoff.sh --idle-cua refused2: malformed name.
- [x] ROOT CAUSE: same OS command now reports type unavailable, PID8048/FD10, rc=2.
  parse_handles confused optional metadata with framing; Apple source warns revoked is uncertain.
- [x] FIX: PID/FD diagnostic, no raw name except exact marker; no exemptions or success change.
- [x] ADDRESSED: bash docs/tasks/artifacts/handoff/run_handoff_probes.sh reports49 fixtures/
  17 actual assertion reds, rc=0; exact diagnostics cover PID/FD/privacy/near-marker.
- [x] NO REGRESSION: python3 -I -B docs/tasks/artifacts/ci_environment/python_cleanup_entry_contract.py
  passes physical14/omissions5/strict statuses, rc=0; book build/coverage9/glossary27 pass0.
  Real clear/held-file/restored proof blocked by D217; host restart then fresh cleanup required.
- [x] LOCKSTEP: live/book/task records updated; bash scripts/check_live_doc_size.sh OK, rc=0.
  promotion: declined (existing strict evidence policy; UTC date verified). Commit0232; no cleanup/cadence change.

- ID: `G1-SLICE.5b.4c.h2.b.p.o.a01.2`
  Status: `done`
  Paths: docs/tasks/artifacts/formula_structure/static_recipe_contract.py
  Goal: copied-book recipe fixtures and real reference consumer; static_recipe_contract.py ownership.
  Acceptance: complete public helper/source/book tree/borrowed-root review before edits; guarded creation/copy/deletion/delegation, original consumer and per-recipe controls, actual captures/body faults and focused real proof.

Whole0195 receipts and original bridge inspection retained in [part247](../history/window10.md#stitchcad-devnotes-part247md).

Whole original completed scopes retained in [part261](../history/window10.md#stitchcad-devnotes-part261md).

- ID: `G1-SLICE.5b.4c.h2.b.p.v.b1`
  Status: `done`
  Goal: STITCHCAD-G1-0199 owns blocking D200 bridge adapter drift before copied-book implementation, plus already-owned whole0198/wanted retention.
  Paths: docs/tasks/artifacts/ci_environment/python_bridge_contract.py
  Acceptance: actual bridge TypeError reproduction; complete coupled ordinary/per-build/current-library captures; original4 children/8 writes/nine faults and early refusals stay; current native bytes/artifact unchanged; full focused verification/commit before parent resumes.

Prior authored node .a01.2.b retained whole in [part255](../history/window10.md#stitchcad-devnotes-part255md).

Whole original completed control scope retained in [part257](../history/window10.md#stitchcad-devnotes-part257md).

- ID: `G1-SLICE.5b.4c.h2.b.p.o.f.i`
  Status: `done`
  Goal: STITCHCAD-G1-0153 owns P1 finite all-Python file-destination inspection/repair ownership, including native entries with verified source/directory guards, before .a01.2.
  Acceptance: current full source/hash/all-call census and native write/child call-site review; watched exact path/kind/owner coverage, native canonical group membership and bounded groups, existing effect-owner agreement; dynamic filename/public-call/borrowed-root resolution owned before each repair, runtime file signoff stays pending; no unsafe writes or native jobs.

Complete committed verification retained in [producer proof](G1-SLICE-runtime-proof.md#complete-python-file-plan); canonical ownership remains above.

- ID: `G1-SLICE.5b.4c.h2.b.p.o.a23.3.1`
  Status: `done`
  Goal: Publication fixture/mutation/pure authority verified; whole0227 scope/proof/lesson retained300.
  Paths: docs/tasks/artifacts/book_publication/book_publication.py;docs/tasks/artifacts/ci_environment/python_publication_fixture_contract.py
  Acceptance: original45/all40 branches/pure APIs/physical12/eleven omission reds and focused gates pass.
  Corrected ledger receipt: original consumed log850 verdicts, not historical852; cleanup3 owns the correction.

Whole0227 task/lesson retained in [part300](../history/stitchcad-devnotes-part300.md).
