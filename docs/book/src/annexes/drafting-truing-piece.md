# Native truing whole-Piece declarations

This companion continues the [selected-source and outcome contracts](drafting-truing-outputs.md).
It declares whole-Piece replacement inputs;actual geometry,currentness,fit and publication remain separate.

## Declare explicit outside-fragment roles

`TruingOutsideRoles` retains two explicit `TruingOutsideIntent` fields,prefix then suffix. Each
field is `Absent` or `Role(LocalTag)`;there is no default or generated tag. `admit` binds the complete
fixed proposal to actual `RequiredTruingEvidence` under `TruingOutsideRoleLimits::new(N,catalog)`.
N counts every selected-domain candidate. The catalogue policy bounds all definitions,active
occurrences and cached original key bytes,including inactive/unrelated declarations.

```rust
use sc_core::{construction::*,ontology::LocalTag};
fn admit<'a>(raw:&'a TruingOutsideRoles,evidence:&'a RequiredTruingEvidence<'a>)
    ->Result<AdmittedTruingOutsideRoles<'a>,TruingOutsideRoleError<'a>> {
    raw.admit(evidence,TruingOutsideRoleLimits::new(1,EdgeOutputRoleCatalogLimits::new(4,3,25)))
}
let raw=TruingOutsideRoles::new(TruingOutsideIntent::Role(LocalTag::new(2)),
    TruingOutsideIntent::Role(LocalTag::new(5)));
assert_eq!(*raw.intent(TruingOutsidePart::Prefix),TruingOutsideIntent::Role(LocalTag::new(2)));
```

Whole N quota precedes fresh D/A/key-byte/owner admission of the **same actual bound catalogue**.
The original binding kernel checks its cached counts without scanning keys or allocating. Only
then are prefix and suffix inspected,in that order. An unaccepted suffix returns the whole original
proposal and context,never an accepted prefix view. All prior storage/workspace obligations remain.

For `Refit`,a positive prefix is exactly original0 through the selected lower parameter;a positive
suffix is exactly the selected upper parameter through original1. The caller supplies neither a
replacement edge nor independently authored range bounds. Each positive outside portion requires a
role. A zero-length outside portion requires `Absent`;supplying a role instead returns the exact
canonical `SourceRange` cause. Original bounded rational parameters are copied without rescaling,
epsilon,rounding or geometry evaluation. This is the original held edge frame,not a current mapping.

For selected1/4–3/4 and selected candidate role9,prefix role2 declares0–1/4 and suffix role5
declares3/4–1. Selected0–3/4 requires absent prefix and a suffix role;selected1/4–1 requires a
prefix role and absent suffix. Selected0–1 requires both absent. `PreserveSource` also requires
both absent for **every** selected domain:the whole original source remains required unchanged,
and no outside output identity is introduced merely because the selected span is partial.

Every supplied role must have an actual definition and active membership. It must differ from
every selected-domain candidate tag and from the other outside role. A missing positive portion
refuses `Missing`;unknown/inactive tags refuse `Undefined`/`Inactive`;reusing selected role9 refuses
`SelectedRole`;prefix2/suffix2 refuses `Duplicate`. Source preservation with any supplied role
refuses `PreserveSource`,before tag lookup. Catalogue quotas still precede those field causes.

Two fixed association slots require constant space and no allocation;work is O(log(N+D+2)),for
D actual catalogue definitions. `proposal()`, `evidence()` and `limits()` retain the actual whole
original parents and policy. `roles()` lends present associations in prefix/suffix order and
reverse. Each authentic `TruingOutsideOutput` retains its whole `context()`,fixed `part()`,original
`intent()` field,exact derived `source()` range and actual active catalogue `role()`.
`logical_output()` declares the original operation/tag tuple;it mints nothing. Errors retain the
same whole proposal/evidence/policy and first quota or fixed-field cause. Original inputs stay unchanged.

These attributions do not choose output parameter orientation,whole-Piece placement/traversal,
source retirement or geometric correspondence. Those need explicit declarations and proof;an
original contour direction is distinct from physical-copy reflection and sewing correspondence.
Whole-Piece orientation/contour/lineage declarations remain required before native family closure.
Actual untouched outside geometry,current source resolution,identity issuance,fit,effects and
atomic publication retain their separate execution/evidence owners.

