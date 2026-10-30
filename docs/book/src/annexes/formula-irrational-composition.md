# Mixed irrational source and recipe execution

The [per-function review](formula-irrational-review.md) establishes each numerical boundary.
This chapter checks what happens when those functions execute together through real source,
recipe and containing Design inputs. Each call finishes its own rounding before its parent
uses the result. Source caching,recipe binding and later reads preserve those boundaries.

## A quotient of rounded components has a different answer

```text
let direct:ratio=tan(30 deg)
let components:ratio=sin(30 deg)/cos(30 deg)
let different:boolean=direct!=components
```

| Binding | Stored value | Successful contributions |
| --- | --- | --- |
| direct | 0.577350 | tan |
| components | 0.577351 | sin,cos |
| different | true | sin,cos,tan |

Tangent rounds the whole quotient of unrounded sine/cosine enclosures. The second expression
uses sine500000 ppm and cosine866025 ppm,which have already rounded separately. Their quotient
is exactly20000000000/34641 ppm before the binding stores577351 ppm. Both results obey their
own contracts;substituting one expression for the other changes the calculation.

## Inverse calls keep the rounding of their argument

```text
let a:angle=atan(tan(30 deg))
let b:ratio=tan(a)
```

The first tangent returns577350 ppm. Atan therefore receives that exact integer Ratio and
returns29.999988 degrees. The later tangent returns577350 ppm again. The intermediate Angle
differs from30 degrees by12 microdegrees. Both bindings retain atan and tan contributions.

Rounding can also reach an exact branch boundary. With explicit156-bit/zero-term Tan limits,
`tan(atan(1000000000000000000.0))` fails at an exact tangent pole:the inner finite atan rounds
to90 degrees. The outer call receives that rounded Angle. It does not recover the unrounded
inner angle or replace the expression with the original Ratio.

Similarly,`sin(-0.000001 deg)` rounds to canonical zero. Consequently
`atan2(sin(-0.000001 deg),cos(-180 deg))` returns the positive180-degree axis. A different
cohort input with an actually negative tiny y component preserves negative180 degrees.
Neither result is subjected to entity-direction modulo.

## A source remains unbound until a recipe stores it

Configure the Derived formula for declared_width as `sqrt((3 um/2)^2)/3`:

```text
let a:length=declared_width*3
let b:length=declared_width
let c:length=b*3
```

The root rounds the exact1.5-micrometre tie to2 micrometres. Division leaves the source value
at2/3 micrometre. The source is evaluated once;its cached value is still unbound.

| Binding | Value before binding | Stored value |
| --- | --- | --- |
| a | 2 micrometres | 2 micrometres |
| b | 2/3 micrometre | 1 micrometre |
| c | 3 micrometres from the stored b | 3 micrometres |

All three retain the actual sqrt contribution. Aliases of the same source reuse its one
unbound result. Another cohort joins atan/tan,arc_length and hypot suppliers:both execution
modes retain identical dependency completion ordinals,source roots and recipe/source reads.
No independently authored value is inserted into a Derived declaration.

## All eight functions can contribute to one value

```text
let a:ratio=tan(atan2(
  sin(atan(3.0/4.0))*hypot(3 mm,4 mm),
  cos(atan(3.0/4.0))*sqrt(arc_length(45 deg,4 mm)^2)
))
```

This fixture stores1.193507. Its result retains all eight successful contributions:
sqrt,hypot,arc_length,sin,cos,tan,atan and atan2. Each parent uses the actual rounded child.
For example,the arc returns3142 micrometres before the square/root and later multiplication.
Another fixture evaluates `arc_length(atan(1.0),hypot(sqrt((3 mm)^2),4 mm))` and stores
3927 micrometres,retaining all four executed calls.

Cancellation to zero,comparison to false and binding/read-back preserve successful
contributions. An eps_num assertion still refuses such a value. The available T2 contribution
guard does not grant a physical-tolerance or geometry certificate.

## Failure order and lazy branches stay observable

Arguments execute in language order. A failing child prevents its parent from producing a
value or adding a successful contribution. These are actual cohort cases:

| Expression and selected caller policy | First outcome |
| --- | --- |
| tan(atan(sqrt(-1.0))),0/0 | Negative sqrt,formula_domain,before Tan admission |
| tan(atan(sqrt(1.0))),155/0 | Tan fold resource refusal:requested156 bits,prior reservation/charge0 |
| atan(1.0/0)+atan2(0 mm,0 mm) | Division,formula_division |
| atan2(0 mm,0 mm)+atan(1.0/0) | Zero vector,formula_domain |
| hypot(1 mm/0,sqrt(-(1 um)^2)) | Division in the first argument |
| hypot(sqrt(-(1 um)^2),1 mm/0) | Negative root in the first argument |

The two quota numbers are maximum candidate bits/cumulative series terms. A resource refusal
retains its original operand,selected policy,next demand and prior progress with no invented
formula-language diagnostic. Supplier and caller policies remain independent through nested
source calls. In particular,a supplier may need2073/512 while its caller uses156/0 for an
exact tangent of a cancellation.

The cohort also checks a mixed left operand failing before an unavailable assertion tolerance,
source-domain versus source-resource order,and a successful statement followed by a refusal
with no accepted partial prefix. Untaken branches and unused sources do not execute their
failing calls or acquire their contributions. Their complete syntax/kind proofs still exist.

## Independent evidence and scope

The mixed population has32 recipes:20 successful and12 first-refusal cases. Both the ordinary
native paired entry and actual containing Design execution compare all64 complete outcomes.
The watcher also checks exact root identity,current scope demand,both policies,unchanged
canonical state,source roots,completion ordinals and every recorded read. The original24-row
paired tangent fixture remains byte-exact and runs through the same shared watcher.

The mixed_irrational_design_oracle.py producer reads no Rust,native output or expected fixture.
The actual book evaluator supplies parsing,argument order,source demand,storage and read events.
Integer midpoint roots and exact Fraction enclosures certify each executed irrational call.
Forward calls use shifted/reflected sine;inverse calls use independent addition reduction;
pi uses atan(1/2)+atan(1/3). Ordinary book Decimal calculations separately agree on every
executed non-Tan mathematical result. Existing independent tangent math/resource references
handle the selected per-call policies. Every finite enclosure must separate its result or
fixture generation refuses;there is no guessed answer or universal practical precision claim.

All12122 frozen bytes regenerate,SHA-256
635d0527785039caff58662c806b9cbb94e490a42cc96e241d451b56dea93c25.
Four native tests cover both populations and both byte regenerations;strict affected lint and
formatting pass. These are scoped mixed-function checks. [Observed native/Chrome numerical
execution](formula-irrational-platform.md) is now available separately. Complete browser source/Design
application workflows,full coupled acceptance,aggregate resources,geometry,provider/replay integration,
physical validation and human release approval retain their owners.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/mixed_irrational_design_oracle.py
python3 -I -B scripts/local_environment.py -- cargo test --offline --locked -p sc-core --test design_structure_contract formula_execute::independent::tangent
```
