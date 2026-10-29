# Bounded span identity histories

The native history component validates supplied span births and splits in authored order. It keeps
retired parents and their two immediate child identities,including nested splits. This is local
identity metadata. Optional actual graph binding,Design ownership and complete resource accounting
are implemented below. Actual paired candidate/reference joins and the SplitSeamSpan bus handler
remain pending.

## Explicit records and limits

| API token | Meaning |
| --- | --- |
| `SeamSpanHistoryEntry` | One supplied root birth or paired split record |
| `Root` | Introduce one independent span identity |
| `Split` | Retire a live parent and introduce two fresh children in original A order |
| `SeamSpanHistoryLimits` | Explicit positive entry and issued-identity allowances |
| `SeamSpanHistory` | Private immutable accepted records and actual issued count |
| `SeamSpanHistoryState` | State only within this history,with actual birth/retirement locations |
| `Live` | Introduced and not later retired in these supplied records |
| `RetiredSplit` | Original parent and borrowed immediate child identities,without reassignment |
| `new` | Construct explicit limits or admit and validate supplied history |
| `entries` | Exact original record slice,or selected entry allowance on the limits |
| `identities` | Selected maximum total issued identities |
| `identity_count` | Actual total introduced identities,including retired ancestors |
| `state` | Allocation-free linear lookup in this immutable history |

Every identity is born once. A split requires a parent introduced earlier and still live in this
history. Both children must be new and distinct,including from all retired ancestors. The first
child remains first even when its numeric ID sorts after the second. The history validates supplied
IDs;it neither generates IDs nor certifies their global freshness. Production creation continues
to use the trusted bus reserve and its injected generator.

Both allowances use explicit positive integers with no default. Empty supplied history is valid
under explicit limits. Entry quota is checked first. The complete identity count then counts one
per root and two per split,including unrelated records and every retired identity. Both counts
must fit before any validation-map insertion. One temporary map retains fixed birth/retirement
metadata per introduced identity and is dropped after validation. The original input vector is
retained without cloning or sorting. Debug prints fixed counts,without walking the records.
These are logical population bounds;allocator bytes,CPU and complete command resources remain
separate obligations.

Selected admission limits are runtime policy inputs. They are not retained in the history and do
not affect equality or canonical identity content. Identical accepted records under different
sufficient allowances remain equal. D335's native regression first reproduced the unwanted policy
dependency,then verifies this separation.

For example,the following public API example is compiled by the native history doctest:

```rust
use core::num::NonZeroUsize;
use sc_core::ontology::{EntityId, SeamSpanHistory, SeamSpanHistoryEntry as E,
    SeamSpanHistoryLimits, SeamSpanHistoryState as S};
let ids = [EntityId::from_bits(8), EntityId::from_bits(3), EntityId::from_bits(5)];
let [parent, first, second] = ids;
let limits = SeamSpanHistoryLimits::new(NonZeroUsize::new(2).unwrap(),
    NonZeroUsize::new(3).unwrap());
let history = SeamSpanHistory::new(vec![E::Root(parent),
    E::Split { parent, children: [first, second] }], limits).unwrap();
assert!(matches!(history.state(parent), Some(S::RetiredSplit { children, .. })
    if *children == [first, second]));
assert_eq!(history.state(first), Some(S::Live { birth_entry: 1 }));
```

## Nested splits preserve the old target

A native case records root8,unrelated root0,split8 into3/5,then split3 into1/2:

| Requested ID | Local history state | Retained evidence |
| --- | --- | --- |
| 8 | retired | birth0,split2,immediate children3/5 |
| 3 | retired | birth2,split3,immediate children1/2 |
| 0 | live | birth1 |
| 5 | live | birth2 |
| 1 or2 | live | birth3 |
| 99 | unknown | no accepted state |

Looking up8 still lends3/5 from its exact retirement record. It never picks1,2 or5 as the old
target's replacement. Looking up a retired child retains its separate split. Borrowed child arrays
cannot outlive their history. No descendant tree or candidate collection is allocated by lookup.
A live result says only that these records have not retired that identity;it supplies no current
Piece/copy/range,original ease-root,physical or mutation-authority certificate.

## Refusal evidence and precedence

| API token | Meaning |
| --- | --- |
| `SeamSpanHistoryError` | Original complete quota or chronological identity refusal |
| `Entries` | Actual entry population exceeds its selected allowance |
| `IdentityCountOverflow` | Checked complete introduction count cannot be represented |
| `Identities` | Actual total introductions exceed their selected allowance |
| `MissingParent` | Split names a parent not introduced by an earlier record |
| `RetiredParent` | Actual parent was already split,with its original split location |
| `ReusedIdentity` | A root/child repeats an earlier birth,or the siblings repeat one another |

Quota checks precede semantic checks. For a malformed two-entry history declaring three IDs,
entry allowance1 refuses the complete two entries first. With entry allowance2 and identity
allowance2,the complete count3 refuses next. With both counts admitted,the original missing parent
is reported. A four-entry nested history with four live leaves still needs allowance6 for all
issued identities;retirement does not refund identity capacity.

