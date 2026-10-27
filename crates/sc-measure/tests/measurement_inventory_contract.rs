//! Owned metadata uses actual existing current-reference checks and singly owned core values.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
        ScalarInputState,
    },
};
use sc_measure::{
    Landmark, LandmarkDefinition, Measurement, MeasurementContext, MeasurementDefinition,
    MeasurementError, MeasurementInventory as Inventory,
    MeasurementInventoryDefinition as Definition, MeasurementInventoryError as Error,
    MeasurementInventoryPosition as Position, MeasurementKind as Kind, MeasurementProcedure,
    MeasurementProcedureDefinition, MeasurementTable, MeasurementTableContext,
    MeasurementTableDefinition, MeasurementTableError,
};
use sc_units::{Length, Unit};

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn token(name: &str) -> MachineToken {
    MachineToken::new(name).expect("fixture token")
}
fn inputs(state: LengthState) -> CanonicalInputs {
    CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(1),
            source: id(300),
            state,
        })
        .expect("explicit canonical fixture state")],
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(2),
                source: id(301),
                kind: ScalarInputKind::Boolean,
                state: ScalarInputState::Unknown {
                    observation: id(302),
                },
            })
            .expect("distinct general fixture scalar"),
        ],
    )
    .expect("unique owned canonical inputs")
}
fn unknown() -> CanonicalInputs {
    inputs(LengthState::Unknown {
        observation: id(303),
    })
}
fn fixture(inputs: &CanonicalInputs) -> Definition {
    let landmarks = [10, 11]
        .into_iter()
        .map(|identity| {
            Landmark::new(LandmarkDefinition {
                id: id(identity),
                name: "caller-authored fixture level".into(),
                kind: Kind::Body,
                source: id(304),
            })
            .expect("explicit fixture landmark")
        })
        .collect::<Vec<_>>();
    let procedures = vec![MeasurementProcedure::new(MeasurementProcedureDefinition {
        id: id(20),
        name: "fixture procedure".into(),
        kind: Kind::Body,
        documentation:
            "Retain caller-authored fixture instructions without a physical repeatability claim."
                .into(),
        source: id(305),
    })
    .expect("documented fixture procedure")];
    let records = MeasurementContext::new(inputs.lengths(), &landmarks, &procedures)
        .expect("fixture target records");
    let measurements = [30, 31]
        .into_iter()
        .map(|identity| {
            Measurement::new(
                MeasurementDefinition {
                    id: id(identity),
                    name: "same display label is legal".into(),
                    token: token("girth"),
                    entered_unit: Unit::Inch,
                    kind: Kind::Body,
                    landmarks: [id(10), id(11)],
                    procedure: id(20),
                    declaration: id(1),
                },
                &records,
            )
            .expect("valid born fixture measurement")
        })
        .collect::<Vec<_>>();
    let context = MeasurementTableContext::new(&measurements, &records)
        .expect("fixture measurement inventory");
    let tables = [40, 41]
        .into_iter()
        .zip(&measurements)
        .map(|(identity, measurement)| {
            MeasurementTable::new(
                MeasurementTableDefinition {
                    id: id(identity),
                    name: "fixture table".into(),
                    entries: vec![measurement.into()],
                },
                &context,
            )
            .expect("distinct tables may bind one spelling to different measurements")
        })
        .collect();
    Definition {
        landmarks,
        procedures,
        measurements,
        tables,
    }
}
fn measurement_error(issue: MeasurementError) -> Error {
    Error::Measurement {
        measurement: id(30),
        issue: Box::new(issue),
    }
}

