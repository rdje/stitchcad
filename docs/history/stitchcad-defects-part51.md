# Sealed archive — D124 recognition contract

Immutable historical segment, sealed by leaf `G1-SLICE.5b.1c.2` on `2026-10-03` (UTC).

- **Sealed identity:** 9 lines, 866 bytes, `sha256:a3dbe0cc4585d94aa2bea24ab84b8081bc4adeecb643109f5cc0c60ed9f29880`
- **Coverage:** D124 recognition contract; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D124** — excluded loop/function forms have no diagnostic recognition contract.
  - Reproduce: actual static_statement loop(width), repeat(2,width), while(width>0 um) raise
    formula_unbound_name; fn helper(width)=width and macro helper(width)=width raise formula_parse.
    Declared loop scalar is accepted. Direct static baseline exit0, execution callbacks trapped.
  - Root: contract6 promises formula_unsupported for loops/iteration/recursion and functions/macros,
    but grammar1.1 reserves only let/assert/if, no excluded source forms are specified, and grammar6
    requires unknown calls formula_unbound_name. A new reserved spelling would change valid names.
  - Impact: no signoff-quality reproducible boundary for product exclusion diagnostics. Owner:
  G1-SLICE.5b.1c.1 diagnosis/proposal now; .1c.2 applies director ruling before product .5b.3.
