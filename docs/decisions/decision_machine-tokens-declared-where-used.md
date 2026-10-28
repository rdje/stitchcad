# A machine token is declared where it is used, and owned by exactly one glossary entry

- **Type:** `decision`
- **Date:** `2026-09-29` (absolute)
- **Status:** `active`
- **Owner / source:** leaves `G0-CONTRACT.13b` and `G0-CONTRACT.1`; established by measuring the
  reference fixture's own token population (defects **D26** and **D27** in `docs/tasks/PLANNING.md`)

answers: "what counts as a machine token?" · "where must a token be declared?" · "why does a specification table lead with a token column?" · "how do I know a chapter's formulas are checkable?" · "what does the `→` in a glossary token cell mean?" · "why is a token never shown to a user?"

## The fact / decision

1. **A machine token is the identifier a concept carries in the API, in serialized files, in formulas
   and in diagnostics** — `dart_intake`, `SeamAllowance`, `seam_allowance_policy`, `Micrometre`. It is
   the half of a term that a machine reads; the glossary's term column is the half a human reads.
2. **A token is never rendered raw to a human.** A user sees the localized term. A token appearing in a
   dialog, a printed label or a tech pack is a defect, and the externalization lint `G0-CONTRACT.16`
   owns is what catches it.
3. **One token, one meaning.** Where several terms share an identifier — `button`, `buttonhole`, `hook
   and bar` and `zipper` are all a `Closure` — exactly one glossary entry *owns* the token and the
   others cross-reference it with `→`. Two owners would be two meanings.
4. **A chapter declares every token it uses, in a table whose first cell names it.** A token that
   appears only inside a formula or a sentence is undeclared, and that is a defect whether or not a
   reader can guess what it meant. Fixture-local tokens are declared by the fixture; platform
   vocabulary is declared by the glossary.
5. **Tokens are ASCII and locale-independent**: value tokens `snake_case`, object tokens the `CamelCase`
   type name, enumerated values `snake_case` or the code the target format itself defines (`V`,
   `castle`, `CUT`). A translated field name or a decimal comma never changes what a file means.
6. **Where code exists, the token is quoted from code, not invented.** The glossary's unit entries carry
   `Micrometre`, `MICRODEGREES_PER_DEGREE`, `UnitError::DomainExceeded` and the six `ToleranceClass`
   variants because those are the identifiers `crates/sc-units` actually declares. A glossary that
   invents a prettier token than the code is a second, drifting source of truth.
7. **The claims are derived, not asserted**: `bash docs/tasks/artifacts/glossary/run_glossary_census.sh`
   re-measures ownership, uniqueness, shape, declaration and every canonical reference.

## Why

Measured on the reference fixture, the one garment every G2 golden, mutation test and agent gate is
built around:

| | tokens used | declared by a table of the chapter | used and undeclared |
| --- | --- | --- | --- |
| before `G0-CONTRACT.13b` | `42` | `23` | `19` |
| after | `52` | `46` | `6` — all glossary vocabulary (`SeamAllowance`, `assumed`, `close`, `known`, `semi`, `walk`) |

Nineteen names the chapter's own formulas used were declared nowhere: §4 named its 17 derived values in
prose ("quarter hip", "front dart centre") while §5's recipe and §4's formulas referred to them as
`quarter_hip` and `front_dart_centre`, and `ease_waist`, `ease_hip`, `sa_cb`, `sa_waist` and
`sa_wb_bottom` appeared inside formulas with no declaring table at all.

That is not a cosmetic gap. An undeclared token has no defined referent, so:

- a reader cannot tell a deliberate name from a typo;
- no instrument can check the arithmetic the token appears in, because nothing says what it holds;
- the value cannot be localized, because nothing binds it to a human term;
- and the chapter cannot be turned into a recipe an implementation evaluates, which is the entire
  premise of a construction-recipe CAD (roadmap ADR-0003).

Forcing every token to a declaration is also what exposed **D27**: `sa_wb_bottom` had no declaring row
because the waistband it belongs to is described two incompatible ways in the same chapter — §4's cut
width is a single band folded lengthwise, §6's piece list is a faced two-piece band. A prose-only
fixture hides that; a fixture that must name every value cannot.

## How to apply

- **Writing a spec chapter:** any table that names values leads with the token. Formulas are written
  over tokens only — no prose word inside a formula — so the formula is checkable by machine.
- **Adding a term to the glossary:** decide whether it *owns* a token or cross-references one (`→`).
  If two entries would own the same token, one of them is not a distinct concept, or the token is wrong.
- **Adding a chapter later:** run the census. Its `C1` rule names every token the spec set uses that
  nothing declares, and its `T2` rule names every token two entries claim.
- **Reviewing a defect in this family:** the question is never "can a reader guess it?" but "which table
  declares it?". If the answer is none, the fix is a declaration, not a clearer sentence.

Related: [[decision_numerical-contract-fixed-point]] · [[decision_product-work-takes-the-frontier]] ·
`docs/book/src/spec/glossary.md` (the machine-token rule as specification) ·
`docs/tasks/artifacts/glossary/run_glossary_census.sh` (the producer) ·
`docs/tasks/PLANNING.md` (defects D26, D27, D28).

## Current API declaration repair0479

answers: "does a documented public API name still need a chapter-local declaration?"

Yes. Current D250 census derives140 token/use pairs across13 chapters,not140 unique defects.
Five command-family chapters now declare their actual public input/authority types and variant/method
meanings. No API spelling,behavior or census exemption changes. Exact delta resolves43 rows with0 new;
97 remain with later owners. Five copied actual declaration removals are refused by name,showing
that these are live declarations rather than an allow-list. Full glossary acceptance remains pending.

## Bounded declaration placement0480

answers: "how do API declarations fit a chapter already at its maintained size ceiling?"

Move a complete coherent contract to a linked bounded chapter,retain old anchors and update actual
navigation/API-status owners. The axis/member body is byte-equal to Git source except one navigation
sentence;no policy ceiling rises. Size-set local API/example declarations resolve65 more C1 rows,
97→32 with0 new;two actual copied declaration removals refuse by name. Execution remains unchanged.

## Formula declaration meaning0481

answers: "does declaring a formula value API certify whole expression execution?"

No. Quote actual static proof/root,canonical scalar/stored binding and runtime value/source/operator/
comparison/assertion/conditional interfaces while retaining their distinct execution boundaries.
Three annexes resolve25 more rows,32→7 with0 new. A bare type name in a table is still not a token
column declaration;the existing bound-value row now quotes its actual name. Rust/API behavior stays unchanged.

## Fixture-local strings0482

answers: "how are short accounting fixture names declared without inventing product defaults?"

Declare actual authored text by its fixture meaning:body/garment/Ease binding tokens,axis name,
coordinate label and parameter tokens. Quote exact native spellings;do not rename them or exempt a
whole family. The last seven rows clear,140→0 overall,and the unchanged census passes every rule.
Advisory investigation/full glossary probes remain separate acceptance,not inferred from a green census.
