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

## Declare explicit result physical-copy requests

After [complete original fold actions](drafting-join-contracts.md#reconcile-every-original-fold-declaration),
`JoinResultCopyManifest` retains the operation and ordered explicit `CutCopyDefinition` requests.
Every request supplies its stable physical-copy id,declared result Piece id and authored/reflected
orientation. No copy id comes from an ordinal,quantity expansion,source id reuse or a validator.
These are requests,not born current CutCopies or a complete result CutPlan.

```rust
use sc_core::{construction::*,ontology::*};
fn requested(operation:EntityId,copy:EntityId,piece:EntityId,orientation:CopyOrientation)->JoinResultCopyManifest {
    JoinResultCopyManifest::new(operation,vec![CutCopyDefinition{id:copy,piece,orientation}])
}
fn copies<'a>(manifest:&'a JoinResultCopyManifest,folds:&'a DeclaredJoinFoldSources<'a>)
    ->Result<DeclaredJoinResultCopies<'a>,JoinResultCopyError<'a>> {
    manifest.admit(folds,JoinResultCopyLimits::new(3,3,1,7))
}
```

`JoinResultCopyLimits::new(C,I,Q,W)` admits every original physical copy C,every original Piece I
and every raw result request Q before fields or allocation. C/I include unrelated,companion and
unselected source members. Checked C+I+Q bounds the complete original-copy,original-Piece and
result-id indexes;logical entries are not allocator bytes. Earlier immutable input/inventory,
identity,population,construction and fold policies remain separate.
For C3/I3/Q1,W7 is exact:C2 refuses `Copies`,I2 `Pieces`,Q0 `Results`,W6 `Workspace`,before an
incorrect operation,quantity mismatch or invalid first copy field.

With quotas met,`Operation` precedes canonical quantity representability and `Count`. Q must
exactly match the one authored result quantity already bound through `JoinCutPopulation`.
An empty list cannot imply that quantity's copies;an extra request is not silently discarded.
Each original request then checks `Piece`, `Duplicate`, `OperationIdentity`, `ResultPieceIdentity`,
`OriginalPieceIdentity`, `OriginalCopyIdentity`. Every request must explicitly name the declared
result Piece. Result-copy ids are unique and locally distinct from the actual operation,result
Piece,and all original Piece/copy ids,including unrelated members and copies proposed for consumption.

Those exclusions cover the actual supplied Piece/copy namespaces only. They do not prove global
Design namespace/kind,current/historical freshness,reserved identities or issuance. The declared
result Piece is itself an unissued request. Actual current identity workflow and complete staged
birth/reference/retirement checks remain mandatory before effects;no validator allocates ids.

`JoinResultCopyError` retains whole `manifest()`,actual complete `folds()`,original `limits()` and
first `refusal()`. Request-specific causes lend the actual `refused_copy()` field. Original namespace
collisions additionally lend the actual `original_piece()` or `original_copy()` inventory member,
without reconstructing equal content. Whole resource/operation/count causes lend no request/member.
No accepted prefix,implicit orientation or replacement copy escapes refusal.

`DeclaredJoinResultCopies` retains those whole parents and reports `source_copy_count()`,
`source_piece_count()`, `copy_count()` and `workspace_entries()`. `source_copy(id)`/`source_piece(id)`
lend actual complete original inventory members by stable identity. `copies()` preserves original
request order and reverse;`copy(id)` queries an actual explicit result request. Each
`JoinResultCopyRequest` lends actual owner,original ordinal and `definition()`;
`id()`, `piece_id()` and `orientation()` retain the original stable identity/Piece/orientation fields.
A request view is not a born CutCopy or a transformed physical placement.

This stage checks quantity and scoped identity bookkeeping. It retains a reflected request even
beside an authored Single population;the full canonical candidate CutPlan must still reject that
orientation mismatch. Two result requests beside only one available A/B source pair likewise do
not establish a permitted splice:complete original actions and exact no-fork A+B pairing must
follow. No source copy is consumed,paired,reassigned or duplicated here.

Work is O(C log(C+2)+I log(I+2)+Q log(Q+C+I+2)+1),with C+I+Q new index entries. No provider,
evidence,registry or geometry call occurs. Source actions,pairing,complete candidate inventory,
orientation/mirroring,copy transforms,consumer repair,currentness,identity issuance and atomic
authority remain separately required.

## Declare every original physical-copy action

`JoinCopySourceManifest` retains the operation and complete ordered
`JoinCopySourceAssignment::new(expected,intent)` rows. `expected` is the original whole
`CutCopyDefinition`:its persistent id selects the actual original copy,and Piece/orientation fields
must remain exact. It is a guard,not a replacement copy. Every row explicitly requests
`JoinCopySourceIntent::RetainOriginal` or `JoinTo { result_copy }`;there is no default.

```rust
use sc_core::{construction::*,ontology::*};
fn action(expected:CutCopyDefinition,intent:JoinCopySourceIntent)->JoinCopySourceAssignment {
    JoinCopySourceAssignment::new(expected,intent)
}
fn sources<'a>(manifest:&'a JoinCopySourceManifest,results:&'a DeclaredJoinResultCopies<'a>)
    ->Result<DeclaredJoinCopySources<'a>,JoinCopySourceError<'a>> {
    manifest.admit(results,JoinCopySourceLimits::new(3,3,6))
}
```

`JoinCopySourceLimits::new(C,M,W)` admits all actual original copies C,all raw action rows M and
checked2M new source-index/association entries before fields or allocation. C includes unrelated
and unselected copies. Earlier complete original/result inventory and construction/fold resources
remain separate;logical entries are not allocator bytes. For C3/M3,W6 is exact:C2 refuses `Copies`,
M2 `Assignments`,W5 `Workspace`,before an incorrect operation or malformed first expected field.

With quotas met,`Operation` precedes `Count`:M must equal the complete actual original copy
population. Each original row then checks `DuplicateCopy`, `MissingCopy`, `SourceField`,followed
by intent-specific ownership and target requirements. Exact unique actual membership covers every
original copy. Piece reassignment or a changed authored/reflected expectation refuses SourceField;
matching geometry or a surviving copy id cannot replace the whole original guard.

RetainOriginal preserves the exact actual original id/Piece/orientation fields. JoinTo first
requires a copy of an actual source A/B Piece (`ForeignPiece` otherwise),then an actual explicit
result-copy request (`UnknownResult` otherwise). An unrelated copy must be retained;it cannot
silently become material for this two-Piece splice. A catalogue-like id or original source-copy id
cannot replace an actual requested result. Copy orientation stays independent of edge traversal,
parameter requests and physical placement;none is transformed or inferred here.

`JoinCopySourceError` retains actual whole `manifest()`,`results()`,original `limits()` and first
`refusal()`. Row causes lend the actual `refused_assignment()` and known actual `refused_copy()`,
if that selector exists. UnknownResult additionally lends its actual authored `refused_result()`
id field. Whole resource/operation/count causes lend none of these fields. No accepted prefix,
replacement original copy or ignored suffix escapes refusal.

`DeclaredJoinCopySources` retains those parents and reports `copy_count()` and
`workspace_entries()`. `assignments()` preserves authored order and reverse;`assignment(id)`
queries an actual original copy action by stable identity. Each `JoinCopySourceAction` lends its
actual owner,original ordinal/raw assignment,actual canonical `source()` copy and
`source_definition()` field. `side()` reports the actual original A/B Piece role,or None for an
unrelated retained copy. `intent()` remains the original authored field;`result()` lends the actual
requested result only for JoinTo. RetainOriginal never implies a replacement target.

This stage can retain complete requests that preserve even the selected A/B copies. That is not
an accepted no-op or an executed join:the [exact no-fork pairing check below](#require-exact-selected-ab-copy-pairing) requires the actual
selected A/B to contribute to the same result and exactly one A+B source pair per result. Additional
source copies cannot be implicitly paired,duplicated or silently discarded. All original id/Piece/
orientation fields remain available for complete candidate inventory and consumer reconciliation.

Work is O(M log(M+C+Q+2)+1) for Q actual result requests,with2M new entries. No provider,evidence,
registry,geometry,physical copy transform,currentness,identity issuance or atomic effect is supplied.
Exact selected correspondence/pairing,canonical inventory/CutPlan,copy/consumer requirements,
achieved geometry and authority remain separately required.

## Require exact selected A+B copy pairing

`DeclaredJoinCopySources::require_pairs` checks the actual selected physical-copy pair and every
explicit result-copy request. `JoinCopyPairLimits::new(C,Q,W)` admits all original copies/actions C,
all requested results Q and checked3Q peak workspace before selected fields,scans or allocation.
Peak workspace counts a result index,staging slots and final associations;successful retained
storage is2Q. Earlier immutable inventory/source/fold/construction policies remain separate.

```rust
use sc_core::construction::*;
fn pairs<'a>(sources:&'a DeclaredJoinCopySources<'a>)
    ->Result<RequiredJoinCopyCorrespondence<'a>,JoinCopyPairError<'a>> {
    sources.require_pairs(JoinCopyPairLimits::new(5,2,6))
}
```

For C5/Q2,W6 is exact:C4 refuses `Copies`,Q1 `Results`,W5 `Workspace`,before any selected-copy
or group inspection. With quotas met,the original selected A action must JoinTo a result,then B
must JoinTo a result (`SelectedRetained` otherwise,in A-before-B order). Their actual requested
result identities must agree (`SelectedResultMismatch` otherwise). An accepted complete retention
request is therefore not sufficient to accept this join or certify a no-op.

Every original JoinTo action then contributes its actual A/B role to the explicitly named result.
The first repeated role for one result refuses `DuplicateSide`,retaining original first/second
source-row positions. Results are then checked in **original requested order**,A before B;
`MissingSide` refuses a result without one actual contributor of each role. There is no inferred
pairing by list position,sorted ids,geometry,orientation or quantity. Complete unique original-copy
actions already prevent one physical copy from contributing twice;this check requires exactly one
A and one B per result. Unrelated and retained copies stay on the whole original action parent.

For example,explicitly assign selected A200 and B201 to result6000,and another actual A203/B204
pair to result6001. Both pairs retain their actual original definitions and requested orientations.
Requesting result6001 without both contributors refuses;assigning two A copies to result6000 also
refuses. If selected B201 names result6001 while selected A200 names6000,the selected mismatch
refuses before either group's later missing/duplicate cause.

`JoinCopyPairError` retains actual whole `sources()`,original `limits()` and first `refusal()`.
Selected-retention/mismatch causes lend actual `selected_source(side)` actions. `refused_source()`
lends the actual selected retained or second duplicate source;missing contributors are never
fabricated. Duplicate/missing-side causes lend the actual `refused_result()` request. Whole quota
causes lend no selected/source/result fields. No accepted pairing prefix escapes refusal.
`SourceInvariant`/`ResultInvariant` denote internal closed-parent faults requiring diagnosis;
they are not physical incompatibility evidence. Neither occurred in the focused checks.

`RequiredJoinCopyCorrespondence` retains those whole parents,`limits()` and peak
`workspace_entries()`. `selected_result()` lends the actual request shared by original selected
A/B copies. `pair_count()` reports all results;`consumed_copy_count()` reports the two unique original
contributors per result. `pairs()` preserves requested order and reverse;`pair(id)` queries an actual
explicit result. Each `JoinCopyPair` lends its owner,actual result request and guarded `source(side)`
action,including original id/Piece/orientation and held operation context.

Work is O(C log(C+Q+2)+Q log(Q+2)+1),with peak3Q new entries and constant count/selected state.
No provider,evidence,registry,geometry,physical transform,currentness,source retirement,identity
issuance or atomic effect is supplied. **Each physical pair** still needs actual bank,pose,ease,
stop and consumer checks in its own copy frames;selected-span evidence cannot certify another
pair. Complete candidate inventory/CutPlan/mirroring and all source/result metadata,geometry and
authority requirements remain mandatory. This is native copy bookkeeping,not physical approval.

The next native inventory checks remain separate:the complete candidate CutPlan,whole Piece-field
resource admission,original field preservation,and joined-result fields. None is established by
copy correspondence alone;physical correspondence and consumer justification remain further obligations.

## Bind the complete candidate copy plan

`RequiredJoinCopyCorrespondence::bind_candidate_plan` borrows the actual supplied candidate
`PieceInventory` and `CutPlan`,revalidates the complete plan against those same Pieces,and requires
exactly the retained-original and explicitly requested-result copy definitions. A plan born against
another inventory is not a certificate for this inventory. Returned views borrow the supplied
objects;canonical revalidation uses a temporary plan that is discarded.

```rust
use sc_core::{construction::*,ontology::*};
fn candidate<'a>(pairs:&'a RequiredJoinCopyCorrespondence<'a>,inventory:&'a PieceInventory,plan:&'a CutPlan)
    ->Result<BoundJoinCandidatePlan<'a>,JoinCandidatePlanError<'a>> {
    pairs.bind_candidate_plan(inventory,plan,JoinCandidatePlanLimits::new(5,3,2,2,3,11))
}
```

`JoinCandidatePlanLimits::new(C,I,Q,J,D,W)` admits all original copies C,original Pieces I,
requested results Q,candidate Pieces J and candidate copies D before fields or allocation.
Checked workspace is **max(J+3D,I+J+D)**:canonical validation temporaries are dropped before the
persistent original-count/candidate-Piece/candidate-copy indexes are allocated. Logical entries
are not allocator bytes;earlier immutable context/storage remains separate. For C5/I3/Q2/J2/D3,
W11 is exact:C4/I2/Q1/J1/D2 refuse their respective whole quotas,and W10 refuses `Workspace`,
even if candidate mirroring or quantity contradicts the supplied plan.
On canonical refusal,the actual diagnostic is boxed after validation temporaries are released;
its one entry fits the same admitted peak because the closed distinct-Piece source inventory has I≥2.

With resources met,`Canonical` retains the actual `CutPlanError` from complete revalidation,
including wrong quantity,reflection or identity. Complete original actions then derive every
original Piece's retained-copy count,including **known zero** for fully consumed sources. Candidate
D must equal retained copies plus Q (`Count` otherwise). Original RetainOriginal copies are checked
in authored source-action order (`MissingRetained`,`RetainedField`),then explicit results in
requested order (`MissingResult`,`ResultField`). Id/Piece/orientation definitions must match exactly.
Unique canonical membership plus exact count excludes hidden,consumed or substituted copies.

For example,two source pairs produce requested results6000/6001 while unrelated copy202 remains.
The complete candidate has those three copies;original Pieces100/101 have derived retained count0,
and Piece102 has count1. A renamed202,202 reassigned to another Piece,an omitted result,or swapped
result orientations refuses even if that candidate plan is otherwise structurally valid.

`JoinCandidatePlanError` lends whole `correspondence()`,`inventory()`,`plan()`,original `limits()`
and first `refusal()`. `canonical_cause()` retains the actual canonical cause. Missing/changed
retained cases lend `refused_source()`;missing/changed results lend `refused_result()`. Field
mismatches additionally lend the actual `candidate_copy()`;missing/whole causes fabricate none.
Internal SourceInvariant/ResultInvariant causes require diagnosis,not physical classification.

`BoundJoinCandidatePlan` retains those whole parents and reports `piece_count()`,`copy_count()`,
`retained_copy_count()` and peak `workspace_entries()`. `original_retained_count(id)` returns a
derived count,including zero,for an actual original Piece;unknown ids return None. `piece(id)` and
`copy(id)` lend actual candidate objects;`joined_piece()` lends the actual declared-result Piece.
These views certify copy-plan structure and exact copy definitions,**not deep Piece-field matching**.

Work is O((C+I+Q+J+D) log(C+I+Q+J+D+2)+1),with the peak above and I+J+D retained entries.
No labels,material reasons,contours,provider/evidence or geometry bodies are scanned. Whole deep
field-resource admission,original field preservation and joined-result fields remain next. Per-pair
physical/consumer proof,currentness,identity/retirement and atomic authority remain separate.
