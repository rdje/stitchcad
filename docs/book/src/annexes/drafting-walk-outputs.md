# Native walk output and shared-mark contracts

Deliberate [walk-edit inputs](drafting-edit-inputs.md#native-deliberate-walk-edit-requests) retain
original A/B anchors,profile bindings and the actual canonical SewingGraph/span. They request two
mark occurrences per pair,without asserting two new semantic notch identities. The native output
family begins by associating actual physical copies and pattern Pieces. Typed roles,complete
assignments,explicit mark sharing/coalescing and evidence requirements follow in owned slices.
Actual marks,walking correspondence/differential/ease,profile truth and atomic edits remain G3/G4/bus.

## Actual physical-copy and pattern owners

`BoundWalkSeamEdit::bind_copy_inventory` requires an actual immutable CutPlan,an actual complete
PieceInventory and borrowed `WalkCopyInventoryLimits::new(C,I,W)`. C counts every supplied copy,
including unrelated copies;I counts every actual inventory Piece. Checked I+3C precedes C,I and
workspace quotas,then all scans,lookups and allocations. No accepted A-side prefix escapes a
B-side refusal. Earlier pair/graph admission and model storage retain their own resource scopes.

| API | Meaning |
| --- | --- |
| `WalkCopyInventoryLimits` | Whole copy/Piece/transient logical-entry allowances |
| `BoundWalkCopyInventory` | Actual original walk context,whole plan/inventory and both real copy/pattern records |
| `WalkCopyInventoryError`, `WalkCopyInventoryRefusal` | Whole actual original inputs,policy and first refusal |

```rust
use sc_core::{construction::*,ontology::*};
fn bind<'a>(source:&'a BoundWalkSeamEdit<'a>,plan:&'a CutPlan,inventory:&'a PieceInventory,
    limits:&'a WalkCopyInventoryLimits)
    ->Result<BoundWalkCopyInventory<'a>,WalkCopyInventoryError<'a>> {
    source.bind_copy_inventory(plan,inventory,limits)
}
let limits=WalkCopyInventoryLimits::new(100,50,350);
assert_eq!(limits.workspace_entries(),350);
```

CutPlan retains its immutable copies,not its birth Piece collection. The canonical constructor
therefore rechecks copied definitions against this actual whole inventory. Its original
`CutPlanError` and error order remain intact,including copy/Piece identity collisions,missing
Pieces,actual cut quantities,authored-only orientations and paired balance. The temporary proof
is dropped before authentic side records are cached;it is neither the returned plan nor an
identity-issuance or publication effect.

The I-entry Piece map,C-entry copy-id set,C input definitions and at most C temporary output
records fit the I+3C logical-entry bound. These are entries,not allocator bytes,prior model storage
or a whole-command resource certificate. Work is O((C+I)log(C+I+2));cached parent/copy/Piece views
then take constant work and retain constant auxiliary references. No pair,consumer registry or
geometry is scanned by this stage.

After canonical association,side A then B resolves the original span's explicit physical-copy id
in the actual complete plan,then that actual copy's pattern Piece in the actual inventory.
`source()`, `plan()`, `inventory()` and `limits()` lend actual whole parents. `copy(side)` and
`piece(side)` lend cached actual members. A reordered plan `[201,200]` and inventory `[101,100]`
still associate A copy200/Piece100 and B copy201/Piece101 by identity. With C2/I2,the bound is W8;
C1,I1 or W7 refuses before discovering an absent B copy201.

## Shared pattern ownership does not assign mark identity

Two distinct physical copies200/201 may both name Piece100. One may be Reflected under
MirroredPairs while the other is Authored. `same_pattern_piece()` reports only that actual owner
identity;it does not coalesce requests or accept equal anchors,profiles or sewing correspondence.
Two original requests can keep different pattern parameters and sample/production profile bindings.
A valid disjoint self-seam can also name the same actual copy on A and B,without erasing either
request field or treating its two positions as one semantic mark.

A plan born against a different quantity or mirroring declaration cannot substitute for the actual
inventory. A reflected A copy valid against a paired birth Piece refuses against actual Single.
An empty canonical plan/inventory refuses the original required A copy rather than manufacturing
it. Neither association nor shared ownership establishes current Design/graph/ledger ownership,
held-anchor liveness or owned intervals,global namespace/issuance,mark-sharing permission,all-copy
consumer effects,geometric correspondence,profile values or publication authority. Original pairs,
profiles,span direction/ease/stops,plans,Pieces and held objects remain unchanged.
