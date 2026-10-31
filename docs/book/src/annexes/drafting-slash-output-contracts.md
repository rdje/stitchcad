# Native slash/spread output contracts

Inputs remain in the [editing reference](drafting-edit-inputs.md). The shared edge-role
metadata also serves the [offset output contracts](drafting-output-contracts.md).

## Native slash/spread boundary and fullness intent

`SlashSpreadOutputIntent::new(input,boundary,fullness)` owns the actual
`SlashSpreadConstruction` and requires both output choices. Moving the input preserves its original
operation,Piece,directed cut,moving bank,pivot or movement axis,and embedded amount document. The
constructor performs no new allocation,evaluation,identity issuance or model edit. It declares
intent;an accepted output manifest and actual geometry remain separate stages.

`SlashSpreadBoundaryPolicy` has two explicit requests:

| Policy | Intent and remaining proof |
| --- | --- |
| `StraightBridges` | Straight connectors at corresponding opened cut mouths;actual endpoints/incidence and supported closed contour must be proved |
| `TrimOverlaps` | Trim overlapping banks;actual retained ranges/intersections and boundary selection must be explicit |

Neither policy supplies a default connector count,generated edge tag,intersection,loop choice or
geometric repair. A trim policy name cannot select an arbitrary loop. Unsupported or ambiguous
region/topology must refuse through the actual G2/G3 execution path;policy construction is not that
validation. The following native contracts retain explicit output roles,source/transform provenance
and trim-selection evidence obligations;actual geometry remains separate.

`SlashSpreadFullnessIntent::{Added,Removed,Unchanged}` records the intended effect separately from
boundary treatment. It stores no independently authored intake,Length/Area amount or solved fullness.
G2/G3 must derive the actual effect from the same original inputs and attribute it to this operation.
`Removed` neither closes a canonical Dart nor substitutes for its conserved intake. `Unchanged` is
an explicit request,not a conclusion inferred from a zero formula or a full Angle turn.

```rust
use sc_core::construction::*;
fn request_bridged_fullness(input:SlashSpreadConstruction)->SlashSpreadOutputIntent {
    SlashSpreadOutputIntent::new(input,SlashSpreadBoundaryPolicy::StraightBridges,
        SlashSpreadFullnessIntent::Added)
}
```

There is no Default or implicit input-to-output conversion. Every boundary/fullness combination
remains authored intent;its applicability depends on actual mode,amount,region and topology. A
negative rotation,zero distance or statically valid `1 mm/0` is preserved rather than normalized
into another policy or accepted geometric result. Original kind proofs remain attached to the
embedded opening document;they execute neither division nor a geometric transform.

`input()`,`boundary_policy()` and `fullness()` borrow actual owned fields. Input dependency views
continue to borrow that actual nested construction and its original cut/pivot/axis fields. No
output Piece,contour,reference retirement,canonical metadata reconciliation or command publication
is performed. Actual G2/G3 sources/results and the later `.5f.4.geometry` integration remain owned.

## Shared edge roles and slash operation binding

Edge-role catalogue and issuance-preserving transition implementation is shared by offset and
slash/spread declarations. The canonical names are `EdgeOutputRoleDefinition`,`EdgeOutputRoleCatalog`,
`AdmittedEdgeOutputRoleCatalog`,`ActiveEdgeOutputRole` and the corresponding catalogue/transition
policy,error and refusal types. These are edge-role metadata types;they introduce no PointRef role
namespace or geometric classifier. The previous `OffsetRole*`,`AdmittedOffsetRole*` and
`ActiveOffsetRole` names remain aliases of the same Rust types. Existing offset examples continue
to compile,without conversion,copied metadata or a second role-history implementation.

The implementation's algorithms and fields are preserved by the extraction. Debug type names and
Display prefixes now say edge-output roles rather than offset roles;typed variants and their
original field data are unchanged. Explicit count/key-byte admission,issued-definition preservation,
active membership and readonly evaluated-candidate presence retain their existing meanings.

`SlashSpreadOutputIntent::bind_edge_roles` requires the actual admitted shared catalogue and
explicit `EdgeOutputRoleCatalogLimits` for this association stage. Complete definitions,active ids
and original key bytes are checked in that order before operation identity. All definitions count,
including inactive ones. The association allocates no workspace;the catalogue's earlier admission
and owned storage retain their own policies. These entry/text allowances are not whole-execution
or allocation-byte guarantees.

