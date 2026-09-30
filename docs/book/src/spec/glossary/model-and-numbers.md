# Glossary: the model and its numbers

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection. Tokens in this part are quoted from `sc-units`
> where the crate already implements them and from the specification chapter that declares them where
> it does not yet.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| adaptive precision | computing a predicate exactly only where a cheap filter cannot already decide it | [units §5](../units-and-tolerances.md) | exact-arithmetic fallback, Shewchuk-style | — |
| angle | a direction or a sweep, stored as an integer count of microdegrees and normalized on storage | [units §1.2](../units-and-tolerances.md) | direction, rotation, sweep | `Angle` |
| area | a product of two lengths; always derived, never entered | [units §1.3](../units-and-tolerances.md) | surface, *Fläche* | `Area` |
| canonicalization ⚠ | fixing order, formatting and ids so the same state always serializes to the same bytes | [ontology §7](../ontology.md) | canonical form, normalization, *Kanonisierung* | — |
| chordal tolerance | the largest distance a tessellated polyline may stand off the curve it replaces | [units §3](../units-and-tolerances.md) | chord height, sagitta, flattening | `GeometricChordal` |
| count | an integer number of things — pieces, plies, notches, stitches — with no unit at all | [units §1.3](../units-and-tolerances.md) | quantity, tally | `Count` |
| cubic Bézier | the only free-form curve in the v1 set: four points, exact to the format's own vocabulary | [units §4](../units-and-tolerances.md) | Bézier, spline (⚠ loosely) | — |
| circular arc | a curve of constant curvature, kept as an arc and not as a polyline | [units §4](../units-and-tolerances.md) | arc, radius curve | — |
| declared domain | a bound tighter than the integer type, so intermediate arithmetic cannot overflow | [units §1.1](../units-and-tolerances.md) | valid range, domain limit | `UnitError::DomainExceeded` |
| deterministic replay | re-running a recipe producing byte-identical output on every platform and compiler | [ontology §3.1](../ontology.md) | reproducible build, replay stability | — |
| dimension error ⚠ | an operation given kinds the grammar has no rule for; refused before any value is computed | [formula language §5.2](../formula-language.md) | kind error, unit mismatch, type error | `formula_dimension` |
| EdgeRef ⚠ | a reference to an edge by the identity of the operation that made it plus a persistent tag | [ontology §1](../ontology.md) | edge id, edge handle | `EdgeRef` |
| entity id | a ULID assigned once at creation, never reused and never re-derived from content | [ontology §1](../ontology.md) | id, identity, *Identität* | `ULID` |
| exact ratio | an integer-to-integer conversion factor, so a unit conversion rounds exactly once | [units §2](../units-and-tolerances.md) | conversion ratio, rational factor | `Unit` |
| exact rational arithmetic | evaluating in integer numerators and denominators, so the four arithmetic operators never round | [formula language §4.2](../formula-language.md) | rational arithmetic, exact arithmetic | — |
| expression | one formula: a tree of operators over names and literals, evaluated to one value of one kind | [grammar §1](../formula-language/grammar.md) | formula, equation | — |
| golden file | a frozen artifact whose bytes are the expectation a test compares against | roadmap §13 · the corpus grows at `G2-2D` | reference file, expected output, baseline | — |
| importer comparison | the tolerance a receiver's own re-import needs before two artifacts count as equal | [units §3](../units-and-tolerances.md) | round-trip tolerance, receiver epsilon | `ImporterComparison` |
| internal unit ⚠ | the one representation every number is stored in: fixed-point micrometres and microdegrees | [units §1](../units-and-tolerances.md) | base unit, storage unit, *interne Einheit* | → `Length` · `Angle` |
| kind (of a value) | which of eight categories a value belongs to — length, angle, area, ratio, count, boolean, point, edge — checked before evaluation | [formula language §2](../formula-language.md) | dimension, type, unit type | — |
| length | a distance or a coordinate, stored as an integer count of micrometres | [units §1](../units-and-tolerances.md) | distance, dimension | `Length` |
| micrometre | 10⁻⁶ m — the internal length unit; one inch is exactly 25 400 of them | [units §1](../units-and-tolerances.md) | micron, µm, *Mikrometer* | `Micrometre` |
| microdegree | 10⁻⁶ degree — the internal angle unit; a full turn is 360 000 000 of them | [units §1.2](../units-and-tolerances.md) | µ°, micro-degree | `MICRODEGREES_PER_DEGREE` |
| numerical tolerance | one quantum of the internal representation — anything looser hides a real bug | [units §3](../units-and-tolerances.md) | ε_num, machine epsilon (⚠ not the same idea) | `Numerical` |
| geometric approximation | the error a deliberate approximation may carry: an offset curve, a fairing | [units §3](../units-and-tolerances.md) | ε_geo, approximation error | `GeometricApproximation` |
| format quantization | the error a target format's own quantum forces, published with the artifact | [units §3](../units-and-tolerances.md) | ε_fmt, output resolution | `FormatQuantization` |
| physical acceptance ⚠ | the tolerance a factory actually holds — **their number, never ours** | [units §3](../units-and-tolerances.md) | ε_phys, shop tolerance, cutting tolerance | `PhysicalAcceptance` |
| parameterized reference | a position along an edge as a rational in [0, 1] of that edge's own length | [ontology §1](../ontology.md) | t-parameter, normalized position, u-value | `t` |
| PointRef | a reference to a constructed point — an intersection, a notch anchor, a grade point | [ontology §1](../ontology.md) | point id, point handle | `PointRef` |
| ratio | a dimensionless quantity scaled to parts per million: ease ratios, shrinkage, multipliers | [units §1.3](../units-and-tolerances.md) | factor, percentage (⚠ a percent is not a multiplier) | `Ratio` |
| repair task ⚠ | a visible, first-class record of a reference an edit orphaned, with its candidate resolutions | [ontology §1.1](../ontology.md) | unresolved-reference task, dangling reference | — |
| robust predicate | an orientation or intersection test that returns the mathematically correct sign | [units §5](../units-and-tolerances.md) | exact predicate, orientation test | — |
| rounding rule ⚠ | the one rule for turning an exact value into a stored integer: half away from zero | [units §2](../units-and-tolerances.md) | rounding mode, *Rundung* | — |
| stable topological reference ⚠ | addressing geometry by identity and parameter, never by array or tessellation index | [ontology §1](../ontology.md) | persistent reference, topological id | → `EdgeRef` · `PointRef` |
| structural limit | a declared bound on a formula's size, so a pathological recipe is refused rather than exhausting memory | [formula language §4.3](../formula-language.md) | size limit, node budget | `max_expression_nodes` |
| tessellation | approximating a curve by a polyline for a target that has no curves | [units §4](../units-and-tolerances.md) | flattening, faceting, *Tessellierung* | — |
| tolerance class ⚠ | which of five named bounds a comparison uses; there is no global epsilon anywhere | [units §3](../units-and-tolerances.md) | epsilon class, comparison class | `ToleranceClass` |
| tolerance name | a reserved name carrying one of the five classes into a comparison, so a comparison always names its class | [formula language §3.1](../formula-language.md) | epsilon name | `eps_num` |
| typed diagnostic | an error that names the quantity, the operation and the rule it broke — never a panic | [units §1.1](../units-and-tolerances.md) | structured error, diagnostic code | `UnitError` |
| unresolved reference | a reference whose target no longer exists; savable and inspectable, but not releasable | [ontology §1.1](../ontology.md) | dangling ref, broken link | — |
