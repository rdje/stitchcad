# Annex: formula statements and ordered recipe syntax

> **Status:** single statements (.3e.1) and ordered recipe syntax (.3e.2) are implemented in sc-core.
> The complete recipe API retains authored order, the4096-statement limit and diagnostic indices.
> Statement/recipe identity is owned by .3f.1; name/type/binding validation and evaluation remain G1-SLICE.5 work.

The [formula grammar](../spec/formula-language/grammar.md#1-the-grammar) gives two statement forms:
let declares a name and kind; assert names a closure check and its tolerance class. The public
FormulaStatement API checks these forms and preserves their expression syntax. It does not calculate
a garment dimension, bind the name or determine whether an assertion holds.
For a first introduction, use [the learning path](../learn/design-to-pattern.md); this annex describes
integration details and the proof boundary.

## Parse and inspect one declaration

```rust
use sc_core::recipe::{FormulaStatement, FormulaStatementKind};

let statement = FormulaStatement::parse(
    "let garment_waist: length = waist_girth + ease_waist"
)?;
assert_eq!(statement.name(), "garment_waist");
if let FormulaStatementKind::Let { declared_kind, expression } = statement.kind() {
    assert_eq!(declared_kind.token(), "length");
    // This explicit step converts literal inputs, without resolving or evaluating names.
    let canonical = expression.normalize_literals()?.canonical_form();
    assert_eq!(canonical.as_str(), "(+ waist_girth ease_waist)");
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

The parsed statement privately owns its flat expression arena and borrows the original source.
Clone retains the same source borrowing; callers cannot mutate names, headers, nodes or edges.
A statement view cannot outlive its owner. Explicit name access exposes customer content; Debug of
statements, role views and errors omits customer names and literal magnitudes.

FormulaStatementKind distinguishes Let with its declared kind and expression from Assert with its
symbolic tolerance and two expressions. This is syntax metadata: the expression is not inferred or
checked against the declared kind yet. FormulaBindingKind is exactly length, angle, area, ratio,
count or boolean. There is still no area or Boolean literal; their values come from expressions.

## Assertions keep both operands and the tolerance name

```rust
use sc_core::recipe::{FormulaStatement, FormulaStatementKind};

let statement = FormulaStatement::parse(
    "assert waist_closure: eps_num = quarter_hip - suppression == quarter_waist"
)?;
if let FormulaStatementKind::Assert { tolerance, left, right } = statement.kind() {
    assert_eq!(tolerance.token(), "eps_num");
    assert_eq!(left.normalize_literals()?.canonical_form().as_str(),
               "(- quarter_hip suppression)");
    assert_eq!(right.normalize_literals()?.canonical_form().as_str(), "quarter_waist");
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

FormulaToleranceName is exactly eps_num, eps_geo, eps_fmt, eps_imp or eps_phys. Parsing retains the
symbolic class; it does not supply a number or factory/export context. The chordal sibling of T2
remains a target property, as [reserved names](../spec/formula-language.md#31-reserved-names) specify.

An assertion has exactly one equality separator at parenthesis depth zero. Comparisons inside
ordinary calls, conditional arguments and grouping remain part of their operand. For example,
assert check: eps_num = (a == b) == if(c == d, e, f) preserves the two grouped/call comparisons.
An ungrouped a == b == c is refused as an ambiguous assertion separator. Each operand separately
retains the existing256-node and16-conditional-depth limits. The separator is statement structure,
not another expression node.

## Locations and refusals

Statement, name and annotation spans refer to the original whole input. Every expression node's
span is also in that same source, including grouping and the exact numeric-unit gap. Outer statement
whitespace is outside the statement span. The parser slices original operands and rebases the flat
arena's locations; it never rebuilds operands by joining lexical tokens or normalizing whitespace.

FormulaStatementError retains a typed FormulaStatementRule and exact offending span. Expression
errors add Binding, AssertionLeft or AssertionRight, retaining the original FormulaParseRule and
diagnostic family. Lexical errors encountered after a valid header also retain their operand role.
Whole-input ASCII preflight can fail before any header is known; that error has its lexical rule.
A missing operand ends at the relevant separator or source end, rather than a guessed token location.

Malformed keywords/names/punctuation and assertion separators use formula_parse. An annotation
outside the six bindable kinds uses formula_dimension; an annotation outside the five tolerance
names uses formula_tolerance_unbound. These annotation families match the curated reference.
Expression structural excess retains formula_domain with the measured bound; unsupported powers
retain formula_unsupported. Customer text is available only through explicit source/span inspection.
No canonical context is invented for malformed syntax; explicit successful literal normalization
provides expression identity separately.

A standalone parse consumes the whole input. A second statement, a bare expression, comments or
semicolons do not become an implicitly accepted recipe. Empty/multiple statement lists use the
[ordered recipe API](#parse-an-ordered-recipe). Parsing does not enforce unique names, resolve origins,
refuse a negative count binding, compare operand kinds, check i64 storage, execute arithmetic or
resolve physical tolerances. Those checks remain mandatory before a recipe can execute.

## Product and independent controls

```bash
cargo test -p sc-core --test formula_statement_contract
python3 -I -B docs/tasks/artifacts/formula_structure/statement_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/statement_mutations.py
```

Nine public contracts verify fifteen independently authored header/operand rows, all six kinds/five
tolerances, exact source/error spans, keyword/header/separator/unit refusals, privacy/clone behavior
and all21 worked statements with their25 independently authored expression identities. Three
compile-fail Rust docs enforce private construction/source borrowing/view lifetime; a runnable doc
verifies explicit parsing and expression identity. No statement execution claim follows.

The reference instrument exercises its actual statement header and recursive expression parser,
then deliberately stops before inference or evaluation. Fifteen rows and six refusal families agree;
the tracked structural suite watches the producer. Curated reference syntax is an independent check,
not evidence that the product resolves or computes those statements.

On64KiB stacks the product handles256-node operands on both sides,16 if levels,50000 grouping pairs
and100000-byte names. Existing257-node/17-level refusals remain measured in each operand. Fifteen
actual production keyword/name/annotation/assignment/separator/operand/span/privacy faults must
compile and fail assertions; the exclusive runner restores exact source. Run it alone.
The failure classifier rejects passing-test-name and expect-only noise, checking failed-test bodies.
Native/release checks and WASM cross-compilation retain their existing scope; browser execution,
statement serialization, numerical evaluation and production approval remain later.

## Parse an ordered recipe

FormulaRecipe.parse consumes the complete original machine source and returns an immutable ordered
slice of FormulaStatement arenas. The grammar permits an empty list: empty input and input containing
only ASCII whitespace both produce zero statements. Statements retain their authored order; parsing
does not sort dependencies, reject duplicate names or check that an assertion holds.

```rust
use sc_core::recipe::{FormulaRecipe, FormulaStatementKind};

let source = "let garment_waist: length = waist_girth + ease_waist\n\
              assert closure: eps_num = garment_waist == target_waist";
let recipe = FormulaRecipe::parse(source)?;
assert_eq!(recipe.statements().len(), 2);
assert_eq!(recipe.statements()[0].name(), "garment_waist");
assert_eq!(recipe.statements()[1].name(), "closure");
if let FormulaStatementKind::Assert { tolerance, .. } = recipe.statements()[1].kind() {
    assert_eq!(tolerance.token(), "eps_num");
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

A view borrows its owning recipe; Clone copies its flat arenas while borrowing the same original
source. Private construction prevents replacing the statement list. Debug prints the statement count
without customer names or literals. Explicit statement/name/source inspection exposes those values.
The recipe has no persistent statement serialization or recipe hash yet; .3f.1 owns that byte contract.

### Boundaries preserve original whitespace

After the required statement header, a let or assert keyword at parenthesis depth zero starts the
next statement. Neither a newline nor a semicolon is required. For readability, place each statement
on its own line, while long expressions may span lines. These sources have the same statement list:

```text
let a: count = 1 let b: count = a + 2

let a: count = 1
let b: count = a
              + 2
```

Header whitespace may also contain line breaks. The numeric-unit gap remains exactly one ASCII
space: 25 mm is valid, but 25 followed by a newline and mm is refused. Source token positions and
original operand slices are retained; the parser does not reconstruct or normalize source strings.

Header tokens are consumed before boundary recognition, so let let: count = 1 reports the reserved
name at that header. A keyword within parentheses remains an invalid expression token. For example,
let a: count = (1 let b: count = 2) refuses the first statement rather than starting a second one.
Comments, semicolons, bare expression statements and trailing junk remain outside the grammar.
Standalone FormulaStatement.parse still refuses multiple statements; use FormulaRecipe.parse for a list.

### Recipe errors and the statement limit

FormulaRecipeError exposes diagnostic_code, statement_index, span and a typed FormulaRecipeRule.
Indices start at1. A nested Statement refusal preserves the original FormulaStatementError, including
its header or operand rule. Statement/name/annotation/expression-node/error spans all refer to the
original complete recipe source. A missing operand ends at the next statement keyword or final EOF.
No partial accepted recipe accompanies an error, and no values or geometry have been computed.

Whole-source ASCII preflight occurs before any statement boundary is identified. Its statement_index
is None, including when the non-ASCII character occurs after otherwise valid statements. The span
still identifies that entire Unicode scalar in the original source; no statement index is guessed.
Other known statement refusals carry Some(index). Trailing junk is part of the current operand and
therefore carries that statement's index.

FormulaRecipe.MAX_STATEMENTS is the fixed4096 bound from the [language contract](../spec/formula-language.md#43-structural-limits).
Exactly4096 syntactically valid statements are accepted. A recognized4097th let/assert keyword is
refused before parsing its header or body, with formula_domain, bound4096, measured4097, index4097
and the exact keyword span. The measured count names the first excess statement, not an unparsed
total. Global ASCII preflight retains precedence over that check. Each expression separately keeps
its256-node/16-conditional bounds; assertion operands do not share a combined node budget.

Parsing alone still accepts a negative count expression, unresolved names, duplicate declarations,
mismatched declared kinds, division by zero and a false assertion. Their syntax is retained for the
separate mandatory static/binding/evaluation checks. It neither converts literal inputs nor supplies
tolerance values. Canonical diagnostic context remains unavailable until successful explicit input
normalization; malformed syntax receives no invented canonical expression.

### Ordered recipe proof boundary

```bash
cargo test -p sc-core --test formula_recipe_contract
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_mutations.py
```

Eight public contracts check nine independently authored whole sources with fourteen original
ordered statements and eighteen operand identities. The independent producer verifies complete
non-whitespace source coverage and token order, then uses the actual curated reference header and
recursive expression parser on each authored statement, deliberately trapping semantics. It does
not claim an independent complete-recipe reference parser.

All21 worked book statements compose into one recipe in their authored order and retain their25
independently authored expression identities. Public controls verify empty/multiline/same-line
lists, exact header/node/refusal spans,1-based indices, whole-source ASCII precedence, no partial
acceptance, no semantic execution, privacy/Clone and the exact4096/4097 boundary. Three compile-fail
Rust docs enforce private construction, original-source borrowing and view lifetime; a runnable doc
checks the public recipe API.

On64KiB stacks the API parses/clones/drops4096 statements, preserves50000 grouping pairs and a
100000-byte name, and retains256-node assertion operands/16 nested conditionals. Existing257-node/
17-depth refusals remain contextual. Fifteen new actual production faults in bounds/order/boundaries/
indices/spans/whole-input/privacy compile and fail assertions. The existing fifteen statement faults
also pass against the shared parser. Both exclusive runners restore exact production bytes; run them
alone. The structural suite watches the independent producer and failure-classifier controls.
Native/release and WASM cross-compilation retain their stated scope. Coupled syntax review .3e.3,
complete recipe normalization/identity .3f, numerical execution and production approval remain owned.
