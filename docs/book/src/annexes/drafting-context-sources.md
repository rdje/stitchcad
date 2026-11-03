# Drafting context sources

## Containing context and remaining integration

Ordered integration must borrow the actual containing `DesignInputs` owner,its exact revision,
the program's explicitly selected Cartesian drafting-frame source,and the corresponding current
geometry snapshot. Equal identity/revision numbers on another owner do not establish that binding.
The frame source retains its authored coordinate-system definition;the checker selects no screen
origin,axis orientation,unit scale,transform or fallback frame. An edge's rational parameter frame
and a physical copy's orientation remain their own sources.

`DraftingContextSource<H>` declares the immutable source port. An actual `DesignFormulaScope`
can now call `bind_drafting_context(&source,limits)` to obtain a private `DraftingContext` binding.
The factory compares the actual owner first,charges complete context content and finishes its
first-failure meter,then compares actual revisions and validates the source before retaining its
frame/snapshot. A foreign owner is refused even when identity/revision numbers match. A source
that ignores a sink error still receives that first refusal;no revision/currentness/geometry query
runs after content refusal. Errors retain original owner/revision borrows or the provider cause.
The port borrows `root()`,`snapshot_revision()`,`frame()` and `snapshot()` from the actual source.
Its `charge_resources` uses the existing restricted first-failure-latching resource sink;
`validate_current` retains the provider's original typed error. Its `geometry` request carries an
explicit `DraftingGeometryReference::Point(PointRef)` or `Edge(EdgeRef)` and returns raw
`DraftingGeometrySource` fields:the record's actual typed reference,frame,snapshot and geometry.
These borrowed source types can be unsized and need neither Clone nor Debug. Copying the raw view
retains all loans;its debug representation shows the reference without source payloads. No default
frame,registry or identity is supplied. `bind_existing_geometry` now checks exact reference/frame/snapshot
binding before returning an existing-output binding. Canonical semantic-target resolution remains
part of complete dependency integration;the geometry port alone does not cover it.

The binding exposes the actual `scope()`,`source()`,`frame()`,`snapshot()`,explicit `limits()` and
charged `demand()`. Copying it copies only held sources and bounded accounting data;its private
fields prevent source substitution and its lifetime cannot outlive those sources. Its Debug output
omits source payloads. Root and context allowances/demands remain separate:whole-program admission
must compose both with its own complete retained content before child work. This binding grants no
accepted program,geometry output/value,physical feasibility,runtime allowance or release approval.

External providers use `charge_design_revision_resources` to charge each retained revision's actual
magnitude into the restricted sink without formatting/copying its counter or supplying cached totals.
For example,a revision just beyond u128 needs129 magnitude bits;zero needs zero:

```rust
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{command::DesignRevision, design::{charge_design_revision_resources,
    DesignResourceLimits, DesignResourceLocation, DesignResourceMeter, DesignResourceScope}};
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let revision = DesignRevision::from_counter(u128::MAX).successor();
let mut meter = DesignResourceMeter::new(DesignResourceLimits::new(
    NonZeroUsize::MIN, NonZeroUsize::MIN, NonZeroU64::new(129).ok_or("bit allowance")?,
));
charge_design_revision_resources(&revision, &mut meter, DesignResourceLocation {
    scope: DesignResourceScope::Inputs, owner: None,
    field: "snapshot.revision", index: None,
})?;
assert_eq!(meter.finish()?.counter_bits(), 129);
# Ok(())
# }
```

Existing `DesignFormulaScope` pins actual scalar/input owners. It supplies no Cartesian frame or
geometry approval. `IdentityLedger` retains topology/current-edge and point-journal contracts;
its edge membership is not a replacement for a complete typed geometry/frame registry.

| Required source | Source boundary |
| --- | --- |
| Containing owner | Borrow the exact Design/program sources and revision;refuse a foreign owner before source queries |
| Frame | Borrow the actual selected Cartesian source,including its authored definition;never infer it from an output id |
| Snapshot | Borrow immutable actual geometry provenance tied to that Design/revision/frame;no boolean currentness claim |
| Existing point/edge | Resolve the exact held reference and output kind to its actual creator,frame and current source;retain original refusal |
| Planned output | Retain the actual earlier operation/output manifest and combined position;static availability grants no live geometry |
| Semantic target | Resolve actual canonical Dart/Piece/seam/grainline/allowance and companion sources;retain their distinct identities |
| Logical demand | Charge the complete held context inventory before queries,normalization or child checking;no unbounded copied registry |

