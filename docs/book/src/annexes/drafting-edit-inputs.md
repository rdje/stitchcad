# Native editing operation inputs

> Input declarations and scoped proofs only. Current geometry,profile resolution and execution
> remain owned. The [operation inventory/shared boundary](drafting-operation-contracts.md) defines
> the complete v1 contract and distinguishes schema acceptance from later real geometry proof.

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

## Native grainline placement input binding

`construction::GrainlinePlacement` retains an operation id and the required existing semantic
`Grainline` identity. It supplies no copied direction,default angle or replacement print axis.
`bind_grainline` checks the actual target identity,then every original arrow/alignment/stripe/plaid
source in that order for definite local self-supply. The constructor also refuses a target with
this operation's own identity. Numeric ids do not establish global prior-operation order.

The private `BoundGrainlinePlacement` view borrows the exact placement and supplied semantic
record. Its `alignment` accessor borrows the actual `GrainAlignment` field;`references` delegates
to that record's directed sources. Parallel alignment stays distinct from an explicit Angle,
Formula id or Profile parameter. No angular value is read or inferred. Stripe and plaid remain
independent optional roles;absent references stay absent,and repeated sources retain every use.

```rust
use sc_core::{construction::*,ontology::*};
fn prepare_grainline(operation:EntityId,grainline:&Grainline)
    ->Result<GrainlinePlacement,Box<dyn std::error::Error>> {
    let input=GrainlinePlacement::new(operation,grainline.id())?;
    let bound=input.bind_grainline(grainline)?;
    assert!(std::ptr::eq(bound.grainline(),grainline));
    assert!(std::ptr::eq(bound.alignment(),&grainline.definition().alignment));
    Ok(input)
}
```

The semantic target retains its Piece owner and directed arrow. This stage issues no new semantic
identity or geometric edge tags and performs no mutation. Straightness,actual angular relation,
material/profile constraints and the derived visible mark remain G2/G4/execution obligations.

Association is checked against the supplied record;it does not prove membership in the actual
current Design/inventory or current ledger coverage. A later deleted plaid source can still pass
this association while `Grainline::validate_current` reports its original Plaid repair cause.
Existing `DesignInputs::grain_allowance_declarations` checks actual canonical Angle metadata for
formula alignment without supplying a value. The eventual ordered operation program must join
those current-root,namespace/frame and evaluation checks before placement/publication. Full
construction integration and platform verification remain deferred under `.5f.4`.

## Native slash/spread opening component

`construction::SlashSpreadOpening` is the numerical amount component of a later full operation.
`rotation(angle_document)` explicitly requires Angle;`separation(distance_document)` explicitly
requires Length. The mode is chosen by the author,not inferred from formula text or runtime values.
A rotation containing `1 mm` is a dimension error;it cannot silently become a separation request.

Each component retains its exact original owned expression document. `normalized` first measures
all actual formula nodes N and admits explicit `SlashSpreadNormalizationLimits` for N and the
conservative `1 + 2N` normalization records:one amount owner,N nodes and at most N call-argument
indices. Complete arithmetic/node/record admission precedes reparse. These quotas exclude earlier
owned documents,parser/checker scratch,allocator/CPU and whole-operation/geometry/runtime usage.

`check_argument` checks the actual normalized amount against the supplied real namespace,then
requires the root kind selected by that actual original mode. Both branches undergo static checks.
The private checked result borrows the normalized owner and retains its real expression proof;
errors retain that owner,mode,actual failing span and typed language or wrong-root-kind cause.
No statement/operation ordinal is invented for this standalone component.

```rust
use sc_core::{construction::*,recipe::*};
let opening=SlashSpreadOpening::separation(
    FormulaExpressionDocument::new("1 mm/0".into())?);
let normalized=opening.normalized(SlashSpreadNormalizationLimits::new(3,7))
    .expect("complete shape for this fixed literal");
let checked=normalized.check_argument(&FormulaNamespace::new([])?)
    .expect("fixed literal has Length kind");
assert_eq!(checked.expression().kind(),FormulaKind::Length);
// No division or geometric separation has executed.
# Ok::<(),Box<dyn std::error::Error>>(())
```

