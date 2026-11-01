# Native join result topology

This companion continues [source attribution and coverage](drafting-join-contracts.md).
Its native declarations retain actual inputs;geometry and authorized effects remain separate.

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

## Require original source-path and bank gluing

`DeclaredJoinCutContours::require_cut_connectivity` checks each declared loop's requested source
path against the actual original cut loops and the exact splice endpoint pairs. It yields
`RequiredJoinCutConnectivity`,not a geometric correspondence or canonical joined Piece.

```rust
use sc_core::construction::*;
fn require_path<'a>(contours:&'a DeclaredJoinCutContours<'a>)
    ->Result<RequiredJoinCutConnectivity<'a>,JoinCutConnectivityError<'a>> {
    contours.require_cut_connectivity(JoinCutConnectivityLimits::new(2,4))
}
```

`JoinCutConnectivityLimits::new(H,U)` admits every proposed result hole group and every cut use
before reading source ports or scanning loops. H1 refuses actual2 as `Holes`;U3 refuses actual4
as `Uses`,even when the first requested adjacency is wrong. All earlier immutable input,original
source,candidate,direction,coverage and cut-use resource policies remain required.

Entry/exit source parameters compose the explicit output traversal with its source-relative
parameter-direction request. SameAsSource+Original enters at source from and exits at to;
ReverseSource+Original enters at to and exits at from. Reversing the authored traversal swaps
those endpoints again. No output or source bounds are rewritten and no interior affine map follows.

Source ports retain original A/B role,original boundary/hole loop and exact field/parameter.
Within an original directed cyclic cut loop,0/1 endpoints normalize to the appropriate original
vertex;the last field wraps to the first. Original reversed fields swap their own entry/exit.
Interior fragment endpoints remain literal field/Param ports. Equal EdgeRefs on distinct A/B
Pieces or unrelated loops cannot substitute for these actual source identities.

`RequiredJoinExactSplice::endpoint_pair(AFrom/ATo)` lends the authentic pair for one explicitly
named A-held endpoint. As with `endpoint_pairs()`,Same matches Afrom/Bfrom and Ato/Bto;Opposite
matches Afrom/Bto and Ato/Bfrom. Those two required links,including transitivity when endpoints
share an original vertex,define permitted bank gluing. Copy reflection and fixed/moving pose
choices do not rewrite these held source-frame correspondences.

The boundary is checked first,then holes in original declared order. Each consecutive exit/entry
must be the same original source port or equivalent through the required bank links;the closing
last-to-first pair is checked too. `Adjacency` reports the actual first original result-use pair.
Reordering complete cut members,flipping one traversal or appending an unrelated hole to the
boundary can therefore fail despite complete unique cut membership. No path is reordered or repaired.

`JoinCutConnectivityError` retains actual `contours()`,original `limits()` and first `refusal()`.
For Adjacency,`from_use()` and `to_use()` lend those actual preceding/next use views and their
original declaration/direction/source parents. Quota causes lend neither use. Unexpected
`BankSourceInvariant`,`UseSourceInvariant` or `DeclaredLoopInvariant` means an internal native
parent/view invariant failed;it must be diagnosed,not labelled an unsupported garment or skipped.
No such invariant failure occurred in the focused native checks.

`RequiredJoinCutConnectivity` retains the same actual cut-use parent and policy.
`connection_count()` counts all consecutive-plus-closing requirements,one per declared cut use.
Work is O(H+U+1),with fixed four bank ports and a loop cursor;there is no new allocation,source
contour scan,registry/provider call or geometric evaluation. Earlier storage remains separate.

This is required symbolic source-path gluing under the authored orientation requests. Actual
mapped endpoints may still fail to coincide,or a posed result may violate CCW/simple outer
geometry,hole winding/classification/containment or exact nondeforming feasibility. Those remain
real G2/G3 obligations,along with numeric correspondence,currentness,noncut/copy/consumer
reconciliation,retirement and authorized atomic publication. A source-path declaration grants
none of those certificates and supplies no substitute geometric result.

## Declare proposed result construction-line uses

`JoinConstructionManifest` retains an explicit operation and ordered proposed construction uses.
Each `JoinConstructionUse::new(tag,direction)` requires an actual output role and explicit traversal
in that output's parameter frame. It does not accept source-relative `JoinParameterDirection`.
Construction geometry can reuse cut or noncut outputs and repeat roles,as the canonical Piece's
construction list permits. Each repeated occurrence remains a separate original authored field.

