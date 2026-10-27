# Material property inputs

`Material::new(definition, &inputs)` implements structural [ontology §6](ontology.md#6-materials)
metadata over [canonical declarations](measurement-inputs.md). It preserves authored identity/source,
name and optional composition. Name and supplied composition must be nonblank; whitespace is retained.
None means composition or shrinkage is unprovided, never an inferred value.

## Typed properties and current checks

| Property | Required target | Current boundary |
| --- | --- | --- |
| Width | Dedicated Length or explicit general Length scalar | Present authored width must be positive |
| Nap, directional print, striped, checked | General Boolean scalars | Unknown/Derived stays unresolved; no false default |
| Optional X/Y shrinkage | Two general Ratio targets | Signed data and state retained; no applied scale |

CanonicalInputTarget pins family/id/kind. Dedicated and general Length representations remain distinct;
a missing dedicated id cannot resolve to a general Length peer. Wrong authored target kinds refuse
before lookup; missing/current-kind changes retain the actual role, family/id and expected/current kind.
Metadata id cannot alias a canonical declaration id. Current checks revisit all supplied properties,
then any available width. Known/Assumed/Preference widths require positive micrometres; Unknown/Derived
widths remain drafts. validate_width_result checks a supplied computed Length separately, without
performing computation or proving its provenance.

For example, width1 can be a dedicated unknown declaration while Boolean inputs2–5 retain their
observation requests. If width1 later becomes zero, validate_current refuses without clamping; the
held target remains inspectable. An explicit false nap Preference reads false with its provenance.
Changing that same-id declaration to Angle refuses instead of converting it.

## Authored draft example

This builder requires the caller's actual declaration1 (Length),2–5 (Boolean),6–7 (Ratio). Their
states remain canonical; fixture metadata supplies no measured facts or standard material vocabulary.

```rust
use sc_core::{ontology::{EntityId, Material, MaterialDefinition, MaterialError,
    MaterialShrinkage, MaterialShrinkageStage}, value::{CanonicalInputs,
    CanonicalInputTarget as Target, ScalarInputKind as Kind}};
fn draft(inputs: &CanonicalInputs) -> Result<Material, MaterialError> {
    let id = EntityId::from_bits;
    let scalar = |n, kind| Target::Scalar { declaration: id(n), kind };
    Material::new(MaterialDefinition {
        id: id(100), source: id(900), name: "Fixture fabric".into(), composition: None,
        width: Target::Length(id(1)), nap: scalar(2, Kind::Boolean),
        directional_print: scalar(3, Kind::Boolean), striped: scalar(4, Kind::Boolean),
        checked: scalar(5, Kind::Boolean),
        shrinkage: Some(MaterialShrinkage {
            x: scalar(6, Kind::Ratio), y: scalar(7, Kind::Ratio), treatment: id(901),
            stage: MaterialShrinkageStage::PostGradePreExport,
        }),
    }, inputs)
}
```

Shrinkage sources retain separate X/Y targets, treatment reference and explicit post-grade/pre-export
intent. None invents zero; requesting an absent shrinkage property refuses. A2% change is20,000ppm,
not a scale factor generated here. G4-PROFILES.13 owns transformation domains, application and policy.

## Inspection and proof limits

definition()/target() retain original metadata. property_declaration() checks one current target and
borrows its record, even an invalid width; validate_current() checks the whole Material. Detached edits require validated replacement. Default
Material/property-view Debug omits customer text/values; explicit definition/record access exposes them.

Design/G4 still owns source/evidence truth and profile resolution. Collection/assignment, geometry,
layout/dual grainlines, shrinkage execution and release readiness remain unimplemented here.

| API token | Meaning here |
| --- | --- |
| `Material` | Immutable structurally checked metadata |
| `MaterialDefinition` | Explicit identity/source/text and canonical property targets |
| `MaterialProperty` | Fixed width/flag/shrinkage role and expected kind |
| `MaterialPropertyDeclaration` | Actual dedicated-length or general-scalar borrow |
| `MaterialShrinkage` | Optional X/Y targets, treatment and explicit stage intent |
| `MaterialShrinkageStage` | Supported unapplied transformation stage |
| `PostGradePreExport` | Explicit post-grade/pre-export intent; no application |
| `MaterialTextField` | Required name or supplied composition |
| `MaterialError` | Actual text/identity/target/kind/width refusal |

Focused native8/privacy3 checks cover this component. Whole Design/integration/platform proof remains pending.
