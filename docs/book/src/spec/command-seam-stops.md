# Semantic stops at paired seam cuts

Native contract cases verify the existing graph's stop membership at paired cuts,including
closed endpoints,explicit side/copy identity,authored order and later-frame anchors. Explicit actual
stop input/workspace,ownership-workspace,bounded current context and actual membership rows are
implemented,including ordered child views and the sealed ease-capture/view join. These cases construct real retained child graphs
through public APIs;they do not install a command effect or create new semantic landmark identities.

| API token | Meaning |
| --- | --- |
| `StopLandmark` | Explicit side and existing semantic landmark ID |
| `SewingLandmark` | Actual semantic Notch or TurnPoint in the registry |
| `Notch` | Semantic matching mark with separate deferred physical representation |
| `TurnPoint` | Born-valid semantic edge anchor with geometric turning deferred |
| `SeamSide` | Actual physical-copy identity and positive original interval |
| `SeamSideId` | Explicit A or B label;never inferred from a matching edge |
| `SeamDirection` | Same or Opposite endpoint correspondence |
| `EdgeRange` | Positive interval whose membership includes both endpoints |
| `Param` | Stored bounded rational position;valid wide original views may refuse |
| `StopOutsideSide` | Actual stop cannot belong to the named child's side interval |
| `CommandStopPartitionLimits` | Explicit complete journal,range and workspace allowances |
| `CommandStopPartitionAdmission` | Borrowed actual model/request/target and admitted input bounds |
| `CommandStopPartitionAdmissionError` | Original model/binding/journal or checked workspace refusal |
| `admit_stop_partition_inputs` | Admit actual inputs before current queries or derived collections |
| `DesignResourceLimits` | Complete represented model envelope,independent of stop workspace |
| `DesignResourceDemand` | Exact complete model walk demand retained by admission |
| `UniquePointLimits` | Complete journal/nested input allowance for later point queries |
| `RangeCoverageLimits` | Same journal allowance and selected peak range portions |
| `PointJournalAdmissionIssue` | Complete actual journal/nested cause,without a standard Error trait |
| `Model` | Original complete model resource refusal before target lookup |
| `Binding` | Actual target or A/B frame/interior cut refusal |
| `Journal` | Complete journal/nested cause before workspace arithmetic |
| `BoundOverflow` | Conservative workspace cardinality cannot be represented |
| `Workspace` | Actual conservative reservation exceeds the selected allowance |
| `CommandStopOwnershipLimits` | Separate explicit complete ownership index/cache allowance |
| `CommandStopOwnershipAdmission` | Actual stop input borrow and admitted ownership bounds |
| `CommandStopOwnershipAdmissionError` | Complete edge-count or reservation/allowance refusal |
| `admit_ownership_workspace` | Count actual copies/Pieces/edges before any current query |
| `EdgeCountOverflow` | Counting another canonical Piece edge cannot be represented |
| `CommandStopCurrentContext` | Private actual current registry/Piece/target and pair-range evidence |
| `CommandStopLandmarkEvidence` | Exact semantic object,held query and unique current point |
| `CommandStopPiecePortion` | Complete live or repair portion with query and original ordinals |
| `CommandStopContextError` | Actual target and complete bounded current-context refusal |
| `CommandStopContextIssue` | Structural,point,range,ownership or derived-population evidence |
| `CommandStopContextRange` | Actual Piece edge,target side or paired side being queried |
| `CommandStopContextBoundary` | Actual Start,Cut or End point on the named side |
| `CommandStopContextPopulation` | Complete Copies,Pieces,Landmarks or OwnedPortions guard |
| `validate_current_context` | Build bounded actual local evidence after both admissions |
| `CommandStopMembership` | Exact authored stop,ordinal,physical copy and semantic current evidence |
| `CommandStopMemberships` | Complete private borrowed rows over the exact current context |
| `CommandStopPairMembership` | First,Second or Both closed current pair memberships |
| `CommandStopMembershipError` | Actual target,failed stop ordinal/reference and original cause |
| `CommandStopMembershipIssue` | Actual structural refusal or original N row-bound guard |
| `derive_stop_memberships` | Classify actual authored references without additional queries |
| `CommandStopChild` | First or Second in unchanged A pair order |
| `CommandStopChildView` | Exact projected pair and ordered borrowed original membership rows |
| `CommandStopPartitions` | Complete two-child references bound to the actual membership owner |
| `CommandStopPartitionError` | Actual row/child and original aggregate2N reference refusal |
| `partition_stops` | Build both child lists without new semantic/source objects or queries |
| `CommandStopSourceBinding` | Exact captured ease and ordered stops bound to original request/target owners |
| `CommandStopSourceBindingError` | Actual expected/captured request or target owner mismatch |
| `bind_source_capture` | Require exact owners before lending joined source/child stop metadata |