## Declare every output parameter direction

`TruingDirectionManifest` retains an explicit operation and ordered
`TruingDirectionAssignment` collection. Each record names a persistent output tag and a required
`TruingParameterDirection`. This dedicated type has no default and cannot be replaced by the
ontology's Piece-traversal `Direction`.

`SameAsSource` requests output start/end correspondence to the attributed source from/to bounds.
`ReverseSource` requests correspondence to source to/from. Neither choice is a numeric or affine
parameter map,an achieved position,Piece traversal,copy reflection or sewing correspondence.

```rust
use sc_core::{construction::*,ontology::{EntityId,LocalTag}};
let raw=TruingDirectionManifest::new(EntityId::from_bits(1000),vec![
    TruingDirectionAssignment::new(LocalTag::new(5),TruingParameterDirection::ReverseSource),
    TruingDirectionAssignment::new(LocalTag::new(9),TruingParameterDirection::SameAsSource),
    TruingDirectionAssignment::new(LocalTag::new(2),TruingParameterDirection::ReverseSource),
]);
assert_eq!(raw.assignments().len(),3);
fn admit<'a>(raw:&'a TruingDirectionManifest,outside:&'a AdmittedTruingOutsideRoles<'a>)
    ->Result<AdmittedTruingDirections<'a>,TruingDirectionError<'a>> {
    raw.admit(outside,TruingDirectionLimits::new(3,3,6))
}
```

`admit` uses the actual complete selected/outside population,not every active catalogue role.
Let T be selected candidates plus present outside roles,and M the complete raw assignment count.
Checked T addition and2M workspace multiplication precede whole T,M and workspace quotas. Those
precede operation equality,exact M=T cardinality,index allocation and original assignment scans.
Each tag must be unique and name an actual selected or outside output. Cardinality **together with**
unique actual-population membership proves complete attribution;equal counts alone prove nothing.

With selected9,prefix2 and suffix5,the actual population is T3. T2 refuses `Outputs` before M2
or W5. T3/M2 refuses `Assignments`;T3/M3/W5 refuses required workspace6. T3/M3/W6 admits
resource equality,then checks the original operation and fields. Missing assignments refuse `Count`.
Three assignments9/2/2 still refuse `Duplicate`;9/2/99 refuse `UnknownOutput` at original index2.
When a whole-domain refit declares only selected9,an otherwise active catalogue role2 is still an
unknown **output**. No missing direction is inferred from that role's presence or list position.

The additional index/association bound is2M logical entries,not allocator bytes or earlier owned
storage. Work is O(M log(M+T+2)). Every refusal retains the actual complete raw `manifest()`,whole
`outside()` parent,original `limits()` and first `refusal()`;no accepted prefix escapes.

`AdmittedTruingDirections` lends those same original parents/policy and reports actual
`workspace_entries()`. `assignments()` traverses original declaration order and reverse;
`assignment(tag)` queries in O(log(M+2)) work. Each authentic `TruingDirectionalOutput` retains
its whole `context()`,original `index()`/`assignment()`,actual selected/outside `source()` member
and explicit `parameter_direction()`. The source member lends its actual held `range()` and
unissued `logical_output()` tuple. No alternate source frame or caller-supplied replacement member
can enter admission. Original tags,directions,source fields and declaration order stay unchanged.

Explicit preservation admits an exactly empty new-output assignment set with T0/M0/W0 and the
actual operation. It supplies no direction defaults,retirement,no-op or fit certificate;original
active metadata remains present. Whole-Piece placement still needs original occurrence ordering
and composition with these explicit source-relative choices. Actual endpoint/stop positions,
interior parameter maps,current source/history,geometry,fit,effects and atomic publication remain
separate execution/evidence obligations.

## Admit the exact held Piece occurrences

`AdmittedTruingDirections::admit_exact_piece_source` binds the actual selected pattern Piece
already held by the original copy inventory. `TruingPieceSourceLimits::new(T,H,R)` explicitly
bounds every selected/outside output T,every original hole header H,and every original occurrence R.
R includes boundary edges,all hole edges,construction lines and **separate fold declarations**.
Unrelated fields count;repeated construction uses count separately. Physical copies do not multiply R.

