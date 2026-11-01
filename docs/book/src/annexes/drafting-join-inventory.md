# Join candidate inventory contracts

This reference follows the [original sources and copy policies](drafting-join-sources.md).
It records native candidate-plan and whole-field resources,explicit companion requirements and
complete original/result field matching. Complete consumer repair,physical validity,currentness
and authority proofs remain separate.

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

## Admit whole original and candidate Piece fields

`BoundJoinCandidatePlan::admit_piece_fields` admits resources for **both complete inventories**
before later semantic comparisons. Original Pieces include fully consumed,unrelated and companion
members. Candidate Pieces remain the exact objects in the supplied candidate plan context.

```rust
use sc_core::construction::*;
fn fields<'a>(plan:&'a BoundJoinCandidatePlan<'a>)
    ->Result<BoundJoinInventoryFields<'a>,JoinInventoryFieldError<'a>> {
    plan.admit_piece_fields(JoinInventoryFieldLimits::new(3,2,0,24,114))
}
```

`JoinInventoryFieldLimits::new(I,J,H,R,B)` first admits all original headers I and candidate
headers J. Complete checked original+candidate hole count H is admitted before hole-child length
scans. Complete checked R then counts boundary,hole,construction and **separate fold declarations**
across every Piece. R is admitted before label/reason lengths;complete checked UTF-8 B is admitted
before any later content or semantic field comparison. Fixed scalar fields are charged by headers.
The implementation exhaustively names every Piece/label field and material variant so new owned
data requires an accounting update. It allocates no workspace and scans no text content.

For I3/J2/H0/R24/B114,I2 refuses `Sources`,J1 `Pieces`,R23 `Occurrences`,B113 `TextBytes`.
A candidate with one added hole and a three-character `ééé` label has H1/R25/B116:the label is
six UTF-8 bytes,not three. H0 refuses `Holes` before reading that child loop's length;R24 refuses
Occurrences before text lengths;B115 refuses TextBytes. Overflow causes retain the relevant whole
hole/occurrence/text arithmetic refusal. No partial inventory is admitted.

Equal content is counted at each original/candidate occurrence. A fold edge repeated from a
boundary is a separate charged field. Every label has name,size,fabric and colorway bytes;
Unresolved material reasons count too. Assigned material has no reason bytes. Fully consumed
originals and unrelated data cannot disappear from the resource envelope.

`JoinInventoryFieldError` retains actual whole `plan()`,original `limits()` and first `refusal()`.
`BoundJoinInventoryFields` retains those same whole parents and reports `hole_count()`,
`occurrence_count()` and `text_bytes()`. `original_pieces()`/`candidate_pieces()` lend every actual
Piece in its inventory's authored order and reverse. `BoundJoinCandidatePlan::original_inventory()`
lends the actual complete source inventory used by this context.

Work is O(I+J+H+1),with constant counters,no allocation,Unicode normalization,blank/content scan,
provider,evidence,registry or geometry call. A changed candidate label,hole or material can have
admissible resources while still contradicting its original or requested field. This stage certifies
resource presence only;original preservation,joined-result semantics,companion/consumer repair,
per-pair physical/currentness/identity and atomic effects remain separately required.

## Request explicit companion-reference repair

A retained separate pair member cannot keep naming a source Piece that disappears from the
candidate inventory. Canonical `PieceInventory` requires reciprocal,opposite-handed,equal-quantity
members. For example,old right member102 names left source100;the join requests new left result5000
and retains102. The proposed102 must explicitly name5000 while keeping its right hand. Keeping
its old100 link fails canonical reciprocity;neither a new name nor reflected copy can supply a mate.

`JoinCompanionRepairManifest` retains the operation and ordered
`JoinCompanionRepairRequest::new(piece,expected,companion,evidence_source)` records. The Piece id
selects the actual retained original;`expected` guards its whole original Mirroring field. The new
companion and evidence source are explicit authored fields. Zero/reused evidence ids are retained
as requests,with no evidence kind,state,currentness,truth or consumer approval inferred.

```rust
use sc_core::{construction::*,ontology::*};
let request=JoinCompanionRepairRequest::new(EntityId::from_bits(102),
    Mirroring::PairMember{handedness:Handedness::Right,companion:EntityId::from_bits(100)},
    EntityId::from_bits(5000),EntityId::from_bits(7001));
fn repairs<'a>(manifest:&'a JoinCompanionRepairManifest,fields:&'a BoundJoinInventoryFields<'a>)
    ->Result<RequiredJoinCompanionRepairs<'a>,JoinCompanionRepairError<'a>> {
    manifest.admit(fields,JoinCompanionRepairLimits::new(1,2))
}
```

