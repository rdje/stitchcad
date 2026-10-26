# Typed artifact requests

sc-core provides DXF, PLT, PDF and tech-pack request schemas plus per-artifact uncertainty inspection.
[Interchange](interchange-dialects.md) defines future writer behavior.

## Request one named DXF target

`ExportDxf::single_instance(instance, target, selected, parameters)` names the Instance, dialect,
exact profile/schema/content pin and resolved parameter-result identity.
`DxfTarget::AamaNamedR12` names `dxf-aama-named`;
`DxfTarget::AstmNumberedR13` names `dxf-astm-num-r13`. Both carry the contract's polyline-only policy.
`ALL` lists both; `from_token` requires exact spelling. Declarations prove no receiver compatibility.

Execution must resolve the actual Instance and selected result against the requested Design snapshot.
Units, cut/sew mapping, allowance policy and other profile bindings come from that result; the request
copies no values or uncertainty states. Missing bindings require the corresponding diagnostic/policy,
never a writer default. A claimed result identity does not certify constraints, provenance or evidence.

```rust
use sc_core::{command::{ArtifactPayload, CommandActorClaim, CommandArtifact, CommandQuery,
    CommandSnapshotContext, DesignRevision, DxfTarget, ExportDxf, QueryPayload},
    ontology::{EntityId, SelectedProfilePin, SelectedProfilePinDefinition}};
let id = EntityId::from_bits;
let selected = SelectedProfilePin::new(SelectedProfilePinDefinition {
    profile_id: "acme-cmt-pt".into(), schema_version: "2.0".into(), content_version: "1.3.0".into(),
})?;
let context = CommandSnapshotContext::new(id(1), CommandActorClaim::Human(id(2)),
    DesignRevision::from_counter(7));
let generation = CommandArtifact::new(context.clone(), ArtifactPayload::ExportDxf(
    ExportDxf::single_instance(id(3), DxfTarget::AamaNamedR12, selected, id(4))));
let inspection = CommandQuery::new(context, QueryPayload::GetUncertaintyClosure {
    artifact: generation.payload().clone(),
});
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Inspect the same requested artifact

`GetUncertaintyClosure { artifact }` carries the identical `ArtifactPayload` used for generation.
It names the configuration; a format token or all Design unknowns cannot substitute for actual dependencies.
A future handler must compute and record that closure. This constructor creates no files or observable cache.

Closure inspection requires inspect; generation requires generate. Both preserve exact snapshot claims.
Constructors grant no authority and execute no handler.

## Registered graded carriages

`GradedSizeSelection::new(set, revision, members)` retains a nonempty ordered member list at one
exact SizeSet `Count` revision. `NoMembers` refuses empty input; `DuplicateMember` retains the first
repeated identity and zero-based positions. It infers neither labels, a base, quantities nor completeness.

With explicit authored bindings:

```rust,ignore
let sizes = GradedSizeSelection::new(set, revision, vec![small, medium])?;
let aama = ExportDxf::base_plus_rules(base_instance, rules, sizes.clone(), pin.clone(), parameters);
let astm = ExportDxf::embedded_rules(base_instance, rules, sizes, pin, parameters);
```

The constructors fix AAMA/R12 plus companion `.rul`, or ASTM/R13 embedded rules, respectively.
`instance()` retains the supplied base; `grading()` exposes its rule table and size selection.
Single-instance requests retain `SingleSize`. Closure inspection carries these same inputs.
Execution must validate actual membership, base/rule/break provenance and profile bindings before
serialization. No deltas are derived from chart differences. All-contours needs its separately
registered/validated target; no writer flag silently adds one.

## Plotter and vector-print requests

`ExportPlt::new(instance, selected_pin, parameters)` names target `plt` and the exact resolved profile
result supplying pen-per-function mapping, declared scale and calibration. The request copies no pen
values or uncertainty states. Execution must validate those actual bindings and provenance.

`ExportPdf::new(instance, settings, profile)` names `pdf-a0`, whose contract also admits tiled A3/A4.
The profile choice is explicit `EvaluationProfile::Logical` or `FactoryResolved`; logical selection
supplies no unbound factory quantity. `PdfPrintSettings::new(layout, orientation, scale_square)` retains:

- `PdfLayout::A0`, or `Tiled { paper, overlap, order }` with A3/A4 paper;
- portrait/landscape orientation and row-major/column-major tile traversal;
- positive overlap for tiling and an exact positive scale-square side, both as `Length`.

```rust
use sc_core::command::{PdfLayout, PdfOrientation, PdfPrintSettings, PdfTileOrder, PdfTilePaper};
use sc_units::Length;
let settings = PdfPrintSettings::new(PdfLayout::Tiled { paper: PdfTilePaper::A4,
    overlap: Length::from_micrometres(15_000)?, order: PdfTileOrder::RowMajor },
    PdfOrientation::Portrait, Length::from_micrometres(100_000)?)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

`NonPositiveScaleSquare` and `NonPositiveOverlap` retain refused values, with no clamping.
These are authored output choices. They certify neither page fit nor printer calibration.
The writer must preserve 1:1 geometry, registration marks, page order and piece identity; a nonfitting
A0 piece refuses instead of silent fitting. Rendering, physical scale checks and unknown-value policy
remain later work. PLT/PDF closure queries carry the same concrete input/settings as generation.

## Canonical tech-pack inputs

`GenerateTechPack::new(InstanceSelection::new(ids)?, profile)` names ordered evaluated sources and an
explicit logical/factory selection. `NoInstances` refuses empty input; `DuplicateInstance` retains the
first repeated id/positions. Lookup, provenance and content coverage remain execution checks.

The shared Design snapshot supplies authored materials/trims, notes, stitch/SPI and POM knowledge;
actual Instances supply evaluated content. The request carries no report edits or invented values.
Piece cut quantities come from cut plans. Unavailable Order size-run quantities remain unresolved.
Generation/closure retain the same inputs; required missing content must produce a diagnostic.
