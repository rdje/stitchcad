# Sealed archive — canonical buttonhole derivation

Immutable historical segment, sealed by leaf `G1-SLICE.4a.3` on `2026-10-02`.

- **Sealed identity:** 13 lines, 1158 bytes, `sha256:319f11b3dbe3a61a5ac6f1b67fb578298294682159067621699271f5be3bea48`
- **Coverage:** canonical buttonhole derivation lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-01)_ — buttonhole length retains a single canonical derivation source

- Button/hole pairs share current placement/count validation. The hole source borrows its owning
  Closure's button-size declaration and recipe operation; no second authored/cached physical length
  can drift. Replacement size/operation changes the observed source while old revision views remain
  unchanged. Current Design queries, not old views, must drive execution and approvals.
- Recipe/Design validates operation kind/dependency; G3 executes physical derivation. The ontology
  specifies that dependency but no physical formula or clearance, so G1 invents neither. DeferredToG3
  keeps that missing execution proof separate from geometric/profile validation and readable sources.
- Five new tests bring Closure contracts to fifteen; independent substitute-source mutations fail
  red. Compile-fail coverage refuses a separate length field; strict Rust/WASM/book pass. Source
  inspection remains available even while current placement repairs block execution.
- promotion: promoted by `decision_physical-cut-copies-have-stable-identities.md`'s button/hole source.
