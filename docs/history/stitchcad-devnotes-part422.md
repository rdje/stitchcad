# Sealed archive — whole0297 authored axis scope

Immutable historical segment, sealed by leaf `G1-SLICE.4c.2.2` on `2026-10-26` (UTC).

- **Sealed identity:** 14 lines, 1407 bytes, sha256:54341d7e340187dd20929c347833f6fb7cf18671f27494d5678cf13fa09f0240
- **Coverage:** whole committed source interval from 35cf8ac82855aea2c2611638121f5e1f88204259.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 25367–26774, 1407 bytes, sha256:54341d7e340187dd20929c347833f6fb7cf18671f27494d5678cf13fa09f0240
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4c.2.1`
  Status: `done`
  Goal:0297 explicit named axis/ordered value definitions, typed categorical/scalar sources and actual canonical borrows.
  Paths: crates/sc-measure/src/size_axis.rs;crates/sc-measure/src/size_axis/source.rs;crates/sc-measure/src/lib.rs;crates/sc-measure/Cargo.toml
  Tests: crates/sc-measure/tests/size_axis_contract.rs. No labels/order inferred; numeric state/source stays singly owned.
  Local identity/name/value/kind/current-source checks; supplied pins are not registry certification.
  Own419 whole0296 receipt task,420 oldest0272 lesson,269 ledger and421 whole completed input-adapter summaries.
  Acceptance0297:
  - [x] ROOT CAUSE0297: rg canonical struct declarations -> value.rs:88/scalar.rs:227, rc=0; private records own source/state; metadata borrows actual records.
  - [x] ADDRESSED0297: cargo test -p sc-measure --test size_axis_contract:5 passed/0 failed, rc=0; exact order/borrows/all kinds/states and current-source refusals.
  - [x] NO REGRESSION0297: selected lib/test strict Clippy rc=0 (test-only panic lint explicitly allowed); cargo fmt rc=0; book build rc=0.
  - [x] LOCKSTEP0297:419/420/269/421 whole committed Git bytes/hash/public reader exact rc=0; book/decision/live pointers aligned.
  Promotion: existing SizeSet decision extended; no new policy. Complete member points/minimum-axis aggregate next .2.
  Commit: STITCHCAD-G1-0297.
