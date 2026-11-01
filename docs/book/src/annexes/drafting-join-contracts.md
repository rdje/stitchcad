# Native Piece join contracts

This reference continues the [editing inputs](drafting-edit-inputs.md). Native declarations
retain actual source/proposal data;geometry,currentness and command publication remain separate.

## Native two-Piece join source intent

`construction::JoinPiecesAtSeam` retains an operation id,a canonical span id and two explicit
pattern-Piece ids in that span's A/B roles. This variant requires distinct pattern Pieces;
it does not mean two physical copies or two ranges of one Piece. Those SewingGraph relations
remain legal in their own domains. The request declares join intent without selecting output
metadata,copy consumption,source retirement,pose or a geometric weld/refit algorithm.

Local checks refuse self-span,then self-Piece A/B,then a repeated Piece. Input order is not sorted
or inferred from seam direction. `bind_source` reads the actual named span in the provided graph,
then handles A completely before B. For each side it requires the actual CutPlan copy,checks its
Piece id against the requested role,and finds exactly one matching Piece in the provided slice.
Missing span precedes all side checks;within a side,missing copy precedes Piece mismatch,then
missing or duplicate named Piece. Equal content cannot make duplicate identities unambiguous.
Duplicate indices name actual positions in that original slice,not recipe statement ordinals.

```rust
use sc_core::{construction::*,ontology::*};
fn request_join(operation:EntityId,span:EntityId,a_piece:EntityId,b_piece:EntityId)
    ->Result<JoinPiecesAtSeam,JoinInputError> {
    JoinPiecesAtSeam::new(operation,span,a_piece,b_piece)
}
fn bind_join<'a>(edit:&'a JoinPiecesAtSeam,graph:&'a SewingGraph,plan:&'a CutPlan,
    pieces:&'a [Piece])->Result<BoundJoinPiecesAtSeam<'a>,JoinBindingError<'a>> {
    edit.bind_source(graph,plan,pieces)
}
```

Success and errors retain the actual request,graph,plan and whole provided Piece slice. The
private `BoundJoinSide` view borrows its complete bound join,original span-side field,actual copy
and uniquely named provided Piece. Its `source_copies` iterator returns actual matching plan
records in authored order. Labels,material assignments including unresolved reasons,quantity,
mirroring,fold intent,face,layer,boundaries and construction content remain on those original
Pieces. No field from A silently overrides B. Direction,ease including retained portions,and
stops remain on the actual span. No joined identity or boundary is produced.

This proves supplied-source association only. Unrelated duplicate Piece ids,complete inventory
resources,current Design membership and source-edge/boundary ownership need their own admission
and current graph/ledger validation. After source deletion the unchanged association can still
bind while current validation refuses. Original-frame references are not silently replaced with
current fragments. The provided Piece records are not thereby certified as a current Design.

### Admit the complete original join inventory

Bind the source join with the actual `PieceInventory::pieces()` slice,then call
`BoundJoinPiecesAtSeam::bind_copy_inventory` with that same inventory and explicit
`JoinCopyInventoryLimits::new(C,I,W)`. C counts every original CutPlan copy;I counts every
inventory member,including unrelated Pieces. Checked W=I+3C bounds temporary logical validation
entries,not allocator bytes,prior input storage or geometry work.

```rust
use sc_core::{construction::*,ontology::PieceInventory};
fn admit<'a>(source:&'a BoundJoinPiecesAtSeam<'a>,inventory:&'a PieceInventory,
    limits:&'a JoinCopyInventoryLimits)
    ->Result<BoundJoinCopyInventory<'a>,JoinCopyInventoryError<'a>> {
    source.bind_copy_inventory(inventory,limits)
}
let limits=JoinCopyInventoryLimits::new(4,3,15);
assert_eq!(limits.workspace_entries(),15);
```

Whole C,I,W quotas precede source association or canonical field scans/allocation. With C2/I2,
C1 refuses `Copies`,I1 refuses `Pieces`,and W7 refuses required8 as `Workspace`. Equality permits
the next check. `WorkspaceOverflow` retains a nonrepresentable complete I+3C bound.

