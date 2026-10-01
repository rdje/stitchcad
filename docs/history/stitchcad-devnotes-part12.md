# Sealed archive — StitchCAD dev notes, ontology slice decomposition

Immutable historical segment, sealed by leaf `G1-SLICE.3c.3b` on `2026-10-01`.

- **Sealed identity:** 16 lines, 1570 bytes, `sha256:a2f04e3db6d57d68ad04b9af26f4e406ba704a90f06fb8550e644473aa51aa96`
- **Coverage:** the oldest live decomposition lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-09-30)_ — decompose a too-big leaf and record its design boundaries before writing code

- `G1-SLICE.3` (the whole garment ontology — identity, the persistent-identity contract, and nine
  geometry-bearing object types with their invariants) was one leaf but is three signoff-quality slices:
  `.3a` the identity types, `.3b` the contract that resolves references under split/merge/reverse/delete,
  `.3c` the object types. They are strictly ordered — the contract consumes the types, the objects consume
  both — so a frontier that tried to take them as one would have produced one unreviewable commit.
- The three cross-cutting design questions were settled and recorded as layer-C decisions BEFORE any code,
  because each is the kind of choice a later slice would otherwise re-litigate or silently contradict: the
  `EntityId` is a dependency-free hand-rolled ULID with an **injected** generator (so recipe/CLI replay stays
  byte-deterministic and the wall-clock never enters canonical content); the edge parameter is a **bounded
  exact rational in `sc-core`**, not the ppm `Ratio` and not the formula evaluator's bigint — with a named
  promotion trigger if `sc-geometry` ever needs it in `sc-units`; and G1 enforces only the **structural**
  invariants, handing CCW winding, simplicity and closure to `G2-2D.1` as a visible `DeferredToG2` state
  rather than claiming a 2D proof it cannot make. **Decide and record the boundaries a slice inherits, then
  implement inside them** — the records are why the next session does not reopen them.
