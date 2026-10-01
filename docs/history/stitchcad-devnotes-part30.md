# Sealed archive — canonical numeric availability

Immutable historical segment, sealed by leaf `G1-SLICE.4b.3` on `2026-10-02`.

- **Sealed identity:** 15 lines, 1356 bytes, `sha256:7fa4db8719f34a01fb62ad926185cbe08c6c90647e6ebc35670050f2b0776086`
- **Coverage:** numeric availability and source-truth lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-01)_ — numeric availability and source truth are separate contracts

- A shared core LengthDeclaration holds exactly one authored state and source. Known requires a
  nonempty distinct evidence inventory; assumed, unknown, preference and derived carry their distinct
  required record identities. Unknown and derived cannot store numeric values, and their queries name
  the observation or formula they require. Signed lengths and explicit zero remain authored input;
  procedure-specific physical domains belong to measurement/recipe validation, not a generic guard.
- sc-measure can borrow core declarations without copying state or introducing a core→measurement
  dependency cycle. A known claim is not proof that evidence exists or fits a scope. Design/recipe and
  G4 retain those checks; no global export approval is inferred from authored state.
- Eight contracts plus three privacy/state doc-tests pass; disabling empty/duplicate evidence checks
  or returning zero for unknown/derived each makes its regression fail. Strict Rust executes 298
  tests; WASM/book and focused censuses pass. D64 corrects stale ontology coverage prose; completed
  construction contracts and ten checklists partition unchanged with committed-payload comparison.
- promotion: promoted by `decision_length-declarations-retain-state-and-provenance.md`.
