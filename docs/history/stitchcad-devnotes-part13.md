# Sealed archive — StitchCAD dev notes, injected identity

Immutable historical segment, sealed by leaf `G1-SLICE.3c.3c` on `2026-10-01`.

- **Sealed identity:** 18 lines, 1612 bytes, `sha256:38e833498284bc29123b93100ac4dc5c14b79ee98c8b675e3368754d0dfc2c4b`
- **Coverage:** the oldest live identity-layer lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-09-30)_ — the identity layer: determinism is a property of the generator, not the id

- `G1-SLICE.3a` landed G1's first new product code: `sc_core::ontology`'s `EntityId` (a hand-rolled
  dependency-free ULID), the injected `IdGenerator`, `EdgeRef`/`PointRef`/`LocalTag`, and the bounded exact
  `Rational` a parameter is stored in — 40 tests (31 unit, 9 recorded-seed properties), and `sc-core` still
  cross-builds to wasm.
- The design point worth keeping: a real ULID embeds a wall-clock timestamp and randomness, yet recipe
  re-evaluation and CLI replay must be byte-identical and canonical content carries no wall-clock. So
  **determinism lives in the generator, not the id** — `EntityId` is just 128 bits, and whoever creates objects
  injects either a `DeterministicIdGenerator` (a counter, for tests and replay) or a clock-plus-entropy
  generator (production, from the command bus at `G1-SLICE.6`). The clock never enters domain code, so the same
  commands always yield the same ids.
- Two house conventions re-confirmed, not reinvented: fallible arithmetic is `checked_*`, not `add`/`sub`
  (clippy's `should_implement_trait`; the precedent `sc-units` set), and a test *helper* that is not a `#[test]`
  fn is not covered by `.clippy.toml`'s `allow-expect-in-tests`, so it carries its own targeted `#[allow]`
  rather than swallowing a failure with `unwrap_or`.
- promotion: declined — the durable design choices are the three decision records `G1-SLICE.3` landed; this is
  one slice's implementation history, and the two conventions are already the house style, not a new rule.