## A stop at the cut belongs to both closed children

For a side interval[0,1] cut at c,the two positive children are[0,c] and[c,1]. Existing graph
validation accepts a stop exactly at c on either child. The partition contract retains every closed
membership,so both children reference that same semantic object at their shared boundary. No new
Notch or TurnPoint is created. References to boundary marks carry no independently authored ease;
ease quantities remain original rounded cumulative boundary differences.

For example,with A cut1/2 and B cut2/3:

| Actual stop | Same direction: A-first/A-second | Opposite direction: A-first/A-second |
| --- | --- | --- |
| A at1/2 | both | both |
| B at1/2 | first | second |
| B at2/3 | both | both |

The pair order is always lower/upper A. Same pairs those with lower/upper B;Opposite pairs them
with upper/lower B. An arbitrary winner at a shared cut would lose a valid boundary reference.
The current graph constructor validates supplied child stop lists;it does not itself execute this
partition or enforce complete command conservation. The membership stage below derives every actual
target reference;ordered views follow below. Whole command conservation retains its effect owner.

## Preserve side,copy and authored order

A stop carries its named side and landmark identity. Its physical copy comes from that child's
actual side. A landmark on one Piece is not reassigned to another Piece because numeric parameters
or labels match. Different physical copies of one Piece remain distinct material domains.

Copy reflection and Same/Opposite correspondence remain separate metadata. A reflected B copy
still uses its own original B source frame;reflection does not silently complement a stored stop
parameter. Native cases retain copy200 on A and reflected copy201 on B while preserving direction.

Filter each child list in original authored order. If the parent lists Turn401 before Notch400,
each child that contains those stops keeps that order,including at a shared cut. Do not sort by
position,kind or identity. A semantic object can also appear under distinct A and B stop tags;
those explicit side references remain distinct even when the landmark ID is the same.

### Touching self-seam example

One physical copy may join disjoint intervals that touch at a boundary. For A[0,1/2] and B[1/2,1]
on copy200,a semantic landmark at1/2 can carry both explicit A and B stop references. The source
intervals have no positive overlap. With cuts A1/4 and B3/4,Same direction puts its B reference in
the first pair and A reference in the second. Opposite direction puts both references in the
second pair. The original authored B-before-A order remains intact. This is one semantic mark;
no new identity or material copy is inferred.

## Use actual current evidence

Born anchors and original source parameters can live in different topology frames. A real source
edge split at1/3 admits a new stop on its first fragment at1/9223372036854775807. Its exact original
position is1/27670116110564327421,which has a65-bit denominator and no stored Param view. The
native inverse diagnostic preserves that valid original coordinate. Existing graph membership
uses the actual current semantic point and resolved child interval coverage;it does not need to
narrow that original coordinate just to classify the stop.

The contract cases keep original A1/2 and B2/3 cuts,which are distinct from that topology split.
They build the actual current Notch and TurnPoint on the proper Piece and preserve their metadata.
A new anchor cannot be born on a retired source edge:the semantic constructor rightly refuses it.
The test captures its original definition before the edit,then supplies the real later anchor.

Actual choices,repairs,lost interiors,wrong sides and numeric refusals remain typed evidence. A
matching source ID or a surviving endpoint supplies no whole interval/currentness certificate.
The bounded production stage must admit its actual populations and point/range workspaces before
querying,retain all refusal context and return no partial accepted partition. Local membership
success will not supply physical correspondence,authority or atomic effects.

## Focused contract proof

Two native cases cover32 independent Fraction membership rows with four exact shared-cut controls,
65-bit later originals and28 wrong-child refusal controls. Four additional copy/direction scenarios
cover reflected copies and touching self-seams. Both semantic kinds,exact IDs,side tags,full original
stops and authored order survive. Physical Notch representation and actual geometric turning stay
deferred to their existing owners.

