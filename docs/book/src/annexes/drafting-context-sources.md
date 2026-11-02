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
