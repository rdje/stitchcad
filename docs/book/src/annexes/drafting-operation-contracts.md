# Drafting operation contracts

> **Inventory complete; implementation remains owned.** This is the v1 operation boundary required
> by G1-SLICE.5f.1 before construction code. The existing scalar formula executor creates no
> geometry. Primitive declarations,real scalar argument proofs and offset input binding are available
> below;remaining schemas,ordered dependencies and geometry execution are pending. No geometry certificate.

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
| Construct point | x,y Length formulas in a named drafting frame | One persistent point output | Typed documents/Length proofs;geometry pending |
| Draw line | Two prior PointRefs | Persistent edge and explicit endpoint provenance | Typed references/dependencies;geometry pending |
| Draw arc | Center PointRef,radius Length,start/sweep Angle formulas | One edge role;no inferred point outputs | Center-based declaration/proofs;geometry pending |
| Draw Bézier | Explicit quadratic/cubic named PointRefs | One edge role;original controls retained | Typed controls/dependencies;geometry pending |
| Offset edge | Directed range,allowance identity and explicit side;canonical corner | G2 fragment lineage;no hidden width/default join | Input binding only;outputs/geometry pending |
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

The center-based variant below declares one edge role and no endpoint entities. A through-points
intent has no native schema and cannot be silently substituted for it. The Bézier source degree
is explicitly quadratic or cubic;no degree or construction mode is inferred from runtime values.

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

## Native point declarations and argument proofs

`sc_core::construction::PointConstruction` owns a creating-operation EntityId and two original
FormulaExpressionDocuments. `coordinate(PointCoordinate::X)` and Y expose the actual documents;
`output()` is always PointRef(operation,LocalTag::FIRST). It is a stable logical output role,not an
issued identity,a registered current point or a geometric value. The constructor retains already
syntax/input-validated documents;it does not infer their kinds or read their values.

```rust
use sc_core::{construction::*,ontology::{EntityId,LocalTag,PointRef},recipe::*};
let point=PointConstruction::new(EntityId::from_bits(42),
    FormulaExpressionDocument::new("25 mm+eps_num".into())?,
    FormulaExpressionDocument::new("0 mm".into())?);
let limits=PointConstructionNormalizationLimits::new(4,10);
let normalized=point.normalized(limits).expect("complete shapes");
let checked=normalized.check_arguments(&FormulaNamespace::new([])?).expect("actual Length proofs");
assert_eq!(checked.coordinate(PointCoordinate::X).kind(),FormulaKind::Length);
assert_eq!(point.output(),PointRef::new(EntityId::from_bits(42),LocalTag::FIRST));
# Ok::<(),Box<dyn std::error::Error>>(())
```

The two normalized arenas borrow their original document text and preserve actual root/node spans.
`PointConstructionNormalizationLimits` explicitly admits complete N nodes and conservative2+2N
retained records:two argument owners,N normalized nodes and at most N call-argument indices. Both
arguments,every branch and every call argument count before either document is reparsed. Node
quota precedes retained-record quota. An exact allowance accepts. Population overflow,Nodes,
RetainedRecords and original document causes remain typed in the normalization error,with the
actual declaration and selected limits. No normalized argument prefix escapes.

This is a logical retained-shape allowance. Earlier document creation,parser/checker scratch,
allocator capacity,CPU and geometry/runtime resources remain separate. The normalized owner exposes
its actual point,limits,node_count(),retained_records() and coordinate arenas;none can be replaced
by caller annotations or an unrelated expression.

`check_arguments(namespace)` checks x then y with the existing whole expression checker. Each real
proof must have a Length root before a private CheckedPointConstruction can exist. Its coordinate
getters expose those actual proofs,including original declarations,dependencies and checked nodes.
No integer/Angle/Boolean is coerced to Length. A Count root in x refuses before a later missing y
name. An untaken branch with incompatible kinds keeps its original expression refusal.

Checking performs no numerical/provider read:an Unknown measurement remains Unknown. For example,
x=`1 um/0`,y=`1 deg` refuses y's actual Angle root before any division. With y=`2 mm`,both static
Length proofs accept while that division remains unevaluated. A successful proof therefore grants
neither a coordinate value nor an accepted point.

