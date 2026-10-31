# Native drafting output contracts

Inputs remain in the [native editing reference](drafting-edit-inputs.md). Candidate roles,
source-domain membership and registered logical declarations are separate from geometric results.

## Offset output-contract recovery

The existing public `IdentityLedger::offset` accepts positive,source-ordered,nonoverlapping
`OffsetInterval`s. It allocates a fresh operation and contiguous tags in that interval order.
Shared interval endpoints become ambiguous candidates;gaps become unmapped repairs. Its point
fold uses the affine map `(source_parameter − from) / (to − from)`. Those are the declared
structural primitive's semantics,not evidence that a geometry engine has computed an offset.

Two additional contracts are required before accepting general drafting offset outputs.
First,a logical output tag is persistent. A caller cannot silently replace it with a new vector
position after fragment cardinality changes. Explicit roles,replay matching and retirement must
be reconciled with the ledger's allocation interface. No arbitrary explicit-tag manifest can be
published by assuming that interface already accepts those tags.

Second,`Param` is normalized arclength,not the polynomial parameter of a Bézier. A source interval
can identify candidate fragments without establishing a point's correct parameter on a normal
offset. Consider the exact cubic controls in micrometres:
`(0,0),(100000,0),(200000,100000),(300000,300000)`. Its curve is
`r(t)=L(t,t²)`,with `L=300000µm` and `0≤t≤1`. For a left normal offset of `d=30000µm`,the regular
offset speed factor is at least0.8,so this example does not rely on a cusp or topology change.

Let `S(t)=L[t√(1+4t²)/2+asinh(2t)/4]`. The normal-offset arclength is
`S_d(t)=S(t)−d atan(2t)`. At the source's halfway arclength point,`t≈0.610738683`:

| Interpretation | Offset arclength fraction |
| --- | --- |
| Normal correspondence of that original point | ≈0.475793971 |
| Whole-interval affine map | 0.5 |

The offset length is about410.468396mm;the two positions differ by about9.935810mm along it.
These are illustrative floating analytic values,checked against derivative quadrature—not a
certified geometry implementation or numerical tolerance proof. They demonstrate that a general
normal correspondence is not established by interval endpoints. The actual native affine-mapping
test passes;it tests its declared primitive,not this geometric correspondence.

The native manifest admission stage below admits explicit logical candidate roles and source
domains only. It must preserve ambiguous/gap outcomes and cannot return a geometrically certified
mapped Param. Separate native tasks own the stable-tag bridge and explicit mapping-kind/evidence/
error-budget contract before family closure. Actual G2 curves,nonlinear mapping accuracy and mark
placement remain `.5f.4.geometry` proof. This recovery changes no ledger algorithm or gate status.

## Native offset candidate-manifest admission

`construction::OffsetCandidateManifest` owns the required operation id,original `DirectedRange`
and all declared `OffsetCandidateFragment`s. Each retains an explicit `LocalTag` and positive
`OffsetInterval` in the original source frame. The raw constructor retains input without accepting
it. These are candidate-role declarations,not G2-computed fragments or an issued identity list.

`admit` requires the actual `BoundOffsetConstruction` and explicit `OffsetCandidateLimits`.
It checks complete N against the selected quota before inspecting fields or allocating its
temporary tag index. Then it refuses empty input,wrong operation or any source edge/bounds/
traversal mismatch. Fragment checks follow original order:duplicate tag,selected-source bounds,
then overlap/descending interval order. Duplicate/order indices identify actual original Vec
positions. There is no sorting,renumbering,equal-content deduplication or accepted prefix.

The count bounds this metadata admission and its temporary tag index. Earlier owned Vec
allocation,native ledger capacity,current-root/replay checks and geometry/execution workspaces
need separate admission. A zero-output geometric collapse cannot be replaced with one invented
edge to satisfy this nonempty manifest. Its execution outcome still requires an explicit contract.

```rust
use sc_core::construction::*;
fn declare_candidates(input:&OffsetConstruction,fragments:Vec<OffsetCandidateFragment>)
    ->OffsetCandidateManifest {
    OffsetCandidateManifest::new(input.operation(),*input.source(),fragments)
}
fn admit_candidates<'a>(manifest:&'a OffsetCandidateManifest,bound:&'a BoundOffsetConstruction<'a>,
    maximum:usize)->Result<AdmittedOffsetCandidates<'a>,OffsetCandidateError<'a>> {
    manifest.admit(bound,OffsetCandidateLimits::new(maximum))
}
```

