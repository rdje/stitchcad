# Native Piece join contracts

This reference continues the [editing inputs](drafting-edit-inputs.md). Native declarations
retain actual source/proposal data;geometry,currentness and command publication remain separate.

## Native two-Piece join source intent

See the [source and policy contract](drafting-join-sources.md#native-two-piece-join-source-intent).

### Admit the complete original join inventory

See the [source and policy contract](drafting-join-sources.md#admit-the-complete-original-join-inventory).

### Require an explicit exact nondeforming splice

See the [source and policy contract](drafting-join-sources.md#require-an-explicit-exact-nondeforming-splice).

### Select joined metadata from actual A/B or authored fields

See the [source and policy contract](drafting-join-sources.md#select-joined-metadata-from-actual-ab-or-authored-fields).

### Declare one canonical result cut population

See the [source and policy contract](drafting-join-sources.md#declare-one-canonical-result-cut-population).

## Admit every original A/B source occurrence

`BoundJoinCutPopulation::admit_source_pieces` inspects the actual original A/B Pieces under
`JoinSourcePieceLimits::new(H,R)`. H counts every original hole group across both Pieces. R counts
every boundary,hole and construction occurrence plus each separately declared fold field,including
unrelated edges and repeated construction uses. Physical-copy quantity does not multiply this
pattern-source population;the complete CutPlan keeps its earlier separate admission.

```rust
use sc_core::construction::*;
fn admit<'a>(population:&'a BoundJoinCutPopulation<'a>)
    ->Result<BoundJoinSourcePieces<'a>,JoinSourcePieceError<'a>> {
    population.admit_source_pieces(JoinSourcePieceLimits::new(3,11))
}
```

Checked full H addition and `Holes` quota precede any child-hole length scan. Then a length-only
sum checks complete R and its `Occurrences` quota before any edge identity is inspected.
`HoleOverflow`/`OccurrenceOverflow` preserve nonrepresentable whole counts. All earlier owned
input,lookup,inventory,metadata and result-population obligations remain separate.

If A has2 boundary edges,2 one-edge holes,3 construction uses and1 separate fold,its R is8.
B with1 boundary edge,1 one-edge hole and1 construction use adds3:whole H3/R11. H2 refuses
required3 even with R0;H3/R10 refuses required11. H3/R11 proceeds to original field inspection.

Every edge prerequisite must be produced by an operation other than this same join operation.
`SelfOperation { side,location }` retains the first cause in A/B,then boundary/hole/construction/
fold order. It is checked only after full H/R quotas. Errors retain the whole original
`population()`,explicit `limits()` and first `refusal()`. `refused_source()` lends the actual
original refused EdgeRef field for SelfOperation,in constant work;count/quota causes return `None`.
No edge payload is substituted or accepted side prefix returned.

`BoundJoinSourcePieces` lends actual `population()`,original `limits()`,checked `hole_count()`/
`occurrence_count()` and original `piece(A/B)`. `occurrences()` lends all original fields in A/B,
then boundary,hole,construction,fold order,and reverse. Each authentic `JoinSourceOccurrence`
retains whole `context()`,original `side()`/`location()`,actual `piece()`,selected `copy()` and
original span `source_side()`. `field()` is the actual directed field or separate undirected fold;
its `edge()` borrows the exact original persistent reference. No traversal direction is fabricated.

Admission and full traversal take O(H+R+1) work and constant extra space,without allocation,
label/material text scanning,provider invocation or geometry evaluation. Field locations are
original indices,not persistent identities or current parameters. No sorting or deduplication occurs.
This source inventory proves no literal selected cut-bank membership,current-root/alias resolution,
entire namespace freshness,coincidence or valid joined topology. A retired source can still lend
its old held fields while actual current graph validation refuses;original references are not remapped.

## Bind both literal held cut banks

`BoundJoinSourcePieces::bind_cut_banks` binds A then B under the same immutable complete H/R
admission. A bank must be an actual boundary or hole field on that role's original supplied Piece,
with the same literal EdgeRef as its actual selected range. A construction match or separate fold
declaration cannot substitute. Canonical Piece birth already excludes duplicate cut-edge identities
across its cut loops;the native binding does not sort or choose among equal-looking fields.

```rust
use sc_core::construction::*;
fn bind<'a>(sources:&'a BoundJoinSourcePieces<'a>)
    ->Result<BoundJoinCutBanks<'a>,JoinCutBankError<'a>> {
    sources.bind_cut_banks()
}
```

`NoExactCutBank { side }` retains the first missing A/B role. The error lends actual `sources()`,
the first `refusal()` and the original refused `source_range()` field. A found A bank does not
escape a B refusal. This cause concerns the literal held variant;it does not prove an edge globally
absent or invalidate a history alias. Actual current projection/ownership remains separately required.

`BoundJoinCutBanks` retains actual complete `sources()`. `banks()` lends both authentic banks in
canonical A/B order and reverse,independent of CutPlan order;`bank(side)` queries one canonical role.
Each `JoinCutBank` retains whole `context()`,original `side()` and authentic source `occurrence()`.
`directed()` lends the actual original boundary/hole field and unchanged Piece traversal.
`range()` and `source_side()` lend the actual selected span fields;`piece()` and `copy()` lend the
original owning Piece and selected physical copy. No separately supplied field or range is attached.

For selected1/4–3/4,the directed contour still names its original whole edge,while `range()` keeps
those exact partial bounds. Neither expands the selected interval nor removes0–1/4 or3/4–1.
Outside-fragment attribution and complete proposed topology remain output requirements. A reflected
copy or fixed B placement never reverses these held bounds or chooses a new contour direction.

Binding takes O(H+R+1) work with two fixed cached associations,no allocation,string scan,provider
call or geometry evaluation. Literal membership proves no currentness,geometric coincidence,
interior parameter map,valid joined contour,retirement or authoring authority. Actual current graph
validation can refuse a retired source while these immutable original bank fields remain lendable.

## Bind the actual complete Edge role catalogue

`BoundJoinCutBanks::bind_edge_roles` binds a distinct admitted `AdmittedEdgeOutputRoleCatalog`
to the original join operation. Construct the catalogue through the shared
[Edge role contract](drafting-output-contracts.md);Point and semantic-Notch catalogues are different
types. Machine keys do not classify geometry or assign roles to a particular bank by their spelling.

```rust
use sc_core::construction::*;
fn bind<'a>(banks:&'a BoundJoinCutBanks<'a>,catalog:&'a AdmittedEdgeOutputRoleCatalog<'a>)
    ->Result<BoundJoinEdgeRoles<'a>,JoinEdgeRoleBindingError<'a>> {
    banks.bind_edge_roles(catalog,EdgeOutputRoleCatalogLimits::new(3,2,20))
}
```

The explicit association limits bound **all** definitions D,active occurrences A and original
machine-key bytes B,including inactive/unrelated definitions. Counts and B are cached by the actual
original catalogue admission. Complete D,A,B quotas precede operation equality;this binding does
not rescan keys,filter to requested outputs,sort declarations or allocate another index.

For definitions `bank_a`,`bank_b`,`inactive`,D3/A2/B20 is exact. D2 refuses `Definitions` before
A or key checks;A1 refuses `Active`;B19 refuses required20 as `KeyBytes`. With all quotas met,
a catalogue owned by1001 refuses `Operation` for actual join1000. These are association limits;
the earlier catalogue's owned storage/admission and complete source/context resources remain separate.

`BoundJoinEdgeRoles` lends actual `banks()`,actual whole `catalog()` and original `limits()`.
Catalogue membership views still lend their original definitions and active order;inactive
definitions remain present. Errors retain those same whole parents,policy and first `refusal()`.
Binding takes constant work/space without scans or allocation.

Empty or wholly inactive catalogues can bind to the same actual operation. This is metadata only:
both source banks and the explicit positive result population remain required. It proves no empty
joined result,no-op,source/output attribution,role history,global freshness,issued identity,
geometric correspondence,valid topology or publication authority.

## Declare a canonical result Piece identity

`BoundJoinEdgeRoles::declare_piece_identity` borrows one explicit canonical result Piece-id field.
It neither allocates nor derives that id from geometry,labels,source ids or statement positions.
This is native declaration data;creation requests still rely on the trusted host to issue identities.

```rust
use sc_core::{construction::*,ontology::EntityId};
fn declare<'a>(roles:&'a BoundJoinEdgeRoles<'a>,piece_id:&'a EntityId)
    ->Result<DeclaredJoinPieceIdentity<'a>,JoinPieceIdentityError<'a>> {
    roles.declare_piece_identity(piece_id,JoinPieceIdentityLimits::new(1))
}
```

`JoinPieceIdentityLimits::new(S)` bounds every selected-span stop occurrence **plus** every
original stop occurrence retained by a Portion ease source. Checked total and `Stops` quota
precede candidate and stop-field inspection;`StopOverflow` retains a nonrepresentable sum.
The original complete provided C/I inventory/plan and A/B H/R policies remain in force.

With one selected stop and that same landmark retained in the original ease snapshot,S is2,
not1. S1 refuses required2 even if the requested id also aliases the join operation. A retained
tail can have no selected stops while its original source still contributes1;that occurrence
cannot disappear merely because it lies outside the current selected portion.

After resource admission,the listed local exclusions run in this order:actual join operation,
every supplied inventory Piece,requested self-companion,original graph,selected span,retained
ease-source span,every original physical copy,A/B edge-producing operations,input assigned
Materials,selected result Material,selected stops,then retained-source stops. Inventory and plan
indices retain original supplied order;source edges retain original A/B/family order.

`Operation`,`InventoryPiece`,`SelfCompanion`,`Graph`,`Span`,`RetainedSpan`,`Copy`,
`SourceOperation`,`SourceMaterial`,`ResultMaterial` and `Stop` preserve the actual first cause.
`JoinIdentityStopOrigin` distinguishes selected and retained lists. Repeated stop references count
in S even though the selected list wins when both lists contain the refused landmark.

Errors lend actual `roles()`,original `piece_id()`,explicit `limits()` and first `refusal()`.
`refused_source()` lends the actual original edge only for SourceOperation;`refused_stop()` lends
the actual original stop only for Stop. Other causes return `None`;no replacement field is attached.
`DeclaredJoinPieceIdentity` retains the same actual roles,id field and limits and reports the
complete `stop_occurrences()` count. It grants no companion existence/reciprocity or publication proof.

Work is O(C+I+H+R+S+1),with constant extra space,no allocation,text scan,registry/provider call
or identity issuance. Formula/Profile source ids are not classified from equal external values.
An unlisted id can pass these local checks,including explicit zero or a value equal to an unread
external source reference;that supplies no global kind,existence,state or freshness certificate.
Actual current/historical namespace,nonreuse,reference resolution and authoring authority remain
mandatory host/program obligations before geometry execution or publication.

## Attribute explicit Edge candidates to original source fields

`JoinEdgeCandidate::new(tag,side,location,range)` retains four explicit fields:the persistent Edge
role tag,the original A/B role,the exact boundary/hole/construction/fold location,and a positive
ascending held `EdgeRange`. The location identifies an original field in this supplied snapshot;
it is neither an output identity nor a claim about current topology. No location,range or traversal
is inferred by sorting ids,copy order,coordinate coincidence or the seam's Opposite direction.

```rust
use sc_core::{construction::*,ontology::*};
fn candidate(tag:LocalTag,side:SeamSideId,source:EdgeRange)->JoinEdgeCandidate {
    JoinEdgeCandidate::new(tag,side,JoinSourceOccurrenceLocation::Boundary(0),source)
}
fn attribute<'a>(manifest:&'a JoinEdgeManifest,identity:&'a DeclaredJoinPieceIdentity<'a>)
    ->Result<AdmittedJoinEdges<'a>,JoinEdgeManifestError<'a>> {
    manifest.admit(identity,JoinEdgeManifestLimits::new(3,6))
}
```

`JoinEdgeManifest::new(operation,candidates)` retains the original complete order and all invalid
unadmitted fields. `admit` checks the complete N and checked2N index/association entries before
inspecting any candidate or allocating those structures. These entry limits do not bound caller
storage,allocator bytes or earlier C/I/H/R/S/catalogue admission;those policies remain required.
For N3,W6 is exact:N2 refuses `Candidates`,W5 refuses `Workspace`,and both precede a wrong operation
or bad tag/location. `WorkspaceOverflow` preserves nonrepresentable complete arithmetic.

After quota admission,the operation must match the actual join. Each candidate then checks its
original tag for `DuplicateTag`, `UndefinedRole`, `InactiveRole`,then its actual source field for
`MissingOccurrence`,then literal range-edge equality for `SourceEdge`. A later bad candidate
returns the whole original proposal and first cause;no accepted prefix is returned. Repeated
source edges/ranges with distinct active output roles remain separate declarations.

`BoundJoinSourcePieces::occurrence(side,location)` performs constant bounded field lookup under
its existing immutable H/R admission. Boundary,hole and construction locations lend actual
`DirectedEdge` fields with their original direction;Fold lends the actual undirected `EdgeRef`.
Absent groups/indices return `None`. Candidate admission uses this same actual source context;
requesting B's range at A's field cannot silently reparent or substitute the source.

`JoinEdgeManifestError` lends actual `manifest()`,actual `identity()`,the explicit `limits()` and
first `refusal()`. `refused_candidate()` lends the original refused field for candidate causes,
including the second field for duplicate tags;whole-resource/operation causes return `None`.
`refused_occurrence()` lends the actual original mismatched field only for `SourceEdge`;other
causes return `None`. It performs no replacement,normalization or source-history projection.

`AdmittedJoinEdges` retains those whole parents and reports `edge_count()` and
`workspace_entries()`. `edges()` iterates original candidate order and reverse;`edge(tag)` uses
only the explicit persistent tag. A `JoinOutputEdge` lends its authentic owner,original index,
actual candidate,actual admitted role,actual occurrence and original range. Its `logical_output()`
is the declared tuple `(actual join operation,explicit tag)`,without issuing or publishing an edge.
Unused active roles remain in the catalogue and are not implicitly retired.

Work is O(N log(N+D+2)) for D admitted definitions,with2N index/association entries. There is no
key scan,source-contour scan,registry/provider call or geometric evaluation at this stage.
An explicit empty proposal can admit N0/W0;both banks and the positive result population still
remain on its original parents. This supplies no empty joined Piece or no-op certificate.

A candidate range can still include a selected bank that the exact splice intends to remove.
This stage proves literal source attribution only. Complete surviving outside-bank coverage,
consumed-bank exclusion,output/source parameter correspondence,whole topology,fold/consumer
ownership,retirement,currentness,global reference truth and atomic effects remain required later.

## Require complete surviving cut-source coverage

Call `AdmittedJoinEdges::require_cut_coverage` to require the exact original boundary/hole domains
that survive removal of both selected banks. This is a native requirement over held source ranges;
it performs no geometric join and constructs no result Piece.

```rust
use sc_core::construction::*;
fn cover<'a>(edges:&'a AdmittedJoinEdges<'a>)
    ->Result<CoveredJoinCutSources<'a>,JoinCutCoverageError<'a>> {
    edges.require_cut_coverage(JoinCutCoverageLimits::new(4,2,4))
}
```

`JoinCutCoverageLimits::new(N,R,W)` admits every original candidate N,every original A+B occurrence
R,and a conservative N-entry sorted index **before** field scans or allocation. R includes repeated
construction and separate fold fields;W counts all N before filtering to cut candidates. Earlier
complete C/I/H/R/S/catalogue/candidate policies and caller-owned storage remain separate obligations.
These entries are not allocator bytes. N4/R2/W4 is exact for four outside candidates on two banks.
N3 refuses `Candidates`;R1 refuses `SourceOccurrences`;W3 refuses `Workspace`,before bank fields.

After quotas,cut candidates with positive intersection with their selected bank refuse
`ConsumedBank` in original candidate order. Endpoint-only contact remains legal. For bank `[1/4,3/4]`,
`[0,1/4]` and `[3/4,1]` survive;`[1/8,1/2]` cannot be declared as a surviving cut fragment.
The same rules apply to an actual bank on a hole. No boundary/hole field is substituted by an
otherwise equal construction or fold field.

Remaining cut candidates are indexed by their original A/B/family/location,then ascending source
from/to and original candidate index. These lookup positions are transient source locators;
persistent output identities still use the actual operation and explicit tag.

Every original cut field is then checked in A/B,boundary/hole order. Unrelated cut edges require
exact `[0,1]` partitions. A selected bank requires exact prefix `[0,from]` then suffix `[to,1]`
partitions,omitting zero-width portions. Missing positive intervals refuse `Gap` with actual side,
location and exact missing bounds. A candidate starting before the already covered cursor refuses
`Overlap` with its original index and covered upper bound. No gap or overlap is repaired or skipped.

For a retained tail with A bank `[1/2,1]` and B bank `[0,1/2]`,A's required cut source is `[0,1/2]`
and B's is `[1/2,1]`. Removing A's candidate refuses its first gap even when B is complete. Two A
candidates `[0,1/2]` and `[1/4,1/2]` overlap;the extra persistent role does not authorize duplicate
cut content. All unrelated original hole/boundary intervals remain required too.

`JoinCutCoverageError` retains actual `edges()`,the explicit `limits()` and first `refusal()`.
`refused_candidate()` lends the actual original field only for ConsumedBank/Overlap. Domain causes
lend their actual `source_occurrence()`;whole quota causes return `None`. A later failure returns
no accepted coverage prefix. Complete original declarations and policies remain available.

`CoveredJoinCutSources` retains those actual parents. `cut_occurrence_count()` includes every
original cut field,including wholly consumed banks;`cut_edge_count()` counts actual indexed cut
candidates. `workspace_entries()` reports conservative whole N admission. `cut_edges()` iterates
source-field/ascending-range order and reverse;`outputs(side,location)` lends that original cut
field's authentic candidates. Missing/noncut locations give an empty iterator. Original authored
candidate order remains on `edges()`,with every construction/fold declaration still present.

Work is O(N log(N+2)+(H+R)log(N+2)),with N sorted-index entries and no registry/provider/geometry
calls. Construction/fold fields are not cut partitions and cannot substitute for missing cut fields;
their semantic reconciliation remains explicit later work.

If both banks occupy their entire original edges,neither has an outside interval. An empty candidate
manifest can satisfy that cut-domain bookkeeping while retaining two original cut occurrences and a
positive result population. It proves no nonempty,closed,simple or otherwise valid result Piece.
Whole output topology,traversal,parameter correspondence,fold/marks/consumer ownership,retirement,
currentness,geometric coincidence and atomic publication remain required before actual execution.

## Declare every output's parameter direction

`JoinParameterDirection` requires `SameAsSource` or `ReverseSource` explicitly. This request names
whether the output's start/end correspond to the attributed ascending source range's from/to or
to/from. It is separate from original Piece traversal,physical-copy reflection,sewing direction
and moving-frame orientation. No direction default or numeric interior map is supplied.

```rust
use sc_core::{construction::*,ontology::*};
fn assignment(tag:LocalTag)->JoinDirectionAssignment {
    JoinDirectionAssignment::new(tag,JoinParameterDirection::ReverseSource)
}
fn direct<'a>(manifest:&'a JoinDirectionManifest,coverage:&'a CoveredJoinCutSources<'a>)
    ->Result<AdmittedJoinDirections<'a>,JoinDirectionError<'a>> {
    manifest.admit(coverage,JoinDirectionLimits::new(2,2,4))
}
```

`JoinDirectionManifest::new(operation,assignments)` preserves original operation,order,tags and
orientation fields. `JoinDirectionLimits::new(N,M,W)` admits the complete actual candidate output
population N,all raw assignments M and checked2M index/association entries before operation,count,
field scans or allocation. `WorkspaceOverflow` preserves a nonrepresentable complete2M count.
Earlier candidate/coverage/C/I/H/R/S/catalogue resources and caller-owned storage remain separate;
logical entries do not measure allocator bytes. Work is O(M log(M+N+2)),with2M new logical entries.

With actual outputs9 and2,N2/M2/W4 is exact. N1 refuses `Outputs`;M1 refuses `Assignments`;W3
refuses `Workspace`,before a wrong manifest operation or bad tag. With quotas met,`Operation`
precedes `Count`;one or three assignments refuse expected2 before their fields are inspected.
Equal count alone cannot establish completeness:original assignments then refuse `Duplicate`
or `UnknownOutput`. Unique membership in the actual N candidates plus M=N supplies one per output.
Tag13 remains unknown here when active in the catalogue but absent from the actual candidate
manifest;inactive7 and undefined99 cannot substitute either. A failure returns no accepted prefix.

N includes every construction/fold candidate on the original manifest,not just the cut-coverage
index. For four cut candidates and six noncut candidates,N is10;four assignments refuse expected10.
All ten can be explicitly directed even when a source field is undirected Fold or has reversed
Piece traversal. The original source field and requested parameter direction remain independent.

`JoinDirectionError` retains actual `manifest()`,actual `coverage()`,explicit `limits()` and first
`refusal()`. `refused_assignment()` lends the actual original field only for Duplicate/UnknownOutput;
the second original duplicate wins for Duplicate. Resource/operation/count causes return `None`.
`AdmittedJoinDirections` retains the same whole parents and reports `workspace_entries()`.
`assignments()` lends original authored order and reverse;`assignment(tag)` uses explicit tag lookup.

Each authentic `JoinDirectionalOutput` lends its whole `context()`,original `index()` and actual
`assignment()`. `source()` is the actual candidate/role/occurrence attribution from that complete
parent;`parameter_direction()` is the original explicit request. Ascending source bounds and
persistent output tuples remain unchanged. No contour traversal is silently flipped or inferred.

Explicit N0/M0/W0 metadata can admit when whole banks leave no outside cut interval. It still
proves no nonempty joined Piece,no-op,achieved endpoint/interior correspondence,currentness,
valid whole contour,geometric splice,semantic reconciliation,retirement or publication authority.

## Declare complete result cut uses

`JoinCutContourManifest` retains an explicit operation,ordered proposed outer-boundary uses and
independently ordered proposed hole-use lists. `JoinCutContourUse::new(tag,direction)` requires a
persistent output tag and explicit Piece traversal in that output's own parameter frame. It does
not accept `JoinParameterDirection`:source-relative parameter orientation and result traversal
remain separate authored fields. No list position becomes an output identity.

```rust
use sc_core::{construction::*,ontology::*};
fn cut_use(tag:LocalTag)->JoinCutContourUse {
    JoinCutContourUse::new(tag,Direction::Reversed)
}
fn place<'a>(manifest:&'a JoinCutContourManifest,directions:&'a AdmittedJoinDirections<'a>)
    ->Result<DeclaredJoinCutContours<'a>,JoinCutContourError<'a>> {
    manifest.admit(directions,JoinCutContourLimits::new(10,2,4,10))
}
```

`JoinCutContourLimits::new(N,H,U,W)` admits the complete actual output population N,including
noncut candidates,and all proposed hole headers H before reading their child lengths. Checked
complete boundary-plus-hole use count U and checked2U+H workspace then precede per-use fields or
allocation. Earlier complete parent/candidate/direction/coverage policies and caller-owned input
storage remain separate. Workspace counts U tag-index entries,U use associations and H hole
associations;it does not measure allocator bytes. Arithmetic failures retain `UseCountOverflow`
or `WorkspaceOverflow` rather than truncating a prefix.

For N10,H2,U4,the required workspace is10. N9 refuses `Outputs`;H1 refuses `Holes`;U3 refuses
`Uses`;W9 refuses `Workspace`,before a wrong operation or bad tag. With quotas met,`Operation`
precedes `EmptyBoundary`,then the first `EmptyHole`,then exact cut-use `Count`. A whole-bank
splice with no surviving cut candidates can satisfy empty cut coverage/directions,but its empty
proposed boundary refuses `EmptyBoundary`;empty metadata cannot create a declared result boundary.

Let C be the actual cut-candidate population. U must equal C,not the complete N when construction
and fold candidates are present. Each original boundary use,then each original hole group/use,
checks `DuplicateTag`, `UnknownOutput`,then `NonCutOutput`. A unique member of the complete
actual cut set is required in every slot. Unique cut membership plus U=C supplies exactly one
use per cut candidate;equal count alone cannot hide repeated roles or missing cut content.
Construction/fold-attributed candidates cannot become cut uses merely because their source edge
also occurs on a cut contour. They remain on the original complete direction/manifest parents
for their separate explicit reconciliation.

For cut outputs9,8,2,10,a proposal can retain boundary order9,8 and hole order[10],[2],with each
traversal explicit. Its four use fields and two group headers remain the actual authored objects;
no sorting of holes or source-driven traversal reversal is performed. Replacing the last use2
with9 refuses the duplicate's original boundary/hole locations. Unknown99 refuses `UnknownOutput`;
an actual construction/fold output refuses `NonCutOutput`. No accepted cut-use prefix escapes.

`JoinCutContourError` lends actual `manifest()`,actual `directions()`,original `limits()` and first
`refusal()`. `refused_use()` lends the original second duplicate,unknown or noncut field;resource,
operation,empty/count causes return `None`. Whole raw lists and complete source parents survive.

`DeclaredJoinCutContours` retains those same parents and reports `use_count()`,`hole_count()` and
`workspace_entries()`. `uses()` follows original boundary then hole-group/field order and reverse;
`boundary()` and `holes()` preserve the original grouping. `use_for(tag)` queries the unique
explicit cut role. Each authentic `JoinPlacedCutUse` lends its whole context,original location,
actual declared field,actual traversal field and authentic directional output/source attribution.
Its `logical_output()` retains the actual operation/persistent-tag tuple without issuing an edge.
Each `JoinCutHole` lends its actual original group,index,whole context and ordered use views.

Work is O(H+U log(U+N+2)),with2U+H additional entries,no source scan,registry/provider call or
geometric evaluation. This stage admits complete cut placement declarations only. Source-path
adjacency,bank-endpoint gluing and loop classification remain separately owned connectivity work;
construction/fold/consumer semantics,copy reconciliation,retirement and publication also remain.
The [canonical Piece invariants](../spec/ontology.md#41-piece) still require actual closed/simple
CCW outer geometry and opposite-wound,strictly internal,mutually disjoint holes. Authored list
membership/traversal supplies none of those geometric certificates. Actual G2/G3 validation and
current source/history/authority checks remain required before accepting a canonical joined Piece.
