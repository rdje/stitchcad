# Native mirror output attribution and contour declarations

Original input,source and metadata parents remain in [Mirror Piece-copy contracts](drafting-mirror-contracts.md).
These declarations supply no current Design,geometry or issuance authority.

## Original-point candidate attribution

`MirrorPiecePointManifest` retains an original operation and ordered `MirrorPiecePointCandidate`
records. Each candidate declares a persistent point-role tag and an original PointRef. Admission
against actual BoundMirrorPiecePointRoles checks metadata attribution;it does not evaluate a
reflection or certify that a source exists in the Piece,current registry or common frame.

| API | Meaning |
| --- | --- |
| `MirrorPiecePointCandidate` | Original explicit tag and PointRef field |
| `MirrorPiecePointManifest` | Whole unadmitted operation/candidate proposal |
| `MirrorPiecePointManifestLimits::new(N,W)` | Explicit whole candidate and workspace-entry allowances |
| `AdmittedMirrorPiecePoints` | Complete locally admitted candidate/role associations |
| `MirrorPiecePoint` | Authentic original candidate,source field and point-role view |
| `MirrorPiecePointManifestError`, `MirrorPiecePointManifestRefusal` | Original proposal/parents/limits and first actual cause |

The checked workspace count2N precedes N and workspace quotas,then operation identity. Only after
those complete checks does indexing begin. The workspace is one N-entry tag index and one
N-entry association collection;these are entry counts,not allocation bytes or prior input-storage
bounds. Existing catalogue D/A/key-byte limits retain their separate scope.

Each original candidate checks duplicate tag,undefined/inactive point role,same-operation source,
then known source/result Piece and assigned Material aliases,in that order. A Piece or Material
identity cannot silently become a point-creating operation. Error fields retain the actual source
PointRef/tag,original candidate position and relevant typed identity. No accepted prefix escapes.

```rust
use sc_core::{construction::*,ontology::*};
let raw=MirrorPiecePointManifest::new(EntityId::from_bits(10),
    vec![MirrorPiecePointCandidate::new(LocalTag::new(9),
        PointRef::new(EntityId::from_bits(90),LocalTag::new(7)))]);
assert_eq!(raw.candidates().first().expect("original field").tag(),LocalTag::new(9));
fn admit<'a>(raw:&'a MirrorPiecePointManifest,roles:&'a BoundMirrorPiecePointRoles<'a>,
    limits:MirrorPiecePointManifestLimits)
    ->Result<AdmittedMirrorPiecePoints<'a>,MirrorPiecePointManifestError<'a>> {
    raw.admit(roles,limits)
}
```

Candidate order and original source fields remain exact,including reverse iteration. Explicit
lookup uses the persistent tag,not its candidate position. A view's point role belongs to the
actual admitted point catalogue. `logical_output()` constructs the declared PointRef tuple from
the actual mirror operation and canonical persistent role tag. This supplies no generated entity,
coordinates,registry insertion,global freshness or publication authority.

Distinct output roles may explicitly name the same original PointRef. Both original candidates
remain present;no role is coalesced or chosen by coordinates or ordering. This metadata does not
prove physical duplication or unique geometric correspondence. Read-only provenance querying
below preserves that ambiguity;actual source/frame/role continuity and whole correspondence remain
required under the existing manifest/copy/semantic/execution owners.

An explicitly empty candidate proposal admits with N0/W0 under the actual operation. Existing
active/inactive role definitions remain unchanged;no missing point is manufactured. This is
empty candidate presence metadata. It does not establish an unchanged result,no-op,complete
source/consumer coverage or executable reflected Piece. Actual source-point ownership/currentness,
common frame,supported reflection domain and semantic/geometry effects remain separate proof.

## Querying original point candidates

`points.query_source(&query,limits)` compares the exact original PointRef creator and tag
against every admitted candidate. It answers which proposed roles name that identity. It does
not resolve the source,compare coordinates or map a point through a reflection.

