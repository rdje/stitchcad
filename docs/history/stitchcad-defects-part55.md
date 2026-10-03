# Sealed archive — D132/D133 publication markup and warning refusal reports

Immutable historical segment, sealed by leaf `G1-SLICE.5b.2c.2` on `2026-10-03` (UTC).

- **Sealed identity:** 15 lines, 1304 bytes, `sha256:2c4850a67bd0f22f1bad9f613aad197cad4d6dda5a9fee445c9122077b2d50e7`
- **Coverage:** D132/D133 publication markup and warning refusal reports; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D132** — draft namespace chapter renders a Rust generic as an unclosed HTML tag.
  - Reproduce: run_book_publication_probes.sh at G1-SLICE.5b.2c.2 prints unclosed HTML tag
    `<formuladeclaration>` for plain `TryFrom<FormulaDeclaration>`; nine census probes still pass0.
  - Root: raw angle brackets outside code markup become HTML instead of the intended API text.
  - Impact: mdBook omits the generic parameter in the reader's only project view.
  - Owner: G1-SLICE.5b.2c.2, fix before commit; wrap exact generic in code and verify source/rendered
    bytes and warning-free publication, without weakening census or changing grammar.

- **D133** — publication runner accepts a successful build containing a renderer warning.
  - Reproduce: namespace publication log has an unclosed HTML tag warning, followed by55 chapters/
    36 API rows and9 pass/0 fail, rc=0. This is a real product-documentation false green.
  - Root: runner builds then checks topology/status; producer never checks builder diagnostics.
  - Impact: missing rendered API text can be reported as a complete publication check.
  - Owner: G1-SLICE.5b.2c.2 before commit; retain the actual builder's diagnostic output and refuse
    warnings, with a copied-book actual malformed-generic counterexample and repaired baseline.