#[test]
fn exact_metadata_order_documents_and_table_scoped_tokens_survive_with_canonical_borrows() {
    let canonical = unknown();
    let supplied = fixture(&canonical);
    let original = supplied.clone();
    let inventory = Inventory::new(supplied, &canonical).unwrap();
    assert_eq!(inventory.definition(), &original);
    assert!(core::ptr::eq(
        inventory.measurement(id(30)).unwrap(),
        inventory.definition().measurements.first().unwrap()
    ));
    assert!(core::ptr::eq(
        inventory.table(id(41)).unwrap(),
        inventory.definition().tables.last().unwrap()
    ));
    assert!(inventory.table(id(99)).is_none());
    assert!(inventory.measurement(id(99)).is_none());
    let records = inventory.record_context(&canonical).unwrap();
    let context =
        MeasurementTableContext::new(&inventory.definition().measurements, &records).unwrap();
    for table in &inventory.definition().tables {
        let declaration = table.declaration(&token("girth"), &context).unwrap();
        assert!(core::ptr::eq(declaration, canonical.length(id(1)).unwrap()));
        assert!(declaration.authored_value().is_err());
    }
}

#[test]
fn repeated_table_identity_reports_original_positions_before_other_reference_checks() {
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    definition
        .tables
        .push(definition.tables.first().unwrap().clone());
    definition.landmarks.clear();
    assert_eq!(
        Inventory::new(definition, &canonical),
        Err(Error::DuplicateIdentity {
            identity: id(40),
            first: Position::Table { index: 0 },
            duplicate: Position::Table { index: 2 },
        })
    );
}

#[test]
fn cross_role_metadata_and_general_scalar_identity_collisions_retain_actual_roles() {
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    definition.landmarks.insert(
        0,
        Landmark::new(LandmarkDefinition {
            id: id(20),
            name: "other landmark".into(),
            kind: Kind::Body,
            source: id(306),
        })
        .unwrap(),
    );
    assert_eq!(
        Inventory::new(definition, &canonical),
        Err(Error::DuplicateIdentity {
            identity: id(20),
            first: Position::Landmark { index: 0 },
            duplicate: Position::Procedure { index: 0 },
        })
    );
    let definition = fixture(&canonical);
    let mut scalars = canonical.scalars().to_vec();
    let mut alias = scalars.first().unwrap().definition().clone();
    alias.id = id(10);
    scalars.push(ScalarInputDeclaration::new(alias).unwrap());
    let current = CanonicalInputs::new(canonical.lengths().to_vec(), scalars).unwrap();
    assert_eq!(
        Inventory::new(definition, &current),
        Err(Error::DuplicateIdentity {
            identity: id(10),
            first: Position::Scalar { index: 1 },
            duplicate: Position::Landmark { index: 0 },
        })
    );
}

#[test]
fn all_owned_measurements_require_actual_targets_even_when_no_table_selects_them() {
    let canonical = unknown();
    for (remove_landmark, expected) in [
        (true, MeasurementError::MissingLandmark(id(10))),
        (false, MeasurementError::MissingProcedure(id(20))),
    ] {
        let mut definition = fixture(&canonical);
        definition.tables.clear();
        if remove_landmark {
            definition.landmarks.clear();
        } else {
            definition.procedures.clear();
        }
        let error = Inventory::new(definition, &canonical).unwrap_err();
        assert_eq!(error, measurement_error(expected));
        assert!(std::error::Error::source(&error).is_some());
    }
    let definition = fixture(&canonical);
    let current = CanonicalInputs::new(
        vec![],
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(1),
                source: id(300),
                kind: ScalarInputKind::Length,
                state: ScalarInputState::Unknown {
                    observation: id(303),
                },
            })
            .unwrap(),
        ],
    )
    .unwrap();
    assert_eq!(
        Inventory::new(definition, &current),
        Err(measurement_error(MeasurementError::MissingDeclaration(id(
            1
        ))))
    );
}