`SourceInventory` refuses a source bound from a different slice,including a separately cloned
equal-content inventory. No source records are silently replaced or compared field by field.
The actual immutable PieceInventory already supplies uniqueness/reciprocal companion checks.
Canonical CutPlan validation then rechecks every original copy against those exact Pieces;
`Inventory(CutPlanError)` retains its real identity,quantity or orientation cause. Companion
consistency does not imply a matching copy population.

For I3/C4,the exact logical workspace allowance is15. An unrelated Piece and its copy count;
a reflected selected copy retains its orientation,and every other copy of either source remains
available in original plan order. No copy is consumed,paired or assigned to a result here.
Success lends actual `source()`, `inventory()` and `limits()` and the checked `workspace_bound()`.
Errors retain those same whole parents and first cause. Both A/B views still borrow the original
Piece/copy fields,span,ease and stops. Work is O((C+I)log(C+I+2));cached storage is constant.
Earlier graph/source lookup resources,current Design membership,contour fields and provider bodies
remain separate. This admission chooses no metadata,joined identity,placement,weld or refit.

### Require an explicit exact nondeforming splice

`JoinExactSpliceRequirement` declares one join domain:remove both selected coincident seam banks
without stretching,gathering,refitting,snapping or forking a copy. This is an explicit requirement,
not an assertion that a SewingGraph correspondence can already form one flat Piece.

The actual allocated ease must resolve to zero. A retained Portion consumes its actual allocation,
not the full parent's amount;symbolic Profile/formula metadata supplies no zero-ease proof. A
resolved nonzero amount contradicts this domain. Unavailable/unsupported/uncertified/resource/
cancellation causes must survive;they are not proof of geometric incompatibility or permission
to discard ease. Other deformation policies require separately declared variants.

`JoinSplicePlacement::KeepOriginal` requires both original resolved placements to meet without
moving either Piece. `RigidAlign { fixed,orientation }` names A/B as the fixed authored Piece
and requests a unique nondegenerate rigid alignment of the other from both actual endpoint pairs.
`JoinMovingOrientation::Preserve` or `Reflect` explicitly selects the moving authored transform's
parity. Physical-copy reflection,Piece traversal and seam correspondence remain independent.
Execution must reconcile the actual copy transforms;the native policy guesses none of them.

```rust
use sc_core::{construction::*,ontology::SeamSideId};
let request=JoinExactSpliceRequirement::new(JoinSplicePlacement::RigidAlign {
    fixed:SeamSideId::B,orientation:JoinMovingOrientation::Preserve,
});
fn require<'a>(inventory:&'a BoundJoinCopyInventory<'a>,request:&'a JoinExactSpliceRequirement)
    ->RequiredJoinExactSplice<'a> {
    inventory.require_exact_splice(request)
}
assert!(matches!(request.placement(),JoinSplicePlacement::RigidAlign { fixed:SeamSideId::B,.. }));
```

`require_exact_splice` binds the complete original inventory and actual request in constant work/
space without scanning,allocating,evaluating or invoking a provider. Success here means required
intent only. `RequiredJoinExactSplice` lends actual `inventory()`, `requirement()`, `source()` and
original `ease()`. Neither placement nor parity has a default.

`endpoint_pairs()` lends exactly two authentic pairs in ascending A-held parameter order and
reverse. Each pair retains its whole `context()`, `end()` (`AFrom`/`ATo`) and actual borrowed
`source_side(A/B)`. `anchor(A/B)` copies the original bounded held anchor:Same pairs A.from with
B.from and A.to with B.to;Opposite pairs A.from with B.to and A.to with B.from. Fixed B or a
reflected selected copy never rewrites these held parameters.

For A0–1/3 and B2/5–1 with Opposite correspondence,the pairs are `(A0,B1)` then `(A1/3,B2/5)`.
They declare no rescaled/affine interior map,current alias or physical position. Actual geometry
must prove entire-bank coincidence and valid complete topology,including all remaining outside
fragments and holes. Endpoint alignment alone is insufficient. Ambiguous or degenerate rigid
alignment must retain its actual refusal;no extra anchor or arbitrary pose is selected.