Within admitted inputs,the actual parent is checked before either child. A previously retired
parent reports its prior split even if the new child IDs also collide. Existing child IDs are
checked in first/second order;equal new siblings name that same split as the first attempted birth.
Errors retain actual record,identity and original birth/split positions. No partial accepted
history escapes a refusal. Full128-bit and zero IDs are accepted as supplied data,without numeric
sorting or a hidden allocation policy.

## Bind history to the actual graph

| API token | Meaning |
| --- | --- |
| `with_span_history` | Consume this graph and bind complete supplied identity history |
| `span_history` | Borrow the exact immutable attached history,or none for a legacy held graph |
| `lookup_span` | Borrow an actual live member or exact immediate retirement evidence |
| `SewingSpanLookup` | Actual graph member or borrowed original retirement record |
| `SeamSpanHistoryBindingError` | Original graph/history identity inconsistency |
| `AlreadyBound` | Existing attached history cannot be silently replaced |
| `MissingLiveSpan` | Actual graph member has no supplied history identity |
| `LiveSpanRetired` | Actual graph member is retired in the supplied history |
| `MissingSource` | Actual retained ease source has no history identity |
| `SourceNotRetired` | Retained original source is incorrectly live in history |
| `UnrelatedSource` | Retained source is not an ancestor of its actual consuming span |
| `UnrepresentedLiveSpan` | A history leaf has no actual live graph member |
| `GraphIdentityCollision` | A supplied span identity collides with the containing graph |
| `SpanHistory` | Design ownership position for one actual history birth and optional child slot |
| `sewing.span_history.entries` | Complete actual history vector in the logical resource walk |

Every actual graph span must be a supplied history leaf,and every supplied leaf must be an actual
member. Every retained original ease-source ID must be retired and an ancestor of each consuming
span. Unrelated families cannot lend one another provenance. Validation checks actual graph members,
then canonical source roots,consumer ancestry and original births,in their respective stored orders.
Ancestry walks strictly earlier birth records without building a descendant tree. Binding builds
no index,clones no model and performs no current reference,physical or provider query.

This compiled public API composes an existing graph and accepted history:

```rust
use sc_core::ontology::{SeamSpanHistory, SeamSpanHistoryBindingError, SewingGraph};
fn bind(graph: SewingGraph, history: SeamSpanHistory)
    -> Result<SewingGraph, SeamSpanHistoryBindingError> {
    graph.with_span_history(history)
}
```

Lookup lends the exact live graph object,not an equal-body replacement. A retired lookup lends both
immediate A-ordered children from the actual attached history,even after further nested splits.
Legacy graphs retain no invented edit sequence:an owned original ease source alone does not supply
retirement evidence. Old command/Gather lookups still keep their original target IDs and current
missing-target refusals;this metadata API does not retarget those consumers.

Current graph validation checks every history birth against actual Piece/copy/landmark/graph
namespaces before span/source queries. Live spans and retained source bodies are then uses of
those same canonical history-owned IDs. Without attached history,the original live-span/source
ownership order is preserved. Complete Design sewing ownership similarly owns every birth once,
including retired intermediate parents,with original record and child-slot positions. The existing
garment census consumes that complete sewing inventory. A retired ID colliding with a canonical
input is refused by the actual Design constructor;local binding grants no global freshness.

Resource accounting charges the actual history vector after canonical ease-source content and
before the separate semantic registry. Each record has only fixed Copy fields. The retained
introduced-ID count is fixed derived metadata;selected admission policy is not stored. Every
snapshot charges its complete history and source bodies again,including shared original backing.
See [actual sewing accounting](model-resource-accounting.md#actual-sewing-accounting).

A native graph has four retained quarter spans,one canonical original source and four history
records. Its graph demand changes from5 to9 collection entries. Limit8 refuses the history field
at consumed5/requested4,with the actual graph owner and no element index. Two snapshots charge18.
The actual Design case retains one original source body and five issued IDs,including a retired
intermediate parent;source/live aliases are owned once. Its three history entries add exactly3
resource items,without changing text or counter demand. Different sufficient admission policies
produce equal actual graph and Design content (D335).

## Implemented scope and next integration

Six native contract cases cover nested/sibling/unknown states,actual record-pointer retention,
empty/full-width IDs,complete quota precedence and original refusal locations. A closed-form
chain check uses depths1,2,17,32 and257:each split retains its old parent,two immediate children
and the expected sibling leaves. Four doctests verify the compiled example,private owner,explicit
limits and borrowed retirement lifetime. Scoped strict lint and formatting pass.

Five actual graph cases and two actual Design cases verify binding,source ancestry,borrowed object
identity,canonical ownership and resource boundaries. Four existing sewing resource cases and29
stop cases pass as direct regressions. Eight relevant doctests cover the history and graph APIs,
privacy and lifetimes. Scoped strict lint and formatting pass;full paired verification is deferred.

Actual paired source/stop candidate joins and held Gather/reference repair evidence follow. No
history input executes a command or retargets a Gather. Real source/provider/physical/authority/
aggregate-effect checks and whole paired command acceptance retain their existing owners.
