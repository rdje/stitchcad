# Drafting operation contracts

> **Inventory complete; implementation remains owned.** This is the v1 operation boundary required
> by G1-SLICE.5f.1 before construction code. The existing scalar formula executor creates no
> geometry. Typed point declarations are the next slice;the remaining operation schemas,ordered
> dependencies and actual geometry execution are pending. This page grants no geometry certificate.

The [ontology](../spec/ontology.md#32-drafting-operations) separates the ordered drafting history
from scalar lets/assertions. A formula can read a prior point or edge through a selector;only a
drafting operation can create an entity and own its persistent output references. Geometry is
derived from that history. Explicit imported primitives retain imported origin and acquire no
invented drafting history.

## Shared operation boundary

Every authored operation needs its own EntityId,typed input fields and a declared output manifest.
An output PointRef or EdgeRef combines that creating operation id with a persistent LocalTag.
Tags identify logical outputs,not positions in a mutable geometry vector. Replaying the same
operation keeps those references. A topology-changing edit uses the existing retirement/repair
protocol;it cannot silently reuse an old tag for a different output.

The schemas below retain unevaluated formula documents and canonical references. Parsing a formula
does not prove its Length/Angle kind,bind a name or produce geometry. Complete static checking must
validate every argument against its actual prior-operation/always-visible namespace before any
value is read. Numerical arguments then evaluate at their actual operation position. Scalar lets
store once;operation arguments do not become extra implicit lets. All input states,approximation
contributions and actual failures survive their normal adapters.

Creating or editing authored operations is a command-bus mutation. Request intent,validated
authored record and accepted geometric result are separate stages. A caller-supplied id or reference
is not permission to issue/reuse an identity. The bus owns fresh identity allocation,revision
preconditions,authorization,atomic publication,audit and undo. A failed later argument or operation
publishes no accepted geometric prefix. A schema constructor alone grants none of that authority.

All families below belong to G1-SLICE.5f.1. Typed point declarations are .1.b;remaining schemas are
.1.c and receive smaller implementation leaves before code. Ordered dependency integration is
.5f.2;selector/parameter error budgets are .5f.3;combined operation replay is .5f.4. G2 owns concrete
geometry and G3 owns the construction/physical behavior assigned to it by the feature matrix.

## Complete v1 inventory

| Family | Required inputs | Output/intent boundary | Native schema today |
| --- | --- | --- | --- |
| Construct point | x,y Length formulas in a named drafting frame | One persistent point output | Pending .1.b |
| Draw line | Two prior PointRefs | Persistent edge and explicit endpoint provenance | Pending .1.c |
| Draw arc | Explicit arc construction variant;Length/Angle arguments and prior points | Persistent curved edge and declared point outputs | Pending .1.c |
| Draw Bézier | Explicit degree and ordered prior control PointRefs | Persistent curved edge;control order retained | Pending .1.c |
| Offset edge | Current EdgeRef/range,canonical allowance and explicit side/corner policy | New edge lineage;no hidden width/default join | Pending .1.c |
| Slash/spread | Current Piece/cut/pivot references and explicit opening mode | New/retired topology and an attributable opening | Pending .1.c |
| Close dart | Canonical Dart id with held intake/closing references | One semantic closure group | CloseDart request exists;geometry pending |
| Add notch | Piece id,EdgeAnchor,NotchProfileBindings | Fresh semantic notch id and retained source anchor | AddNotch request exists;geometry pending |
| Place grainline | Current Piece and canonical grain direction/anchor intent | Persistent semantic placement and derived mark | Canonical intent exists;operation pending |
| Walk seam | Actual seam correspondence and explicit balance-mark intent | Differential report plus deliberate mark edits | Query exists;editing operation pending |
| True seam | Actual correspondence,declared ease and explicit adjustment policy | Attributed edge adjustment and remaining differential | Pending .1.c |
| Mirror | Current source geometry and explicit reflection axis/mode | Declared output identities and source mapping | Cut orientation exists;operation pending |
| Join pieces | Current pieces/seam correspondence and explicit join intent | Output Piece/topology with semantic lineage | SewingGraph exists;operation pending |

The last column is an availability boundary. Stable reference types,structural inventories or a
named command are not substitutes for an implemented drafting record and its executor.

## Points and primitive curves

A point declaration retains the two actual expression documents for x and y,its creating operation
identity and its persistent point output. Both expected argument kinds are Length. The eventual
whole-operation checker must reject a Count,Angle,Boolean or geometry reference in either slot
before coordinates are evaluated. The schema must obtain its drafting frame explicitly from the recipe;
there is no ambient screen coordinate system. No x/y default is supplied. A typed declaration
retains authored syntax/canonical identity without pretending to be an evaluated point.

For example,x=`waist/4` and y=`0 mm` remain two formulas over the actual containing namespace.
`let p:point=...` is not added to the scalar grammar. The operation's output manifest supplies the
PointRef and any named geometry declaration that later selectors consume. Missing measurements
remain missing,even when another coordinate could already be computed.

A line retains its ordered endpoint PointRefs. Their currentness,frame compatibility and geometric
degeneracy need actual validation;well-formed reference bytes alone prove none of those properties.
The direction from first endpoint to second is meaningful and cannot be discarded by sorting ids.

An arc schema must select its construction variant explicitly. Center/radius/start/sweep inputs
cannot be confused with a through-points construction. Radius is Length;start/sweep are Angle;
point alternatives use prior PointRefs. Formula Angle preserves sign and complete turns. A later
geometry implementation declares its supported sweep/degeneracy domain and refuses outside it;
it cannot silently reduce a sweep modulo360 or choose an alternative arc. Endpoint/output tags
must be defined for closed and open variants before that schema is accepted.

A Bézier schema similarly declares its supported degree and exact ordered control-point slots.
It preserves endpoint/control roles and source provenance. Curve parameterization,degeneracy,
length and approximation accuracy remain actual geometry obligations. A sampled polyline is not
silently substituted for the authored curve.

The unresolved arc variants/output manifest and Bézier degree domain are owned schema work in
.1.c,not a choice made by a caller's float values at runtime. Their implementation leaves must
finish those public contracts before exposing construction or selector execution.

## Edits and garment semantics

Offset reads its actual canonical SeamAllowance intent,including width source and corner policy.
An unresolved profile/Length source stays unresolved. Selected side and range orientation must be
explicit. The executor needs actual source geometry,corner/loop classification and bounded error;
a symbolic allowance record or a bounding box cannot certify the resulting offset.

Slash/spread retains its actual target Piece,cut and pivot references plus a discriminated opening
request. Angle opening and Length separation are different argument kinds and require explicit
variants. The schema must declare which variants it supports,which region moves and how changed
references map to outputs. No inferred solver or implicit geometric repair enters a scalar formula.

Existing CloseDart retains only the canonical Dart target id. The executor resolves that actual
record's intake and closing references instead of accepting copied client geometry or an invented
intake. Its geometry and complete command integration remain pending. Existing AddNotch retains
the actual Piece id,EdgeAnchor and both sample/production NotchProfileBindings;the executing bus
supplies the new semantic identity. A persistent edge parameter is not a tessellation index or an
already validated cutting mark.

Grainline placement consumes actual canonical direction/anchor intent for the named Piece. Its
Angle and reference requirements remain explicit;material/profile constraints do not become
default directions. Existing structural grainline data supplies intent,not an executed mark.

Walk is an editing operation when it deliberately plants balance notches. The existing
`CommandId::WalkSeamSpan` is a Query;its differential observation cannot perform those edits or
stand in for that operation. Editing walk must retain the actual seam correspondence,declared
mark positions/representations and fresh identity policy through a mutation group. Underlying
length queries remain read-only. Typed command/payload/authority integration is .6 work alongside
the .1.c operation schema;no existing Query is silently reclassified as a mutation.

True deliberately adjusts the named seam under an explicit adjustment policy and declared ease.
It reports any remaining differential rather than hiding it in geometry. The actual target span,
affected edges,which side may change and protected landmarks must be retained. A comparison within
tolerance is an observation;it does not itself supply permission or an adjustment operation.

Mirror declares its reflection axis,source geometry and output mode. A CutPlan's mirrored physical
copy orientation already exists,but does not execute a geometric mirror operation. New output
identities or explicit retirement mappings remain necessary;ids cannot be reassigned by coordinate
coincidence. Join likewise consumes actual Piece/seam correspondence and preserves semantic
content/material/mark ownership in its declared output mapping. A SewingGraph relation alone does
not join geometric boundaries or prove that the result is a simple valid Piece.

## Selectors consume actual earlier outputs

The existing [selector signatures](../spec/formula-language/grammar.md#61-geometry-selectors) stay
authoritative: x/y return Length in the point's drafting frame;dist returns Length;dir alone
normalizes its direction to `[0,360)`;len reports actual edge arc length. param_at takes a Length
within that edge's length and returns Ratio;point_at takes Ratio in `[0,1]` and returns a point.
Neither creates a persistent entity. A derived point query is not a new authored operation id.

Stable references must resolve against the actual operation output/topology history. Missing,
retired,ambiguous or future suppliers retain their actual context. Formula dependency validation
must include every operation argument and geometry source before any value executes. A later
untaken branch cannot hide an invalid static reference or unsupported capability.

Parameter quantization and geometric approximation share an explicit error budget based on actual
edge length. The existing radius5m/270° counterexample exceeds5µm parameter displacement despite
fitting the10m box. Neither that box nor the fixed Ratio quantum proves placement within T2.
The bridge must retain actual length/parameter bounds and geometry approximation contribution
before accepting a placement;G2 supplies the concrete evidence.

## What is next

The next native slice is a typed point-from-formulas declaration,with private retained fields,
original expression ownership and a stable point-output identity. It will retain syntax and
expected Length roles;it will not evaluate coordinates,publish a mutation or introduce a geometry
kernel. Remaining primitive/edit schemas finish .1 before ordered construction/selector work.
Full geometry,physical behavior,storage and complete operation replay remain owned milestones.