Focused native checks cover these declared choices and authentic partial/eased/reflected source
views. Actual fitting/deformation,geometry,currentness,observer/authority truth and publication
remain separate;no full-envelope or independent approval follows.

### Select joined metadata from actual A/B or authored fields

`JoinPieceMetadataPolicy` requires four independent `JoinPieceFieldIntent<T>` choices:material,
assembly layer,label text and cutting face. Each is `FromSide(A/B)` or `Authored(value)`. No side
wins automatically;source id order,fixed placement and physical-copy reflection select no field.
All four choices are explicit even when A and B happen to carry equal values.

```rust
use sc_core::{construction::*,ontology::*};
let policy=JoinPieceMetadataPolicy::new(
    JoinPieceFieldIntent::FromSide(SeamSideId::A),
    JoinPieceFieldIntent::Authored(2),
    JoinPieceFieldIntent::Authored(LabelText {
        name:"joined front".into(),size:"M".into(),fabric:"cotton".into(),colorway:"blue".into(),
    }),
    JoinPieceFieldIntent::FromSide(SeamSideId::B),
);
fn bind<'a>(splice:&'a RequiredJoinExactSplice<'a>,policy:&'a JoinPieceMetadataPolicy,
    limits:&'a JoinPieceMetadataLimits)->Result<BoundJoinPieceMetadata<'a>,JoinPieceMetadataError<'a>> {
    splice.bind_metadata(policy,limits)
}
assert!(matches!(policy.material(),JoinPieceFieldIntent::FromSide(SeamSideId::A)));
```

`bind_metadata` selects original borrowed fields without cloning,normalizing or comparing their
contents. `FromSide` lends the actual original Piece field;`Authored` lends the actual field in
the original policy. Material assignment retains its exact assigned id or unresolved reason.
The four policy getters retain those origin choices separately from the selected values.

`JoinPieceMetadataLimits::new(B)` bounds the complete selected result's UTF-8 bytes:all four
label strings plus its unresolved material reason,if any. Cached String lengths are summed with
checked addition before any blank-field scan. `TextOverflow` retains a nonrepresentable full sum;
`TextBytes { required,allowed }` precedes field completeness. Previously owned policy/source
storage and unselected text have separate obligations;this check neither scans nor bounds them.

Label `front/M/cotton/blue` contributes16 bytes. With selected unresolved reason `source A pending`,
the total is32;allowance31 refuses required32. A selected assigned material contributes no reason
bytes. `合片/M/linen/蓝` contributes15 UTF-8 bytes,not a character count;reason `new β` adds6 for21.

After whole B admission,the existing canonical Piece rules check label name,size,fabric,colorway
in that order,then require a nonblank unresolved-material explanation. Unicode whitespace still
counts as blank. `IncompleteLabel(LabelField)` and `UnexplainedMaterial` retain the original
whole splice,policy and limits;no accepted field prefix escapes.

`BoundJoinPieceMetadata` lends actual `splice()`, `policy()`, `limits()`,selected `material()`,
`layer_index()`, `label()` and `cutting_side()`,plus complete `text_bytes()`. Binding takes O(B+1)
work and constant space,with no allocation,registry lookup,provider call or geometry work.
These are locally complete fields. Assigned material existence/kind,state/profile compatibility,
size truth,face/parity compatibility,cut/fold/mark reconciliation and authority remain separate.
No Piece,physical copy or printed quantity/hand/fold indicator is created here.

### Declare one canonical result cut population

`JoinCutPopulation::new(quantity,mirroring)` requires the result's total physical-cut quantity
and canonical `Mirroring` explicitly. It inherits neither source's quantity. Raw construction
retains even invalid values so a contextual refusal can show the complete original declaration.

