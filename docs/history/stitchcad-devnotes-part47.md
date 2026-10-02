# Sealed archive — canonical literal display

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.1c` on `2026-10-02`.

- **Sealed identity:** 17 lines, 1561 bytes, `sha256:1b718ccaf3de48ad2466af226bbdc3988f3e27fd9e0e62423b2c21db305e0f43`
- **Coverage:** original canonical-literal lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-02)_ — canonical literal display must not conceal a different value

- Actual reference diagnostic reproduces fractional unit and bare-decimal nodes: two canonical-zero
  literals accumulate 4/5 internal quantum and bind 1. L1 previously rounded only the displayed node,
  masking the disagreement with canonical kind:integer identity. Scoped source history identifies
  3704b8a G0-CONTRACT.9 as the introducing parser; later D75/D76 repairs did not alter literal values.
- Convert/round each literal once at input, before expression arithmetic. Counts retain kind and
  bare decimals/pct retain ratio scaling; no angle modulo or arithmetic-node folding is introduced.
  Sixty explicit rows/360 controls use an independent Decimal rounding oracle and kind-preserving
  respellings; six actual guards discriminate quantum, ties, scale, kind and direct unit factors.
- D80/D81 repair duplicate units numbering and a stale live next pointer. Formula contract now
  explains the existing canonical-input boundary; details and honest proof gaps stay in the annex.
- The wider diagnostic exposes D82 early operator rounding and D83 unenforced numeric domains;
  .5a.3b.2/.3 own immediate repairs before product normalization. Published example agreement is
  still curated scope, not complete exact-arithmetic or arbitrary-input production verification.
- Completed rounding evidence and oldest live payloads preserve committed predecessor text.
- promotion: declined (routine reference repair; literal conversion/canonical identity policy unchanged).
