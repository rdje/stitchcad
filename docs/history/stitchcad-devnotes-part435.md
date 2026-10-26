# Sealed archive — whole0303 signed break declarations

Immutable historical segment, sealed by leaf `G1-SLICE.4c.3d.2` on `2026-10-26` (UTC).

- **Sealed identity:** 15 lines, 1671 bytes, sha256:39200ab148ee23a209ecbb1d3503f8729a265c54b616afd333c4426cfe69a89d
- **Coverage:** whole committed source interval from 28caa2235ac9ed9081d35be573ea69e31d2069ff.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 26706–28377, 1671 bytes, sha256:39200ab148ee23a209ecbb1d3503f8729a265c54b616afd333c4426cfe69a89d
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4c.3d.1`
  Status: `done`
  Goal:0303 immutable signed break declarations keyed by actual axis, named measurement binding and authored adjacent pair.
  Paths: crates/sc-measure/src/size_break.rs;crates/sc-measure/src/size_break/;crates/sc-measure/src/size_axis/source.rs;
  crates/sc-measure/src/lib.rs;crates/sc-measure/Cargo.toml;crates/sc-measure/tests/size_break_contract.rs; book/decision/live docs.
  Shared canonical inventory retains delta/source/state; actual membership/axes/table/measurement contexts and exact pins
  validate references, adjacency and saved dimension binding. Reject metadata/source conflicts, never subtract chart values.
  Focused native break/axis targets, selected lint/fmt/book. Own433 whole0302 task,434 oldest0278 lesson,275 ledger
  from7e0ca8d with whole Git/hash/public comparisons before retirement. Collection/composite readiness follows children.
  Acceptance0303:
  - [x] ROOT CAUSE0303: rg definitions -> size_break.rs:13/34, context.rs:15/117 and table.rs:9, rc=0; canonical binding and authored adjacency explicit.
  - [x] ADDRESSED0303: cargo test -p sc-measure --test size_break_contract --test size_axes_contract ->5+5 passed/0 failed, rc=0; signed values/all states/current sources and key refusals.
  - [x] NO REGRESSION0303: selected lib/two-test strict Clippy rc=0; fmt rc=0; book rc=0; original source/error borrows and cross-inventory conflicts checked.
  - [x] LOCKSTEP0303:433/434/275 whole7e0ca8d Git/hash/public exact rc=0; book/decision/live align, no coverage/path-readiness claim.
  Promotion: existing SizeSet decision extended. Commit: STITCHCAD-G1-0303. Next .3d.2 required break coverage.
