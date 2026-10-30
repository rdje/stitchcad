# Bounded span identity histories

The native history component validates supplied span births and splits in authored order. It keeps
retired parents and their two immediate child identities,including nested splits. This is local
identity metadata. Optional actual graph binding,Design ownership and complete resource accounting
and bounded actual source/stop graph candidates are implemented below. Held Gather identity
inspection and complete garment candidate ownership/resources are available. The SplitSeamSpan
bus handler and physical/provider/authority/effect integration remain pending.

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
| `EntryCountOverflow` | Appending a record cannot be represented as a complete entry count |
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

## Extend an accepted snapshot

| API token | Meaning |
| --- | --- |
| `with_split` | Build one new immutable snapshot from an accepted prefix and supplied split |

The complete next population counts every existing record plus one split,and every previously
issued identity plus both children. Explicit entry quota precedes identity quota,both before any
vector allocation or prefix copy. Missing or retired parent comes next,then first/second child
nonreuse. Retired ancestors and unrelated roots still own their IDs;retirement refunds no capacity.
Original refusal locations use the actual next record and original birth/retirement positions.

After admission and all identity checks,the operation copies the unchanged accepted prefix once
and appends the split. It builds no new validation map. The old snapshot remains usable and exact
on success and refusal. Different sufficient allowances produce equal new canonical content.
Child order stays supplied A order,including when the first ID sorts after the second.

This example is compiled as the new API's native doctest:

```rust
use core::num::NonZeroUsize;
use sc_core::ontology::{EntityId, SeamSpanHistory, SeamSpanHistoryEntry as E,
    SeamSpanHistoryLimits, SeamSpanHistoryState as S};
let parent = EntityId::from_bits(7);
let children = [EntityId::from_bits(9), EntityId::from_bits(3)];
let limits = SeamSpanHistoryLimits::new(NonZeroUsize::new(2).unwrap(),
    NonZeroUsize::new(3).unwrap());
let original = SeamSpanHistory::new(vec![E::Root(parent)], limits).unwrap();
let candidate = original.with_split(parent, children, limits).unwrap();
assert_eq!(original.state(parent), Some(S::Live { birth_entry: 0 }));
assert!(matches!(candidate.state(parent), Some(S::RetiredSplit { children: ids, .. })
    if *ids == children));
assert_eq!(candidate.entries().first(), original.entries().first());
```

This is a supplied identity transition. It generates or reserves no IDs and grants no global
freshness,actual graph edit,physical correspondence or command authority. Production creation
continues through the trusted bus reserve. Actual source/stop/graph candidates and held-reference
repair remain their next integration stages. Count limits bound logical populations,not allocator
bytes or CPU. Checked count-overflow guards are separate from the realizable native boundary cases.

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

## Build the actual paired graph candidate

| API token | Meaning |
| --- | --- |
| `CommandSeamSplitGraphLimits` | Explicit graph payload and complete next-history allowances |
| `CommandSeamSplitGraphAdmission` | Exact actual owners,admitted payload and extended history |
| `CommandSeamSplitGraphError` | Original owner,quota,identity,current graph or output refusal |
| `admit_graph_candidate` | Join complete actual source admission with captured ease/stop owners |
| `build` | Consume the admission and construct a structurally current graph candidate |
| `source` | Exact complete source admission |
| `stop_source` | Exact captured ease and ordered stop owners |
| `children` | Tentative supplied child identities in original A order |
| `limits` | Selected runtime allowances;not canonical graph content |
| `reservation` | Conservative complete graph payload reservation before copying |
| `resources` | Selected graph resource envelope |
| `history` | Selected complete next-history allowances |
| `SourceOwner` | Different actual source state,even if IDs/content compare equal |
| `RequestOwner` | Different actual request object |
| `TargetOwner` | Different actual target object or missing actual graph-member ownership |
| `MissingHistory` | Legacy graph has no attached actual history;past edits are not inferred |
| `SpanCountOverflow` | Next graph span population cannot be represented |
| `Resources` | Original resource refusal,including field and owner |
| `History` | Original complete next-history or child nonreuse refusal |
| `Portion` | Original captured source-frame/interior refusal |
| `Graph` | Original current copy/range/source/stop/namespace/distribution refusal |
| `HistoryBinding` | Original exact live-leaf/source-ancestry refusal |
| `ReservationExceeded` | Measured output exceeds its pre-copy reservation;no graph escapes |
| `candidate.span_split.source` | Full captured-root reservation |
| `candidate.span_split.stops` | Actual aggregate child stop-reference reservation |
| `candidate.span_split.children_and_history` | Two child slots plus one appended history record |