The new independent stop oracle is registered in the closed required oracle gate. Ten producers
match twelve frozen fixtures;the nine gate controls still pass. Actual current membership follows
the input admission below,with ordered borrowed views available after the complete membership rows.
Whole paired lineage/atomic effects and physical/mutation acceptance remain later work.

## Admit actual inputs before current queries

Call the real request's `admit_stop_partition_inputs(&state,model_limits,stop_limits)` over the
actual owned DesignSewing component. The source host implements the existing readonly reference
and exhaustive resource contracts. This stage walks the complete represented model under its
explicit logical envelope,then binds the actual held target and source cuts. It does not run
semantic/current callbacks,resolve points/ranges,build indexes,clone roots or evaluate values.

Next it uses the existing complete journal/nested admission shared by point and range queries.
Unrelated edits and nested declared-edge/offset-fragment collections remain counted. It then
checks a separate conservative workspace reservation before any derived allocation.

Let N be the actual target span's authored stop count,M the full supplied semantic registry count,
and B the selected per-range peak. Reserve M+N+2N+4B logical items:

| Reserved population | Bound | Purpose |
| --- | --- | --- |
| Complete semantic metadata index | M | Includes every unused registry entry |
| Target stop membership rows | N | One row per actual authored side/landmark reference |
| Total child stop references | 2N | A shared cut may reference the same stop in both children |
| Four paired-side range caches | 4B | Two A-ordered pairs,each with A and B coverage |

The range peak includes active input and transient output. Earlier cached queries plus the active
query fit within four such allowances. These are conservative logical populations;fixed headers,
allocator capacity,CPU and repeated whole-command snapshots require their own accounting. No
workspace is allocated by admission itself. Even a very large representable reservation remains
scalar metadata;overflow refuses instead of substituting a smaller guessed allowance.

For example,N2/M2/B2 reserves16 items. Limit15 refuses with actual target,all three selected
populations,required16 and limit15. Select every allowance explicitly:

```rust
use core::num::NonZeroUsize;
use sc_core::command::CommandStopPartitionLimits;
use sc_core::ontology::UniquePointLimits;

let stop_limits = CommandStopPartitionLimits::new(
    NonZeroUsize::new(16).unwrap(),
    UniquePointLimits::new(NonZeroUsize::new(1).unwrap(), NonZeroUsize::new(2).unwrap()),
    NonZeroUsize::new(2).unwrap(),
);
```

These numbers belong to that fixture,not an implicit default policy. The complete model envelope
is separately selected. `range_limits()` preserves the exact same journal/nested allowance with
the chosen B. Existing point/range queries still own their concrete per-query refusal behavior.

### Target stops and full original stops remain distinct

Partition the actual target's authored stop list. A retained target can have N0 while its original
ease root still retains two original stops for quantity provenance. Do not backfill that root list
into the child. With M2/B2,the conservative input reservation remains10 even for N0. Adding an
unused actual TurnPoint makes M3 and reserves11;limit10 cannot hide the extra registry entry.

The private admission token lends the exact `state()` and `binding()`,including original request,
target and projected pairs. It retains `model_demand()`,`model_limits()` and the exact stop limits.
Getters expose actual N/M,range cache,membership,child reference and aggregate reservations. Its
fields and lifetime prevent fabricating admitted inputs or escaping their actual owners. No
semantic object or root has been copied by this stage.

### Refusal order and currentness boundary

The exact order is complete model envelope,actual target/A/B frame/interior cuts,complete journal/
nested admission,checked reservation,then selected workspace allowance. A complete model refusal
precedes a missing target;a missing target precedes journal quota. Complete journal count precedes
nested processing,and both precede a huge selected range budget that cannot be represented.

Model and binding variants retain their original standard error sources. Journal retains the
full existing typed issue,including actual operation/collection/consumed/requested/limit;that
fixed evidence does not implement the standard Error trait. Workspace and overflow variants retain
actual target/counts/selected budget data. No partial successful input token escapes any refusal.

An unrelated second declaration changes the complete journal from1 to2 entries and nested edges
from2 to3. Journal limit1 refuses first;with2 entries admitted,nested limit2 refuses the later
actual declaration with consumed2/requested1. A limit3 then admits the complete inputs. Multiplying
an enormous B by4 and adding actual N/M also use checked arithmetic before allocation.

These are admitted held inputs,not accepted current memberships. After copy96000 is explicitly
replaced with96010,the original graph can remain inspectable and its stop input metadata can
admit. The earlier complete current validator still refuses MissingCopy on the actual child/A.
No current callback or numeric value has been smuggled into this token. Bounded copy/point/range
validation must follow even when N is zero;only its successful result can support later ordered
partition views. Whole source/physical/authority/aggregate operation/effect readiness stays separate.

