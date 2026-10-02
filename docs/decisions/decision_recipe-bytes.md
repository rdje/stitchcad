# Canonical statement and ordered-recipe bytes

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` technical contract; independent evidence approval **unapproved**.
- **Owner / source:** G1-SLICE.5a.3f.1a, D109; engineer acting under the director's standing
  decide-and-act delegation and governance §6.1.
- **Author / applier:** the repository engineer, the same party; no independent approval claimed.

answers: "which bytes identify bindings, assertions and whole recipes?" · "what identifies an empty recipe?"

## Context and decision

Grammar §4 already fixes (bind name kind expression) and D103 fixes all expression bytes.
D109 found no exact assertion payload or empty/whole-recipe envelope. Implementing them without
first fixing their byte contract would leave persistent identity to an implementation accident.

| Role | Exact byte template |
| --- | --- |
| Binding | (bind NAME KIND EXPR) |
| Assertion | (assert NAME TOLERANCE LEFT RIGHT) |
| Ordered recipe | (recipe STATEMENT1 STATEMENT2 ...) |
| Empty recipe | (recipe) |

Uppercase parts above are placeholders, never emitted tokens. NAME is the original valid machine
identifier. KIND is one of six declared binding kinds; TOLERANCE is one of five symbolic tolerance
names. Every EXPR/LEFT/RIGHT is the existing canonical expression byte representation.
Exactly one ASCII space separates parts; parentheses touch their first/last parts. No comments,
outer padding, line separators or terminal newline are emitted. Empty syntax, including whitespace
only, maps to (recipe). A one-statement recipe retains the recipe wrapper.

The assertion's equality is supplied by its statement role, so no extra (== LEFT RIGHT) wrapper
is emitted. This mirrors the public syntax API's two independent operands. Grouped comparisons
inside either operand remain their own expression nodes. The wrapper preserves ordered statements
without relying on newline conventions. It carries no project schema/version or wall clock.

```text
let width: length = 2.5 cm
  => (bind width length length:25000)
assert closure: eps_geo = -1 mm == -0.1 cm
  => (assert closure eps_geo (- length:1000) (- length:1000))
let width: length = 25 mm
assert closure: eps_num = width == target
  => (recipe (bind width length length:25000) (assert closure eps_num width target))
empty or whitespace-only source
  => (recipe)
```

## Consequences and evidence boundary

Names, declared kinds, symbolic tolerance classes and operand/statement order remain identity-bearing.
Aliases/grouping/source whitespace and spans remain outside bytes. No sorting, duplicate merging,
sign folding, annotation inference, unknown-name/type validation, tolerance-value resolution or
numerical execution occurs. Literal conversion is required before product serialization: all input
refusals still abort normalization with exact available statement/operand/source context.

These are separate typed expression/statement/recipe identity domains. Canonical text alone is not
an untyped discriminator: bind/recipe are also permitted ordinary call names in expression syntax.
G1-SLICE.7 must frame typed project fields and digest domains before storage or hashing; this record
does not define a universal text reader, project schema or digest namespace.

Independently authored statement and whole-recipe byte fixtures are checked by the actual recursive
book-reference parser/renderer, with dimensional inference/evaluation trapped. Whole-recipe chunks
are independently authored and checked for complete ordered token coverage; this is not an
independent complete-recipe parser. Actual renderer faults must fail authored byte assertions and
restore exact source. The structural suite watches this producer and its fault anchors.
These controls establish the technical specification, not compiled product serialization proof.
Product normalized statements/recipes are implemented by .1b and their owned serializer by .1c;
compiled byte/fault controls and completed coupled review .3f.2 are recorded in the recipe-input annex.
Static validation, numerical evaluation and typed persistence remain separate obligations.

## Re-open condition

The director may replace these spellings after reviewing a concrete alternative and its identity/
compatibility consequences. Before any persisted product format exists, replace this contract and
fixtures together; after one exists, G1-SLICE.7 owns schema/version/migration compatibility first.
Independent evidence approval belongs to the reviewer named by governance §2, never this author.
