# Native slash/spread Piece reconciliation

This companion continues the [output and reference-disposition contracts](drafting-slash-output-contracts.md).
Native declarations retain original owners and request evidence. They do not create a geometrically
valid replacement Piece,change canonical metadata or authorize publication. The nonempty changed
contour/copy/effect paths,explicit no-new-edge admission and source-preserving/reconciled outcome
requirements are available. The native contract checkpoint is below;actual geometry/current Design/
semantic repair stays G2/G3/Design/bus.

## Canonical source-metadata preservation

`SlashSpreadPieceMetadataRequirement::preserve_source(fold,fold_correspondence,all_copy_effects)`
explicitly requests preservation of the actual source Piece's identity,total cut quantity,mirroring
and companion metadata,cutting side,material assignment,assembly layer and authored label. It stores
no cloned metadata,second cut quantity or independent label-derived hand/fold authority. There is
no default preservation request. Unrelated authored metadata changes keep their canonical owners.

The two required evidence-source identities request actual fold correspondence and effects on
every physical-copy/semantic/sewing consumer. They are original caller fields,not provider lookup,
accepted evidence or a certificate that copies or consumers have been reconciled.

`required_boundary.preserve_piece_metadata(&request)` requires the actual complete boundary/
consumer/source/contour context. A successful private `BoundSlashSpreadPieceMetadata` borrows that
original source Piece and exposes its canonical metadata by reference:

| View | Authoritative original field |
| --- | --- |
| `piece_id()`, `quantity()`, `mirroring()` | Actual source identity,total cut copies and handedness/companion metadata |
| `cut_on_fold()`, `cutting_side()` | Actual source fold state and fabric cutting side |
| `material()`, `layer_index()`, `label()` | Exact source assignment/reason,layer and authored label text |
| `printed_label()` | Source label with quantity,L/R and fold indicators derived from the same canonical cut metadata |

Borrowing preserves exact unresolved material reason text and label spelling. A cut-four mirrored
source remains cut-four with paired metadata. No label text can silently replace that quantity or
handedness. `source_piece()` exposes the original owner;its old contours are context,not the proposed
output contours. The boundary owner retains the actual proposed contour declaration separately.

```rust
use sc_core::construction::*;
fn preserve_metadata<'a>(boundary:&'a RequiredSlashSpreadBoundary<'a>,
    request:&'a SlashSpreadPieceMetadataRequirement)
    ->Result<BoundSlashSpreadPieceMetadata<'a>,SlashSpreadPieceMetadataError<'a>> {
    boundary.preserve_piece_metadata(request)
}
```

## Explicit fold association

The request must explicitly choose a `SlashSpreadFoldIntent` matching the original cut-on-fold state:

| Intent | Structural association required |
| --- | --- |
| `NoFold` | Original Piece is not cut on fold;an existing fold cannot be removed through this preservation request |
| `RetainOriginal` | Original Piece is cut on fold and its exact original fold identity is retained on the proposed outer boundary |
| `Derived { tag }` | Original Piece is cut on fold;tag names an actual admitted Source-provenance candidate from its exact original fold,used on the proposed outer boundary |

A bridge or another source edge cannot become the original fold by occupying a boundary position.
A candidate used only in a hole or construction line cannot be the outer fold. Original traversal
and candidate source direction remain explicit;tag is persistent output identity,not a contour index.
A derived candidate may declare a source subrange;actual whole fold-domain coverage and physical
fold correspondence still require the requested evidence. Provenance alone cannot prove them.

Admission first checks fold-state agreement,then the required original fold field. RetainOriginal
then checks actual outer retained membership. Derived checks candidate existence,outer-boundary use
and exact original-fold source provenance,in that order. Typed errors retain the original complete
request and actual whole boundary context. No missing fold or substitute candidate is manufactured.

`source_fold()` borrows the original canonical fold field when present. `fold_output()` exposes the
authentic admitted candidate only for a derived fold. `fold_edge()` returns the declared retained
or new logical fold identity;that new identity is still unissued. No global source retirement or
physical fold truth follows from this association. The original Piece and ledger stay unchanged.

The scan uses the already admitted complete N candidates and M contour occurrences and allocates
nothing. Earlier source/contour bounds retain their scope;this grants no whole-operation CPU/memory
or currentness guarantee. Actual fold geometry,copy inventory/copy effects,current semantic ownership,
material registry,identity history and atomic publication remain separate obligations. Physical-copy
binding,effect and zero/no-new-edge outcome requirements follow below.

## Preserve the actual physical-copy inventory

`bound_metadata.preserve_physical_copies(&cut_plan,&limits)` requires the actual immutable CutPlan
and borrowed `SlashSpreadCopyLimits::new(C)`. C is the **complete** plan population,including
copies of unrelated Pieces. The complete quota is checked before scanning target copies or
reading their orientation/quantity association. It allocates nothing and generates no identities.

