# Annex: exact rounding of wide magnitudes

The [numerical contract](../spec/units-and-tolerances.md#23-public-rounding-endpoints) has three public
rounding entry points in sc-units. They share one private magnitude decision, then apply their own result
and diagnostic contracts. This is an implemented numeric primitive, not formula literal conversion,
binding or evaluation.

| Entry point in sc_units::round | Inputs | Result | Boundary |
| --- | --- | --- | --- |
| div_round_half_away_from_zero | signed i128 numerator/denominator | signed i64 or UnitError | both i64 endpoints; zero and overflow refuse |
| div_round_half_away_from_zero_unsigned | unsigned u128 numerator/denominator | unsigned u128 or UnitError | full 128-bit magnitude; zero refuses |
| round_magnitude_from_half_comparison | unsigned integer floor and exact fractional-half comparison | unsigned u128 or UnitError | required increment beyond u128 refuses |

The wide entry point keeps a positive literal child wider than signed storage. For example, the
positive magnitude 2^63 beneath unary minus can survive rounding before a later binding stores
signed i64 MIN. A magnitude round imposes no quantity scalar domain or binding width; the consumer
must enforce those separately. Direction normalization is also separate.

## Arithmetic and totality

For a nonzero denominator *d*, integer division gives quotient *q* and remainder *r* with 0 ≤ *r* < *d*.
The half-away rule increments the magnitude exactly when *r* ≥ *d* − *r*. The subtraction always
fits. Doubling *r* would overflow for some valid full-u128 inputs, so the ratio entry points avoid that
predicate. If *d* = 1 there is no remainder; if *d* ≥ 2 the quotient has room for an increment. A
checked increment preserves diagnostic totality without wrapping, saturating or narrowing.

The signed entry point reconstructs sign only after checking its i64 result. Its negative endpoint
has one extra unit of magnitude and remains explicitly valid. A zero denominator retains the public
entry point's operation name through the shared helper; signed overflow retains the signed API name.

## Independent controls

From the repository root:

```bash
cargo test -p sc-units --test unsigned_round_contract --test round_contract
cargo test -p sc-units --release --test unsigned_round_contract --test round_contract
python3 -I -B docs/tasks/artifacts/formula_structure/unsigned_round_reference.py
bash docs/tasks/artifacts/formula_structure/run_unsigned_round_mutations.sh
bash docs/tasks/artifacts/formula_structure/run_round_mutations.sh
```

Five new public contracts cover full-width inputs/high remainders, even/odd half neighborhoods,
zero-operation context, the signed bridge and 138 independent fixture rows. The fixture producer/
verifier uses Decimal with 120 significant digits and half-up rounding rather than the production
quotient/remainder algorithm. Its precision exceeds 39 integer digits plus the smallest nonzero
distance from a half-integer, at least 1/(2*d) for d ≤ 2^128−1; exact ties are terminating halves.
The existing four signed public contracts and 36 independent Fraction rows remain required.

The prior implementation had nine compiled actual debug faults fail public assertions: output/numerator/denominator
narrowing, doubled remainder, missing half tie/increment, zero guard, unsigned operation and signed
forwarding context. The doubled-remainder fault also fails a compiled release assertion, exposing
wrapped arithmetic rather than relying only on debug overflow panic. Five existing signed guard
mutations discriminated at that checkpoint. Every runner restores production source byte-identically; run them
exclusively without other builds, probes or gates.

Both mutation entries prepare the [local producer profile](../build-and-checks.md#direct-python-producers)
and validate their declared source/output paths before writing or compiling. Signed rounding now
plans all five numbered logs and restored.log, its mutable implementation and read-only current
test/Fraction fixture before reads. Complete dispatch captures, five physical refusals and actual
file/fixture/site omissions are verified separately from native outcomes.

Its totality helper explicitly asserts that the public call did not unwind before extracting the
result. A labelled expect message alone is insufficient mutation evidence. Five real faults must
fail at current guarded assertion macro byte locations, including that helper. Ten selected build/
source inputs stay frozen; all four public tests and 36 fixture rows remain intact apart from the
helper repair. Public Cargo output selects the restored four-test artifact.

Unsigned rounding also plans its nine numbered logs, release-remainder.log, restored-debug.log
and restored-release.log before reads. Its mutable implementation, current test and imported
138-row Decimal fixture are guarded inputs. The totality helper uses an explicit assertion.
Independent captures verify all twelve children/files, including the extra release call while the
doubled-remainder fault is still applied and both restorations afterward. Physical aliases, missing
inputs and actual file/profile/helper omissions are refused independently of native outcomes.
Earlier execution observed nine debug faults and the extra release fault fail at assertion macro sites.
Ten selected inputs remain frozen, the original CALL-derived cases stay exact and both public
Cargo-selected restored artifacts execute all five contracts. Production arithmetic is unchanged.

The structural suite watches the independent Decimal fixture verifier. Strict native tests and real
WASM cross-compilation verify their stated scope; WASM compilation is not a runtime numerical
certificate. G1-SLICE.5a.3c.1 owns this primitive. [Individual literal conversion](formula-literals.md) is implemented under .5a.3c.2. Whole-expression literal
normalization is implemented under .5a.3c.3; serialization, binding/evaluation, geometry and API/MCP remain later work.

## Exact decomposed magnitude input

| API token | Meaning |
| --- | --- |
| `round_magnitude_from_half_comparison` | Shared magnitude rounding from an integer floor and exact fractional-half comparison |
| `Ordering` | Standard Rust comparison result supplied by exact arithmetic |
| `Less` | Fractional part below one half,including an exact integer |
| `Equal` | Fractional part exactly one half |
| `Greater` | Fractional part above one half |

A wide exact ratio need not fit u128 to use the one rounding rule. Its consumer derives the
nonnegative integer floor and compares the remainder with denominator minus remainder in its own
admitted workspace. The new entry point accepts that comparison and a u128 floor. It neither
narrows the original ratio nor reinterprets the comparison as approximate evidence. The consumer
owns the exact derivation,nonzero denominator,workspace admission,sign and final scalar domain.
The rounding function cannot validate information that was not supplied to it.

```rust
use core::cmp::Ordering;
use sc_units::round::round_magnitude_from_half_comparison as round;
assert_eq!(round(0, Ordering::Equal)?, 1);
assert_eq!(round(4, Ordering::Less)?, 4);
assert_eq!(round(4, Ordering::Greater)?, 5);
assert_eq!(round(u128::MAX, Ordering::Less)?, u128::MAX);
# Ok::<(), sc_units::UnitError>(())
```

An exact half or above-half fractional part increments the floor;otherwise it remains unchanged.
At u128 maximum,a required increment returns Overflow with the new function's operation name.
There is no division-by-zero variant for this decomposed input;no quotient exists for such an
original ratio,so its consumer must refuse before calling. Existing signed/unsigned ratio entry
points derive their comparisons internally and preserve their original values and error names.
All three delegate to the same private increment decision. The crate remains dependency-free.

Focused native checks pass11 contracts:2 new decomposed-input contracts and the existing4 signed/
5 unsigned contracts. They include125 nonzero independent Decimal rows plus explicit full-width
floor/half/overflow cases;the full138-row Decimal producer also passes. The source doctest passes.
The old mutation drivers retain their fault families/dispatch/restoration rules with updated exact
remainder/tie anchors;read-only census finds all5+9 anchors uniquely. Compiled mutation execution
for this refactor is deferred to full paired acceptance or required pre-push probes;earlier mutation
results above are not a verdict for the refactored source. No weighted quantity execution follows
from this primitive alone.
