# Owned Design metadata sources

`DesignMetadata<H>` owns a DesignMetadataHost and its identity census; CanonicalInputs owns values.
MeasurementInputs implements this core port without reversing dependencies. Full Design assembly
follows; admission selects no table, SizeSet or Factory Profile.

## Admission and current reads

`DesignMetadata::new(host, &canonical)` checks the host's stable census, duplicate metadata ids and
collisions with either canonical declaration family, then calls the host's actual current validator.
An explicitly empty host is legal. Referenced input/evidence ids are uses, not metadata owners.
Family and authored positions are retained in collision witnesses; records are never deduplicated.

`measurement_declaration(measurement, &canonical)` repeats whole-scope validation, requires actual
Measurement metadata, then resolves its explicit binding through the host. The returned object must
be the actual dedicated canonical length in this supplied inventory. A detached equal copy refuses.
A canonical id cannot invent metadata or bindings. Unknown/Derived stays unchanged. Replacement
inputs require current validation. The returned borrow retains host and canonical context.

```rust
use sc_core::{
    design::{DesignMetadata, DesignMetadataError, DesignMetadataHost},
    ontology::EntityId,
    value::CanonicalInputs,
};
fn inspect<H: DesignMetadataHost>(host: H, canonical: &CanonicalInputs, measurement: EntityId)
    -> Result<(), DesignMetadataError<H::Error>>
{
    let metadata = DesignMetadata::new(host, canonical)?;
    let declaration = metadata.measurement_declaration(measurement, canonical)?;
    assert!(canonical.length(declaration.id()).is_some_and(|actual|
        core::ptr::eq(actual, declaration)));
    Ok(())
}
```

The host must supply an immutable complete snapshot and actual semantic validation. Core checks
observable identities/object membership; it cannot prove arbitrary provider binding semantics.
There is no stored self-reference or copied value/state cache. host() and entities() are shared views.
Default Debug exposes only the entity count.

## Public vocabulary and boundary

| API or variant | Meaning |
| --- | --- |
| `DesignMetadataHost` | Readonly higher metadata contract with original typed Error |
| `DesignMetadata` | Private checked host owner; admission is scoped to these canonical inputs |
| `DesignHostEntity` | Actual owned id, family and original within-family index |
| `DesignHostEntityKind` | Landmark, Procedure, Measurement, MeasurementTable, Ease or EaseSet |
| `DesignMetadataError` | Current scope or explicit binding refusal |
| `DuplicateIdentity` | Two actual metadata positions claim one id |
| `CanonicalIdentityCollision` | Metadata claims a canonical input id |
| `ChangedEntities` | Provider violated the stable census contract |
| `Host` | Original typed provider refusal, also retained as error source |
| `MissingMeasurement`, `WrongEntityKind` | Absent metadata or an actual different role |
| `NoncanonicalDeclaration` | Returned declaration is detached from the supplied canonical inventory |

Other Design identities, SizeSet metadata, formula source composition and whole readiness retain
separate owners. Profile ids, labels and result claims cannot resolve physical fields, including when
an id coincides with a numeric declaration; [G4 profile boundaries](material-inputs.md) still apply.
Focused native and privacy checks cover this port; full Design execution and integration remain
unverified. Run the focused contract through the local producer profile:

```bash
cargo test -p sc-core --test design_metadata_contract
```

## Actual measurement host

MeasurementInputs implements the port with owned landmarks, procedures, measurements, tables,
Ease mappings and sets, in that order and their authored order within each family. Referenced
canonical/evidence ids are excluded. Whole validation uses its actual current validators, including
unused mappings and compression permission. Measurement bindings use actual metadata and the
borrowed record context; a general scalar peer cannot replace a dedicated length target.

| API or variant | Meaning |
| --- | --- |
| `MeasurementHostError` | Original typed sc-measure port refusal |
| `Inputs`, `RecordContext`, `Measurement` | Whole inventory, target context or explicit binding cause |
| `MissingMeasurement` | Actual measurement metadata is absent |

The direct trait binding method checks the selected declaration/context only. DesignMetadata adds
whole-scope validation and canonical object membership. No borrowed context is stored.

```bash
cargo test -p sc-measure --test measurement_inputs_contract
```

## Owned common SizeSet metadata

SizeSetMetadata owns exact membership, axes/member points, required dimensions/available breaks
and an explicit Design table id. This is the common external SizeSet source component; chart-family
ownership and full Design composition follow. Canonical values and measurement/Ease metadata stay
in their original supplied owners. Empty break inventories can be valid drafts, without coverage.