The scan selects copies by the actual inherited pattern Piece identity. Their total must equal
the authoritative source quantity. A MirroredPairs source requires equal authored/reflected
populations;Single and separately handed PairMember sources allow authored copies only. Copy
orientation is independent of source/output edge traversal and is not applied to geometry here.

The actual CutPlan was structurally validated against its birth Piece collection. That alone
cannot establish its association with this source metadata. The actual target population and
permitted orientations must be checked against the inherited source fields;incompatible populations
refuse. Current Design ownership,companion/namespace truth and revision binding remain separate checks.

```rust
use sc_core::{construction::*,ontology::CutPlan};
fn preserve_copies<'a>(metadata:&'a BoundSlashSpreadPieceMetadata<'a>,plan:&'a CutPlan,
    policy:&'a SlashSpreadCopyLimits)->Result<BoundSlashSpreadCopies<'a>,SlashSpreadCopyError<'a>> {
    metadata.preserve_physical_copies(plan,policy)
}
```

Refusal order is complete C quota,first unexpected target reflection in original plan order,
complete target quantity,then paired-population balance. Errors retain the actual complete
metadata/boundary/source context,CutPlan and caller-owned policy. No missing copy is manufactured
and no valid prefix is exposed. Counts are bounded by the actual C-entry slice;quantity conversion
refuses a differing population instead of narrowing it.

`BoundSlashSpreadCopies::copies()` borrows **every** actual target copy in original plan order.
Reverse iteration reverses that order while keeping ids,orientations and original definition
fields. `copy_count()` is the complete immutable admission summary,not another authored quantity.
`plan()` retains the whole inventory,including unrelated copies. No first copy or list position
substitutes for the complete set or supplies a new identity.

For a cut-four mirrored Piece,copies410/207/900/805 keep their individual ids and authored/reflected
orientations. Reordering those records changes iteration order,not identity or neighbour ownership.
An additional unrelated copy consumes C allowance while remaining outside this Piece's returned
set. A valid authored-only cut-four plan cannot satisfy inherited mirrored-pair metadata. A reflected
copy cannot satisfy Single or separately handed PairMember metadata merely by sharing its Piece id.

This native association preserves existing physical-copy identities;it reflects no geometry and
edits no sewing graph,copy definition,Piece or ledger. The explicit all-copy-effect evidence request
remains required for every actual copy and semantic/sewing consumer. Pattern edits affect their
shared Piece;they cannot silently select one physical copy or transfer its neighbours. Actual
current ownership,all-copy effects,fold geometry and publication authority remain G2/G3/Design/bus
proof. Attributed fullness/effect and explicit zero/no-new-edge outcome requests follow below.

## Attributed fullness and canonical intake requirements

`SlashSpreadEffectRequirement::new(derivation,attribution,canonical_intakes)` requires three explicit
evidence-source identities. Derivation must establish the actual effect and its supported observable/
domain;attribution must connect it to this operation's same original inputs;canonical-intake evidence
must reconcile affected Dart/Tuck/Pleat/Gather owners and their closing/conservation obligations.
These are requested sources,not supplied evidence,registry resolution or an achieved result.

`bound_copies.require_attributed_effect(&request)` binds them to the actual complete copy/metadata/
boundary/source context. It allocates nothing and performs no formula evaluation,geometry,closure,
copy selection or semantic edit. There is no independently authored Length,Area or intake field.
The original all-copy-effect evidence source remains on the canonical metadata requirement rather
than being entered again. Every actual admitted target copy and its semantic/sewing consumers remains
in scope;one selected copy cannot substitute for the complete inventory.

```rust
use sc_core::construction::*;
fn require_effect<'a>(copies:&'a BoundSlashSpreadCopies<'a>,request:&'a SlashSpreadEffectRequirement)
    ->RequiredSlashSpreadEffect<'a> {
    copies.require_attributed_effect(request)
}
```

The private `RequiredSlashSpreadEffect` exposes the actual original `output_intent()`,
`requested_fullness()`,spatial `input()` and embedded `opening()` by reference. It stores no duplicate
fullness policy or solved opening. `copies()` retains every actual target-copy identity;
`all_copy_effects_source()` borrows the original metadata request field. `requirement()` retains
all three original derivation/attribution/canonical-intake source fields. Shared provider identities
would not merge these scopes or certify the provider.

Added/Removed/Unchanged remain the original intended effect. No angle sign,separation direction,
zero formula or full turn establishes the actual result here. A Removed request neither closes a
canonical Dart nor substitutes for its conserved intake. Existing intake parameter/formula/profile
owners remain authoritative;actual closing operations must satisfy their own conservation checks.
A provider must justify the effect,not copy an independently entered amount into a successful report.

