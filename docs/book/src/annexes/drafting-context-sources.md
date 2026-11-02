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

### Next namespace stages — planned

The mixed namespace/controller is **planned**. The existing actual `DesignInputs::formula_namespace()`
factory checks current inputs and obtains actual parameter declarations,then selected measurement/Ease
host declarations. It supplies reserved metadata and checks collisions. This contract does not silently
supply a complete Profile/Material or geometry-name environment. Additional always-visible sources
and per-operation output names must remain explicit actual inputs with complete retained-content admission.

The existing scalar `FormulaNameCursor` retains its actual whole recipe and genuine scalar ordinal.
Its current-statement scope invokes the existing statement checker with original spans. Native
argument checkers now also accept the [borrowed visible-name scope](formula-name-scopes.md#borrow-visible-metadata-without-the-map)
and delegate to their same kernels. The cursor map stays private,including
against cloning a final namespace out of a prefix,and retain actual scalar declarations. Earlier
geometry insertion remains metadata until the complete controller checks every kind/dependency.

Planned output names must borrow their real creating step and manifest. Point,line,center-arc and
Bézier schemas expose fixed outputs through their actual public schema methods. Variable point/edge
roles have separate typed active-role catalogue views retaining original definitions and membership.
Local catalogue keys are not global formula names. Names and explicit tags are never generated from
vector order,guessed from an operation family or treated as live geometry in the current snapshot.
Catalogue metadata alone grants no canonical history/frame/currentness or evaluated-presence proof.

Borrowed visible-name/kernel adapters and raw initial/output-name owners are implemented;remaining stages
cover fresh complete held-content/prospective name-work admission and ordered metadata integration. Complete
program and name/source/catalogue fields,including all retained definitions,active lists,indexes and
text,are admitted before namespace callbacks or child checking. Existing geometry still requires the
actual context's exact source binding;planned geometry stays tied to its earlier original step/manifest.
The resulting metadata stage is not an accepted program. Complete kinds,dependencies and source truth
remain in their separately owned checking stages before values,followed by coupled feature verification.

### Explicit raw name and output sources

`DraftingProgramNameInputs::new(program,initial,outputs)` now retains the actual normalized program
loan and both complete original vectors. `initial()` borrows explicit extra `FormulaInitialDeclaration`
locators;the Root-derived parameter/measurement/Ease environment and reserved metadata compose later.
These remain metadata claims with their original names and source records;construction does not
validate canonical targets,existing geometry,source truth or collisions. Borrowed tokens/records
cannot be replaced while the names are used. `outputs()` borrows all original named output occurrences.

Each `DraftingOutputName::new(position,name,source)` owns an explicit global `MachineToken`,keeps
the requested one-based combined creating position and borrows a `DraftingGeometryNameSource`:
point,line,center-arc,Bézier,active point role or active edge role. Fixed variants retain the actual
original schema and use its published `output()`;variable variants retain authentic typed active
catalogue membership,its complete catalogue and actual definition/tag. `source().reference()` exposes
that exact persistent reference without allocating an identity or constructing geometry.

A global name such as `front_waist` can deliberately differ from a local catalogue role key such as
`waist_corner`;neither spelling is generated from the other. `declaration()` borrows the actual global
name and carries its source's PointRef/EdgeRef as raw formula metadata. No anonymous caller-provided
output reference replaces the original schema/catalogue handle. The role catalogue's metadata
admission still proves neither actual creating-step association nor canonical history/frame/presence.

```rust
use sc_core::{construction::{DraftingContextSource, DraftingGeometryNameSource, DraftingOutputName,
    DraftingProgramNameInputs, NormalizedDraftingProgram, PointConstruction},
    design::DesignMetadataHost, name::{MachineToken, MachineTokenError}};
fn raw_names<'n,'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H> + ?Sized>(
    program:&'n NormalizedDraftingProgram<'p,'a,H,S>,point:&'n PointConstruction)
    -> Result<DraftingProgramNameInputs<'n,'p,'a,H,S>,MachineTokenError> {
    let output=DraftingOutputName::new(2,MachineToken::new("front_waist")?,
        DraftingGeometryNameSource::Point(point));
    let names=DraftingProgramNameInputs::new(program,vec![],vec![output]);
    assert!(std::ptr::eq(names.program(),program));
    Ok(names)
}
```

The helper above requests position2 explicitly;it does not check whether that program position is
this point. Zero,repeated,out-of-range,colliding or mismatched records remain inspectable in raw inputs
for the later complete admission/association stages. Empty vectors still pin the actual program.
Constructors clone neither vectors nor documents,create no namespace,perform no callbacks or values,
and expose no accepted name/program prefix. Default Debug shows counts/positions/source families,
without authored names or formulas. Complete fresh held-content/name-work admission and ordered
visibility remain the next implementation stages.

### Additional held-field charges

`DraftingProgramNameInputs::charge_extra_resources(sink)` now charges its complete additional input
component. The initial vector precedes original name text and actual canonical Length evidence
vectors;fixed declaration/source identifiers carry no extra dynamic content. The output vector
precedes each explicit global token and its original source. Fixed schemas reuse their existing
complete held-field producers,including both buffers of every Point/Arc argument document.

A typed active-role source charges its whole actual catalogue,including inactive definitions and
keys,original active list,retained tag index,active index and active associations. Complete collection
lengths are charged before nested keys;prior peak admission-work counts do not substitute for actual
held fields. For two definitions and one active role,the actual raw2+1 and retained2+1+1 populations
are seven entries. Reusing that catalogue twice counts both logical source occurrences;no address or
identity deduplication removes input. Point/edge domains retain distinct static field paths.

For example,one initial Length name with two evidence entries,then two output names using that same
catalogue,requires `1 + 2 + 2 + 7 + 7 = 19` entries. All inactive and repeated key text counts too.
Resource locations preserve actual metadata/canonical/operation owners and collection-specific
original indexes. Empty components preserve an earlier latched refusal. The walk clones/formats
nothing,looks up no source,reads no numeric availability and grants no whole-name/program acceptance.

```rust
use sc_core::{construction::{DraftingContextSource, DraftingProgramNameInputs},
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError,
    DesignResourceLimits, DesignResourceMeter}};
fn extra_demand<H:DesignMetadataHost,S:DraftingContextSource<H> + ?Sized>(
    names:&DraftingProgramNameInputs<'_, '_, '_,H,S>,policy:DesignResourceLimits)
    -> Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(policy);
    names.charge_extra_resources(&mut meter)?;
    meter.finish()
}
```

This demand covers additional inputs;the actual containing Root/context/raw program and retained
normalized arenas compose separately. Fresh complete composition,prospective name-work bounds and
creating-step associations remain owned admission stages before namespace construction/checking.

### Complete retained program charges

`NormalizedDraftingProgram::charge_resources(sink)` composes a fresh walk of the actual Root,
context source,mixed vector and native fields with all retained source,scalar and native arenas.
It precharges each actual source/statement/slot/node vector and each nested call-index vector.
Borrowed names/source text and fixed Copy literals own no additional text or counter buffers.
Repeated native steps count their separate arenas;unused sources and untaken branches count too.

For the complete seven-step example above,the actual extra population is source owners2,source
nodes11,call indices2,scalar statements2,scalar nodes8,mixed slots7 and native nodes12:44 entries.
The conservative84-record normalization allowance is a different quantity and never substitutes
for this held-content walk. Source locations retain source identity;native locations retain actual
operation identity and original zero-based mixed index. A nested call-vector location uses its
semantic node index. Empty programs still charge their actual Root/context and preserve first refusal.

```rust
use sc_core::{construction::{DraftingContextSource, NormalizedDraftingProgram},
    design::{DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
    DesignResourceMeter, DesignResourceSource}};
fn program_demand<H:DesignResourceSource,S:DraftingContextSource<H> + ?Sized>(
    program:&NormalizedDraftingProgram<'_, '_,H,S>,policy:DesignResourceLimits)
    -> Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(policy);
    program.charge_resources(&mut meter)?;
    meter.finish()
}
```

This producer performs no normalization,source currentness,geometry lookup or numeric evaluation.
The additional-name producer composes separately into the same cumulative sink;complete content
and prospective name-work bounds must still precede namespace construction and ordered checking.

### One cumulative complete name-content walk

`DraftingProgramNameInputs::charge_resources(sink)` now composes the two complete producers in
order:actual normalized program first,additional initial/output names and their original sources
second. The same sink retains all earlier consumption and its original first refusal. Repeated
source content counts in both logical occurrences;no old admitted demand provides credit.

For the six-source example,the empty program's normalized vectors add zero entries but its actual
Root/context remain. The17 additional entries and all name/document/key bytes follow those fields.
A budget one entry below the complete sum refuses the final edge-role association at output index5
with the original operation owner;its consumed count includes the earlier Root/context entries.
A Root text refusal precedes context charging and every additional-name field. Empty inputs still
preserve an earlier cumulative failure.

```rust
use sc_core::{construction::{DraftingContextSource, DraftingProgramNameInputs},
    design::{DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
    DesignResourceMeter, DesignResourceSource}};
fn complete_name_demand<H:DesignResourceSource,S:DraftingContextSource<H> + ?Sized>(
    names:&DraftingProgramNameInputs<'_, '_, '_,H,S>,policy:DesignResourceLimits)
    -> Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(policy);
    names.charge_resources(&mut meter)?;
    meter.finish()
}
```

Success accounts held logical content only. It performs no namespace construction,position/source
association,currentness check or value query. Prospective namespace work and whole admission remain
the next stage;this producer introduces no alternate Root or caller-provided demand.
