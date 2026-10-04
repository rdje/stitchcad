# Annex: formula literal normalization

The current sc-core recipe API can convert an individual parsed literal into its typed canonical
integer. Parsing and input conversion remain separate operations: parsing checks syntax;
conversion checks exact rational width, rounds once and checks the input scalar domain.
The [whole-expression API](#normalize-every-literal-in-an-expression) converts every literal while
retaining the syntax graph. The [canonical serializer](#serialize-canonical-expression-identity) emits
owned expression identity bytes. [Complete recipe normalization/identity](formula-recipe-inputs.md) and
[whole recipe kind proofs](formula-checked-recipes.md) are implemented separately; numerical binding
and evaluation remain later work.
See the [language](../spec/formula-language.md) and [grammar](../spec/formula-language/grammar.md).

## Inspect one literal

```rust
use sc_core::recipe::{FormulaExpression, FormulaLiteralKind};

let syntax = FormulaExpression::parse("2.5 cm")?;
let literal = syntax.root().normalized_literal()?.expect("literal root");
assert_eq!(literal.kind(), FormulaLiteralKind::Length);
assert_eq!(literal.magnitude(), 25_000);
assert_eq!(literal.number(), "2.5"); // original borrowed spelling
# Ok::<(), Box<dyn std::error::Error>>(())
```

FormulaNode.normalized_literal converts only that node. It returns None for a name, unary minus,
call or other nonliteral; it does not visit children. To inspect a literal beneath unary minus,
inspect the syntax child explicitly. Unary minus remains its own node, so the positive 2^63
microdegree child can be retained before a future signed binding stores i64 MIN.

| Public API | Contract |
| --- | --- |
| FormulaLiteral | Private construction; borrowed original number/unit/span and once-rounded u128 magnitude |
| FormulaLiteralKind | Count, ratio, length or raw angle; no area or boolean input literal |
| FormulaLiteralError | Located formula_domain refusal; no customer spelling in diagnostics |
| FormulaLiteralRule | Rational-width witness, rounded length-domain value or shared-rounding refusal |
| FormulaRationalComponent | Numerator or denominator that exceeds the reduced exact width |

Read access exposes number, unit, span, kind and magnitude. Debug omits the spelling and magnitude.
The value cannot outlive its source/parsed-node view or be retargeted by field mutation. Its span
includes grouping parentheses, matching the syntax view; source locations are not canonical identity.

## Exact conversion order

A bare integer is a count. A bare decimal is a ratio in parts per million, even when its fraction
contains only zeroes. Length factors are 1 for um, 1,000 for mm, 10,000 for cm, 1,000,000 for m and 25,400 for in.
Degree inputs multiply
by 1,000,000, and pct inputs multiply by 10,000 to obtain parts per million. The original spelling survives.

1. Interpret the unsigned decimal and unit multiplier exactly.
2. Reduce the converted rational and require each component to fit 128 magnitude bits.
3. Use the [shared unsigned rounding primitive](numeric-rounding.md) once, half away from zero.
4. Check the rounded length input against 1,000,000,000 micrometres. Count, ratio and raw angle retain
   the full 128-bit magnitude; signed binding width is a later boundary.

A value below half a quantum becomes a canonical zero only if its exact pre-round rational fits.
Thus rounding cannot rescue an excessive denominator. Conversely, 1000000000.4 um is allowed because
its once-rounded length is at the scalar bound, while 1000000000.5 um refuses. Formula angles retain
complete turns: 360 deg is 360,000,000 microdegrees, and 720 deg is 720,000,000. There is no direction modulo.
A raw mantissa wider than 128 bits may be valid after reduction; it is not parsed prematurely as i128.

## Bounded decimal workspace

The converter borrows the already validated ASCII spelling. It virtually removes leading zeroes and
only fractional trailing zeroes; an all-zero spelling reduces to 0/1 regardless of its length. This
preserves count/ratio kind and integer trailing zeroes.

For a nonzero fractional mantissa after trimming, at least one of factors 2/5 is absent. Every closed
unit multiplier contains at most six of either factor. A fractional scale greater than 134 therefore
proves an excessive denominator. More than scale + 39 significant digits implies a value at least 10^39,
so the reduced numerator exceeds u128 MAX. These are mathematical refusal witnesses for the existing
128-bit rule, not added spelling limits. Arbitrarily long leading/trailing zeroes remain accepted.

The remaining mantissa needs at most 173 decimal digits of temporary storage. Exact long division by
2 and 5 cancels denominator factors against the multiplier and mantissa. Checked u128 construction
then establishes the reduced numerator/denominator before rounding. No floating point, new dependency
or input-sized allocation is used. Work scans the borrowed spelling and then operates on bounded digits.

Rational-width errors name max_rational_bits, bound 128, the excessive component and an explicit
measured lower bound of at least 129 bits. They do not pretend to compute an exact bit count for a huge
input. Length errors report the actual rounded value and maximum. Low-level spans do not supply
statement/canonical command context; that belongs to future recipe/command layers.

## Verification and scope

From the repository root:

```bash
cargo test -p sc-core --test formula_literal_contract
python3 -I -B docs/tasks/artifacts/formula_structure/literal_normalization_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/literal_normalization_mutations.py
```

The initial five public contracts consume 100 independently authored Fraction fixture rows covering unit/kind/
quantum/width/scalar boundaries and wide cancellable mantissas. Additional source/privacy/unary and
100,000-digit zero/padding/refusal cases exercise the public node API. Two compile-fail doctests retain
private construction and source lifetime. The structural suite watches the independent fixture oracle.
Actual compiled conversion, reduction, width, kind, direction, narrowing, domain, diagnostic and source
faults must fail assertions in failed-test bodies. The direct entry uses the
[producer profile](../build-and-checks.md#direct-python-producers) before source reads/output writes,
retains prepared child stores and rebuilds the complete focused target after exact restoration.
Classifier-only mode checks anchors and refuses passing-name/expect/compiler noise without compiling
a fault. Standing structural controls and calibrated actual classifier faults watch this boundary.

G1-SLICE.5a.3c.2 owns this API. This API converts one literal; the separate whole-arena API below converts all literal inputs.
Neither checks operators/names, folds sign, binds values, evaluates or constructs geometry.
The separate canonical serializer below emits normalized expression bytes without execution. Strict native checks and real WASM cross-compilation verify their stated scope; cross-
compilation alone is not a browser runtime or cross-platform numeric certificate.

## Normalize every literal in an expression

FormulaExpression.normalize_literals builds a separate FormulaNormalizedExpression in one flat pass.
The syntax arena stays unchanged and may be reused. The new arena owns its node and call-edge storage
while borrowing original names and literal spellings; it remains usable after the syntax arena is dropped.

```rust
use sc_core::recipe::{FormulaExpression, FormulaNormalizedNodeKind};

let syntax = FormulaExpression::parse("-720 deg + turn_allowance")?;
let normalized = syntax.normalize_literals()?;
drop(syntax);
assert_eq!(normalized.node_count(), 4);
assert_eq!(normalized.conditional_depth(), 0);
if let FormulaNormalizedNodeKind::Binary { left, right, .. } = normalized.root().kind() {
    assert!(matches!(left.kind(), FormulaNormalizedNodeKind::Negate(_)));
    assert!(matches!(right.kind(), FormulaNormalizedNodeKind::Name("turn_allowance")));
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

| Public API | Contract |
| --- | --- |
| FormulaNormalizedExpression | Immutable, privately constructed flat arena; root/count/conditional depth; source-borrowing clone |
| FormulaNormalizedNode | Read-only source span and kind; children belong to the same arena |
| FormulaNormalizedNodeKind | Typed literal, borrowed name, unary minus, square, binary operator, ordered call or three-part conditional |
| FormulaNormalizedArguments | Ordered, exact-size, fused iterator; cloning preserves its current position |

Every semantic node, operator, name, ordered child, source span and depth survives. Literals use the
individual conversion above. Unary minus stays a node; the positive 2^63 angle child is retained.
Grouping changes spans rather than node counts, and a square's exponent is operator payload.
Root/child indices are private; callers cannot forge stored arena handles or retarget its edges. Views cannot
outlive their normalized arena. Debug of the arena, node view and argument iterator omits customer
source. Explicit kind inspection exposes borrowed names and literal read access deliberately.

All literals are converted, including every call argument and both conditional branches. An invalid
literal in an untaken branch still refuses input normalization. The first refused literal in arena
construction order returns its original FormulaLiteralError/span; no partial result is published.
Repeated attempts leave syntax unchanged and return the same refusal.

This stage does not validate operator dimensions or names and does not run arithmetic. For example,
an unknown function, unary minus on a count, mixed length/angle addition and 1/0 remain inspectable
normalized syntax for later validators. A valid wide literal followed by an overflowing addition is
retained without executing that addition. Successful normalization is not permission to execute a recipe.

## Whole-arena proof

```bash
cargo test -p sc-core --test formula_normalized_contract
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_expression_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_expression_mutations.py
```

Eight public contracts cover 24 authored shape/count/depth rows checked by the independent book
reference, all 100 literal rows nested in calls, all 25 worked binding/assertion expressions, immutable
source spans/borrowing/clone/drop, every literal position's atomic refusal, unevaluated syntax,
argument order/size/fusion and privacy. Three compile-fail doctests enforce private construction,
source lifetime and view lifetime; a runnable doctest demonstrates independence from syntax storage.

On a 64 KiB stack, conversion/clone/drop handles 50,000 grouping pairs, 256 unary/call nodes and 16 nested
conditional levels. Syntax at 257 nodes or 17 levels still refuses. The flat normalizer preserves the
existing bounds; it adds no new cap or recursive traversal. Call-edge storage stays bounded by the
validated syntax graph, and per-literal decimal workspace retains the bound described above.

Seventeen actual compiled faults alter root/name/unary/square/operator identity, child/branch order,
call coverage, depth/span, literal unit/refusal, iterator behavior or Debug privacy. They must fail
assertions in failed-test bodies; the direct entry validates source/output before reads/writes,
preserves prepared child stores and rebuilds the full focused target after exact restoration.
Classifier-only mode checks anchors and failed-body/noise classification without compiling a fault.
The structural suite watches the
independent shape verifier. G1-SLICE.5a.3c.3 owns this whole-arena stage; .4 completes its coupled review below.
Canonical S-expression serialization is implemented separately below;
[ordered statement/recipe identity](formula-recipe-inputs.md) is also available.
Name/kind checks are implemented separately. Numerical binding/evaluation, geometry and command/API/MCP
integration remain later work. Native/release/WASM checks retain their stated scope.

## Coupled normalization review

G1-SLICE.5a.3c.4 closes the scoped production normalization prerequisites. It checks the
[language's input and structural rules](../spec/formula-language.md#42-exact-arithmetic-and-the-two-places-a-value-rounds),
[literal/unit/canonical rules](../spec/formula-language/grammar.md#2-literals-and-their-units),
[shared rounding](numeric-rounding.md), and the D95/D84 width/angle rulings against actual public APIs.

| Obligation | Current product evidence |
| --- | --- |
| Closed spelling/unit vocabulary and separate unary syntax | Existing lexical/expression contracts and privacy/lifetime docs |
| Integer count versus decimal/percent ratio; exact unit factors | Original 100 independent Fraction rows and individual/nested public tests |
| Reduced converted rational width before input rounding | Width/cancellable-mantissa controls; new 176 reduction-frontier rows below |
| One input rounding; sub-quantum zeros; scalar length after rounding | Public literal/Fraction controls and shared unsigned/signed rounding contracts |
| Positive 128-bit child before signed binding; raw angle turns | Wide/unary/turn shape controls; bindings remain a separate future boundary |
| Source spelling/unit/span/kind and all ordered structure | 24 independent shape rows, 25 book expressions, public source/iterator/clone tests |
| Every call argument and both branches; atomic located refusal | Nested 100-row controls and all-position refusal/repeat tests |
| Private storage and source/arena lifetimes; text-free Debug | Compile-fail doctests and public source/privacy controls |
| Existing 256-node/16-if bounds; flat conversion/clone/drop | Boundary/50,000-grouping controls on a 64 KiB stack |
| Existing bounds reject no valid long input | Fraction reduction-frontier controls and bounded-workspace witnesses |

The reduction-frontier producer checks all seven unit multipliers and bare decimal ratios. Powers
of 2 at fractional scales 54–63 and powers of 5 at scales 125–136 isolate the two uncancelled
prime factors. Fraction computes each exact reduced value independently of decimal long division.
Of 176 rows, 103 are valid canonical zeros and 73 exceed the denominator width. The new public
contract checks each row individually and inside a call in a conditional's else branch, including
original error spans and repeated refusal. The angle cases accept scale 133 and refuse scale 134.

The producer also verifies the actual published unit factors, maximum prime valuations of six,
39 digits for u128 MAX, and the conservative scale/workspace witnesses 134/173. Four compiled
actual faults prematurely restrict scale/raw digits or remove mantissa/unit cancellation; each must
fail this new public assertion oracle. The runner restores exact production source. Existing
rounding, individual-conversion and arena mutation families retain their separate proof scopes.

```bash
cargo test -p sc-core --test formula_literal_contract coupled_reduction_frontier_preserves_valid_inputs_and_located_width_refusals -- --exact
python3 -I -B docs/tasks/artifacts/formula_structure/reduction_boundary_reference.py
bash docs/tasks/artifacts/formula_structure/run_reduction_boundary_mutations.sh
```

The structural suite watches the new independent producer. Production rounding, conversion and
arena sources remain byte-identical to G1-0058/G1-0059/G1-0060. Six current individual-literal
contracts include the original five plus this coupled check; the eight arena contracts are unchanged.
These controls and the bounded arithmetic argument establish the stated normalization scope.
They provide no correctly rounded arbitrary transcendental or cross-platform numerical certificate.

Canonical serialization is [G1-SLICE.5a.3d.2](#serialize-canonical-expression-identity), with coupled
identity review .3d.3 complete below; [ordered statement identity](formula-recipe-inputs.md) is available.
Names/types, numeric binding/evaluation,
entity direction integration, geometry, storage and command/API/MCP execution remain later work.
A normalized graph contains literal inputs and unevaluated operators; it is not an executable recipe.

## Canonical expression bytes

Before implementing the canonical serializer, G1-SLICE.5a.3d.1 found D103: grammar §4 previously did not
spell unary-minus or square nodes byte for byte. The reference models them as tuples; the normalized
shape fixtures are inspection labels. Those labels do not select a persistent interchange format.

The director settled D103 on 2026-10-02: unary (- child), square (^2 child). The complete mapping is:

| Role | Canonical bytes |
| --- | --- |
| Literal | kind:unsigned-decimal-integer, using the existing normalized magnitude |
| Name | bare lower-snake identifier |
| Unary minus | (- child) |
| Fixed square | (^2 child) |
| Binary operation | (symbol left right), using +, -, *, /, ==, !=, <, <=, > or >= |
| Ordinary call | (function arg1 arg2 ...), retaining every argument in order |
| Conditional | (if condition then else), retaining all branches |

One ASCII space separates parts; there is no terminal newline or extra padding. Names and integer
magnitudes are explicit customer data, while source spellings, unit aliases and grouping spans stay
outside identity. Canonicalization does not simplify signs, reorder children, evaluate arithmetic,
validate names/types or discard a branch. A serialized expression is not an executable recipe.

```text
-0 deg                  => (- angle:0)
--0 deg                 => (- (- angle:0))
-2.5 cm ^ 2             => (- (^2 length:25000))
(2.5 cm + 25 mm) ^ 2    => (^2 (+ length:25000 length:25000))
if(a, 360 deg, -720 deg) => (if a angle:360000000 (- angle:720000000))
probe(-1 um, 1 um ^ 2)  => (probe (- length:1) (^2 length:1))
```

The unselected alternative fixed-square spelling was (^ child count:2). Its exponent is fixed operator payload,
not another semantic node. The ruling chooses the shorter ^2 opcode to retain the one-child shape.
Both symbolic forms distinguish the operator from an ordinary call. Named neg/square tags would
collide with neg(1 um)/square(1 um): these names are not reserved, and parsing/normalization retains
them as ordinary calls before later function validation. Unary (- child) differs from binary
subtraction by arity. Neither choice changes the authored machine syntax or numeric semantics.

The tracked canonical_contract_inventory.py checks seven reference node roles, all ten binary symbols,
both distinct named-call examples and all six authored byte examples above. Its recursive renderer
uses the independent book reference, never a product arena. Four actual renderer tag/branch/argument
faults must fail those byte assertions; the exclusive runner restores the producer byte-identically.
These are interpreter assertion controls, not compiled product serializer proof. The structural suite
watches the inventory. The detailed repository
decision is docs/decisions/decision_canonical-expression-spelling.md. D103 closes for the missing
byte contract; the .3d.2 product serializer and .3d.3 identity review are complete below.
[Ordered statement/recipe identity](formula-recipe-inputs.md) is implemented separately.
Hashes, project persistence and execution remain future work.


```bash
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_contract_inventory.py
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_contract_inventory_mutations.py
```


## Serialize canonical expression identity

FormulaNormalizedExpression.canonical_form produces a privately constructed FormulaCanonicalExpression.
The result owns its ASCII bytes and outlives source, syntax and normalized arenas. Explicit as_str
reads those bytes; into_string transfers them into an ordinary owned string. Clone preserves identity;
Eq compares exact bytes, rather than evaluated values. Debug shows only byte count, omitting customer
names and literal magnitudes. Canonical content is explicit customer data when accessed deliberately.

```rust
use sc_core::recipe::FormulaExpression;

let canonical = {
    let source = String::from("-2.5 cm ^ 2");
    let syntax = FormulaExpression::parse(&source)?;
    let normalized = syntax.normalize_literals()?;
    normalized.canonical_form()
};
assert_eq!(canonical.as_str(), "(- (^2 length:25000))");
let equivalent = FormulaExpression::parse("-25 mm ^ 2")?.normalize_literals()?.canonical_form();
assert_eq!(canonical, equivalent);
# Ok::<(), Box<dyn std::error::Error>>(())
```

The serializer implements the exact expression mapping above, without another input conversion,
rounding or arithmetic step. A flat heap action stack emits nodes in source child order. It retains
full u128 magnitudes, every ordinary argument and all three conditional children. One ASCII space
separates parts; output has no comments, grouping spans, unit aliases, padding or terminal newline.
There is no recursive traversal and no added numerical/input cap. Memory holds the output bytes and
an action stack bounded by the validated expression's existing node/edge limits. Large valid names
require corresponding output storage; the serializer does not silently truncate or reject them.

Unary and square operators remain distinct from calls named neg/square. Zero/sign/kind, comparison
symbols, call order and branch order remain part of identity. No algebraic simplification occurs:
a + 0 differs from a. Unknown functions, incompatible dimensions, negative counts and division by
zero can serialize as syntax; later recipe validators still own their refusal. Serialization does not
certify a valid or executable recipe and produces no geometry, command, project envelope or hash.
FormulaExpression.parse reads the authored machine syntax, such as -x or x ^ 2; canonical bytes
are identity output, not an additional input syntax accepted by that parser. Canonical decoding and
project persistence are not supplied by this API.

## Canonical serializer proof

```bash
cargo test -p sc-core --test formula_canonical_contract
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_expression_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_expression_mutations.py
```

The initial .3d.2 seven public contracts compare 55 authored byte fixtures with the independent recursive book-reference
renderer, cover all 100 Fraction literal inputs/refusals nested in calls, and verify the six canonical
examples above. They check unit aliases/Eq versus unequal kind/sign/operator/order, owned lifetimes,
clone/extraction/privacy and unevaluated branches. Private construction has a compile-fail doctest;
a runnable doctest demonstrates the public API. The structural suite watches the byte fixture oracle.

On a 64 KiB stack, parsing/normalization/serialization handles 256-node unary and call chains, 16 if
levels, 50,000 grouping pairs and a 100,000-byte identifier. Existing node257/if17 refusals stay intact.
Nineteen actual compiled faults alter root, kind/magnitude/full width, name/sign/square/symbols,
ordered binary/call/conditional children, call coverage, whitespace/newline or Debug privacy. They
must fail public assertions in failed-test bodies. Reported panic paths, lines and columns must
match assertion macro sites in the current focused test source; custom assert messages are valid.
Passing names, summaries and assertion-labelled expect messages cannot supply proof. The entry
validates both implementation and test-source paths/output before reads/writes and
preserves prepared child stores. Its exclusive runner restores exact source and rebuilds the full
focused target, including after coupled selection. Run mutations alone.

G1-SLICE.5a.3d.2 owns this product serializer; .3d.3 completes the coupled identity review below.
[Ordered recipe normalization/statement identity](formula-recipe-inputs.md) is now available.
Name/kind checks are implemented separately. Numerical binding/evaluation, geometry, storage and
command/API/MCP remain later work. [Ordered statements](formula-statements.md#parse-an-ordered-recipe)
have a separate syntax API. Native/release checks and WASM cross-compilation retain their stated scope; compilation
alone is not a browser runtime or cross-platform numerical certificate.


## Coupled canonical identity review

G1-SLICE.5a.3d.3 completes the expression identity review against grammar §4/§5 and D84/D95/D103.
Every expression requirement maps to a public control; this closes expression serialization only.
The source for the serializer, normalized arena, literals and rounding remains byte-identical to
its verified implementation commit. The additional controls exercise the coupled public pipeline.

| Obligation | Public proof |
| --- | --- |
| Seven expression roles, ten binary symbols and exact ASCII spacing | Authored55 bytes versus independent recursive book-reference rendering; public fixture assertions |
| D103 unary/square identity, precedence, kind/sign/zero/order distinctions | Authored bytes and unequal identity contracts; operator/call names remain distinct |
| D84 raw sign/turns and D95 full128 literal magnitudes | Wide byte fixtures and nested100 Fraction acceptance/refusal rows; no later binding claim |
| Input respellings and source spans outside identity | Root aliases and nested conditional/call aliases including ASCII whitespace; signed turns/kinds still distinguish bytes |
| Complete worked-example population | Authored25 byte rows: seventeen binding expressions and eight assertion sides; source population and independent reference both agree |
| Every child at legal width/depth, no extra truncation | Distinct255-argument call,256-node chains,16 if levels,50000 groups and100000-byte name on64KiB stack |
| Owned result, private construction, Eq/Clone/extraction/privacy | Public lifetime and Debug assertions plus compile-fail/runnable Rust docs |
| Unevaluated identity | Unknown calls/branches, mismatched dimensions, negative count and zero division serialize without granting recipe acceptance |

The canonical suite now has nine public tests. Three additional actual compiled faults (worked
addition symbol, wide-call order and loss of the last legal argument) fail the two new coupled
tests as assertions. The exclusive runner restores exact source and rebuilds all nine tests. The initial nineteen fault
controls remain retained proof of the unchanged serializer; the three new reds verify the additional
controls, rather than claiming a fresh execution of the initial nineteen.

```bash
cargo test -p sc-core --test formula_canonical_contract
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_expression_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_expression_mutations.py --coupled
```

The reference producer watches exact worked-source membership as well as bytes. Native/release tests
and three-crate WASM compilation pass within their existing scope. Statement bind/assert bytes,
complete recipe input normalization, name/type checks, bound numeric values, evaluation, geometry, storage and
command/API/MCP remain owned by later leaves. Expressions that serialize can still be refused there.