## Admit the complete ownership workspace separately

Current Piece/range ownership needs its own complete indexes and cached interval evidence.
After stop input admission,call `inputs.admit_ownership_workspace(ownership_limits)`. This counts
the actual immutable model's full copy population C,Piece population P and canonical held edge
population E. Edges include every boundary,hole and construction-line entry in authored order,
including unused Pieces and copies. No semantic/current callback,query,index,cache or clone runs.

Choose the positive `CommandStopOwnershipLimits` explicitly;there is no default. Let B be the
exact per-range peak already selected by the stop input token. Reserve C+P+(E+1)B logical items:

| Population | Bound | Purpose |
| --- | --- | --- |
| Copy and Piece indexes | C+P | Complete actual borrowed metadata rows |
| Retained owned interval evidence | E*B | At most B portions from every actual held edge |
| One transient Piece-range query | B | Active bounded fold alongside the retained cache |

The original M+N+2N+4B stop/registry/pair reservation remains separate and unchanged. Ownership
admission cannot widen point/range limits or borrow a different same-ID input owner. It retains
the exact `inputs()` borrow,selected `limits()`,actual C/P/E and checked interval/cache reservation.
All edge increments,multiplications and additions are checked before allocation. The token has
private fields and cannot escape its actual admitted input owner.

```rust
use sc_core::command::{CommandStopPartitionAdmission,CommandStopOwnershipAdmission,
    CommandStopOwnershipAdmissionError,CommandStopOwnershipLimits};
use sc_core::design::DesignReferenceHost;

fn ownership<'a,H:DesignReferenceHost>(inputs:&'a CommandStopPartitionAdmission<'a,H>,
    limits:CommandStopOwnershipLimits)
    ->Result<CommandStopOwnershipAdmission<'a,H>,CommandStopOwnershipAdmissionError>
{
    inputs.admit_ownership_workspace(limits)
}
```

With C2/P2/E2/B2,the retained interval bound is4 and the complete ownership reservation is10;
limit9 refuses with actual target/counts/required10/limit9. This is additional to the earlier16-item
stop input reservation. An actual unused Piece,its required physical copy and three additional
held edges change C/P/E to3/3/5. The retained bound becomes10 and the complete reservation18;
limit17 cannot hide that unused schema data even though target stops and registry remain2/2.

The earlier stop/pair reservation can fit a huge selected B while the complete ownership cache
cannot. Its checked E*B or total then refuses with the actual C/P/E/B and target identity before
querying or allocating. The extra ownership cap is not inferred from model totals or silently
subtracted from the original stop allowance. These remain conservative logical bounds,with
allocator bytes,CPU and aggregate command/snapshot policy owned separately.

A held missing copy still admits these metadata counts. The original current validator continues
to refuse the actual child/A/copy;ownership admission supplies no current membership certificate.
Bounded complete registry/Piece and target/cut/range validation is available below;actual stop rows and
ordered child views follow. Physical geometry,current effect authority and whole paired acceptance remain
separate. Three focused actual Design cases and privacy/default/lifetime doctests cover this stage.

## Validate the bounded actual current context

Call the ownership token's `validate_current_context()`. A successful private context borrows that
exact token and its actual immutable input owner. It cannot be constructed from IDs or returned
after its admitted owner expires. No value provider,current host callback or legacy choice-tree
resolver runs. The complete journal/nested and range limits remain the exact selected allowances.

```rust
use sc_core::command::{CommandStopOwnershipAdmission,CommandStopCurrentContext,
    CommandStopContextError};
use sc_core::design::DesignReferenceHost;

fn current<'a,H:DesignReferenceHost>(ownership:&'a CommandStopOwnershipAdmission<'a,H>)
    ->Result<CommandStopCurrentContext<'a,H>,CommandStopContextError>
{
    ownership.validate_current_context()
}
```

The stage indexes every actual copy and checks the target's named A then B copy before range
queries. The immutable structural owner's constructor already validated its actual copy plan
against its Piece inventory. The context uses those exact objects and checks held graph references
against them. It does not clone a plan to reconstruct that birth proof inside the C+P index budget.

