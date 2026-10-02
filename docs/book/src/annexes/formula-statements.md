# Annex: single formula statement syntax

> **Status:** implemented by G1-SLICE.5a.3e.1 in sc-core. This API reads exactly one statement.
> Ordered recipe composition, its4096-statement limit and recipe diagnostic ordinals remain .3e.2.
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
semicolons do not become an implicitly accepted recipe. Empty/multiple statement lists belong to the
ordered recipe API still owned by .3e.2. Parsing does not enforce unique names, resolve origins,
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
complete recipes, statement serialization, numerical evaluation and production approval remain later.
