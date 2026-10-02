# Annex: exact rounding of wide magnitudes

The [numerical contract](../spec/units-and-tolerances.md#23-public-rounding-endpoints) has two public
rounding entry points in sc-units. They share one private magnitude rule, then apply their own result
and diagnostic contracts. This is an implemented numeric primitive, not formula literal conversion,
binding or evaluation.

| Entry point in sc_units::round | Inputs | Result | Boundary |
| --- | --- | --- | --- |
| div_round_half_away_from_zero | signed i128 numerator/denominator | signed i64 or UnitError | both i64 endpoints; zero and overflow refuse |
| div_round_half_away_from_zero_unsigned | unsigned u128 numerator/denominator | unsigned u128 or UnitError | full 128-bit magnitude; zero refuses |

The wide entry point keeps a positive literal child wider than signed storage. For example, the
positive magnitude 2^63 beneath unary minus can survive rounding before a later binding stores
signed i64 MIN. A magnitude round imposes no quantity scalar domain or binding width; the consumer
must enforce those separately. Direction normalization is also separate.

## Arithmetic and totality

For a nonzero denominator *d*, integer division gives quotient *q* and remainder *r* with 0 ≤ *r* < *d*.
The half-away rule increments the magnitude exactly when *r* ≥ *d* − *r*. The subtraction always
fits. Doubling *r* would overflow for some valid full-u128 inputs, so neither entry point uses that
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

Nine compiled actual debug faults must fail public assertions: output/numerator/denominator
narrowing, doubled remainder, missing half tie/increment, zero guard, unsigned operation and signed
forwarding context. The doubled-remainder fault also fails a compiled release assertion, exposing
wrapped arithmetic rather than relying only on debug overflow panic. Five existing signed guard
mutations still discriminate. Every runner restores production source byte-identically; run them
exclusively without other builds, probes or gates.

The structural suite watches the independent Decimal fixture verifier. Strict native tests and real
WASM cross-compilation verify their stated scope; WASM compilation is not a runtime numerical
certificate. G1-SLICE.5a.3c.1 owns this primitive. Exact typed literal conversion/normalization,
canonical serialization, binding/evaluation, geometry and API/MCP integration remain separate work.
