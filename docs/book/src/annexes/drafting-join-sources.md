# Native join sources and policies

This companion retains the complete source intent and authored policy declarations.
Continue with [original fields and output contracts](drafting-join-contracts.md#admit-every-original-ab-source-occurrence).

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

Continue with [complete original source occurrences](drafting-join-contracts.md#admit-every-original-ab-source-occurrence).
