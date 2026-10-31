# Native mirror Piece-copy contracts

## Native reflected Piece-copy inputs

`construction::MirrorPieceCopy` declares a reflected copy of the named existing Piece. The type
makes copy intent explicit. It retains an operation id,source Piece id and the authored `MirrorAxis`:
`ThroughPoints { start,end }` uses two actual PointRefs;`AlongRange` uses an actual DirectedRange.
No axis mode is inferred from geometry or formula values.

```rust
use sc_core::{construction::*,ontology::*};
let point=|id|PointRef::new(EntityId::from_bits(id),LocalTag::FIRST);
let input=MirrorPieceCopy::new(EntityId::from_bits(10),EntityId::from_bits(50),
    MirrorAxis::ThroughPoints { start:point(99),end:point(80) })?;
assert_eq!(input.source_piece(),EntityId::from_bits(50));
assert_eq!(input.dependency_count(),2);
# Ok::<(),Box<dyn std::error::Error>>(())
```

Local self-supply refuses the source Piece first,then start/end or range in original field order.
Private dependency views borrow the actual input and underlying axis fields. Repeated point uses
remain separate occurrences;directed range bounds/traversal and point source order stay authored.
The iterator covers axis occurrences only;the source Piece is a separate program dependency.
Reversing a valid geometric axis may preserve its eventual reflection,but this input stage does
not rewrite authored sources to establish that equality.

The constructor certifies neither a current Piece nor a usable axis. G2/program checks must prove
source existence/common frame and axis nondegeneracy/straightness. Repeated point references can
be retained as authored input without proving a valid reflection axis. A curved range cannot be
silently replaced with its chord or tangent.

No new Piece identity,edge/point tags or geometry is produced. Copy metadata,labels,materials,
handedness,cut quantities,pair reciprocity and sewing/reference mappings require explicit output
policy under `.1.c.mirror.outputs`,followed by the bus's identity allocation and atomic publication.
The existing `Mirroring` cut-plan field supplies no drafting reflection result. Current coverage,
reflection/error budgets and the stable output manifest remain actual G2/G3/execution obligations;
full construction verification stays deferred under `.5f.4`.

### Binding the actual mirror source Piece

`MirrorPieceCopy::bind_piece(&piece,&limits)` binds the actual immutable source before later
result policies. It requires an explicitly supplied `MirrorPieceSourceLimits`;there is no default.
The bound view retains the original input,axis,Piece and policy by reference. It copies no Piece,
metadata,physical cut copy or geometry,and creates no output identity.

| API | Meaning |
| --- | --- |
| `MirrorPieceSourceLimits::new(H,R)` | Whole hole groups and all original edge-field occurrences |
| `BoundMirrorPieceCopy` | Private association of the actual input,Piece,limits and admitted R |
| `MirrorPieceEdgeLocation` | Boundary,hole/construction/fold role and original field position |
| `MirrorPieceEdgeSource` | Actual directed field or undirected fold declaration |
| `MirrorPieceEdge` | Authentic field occurrence borrowing its bound owner and source |
| `MirrorPieceSourceError`, `MirrorPieceSourceRefusal` | Original context and first actual admission cause |

Identity mismatch refuses first. The whole H quota precedes reading individual hole lengths.
A checked sum then counts every boundary,hole,construction and fold field;the complete R quota
precedes any edge-source scan. A repeated construction edge counts each time,and a fold declared
on the boundary counts again as a separate field. For two boundary edges,two one-edge holes,
three construction uses and one fold declaration,R is8 even when some identities are shared.
The constructor uses O(H+R) work and constant auxiliary space;this bounds source association,
not whole-operation execution or already allocated Piece storage.

```rust
use sc_core::{construction::*,ontology::Piece};
fn bind_source<'a>(input:&'a MirrorPieceCopy,piece:&'a Piece,
    limits:&'a MirrorPieceSourceLimits)
    ->Result<BoundMirrorPieceCopy<'a>,MirrorPieceSourceError<'a>> {
    input.bind_piece(piece,limits)
}
let limits=MirrorPieceSourceLimits::new(2,8);
assert_eq!((limits.holes(),limits.edges()),(2,8));
```

