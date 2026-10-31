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
Actual history/undo/currentness,current walk-parent validity,semantic assignment/sharing,provider
truth,all-copy effects and atomic publication remain their separate required execution work.

## Bind notch roles to the actual walk context

`BoundWalkCopyInventory::bind_notch_roles` associates the actual complete walk/copy/pattern
context with an `AdmittedNotchOutputRoleCatalog` under an explicit fresh D/A/B association policy.
Only the distinct notch catalogue type is accepted;point or edge catalogue proofs cannot substitute.
Complete definitions,active occurrences and cached whole machine-key bytes are checked before the
catalogue operation is compared with the original actual WalkSeamEdit operation.

| API | Meaning |
| --- | --- |
| `BoundWalkNotchRoles` | Actual complete copy/source context and typed notch catalogue association |
| `WalkNotchRoleBindingError`, `WalkNotchRoleBindingRefusal` | Whole original parents,policy and first quota/operation cause |

```rust
use sc_core::construction::*;
fn bind<'a>(copies:&'a BoundWalkCopyInventory<'a>,catalog:&'a AdmittedNotchOutputRoleCatalog<'a>,
    limits:NotchOutputRoleCatalogLimits)
    ->Result<BoundWalkNotchRoles<'a>,WalkNotchRoleBindingError<'a>> {
    copies.bind_notch_roles(catalog,limits)
}
let limits=NotchOutputRoleCatalogLimits::new(3,2,16);
assert_eq!(limits.key_bytes(),16);
```

The selected association limits may be lower than the catalogue's earlier admission policy.
For definitions `[9 balance,2 end,7 latent]`,active `[2,9]`,whole B is16. D2,A1 or B15 refuses
before discovering that catalogue operation99 differs from actual walk operation10. Ordinary
operation id0 receives the same equality check;it cannot act as an omitted-operation sentinel.
Errors lend the actual whole original copy context,catalogue,policy and first cause without any
accepted prefix. Cached original counts make association constant work/space with no scan or
allocation;earlier whole graph/input/copy/inventory/catalogue bounds retain their own scope.

`copies()` lends the same actual copy/source/Piece context;`catalog()` lends the same distinct
notch catalogue and authentic original membership fields;`limits()` retains the original explicit
association policy. Role views keep their actual admitted notch owner,not an equal-looking clone.
The original pair anchors,all profile contexts,span direction/ease/stops,physical-copy/Piece fields,
metadata definitions and original orders remain unchanged on success/refusal.

Empty or inactive metadata can bind a matching operation without dropping any original requested
occurrence. A bound empty catalogue still retains the original one pair/two requests. This is
neither a complete mark-assignment manifest nor proof that the walk creates zero semantic marks.
No2N-new-id rule,implicit per-copy duplication,sharing/coalescing or existing-Notch reassignment
is introduced. Complete explicit assignment/identity/sharing policy and current Design/history,
correspondence/differential/ease/profile/all-copy/effect/authority evidence remain required.
No semantic Notch,PointRef or EdgeRef is issued and no geometry or atomic edit executes here.

## Complete original occurrence-to-role attribution

`WalkNotchAssignmentManifest` retains an explicit operation and ordered `WalkNotchRoleAssignment`
entries. Each entry names an original zero-based input pair,A/B side and proposed active LocalTag.
The pair position is an attribution coordinate into this actual immutable input,not a persistent
semantic identity or recipe statement ordinal. A complete assignment gives every original field
one role proposal;it does not yet admit shared-mark semantics or assign a global Notch EntityId.

| API | Meaning |
| --- | --- |
| `WalkNotchRoleAssignment` | Original pair/side occurrence and proposed active local role |
| `WalkNotchAssignmentManifest` | Whole original operation and ordered attribution proposal |
| `WalkNotchAssignmentLimits::new(N,M,W)` | Whole input pairs,manifest entries and checked2M workspace allowances |
| `WalkNotchAssignmentError`, `WalkNotchAssignmentRefusal` | Whole actual original inputs,policy and first refusal |
| `AdmittedWalkNotchAssignments` | Complete authentic occurrence attribution with actual walk/role parents |
| `BoundWalkNotchAssignment` | Authentic original entry,pair/request field,active role and actual copy/Piece views |

```rust
use sc_core::{construction::*,ontology::*};
let raw=WalkNotchAssignmentManifest::new(EntityId::from_bits(10),vec![
    WalkNotchRoleAssignment::new(0,SeamSideId::B,LocalTag::new(9)),
    WalkNotchRoleAssignment::new(0,SeamSideId::A,LocalTag::new(2))]);
fn admit<'a>(raw:&'a WalkNotchAssignmentManifest,roles:&'a BoundWalkNotchRoles<'a>)
    ->Result<AdmittedWalkNotchAssignments<'a>,WalkNotchAssignmentError<'a>> {
    raw.admit(roles,WalkNotchAssignmentLimits::new(1,2,4))
}
assert_eq!(raw.assignments().len(),2);
```

Checked2M,whole N/M and workspace quotas precede operation equality,all scans and allocation.
N includes all actual original pairs;M includes every original manifest entry. New workspace is
an M-entry occurrence index and M-entry association vector;these count logical entries,not bytes
or earlier owned input/catalogue/copy storage. Work is O((M+N)log(M+D+2)) for D admitted role
definitions. Earlier source/pair/graph/resource scopes retain their own obligations.

Each original entry checks pair bounds,duplicate occurrence,then undefined/inactive role,in that
order. Repeating the same pair/A/B occurrence refuses even if its second role differs. After all
entries,complete coverage follows original input pair/A/B order. First missing input field refuses;
no missing assignment is manufactured and no accepted prefix escapes. With one actual pair and
two entries,W4 is complete;N0,M1 or W3 refuses before an incorrect operation or invalid first pair.
An empty manifest refuses missing pair0/A,despite empty role metadata being a valid earlier state.

`assignments()` lends all entries in original manifest order and supports exact size/reverse
iteration. `requests()` lends every original input field in pair/A/B order;reverse traversal
reverses that source order. `assignment(pair,side)` selects an actual original occurrence in
O(log(M+2)) work. Complete request traversal costs O(N log(M+2)) without allocation;repeated
traversal retains the caller's whole-work budget. None of these positions supplies an output id.

Each view keeps its actual complete `owner()`,original manifest `index()` and `assignment()`,
whole original `pair()` and exact A/B `requested()` anchor/profile fields. `role()` lends authentic
active membership in the same admitted notch catalogue;`copy()` and `piece()` lend the actual
cached original side owners. Equal-looking raw values cannot reparent these private associations.
The complete original manifest,walk input,source fields,catalogue,copy/Piece definitions and all
symbolic profile contexts remain unchanged on success/refusal.

## Grouping proposals retain every request

Repeated role tags are permitted as explicit grouping proposals. For two pairs,all four original
occurrences may propose role2 while roles7/9 remain unused active metadata. Every occurrence,
anchor,profile field,physical-copy id and pattern owner still remains separately inspectable.
Unused active metadata is legal;canonical active membership is not evaluated-instance presence.
Four requests therefore certify neither four new semantic identities nor one accepted shared mark.

This attribution stage can retain a grouping proposal whose occurrences have different pattern
owners or fields. Such a proposal has not passed the later explicit semantic sharing/coalescing
checks. Complete semantic identity/retention declarations,pattern/anchor/profile consistency,
actual source correspondence and all currentness/provider/effect/authority requirements remain
mandatory. No coordinate coincidence,matching tag or complete occurrence count supplies those
proofs. The original paired requests remain intact until a supported complete atomic edit is proven.
