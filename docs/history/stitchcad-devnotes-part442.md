# Sealed archive — whole0306 garment input composition

Immutable historical segment, sealed by leaf `G1-SLICE.4c.3d.3.2` on `2026-10-26` (UTC).

- **Sealed identity:** 18 lines, 1908 bytes, sha256:ef1580913a8bcd5693eaab00d5df1235cd03afd91fdff8a4a126872a7ed3df08
- **Coverage:** whole committed source interval from 4ae965017e4e6af306e0d2dc9d992b01f37e2e9a.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 27505–29413, 1908 bytes, sha256:ef1580913a8bcd5693eaab00d5df1235cd03afd91fdff8a4a126872a7ed3df08
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4c.3d.3.1`
  Status: `done`
  Goal:0306 garment SizeSet input view borrows actual membership/axes/chart/breaks and typed structural path requirements.
  Paths: crates/sc-measure/src/size_set.rs;crates/sc-measure/src/size_set/;crates/sc-measure/src/lib.rs;
  crates/sc-measure/src/size_break/context.rs;crates/sc-measure/src/size_chart.rs;crates/sc-measure/src/size_chart_collection.rs;
  crates/sc-measure/tests/garment_size_set_inputs_contract.rs;crates/sc-measure/tests/support/size_break_fixture.rs; book/decision/live docs.
  Require shared actual membership, measurement context and Design table snapshots across views; reject cross-inventory
  identity conflicts. Draft structure remains explicit; regeneration needs complete chart, grading also break coverage.
  No value/state copies, registry/approval/geometry or numeric-execution claim. MTM integration remains .3.2.
  Own440 whole0305 scope,441 oldest0281 lesson,278 ledger from79e0f1f, exact whole Git/hash/public before retirement.
  Focused new garment-input/prior break collection targets, selected strict lint/fmt/book; broad checks deferred.
  Acceptance0306:
  - [x] ROOT CAUSE0306: rg composite/path/snapshot -> garment.rs:14/122 and size_set.rs:9/18, rc=0; actual shared context records, no metadata reconstruction.
  - [x] ADDRESSED0306: cargo test -p sc-measure --test garment_size_set_inputs_contract --test size_breaks_contract ->5+5 passed/0 failed, rc=0; borrows/paths/gaps/snapshots/aliases/Unknown preserved.
  - [x] NO REGRESSION0306: selected lib/two-test strict Clippy rc=0; fmt rc=0; book rc=0. Shared fixture has a scoped unused-helper allowance for coupled contexts.
  - [x] LOCKSTEP0306:440/441/278 whole79e0f1f Git/hash/public exact rc=0; book/decision/live aligned; MTM and actual execution remain owned.
  Promotion: existing SizeSet decision extended. Commit: STITCHCAD-G1-0306. Next .3.2 MTM inputs.

