# Join candidate inventory contracts

This reference follows the [original sources and copy policies](drafting-join-sources.md).
It records native candidate-plan and whole-field resource contracts. Original/result semantic
matching,explicit consumer repair and all physical/currentness/authority proofs remain separate.

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
