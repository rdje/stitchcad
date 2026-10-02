# Sealed archive — D95 literal-node storage ruling

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.3b` on `2026-10-02`.

- **Sealed identity:** 13 lines, 1199 bytes, `sha256:52a62b361fb9ea48abea4cff08af200158673cd2382d0a4be57c277d0e69bca1`
- **Coverage:** D95 original description and received ruling, copied unchanged; verified closure is recorded live.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D95** — signed literal endpoint policy is ambiguous between canonical storage and unary syntax.
  - Reproduce: actual parse -9223372036854.775808 deg creates neg(lit angle:9223372036854775808)
    and evaluates to i64 MIN; the literal child alone is positive2^63 and cannot fit signed i64.
    -9223372036854.775807 deg - 0.000001 deg reaches the same endpoint using i64-safe children.
  - Contract tension: formula2 gives i64 internal values; grammar1/4 separates minus and stores
    kind:integer literal nodes; exact arithmetic uses128-bit-bounded rational values. The contract
    does not explicitly choose whether that literal-node integer is itself restricted to i64.
  - Impact: an undocumented choice would reject a valid signed endpoint spelling, widen persisted
    literal nodes, or alter canonical operator identity by sign folding.
  - Ruling received: exact128-bit literal nodes; i64 only for bound numeric values.
    Adopted by .3a in `docs/decisions/decision_literals.md`; no sign folding.
  - Owner/schedule: G1-SLICE.5a.3b.3b.3b verifies the canonical boundary before D95 closure.
    Independent .3a repairs numeric binding storage now; .3c owns complete boundary review.
