# Sealed archive — reference structural bounds and fragment ownership

Immutable historical segment, sealed by leaf `G1-SLICE.5a.2a` on `2026-10-02`.

- **Sealed identity:** 18 lines, 1734 bytes, `sha256:dc1277c60ffc5854d22e0530acef579d9512253600d39f61932cc139c805ac7f`
- **Coverage:** D75 and D77, closed and verified by STITCHCAD-G1-0038; logged descriptions unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D75** — formula reference census omits function-call arguments from structural measurements.
  - Reproduce: copied-book dart_count expressions abs(1 plus 128 zeros) and abs-wrapped 17-level if
    keep the published value 1, yet run_formula_language_census.sh returns rc=0 for both, reporting
    seven nodes/zero depth/zero mismatches. Actual forms have 258 nodes and 17 nested conditionals.
  - Root cause: count_nodes/if_depth traverse only tuple children; call arguments are stored as lists.
    Depth is compared only in aggregate L8, so hidden descendants evade both measurement and refusal.
  - Impact: the independent oracle falsely certifies pathological expressions before product parsing;
    reference structural-limit evidence is unsound for calls and conditionals inside their arguments.
  - Owner: `G1-SLICE.5a.2a`, priority blocking parser proof, fix now with complete iterative traversal,
    early depth refusal, paired bound controls and actual guard mutations; unchanged language limits.

- **D77** — task coverage census rejects a sibling referenced through a valid Markdown fragment link.
  - Reproduce: full make probes gives tree coverage 5 pass/2 fail, rc=2 overall; direct census reports
    G1-SLICE-formulas ORPHAN despite its parent link ending .md#lexical-contract-and-evidence--preserved-from-60c7305.
  - Root cause: sibling-owner regex requires the closing parenthesis immediately after .md.
  - Impact: ordinary section navigation falsely blocks the owned evidence partition's handoff.
  - Owner: `G1-SLICE.5a.2a`, small blocking publication prerequisite, fix now with optional fragment
    recognition and paired exact-target controls; preserve unlinked sibling refusal and owner semantics.
