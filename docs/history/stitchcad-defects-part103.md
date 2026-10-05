# Sealed archive — whole D182 report and verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n08.2` on `2026-10-05` (UTC).

- **Sealed identity:** 17 lines, 1518 bytes, `sha256:7fd3c611d19d65379a5e63a7443a653970a9afeb597216eb26ec8bf030cf511e`
- **Coverage:** whole newly authored report/closure; docs/tasks/PLANNING.md byte interval 79469–80509, SHA-256 ca8f6f50b4d140c2d95a792f4e81ef97c948df23bd56993dc589d520141c2008.
  Report authored over baseline a041a682acc50c08913000db10c7682e6c07f500 and absent from that commit; closure follows.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D182** (P1, immediate `.f.n08.2`): actual result-kind fault2 makes the dependency public
  test reject through unguarded extraction533:44 instead of an assertion macro. Reproduce:
  target/checked-mode-dependency-diagnostic.py selects that test with the original real fault2;
  actual native101/nonmacro1, exact checked source/all inputs/full7 restoration0.
  Impact: an entry-wide macro from another failed test can hide this body's extraction panic.
  Own focused bounded-conditional case6 diagnostic next, then measured public preconditions/full16
  per-body replay/current artifact/full required Rust gate. Original D177 bounded closure stays.
  Second independent real case6 selects bounded conditional extraction659:52, native101/nonmacro1;
  exact all inputs/source/full7 restored0 before repair. Root cause: both tests unwrap rejected
  checked proofs before asserting acceptance. Own two explicit acceptance macro preconditions;
  after full16 real faults must qualify each failed test body, not only the entry-wide aggregate.

D182 closes at .n08.2: two measured dependency533:44/bounded659:52 extraction paths now assert
acceptance before extraction. Exclusive original16 real faults match independently authored failed
sets; every failed test body names a current macro, with no extraction-only failure. Frozen72
inputs/63 production Rust HEAD bytes/two public guards/17 local logs/full7 artifact exact0. Full
fmt/strict Clippy/native59groups703pass0 gate consumed. D177 bounded earlier closure stays.
