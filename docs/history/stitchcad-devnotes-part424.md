# Sealed archive — whole0298 complete axis points

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.b.2.b` on `2026-10-26` (UTC).

- **Sealed identity:** 17 lines, 1644 bytes, sha256:cda4df272d914d952f84a752df50b0ac37ce714501e23c213a8c57b43e8e8c1b
- **Coverage:** whole committed source interval from 5e15254f7cd2b898ec4eec6b1785ea6b36eb8e6a.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 25631–27275, 1644 bytes, sha256:cda4df272d914d952f84a752df50b0ac37ce714501e23c213a8c57b43e8e8c1b
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4c.2.2`
  Status: `done`
  Goal:0298 complete axes over actual pinned membership, >=1 authored axis, explicit complete member coordinates,
  current values, no grid interpolation, no implicit axis/value/base/order and full numeric source retention.
  Paths: crates/sc-measure/src/size_axes.rs;crates/sc-measure/src/size_axes/;crates/sc-measure/src/lib.rs;
  crates/sc-measure/src/size_axis/source.rs;crates/sc-measure/tests/size_axes_contract.rs; crate metadata/book/live docs.
  Validate exact pins, global local identities/names, complete member-axis coverage, coordinate ownership and
  distinct member points; retain authored vectors and actual member/value borrows. No Cartesian fill or numeric sorting.
  Own422 whole0297 task,423 oldest0273 lesson and270 ledger from35cf8ac; whole source/hash/public checks before removal.
  Acceptance0298:
  - [x] ROOT CAUSE0298: rg SizeAxes/SizeMemberPoint -> size_axes.rs:23/32/43/49 and validation.rs:7, rc=0; explicit coordinates over actual membership, not labels.
  - [x] ADDRESSED0298: cargo test -p sc-measure --test size_axes_contract --test size_axis_contract ->5+5 passed/0 failed, rc=0; sparse/singleton/current source and refusal cases.
  - [x] NO REGRESSION0298: selected lib/two-test strict Clippy rc=0, formatting rc=0 and book rc=0; nested error sources/original record borrows checked.
  - [x] LOCKSTEP0298:422/423/270 whole35cf8ac Git bytes/hash/public reader exact rc=0; no cap raised, book/live/decision aligned.
  Promotion: existing SizeSet decision extended; no new policy. Complete SizeSet chart/break/resolution remains .4c.3/.4.
  Commit: STITCHCAD-G1-0298.

