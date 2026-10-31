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

Candidate-to-actual-declaration binding and cross-replay role/retirement policy remain pending
native contracts. Fresh declarations alone do not establish those facts. Actual derived geometry,
source-to-output parameter correspondence and atomic bus publication stay separately owned.