```rust
use sc_core::construction::*;
fn bind<'a>(directions:&'a AdmittedTruingDirections<'a>)
    ->Result<BoundExactTruingPieceSource<'a>,TruingPieceSourceError<'a>> {
    directions.admit_exact_piece_source(TruingPieceSourceLimits::new(3,2,8))
}
```

The whole T quota precedes the whole H quota. Only then does a length-only scan of the H hole
headers compute checked R;the whole R quota precedes any individual edge-field inspection.
Admission uses O(H+R) work,constant additional space and no allocation or label/material string
scan. Earlier parent resource obligations remain separate. The policy counts logical fields,
not bytes,allocator capacity,global storage or geometry work.

For a Piece with two boundary edges,two one-edge holes,three construction lines and one fold
declaration,R is8. T3/H2/R7 refuses `Occurrences` with required8,even if only one boundary edge
and two construction lines name the selected source. T2/H1/R7 refuses `Outputs` first;
T3/H1/R7 refuses `Holes`. T3/H2/R8 admits those resource counts before checking literal membership.
The fold declaration counts again because it is a distinct original field from its boundary use.

At least one contour field must **literally** name the selected held `EdgeRef`.
`NoExactOccurrence` means this exact-held variant cannot place a replacement in the supplied
Piece. It does not prove global absence or current invalidity:aliases,history and correspondence
still need current-source resolution and evidence. A structurally valid independently supplied
Piece can share the expected Piece identity without carrying that literal edge. Empty new-output
preservation still requires the actual original Piece and its literal source occurrence.

`BoundExactTruingPieceSource` retains actual `directions()`,actual `piece()`,original `target()`
and `limits()`. Its `occurrence_count()` includes every original field;`contour_matches()` and
`fold_matches()` separately report literal selected-edge occurrences. `occurrences()` lends all
boundary fields,then holes in authored order,then construction lines,then fold declarations;
reverse traversal returns the exact reverse order. No field is filtered,deduplicated or retargeted.

Each authentic `TruingPieceOccurrence` retains its whole `context()`,original `location()`,
actual borrowed `field()` and literal `is_target()` result. `TruingPieceOccurrenceLocation`
distinguishes boundary index,hole/index,construction index and fold index. The field is either
`Directed(&DirectedEdge)`,including the original unchanged traversal direction,or `Fold(&EdgeRef)`.
Its `edge()` lends the actual original field. This field enum alone is not a whole-context proof.
Errors retain the complete original directions,selected Piece,policy and first quota/overflow or
literal-membership cause;no partial occurrence admission escapes.

This step supplies original placement inputs. The following contour declaration uses them;
explicit fold choice,whole-edge lineage,currentness,mapped geometry,endpoint/stop fit,all-copy
effects,authority and atomic publication retain their separate declaration/execution owners.

## Declare complete replacement contours

`BoundExactTruingPieceSource::declare_contours` declares every boundary,hole and construction
use. The caller supplies `TruingContourLimits::new(T,H,R,G,W)`,with no defaults or truncation.
T counts every selected/outside output,H every original hole header,and R every original contour
and separate fold field. Let C be original contour uses and K literal selected-edge contour uses.
For `Refit`,G is C−K+K×T. For `PreserveSource`,G is C. W is checked T+G additional logical entries:
one temporary source-order index and the complete generated association collection.

```rust
use sc_core::construction::*;
fn contours<'a>(source:&'a BoundExactTruingPieceSource<'a>)
    ->Result<TruingPieceContours<'a>,TruingContourError<'a>> {
    source.declare_contours(TruingContourLimits::new(4,2,8,16,20))
}
```

Whole T,H and R quotas precede checked G/W arithmetic. Whole G then W quotas precede all
individual field scans,sorting or allocation. For C7,K3,T4,the complete generated count is16
and additional workspace20. G15 refuses `GeneratedOccurrences`;G16/W19 refuses `Workspace`.
R includes a separate fold field even though G contains only contour uses. Bounds count logical
entries,not bytes,allocator capacity,earlier parent storage or geometry work.

