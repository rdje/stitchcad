# Sealed archive — completed exact rational primitive parent scope

Immutable historical segment, sealed by leaf `G1-SLICE.5c.3.2` on `2026-10-25` (UTC).

- **Sealed identity:** 12 lines, 981 bytes, sha256:002280aaf5a1c9a0afec6302c664efd53570454c27dfd1ee74befd2dd8c7da03
- **Coverage:** whole committed source interval from a9d86c4015fe5ecda988a7ee0be121e189ef63e6.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 20656–21637, 981 bytes, sha256:002280aaf5a1c9a0afec6302c664efd53570454c27dfd1ee74befd2dd8c7da03
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.1`
  Status: `done`
  Goal: exact signed rational value primitive with positive denominator and reduced width128.
  Acceptance: full unsigned magnitudes/signs, zero, reduction and cancellation before width
  judgment, overflow-safe bounded workspace and typed numerator/denominator measured refusal.
  Independent arbitrary-precision oracle checks arithmetic; no dependency/license/WASM guess.
  Director2026-10-25: next independent product feature after the dirty0243 slice is committed.
  D154/D156 and remaining audit verification stay owned; they do not block this pure primitive.
  Start with a bounded implementation slice and one affected test; broad integration stays deferred.
  Children: .1 canonical value construction; .2 bounded arithmetic workspace/reduced-width refusal;
  .3 independent oracle integration. Parent stays open until all are implemented.
  Verification: focused native target4/0 with34 Fraction rows in0247; broader/platform .5c.4.
