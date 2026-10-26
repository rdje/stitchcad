# Sealed archive — whole0304 required break coverage

Immutable historical segment, sealed by leaf `G1-SLICE.4c.3d.3.h15` on `2026-10-26` (UTC).

- **Sealed identity:** 16 lines, 1830 bytes, sha256:cee8b2c7e2ac169b8e45f2609fa1d33b65a141f2b91c7f9d604de8f5400d90df
- **Coverage:** whole committed source interval from ba41e9a7a4f9592c732c7c5a0ed23e7c51d80c0a.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 26908–28738, 1830 bytes, sha256:cee8b2c7e2ac169b8e45f2609fa1d33b65a141f2b91c7f9d604de8f5400d90df
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4c.3d.2`
  Status: `done`
  Goal: explicit required axis/dimension inventory and per-adjacent-pair break coverage; partial drafts remain explicit;
  missing dimension/pair diagnostic, singleton MTM path-2 refusal, exact current break/source pins.
  Unit0304 paths: crates/sc-measure/src/size_breaks.rs;crates/sc-measure/src/size_breaks/;
  crates/sc-measure/src/size_break/context.rs;crates/sc-measure/src/lib.rs;crates/sc-measure/tests/size_breaks_contract.rs;
  crates/sc-measure/tests/support/size_break_fixture.rs;crates/sc-measure/tests/size_break_contract.rs; book/decision/live docs.
  Partial drafts allowed; coverage never inferred from available records or chart arithmetic. Preserve authored vectors,
  all states and actual declarations. Own435 whole0303 task,436 oldest0279 lesson,276 ledger from28caa22; whole Git/hash/public proof.
  Focused new/affected native tests, selected strict lint/fmt/book; full CI deferred to integration/pre-push.
  Acceptance0304:
  - [x] ROOT CAUSE0304: rg required/available declarations -> size_breaks.rs:10/40/51 and coverage83/110, rc=0; independent requirements, no inferred dimensions.
  - [x] ADDRESSED0304: cargo test -p sc-measure --test size_breaks_contract --test size_break_contract ->5+5 passed/0 failed, rc=0; exact gaps/duplicates/singletons/current sources.
  - [x] NO REGRESSION0304: selected lib/two-test strict Clippy rc=0 after unused fixture imports removed; fmt rc=0; book rc=0. Shared validation preserves prior declaration tests.
  - [x] LOCKSTEP0304:435/436/276 whole28caa22 Git/hash/public exact rc=0; book/decision/live aligned. No numeric grading or composite readiness claim.
  Promotion: existing SizeSet decision extended. Commit: STITCHCAD-G1-0304. Next .3d.3 composition; archive working64 requires window15 before further retirements.
