# Native truing output and source contracts

This companion follows the [constrained seam-truing input contract](drafting-edit-inputs.md#native-constrained-seam-truing-inputs).
That input chooses an adjusted side and pattern Piece,retains the actual span and selected copy,
and borrows an explicit sourced nonnegative T2 length-fit requirement. Its endpoint/stop locks
specify permissions,not a unique fitted curve. Actual fitting and complete variant coverage remain
separate obligations.

## Associate the actual complete copy and Piece inventory

`BoundConstrainedTruing::bind_copy_inventory` borrows the complete actual `PieceInventory` under
`TruingCopyInventoryLimits::new(C,I,W)`. It uses the exact CutPlan already retained by the actual
input source;no second supplied plan can replace that parent. `BoundTruingCopyInventory` retains
the actual source,plan,whole inventory and original explicit limits.

```rust
use sc_core::{construction::*,ontology::PieceInventory};
fn bind<'a>(source:&'a BoundConstrainedTruing<'a>,inventory:&'a PieceInventory,
    limits:&'a TruingCopyInventoryLimits)
    ->Result<BoundTruingCopyInventory<'a>,TruingCopyInventoryError<'a>> {
    source.bind_copy_inventory(inventory,limits)
}
let limits=TruingCopyInventoryLimits::new(2,2,8);
assert_eq!(limits.workspace_entries(),8);
```

C counts every retained-plan copy;I every supplied Piece,including unrelated records. Checked
I+3C precedes whole C,I and workspace quotas,before scans or allocation. Canonical `CutPlan::new`
then validates copied definitions against this exact inventory,including complete quantities,
orientations and pair/fold policy. Its temporary proof is dropped before authentic A/B records are
cached. A plan's earlier birth against another Piece collection is not proof for this inventory.

The transient bound counts logical entries,not allocator bytes,prior immutable storage or a whole
execution budget. Association takes O((C+I)log(C+I+2)) work with constant cached side references.
It scans no nested geometry,registry or observers and solves no fit. Quota equality is accepted.
With C2/I2,the bound is8;with C2/I1,it is7. W7 refuses complete required8 for a two-Piece context
before any canonical quantity mismatch can be inspected.

After canonical validation,both original span sides are resolved in A/B order. `copy(side)` lends
the actual retained-plan physical copy;`piece(side)` lends its actual supplied pattern owner.
Neither owner is inferred from an edge id or array position. `selected_piece()` follows the
original explicit adjusted side;`counterpart_piece()` follows the other original side. These are
actual borrows,not equal-looking reconstructed Pieces. `source()` preserves the original graph,
span,direction,ease,stops,selected copy and quantitative provider,including its actual tolerance
reference and derivation. `plan()`, `inventory()`, `limits()` and `workspace_bound()` expose the
whole actual context and conservative temporary bound.

The input binding checks the selected copy only. If an otherwise canonical retained plan contains
A200 but omits the graph's B201,the new association refuses `MissingCopy { side:B, copy:201 }`.
Selecting B with A absent still refuses original A first. No partial side context escapes.
An unrelated supplied Piece102 without its required plan copy also causes canonical refusal;
filtering to the selected Piece would hide that obligation. Errors preserve the whole actual
source,plan,inventory,limits and first resource,canonical or original A/B cause.

## Shared pattern ownership is a coupled refit

Two physical copies may share actual Piece100,including authored A200 and reflected B201.
The retained plan keeps its original order,for example `[201,200]`;the A/B cache still follows
actual source identities. `same_pattern_piece()` reports shared ownership,not an independent
counterpart or geometric fit. Both selected/counterpart getters then lend the same actual Piece.
The original `affected_copies()` inventory still returns both physical copies.

A refit edits the pattern Piece,so every physical copy and any other observing seam,mark or
constraint may be affected. Shared ownership cannot silently freeze the counterpart,fork a copy,
move endpoints/stops or replace ease. Complete current Design/graph/plan/ledger ownership,
held-reference repair,observer/all-copy effects,deterministic output/curve policy,actual fit and
length-error evidence,authoring authority and atomic publication remain required. This stage
constructs no output topology and certifies no endpoint,stop,geometry or physical result.

## Bind explicit edge-output roles

`BoundTruingCopyInventory::bind_edge_roles` associates the actual complete source/copy/Piece
context with an `AdmittedEdgeOutputRoleCatalog`. It produces `BoundTruingEdgeRoles` under explicit
`EdgeOutputRoleCatalogLimits`. These are edge-role declarations;Point and semantic-Notch
catalogues are different types and cannot supply this binding.

```rust
use sc_core::{construction::*,name::MachineToken,ontology::{EntityId,LocalTag}};
fn bind<'a>(copies:&'a BoundTruingCopyInventory<'a>,catalog:&'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits:EdgeOutputRoleCatalogLimits)->Result<BoundTruingEdgeRoles<'a>,TruingEdgeRoleBindingError<'a>> {
    copies.bind_edge_roles(catalog,limits)
}
let raw=EdgeOutputRoleCatalog::new(EntityId::from_bits(1000),
    [(9,"refit"),(2,"retained"),(7,"inactive")].into_iter().map(|(tag,key)|
        EdgeOutputRoleDefinition::new(LocalTag::new(tag),MachineToken::new(key).expect("literal key"))
    ).collect(),vec![LocalTag::new(2),LocalTag::new(9)]);
let catalog=raw.admit(EdgeOutputRoleCatalogLimits::new(3,2,21)).expect("bounded literal metadata");
assert_eq!(catalog.key_bytes(),21);
```

The whole D-definition,A-active-occurrence and B-key-byte quotas precede operation equality.
D includes inactive definitions;A counts every original active occurrence;B is the catalogue's
complete cached original machine-key byte count. A valid catalogue for operation1001 still
refuses binding to actual truing operation1000. D2 refuses required3 before A1,B20 or operation
inspection;with D3/A2,B20 refuses required21 before that owner mismatch.

Association reads cached counts and original operation fields only. It takes constant work and
space,scans no keys,allocates nothing and mutates neither original input. Earlier catalogue
admission and owned storage retain their separate full-population/workspace obligations.
`copies()`, `catalog()` and `limits()` lend the actual complete parents and original explicit
policy. Catalogue definition/active order,machine keys,local tags and authentic member borrows
stay unchanged. Errors retain the same whole copy/Piece/source and catalogue context,policy and
first complete quota or operation cause;no accepted prefix escapes.

Empty metadata and a catalogue containing only inactive definitions can both bind. They do not
prove an empty refit result or erase either physical copy,shared-Piece coupling,stops or the actual
fit provider. Active membership likewise supplies no evaluated curve or issued identity. Complete
output manifests,reference/endpoint/stop mappings,deterministic variant/refit policy,actual history,
geometry/length-fit evidence,current Design,all-copy effects and atomic authoring authority remain
required. A caller-supplied role tag cannot grant topology issuance or canonical publication.

## Original candidate roles and source ranges

`TruingEdgeManifest` owns an explicit operation and ordered `TruingEdgeCandidate` collection.
Each candidate retains an original local output tag and positive held `EdgeRange`. The range's
edge and exact from/to parameters are metadata,not a curve fit or a geometric parameter map.
`admit` binds this whole proposal to actual `BoundTruingEdgeRoles` under
`TruingEdgeManifestLimits::new(N,W)`,producing `AdmittedTruingEdges`.

```rust
use sc_core::{construction::*,ontology::{EntityId,EdgeRange,LocalTag}};
fn proposal(source:EdgeRange)->TruingEdgeManifest {
    TruingEdgeManifest::new(EntityId::from_bits(1000),vec![
        TruingEdgeCandidate::new(LocalTag::new(9),source),
        TruingEdgeCandidate::new(LocalTag::new(2),source),
    ])
}
fn admit<'a>(raw:&'a TruingEdgeManifest,roles:&'a BoundTruingEdgeRoles<'a>)
    ->Result<AdmittedTruingEdges<'a>,TruingEdgeManifestError<'a>> {
    raw.admit(roles,TruingEdgeManifestLimits::new(2,4))
}
```

Checked2N,whole candidate and workspace quotas,and actual original operation equality precede
candidate scans/index allocation. Original candidate order checks duplicate tag,undefined role,
inactive role,self-supplied edge creator,then actual selected/counterpart Piece and assigned-Material
creator aliases. A PointRef cannot substitute for the required EdgeRange. The known Piece/Material
checks inspect those two actual cached owners only;they are not a complete namespace census.
No registry lookup,geometry evaluation or whole inventory scan occurs here.

A tag index and original association list use2N new logical entries,not allocator bytes or earlier
owned storage. Work is O(N log(N+D+2)),for D admitted catalogue definitions. Repeated source ranges
with distinct roles remain separate proposals. For roles9/2 both naming source interval1/4–3/4,
admission retains both original ranges and their order. It proves no coverage or admissible sharing
of those overlapping attributions. Missing active catalogue roles are not automatically retired.

`manifest()`, `roles()` and `limits()` retain the actual whole original parents and policy.
`edges()` traverses original candidate order in both directions;`edge(tag)` queries in
O(log(N+2)) work. Each authentic `TruingOutputEdge` lends its complete `owner()`,original
`index()`,actual `candidate()`,active catalogue `role()` and actual original `source()` range.
`logical_output()` declares the actual operation/tag EdgeRef tuple;it neither mints topology nor
authorizes reuse or publication. No accepted prefix escapes any quota/owner/original-entry refusal.

An explicitly empty manifest can admit with N0/W0 as presence metadata while active roles remain
in the same actual catalogue. This does not certify an empty refit,no-op or retired entity/role.
An unlisted creator555 can likewise remain an attributed range without source-existence or selected-
domain proof. Mandatory later domain/coverage checks must refuse unsuitable sources before any
complete result can be accepted. Original source owners,span/ease/stops,quantitative fit provider,
role metadata and raw candidates remain unchanged on success/refusal.

Source-domain coverage,explicit refit/no-op semantics,endpoint/stop/reference mappings,deterministic
curve/refit variant policy and actual current geometry remain required. None follows from positive
source parameters,complete candidate admission,an active output role or a declared logical tuple.
Actual current Design/history,all-copy/observer effects,authoring authority and atomic publication
retain their separate execution owners.

## Bind the original selected held source domain

`AdmittedTruingEdges::bind_source_domain` checks every original candidate against the actual range
field of the selected original span side. The caller supplies only
`TruingSourceDomainLimits::new(N)`,not a replacement domain. The result is
`BoundTruingSourceDomain`,which retains the complete original candidates and exact selected field.

```rust
use sc_core::construction::*;
fn bind<'a>(edges:&'a AdmittedTruingEdges<'a>,maximum:usize)
    ->Result<BoundTruingSourceDomain<'a>,TruingSourceDomainError<'a>> {
    edges.bind_source_domain(TruingSourceDomainLimits::new(maximum))
}
let limits=TruingSourceDomainLimits::new(2);
assert_eq!(limits.candidates(),2);
```

The whole N quota precedes any range/parameter inspection. Original candidate order checks exact
edge creator **and tag**,then lower bound,then upper bound. `Param` ordering uses the canonical
bounded exact rational comparison;there is no epsilon,rescaling or coordinate/length calculation.
Binding takes O(N) work and constant auxiliary space,with no allocation or ledger/geometry query.
Equality at either selected interval endpoint is accepted for a positive candidate range.

For selected domain1/4–3/4,the same edge's1/4–1/2,1/2–3/4 and whole1/4–3/4 intervals pass.
A0–1 candidate refuses `BeforeStart` first,even though its upper bound also lies outside. A1/2–7/8
candidate refuses `AfterEnd`. The counterpart's edge refuses `Edge` even when both edges share the
same creator;their persistent tags differ. A later invalid candidate returns its original index
and no prefix domain context. Whole quota failure precedes any of these field causes.

`candidates()`, `domain()` and `limits()` retain the actual complete parents,original selected
range and explicit policy. `edge_count()` counts all original candidates. `edges()` lends authentic
members in original order and reverse;`edge(tag)` queries in O(log(N+2)) work. A member retains
its actual whole `context()`,original `candidate()`,actual `source()` range and original `domain()`
field. Errors lend the same whole original candidate/source context,domain,policy and first cause.
Neither success nor refusal mutates or normalizes any original field.

This is membership in the **held original parameter domain**. It is not current ledger resolution,
current Piece membership,coverage,nonoverlap,an output parameter map,endpoint/stop preservation or
a fitted curve. Two equal middle3/8–5/8 candidates may both be members of1/4–3/4 while overlapping
and leaving gaps. N0 can likewise bind while the actual selected domain remains positive and roles
remain active;that proves no empty result,no-op or retirement. Mandatory coverage/refit-no-op,
reference/variant,currentness,geometry/all-copy and atomic execution obligations remain separate.

## Require an exact selected-source partition

`BoundTruingSourceDomain::bind_source_coverage` requires the original candidates to partition the
actual selected held domain exactly once. It returns `CoveredTruingSourceDomain` under explicit
`TruingSourceCoverageLimits::new(N,W)`. Membership is a prerequisite;the caller cannot supply a
different domain or bypass the original candidate/source parents.

```rust
use sc_core::construction::*;
fn cover<'a>(source:&'a BoundTruingSourceDomain<'a>)
    ->Result<CoveredTruingSourceDomain<'a>,TruingSourceCoverageError<'a>> {
    source.bind_source_coverage(TruingSourceCoverageLimits::new(2,2))
}
let limits=TruingSourceCoverageLimits::new(2,2);
assert_eq!((limits.candidates(),limits.workspace_entries()),(2,2));
```

Whole N quota,then whole W quota precede source inspection and index allocation. W counts one
additional entry per original candidate,including every repeated source. Earlier candidate/role
storage and allocation bytes remain separate. With two candidates,N1 refuses `Candidates` before
W0 or any discontinuity. N2/W1 refuses `Workspace`;N2/W2 admits the complete index.

The borrowed index sorts by exact source `from`,then `to`,then original candidate index. An exact
forward scan from the selected lower bound refuses the first positive `Gap` or `Overlap`. A gap
carries its exact from/to parameters and next original candidate index;`None` identifies a trailing
or whole-domain gap. An overlap carries both original indices and its exact intersection bounds.
Endpoint touches pass. Equal ranges are ordered by original index and refuse positive duplication.
No accepted prefix escapes. Work is O(N log N),with one N-entry index sorted in place;there is no
ledger/geometry query,parameter arithmetic,rescaling,rounding or epsilon.

For actual domain1/4–3/4,candidates declared as role9:1/2–3/4 and role2:1/4–1/2 pass.
`source_order()` lends original indices `[1,0]`,while `source_domain().edges()` retains `[0,1]`.
A1/4–3/8 then5/8–3/4 proposal refuses the exact3/8–5/8 gap. A1/4–5/8 then1/2–3/4
proposal refuses overlap1/2–5/8. An empty proposal refuses the whole positive selected domain;
empty membership never becomes complete coverage or a no-op. Even a positive discontinuity smaller
than floating-point resolution remains distinct under canonical exact rational ordering.

`source_domain()`, `domain()` and `limits()` preserve the actual complete membership context,
original selected range field and explicit policy. `workspace_entries()` reports the complete
additional index population. `source_order()` supports exact length and reverse iteration;members
retain their original membership context,candidate fields and active roles. Errors retain that
same whole actual membership and policy. Neither success nor refusal changes original fields/order.

This is a nonoverlapping partition of **held source parameters**,not a geometric output map or
traversal direction. It does not select a fitted curve,prove a refit/no-op,issue identities,preserve
endpoint/stop positions,resolve current references or repair observers. Explicit outcome/reference/
variant policy,actual fit and length evidence,current Design/history,all-copy effects,authoring
authority and atomic publication remain separately required.

## Require an explicit refit or source-preserving outcome

`BoundTruingSourceDomain::require_outcome` takes an explicit `TruingOutcomeIntent` and
`TruingSourceCoverageLimits`,returning `RequiredTruingOutcome`. Neither intent has a default.
The operation retains its original constrained-truing input,including both endpoint/stop locks,
span/ease,selected copy/pattern Piece and actual quantitative fit provider.

```rust
use sc_core::construction::*;
fn require<'a>(source:&'a BoundTruingSourceDomain<'a>,intent:TruingOutcomeIntent,
    limits:TruingSourceCoverageLimits)->Result<RequiredTruingOutcome<'a>,TruingOutcomeError<'a>> {
    source.require_outcome(intent,limits)
}
let preserve=TruingOutcomeIntent::PreserveSource;
assert_ne!(preserve,TruingOutcomeIntent::Refit);
```

`PreserveSource` explicitly requests the exact original Piece,copies,span,ease,stops,active roles
and identity history. It requires zero candidate edges and creates no source-order index. Empty
metadata alone never selects this mode. Existing active roles remain active;the request grants no
retirement or new logical output identity. Actual source preservation,fit and observer evidence
remain required even if the source is already thought to satisfy the seam constraint.

`Refit` requires the authentic complete exact selected-source partition from `bind_source_coverage`.
Its N/W quotas and first exact gap/overlap cause are retained. Empty candidates therefore refuse
the whole positive-domain gap;there is no fallback to preservation. A one-candidate whole-domain
partition can request refitting without proving that any physical change occurred.

The whole candidate quota precedes either branch. For one candidate,N0 refuses `Candidates`
before inspecting the requested intent. With N1,`PreserveSource` refuses `NewEdges { count:1 }`,
even if that candidate covers the whole domain. Preservation allocates no index,so W0 is sufficient
for its valid empty case. Refit with N1/W0 refuses the original `Coverage(Workspace)` cause.
Preservation checks take constant work/space;refitting retains the partition's bounded index/work.

`source_domain()`, `intent()` and `limits()` expose the actual complete membership parent and
original explicit request/policy. `coverage()` lends the authentic partition for Refit and returns
`None` only for explicit empty source preservation. Errors keep those same whole parents and the
first resource,preservation or original partition cause. Original candidates,roles and source fields
remain unchanged. These are required outcomes,not achieved no-op/refit,geometric map,deterministic
curve selection,currentness,all-copy observer or atomic publication certificates.
