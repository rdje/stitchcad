# Sealed archive — completed correctly rounded sqrt scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5d.2.2` on `2026-10-25` (UTC).

- **Sealed identity:** 20 lines, 1731 bytes, sha256:0e656b215ce455118f46725e6dfdcbd20c22bfb03e373510d9575886af99efd2
- **Coverage:** whole committed source interval from e646da3ce943dd3f2f329013126085b14966bc79.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 28599–30330, 1731 bytes, sha256:0e656b215ce455118f46725e6dfdcbd20c22bfb03e373510d9575886af99efd2
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.2.1`
  Status: `done`
  Goal:0255 correctly rounded sqrt area/ratio via the recorded exact midpoint/bisection plan.
  Paths: crates/sc-core/src/recipe/roots.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_root_value_contract.rs
  Own bounded private integer root, metadata signature reuse, actual negative/kind refusals and
  existing25-row root fixture integration (sqrt subset now, hypot .2 next). Ratio returns ppm;
  no intermediate float/width/storage guess. Only completed rounded scalar result is checked.
  Source-call provenance remains .5e.3; no evaluator/irrational-family closure claim. One focused
  native root target; broad/platform/mutation .5d.4/pre-push. Own complete327/328/227 retention,
  preserving four closed marks/allowance nodes/paths under caps; sync book/live before hypot.

- [x] REPRODUCE / ISSUE:5fae1cb supplies root plan/fixtures but no production sqrt, inspection0.
- [x] ROOT CAUSE: exact midpoint and finite integer root avoid approximation/tie errors; cargo test
  --offline -p sc-core --test formula_root_value_contract:2 passed/0 failed, rc=0.
- [x] FIX: bounded bisection/ratio scale and metadata signatures, actual negative/kind refusals.
- [x] ADDRESSED: same native target replays15 independent sqrt rows and wrong-kind cases,2/0, rc=0.
- [x] NO REGRESSION: same target exercises completed values/full widths;15 rows/2 tests, rc=0;
  earlier arithmetic unchanged, broad/platform .5d.4/pre-push, provenance .5e.3.
- [x] LOCKSTEP:327/328/227 whole Git/public bytes exact, rc=0; four legacy IDs/Goals retained,
  book/live aligned and stale quantity-follow prose fixed. promotion: numerical decision gains sqrt scope.