Every canonical held Piece edge is queried through bounded interval coverage. All live and repair
portions move into the admitted cache,including unknown,lost and numeric repairs on unused Pieces.
`piece(id)` returns the actual Piece;`piece_portions(id)` exposes every retained row. Each row's
`held()`,`source_edge_index()` and `portion_index()` preserve its whole-edge query and original
boundary/hole/construction traversal provenance. Working order groups current edges and parameters
for exact union containment;in-place unstable sorting uses original ordinals for deterministic ties
and requires no extra heap index. No repair is silently filtered to make a Piece appear healthy.

The complete semantic registry is then validated,including unused Notches and TurnPoints. Each
mark must have its actual Piece,a unique bounded current point and membership in that Piece's
surviving interval union. `landmark(id)` lends the original object and its complete held/current
point evidence. It does not clone semantic definitions or require a stored original source Param.
A later anchor on the first third of an old edge at local1/9223372036854775807 remains valid here,
even though the independent inverse contract gives its original position a65-bit denominator.

### Whole intervals and target points are separate obligations

For each named target side,the held Start,Cut and End points must resolve uniquely. A topology
split at the command cut refuses with the original SplitChoice evidence. Rejoining both fragments
later does not erase that held-reference choice. The same rule applies to a retained target's
endpoint,even if that target has no authored stops. No immediate branch candidate is chosen for it.

The stage then requires nonempty,complete bounded target A/B coverage,each current portion wholly
contained in the actual named copy's Piece. Same-copy target ranges may touch,but positive current
overlap refuses. Full-side temporary evidence is released before four A-ordered pair-side caches
are built. `first_pair_coverage(side)` and `second_pair_coverage(side)` retain those exact complete
coverages;Opposite correspondence changes B pairing without changing side/copy identity.

Checking surviving endpoints and the cut is insufficient. An actual offset retaining[0,2/5] and
[3/5,1] leaves those tested points and the two quarter-position marks valid when the cut is1/4.
The full target nevertheless refuses with all three portions,including the lost interior. A separate
actual split/merge/offset case has all six A/B Start/Cut/End points unique but an internal range
boundary whose final exact ratio exceeds the stored field. That refuses with the complete numeric
range evidence and its original UnitError. The query kernel is unchanged by this context stage.

### Evidence,limits and scope

`CommandStopContextError` retains the actual consuming target and an inspectable issue. Structural
errors retain original copy/Piece/identity/self-overlap causes. Point issues retain the actual mark
or side/boundary and fixed query resolution. Coverage-limit issues retain the original query error
and its Piece-edge/target/pair location;coverage and outside-Piece issues retain the complete query
evidence. Standard error sources borrow the original structural,range quota or numeric error;
choice and nonnumeric repairs remain typed evidence without an invented numeric cause.

The C/P/M indexes and E*B cache use the already admitted logical populations,guarding every derived
append or insertion. One transient Piece query fits its separate B allowance;paired query caches
fit the original4B allowance even at peak. Actual index/cache counts are inspectable through
`copy_count()`,`piece_count()`,`landmark_count()` and `owned_item_count()`. Fixed-size point evidence
does not allocate a choice tree. Allocator capacity,CPU and aggregate command/snapshot policy remain
separate obligations. Every refusal returns no partial successful context.

This is local registry and target evidence. A lost interval on an unused Piece is retained in
`piece_portions()` while the valid target can still obtain a context;that success makes no claim
about whole Piece health. It also supplies no stop-reference conservation,original ease-root/value
evaluation,physical correspondence,current mutation authority or atomic command effect. Actual
authored stop rows and ordered child views are available below,with their own implementation owners.

Ten actual cross-crate Design cases cover the successful and refusing paths above,including exact
borrowed object identities,unused registry data,N0 missing copies,identity-range quota1 versus
required peak2,touching self-seams,later anchors and original numeric sources. Privacy/lifetime and
public API doctests pass. Component and whole paired acceptance retain their later task owners.

## Derive actual authored membership rows

Call `context.derive_stop_memberships()` after bounded current validation. This stage needs no host
trait or new point/range query. It borrows the exact current context,actual target stop references,
named physical copies and already validated semantic evidence. It compares each current point
against the named side's two cached closed paired interval coverages. The original N reservation
guards each row before append. No inverse,narrowed source coordinate,value evaluation,new semantic
identity or extra lookup collection is introduced.

