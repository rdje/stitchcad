# Sealed archive — StitchCAD dev notes, structural pieces

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4a.2b` on `2026-10-01`.

- **Sealed identity:** 15 lines, 1334 bytes, `sha256:1b362d26c2e64256953d65ed0c454b8b235665a8632e444ee9f0f4c7f6b12ade`
- **Coverage:** the oldest structural-piece lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-01)_ — structural pieces and the endpoint/range distinction

- `G1-SLICE.3c.1` implements immutable pieces with private validated content. A public definition is
  editable input; the constructor checks live references, distinct cyclic cut loops, the cut plan,
  complete labels and explicit unresolved-material reasons. Cut quantity means total physical copies;
  mirrored pairs require an even quantity. Label cut information derives from the plan.
- The geometric state has only `DeferredToG2`; cyclic ordering cannot prove endpoint coincidence,
  winding or containment. Endpoint queries likewise do not certify an entire contour. The tracked D55
  counterexample deletes a middle fragment while both original endpoints still resolve. `.3c.2` owns
  the range contract immediately next, before sewing spans can make that mistaken inference.
- Checks: 12 piece-contract tests, existing unit/property suites, a compile-fail privacy check, strict
  clippy, wasm cross-build and book build. The first clippy run refused manual divisibility syntax;
  using the toolchain's integer predicate resolved it before signoff.
- promotion: declined (structural/geometric separation is already a decision record; the new range
  risk is an open defect and scheduled implementation contract in the task tree, not a settled rule).
