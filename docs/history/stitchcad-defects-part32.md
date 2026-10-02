# Sealed archive — D84 reference review and D100 label

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3c.3` on `2026-10-02`.

- **Sealed identity:** 22 lines, 2037 bytes, `sha256:9561b9c34d3c940e643e2eb5a402e1e848a23237c51f50afc5036f4328a04918`
- **Coverage:** D84/D100; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D84** — normalized formula angle storage conflates direction with signed/multi-turn sweep.
  - Contract evidence: formula kind table 2 says angle is normalized and includes direction/sweep/
    grain deviation; units 1.2 normalizes entity angles and equality; grammar 6 arc_length uses
    angle * pi/180 * radius, and unary/ordinary angle arithmetic is exact by formula 4.2.
  - Consequence: binding 360 deg then using it as sweep becomes zero if every let angle normalizes,
    unlike the same direct full-turn expression. Normalizing a negative sweep also changes its sign.
    sc-units Angle explicitly represents a normalized direction; it cannot preserve a full turn.
  - Impact: canonical literal/binding/equality semantics cannot safely inherit that entity type without
    a direction/sweep distinction. This is a contract decision, not just a missed modulo call.
  - Owner/schedule: G1-SLICE.5a.3b.3c verifies bindings/equality and signed inverse-trig behavior.
    Director ruling received 2026-10-02: preserve signed/multi-turn formula angles; normalize entity
    directions. Canonical record: `docs/decisions/decision_angles.md`.
    Specifications align now; reference atan/atan2 still normalize outputs, owned repair pending.
    Scalar/rational domain repairs .3a/.3b proceed independently. D84 remains open until verified.

- **D100** — the current numeric parent’s children summary still calls canonical limits i64.
  - Reproduce: inspect G1-SLICE.5a.3b.3b Children: “reference canonical/binding i64 limits”.
    Its .3b child and decision_literals specify128-bit canonical literal width versus i64 bindings.
  - Root/impact: D97 corrected the parent Goal, but the adjacent live children label escaped that
    scoped repair; it can misdirect a reader even though the verified numeric implementation agrees.
  - Owner/schedule: `G1-SLICE.5a.3b.3c.3`, P2 next in the final reference review; preserve the old
    label/evidence, correct the live summary and verify current parent/child contract agreement.