```rust
use sc_core::{construction::*,ontology::Mirroring};
let population=JoinCutPopulation::new(4,Mirroring::MirroredPairs);
assert_eq!(*population.quantity(),4);
fn bind<'a>(metadata:&'a BoundJoinPieceMetadata<'a>,population:&'a JoinCutPopulation)
    ->Result<BoundJoinCutPopulation<'a>,JoinCutPopulationError<'a>> {
    metadata.bind_cut_population(population)
}
```

`bind_cut_population` refuses `ZeroQuantity` first,then
`UnpairedQuantity { quantity }` for an odd MirroredPairs total. Four means four cuts including
both hands,not four pairs;three is retained and refused rather than rounded or doubled. A separate
`PairMember` retains its actual hand and companion id;identity/reciprocity checks still follow.

Success lends actual `metadata()`,the whole original `population()` and original borrowed
`quantity()`/`mirroring()` fields. Errors retain that same complete metadata/source/splice context,
original population and first `refusal()`. No default population,second quantity or print view is
introduced. The later printed quantity/hand/fold indicators must derive from the same canonical
cut and contour declarations.

Binding performs constant work/space with no allocation,scan,identity issuance or source mutation.
Even a locally valid large total declares intent;it allocates no copies and proves no source
consumption,result membership,orientation,full CutPlan or material/geometry compatibility.
Complete copy actions,companion binding,fold/print reconciliation and authority remain required.

Remaining output contracts must explicitly reconcile fold/mark ownership,copy correspondence and
copy mappings,source retirement/reference lineage and the permitted join variant before family
closure. Partial spans,declared ease and physical-copy orientation cannot be ignored. Sewing
correspondence alone does not establish congruent boundaries,a valid placement or a simple
joined Piece. The explicit exact-splice policy above supplies intent;actual geometry/semantic checks,
all observers and actual G2/G3/bus/undo/results remain separately owned execution work.

Output roles,source candidates and their native admission/query contracts are in the
[output reference](drafting-output-contracts.md).

## Admit every original A/B source occurrence

`BoundJoinCutPopulation::admit_source_pieces` inspects the actual original A/B Pieces under
`JoinSourcePieceLimits::new(H,R)`. H counts every original hole group across both Pieces. R counts
every boundary,hole and construction occurrence plus each separately declared fold field,including
unrelated edges and repeated construction uses. Physical-copy quantity does not multiply this
pattern-source population;the complete CutPlan keeps its earlier separate admission.

```rust
use sc_core::construction::*;
fn admit<'a>(population:&'a BoundJoinCutPopulation<'a>)
    ->Result<BoundJoinSourcePieces<'a>,JoinSourcePieceError<'a>> {
    population.admit_source_pieces(JoinSourcePieceLimits::new(3,11))
}
```

Checked full H addition and `Holes` quota precede any child-hole length scan. Then a length-only
sum checks complete R and its `Occurrences` quota before any edge identity is inspected.
`HoleOverflow`/`OccurrenceOverflow` preserve nonrepresentable whole counts. All earlier owned
input,lookup,inventory,metadata and result-population obligations remain separate.

If A has2 boundary edges,2 one-edge holes,3 construction uses and1 separate fold,its R is8.
B with1 boundary edge,1 one-edge hole and1 construction use adds3:whole H3/R11. H2 refuses
required3 even with R0;H3/R10 refuses required11. H3/R11 proceeds to original field inspection.

Every edge prerequisite must be produced by an operation other than this same join operation.
`SelfOperation { side,location }` retains the first cause in A/B,then boundary/hole/construction/
fold order. It is checked only after full H/R quotas. Errors retain the whole original
`population()`,explicit `limits()` and first `refusal()`. `refused_source()` lends the actual
original refused EdgeRef field for SelfOperation,in constant work;count/quota causes return `None`.
No edge payload is substituted or accepted side prefix returned.