| API | Meaning |
| --- | --- |
| `MirrorPiecePointQuery::new(source)` | Exact original source identity request |
| `MirrorPiecePointQueryLimits::new(N,M)` | Explicit whole candidate and complete match allowances |
| `MirrorPiecePointMatch` | Actual owner,query,limits,count and authentic candidate views |
| `MirrorPiecePointMatchKind` | Unique,Ambiguous or Gap in this admitted proposal |
| `MirrorPiecePointQueryError`, `MirrorPiecePointQueryRefusal` | Original owner/query/policy and whole refusal |

The full actual candidate count must fit N before any source scan,including candidates unrelated
to the query. Admission then counts all matches and refuses if their complete population exceeds
M. Neither quota returns a truncated prefix. This pass uses O(N) work and constant auxiliary
space without a new index or array. `matches()` scans original candidate order,including reverse
iteration;each repeated caller pass performs its own scan. The limits bound this query admission,
not arbitrary subsequent caller work,earlier input allocation or complete mirror execution.

```rust
use sc_core::{construction::*,ontology::*};
let query=MirrorPiecePointQuery::new(PointRef::new(EntityId::from_bits(90),LocalTag::new(7)));
fn inspect<'a>(points:&'a AdmittedMirrorPiecePoints<'a>,query:&'a MirrorPiecePointQuery)
    ->Result<MirrorPiecePointMatch<'a>,MirrorPiecePointQueryError<'a>> {
    points.query_source(query,MirrorPiecePointQueryLimits::new(100,4))
}
assert_eq!(*query.source(),PointRef::new(EntityId::from_bits(90),LocalTag::new(7)));
```

With candidates `[role9 -> 90:7,role2 -> 90:7]`,query90:7 is Ambiguous with two matches in
that exact order. `unique()` returns none;the API never chooses the first role. With candidates
`[role9 -> 90:7,role2 -> 91:3]`,query90:7 is Unique and lends the actual role9 candidate.
Query90:8 is Gap even though its creator matches90:7. Querying the declared output10:9 is also
Gap unless an original source explicitly names that identity;this API is not output-tag lookup.

Gap means no candidate in this proposal names the original identity. It does not mean the
original is globally missing,retired or geometrically unmappable. An empty proposal admits a
zero-work/zero-match Gap while canonical role history stays intact. Unique likewise establishes
only one proposed attribution;source existence,Piece membership,current frame/registry,
geometric correspondence,registration repair and actual identity issuance remain separate work.

## Original-edge candidate attribution

`MirrorPieceEdgeManifest` proposes ordered persistent edge-role tags with exact original EdgeRefs.
Admission borrows the actual DeclaredMirrorPieceIdentity and its original edge-role catalogue,
metadata and source Piece. Point-role membership and PointRefs cannot substitute for these types.
An admitted candidate set is an intermediate declaration;complete source occurrence coverage and
reflected contour mapping remain the following steps.

| API | Meaning |
| --- | --- |
| `MirrorPieceEdgeCandidate` | Explicit persistent tag and original EdgeRef field |
| `MirrorPieceEdgeManifest` | Original operation and whole candidate proposal |
| `MirrorPieceEdgeManifestLimits::new(N,W)` | Complete candidate and workspace-entry allowances |
| `AdmittedMirrorPieceEdges` | Actual canonical Piece declaration and admitted candidate associations |
| `MirrorPieceOutputEdge` | Authentic output candidate,edge-role view and original source field |
| `MirrorPieceEdgeManifestError`, `MirrorPieceEdgeManifestRefusal` | Original proposal,parent,limits and first cause |

The checked workspace count2N,whole N quota and whole workspace quota precede operation comparison
and allocation. Workspace is one N-entry tag index and one N-entry association collection;these
are entry counts,not allocated bytes or earlier storage. The original edge catalogue retains its
separate definition/active/key-byte limits. Each candidate then checks duplicate tag,undefined or
inactive role,self-supply,and known source/result Piece or assigned Material creator aliases.
No accepted prefix escapes. Views and errors retain actual parents and original fields.