```rust
use sc_core::{construction::*,ontology::*};
fn line(tag:LocalTag)->JoinConstructionUse {
    JoinConstructionUse::new(tag,Direction::Original)
}
fn declare_lines<'a>(manifest:&'a JoinConstructionManifest,path:&'a RequiredJoinCutConnectivity<'a>)
    ->Result<DeclaredJoinConstructionUses<'a>,JoinConstructionUseError<'a>> {
    manifest.admit(path,JoinConstructionUseLimits::new(10,4,4))
}
```

`JoinConstructionUseLimits::new(N,U,W)` admits every actual candidate output N,every proposed
construction occurrence U and U additional associations before operation,fields or allocation.
Repeated roles still count separately. Earlier complete input/source/candidate/direction/cut-use/
path resources and caller-owned storage remain separate;these entries are not allocator bytes.
For N10/U4,W4 is exact:N9 refuses `Outputs`,U3 refuses `Uses`,W3 refuses `Workspace`,all before a
wrong operation or unknown tag. With quotas met,`Operation` precedes the first `UnknownOutput`.
An active catalogue role absent from the actual candidate/direction context remains unknown.

For authored uses4-Original,9-Reversed,4-Reversed,6-Original,the four fields remain in that exact
order. Role4 is not deduplicated and its two traversal fields remain separate. A cut role such
as9 or fold-attributed role such as6 can be proposed as construction geometry without becoming
a new cut contour or fold indicator. Original source-field retention/omission/mapping and result
semantic meaning still require their separately owned complete reconciliation.

`JoinConstructionUseError` lends actual `manifest()`,actual required cut `connectivity()`,original
`limits()` and first `refusal()`. `refused_use()` lends the actual original first UnknownOutput
field;whole-resource/operation causes return `None`. A later failure returns no accepted prefix.

`DeclaredJoinConstructionUses` retains those same whole parents and reports `use_count()` and
`workspace_entries()`. `uses()` traverses original order and reverse. `uses_for(tag)` scans all U
associations and lends every matching occurrence in original order or reverse;it allocates no
extra index and does not merge repetitions. Each authentic `JoinPlacedConstructionUse` lends
its whole context,original index,actual declared/traversal fields and original directional
output/candidate/source attribution. `logical_output()` remains the declared operation/tag tuple,
without minting or publishing an edge. Admission work is O(U log(N+2)+1),with U new associations
and no source/key scan,registry/provider call or geometric evaluation.

An explicit empty proposal can admit U0/W0 while retaining all original construction/fold fields
on its immutable parents. This records proposed data only;it authorizes no source omission,
semantic source loss,empty construction result or no-op. Complete original-field reconciliation,
fold/consumer/copy requirements,currentness,geometry,retirement and atomic authoring authority
remain mandatory later stages. This declaration alone cannot close the join's noncut obligations.

## Declare every original construction-field action

`JoinConstructionSourceManifest` retains an operation and ordered explicit actions for every
original A/B construction occurrence. `JoinConstructionSourceAssignment::new` names its A/B role,
snapshot field index,expected `DirectedEdge` and explicit `JoinConstructionSourceIntent`.
The expected persistent EdgeRef **and traversal** must match the actual original field. Snapshot
positions locate fields on these immutable parents;they are never standalone persistent geometry
identities and cannot silently substitute a moved/changed field.

```rust
use sc_core::{construction::*,ontology::*};
fn omit(side:SeamSideId,index:usize,expected:DirectedEdge,evidence:EntityId)
    ->JoinConstructionSourceAssignment {
    JoinConstructionSourceAssignment::new(side,index,expected,
        JoinConstructionSourceIntent::Omit{evidence_source:evidence})
}
fn declare_actions<'a>(manifest:&'a JoinConstructionSourceManifest,uses:&'a DeclaredJoinConstructionUses<'a>)
    ->Result<DeclaredJoinConstructionSources<'a>,JoinConstructionSourceError<'a>> {
    manifest.admit(uses,JoinConstructionSourceLimits::new(10,4,8))
}
```

`Retain` explicitly requires later complete target mapping for that source occurrence.
`Omit { evidence_source }` preserves an explicit request for omission/consumer-repair evidence.
It neither accepts that evidence nor authorizes deletion,source retirement or result publication.
Zero/shared requested ids are ordinary authored requests,not defaults,registry kind/state/existence
facts or accepted approval. Equal requested sources never merge the separate original field scopes.

`JoinConstructionSourceLimits::new(R,M,W)` admits the complete cached original A+B occurrence
count R,including cut/fold fields,then checked original construction count K,all raw assignments M
and checked2M source-index/association entries before source fields or allocation. Earlier complete
parent policies and caller-owned input storage remain required;entries do not measure allocator bytes.
`SourceCountOverflow` and `WorkspaceOverflow` retain nonrepresentable complete arithmetic.
For R10,K4,M4,W8 is exact:R9 refuses `SourceOccurrences`;M3 refuses `Assignments`;W7 refuses
`Workspace`,before a wrong operation or invalid source locator.

