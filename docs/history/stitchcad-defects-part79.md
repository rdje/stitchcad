# Sealed archive — Complete original D160 assertion classifier report

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.n.f5` on `2026-10-04` (UTC).

- **Sealed identity:** 6 lines, 576 bytes, `sha256:0f860a0a54bccf9d46962447b2c8c5510e912af79c52d329f192636a21cac96d`
- **Coverage:** Complete original D160 report; exact cf1f1e0 HEAD byte interval.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D160** — canonical-expression native classifier accepts expect-only failure when a passing
  assertion_name occurs elsewhere. Compile its actual Assert predicate against independent noise:
  target/d160-classifier-baseline.log, rc=0; no compiler or source writes. Root: whole-output
  substring at canonical_expression_mutations.py:67 (blame8ed28930). Impact: false assertion proof.
  Own G1-SLICE.5b.4c.h2.b.p.n.f5, P1 immediately after .n.f4; failed-body classifier, actual broad
  fault/noise controls and exclusive default/coupled native restoration before closure.