For refitting,the actual complete selected/outside outputs are sorted by their attributed
ascending source bounds. Earlier coverage and outside admission already require their complete
whole-edge partition. Raw declaration order and tag order never choose placement. Each literal
source use is replaced by that complete sequence. Every unaffected use remains exact,in place.
Boundary order,hole order and construction order are preserved;no closing-edge duplication is added.

Suppose source-order roles are prefix2,selected9,selected11,suffix5,with requested parameter
directions reverse,same,reverse,same. An original forward use becomes
`2 reversed,9 original,11 reversed,5 original`. An original reversed use becomes
`5 reversed,11 original,9 reversed,2 original`:both sequence and traversal compose with the
original use. Repeated construction uses each receive the complete sequence. A source in a hole
replaces that hole's occurrence;unrelated outer/hole fields remain exact. Whole-domain refits have
no outside roles;domains touching one endpoint include only the positive opposite outside portion.

For preservation,T is0 and every original directed contour field remains unchanged,including
its original direction and repeated uses. A seven-use source therefore has G7/W7. No new-output
direction,identity,fold axis or no-op certificate is inferred from that empty output collection.

`TruingPieceContours` retains actual `source()` and `directions()`,original `limits()`,checked
`workspace_bound()`,complete `occurrence_count()` and original `hole_count()`. `occurrences()`
lends all generated boundary,hole and construction uses in order and reverse. Each authentic
`TruingPlacedContourEdge` retains its whole `context()`,global `index()`,generated `location()`,
actual original `original()` field and complete `directed()` declaration. `output()` is the actual
explicit direction/source member for a derived use,or `None` for an unchanged original use.
No standalone tag or independently authored parent can substitute for this attribution.

Work is O(T log(T+2)+H+R+G),with at most W additional logical entries. Errors retain the whole
original source,policy and first complete quota or checked-count cause;no partial plan escapes.
All original inputs and non-contour Piece metadata remain unchanged and available through
`source().piece()`. These logical edge tuples remain unissued. The result is a contour declaration;
fold reconciliation follows below. Lineage,current history,canonical Piece birth,geometry,fit,
effects,authority and atomic publication remain separate requirements.

## Reconcile the fold explicitly

`TruingFoldRequirement::new(intent,correspondence)` retains an explicit `TruingFoldIntent` and
required geometric fold-correspondence source identity. Neither has a default. The source name
requests evidence;admission performs no provider call,lookup or evidence-body scan. It remains
required even for explicit no-fold treatment. Earlier whole-source/effect evidence obligations remain.

```rust
use sc_core::{construction::*,ontology::{EntityId,LocalTag}};
let request=TruingFoldRequirement::new(TruingFoldIntent::Derived{tag:LocalTag::new(9)},
    EntityId::from_bits(7001));
fn fold<'a>(contours:&'a TruingPieceContours<'a>,request:&'a TruingFoldRequirement)
    ->Result<RequiredTruingFold<'a>,TruingFoldError<'a>> {
    contours.require_fold(request,TruingFoldLimits::new(4,16))
}
```

`require_fold` first checks complete T outputs and G generated contour uses under
`TruingFoldLimits::new(T,G)`,including unrelated hole/construction uses. Quotas precede original
fold-state/field and proposed boundary inspection. For T4/G16,T3 refuses `Outputs`;T4/G15
refuses `Occurrences`,even if the supplied fold choice also contradicts the original state.

The original Piece's `cut_on_fold` remains authoritative. `NoFold` requires an original Piece
without a fold. `RetainOriginal` and `Derived { tag }` require an original cut-on-fold Piece and
its actual fold field. This preservation request cannot silently remove or add the cutting state.
All original non-contour metadata remains authoritative and unchanged through `source_piece()`:
identity,quantity,mirroring,cutting side,material,layer and label are not independently reauthored.

`RetainOriginal` requires an **unchanged** use of the exact original fold identity on the proposed
outer boundary. A matching logical tuple from a derived use cannot substitute for retained provenance.
When refitting replaces that original fold edge,retention refuses `FoldNotRetained`;the caller
must explicitly attribute a derived fold. Preservation can retain the original fold with no new outputs.

