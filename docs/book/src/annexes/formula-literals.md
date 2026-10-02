# Annex: formula literal normalization

The current sc-core recipe API can convert an individual parsed literal into its typed canonical
integer. Parsing and input conversion remain separate operations: parsing checks syntax;
conversion checks exact rational width, rounds once and checks the input scalar domain.
Whole normalized expressions, canonical serialization, binding and evaluation remain later work.
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
contains only zeroes. Length factors are um1/mm1000/cm10000/m1000000/in25400; degree inputs multiply
by1000000, and pct inputs multiply by10000 to obtain parts per million. The original spelling survives.

1. Interpret the unsigned decimal and unit multiplier exactly.
2. Reduce the converted rational and require each component to fit128 magnitude bits.
3. Use the [shared unsigned rounding primitive](numeric-rounding.md) once, half away from zero.
4. Check the rounded length input against1000000000 micrometres. Count, ratio and raw angle retain
   the full128-bit magnitude; signed binding width is a later boundary.

A value below half a quantum becomes a canonical zero only if its exact pre-round rational fits.
Thus rounding cannot rescue an excessive denominator. Conversely,1000000000.4 um is allowed because
its once-rounded length is at the scalar bound, while1000000000.5 um refuses. Formula angles retain
complete turns:360 deg is360000000 microdegrees, and720 deg is720000000. There is no direction modulo.
A raw mantissa wider than128 bits may be valid after reduction; it is not parsed prematurely as i128.

## Bounded decimal workspace

The converter borrows the already validated ASCII spelling. It virtually removes leading zeroes and
only fractional trailing zeroes; an all-zero spelling reduces to0/1 regardless of its length. This
preserves count/ratio kind and integer trailing zeroes.

For a nonzero fractional mantissa after trimming, at least one of factors2/5 is absent. Every closed
unit multiplier contains at most six of either factor. A fractional scale greater than134 therefore
proves an excessive denominator. More than scale+39 significant digits implies a value at least10^39,
so the reduced numerator exceeds u128 MAX. These are mathematical refusal witnesses for the existing
128-bit rule, not added spelling limits. Arbitrarily long leading/trailing zeroes remain accepted.

The remaining mantissa needs at most173 decimal digits of temporary storage. Exact long division by
2 and5 cancels denominator factors against the multiplier and mantissa. Checked u128 construction
then establishes the reduced numerator/denominator before rounding. No floating point, new dependency
or input-sized allocation is used. Work scans the borrowed spelling and then operates on bounded digits.

Rational-width errors name max_rational_bits, bound128, the excessive component and an explicit
measured lower bound of at least129 bits. They do not pretend to compute an exact bit count for a huge
input. Length errors report the actual rounded value and maximum. Low-level spans do not supply
statement/canonical command context; that belongs to future recipe/command layers.

## Verification and scope

From the repository root:

```bash
cargo test -p sc-core --test formula_literal_contract
python3 -I -B docs/tasks/artifacts/formula_structure/literal_normalization_reference.py
bash docs/tasks/artifacts/formula_structure/run_literal_normalization_mutations.sh
```

Five public contracts consume100 independently authored Fraction fixture rows covering unit/kind/
quantum/width/scalar boundaries and wide cancellable mantissas. Additional source/privacy/unary and
100000-digit zero/padding/refusal cases exercise the public node API. Two compile-fail doctests retain
private construction and source lifetime. The structural suite watches the independent fixture oracle.
Actual compiled conversion, reduction, width, kind, direction, narrowing, domain, diagnostic and source
faults must fail public assertions; the exclusive runner restores source byte-identically.

G1-SLICE.5a.3c.2 owns this API. This is single-literal input conversion; it does not normalize a whole
arena, check operators/names, fold sign, bind values, evaluate, serialize canonical bytes or construct
geometry. Strict native checks and real WASM cross-compilation verify their stated scope; cross-
compilation alone is not a browser runtime or cross-platform numeric certificate.