```rust
use sc_core::{construction::*,ontology::*};
let raw=MirrorPieceEdgeManifest::new(EntityId::from_bits(10),vec![
    MirrorPieceEdgeCandidate::new(LocalTag::new(9),
        EdgeRef::new(EntityId::from_bits(40),LocalTag::FIRST))]);
fn admit<'a>(raw:&'a MirrorPieceEdgeManifest,piece:&'a DeclaredMirrorPieceIdentity<'a>)
    ->Result<AdmittedMirrorPieceEdges<'a>,MirrorPieceEdgeManifestError<'a>> {
    raw.admit(piece,MirrorPieceEdgeManifestLimits::new(100,200))
}
assert_eq!(raw.candidates().first().expect("original").tag(),LocalTag::new(9));
```

`edges()` retains original proposal order and supports reverse iteration. `edge(tag)` uses the
persistent tag even when candidate order differs from catalogue active order. `logical_output()`
constructs the declared EdgeRef from the actual mirror operation and canonical role tag;it does
not issue an identity,insert an edge into a registry or evaluate reflected geometry.

Two distinct roles may still propose the same source at this intermediate stage. Neither is
silently selected or merged. Admission also permits sources not yet established as members of
the Piece;the complete-coverage stage must resolve those cases against every actual boundary,
hole,construction and fold occurrence. Repeated occurrences must remain distinct authored uses
even when their original EdgeRef identity is shared. No direction or contour order is inferred here.

An empty proposal admits under N0/W0 without retiring active or inactive canonical roles. This
is candidate presence metadata,not an executable empty Piece,complete copy,no-op or unchanged
result. Actual source coverage,currentness,frame,geometric correspondence,global freshness,
physical-copy/semantic effects and publication remain separate requirements.

## Complete source-edge coverage

`edges.admit_source_coverage(limits)` requires exactly one candidate for each distinct EdgeRef
in the actual bound source Piece. It accounts for every original boundary,hole,construction and
fold occurrence. Two roles proposing the same source refuse as AmbiguousSource;an uncovered
original field refuses as MissingSource. After all occurrences match,any candidate unused by
the Piece refuses as UnusedCandidate. Nothing is selected by coordinates or a tag alone.

| API | Meaning |
| --- | --- |
| `MirrorPieceEdgeCoverageLimits::new(N,R,W)` | Complete candidates,source occurrences and workspace entries |
| `MirrorPieceEdgeCoverage` | Whole unique source attribution with all original occurrences |
| `MirrorPieceMappedEdgeOccurrence` | Authentic original source field and unique output candidate |
| `MirrorPieceEdgeCoverageError`, `MirrorPieceEdgeCoverageRefusal` | Actual owner/policy and first whole failure |

Checked N+R,whole N,whole R and workspace quotas precede source inspection and allocation. The
workspace has an N-entry source index and R-entry occurrence association vector;these count
entries,not bytes or prior storage. Work is O((N+R)log(N+2)). Earlier source and catalogue quotas
retain their separate scope. Duplicate-source refusal follows original proposal order,missing
sources follow actual boundary/hole/construction/fold order,and unused candidates follow proposal
order rather than the source index's sorted keys. No partial coverage escapes a refusal.

```rust
use sc_core::construction::*;
fn cover<'a>(edges:&'a AdmittedMirrorPieceEdges<'a>)
    ->Result<MirrorPieceEdgeCoverage<'a>,MirrorPieceEdgeCoverageError<'a>> {
    edges.admit_source_coverage(MirrorPieceEdgeCoverageLimits::new(4,8,12))
}
```

For original identities `[40,41,42,43,41,41,43,40]`,a proposal mapping40→role9,41→role2,
42→role4 and43→role6 produces eight occurrence views with roles `[9,2,4,6,2,2,6,9]`.
The four source identities share output candidates across repeated uses. Every occurrence keeps
its original location,actual source field and directed/fold distinction;no authored use disappears.
`occurrences()` supports reverse iteration,while `edge_for_source(&original)` returns the unique
actual candidate for that exact original EdgeRef. Proposal order may differ from both source
occurrence order and catalogue active order;none becomes a persistent identity.

An empty candidate proposal can pass intermediate admission but fails this stage at the first
actual source occurrence. This does not retire canonical roles. Success establishes complete
identity attribution against the supplied immutable Piece snapshot. It does not prove current
registry membership,original/common frames,supported reflection,output winding or directions,
geometric validity,identity issuance,physical copies or semantic effects. The following native
contour declaration supplies the desired construction/fold mapping,without those actual proofs.

