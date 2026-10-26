# Sealed archive — whole0292 identity-exhaustion repair scope

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.a` on `2026-10-26` (UTC).

- **Sealed identity:** 18 lines, 1828 bytes, sha256:b68170f78d50409e2c187f93ae000d7e5c47584562397e1bc82a1af369d6d671
- **Coverage:** whole committed source interval from 6af49fa988b1408fd0b16cd17a886e4161283d65.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 38706–40534, 1828 bytes, sha256:b68170f78d50409e2c187f93ae000d7e5c47584562397e1bc82a1af369d6d671
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.2.1`
  Status: `done`
  Goal:0292 own D225 generator exhaustion before command identities; reproduce MAX→0 wrap then typed refusal.
  Paths: crates/sc-core/src/ontology/id.rs;crates/sc-core/src/ontology/topology.rs;crates/sc-core/src/ontology/mod.rs
  Own affected crates/sc-core tests/callers and crates/sc-core/tests/id_exhaustion_contract.rs.
  Additional source caller: crates/sc-core/src/ontology/reference.rs.
  Audit actual call sites; make allocation fallible and preserve ledger state on exhaustion.
  Own405 whole0291 catalog task,406 oldest0267 lesson,264 whole0291 ledger before live growth.
  Also own407 whole sourced-call summaries/retentions and408 whole archive13 summary/retention.
  Focused allocator/ledger boundary checks and affected compile/lint; book/live/identity decision sync.
  Acceptance0292:
  - [x] REPRODUCE: native id_exhaustion_contract fails101 with MAX→0, before the fix.
  - [x] ROOT CAUSE0292: cargo test -p sc-core --test id_exhaustion_contract red101; wrapping_add and infallible IdGenerator could not represent exhaustion.
  - [x] FIX: fallible allocation, terminal optional counter and original error propagation before six ledger edits; actual callers migrated.
  - [x] ADDRESSED0292: same focused target3/0, rc=0; MAX/MAX-1 terminal cases, twelve allocator refusals and six precondition-priority cases preserve complete state.
  - [x] NO REGRESSION0292: ontology::id unit10/0; sc-core --all-targets --no-run and selected lib/changed-test strict Clippy/fmt pass, rc=0. No full runtime suite; integration/pre-push owns broad checks.
  - [x] LOCKSTEP:405/406/407/408/264 whole Git/hash/public exact rc=0; book build rc=0. D225 closed, G1still5/18; no persisted identity change.
  Promotion declined: repair to existing nonreuse contract. Commit: STITCHCAD-G1-0292.