The actual cut-four copy case retains all four ids/orientations and the exact original Added/input/
opening fields after requesting effect evidence. Its Piece,copy inventory and ledger remain unchanged.
This completes only the native effect **requirement declaration**. Actual supported domain/effect,
canonical intake conservation,all-copy semantic/sewing effects,current ownership and atomic publication
keep their G2/G3/Design/bus owners. Explicit no-new-edge admission follows;source-preserving/reconciled
outcome choice follows below.

## Explicit no-new-edge admission

An explicit zero-candidate declaration needs no fabricated edge. Construct
`SlashSpreadEdgeManifest::new(operation,vec![])` and explicitly call
`admit_no_new_edges(&bound_roles,limits)`. Ordinary `admit` continues to require a nonempty candidate
list and returns Empty for that same raw manifest. Both entry points share the complete resource/
operation/candidate kernel;the population choice is explicit rather than a fallback.

The no-new-edge entry checks complete2N workspace,N/D/A/key-byte quotas first,then requires N0,
then requires the actual bound operation. CandidatesPresent refuses any supplied candidate without
discarding it. Quotas precede that population cause;population precedes the operation cause. Errors
retain the original complete manifest,bound owners and explicit policy. At N0,no candidate or index
entry is allocated and no identity is issued. Prior catalogue/storage and whole-operation resources
retain their existing scope.

```rust
use sc_core::construction::*;
fn admit_no_new_edges<'a>(manifest:&'a SlashSpreadEdgeManifest,roles:&'a BoundSlashSpreadEdgeRoles<'a>,
    policy:SlashSpreadEdgeManifestLimits)->Result<AdmittedSlashSpreadEdges<'a>,SlashSpreadEdgeManifestError<'a>> {
    manifest.admit_no_new_edges(roles,policy)
}
```

The authentic `AdmittedSlashSpreadEdges` has zero outputs and zero2N workspace,while retaining
its actual raw manifest and bound role/intent owners. Empty catalogues and catalogues with active
unused roles are both valid metadata. This admission deactivates no role,retires no canonical
identity and edits no previously issued edge. Historical output/current-role reconciliation remains
required by its actual operation-history,consumer and publication owners.

Pass that actual empty edge proof into the existing contour admission with N0. Every target use
must be an exact retained source identity:an Output tag names no admitted candidate and refuses.
All complete source/group/use quotas,source association,self-cycle checks,cut uniqueness and
nonempty boundary/hole rules remain. Zero new edges do not make an empty or collapsed Piece valid.
Unsupported physical collapse must follow the actual domain/refusal contract.

Source identity coverage,reference eligibility,actual registration plans,the five whole-consumer
evidence requests,boundary/domain/interval requirements,canonical metadata,actual copies and
attributed effect all retain their existing admission paths. Nothing bypasses those checks. A retained
source query has at most one identity match at N0;Unique remains metadata rather than physical/current
proof. Gaps and actual registrations remain visible;no registration is withdrawn to hide absence.

For example,retain the original outer edge and original construction edge. The complete path can
bind with zero new candidates while keeping both registered owners and all physical-copy identities.
Already issued role2/role9 edges remain globally live,and their catalogue membership is unchanged.
No journal edit or geometry result is supplied by this declaration.

Alternatively,retain the outer edge but omit the original construction edge. This also uses no new
edges,but its contours changed. The omitted source's eligibility query returns Gap while its global
edge and registration can still resolve. All current semantic-ownership/repair evidence remains
required. Zero candidate count leaves the original Added/Removed/Unchanged request untouched.

No-new-edge admission therefore proves neither source preservation nor an achieved zero effect.
The explicit outcome choice below separately checks exact source-contour/order/direction preservation
and required Unchanged intent. Actual geometry,current ownership,effect,canonical history/retirement,
repair/authority and atomic publication keep their G2/G3/Design/bus proof owners.

## Explicit source-preserving or reconciled outcome

`required_effect.require_outcome(intent)` requires an explicit `SlashSpreadOutcomeIntent` on the
actual complete effect/copy/metadata/boundary/source context. There is no default or automatic
choice based on zero candidates. Both modes return a private `RequiredSlashSpreadOutcome` carrying
the original intent and actual whole context;this is a requirement declaration,not execution.

| Intent | Structural requirement and meaning |
| --- | --- |
| `PreserveSource` | N0,original authored Unchanged fullness and exact original boundary,hole groups/holes and construction occurrences,including order and traversal |
| `ReconcileContours` | Use the actual admitted contour declaration with either candidate population;infer neither physical change nor no-change |