After complete population admission,definite self-supply refuses the first source edge created
by this mirror operation,in boundary order,then holes and their edge order,construction uses,
and fold declarations. No accepted prefix escapes. Errors retain the actual input,Piece,policy,
original location and edge identity/tag. The immutable source is unchanged on success or refusal.

`bound.edges()` enumerates every original occurrence in that same order,including repeats.
Reverse iteration retains original positions. Directed fields keep their actual authored
traversal;fold fields have no invented direction. These positions locate fields and never become
persistent identities. Borrowing the complete Piece retains its original quantities,handedness,
companion,fold,face,material,layer and label;it does not select those fields as result metadata.

This association establishes neither a current Design member nor current edge ownership/coverage,
axis validity,common frame,reflection/winding,source-output correspondence or sewing effects.
A structurally valid Piece retains its original geometric deferral. Explicit result metadata,
fresh output roles/manifests,physical-copy and whole semantic/sewing policies remain the next
native output slices;actual reflection/currentness/publication retain their existing owners.

### Explicit result cut policy

After binding the actual source,`source.bind_cut_policy(&policy)` associates four explicitly
chosen result cut fields. Nothing defaults from source geometry,names,labels or cutting side.
This stage retains a request for a new Piece;it does not construct that Piece or physical copies.

| API | Meaning |
| --- | --- |
| `MirrorPieceCutPolicy::new(quantity,mirroring,fold,cutting_side)` | One canonical set of requested result cut fields |
| `MirrorPieceFoldIntent::NoFold` | Explicitly request no result fold,including when the source has one |
| `MirrorPieceFoldIntent::ReflectedSource` | Require later output attribution to the actual original source fold |
| `BoundMirrorPieceCutPolicy` | Private association of actual source and locally admitted cut request |
| `MirrorPieceCutPolicyError`, `MirrorPieceCutPolicyRefusal` | Original source/policy and first local refusal |

The raw policy has no default and retains caller fields unchanged. Binding refuses zero quantity,
then an odd MirroredPairs total,then a requested reflected source fold when the actual source has
none. Single and separate PairMember quantities need only be positive at this stage. Quantity
counts all requested physical copies;MirroredPairs includes both hands. No upper cut quantity is
invented by this declaration,and no copy list is allocated from it.

```rust
use sc_core::{construction::*,ontology::*};
let policy=MirrorPieceCutPolicy::new(1,Mirroring::Single,
    MirrorPieceFoldIntent::NoFold,CuttingSide::Face);
assert_eq!(*policy.quantity(),1);
assert_eq!(*policy.fold(),MirrorPieceFoldIntent::NoFold);
```

The bound quantity,mirroring and cutting-side accessors borrow the original policy fields.
`cut_on_fold()` derives from the one explicit fold policy;it is not authored a second time.
`fold_source()` borrows the original source fold field only for ReflectedSource. It supplies no
new output edge or reflected geometry. Requesting NoFold explicitly drops the result fold
instruction;the source Piece and its fold stay unchanged,and semantic effects remain required.

A source cut-four mirrored Piece may explicitly request a new Single Piece cut once with a
chosen cutting side. This declaration neither edits the source quantity nor silently expands,
removes or transfers its existing physical copies. Single refers to authored copies of the new
pattern geometry;geometric recipe reflection and a CutPlan copy's reflected orientation remain
separate. PairMember retains explicit handedness and companion id without name-based inference.
New Piece identity,companion existence/reciprocity/opposite hand/equal quantity,the complete
result CutPlan and actual geometric hand/face/fold correspondence still require later evidence.
Material,layer,label and the cut-derived print view are selected explicitly below. The cut
association alone cannot stand in for complete output metadata or sewing policy.

### Explicit material,layer,label and print data

`cut.bind_metadata(&policy,&limits)` completes the local result-metadata choices against the
actual bound source and canonical cut request. Every field deliberately selects PreserveSource
or Replace;there is no implicit inheritance and no name-based handedness or face inference.
The original source remains unchanged,and no Piece,physical copy or geometric output is created.

