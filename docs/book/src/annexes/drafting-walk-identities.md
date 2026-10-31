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

Bounded known-identity exclusions and explicit role-to-semantic-identity/retention proposals
follow in owned slices. Actual current Design/registry/history,global reservation/nonreuse,
semantic sharing/correspondence,profile and all-copy effects,geometry and atomic authorized
publication retain their separate required execution owners.
