# Sealed archive — whole0300 pinned query scope

Immutable historical segment, sealed by leaf `G1-SLICE.4c.3d.0` on `2026-10-26` (UTC).

- **Sealed identity:** 14 lines, 1409 bytes, sha256:ffce3b9236c6fdaa88c9e7be039cfc9442e3dcee34a50a003faa7d80c94bf157
- **Coverage:** whole committed source interval from e77742e88a485ab04137d7b8234482bb30fca29d.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 41423–42832, 1409 bytes, sha256:ffce3b9236c6fdaa88c9e7be039cfc9442e3dcee34a50a003faa7d80c94bf157
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.2.2.c.1`
  Status: `done`
  Goal:0300 immutable query design/actor/exact-revision context and GetPieceList/WalkSeamSpan payloads.
  Paths: crates/sc-core/src/command/query.rs;crates/sc-core/src/command.rs;crates/sc-core/tests/command_query_contract.rs;
  book/command decision/live docs. Query identity derives from payload; no mutation retry/group or execution grant.
  Preserve canonical span identity without copied sewing state or assumed geometry; actual snapshot/auth/handlers later.
  Own426 whole0299 task,427 oldest0275 static-node lesson and272 ledger from198bbb1; whole Git/hash/public checks.
  Acceptance0300:
  - [x] ROOT CAUSE0300: rg query declarations -> query.rs:8/51/74, rc=0; exact Design/actor/revision context and payload-derived canonical queries.
  - [x] ADDRESSED0300: cargo test -p sc-core --test command_query_contract --test command_catalog_contract ->2+2 passed/0 failed, rc=0; large exact revisions, actor/principal and identity metadata.
  - [x] NO REGRESSION0300: selected strict lib/two-test Clippy rc=0; fmt rc=0; census17/5/5/0 and book rc=0. No handlers/query results claimed.
  - [x] LOCKSTEP0300:426/427/272 whole198bbb1 Git bytes/hash/public reader exact rc=0; book/decision/live aligned without raising caps.
  Promotion: existing command decision extended; no new policy. Broad verification remains integration/pre-push.
  Commit: STITCHCAD-G1-0300.