`JoinCompanionRepairLimits::new(M,W)` admits every raw request M and checked2M source-index/
association entries before operation,fields or allocation. Earlier whole I/J/H/R/B/copy resources
remain immutable and separate. For M1,W2 is exact:M0 refuses `Repairs`,W1 `Workspace`,before an
incorrect operation or malformed first source guard.

With quotas met,`Operation` precedes original rows. Each row checks `Duplicate`,`MissingSource`,
`SourceField`,`NotPairMember`,`SourceNotRetained`,then `CompanionNotSource` and
`CompanionNotConsumed`. The old mate must be an actual fully consumed A/B source. `ResultIdentity`
requires the explicit new target to be the declared joined result. `ResultPairing` requires the
**authored result population** to name this retained member with opposite hand. `CandidateField`
requires the actual candidate member to preserve its original hand and name that same new result.
Candidate reciprocity alone cannot replace authored result intent,or permit a changed original hand.
Internal source/candidate invariant faults require diagnosis;they are not compatibility evidence.

`JoinCompanionRepairError` retains actual whole `manifest()`,`fields()`,original `limits()` and
first `refusal()`. Row causes lend `refused_request()`,the known actual `original_piece()`,and
`candidate_piece()` if present. Whole resources/operation lend none;no accepted prefix or
reconstructed source escapes refusal.

`RequiredJoinCompanionRepairs` retains those whole parents and reports `repair_count()` and
`workspace_entries()`. `repairs()` preserves original order and reverse;`repair(piece)` queries an
actual requested original scope. Each `JoinCompanionRepair` lends its owner,raw request,actual
original/candidate members,original fully consumed `retired_companion()`,original/candidate
mirroring fields,and actual authored evidence id. The accessor names proposed reconciliation;
no Piece is retired or reference reassigned by this binding.

An explicit empty request list declares no mirroring exceptions. It can bind beside changed
candidate fields,so the following original-field comparison must still reject every uncovered
change. Even admitted requests remain visible consumer/evidence obligations;they do not accept
source retirement,currentness or historical reference reassignment. Every other original field
must remain exact except canonical quantity derived from retained copies.

Work is O(M log(M+I+J+2)+1),with2M new entries. No provider,evidence,registry,geometry,physical
mirroring,parity inference or atomic effect is supplied. Whole original preservation,joined-result
fields,complete consumer repair,current identity and per-pair physical proof remain required.

## Preserve every remaining original Piece field

`RequiredJoinCompanionRepairs::match_original_pieces` compares every actual original in its
inventory's authored order,including unrelated Pieces and fully consumed sources. The closed
candidate plan supplies a derived retained-copy count for each original. Count0 requires candidate
absence;positive count requires the original identity in the candidate with that exact quantity.
Fully consumed absence is structural bookkeeping;this comparison performs no retirement.

```rust
use sc_core::construction::*;
fn originals<'a>(repairs:&'a RequiredJoinCompanionRepairs<'a>)
    ->Result<MatchedJoinOriginalPieces<'a>,JoinOriginalPieceError<'a>> {
    repairs.match_original_pieces()
}
```

For each retained original,the comparator checks identity,ordered boundary,ordered hole groups,
ordered construction fields,derived quantity,mirroring,fold flag,separate ordered fold edges,
cutting face,material,layer and all four label fields. Repeated edges,group order,traversal direction,
assigned material ids and unresolved reason strings remain exact. Labels compare name,size,fabric,
colorway as exact UTF-8;there is no trimming,case folding or Unicode normalization. Exhaustive
PieceDefinition/LabelText field patterns require a comparison update when owned fields are added.

The only permitted differences are derived retained quantity and an already admitted explicit
companion field. For example,source100 has two authored copies200/203;the selected200 is consumed
and203 remains. Candidate100 must have quantity1 and preserve every other field. A candidate that
renames203,reassigns it or changes its orientation already fails the preceding exact copy-plan gate.
Changing100's contour,material,label,fold or hand is still refused here.

In the separate-member example above,102 may replace companion100 with5000 only through the
actual guarded repair requirement,while preserving its right hand and every other field. An empty
repair list beside the same otherwise canonical candidate refuses `Field { piece:102,field:Mirroring }`.
Admitted repair requests remain outstanding consumer/evidence obligations;this field match does
not accept evidence0,approve historical reference reassignment or authorize source retirement.