`BoundJoinSourcePieces` lends actual `population()`,original `limits()`,checked `hole_count()`/
`occurrence_count()` and original `piece(A/B)`. `occurrences()` lends all original fields in A/B,
then boundary,hole,construction,fold order,and reverse. Each authentic `JoinSourceOccurrence`
retains whole `context()`,original `side()`/`location()`,actual `piece()`,selected `copy()` and
original span `source_side()`. `field()` is the actual directed field or separate undirected fold;
its `edge()` borrows the exact original persistent reference. No traversal direction is fabricated.

Admission and full traversal take O(H+R+1) work and constant extra space,without allocation,
label/material text scanning,provider invocation or geometry evaluation. Field locations are
original indices,not persistent identities or current parameters. No sorting or deduplication occurs.
This source inventory proves no literal selected cut-bank membership,current-root/alias resolution,
entire namespace freshness,coincidence or valid joined topology. A retired source can still lend
its old held fields while actual current graph validation refuses;original references are not remapped.

## Bind both literal held cut banks

`BoundJoinSourcePieces::bind_cut_banks` binds A then B under the same immutable complete H/R
admission. A bank must be an actual boundary or hole field on that role's original supplied Piece,
with the same literal EdgeRef as its actual selected range. A construction match or separate fold
declaration cannot substitute. Canonical Piece birth already excludes duplicate cut-edge identities
across its cut loops;the native binding does not sort or choose among equal-looking fields.

```rust
use sc_core::construction::*;
fn bind<'a>(sources:&'a BoundJoinSourcePieces<'a>)
    ->Result<BoundJoinCutBanks<'a>,JoinCutBankError<'a>> {
    sources.bind_cut_banks()
}
```

`NoExactCutBank { side }` retains the first missing A/B role. The error lends actual `sources()`,
the first `refusal()` and the original refused `source_range()` field. A found A bank does not
escape a B refusal. This cause concerns the literal held variant;it does not prove an edge globally
absent or invalidate a history alias. Actual current projection/ownership remains separately required.

`BoundJoinCutBanks` retains actual complete `sources()`. `banks()` lends both authentic banks in
canonical A/B order and reverse,independent of CutPlan order;`bank(side)` queries one canonical role.
Each `JoinCutBank` retains whole `context()`,original `side()` and authentic source `occurrence()`.
`directed()` lends the actual original boundary/hole field and unchanged Piece traversal.
`range()` and `source_side()` lend the actual selected span fields;`piece()` and `copy()` lend the
original owning Piece and selected physical copy. No separately supplied field or range is attached.

For selected1/4–3/4,the directed contour still names its original whole edge,while `range()` keeps
those exact partial bounds. Neither expands the selected interval nor removes0–1/4 or3/4–1.
Outside-fragment attribution and complete proposed topology remain output requirements. A reflected
copy or fixed B placement never reverses these held bounds or chooses a new contour direction.

Binding takes O(H+R+1) work with two fixed cached associations,no allocation,string scan,provider
call or geometry evaluation. Literal membership proves no currentness,geometric coincidence,
interior parameter map,valid joined contour,retirement or authoring authority. Actual current graph
validation can refuse a retired source while these immutable original bank fields remain lendable.

## Bind the actual complete Edge role catalogue

`BoundJoinCutBanks::bind_edge_roles` binds a distinct admitted `AdmittedEdgeOutputRoleCatalog`
to the original join operation. Construct the catalogue through the shared
[Edge role contract](drafting-output-contracts.md);Point and semantic-Notch catalogues are different
types. Machine keys do not classify geometry or assign roles to a particular bank by their spelling.

```rust
use sc_core::construction::*;
fn bind<'a>(banks:&'a BoundJoinCutBanks<'a>,catalog:&'a AdmittedEdgeOutputRoleCatalog<'a>)
    ->Result<BoundJoinEdgeRoles<'a>,JoinEdgeRoleBindingError<'a>> {
    banks.bind_edge_roles(catalog,EdgeOutputRoleCatalogLimits::new(3,2,20))
}
```

The explicit association limits bound **all** definitions D,active occurrences A and original
machine-key bytes B,including inactive/unrelated definitions. Counts and B are cached by the actual
original catalogue admission. Complete D,A,B quotas precede operation equality;this binding does
not rescan keys,filter to requested outputs,sort declarations or allocate another index.

