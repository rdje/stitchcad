# Sealed archive — whole0228 cleanup task receipt

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.cleanup4.r` on `2026-10-24` (UTC).

- **Sealed identity:** 53 lines, 5084 bytes, sha256:0701c679d38278e1dd2d0ea45e7c09fb53834856db30e484f1c3926c3e73c693
- **Coverage:** whole completed cleanup3 task and commit receipts from 14fa2823600df9949375fb39f54ae1e0aeea6f8f.
- Source interval: docs/tasks/G1-SLICE-producers.md bytes 55737–60821; exact complete record, excluding its separator blank line.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

- ID: `G1-SLICE.5b.4c.h2.b.p.o.f.cleanup3`
  Status: `done`
  Goal: required overdue24h cleanup on recovery2026-10-08, after0227 committed and before .a23.3.2.
  Own: docs/ARTIFACT_CLEANUP.md; target/artifact_cleanup_audit/recovery-20261008/ and ignored candidates.
  Acceptance: preserve0227 logs in protected audit storage before deletion; inspect frozen cleanup plan,
  apply with OS-visible idle proof; immediately verify selected absence/tracked bytes/HEAD/producer exact
  without activation, then regenerate/check native/WASM/book and focused controls. Record latest cleanup,
  commit clean state, no background work at final handoff. Preserve stores/deps/uncertain fixtures.
  Own whole0227 scope/lesson/ledger retention before replacing live records. Its completed receipt typed
  ledger852 while the consumed original log says850; preserve the historical receipt, correct the live
  projection from that output and record the correction here. No producer behavior changed.
  Apply terminal rc=0: four ignored trees/1378 logs/1504 files/11240863B removed;1382 selections absent,
  tracked bytes/HEAD/producer exact before activation. OS-visible embedded handoff: OK, rc=0.
  Audit target/artifact_cleanup_audit/recovery-20261008/; fourteen copied0227 logs remain exact.
  Preserve933 unsafe/uncertain entries and protected stores/deps; native/WASM/book regeneration passes.

- [x] REPRODUCE / ISSUE: prior cleanup2026-10-05 22:28UTC is overdue at recovery2026-10-08.
  cleanup.py plan target/artifact_cleanup_audit/recovery-20261008 derives1382 candidates/1504files/
  11240863B/933skips, rc=0, plan-output.txt and inspected plan.json in that protected audit.
- [x] ROOT CAUSE: cadence/policy selects four ignored trees and1378 logs; physical aliases/Git boundaries
  are refused. Independent full plan review verifies permitted roots/protected exclusions, rc=0.
  cleanup.py plan and preserved skip reasons pinpoint both hard-linked incremental caches; no deletion there.
- [x] FIX: fourteen consumed0227 logs copied byte-exactly into protected audit/evidence before cleanup.
  Whole0227 scope/lesson/ledger retained300/207 from55613e7 exact intervals/digests/public reads; prior
  history unchanged, rc=0, continuity-final.txt. Historical852 retained; live850 independently matches
  protected publication-fixture-ledger.txt. No code or producer behavior changes in this leaf.
- [x] ADDRESSED: cleanup.py apply same frozen audit terminal rc=0; four trees/1378logs/1504files removed,
  1382 selections absent/zero residue/tracked_changefalse. Independent read-only census before activation
  verifies tracked bytes/HEAD/producer exact and fourteen retained logs, rc=0, result.json/residue-proof.txt.
  OS-visible embedded check_handoff.sh prints handoff: OK, rc=0; all application results consumed.
- [x] NO REGRESSION: scripts/run_make.sh check/wasm/book terminal rc=0: fmt, strict Clippy, all native tests/
  doctests, actual three-crate WASM and mdBook regeneration. Publication45/fixture controls/cleanup15+11reds/
  coverage9/census258 paths and archive644 records pass, rc=0, native-check.txt/wasm-check.txt/book-build.txt/
  publication-final.txt/fixture-final.txt/cleanup-controls.txt/coverage-final.txt/census-final.txt/archive-final.txt.
- [x] LOCKSTEP: latest cleanup record/CH/DEV/MEMORY/index/G1 frontier route .a23.3.2; whole prior records,
  fixed caps and400 cadence preserved. Cleanup changes no LIVE_STATUS categories; G1 remains In Progress,
  G0 Mostly Done and later lanes Not Started. promotion: declined (existing cleanup/continuity rules).
  Ledger/final staged doctrine/OS-idle results consumed before commit0228 and requested final stop.
  Final ledger853 real verdicts/11 arms/13 pointers and all13 staged doctrines green, rc=0,
  protected ledger-final.txt/staged-gate.txt. Size checker OK under unchanged bounds, rc=0, size-final.txt.
  Final pre-commit OS-visible check_handoff.sh --idle-cua: handoff: OK, rc=0. No pending jobs or CUA work.

0227 commit:55613e7; subject STITCHCAD-G1-0227 (leaf G1-SLICE.5b.4c.h2.b.p.o.a23.3.1): guard copied publication fixtures.
0227 files: CHANGELOG.md, DEV_NOTES.md, LIVE_STATUS.md, MEMORY.md, docs/TASK_TREE.md,
docs/book/src/build-and-checks.md, docs/tasks/G1-SLICE-producers.md, docs/tasks/G1-SLICE.md;
docs/history/stitchcad-changelog-part206.md, stitchcad-devnotes-part298.md, stitchcad-devnotes-part299.md;
docs/tasks/artifacts/book_publication/book_publication.py; docs/tasks/artifacts/ci_environment/
producer_entries.tsv, python_effect_entries.tsv, python_file_entries.tsv, python_publication_fixture_contract.py,
run_ci_environment_probes.sh. The last filenames share their immediately preceding directory.
0228 subject: STITCHCAD-G1-0228 (leaf G1-SLICE.5b.4c.h2.b.p.o.f.cleanup3): complete crash recovery cleanup.
0228 files: CHANGELOG.md, DEV_NOTES.md, MEMORY.md, docs/ARTIFACT_CLEANUP.md, docs/TASK_TREE.md,
docs/tasks/G1-SLICE-producers.md, docs/tasks/G1-SLICE.md, docs/history/stitchcad-changelog-part207.md,
docs/history/stitchcad-devnotes-part300.md. Final commit ID: git log --oneline -1.
