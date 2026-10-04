# The formula language

> **Status:** normative specification, gate **G0** (roadmap ADR-0003 and §11's G0 exit clause:
> "ADR-0003 (drafting paradigm + formula language v1)"). `sc-core` currently implements
> [borrowed lexing and bounded expression syntax](../annexes/formula-syntax.md) at G1-SLICE.5a.1/.2.
> [Owned canonical expression identity](../annexes/formula-literals.md#serialize-canonical-expression-identity)
> is implemented and reviewed; [ordered statement syntax](../annexes/formula-statements.md#parse-an-ordered-recipe) is also available.
> [Complete recipe input normalization and identity](../annexes/formula-recipe-inputs.md) are implemented
> and [reviewed together](../annexes/formula-recipe-inputs.md#coupled-input-and-identity-review);
> the [syntax milestone](../annexes/formula-recipe-inputs.md#syntax-milestone-and-the-route-to-execution)
> is complete; [independent static signature checks](../annexes/formula-static-validation.md)
> now review the book reference, including [namespaces and static headers](../annexes/formula-static-validation.md#names-and-single-statement-static-checking)
> and [whole-recipe preflight](../annexes/formula-static-validation.md#whole-recipe-before-execution).
> The [complete static review map](../annexes/formula-static-validation.md#complete-static-review-and-remaining-contracts)
> records remaining product/runtime contracts; D124 recognition preserves the existing grammar.
> Product [metadata and initial reads](../annexes/formula-declarations.md), plus [ordered name scopes](../annexes/formula-name-scopes.md), are available at .5b.2.
> [Source-bearing call lookup](../annexes/formula-call-lookup.md) supplies exact callee refusals at .5b.3c.2a.
> Product name/type/binding validation and evaluation remain G1-SLICE.5b–.5g work; final acceptance
> makes every worked example a product evaluation test. Terms are defined in the
> [glossary](glossary.md); every garment number is the [reference skirt](reference-skirt.md)'s, and
> every number's representation is the [units chapter](units-and-tolerances.md)'s.

A construction recipe is a list of statements and every statement is one formula. This chapter
defines the normative grammar, values, names, order, rounding and errors, including exact expression,
statement and ordered-recipe identity bytes ([canonical grammar](formula-language/grammar.md#4-the-canonical-form)).
Product statement/recipe [input normalization](../annexes/formula-recipe-inputs.md) is implemented at
G1-SLICE.5a.3f.1b; owned statement/recipe serialization is available at .1c.
Four properties are requirements, not taste:

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
Statement and whole-recipe identity are implemented and reviewed; bindings and evaluation remain
pending. Reference instruments
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

The book reference has [scoped origin/context read controls](../annexes/formula-runtime-validation.md#missing-values-by-origin),
including supplied optional contexts and named missing-value refusals. Product adapters remain pending.

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

No origin may re-bind a reserved name; the refusal is `formula_rebinding` with the reserved-case
source arguments (§5.2.1). T2's chordal sibling (100 µm for polyline-only targets) is a
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
is why a comparison involving one names **T2 or looser** and never T1 (units §3). An inadmissible
T1 class raises formula_domain, retaining the comparison, requested class and contribution sources.
Input conversion and binding rounding alone do not introduce an irrational or explicit-quantization
contribution. The [runtime controls](../annexes/formula-runtime-validation.md) distinguish retained
contributions from untaken branches and independent point coordinates.

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

The book's [reference assertion controls](../annexes/formula-runtime-validation.md#reference-runtime-assertion-controls)
verify named false-assertion diagnostics and their values/class. Product evaluation remains pending.

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
| `formula_dimension` | an operation has no rule for the kinds it was given, or a let annotation is unbindable | actual/wanted kinds for operations; case-specific header arguments (§5.2.5) |
| `formula_unbound_name` | a name is undeclared, or a call names no built-in function or selector | the name, the origins searched |
| `formula_ambiguous_name` | two origins bind one name | the name, both origins |
| `formula_rebinding` | a recipe binding repeats a name, or any origin attempts a reserved name | the name and case-specific binding sources (§5.2.1) |
| `formula_division` | a divisor is zero | the expression, the divisor's name or literal |
| `formula_domain` | a value/domain or §4.3 limit is exceeded, or a comparison uses inadmissible T1 (§4.2) | quantity/operation/bound/measured size; for T1: comparison, requested class, contribution sources |
| `formula_unknown` | an operand's state is `unknown`, so it has no value | the name, its state, its origin, the artifact the policy matrix blocks |
| `formula_tolerance_unbound` | a context-supplied tolerance is read where no context supplies it | the name, the context it was read in |
| `formula_assertion` | an `assert` does not hold at its class | the assertion's name, both values, the tolerance class |
| `formula_cycle` | a persisted recipe's graph is cyclic | the names in the cycle |
| `formula_unsupported` | a construct §6 excludes and the envelope does not own | the construct, the exclusion clause |

#### 5.2.1 Binding-refusal sources

`formula_rebinding` has two cases. A repeated recipe binding carries the name, the prior and
attempted binding sources, and both actual one-based statement indices. A reserved-name attempt
carries the name, the reserved metadata source (fixed kind, origin and required context), and
the attempted binding source/origin. If the attempt is a recipe let, retain its actual ordinal,
whole-statement span and name span. Initial authored inputs and reserved metadata have no recipe
statement index; none is invented. Source arguments retain canonical identities/references and
source locations where available, without copying numerical values or input state.

Detached statement checking has no whole-recipe ordinal or prior statement location; it retains
the available source spans and declared metadata, without claiming complete whole-recipe context.
The [reference diagnostic controls](../annexes/formula-static-validation.md#reserved-name-diagnostic-sources)
distinguish that scope from whole-recipe checking and canonical product adapters. Two authored
declarations colliding remain `formula_ambiguous_name`, including two from the same origin.

Ambiguity retains `name` and both `origins` in binding order, including two equal origins.
Its `prior_source` and `attempted_source` carry the actual kind/origin and source role.
An initial pair collision can retain each actual one-based `declaration_index` in the supplied
ordered declarations; that index is not a recipe statement index. A recipe attempt retains its
actual name/statement spans and, during whole checking, its real statement ordinal. A detached
metadata dictionary supplies no original declaration position, so none is fabricated. Canonical
product declarations retain their actual entity/record identities instead of pretending to be
reference source positions. These arguments inspect metadata only, never numerical values/state.

#### 5.2.2 Call-lookup sources

A call searches a different domain from a data-name read: first the envelope alias table (§5.3),
then the closed built-in/selector catalog. A scalar, geometry or reserved-name declaration does
not make its name callable. Formula_unbound_name retains the exact callee, lookup scope
formula_call and the ordered origins_searched list: envelope, builtin_catalog. These are call
lookup sources, not extra origins in §3. After syntax/input validation, static name/kind checking
refuses an unknown or envelope callee before inspecting its arguments. The if special form retains its existing grammar and static branch checks.

For a formula-call envelope refusal, the request scope is formula_call and the requested kind is
the actual construct spelling, including its alias. It does not claim that geometric constraint
parameters or an entity exist. Env_nurbs retains that curve_kind and the supported_curve_set
line_segment, circular_arc, cubic_bezier (units §4). Env_sketch_constraints retains that
constraint_kind and recipe_alternative ordered_construction_recipe. These alternative tags are
diagnostic metadata, not new callables or keywords. Both refusals retain the exact name and the
searched envelope source; builtin_catalog was not searched after that refusal.

These name-only queries carry no statement index, source span, canonical expression, numeric
value or geometry. An enclosing checker may attach only context actually present (§5.2).

#### 5.2.3 Expression dimension arguments and error selection

An expression dimension refusal carries operation, operand_kinds, operand_tolerances and
wanted_signatures. Operation is the actual operator token (unary/binary minus is -, square is ^2)
or exact built-in/special-form name. Operand_kinds retains every resolved immediate operand in
source order. Operand_tolerances retains the actual reserved tolerance name for a direct symbolic
name operand, otherwise none; a computed length cannot impersonate a class. Grouping is transparent.
Each wanted signature retains its ordered operands, variadic flag and result. Exact kind names,
shared T/N and the tolerance requirement have the meanings of grammar §5–§7; every alternative
is retained, including directed products/quotients. A variadic row repeats its shared T from one
operand; the structural argument bound remains a separate syntax/input rule.

Complete recipe checking has three ordered phases. First, after whole-source lexical validation,
every identified statement passes header/operand syntax and structural bounds. Raw annotation
admissibility retains its header priority before RHS grammar (§5.2.5). Second, every literal input
is normalized and checked in statement/operand source order, including untaken branches. Third,
static name/kind checking runs in source order and publishes only validated prior let metadata.
Each phase completes for the whole source before the next starts. A later malformed assertion
class or structural bound therefore precedes an earlier over-width literal, and either precedes
an earlier missing name or kind mismatch. The closed token alone does not identify the phase:
both a structural bound and a literal magnitude can raise formula_domain. Successful static
checking reuses the actual normalized operands and original locations, without reparsing or
returning an accepted prefix. Detached expression/statement adapters retain their local scopes.
During static checking, an ordinary call resolves its callee
before its arguments (§5.2.2). Known operations resolve child expressions left-to-right before
checking their complete immediate signature. For if, the order is condition, then branch, else
branch; for within it is all positional operands, followed by the symbolic-class requirement.
Thus an unbound child in if(1,missing,1 mm) wins over the bad condition kind: there is no complete
actual operand-kind list yet. If every child resolves, the complete dimension refusal retains
all three kinds. No unavailable kind is guessed, and no branch is numerically evaluated here.

The arc_length recommendation applies only to a refused angle/length multiplication, as grammar
§5.1 specifies. A bare call lookup has no expression operands; header-only refusals and operation
geometry adapters retain their separate checking scopes and are not accepted-expression proofs.

#### 5.2.4 Geometry argument refusals

An operation provider's point coordinates x/y and edge length len must each have kind length.
Their argument sources are parsed first, then all child kinds resolve in that order before any
argument evaluates. An unresolved or otherwise invalid child keeps its own diagnostic; the provider
cannot report a complete kind tuple until every child resolves. These checks preserve the grammar.

For a provider-kind mismatch, formula_dimension carries diagnostic_scope geometry_arguments,
operation (the actual provider kind point or edge), operand_names (x/y or len in order),
operand_kinds (all actual resolved kinds) and wanted_kinds (length at every argument position).
Point and edge here identify operation providers; they are not additional formula callables.
This boundary has no recipe ordinal, entity identity or canonical expression unless an enclosing
operation supplies it. No absent context is fabricated. Static refusal computes no value and
publishes no provider result, cache flag or contribution metadata. Parsing retains its existing
literal-input conversion. Successful evaluation preserves each coordinate's contribution sources.

The [reference geometry controls](../annexes/formula-static-validation.md#geometry-provider-argument-checking)
exercise this local adapter boundary. They do not certify the product operation graph, physical
geometry or atomic execution of a complete recipe; those retain their implementation owners.

#### 5.2.5 Binding-header dimension arguments

A let annotation can fail before its RHS has a typed kind. That refusal keeps formula_dimension,
with diagnostic_scope binding_annotation, operation let, name, raw_annotation, annotation_span
and wanted_kinds: length, angle, area, ratio, count, boolean in that order. The raw spelling is
not a declared kind. No expression kind or canonical expression is claimed. Whole-source lexical
preflight keeps precedence; after it, an unbindable annotation precedes RHS grammar/inference.

A bindable annotation whose fully checked RHS has another kind uses diagnostic_scope binding_kind,
operation let, name, declared_kind, expression_kind, annotation_span and wanted_kinds containing
only the declared kind. An invalid RHS child keeps its own diagnostic before this mismatch.
These case-specific fields distinguish annotation admissibility from an actual kind comparison.

annotation_span is the half-open byte span of the original annotation token. A detached statement
uses statement-local offsets and omits statement_index. Complete ordered recipe preflight uses
whole-source offsets and its genuine one-based statement_index. The tuple-AST reference has no
canonical expression factory and supplies none; product integration may attach only its actual
normalized owner. Static refusal publishes no binding or partial plan and invokes no evaluation.

### 5.3 Precedence with the envelope's diagnostics

Where the refused construct is a *feature* the [envelope](feature-matrix.md) dispositions, the
envelope's token is raised and not a formula token: the matrix is the boundary of the release claim,
and two token sets must not compete for one refusal.

| Construct asked for | Raised | Owned by |
| --- | --- | --- |
| `nurbs`, `spline`, `bspline` | `env_nurbs` | the matrix §10 |
| `solve`, `constraint`, `fixpoint` | `env_sketch_constraints` | the matrix §10 |

## 6. Exclusions

The capabilities below remain excluded under the existing v1 grammar. The director's D124 ruling
preserves its source forms and keywords; diagnostic recognition is verified in the
[static review annex](../annexes/formula-static-validation.md#complete-static-review-and-remaining-contracts).

| Excluded | Why | Diagnostic |
| --- | --- | --- |
| NURBS-class curves and expressions | the curve set is three primitives (units §4) | `env_nurbs` |
| implicit solving inside an expression | the CSP is a separate engine (roadmap §6.2, ADR-0001) | `env_sketch_constraints` |
| loops, iteration, recursion | a recipe is a bounded ordered list; repetition is an operation list | unknown call: `formula_unbound_name`; malformed syntax: `formula_parse` |
| user-defined functions and macros | v1 has no function-definition or macro scope | unknown call: `formula_unbound_name`; malformed syntax: `formula_parse` |
| text values, a size label in a formula | a label is prose ([size sets](size-sets.md) §3) | `formula_parse` |
| a formula that creates a point or an edge | geometry is an operation's, with its identity (§2) | `formula_dimension` |
| an exponent other than 2 | an area is the only product the model stores (units §1.3) | `formula_unsupported` |
| an area literal, a radian literal | an area is derived; π is irrational ([grammar §2](formula-language/grammar.md)) | `formula_parse` |
| exponent notation, a float literal | no float enters canonical content (units §7) | `formula_parse` |
| a comment inside an expression | it would not survive canonicalization ([grammar §4](formula-language/grammar.md)) | `formula_parse` |

The excluded capabilities above introduce no extra source forms or keywords. Only let, assert and
if are reserved (grammar §1.1). Unknown calls, including loop(width) or repeat(2,width), raise
formula_unbound_name before argument semantics. Malformed definitions such as fn helper(width)=width
raise formula_parse. Scalar names loop, repeat, while, fn and macro remain ordinary identifiers.
A recognized non-square exponent retains formula_unsupported; envelope calls retain §5.3 precedence.
The [static recognition controls](../annexes/formula-static-validation.md#complete-static-review-and-remaining-contracts)
watch these boundaries without executing values.

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
