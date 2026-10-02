# Annex: formula syntax API

The formula contract is normative; the current `sc_core::recipe` implementation supplies a borrowed
**lexical stream**. It separates words, numbers and symbols, retaining exact machine spelling and
half-open byte spans. It supplies no expression tree, numeric conversion, binding, evaluation or
canonical formula identity. Those remain G1-SLICE.5 work. See [availability](../availability.md) and
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

G1-SLICE.5a.2 owns expression trees; .5a.3 owns exact literals/canonical ordered recipes and .5a.4
reviews syntax completion. Exact evaluation, geometry, command/API/MCP workflows and release proofs
remain pending. A successful library WASM build is not a working browser application.

## Reference structural-limit controls

The chapter census is a reference evaluator, separate from the product lexer and future parser.
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
agreement. G1-SLICE.5a.2b.2 owns product expression trees after the reference prerequisites.

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
complete command diagnostics or a product parsing/evaluation API. Product AST work belongs to
G1-SLICE.5a.2b.2; canonical recipes and later checker/evaluator owners remain pending.
