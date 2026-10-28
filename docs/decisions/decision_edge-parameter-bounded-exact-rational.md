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

## Formula workspace implementation — 2026-10-25

Leaf `G1-SLICE.5c.1.2.2` implements formula sums/differences with private num-bigint 0.4.6
intermediates, pinned in Cargo.toml/Cargo.lock. Published cached package metadata and offline Cargo
resolution identify MIT OR Apache-2.0 for num-bigint, num-integer and num-traits; autocfg is
Apache-2.0 OR MIT. The package's [upstream license](https://github.com/rust-num/num-bigint)
also offers MIT/Apache-2.0. No optional random/serialization feature is required.

FormulaRational retains private canonical u128 magnitudes and an independent sign. Construction,
products and quotients need no allocation; sums form signed cross-products in the bounded private
workspace. Two admitted 128-bit components give at most a 257-bit raw numerator and 256-bit positive
denominator. Euclidean reduction cannot increase either width. Narrow only the reduced components;
otherwise refuse with their exact bit count. This admits wide intermediates that cancel without
inventing a custom wide-integer kernel. The primitive exposes no dependency type and changes no
stored edge parameter, quantity domain or binding rule. The focused native target passes, including34 independent
Fraction oracle rows in `.5c.1.3`; broader/platform integration remains `.5c.4`/pre-push.

## Exact interval partition0485

answers: "must splitting a held interval rescale its source parameters or allocate new topology?"

EdgeRange::split_at copies the original edge and exact bounds into two positive intervals meeting
at a strict interior cut. It needs only exact ordering,so full-width admitted parameters require no
arithmetic recomputation or new identity. RangeSplitError retains the original range/cut. Native
coverage uses exact membership samples and the existing later-edit interval resolver,including
unknown-source repairs. This primitive supplies interval evidence;actual paired sewing/copy/ease/
stop/lineage and transaction obligations remain G1-SLICE.6.5.3.2. Four new cases plus13 existing
range tests and one compiled example pass;it grants no geometric or physical certificate.