| API | Meaning |
| --- | --- |
| `MirrorPieceFieldIntent<T>` | Explicit PreserveSource or Replace for a typed field |
| `MirrorPieceMetadataPolicy` | Required independent material,layer-index and label-text choices |
| `MirrorPieceMetadataLimits::new(B)` | Complete resolved result-text allowance in UTF-8 bytes |
| `BoundMirrorPieceMetadata` | Authentic selected field borrows and canonical cut context |
| `MirrorPieceMetadataError`, `MirrorPieceMetadataRefusal` | Original cut/policy/limits and first complete refusal |

PreserveSource borrows the actual original field. Replace borrows the actual caller-supplied
value. These choices are independent:for example,preserve the source material and assembly
layer while supplying a new label,or replace an unresolved material while keeping the text.
Unresolved reason spelling,label whitespace and Unicode text remain exact;checking a blank
field does not trim the stored value. Layer index remains an explicit integer,not a placement.

```rust
use sc_core::{construction::*,ontology::*};
let policy=MirrorPieceMetadataPolicy::new(
    MirrorPieceFieldIntent::PreserveSource,
    MirrorPieceFieldIntent::Replace(2),
    MirrorPieceFieldIntent::PreserveSource);
fn bind_fields<'a>(cut:&'a BoundMirrorPieceCutPolicy<'a>,policy:&'a MirrorPieceMetadataPolicy,
    limits:&'a MirrorPieceMetadataLimits)
    ->Result<BoundMirrorPieceMetadata<'a>,MirrorPieceMetadataError<'a>> {
    let metadata=cut.bind_metadata(policy,limits)?;
    assert_eq!(metadata.printed_label().quantity,*cut.quantity());
    Ok(metadata)
}
assert_eq!(policy.layer_index(),&MirrorPieceFieldIntent::Replace(2));
```

B counts all four resolved label fields plus the resolved unresolved-material reason,if any.
Assigned material has no reason text. Lengths are checked and summed before scanning any text;
the whole B quota precedes completeness checks. The canonical label order is name,size,fabric,
colorway,then the unresolved material reason. Piece birth and mirror-result metadata share that
label rule. Admission uses O(1+B) work and constant auxiliary space without cloning strings.
This allowance covers resolved result text;it does not claim to bound unselected original fields,
already allocated source/policy storage or whole-operation memory. Material existence/kind/state,
size-set truth and physical meaning remain separate from local nonblank completeness.

For example,the label fields ` 左 `,`L`,`绸`,`赤` total12 UTF-8 bytes. An unresolved reason
` 待定 ` contributes8 more,so B20 admits the complete20-byte selection and B19 refuses before
any blank scan. An assigned material plus four one-byte label fields costs4 bytes,even if the
unselected source material carries a longer reason. All original selected spelling is retained.

`printed_label()` borrows the selected label text and derives quantity,mirroring/hand and fold
from the same admitted result cut policy. A source cut-four mirrored Piece can preserve its text
while the explicit result policy prints cut one,Single,no fold. The source print view remains
cut four with its original fold. Neither print quantity nor hand/fold is authored in label text
again. No print layout,output identity,companion reciprocity,complete CutPlan,material readiness,
source-output fold correspondence,geometry or sewing effect is certified by this local metadata
stage;the manifest,copy and semantic-policy slices remain required before native family closure.

## Shared edge-output role association

`metadata.bind_edge_roles(&catalog,limits)` binds the actual mirror source/cut/field metadata to
an admitted `EdgeOutputRoleCatalog`. It uses the same shared edge-role definitions,membership
and transition contracts as offset and slash/spread;the legacy OffsetRole names remain aliases.
No second role history or conversion is introduced.

| API | Meaning |
| --- | --- |
| `BoundMirrorPieceMetadata::bind_edge_roles` | Whole catalogue admission against the actual mirror operation |
| `BoundMirrorPieceEdgeRoles` | Authentic metadata/catalogue owner association |
| `MirrorPieceRoleBindingError`, `MirrorPieceRoleBindingRefusal` | Original owners,limits and first quota/operation cause |

