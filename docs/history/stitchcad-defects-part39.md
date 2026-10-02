# Sealed archive — D110 available formula diagnostic context

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3e.3` on `2026-10-02`.

- **Sealed identity:** 9 lines, 879 bytes, `sha256:8706139cc7f71d52362943aac5fd765e4176676a4130082fe2e03c0aa61e2180`
- **Coverage:** D110; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D110** — formula diagnostics unconditionally require context malformed syntax cannot supply.
  - Reproduce: contract5.2 says every token carries statement index and canonical expression;
    parsing a non-ASCII whole source preflights before boundaries, while an invalid expression has
    no normalized accepted syntax to canonicalize. Public recipe error correctly reports None for
    the first case and exact span/rule rather than invented expression identity for the second.
  - Impact: normative unconditional wording contradicts the documented immutable syntax API and
    demands fabricated context or a false compliance claim. Runtime diagnostic obligations remain.
  - Owner/schedule: G1-SLICE.5a.3e.3, P2 now; qualify context availability, preserve exact source/
    typed rule and known1-based index, map semantic/command diagnostic argument owners explicitly.
