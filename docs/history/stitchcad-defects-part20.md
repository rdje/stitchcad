# Sealed archive — exact reference arithmetic

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.2` on `2026-10-02`.

- **Sealed identity:** 6 lines, 527 bytes, `sha256:f1308faa8b81d52a512b4769c1a13834e596d7467b7d712ea21bf2945970dbe2`
- **Coverage:** D82, closed and verified by STITCHCAD-G1-0043; description unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D82** — reference arithmetic rounds exact intermediate operators.
  - Reproduce: literal_diagnostic.py actual evaluator: 1 um / 2 + 1 um / 2 → 2 rather than 1;
    0.000001 ^ 2 → ratio 0 rather than exact internal 1/1000000.
  - Root: evaluate square/product/quotient and param_at call rnd before binding, contrary to 4.2.
  - Impact: curated examples can miss accumulated quantization and this is not an exact oracle.
  - Owner/schedule: G1-SLICE.5a.3b.2, immediately after D79 literal repair, before product proof.
