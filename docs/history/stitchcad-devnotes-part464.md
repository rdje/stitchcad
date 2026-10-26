# Sealed archive — whole0315 cleanup attestation repair

Immutable historical segment, sealed by leaf `G1-SLICE.4c.4.cleanup5` on `2026-10-26` (UTC).

- **Sealed identity:** 17 lines, 1719 bytes, sha256:da71e57e64bad4d2bd3f493657ca6cf6892f9564b7e541b6ff8590a18e05c9ad
- **Coverage:** whole committed source interval from 35e2650bb0bb20f59f2cf3d461416027682fa46a.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 31846–33565, 1719 bytes, sha256:da71e57e64bad4d2bd3f493657ca6cf6892f9564b7e541b6ff8590a18e05c9ad
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4c.4.cleanup5.a`
  Status: `done`
  Goal:0315 fix D232: cleanup apply lacks canonical --idle-cua attestation passthrough; actual default blocks4
  metadata-only processes while attested handoff passes0. First unavailable-type refusal occurred before deletion;
  selected1047 paths remain, raw ps/lsof/retry receipts retained in feature-20261026 audit. SPINE.23r owns
  intermittent unavailable-type capture/root cause; do not classify its uncaptured original as a proven race.
  Own cleanup.py apply/main CLI, python_cleanup_entry_contract.py capture/projection controls and focused
  attestation tests there; docs/ARTIFACT_CLEANUP.md, governance/PLANNING/live/task docs. No checker policy changes.
  Attestation is explicit apply-only; no default/env inference, real handles/workers remain blocking.
  Retain461 whole0314 task,462 oldest0291 lesson,287 ledger and463 old governance cleanup-run receipt from
  aba572c, exact whole Git/hash/public before retirement. Fresh plan after committed tool repair owns actual deletion.
  - [x] ROOT CAUSE (WHY + WHERE): actual default blocks4 metadata-only PIDs; --idle-cua passes;1047 candidates intact rc=0.
  - [x] ADDRESSED: idle_attestation_controls default/explicit captures, old refusal,2 reds/5 invalid CLI refusals rc=0.
  - [x] NO REGRESSION: original_body_controls/certificate/blocked-clear controls rc=0; handoff49 fixtures/17 reds/book rc=0.
  Verification: direct focused functions in python_cleanup_entry_contract.py and handoff/contract.py rc=0;
  461/462/463/287 whole Git intervals/hash/public reader equality rc=0. LIVE_STATUS G1still5/18.
  Commit: STITCHCAD-G1-0315; cleanup5 remains in progress, fresh plan after commit; broad gates later.