Admission requires the complete source token and actual sealed capture/stop join. It compares the
actual state pointer,request pointer and target pointer in that order;equal IDs or equal bodies do
not substitute. The target must be the actual graph member. Existing attached history is mandatory.
The caller supplies tentative children;this stage generates or reserves no identities.

Before copying history or graph definitions,the conservative payload reservation charges the full
actual source graph,full captured-root population,aggregate child stop references and three extra
entries for two children and one history record. With source graph population S,capture population
C and child references R,this is S+C+R+3. Existing source/history content and discarded target data
remain conservatively charged. The graph and history envelopes are independent explicit inputs.
Then the bounded immutable transition checks complete next counts and identity rules before copying
the exact history prefix once. No input host or landmark registry is cloned.

Build replaces the target in place by its two A-ordered child definitions. Unrelated definitions
remain exact and in authored order. Children retain exact paired source intervals,physical-copy
identities,direction and ordered authored closed stop memberships. A shared cut stop keeps its same
semantic ID in both children. Full original ease bodies remain shared and unchanged;child amounts
are not new literals,and missing target stops are not filled from the original root.

The actual graph constructor checks current structural sources. Exact live/history ancestry binds
the new history,and current namespace validation checks every history birth. A final actual resource
walk must fit both the selected envelope and pre-copy reservation. No partial graph escapes a
refusal,and the original source state/history remain unchanged on both outcomes.

This compiled API example consumes the actual externally owned prerequisites:

```rust
use sc_core::command::{CommandStopSourceBinding,CommandSeamSplitSourceAdmission,
    CommandSeamSplitGraphError,CommandSeamSplitGraphLimits};
use sc_core::{design::DesignReferenceHost,ontology::{EntityId,SewingGraph}};
fn candidate<'a,H:DesignReferenceHost>(joined:&'a CommandStopSourceBinding<'a,H>,
    source:&'a CommandSeamSplitSourceAdmission<'a,H>,children:[EntityId;2],
    limits:CommandSeamSplitGraphLimits)->Result<SewingGraph,CommandSeamSplitGraphError>{
    joined.admit_graph_candidate(source,children,limits)?.build()
}
```

In the actual Design fixture,the target sits between two unrelated weighted spans. Original graph
population18,capture5,child references2 and extra3 reserve28 entries. The final graph measures23.
Limit27 refuses at consumed25/requested3 with the actual graph owner;limit28 admits. The history
changes from three independent roots to four records/five issued IDs. Nested splitting appends
another record,keeps all seven issued IDs and retains the same full original root. A Notch exactly
at the nested cut stays in both children without cloning the semantic object or choosing a side.

These are structurally checked graph candidates. They are not complete Design candidates,global
identity-reserve certificates,original-notch or weighted quantity readiness,physical correspondence,
provider availability,authorization or atomic effects. Symbolic formula/profile metadata remains
symbolic. Whole-Design construction and held Gather repair must still run before publication;
trusted production allocation continues through the bus reserve. Bounds concern logical payload
populations,not allocator/RSS,temporary validation indices or CPU.

## Prepare a complete owned garment candidate

| API token | Meaning |
| --- | --- |
| `CommandSeamSplitGarmentAdmission` | Exact complete original garment and admitted clone/output envelope |
| `CommandSeamSplitGarmentError` | Original owner,quota,current source,namespace,clone or output refusal |
| `DesignGarment` | Complete canonical owner,including inspectable held damage |
| `DesignResourceLimits` | Explicit complete collection,text and counter envelope |
| `DesignResourceSource` | Trusted host's honest exhaustive readonly accounting port |
| `admit_garment_candidate` | Extend actual graph admission with whole original garment checks |
| `original` | Borrow the exact full original owner |
| `graph_admission` | Borrow the actual prepared source/stop/history graph stage |
| `Owner` | Supplied garment does not own the exact actual source sewing state |
| `Current` | Original complete garment's actual current-source refusal |
| `ChildOwned` | First A-ordered proposed child already belongs to a canonical owner |
| `Sewing` | Original candidate sewing ownership/current-input constructor refusal |
| `Garment` | Original complete candidate ownership/current-input constructor refusal |
| `ContextChanged` | Cloned content outside the prepared graph changed |