## Reflected contour declarations

`coverage.declare_contours(&policy,limits)` declares desired logical output fields under complete
source-edge coverage. Every actual output candidate needs one explicit parameter direction in
MirrorPieceContourPolicy. No direction is inferred from role keys,proposal order or coordinates.

| API | Meaning |
| --- | --- |
| `MirrorPieceEdgeParameterization::new(tag,direction)` | Explicit output parameter frame relative to reflected original source t |
| `MirrorPieceContourPolicy` | Whole original operation and ordered parameter entries |
| `MirrorPieceContourLimits::new(P,H,R,W)` | Whole parameter/output,hole,source-occurrence and workspace allowances |
| `MirrorPieceContours` | Desired logical boundary,holes,construction and cut-derived fold declarations |
| `MirrorPieceContourError`, `MirrorPieceContourRefusal` | Actual coverage/policy/limits and first whole refusal |

Direction::Original requires output t to follow reflected original source t. Direction::Reversed
requires output t to follow1-source t. These are declared requirements for later correspondence
proof,not evidence of actual current edge frames. For each original directed use,the output
direction composes the authored direction with this explicit parameter direction. Boundary and
hole traversal adds one further reversal,while construction traversal does not.

A supported reflection reverses loop winding. To retain the source loop's intended winding,the
desired boundary reverses original edge order and each traversal. Each hole does the same within
its original group;hole-group order remains intact. Construction uses retain authored order and
their reflected traversal,including repeated uses. Source winding/closure and actual reflection
must still be verified geometrically. This API cannot establish CCW or valid cutouts by itself.

```rust
use sc_core::{construction::*,ontology::*};
let policy=MirrorPieceContourPolicy::new(EntityId::from_bits(10),vec![
    MirrorPieceEdgeParameterization::new(LocalTag::new(9),Direction::Original)]);
fn declare<'a>(source:&'a MirrorPieceEdgeCoverage<'a>,policy:&'a MirrorPieceContourPolicy)
    ->Result<MirrorPieceContours<'a>,MirrorPieceContourError<'a>> {
    source.declare_contours(policy,MirrorPieceContourLimits::new(100,10,200,410))
}
assert_eq!(policy.parameterizations().first().expect("explicit").direction(),&Direction::Original);
```

For boundary `[source40 Reversed,source41 Original]`,mapped to roles9 and2 with Original
parameter directions,the desired output boundary is `[role2 Reversed,role9 Original]`.
If role2 instead declares Reversed parameter direction,its output traversal becomes Original.
For a hole `[source42 Reversed,source43 Original]`,mapped to roles4 and6,Original parameter
directions produce `[role6 Reversed,role4 Original]`. Construction `[source41 Reversed,
source41 Original]` remains in that occurrence order,with parameter directions composed separately.

Fold presence comes only from the already bound cut policy. NoFold produces no fold declaration;
ReflectedSource maps the actual original fold to its covered logical output. The earlier coverage
still accounts for the original fold occurrence in both cases. No fold direction is invented,and
the printed fold indication retains the same canonical cut authority.

For K raw parameter entries,N actual outputs,H actual hole groups and R original occurrences,
checked2K+H+R precedes whole K and N allowances,both using P,then H,R,workspace and operation
checks. Parameter indexing then refuses duplicate/unknown tags in policy order,followed by the
first uncovered parameter in candidate order. There are no defaults or partial contours. The
workspace bound counts an index and parameter associations of K entries each,H hole groups and
at most R output occurrences;NoFold can use fewer. It counts entries,not allocation bytes or prior
storage. Work is O((K+N+R)log(K+N+2)+H). Actual original fields,parents and complete policy remain
inspectable;`parameterization(tag)` lends the actual parameter field regardless of policy order.

The result is a logical declaration rather than an evaluated Piece or registry mutation. Actual
current sources,common/original frames,supported axis/reflection,geometric winding/closure and
error bounds,identity issuance,physical-copy associations and semantic/sewing effects remain
separate requirements. Native edge declarations are complete through this stage;the manifest's
supported correspondence requirements remain the next owner.

