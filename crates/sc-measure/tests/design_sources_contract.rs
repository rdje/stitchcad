//! Actual combined hosts expose complete source roles and retain current canonical object membership.
#![allow(clippy::expect_used)]
#[path = "support/owned_size_set_fixture.rs"]
mod support;
use sc_core::{
    design::{
        DesignHostEntity, DesignHostEntityKind as Kind, DesignMetadata, DesignMetadataError,
        DesignMetadataHost, DesignSizeSetHost, DesignSizeSetReference,
    },
    value::{
        CanonicalInputs, ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
        ScalarInputState,
    },
};
use sc_measure::*;
use sc_units::Count;
use support::id;
fn fixture() -> (CanonicalInputs, DesignSources) {
    let (canonical, measurements, mtm, garment) = support::inventory_fixture();
    let size_sets =
        SizeSetInventory::new(vec![mtm.into(), garment.into()], &canonical, &measurements)
            .expect("actual complete inventory");
    let sources = DesignSources::new(
        DesignSourcesDefinition {
            measurements,
            size_sets,
        },
        &canonical,
    )
    .expect("actual combined sources");
    (canonical, sources)
}
#[test]
fn actual_catalog_contains_all_owned_metadata_once_with_authored_role_positions() {
    let (canonical, sources) = fixture();
    let expected = [
        (20, Kind::Landmark, 0),
        (21, Kind::Landmark, 1),
        (22, Kind::Landmark, 2),
        (23, Kind::Landmark, 3),
        (24, Kind::Procedure, 0),
        (25, Kind::Procedure, 1),
        (1, Kind::Measurement, 0),
        (2, Kind::Measurement, 1),
        (3, Kind::Measurement, 2),
        (50, Kind::MeasurementTable, 0),
        (51, Kind::MeasurementTable, 1),
        (30, Kind::Ease, 0),
        (31, Kind::Ease, 1),
        (60, Kind::EaseSet, 0),
        (40, Kind::SizeSet, 0),
        (41, Kind::SizeMember, 0),
        (80, Kind::SizeAxis, 0),
        (81, Kind::SizeAxisValue, 0),
        (70, Kind::SizeChart, 0),
        (1000, Kind::SizeSet, 1),
        (1001, Kind::SizeMember, 1),
        (1002, Kind::SizeAxis, 1),
        (1003, Kind::SizeAxisValue, 1),
        (1004, Kind::SizeChartObservation, 0),
        (1005, Kind::SizeChartObservation, 1),
        (1006, Kind::SizeChart, 1),
    ]
    .map(|(identity, kind, index)| DesignHostEntity {
        id: id(identity),
        kind,
        index,
    });
    assert_eq!(sources.entities(), expected);
    let metadata = DesignMetadata::new(sources, &canonical).expect("actual one-way core admission");
    assert_eq!(metadata.entities(), &expected);
    let declaration = metadata
        .measurement_declaration(id(1), &canonical)
        .expect("actual body binding");
    assert!(core::ptr::eq(
        declaration,
        canonical.length(id(10)).expect("actual canonical record")
    ));
    assert_eq!(
        metadata.measurement_declaration(id(2000), &canonical),
        Err(DesignMetadataError::MissingMeasurement {
            measurement: id(2000)
        })
    );
    assert!(!format!("{metadata:?}").contains("Client"));
}
#[test]
fn actual_size_facet_requires_exact_id_revision_and_not_numeric_presence() {
    let (canonical, sources) = fixture();
    let actual = DesignSizeSetReference::new(id(40), Count::new(2));
    assert_eq!(sources.validate_size_set(actual, &canonical), Ok(()));
    let wrong = DesignSizeSetReference::new(id(40), Count::new(3));
    assert_eq!(
        sources.validate_size_set(wrong, &canonical),
        Err(DesignSourcesError::SizeSets(Box::new(
            SizeSetInventoryError::RevisionMismatch {
                expected: wrong,
                actual
            }
        )))
    );
    let absent = DesignSizeSetReference::new(id(2000), Count::new(0));
    assert!(canonical.scalar(id(2000)).is_some());
    assert_eq!(
        sources.validate_size_set(absent, &canonical),
        Err(DesignSourcesError::SizeSets(Box::new(
            SizeSetInventoryError::MissingSet { expected: absent }
        )))
    );
}
#[test]
fn core_reads_revalidate_whole_actual_host_and_preserve_unselected_set_cause() {
    let (canonical, sources) = fixture();
    let metadata = DesignMetadata::new(sources, &canonical).expect("original actual host");
    let replacement = CanonicalInputs::new(canonical.lengths().to_vec(), vec![])
        .expect("removed garment-only coordinate");
    let direct = metadata
        .host()
        .measurement_declaration(id(1), &replacement)
        .expect("targeted binding still valid");
    assert!(core::ptr::eq(
        direct,
        replacement
            .length(id(10))
            .expect("actual replacement record")
    ));
    assert!(
        matches!(metadata.measurement_declaration(id(1),&replacement),
        Err(DesignMetadataError::Host(DesignSourcesError::SizeSets(issue)))
        if matches!(issue.as_ref(),SizeSetInventoryError::InvalidSet {set,..} if set.id==id(1000)))
    );
    assert!(metadata.measurement_declaration(id(1), &canonical).is_ok());
}
#[test]
fn core_host_census_keeps_set_ownership_disjoint_from_replacement_canonical_ids() {
    let (canonical, sources) = fixture();
    let metadata = DesignMetadata::new(sources, &canonical).expect("original host");
    let mut scalars = canonical.scalars().to_vec();
    scalars.push(
        ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
            id: id(40),
            source: id(2020),
            kind: ScalarInputKind::Boolean,
            state: ScalarInputState::Unknown {
                observation: id(2021),
            },
        })
        .expect("detached current input claiming set identity"),
    );
    let replacement = CanonicalInputs::new(canonical.lengths().to_vec(), scalars)
        .expect("unique canonical identities");
    assert_eq!(
        metadata.validate_current(&replacement),
        Err(DesignMetadataError::CanonicalIdentityCollision {
            entity: DesignHostEntity {
                id: id(40),
                kind: Kind::SizeSet,
                index: 0
            }
        })
    );
}