```rust
use sc_core::construction::*;
fn bind_slash_roles<'a>(intent:&'a SlashSpreadOutputIntent,catalog:&'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits:EdgeOutputRoleCatalogLimits)->Result<BoundSlashSpreadEdgeRoles<'a>,SlashSpreadRoleBindingError<'a>> {
    intent.bind_edge_roles(catalog,limits)
}
```

Success and errors retain both actual complete owners and the selected policy. Operation mismatch
refuses;it cannot borrow a foreign operation's role catalogue. `BoundSlashSpreadEdgeRoles` exposes
its original `intent()`,`catalog()` and `limits()`. The catalogue's own active views still retain
actual active-list tag and definition fields in their original orders. Those views prove catalogue
membership;the complete binding supplies the checked slash-operation association.

For definitions `[9:moving,2:stationary,7:latent]` and active ids `[2,9]`,the original active order
remains2,9 while definitions remain9,2,7. Nothing renumbers tags or removes latent7. Empty initial
or fully inactive catalogues can also bind as metadata;they supply no executable contour or complete
output manifest. Changing membership still requires the existing issuance-preserving transition
checks and actual authoring/history authority.

A numeric operation match does not prove its actual ordered recipe kind,current Design membership,
global namespace,issuance or geometric continuity. Native source/transform/trim-selection admission
is described below;actual G2/G3 contours,effects,mappings and bus publication
retain their separate owners. No catalogue alias or binding creates a Piece or retires an edge.

## Native edge candidate provenance and admission

`SlashSpreadEdgeManifest` owns the required operation id and all original
`SlashSpreadEdgeCandidate`s. Each candidate has an explicit persistent tag and a discriminated
`SlashSpreadEdgeOrigin`. Raw construction retains every field without accepting a result or
issuing an identity. Candidate order is authored order,not a generated edge-name scheme.

| Origin | Explicit fields | Meaning |
| --- | --- | --- |
| `Source` | Positive original DirectedRange and Stationary/Moving motion | Retain that source portion with its declared traversal and operation-specific transform |
| `StraightBridge` | Lower/Upper cut end and bank-to-bank direction | Request a straight connector between corresponding stationary and moving cut-end occurrences |

`Source` preserves its own original edge frame,bounds and traversal. It can represent an explicitly
retained portion after trimming. `Moving` refers to the actual input's rotation or directed
separation;`Stationary` retains its original geometry. Neither label proves the region was classified
correctly or that the range belongs to the current Piece.

`Lower` and `Upper` name the original cut range's parameter bounds. They do not swap when cut
traversal is Reversed. `StationaryToMoving` and `MovingToStationary` select the connector's output
traversal explicitly. These fields create no PointRef,coordinate or nonzero-length certificate.
A rotation pivot can make a proposed connector degenerate;actual geometry must diagnose it or
supply the correct explicit result rather than invent an edge. No fixed connector count is assumed.

`admit` requires the actual `BoundSlashSpreadEdgeRoles` and explicit
`SlashSpreadEdgeManifestLimits`. It first checks complete2N workspace,N candidates,D definitions,
A active ids and original key bytes. Only then does it check nonempty input and matching operation.
Each original candidate follows in order:duplicate tag,defined role,active role,then local source
or boundary-policy cause. A same-operation source is a definite dependency cycle. A straight bridge
under `TrimOverlaps` refuses rather than changing the authored policy.

