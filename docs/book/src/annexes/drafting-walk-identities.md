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
proposals,which remain owned next work. Actual current Design/registry/history,global reservation/nonreuse,
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
