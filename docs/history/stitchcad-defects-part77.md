# Sealed archive — Original committed D159 assertion classifier report

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.n.f3` on `2026-10-04` (UTC).

- **Sealed identity:** 7 lines, 666 bytes, `sha256:2c21089cc0d16cb8449423fbe460eb3fd7f0f87e8c1a59086df4e111b9d6d9da`
- **Coverage:** Original committed D159 assertion classifier report; complete exact ecc67e5 HEAD byte interval.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D159** — literal/arena native assertion predicates accept failing expect-only noise.
  Reproduce: evaluate each actual result assertion AST with101/FAILED and a passing assertion_name
  plus an expect-only failed body; both accept, target/d159-classifier-baseline.log, no dispatch.
  Root: b'assertion' searches all output rather than the failed-test body; scoped introducing blame
  retained in baseline. Impact: mutation evidence can claim an assertion red without one.
  Own G1-SLICE.5b.4c.h2.b.p.n.f3, P1 immediately next with these entries' D156 guards;
  calibrated actual classifier body faults and exclusive native source/current artifact restoration.