The workspace count is one N-entry temporary tag index plus N retained candidate/active-role pairs.
The catalogue's prior allocation and whole storage/memory/geometry/query-batch resources retain their
own admission. Limits bound complete populations,never an accepted prefix. Empty input refuses
through ordinary `admit`;a role catalogue's valid empty state does not automatically admit a manifest.
[Explicit no-new-edge admission](drafting-slash-piece-contracts.md#explicit-no-new-edge-admission)
uses a separate population choice under the same resource and owner checks.

```rust
use sc_core::construction::*;
fn admit_slash_edges<'a>(manifest:&'a SlashSpreadEdgeManifest,bound:&'a BoundSlashSpreadEdgeRoles<'a>,
    limits:SlashSpreadEdgeManifestLimits)->Result<AdmittedSlashSpreadEdges<'a>,SlashSpreadEdgeManifestError<'a>> {
    manifest.admit(bound,limits)
}
```

Errors retain the actual whole original manifest,bound intent/catalogue,selected policy and first
typed cause. Quota,empty and operation failures precede candidate inspection. Duplicate/undefined/
inactive/self-source/policy failures report actual original candidate positions. Nothing sorts,
renumbers,deduplicates,changes membership or returns a partially accepted list.

`AdmittedSlashSpreadEdges::outputs()` returns private views pairing original candidate fields with
actual catalogue active-list/definition fields in candidate order. Each retains its whole admitted
owner and original index. `logical_output()` uses the explicit operation/tag,not that index. For
candidate tags9,2 and catalogue active order2,9,both orders remain intact. Reverse iteration yields
2,9 without changing their identities.

Different explicit tags with equal source provenance remain distinct role requests. Such metadata
admission is not proof that coincident output geometry is appropriate. A manifest containing only
tag9 leaves active tag2 and inactive issued definitions intact;result absence never retires them.
Actual role matching remains G2/G3 work and must not infer continuity from equal keys or coordinates.

This component admits edge-role/provenance declarations only. It does not establish a complete
boundary/hole loop,trim intersection choice,source ownership/frame,current reference coverage or
retirement,metadata/copy/semantic reconciliation,measured fullness or publication authority. Those
native reconciliation requirements are described below and in the Piece companion. Actual curves,
closure,reference maps and approximation/parameter budgets still require G2/G3 and `.5f.3`/`.5f.4.geometry`.
No ledger edit,mapped Param or accepted geometric prefix is supplied by this admission.

## Native contour uses and retained source identities

`SlashSpreadContours` owns ordered boundary,hole and construction lists. Every
`SlashSpreadContourEdge` explicitly selects an admitted `Output { tag,direction }` or a
`Retained(DirectedEdge)`. Retention preserves the original EdgeRef;an unchanged source does not
need a fresh identity merely to appear in the result. Partial trimmed sources use explicit new
candidate provenance rather than pretending that part of an old edge is its whole identity.

A use's Direction describes contour traversal in that edge's own parameter frame. For a new
output,this is distinct from the source-range traversal or bridge direction that defined the
candidate's provenance. For a retained edge,it can request opposite contour traversal without
renaming the original edge or executing a topology reversal.

`admit` requires the actual admitted candidates,actual immutable source `Piece` and explicit
`SlashSpreadContourLimits`. The source id must match the slash input's original target. This is
source association,not current Design/revision/operation-kind or geometric ownership authority.
Retained references must occur exactly in that source Piece's original held edge inventory;
a current-child alias is not silently substituted. No particular duplicate source occurrence is
selected:membership is by stable identity and the complete original source remains available.

Whole target/source hole-group quotas precede traversal of those groups. Checked complete counts
then admit N candidates,R source occurrences and M target uses. Source and target construction
repetitions count. With K target cut uses,the conservative peak entry bound is2N+R+K+M:the candidate index,
used-output set,source identity set,cut identity index and retained occurrence associations.
These are entry counts,not prior Vec/String allocation,whole bytes or whole execution guarantees.
Source identity repetition can make its set smaller than R;R still counts every original source
occurrence. No index or association allocation precedes complete quota admission.

After quotas,source identity and nonempty boundary,the complete source inventory must contain no
same-operation output dependency. Target checks follow boundary order,hole order and construction
order. Every hole must be nonempty. Output tags must name actual admitted candidates;retained
references cannot name this same operation or a foreign source edge. No cut-edge identity can
repeat across boundary and holes,even with opposite traversal. Construction uses may repeat cut
edges or one another,matching the existing `Piece` structural rules. Finally,every original candidate
must have at least one declared use;the first unused original candidate refuses rather than vanishes.

```rust
use sc_core::{construction::*,ontology::Piece};
fn admit_slash_contours<'a>(contours:&'a SlashSpreadContours,edges:&'a AdmittedSlashSpreadEdges<'a>,
    source:&'a Piece,limits:&'a SlashSpreadContourLimits)
    ->Result<AdmittedSlashSpreadContours<'a>,SlashSpreadContourError<'a>> {
    contours.admit(edges,source,limits)
}
```

Private `SlashSpreadContourUse` views retain the actual raw occurrence,its original
`SlashSpreadContourLocation` and either the authentic candidate view or actual retained field in
the contour declaration. `edge()` returns the explicit logical output reference or unchanged
retained identity. `direction()` retains authored traversal. Iteration visits boundary,each hole,
then construction without sorting or deduplication;reverse iteration preserves those same identities.
Errors retain the entire original plan,candidate proof,provided source,policy and typed cause.
The contour policy is borrowed unchanged on both success and refusal;keep that policy owner alive
with the other inputs. No error-path boxing or copied five-field policy is needed. A repeated-cut
refusal retains both original positions;`error.repeated_edge()` reads that identity from the actual
first occurrence instead of duplicating it in the refusal payload.

For example,a boundary can retain an old source edge and use output9 in reverse,a hole can name
output2,and construction can explicitly use9 twice plus an original construction edge. All six
occurrences remain distinct;both new candidates are used and the retained ids stay unchanged.
With N2,R2,K3,M6,the peak population is15 entries. This example is structural only:a one-edge
hole still needs real endpoint closure,shape,winding and containment proof from G2.

This stage publishes no replacement Piece,retirement,reference map,fold/copy/metadata change or
measured fullness. Missing original source portions and dependent semantic references still require
explicit reconciliation. Actual trim selection,closure,current sources,geometry/error bounds and
bus authority remain separately owned. Explicit no-new-edge admission is described in the
[Piece companion](drafting-slash-piece-contracts.md#explicit-no-new-edge-admission);source-preserving
[outcome choice](drafting-slash-piece-contracts.md#explicit-source-preserving-or-reconciled-outcome)
checks source preservation separately. The ordinary nonempty primitive never needs fabricated geometry.

## Native declared-source reference eligibility

`AdmittedSlashSpreadContours::query_source_reference` classifies declared eligibility for an
original `EdgeAnchor`. It is a readonly metadata query,not a journal fold,actual geometry result
or permission to repoint a semantic reference. The exact provided anchor is owned unchanged by
success or error;the complete contour/source context and caller-owned policy remain borrowed.

`SlashSpreadReferenceQueryLimits` explicitly bounds complete N candidates,R original source
occurrences and M contour occurrences. All quotas and checked2N+1 potential match slots precede
source/frame scans. The contour proof records actual R at admission,including duplicate source
construction uses;queries check it without rebuilding or undercounting a deduplicated index.
There is no query-path allocation. Repeated calls/iteration and whole query batches keep their
own caller work budget;these counts do not certify whole storage or geometry execution resources.

Source frames and scope are explicit:

| Original frame | Admitted query scope |
| --- | --- |
| An edge held by the actual original source Piece | Its whole normalized unit interval |
| An external input cut edge | Its declared selected cut interval |
| An external candidate source edge | Union of its declared source intervals |
| An alias or any undeclared source identity | Typed SourceEdge refusal |

Unknown source identity precedes outside-scope refusal. A known external frame queried outside
all its declared domains returns `SourcePosition`. An uncovered point inside valid scope returns
a `Gap` query. Membership uses exact original edge ids and normalized arclength parameters;
Reversed traversal changes neither interval bounds nor original Lower/Upper cut-end names.
No current-child alias is silently followed and no registered source existence is proved by a name.

```rust
use sc_core::{construction::*,ontology::EdgeAnchor};
fn query_slash_source<'a>(contours:&'a AdmittedSlashSpreadContours<'a>,anchor:EdgeAnchor,
    limits:&'a SlashSpreadReferenceQueryLimits)
    ->Result<SlashSpreadReferenceQuery<'a>,SlashSpreadReferenceQueryError<'a>> {
    contours.query_source_reference(anchor,limits)
}
```

`matches()` returns the retained original identity once,then candidate matches in original edge
manifest order. A positive `Source` domain contributes its authentic candidate view when it contains
the query. A straight bridge at the exact queried original cut bound contributes two bank endpoint
occurrences,one Stationary and one Moving,in that bridge's declared Start/End order. These are
`SlashSpreadBridgeEndpoint` declarations,not computed output Params or proof of a nonzero bridge.
Both directions are explicit:StationaryToMoving starts at Stationary;MovingToStationary starts at Moving.

A source candidate used in several contours still contributes one candidate identity. Multiple
uses of a retained edge likewise contribute one retained identity,while `match.retained_uses()`
returns every original retained occurrence in its authored order. No first occurrence substitutes
for that evidence. Different candidate tags remain distinct even with equal source data;coordinate,
name or numeric-amount equality does not authorize coalescing. Both endpoints of one bridge share
an EdgeRef but remain different eligibility occurrences;edge identity alone is not a match key.

`kind()` counts all matches as `Unique`,`Ambiguous` or `Gap`. For a retained original source,an
output9 using that source,and output2 bridging its upper cut end:

| Query | Declared matches | Classification |
| --- | --- | --- |
| Interior of the first source | Retained original,source candidate9 | Ambiguous |
| Exact upper cut bound | Retained original,source9,bridge2 Start and End | Ambiguous |
| Repeatedly retained second source | One retained identity with all occurrence evidence | Unique |
| Original second source omitted from the result | none | Gap |

These are declared reference alternatives,not physical point-equivalence or current-liveness
certificates. Coincident bridge/fragment endpoints are not silently equated. A later real ledger
split leaves the original metadata query inspectable;querying its new child as an undeclared alias
refuses. Gap/ambiguity creates no ledger edit,canonical role retirement or repair discharge.

Private match views retain their actual query and original candidate fields. Reverse iteration
reverses occurrence order without changing tags or bank/endpoint declarations. The maximum2N+1
slots are a conservative bound,not an observed count or allocation footprint. Actual source maps,
current resolution,trim coverage/retirement/visible repair,semantic ownership and atomic publication
still need actual G2/G3/Design/bus evidence before use;native disposition requirements follow below.

## Global topology,registration repair and semantic ownership

The source-eligibility query does not authorize disposition. The published native ledger has
separate scopes that a local slash plan must retain:

| Public action | Actual scope | Additional obligation |
| --- | --- | --- |
| `IdentityLedger::delete` | Remove a global live edge and append its edit;every held reference folds through it | Whole affected-source/consumer evidence and actual command authority |
| `retire(owner,edge,param)` | Withdraw that owner's exact registration | Legitimate holder withdrawal and coordinated semantic state;this is not edge deletion |
| `repoint(owner,old_edge,old_param,new_edge,new_param)` | Change that owner's exact registration to an explicitly supplied live target | Actual correspondence and coordinated held-object repair |
| Replace a Piece contour while keeping old global edges | Change the proposed Piece's membership | Current semantic anchors must still belong to their intended Piece |

A focused public-interface case registers two owners at the same original source point. Deleting
the edge leaves both registrations unchanged and derives two repairs. Explicitly repointing one
owner leaves the other's repair open. Withdrawing the second registration removes that registration's
repair;the deleted source edge stays deleted. Registration repair appends no topology edit and
silently fixes no other owner. The selected new parameter in this test is caller data,not a computed
slash correspondence or permission to choose a point in production.

The other case constructs two actual structural Pieces that share one original edge and gives
each a born-valid notch on it. This is accepted by the scoped Piece inventory. A detached replacement
for the first Piece uses a newly declared edge while the other Piece keeps the original. The
original source remains live,both ledger point resolutions succeed and the ledger's narrow
`release_readiness` reports Releasable. The old notch definition nevertheless fails public anchor
membership validation against the proposed replacement with `AnchorOutsidePiece`.

That membership check uses the public birth validator solely as a diagnostic;rebuilding a held
object is not offered as its current-source repair procedure. The original objects remain unchanged.
Even explicitly repointing the first registration leaves its immutable notch definition holding the
old anchor;the other owner's original registration remains intact. A registration edit and a
canonical semantic-object replacement must be coordinated by their actual authoring/repair authority.

This is expected primitive behavior,not a discovered ledger defect. The ledger's point verdict
has the [documented narrower scope](../spec/ontology-implementation.md#executable-ontology-at-g1)
than complete Piece/garment/current-source validation. A stable or globally live EdgeRef alone
cannot establish that the intended Piece still owns its semantic position.

The native disposition plan therefore must state whole source coverage and the intended treatment
of global geometry,held owner references and visible repairs separately. It cannot infer global
retirement from a source's absence in one Piece result,withdraw a consumer registration to hide
loss,or turn `Unique` eligibility into an unproved target Param. Shared source geometry and other
owners remain explicit dependencies. Actual maps,trim selection,current semantic/registry checks,
identity history and atomic publication keep their G2/G3/Design/bus owners. No command or runtime
algorithm changes in this contract-recovery slice.

## Complete declared source-identity intent

`SlashSpreadSourcePlan` owns an ordered list of `SlashSpreadSourceDisposition`s. Each request
names an exact original EdgeRef and an explicit `SlashSpreadGlobalSourceIntent`:

| Intent | Required meaning |
| --- | --- |
| `Preserve` | Keep the global source;reconcile local Piece membership and held semantic references separately |
| `RequestRetirementReview` | Require whole affected-source/consumer evidence and actual command authority before any global retirement |

There is no inferred intent for a source omitted from the proposed contours. A retirement-review
request grants no edit permission,withdraws no registration and changes no held object. Other
Pieces may share the source. Even a Preserve request cannot prove that a notch still belongs to
the proposed Piece. Actual global treatment remains a later authorized publication decision.

`plan.bind(&contours,&limits)` requires the actual admitted contour/source context and borrowed
`SlashSpreadSourcePlanLimits::new(N,R,M,P)`:complete candidates,original Piece occurrences,target
contour occurrences and authored requests. It counts conservative `R+1+N+P` temporary index entries
and checks all four quotas before allocating indexes or scanning source/request fields. The source
identity index has at most R Piece occurrences,one cut and N candidate sources;the request index
has at most P entries. These bounds describe entries,not allocation bytes or a global consumer census.

Every distinct identity from the original Piece edges,the input cut and every candidate Source
range must have exactly one explicit request. External cuts and provenance sources are included
even when absent from the Piece. Original repeated construction occurrences need no duplicate
request. This is complete **identity coverage**,not whole interval coverage or source existence.
A declaration may still name an external source whose current liveness has not been checked.

```rust
use sc_core::construction::*;
fn bind_source_intent<'a>(plan:&'a SlashSpreadSourcePlan,contours:&'a AdmittedSlashSpreadContours<'a>,
    policy:&'a SlashSpreadSourcePlanLimits)
    ->Result<BoundSlashSpreadSourcePlan<'a>,SlashSpreadSourcePlanError<'a>> {
    plan.bind(contours,policy)
}
```

After whole-resource checks,binding inspects requests in authored order:unknown exact source,
duplicate source,then retirement review of an identity retained by a target contour. That last
request contradicts the proposed retained identity and refuses. It then reports the first missing
identity in original Piece,cut,candidate order. Aliases are not followed. Errors retain the actual
whole plan,contours and caller-owned policy;no accepted prefix escapes.

`BoundSlashSpreadSourcePlan::sources()` preserves authored request order. Private request views
retain their actual parent,index and original fields. `piece_uses()` returns every original Piece
occurrence and direction;`cut()` borrows the actual cut field when applicable;`derived_outputs()`
returns all authentic Source-provenance candidates;`retained_uses()` returns every original retained
contour occurrence. No occurrence is silently selected as an owner or correspondence. Repeated
output uses do not invent extra candidates,and distinct declared source tags are not coalesced.

For the earlier example,the first source and retained construction edge each need a Preserve
request. If the cut instead names external edge55 and candidate9 derives from external edge56,
those two identities need their own explicit requests too. A complete four-source plan can bind
without proving either external edge live. It issues no outputs and appends no ledger entry.

Consumer/registration/held-object evidence requests are described below;trim interval/selection
requirements are described in the boundary section below. Actual geometry,maps,current whole-source and owner censuses,
semantic replacement,authority,identity history and atomic publication retain their G2/G3/Design/bus
owners. This bound declaration supplies none of those certificates.

## Complete actual registered-source requests

`SlashSpreadRegistrationPlan` owns ordered `SlashSpreadRegistrationDisposition` requests. Each
request holds the exact original owner and EdgeAnchor plus an explicit `SlashSpreadRegistrationIntent`:

| Intent | Remaining obligation |
| --- | --- |
| `KeepOriginalGlobalSource` | Keep the global source and prove current held-object ownership/membership separately |
| `RequireCorrespondenceOrVisibleRepair` | Supply actual correspondence or retain an explicit visible repair;metadata uniqueness selects no target |
| `RequestWithdrawalReview` | Establish legitimate holder withdrawal,coordinated semantic state and actual authority before any registration removal |

A withdrawal-review request never calls `retire` or discharges a repair. A correspondence request
contains no guessed output Param. Keeping a global source does not prove that it remains in a
particular replacement Piece. The request owner is an actual registration key here,not a claim
that this owner belongs to the edited Piece or has authorized the requested action.

`plan.bind(&sources,&ledger,&limits)` requires the actual complete bound source plan,an immutable
IdentityLedger and borrowed `SlashSpreadRegistrationPlanLimits::new(S,L,K)`. S counts all bound
source identities,L counts **every** actual ledger registration including unrelated edges,and K
counts all authored requests. `IdentityLedger::registration_count()` exposes that actual complete
population without folding references or scanning registration fields. Checked `S+L+2K` workspace
and all quotas precede indexing or request-field checks.

The temporary indexes contain S source views,at most L source registrations and at most K request
keys;K authentic source/registration pairs are retained. The bound is in entries,not allocation
bytes. An unrelated ledger registration consumes L allowance even though it needs no request in
this source scope. Earlier declarations and whole-operation resource policy remain separate.

```rust
use sc_core::{construction::*,ontology::IdentityLedger};
fn bind_registered_sources<'a>(plan:&'a SlashSpreadRegistrationPlan,sources:&'a BoundSlashSpreadSourcePlan<'a>,
    ledger:&'a IdentityLedger,policy:&'a SlashSpreadRegistrationPlanLimits)
    ->Result<BoundSlashSpreadRegistrationPlan<'a>,SlashSpreadRegistrationPlanError<'a>> {
    plan.bind(sources,ledger,policy)
}
```

Every actual registration whose exact stored EdgeRef is in the bound source plan needs exactly
one request,including registrations held by other owners and those currently unresolved. An
empty request list binds when this actual registered-source subset is empty. This grants no
claim that all held structural or semantic references have been registered.

After whole-resource checks,binding examines requests in original order:unknown source,unknown
exact owner/edge/Param registration,duplicate exact key,then KeepOriginalGlobalSource conflicting
with that source's RequestRetirementReview intent. It then returns the first uncovered source
registration in actual ledger order. Different owners at the same source point are separate keys.
Aliases and other parameters are not substituted. Every refusal keeps the actual complete plans,
ledger and caller-owned policy;no accepted prefix escapes.

Private `SlashSpreadRegisteredSourceRequest` views preserve authored request order and fields,
the actual source request and the actual registration value including its unchanged split choice.
The parent retains an immutable borrow of that ledger. Two owners at a later split therefore keep
their distinct explicit/absent choices. The plan does not resolve the point,select a split side or
transfer another owner's choice. It makes no current-liveness certificate.

If a later global deletion leaves both owners unresolved,complete correspondence/withdrawal-review
requests can still bind:the two original repairs remain open and both registrations remain present.
No journal entry is appended. These are requests about the actual current registration inventory,
not evidence that the proposed treatment has happened or that publication is ready.

Whole held-object/source-consumer coverage beyond registrations,current Design/Piece/garment
ownership,proved maps,visible repair reporting,semantic replacement and actual command authority
remain explicit requirements in the consumer contract below and later G2/G3/Design/bus work.
A registered-source plan alone cannot authorize global retirement or hide an unregistered holder.

## Whole held-consumer evidence requests

`SlashSpreadConsumerEvidenceSources::new` requires five explicit evidence-source identities.
`registered_plan.require_consumer_evidence(&sources)` binds them to the actual complete registered
source plan,which retains its original source/contour/ledger context. This constructs required
evidence requests;it accepts no provider evidence and returns no execution-readiness verdict.

| Evidence kind | Required scope |
| --- | --- |
| `WholeSourceConsumers` | Whole affected-source use across actual structural and semantic holders,other Pieces and physical copies,including unregistered holders |
| `CurrentOwnership` | Actual current Design/garment/Piece/frame/liveness and intended semantic-position ownership |
| `Correspondence` | Actual point and interval correspondence,with explicit handling of ambiguous alternatives and applicable error evidence |
| `VisibleRepairs` | Complete visible gaps,ambiguities and repairs;legitimate coordinated withdrawal must have evidence |
| `AuthoringAuthority` | Actual authority for global edits,selected-holder repairs and canonical semantic-object replacement |

All five are mandatory even when every source intent is Preserve or the registered-source subset
is empty. Zero registrations do not prove zero held consumers. One provider/source identity may
serve several scopes,but equal identities never collapse obligations. Names are original authored
request fields;no registry lookup,currentness or provider availability follows from them.

```rust
use sc_core::construction::*;
fn require_whole_consumers<'a>(registered:&'a BoundSlashSpreadRegistrationPlan<'a>,
    sources:&'a SlashSpreadConsumerEvidenceSources)->RequiredSlashSpreadConsumerEvidence<'a> {
    registered.require_consumer_evidence(sources)
}
```

`RequiredSlashSpreadConsumerEvidence::requirements()` always exposes five distinct private
views,in whole-consumer,ownership,correspondence,visible-repair,authority order. Each retains its
actual parent,kind and original requested source field. `registration_plan()` and
`evidence_sources()` expose the authentic whole contexts. This fixed traversal allocates nothing
and performs no consumer scan or ledger fold;the earlier registration/source admission budgets
keep their original scope. Actual whole-consumer discovery and evidence validation need their own
complete execution resource policy.

For the two unresolved registered owners,requesting all five evidence sources leaves both
registrations and both repairs unchanged. Reusing one provider identity for three scopes still
produces all five obligations. For an empty registered-source subset,all five requests remain.
Neither case authorizes retirement,selects an output parameter or withdraws a held reference.

This completes the native consumer **requirement declaration**,with focused checks. Actual whole
consumer evidence,current semantic ownership,maps,repair visibility and command authority remain
G2/G3/Design/bus work. Trim/whole interval-coverage requests follow below;Piece metadata/fold/copy/effect
and explicit zero/no-new-edge outcome requirements are in the Piece companion. Full verification
and later geometry-backed integration have not been performed by these declaration slices.

## Boundary selection and whole interval-coverage requests

`SlashSpreadBoundaryRequirement::new(supported_domain,source_coverage,boundary)` requires explicit
evidence-source identities for the supported motion/region/pivot-or-axis/topology domain and whole
source-interval coverage. Identity coverage in the source plan supplies neither proof. Whole
interval coverage must account for retained,derived and omitted/trimmed domains and bank provenance;
it cannot inspect only contour endpoints or silently discard a lost interior.

`SlashSpreadBoundaryEvidenceRequest` requires the policy-specific source fields:

| Variant | Required actual evidence |
| --- | --- |
| `StraightBridges { incidence,closure }` | Corresponding opened-mouth endpoints for every declared bridge;complete boundary/hole closure and supported unique topology |
| `TrimOverlaps { arrangement,unique_selection }` | Complete bank intersections,retained ranges and overlap arrangement;unique selected boundary/holes matching the actual declared contours |

A source name supplies no geometry or provider existence. A trim request names no arbitrary loop
index and chooses no ambiguous branch. G2/G3 must refuse unsupported or ambiguous region/topology,
prove the unique supported selection and justify all retained ranges/intersections. Actual point
correspondence and applicable error evidence remain required by the whole consumer contract.

```rust
use sc_core::construction::*;
fn require_boundary<'a>(consumers:&'a RequiredSlashSpreadConsumerEvidence<'a>,
    request:&'a SlashSpreadBoundaryRequirement)
    ->Result<RequiredSlashSpreadBoundary<'a>,SlashSpreadBoundaryRequirementError<'a>> {
    consumers.require_boundary(request)
}
```

`require_boundary` compares the requested bridge/trim evidence mode with the actual original
output intent's boundary-policy field. A mismatch returns a typed error retaining the entire
original request and actual whole consumer/source/contour/ledger context;`required_policy()`
borrows that original field. A matching request returns a private `RequiredSlashSpreadBoundary`
with the same authentic owners. Fixed field comparison allocates nothing and scans no geometry,
source or consumer. It accepts no provider evidence and computes no intersection,loop or target Param.

All four requested scopes remain explicit even if their source identities are equal. With two
actual unresolved registrations,a matching trim/domain/coverage request leaves both repairs and
registrations intact,issues no output edges and appends no journal entry. An empty registry still
retains the whole held-consumer requirements. Earlier complete declaration/registration bounds
keep their own scope;actual arrangement/discovery/evidence work requires its own execution budget.

This closes the native source-reference **disposition requirement declaration** with focused
checks. Actual current source/consumer coverage,maps,trim selection,semantic repair,authority and
atomic publication remain separately owned G2/G3/Design/bus work. Piece metadata/fold/copy/effect
and outcome requirements are in the companion;full verification and later geometry-backed integration
remain deferred.

[Piece metadata,fold and copy reconciliation](drafting-slash-piece-contracts.md) continues this native declaration path.
