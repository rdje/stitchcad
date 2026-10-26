# Sealed archive — whole0309 sizing integration repairs

Immutable historical segment, sealed by leaf `G1-SLICE.4c.3e.2` on `2026-10-26` (UTC).

- **Sealed identity:** 17 lines, 1785 bytes, sha256:35fdef7b733cdd5b8fd78409a4a47c2b917e3947ca46464c91a5bfaf45fc2e21
- **Coverage:** whole committed source interval from 19dbadc806fce8d7178b26ba712d0c5485be507c.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 28681–30466, 1785 bytes, sha256:35fdef7b733cdd5b8fd78409a4a47c2b917e3947ca46464c91a5bfaf45fc2e21
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4c.3e.1`
  Status: `done`
  Goal:0309 diagnose/repair MTM mutation-entry planning after its Rust fixture became a shared module.
  Paths: docs/tasks/artifacts/mtm_chart/mtm_chart_mutations.py;
  docs/tasks/artifacts/ci_environment/python_producer_contract.py; expert MTM annex/decision/live docs.
  Current entry plans implementation/test root only; the imported fixture is now required before any activation/effect.
  Use existing independent capture/physical-refusal controls with an explicit fixture oracle, not full native mutation replay.
  Own D230 companion observation/collection/membership/MTM book status sync during this integration repair.
  Own446 whole0308 task,447 oldest0285 lesson,281 ledger and138 whole closed D225-D227 from7618d20.
  Verify whole Git/hash/public bytes, focused pre-repair failure/post-repair controls and book build.
  Acceptance0309:
  - [x] ROOT CAUSE0309: existing measure_refusals with independent fixture oracle reproduces actual pre-repair missing-fixture activation/effect; controlled red, source hashes unchanged, rc=0.
  - [x] ADDRESSED0309: explicit read-only fixture plan and independent expected input; existing capture validates13 children/files,5 physical refusals pass and compiled omission red fails, rc=0.
  - [x] NO REGRESSION0309: actual implementation/entry/root-test/fixture before-after hashes equal; no real Rust mutation campaign. D230 source-status links align; book rc=0.
  - [x] LOCKSTEP0309:446/447/281/138 whole7618d20 Git/hash/public exact rc=0; PLANNING/book/live align. Existing full mutation replay remains integration/pre-push.
  Promotion declined: existing complete-input producer contract and current documentation boundaries; no new policy. Commit: STITCHCAD-G1-0309; .3e.2 scope review next.