After quotas,`Operation` precedes exact `Count` M=K. Original rows then check `DuplicateSource`,
`MissingSource`,then expected edge/traversal `SourceField`. Unique actual construction membership
plus M=K declares every original occurrence,including repeated edges with distinct direction/slots;
equal count alone cannot hide a repeated source or omitted field. Original authored row order remains.
A source index that exists with a different edge or direction refuses rather than rebinding.

For original A0,A1,B0,B1,rows B1-Omit,A0-Retain,B0-Omit,A1-Retain can retain their authored order.
The two omission fields can explicitly request the same id,including0;both still require separate
actual scope/consumer justification. Three or five rows refuse expected4 before their bad fields.
An empty action list is complete only when actual K is0;it refuses expected4 for four originals.

`JoinConstructionSourceError` lends actual `manifest()`,actual proposed `uses()`,original `limits()`
and first `refusal()`. `refused_assignment()` lends the actual first refused raw row,including the
second duplicate;resource/operation/count causes return `None`. `refused_source_field()` lends
the actual original mismatched field only for SourceField,never a copied expected replacement.
A later failure returns no accepted action prefix.

`DeclaredJoinConstructionSources` retains those whole parents,complete `source_count()` and
`workspace_entries()`. `assignments()` lends original order and reverse;`assignment(side,index)`
queries the unique actual source slot. Each authentic `JoinConstructionSourceAction` lends its
whole context,original assignment index,actual raw assignment,actual `source_field()`,original
Piece/physical copy,and original intent/evidence-source field. No source fields or requests are copied
into replacement owners. Work is O(M log(M+2)+1),with2M new index/association entries and no source/key
scan,registry/provider/evidence call or geometric evaluation.

This stage proves complete explicit source-action requests only. Retained output groups,whole
source-range coverage,original/result multiplicity,additional target content and actual omission
justification/visible repairs remain mandatory mapping/evidence work. Fold/copy/consumer semantics,
currentness,real geometry,retirement and authorized atomic effects remain separate obligations.
An action list can therefore bind beside an empty proposed construction list without certifying
that Retain has been fulfilled or Omit justified. The join's source reconciliation remains open.

## Group retained construction output references

`JoinConstructionGroupManifest` retains an explicit operation and ordered
`JoinConstructionMappingGroup::new(side,index,outputs)` records. Each selector names an actual
Retain action in the guarded source-action parent;its original persistent edge/traversal expectation
was already checked there. The snapshot selector is not an independent geometry identity.
Every output reference is an explicit persistent LocalTag,not a candidate list position.

```rust
use sc_core::{construction::*,ontology::*};
fn retained(side:SeamSideId,index:usize,tags:Vec<LocalTag>)->JoinConstructionMappingGroup {
    JoinConstructionMappingGroup::new(side,index,tags)
}
fn group<'a>(manifest:&'a JoinConstructionGroupManifest,sources:&'a DeclaredJoinConstructionSources<'a>)
    ->Result<DeclaredJoinConstructionGroups<'a>,JoinConstructionGroupError<'a>> {
    manifest.admit(sources,JoinConstructionGroupLimits::new(4,4,4,12))
}
```

`JoinConstructionGroupLimits::new(K,G,L,W)` admits **all** source-action scopes K,including Omit,
and every raw group header G before nested lengths. Complete checked tag-reference count L and
checked2G+L index/group/output entries then precede fields or allocation. Earlier complete source,
candidate,action and proposed-use resources remain separate;logical entries are not allocator bytes.
`ReferenceOverflow`/`WorkspaceOverflow` retain nonrepresentable complete arithmetic. For K4/G4/L4,
W12 is exact:K3 refuses `Sources`,G3 refuses `Groups`,L3 refuses `References`,W11 refuses `Workspace`
before a wrong operation or malformed source/tag field.

With quotas met,`Operation` precedes `Count`:G must equal the actual number of Retain actions.
The complete source-action intents are scanned under K. Each original group then checks
`DuplicateSource`, `MissingSource`, `OmittedSource`, `EmptyGroup`,then each tag's `UnknownOutput`,
`SourceSide`, `SourceEdge`. Unique actual Retain membership plus exact count supplies one group
per Retain scope. Omit cannot also supply a retained group. A bare catalogue role cannot replace
membership in the actual directional candidate set.