`JoinOriginalPieceRefusal::Field` retains the actual original id and first
`JoinOriginalPieceField` in Piece-definition order;labels identify their `LabelField` in the order
above. Boundary/Holes precede a changed label. CountInvariant/MembershipInvariant mean an internal
closed-parent fault requiring diagnosis,not physical incompatibility. `JoinOriginalPieceError`
lends the whole actual `repairs()` parent,first `refusal()`,actual `original_piece()` and known
`candidate_piece()`;no reconstructed source or accepted prefix escapes a refusal.

`MatchedJoinOriginalPieces` retains the whole repair parent and reports `original_piece_count()`,
`retained_piece_count()` and `consumed_piece_count()`;these count Piece definitions,not physical
copies. `originals()` lends authentic original scopes in authored order and reverse;`original(id)`
queries actual original membership,new-result/unknown ids returning None. Each `JoinOriginalPiece`
lends `context()`,`original()`,the actual `candidate()` if retained,and `companion_repair()` if
explicitly requested. `retained_quantity()` is the matched canonical quantity,known0 for consumed
originals. Original pre-join quantity and old companion fields remain available through `original()`.

Whole immutable I/J/H/R/B and copy/repair resources were admitted before comparison. No additional
allocation,quota,provider,evidence,registry or geometry call occurs. Work is
O(I log(I+J+M+2)+H+R+B+1),with constant auxiliary storage;earlier owned contexts remain separate.
This proves complete original field preservation in the supplied candidate only. Joined-result
fields,full consumer repair,currentness,global identity and per-pair physical/atomic proof remain pending.

## Match every declared joined-result field

`MatchedJoinOriginalPieces::match_result_piece` compares the actual supplied result Piece against
the complete explicit native request. It requires the preceding exact copy plan,whole field-resource
admission,guarded companion requirements and full original-field matching. The returned
`MatchedJoinPieceInventory` reconciles all candidate copy definitions and Piece fields;it grants
no physical/currentness/evidence or mutation authority.

```rust
use sc_core::construction::*;
fn result<'a>(originals:&'a MatchedJoinOriginalPieces<'a>)
    ->Result<MatchedJoinPieceInventory<'a>,JoinResultPieceError<'a>> {
    originals.match_result_piece()
}
```

Checks follow Piece-definition order:declared result identity,complete outer boundary,complete
ordered hole groups,complete construction occurrences,canonical requested quantity and mirroring,
explicit fold presence/exact fold edge,selected cutting face,material,layer and all four label fields.
Boundary/hole/construction fields match exact logical output tuples and traversal directions;
length/group boundaries and authored order remain significant. Equal geometry or the same set of
edges cannot replace ordered equality. Repeated construction uses remain repeated actual fields.
Exhaustive PieceDefinition and shared exhaustive LabelText patterns require future field updates.

For example,a request with boundary9-Reversed/8-Original,two holes10-Original then2-Reversed,
construction tags11/5/4/12/4/11/5/12 with their explicit directions,and fold9 must match exactly.
Reversing the boundary,swapping hole groups or construction uses,selecting another outer fold edge,
or replacing CutOnFold with NoFold refuses even when PieceInventory and CutPlan are canonical.
The fold flag derives from the explicit intent;fold edges are exactly one selected tuple or empty
for NoFold. Neither independent print prose nor an arbitrary boundary edge can replace that choice.

Material,layer,label and cutting face match their independently selected A/B or authored values.
Unresolved material reasons and label name/size/fabric/colorway are exact UTF-8 fields,with no
trimming or normalization. Another nonblank reason,label,valid face or layer remains a mismatch.
This equality supplies no material kind/state,physical face compatibility or label truth evidence.

`JoinResultPieceError` retains actual whole `originals()`,first `field()` as
`JoinResultPieceField`,and actual `candidate_piece()`. Label causes retain their `LabelField` in
name/size/fabric/colorway order. No reconstructed expected Piece or accepted prefix escapes.
`MatchedJoinPieceInventory` lends the same whole original proof through `originals()`,actual full
copy/inventory context through `plan()`,and actual matched supplied result through `result()`.
Original quantities,old companions,fold/construction omission requests,additional content and
explicit companion/evidence requirements remain available through their authentic parents.

Whole immutable I/J/H/R/B and all copy/proposal resources precede comparisons. Expected lengths
are checked before ordered traversal. Work is O(H+R+B+1),constant auxiliary storage,no new quota,
allocation,provider,evidence,registry or geometry call;earlier declaration bounds/storage remain
separate. Matching fold/construction fields does not approve omitted source data or a new fold axis.
Complete consumer repair,current identity/global issuance,actual per-pair geometry/pose/ease,
domain acceptance and atomic publication remain required. This is complete native field matching,
with focused checks;full geometry-backed integration and broad verification remain separately owned.

