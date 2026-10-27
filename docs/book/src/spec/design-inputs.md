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