The explicit `EdgeOutputRoleCatalogLimits` bounds all D definitions,including inactive ones,
all A active ids and complete original machine-key bytes B. Those three quotas precede the
actual operation identity check,in that order. The catalogue's existing admission supplies its
original key-byte count,so association uses constant work and no new workspace. Earlier catalogue
allocation and owned-storage limits retain their separate scope. A caller may choose a tighter
association allowance than the one used to admit the original catalogue.

```rust
use sc_core::construction::*;
fn associate<'a>(metadata:&'a BoundMirrorPieceMetadata<'a>,
    catalog:&'a AdmittedEdgeOutputRoleCatalog<'a>,limits:EdgeOutputRoleCatalogLimits)
    ->Result<BoundMirrorPieceEdgeRoles<'a>,MirrorPieceRoleBindingError<'a>> {
    metadata.bind_edge_roles(catalog,limits)
}
```

The bound view borrows both authentic parents. Active-role views still point to the catalogue's
original fields and preserve authored active order and explicit persistent tags. Positions are
not identities. Errors retain the whole metadata/catalogue and the selected limits;no accepted
prefix escapes. Definitions or keys are never rewritten to match another operation.

Empty and fully inactive catalogues remain valid metadata states here. They do not supply an
executable copy manifest or retire issued roles. Actual fresh Piece identity,complete candidate/
source-contour correspondence and point-role policy remain later manifest steps. The edge-role
catalogue grants no PointRef role namespace,global freshness,canonical issued-history completeness,
geometric role continuity,current Design/operation-kind proof or command publication authority.

## Canonical result Piece identity declaration

`roles.declare_piece_identity(&canonical_piece_id)` associates an explicit canonical result
Piece identity with the actual role/catalogue/source/metadata parents. It allocates no id and
creates no Piece. The result is deliberately a declaration,not a fresh or issued identity proof.

| API | Meaning |
| --- | --- |
| `BoundMirrorPieceEdgeRoles::declare_piece_identity` | Local typed-identity checks against the actual bound parents |
| `DeclaredMirrorPieceIdentity` | Authentic canonical identity-field and parent borrows |
| `MirrorPieceIdentityError`, `MirrorPieceIdentityRefusal` | Original field/parents and first intrinsic or typed-source alias |

The [Design structural contract](../spec/ontology-implementation.md) gives Piece and topology
operations separate owned identities. Copy intent requires a different result Piece from the
actual source. A separately handed result must also name a distinct companion. These three
checks run first:mirror operation,actual source Piece,then requested self-companion.

The declaration then rejects a result id used as an original source-edge creator,in original
boundary/hole/construction/fold order;then the first authored axis-source creator;then the source
and selected result assigned Material ids. Errors preserve the exact original source edge/tag/
location or axis dependency position through their actual parents. A missing typed source cannot
be repaired by silently using its operation/Material identity for a Piece. The source population
R was already admitted,so this stage uses O(R+1) work and constant auxiliary space.

```rust
use sc_core::{construction::*,ontology::EntityId};
fn declare<'a>(roles:&'a BoundMirrorPieceEdgeRoles<'a>,canonical_piece_id:&'a EntityId)
    ->Result<DeclaredMirrorPieceIdentity<'a>,MirrorPieceIdentityError<'a>> {
    roles.declare_piece_identity(canonical_piece_id)
}
```

A source companion reference is a use,not another owned declaration. For example,a native right
source100 may already name planned left companion200;the explicit left result200 names100 as its
companion. That reference alone does not prove200 already has an owner,and must not be rejected
as a duplicate owner. The result still cannot name200 as its own companion. Complete inventory
reciprocity,opposite hands/equal quantities,actual current ownership and geometric mirroring remain
later checks. If200 is already owned or was previously issued,the complete namespace/nonreuse
check must refuse it;this local declaration does not establish its absence.

The bound view borrows the actual canonical identity field. Identity is never inferred from
ordinal,label,coordinate or numerical ordering. Zero is not an absence sentinel:the existing
injected deterministic generator supports an explicit zero-start sequence. None is generated here.

