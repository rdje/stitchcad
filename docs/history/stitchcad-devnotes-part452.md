# Sealed archive — whole0311 profile pin and origin

Immutable historical segment, sealed by leaf `G1-SLICE.4c.4.2` on `2026-10-26` (UTC).

- **Sealed identity:** 20 lines, 1857 bytes, sha256:21fc54e05c8bf0f07c100f3f78a9da54f40b1908b2350a0850c3cab254c064ab
- **Coverage:** whole committed source interval from 214c80af6eda898b3061510f384b4035263486e2.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 29857–31714, 1857 bytes, sha256:21fc54e05c8bf0f07c100f3f78a9da54f40b1908b2350a0850c3cab254c064ab
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4c.4.1`
  Status: `done`
  Goal:0311 exact selected profile/schema/content labels and immutable source/resolved SizeSet origin metadata.
  Paths: crates/sc-measure/src/size_set_resolution.rs;crates/sc-measure/src/size_set_resolution/;
  crates/sc-measure/src/size_set.rs;crates/sc-measure/src/size_set/;crates/sc-measure/src/lib.rs;
  crates/sc-measure/src/size_chart_collection.rs;crates/sc-measure/src/mtm_chart.rs;
  crates/sc-measure/tests/size_set_profile_pin_contract.rs; garment/MTM composite tests;
  docs/book/src/spec/size-set-transformations.md;book SUMMARY/status/main/decision/live docs.
  Roadmap8's schema/content versions stay distinct and opaque here; G4 owns grammar/registry/profile validation.
  Source reference must match actual current inputs; allocated resolved identity cannot alias known source entities
  or provenance. No allocator, transformed output, numeric/default/profile resolution or approval claim.
  Own450 whole0310 task,451 oldest0287 lesson and283 ledger from5a87ed0; whole Git/hash/public before retirement.
  Focused pin/origin and affected composite tests, selected strict lint/fmt/book.
  - [x] ROOT CAUSE (WHY + WHERE): roadmap8 separates schema/content; existing symbolic bindings stay G4-deferred; API check rc=0.
  - [x] ADDRESSED: exact pins/origin refuse stale source and input/provenance target aliases; native18 rc=0.
  - [x] NO REGRESSION: affected garment8/MTM7 included; strict selected lint/fmt/book rc=0.
  Verification: profile3+garment8+MTM7 contracts18/0; cargo check/lib, strict clippy and fmt rc=0;
  book rc=0;450/451/283 whole source intervals/hash/public-reader byte equality rc=0.
  LIVE_STATUS reviewed: G1 In Progress5/18 unchanged; G0 closure unapproved, later gates not started.
  Commit: STITCHCAD-G1-0311. Next .4c.4.2; broad verification remains integration/pre-push.
