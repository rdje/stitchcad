# Sealed archive — whole D183 report and verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n08.3` on `2026-10-05` (UTC).

- **Sealed identity:** 22 lines, 1928 bytes, `sha256:53360bda7ac146d26310978579f85dc0917147d29aaa8c7389961b7f494b9d80`
- **Coverage:** whole new report/closure; docs/tasks/PLANNING.md interval 79252–80499, SHA-256 27e45086c3dde3620ec4b67fb0e9c3953d1211cf47aeb14e0afa21a934dd5ef4.
  Report authored over baseline 9d1830316b63fee8361644a7a729ecca4ff1a99f and absent from that commit; closure follows.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D183** (P1, immediate `.f.n08.3`): actual checked-statement annotation-guard fault1
  makes the privacy contract extract unexpected acceptance through unwrap_err544:10,
  native101/nonmacro1, target/checked-stmt-header-before.log/.json. Reproduce via
  checked-stmt-rejection-diagnostic.py header; all inputs/exact source/full7 restoration
  required before next native child. Impact: another failed body's macro can mask extraction
  in entry-wide evidence. Own comparison and prior-scope diagnostics, measured public guards,
  all22 per-body real replay/current full7 artifact/required Rust gate before closure.
  Second real fault9 selects direct-class extraction479:14, native101/nonmacro1,
  checked-stmt-comparison-before.log/.json; exact source/all inputs/full7 restoration required.
  Third real fault22 selects prior-scope extraction387:74, native101/nonmacro1,
  checked-stmt-scope-before.log/.json; exact source/all inputs/full7 restoration required.
  Root cause: privacy and direct-class tests extract rejection before asserting it; prior-source
  test extracts acceptance before checking it in two blocks. Own four preconditions across
  those three existing tests, including the second prior-metadata block with the same cause.

D183 closes at .n08.3: three real original faults1/privacy544:10,9/direct-class479:14,
22/prior-scope387:74 prove extraction before macro guards. Four acceptance/refusal preconditions
across three existing public tests now guard both prior-source blocks and both error extractions.
Exclusive22 real faults match source-derived failed sets; ordinal15 prediction correction
was independently repeated with AC/current macros/frozen72/full7 restoration0; every failed body reaches a current macro.
Frozen72 inputs/63 unchanged production Rust/23 single-link logs/current full7 artifact exact0.
Full fmt/strict Clippy/native59groups703pass0 consumed; original D177/D182 closures stay.
