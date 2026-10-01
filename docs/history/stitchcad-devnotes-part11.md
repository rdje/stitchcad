# Sealed archive — StitchCAD dev notes, property-test framework

Immutable historical segment, sealed by leaf `G1-SLICE.3c.2b.2` on `2026-10-01`.

- **Sealed identity:** 15 lines, 1384 bytes, `sha256:ae04eadff5fb3b41b76ecd71af25fde284f5c7e490b369b317aff6d644941880`
- **Coverage:** the oldest live `2026-09-30` lesson (property-test framework), copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-09-30)_ — the first property tests quietly set the framework every later crate inherits

- `sc-units`' suite (`crates/sc-units/tests/property.rs`) hand-rolls 21 properties over a deterministic
  xorshift64* generator with a recorded seed and zero dev-dependencies. That was not a stylistic preference:
  the crate must stay dependency-free to serve `wasm-viewer` and byte-stable golden files, and a framework's
  shrinker pulls a tree into the graph. The choice was made implicitly when `G0-CONTRACT.18` landed the suite,
  but recorded nowhere — so the G1 Open Question "proptest vs quickcheck, decided in `.2`" was still open with
  the answer already shipped beside it.
- Closing `.2` records it as `decision_property-tests-dependency-free-recorded-seed.md` (with `answers:`, so a
  later crate asking "do we use proptest?" finds it): dependency-free hand-rolled with a recorded seed is the
  default on the wasm-viewer critical path; a crate off that path may adopt a framework only by its own
  recorded decision that must not leak into a dependency-free graph. **The first instance of a pattern is the
  decision; record it where the pattern is set, not where the tenth crate re-argues it.**
- This entry is the promoted lesson for the slice: the new decision record carries `answers:`, which is the
  LESSON-PROMOTION promote path, so no decline token is needed.