The provider follows the existing immutable Design-source port discipline:its enumeration must
be exhaustive and stable,and shared access must preserve the actual snapshot. The core checks
owner/revision/source bindings,complete logical bounds and typed results;the port is no security
boundary against a dishonest implementation. Returned causes stay typed and identify the original
request and source. Missing,retired,wrong-kind,foreign-frame and unsupported results remain distinct;
none becomes a nearest surviving entity or an empty successful result.

There are two availability questions. Existing geometry must resolve in the actual source
snapshot. A new output from an earlier declaration may be statically named by the program's actual
manifest,although it has not been executed. It becomes geometric input only after its creator
succeeds during ordered execution. Future/self references fail the order check. A later failure
returns no accepted program prefix and publishes no partial geometry. Static checking reads no
coordinate or length values and cannot certify the future result's geometry or physical feasibility.

For example,Design A revision7 selects frame F and snapshot R. A line after a new point declaration
may refer to that declaration's actual persistent point output;it must not claim the point is
already present in R. An imported edge in R can supply an existing dependency only with its actual
creator/kind/frame/currentness. An equal-looking Design B,an orphaned edge or a frame G without an
explicit supported mapping is refused. The original records remain available for repair;the checker
invents no transform and rewrites no reference. Geometry-selector values and any supported mapping
remain later execution work,with their own numeric/error evidence.

Implementation order is context source/binding,whole mixed program,names,complete dependencies,
then whole checking. Coupled verification follows those implementations;real selectors and geometry
remain G1-SLICE.5f.3/.5f.4.geometry and G2/G3. This contract grants no execution,release or domain approval.

## Original record identity,including empty payloads

Frame and snapshot ports return an actual owned `DraftingSourceRecord<Payload>`,and the raw geometry
response borrows those same original records. Its `data()` exposes the original definition without
copying or accepting it. Each record has a fixed nonzero storage footprint even when its payload is
zero-sized or an empty unsized view. Distinct live source records therefore remain distinguishable;
plain payload addresses alone cannot establish that identity. `is_same_record` compares the actual
record storage independently of payload equality or unsized metadata. Equal-looking definitions do
not merge records. No Clone/Default is supplied;Debug hides the definition.

A record can move before borrowing. While held by a context or raw response,it cannot be replaced
or moved. This is transient original-source binding,not a persistent geometric identifier,address
serialization or identity issuance. Original PointRef/EdgeRef creator/tag semantics remain unchanged.
Provider accounting still walks all logical dynamic payloads;the fixed record byte supplies no
resource/currentness/geometry approval and replaces no held source field.

```rust
use sc_core::construction::DraftingSourceRecord;
let records = [DraftingSourceRecord::new(()), DraftingSourceRecord::new(())];
let [first, second] = &records;
assert_eq!(first.data(), second.data());
assert!(!first.is_same_record(second));
let empty = DraftingSourceRecord::new([0_u8; 0]);
let borrowed: &DraftingSourceRecord<[u8]> = &empty;
assert!(borrowed.data().is_empty());
assert!(core::mem::size_of_val(borrowed) > 0);
```

## Bind an existing typed output

After context admission,`context.bind_existing_geometry(requested)` makes one typed source lookup.
It first matches the returned PointRef/EdgeRef kind and full persistent reference,then the original
frame record,then the original snapshot record. Equal definitions or ids in different records do
not substitute for those original sources. Success returns a private `DraftingGeometryBinding`
retaining the actual context and original opaque geometry record;Copy retains every source loan.
`reference()`,`context()`,`geometry()` and `source()` expose those exact matched sources.

For example,a consumer can borrow the original source record without copying it or reading values:

```rust
use sc_core::{construction::{DraftingContext, DraftingContextSource, DraftingGeometryError,
    DraftingGeometryReference}, design::DesignMetadataHost};
fn original_geometry<'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(
    context:DraftingContext<'a,H,S>, requested:DraftingGeometryReference,
) -> Result<&'a S::Geometry,DraftingGeometryError<'a,H,S>> {
    let binding = context.bind_existing_geometry(requested)?;
    assert_eq!(binding.reference(), requested);
    Ok(binding.geometry())
}
```

