# Sealed archive — D119/D123 whole-recipe reference checks

Immutable historical segment, sealed by leaf `G1-SLICE.5b.1b.2` on `2026-10-02`.

- **Sealed identity:** 13 lines, 1132 bytes, `sha256:7c163c27e36ee85410c17895694e7fc54d6800f2f9e5d69448ff6accba2fdd95`
- **Coverage:** D119, D123; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D119** — reference lacks whole-recipe static preflight before evaluating earlier statements.
  - Reproduce: actual EV has no preflight method; book L2 loop calls statement/evaluate while
    discovering each later row's static errors. Baseline method census prints False, exit0.
  - Root: statement mixes inference and evaluation; no complete ordered static recipe stage.
  - Impact: reference cannot prove contract5.1/9's no-statement-execution after a late static error.
  - Owner: G1-SLICE.5b.1b.2, next after clean namespace/header repair; before product static graph.

- **D123** — L8 measures a recipe by adding unrelated refusal cases.
  - Root: len(bind_rows)+len(asserts)+len(refusals) combines21 worked statements and13 separate
    candidates, including bare expressions, into34 recipe statements. No single recipe has34.
  - Reproduce: copied book max_recipe_statements25; actual preflight accepts21 but L8 refuses34.
  - Impact: false structural refusal and misleading measured recipe size. Owner: G1-SLICE.5b.1b.2;
    repair from actual accepted plan and independently falsify with the old aggregate formula.
