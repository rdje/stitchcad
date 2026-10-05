# Sealed archive — whole D184 report and verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n08.4` on `2026-10-05` (UTC).

- **Sealed identity:** 14 lines, 1188 bytes, `sha256:54beb0e9c9a6ac86fb9c9f33a3aaeba1b8a5d46cbdcb1f7405d88887c58d1768`
- **Coverage:** whole report/closure; docs/tasks/PLANNING.md interval 79534–80187, SHA-256 c5c24077c1d0143ef3c0b02473b263bd1eb7f7b97db3a1faa71e9d478978c09c.
  Report authored over baseline cde147a62491c0673a6f5bfb15561b3ad6155422 and absent from that commit; closure follows.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D184** (P1, immediate `.f.n08.4`): original initial-metadata fault2 makes the whole
  dependency contract extract rejected proof449:55 instead of reaching a macro. Reproduce:
  target/checked-recipe-mode-dependency-diagnostic.py; native101/nonmacro1, before.log/.json.
  Root cause: unknown-record and alternate-record proof blocks unwrap acceptance before asserting
  it. Impact: another failed body's macro can conceal this extraction. Own two preconditions in
  the same existing test (first measured, alternate shares cause), all19 per-body replay/current
  artifact/full703 Rust gate. Source/all inputs/full7 restoration required before edits.

D184 closes at .n08.4: real original2 dependency extraction449:55/native101/nonmacro1
restored exact source/all inputs/full7 before edits. Two acceptance guards in the same
unknown/alternate-record test precede extraction; first measured, alternate shares cause.
Exclusive19 real faults match source-derived failed sets/every failed body current macro.
Frozen72 inputs/63 production Rust HEAD/21 single-link baseline-fault-restored logs/current
full7 artifact exact0. Full fmt/strict Clippy/native59groups703passed/0failed/0ignored0.