```rust
use sc_core::command::{CommandStopCurrentContext,CommandStopMemberships,
    CommandStopMembershipError};

fn memberships<'a,H>(context:&'a CommandStopCurrentContext<'a,H>)
    ->Result<CommandStopMemberships<'a,H>,CommandStopMembershipError>
{
    context.derive_stop_memberships()
}
```

The private result's `context()` returns that actual source borrow. Its `rows()` have unchanged
authored order and exactly one row per actual target stop. Each row retains `ordinal()`,`stop()`,
`copy()`,`landmark()` and `membership()`. The stop getter lends the original side/landmark pair;
the copy getter lends the exact physical copy;the landmark view lends the original Notch or Turn,
its held query resolution and current point. No position/kind/ID sorting changes authored order.
The semantic object's profile/geometric obligations stay visible in that original object.

Membership is First,Second or Both in A pair order. `includes_first()` and `includes_second()`
provide the two explicit predicates. At a closed cut,Both preserves the same semantic identity in
both children. Reflection does not complement a coordinate. Two side tags of one semantic object
remain two separate authored rows,each with its own named physical copy. Distinct reflected copies
of one Piece and a touching same-copy self-seam retain their different material-domain meanings.

### Shared current boundaries do not prove a unique original position

The existing closed current interval rule also applies after noninjective topology mappings. For
example,split the original edge at1/2,reverse the second fragment and merge both with equal lengths.
Its original1/2 and original1 both reach the same current1/2,with Original and Reversed mappings.
With an original command cut at3/4,the first child's current coverage includes[0,1/2] and[3/4,1];
the second includes[1/2,3/4]. A real later anchor at current1/2 therefore receives Both even though
it differs from the requested cut's current3/4. Arbitrarily selecting one original would change
that result. The native case independently invokes the inverse API and retains both exact origins;
the membership row keeps the actual held/current anchor without selecting either.

This is current membership evidence. Original-notch ease queries separately require a unique
original context and refuse actual ambiguity. Membership success never overrides that requirement
or supplies physical correspondence. The actual split/reverse/merge case and these boundaries are
tracked by the membership leaf and the later paired acceptance owner.

### Stop-reference refusal remains distinct from context validation

Every referenced landmark must exist in the complete registry and belong to the actual named
copy's Piece. Its current point must lie in at least one paired interval on that side. A registry
entry can be individually valid on another Piece or elsewhere on the same Piece while the held
target's stop reference is invalid. Such references refuse with the original MissingLandmark or
StopOutsideSide SewingError,actual consuming target,authored ordinal and side/landmark pair.
The standard error source borrows that concrete original error. A row-bound refusal retains actual
retained/requested/limit values. No accepted prefix escapes,including when a later row fails after
an earlier valid stop. Upstream point choices,repairs and numeric causes remain current-context
refusals;this stage cannot replace them with membership success.

An N0 retained target yields an empty row list even when its full original ease root has two stops.
The original list is never backfilled. Five focused actual Design cases compare all32 frozen
independent membership rows with four shared-cut controls,both semantic kinds,authored Turn before
Notch order and later anchors. They also cover missing/wrong-Piece/outside-side refusals after a
valid prefix,reflected copies,touching self-seams,shared side tags and the two-origin case above.
Privacy/lifetime and public API doctests pass. Ordered borrowed child views follow below. The sealed
actual caller join follows the views below;component and whole paired/physical/authority/effect
verification retain their later owners.

## Obtain two ordered borrowed child views

Call `memberships.partition_stops()` over the externally owned complete membership result. The
private partition token borrows that exact owner and stores two lists of references to its original
rows. Iterate the authored rows once and append each to the First and/or Second list indicated by
its complete membership. Every push checks the original aggregate2N reservation before growth.
The bound counts references across both lists,including a shared row twice. It does not clone a
semantic object,stop definition,source root or projected range,and it performs no query/value read.

```rust
use sc_core::command::{CommandStopMemberships,CommandStopPartitions,CommandStopPartitionError};

fn children<'a,H>(memberships:&'a CommandStopMemberships<'a,H>)
    ->Result<CommandStopPartitions<'a,H>,CommandStopPartitionError>
{
    memberships.partition_stops()
}
```

Use `partitions.child(CommandStopChild::First)` or Second to obtain a borrowed view. Its `child()`
identifies the A-ordered child,`pair()` returns the exact original projected pair metadata,and
`rows()` lends the ordered references. Each still exposes its original stop,semantic object,copy,
held/current evidence and authored ordinal. Both views reference the exact same row at a shared
boundary. `memberships()` recovers the original complete owner;`reference_count()` gives actual
aggregate references. The partition and its child views cannot escape their actual borrowed owners.

