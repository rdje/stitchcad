# Annex: formula syntax API

The formula contract is normative. The current `sc_core::recipe` implementation supplies a borrowed
lexical stream and an immutable expression syntax tree with exact source spans and structural bounds.
Syntax does not supply numeric conversion, canonical identity, ordered recipes, binding, type/name
validation or evaluation. Those remain G1-SLICE.5 work. See [availability](../availability.md) and
[the complete grammar](../spec/formula-language/grammar.md).

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

Sixty explicit rows and 360 controls check all unit factors, below/at/above ties, zeroes, leading
zeroes, decimal precision, kind-preserving respellings, sign symmetry and sums of converted inputs.
An independent standard-library Decimal oracle checks every explicit row. Six actual reference
mutations require assertion failures and exact source restoration; run that command alone.
The diagnostic prints observations, and its exit status alone is not a correctness verdict.

The reference remains a curated book instrument. D82's premature intermediate arithmetic rounding
and D83's incomplete numeric-domain/stored-angle enforcement remain owned by G1-SLICE.5a.3b.2/.3.
For instance, the instrument currently computes `1 um / 2 + 1 um / 2` as 2 rather than exact 1.
This literal proof does not certify exact operator evaluation, rational bounds, all numeric domains
or stored angles. Current published example checks retain their row scope. Product canonicalization
and evaluation remain pending; the syntax API above performs no numeric computation.