`PointConstructionCheckError` retains the actual normalized owner,PointCoordinate role,required
Length kind,original span and Expression or Kind cause. An expression refusal keeps its original
typed name/operator/call error;only a completed wrong-kind root uses the coordinate-kind refusal.
The canonical family remains the original token,or formula_dimension for that actual wrong root.
No recipe statement ordinal,geometry registration or frame context is invented. Debug omits
authored source text;explicit document/proof getters retain it when requested. The owned declaration,
normalized arenas and checked proofs enforce their actual borrowing lifetimes and private fields.

These declarations/proofs do not allocate identities,bind geometry names,choose a drafting frame,
execute selectors or publish through the command bus. Containing frame/current registry/order and
operation-wide execution are .5f.2/.3 work. Remaining primitive/edit schemas finish .1 first.
Full geometry,physical behavior,storage and complete operation replay remain owned milestones.

## Native line declarations and dependency views

`LineConstruction::new(operation,start,end)` retains the two original PointRefs in their declared
direction. Its logical output is EdgeRef(operation,LocalTag::FIRST). Inputs are PointRefs by type;
an EdgeRef is not coerced into an endpoint. Neither id magnitude nor ULID sort order determines
whether an input operation is earlier in the actual program.

```rust
use sc_core::{construction::*,ontology::{EntityId,PointRef,LocalTag}};
let start=PointRef::new(EntityId::from_bits(99),LocalTag::FIRST);
let end=PointRef::new(EntityId::from_bits(80),LocalTag::new(7));
let line=LineConstruction::new(EntityId::from_bits(10),start,end)?;
let uses=line.dependencies().map(|use_|(use_.endpoint(),use_.point())).collect::<Vec<_>>();
assert_eq!(uses,vec![(LineEndpoint::Start,start),(LineEndpoint::End,end)]);
# Ok::<(),Box<dyn std::error::Error>>(())
```

The constructor rejects a definite local self-supply:an endpoint whose creator is this same line
operation cannot be a prior supplier. It checks Start before End. `LineConstructionError` retains
the original operation,first refused LineEndpoint and complete PointRef/local tag. It invents no
formula name,source span or global cycle path. This check does not prove that another creator is
earlier,current or even registered;those actual program checks remain .5f.2 work.

`dependencies()` yields two LinePointDependency views in Start/End order. Each privately retains
this exact line and its actual field role. `line()`,`endpoint()` and `point()` expose that same
source relationship. A foreign line/point cannot be attached to a view,and a view cannot outlive
its declaration. Equal repeated PointRefs remain two source occurrences with distinct roles;
there is no sorting or deduplication. Reversing the declaration changes its source meaning while
preserving the declared logical output role. Actual topology repair/publication still has to
account for an edit's changed direction before any existing reference can be used.

A declaration can retain the same point in both fields. It does not thereby certify a nonzero
line or accept a degenerate geometric edge. No coordinate,distance,frame,registry state,identity
issuance or geometric result is computed. The containing program and actual geometry executor
must validate those properties before acceptance. Remaining curve/edit schemas are still pending.

## Native center-based arc declaration and scalar proofs

`CenterArcConstruction` explicitly retains a prior center PointRef and three original expression
documents. It is the center/radius/start/sweep variant;it cannot silently become an arc through
points. The logical output manifest contains only EdgeRef(operation,LocalTag::FIRST). No endpoint
PointRefs are issued or inferred,including for a closed sweep. Later point_at queries remain
read-only observations of actual geometry,not new persistent entity identities.

| Scalar role | Required kind | Meaning before geometry |
| --- | --- | --- |
| Radius | Length | Actual expression retained;positive radius not proved |
| StartAngle | Angle | Original sign and complete turns retained |
| SweepAngle | Angle | Signed sweep retained;no modulo/nonzero/supported-domain proof |