A provider refusal retains the exact request and original typed cause through the error source
chain. A mismatch retains the original context,request,complete raw response and first
`DraftingGeometryBindingIssue`:Reference,Frame or Snapshot. In particular,a wrong-kind response
cannot become a valid point by matching an id. Multiple wrong fields report Reference before Frame
before Snapshot,while retaining every actual returned handle for diagnosis. Binding/mismatch Debug
output omits stored geometry payloads;original provider error content stays with its own error type.

These are bindings of existing source records under the exhaustive immutable provider contract.
They evaluate no coordinate,length,selector or formula,create no output,perform no repair/transform
and issue no identity. Planned earlier outputs stay in their actual operation manifests until
execution creates them;this API never materializes them. An output binding grants no whole-program
order/dependency/resource proof,geometry/physical feasibility,runtime allowance or release approval.

## Mixed program sources and admission

`DraftingProgramInputs::new(context,steps)` now owns the original `Vec<DraftingProgramStep>` and
pins its actual bound context. `context()` returns that borrowed handle;`recipe()` returns only its
actual Root's whole original recipe;`steps()` borrows the complete unchanged sequence. No foreign
recipe argument, copied source, provider callback, parsing, output identity or namespace is involved.
The owner has private fields and cannot outlive its actual context or native declarations.

`DraftingProgramStep::Scalar { ordinal }` retains a requested one-based scalar ordinal separately
from its mixed position;`Drafting` retains the actual `DraftingOperationInput`. Zero, missing,
repeated, out-of-order or out-of-range requests and empty sequences survive raw construction.
`admit(&DraftingProgramLimits)` now requires every original scalar statement,including each
assertion,exactly once in original relative order. `normalized()` now derives complete child arenas.
No topological reordering,statement extraction or implicit scalar source is introduced.

For example,this function retains actual inputs without accepting scalar coverage or reading values:

```rust
use sc_core::{construction::{DraftingContext, DraftingContextSource, DraftingOperationInput,
    DraftingProgramInputs, DraftingProgramStep, PointConstruction}, design::DesignMetadataHost};
fn retain<'a,H:DesignMetadataHost,S:DraftingContextSource<H> + ?Sized>(
    context:DraftingContext<'a,H,S>,point:&'a PointConstruction) -> DraftingProgramInputs<'a,H,S> {
    let inputs = DraftingProgramInputs::new(context,vec![
        DraftingProgramStep::Scalar { ordinal:1 },
        DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(point)),
        DraftingProgramStep::Scalar { ordinal:2 },
    ]);
    assert!(std::ptr::eq(inputs.recipe(),context.scope().root().recipe()));
    inputs
}
```

For example, these are illustrative positions, not a new text grammar:

| Combined position | Actual original source |
| --- | --- |
| 1 | Scalar statement 1 in the Root recipe |
| 2 | Original borrowed drafting declaration and all its argument documents |
| 3 | Scalar statement 2, still source ordinal 2 with its original whole-recipe spans |

Scalar recipe documents now expose factory-derived statement, expression and node counts through
`shape()`; expression documents retain their complete cached node count. These metadata preserve both
assertion operands, all conditional branches and unused source bodies, with no value reads, caller
annotations or parsing merely to discover a required population. Existing grammar/literal limits
and scalar canonical identity stay unchanged;that identity is not a mixed-program serializer/hash.

Complete input admission now walks actual Root/context/native fields, including both original and
canonical buffers, retained copies/caches, unbounded counters, the mixed order and Walk's original pair
vector. Point declarations own two argument documents, center arcs own three, and slash/spread owns
one. Other raw families retain their original fixed references and symbolic intents. Complete normalization preflight includes three cohorts:
all separately identified Root formula-source bodies, all scalar recipe operands, and all native
argument documents. Repeated/unused/untaken content counts;identity/address deduplication cannot
remove it. Existing source/argument conservative record bounds remain authoritative and compose
before child factories;they certify no allocator-byte, parser-scratch, CPU or runtime allowance.

Recipe counts,exhaustive native fields,raw mixed steps and complete input admission are implemented;
normalization and combined borrowed step views are also available. Existing document factories retain
their original typed/allocation causes;failure returns no normalized program prefix. Normalized
views must borrow actual owners and keep source ordinals separate from combined positions.

Actual name/output-manifest and canonical semantic bindings remain explicitly supplied at the
namespace/dependency stages, with their own complete held-content admission. The raw source owner
invents no output name, role, frame, manifest or identity. Whole name/kind/order/dependency checking
and coupled verification follow the complete integration set;normalization grants no program,
geometry, physical or release approval.

### Complete content, shape and original order

