# The edge parameter is a bounded exact rational in sc-core, not a sc-units primitive

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `G1-SLICE.3a` — the representation of a parameterized
  reference's position (`docs/book/src/spec/ontology.md` §1, glossary `parameterized reference` → `t`).

answers: "what type is an edge parameter?" · "is the edge parameter a Ratio?" · "where does the exact rational live?" · "does sc-units get a Rational type?" · "why not reuse the formula language's rational?" · "how exact is a split or merge parameter?"

## The fact / decision

A parameterized reference's position `t` is a **bounded exact rational in `[0, 1]`** — an integer numerator
and denominator (`i64`, reduced to canonical form, `i128` intermediates) — defined in **`sc-core`'s ontology
module**. It is **not** `sc-units`' parts-per-million `Ratio`, and **not** the formula evaluator's
arbitrary-precision rational. Split, merge and reverse recompute `t` **exactly** (`t/s`, by arc length,
`1 − t`); an intermediate that does not fit `i64` after reduction is a **typed diagnostic**, never a silent
round.

## Why

1. **The spec requires an exact rational, and a fixed-point would drift.** Ontology §1: the parameter is "a
   rational in `[0, 1]` … so it survives a change of tessellation and a change of units". `sc-units::Ratio` is
   parts-per-million (glossary `ratio`), a fixed-point that rounds on construction; recomputing `t/s` in ppm
   would round on every split and merge and accumulate drift, defeating the reference stability §1.1 exists to
   guarantee. Exactness is the point.
2. **The formula language's rational is a different thing.** Formula §4.2 specifies *arbitrary-precision*
   integers for expression arithmetic, bounded by `max_rational_bits = 128` on the reduced result, and it is
   **internal to evaluation** — a `let` binding rounds to the internal integer (Length/Angle) by the one
   rounding rule. That arbitrary-precision rational needs a bigint dependency and is `G1-SLICE.5`'s concern;
   it never escapes as a stored value, so it is not the stored edge parameter.
3. **YAGNI keeps it out of the dependency-free numerical contract.** At G1 the only consumer of a stored exact
   rational is the ontology's `EdgeRef`/`PointRef`; `sc-geometry` (G2) does not exist yet. Adding a `Rational`
   to `sc-units` — and amending its normative G0 chapter — before a second consumer needs it would widen the
   crate every runtime profile depends on for a type only the ontology uses.

## How to apply

- Define the bounded exact rational in `sc-core`'s ontology module as the parameter representation, with exact
  `+ − × ÷`, reduction, total ordering, an `in_unit_interval` predicate, and overflow/division-by-zero as
  `UnitError`-shaped diagnostics (no panics, per the workspace lints).
- **Promotion trigger, named so it is not rediscovered as a surprise:** if `sc-geometry` (G2) needs the same
  `[0, 1]` parameter for curves — and it cannot depend on `sc-core`, which depends on it — **promote the type
  to `sc-units` then**, by a recorded decision that supersedes this one, and amend the units chapter at that
  point. Do not pre-emptively widen `sc-units`.
- Related: [[decision_numerical-contract-fixed-point]] (the ppm `Ratio` this is distinct from),
  [[decision_entity-identity-ulid-injected-generator]] (the identity this parameter attaches to).