Exact field reconciliation supplies a proposed inventory for explicit consumer/reference repair.
Complete original observer ownership and mapped-or-visible repair requirements must precede effects.

## Require explicit complete consumer-source obligations

`MatchedJoinPieceInventory::require_consumers` binds five explicit source requests to the actual
complete candidate/original/copy/repair context. This follows the native mirror/truing request
convention;the returned `RequiredJoinConsumers` is an outstanding requirement association,not a
consumer census,evidence verdict,reference repair or execution-readiness certificate.

```rust
use sc_core::{construction::*,ontology::EntityId};
fn bind<'a>(inventory:&'a MatchedJoinPieceInventory<'a>,sources:&'a JoinConsumerSources)
    ->RequiredJoinConsumers<'a> { inventory.require_consumers(sources) }
let sources=JoinConsumerSources::new(EntityId::from_bits(7001),EntityId::from_bits(7002),
    EntityId::from_bits(7003),EntityId::from_bits(7004),EntityId::from_bits(7005));
```

All five `JoinConsumerEvidenceKind` scopes are mandatory and remain distinct:

- `WholeSourceConsumers`:complete current original/affected result marks,grainlines,sewing and
  other structural/semantic/reference consumers,including shared edges,unselected/retained copies
  and separately identified companions. Selected-span evidence cannot certify the rest.
- `SourcePreservation`:every original holder,field and held key reconciles with explicit copy/
  field dispositions. Construction/fold omissions,additional content,new fold axes and companion
  changes require actual justification. Historical objects are not rewritten by this binding.
- `CurrentOwnership`:actual current Design/revision,graph,plan,Piece/frame/copy/material and
  semantic owner registries;matching numeric ids or an earlier successful query are insufficient.
- `ResultSemantics`:complete result marks/grainlines/seams/other semantics have explicit owners
  and mappings. Copy quantities,reflection,names and list positions create no semantic object ids.
- `VisibleRepairs`:every held point/range and registration has explicit mapping,ambiguity,gap or
  visible repair. An exact owner/edge/parameter registration repair does not update an immutable
  Notch or other semantic holder still containing the old field;coordinated semantic repair remains.

`JoinConsumerSources::new(whole,preservation,ownership,result,repairs)` requires every field
without a Default. Zero/reused source ids remain ordinary unresolved requests. `source(kind)`
lends the actual original field. Equal values neither merge obligations nor certify evidence kind,
existence,state,currentness,truth,empty consumers or human approval.

`RequiredJoinConsumers` lends actual `inventory()` and whole original `sources()`.
`requirements()` visits all five scopes in the order above and reverse;`requirement(kind)` returns
the actual scope/source-field view. Every `JoinConsumerRequirement` lends its whole `context()`,
distinct `kind()` and original `source()`. Original field/copy actions,old held companion fields,
construction/fold omissions and authored evidence requests remain available through those parents.

Fixed borrowed association and five-item traversal take constant work/space with no allocation,
quota,registry,consumer,provider,text or geometry scan. Earlier whole resources remain separate.
Canonical intake/sewing/all-copy effects,global identity/lineage,actual repair truth,physical
correspondence and atomic authoring authority remain separate explicit requirements and execution
work. No source disappearance in one Piece authorizes global edge retirement or registration withdrawal.

### Native consumer declaration checkpoint

The native five-scope declaration review is complete. Whole-consumer/current-owner requests
cover actual Design and copy-addressed sewing sources;source/result/visible-repair requests retain
all holder ownership,explicit changes and mapped-or-visible obligations. This follows the canonical
[held-reference contract](../spec/ontology.md#11-the-persistent-identity-contract)
and [copy-addressed sewing contract](../spec/ontology.md#42-seamspan-and-sewinggraph).
The native contour/construction/fold declaration checkpoint includes those outstanding requirements;
it certifies neither a current consumer census nor discharged source loss,registration or semantic repair.

Complete native field/copy/contour declarations therefore remain distinct from actual acceptance.
Canonical cut/print indicators must use the matched result's canonical fields;lineage still needs
explicit source retention/retirement and global identity/history requirements. Canonical intake,
sewing/ease/stops,all-copy physical effects and atomic authoring authority remain explicit later work.
Actual geometry,domain/envelope evidence and full integration remain G2/G3 and deferred verification;
this declaration checkpoint grants no independent approval or release readiness.
