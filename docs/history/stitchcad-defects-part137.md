# Sealed archive — whole D70 axes-cardinality conflict

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.b.2.a` on `2026-10-26` (UTC).

- **Sealed identity:** 12 lines, 1069 bytes, sha256:97ed3eddec0f0ef62b494b3b87296f170d853bdd66064e8c9c0fd576a16635d8
- **Coverage:** whole committed source interval from e95fa7af29b543fb96d28e3bd691058011f2cbc5.
- Whole source interval: docs/tasks/PLANNING.md bytes 34744–35813, 1069 bytes, sha256:97ed3eddec0f0ef62b494b3b87296f170d853bdd66064e8c9c0fd576a16635d8
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- **D70** — SizeSet axes cardinality has conflicting normative requirements.
  - Reproduce: rg -n 'axes|single axis|two representations' docs/book/src/spec/size-sets.md reports
    .2 field row: axes optional and present only for multidimensional systems; .7: one-dimensional
    system has a single axis and the model does not carry two representations. Ownership decision
    names multidimensional axes but does not resolve this cardinality contradiction.
  - Impact: a production schema cannot decide whether an ordinary one-dimensional range requires
    an explicit axis or may omit it; the choice changes authored data, validation and agent APIs.
  - Owner/schedule: G1-SLICE.4c.2 before axes implementation; director ruling requested. Independent
    membership .4c.1 proceeds, chart/resolution contracts remain fully owned. No axis default invented.
  - Decision requested: explicit axes for every range (including one-dimensional; recommended) versus
    allowing one-dimensional ranges to omit axes, with explicit axes required for multidimensional sets.

