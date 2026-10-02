# Sealed archive — D104 crate numerical/status documentation

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3d.2` on `2026-10-02`.

- **Sealed identity:** 9 lines, 865 bytes, `sha256:9f08224048750adadb45e24288d938fb79f780da24279357e108abbb8835f33c`
- **Coverage:** D104; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D104** — crate overview still states every angle is i64, omitting wide raw formula literals.
  - Reproduce: sc-core/src/lib.rs numerical-contract paragraph says every angle is an i64 count of
    microdegrees; its recipe summary also omits current literal normalization. Actual FormulaLiteral
    holds full-u128 positive magnitudes with separate unary syntax; D95 distinguishes numeric binding.
  - Impact: public Rust documentation can mislead integration authors about the formula/storage
    boundary despite the current book/decision being correct. No runtime numeric conversion defect.
  - Owner/schedule: G1-SLICE.5a.3d.2, P2 in the current serializer slice. Correct crate/API status and
    numerical scope with explicit entity direction versus formula literal/binding distinctions;
    verify Rust docs/public contract and book agree before closure.