For two authored marks both at a closed cut,N2 produces two references in each child,total4,
exactly the conservative2N cap. It creates neither additional semantic marks nor new span IDs.
An empty retained target produces two empty lists,total0,even when its ease root has two original
stops. A later checked-growth refusal retains actual target,row ordinal,stop,child,retained count,
requested1 and limit;neither partial child list escapes. Allocator capacity and whole command
snapshot/runtime policy stay separate from this admitted logical reference population.

The actual Design integration cases independently admit and capture the original source,then use
these borrowed stop views and its retained portions to construct real child graphs. All32 frozen
expected cases pass,including four shared-cut controls,both directions/semantic kinds and later
anchors. Both children retain one shared canonical ease root with its original stops and regions;
the actual candidate's current metadata preflight passes. Four reflected/touching copy/direction
scenarios preserve row pointers,two semantic side tags and authored order. A separate N0 case
preserves empty views. Three focused native cases and privacy/lifetime/API doctests cover the stage.

Those native callers intentionally use the same actual request/state for both pipelines. A sealed
production join checking the exact ease-capture and stop-view request/span owners is available
below. Same IDs or equal source definitions cannot replace that provenance requirement. These
views supply no new live child lineage,physical matching,fresh mutation authority,atomic effect,
undo/replay/cancellation or full paired verification. Those requirements retain their task owners.

## Bind the captured source to the actual stop views

Call `partitions.bind_source_capture(&capture)` before consuming captured ease with the child stops.
The stage compares the capture's original request pointer to the stop input token's request first,
then compares their actual target span pointers. Both must match. Equal request bodies,same target
IDs and equal graph definitions cannot stand in for the original owners. An equal cloned Design
has different actual graph members and therefore refuses. A distinct equal request refuses first,
including when its target owner also differs.

```rust
use sc_core::command::{CommandEaseCapture,CommandStopPartitions,CommandStopSourceBinding,
    CommandStopSourceBindingError};

fn join<'a,H>(capture:&'a CommandEaseCapture<'a>,partitions:&'a CommandStopPartitions<'a,H>)
    ->Result<CommandStopSourceBinding<'a,H>,CommandStopSourceBindingError<'a>>
{
    partitions.bind_source_capture(capture)
}
```

The private successful view borrows both externally owned tokens;there is no self-reference or
lifetime extension. It allocates,clones and queries nothing. `capture()` returns the exact admitted
capture,including original root,portion and selected capture limits. `partitions()` returns the
exact stop views and their complete membership/current/input/limit provenance. `child(First)` or
Second lends the existing actual child view,now bound to that same captured request and target.
Request and Target errors retain the actual expected and captured objects for inspection,including
when their IDs print identically. A fixed owner mismatch has no invented standard error cause.

Actual native callers join complete Design source admission and capture with the separately
admitted stop input/ownership/current/membership/view pipeline. Signed weighted cases on A and B
preserve the existing original cumulative allocation and signed conservation. Retained notch cases
separately admit and validate the full original notch context,then preserve its exact original
quantities while child stop lists remain empty. That root still retains both original stops;none
are backfilled into the actual N0 target. Formula and Profile cases retain their original metadata
without obtaining numeric values. These callers construct real retained child graphs with one
canonical original root and successful current metadata preflight;the original Design stays unchanged.

Four focused native cases cover those joins plus independent source-owner controls. A boxed equal
request and a cloned equal-body Design demonstrate actual Request and Target refusals,with both
original references retained and request-before-target priority. Privacy/lifetime/API doctests pass.
These are focused implementation checks;the stop component checkpoint follows completion,with
full paired/physical/authority/effect acceptance owned separately.

This token proves owner coherence. A metadata capture can be obtained through its own public
admission without complete Design source preflight,so this join does not add that preflight or
certify the full original root's currentness. Real callers must still use actual Design source
admission and the selected original quantity/context APIs as needed. It supplies no missing Formula
or Profile value,physical correspondence,aggregate runtime/snapshot budget,fresh mutation authority,
new live child lineage,atomic command effect or release approval. The SplitSeamSpan bus handler and
its combined undo/replay/cancellation/quota proof remain unimplemented and explicitly owned.