This API represents canonical native declaration data. It adds no new-id field to a creation
command request. Production identity supply remains the trusted injected host/bus reserve;preview
must not mint identities or let request callers choose them. Actual current/historical namespace,
reserved-id nonreuse,operation-kind/ordering,canonical issued-role history and atomic publication
remain required under the existing .5f.2/.6 owners. Point-role identity,complete source/output
correspondence,copy and semantic policies remain separate manifest/family steps.

## Typed point-role catalogue metadata

Point output roles have their own catalogue and membership types. They share raw tag/key data
and bounded metadata admission with edge roles,but a point-domain proof cannot be used as an
edge-domain proof. The implementation keeps its shared admission component private;there is no
public admitted-edge getter or conversion. This stage generates no PointRef or coordinates.

| API | Meaning |
| --- | --- |
| `PointOutputRoleDefinition` | Shared raw machine-key/tag record;not a membership witness |
| `PointOutputRoleCatalogLimits`, `PointOutputRoleCatalogRefusal` | Shared explicit quota/typed-cause data |
| `PointOutputRoleCatalog` | Distinct owned original point definitions and active membership |
| `AdmittedPointOutputRoleCatalog` | Private point-domain admission with original owner/field views |
| `ActivePointOutputRole` | Authentic point membership and original definition/tag fields |
| `PointOutputRoleCatalogError` | Actual point owner,limits and original shared refusal |

D counts every declared definition,including inactive roles;A counts every supplied active-tag
occurrence. The complete checked workspace count2D+2A and D/A/key-byte quotas precede index
allocation. Definitions are checked in original order,tag before key;active entries retain
original order,duplicate before undefined. Counts bound index entries,not prior String/Vec
allocation,all allocation bytes or complete operation work. No default allowance is supplied.

```rust
use sc_core::{construction::*,name::MachineToken,ontology::*};
let raw=PointOutputRoleCatalog::new(EntityId::from_bits(10),
    vec![PointOutputRoleDefinition::new(LocalTag::new(9),MachineToken::new("pivot").expect("valid key")),
         PointOutputRoleDefinition::new(LocalTag::new(2),MachineToken::new("tip").expect("valid key"))],
    vec![LocalTag::new(2),LocalTag::new(9)]);
let catalog=raw.admit(PointOutputRoleCatalogLimits::new(2,2,8)).expect("complete metadata");
assert_eq!(catalog.workspace_entries(),8);
let role=catalog.active_role(LocalTag::new(9)).expect("actual canonical membership");
assert!(std::ptr::eq(role.catalog(),&catalog));
assert_eq!(role.definition().key().as_str(),"pivot");
```

Definitions and active tags remain borrowed from the actual original point catalogue. Lookups
include inactive definitions;active lookups/forward/reverse iteration return point-typed views
only. Equal contents in a second catalogue do not reparent a view. Explicit point and edge
catalogues may contain equal raw tags/keys,but their admitted membership types remain distinct.
Sharing a raw record does not supply either domain's ownership proof.

Empty and fully inactive point catalogues remain legal metadata states. They neither generate
points nor retire issued roles. Actual canonical issued-definition completeness,history-preserving
transitions,mirror operation association,source/output point attribution,current registry/frame,
geometric correspondence,nonreuse and publication remain separate requirements. The existing
PointConstruction's fixed logical point role is unchanged;this catalogue does not rewrite it.

## Point-role history-preserving metadata transitions

`before.admit_transition(&after,limits)` compares actual admitted point catalogues while retaining
both original owners. Its transition and activated/deactivated membership views are point-typed;
no public conversion exposes the private shared edge transition. Raw count/refusal data remains
shared metadata,not authority to change canonical history.

| API | Meaning |
| --- | --- |
| `PointOutputRoleTransitionLimits`, `PointOutputRoleTransitionRefusal` | Shared combined-bound/typed-cause records |
| `AdmittedPointOutputRoleCatalog::admit_transition` | Preserve every supplied prior point definition |
| `AdmittedPointOutputRoleTransition` | Actual before/after owners,counts and point-domain field views |
| `PointOutputRoleTransitionError` | Original point parents,limits and first actual refusal |