For definitions `bank_a`,`bank_b`,`inactive`,D3/A2/B20 is exact. D2 refuses `Definitions` before
A or key checks;A1 refuses `Active`;B19 refuses required20 as `KeyBytes`. With all quotas met,
a catalogue owned by1001 refuses `Operation` for actual join1000. These are association limits;
the earlier catalogue's owned storage/admission and complete source/context resources remain separate.

`BoundJoinEdgeRoles` lends actual `banks()`,actual whole `catalog()` and original `limits()`.
Catalogue membership views still lend their original definitions and active order;inactive
definitions remain present. Errors retain those same whole parents,policy and first `refusal()`.
Binding takes constant work/space without scans or allocation.

Empty or wholly inactive catalogues can bind to the same actual operation. This is metadata only:
both source banks and the explicit positive result population remain required. It proves no empty
joined result,no-op,source/output attribution,role history,global freshness,issued identity,
geometric correspondence,valid topology or publication authority.

## Declare a canonical result Piece identity

`BoundJoinEdgeRoles::declare_piece_identity` borrows one explicit canonical result Piece-id field.
It neither allocates nor derives that id from geometry,labels,source ids or statement positions.
This is native declaration data;creation requests still rely on the trusted host to issue identities.

```rust
use sc_core::{construction::*,ontology::EntityId};
fn declare<'a>(roles:&'a BoundJoinEdgeRoles<'a>,piece_id:&'a EntityId)
    ->Result<DeclaredJoinPieceIdentity<'a>,JoinPieceIdentityError<'a>> {
    roles.declare_piece_identity(piece_id,JoinPieceIdentityLimits::new(1))
}
```

`JoinPieceIdentityLimits::new(S)` bounds every selected-span stop occurrence **plus** every
original stop occurrence retained by a Portion ease source. Checked total and `Stops` quota
precede candidate and stop-field inspection;`StopOverflow` retains a nonrepresentable sum.
The original complete provided C/I inventory/plan and A/B H/R policies remain in force.

With one selected stop and that same landmark retained in the original ease snapshot,S is2,
not1. S1 refuses required2 even if the requested id also aliases the join operation. A retained
tail can have no selected stops while its original source still contributes1;that occurrence
cannot disappear merely because it lies outside the current selected portion.

After resource admission,the listed local exclusions run in this order:actual join operation,
every supplied inventory Piece,requested self-companion,original graph,selected span,retained
ease-source span,every original physical copy,A/B edge-producing operations,input assigned
Materials,selected result Material,selected stops,then retained-source stops. Inventory and plan
indices retain original supplied order;source edges retain original A/B/family order.

`Operation`,`InventoryPiece`,`SelfCompanion`,`Graph`,`Span`,`RetainedSpan`,`Copy`,
`SourceOperation`,`SourceMaterial`,`ResultMaterial` and `Stop` preserve the actual first cause.
`JoinIdentityStopOrigin` distinguishes selected and retained lists. Repeated stop references count
in S even though the selected list wins when both lists contain the refused landmark.

Errors lend actual `roles()`,original `piece_id()`,explicit `limits()` and first `refusal()`.
`refused_source()` lends the actual original edge only for SourceOperation;`refused_stop()` lends
the actual original stop only for Stop. Other causes return `None`;no replacement field is attached.
`DeclaredJoinPieceIdentity` retains the same actual roles,id field and limits and reports the
complete `stop_occurrences()` count. It grants no companion existence/reciprocity or publication proof.

Work is O(C+I+H+R+S+1),with constant extra space,no allocation,text scan,registry/provider call
or identity issuance. Formula/Profile source ids are not classified from equal external values.
An unlisted id can pass these local checks,including explicit zero or a value equal to an unread
external source reference;that supplies no global kind,existence,state or freshness certificate.
Actual current/historical namespace,nonreuse,reference resolution and authoring authority remain
mandatory host/program obligations before geometry execution or publication.