`DraftingProgramLimits::new(content,expressions,nodes,retained_records)` selects every allowance
explicitly. Content uses `DesignResourceLimits`;the three normalization allowances can be zero for
empty cohorts. Admission freshly charges the actual Root,then the complete context source,then
precharges the mixed vector before visiting native fields. It does not substitute earlier component
demands or deduplicate a retained copy. Even an ignored source-sink failure remains the first cause.
An empty sequence still needs its actual Root/context content allowance.

After held content passes,checked arithmetic counts all three complete cohorts from immutable factory
metadata,with no parsing,normalization or values. `DraftingProgramShape` reports mixed steps,scalar
statements,Root source bodies,native arguments,all expressions and all nodes. The conservative
prospective retained-record bound is `steps + statements + expressions + 2*nodes`:mixed normalized
slots,scalar statement records,expression owner-views,nodes and possible call-argument indices.
Inline owners can be counted twice;this composes the existing per-source/per-native conservative
bounds and grants no allocator capacity,parser scratch,CPU,checker or runtime allowance.

For example,two unused Root bodies with11 nodes,three scalar operands with8 nodes and eight native
argument occurrences with12 nodes require13 expressions and31 nodes. Seven mixed steps and two
scalar statements then require a conservative `7 + 2 + 13 + 2*31 = 84` records. Repeating one point
input counts both documents again. An expression limit12 refuses before a node limit30;with13
expressions allowed,the node limit30 refuses before a record limit83. A content refusal precedes
all those quotas and scalar-order checking.

Only then does admission compare scalar requests with actual original recipe ordinals. For a two
statement recipe,`[1,drafting,2]` preserves scalar ordinal2 at mixed position3;`[1,drafting,1]`
refuses at position3 with expected2/requested1. `[1,2,2]` reports no next expected ordinal;`[1]`
reports complete required2/provided1. Zero,out-of-range,reordered and missing assertion requests
are refused without clipping,sorting or returning an admitted prefix.

```rust
use sc_core::{construction::{AdmittedDraftingProgramInputs, DraftingContextSource,
    DraftingProgramAdmissionError, DraftingProgramInputs, DraftingProgramLimits},
    design::DesignResourceSource};
fn inspect<'p,'a,H:DesignResourceSource,S:DraftingContextSource<H> + ?Sized>(
    inputs:&'p DraftingProgramInputs<'a,H,S>,policy:&'p DraftingProgramLimits)
    -> Result<AdmittedDraftingProgramInputs<'p,'a,H,S>,DraftingProgramAdmissionError<'p,'a,H,S>> {
    let admitted=inputs.admit(policy)?;
    assert!(std::ptr::eq(admitted.inputs(),inputs));
    assert!(std::ptr::eq(admitted.limits(),policy));
    assert_eq!(admitted.shape().step_count(),inputs.steps().len());
    Ok(admitted)
}
```

Success privately retains the complete original input owner,actual policy,fresh cumulative demand
and complete shape. Errors retain original inputs/policy and the first content,population or
original-order cause;resource causes keep their typed error chain. A small diagnostic box compacts
uncommon semantic/content errors;no child normalization/allocation cause is replaced by it.
Admitted inputs are not normalized arenas or a whole namespace/kind/dependency/current-output
proof. No additional currentness or geometry callback runs during this admission;the held context
continues to rely on its honest immutable source contract. Complete static checking follows normalized
arenas and combined borrowed views,then coupled feature verification.

### Complete original child normalization

`AdmittedDraftingProgramInputs::normalized()` now derives every original child arena after complete
admission. The admitted handle copies only original input/policy borrows and fixed demand/shape;
providers and model data need neither Clone nor Debug. The factory first fallibly reserves the whole
mixed-slot vector,then calls the existing complete Root source factory,whole original recipe factory,
and native point/arc/opening factories in mixed order. It returns one private `NormalizedDraftingProgram`
only after every child succeeds. No partial owner,substring parsing,reconstructed span or self-reference
is introduced. All unused sources and repeated native occurrences are normalized too.

Child ceilings come from the explicit admitted whole expression/node/record policy. They do not grant
each child a separate aggregate allowance:actual complete demand was already bounded across all three
cohorts. Existing language/literal rules and original child factories stay authoritative. Source and
outer-vector allocation errors remain inline;other errors retain their original typed recipe/argument
cause and genuine mixed position. Error accessors retain the complete original input owner and policy;
no newly allocated wrapper replaces a failed allocation cause. Allocation fault verification remains
part of the separately owned coupled feature verification.

