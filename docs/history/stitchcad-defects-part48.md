# Sealed archive — D122 and D127 origin/context reads

Immutable historical segment, sealed by leaf `G1-SLICE.5e.1a` on `2026-10-02` (UTC).

- **Sealed identity:** 19 lines, 1666 bytes, `sha256:d87df98a5fb795ffb7e884ebacf7770bcfcad5e930c09e852cdb0005aaa0dd36`
- **Coverage:** D122 and D127; original reports/payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D122** — reference missing-value reads use the wrong origin/context diagnostic.
  - Reproduce: actual size_index without context raises formula_tolerance_unbound; geometry p with
    no value raises formula_unknown; tolerance-origin missing_tol with no value also raises
    formula_unknown. Direct statement baselines exit0 after catching actual refusals.
  - Root: value_of_name uses one reserved unavailable branch and one generic absent-value branch;
    contract3 assigns unbound-name to geometry/recipe/size and tolerance-unbound to tolerance.
  - Impact: reference numeric reads cannot prove origin-specific context/error arguments.
  - Owner: G1-SLICE.5e.1, high priority before using the reference for product input-adapter proof;
    test every origin/available-context distinction. Static metadata-only proof is unaffected.

- **D127** — malformed reference declarations leak host exceptions instead of named refusals.
  - Reproduce: statement input_value with absent kind/origin raises KeyError; list-valued origin
    raises TypeError; a missing measurement value without state raises KeyError. Actual loader/
    direct statements reproduced all four, caught exceptions, baseline exit0.
  - Root: namespace indexes required metadata before guarding its shape and hashes unchecked origin;
    missing-value reader indexes state before producing a diagnostic.
  - Impact: malformed input can terminate the reference consumer without a stable diagnostic.
  - Owner: G1-SLICE.5e.1a now with D122; guard required declaration fields/types and missing-fact state,
    preserve metadata-only static checks and established populated numerical fixtures.
