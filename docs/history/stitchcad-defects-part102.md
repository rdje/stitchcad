# Sealed archive — whole D181 report and verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n08.2` on `2026-10-05` (UTC).

- **Sealed identity:** 10 lines, 876 bytes, `sha256:5185a95f2314153f6402bededcf1272dfe17f164f01bc69be287e6a13e89f666`
- **Coverage:** whole newly authored report/closure; docs/tasks/PLANNING.md byte interval 78884–79468, SHA-256 435a6646e9c1149e7dfee5a403e1aba48bc2e37a48753fe953bfcd264b17fc29.
  Report authored over baseline a041a682acc50c08913000db10c7682e6c07f500 and absent from that commit; closure follows.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D181** (P2, owned immediate `.f.n08.2`): grammar6 still says statement/whole validation pending
  although current-statement and complete recipe kind proofs are implemented and verified.
  Reproduce: grammar.md section6 availability footer versus public check_kinds in namespace/ordered
  and checked_recipe and the current public recipe contracts. Impact: mdBook underreports available
  engine behavior. Correct availability prose only; retain formal grammar/diagnostic tokens, verify
  scoped publication/public API evidence, then seal whole report and closure before signoff.

D181 closes at .n08.2: grammar6 availability prose now links the implemented current-statement
and complete recipe kind proofs. Exact introduction c13b7d8/implementations2fa9d5d/e3794738 and
public checked APIs establish scope; publication45 passes0. Formal grammar/tokens/tables unchanged.
