# Sealed archive — public operator invariants

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.3c.1` on `2026-10-02`.

- **Sealed identity:** 18 lines, 1673 bytes, `sha256:2f4b81de998ce02435c020b4bfc3a7ad4a7e313b0f16ad8d9eea50ca97d42982`
- **Coverage:** public-operator lesson; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-02)_ — public operators must close the constructor invariant

- D89 review found Length’s Add/Sub directly construct Self from raw integer sums/differences. Valid
  ±1 km operands produce ±2 km lengths without typed refusal; constructors and checked methods
  reject those results. The private-field domain guarantee was therefore false for public operators.
- Trait Output now returns Result<Length,UnitError>, delegating to checked_add/checked_sub. Ordinary
  values, inclusive endpoints, cancellation and signed crossings preserve the same exact numeric
  contract. Callers migrate to `(left + right)?` / `(left - right)?`; crate docs demonstrate handling.
- Public predecessor tests fail three assertions, not compilation. Four current contracts include an
  explicit Result type and a nine-by-nine i128 pair oracle; six actual production bypass/operation/
  saturation mutations compile and fail assertions, then restore source bytes exactly.
- D90 also surfaced: DomainExceeded lacks the failing operation. .3b.1b owns its public error/call-site
  repair immediately next. This slice certifies operator domain closure, not complete diagnostic
  context, formula normalization/evaluation, geometry, MCP or production release. D83/D84 retain owners.
- D91: book L6b mistook valid inline Rust question-mark handling for formula syntax (13 pass/2 fail).
  Explicit Rust fences unblock publication; .3b.1c owns context-aware census proof after D90.
- Completed rational protocol/checklist/journal and oldest live payloads preserve predecessor bytes.
- promotion: declined (routine enforcement of the existing numeric domain and typed-refusal contract).
