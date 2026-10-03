# Sealed archive — D130 draft input origin overgeneralization

Immutable historical segment, sealed by leaf `G1-SLICE.5b.2b` on `2026-10-03` (UTC).

- **Sealed identity:** 8 lines, 772 bytes, `sha256:5f6b8b11ce156b48b89998023d60e489cb5e532cc63360b3d514017b5caa5923`
- **Coverage:** D130 original report before the type repair; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D130** — draft generic declaration builder admits non-length measurement/Ease metadata.
  - Reproduce: initial formula_declaration_contract input matrix passes Angle/Area/Ratio/Count/
    Boolean at Measurement and Ease origins through FormulaDeclaration::input; seven tests pass0.
  - Root: reference metadata matrix was overgeneralized into a canonical input adapter. Existing
    measurement-metadata/ease-inputs contracts require canonical LengthDeclaration targets.
  - Impact: the draft product declaration API could claim an impossible canonical measurement kind.
  - Owner: G1-SLICE.5b.2b before commit; generic scalar metadata must have a separate three-domain
    type, while measurement/Ease construction retains forced-length canonical record borrowing.
