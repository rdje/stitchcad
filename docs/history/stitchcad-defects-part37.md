# Sealed archive — D106/D107 statement-slice documentation and evidence

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3e.1` on `2026-10-02`.

- **Sealed identity:** 16 lines, 1485 bytes, `sha256:a21dc3025cefe792e2627332f074ad666255d2a84d4c8b68c7e8d69a2608d2ab`
- **Coverage:** D106/D107; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D106** — current literal-storage decision still calls canonical serialization future work.
  - Reproduce: final paragraph of docs/decisions/decision_literals.md says canonical serialization
    and evaluation remain future G1 work;5d8ab5b implements and reviews public expression identity.
  - Impact: durable layer-C proof boundary disagrees with current book/API status and can misroute
    future work. Numeric128-bit literal/i64-binding ruling itself remains correct.
  - Owner/schedule: G1-SLICE.5a.3e.1, P2 now; distinguish completed expression identity from future
    ordered statement identity and evaluation, verify current decision/book/public API agree.

- **D107** — new statement fault runner searched the entire test output for its assertion marker.
  - Reproduce: target/statement_mutations/fault-4.log fails only an expect on valid-statement refusal;
    the marker is supplied by a passing test named assertion_separator_ignores_all_grouped_and_call_comparisons.
    The original whole-output b'assertion' predicate therefore calls that an actual assertion red.
  - Impact: new verification classification can confuse a fixture panic with a discriminating
    assertion failure. The compiled refusal is real, but the published assertion class is unearned.
  - Owner/schedule: G1-SLICE.5a.3e.1, P1 now; explicitly assert fixture acceptance, classify only
    failed-test bodies, reject the observed expect-only output, rerun every statement fault and restore.
