# Sealed archive — D105 canonical expression book status

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3d.3` on `2026-10-02`.

- **Sealed identity:** 8 lines, 882 bytes, `sha256:ae3b000afe784e0d3dfc84f6da3850a12c068ef13bb04486e994c64a6ab5c4fc`
- **Coverage:** D105; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D105** — syntax annex and formula introduction lag implemented canonical expression identity.
  - Reproduce: rg -n 'Product canonicalization and evaluation remain pending|Canonical expressions and recipe evaluation remain' docs/book/src/annexes/formula-syntax.md docs/book/src/spec/formula-language.md.
    Annex line247 says product canonicalization is pending although canonical_form is public at
    8ed2893; the introduction still groups canonical expressions with future evaluation.
  - Impact: readers using the book cannot reliably distinguish available expression identity from
    future ordered recipes/evaluation. No runtime serializer defect is established.
  - Owner/schedule: G1-SLICE.5a.3d.3, P2 now; link current owned-expression API and preserve explicit
    syntax-only/reference/evaluation boundaries. Verify scoped source and rendered book before closure.