#[test]
fn same_id_landmark_or_procedure_domain_changes_use_original_current_validation() {
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    let mut landmark = definition.landmarks.first().unwrap().definition().clone();
    landmark.kind = Kind::Garment;
    definition.landmarks.remove(0);
    definition
        .landmarks
        .insert(0, Landmark::new(landmark).unwrap());
    assert_eq!(
        Inventory::new(definition, &canonical),
        Err(measurement_error(MeasurementError::LandmarkKindMismatch {
            measurement: id(30),
            landmark: id(10),
            expected: Kind::Body,
            actual: Kind::Garment,
        }))
    );
    let mut definition = fixture(&canonical);
    let mut procedure = definition.procedures.first().unwrap().definition().clone();
    procedure.kind = Kind::Garment;
    definition.procedures.clear();
    definition
        .procedures
        .push(MeasurementProcedure::new(procedure).unwrap());
    assert_eq!(
        Inventory::new(definition, &canonical),
        Err(measurement_error(MeasurementError::ProcedureKindMismatch {
            measurement: id(30),
            procedure: id(20),
            expected: Kind::Body,
            actual: Kind::Garment,
        }))
    );
}

#[test]
fn stale_table_bindings_do_not_transfer_to_a_same_token_peer() {
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    let mut changed = definition
        .measurements
        .first()
        .unwrap()
        .definition()
        .clone();
    changed.token = token("renamed");
    let records = MeasurementContext::new(
        canonical.lengths(),
        &definition.landmarks,
        &definition.procedures,
    )
    .unwrap();
    let replacement = Measurement::new(changed, &records).unwrap();
    definition.measurements.remove(0);
    definition.measurements.insert(0, replacement);
    let error = Inventory::new(definition, &canonical).unwrap_err();
    assert_eq!(
        error,
        Error::Table {
            table: id(40),
            issue: Box::new(MeasurementTableError::TokenMismatch {
                table: id(40),
                measurement: id(30),
                expected: token("girth"),
                actual: token("renamed"),
            })
        }
    );
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn same_id_canonical_state_edits_are_current_without_copying_or_reconverting_entered_units() {
    let canonical = unknown();
    let inventory = Inventory::new(fixture(&canonical), &canonical).unwrap();
    let original = inventory.definition().clone();
    let value = Length::from_micrometres(-5).unwrap();
    let current = inputs(LengthState::Assumed {
        value,
        assumption: id(307),
    });
    inventory.validate_current(&current).unwrap();
    let records = inventory.record_context(&current).unwrap();
    let context =
        MeasurementTableContext::new(&inventory.definition().measurements, &records).unwrap();
    let actual = inventory
        .table(id(40))
        .unwrap()
        .declaration(&token("girth"), &context)
        .unwrap();
    assert!(core::ptr::eq(actual, current.length(id(1)).unwrap()));
    assert_eq!(actual.authored_value().unwrap(), value);
    assert_eq!(
        inventory
            .measurement(id(30))
            .unwrap()
            .definition()
            .entered_unit,
        Unit::Inch
    );
    assert_eq!(inventory.definition(), &original);
    assert!(canonical.length(id(1)).unwrap().authored_value().is_err());
    let empty = CanonicalInputs::new(vec![], vec![]).unwrap();
    assert_eq!(
        inventory.validate_current(&empty),
        Err(measurement_error(MeasurementError::MissingDeclaration(id(
            1
        ))))
    );
}

#[test]
fn empty_drafts_and_detached_edits_mint_no_content_and_do_not_mutate_owned_metadata() {
    let empty = CanonicalInputs::new(vec![], vec![]).unwrap();
    let definition = Definition {
        landmarks: vec![],
        procedures: vec![],
        measurements: vec![],
        tables: vec![],
    };
    assert_eq!(
        Inventory::new(definition.clone(), &empty)
            .unwrap()
            .definition(),
        &definition
    );
    let canonical = unknown();
    let inventory = Inventory::new(fixture(&canonical), &canonical).unwrap();
    let mut detached = inventory.definition().clone();
    detached.tables.clear();
    assert_eq!(inventory.definition().tables.len(), 2);
    assert_ne!(&detached, inventory.definition());
}
