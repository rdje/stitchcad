# Sealed archive — semantic bounds and delimiter nesting

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.1a` on `2026-10-02`.

- **Sealed identity:** 19 lines, 1803 bytes, `sha256:c9686523a1b7a970e7495829a38e637f53ba65420374ae36e5a151f12a38350d`
- **Coverage:** original syntax-stack lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-02)_ — semantic bounds and delimiter nesting are different parser obligations

- The formula grammar permits unlimited grouping, which creates no semantic node. Checking a bound
  after recursive descent would still overflow the call stack. Explicit operator/value/delimiter stacks
  and a flat arena parse/drop without input recursion; no new grammar cap is invented for parentheses.
- Nodes are reserved on encounter; the 257th refuses before unbounded prefix/call construction.
  Conditional frames count every branch and nested ordinary call, refuse level 17 and maximize sibling
  depths. Grouping extends source spans without adding nodes; square payload 2 is not a child.
- Arena edges/root are privately generated from existing nodes; read-only views bind children to that
  same arena. Three localized inspection indexing allowances rely on those construction invariants;
  the parser itself uses fallible stack/arena access. A 20736-token corpus checks no internal refusal
  and that every successful arena node belongs to its root. Customer source stays out of Debug/errors.
- 15 contracts/three privacy-lifetime docs, twelve independent reference shape/count/depth fixtures and
  eleven actual mutation assertion reds pass. Small-stack grouping at 50000 levels exercises parse/drop;
  restored strict 472 tests/WASM pass. Initial borrow-check/helper-lint errors were corrected before
  signoff; test-helper expect allowances do not relax production panic/error rules.
- The API retains number spelling/unit tokens without converting or evaluating. AST structure is not
  canonical identity or validated recipe; .5a.3 and later static/evaluation owners remain explicit.
- promotion: declined (routine syntax implementation; normative grammar/identity policy already ADR-0003).
