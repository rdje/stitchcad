# Sealed archive — whole D217 diagnostic records before the causal reproduction

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.cleanup4.hunt` on `2026-10-25` (UTC).

- **Sealed identity:** 100 lines, 8170 bytes, sha256:1295098af2910b379b2fc11ef2a361cb6cb6912980372bb3a3b545aa30dad074
- **Coverage:** whole0234/0233/0232 notes and complete0234 diagnostic task record from ccc2a66.
- Source interval: DEV_NOTES.md bytes 1596–8398; sha256:671d71a4daace7903b617b2110d5c9031f67b6247bb5ca51a9d709e989909aec
- Source interval: docs/tasks/G1-SLICE-producers.md bytes 52725–54092; sha256:a3f64b1363622f3ff63f18a1816a25f6ac7f76bca5be216cdafa5302685e2cfc
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

## _(2026-10-25 UTC)_ — D217 needs causal discrimination before a permanent-fix claim

0234 cleanup4.hunt instruments the unresolved production descriptor instead of inferring its
origin from lsof's marker. Snapshot of PID8048/65756/68313 shows only descriptor10's vnode metadata
unavailable, both native flavors returning zero/errno2, with unchanged enumerations and successful
neighboring metadata. Enumeration stability alone cannot exclude descriptor reuse. Installed
runtime0.164.0, Darwin27.0.0/build26A434, kernel13432.1.9; exact public kernel tag unavailable.
Primary source bounds: Apple's process_vnode translates ENOENT to the marker before setting type;
published pid_vnodeinfo/pid_vnodeinfopath use vnode acquisition and stat. No installed-instruction proof.

Owned terminal revocation and removal of an event-only directory independently reproduce the exact
native/lsof signature. Ordinary unlinked files/directories stay readable; a closed descriptor gives
errno9. Local fstat distinguishes the two reproduced mechanisms (errno9 versus2); remote libproc
cannot choose between them. The production descriptor's original identity and invalidation event
remain unknown. Neither lsof-only failure nor the historical creator has been established.

Isolated installed app-server initialization sends no thread/model request, uses empty project-owned
state, and deliberately preserves only a selected controlled descriptor. Clean and live-directory
arms initialize with readable vnodes; inheriting a removed event-only directory reproduces zero/
errno2/untyped revoked. Fresh initialization alone does not imply long-term prevention. Final runtime
children exit0 and their isolated process groups disappear. The first teardown checked immediately
after signalling and caught a transient group; a fresh process census found no surviving diagnostic
runtime. Bounded teardown now waits for group disappearance and refuses surviving descendants.

Calibration originally counted its own fault literal as a second anchor. Restricting replacement to
the parsed controls function fixes the calibration; two actual lifecycle omissions now fail their
independent assertions. Pure controls reject five corrupted observation sets. Native calibration
is watched by contract.py --real; its subsequent production baseline still refuses2, so the real
held-file/restored proof and cleanup remain blocked. The native calibrator itself passes10 cases/
two actual assertion reds; no production exemption or cleanup behavior change.

Receipts: target/artifact_cleanup_audit/startup-20261024/d217-hunt-snapshot.json,
d217-hunt-event-controls.json, d217-hunt-calibration-final.json, d217-hunt-runtime-none-final.json,
d217-hunt-runtime-live-final.json, d217-hunt-runtime-removed-complete.json and d217-hunt-handoff-real.log.
The final two runtime controls additionally record explicit process-group disappearance; no pending
diagnostic work may outlive the checkpoint. The full test plan and remaining discrimination are in
the cleanup annex. Five whole completed task records are retained303 from exact cdc6a77 intervals;
the tracked diagnostic rechecks complete source membership and bytes, without raising any cap.
That record crossed the fixed64-file resident history bound. Window11 retains all54 committed raw
files (169756B) in a deterministic47725B payload; full source proof passes with no missing/extra.
The previous ten manifests/catalogs/payloads stay exact; live public links move to the new catalog.
Final -verified runtime captures supersede the earlier iteration logs; all three exit0/group gone.
Post-commit fd7c41d: window_contract.py passes668 controls/647 logical reads, rc=0; a copied edit
of the newest committed catalog passes ordinary inventory and fails the actual retention guard.
Receipt: d217-hunt-window-committed.log in the protected startup audit. Earlier 667 lacked that
post-commit guard arm. All diagnostic jobs consumed; production handoff still needs complete evidence.
Next action must run from a standalone external terminal and interrupts sessions using this daemon.
For the observed installed release, launch the restart CLI with unrelated descriptors closed:

```bash
python3 -I -B -c 'from pathlib import Path; import subprocess; subprocess.run([str(Path.home()/".codex/packages/app-server-daemon/releases/0.164.0-aarch64-apple-darwin/bin/codex"),"app-server","daemon","restart"],close_fds=True,check=True)'
```

Then resume this investigation, capture replacement descriptors before cleanup, and rerun the real
handoff control. This tests recovery; it does not establish the original descriptor's historical owner.
Promotion declined: bounded investigation evidence, with the unresolved cause explicitly owned.

## _(2026-10-25 UTC)_ — cleanup recheck still needs external runtime recovery

0233 cleanup4: requested OS-visible bash scripts/check_handoff.sh --idle-cua still returns2 for
PID8048/FD10, type unavailable with the exact revoked marker. python3 -I -B
docs/tasks/artifacts/handoff/contract.py --real passes49 fixtures/17 actual assertion reds,
then exits1 at the same refused baseline before creating its holder. No new diagnosis or exemption;
no fresh plan/apply, deletion or cadence advance. Cleanup4 remains blocked; finish it and stop after
external runtime restart and complete clear/held-file/restored evidence. Promotion declined: same D217
blocker and existing recovery procedure. Receipts in target/artifact_cleanup_audit/startup-20261024/:
recheck-20261025-handoff.log and recheck-20261025-0953-real.log. Recheck complete; cleanup blocked.

## _(2026-10-25 UTC)_ — unavailable lsof metadata needs an actionable refusal

0232 cleanup4.fix: installed lsof's field-output manual permits unavailable fields to be omitted.
Apple process_vnode maps ENOENT to an uncertain revoked marker; treating that as inert would
discard evidence. Read-only libproc flavors1/2 also return zero with errno2 for the observed
descriptors; ps ancestry places code-mode-host outside the caller chain. Kernel cause unproved.
The parser now distinguishes unavailable type metadata from malformed process/file/name framing,
reporting PID/FD and only the exact revoked marker. No success, exemption or blocking rule changes.
Handoff controls pass49 cases/17 actual faults; three exact diagnostic checks reject wrong PID/FD,
path leakage and marker near-matches. The real OS census still refuses2 on PID8048/FD10.
D217 remains owned: host runtime restart, real clear/held-file/restored proof, fresh cleanup plan.
No cleanup or cadence update. Promotion declined: existing incomplete-evidence rule; recovery and
primary sources belong in the cleanup annex. Original0229 interpretation below is superseded only
as to its malformed-record diagnosis: the refusal remains correct.

- ID: `G1-SLICE.5b.4c.h2.b.p.o.f.cleanup4.hunt`
  Status: `in_progress`
  Goal: director2026-10-25 D217 cause/repair; recovery verified, historical origin open.
  Paths: docs/tasks/artifacts/handoff/d217_probe.py;docs/tasks/artifacts/handoff/contract.py;docs/tasks/artifacts/handoff/run_handoff_probes.sh
  Additional paths: docs/tasks/artifacts/ci_environment/producer_entries.tsv;docs/tasks/artifacts/ci_environment/python_file_entries.tsv
  Acceptance: historical lifecycle proof; strict checker; recovery/cleanup evidence.
  Plan/evidence: cleanup annex; DEV_NOTES0234/0235. Own retained303/window11, unchanged bounds.

- [x] REPRODUCE / ISSUE: native/lsof PID8048/FD10 ENOENT; d217-hunt-snapshot.json rc=0.
  Historical identity/trigger unresolved; exact installed kernel branch unproved.
- [x] ROOT CAUSE: d217_probe.py controls separates two mechanisms;10 observations rc=0.
- [x] FIX: watched diagnostic; checker unchanged; retained verifies4144B rc=0.
- [x] ADDRESSED: calibrate10/two omission reds rc=0; runtime arms exit0/group gone.
- [x] NO REGRESSION: handoff49/17, cleanup physical14/omissions5/statuses, publication45,
  producer259-path census/coverage rc=0 in0234. Real recovery clear0/held1/restored0 in0235.
- [x] LOCKSTEP: book/live/frontier/DEV/CH/MEMORY synced; size rc=0 in0234.
  promotion: declined (scoped diagnostic; original cause open).