Canonical Piece metadata/fold state and every actual target-copy identity are already retained by
the preceding owners. PreserveSource adds exact contour preservation. Merely keeping the same edge
membership is insufficient:a reversed boundary edge,missing/repeated construction occurrence or
reordered hole group does not match the source declaration. Aliases are not substituted and lists
are not sorted to manufacture a match.

```rust
use sc_core::construction::*;
fn require_source_preservation<'a>(effect:&'a RequiredSlashSpreadEffect<'a>)
    ->Result<RequiredSlashSpreadOutcome<'a>,SlashSpreadOutcomeError<'a>> {
    effect.require_outcome(SlashSpreadOutcomeIntent::PreserveSource)
}
```

PreserveSource refuses in this order:NewEdges,Fullness,Boundary,HoleGroups,the first differing
ordered Hole,then Construction. Hole diagnostics retain the actual group position,not an invented
persistent hole identity. Errors keep the original explicit intent and actual whole effect context.
No partially accepted preservation state escapes. ReconcileContours still retains all earlier
source,semantic,geometry/effect and authority evidence requirements.

For exact retained original contours at N0 and authored Unchanged,either mode can be explicitly
requested. PreserveSource establishes those structural associations;ReconcileContours leaves its
meaning at reconciliation. A candidate-bearing plan may request ReconcileContours but cannot claim
PreserveSource. An N0 plan with Added or Removed likewise cannot claim source preservation. No angle,
zero formula or full turn changes the original fullness declaration to satisfy this check.

Two actual original hole groups remain distinguishable even if their geometric validity is still
deferred. Swapping them refuses source preservation at the first differing group;omitting one refuses
HoleGroups;reversing one hole edge refuses that Hole. Omitting,repeating or reversing a construction
occurrence refuses Construction. These may remain explicit reconciled declarations with their own
required evidence;none is silently relabelled unchanged.

`RequiredSlashSpreadOutcome::effect()` retains the actual whole context;`intent()` retains the
original caller choice;`new_edge_count()` derives the actual complete original candidate population.
The comparison allocates nothing and uses already admitted whole N/R/M populations. It grants no
whole-operation execution budget or currentness proof. Actual supported domain,zero/fullness effect,
canonical intake conservation,copy/sewing effects,current identity history/ownership,visible repair,
authority and atomic publication remain G2/G3/Design/bus proof. The native checkpoint below has
component scope;full verification and geometry-backed integration have not passed in these slices.

## Native slash contract checkpoint

The native slash/spread declaration inventory covers the following boundaries. These are usable
structural declarations and required-evidence associations;they do not execute a geometry engine.

| Native contract | Available behavior | Remaining proof |
| --- | --- | --- |
| Spatial inputs/opening | Actual Piece/cut/bank,pivot or direction and Angle/Length documents;normalization/root-kind checks | Ordered current frame/registry and actual region/motion |
| Shared roles/edge manifest | Persistent definitions/active ids,actual operation association,source/bridge provenance;ordinary nonempty or explicit N0 admission | Issuance/current history,geometric role continuity |
| Contours/source eligibility | Exact retained/new occurrences,source membership,all declared alternatives and visible gaps | Actual closure/trim,maps and current semantic ownership |
| Source/registered consumers | Whole source-identity intent and actual registered-source requests;original owner/anchor/choice preserved | Whole held-consumer census,actual correspondence and repair |
| Consumer/boundary evidence | Five mandatory consumer scopes;domain/interval and bridge/trim selection requests | Accepted provider evidence,unique supported geometry,authority |
| Piece/fold/copies | Borrowed canonical fields/print view,explicit fold attribution,every actual target copy and population association | Physical fold/copy geometry,current Design/namespace and sewing effects |
| Fullness/outcome | Original-input/all-copy derivation requests;explicit source-preserving or reconciled declarations | Achieved effect/canonical intake,actual zero effect and atomic publication |

The scoped component checkpoint passed51 native contracts across15 slash suites and53 actual slash
API/book doctests. Runtime Rust/tests/Cargo bytes are unchanged in this checkpoint;the checked source
is the0716 implementation. This is not workspace/full-CI,platform parity or a physical verdict.

The declaration checkpoint closes only this native family's schema/association scope. Actual
ordered declarations,containing frames,current registries and issuance/history binding remain
G1-SLICE.5f.2;real selector/current-geometry and actual-length error budgets remain .5f.3;geometry-backed
replay of trim/fullness/fold/copy/semantic effects remains .5f.4.geometry after the real G2/G3 engines.
Actual command authority,repair coordination and atomic publication retain their Design/bus owners.
Storage/app/platform/full-project checks keep their separate owners. This component checkpoint
grants neither whole G1/G0 closure nor release approval.
