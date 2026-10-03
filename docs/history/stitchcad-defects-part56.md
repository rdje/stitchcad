# Sealed archive — D134/D135 reference guidance and review status reports

Immutable historical segment, sealed by leaf `G1-SLICE.5b.3a.1` on `2026-10-03` (UTC).

- **Sealed identity:** 15 lines, 1306 bytes, `sha256:e4113b96c8a5f3d29e8fe2cc7fe2de5dfe5a9ef0c80f73086f17d22ec8664ddb`
- **Coverage:** D134/D135 reports; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D134** — actual reference parse/infer adds arc_length(angle, radius) advice to both
  angle/length quotients. Reproduction through static_signature_contract.load_reference:
  angle_value / length_value and length_value / angle_value each emit formula_dimension with
  that product advice; both multiplication orders do too (rc=0). Grammar5.1 prescribes the hint
  for angle times length; division has a different dimension and replacing it with arc_length
  changes the meaning. Root cause: reference infer's absent-pair hint checks only the two kinds,
  and the supposedly independent static matrix requires the same quotient mistake. Owner
  G1-SLICE.5b.3a.1; fix now before function metadata, assert guidance presence and absence over
  all binary operators/pairs and inject actual reference regressions. Product metadata is correct.

- **D135** — formula-static-validation annex still says product namespaces/signatures remain
  .5b.2c/.2d–.4, despite public namespace/read/scope APIs committed at0087–0089 and operator
  metadata at0090. The implementation status, API table and public contracts establish their
  completed metadata boundary; full accepted expression/graph is still pending. Owner
  G1-SLICE.5b.3a.1; correct this touched annex now with links to precise implemented APIs.