`Derived { tag }` first requires an actual selected/outside output,then an actual proposed outer
use of that output,then attribution to the exact original fold `EdgeRef`. An unknown or merely
catalogued tag refuses `Output`. An output used only in a hole or construction line refuses
`OutputNotBoundary`. An outer output attributed to another original edge refuses
`OutputNotFoldSource`. Original state mismatch refuses `FoldStatus` before any of those tag causes.

For a partial refit of the original fold,prefix2,selected9/11 and suffix5 can each carry structural
attribution to that held fold edge. Choosing one does **not** prove that its source subrange covers
the whole original geometric fold or establishes the physical fold axis. Those actual domain/position
and all-copy effects require the named correspondence evidence and existing execution obligations.
No candidate is selected by list position,length or an inferred geometric axis.

`RequiredTruingFold` retains actual `contours()`,original `requirement()` and `limits()`.
`source_piece()` and `source_fold()` lend the original Piece and fold field. `output()` lends the
authentic explicit derived direction/source member when applicable;it is `None` for original/no-fold
treatment. `edge()` returns the requested original or new logical fold identity,or `None` for
no fold. Every new logical identity remains unissued. Errors retain the whole original contour
context,request,policy and first quota/state/attribution cause;no partial fold admission escapes.

The check uses O(log(T+2)+G) work,constant additional space and no allocation or string scan.
Earlier source/contour limits retain their scope. Whole-edge lineage,actual current history,
canonical Piece birth,geometric fold correspondence,fit,copy effects,authority and atomic publication
remain separate requirements. Structural outer membership alone supplies none of those proofs.

## Request whole-edge identity and reference continuity

`RequiredTruingFold::require_lineage` retains an explicit `TruingSourceIdentityIntent` for the
**entire original held edge**,including outside portions of a partial selected span.
`RetainOriginal` requests that edge remain live. `RetireOriginal` requests its retirement with
actual identity history and correspondence. Neither has a default;neither applies a journal edit.
The original canonical pattern Piece identity remains required,with all its physical copies.

```rust
use sc_core::construction::*;
fn lineage<'a>(fold:&'a RequiredTruingFold<'a>,intent:&'a TruingSourceIdentityIntent)
    ->Result<RequiredTruingLineage<'a>,TruingLineageError<'a>> {
    fold.require_lineage(intent,TruingLineageLimits::new(4,16))
}
```

Whole T actual outputs and G generated contour uses are checked under
`TruingLineageLimits::new(T,G)` before inspecting the requested disposition and outcome.
For T4/G16,T3 refuses `Outputs`;T4/G15 refuses `Occurrences`. Unrelated uses count.
Explicit `PreserveSource` requires `RetainOriginal`:after quotas,a retirement request refuses
`PreserveSource`. Even preservation with no new outputs retains mandatory history and repair requests.

For refitting,the authentic candidate admission already forbids an operation from supplying its
own source edge. Complete contour generation replaces **every** literal original target use;
derived output creators therefore differ from that original source creator. Fold admission
requires actual proposed outer membership. Together these parent contracts already exclude a
result contour or result fold that still names the retired source identity. Lineage admission
reuses those actual guarantees and performs constant work with no field scan or allocation.

Either requested disposition during `Refit` still requires actual whole-source/history/observer
evidence. Retention does not silently move an old semantic reference onto a new output:if the
old edge no longer belongs to the modified Piece,the reference still needs a justified mapping
or visible repair. Retirement cannot be applied globally from one Piece proposal;other Pieces,
copies and semantic/sewing consumers require the complete current-owner and all-copy checks.
No request silently picks a fragment at a split endpoint or hides an unresolved reference.

`RequiredTruingLineage` retains actual `fold()`,original `intent()` and `limits()`. `source_edge()`
returns the original whole held identity;`piece_id()` lends the actual original canonical Piece id.
`evidence()` retains the same complete mandatory policy. `history_source()` and `repair_source()`
lend its **actual original** `IdentityHistory` and `VisibleRepairs` fields,including equal values
in distinct scopes. There is no independently authored replacement provider or accepted evidence.

