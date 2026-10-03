# Sealed archive — D140 original geometry-kind report

Immutable historical segment, sealed by leaf `G1-SLICE.5f.3a` on `2026-10-03` (UTC).

- **Sealed identity:** 9 lines, 877 bytes, `sha256:c59b89a4722ff77c6a495f06cf441173699a84161cc0b050ce4bf3642fcd030a`
- **Coverage:** D140 original geometry-kind report; original payload unchanged from 481b47f4186a9b1d4f9bba26c9edcf9c809713a4.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D140** — reference resolve_geometry discards point-coordinate kinds: exprs x=1.0,y=2 deg
  returns (Fraction(1000000),Fraction(2000000)) as coordinates, while valid x=1 mm,y=2 mm gives
  (Fraction(1000),Fraction(2000)); direct actual loader/resolve calls reproduce, rc=0. Source takes
  xs.v/ys.v without Length checks. Edge len=1 deg raises formula_dimension with {}, after evaluating
  that expression. Impact: invalid coordinate kinds can silently become geometry; static rejection
  before values is unproved for operation formulas. Owner G1-SLICE.5f.3a, P0 immediately after
  D138's dimension constructor slice and before product expression checker .5b.3c.2b.2. Require all
  coordinate/edge Length checks before numeric work, exact actual/wanted payloads and original
  provenance, actual compiled counterfactuals and no accepted prefix/cache mutation on refusal.
