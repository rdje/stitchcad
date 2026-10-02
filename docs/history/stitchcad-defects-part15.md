# Sealed archive — formula clause references and ASCII whitespace

Immutable historical segment, sealed by leaf `G1-SLICE.5a.1` on `2026-10-02`.

- **Sealed identity:** 15 lines, 1363 bytes, `sha256:7eed9d05b5f9838f5f34c62032512f8d07579f4df82c1d771bcaf84a75e373bc`
- **Coverage:** D73 and D74, closed and verified by STITCHCAD-G1-0037; logged descriptions copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D73** — ADR-0003 points implementation/re-open guidance at obsolete formula chapter clauses.
  - Reproduce: `rg -n '§10|§11' docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md`
    finds examples and exclusions references, while `rg -n '^## [0-9]' docs/book/src/spec/formula-language.md`
    shows only §1–§9; examples live in examples.md and exclusions are §6.
  - Impact: durable implementation guidance points at clauses that do not exist after partitioning.
  - Owner: `G1-SLICE.5a.1`, correct now as part of formula contract recovery; use explicit linked
    examples §2–§4 and main exclusions §6, verify targets and retain locked language decisions.

- **D74** — formula lexer does not skip ASCII vertical-tab whitespace promised by its protocol.
  - Reproduce: `cargo test -p sc-core --test formula_lex_contract` gives 12 pass/1 fail, rc=101;
    whitespace-gap fixture refuses byte 11 (vertical tab) as UnsupportedCharacter. The scanner's
    is_ascii_whitespace predicate omits that character; the input/position diagnostic is correct.
  - Impact: generic ASCII whitespace scope is inconsistent with the owned lexical contract.
  - Owner: `G1-SLICE.5a.1`, fix now with explicit vertical-tab handling and the gap regression;
    retain exact single-space unit-literal validation as a later parser obligation.
