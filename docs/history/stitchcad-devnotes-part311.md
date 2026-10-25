# Sealed archive — completed host inheritance repair and activation proof

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a25.1` on `2026-10-25` (UTC).

- **Sealed identity:** 22 lines, 1932 bytes, sha256:d6e1aa2f98521d035befc28eb3bc4c6d14e2daa3b83c03a136c23d8763409eab
- **Coverage:** whole .f.cleanup4.hunt scope and verification from 4027b36317768f8b1ea4d097d17b73aec3ee1571; ownership remains live.
- Whole source interval: docs/tasks/G1-SLICE-producers.md bytes 47331–49263, 1932 bytes, sha256:d6e1aa2f98521d035befc28eb3bc4c6d14e2daa3b83c03a136c23d8763409eab
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5b.4c.h2.b.p.o.f.cleanup4.hunt`
  Status: `done`
  Goal: D217 repair active/verified; stop before publication as requested.
  Paths: docs/tasks/artifacts/handoff/d217_probe.py;docs/tasks/artifacts/handoff/contract.py;docs/tasks/artifacts/handoff/run_handoff_probes.sh
  Additional paths: docs/tasks/artifacts/ci_environment/producer_entries.tsv;docs/tasks/artifacts/ci_environment/python_file_entries.tsv
  Paths: docs/tasks/artifacts/handoff/d217_daemon.py;docs/tasks/artifacts/handoff/d217_upstream.py;docs/tasks/artifacts/handoff/d217_daemon_fix.patch
  Evidence: cleanup annex; DEV_NOTES0236/0237. Own retained303/304/window11; caps unchanged.
  Own docs/history/stitchcad-changelog-part208.md for the whole-ledger rollover.
  Own docs/history/stitchcad-changelog-part209.md for whole0235 retention.

  Original diagnostic acceptance retained in [part304](../history/stitchcad-devnotes-part304.md).
- [x] REPRODUCE / ISSUE: d217_daemon.py --binary ~/.codex/packages/app-server-daemon/releases/0.164.0-aarch64-apple-darwin/bin/codex;
  five cases rc=0; three propagate bad metadata.
- [x] ROOT CAUSE: d217_daemon.py identity/flags trace names pid_start's unfiltered spawn, rc=0.
- [x] FIX: d217_upstream.py verify reconstructs exact two-file patch, rc=0; selected hash verified0237.
- [x] ADDRESSED: d217_daemon.py --binary "$PWD/target/d217-codex-build/release/codex" --expect-contained;
  five cases rc=0; d217_upstream.py calibrate: compiled omission red/restored3 pass0.
  0237 live hash/41 files/51 vnodes verified; contract.py --real clear0/held1/restored0, rc=0.
- [x] NO REGRESSION: upstream cargo test -p codex-app-server-daemon --lib:88 pass0;
  d217_upstream.py rehearse: install/exact rollback rc=0; owned jobs stopped.
- [x] LOCKSTEP: book/live/task/archive/registries synced; retained4144/8170/3030B and size rc=0.
  promotion: declined (scoped host defect; original FD10 identity remains unknown).
