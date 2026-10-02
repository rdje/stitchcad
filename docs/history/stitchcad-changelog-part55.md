# Sealed archive — STITCHCAD-G1-0037

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3c.3` on `2026-10-02`.

- **Sealed identity:** 15 lines, 1284 bytes, `sha256:49280c1fa725af3916ecbf849c2a0b1443cff05a749d97875254d60c4b867a2c`
- **Coverage:** STITCHCAD-G1-0037; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0037 - borrowed machine-form lexing retains exact source spans (leaf `G1-SLICE.5a.1`)

The new core recipe front-end scans ASCII keywords, identifiers, numbers, operators and punctuation
without cloning source names/numbers or interpreting values. Immutable lexemes retain original text
and half-open byte spans; first error/end fuse the iterator. Shared spelling/keyword classification
preserves MachineToken behavior. Typed errors and lexer Debug do not dump customer source. Lexical
success supplies no valid-expression, numeric, canonical-identity or executable-recipe claim.

Thirteen contracts scan every worked machine example and exercise borrowing, precise refusals and
parser-owned boundaries; two privacy/lifetime doctests pass. Nine actual guard mutations fail contract
assertions, restore exact production bytes, and final strict native/WASM checks pass. D73 obsolete ADR
clause links and D74 vertical-tab handling are fixed. Book learning/status/index and the detailed syntax
annex align with code/roadmap ownership; 48 chapters/15 APIs and nine publication probes pass. Older
publication evidence and sealed histories retain exact predecessor bytes. G1 remains 5/18 top-level;
next .5a.2 expression trees. D70's required axes ruling remains unanswered.