Original signs and full Angle turns remain authored. An Unknown canonical Length source is still
statically Length,and its actual supplier remains attached;no observation or zero is supplied.
The examples assert the known fixed literal shapes/kinds. Typed normalization/check errors borrow
those original owners;applications must keep the owners alive when returning or inspecting such
errors. These assertions do not execute numerical values or geometry.
A successful kind proof grants neither a supported geometric domain nor a usable numeric value.
For example,`1 mm/0` has Length kind but still needs normal numerical execution and its refusal.

This amount component supplies no operation id,Piece,cut,pivot,moving bank,translation direction
or output manifest. The spatial declaration below retains the actual required sources;none is
inferred from the amount. Currentness,region/domain validation,transform evaluation,intake/fullness
and source/output mappings remain `.5f.3`/G2/G3 execution work. Output contracts and command
publication remain owned;full construction verification stays deferred under `.5f.4`.

## Native slash/spread spatial inputs

`construction::SlashSpreadConstruction` joins the actual amount component to explicit spatial
inputs. Both constructors require an operation id,the target Piece id,an original `DirectedRange`
cut and `SlashSpreadSide::{Left,Right}` for the moving bank relative to that cut traversal. This
side is neither fabric face nor an inferred winding choice. The other bank remains stationary.
Reversing the cut without changing the selected side changes authored intent;no side swap occurs.

`rotation` additionally requires an actual `PointRef` pivot and an Angle document. `separation`
requires an actual `DirectedRange` movement axis and a Length document. Positive separation follows
that directed axis;the cut's normal is never invented. Constructors create the matching explicit
opening component,so a wrong-kind amount remains a kind error for the originally selected mode.
Rotation has no translation-axis field;separation has no pivot field. Neither gets a default.

```rust
use sc_core::{construction::*,ontology::*,recipe::*};
fn declare_separation(operation:EntityId,piece:EntityId,cut:DirectedRange,axis:DirectedRange)
    ->Result<SlashSpreadConstruction,Box<dyn std::error::Error>> {
    let input=SlashSpreadConstruction::separation(operation,piece,cut,SlashSpreadSide::Right,
        axis,FormulaExpressionDocument::new("10 mm".into())?)?;
    let normalized=input.opening().normalized(SlashSpreadNormalizationLimits::new(1,3))
        .expect("complete shape for the fixed 10 mm literal");
    let checked=normalized.check_argument(&FormulaNamespace::new([])?)
        .expect("fixed literal has Length kind");
    assert_eq!(checked.expression().kind(),FormulaKind::Length);
    Ok(input)
}
```

Definite local self-supply is refused in original Piece,cut,then pivot/axis order. Each geometry
`dependencies` view is private,borrows the original declaration and exposes the actual underlying
cut/pivot/axis field with its role. There are exactly two such occurrences,including repeated range
uses. The target Piece and actual amount formula dependencies remain separate parts of the whole
program graph;this iterator does not claim to enumerate that complete graph.

These are spatial input declarations. A current geometry executor must validate the target Piece
and common drafting frame,the complete current source coverage,a straight supported cut,a unique
applicable moving bank,pivot incidence or a nondegenerate straight movement axis,and supported
numerical/geometric domains. Selecting Left or Right does not prove that such a region exists or
resolve multiple candidates. Failed geometry cannot be replaced with a guessed region or axis.