The constructor receives already syntax/input-validated FormulaExpressionDocuments. It rejects a
center created by that same operation,retaining the actual operation and PointRef/local tag in
CenterArcConstructionError. Another creator's id,however large or small,is not evidence of prior
order or currentness. `center_dependency()` returns an authentic borrowed CenterArcPointDependency;
its `arc()` and `point()` identify this exact original declaration/field. No foreign center or
declaration can be substituted and the view cannot outlive its owner.

```rust
use sc_core::{construction::*,ontology::{EntityId,PointRef,LocalTag},recipe::*};
let arc=CenterArcConstruction::new(EntityId::from_bits(10),
    PointRef::new(EntityId::from_bits(1),LocalTag::FIRST),
    FormulaExpressionDocument::new("1 mm/0".into())?,
    FormulaExpressionDocument::new("90 deg".into())?,
    FormulaExpressionDocument::new("-450 deg".into())?)?;
let limits=CenterArcNormalizationLimits::new(6,15);
let normalized=arc.normalized(limits).expect("complete formula shapes");
let checked=normalized.check_arguments(&FormulaNamespace::new([])?).expect("actual field kinds");
assert_eq!(checked.argument(CenterArcArgument::Radius).kind(),FormulaKind::Length);
# Ok::<(),Box<dyn std::error::Error>>(())
```

`normalized(limits)` measures complete N nodes and3+2N conservative formula-normalization records
before reparsing any argument:three argument owners,N normalized nodes and at most N call-argument
indices. Every argument/branch participates. Node quota precedes retained-record quota. It retains
the actual arc,selected limits,complete counts and original argument arenas. Complete population/
quota or original document failure exposes no normalized prefix. Earlier input documents/center
metadata,parser/checker scratch,allocator capacity,CPU and geometry/runtime are outside this quota.

`check_arguments(namespace)` obtains each real whole expression proof in Radius/StartAngle/
SweepAngle order,then enforces that field's required root kind. Only complete success constructs
CheckedCenterArcConstruction. Both point and arc factories use the same private measurement/static
checking kernels;their public owner/policy/role/error types remain separate. A caller cannot supply
an expected-kind annotation or a detached proof to this constructor.

Radius=`1 mm/0`,start=`0 deg`,sweep=`1` refuses the actual Count sweep before any radius division.
An untaken incompatible branch retains its original expression error. An Unknown radius remains
Unknown;successful static Length proof supplies no number. Signed sweeps beyond a full turn retain
their original formula bytes and are never silently reduced modulo360 at this stage.

CenterArcCheckError retains the actual normalized owner,argument role,required kind,original span
and Expression or Kind cause. It preserves the existing canonical formula token,including
formula_dimension for an actual wrong root. No program ordinal,cycle path,center frame or current
reference is invented. Private fields and borrowing lifetimes keep all proofs with their actual
source owners;Debug omits authored formula text.

These proofs do not accept a geometric arc. For example,a negative Length radius or zero Angle
sweep can have valid static kinds. Actual geometry must enforce positive radius,nonzero/supported
sweep,center/frame/currentness and its approximation budget before acceptance. The D126
actual-edge-length/parameter budget remains mandatory. No curve length,endpoint coordinates,
geometry,identity issuance or command publication is computed by this declaration API.

## Native quadratic and cubic source declarations

`BezierConstruction::new(operation,controls)` retains a closed BezierControlPoints variant:
Quadratic has named start/control/end PointRefs;Cubic has start/first_control/second_control/end.
No integer degree or arbitrary vector is interpreted to choose a mode. Each field is a PointRef
by type. The chosen input BezierDegree survives even when control references coincide.

```rust
use sc_core::{construction::*,ontology::{EntityId,PointRef,LocalTag}};
let point=|id|PointRef::new(EntityId::from_bits(id),LocalTag::FIRST);
let curve=BezierConstruction::new(EntityId::from_bits(10),BezierControlPoints::Quadratic {
    start:point(99),control:point(80),end:point(70) })?;
assert_eq!(curve.degree(),BezierDegree::Quadratic);
assert_eq!(curve.dependency_count(),3);
# Ok::<(),Box<dyn std::error::Error>>(())
```

