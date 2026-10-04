# Annex: formula statements and ordered recipe syntax

> **Status:** single statements (.3e.1) and ordered recipe syntax (.3e.2) are implemented in sc-core.
> The complete recipe API retains authored order, the4096-statement limit and diagnostic indices.
> Whole input normalization is [implemented](formula-recipe-inputs.md) at .3f.1b;
> statement/recipe identity is implemented at .3f.1c. [Current scoped kind checks](formula-name-scopes.md#check-the-actual-current-statement)
> and [whole recipe kind proofs](formula-checked-recipes.md) are implemented separately; numerical evaluation remains later work.

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
names uses formula_parse with UnknownTolerance and the original annotation span. These families
match the curated reference. A valid class with unavailable context retains the separate runtime
formula_tolerance_unbound rule; parsing supplies no context value.
Expression structural excess retains formula_domain with the measured bound; unsupported powers
retain formula_unsupported. Customer text is available only through explicit source/span inspection.
No canonical context is invented for malformed syntax; explicit successful literal normalization
provides expression identity separately.

For assert closure:eps_chord=1 mm==1 mm, UnknownTolerance identifies the eps_chord span and
formula_parse identifies the grammar refusal. Missing assignment or a malformed RHS after that
annotation does not replace the earlier header error. In a complete recipe, the same nested rule
keeps the containing statement's genuine ordinal and whole-source span.
By comparison, assert closure:eps_phys=missing_left==missing_right is accepted syntax; later
name/static and runtime context checks have their own boundaries. Parsing cannot claim that the
names exist or the physical tolerance is available.

The public contract exercises144 invalid-class refusals across nine independent spellings,
four RHS/assignment forms and standalone/three ordered placements, plus10 valid-class acceptances.
All15 actual compiled statement fault controls remain required, including restoring the old
runtime-token mapping as a body assertion fault. Source restoration and noise refusal are enforced
by the tracked statement_mutations.py producer.

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

10 public contracts verify fifteen independently authored header/operand rows, all six kinds/five
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
compile and fail assertions; the exclusive runner restores exact source and rebuilds the complete
focused statement target before returning. Its [producer profile](../build-and-checks.md#direct-python-producers)
validates declared source/output before reads/writes and preserves prepared child stores. Run it alone.
The failure classifier rejects passing-test-name and expect-only noise, checking failed-test bodies.
Native/release checks and WASM cross-compilation retain their existing scope; browser execution,
numerical evaluation and production approval remain later. Normalized statement/recipe
serialization is [implemented separately](formula-recipe-inputs.md).

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
This syntax view has no canonical-byte factory. [Normalize the recipe](formula-recipe-inputs.md)
to obtain the separate immutable statement/recipe serialization APIs. Persistence and recipe hashing
retain their later owners.

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

The initial eight public contracts check nine independently authored whole sources with fourteen original
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
alone. The entry validates both sources/output before reads/writes and preserves prepared child stores.
After exact restoration it rebuilds the full focused recipe target, including after coupled mode.
Classifier-only and coupled selection remain available. The structural suite watches the independent
producer and failure-classifier controls.
Native/release and WASM cross-compilation retain their stated scope. Coupled syntax review .3e.3
is complete below; recipe normalization/identity .3f is implemented separately. Numerical execution
and production approval remain owned future work.

## Coupled syntax and diagnostic review

G1-SLICE.5a.3e.3 reviews the complete standalone/ordered syntax obligation set against grammar1,
contract2/4.3/5 and the worked statements. It closes .3e for immutable syntax and available diagnostic
context. No product parser/serializer implementation changes in this review; the public recipe suite
now has eleven tests. Exact numerical evaluation and the full command diagnostic envelope remain later.

| Obligation | Product evidence | Independent check or remaining owner |
| --- | --- | --- |
| ASCII/lower-snake/three keywords | Lexical and statement contracts, original spans | Shared machine-token fixtures and actual recursive reference |
| Closed six kind/five tolerance annotations | Fifteen statement rows and typed header refusals | Actual reference headers; no type inference or tolerance values |
| Empty/ordered/multiline/same-line recipes | Original source/ordered slice tests | Nine authored complete sources, fourteen statements/eighteen identities |
| Token boundaries, grouping, exact unit gaps | New coupled zero-gap/whitespace/prefix controls | Four authored sources/eight statements/ten identities; sixteen malformed reference cases |
| Assertion separator and both operands | Grouped/call comparisons and missing/multiple-separator refusals | Actual reference split; independent operand bytes |
| Global header/node/error spans and known indices | Exact later-statement twelve-header/eight-operand matrix | Authored source positions; global ASCII preflight has no known index |
| Separate256-node/4096-statement/16-if budgets | New simultaneous maximum on64KiB stack | Independent256-node/16-depth shape; normative4096 cap |
| Immutability/source/view lifetimes/Clone/privacy | Private constructors, six compile-fail docs, two runnable docs, opaque Debug controls | Actual statement/recipe fault controls; explicit inspection remains customer-bearing |
| All worked source statements and operand identity | Seventeen bindings/four assertions in order; twenty-five expression byte controls | Independently authored byte population and actual recursive reference |
| Complete recipe normalization/identity | Syntax retains names/annotations/order/source without conversion | .3f.1b/.1c/.3f.2; exact statement/recipe bytes specified below |
| Static names/types, numeric binding, assertion execution and geometry | Syntax accepts unevaluated expressions; it grants no result | G1-SLICE.5; later tasks decomposed before implementation |
| Semantic diagnostic arguments/localized command envelope | Typed syntax refusal/source/known index only | G1-SLICE.5/.6; no invented canonical context for malformed input |

### Token boundaries need distinct tokens

The grammar requires no delimiter between statements. If tokenization can distinguish a keyword,
after a digit or closing parenthesis for example, no whitespace is required: let a: count = 1let b:
count = 2 is two statements. Prefer one statement per line when authoring. A keyword prefix inside
an identifier remains one name: let_value and assertion are ordinary identifiers. A name ending
with let, or a unit followed by let with no gap, is also one word; blet or mmlet does not secretly
start another statement. The new controls check these cases and every ASCII whitespace character.

### Context survives without claiming evaluation

The later-statement matrix checks exact byte positions and typed rules for twelve malformed headers
and eight missing/invalid operands. Known indices remain1-based even after two accepted statements.
A missing operand ends at the next keyword or EOF. Its error exposes no partially accepted recipe.
The contract5.2 D110 correction supplies index/canonical identity where available: global ASCII
preflight has no identified statement, and refused/non-normalizable syntax has no canonical identity.
Exact source rule/span remain mandatory; semantic diagnostics still owe their table's arguments.

The simultaneous maximum uses4096 assertions, each with two256-node operands containing16 if
levels. Each operand has one literal, sixteen conditionals adding three nodes apiece, and207 unary
nodes. The actual recursive reference checks that independently constructed shape before the public
product test parses and drops the whole recipe on a64KiB stack. This proves the three syntax budgets
remain separate for that maximum; it supplies no browser runtime, numerical or memory-performance
certificate. The small-stack source/Clone/grouping/name controls keep their earlier scope.

Five additional actual compiled faults require failure in only the new coupled tests: invented
boundary whitespace, a flattened later index, substituted nested rule, wrong header EOF origin
and refusal of the valid simultaneous maximum. The exclusive runner restores both product sources
byte-identically; the tracked structural suite watches its anchors and failure classifier.

```bash
cargo test -p sc-core --test formula_recipe_contract
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_mutations.py --coupled
```

D109 identified the exact assertion/whole-recipe byte gap before serializer implementation. The
contract below now settles it; product statement/recipe normalization and serialization are available at
.3f.1b/.1c. This syntax review grants no project hash, storage or
production approval.

## Canonical statement and recipe byte contract

Grammar §4.1 specifies the full byte envelope before product serialization. A binding retains
(bind NAME KIND EXPR). An assertion is (assert NAME TOLERANCE LEFT RIGHT), with no extra equality
wrapper: its role already means compare two operands at the named class. A complete ordered recipe
is (recipe STATEMENT1 STATEMENT2 ...); empty or whitespace-only syntax is (recipe). Uppercase parts
are placeholders. One ASCII space separates parts and there is no terminal newline.

The specification preserves name, declared kind/symbolic tolerance and operand/statement order.
Unit aliases, source gaps/spans and redundant grouping stay outside identity, as for expressions.
A repeated declaration remains present in order; later static validation may refuse it, but input
normalization or identity construction does not silently merge it. Unknown calls, raw signed/multi-turn
angles, incompatible declared/expression kinds and unevaluated branches retain their existing scope.

Sixteen independently authored statement-byte rows cover all six kinds and five tolerance names,
length aliases, grouped comparisons, unknown ordered calls, raw-turn/sign identity and the positive
2^63 child of the minimum signed angle. Nine authored whole sources cover empty/all-whitespace,
single statements, adjacent/multiline order, duplicate/forward bindings and nested operands; their
twelve statement chunks preserve complete original token coverage. The actual recursive book-reference
syntax supplies operands; inference/evaluation are trapped. Authored chunks do not claim an independent
whole-recipe parser or compiled product serializer.

Nine actual interpreter faults must fail those exact-byte assertions: binding opcode, missing name,
missing annotation, assertion operand reversal/extra equality wrapper, recipe opcode/order,
terminal newline and lost empty envelope. The exclusive runner restores the tracked producer exactly;
the structural suite watches its producer, actual anchors and classifier noise controls. These are
specification/reference controls, not product implementation proof.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_byte_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_byte_mutations.py
```

D109's technical decision is docs/decisions/decision_recipe-bytes.md. The engineer authored and
applied it under standing delegation; independent evidence approval remains unapproved and the
director may re-open the spelling. Product immutable normalization is [available](formula-recipe-inputs.md) at .3f.1b, with [owned identity](formula-recipe-inputs.md#own-canonical-statement-and-recipe-identity) at .1c.
[Coupled contract review .3f.2](formula-recipe-inputs.md#coupled-input-and-identity-review) is complete. Future .7 frames typed schema/digest domains: bind/recipe are valid
ordinary expression call names, so canonical text is interpreted within its known identity type.
No project format, hash, save/recovery, evaluator, MCP or production release is supplied by this record.