This stage issues no new Piece or geometric output tags and mutates nothing. Explicit boundary/
fullness intent is available in the [output reference](drafting-slash-output-contracts.md#native-slashspread-boundary-and-fullness-intent).
Output-role/manifest admission remains `.1.c.slash.outputs.manifest` work;actual fullness/intake,
new/retired topology and source mappings require G2/G3 before publication. No default connector
crosses an opened cut. Full operation integration remains `.5f.4` work.

## Native reflected Piece-copy inputs

The complete [mirror input/source/metadata contracts](drafting-mirror-contracts.md#native-reflected-piece-copy-inputs)
live in their bounded companion. This original anchor remains available.

### Binding the actual mirror source Piece

See [actual bounded source binding](drafting-mirror-contracts.md#binding-the-actual-mirror-source-piece).

### Explicit result cut policy

See [explicit canonical cut policy](drafting-mirror-contracts.md#explicit-result-cut-policy).

### Explicit material,layer,label and print data

See [field selection and canonical print data](drafting-mirror-contracts.md#explicit-materiallayerlabel-and-print-data).

## Native deliberate walk-edit requests

`construction::WalkSeamEdit` owns an operation id,the required canonical span id and the original
ordered `WalkBalancePair` collection. Each pair requires two `WalkBalanceNotch` inputs:one for
side A and one for B. Each retains its own exact `EdgeAnchor` and all `NotchProfileBindings`,with
sample/production contexts separate. There is no caller-supplied new notch id or copied span ease,
physical-copy identity or Piece metadata. The existing `QueryPayload::WalkSeamSpan` stays read-only.

The raw constructor retains input without accepting it. `admit(WalkEditLimits)` measures complete
N pairs and checked2N requested occurrences before source inspection. It refuses the complete
pair quota first,then an empty edit,then local self-span or the first self-anchor in pair/A/B order.
A report-only walk uses the read query;this edit type requests at least one pair. Admission errors
borrow the original edit and policy,so every input remains inspectable. These quotas cover pairs
and their request population,not earlier Vec allocation,graph lookup/currentness,profile/geometry
workspaces or whole-command resources. Two requests per pair are not two allocated output ids.

Only `AdmittedWalkSeamEdit` can call `bind_source` on an actual `SewingGraph`. Lookup selects that
graph's real named `SeamSpan`,then checks every requested anchor against its actual A/B source.
The edge must match the original side frame exactly,and the parameter must lie within inclusive
original range bounds. Missing target precedes pair checks;edge mismatch precedes parameter refusal.
No current-fragment alias,parametric remapping,sorting,deduplication or counterpart is guessed.

```rust
use sc_core::{construction::*,ontology::*};
fn request_balance_edit(operation:EntityId,span:EntityId,a:EdgeAnchor,b:EdgeAnchor,
    a_profile:NotchProfileBindings,b_profile:NotchProfileBindings)->WalkSeamEdit {
    let pair=WalkBalancePair::new(WalkBalanceNotch::new(a,a_profile),WalkBalanceNotch::new(b,b_profile));
    WalkSeamEdit::new(operation,span,vec![pair])
}
fn admit_edit<'a>(edit:&'a WalkSeamEdit,maximum_pairs:usize)
    ->Result<AdmittedWalkSeamEdit<'a>,WalkEditAdmissionError<'a>> {
    edit.admit(WalkEditLimits::new(maximum_pairs))
}
fn bind_edit<'a>(admitted:&'a AdmittedWalkSeamEdit<'a>,graph:&'a SewingGraph)
    ->Result<BoundWalkSeamEdit<'a>,WalkEditBindingError<'a>> {
    admitted.bind_source(graph)
}
```

These helpers keep actual error owners in the caller. Binding errors retain the admitted edit,
actual graph and original missing-span or pair/side/anchor/source-range cause. Anchor/range causes
borrow the actual original fields without an error-path allocation. Pair indices are
actual zero-based positions in the original request Vec,not invented recipe statement positions.
The private bound views borrow the actual graph/span and original pair fields. `source_side`
returns the real span-side field,whose `copy` is a physical-copy id. Direction,ease and stops stay
on that unchanged span. Repeated requests remain separate occurrences;profile origins stay symbolic.

Bounds and association prove neither geometric sewing correspondence nor current Design/graph/
ledger ownership. Different A/B source parameters can be retained as deliberate input without a
claim that they are matching balance positions. After an edge deletion,association can still bind
the unchanged original range while `SewingGraph::validate_current` returns its real range-repair
cause. G3 must measure correspondence/differential/ease,validate stops and create the actual edit;
G4 resolves profile representations. This stage reports no length and creates/publishes no notch.

[Native output contracts](drafting-walk-identities.md#explicit-exact-field-sharing-and-retained-revisions)
now declare explicit semantic identity,field and coalescing policies,including physical copies
sharing a pattern Piece. Actual mark results and atomic execution
remain G3/bus/full `.5f.4` work. No query result or2N request count substitutes for those results.
[Actual physical-copy/pattern association](drafting-walk-outputs.md) now supplies both real side
copies and owners before those output policies;shared ownership alone coalesces no mark.

## Native constrained seam-truing inputs

`construction::ConstrainedSeamTruing` retains deliberate refit intent and binds actual inputs;
it executes no curve fitting or editing operation.
[Ontology §3.2](../spec/ontology.md#32-drafting-operations) and Roadmap §3.1 require first-class
`true` on a SewingGraph correspondence. The [skirt waist](../spec/reference-skirt.md#5-drafting-recipe)
is trued as part of dart closure;its 18.5cm target does not define an independent edge-length
variant of `true`. Dart centre,legs,intake and waist truing belong to one atomic group under the
[command contract](../spec/command-layer.md#3-atomic-groups-and-the-granularity-of-undo).

This variant requests a refit of the selected side of a named canonical span while preserving
both selected range endpoint positions and all declared span stops. The request must explicitly
name the adjusted side A/B,the target pattern Piece and its length-fit tolerance source. Binding
retains the actual graph/span and the actual CutPlan copy for that side,and checks that the
copy belongs to the requested Piece. A copy id is not a Piece id. The policy forbids automatic
endpoint/stop movement and copy forking. An infeasible constrained fit must refuse.

Executing the request edits the canonical Piece,so every physical copy sees the change. Another
span,mark or constraint may observe that geometry. Execution must preserve those obligations or
refuse;selecting one physical copy cannot silently grant an isolated edit. Both sides can even
refer to copies of the same Piece. Binding this association grants no current-root,observer or
independent-counterpart proof. G3 must handle the coupled constraints rather than freezing a
counterpart that the same edit would change.

Keep the actual span's direction,ease allocation and stops. Signed ease means `length(A) −
length(B)`,independent of traversal direction. If the counterpart is fixed,adjusting A seeks
`length(B) + allocated_ease`;adjusting B seeks `length(A) − allocated_ease`. For example,at B=300mm
and allocated ease=12mm,the A target is312mm;at A=312mm,the B target is300mm. A child span consumes
its actual `SeamEase::Portion`,never the parent's full declared ease. Formula/profile resolution,
length-domain refusal and the actual feasible curve remain execution obligations. The caller
supplies neither a copied target length nor a replacement ease value.

The fitting budget is an explicit sourced,nonnegative Length tolerance for geometric refitting,
classified `ToleranceClass::GeometricApproximation`,with the original derivation retained.
Binding checks the requested source identity,class and nonnegative limit against the actual
borrowed `TruingFitProvider`. Length already enforces its magnitude domain;zero is retained as
an explicit requirement that execution may find infeasible. A nonempty derivation is metadata,not evidence that
the requirement or source registry is valid. T1 exact quantity/ease accounting stays separate;
T3/T4/T5 and chordal deviation do not become a seam-fit threshold. The formula context's intrinsic
`EPSgeo` remains unchanged. No default micrometre number is selected for this operation.

G3 must derive a length-fit error budget from actual curves and length evaluation;pointwise
distance or a bounding box alone does not establish an arclength bound. Source/evidence approval,
any explicit profile loosening and artifact effects follow the
[tolerance policy](../spec/units-and-tolerances.md#3-the-five-tolerance-classes). A tolerance check
cannot grant editing authority. Execution reports achieved differential,allocated ease and any
unexplained residual;it must never relabel unexplained mismatch as ease.

The constructor refuses local self-supply in span,Piece,tolerance-source order. It does not infer
global namespace roles from equal external ids. `bind_source` checks missing span,missing selected
copy,Piece association,tolerance-source identity,class,then negative limit in that order. Success
and errors retain the original request and actual graph/plan/provider;private bound fields cannot
be forged. `source_side`/`counterpart_side` borrow original span fields. `affected_copies` returns
actual matching CutPlan records in authored plan order;it is not a complete observer census.
No bound result can outlive its source owners. Whole graph lookup/currentness and execution
workspaces need their own program/command admission;this fixed-size request declares no quotas
for those later stages. No current-fragment alias or detached replacement span is silently used.

```rust
use sc_core::{construction::*,ontology::*};
use sc_units::Tolerance;
fn request_refit(operation:EntityId,span:EntityId,side:SeamSideId,piece:EntityId,
    fit_source:EntityId)->Result<ConstrainedSeamTruing,TruingInputError> {
    ConstrainedSeamTruing::new(operation,span,side,piece,fit_source)
}
fn bind_refit<'a>(edit:&'a ConstrainedSeamTruing,graph:&'a SewingGraph,plan:&'a CutPlan,
    source:EntityId,tolerance:&'a Tolerance)->Result<BoundConstrainedTruing<'a>,TruingBindingError<'a>> {
    edit.bind_source(graph,plan,TruingFitProvider { source,tolerance })
}
```

These helpers retain actual error owners in the caller;they do not copy a span/ease or invent a
budget value. [Actual complete copy/Piece association](drafting-truing-outputs.md) now binds both
original sides and preserves shared-Piece coupling. The companion's
[native declaration checkpoint](drafting-truing-piece.md#native-declaration-checkpoint-and-remaining-execution)
now covers held-source outputs,whole-Piece reconciliation and deterministic curve priority.
Endpoint-moving or copy-forking variants require explicit semantics before they can be accepted.
Endpoint/stop locks specify permissions;the explicit ordered curve policy defines selection intent.
No solver or complete v1 physical coverage follows from these association APIs.
Actual constrained fitting,all observers,atomic bus/undo and real result
verification remain G3/full `.5f.4` work;this contract closes none of those gates.

## Native two-Piece join source intent

The [native join reference](drafting-join-contracts.md) retains the complete original source,
inventory,exact-splice,metadata and population contracts,with their maintained examples.
Actual geometry,currentness,copy/contour/lineage effects and atomic publication remain separate.

## Names used in this chapter

Every API name and example name this chapter uses in code style, with its meaning. API meanings come from the code's own documentation; example names are not defaults.

| Name | Meaning |
| --- | --- |
| `AdmittedWalkSeamEdit` | Type: Complete admitted original edit;actual source binding follows through this stage only. |
| `affected_copies` | Method returning every plan copy of this Piece, in original plan order, including the counterpart when shared. |
| `alignment` | Accessor borrowing the actual `GrainAlignment` field, preserving its original angle intent. |
| `allocated_ease` | Illustrative ease amount allocated between two span sides in this example formula. |
| `bind_allowance` | Method or function: Associate the actual provided descriptor by identity and original source edge. The returned view borrows both original owners. |
| `bind_grainline` | Method or function: Bind the actual supplied record and inspect every original reference role for self-supply. Optional print references stay optional; repeated sources remain distinct role occurrences. |
| `bind_source` | Method binding the admitted edit to the actual graph span, selected copy and target Piece. |
| `BoundGrainlinePlacement` | Type: Authentic original placement/semantic-source association; neither values nor mark geometry. |
| `BoundOffsetConstruction` | Type: Authentic association of the supplied input and descriptor, without registry/currentness proof. |
| `check_argument` | Method or function: Check the actual whole amount against its explicit mode's required kind before any value. |
| `copy` | Field of a span side: the id of its physical copy. |
| `counterpart_side` | Method or function: Actual other side;shared Piece/copy constraints may prevent it remaining geometrically fixed. |
| `dependencies` | Private view borrowing the original declaration and exposing its actual underlying fields, in variant order. |
| `EPSgeo` | The specification's geometric tolerance `eps_geo`; its value is not changed by this operation. |
| `normalized` | Method that first measures the original documents, then returns the actual complete normalized owner. |
| `references` | Accessor returning the actual directed source occurrences, in original role order. |
| `rotation` | Constructor retaining an explicit rotation amount, which must be an Angle document; the spatial input also requires a pivot. |
| `separation` | Constructor retaining an explicit separation amount, which must be a distance document; no direction or value is inferred. |
| `SlashSpreadNormalizationLimits` | Type: Explicit complete node and conservative `1 + 2N` formula normalization allowances. Original input/parser/checker/runtime/allocator/CPU and whole geometry usage are separate. |
| `source_side` | Method returning the actual original selected span-side field, including its physical-copy id. |
| `TruingFitProvider` | Type: Required quantitative length-fit source;the actual tolerance remains borrowed and unchanged. Source-registry membership,derivation truth and fit/error-budget evidence remain separate. |
| `WalkBalanceNotch` | Type: One requested balance notch,without a client-supplied new identity or resolved profile values. |
| `WalkBalancePair` | Type: Both explicitly requested sides of one balance pair;equal parameters are not correspondence proof. |
