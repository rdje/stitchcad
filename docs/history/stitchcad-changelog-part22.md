# Sealed archive — StitchCAD deterministic identity layer

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4d.1` on `2026-10-01`.

- **Sealed identity:** 24 lines, 2100 bytes, `sha256:9050689cff4c69c2dd7ff1970e54b5398a0891411e73555ae99d34c6534cfa3c`
- **Coverage:** STITCHCAD-G1-0004, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0004 - the identity layer: determinism is a property of the generator, not the id (leaf `G1-SLICE.3a`)

G1's first new product code. `sc_core::ontology` now carries the identity layer the whole ontology rests on,
implemented against the two design decisions `G1-SLICE.3` recorded:

- `id` — `EntityId`, a hand-rolled dependency-free ULID (128 bits: a 48-bit timestamp and 80 bits of
  randomness, 26-character Crockford base32, lexicographically sortable), and the injected `IdGenerator` trait
  with a `DeterministicIdGenerator`. The design point: a real ULID embeds a wall-clock and randomness, yet
  recipe re-evaluation and CLI replay must be byte-identical and canonical content carries no wall-clock — so
  determinism lives in the *generator*, not the id. Domain code never reads a clock; the composition root
  injects a deterministic generator for replay and a clock-plus-entropy one for production (`G1-SLICE.6`).
- `rational` — `Rational`, a bounded exact rational (`i64` numerator/denominator, `i128` intermediates, reduced
  canonical form). The four operators never round, so a parameter survives unbounded splits and merges with no
  drift; a result past `i64` is a typed `UnitError::Overflow`, never a wrap. Not `sc-units`' ppm `Ratio`, not
  the formula evaluator's bigint.
- `reference` — `EdgeRef`/`PointRef` (the creating operation's `EntityId` plus a persistent `LocalTag`, never
  an array index) and `Param`, a `Rational` constrained to `[0, 1]` at construction.

Validation: `cargo test -p sc-core` → 31 unit + 9 dependency-free recorded-seed properties, all green;
`make check` clean at clippy `-D warnings`; `make wasm` cross-builds `sc-core`; `make gate` → `=== all doctrines
green ===`. Two house conventions re-confirmed: fallible arithmetic is `checked_*` (clippy's
`should_implement_trait`, the precedent `sc-units` set), and a non-`#[test]` helper carries its own targeted
`#[allow(clippy::expect_used)]` because `.clippy.toml`'s `allow-expect-in-tests` does not reach it. The
frontier advances to `.3b` (the persistent-identity contract).