This stage consumes graph admission and checks the actual garment's sewing-owner pointer first.
Equal content/IDs from a different garment do not substitute. Before any garment-component clone
or identity census,it charges the complete original garment and the same conservative graph
reservation through one cumulative meter. All collection,text and counter demand keeps its actual
component locations. Explicit whole-garment limits are independent of the earlier graph/history
limits. Existing graph history was already separately admitted before this stage.

Next it validates the complete original garment,including every unused component and construction.
Then it checks both tentative children in A order against the complete original namespace. A child
colliding with an actual Pocket is refused at that Pocket's original position;being absent from
the sewing graph alone never proves whole-Design freshness. Previously retired history identities
remain owned and are never refunded or reused.

Build moves the prepared graph and clones the actual inputs,landmarks and every other garment
component once. Exhaustive owner views require an explicit decision when the schema gains a field.
Existing sewing/garment constructors validate full ownership/current inputs. Unchanged-context
comparisons cover canonical inputs/host and every held component outside the graph. A final actual
whole-garment resource walk must fit the selected envelope and complete pre-copy reservation.
The actual source port must honor its honest exhaustive readonly accounting contract;Clone and
PartialEq must preserve semantic content. This is no security claim against dishonest host code.

This compiled public example constructs the complete inspectable owner:

```rust
use sc_core::command::{CommandSeamSplitGraphAdmission,CommandSeamSplitGarmentError};
use sc_core::design::{DesignGarment,DesignResourceSource,DesignResourceLimits};
fn candidate<'a,H:DesignResourceSource+Clone+PartialEq>(graph:CommandSeamSplitGraphAdmission<'a,H>,
    original:&'a DesignGarment<H>,limits:DesignResourceLimits)
    ->Result<DesignGarment<H>,CommandSeamSplitGarmentError<H::Error>>{
    graph.admit_garment_candidate(original,limits)?.build()
}
```

The actual full garment fixture owns Pockets,notions,Closures,layers,Hems and all six construction
families. Its original demand is109 collection entries,937 text bytes and zero counter bits.
The shared fixture includes formula92,authored as authored_body+40 mm. Its source and lookup
vectors add two entries;the original19-byte expression and30-byte canonical buffer add49 bytes
to the older107-entry/888-byte fixture. The watcher checks those exact buffers before the complete
resource walk,so a changed shared fixture cannot silently reuse an obsolete footprint.

The graph reserves six entries,so complete admission needs115. The built owner measures112 entries
and the same937 text bytes/zero counter bits. Limit114 refuses at consumed112/requested3,with the
original graph owner and field. Limit115 admits. Unchanged component equality and current original
checks pass;the full source remains exact after successful and refused preparations.

The old Gather stays in the actual construction inventory with its original target/copy/side/
direction/closing-operation IDs. The returned candidate is globally owned and inspectable;fresh
current validation still reports the original typed Gather missing-span cause for its now-retired
target. Identity inspection lends both immediate children. Attempting another preparation from
that already-damaged whole garment refuses its original current cause before further clones.
No cached successful current certificate,repair selection or silent retarget is produced.

The actual private bus reserve constructor accepts the two new terminal128-bit IDs before they
are owned,then refuses the candidate's retired parent and both children as AlreadyOwned with
exact history birth/child positions. It observes no authority snapshot. This verifies local
namespace/reserve configuration only;it does not consume a cursor,authorize anything,prove atomic
reserve/model publication or supply persistent global issuance across restarts. Production
allocation still uses the trusted private reserve stage before effects. No new handler is enabled.

Four actual garment cases verify complete content/ownership,original Gather repair,wrong owner,
resource-before-namespace priority,Pocket collision,exact boundary retry and original current
refusal. Four doctests cover admission/build composition,private owner and source lifetime.
Five existing graph cases still pass after shared reservation factoring. Scoped strict lint and
formatting pass;whole paired/physical/provider/authority/effect verification retains its later owners.

## Inspect a held Gather after retirement

