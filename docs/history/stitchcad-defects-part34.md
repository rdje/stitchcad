# Sealed archive — D103 canonical byte spelling gap

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3d.1` on `2026-10-02`.

- **Sealed identity:** 11 lines, 1082 bytes, `sha256:dc5187721969896f8a121749c5db179701eb213e2608cf1210f508ee366d6448`
- **Coverage:** D103; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D103** — the expression canonical-byte contract leaves unary minus and square unspecified.
  - Reproduce: grammar4 defines op/name/kind:integer, spacing and two binding examples, but neither
    publishes exact unary/square bytes. The actual reference returns tuples, not an S-expression
    serializer. Normalized inspection shapes use neg/square as test labels, not wire contracts.
  - Impact: selecting persistent identity bytes silently would preempt the director's language review.
    Reusing (neg child)/(square child) also collides with distinct ordinary calls of those names;
    they are not reserved keywords, and normalization preserves calls before name/type validation.
  - Owner/schedule: G1-SLICE.5a.3d.1 documents concrete choices now; .3d.2 implements only after
    ruling and .3d.3 verifies closure. P1 before production canonical bytes; no existing serializer bug.
  - Proposal: unary (- child), square (^2 child); alternative square (^ child count:2), whose fixed
    exponent is operator payload, not another AST node. Both avoid named-call collisions.