The declaration's only output role is EdgeRef(operation,LocalTag::FIRST). It creates no endpoint or
control-point entities. `controls()` exposes the actual original layout. `dependency_count()` is
three or four from that explicit variant,not a caller annotation. `dependencies()` yields the real
fields in their authored order. Roles are Start/FirstControl/End for a quadratic and
Start/FirstControl/SecondControl/End for a cubic. Quadratic input has no SecondControl occurrence.

Each private BezierPointDependency borrows this exact declaration and its actual underlying field.
`construction()`,`role()`,`point()` and `point_ref()` expose that same source relationship.
No foreign point can be attached to a view and no view can outlive its declaration. Equal repeated
PointRefs remain separate original uses. Reversing a declaration changes the ordered source intent
while preserving the logical output role;actual topology/reference repair before publication
remains a program/geometry obligation.

A source created by this same operation cannot be a prior control. The constructor checks real
field order and returns BezierConstructionError with the actual operation,first refused role and
original PointRef/local tag. This definite local refusal proves no other supplier's order,
currentness or frame. It invents no formula span/name or global cycle path.

The [curve representation contract](../spec/units-and-tolerances.md#4-curve-representation) remains
cubic. G2 must promote actual quadratic geometry exactly,with no approximation. This authored
declaration preserves the three original quadratic source references;it does not fabricate four
cubic source references or compute degree-elevated coordinates. Cubic declarations retain all four
original sources. Higher-order/NURBS support remains deferred with the existing typed refusal/loss
contract;no fallback degree or polyline is silently chosen.

Repeated/coincident controls can form declarative input without proving a nondegenerate geometric
curve. No coordinates,length,continuity,frame,registry state,exact elevation,approximation budget,
identity issuance or command publication is computed here. Remaining editing schemas and the
whole operation graph/geometry integration remain owned work.

## Native offset input/source association

`construction::OffsetConstruction` retains an operation id,the existing `DirectedRange`
(positive `EdgeRange` plus explicit `Direction`),explicit `OffsetSide::{Left,Right}` and the required `SeamAllowance` identity.
The range's exact ascending bounds stay in its original edge frame. Traversal controls which way
it is followed;left/right are relative to that traversal. They do not mean inside/outside a Piece,
fabric face or cutting side. Nothing chooses a winding-based side or join automatically.

The constructor refuses definite self-supply:an edge created by this operation first,then a
required allowance with that same identity. Numeric id order supplies no prior-operation proof.

`bind_allowance` checks the provided actual descriptor's identity,then its original source edge.
It returns a private `BoundOffsetConstruction` view borrowing that exact declaration and descriptor.
Its width,corner and inclusion accessors borrow the actual descriptor fields. Formula and Profile
origins stay symbolic;there is no copied client width,zero fallback or corner override.

```rust
use sc_core::{construction::*,ontology::*};
fn prepare_left_offset(operation:EntityId,allowance:&SeamAllowance)
    ->Result<OffsetConstruction,Box<dyn std::error::Error>> {
    let input=OffsetConstruction::new(operation,
        DirectedRange { range:EdgeRange::whole(allowance.definition().edge),
            direction:Direction::Original },OffsetSide::Left,allowance.id())?;
    let bound=input.bind_allowance(allowance)?;
    assert!(std::ptr::eq(bound.allowance(),allowance));
    assert!(std::ptr::eq(bound.width(),&allowance.definition().width));
    Ok(input)
}
```

This is an association check against the supplied record. It does not prove that record belongs
to the containing current Design or inventory. A same-id record holding a different source edge
is refused;no current fragment is silently substituted for an authored historical edge. Matching
ids and edges alone do not validate the current Piece/ledger coverage. For example,after source
deletion this association can still succeed while `SeamAllowance::validate_current` reports its
original missing-interval repair evidence. The program must perform current-root and dependency
checks before execution;the bound view certifies neither values nor geometry.

This input stage declares no fixed output count or tags. G2 must derive the actual fragment
intervals and use the published identity-ledger contract for unambiguous mappings and visible
repair tasks. Source geometry,width resolution,corner construction,loops,error bounds,output
manifest and command publication remain pending. A Bézier offset cannot be assumed to be one
exact Bézier,or certified from a bounding box. Full operation integration remains `.5f.4` work.
