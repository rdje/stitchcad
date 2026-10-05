# Sealed archive — whole D179 report and verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n08.4` on `2026-10-05` (UTC).

- **Sealed identity:** 22 lines, 1860 bytes, `sha256:5f29587e6336ab50c0e13d283c26271c553b13c387a90b10214c532c9c545fe7`
- **Coverage:** whole report/closure; docs/tasks/PLANNING.md interval 77798–79133, SHA-256 908282111a834c19ff9e13207a015088877693d324382fc59a6cc4fe4059141a.
  Report is exact cde147a62491c0673a6f5bfb15561b3ad6155422 committed text; closure follows.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D179** — four further canonical/checked failed-body classifiers accept nonthread macro
  mentions. Actual AST-extracted functions accept compiler-note panicked-at fixtures at current
  canonical27:9/checked-expression194:5/checked-statement71:5/checked-recipe69:5 macro locations,
  rc=0, target/canonical-thread-diagnostic.json; no entry imports/source effects/native children.
  Root cause: unanchored panic regex, same boundary as the bounded four-entry D178 closure.
  Impact: false acceptance of mutation evidence. P1, owned immediately by
  G1-SLICE.5b.4c.h2.b.p.o.f.n08.1–4; require current thread macro locations, independent nonthread
  controls/legacy-regex faults and exclusive original real cases/full current artifact before closure.
  Canonical .n08.1 fixed: independent nonthread/legacy-regex controls and19+3 actual macro faults/
  authored failed sets/26 frozen inputs/final full9 artifact0. Checked .n08.4 remains immediate P1.
  Checked expression .n08.2 fixed:16 real faults/every failed body macro/72 frozen inputs/
  full7 artifact/full Rust703 exact0. Checked recipe P1 remains .n08.4 next.
  Checked statement .n08.3 fixed:22 real faults/every failed body macro/72 frozen inputs/
  current full7 artifact/four public preconditions/full Rust703 exact0. Recipe .n08.4 P1 next.
  No broader count claimed.

D179 closes at .n08.4: all four bounded canonical/checked classifiers require a current
thread macro and independently refuse nonthread mentions/legacy regex faults. Real cases
canonical19+3/expression16/statement22/recipe19 and full9/7/7/7 artifacts were consumed
with exact frozen inputs/current per-body macros before each next unit. Recipe retains its
baseline/count2 replacements/full7 restoration; shared546/35/36/204 and full703 gate0.
No broader classifier population is claimed; earlier D178 bounded closure stays.