Complete combined D definitions,A active occurrences and key bytes B are checked before owner
and definition comparisons. Count overflow refuses first,then D/A/B quotas,operation mismatch,
then the first original before-definition whose tag was forgotten or key reassigned. Inactive
prior definitions count and must remain. No work index is allocated by this comparison;the
existing catalogue indexes retain their own admission scope.

```rust
use sc_core::construction::*;
fn transition<'a>(before:&'a AdmittedPointOutputRoleCatalog<'a>,
    after:&'a AdmittedPointOutputRoleCatalog<'a>,limits:PointOutputRoleTransitionLimits)
    ->Result<AdmittedPointOutputRoleTransition<'a>,PointOutputRoleTransitionError<'a>> {
    before.admit_transition(after,limits)
}
```

`introduced()` returns actual new after-definition fields with original after indices and order.
`activated()` returns actual after point-membership views in after order;`deactivated()` returns
actual before views in before order. Reverse iteration preserves those original positions.
Neither source changes,and no accepted prefix or synthesized role escapes a refusal.

For example,definitions9/main and2/end with active9 may become definitions7/new,2/end,9/main
with active2,7. Combined counts are D5,A3,B17;introduced7 keeps after index0,activation reports2,7,
and deactivation reports9. Restoring active9 later must retain all three definitions,including
inactive7. Replaying the older two-definition snapshot would forget7 and refuses even though
its former active list was valid. An absent evaluated point is not a retirement instruction.

This checks supplied structural transition intent. It does not establish that either caller
supplied the complete actual issued history,that an undo is authorized,or that role geometry is
continuous. Actual canonical history/maximum-issued retention,current Design,point registry/frame,
mirror/source-output association,nonreuse and atomic publication remain separate owners. No
PointRef or coordinates are generated,and catalogue sources remain immutable.

## Binding point roles to the actual mirror declaration

`identity.bind_point_roles(&points,limits)` associates an actual canonical Piece declaration
with an actual admitted point catalogue. The declaration already retains its original mirror
input,source Piece,cut/field metadata and edge-role parents. The point catalogue remains a
distinct domain:an admitted edge catalogue cannot substitute for this argument.

| API | Meaning |
| --- | --- |
| `DeclaredMirrorPieceIdentity::bind_point_roles` | Complete point bounds before actual mirror-operation association |
| `BoundMirrorPiecePointRoles` | Authentic declared Piece and typed point-catalogue parents |
| `MirrorPiecePointRoleBindingError` | Original declaration,catalogue,limits and first refusal |
| `MirrorPiecePointRoleBindingRefusal` | Shared D/A/key-byte/operation cause data,not a membership proof |

All point definitions D,including inactive ones,all active ids A and complete original key bytes
B must fit the explicit PointOutputRoleCatalogLimits before operation comparison. The expected
operation comes from the actual original MirrorPieceCopy input,not the new Piece id or a caller
substitute. The earlier catalogue admission supplies cached byte counts,so this association uses
constant work and no new workspace. It grants no prior-allocation or whole-execution bound.

```rust
use sc_core::construction::*;
fn bind_points<'a>(piece:&'a DeclaredMirrorPieceIdentity<'a>,
    points:&'a AdmittedPointOutputRoleCatalog<'a>,limits:PointOutputRoleCatalogLimits)
    ->Result<BoundMirrorPiecePointRoles<'a>,MirrorPiecePointRoleBindingError<'a>> {
    piece.bind_point_roles(points,limits)
}
```

The declaration chain is bind_piece,bind_cut_policy,bind_metadata,bind_edge_roles,
declare_piece_identity,then bind_point_roles. Every stage borrows the actual prior context.
Success and refusal preserve the actual canonical identity field and complete point catalogue;
active views still borrow their original point fields in authored order. Neither parent changes.

Empty and fully inactive point catalogues remain metadata states. They generate no PointRef,
retire no issued role and supply no executable source/output point manifest. Actual original
source-point attribution,current registry/membership/common frame,geometric correspondence,
canonical issued-history completeness,global freshness and publication still require their
separate owners. This binding cannot be used as point-source or actual reflection evidence.

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

