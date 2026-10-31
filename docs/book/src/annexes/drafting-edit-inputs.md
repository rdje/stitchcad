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

This stage issues no new Piece or geometric output tags and mutates nothing. Explicit bridge/trim
policy,fullness/intake effects and actual new/retired topology remain `.1.c.slash.outputs` work;
there is no default connector across an opened cut. G2/G3 must supply those results and their
source mappings before command publication. Full operation integration remains `.5f.4` work.

## Native reflected Piece-copy inputs

`construction::MirrorPieceCopy` declares a reflected copy of the named existing Piece. The type
makes copy intent explicit. It retains an operation id,source Piece id and the authored `MirrorAxis`:
`ThroughPoints { start,end }` uses two actual PointRefs;`AlongRange` uses an actual DirectedRange.
No axis mode is inferred from geometry or formula values.

```rust
use sc_core::{construction::*,ontology::*};
let point=|id|PointRef::new(EntityId::from_bits(id),LocalTag::FIRST);
let input=MirrorPieceCopy::new(EntityId::from_bits(10),EntityId::from_bits(50),
    MirrorAxis::ThroughPoints { start:point(99),end:point(80) })?;
assert_eq!(input.source_piece(),EntityId::from_bits(50));
assert_eq!(input.dependency_count(),2);
# Ok::<(),Box<dyn std::error::Error>>(())
```

Local self-supply refuses the source Piece first,then start/end or range in original field order.
Private dependency views borrow the actual input and underlying axis fields. Repeated point uses
remain separate occurrences;directed range bounds/traversal and point source order stay authored.
The iterator covers axis occurrences only;the source Piece is a separate program dependency.
Reversing a valid geometric axis may preserve its eventual reflection,but this input stage does
not rewrite authored sources to establish that equality.

The constructor certifies neither a current Piece nor a usable axis. G2/program checks must prove
source existence/common frame and axis nondegeneracy/straightness. Repeated point references can
be retained as authored input without proving a valid reflection axis. A curved range cannot be
silently replaced with its chord or tangent.

No new Piece identity,edge/point tags or geometry is produced. Copy metadata,labels,materials,
handedness,cut quantities,pair reciprocity and sewing/reference mappings require explicit output
policy under `.1.c.mirror.outputs`,followed by the bus's identity allocation and atomic publication.
The existing `Mirroring` cut-plan field supplies no drafting reflection result. Current coverage,
reflection/error budgets and the stable output manifest remain actual G2/G3/execution obligations;
full construction verification stays deferred under `.5f.4`.