A candidate must use the same original A/B role and persistent source edge as its guarded field.
Equal EdgeRefs on different A/B Pieces cannot substitute. Within the same actual source frame,
a candidate attributed to a cut,construction or fold occurrence can name that same canonical edge;
this associates source geometry only,without accepting any resulting fold or construction semantics.

Original group/tag order remains intact. Distinct original repeated construction fields can
explicitly reference the same role in separate groups;their source scopes remain separate. A partial
range or repeated tag can still associate at this stage:ordered whole0..1 coverage and target-use
presence/multiplicity are mandatory subsequent checks. Group association is not their certificate.
If every source action is Omit,an explicit empty group list can cover the no-Retain scope while all
original omission requests remain outstanding.

`JoinConstructionGroupError` lends actual `manifest()`,actual guarded `sources()`,original `limits()`
and first `refusal()`. Group-specific causes lend the original `refused_group()`;output-specific
causes lend the actual original `refused_tag()`. `refused_source_action()` lends the actual known
source action for a group cause,if present. `refused_output()` lends the actual directional candidate
only for SourceSide/SourceEdge. Whole-resource/operation/count causes lend none of those fields.
No accepted prefix,replacement source,role sorting or rescaling is returned after a later failure.

`DeclaredJoinConstructionGroups` retains those whole parents and reports `group_count()`,
`reference_count()` and `workspace_entries()`. `groups()`/`outputs()` preserve authored order and
reverse;`group(side,index)` queries an actual Retain scope. `JoinConstructionOutputGroup` lends its
actual original definition,source action and ordered member views. Each `JoinConstructionGroupOutput`
lends its original group/reference indices,actual group/tag field,source action,directional output
and actual positive ascending source range. Repeated references remain separate original fields.

`required_traversal()` states which proposed construction traversal would preserve that original
field's direction under the explicit output parameter-orientation request. Original+SameAsSource
requires Original;Original+ReverseSource requires Reversed. An original reversed field swaps those
requirements. This is a requirement,not an authored target value or achieved geometric map.

Work is O(K+G log(G+K+2)+L log(N+2)+1) for N actual candidates,with2G+L additional entries. There
is no provider,registry,evidence or geometry call. Complete retained interval partitions,actual
result-use traversal/multiplicity,additional-result intent,omission/consumer justification,fold/copy
semantics,currentness,geometry,retirement and atomic authority remain separately required.

## Require whole retained construction-field partitions

`DeclaredJoinConstructionGroups::require_source_coverage` checks every Retain group's exact
whole0..1 source interval in its **original explicit reference order**. Original source fields walk
from0 to1;reversed source fields walk from1 to0. Each candidate's `EdgeRange` stays positive
ascending in that original source frame. Output parameter-orientation requests determine the
separate required target traversal;they do not reverse or rescale these source intervals.

```rust
use sc_core::construction::*;
fn coverage<'a>(groups:&'a DeclaredJoinConstructionGroups<'a>)
    ->Result<CoveredJoinConstructionGroups<'a>,JoinConstructionCoverageError<'a>> {
    groups.require_source_coverage(JoinConstructionCoverageLimits::new(4,8))
}
```

`JoinConstructionCoverageLimits::new(G,L)` admits all actual retained groups G and all original
references L before source directions or interval fields are read. For G4/L8,G3 refuses `Groups`
and L7 refuses `References`,even if the first group has invalid interval ordering. Earlier whole
source/action/group policies remain in force;this scan allocates no additional workspace.

For an Original source field,two references covering0..half and half..1 pass in that order.
A Reversed source field requires half..1 followed by0..half. Swapping either list is refused;
there is no automatic sort. A partial first interval produces `Gap` before that reference. Missing
final coverage produces `Gap` with `index:None`. Missing bounds remain ascending regardless of
field direction. A reference that starts behind the exact traversal cursor produces `Overlap`,
including repeated whole or partial ranges. Each Retain scope is checked separately,even when
multiple original fields explicitly reference the same source geometry.

`JoinConstructionCoverageError` retains actual whole `groups()`,original `limits()` and first
`refusal()`. Interval causes lend the actual `refused_group()`. `refused_output()` lends the
actual next or overlapping original reference;final gaps and whole quota causes return None.
Refusal returns no accepted prefix and never substitutes a copied source field or interval.

`CoveredJoinConstructionGroups` retains actual whole `groups()` and `limits()`,with complete
`group_count()`/`reference_count()`. Work is O(G+L+1),with constant cursor state and no provider,
registry,geometry or evidence call. If every original action explicitly requests Omit,the empty
Retain group scope has vacuous source coverage;those omission requests are still outstanding.