`sources()` exposes the actual complete Design-backed source owner;`recipe()` exposes the actual whole
Design-backed scalar recipe owner. `operations()` borrows a complete position-aligned slice:scalar
positions are `None`,and native positions hold a `NormalizedDraftingOperation`. Point and center-arc
variants own all actual argument arenas;slash/spread retains its actual whole declaration and normalized
opening. `Other` retains original families with no scalar argument documents. For `[scalar1,point,scalar2]`,
the slots are `[None,Some(Point(...)),None]`;scalar2 stays in the original whole recipe with its original
ordinal2 and whole-source span. Combined borrowed step views are now available.

```rust
use sc_core::{construction::{AdmittedDraftingProgramInputs, DraftingContextSource,
    DraftingProgramNormalizationError, NormalizedDraftingProgram}, design::DesignMetadataHost};
fn normalize<'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H> + ?Sized>(
    admitted:AdmittedDraftingProgramInputs<'p,'a,H,S>)
    -> Result<NormalizedDraftingProgram<'p,'a,H,S>,DraftingProgramNormalizationError<'p,'a,H,S>> {
    let program=admitted.normalized()?;
    assert!(std::ptr::eq(program.admitted().inputs(),admitted.inputs()));
    assert_eq!(program.operations().len(),admitted.shape().step_count());
    let _sources=program.sources(); // All actual Root bodies, including unused ones.
    let _recipe=program.recipe(); // Whole scalar recipe, including every assertion.
    Ok(program)
}
```

Normalization resolves no name,kind,source dependency,current geometry or value. An unresolved name,
unknown call,untaken division by zero or wrong point-argument kind can still be represented as normalized
syntax. These require the later complete static and value stages. An empty program retains its actual
Root/context with empty source,recipe and native arenas;it acquires no synthesized statement or default.

### Borrowed combined steps

`NormalizedDraftingProgram::steps()` now returns an allocation-free,exact-sized,bidirectional fused
iterator over actual normalized owners in original mixed order. A private `DraftingProgramStepView`
retains its genuine one-based `position()` and borrowed `kind()`. `DraftingProgramStepKind::Scalar`
contains the original scalar `ordinal` and a reference to the actual `FormulaNormalizedStatement`
in the complete recipe arena. `Drafting` borrows the actual `NormalizedDraftingOperation`,whose
argument arenas and original declaration remain accessible without copying or reparsing.

For `[scalar1,point,scalar2]`,scalar2 still has ordinal2 at position3. Calling `nth(2)` directly,
skipping earlier steps,reversing,or interleaving front/back traversal returns that same actual
statement. Each scalar uses direct original-ordinal lookup;there is no mutable scalar cursor that
could lag skipped requests. Statement spans still address the original whole recipe,including
original line offsets and assertion operands. No normalized statement is reconstructed from a source
substring or canonical text. The view cannot outlive its normalized arena,and its private position
cannot be rewritten by callers. Debug output reports position/ordinal/family without formula text.

For example,this helper selects a zero-based mixed index and returns both distinct one-based
positions plus the actual borrowed statement. A native step returns None:

```rust
use sc_core::{construction::{DraftingContextSource, DraftingProgramStepKind,
    NormalizedDraftingProgram}, design::DesignMetadataHost, recipe::FormulaNormalizedStatement};
fn original_scalar<'n,'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H> + ?Sized>(
    program:&'n NormalizedDraftingProgram<'p,'a,H,S>,mixed_index:usize)
    -> Option<(usize,usize,&'n FormulaNormalizedStatement<'a>)> {
    let step=program.steps().nth(mixed_index)?;
    match step.kind() {
        DraftingProgramStepKind::Scalar {ordinal,statement} => Some((step.position(),ordinal,statement)),
        DraftingProgramStepKind::Drafting(_) => None,
    }
}
```

The complete original program source/admission/normalization/view pipeline is implemented with
focused checks. Names,whole kind/dependency/current-output checks,values and geometry remain later
stages;these loans create no accepted executable prefix or release approval. Coupled feature and
allocator-fault verification remains separate and pending.

### Mixed names and admission

