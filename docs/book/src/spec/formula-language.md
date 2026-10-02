# The formula language

> **Status:** normative specification, gate **G0** (roadmap ADR-0003 and §11's G0 exit clause:
> "ADR-0003 (drafting paradigm + formula language v1)"). `sc-core` currently implements
> [borrowed lexing and bounded expression syntax](../annexes/formula-syntax.md) at G1-SLICE.5a.1/.2.
> [Owned canonical expression identity](../annexes/formula-literals.md#serialize-canonical-expression-identity)
> is implemented and reviewed; [ordered statement syntax](../annexes/formula-statements.md#parse-an-ordered-recipe) is also available.
> Statement identity and evaluation remain G1-SLICE.5 work; final acceptance
> makes every worked example a product evaluation test. Terms are defined in the
> [glossary](glossary.md); every garment number is the [reference skirt](reference-skirt.md)'s, and
> every number's representation is the [units chapter](units-and-tolerances.md)'s.

A construction recipe is a list of statements and every statement is one formula. This chapter
defines the normative grammar, values, names, order, rounding and errors. The exact assertion and
complete-recipe identity bytes still need the contract owned by G1-SLICE.5a.3f.1 before serialization
([canonical grammar](formula-language/grammar.md#4-the-canonical-form)). Four properties are requirements, not taste:

- **Statically dimensioned.** Every name and literal has a kind and every operation declares the
  kinds it accepts, so a recipe that does not check is refused *before any value is computed*.
- **Total.** Every way an expression can fail is a named diagnostic with typed arguments (§5): no
  undefined behaviour, no NaN, no silent default, no partial geometry.
- **Deterministic.** Evaluation is exact rational arithmetic with two declared rounding points
  (§4.2), so one recipe over one input yields the same integers on every platform.
- **Uncertainty-aware.** A name whose state is `unknown` has no value, and reading it raises a
  diagnostic naming it (§5.2): the language cannot default a fact nobody knows.

The reasoning behind every rule — the candidates rejected, the trade-offs taken, the conditions
that re-open one — is in
`docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md`. This chapter states
the contract; that record states why it is that contract.

## 1. The paradigm, and the drafting system named with it

ADR-0003 makes the **construction recipe** primary: a measurement table, a formula graph and
ordered drafting operations, with geometry as the derived result. Three consequences bind this
chapter. Geometric sketch constraints are annotations, so no solver is ever called from inside an
expression and nothing iterates to a fixpoint (§6). Grading is a second instantiation path, so the
language reads the size context ([grammar §7.1](formula-language/grammar.md)) and nothing else
about sizes. Declaration order is the evaluation order (§4.1), so a recipe reads like the drafting
sequence it records and a reordering is a change of meaning.

### 1.1 The named reference drafting system

ADR-0003's second half requires one documented drafting system to ship as reference blocks, decided
at G0. **The named system is Aldrich's metric pattern cutting** — Winifred Aldrich, *Metric Pattern
Cutting for Women's Wear*, Wiley. Free drafting stays the primary authoring mode, and the reference
block set is that system's blocks transcribed into recipes. Four rules follow:

- the **method** is adopted, never the text: construction sequences and their measurement-derived
  numbers are cited per step by edition and page, and no text, table or illustration is reproduced;
- **no content of it has been read in this repository**, so every number taken from it is
  `unverified-with-owner` until the source is procured, and enters then as `read-external` (§8);
- the **reference skirt is not a transcription of it** — the fixture declares its own constants
  ([fixture §3](reference-skirt.md)), so it stays an independent subject for the G2 goldens;
- **`G3-GRADING.16` ships the blocks**, because a decision with no owner is a wish.

## 2. Values and kinds

A value has exactly one **kind**: the units chapter's dimensions (§1.3) plus the three the language
needs for geometry and conditions.

| Kind | Bound form | Holds | Bindable by `let` |
| --- | --- | --- | --- |
| `length` | i64 micrometres | a distance, a coordinate, an allowance width | yes |
| `angle` | signed i64 microdegrees, turns preserved | a direction, a sweep, a grain deviation | yes |
| `area` | i64 square micrometres | a derived product of two lengths, never an input | yes |
| `ratio` | i64 parts per 10⁶ | a scale factor, a shrinkage, an edge parameter | yes |
| `count` | i64, never negative | pieces, darts, plies, a size ordinal | yes |
| `boolean` | one of two values | the result of a comparison | yes |
| `point` | a `PointRef` | a point an operation constructed | no |
| `edge` | an `EdgeRef` | an edge an operation constructed | no |

These numeric i64 forms describe **bound values**. Canonical literal nodes and exact expression
values may be wider, up to the 128-bit rational limit (§4.3); unary minus keeps its own node.
A numeric binding rounds once and refuses an integer outside its signed i64 range. Count stays
nonnegative; length/area also retain their smaller scalar domains. See the
[expert binding annex](../annexes/formula-syntax.md#reference-numeric-binding-storage-controls).
The [reference boundary review](../annexes/formula-syntax.md#complete-reference-numeric-boundary-review)
maps the four verified numeric boundaries to their independent controls. Individual literals and whole product syntax arenas have [explicit input normalization](../annexes/formula-literals.md).
The [coupled normalization review](../annexes/formula-literals.md#coupled-normalization-review) is complete.
[Canonical expression serialization](../annexes/formula-literals.md#serialize-canonical-expression-identity)
is implemented; ordered statements also have [explicit syntax inspection](../annexes/formula-statements.md#parse-an-ordered-recipe).
Statement identity, bindings and evaluation remain pending. Reference instruments
supply no product execution claim.

Formula angle bindings preserve sign and complete turns; equality does not apply direction modulo.
For example, bound 360 degrees differs from zero and retains a full-turn sweep. Direction fields on
entities normalize under [units §1.2](units-and-tolerances.md#12-angles-fixed-point-microdegrees).
The [expert angle annex](../annexes/formula-syntax.md#reference-angle-conversion-and-direction-controls)
records the verified D84 reference contract and its proof boundary; product numeric binding is pending.

A `point` and an `edge` are values but not numbers: they may be arguments of the geometry selectors
([grammar §6.1](formula-language/grammar.md)) and nothing else, because only a drafting operation
may create geometry and only an operation can own an entity's identity (ontology §1, §3.2). No kind
converts implicitly — the only kind changes are the ones the operator and function tables declare —
so a length added to an angle is refused at check time, not discovered in a fitted garment.

## 3. Names

One flat namespace per recipe. Every name has one **origin**, which decides where its value comes
from and what happens when there is none:

| Origin | Supplied by | If there is no value |
| --- | --- | --- |
| `measurement` | the `MeasurementTable` (ontology §2.1) | `formula_unknown` — a body fact nobody measured |
| `ease` | the `Ease` set (ontology §2.2) | `formula_unknown` |
| `parameter` | the design's parameters (ontology §3.1) | `formula_unknown` |
| `profile` | the Factory Profile, at instantiation | `formula_unknown`; the policy matrix decides what it blocks |
| `material` | the piece's `Material` (ontology §6) | `formula_unknown` |
| `geometry` | a prior operation's `PointRef` or `EdgeRef` | `formula_unbound_name` |
| `recipe` | a `let` statement above this one | `formula_unbound_name` |
| `size` | the size context ([grammar §7.1](formula-language/grammar.md)) | `formula_unbound_name` — no size, no instance |
| `tolerance` | a reserved name (§3.1) | `formula_tolerance_unbound` |

Three rules make the namespace safe rather than convenient. A name is bound **once** per recipe — a
second `let` for it is `formula_rebinding` — because a recipe is a history and a name that changes
meaning halfway through makes every later statement ambiguous. Two origins binding one name is
`formula_ambiguous_name` at check time, never a shadow: a profile parameter that quietly overrides a
measurement is how a factory gets a pattern nobody authored. And there are **no text values**, so a
name cannot hold a label, a note or a material's name — prose is presentation
([grammar §3](formula-language/grammar.md)) or data on the object (ontology §4.1).

### 3.1 Reserved names

| Name | Kind | Value | Bound where |
| --- | --- | --- | --- |
| `eps_num` | length | 1 µm — the T1 class | always |
| `eps_geo` | length | 10 µm — the T2 internal class | always |
| `eps_fmt` | length | the target format's quantum (T3) | an export context only |
| `eps_imp` | length | the receiver's declared tolerance (T4) | an export context only |
| `eps_phys` | length | the factory's own tolerance (T5) | a profile that supplies one |
| `size_index` | count | the ordinal in the size set's authored order, from 1 | a size context |
| `size_count` | count | how many sizes the set holds | a size context |
| `is_base_size` | boolean | whether this instance is the base size | a size context |

No origin may re-bind a reserved name. T2's chordal sibling (100 µm for polyline-only targets) is a
property of the export target, resolved at serialization, and is not a formula name. `eps_fmt`,
`eps_imp` and `eps_phys` are `unknown` inside a recipe: the class exists, the number does not yet.

## 4. Evaluation, rounding, determinism

### 4.1 Order

Evaluation is a **single pass in declaration order**. A statement sees the names bound above it and
the always-visible origins (§3); a forward reference is `formula_unbound_name` and is never resolved
by reordering, because reordering would make a recipe's meaning depend on its evaluator. The formula
graph of ontology §3.1 is the dependency relation these statements induce and declaration order is a
topological order of it, so a cycle is unrepresentable: a persisted recipe containing one is corrupt
or hand-edited, and the loader refuses it with `formula_cycle`. Re-evaluation runs from statement 1
(recipe replay, ontology §3.1); an implementation may cache, the cache must be invisible, and a
cached result that differs from a fresh one is a defect.

### 4.2 Exact arithmetic, and the two places a value rounds

Literal input first converts once into its canonical internal integer, by
[grammar §2](formula-language/grammar.md). That input conversion precedes expression arithmetic:
two literals each below half a quantum are two canonical zeros. The rounding points below concern
operations on those canonical inputs; they do not retain hidden fractions in an integer literal.

Arithmetic is **exact rational arithmetic** — `+` `-` `*` `/` never round, and an implementation uses
arbitrary-precision integers for numerator and denominator. The contract is the value, not the
representation: a numerator or denominator past `max_rational_bits` (§4.3) is `formula_domain`, not a
hang. The signed length/area domains in units §1.1 and the nonnegative count domain (§2) apply to
every completed numeric expression result, using its exact internal-unit value before binding
rounding. A later cancellation or rounding cannot rescue an out-of-domain intermediate. Canonical
literal inputs are checked after their one input conversion/rounding; their exact pre-round value
still obeys the reduced-rational width bound. An exact count fraction may remain nonnegative until
binding rounds it to an integer; unary count negation remains a dimensional refusal (grammar §5).
The geometry-only piece-box limit needs geometry context and is not a smaller scalar length bound.
Implicit rounding has two boundaries:

- **an irrational call** — `sqrt`, `hypot`, the trigonometric functions and `arc_length` return the
  nearest internal quantum to the true value, ties away from zero. They are specified by that
  *result*, not by an algorithm, so an implementation may not use a different libm and get a
  different answer;
- **a binding** — a numeric `let` rounds once by the units §2 rule, checks its signed i64 storage
  range and scalar domain, and exposes that stored integer to subsequent statements. An out-of-range
  integer is `formula_domain`, naming the binding, kind, bounds and rounded measured value.
  Boolean bindings keep their boolean kind.

The explicit `round_to` function is an authored quantization operation: its step is part of the
recipe ([grammar §6](formula-language/grammar.md)). Ordinary rational operators do not add that
operation implicitly. An expression with no irrational or explicit rounding call therefore computes
exactly and rounds once at binding. A value produced
by one is the nearest representable approximation and not a value identical by construction, which
is why a comparison involving one names **T2 or looser** and never T1 (units §3).

### 4.3 Structural limits

Declared constants of the language, so a pathological or corrupt recipe is refused instead of
exhausting memory. Exceeding one is `formula_domain`, naming the limit and the measured size.

| Limit | Value | Bounds |
| --- | --- | --- |
| `max_expression_nodes` | 256 | the nodes in one expression |
| `max_recipe_statements` | 4096 | the statements in one recipe |
| `max_if_depth` | 16 | nested conditionals |
| `max_rational_bits` | 128 | the bits of a numerator or denominator |

`max_expression_nodes` is derived, not a round number: the census §8 names measures the largest
expression this book's examples carry and refuses a limit that does not exceed it by the margin the
decision record states. A recipe that reaches a limit is a design smell — a block that should be
several operations — and raising a limit is a recorded decision, never a way to land a recipe.

### 4.4 Determinism

One recipe over one input (measurement table, parameters, profile, size context) yields the same
bound values and the same geometry on every platform, byte-identically after canonicalization
(units §7). Nothing outside the input enters: no wall-clock value, no randomness, no environment, no
locale, no float in canonical content.

## 5. Errors

### 5.1 When each check runs

**Static — before any value is computed:** `formula_parse`, `formula_dimension`,
`formula_unbound_name`, `formula_ambiguous_name`, `formula_rebinding`, `formula_unsupported`, and
the node, statement and conditional limits of §4.3.

**Runtime — while evaluating:** reduced numeric-value widths (§4.3), `formula_division`, `formula_domain`, `formula_unknown`,
`formula_tolerance_unbound`, `formula_assertion`, and `formula_cycle` at load.

A static refusal rejects the recipe whole: no statement is evaluated and no geometry is produced,
which is the property the [envelope](feature-matrix.md) §10 requires of every refusal. A runtime
failure abandons the evaluation the same way and never resumes with a substituted value.

### 5.2 The diagnostic set

A diagnostic carries the statement index and canonical expression where available, together with
the clause or source rule it broke. A parsing failure retains its exact offending span and typed
rule. Whole-source ASCII preflight precedes identified statement boundaries and therefore supplies
no guessed index; malformed or non-normalizable syntax has no accepted canonical expression.
Do not fabricate that context. The [syntax evidence map](../annexes/formula-statements.md#coupled-syntax-and-diagnostic-review) distinguishes available context from later semantic/command arguments.
A token is stable, is localized as the [internationalization chapter](i18n-architecture.md) §3
specifies, and is never rendered raw to a user.

| Token | Raised when | Required arguments |
| --- | --- | --- |
| `formula_parse` | the surface form is not in [grammar §1](formula-language/grammar.md) | the offending span, its position, the rule |
| `formula_dimension` | an operation has no rule for the kinds it was given | the operator or function, every operand's kind, the kind wanted |
| `formula_unbound_name` | a name nothing visible declares | the name, the origins searched |
| `formula_ambiguous_name` | two origins bind one name | the name, both origins |
| `formula_rebinding` | a `let` re-binds a name this recipe already bound | the name, both statement indices |
| `formula_division` | a divisor is zero | the expression, the divisor's name or literal |
| `formula_domain` | a value is outside its declared domain (units §1.1) or a §4.3 limit | the quantity, the operation, the bound, the measured size |
| `formula_unknown` | an operand's state is `unknown`, so it has no value | the name, its state, its origin, the artifact the policy matrix blocks |
| `formula_tolerance_unbound` | a context-supplied tolerance is read where no context supplies it | the name, the context it was read in |
| `formula_assertion` | an `assert` does not hold at its class | the assertion's name, both values, the tolerance class |
| `formula_cycle` | a persisted recipe's graph is cyclic | the names in the cycle |
| `formula_unsupported` | a construct §6 excludes and the envelope does not own | the construct, the exclusion clause |

### 5.3 Precedence with the envelope's diagnostics

Where the refused construct is a *feature* the [envelope](feature-matrix.md) dispositions, the
envelope's token is raised and not a formula token: the matrix is the boundary of the release claim,
and two token sets must not compete for one refusal.

| Construct asked for | Raised | Owned by |
| --- | --- | --- |
| `nurbs`, `spline`, `bspline` | `env_nurbs` | the matrix §10 |
| `solve`, `constraint`, `fixpoint` | `env_sketch_constraints` | the matrix §10 |

## 6. Exclusions

| Excluded | Why | Diagnostic |
| --- | --- | --- |
| NURBS-class curves and expressions | the curve set is three primitives (units §4) | `env_nurbs` |
| implicit solving inside an expression | the CSP is a separate engine (roadmap §6.2, ADR-0001) | `env_sketch_constraints` |
| loops, iteration, recursion | a recipe is a bounded ordered list; repetition is an operation list | `formula_unsupported` |
| user-defined functions and macros | a `let` chain covers the need; a definition needs a scope rule and an answer on recursion that v1 does not have | `formula_unsupported` |
| text values, a size label in a formula | a label is prose ([size sets](size-sets.md) §3) | `formula_parse` |
| a formula that creates a point or an edge | geometry is an operation's, with its identity (§2) | `formula_dimension` |
| an exponent other than 2 | an area is the only product the model stores (units §1.3) | `formula_unsupported` |
| an area literal, a radian literal | an area is derived; π is irrational ([grammar §2](formula-language/grammar.md)) | `formula_parse` |
| exponent notation, a float literal | no float enters canonical content (units §7) | `formula_parse` |
| a comment inside an expression | it would not survive canonicalization ([grammar §4](formula-language/grammar.md)) | `formula_parse` |

### 6.1 Named v2 candidates

Promised nowhere. Each is a domain decision, and each arrives with a worked example over a real
garment or it does not arrive:

| Candidate | What it must earn first |
| --- | --- |
| `floor_to` | a reason a recipe must round down, and the seam it protects |
| `ceil_to` | the same, rounding up |
| `case` | a branch `if` cannot express without nesting past `max_if_depth` |
| `translate` | a point moved by a length in a direction, which §2 refuses today |
| `fn` | a user-defined function: a scope rule, and an answer on recursion |

## 7. The parts of this chapter

The language is one contract in three files, because a specification whose tables, prose and
examples share one file outgrows the size this book's chapters are held to. Every part is listed
here, and the census §8 names refuses a part that is not:

| Part | What it settles |
| --- | --- |
| this file | the contract: the paradigm, kinds, names, evaluation, errors, exclusions |
| [Grammar, operators and functions](formula-language/grammar.md) | the syntax: grammar, literals and units, the display and canonical forms, operators, functions, selectors, conditionals |
| [Worked examples](formula-language/examples.md) | every rule executed over the reference skirt: bindings, assertions, refusals |

## 8. Verification status of the claims in this chapter

- **The grammar, the kind rules, the operator and function tables, the diagnostics, the limits and
  the canonical form are project decisions**, sourced by this chapter and reasoned in
  `docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md`.
- **Every value in the [worked examples](formula-language/examples.md) is `derived`** from the
  fixture's declared tables and re-derived by a tracked producer rather than by reading:
  `docs/tasks/artifacts/formula_language/run_formula_language_census.sh` evaluates each row with an
  evaluator written from this chapter's rules, over the constants the fixture chapter declares, and
  refuses a row whose value, kind or diagnostic disagrees. `run_formula_language_probes.sh` is its
  ground truth.
- **`dart_intake_max` = 5.0 cm is `assumed`.** It is a plausible single-dart maximum and it makes the
  examples' `dart_count` agree with the fixture's one dart per quadrant, but it is a sewing
  judgement. The seat that owes it is the sewing/factory domain expert, which is **vacant** and not
  acting ([governance §8.1](../governance.md)), so no signature this repository can produce confirms
  it; the population waiting on that seat is derived by
  `docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh`.
- **`shrinkage` is `unknown` by construction** — the example exists to show the language refusing to
  invent it. Whether a shrinkage default may exist as a `preference` with provenance (ontology §5) is
  the same vacant domain expert's, with the artifact policy matrix (G4) deciding what it blocks.
- **The named drafting system of §1.1 is `unverified-with-owner`.** Its bibliographic record is
  `read-external` — OpenLibrary's work record OL16995319W, read `2026-09-30`: six editions, Wiley and
  Blackwell, 256 pages in the 2015 edition, ISBN 9781119028284 — and its content has not been read
  here. The procurement owner seat ([governance §8.1](../governance.md), held acting) owes the source,
  the vacant domain expert seat owes the review of the transcription, and `G3-GRADING.16` owes the
  blocks. Whether shipping drafting data derived from a commercial source needs a licence clearance is
  an open question for the project owner's seat, flagged and not resolved.
- **The internal unit, the rounding rule, the tolerance classes and the declared domain are the units
  chapter's**, cited where they bind. This chapter restates their consequences for expressions and
  owns nothing they own.

## 9. What must be true in tests

- Every binding in the [examples](formula-language/examples.md) §2 evaluates to its published value,
  on two platforms, byte-identically (G1).
- Every refusal in examples §4 raises exactly the named token with its required arguments and
  produces no geometry; every assertion in examples §3 holds, and a falsified one raises
  `formula_assertion`.
- Every example name the fixture chapter also derives agrees with the fixture's published value —
  the two chapters describe one garment, which is the D27 defect class, checked and not read.
- Kind checks are static: a recipe with a kind error evaluates no statement at all.
- Canonicalization: `2.5 cm` and `25 mm` produce one node, a respelling does not change a formula's
  identity, and a canonical file contains no float.
- An `unknown` in an untaken branch does not block; one in a taken branch does.
- A tolerance comparison names its class, and comparing two values of which one came from an
  irrational call is refused at T1.
- The structural limits refuse a pathological recipe rather than exhausting memory. Current
  [reference node/depth controls](../annexes/formula-syntax.md#reference-structural-limit-controls)
  have explicit instrument scope; [product syntax controls](../annexes/formula-syntax.md#product-structural-bounds-and-refusal-scope)
  check parser bounds independently. The largest
  expression in the book's examples stays inside `max_expression_nodes` by the margin §4.3 claims.
- Every display operator this book uses has a machine form in [grammar
  §3](formula-language/grammar.md), every part of this chapter is listed in §7, and every operator,
  function, reserved name and diagnostic token its chapters use is declared by one of its tables —
  all derived by the census §8 names, so a later chapter cannot invent vocabulary silently.