Whole source-interval coverage does not establish actual result-use presence,traversal or
multiplicity. For example,four retained whole-field groups can pass beside an empty proposed
construction list;target mapping must still refuse that missing result content. Additional result
intent,omission/consumer justification,achieved parameter maps,fold/copy semantics,currentness,
geometry,retirement and atomic authority remain separate requirements.

## Assign retained references to actual construction occurrences

`JoinConstructionTargetManifest` retains the operation,ordered
`JoinConstructionTargetAssignment::new(side,source_index,uses)` rows,and an explicit
`JoinAdditionalConstructionIntent`. Each row names an actual whole covered Retain group. Its
ordered `uses` list contains one original proposed-construction occurrence ordinal per ordered
source reference. These snapshot ordinals distinguish repeated uses;they are not persistent geometry
identities. Persistent output identity still comes from each actual proposed use's tag.

```rust
use sc_core::{construction::*,ontology::*};
fn assignment(side:SeamSideId,index:usize,uses:Vec<usize>)->JoinConstructionTargetAssignment {
    JoinConstructionTargetAssignment::new(side,index,uses)
}
fn targets<'a>(manifest:&'a JoinConstructionTargetManifest,coverage:&'a CoveredJoinConstructionGroups<'a>)
    ->Result<DeclaredJoinConstructionTargets<'a>,JoinConstructionTargetError<'a>> {
    manifest.admit(coverage,JoinConstructionTargetLimits::new(4,8,9,20))
}
```

`JoinConstructionTargetLimits::new(M,T,U,W)` admits all raw assignment headers M before nested
lengths,then checked complete selector count T,checked M+2T workspace,and all actual proposed uses U
before fields or allocation. U includes every unmapped additional occurrence. Workspace counts
logical transient source-index entries M,target-index entries T and associations T,not allocator
bytes. Successful retained storage is2T;earlier immutable source coverage and use policies remain.
For M4/T8/U9,W20 is exact:M3 refuses `Groups`,T7 `References`,U8 `Uses`,W19 `Workspace`,before
an incorrect operation or malformed first source/target selector. Arithmetic refuses explicitly.

With resources met,`Operation`,whole `Count` and whole `ReferenceCount` precede row fields.
Each original row checks `DuplicateSource`, `MissingSource`, `GroupReferenceCount`;each ordered
reference then checks `DuplicateUse`, `MissingUse`, `Output`, `Traversal`. Exact unique actual
Retain membership covers all source groups. Every row must have exactly its group's reference
count. Every selected ordinal must name an actual proposed occurrence with the source reference's
persistent tag and `required_traversal()`. A catalogue role or a matching tag alone is insufficient.

Every target occurrence can carry only one retained reference. Distinct original source fields
that reuse geometry still require distinct actual result occurrences. Explicit assignments can name
interleaved original result positions;raw rows,reference order and actual proposed lists remain
unchanged. There is no inferred occurrence matching,sorting,deduplication or geometry substitution.
An empty proposed list cannot carry nonempty retained groups;it refuses `MissingUse`.

`JoinConstructionTargetError` retains actual whole `manifest()`,`coverage()`,original `limits()`
and first `refusal()`. Row causes lend the original `refused_assignment()` and actual known
`refused_source_group()`,if present. Reference causes lend the original `refused_use_index()` and
actual `refused_source_output()`. Duplicate-use/tag/traversal causes lend the actual `refused_use()`;
a missing target has no replacement occurrence loan. Whole resource/owner/count causes lend none.

`DeclaredJoinConstructionTargets` retains those whole parents and reports `mapping_count()` and
peak `workspace_entries()`. `mappings()` preserves original row/reference order and reverse;
`mapping_for_use(index)` queries the actual assigned occurrence. Each `JoinMappedConstructionTarget`
lends its actual owner,original indices/raw row/ordinal field,source group/source reference and
actual proposed occurrence with original tag/traversal fields. No accepted prefix escapes refusal.

Additional intent is explicit `SourceMappedOnly` or `AllowAdditional{evidence_source}`. This stage
retains that request only:even SourceMappedOnly can bind eight retained references beside nine
proposed uses. The next whole-use reconciliation must reject the extra use or retain explicit
additional-content requirements. An evidence id,including0,is no kind/state/currentness or approval
certificate. Source Omit requests still need separate source/consumer justification.

Work is O(M log(M+G+2)+T log(T+2)+1) for G covered source groups. No provider,evidence,registry or
geometry is called. Complete leftover-use reconciliation,additional/omission justification,achieved
maps,fold/copy semantics,currentness,retirement and atomic authority remain required separately.
