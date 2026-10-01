# Sealed archive — identifier spelling and binding authority

Immutable historical segment, sealed by leaf `G1-SLICE.4c.1` on `2026-10-02`.

- **Sealed identity:** 14 lines, 1283 bytes, `sha256:317f385abdfa4f4df460d761f5e26b6a4d048693420188a0465e6cc1ce309b6a`
- **Coverage:** identifier grammar/binding lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-01)_ — identifier syntax and binding authority are separate checks

- Core MachineToken is shared below metadata and recipes. It preserves ASCII lower-snake bytes,
  refusing malformed starts/segments, whitespace, Unicode lookalikes, uppercase and the three grammar
  keywords. Built-in parameter names remain valid references; metadata/recipe owners must separately
  refuse rebinding. Tokens provide no localized display label, text scalar or source-truth claim.
- Six contracts and a private-field doc-test pass. Four independent spelling/keyword mutations fail
  actual regressions; restored strict Rust executes 305 tests with WASM/book green. Grammar and book
  declare the same syntax, including digit-bearing segments; there is no normalization or auto-rename.
- Measurement metadata/runtime integration and observed-CI signoff are separate safe slices. The
  completed length-input contract/checklist moves unchanged to a bounded semantic sibling before
  parent pressure grows; the current checklist remains first. D66 landing-page status is owned by
  the runtime slice; D65 remains scheduled at its required-seal trigger, with history now 62/64 files.
- promotion: promoted by `decision_length-declarations-retain-state-and-provenance.md`'s token section.
