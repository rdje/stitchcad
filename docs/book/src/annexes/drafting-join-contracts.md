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

See the [result topology contract](drafting-join-topology.md#declare-complete-result-cut-uses).

## Require original source-path and bank gluing

See the [result topology contract](drafting-join-topology.md#require-original-source-path-and-bank-gluing).

## Declare proposed result construction-line uses

See the [result topology contract](drafting-join-topology.md#declare-proposed-result-construction-line-uses).

## Request explicit result fold attribution

After whole [construction source/target reconciliation](drafting-join-topology.md#reconcile-every-proposed-construction-occurrence),
`RequiredJoinConstructionTargets::require_fold` binds a `JoinFoldRequirement` containing the
operation,explicit `JoinFoldIntent` and required geometric correspondence evidence source.
`NoFold` requests a result without a fold;`CutOnFold { tag }` requests exactly one actual outer
cut output as its fold. Neither intent nor requirement has a default. Even NoFold retains an
explicit correspondence request. An evidence id,including0,is no accepted evidence or authority.

```rust
use sc_core::{construction::*,ontology::*};
let request=JoinFoldRequirement::new(EntityId::from_bits(1000),
    JoinFoldIntent::CutOnFold{tag:LocalTag::new(9)},EntityId::from_bits(7001));
fn fold<'a>(construction:&'a RequiredJoinConstructionTargets<'a>,request:&'a JoinFoldRequirement)
    ->Result<RequiredJoinFold<'a>,JoinFoldError<'a>> {
    construction.require_fold(request,JoinFoldLimits::new(10,10,4))
}
```

`JoinFoldLimits::new(R,N,C)` admits every original A/B source field R,every actual cut/noncut
candidate N and every actual cut use C,including unrelated holes,before operation or fold/tag fields.
R includes the original separate fold declarations;earlier complete source/construction/mapping
resource policies remain. For R10/N10/C4,R9 refuses `Sources`,N9 `Outputs`,C3 `CutUses`,before
an incorrect operation or unknown fold tag. With quotas met,`Operation` precedes tag checks.

A CutOnFold tag must name an actual directional candidate (`Output` otherwise),then an actual
outer cut use (`OutputNotBoundary` otherwise). A hole,construction-only use,fold-field-attributed
noncut candidate or merely catalogued role cannot substitute. Neither geometry equality nor a
candidate list position selects a fold. NoFold does not inspect or silently discard source folds.

`JoinFoldError` retains actual whole `construction()`,raw `requirement()`,original `limits()` and
first `refusal()`. Output-specific causes lend the actual original `refused_tag()` field;whole
resource/operation causes return None. No replacement source,fold tag or accepted prefix is returned.

`RequiredJoinFold` retains those whole parents and lends the actual selected outer `boundary()`
use,or None for NoFold. `cut_on_fold()` derives the desired result flag from that explicit request;
there is no second independently entered boolean. `edge()` reports the desired actual logical
output tuple without issuing it. Printed cutting/fold indicators and the final canonical Piece
must derive from complete copy/contour/fold reconciliation later.

This stage binds desired result intent only. It can retain NoFold beside two original A/B fold
fields,or CutOnFold beside sources without folds;neither grants permission to change their cutting
state. The [original fold retention/omission requests below](#reconcile-every-original-fold-declaration)
must also be reconciled,and real
correspondence/source/consumer justification is required before effects. A source subrange on an
outer output does not establish a whole physical fold axis or original-state compatibility.

Work is O(log(N+2)+log(C+2)+1),with constant additional state,no allocation,provider,evidence,
registry or geometry call. Complete original fold actions,physical copy/mark/consumer requirements,
currentness,achieved maps,geometry,retirement and atomic authority remain required separately.

## Reconcile every original fold declaration

`JoinFoldSourceManifest` retains the operation and complete ordered
`JoinFoldSourceAssignment::new(side,index,expected,intent)` rows. Each selector names an actual
original A/B fold declaration;`expected` guards its persistent EdgeRef,not a point or a replacement
source. The ordinal is an original snapshot selector,not a geometry identity. Each row explicitly
requests `JoinFoldSourceIntent::Retain` or `Omit { evidence_source }`;there is no default.

```rust
use sc_core::{construction::*,ontology::*};
fn action(side:SeamSideId,index:usize,edge:EdgeRef,intent:JoinFoldSourceIntent)->JoinFoldSourceAssignment {
    JoinFoldSourceAssignment::new(side,index,edge,intent)
}
fn sources<'a>(manifest:&'a JoinFoldSourceManifest,fold:&'a RequiredJoinFold<'a>)
    ->Result<DeclaredJoinFoldSources<'a>,JoinFoldSourceError<'a>> {
    manifest.admit(fold,JoinFoldSourceLimits::new(10,2,4))
}
```

`JoinFoldSourceLimits::new(R,M,W)` admits all original A/B source fields R before reading fold
counts,and all raw action headers M before fields or allocation. Complete original A+B fold count
and2M workspace arithmetic are checked;workspace counts source-index entries M and associations M,
not bytes or earlier immutable source/result storage. For R10/M2,W4 is exact:R9 refuses `Sources`,
M1 `Assignments`,W3 `Workspace`,before an incorrect operation or malformed original selector.

With resources met,`Operation` precedes `Count`:M must equal the complete original fold population.
Each original row then checks `DuplicateSource`, `MissingSource`, `SourceField`,followed by its
intent's requirements. Unique actual original membership plus exact count covers every fold field,
even if the original A/B Pieces contain byte-equal geometry. The expected-field guard cannot replace
the actual original field loan. An original no-fold pair requires an explicit empty action list.

Retain first requires an actual requested result fold (`ResultNoFold` otherwise),then exact
original A/B attribution (`SourceSide`),then the exact original persistent fold edge (`SourceEdge`).
An actual outer bank edge cannot replace another original fold edge on that same Piece. Equal old
EdgeRefs across A/B Pieces cannot merge their source frames. A partial range attributed to an old
fold can pass structural attribution;the named correspondence request must still justify whole
physical fold/state/copy compatibility. Retain does not provide an achieved axis or geometric map.

Omit retains an explicit independent evidence request for that original declaration. Zero or
shared evidence ids remain authored fields;their scopes never merge and no evidence kind,state,
currentness,source/consumer justification or deletion permission is inferred. NoFold with existing
folds therefore needs complete Omit rows. A requested new result fold beside original no-fold
sources retains the parent correspondence obligation;empty source coverage cannot approve that axis.

`JoinFoldSourceError` retains actual complete `manifest()`,`fold()`,original `limits()` and first
`refusal()`. Row causes lend the original `refused_assignment()` and actual `refused_source()`
occurrence if that selector exists. Whole resource/operation/count causes lend no row/source fields.
No accepted prefix,source substitute or inferred fold choice escapes refusal.

`DeclaredJoinFoldSources` retains those whole parents and reports `source_count()` and
`workspace_entries()`. `assignments()` preserves original order and reverse;`assignment(side,index)`
queries an actual guarded original fold scope. Each `JoinFoldSourceAction` lends its actual owner,
original row ordinal/raw assignment,actual original fold occurrence/Piece/copy and persistent
`source_field()`. `intent()`/`evidence_source()` lend the original authored request fields.

For example,select an actual A fold output,Retain the guarded A declaration,and explicitly Omit
the guarded B declaration with its own evidence request. Alternatively,request NoFold and supply
complete explicit Omit rows. These are native declarations of required reconciliation;neither
accepts original state changes,evidence truth,physical copy effects or consumer repair.

Work is O(M log(M+2)+1),with2M new entries and no provider,evidence,registry or geometry call.
This completes native original/result fold bookkeeping. Canonical cut/print binding,physical copy
correspondence,marks and other consumers,currentness,actual geometry,retirement and atomic
authority remain separate requirements. Final printed fold state must derive from that complete
canonical reconciliation,not a second authored flag.

## Names used in this chapter

Every API name and example name this chapter uses in code style, with its meaning. API meanings come from the code's own documentation; example names are not defaults.

| Name | Meaning |
| --- | --- |
| `Active` | Refusal cause: the complete active-occurrence population exceeds the selected policy. |
| `admit` | Admission method: checks complete populations against explicit limits before reading fields, then returns the admitted view or the first refusal. |
| `AdmittedEdgeOutputRoleCatalog` | Type: Complete structural catalogue admission,not issuance history,global namespace or geometric truth. |
| `AdmittedJoinDirections` | Type: Authentic complete candidate orientation requests,not achieved maps or contour placement. |
| `AdmittedJoinEdges` | Type: Whole local Edge-role/source attribution,not coverage,executed geometry or an issued Piece. |
| `Assignments` | Refusal cause: the whole raw assignment population exceeds the selected policy before any row is read. |
| `bank_a` | Example catalogue definition name for join bank A. |
| `bank_b` | Example catalogue definition name for join bank B. |
| `BoundJoinCutBanks` | Type: Authentic complete A/B held bank association,not a geometric or current join certificate. |
| `BoundJoinEdgeRoles` | Type: Authentic complete actual operation/Edge catalogue association,not an output or issued identity. |
| `BoundJoinSourcePieces` | Type: Authentic complete bounded original sources,not a currentness/topology/namespace certificate. |
| `ConsumedBank` | Variant of JoinCutCoverageRefusal: A cut candidate has positive intersection with this side's selected bank domain. |
| `Copy` | Variant of JoinPieceIdentityRefusal: An original physical copy already owns this identity. |
| `CoveredJoinCutSources` | Type: Complete original surviving cut partitions,not a complete joined Piece or currentness proof. |
| `CutUses` | Variant of JoinFoldRefusal: Complete actual cut uses exceed policy. |
| `DeclaredJoinFoldSources` | Type: Complete native original fold actions;omission/correspondence/state/copy/consumer truth outstanding. |
| `DeclaredJoinPieceIdentity` | Type: Authentic local canonical Piece-id declaration,not an issued/fresh/current result or authority. |
| `Definitions` | Refusal cause: the whole definition population, including inactive and unrelated roles, exceeds the selected policy. |
| `Duplicate` | Refusal cause: an original selector, tag or requested id repeats in original request order. |
| `DuplicateSource` | Refusal cause: one original source selector repeats across the raw rows or groups. |
| `DuplicateTag` | Refusal cause: an explicit persistent tag repeats in original declaration order. |
| `expected` | Constructor argument guarding the original persistent field; it is not a replacement source. |
| `Gap` | Outcome or refusal for an uncovered positive interval: no candidate or declared domain contains this source position. |
| `HoleOverflow` | Refusal cause: the complete hole count cannot be represented. |
| `Holes` | Refusal cause: the complete hole population exceeds the selected policy before any child-hole scan. |
| `inactive` | Example catalogue definition name for a role that is defined but not active. |
| `InactiveRole` | Refusal cause: the candidate names an issued role that is canonically inactive. |
| `InventoryPiece` | Variant of JoinPieceIdentityRefusal: A supplied original inventory Piece already owns this identity. |
| `JoinCutBank` | Type: One authentic cut bank with its whole A/B context and actual original directed field/range. |
| `JoinCutCoverageError` | Type: Whole actual attributed manifest,policy and first original failure;no accepted prefix escapes. |
| `JoinDirectionalOutput` | Type: Authentic actual original direction request and source attribution,not a geometric map. |
| `JoinDirectionError` | Type: Actual complete raw manifest,cut-coverage/source parents,policy and first cause. |
| `JoinEdgeManifestError` | Type: Whole original manifest,identity/context,policy and first cause survive every refusal. |
| `JoinFoldError` | Type: Whole original construction/source context,raw request,policy and first refusal remain borrowed. |
| `JoinFoldIntent` | Enum: Explicit desired result cutting state;no inferred axis,source preservation or default. |
| `JoinFoldRequirement` | Type: Explicit operation,result fold choice and required geometric correspondence evidence source. |
| `JoinFoldSourceAction` | Type: One authentic original action,actual fold occurrence/Piece/copy and unchanged evidence field. |
| `JoinFoldSourceError` | Type: Whole original manifest,result/source parents,policy and first cause remain borrowed. |
| `JoinFoldSourceManifest` | Type: Explicit operation and complete ordered guarded original fold actions. |
| `JoinIdentityStopOrigin` | Enum: Actual original stop list containing a refused known landmark reference. |
| `JoinOutputEdge` | Type: Actual whole-owner/candidate/role/original-occurrence association;logical output is not issuance. |
| `JoinParameterDirection` | Enum: Requested output start/end orientation relative to the attributed ascending source range. This is distinct from Piece traversal,physical-copy reflection and sewing direction. |
| `JoinSourceOccurrence` | Type: One authentic original source field with actual Piece/copy/held-range context. |
| `KeyBytes` | Refusal cause: the complete machine-key bytes exceed the selected policy. |
| `MissingOccurrence` | Refusal cause: this original A/B Piece has no field at the requested family and location. |
| `OccurrenceOverflow` | Refusal cause: the complete occurrence count cannot be represented. |
| `Occurrences` | Refusal cause: the complete occurrence population exceeds the selected policy before any edge field is read. |
| `Operation` | Refusal cause: the raw manifest or catalogue is owned by another operation than the one selected. |
| `Output` | Refusal cause: the tag or use names no actual candidate output on the required contour. |
| `OutputNotBoundary` | Refusal cause: the actual output has no use on the proposed outer boundary. |
| `Outputs` | Refusal cause: the whole actual output-candidate population exceeds the selected policy. |
| `RequiredJoinFold` | Type: Authentic explicit desired result fold/outer attribution;source actions and geometric proof pending. |
| `ResultMaterial` | Identity refusal: the selected result metadata already requires this identity as its Material. |
| `ResultNoFold` | Variant of JoinFoldSourceRefusal: Retain requires an actual explicitly requested result fold,not NoFold. |
| `RetainedSpan` | Variant of JoinPieceIdentityRefusal: The retained original ease-source span already owns this identity. |
| `ReverseSource` | Variant of `JoinParameterDirection`: Output start/end correspond to source to/from;no interior or affine map is certified. |
| `SameAsSource` | Variant of `JoinParameterDirection`: Output start/end correspond to source from/to;actual correspondence still needs evidence. |
| `SelfCompanion` | Identity refusal: a requested separate result pair member cannot be its own companion. |
| `SourceEdge` | Refusal cause: the candidate or range belongs to another original persistent source edge. |
| `SourceField` | Refusal cause: the expected whole original field differs from the actual original field. |
| `SourceMaterial` | Identity refusal: a supplied inventory Piece already requires this identity as its assigned Material. |
| `SourceOccurrences` | Refusal cause: the whole original occurrence population, including noncut fields, exceeds the selected policy. |
| `SourceOperation` | Identity refusal: an actual original A/B edge-producing operation already owns this identity. |
| `Sources` | Refusal cause: the whole original source population exceeds the selected policy. |
| `SourceSide` | Refusal cause: the candidate belongs to another original A/B source role, even for equal edge references. |
| `Span` | Identity refusal: the actual selected span already owns this identity. |
| `Stop` | Variant of JoinPieceIdentityRefusal: An actual selected/retained stop requires this identity as its landmark. |
| `StopOverflow` | Variant of JoinPieceIdentityRefusal: Checked complete stop sum cannot be represented. |
| `Stops` | Refusal cause: the complete original stop population exceeds the selected policy. |
| `UndefinedRole` | Refusal cause: the candidate has no definition in the actual complete catalogue. |
| `UnknownOutput` | Refusal cause: the tag names no output in the actual complete candidate context. |
| `WorkspaceOverflow` | Refusal cause: the checked complete workspace bound cannot be represented. |