`outputs()` lends every authentic selected/outside direction/source member in original declaration
order and reverse. Their held source ranges and requested directions describe required
correspondence;they supply no numeric,affine or achieved geometric parameter map.
`retained_contours()` lends every unchanged original use in output order and reverse,including
repeated uses. It costs O(G) traversal and allocates nothing;unrelated identities are not retired.
The full original Piece metadata and explicit fold remain available through the actual parents.

Errors retain the complete original fold/contour/source context,the actual intent field,policy
and first quota or preservation contradiction. No identity is generated,retired,reused or published.
Current liveness/nonreuse,whole-Design history,actual geometric maps,visible repair truth,fit,
all-copy effects,authority and atomic publication retain their separate execution/evidence owners.
These native whole-Piece declarations pass focused checks;the deterministic curve/refit selection
and supported-domain review remains required before native truing family closure.

## Planned deterministic curve selection

The next native policy is **ordered declared curve alternatives**. This section specifies the
planned declaration contract;its native admission and actual execution are not implemented yet.
It extends the existing constrained endpoint/stop-preserving variant. Endpoint movement,isolated
copy forks and a separately authored target length/ease are not inferred by this policy.

Locks and a target differential do not uniquely select a curve. For example,quadratic controls
`(0,0),(1,1/2),(2,0)` and `(0,0),(1,−1/2),(2,0)` produce distinct curves with the same endpoints
and pointwise squared speed `5−4t+4t²`. Their arclengths are equal;an endpoint stop distinguishes
neither. This is a selection example,not implementation or physical-fitting evidence.

For each actual **selected-domain output role**,the declaration will supply a nonempty ordered
list of explicit curve alternatives. Each alternative is one actual `DirectedRange` referring to
a named curve supplied earlier in the recipe. The published native line,center-arc and quadratic/
cubic Bézier declarations supply stable logical edge outputs. The canonical executor still has to
validate their actual supported line/arc/cubic geometry,including exact quadratic promotion.
NURBS remain deferred by the roadmap. No detached control-point cache becomes authoritative.

The alternative's directed traversal declares which chosen-curve endpoint becomes output start.
That chosen-curve frame is distinct from the output's requested orientation relative to the
**old** source range,and from Piece traversal. Actual endpoint/interior correspondence must reconcile
all frames. Curve references alone establish no geometry,currentness,prior-order or fit certificate.

Outside prefix/suffix roles have no independent curve alternatives:their geometry remains required
from the corresponding original source portions. `PreserveSource` has an exactly empty new-curve
proposal and retains its original obligations. For `Refit`,every actual selected role requires one
group;unknown/outside roles,duplicates and missing/empty groups cannot silently supply a curve.

Selection will use the explicit **first feasible complete assignment** policy. Groups and choices
retain authored order. With role9 choices20/21 and role11 choices30/31,the complete order is
`(20,30),(20,31),(21,30),(21,31)`;the rightmost group varies fastest. Persistent ids are never sorted
to establish priority. Feasibility is evaluated on the complete proposed Piece and all coupled
copies/observers,including both span sides if the same Piece changes both. Choosing each group's
first locally feasible curve is insufficient. The first complete assignment with all required
certified predicates passing determines the requested result.

An unavailable source,unsupported evaluator,uncertified result,resource failure or cancellation
must retain its actual cause;it cannot be treated as a proved infeasible candidate to skip.
Exhausting the declared alternatives can establish only that **this declared domain** contains
no certified-feasible assignment,not impossibility over every curve. Candidate generation and
complete envelope coverage remain G3 implementation/validation obligations.

Native admission will bound complete selected outputs N,raw groups M,all choice occurrences K,
the checked complete combination count P and index/association workspace. Whole bounds precede
field inspection or allocation;no prefix acceptance or default choice. The combination bound
counts declarations,not actual fit cost or memory bytes. Geometry execution needs its own full
resource admission before evaluating any assignment.

The next slice implements that native proposal/association only. The later coverage review remains
open until supported-domain/variant obligations are reconciled. Actual curve generation,certified
lengths/maps,feasibility,global currentness,repairs,effects,authority and atomic publication remain
with the existing G1 ordered-program/geometry and G2/G3/bus owners. This plan narrows no v1 gate.