The [mixed drafting name reference](formula-name-scopes.md#mixed-drafting-names-and-admission)
covers actual initial/output sources,complete cumulative held-content producers and prospective
name-work admission. Source/position association and ordered namespace integration follow there.

## Authentic planned-output sources

`DraftingPlannedOutput` retains authentic field views from the existing published family factories:
Offset active candidates,Slash/spread edges,Truing edges,Mirror edges and points,and Join edges.
These are six view variants across five native families. Each retains its complete original
manifest/source/copy/identity parents and actual active role. A raw catalogue cannot manufacture
one of these family fields. Planned logical outputs remain unissued and absent from live geometry.

`input()` follows the actual public parent chain to its original creating schema;it copies no input
and reconstructs no binding from IDs. `name_source()` returns the exact typed active catalogue,
definition and tag loan. Offset retains the intentional shared Edge aliases;Mirror points retain
the distinct point domain. `reference()` uses the actual field's logical output,without issuance,
values,geometry or a fallback reference. Debug formatting exposes the family only.

`DraftingPlannedOutputSource` is a raw response containing that authentic field and the provider's
original `DraftingSourceRecord` frame and snapshot loans. Copying it copies only views and loans;
provider payloads need neither Clone nor Debug and stay out of diagnostics. Public construction
proves no containing Root,input,catalogue,frame,snapshot or accepted-program association.

```rust
use sc_core::construction::{ActiveOffsetCandidate, DraftingPlannedOutput,
    DraftingPlannedOutputSource, DraftingSourceRecord};
fn raw_planned_source<'a,F:?Sized,S:?Sized>(field:ActiveOffsetCandidate<'a>,
    frame:&'a DraftingSourceRecord<F>,snapshot:&'a DraftingSourceRecord<S>)
    ->DraftingPlannedOutputSource<'a,F,S> {
    let source=DraftingPlannedOutputSource {output:DraftingPlannedOutput::Offset(field),frame,snapshot};
    let copied=source;
    assert!(copied.frame.is_same_record(frame));
    copied
}
```

`DraftingPlannedOutputProvider` extends the actual immutable context source with `planned_output()`.
Its request is a genuine original output-position view from complete bounded work. Core will call
only for unresolved role candidates;fixed-schema names need no planned lookup. The provider returns
its actual pre-existing family-factory field and original context records,or original typed cause.
All held parent inventories,retained indexes and duplicated snapshots belong in the inherited
exhaustive content walk. Queries allocate/clone nothing,read no values and perform no existing-
geometry lookup to make a future output live. Missing or wrong-family sources remain refusals.

The provider shares the context port's immutable honest/exhaustive integration obligation. The
complete binding stage below compares actual input owner,catalogue/definition/tag and original
frame/snapshot record identity. This raw facet supplies no bound output,namespace prefix,currentness,
physical feasibility or whole-program approval. Original causes remain typed throughout.

## Bind every planned role source

`DraftingProgramOutputPositions::bind_role_sources()` borrows the complete admitted position owner
and first reserves all original output slots fallibly. The earlier explicit name-work policy
already bounds this complete population. Allocation failure retains the original TryReserveError
inline,before provider queries or a diagnostic box;allocator-fault integration verification remains
with the final feature gate. Fixed-schema outputs use None slots and perform no planned lookup.

For each unresolved role,in original output-vector order,the readonly provider returns an authentic
family field and original context records. Core checks actual creating input owner first,then
Point/Edge role kind,actual admitted catalogue owner,original definition and active-tag fields,
logical reference,frame record and snapshot record. Equal IDs or equal-looking schema/catalogue
copies do not replace those original owners. Frame/snapshot comparison uses SourceRecord identity,
so equal payloads and empty payloads cannot substitute another record. No value or existing-
geometry lookup,namespace insertion,identity issuance or transform runs here.

`BoundDraftingProgramOutputRoles` has private fields,borrows its original positions and owns only
the complete original-order result vector. `sources()` contains None for fixed schemas and Some
for checked planned-role sources. For `[role,fixed,role]`,the complete result is
`[Some(original),None,Some(original)]` with two provider queries. Repeated occurrences remain
separate;empty and all-fixed lists query nothing. The owner cannot outlive its position loan and
opaque provider records require neither Clone nor Debug.

```rust
use sc_core::{construction::{DraftingPlannedOutputProvider, DraftingProgramOutputPositions},
    design::DesignMetadataHost};
fn bind_sources<H:DesignMetadataHost,S:DraftingPlannedOutputProvider<H> + ?Sized>(
    positions:&DraftingProgramOutputPositions<'_, '_, '_, '_,H,S>)->Result<usize,String> {
    let bound=positions.bind_role_sources().map_err(|error|error.to_string())?;
    Ok(bound.sources().len())
}
```

`DraftingOutputRoleError` retains the actual positions/input/policy. Its refusal is original
Allocation,Source with original output index/typed cause,or Mismatch with complete returned source,
original output index and first issue. `Error::source()` preserves original allocator/provider
causes. A later failure returns no accepted prefix. Diagnostics omit authored names and opaque
records. Authentic input mismatch precedes role metadata;metadata precedes frame;frame precedes
snapshot. The focused example rejects a foreign equal-ID schema,a second admission of the same raw
catalogue,a different definition,a cross Point/Edge claim and equal-looking foreign context records.

Providers can reuse `DraftingGeometryNameSource::charge_resources(sink,index)` for the complete
source component before queries:fixed held schema fields or all actual catalogue fields/retained
indexes/key bytes. The explicit index identifies its original containing source occurrence;it is
not a resource total. Global output-name text and other parent/cache collections compose separately.
The name-input producer uses that same component,keeping original field order and first refusal.

This binds planned role metadata to its actual input and containing context. The complete initial-source stage below follows.
Reserved/cross-origin collisions,earlier-only name visibility,whole kinds and canonical/geometry/
semantic dependencies remain separate owned stages before values and whole-program acceptance.

## Raw extra-initial metadata sources

The mixed drafting path now has a readonly `DraftingInitialInputProvider` facet for extra initial
scalar/Length metadata. It receives an existing `FormulaCanonicalInput` join against the actual
Root inventory after complete held-content/prospective bounds. That existing join checks dedicated
Length owner or general scalar identity/kind without reading Unknown/Derived states. Source family
is preserved;a general Length scalar cannot replace a dedicated Length record.

`DraftingInitialInputSource` retains the join and actual provider-owned metadata/snapshot
`DraftingSourceRecord` loans. Copying this raw view copies only those loans;opaque payloads need
neither Clone nor Debug. Diagnostics show kind/origin and omit names,canonical state and payloads.
Scalar metadata introduces no Cartesian frame. The metadata record must actually expose the
returned declaration under that original snapshot;echoed caller IDs alone supply no source truth.

```rust
use sc_core::{construction::{DraftingInitialInputSource, DraftingSourceRecord},
    recipe::FormulaCanonicalInput};
fn raw_initial_source<'a,M:?Sized,S:?Sized>(input:FormulaCanonicalInput<'a>,
    metadata:&'a DraftingSourceRecord<M>,snapshot:&'a DraftingSourceRecord<S>)
    ->DraftingInitialInputSource<'a,M,S> {
    let source=DraftingInitialInputSource {input,metadata,snapshot};
    let copied=source;
    assert!(copied.metadata.is_same_record(metadata));
    copied
}
```

The provider's inherited exhaustive content walk includes all metadata collections,tokens,caches
and snapshots before lookup. `initial_input()` allocates/clones nothing and reads no authored or
computed value. Missing,retired,wrong-origin or unsupported sources retain original typed causes.
Shared access preserves immutable original records under the same honest integration contract as
the context port;this is no security boundary against fabricated metadata.

The canonical join proves membership in its supplied inventory,not its containing Design or actual
metadata ownership. Public raw-source construction also proves no containing Root/snapshot/name
association. The complete binding stage below compares the actual Root inventory,original
metadata declaration and snapshot record,retaining the provider's metadata source loan. Existing initial Point/Edge inputs use the
existing-geometry port separately;every programme creator must be rejected as initial geometry
before a live lookup. Namespace collisions,ordered visibility,values and whole acceptance remain
later owned stages. No raw response becomes an accepted prefix.


## Bind every extra-initial source

`BoundDraftingProgramOutputRoles::bind_initial_sources()` borrows the complete role proof and
reserves one result slot per original extra-initial occurrence before any initial-source query.
The earlier retained-work bound already includes these slots. Repeated declarations retain
separate slots;the empty vector performs no query. No values or namespace factory runs.

For a scalar/Length occurrence,the existing canonical join first resolves the original declaration
against this context's actual Root inventory. Original missing-family,scalar-kind or equal-ID
foreign Length-owner causes survive before the metadata callback. Unknown/Derived records stay
unread. The response then binds in this order:

1. Actual canonical inventory owner,not equal record IDs in a different inventory.
2. Original borrowed name field,not an equal-spelling replacement token.
3. Source family,origin,metadata ID,canonical target/kind and actual dedicated Length owner.
4. Original containing snapshot SourceRecord identity.

The complete response retains the provider's original opaque metadata record. Its obligation to
expose that actual declaration under that snapshot remains the honest immutable port contract;
core cannot inspect arbitrary opaque metadata payloads. There is no copied metadata registry or
source truth inferred from reported IDs alone.

For each Point/Edge occurrence,the binder scans **every** original native operation's creator ID
before that occurrence's live lookup,including operations without a named output. A matching
creator refuses with the original initial index,creator and genuine mixed position,even if that
reference is already present in the supplied snapshot. This prevents all-future initial geometry.
Other references reuse `DraftingContext::bind_existing_geometry()` and retain its exact reference,
frame,snapshot and opaque output loans or its original provider/mismatch cause.

```rust
use sc_core::{construction::{BoundDraftingProgramOutputRoles,DraftingInitialInputProvider},
    design::DesignMetadataHost};
fn bind_initials<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    roles:&BoundDraftingProgramOutputRoles<'_, '_, '_, '_, '_,H,S>)->Result<usize,String> {
    let bound=roles.bind_initial_sources().map_err(|error|error.to_string())?;
    Ok(bound.sources().len())
}
```

`BoundDraftingProgramInitials` privately retains the prior roles and complete original-order
`DraftingInitialBinding` vector. For `[Length,Count,Length,Point,Edge]`,the three metadata loans and
two existing-geometry loans retain that order. `roles()` reaches the exact positions,bounded work,
actual input and selected policy;`sources()` only borrows the complete result. The inspectable
binding enum alone cannot manufacture this private complete owner.

Any failure discards the successful local prefix. `DraftingInitialError` keeps original roles and
Allocation,Canonical,Source,Mismatch,PlannedCreator or Geometry causes with original indexes and
witnesses. Allocation keeps its TryReserveError inline;boxes compact semantic diagnostics only.
Debug omits authored names and opaque payloads. The [bounded actual initial namespace](formula-declarations.md#actual-mixed-program-initial-namespace)
checks reserved/cross-origin/repeated names. Earlier-only mixed visibility and whole kind checks
are implemented in [the ordered programme](formula-declarations.md#complete-dependency-composition);
complete geometry/semantic binding and values retain their separate stages.


## Source facet for unnamed earlier geometry

`DraftingGeometryStepProvider::planned_geometry(request)` is the readonly planned-family facet
for a genuine `DraftingGeometryStepRequest`. The original named-output port remains available.
An unnamed earlier output needs no invented global name or named-output occurrence.

The request retains its actual earlier normalized step,original creating input and persistent
reference. A variable request returns the existing `DraftingPlannedOutputSource`:an authentic
Offset,Slash,Truing,Mirror or Join field with original family parents and actual frame/snapshot
records. All held parents,indexes and duplicated snapshots belong in the provider's inherited
exhaustive resource walk. Providers and opaque records need neither Clone nor Debug.

```rust
use sc_core::{construction::*,design::DesignMetadataHost};
fn original_planned_field<'v,'i,H:DesignMetadataHost,S:DraftingGeometryStepProvider<H> + ?Sized>(
    source:&'v S,request:DraftingGeometryStepRequest<'v,'i>)
    ->DraftingPlannedOutputSourceResult<'v,S::Frame,S::Snapshot,S::Error> {
    source.planned_geometry(request)
}
```

Return an actual already-held family field or the original typed missing/unsupported/current-source
cause. No allocation,cloning,manufactured parent proof,authored value,existing-geometry lookup or
currentness mutation may create a response. A fixed-schema output needs no callback. The focused
raw-facet test requests an actual unnamed earlier Offset output,retains its original input and
frame/snapshot loans,and preserves an original provider refusal without translation.

This raw response is not source acceptance. Core calls remain pending until complete geometry
content/work is admitted. The later binding controller must check original input identity,exact
reference and original frame/snapshot records,then retain the complete result or first refusal.
The port grants no geometry work allowance,live planned output,physical result or whole programme.


## Fresh geometry content accounting

Geometry work admission reruns the original complete name/programme resource producers in one
fresh meter:Root/context/provider-held parents,raw/normalized sources and native/name fields all
participate. Prior successful demands supply no credit. This required readonly resource walk is
the only provider callback during admission;geometry lookup,currentness validation and values do
not run. A content shortage precedes even zero geometry quotas and retains its original field
cause. Source/order/graph/canonical proof buffers keep their separately admitted policies.
