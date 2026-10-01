# Sealed archive — StitchCAD ontology decomposition

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4c.2` on `2026-10-01`.

- **Sealed identity:** 21 lines, 1831 bytes, `sha256:ab5e04cacbbc8e374be8c6fcf8c316c8700e0c6e09d10c025e2ecd73df047f92`
- **Coverage:** STITCHCAD-G1-0003, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0003 - the ontology leaf is three slices, and its design boundaries are recorded first (leaf `G1-SLICE.3`)

`G1-SLICE.3` named the whole garment ontology as one leaf — identity, the persistent-identity contract, and
nine geometry-bearing object types with their invariants. That is three signoff-quality slices, not one, and
they are strictly ordered (the contract consumes the identity types; the objects consume both). This slice
decomposes `.3` into `.3a` (identity types), `.3b` (the persistent-identity contract) and `.3c` (the object
types), and records the three cross-cutting design boundaries BEFORE any code, so each implementation slice
builds against a fixed design rather than re-deciding it.

The three decisions, each a layer-C record with `answers:` so a later slice asking the question finds it:
`decision_entity-identity-ulid-injected-generator.md` — an `EntityId` is a dependency-free hand-rolled ULID
produced through an injected `IdGenerator`, because `sc-core` builds for wasm and recipe/CLI determinism
forbids a free-function clock; `decision_edge-parameter-bounded-exact-rational.md` — the edge parameter `t` is
a bounded exact rational in `sc-core`'s ontology, not the ppm `Ratio` and not the formula evaluator's
arbitrary-precision rational, with a named promotion trigger to `sc-units` if `sc-geometry` (G2) needs it;
`decision_ontology-invariants-structural-g1-geometric-g2.md` — G1 enforces the structural invariants and hands
CCW winding, simplicity, closure and intake conservation to `G2-2D.1` as a visible `DeferredToG2` state,
never a 2D-correctness claim.

No Rust changes; `make gate` stays `=== all doctrines green ===` and the regenerated Knowledge Map carries the
three new records. The tree is 18 leaves; the frontier advances to `.3a`, G1's first new product code.