new(definition, &canonical, &inputs) checks all owned ids across canonical/measurement/Ease records,
then the set, members, each axis/its values and breaks in authored order. Even an unselected table
cannot reuse a member/axis/value/break id. Source references are uses, not extra declarations.
It then invokes actual current input/context/axes/break validators and requires the exact Design
table. Equal-content tables cannot replace a missing id. Replacement canonical inputs are rechecked.

with_context validates that whole scope before visiting a temporary SizeBreakContext. The callback
cannot retain the local contexts; owned results may be returned. No self-reference or value/state
cache is stored. Original canonical declarations are borrowed, including Unknown/Derived. Failures
skip the callback and preserve the typed cause. Break declaration error sources expose the concrete
SizeBreakError, including MissingDelta, rather than its allocation wrapper (D255 correction).

| API or variant | Meaning |
| --- | --- |
| `SizeSetMetadata`, `SizeSetMetadataDefinition` | Private common owner and explicit editable construction input |
| `SizeSetMetadataPosition` | Original ownership witness: Inputs, Set, Member, Axis, AxisValue, Break |
| `SizeSetMetadataError` | DuplicateIdentity, Inputs, MissingDesignTable, RecordContext, TableContext, Sources, BreakContext or Breaks |

identities() lists only common owned ids; definition() is shared. Debug exposes counts only.
validate_current checks current structure; chart coverage, grade coverage, evaluation and readiness
remain separate. For example, an authored required waist break with no available delta can be saved;
require_grade_coverage still refuses its missing cell. No zero delta is invented.

```bash
cargo test -p sc-measure --test size_set_metadata_contract
```

## Owned garment SizeSet sources

GarmentSizeSet owns common metadata, all observations and the chart. Construction checks source ids,
actual common sources, every observation and the original composite. A missing unused observation
table refuses. Design references this external object; no values/contexts are copied or stored.

with_inputs revalidates before visiting temporary GarmentSizeSetInputs. Readonly chart_context()/
break_context() expose actual shared contexts. Inside the callback, the chart's declaration(member,
pom, view.chart_context()) and a break's delta(view.break_context()) borrow original canonical data.
Owned results may return; local views/contexts cannot escape.

| API or variant | Meaning |
| --- | --- |
| `GarmentSizeSet`, `GarmentSizeSetDefinition` | Private garment source owner and exact construction input |
| `GarmentSizeSetPosition` | Common, Observation or Chart original ownership witness |
| `GarmentSizeSetError` | DuplicateIdentity, Metadata, Context, Observation, CollectionContext or Inputs cause |

Partial chart/break drafts remain valid. validate_inputs_for requires the original structural
coverage for Regeneration or GradeRules; it computes no geometry or unknown quantity. reference()
retains the actual set/revision, identities() lists owned ids, and Debug reveals observation count.
MTM/unified source-host composition and full Design/execution remain later owners.

```bash
cargo test -p sc-measure --test garment_size_set_contract
```

## Owned MTM SizeSet sources

MtmSizeSet owns singleton common metadata and the actual MtmChart. MeasurementInputs retains actual
body/Ease/table/set ownership. Construction checks complete input/common/chart ids and reuses the
original validators through temporary contexts sharing actual membership, records and Design table.
No body-plus-Ease value, profile selection or context cache is created.

with_inputs revalidates before visiting MtmSizeSetInputs. Its readonly chart_context()/break_context()
permit explicit quantity/coverage queries. The chart's body_declaration(pom, context) and
ease_declaration(pom, context) borrow separate actual canonical records. Local views cannot escape;
owned callback results may return. Missing replacement sources refuse before the callback.

| API or variant | Meaning |
| --- | --- |
| `MtmSizeSet`, `MtmSizeSetDefinition` | Private external MTM source owner and exact construction input |
| `MtmSizeSetPosition` | Common or Chart ownership witness |
| `MtmSizeSetError` | DuplicateIdentity, Metadata, Eases, Context or Inputs original cause |

Partial mappings stay drafts; Regeneration requires coverage, while GradeRules keeps the original
GradeRulesUnsupported chart/member refusal. Composite SizeSetInputError sources expose concrete
chart/MTM/break causes rather than boxes (D256 correction). No numeric execution/readiness follows.

```bash
cargo test -p sc-measure --test mtm_size_set_contract
```
