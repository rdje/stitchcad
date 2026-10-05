# Sealed archive — whole D187 report and verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n09.2` on `2026-10-05` (UTC).

- **Sealed identity:** 17 lines, 1440 bytes, `sha256:3d61566a426b165ac4824c9ba65b6e98cbe7c58ee6cb5f85acb65ca5bbf44131`
- **Coverage:** whole report/closure; docs/tasks/PLANNING.md interval 79603–80344, SHA-256 7228f0becd71364624053bd1397b9c2efeb19360409d42b26a04f9b08cd84a69.
  Report authored over baseline 19c7c93d433bd640bfa679382c64e5c8f4f34cf4 and absent from that commit; closure follows.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D187** — ordered-name public contracts extract advance_metadata errors inside assertion
  arguments before any current macro guards acceptance. Actual original faults1,2,3,10 leave
  seven nonmacro panic bodies: reserved test222:47 (1/3/10), input/geometry312:43 (1/3),
  repeated-let254:39 (2) and255:39 (10). All18 authored failing sets match, so an any-macro
  classifier can hide these bodies. Independent per-body proof refused before signoff.
  Both source files/all72 inputs/current full8 ca5b93b2 artifact restored/executed0 before edits.
  Own G1-SLICE.5b.4c.h2.b.p.o.f.n09.2, P1 now: guard every true advance's Result before
  extraction, preserve tests/cases/argv, replay all18/every body and full Rust gate before closure.

D187 closes at .n09.2: original eighteen failures kept exact authored sets but seven
bodies from faults1/2/3/10 extracted Err before a macro. Original nineteen whole logs and
full8 restoration were copied/verified before edits. One advanced helper asserts Result
acceptance before extraction at fourteen existing true-advance calls; eight tests, original
eighteen CASES/two source bindings/full Cargo argv remain exact. Fresh exclusive eighteen
real faults match every unchanged authored set; every failed body reaches a current macro.
Both sources/72 inputs/63 production Rust HEAD/19 local logs/current full8 artifact exact0.
Full fmt/strict Clippy/native59 groups703 passed/0 failed/0 ignored0.
