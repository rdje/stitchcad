# Coupled irrational function review

The root,trigonometric and inverse kernels are implemented. This chapter tracks their coupled
rounding,error and provenance review under G1-SLICE.5d.4. Each function keeps its own numerical
rounding boundary;an algebraically equivalent expression need not have the same rounded result.
Scoped fixtures do not grant platform identity,full resource bounds or production approval.

| Family | Available implementation | Coupled review boundary |
| --- | --- | --- |
| sqrt/hypot | Exact integer roots and sourced calls | Admitted bounds and85 independent rows below;actual body faults/platform remain |
| arc_length | Signed/full-turn fixed-pi certified rounding | Coupled source/error/fault review remains .5d.4.arc |
| sin/cos | Exact folding and complete forward threshold certificate | Coupled actual-body/source/platform review remains .5d.4.forward |
| atan/atan2 | Principal branches and complete inverse threshold certificate | Coupled actual-body/source/platform review remains .5d.4.inverse |
| tan | Explicit per-call quotas,native139 and real paired24 acceptance | Coupled adaptive/error/fault review remains .5d.4.tangent |

The [canonical numerical contract](formula-rationals.md)
records the algorithms. [Tangent resources](formula-trig-certificates.md) retain
separate request admission. Original forward/inverse certificate stores are protected evidence;
this review reuses them read-only and never reruns an old installation transaction.

## Exact root boundaries

For a nonnegative radicand N/D,the native root is floor(sqrt(floor(N/D))) followed by the exact
midpoint comparison4N>=D(2k+1)^2. Python math.isqrt supplies an independent integer root algorithm;
Fraction compares the exact half-square and its two-sided neighbors. Both avoid floating point.
The existing all-admitted argument bounds area/Ratio/hypot bisection by30/74/31 iterations.
Hypot may require513/512 raw numerator/denominator bits and a576-bit midpoint product;this is
private function arithmetic,not an intermediate Area value or another language limit.

The extended root fixture preserves every byte of its original25-row prefix and adds60 rows.
All85 cases cover51 square roots and34 hypotenuses:exact ties,neighbors beyond float precision,
full128-bit Ratio components,wide positive Ratio results,negative roots,signed Length operands,
final Length-domain refusal and all four signs of a513-bit private hypot expansion.
The exact large Ratio half-square uses odd m=2^63-1 and input(m^2+delta)/256 for delta=-1,0,1.
After the Ratio scale,its root is adjacent to125m/2,well beyond signed binding capacity.
Area/Ratio neighbors at several scales include an exact rounded maximum Length;hypot with a
maximum Length and a tiny full-width second operand must not fail an intermediate Area bound.

The actual sourced watcher runs every row through real upstream cos/sin zero arithmetic,then the
actual sqrt/hypot calls. No contribution flags or computed kernel outputs are supplied. Successful
zero,perfect and rounded roots keep every executed upstream/root call contribution. T1 rejects
these contributions even when the final value is exact;actual binding/read retains the integer
and sources. All three original mathematical refusals stay typed,with their original input or
rounded final-domain witness. Five wider successful Ratio roots fail only at signed binding.

```rust
use sc_core::{name::MachineToken,recipe::{FormulaEvaluatedScalar as V,FormulaNumericValue as N,
    FormulaKind as K,FormulaRational as R,FormulaBuiltin as B}};
let input=V::from_numeric(N::new(K::Ratio,R::new(u128::MAX,1,false)?)?);
let root=input.checked_sqrt()?;
assert!(root.sources().contains(B::Sqrt));
assert!(root.numeric().expect("actual Ratio").rational().numerator_magnitude()>i64::MAX as u128);
let name=MachineToken::new("wide_root")?;
assert!(root.bind(&name).is_err());
# Ok::<(),Box<dyn std::error::Error>>(())
```

Six native tests check every numeric row,actual wrong-kind priority,the sourced/storage cases
and complete independent regeneration of6476 frozen bytes. The fixture SHA256 is
e4b701eb22add56a126b980cced076b4b694223e71bebcdfa650f17d2145765d.
The producer reads no Rust,native result or frozen expected fixture. Its explicit assertions
verify the513/512-bit witness and the original128-bit components before emitting rows.
These finite checks support the existing exact all-input argument;they are not exhaustive input
coverage. Actual implementation-body faults are the next root review owner .5d.4.roots.faults.
Mixed-call chains,cross-platform execution and complete coupled closure remain separate tasks.