Success and errors retain actual manifest,bound input/allowance and selected policy. The private
admitted view cannot outlive those owners. `fragments` exposes the original fields;`outputs`
pairs each with its logical `EdgeRef(operation,explicit_tag)` in original source order. For
example,tags9,2 and4294967295 stay in that order rather than becoming0,1,2. Traversal can be
Reversed while domains remain ascending in their original frame. Gaps and shared endpoints are
retained;their admitted-only membership query is described below.

Admission checks tag uniqueness within this manifest,not stable-role correspondence across
replay. It allocates no ledger operation,rewrites no reference and computes no curve,coordinate,
length or output Param. Actual allowance width/corner/inclusion stay on the bound descriptor.
The stable-tag bridge,mapping evidence and actual G2 results remain separate owned obligations.

## Native offset candidate membership

Only `AdmittedOffsetCandidates` can call `query_source(EdgeAnchor)`. The exact original edge is
checked before inclusive selected-range bounds. An alias/current fragment is not silently
substituted. The provided anchor value is owned unchanged by the resulting query or typed error;
both retain the actual admitted proof. An out-of-range position is an error;an uncovered position
inside the selected source is a valid `Gap` outcome.

```rust
use sc_core::{construction::*,ontology::EdgeAnchor};
fn query_candidates<'a>(admitted:&'a AdmittedOffsetCandidates<'a>,anchor:EdgeAnchor)
    ->Result<OffsetCandidateQuery<'a>,OffsetCandidateQueryError<'a>> {
    admitted.query_source(anchor)
}
```

`kind()` reports `Unique`,`Ambiguous` or `Gap` from all containing source domains. `candidates()`
returns every original containing occurrence in source order;shared interval endpoints retain
both sides. Each private `OffsetCandidateMatch` borrows the actual query and original fragment
field. Its index is the occurrence's original Vec position;`logical_output()` uses the explicit
tag rather than that index. Reversed iteration preserves those tags. A gap has no guessed
candidate and ambiguity has no default choice. No accepted candidate prefix is returned.

For declared domains `[1/4,1/3]` at tag9,`[1/3,1/2]` at tag2 and `[2/3,3/4]` at tag4294967295:

| Original source parameter | Membership | Logical tags |
| --- | --- | --- |
| 1/4 | Unique | 9 |
| 1/3 | Ambiguous | 9,2 |
| 7/12 | Gap | none |

These are metadata declarations,not a geometry-engine result or a choice of output position.

Each classification/iteration scans the admitted population without query-path allocation;whole
query-batch resources are a separate caller obligation. Traversal Reversed does not change the
original source parameters used for membership. After a real structural ledger offset,the
unchanged candidate metadata remains inspectable in its original frame while a query using the
new ledger edge is refused. This query creates no ledger repair and certifies no current geometry.

`Unique` means exactly one supplied candidate domain,not a unique geometric point. The result
contains no mapped output Param,curve coordinate,length or error-budget proof. Stable-tag
reconciliation/publication and the explicit geometric mapping contract remain pending before
offset-family closure;actual G2 results still need their separately owned verification.

## Native explicit-tag edge declarations

`IdentityLedger::declare_tagged_edges(operation,tags)` records the explicit output roles of an
already identified canonical operation. It uses that supplied operation id and retains every
tag in caller order;it does not mint an operation id or call an IdGenerator. The host still owns
issuance,global namespace/current-root checks and command authority. A well-formed supplied id
alone proves none of those facts.

The method checks nonempty count,the public `MAX_EDGES_PER_OPERATION` capacity,tag uniqueness,
then absence of the operation id from the whole journal. Every refusal leaves the ledger's
journal,live edges and registrations unchanged. A fully retired declaration or a noncreating
edit still reserves its operation id. Repeating an already recorded operation is a refusal;
canonical replay records the same operation/tag sequence in a fresh evaluation ledger instead.
Native count-based `declare_edges` keeps its injected allocator and existing behavior.

```rust
use sc_core::ontology::*;
fn record_canonical_roles(ledger:&mut IdentityLedger,operation:EntityId,tags:&[LocalTag])
    ->Result<Vec<EdgeRef>,LedgerError> {
    ledger.declare_tagged_edges(operation,tags)
}
```

