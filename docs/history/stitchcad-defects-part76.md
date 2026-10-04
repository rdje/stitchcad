# Sealed archive — Original D158 statement serialization scope report

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.n.f2` on `2026-10-04` (UTC).

- **Sealed identity:** 8 lines, 716 bytes, `sha256:ccd3bdf5cae28cfdfec234a2651e695f9012f986258268aed13e85c5e1a88ff0`
- **Coverage:** Original D158 statement serialization scope report; exact pre-repair working report recorded before repair; no committed original was available.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D158** — statement verification/syntax text still schedules canonical serialization .3f.
  Reproduce: docs/book/src/annexes/formula-statements.md:140/167 calls serialization pending;
  public canonical_recipe.rs:107/130 exposes normalized statement/recipe canonical_form.
  Root: old 415d577f/1ac495b9 syntax-only clauses survived later normalized byte factories.
  Impact: current reader cannot tell parser proof limits from missing implemented serialization.
  Own G1-SLICE.5b.4c.h2.b.p.n.f2, P1 fix now in touched statement verification section;
  retain original working report, clarify separate normalized factory/persistence limits and
  calibrate actual publication refusal. No grammar/Rust API change.
