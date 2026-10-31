# Native walk output and shared-mark contracts

Deliberate [walk-edit inputs](drafting-edit-inputs.md#native-deliberate-walk-edit-requests) retain
original A/B anchors,profile bindings and the actual canonical SewingGraph/span. They request two
mark occurrences per pair,without asserting two new semantic notch identities. Actual physical
copy/pattern association and distinct declared notch-role metadata are available below. Complete
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

## Distinct semantic-notch role catalogue

`NotchOutputRoleCatalog` retains an explicit operation,all supplied role definitions and a separate
ordered active-tag list. Each `NotchOutputRoleDefinition` pairs a persistent LocalTag with a validated
MachineToken key. These tags name semantic output roles;they are not Notch EntityIds,PointRefs or
EdgeRefs. No catalogue or membership call allocates an entity id,constructs a Notch or decides how
many distinct semantic marks the original requested occurrences require.

| API | Meaning |
| --- | --- |
| `NotchOutputRoleCatalog`, `NotchOutputRoleDefinition` | Whole raw semantic-role metadata and original fields |
| `NotchOutputRoleCatalogLimits::new(D,A,B)` | Whole definitions,active occurrences and machine-key-byte allowances |
| `AdmittedNotchOutputRoleCatalog`, `ActiveNotchOutputRole` | Distinct authentic notch catalogue/member owners |
| `NotchOutputRoleCatalogError`, `NotchOutputRoleCatalogRefusal` | Actual original notch catalogue,policy and first refusal |

```rust
use sc_core::{construction::*,name::MachineToken,ontology::{EntityId,LocalTag}};
let raw=NotchOutputRoleCatalog::new(EntityId::from_bits(10),vec![
    NotchOutputRoleDefinition::new(LocalTag::new(9),MachineToken::new("balance")?)],
    vec![LocalTag::new(9)]);
let roles=raw.admit(NotchOutputRoleCatalogLimits::new(1,1,7))
    .expect("complete local metadata;borrowed errors stay with their catalogue");
let member=roles.active_role(LocalTag::new(9)).expect("actual active role");
assert_eq!(member.definition().key().as_str(),"balance");
assert_eq!(roles.workspace_entries(),4);
# Ok::<(),Box<dyn std::error::Error>>(())
```

Private composition reuses the published bounded raw edge-role metadata algorithm. Definition,
quota and refusal records are aliases shared with point/edge metadata;they carry no membership
proof themselves. Catalogue,admitted catalogue and active-member types remain distinct. There
is no public conversion to an admitted point or edge catalogue/member witness. Equal raw tags,
keys or operation ids cannot reparent an authentic notch member to another equal-looking owner.

Admission checks complete D/A populations and checked2D+2A workspace arithmetic,then D/A/B quotas
before indexes. Original duplicate-tag precedes duplicate-key checks;active occurrences retain
original duplicate/undefined diagnostics. These are bounded logical entries,not allocator bytes
or earlier owned Vec storage. Work is O((D+A+B)log(D+A+2)),including key-comparison byte costs.
All original input remains available on
refusal;no accepted prefix escapes. The existing shared kernel and point catalogue are unchanged.

`definition(tag)` lends actual original definitions,including inactive ones. `active_role(tag)`
and `active_roles()` lend only actual active-tag fields and their associated original definitions;
iteration follows authored active order and supports reverse traversal. `catalog()`, `index()`,
`tag()` and `definition()` retain the authentic owner and original field positions;an index never
supplies a persistent tag or semantic identity. Empty/inactive catalogues are metadata states,
not an executable empty walk edit,proof of no semantic outputs or permission to retire a role.

For original definitions `[9 balance,2 end,7 latent]` and active `[2,9]`,whole B is16 and workspace10.
Active iteration gives end then balance,while latent remains an inspectable inactive definition.
The exact original request fields and all metadata orders survive success/refusal. All supplied
issued definitions,including inactive ones,remain required input. Actual issued-history completeness,
current operation kind/Design/global namespace,transition authority,actual walk-parent binding,
semantic EntityId assignment,mark sharing and geometry/publication remain separate obligations.

## Preserve supplied notch-role history

`before.admit_transition(&after,NotchOutputRoleTransitionLimits::new(D,A,B))` requires both actual
notch catalogues to be admitted already. D,A and B are their complete combined definition,active
occurrence and machine-key-byte counts. Checked sums and their quotas precede operation-owner
comparison and any definition scan. Source parents and their earlier storage/admission scopes
remain unchanged;private composition reuses the published metadata-preservation algorithm.

| API | Meaning |
| --- | --- |
| `NotchOutputRoleTransitionLimits`, `NotchOutputRoleTransitionRefusal` | Explicit combined policy and original refusal data |
| `NotchOutputRoleTransitionError` | Actual complete notch before/after parents,policy and first cause |
| `AdmittedNotchOutputRoleTransition` | Distinct authentic notch-domain structural transition intent |

```rust
use sc_core::construction::*;
fn transition<'a>(before:&'a AdmittedNotchOutputRoleCatalog<'a>,
    after:&'a AdmittedNotchOutputRoleCatalog<'a>,limits:NotchOutputRoleTransitionLimits)
    ->Result<AdmittedNotchOutputRoleTransition<'a>,NotchOutputRoleTransitionError<'a>> {
    before.admit_transition(after,limits)
}
let limits=NotchOutputRoleTransitionLimits::new(5,3,17);
assert_eq!(limits.key_bytes(),17);
```

Every supplied before definition,including inactive roles,must retain its tag and exact machine
key in the after catalogue. Refusal order is combined count overflow,D/A/B quotas,operation owner,
then first forgotten or reassigned definition in original before order. No accepted prefix escapes.
Neither parent changes. This validates supplied metadata intent,not actual issued-history
completeness,canonical operation kind,current Design membership or geometric role continuity.

For before definitions `[9 main,2 end]`,active `[9]`,and after definitions `[7 new,2 end,9 main]`,
active `[2,7]`,combined D5/A3/B17 admits. `introduced()` lends new's actual after index0/definition;
`activated()` lends end then new from the actual after owner;`deactivated()` lends main from the
actual before owner. Reverse iteration reverses each original order. Counts summarize complete
combined metadata,not semantic output entities or a transition execution result.

A later membership restoration to `[9]` must still retain definition7 even when inactive. Reusing
an old snapshot that forgets7 refuses;preserving all three definitions while restoring active9
can be structurally admitted. This grants no recorded undo or reactivation authority. Deactivated
roles retain their definitions;no semantic Notch is removed,retargeted,reused or transferred by
this call,and an introduced role tag supplies no new global semantic identity.

Admission allocates no additional index and uses constant new auxiliary storage. Its work and
each complete delta traversal are bounded by the already counted metadata populations and string
bytes,conservatively O((D+A+B)log(D+A+2)). Repeated view traversal retains the caller's whole-work
budget. Definition/quota/refusal aliases are raw shared data;the admitted transition and its
membership views have distinct notch owners and no conversion to point/edge transition proofs.
Actual history/undo/currentness,walk-parent association,semantic assignment/sharing,provider
truth,all-copy effects and atomic publication remain their separate required execution work.
