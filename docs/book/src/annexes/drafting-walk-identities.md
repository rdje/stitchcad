# Native walk semantic identity and retention

This companion follows [complete original occurrence attribution](drafting-walk-outputs.md#complete-original-occurrence-to-role-attribution).
Role tags propose semantic result groups;they are not global Notch EntityIds. Explicit semantic
identity/retention plans and checked sharing remain separate from complete occurrence attribution.
Any later new-identity proposal must originate from the trusted application's injected reservation
workflow. No client mutation request names a new id. The native steps here neither access a reserve
nor issue,retire or globally validate semantic identities.

## Index actual supplied existing Notch objects

`AdmittedWalkNotchAssignments::bind_existing_notches` borrows an explicit whole collection of
caller-supplied existing immutable canonical Notch objects. It produces `BoundWalkExistingNotches`
under `WalkExistingNotchLimits::new(K)`. K counts every provided record,including unrelated marks,
and is checked before any id scan or allocation. A K-entry identity index then rejects the first
repeated original semantic id,even when both supplied objects have equal fields.

| API | Meaning |
| --- | --- |
| `WalkExistingNotchLimits` | Whole provided-record and new logical index-entry allowance |
| `WalkExistingNotchError`, `WalkExistingNotchRefusal` | Actual whole attribution/collection/policy and first cause |
| `BoundWalkExistingNotches` | Actual complete supplied-object membership and stable-id lookup |

```rust
use sc_core::{construction::*,ontology::Notch};
fn bind<'a>(assignments:&'a AdmittedWalkNotchAssignments<'a>,notches:&'a [Notch],
    limits:WalkExistingNotchLimits)
    ->Result<BoundWalkExistingNotches<'a>,WalkExistingNotchError<'a>> {
    assignments.bind_existing_notches(notches,limits)
}
let limits=WalkExistingNotchLimits::new(100);
assert_eq!(limits.notches(),100);
```

The supplied objects carry their actual born Piece owners,held anchors and all symbolic profile
bindings. Admission scans ids only;it clones none of those fields and creates no replacement Notch.
`assignments()` lends the same actual complete occurrence/copy/role/source context;`notches()`
lends the whole original collection in authored order;`limits()` retains the explicit policy.
`indexed_count()` reports its complete K-entry index,not a result/output or issuance count.

`notch(id)` returns the actual provided object by stable semantic identity in O(log(K+2)) work.
For collection `[Notch701 on Piece101,Notch700 on Piece100]`,lookup700 lends the actual second
object,without inferring identity from ordinal or coordinates. Reordering the collection changes
iteration order and duplicate positions,not an object's identity,owner,anchor or profile fields.
Whole admission takes O(K log(K+2)) work and K new logical index entries. These are entries,not
allocator bytes,earlier owned object storage,unprovided registry work or a whole-execution budget.

With collection `[701,700,701]`,K2 refuses complete required3 before duplicate inspection;K3 reports
first0/second2/id701. Errors retain the whole actual collection and context;no accepted prefix
escapes. A different canonical owner/anchor on the repeated id also refuses. Id uniqueness does
not depend on object equality or whether a caller intends to retain one of the records later.

## Supplied membership has a limited scope

An explicitly empty supplied collection is legal and does not drop any original requested
occurrence. It proves neither an empty actual Design registry nor the absence/freshness of an
unlisted global identity. `notch(id) == None` means only unlisted in this supplied collection.
This stage does not inspect every supplied object's other fields or certify current liveness,
owned source intervals,profile truth,actual registry completeness or global namespace/history.

Unrelated provided Notch702 may retain its actual born owner Piece600 even when that Piece is
outside the supplied walk copy inventory. Lookup lends that same object;it cannot retarget it to
walk Piece100. Later retention suitability and complete identity/sharing checks must establish
which provided objects are legitimate results for actual role groups. Merely finding an id in
this index supplies none of those permissions. Original Notches,owners,fields,collections and
held references remain unchanged on success/refusal.

The scoped known-identity exclusions below precede explicit role-to-semantic-identity/retention
proposals below. Actual current Design/registry/history,global reservation/nonreuse,
semantic sharing/correspondence,profile and all-copy effects,geometry and atomic authorized
publication retain their separate required execution owners.

## Bounded scoped known-id exclusions

`BoundWalkExistingNotches::bind_identity_exclusions` builds a scoped known-EntityId set under
`WalkNotchIdentityExclusionLimits::new(C,I,H,R,N,K,W)`. It creates or reserves no semantic identity.
A true `excludes_new_notch(id)` result names an inspected known reference;false means only
unlisted here. Actual current namespace completeness,kind consistency,global history/nonreuse,
reservation and publication authority remain separate mandatory checks.

| API | Meaning |
| --- | --- |
| `WalkNotchIdentityExclusionLimits` | Explicit complete copy/Piece/hole/edge/pair/Notch/workspace allowances |
| `WalkNotchIdentityExclusionError`, `WalkNotchIdentityExclusionRefusal` | Whole actual original context,borrowed policy and first resource refusal |
| `BoundWalkNotchIdentityExclusions` | Actual complete parents and scoped known-id set,without freshness authority |

```rust
use sc_core::construction::*;
fn bind<'a>(existing:&'a BoundWalkExistingNotches<'a>,limits:&'a WalkNotchIdentityExclusionLimits)
    ->Result<BoundWalkNotchIdentityExclusions<'a>,WalkNotchIdentityExclusionError<'a>> {
    existing.bind_identity_exclusions(limits)
}
let limits=WalkNotchIdentityExclusionLimits::new(2,2,0,3,1,2,44);
assert_eq!(limits.workspace_entries(),44);
```

C counts every supplied CutPlan copy;I every supplied Piece;N every original walk pair;K every
supplied existing Notch. Those four complete quotas precede any scan. H then counts every Piece
hole group before child lengths are scanned. R counts every boundary,hole,construction and fold
occurrence,including repeated identities. Checked whole H/R and their quotas precede creator reads.
Finally checked W = C+2I+R+14N+9K+3 precedes all id scans and known-set allocation. No prefix set
escapes a refusal. No label or unresolved-Material text is scanned by this stage.

| Inspected fields | Entry upper bound |
| --- | --- |
| All supplied physical-copy ids | C |
| All supplied Piece ids and assigned Material ids | 2I |
| All supplied Piece edge creators,including construction/fold repeats | R |
| Both original requested anchor creators and all six profile ids per mark | 14N |
| Every supplied Notch id,born Piece,anchor creator and six profile ids | 9K |
| Actual walk operation,actual graph and selected span ids | 3 |

Shared values may deduplicate set keys but never reduce these complete input/workspace allowances.
Exhaustive profile-field patterns keep the six-field accounting coupled to the canonical profile
records. This is one bounded identity set,not another copy/Notch index or cloned source model.
Work is O((C+I+H+R+N+K)log(C+I+R+N+K+2)),with at most W new logical entries. Entries are not
allocator bytes,prior immutable/index storage or a whole execution budget.

For the documented two-copy/two-Piece fixture with one original pair,two provided Notches,H0/R3,
W44 covers24 distinct inspected ids. This includes construction-only creator3,even though neither
requested anchor uses it,and both source/Notch sample/production profile declarations. An added
hole and fold give H1/R5/W46;the repeated edge uses still consume R allowance. Assigned Materials
777/888 are excluded as well. An unrelated provided Notch702 on Piece600,creator4 and profiles
3000–3005 keeps every one of those references excluded,without becoming an authorized retained result.

`existing()` lends the same actual complete supplied Notch/assignment/copy/role/source context;
`limits()` lends the original explicit policy. `holes()`, `edges()` and `workspace_bound()` expose
complete admission counts. `indexed_count()` reports distinct inspected ids,not a global namespace
census. Queries take O(log(W+2)) work. Even with K0,all other original exclusions and requests remain.

The set does not inspect every other graph span,stop,ease/provider record or external registry and
history entry. Unlisted999 could still be occupied elsewhere or previously issued. Requested
profile identities are references,not accepted profile values or existence evidence. A lookup
therefore cannot certify global freshness,absence,kind consistency or reservation,and it cannot
authorize reuse after retirement. Explicit host identity/retention proposals must still pass the
complete current Design/private-reserve and semantic sharing/effect/publication requirements.

## Complete present-role host identity proposals

`WalkNotchIdentityManifest` supplies one explicit identity choice for every role actually used by
an original assigned occurrence. `Retain { role, notch }` names a supplied existing canonical Notch;
`Propose { role, id }` carries a trusted-host identity proposal from the private reservation workflow.
This is host plan metadata. A client mutation request cannot name a new id,and native admission
neither consults that private reserve nor issues an identity.

```rust
use sc_core::{construction::*,ontology::{EntityId,LocalTag}};
fn bind<'a>(raw:&'a WalkNotchIdentityManifest,known:&'a BoundWalkNotchIdentityExclusions<'a>)
    ->Result<AdmittedWalkNotchIdentities<'a>,WalkNotchIdentityError<'a>> {
    raw.admit(known,WalkNotchIdentityLimits::new(2,4,10))
}
let choices=WalkNotchIdentityManifest::new(EntityId::from_bits(10),vec![
    WalkNotchIdentityIntent::Retain {role:LocalTag::new(9),notch:EntityId::from_bits(701)},
    WalkNotchIdentityIntent::Propose {role:LocalTag::new(2),id:EntityId::from_bits(500)},
]);
assert_eq!(choices.intents().len(),2);
```

For E original identity entries and M complete original assignments,checked 3E+M precedes the
whole M,E and workspace quotas,then actual original walk-operation equality. Only then may scans
and allocation begin. The temporary upper bound covers an M-entry source-role map,E role-index
entries,E semantic-id entries and E original associations. It counts logical entries,not bytes or
prior object/index storage. The temporary source-role and semantic-id maps are dropped after
complete admission;the role index and associations remain. Source-role indexing selects the
unique authentic catalogue member,never a representative mark,anchor or physical copy.

Each original entry checks duplicate role,actual assigned-role membership,duplicate semantic id,
then supplied existing-object lookup or scoped known-id exclusion,in that order. Both choice
kinds participate in the same semantic-id uniqueness check. Complete coverage then follows the
original source pair/A/B order. Unused active catalogue roles need no choice;an entry naming one
refuses as unassigned. With only role9 supplied,the first original pair0/A role2 is missing.
A refusal retains the whole original plan,actual complete parents,explicit policy and first cause;
no accepted prefix escapes.

`AdmittedWalkNotchIdentities` lends `manifest()`, `known()` and `limits()` unchanged.
`actions()` traverses original identity-entry order in both directions;`action(role)` returns the
authentic choice in O(log(E+2)) work. Each action retains its actual owner,index,original intent
and id field,and actual active catalogue member. `source()` lends either the actual provided
prior Notch or the actual original proposed id field. No replacement Notch is manufactured.
Whole admission takes O((E+M)log(E+M+K+X+2)) work,where K is the supplied-Notch index size and
X the scoped known-id set size. Local tags and EntityIds introduce no string-key comparison cost.

Four original requests in two pairs may explicitly use roles2/9 with proposed ids500/501.
That is two identity proposals,not four automatically generated ids,and it still proves no shared
semantic mark. `group_count()`, `retention_count()` and `proposal_count()` report complete choices;
`workspace_bound()` reports the conservative temporary upper bound. Reordering choices preserves
identity while changing only original-entry iteration and duplicate positions.

Retaining701 keeps that semantic identity and lends its actual prior object as context. It does
not decide whether result fields stay unchanged or are explicitly revised,and it grants no
replacement authority. Pattern owner,anchor and every profile binding still need the explicit
field/sharing policy and subsequent currentness/effect checks. Unmentioned supplied Notches remain
untouched;absence from this plan retires no entity or role. Repeated role tags alone accept no
coalescing across different Pieces,anchors or profiles. An unlisted proposed id,including ordinary
zero,still needs actual global reservation/nonreuse validation before any publication. Geometry,
physical-copy correspondence,profile truth,visible repair and authorized atomic edits remain owed.

## Explicit exact-field sharing and retained revisions

`WalkNotchFieldManifest` supplies one `WalkNotchFieldIntent` for every present semantic identity
role. Each intent explicitly selects both an occurrence policy and an identity-field policy.
There is no default and repeated tags never choose a policy automatically.

| Policy | Required meaning |
| --- | --- |
| `SingleOccurrence` | Exactly one original occurrence uses this role |
| `CoalesceIdentical` | One or more occurrences share the same actual pattern Piece,exact anchor and all profile fields |
| `NewRequested` | A proposed new identity takes the common requested fields |
| `RetainUnchanged` | An actual supplied prior Notch already has the exact requested owner,anchor and profiles |
| `ReviseRetained` | An actual supplied prior identity explicitly proposes requested anchor/profile revisions on its same Piece |

`NewRequested` requires an actual prior `Propose` choice;the two retained policies require `Retain`.
Neither retained policy changes the prior semantic owner's Piece. A proposed anchor/profile revision
is plan metadata,not permission to replace a Notch or a certificate that its requested anchor is
current. Original immutable objects and all held references remain unchanged.

```rust
use sc_core::{construction::*,ontology::{EntityId,LocalTag}};
fn bind<'a>(raw:&'a WalkNotchFieldManifest,ids:&'a AdmittedWalkNotchIdentities<'a>)
    ->Result<AdmittedWalkNotchFields<'a>,WalkNotchFieldError<'a>> {
    raw.admit(ids,WalkNotchFieldLimits::new(2,4,2,6))
}
let raw=WalkNotchFieldManifest::new(EntityId::from_bits(10),vec![
    WalkNotchFieldIntent::new(LocalTag::new(9),WalkNotchSharingPolicy::CoalesceIdentical,
        WalkNotchFieldPolicy::RetainUnchanged),
    WalkNotchFieldIntent::new(LocalTag::new(2),WalkNotchSharingPolicy::CoalesceIdentical,
        WalkNotchFieldPolicy::NewRequested),
]);
assert_eq!(raw.intents().len(),2);
```

For E complete identity choices,M complete original assignments and F original field policies,
checked E+2F precedes whole M,E,F and workspace quotas,then actual walk-operation equality.
Original policy order checks duplicate role,present identity membership and identity-field kind.
Only then does source pair/A/B order check complete policy coverage and field agreement. Single
occurrence refuses a second use even when its fields are identical. Coalescing compares Piece,
anchor,then complete profile bindings against the first original occurrence. Exact equality covers
style,sample depth/width,production depth/width and encoding. Coordinate coincidence alone is
irrelevant. At the first use of a retained identity,the actual prior Piece must match;unchanged
policy also checks its actual prior anchor and complete profile bindings.

The temporary logical upper bound covers F policy-index entries,at most E group entries,and F
associations. It excludes earlier storage and allocator bytes. Work is
O((M+E+F)log(M+E+F+2));there are no string keys. Temporary groups disappear after complete
admission. No representative fields escape until every original occurrence has passed.

`AdmittedWalkNotchFields` retains the actual `manifest()`, `identities()` and `limits()`.
`groups()` follows original policy order in both directions;`group(role)` queries in
O(log(F+2)) work. A group lends its actual original intent,authentic identity choice and complete
owner. `first()` identifies the first original pair/A/B occurrence after full agreement;
`piece()`, `anchor()` and `representation()` borrow its actual now-proven common fields.
`occurrence_count()` counts every original occurrence in that group,not newly issued entities.
No replacement definition,canonical Notch,geometry or profile value is manufactured.

For copies200/201 of the same actual Piece100,two requests can deliberately share role2/id500
when both requested anchors and all profiles are identical. The native result has one group with
two occurrences;both physical-copy identities remain separately inspectable in the actual parents.
With different Pieces,the same role refuses Piece disagreement. On one Piece,different parameters
or any one of the six profile fields refuse their exact cause. Four repeated requests can instead
form two deliberately shared groups. Choosing `SingleOccurrence` refuses those repeated uses.

If retained700 has prior anchor1/4 but the request is1/3,`RetainUnchanged` refuses Anchor.
`ReviseRetained` can accept the explicit requested-field proposal on the same Piece while lending
the original unchanged700 as prior context. The same distinction applies to profile bindings.
Trying to retain a Notch born on another Piece refuses Piece even with revision selected.
Unmentioned supplied Notches remain untouched;absence never retires an entity or active role.

A semantic Notch belongs to a pattern Piece;its physical uses are not limited to the two selected
sewing copies. Exact field agreement therefore grants no all-copy suitability or correspondence
certificate. Complete current Design/registry/history,private reserve/nonreuse,visible repair,
correspondence/differential/ease,profile resolution and all-copy/authority evidence remain mandatory
before canonical construction or atomic publication. Native policy agreement executes no edit.

## Mandatory walk evidence requests

`AdmittedWalkNotchFields::require_evidence` binds a complete `WalkNotchEvidencePolicy` to the
actual field/identity/attribution/copy/role/source context. This creates `RequiredWalkNotchEvidence`,
an authentic association of pending obligations. It accepts no provider result and performs no
registry lookup,geometry work,canonical Notch construction or edit.

| Mandatory scope | Evidence still required from execution |
| --- | --- |
| `CurrentOwnership` | Actual complete current Design/revision,graph,plan,Pieces,ledger and semantic registries |
| `IdentityHistory` | Global issued/nonreused identities,private reservation and retained role/entity history |
| `CanonicalNotches` | Canonical birth/revision,prior holder preservation and complete result semantics |
| `VisibleRepairs` | All affected consumers,held references,ambiguity choices,gaps and visible repairs |
| `Correspondence` | Actual paired source positions and physical sewing correspondence |
| `Differential` | Actual measured lengths and signed differential under declared error budgets |
| `EaseAndStops` | Original copy-addressed direction,ease distribution,stops/turns and conservation |
| `Profiles` | Style,sample/production dimensions and encoding resolved with actual profile evidence |
| `AllCopyEffects` | Every affected pattern copy and its semantic/sewing neighbours and effects |
| `AuthoringAuthority` | Actual authoring authority,selected-holder repair and atomic publication/history |

Four source fields are explicit in `WalkNotchContextSources`;six in `WalkNotchEffectSources`.
`WalkNotchEvidencePolicy::new` requires both complete records. There are no optional or defaulted
scopes. `context_sources()` and `effect_sources()` borrow the actual embedded records;
`source(kind)` lends the actual original requested identity field,not a resolved provider.

```rust
use sc_core::{construction::*,ontology::EntityId};
let source=EntityId::from_bits(0);
let policy=WalkNotchEvidencePolicy::new(
    WalkNotchContextSources::new(source,source,source,source),
    WalkNotchEffectSources::new(source,source,source,source,source,source),
);
fn require<'a>(fields:&'a AdmittedWalkNotchFields<'a>,policy:&'a WalkNotchEvidencePolicy)
    ->RequiredWalkNotchEvidence<'a> {
    fields.require_evidence(policy)
}
assert_eq!(*policy.source(WalkNotchEvidenceKind::AllCopyEffects),source);
```

Zero is an ordinary source value,not an absent/satisfied sentinel. Shared values also leave every
field and scope distinct. `requirements()` returns all ten obligations in the table's fixed order,
with exact-size forward/reverse traversal. Each item lends its actual complete `context()`,distinct
`kind()` and original `source()` field. `fields()` and `policy()` retain the same actual complete
parents. Borrowing and fixed ten-item traversal take constant work/auxiliary space and allocate
nothing;this bound covers the native request association,not future provider execution.

Two shared field groups covering four original requests still require all ten scopes. One provider
may eventually serve several scopes,but its existence or success cannot be inferred from equal
ids. Retaining prior marks,unchanged fields,unlisted proposed ids or exact group agreement discharges
none of these obligations. Registry completeness and all-copy effects include unselected copies,
other sewing neighbours and consumers of revised prior marks;the two selected sides are no census.

Actual source/currentness checks,global private-reserve/nonreuse,Notch construction/revision,
profile and geometry providers,visible repair,atomic execution and broader integration remain
separately owned. This native request association is not an execution-readiness or release token.
