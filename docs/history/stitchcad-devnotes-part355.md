# Sealed archive — complete rational products and quotients lesson

Immutable historical segment, sealed by leaf `G1-SLICE.5e.3.3` on `2026-10-26` (UTC).

- **Sealed identity:** 13 lines, 1117 bytes, sha256:8229cab814e4d315295a6fa58b9c7fb0d9b82e202aa630de91e9eb6377140222
- **Coverage:** whole committed source interval from 1e836c155ca107c3339b4720b063af87944e0d77.
- Whole source interval: DEV_NOTES.md bytes 27535–28652, 1117 bytes, sha256:8229cab814e4d315295a6fa58b9c7fb0d9b82e202aa630de91e9eb6377140222
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

## _(2026-10-25 UTC)_ — exact rational products and quotients

0245 .5c.1.2.1 cross-cancels normalized factors before checked multiplication, so oversized raw
products can reduce to valid128-bit results. The surviving factors are pairwise coprime across the
fraction; product widths are therefore the fully reduced widths. Division checks zero before
forming the reduced reciprocal and reuses multiplication. Canonical zero/sign and constructor
contracts remain intact. RationalWidth reports actual component bits and fixed128 bound/token;
ZeroDivisor retains formula_division. Four bounded64-bit partial products recover the overflowing
product's upper128 bits exactly; their sums fit u128 by the documented carry bound.
One affected native target passes2/0, rc=0, covering cancellation, signs, zero priority and129/256-bit
numerator/denominator witnesses. No full regression/Clippy/WASM/oracle-campaign pass is claimed.
Sums/differences and typed/evaluator integration remain owned. Whole0244 task retained314; book/live
stay aligned. Promotion declined: existing exact-numeric contract and focused-check policy.