This returns live logical `EdgeRef(operation,tag)` values and appends one actual
`TopologyEdit::Declared` entry in original tag order. It supplies no curve coordinates or
geometric certificate. Complete count checks precede the temporary uniqueness set;whole journal
lookup/command resources and actual role continuity across revisions or sizes remain separate.

Source preservation is deliberate. [Allowances are re-derived from net geometry](../spec/instantiation-paths.md)
and [profile line mapping preserves line meanings](../spec/interchange-dialects.md#4-cut-as-1-and-sew-as-1).
The existing `IdentityLedger::offset` retires its source;it is a different destructive topology
primitive. Declaring new derived outputs must not silently replace a net line with a cut line or
consume unselected parts of a source. The new declaration path changes no existing edge,
registration or reference resolution. Export inclusion remains the actual profile's explicit
policy;this primitive selects neither an export layer nor an allowance geometry.

Candidate-to-actual-declaration binding is available below;cross-replay role/retirement policy
remains pending. Fresh declarations alone do not establish those facts. Actual derived geometry,
source-to-output parameter correspondence and atomic bus publication stay separately owned.

## Native candidate-to-declaration binding

An admitted manifest's `bind_declaration` reads an actual provided `IdentityLedger`;it never
allocates a journal operation or edits source geometry. `OffsetDeclarationLimits` explicitly
bounds complete N candidate roles and J supplied journal entries. Checked2N counts the N-entry
temporary tag index plus N retained associations before lookup/allocation. These are entry
populations,not a complete allocation-byte,earlier-storage or whole-execution budget.

```rust
use sc_core::{construction::*,ontology::IdentityLedger};
fn bind_recorded_roles<'a>(admitted:&'a AdmittedOffsetCandidates<'a>,ledger:&'a IdentityLedger,
    limits:OffsetDeclarationLimits)->Result<DeclaredOffsetCandidates<'a>,OffsetDeclarationError<'a>> {
    admitted.bind_declaration(ledger,limits)
}
```

Quota refusal precedes operation lookup. Binding then requires one uniquely named operation
record,`TopologyEdit::Declared`,the same complete output count and every original explicit tag.
Missing and ambiguous records,wrong kind/count and the first absent tag retain actual admitted
owner,ledger,policy and cause. Journal indices identify actual records,not recipe statement
ordinals. No first-match choice or accepted association prefix escapes. In particular,a
destructive `Offset` entry cannot masquerade as the required declaration.

`DeclaredOffsetCandidates` retains the actual record. Its private output views borrow that
complete association,original candidate fields and actual `EdgeRef` fields in the journal's
Declared Vec. Matching uses explicit tags;the journal can list tag4294967295,2,9 while candidate
views remain in original source order9,2,4294967295. Neither sequence is rewritten. The view's
index remains its original candidate occurrence index,not the persistent edge identity.

This is historical declaration association. An output can have been retired by a later journal
edit while its original declaration remains authentic. Callers still need current liveness/
reference resolution,current Design membership,role continuity,actual G2 geometry/mapping and
command authority before use or publication. The binding supplies none of those certificates.

## Planned canonical role continuity

The next native component separates three different facts. A canonical catalogue retains all issued
local role-key/tag definitions for its operation. Active canonical membership is an explicit subset
controlled by authoring/history operations. An evaluated manifest reports which roles occur in that
particular result;it does not rewrite either of the first two.

For example,roles `main` at tag9 and `trimmed_end` at tag2 can both be canonically active. One size
may produce only tag9 and another both9 and2. The first result cannot retire tag2. Repeated evaluation
cannot compact those values into0,1 or silently assign an old tag to a new branch.

Undo/history restoration may reactivate the same declared role while restoring membership,but it
must not rewind the issued definition history. If an earlier membership snapshot predates a newly
issued role,the current issued definition remains reserved even when absent from the restored
active subset. This follows the existing
[private reserve and issued-revision separation](../spec/command-execution.md#private-identity-reserve-and-actual-notch-preview).
Actual history authority and publication are not supplied by a metadata proposal.

Native catalogue admission and structural transitions are implemented below;readonly
manifest-presence binding remains pending. Local machine
keys identify declared roles;they are not geometric matching rules. Unknown/ambiguous geometric roles
must not acquire a false continuity certificate from matching names or source intervals. Actual
G2/G3 evidence,current operation/Design ownership,storage and bus integration remain separately owned.

The active journal guidance was also corrected under D373:merge events retain the supplied Length
evidence;the ledger does not compute curve length. D374 corrects the old `Declared` Rustdoc to match
the existing explicit sparse/caller-order tag API. Both are documentation corrections;the affine
primitive and runtime algorithms remain unchanged.

## Native declared-role catalogue admission

`construction::OffsetRoleCatalog` owns an operation id,all supplied `OffsetRoleDefinition`s and a
separate active-tag list. Each definition retains an explicit `LocalTag` and validated `MachineToken`
local key. These are declared identity keys,not global recipe-variable bindings or geometric matching
rules. The raw constructor owns both original lists without accepting them or issuing identities.

`OffsetRoleCatalogLimits` explicitly bounds N definitions,A active occurrences and total key bytes.
Admission checks complete workspace counts,definition quota,active quota and checked total key bytes
before allocating indexes. Peak entry populations are two N-entry definition indexes and two
A-entry membership collections. These are count bounds,not prior String/Vec allocation,whole memory
or execution guarantees. Machine-token lexical validation occurred before this catalogue stage.

Only after complete resource admission are definitions checked in original order,tag before key.
Active ids follow in original list order,duplicate before undefined. Exact duplicate keys/tags or
membership ids refuse;no renaming,sorting,deduplication or accepted prefix is returned. Errors retain
the actual whole raw catalogue,selected policy and original cause indices.

```rust
use sc_core::{construction::*,name::MachineToken,ontology::*};
let tag=LocalTag::new(9);
let role=OffsetRoleDefinition::new(tag,MachineToken::new("main").expect("literal local key"));
let raw=OffsetRoleCatalog::new(EntityId::from_bits(10),vec![role],vec![tag]);
let admitted=raw.admit(OffsetRoleCatalogLimits::new(1,1,4)).expect("complete literal metadata");
assert!(admitted.is_active(tag));
assert_eq!(admitted.definition(tag).expect("declared role").key().as_str(),"main");
```

`AdmittedOffsetRoleCatalog` borrows the actual raw owner and policy. `definition(tag)` returns the
original definition,including inactive ones. `active_roles()` returns private views pairing each
actual active-list tag field with its original definition in active-list order,which can differ
from definition order. The active-list index is an occurrence position,not persistent identity.
All declared keys/tags remain unchanged. Empty initial catalogues and fully inactive catalogues are
valid metadata states;neither creates an implicit `main` role or proof that an offset can execute.

This validates current supplied metadata only. It cannot prove the list includes all previously
issued roles,an actual canonical operation owns it,an instance contains a role,or any geometry
matches its key. Native transition checks are described below;readonly manifest-presence binding is next;
current Design/namespace,actual history authority,G2/G3 matching and publication remain separate.

## Native issuance-preserving transition intent

`AdmittedOffsetRoleCatalog::admit_transition` compares two actual admitted catalogues under an
explicit `OffsetRoleTransitionLimits` policy. Complete combined definitions,active occurrences
and key bytes are checked before operation identity or role content. It allocates no new work
index and changes neither source. Earlier catalogue allocation and whole execution remain separate.

```rust
use sc_core::construction::*;
fn check_role_edit<'a>(before:&'a AdmittedOffsetRoleCatalog<'a>,after:&'a AdmittedOffsetRoleCatalog<'a>,
    limits:OffsetRoleTransitionLimits)->Result<AdmittedOffsetRoleTransition<'a>,OffsetRoleTransitionError<'a>> {
    before.admit_transition(after,limits)
}
```

Both catalogues must name the same operation. Every original before-definition,including inactive
ones,must remain in the after catalogue with the same explicit tag and exact local key. A missing
old definition or changed key refuses at its actual original before index. After admission has
already established unique keys/tags,so an old key cannot silently move to another tag. Complete
resource failures precede these content failures;errors retain both original admitted owners,
selected policy and typed cause. No accepted prefix is returned.

The private transition view exposes actual introduced after-definition fields with original indices.
`activated()` and `deactivated()` return actual membership views in their respective original list
orders. Active membership may explicitly change while all issued definitions remain reserved.
These iterators describe a proposal;they mint no identity and authorize no mutation or undo.

For a membership restore,copying an older entire catalogue can forget definitions issued after
that snapshot and is refused. A proposal retaining the complete current issued definitions while
restoring the earlier active subset passes these structural checks. Applying an actual recorded
history action still requires the bus's real history/authority evidence. Equal key/tag metadata
proves no geometric role continuity,current Design ownership or source-to-derived point map.