The Gather's identity query is available through inspect_span_reference,documented with the
[Gather contract](ontology-constructions.md). It checks the original graph ID before inspecting
the exact held span ID. Live results borrow the actual graph member;retired results borrow both
immediate child IDs from the original retirement record. Later splits of those children do not
replace the old target's children with a chosen descendant. Unknown targets and legacy source
provenance without attached history remain missing;no history is manufactured.

This native doctest compiles the public inspection API:

```rust
use sc_core::ontology::{Gather,GatherError,SewingGraph,SewingSpanLookup};
fn inspect<'a>(gather:&Gather,graph:&'a SewingGraph)
    ->Result<SewingSpanLookup<'a>,GatherError>{
    gather.inspect_span_reference(graph)
}
```

The query does not update the Gather,select a child,query geometry/providers or allocate a result
collection. It supplies identity evidence for repair. Existing intake/current queries retain their
original missing-span refusal for a retired target,plus original copy/sign/source checks for live
targets. Identity inspection is separate from successful current validation or completed repair.
Borrowed members/retirement arrays cannot outlive their actual graph.

The native core case splits301 into702/701,then702 into800/799. The original held Gather still
names301 and lends702/701 from the exact original history record on both later graphs. Its intake
and current validation still refuse301. The actual paired Design case similarly retains the held
Gather and old SplitSeamSpan request after real graph candidate construction:inspection reports
the original target's immediate children,while current Gather intake and the old request refuse
the retired target. Neither selects one of the new children or changes the held input.

Two core cases also verify original live-member pointers,wrong graph,unknown/legacy absence and
unchanged live-copy validation. All18 Gather contracts and five actual graph candidate cases pass.
Four Gather doctests include API composition and graph lifetime;scoped strict lint/formatting
pass. Complete garment preparation above preserves those held references and current refusals;
publication and completed repair retain their separate obligations.

## Implemented scope and next integration

Six native contract cases cover nested/sibling/unknown states,actual record-pointer retention,
empty/full-width IDs,complete quota precedence and original refusal locations. A closed-form
chain check uses depths1,2,17,32 and257:each split retains its old parent,two immediate children
and the expected sibling leaves. Four doctests verify the compiled example,private owner,explicit
limits and borrowed retirement lifetime. Scoped strict lint and formatting pass.

Three extension cases verify exact snapshots/prefixes,complete quota/parent/child precedence,
unrelated and retired ID nonreuse,empty history and descending full-width child IDs. A120-case
comparison agrees with the existing complete map validator,which rebuilds the entire supplied
history through a separate algorithm. This is a component comparison,not an external global-ID
oracle. The extension doctest also compiles;full paired verification remains deferred.

Five actual graph cases and two actual Design cases verify binding,source ancestry,borrowed object
identity,canonical ownership and resource boundaries. Four existing sewing resource cases and29
stop cases pass as direct regressions. Eight relevant doctests cover the history and graph APIs,
privacy and lifetimes. Scoped strict lint and formatting pass;full paired verification is deferred.

Four actual candidate cases cover Same/Opposite and A/B pairing with literal/formula/profile ease,
exact unrelated-member retention,full original root sharing,nested history and shared cut stops.
They also verify owner substitution,legacy history,resource/history precedence,current namespace
refusal,unchanged originals and exact-boundary retry. Four doctests verify API composition,private
admission,borrowed source lifetime and no default policy;four existing source-join cases pass.

Held Gather inspection and complete garment ownership/resource preparation are implemented.
The deferred lineage component checkpoint is complete after these implementation slices. It
passes47 direct native cases:history9,actual graph/history5,Gather18,actual Design history2,
graph/garment candidates9 and captured source joins4. Its21 doctests cover history5,graph4,
Gather4 and candidates8,including owner/privacy/lifetime boundaries and compiled API examples.
The Rust sources remain exact to the completed0573 implementation;its scoped strict lint and
formatting still apply. This checkpoint changes no runtime code and runs no broad CI/mutation audit.

Actual paired effects still require real formula/Profile/geometry/physical providers,current
repair policy,fresh authority,aggregate runtime admission and atomic model/reserve/history
publication with undo/replay/cancellation. No history input executes a command or retargets a
Gather. Formula source-order execution resumes under the [checked runtime plan](../annexes/formula-execution-plan.md).
The whole paired command acceptance remains separately owned until those dependencies exist.
