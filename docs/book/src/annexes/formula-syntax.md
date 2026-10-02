# Annex: formula syntax API

The formula contract is normative. The current `sc_core::recipe` implementation supplies a borrowed
lexical stream and an immutable expression syntax tree with exact source spans and structural bounds.
Syntax parsing does not perform numeric conversion. Parsed literals and whole expression arenas now have explicit
[normalization APIs](formula-literals.md). [Canonical expression identity](formula-literals.md#serialize-canonical-expression-identity) is now
implemented separately, as is [whole statement/recipe identity](formula-recipe-inputs.md). Binding,
type/name validation and evaluation remain G1-SLICE.5 work. See [availability](../availability.md) and
[the complete grammar](../spec/formula-language/grammar.md).
Single forms and ordered recipes have a separate [statement syntax API](formula-statements.md).

## Read a machine statement

```rust
use sc_core::recipe::{FormulaLexer, FormulaLexemeKind};

let source = "let garment_waist: length = waist_girth + ease_waist";
let mut lexer = FormulaLexer::new(source);
let first = lexer.next().expect("item").expect("machine token");
assert_eq!(first.kind(), FormulaLexemeKind::Let);
assert_eq!(first.text(), "let");
assert_eq!((first.span().start(), first.span().end()), (0, 3));
```

A token borrows its original source. Its text and location cannot be retargeted by field mutation,
and it cannot outlive the source. A source span is a location in this input, not a stable garment
entity or a canonical expression identity. Keep the source when presenting a location to a user.

| API | Contract |
| --- | --- |
| FormulaLexer | Borrowed iterator of token results; first error and end permanently stop it |
| FormulaLexeme | Read-only kind, exact borrowed text and source span |
| FormulaSourceSpan | Inclusive start and exclusive end byte offsets |
| FormulaLexemeKind | Keywords, identifier/number roles, punctuation and operators |
| FormulaLexicalError | Refused rule/location; diagnostic family formula_parse |
| FormulaLexicalRule | ASCII, spelling, fraction, comparison-pair, character or source-boundary refusal |

## Lexical rules and error scope

Before any token is produced, ASCII preflight refuses the first non-ASCII scalar, with its complete
UTF-8 byte span. Display glyphs such as `√` and `°` are not machine syntax. Identifier spelling uses
the same lower-snake rule as MachineToken: only `let`, `assert` and `if` are grammar keywords.
Kinds, units, function names and reserved inputs remain identifier tokens; lexing grants no permission
to bind or call them.

Number tokens preserve digits and an optional nonempty decimal fraction, including leading zeroes
and original precision. A minus sign is a separate token. No numeric overflow/rounding decision is
made here. Operators use longest comparison pairs: `==`, `!=`, `<=` and `>=`; lone `!` refuses.
General ASCII whitespace, including vertical tab, is skipped. Original byte gaps remain available
for the later parser's exact single-space separator between a literal and its unit.

Errors carry the rule and location rather than customer source. Lexer Debug reports scope/state
without dumping the input; a lexeme's Debug/text intentionally exposes that one lexeme for inspection.
A later parser must attach statement/expression context where available. This low-level error is not
a complete command diagnostic or a certificate of a valid recipe.

## What a successful scan does not prove

`1e3`, `a < b < c`, `if(a, b)` and `// comment` can consist entirely of tokens. The parser must refuse
invalid adjacency, chained comparison, incomplete conditional and comments. Units, square-only power,
closed function vocabulary, node/statement/depth/rational bounds, name/type checking and numeric
semantics are separate obligations. A scan must not be used as a recipe-validation API.

The [worked examples](../spec/formula-language/examples.md) are lexical test inputs: all 17 bindings,
four assertions and 13 refusal forms are scanned. The text-literal refusal fails at lexing; other
refusals are deliberately left to their syntax/type/name/evaluation owners. No example's numerical
value is certified by this scanner. The independent chapter census retains its reference-evaluator
scope until product evaluation is implemented.

## Verification and ownership

G1-SLICE.5a.1 owns the scanner, shared spelling/keyword classifier and this annex. Native strict checks,
WASM cross-compilation, borrowing/privacy doctests and source/book publication checks cover this
slice. Thirteen lexical contracts and nine deliberate production guard mutations exercise ASCII,
spelling, keyword role, comparisons, decimal fractions, spans, termination, privacy and whitespace.
The mutation runner restores both production sources byte-identically and must run without overlapping
builds or gates:

```bash
cargo test -p sc-core --test formula_lex_contract
bash docs/tasks/artifacts/formula_lex/run_formula_lex_mutations.sh
```

G1-SLICE.5a.2b.2 implements expression trees below; .5a.3 owns exact literals/canonical ordered recipes and .5a.4
reviews syntax completion. Exact evaluation, geometry, command/API/MCP workflows and release proofs
remain pending. A successful library WASM build is not a working browser application.

## Reference structural-limit controls

The chapter census is a reference evaluator, separate from the product lexer and syntax parser.
G1-SLICE.5a.2a corrected D75: its walkers previously skipped ordinary calls' argument lists, letting
258-node and 17-level conditional fixtures pass while reporting only seven nodes/zero depth. This
was an instrument defect; no production evaluation API existed to certify those forms.

The reference now traverses every semantic child, including every call argument and both conditional
branches. Grouping and a square's exponent payload are not extra expression nodes. Traversal itself
is iterative; a 5001-node prebuilt AST exercises walker stack safety without accepting that AST as a
valid expression. The reference parser remains an instrument for the book, not a production parser
with arbitrary-input safety guarantees.

Controls accept exactly 256 semantic nodes and 16 nested conditional levels. Forms at 257/258 nodes
or 17 levels raise formula_domain with the measured size and unchanged bound before inference or
evaluation. Depth inside ordinary calls and untaken branches is static structure, so it must refuse
even if evaluation would never visit that branch. Sibling depths are maximized, not added together.
These controls establish node/depth coverage; they do not independently certify the statement or
rational-bit limits, dimensional correctness, numerical values or complete recipe diagnostics.

```bash
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
bash docs/tasks/artifacts/formula_structure/run_formula_structure_mutations.sh
```

The first command runs 16 structural controls/refusals and two copied-book refusal cases. The second
runs alone: it disables four actual reference traversal/depth guards, requires assertion failures,
and restores exact source bytes. Existing formula-language probes still verify chapter/fixture/value
agreement. The product syntax parser below has its own boundary and stack-safety contracts.

## Reference machine-input controls

The book's reference input parser now refuses malformed lower-snake identifiers, non-ASCII machine
source, keywords used as ordinary names/declaration names/call names, and empty call argument lists.
`if(a, b, c)` retains its special three-part grammar; `if_else(a)` is an ordinary syntactic call.
These controls do not grant permission to call an unknown function or rebind a reserved input.

A numeric token followed by a known unit must have exactly one original ASCII space between them:
`1 cm` is valid; `1cm`, `1  cm` and `1\tcm` refuse formula_parse. Other general ASCII whitespace
remains valid between tokens. Source positions survive filtering, so assertion splitting cannot
silently repair an invalid unit separator. This repairs D76; it changes no normative language rule.

```bash
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
bash docs/tasks/artifacts/formula_structure/run_formula_input_mutations.sh
```

The suite also runs 130 machine-input controls using the actual reference and all seven canonical
unit-table entries, plus three copied-book refusal cases. The mutation command runs alone, disables
nine actual input guards, requires assertion failures and restores exact source bytes. Existing
structural and numerical chapter checks retain their separate scope. The reference is a curated
book instrument; these results are not arbitrary-input production safety, full binding/type checking,
complete command diagnostics or a product parsing/evaluation API. Product AST work is implemented by
G1-SLICE.5a.2b.2 below; canonical recipes and later checker/evaluator owners remain pending.

## Parse and inspect one expression

```rust
use sc_core::recipe::{FormulaExpression, FormulaNodeKind};

let expression = FormulaExpression::parse("waist_girth + 1 cm").expect("valid syntax");
assert_eq!(expression.node_count(), 3);
assert_eq!(expression.conditional_depth(), 0);
assert!(matches!(expression.root().kind(), FormulaNodeKind::Binary { .. }));
```

FormulaExpression borrows name and numeric spelling while privately owning a flat semantic arena.
Root/child views cannot outlive that arena, and callers cannot forge indices, mutate its nodes or
transfer an index from another expression. Explicit inspection returns source text; Debug of the
expression, node view, argument iterator and errors omits customer source. A cloned tree owns its
own arena and still borrows the original spelling.

| API | Meaning |
| --- | --- |
| FormulaExpression | Whole-expression syntax, read-only root/count/depth; no statement or value |
| FormulaNode / FormulaNodeKind | Read-only source extent and inspected literal/name/operator/call/if structure |
| FormulaArguments | Ordered nonempty call arguments; exact-size, fused iteration |
| FormulaUnit | Seven closed machine unit tokens; no conversion here |
| FormulaBinaryOperator | Syntax role, with no dimensional permission implied |
| FormulaParseError / FormulaParseRule | Source span and typed syntax or measured-limit refusal |
| FormulaExpressionLimit | Fixed max_expression_nodes 256 and max_if_depth 16 |

`-x ^ 2` has a negation around a square; `(-x) ^ 2` has a square around a negation.
Addition/multiplication/division associate left. A comparison cannot chain within one expression;
parenthesized comparisons form separate syntax, with their dimensional validity checked later.
The exact exponent token is `2`; `02`, `2.0`, `3` or a name raises formula_unsupported. A second
ungrouped square suffix is a grammar refusal. Calls require at least one argument; the `if` special
form requires exactly three and preserves both branches. An unknown well-spelled call name is still
syntax: the later closed-vocabulary/name checker owns permission to call it.

Literal units use exactly one original ASCII space. Numeric digits/precision remain unconverted:
`2.5 cm` and `25 mm` are not yet one canonical syntax identity. Units/kinds, semantic node structure
and source locations are distinct concepts. Grouping extends the enclosed node's extent without
adding a semantic node; a square exponent is operator payload, not another literal node.

## Product structural bounds and refusal scope

The parser streams the existing lexer with explicit operator/value/delimiter stacks. Parse and arena
destruction do not recurse with input nesting. Exactly 256 semantic nodes and 16 conditional levels
are permitted. The next encountered node/conditional raises formula_domain with a typed limit,
fixed bound and measured 257/17 size. That measurement is the refusal point, not an invented final
size. Ordinary call arguments and every static branch count; sibling conditional depths maximize.

Grouping has no invented language cap: a small-stack contract parses/drops 50000 nested parentheses,
while pathological unary/call chains refuse at the semantic bound. Delimiter workspace is linear in
source length, the semantic arena is bounded. Input-byte/work budgets at exposed command/API/MCP
boundaries remain roadmap §10 work; this library is not that complete workflow.

Syntax/lexical refusals carry formula_parse, except unsupported exponents. They locate offending
tokens/gaps, an unclosed opener or zero-width EOF. Errors preserve lexical rule/spans. This low-level
expression API has no statement index or canonical expression context; later recipe/command wrappers
must attach available context. It cannot invent a canonical form for unparseable source.

```bash
cargo test -p sc-core --test formula_expression_contract
bash docs/tasks/artifacts/formula_structure/run_formula_expression_mutations.sh
```

Fifteen contracts include all 17 published binding expressions, eight assertion sides, three syntax
refusals and ten forms retained for later semantic refusal owners. A 20736-input short-token corpus
checks internal construction and root reachability. Three privacy/lifetime doctests cover the arena
and source borrows. Twelve explicit shape/count/depth fixtures are independently checked by the
product and the existing reference; the structural probe suite runs their reference half. Eleven
actual production guard/order mutations require assertion reds and byte-identical restoration; run
that command alone, without builds, probes or gates overlapping it. Strict native and three-library
WASM checks pass locally; no browser application or new remote-CI result is implied.

G1-SLICE.5a.2 is complete for expression syntax. Canonical literals/recipes and statement/rational
bounds belong to .5a.3/.4 and later numeric checking; name/type checks, exact evaluation/DAG,
operation recipes and geometry remain explicit owners under .5 and later gates. Parsing an unknown
value's name does not read it or grant a numeric fallback.

## Reference literal quantum and identity controls

D79 is repaired at the input boundary: bare decimals and all seven unit forms convert exactly once
and round half away from zero to their canonical internal integer. Counts remain counts; a bare
decimal or percent remains a ratio. Minus is still a separate operator, preserving sign symmetry.
The census compares that actual node to the published canonical integer; it no longer rounds a
fractional node's display to conceal a mismatch.

For example, each literal in `0.00004 cm + 0.00004 cm` becomes length:0 before addition, matching
`0 um + 0 um`. The same holds for two bare `0.0000004` ratios. `0.00005 cm` becomes length:1,
while `0.0000005` becomes ratio:1. Kind is part of identity: count 1 and ratio 1.0 remain distinct.
Angles retain their literal microdegree value here; stored-angle normalization is a separate audit.

```bash
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
bash docs/tasks/artifacts/formula_structure/run_literal_mutations.sh
python3 -I -B docs/tasks/artifacts/formula_structure/literal_diagnostic.py
```

Sixty explicit rows and 361 controls check all unit factors, below/at/above ties, zeroes, leading
zeroes, decimal precision, kind-preserving respellings, sign symmetry and sums of converted inputs.
An independent standard-library Decimal oracle checks every explicit row. Six actual reference
mutations require assertion failures and exact source restoration; run that command alone.
The diagnostic prints observations, and its exit status alone is not a correctness verdict.

The reference remains a curated book instrument. These literal controls do not certify rational
bounds, all numeric domains or stored angles on their own: the scoped boundary review below joins
those proofs; the signed-angle review below completes D84 reference semantics. Operator
precision has its separate proof below. Current published example checks retain their row scope.
Product literal normalization and [owned canonical expression identity](formula-literals.md#serialize-canonical-expression-identity)
are implemented. The syntax API above performs no computation; recipe evaluation remains pending.

## Reference exact-arithmetic controls

D82 is repaired: square, multiplication, division and the reference's rational edge selector retain
exact reduced Fractions in the result kind's internal units. The arithmetic operators do not round
their output. `1 um / 2 + 1 um / 2` therefore computes exactly 1 um; `0.000001 ^ 2` retains internal
ratio 1/1000000 until binding rounds it to zero. A later operation can use that fraction before it
is bound, so it must not disappear at the square operator.

Explicit `round_to` still quantizes to the authored step. Irrational calls still return the nearest
internal quantum, half away from zero. A binding still rounds once; the reference census stores that
integer at the statement boundary. These operations are distinct from canonical literal conversion.
Kind signatures, ratio scaling, lazy conditional evaluation and typed zero division remain unchanged.

```bash
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
bash docs/tasks/artifacts/formula_structure/run_arithmetic_mutations.sh
```

Twenty-four explicit expressions and 100 independent Fraction parameter cases supply 162 controls
for precision, dimensional result, re-association, binding ties, signs, comparisons and preserved
rounding/refusal boundaries. Nine actual arithmetic/scale/zero/branch/quantization mutations require
assertion failures and byte-identical restoration; run alone. The selector checks use the reference's
length-only edge model, and certify no real curve inversion or geometric accuracy.

D83’s scoped reference boundary review is complete below; width/scalar/binding/literal proofs
retain their separate oracles; the signed-angle review below completes D84 reference semantics.
These curated book controls can check their exact stated boundaries. They do not supply a production
evaluator, cross-platform numerical signoff, command API or release certificate; those remain
separate product obligations.

## Reference angle conversion and direction controls

Reference D85/D86/D87 are repaired. Trigonometric input and arc_length convert internal microdegrees
directly to radians by multiplying by pi/(180*1000000). A full-turn sweep remains a full turn and a
signed or fractional sweep remains signed or fractional; this conversion does not normalize a sweep.
The read-only dir selector rounds its microdegree result to the nearest quantum before direction
normalization, corresponding modulo360 degrees to signed atan2 on the same vector. Tangent at an exact odd quarter-turn refuses
formula_domain, including negative and multi-turn poles; neighboring microdegree inputs remain finite.

For example, `arc_length(360 deg, 1 um)` returns 6 um at the declared rounding, `sin(90 deg)` returns
ratio 1000000, and `dir` for the vector (1,6) returns normalized angle 80537678. Previously, degrees
were scaled incorrectly by a million and direction output truncated rather than rounding.

```bash
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
bash docs/tasks/artifacts/formula_structure/run_angle_mutations.sh
python3 -I -B docs/tasks/artifacts/formula_structure/literal_diagnostic.py
```

Forty-two explicit angular identities/results agree with an independent standard-library math
oracle on defined curated arguments. They supply 72 controls, including signed quadrant/normalized-dir correspondence,
full/signed/multi-turn sweeps, fractional microdegrees, exact poles and finite neighbors. Seven actual
conversion/sweep/precision/direction/pole mutations require assertion reds and exact restoration;
run alone. This remains a curated 60-digit Decimal reference, not an arbitrary-input certificate of
correctly rounded transcendental evaluation or a production geometry implementation.

The director's D84 ruling preserves signed/multi-turn formula angles and normalizes entity direction
fields. A bound 360-degree sweep therefore retains its full turn; formula equality does not collapse
it to zero. `dir` still normalizes by its explicit function contract. The durable record is `docs/decisions/decision_angles.md`.

G1-SLICE.5a.3b.3c.2 removes only outer inverse modulo and updates three negative inverse rows in the
42-row oracle set to signed principal results. The signed binding/equality/sweep controls below
verify D84 separately from D83's numeric boundaries; the final review below closes both prerequisites.
Product numeric binding/evaluation is pending.

## Reference rational-value boundaries

The D83 width repair refuses reduced exact numerators or denominators wider than 128 bits with
`formula_domain`, naming the operation, `max_rational_bits=128` and the measured width. Converted
literal input is checked before quantum rounding: a tiny fraction must not disappear into zero and
hide an oversized denominator. A long spelling that reduces exactly to zero or one remains legal.
Each completed numeric expression result is checked in its kind's internal units. A later cancellation
cannot rescue an oversized earlier sum. Raw cross-products and temporary true-unit scale conversions
are not values under this bound; only the reduced result counts. Untaken branches are parsed and
typed, but their arithmetic is not computed. Canonical literal conversion still checks their inputs.

```bash
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
bash docs/tasks/artifacts/formula_structure/run_rational_mutations.sh
```

Sixty-one independent Fraction controls cover 127/128/129-bit boundaries, denominator width,
converted input, signs, reduction, scale, selectors and lazy branches. Twelve actual mutations require
assertion failures and exact restoration; run alone. The earlier literal/arithmetic/angle six/nine/
seven mutation controls also pass. Angle tests require the exact pole reason: an unrelated rational
refusal cannot count as evidence for a missing mathematical-domain guard (D88).

D83's scalar/binding/literal controls and completed scoped boundary review appear below.
D84 signed-angle verification is reviewed below. [Individual product literals](formula-literals.md)
now normalize explicitly, individually or across a whole syntax arena; full evaluation remains pending. The curated transcendental reference is not a production certificate.

## Inline documentation language context

The book vocabulary census cannot infer a code span's language from punctuation: a Rust error-handling
expression can resemble a formula but legitimately use a question mark. D91 supplies explicit author
context for a single inline span in chapters outside the three normative formula parts:

```markdown
<!-- stitchcad-inline: rust -->`(left + right)?` and `waist_girth + ease_waist`
```

The marker must be exact, on the same line and immediately followed by the one backtick span (optional
whitespace only). It excludes that Rust span from formula-vocabulary classification; the adjacent
formula remains checked. Unknown, malformed, duplicate or detached markers refuse L6e. Normative
formula contract/grammar/example parts reject foreign annotations, preserving their formula positions.
Rust fences retain their existing scope. This declares author intent; it does not compile or certify
Rust and does not extend the formula alphabet. An unannotated question mark still refuses L6b.

```bash
bash docs/tasks/artifacts/formula_language/run_formula_language_probes.sh
bash docs/tasks/artifacts/formula_language/run_inline_context_mutations.sh
```

The watched suite includes thirteen independently authored copied-book context verdicts alongside
its fifteen existing agreement/refusal arms. Five actual classifier mutations discriminate removed,
whole-line or normative exemptions, missing refusal counts and unknown-language acceptance; they
require assertion reds and byte-identical restoration. Run mutations alone. Numeric reference and
product syntax behavior are unchanged; evaluator/API/MCP and release proof remain separately owned.

## Reference scalar-domain boundaries

D83 length and area results now obey the signed limits from the units chapter; count results never
become negative. The reference reads those declarations rather than copying numeric constants.
Reduced rational width is checked first. Canonical literals check their rounded input value even in
untaken branches; completed expression values check their exact fraction before binding rounding.
For example, `1000000000.4 um` rounds to a valid endpoint at input, but
`1000000000 um + 1 um / 10` refuses before a binding can hide its excess. A count expression `0 - 1`
also refuses, while an exact nonnegative count fraction can survive until its binding round.

Every numeric name, operator and call result enters the same scalar boundary, including selectors
and implicit tolerance reads. Refusals name the actual operation, kind, signed measured fraction and
inclusive bounds. A lazy branch's arithmetic does not run, but its literals still canonicalize.
Opaque point/edge references are not scalar numbers; selecting their numeric contents checks those
returned values. This is the curated reference's limited geometry model, not geometric signoff.

```bash
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
bash docs/tasks/artifacts/formula_structure/run_scalar_mutations.sh
```

Fifty-seven independent Fraction/domain controls cover endpoints, fractional excess, signs,
canonical half-quantum boundaries, calls/selectors/tolerances, lazy branches, declared-bound changes
and quiet shared setup. Eleven actual scalar/context mutations require assertion reds and exact
restoration; run alone. Existing twelve rational, six literal, nine arithmetic, seven angular and
five inline-context mutation controls also pass. Width fixtures that used invalid huge lengths now
use unrestricted kinds or tiny valid fractions, preserving their sixty-one width controls.

D94 separates shared table-driven setup from arithmetic assertions: loading scalar, rational or
angular context no longer executes another family's tests. The quiet-load control and mutation
verify this boundary. Shared setup is not an independent numeric oracle; each family's explicit
expected values/refusals remain independently authored.

D83 binding/literal controls and completed scoped review follow; D84 signed-angle
verification is reviewed below. No new magnitude limit is added to exact ratio/angle results by this scalar slice. Product
normalization/evaluation, real geometry, command/API/MCP and production release remain separate work.


## Reference numeric binding storage controls

The director’s D95 ruling separates canonical literal width from bound storage. Exact literal nodes
retain the existing128-bit rational limit; signed unary operators keep their identity. Numeric let
rounds once, checks the kind’s declared signed storage and existing scalar domain, then returns
that integer. Count stays nonnegative; Boolean stays Boolean. Subsequent statements read the stored
integer. An exact unbound expression can remain fractional or wider than i64 within its other limits.

For example, length expression1 um /2 is exactly half a micrometre, but binding it stores1 µm.
Reading that binding twice produces2 µm. At the signed i64 endpoint, adding0.49 internal units
still binds to the endpoint; adding0.5 refuses after rounding. The direct lowest signed angle
spelling retains unary minus of a positive2^63 literal. A positive bound2^63 refuses with binding,
kind, inclusive bounds and measured integer; subtracting1 first may give a valid binding.

```bash
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
bash docs/tasks/artifacts/formula_structure/run_binding_mutations.sh
```

Eighty independent Fraction/Decimal controls verify inclusive endpoints, signed half-quanta,
negative Count refusal, exact unbound/cancelled results, Boolean preservation, unchanged caller-owned
binding environments and replay. An altered copied specification supplies32-bit angle storage to
prove the reader consumes declarations; a missing declaration refuses. Two copied-book bindings
verify the actual census/replay consumer. Twelve compiled actual mutations cover binding bypass,
truncation, omitted/excluded bounds, duplicated width, token/context loss, Boolean reclassification,
caller-environment writes and altered census consumption. Each requires an assertion red and exact
source restoration; run alone.

This reference statement method returns a binding without publishing it into the caller’s supplied
environment. The curated replay caller stores that result explicitly; this is not a production
transaction guarantee or a general environment implementation. Existing scalar/rational controls
remain separate. D95 canonical-node controls and the completed D83 scoped review follow.
D84 inverse-trig/equality verification is reviewed below. Production normalization/serialization/evaluation,
real geometry, API/MCP control and release signoff remain future work.


## Reference canonical literal width and identity controls

D95’s received ruling is verified at the reference parser’s literal nodes. The128-bit limit means
absolute reduced numerator/denominator magnitude, including unsigned magnitudes above signed i128
MAX. A canonical literal may exceed i64. Numeric binding is the later signed64 boundary, with its
other scalar rules. Unary minus retains its node; no sign folding or algebraic simplification is
introduced. Positive zero, unary-negative zero and double negation keep distinct operator shapes.

```bash
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
bash docs/tasks/artifacts/formula_structure/run_canonical_literal_mutations.sh
```

One hundred forty-six independently authored node/Decimal controls cover63/64/127/128-bit literal
magnitudes,129-bit refusals, signed i64 MIN’s wide child, Count negation refusal, later binding
refusal/cancellation, unit aliases, kind identity, reducible long spellings, input half-quanta,
pre-round rational width and post-round scalar bounds. Decimal supplies independent quantum results;
expected literal/operator tuples are authored separately from the reference parser. Twelve compiled
actual faults change width, unary identity, kind, quantization or domain guards; each requires an
assertion red and exact source restoration. Run mutations alone.

Together with binding80 and the preceding lexical/scalar/rational suites, these controls close
D95’s specification ambiguity. D83’s scoped reference review is complete below;
Individual product conversion .5a.3c.2 now supplies [public-contract proof](formula-literals.md)
for width/kind/source and unary separation; .5a.3c.3 now supplies the whole normalized arena. These tuples are reference-model nodes; no production canonical
serializer, persistent formula identity or evaluator is certified by this slice.

## Complete reference numeric boundary review

G1-SLICE.5a.3b.3b.3c.2 closes D83 for the curated reference model after reviewing all original
numeric obligations. The following boundaries are distinct; passing one cannot stand in for another.
The four families re-run344 independently authored controls, and47 actual implementation faults
produce assertion reds before byte-identical evaluator/setup restoration.

| Boundary | Independent producer | Required behavior |
| --- | --- | --- |
| Reduced width | `rational_contract.py`,61 controls/12 actual reds | Converted exact inputs and every completed numeric node obey128-bit numerator/denominator width; oversized children cannot cancel later |
| Scalar domain | `scalar_contract.py`,57 controls/11 actual reds | Canonical input checks its rounded quantum; completed length/area/Count results check exact signed/nonnegative limits before binding |
| Stored numeric value | `binding_contract.py`,80 controls/12 actual reds | Numeric let rounds once, checks inclusive signed i64/scalar bounds and returns the stored integer; Boolean and caller state keep their contracts |
| Literal/operator identity | `canonical_literal_contract.py`,146 controls/12 actual reds | D95 literals may exceed i64 up to128-bit magnitude; kind/unary nodes, input quantum and later binding refusal remain distinct |

All producers and mutation runners live in `docs/tasks/artifacts/formula_structure/`:

```bash
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
bash docs/tasks/artifacts/formula_structure/run_rational_mutations.sh
bash docs/tasks/artifacts/formula_structure/run_scalar_mutations.sh
bash docs/tasks/artifacts/formula_structure/run_binding_mutations.sh
bash docs/tasks/artifacts/formula_structure/run_canonical_literal_mutations.sh
```

Run mutations exclusively; they temporarily alter the actual reference source and restore it.
Fraction/Decimal and authored canonical tuples supply the independent expectations. Altered or
missing specification declarations, exact intermediate refusals, inclusive endpoints, storage ties,
typed operation context, caller state and actual book replay retain discriminating controls.
D89/D90 public-operator/diagnostic prerequisite sources remain byte-identical to their verified
G1-0047 state; D91’s explicit language-context family still passes. D97 corrects only the live
parent’s superseded canonical-i64 wording; past plans and original defect descriptions stay intact.

This completes the reference boundary repair. D84 signed inverse-trig/binding/equality verification
is reviewed below. Production canonical literals, serialization, full evaluation,
real geometry and arbitrary-input transcendental/cross-platform correctness still require their
own product proofs. The reference model is an instrument for this book, not a shipped evaluator.

## Complete binding replay and display

The actual example census consumes all six bindable kinds declared by formula §2. D99’s former
four-kind whitelist rejected valid Area/Boolean statements after the reference binding method had
accepted them. The Value column now handles derived Area in cm² (100000000 µm² per cm²) and Boolean
as true/false (internal1/0). Fixed decimal formatting keeps a1 µm² area visible as0.00000001 cm²;
Decimal’s scientific spelling1E-8 cannot silently fail the table’s decimal grammar.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/binding_replay_contract.py
bash docs/tasks/artifacts/formula_structure/run_binding_replay_mutations.sh
```

Nineteen independent consumer/format/declaration verdicts exercise fifteen added positive rows
alongside all seventeen original examples, signed Area ties, a sub-half zero, stored-value reads,
Boolean true/false and conditional reads, wrong values/units/text, opaque-kind refusal, a copied
Boolean bindability removal and invalid Boolean display state. Nine compiled actual source faults
must fail these assertions: kind whitelist, Area scale/unit, Boolean truth/state/mismatch, scientific
formatting, changed stored integer and lost replay kind. Sources restore byte-identically. The
existing structural suite watches the positive/refusal family; run actual mutations exclusively.

This repairs the book instrument without adding Area/Boolean source literals or a production
formula evaluator. D84 signed-angle controls below retain a separate proof boundary.


## Signed principal angles and raw formula sweeps

The reference atan/atan2 result rounds to a signed microdegree without direction modulo. Their
principal branches and rounded endpoint behavior are specified in grammar §6. Exact zero has one
numeric value, so atan2(-0 um, -1 um) gives+180 degrees. A negative component close to the same axis
may instead round to-180 degrees. The direction selector dir normalizes its rounded result.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/signed_angle_contract.py
bash docs/tasks/artifacts/formula_structure/run_signed_angle_mutations.sh
```

Ninety independent controls cover axes/quadrants, both atan2 signatures and exact dimension/zero
refusals, signed zero, finite rounded endpoint cases, raw bindings/ties/order/equality, signed/full/
multi-turn arc lengths, fractional versus once-rounded bound input, normalized directions, and
actual copied-book replay. Eight new copied rows join all seventeen original rows; a wrong unsigned
Value cell refuses. Independent authored integers/Fraction and curated standard-library math
expectations do not call the reference's inverse/modulo/rounding functions.

For example, binding360 degrees keeps360000000 microdegrees; it differs from zero, and its radius
1 µm arc rounds to6 µm. A720-degree arc rounds to13 µm, and a-720-degree arc to-13 µm. An exact
half-microdegree angle with radius1000 m gives9 µm; binding that angle first rounds it to1 microdegree,
so its later arc gives17 µm. Negative counterparts retain their sign.

Fifteen compiled actual faults must fail these assertions: atan/atan2 modulo, lost inverse sign,
binding/comparison/arc/literal modulo, unnormalized dir, argument order, zero branch/vector/vertical
axis errors, inverse truncation and binding ties. Exact source restoration follows every fault.
The structural suite watches the independent positive/refusal family; mutation runners run alone.
Existing42-row/72-control/math42 angular controls and seven actual angular reds also remain required.

This is curated reference proof, not arbitrary-input correctly rounded transcendental evaluation,
a cross-platform certificate, entity direction integration or a production formula evaluator.
G1-SLICE.5a.3b.3c.3 completes the final D84/reference review below; individual literals now normalize
under .5a.3c.2 and whole arenas under .5a.3c.3; execution remains pending.


## Completed angle and reference obligation review

G1-SLICE.5a.3b.3c.3 closes D84's received contract and scoped reference repair. Every original
consequence has a discriminating control; the prerequisite evaluator/setup/oracle sources remain
byte-identical to their verified G1-0056 state.

| Obligation | Independent control | Discriminating actual fault |
| --- | --- | --- |
| Signed principal inverse result | signed90 axes/quadrants, both signatures, branch/rounded endpoints and one exact zero | inverse modulo/sign/argument order/axis/zero-vector/truncation |
| Raw formula bindings/equality | signed90 ties/full turns/order; binding80 storage; real copied-book replay | binding/literal/comparison modulo and binding tie truncation |
| Signed/full/multi-turn sweep | signed90 direct/bound arcs and fractional input; angular72/math42 | arc/conversion modulo or lost fractional precision |
| Explicit normalized direction | signed90 negative vectors; angular72 nearest quantum | missing dir modulo or direction truncation |
| Six-kind replay | replay19 verdicts, Area/Boolean stored reads and refusals | kind/state/value/scale/unit/format faults |

Re-run the structural suite, signed-angle and angular mutation runners named above. Signed90 and
angular72/math42 controls pass; fifteen/seven compiled actual faults require assertion reds with
exact restoration. Existing numeric and six-kind replay families also pass. This joins D83's earlier
rational/scalar/binding/literal review without treating a diagnostic exit as a correctness verdict.

D100 corrects an adjacent live task label missed by D97: canonical literal width is128-bit,
numeric binding storage is i64. The previous parent and defect text remain intact in task/history
records. Current goals, function/type contracts, independent controls and the roadmap agree.

The reference prerequisites are complete. Product literal normalization/canonical expression bytes
have [separate scoped proof](formula-literals.md). Ordered recipe/binding/evaluation, entity direction
integration, real geometry and production
API/MCP/release remain future work. Curated inverse results certify neither arbitrary-input
correct rounding nor cross-platform transcendental behavior.
