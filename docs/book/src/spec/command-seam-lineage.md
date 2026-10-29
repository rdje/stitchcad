# Bounded span identity histories

The native history component validates supplied span births and splits in authored order. It keeps
retired parents and their two immediate child identities,including nested splits. This is local
identity metadata. Graph/Design ownership,retained-source aliases and actual paired candidate
binding are the next integration stages. The SplitSeamSpan bus handler remains pending.

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
| `limits` | Exact selected admission limits |
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
retained without cloning or sorting. Debug prints fixed counts/limits,without walking the records.
These are logical population bounds;allocator bytes,CPU and complete command resources remain
separate obligations.

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

## Implemented scope and next integration

Five native contract cases cover nested/sibling/unknown states,actual record-pointer retention,
empty/full-width IDs,complete quota precedence and original refusal locations. A closed-form
chain check uses depths1,2,17,32 and257:each split retains its old parent,two immediate children
and the expected sibling leaves. Four doctests verify the compiled example,private owner,explicit
limits and borrowed retirement lifetime. Scoped strict lint and formatting pass.

The next stage binds these records to the actual graph's live spans,retained original ease-source
identities and complete Design ownership/resource walks. Actual paired source/stop joins and
held Gather/reference repair evidence follow. No history input installs a graph or retargets a
Gather. Real source/provider/physical/authority/aggregate-effect checks and whole paired command
acceptance retain their existing owners.
