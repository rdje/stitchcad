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
