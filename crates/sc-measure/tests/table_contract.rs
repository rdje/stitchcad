//! Named table binding, current reassignment and canonical-input regression contracts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError},
};
use sc_measure::{
    Landmark, LandmarkDefinition, Measurement, MeasurementBinding, MeasurementContext,
    MeasurementDefinition, MeasurementError, MeasurementKind, MeasurementProcedure,
    MeasurementProcedureDefinition, MeasurementTable, MeasurementTableContext,
    MeasurementTableDefinition, MeasurementTableError,
};
use sc_units::{Length, Unit};
use std::error::Error;

fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn token(name: &str) -> MachineToken {
    MachineToken::new(name).unwrap()
}
struct Fixture {
    declarations: Vec<LengthDeclaration>,
    landmarks: Vec<Landmark>,
    procedures: Vec<MeasurementProcedure>,
    measurements: Vec<Measurement>,
}
impl Fixture {
    fn new() -> Self {
        let mut fixture = Self {
            declarations: [10, 11].into_iter().map(|n| LengthDeclaration::new(LengthDeclarationDefinition {
                id: id(n), source: id(90), state: LengthState::Assumed { value: Length::from_micrometres(740_000).unwrap(), assumption: id(91) },
            }).unwrap()).collect(),
            landmarks: [20, 21, 22, 23].into_iter().map(|n| Landmark::new(LandmarkDefinition {
                id: id(n), name: format!("Fixture landmark {n}"), kind: if n < 22 { MeasurementKind::Body } else { MeasurementKind::Garment }, source: id(92),
            }).unwrap()).collect(),
            procedures: [30, 31].into_iter().map(|n| MeasurementProcedure::new(MeasurementProcedureDefinition {
                id: id(n), name: format!("Fixture procedure {n}"), kind: if n == 30 { MeasurementKind::Body } else { MeasurementKind::Garment }, source: id(93),
                documentation: "Fixture only: use the authored landmarks and record the declared unit/source assumption.".to_owned(),
            }).unwrap()).collect(),
            measurements: Vec::new(),
        };
        let records = fixture.records();
        fixture.measurements = vec![
            Measurement::new(
                MeasurementDefinition {
                    id: id(1),
                    name: "Waist".to_owned(),
                    token: token("body_waist"),
                    entered_unit: Unit::Centimetre,
                    kind: MeasurementKind::Body,
                    landmarks: [id(20), id(21)],
                    procedure: id(30),
                    declaration: id(10),
                },
                &records,
            )
            .unwrap(),
            Measurement::new(
                MeasurementDefinition {
                    id: id(2),
                    name: "Waist".to_owned(),
                    token: token("garment_waist"),
                    entered_unit: Unit::Inch,
                    kind: MeasurementKind::Garment,
                    landmarks: [id(22), id(23)],
                    procedure: id(31),
                    declaration: id(11),
                },
                &records,
            )
            .unwrap(),
        ];
        fixture
    }
    fn records(&self) -> MeasurementContext<'_> {
        MeasurementContext::new(&self.declarations, &self.landmarks, &self.procedures).unwrap()
    }
    fn definition(&self) -> MeasurementTableDefinition {
        MeasurementTableDefinition {
            id: id(3),
            name: "Table de mesures / measurements".to_owned(),
            entries: self
                .measurements
                .iter()
                .map(MeasurementBinding::from)
                .collect(),
        }
    }
    fn table(&self) -> MeasurementTable {
        let records = self.records();
        MeasurementTable::new(
            self.definition(),
            &MeasurementTableContext::new(&self.measurements, &records).unwrap(),
        )
        .unwrap()
    }
    fn replace(&mut self, index: usize, definition: MeasurementDefinition) {
        self.measurements[index] = Measurement::new(definition, &self.records()).unwrap();
    }
}

#[test]
fn mixed_body_pom_table_retains_named_ordered_bindings_and_borrows_current_metadata() {
    let fixture = Fixture::new();
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    let definition = fixture.definition();
    let table = MeasurementTable::new(definition.clone(), &context).unwrap();
    assert_eq!(table.id(), id(3));
    assert_eq!(table.definition(), &definition);
    assert_eq!(table.validate_current(&context), Ok(()));
    for measurement in &fixture.measurements {
        let binding = MeasurementBinding::from(measurement);
        assert_eq!(binding.measurement, measurement.id());
        assert_eq!(binding.kind, measurement.definition().kind);
        assert_eq!(binding.declaration, measurement.definition().declaration);
        let selected = table.measurement(&binding.token, &context).unwrap();
        assert!(std::ptr::eq(selected, measurement));
        assert!(std::ptr::eq(
            table.measurement_by_id(measurement.id(), &context).unwrap(),
            measurement
        ));
        assert!(std::ptr::eq(
            table.declaration(&binding.token, &context).unwrap(),
            measurement.declaration(context.records()).unwrap()
        ));
    }
}

#[test]
fn an_empty_named_draft_is_legal_but_blank_names_are_not() {
    let records = MeasurementContext::new(&[], &[], &[]).unwrap();
    let context = MeasurementTableContext::new(&[], &records).unwrap();
    let empty = MeasurementTableDefinition {
        id: id(3),
        name: "Empty draft".to_owned(),
        entries: Vec::new(),
    };
    let table = MeasurementTable::new(empty.clone(), &context).unwrap();
    assert_eq!(table.validate_current(&context), Ok(()));
    assert_eq!(
        table.measurement(&token("body_waist"), &context),
        Err(MeasurementTableError::MissingToken {
            table: id(3),
            token: token("body_waist")
        })
    );
    for blank in ["", " \t\n", "\u{2003}"] {
        let mut definition = empty.clone();
        definition.name = blank.to_owned();
        assert_eq!(
            MeasurementTable::new(definition, &context),
            Err(MeasurementTableError::EmptyName(id(3)))
        );
    }
    let mut named = empty;
    named.name = "  Mesures — valid authored spacing  ".to_owned();
    assert_eq!(
        MeasurementTable::new(named.clone(), &context)
            .unwrap()
            .definition(),
        &named
    );
}

#[test]
fn duplicate_entry_id_and_token_refuse_without_coalescing_or_first_peer_selection() {
    let mut fixture = Fixture::new();
    let mut alternate = fixture.measurements[0].definition().clone();
    alternate.id = id(4);
    fixture
        .measurements
        .push(Measurement::new(alternate, &fixture.records()).unwrap());
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    let mut duplicate_id = fixture.definition();
    duplicate_id.entries = vec![duplicate_id.entries[0].clone(); 2];
    assert_eq!(
        MeasurementTable::new(duplicate_id, &context),
        Err(MeasurementTableError::DuplicateIdentity(id(1)))
    );
    assert_eq!(
        MeasurementTable::new(fixture.definition(), &context),
        Err(MeasurementTableError::DuplicateToken {
            table: id(3),
            token: token("body_waist")
        })
    );
}

#[test]
fn current_inventory_id_collisions_refuse_before_any_table_lookup() {
    let mut fixture = Fixture::new();
    fixture.measurements.push(fixture.measurements[0].clone());
    let records = fixture.records();
    assert_eq!(
        MeasurementTableContext::new(&fixture.measurements, &records).err(),
        Some(MeasurementTableError::DuplicateIdentity(id(1)))
    );
    fixture.measurements.pop();
    let mut declaration = fixture.declarations[0].definition().clone();
    declaration.id = id(1);
    fixture
        .declarations
        .push(LengthDeclaration::new(declaration).unwrap());
    let records = fixture.records();
    assert_eq!(
        MeasurementTableContext::new(&fixture.measurements, &records).err(),
        Some(MeasurementTableError::DuplicateIdentity(id(1)))
    );
}

#[test]
fn table_identity_cannot_reuse_any_supplied_measurement_or_target_identity() {
    let fixture = Fixture::new();
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    for collision in [1, 2, 10, 20, 30] {
        let mut definition = fixture.definition();
        definition.id = id(collision);
        definition.entries.clear();
        assert_eq!(
            MeasurementTable::new(definition, &context),
            Err(MeasurementTableError::DuplicateIdentity(id(collision)))
        );
    }
    let mut definition = fixture.definition();
    definition.entries[0].measurement = definition.id;
    assert_eq!(
        MeasurementTable::new(definition, &context),
        Err(MeasurementTableError::DuplicateIdentity(id(3)))
    );
}

#[test]
fn token_uniqueness_is_scoped_to_each_table_even_with_unrelated_same_token_metadata() {
    let mut fixture = Fixture::new();
    let original = fixture.table();
    let mut alternate = fixture.measurements[0].definition().clone();
    alternate.id = id(4);
    alternate.declaration = id(11);
    fixture
        .measurements
        .push(Measurement::new(alternate, &fixture.records()).unwrap());
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    let other = MeasurementTable::new(
        MeasurementTableDefinition {
            id: id(5),
            name: "Separate table".to_owned(),
            entries: vec![MeasurementBinding::from(&fixture.measurements[2])],
        },
        &context,
    )
    .unwrap();
    assert_eq!(
        original
            .measurement(&token("body_waist"), &context)
            .unwrap()
            .id(),
        id(1)
    );
    assert_eq!(
        other
            .measurement(&token("body_waist"), &context)
            .unwrap()
            .id(),
        id(4)
    );
    assert_eq!(
        original.measurement_by_id(id(4), &context),
        Err(MeasurementTableError::MissingBinding {
            table: id(3),
            measurement: id(4)
        })
    );
}

#[test]
fn removing_a_bound_measurement_never_transfers_its_token_to_a_surviving_peer() {
    let mut fixture = Fixture::new();
    let table = fixture.table();
    let mut peer = fixture.measurements[0].definition().clone();
    peer.id = id(4);
    fixture.measurements[0] = Measurement::new(peer, &fixture.records()).unwrap();
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    let expected = MeasurementTableError::MissingMeasurement {
        table: id(3),
        measurement: id(1),
    };
    assert_eq!(
        table.measurement(&token("body_waist"), &context),
        Err(expected.clone())
    );
    assert_eq!(table.validate_current(&context), Err(expected.clone()));
    assert_eq!(
        MeasurementTable::new(table.definition().clone(), &context),
        Err(expected)
    );
    assert_eq!(table.definition().entries[0].measurement, id(1));
}

#[test]
fn current_token_reassignment_is_not_an_implicit_table_rename() {
    let mut fixture = Fixture::new();
    let table = fixture.table();
    let mut current = fixture.measurements[0].definition().clone();
    current.token = token("renamed_waist");
    fixture.replace(0, current);
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    let expected = MeasurementTableError::TokenMismatch {
        table: id(3),
        measurement: id(1),
        expected: token("body_waist"),
        actual: token("renamed_waist"),
    };
    assert_eq!(
        table.measurement(&token("body_waist"), &context),
        Err(expected.clone())
    );
    assert_eq!(table.validate_current(&context), Err(expected));
    assert_eq!(
        table.measurement(&token("renamed_waist"), &context),
        Err(MeasurementTableError::MissingToken {
            table: id(3),
            token: token("renamed_waist")
        })
    );
    assert_eq!(table.definition().entries[0].token, token("body_waist"));
}

#[test]
fn current_domain_reassignment_is_refused_even_when_new_metadata_is_valid_in_its_new_domain() {
    for index in [0, 1] {
        let mut fixture = Fixture::new();
        let table = fixture.table();
        let original = fixture.measurements[index].definition().clone();
        let mut changed = fixture.measurements[1 - index].definition().clone();
        changed.id = original.id;
        changed.token = original.token.clone();
        changed.declaration = original.declaration;
        fixture.replace(index, changed.clone());
        let records = fixture.records();
        let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
        assert_eq!(
            table.measurement(&original.token, &context),
            Err(MeasurementTableError::KindMismatch {
                table: id(3),
                measurement: original.id,
                expected: original.kind,
                actual: changed.kind
            })
        );
    }
}

#[test]
fn current_scalar_reassignment_requires_explicit_binding_replacement() {
    let mut fixture = Fixture::new();
    let table = fixture.table();
    let mut changed = fixture.measurements[0].definition().clone();
    changed.declaration = id(11);
    fixture.replace(0, changed);
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    assert_eq!(
        table.declaration(&token("body_waist"), &context),
        Err(MeasurementTableError::DeclarationMismatch {
            table: id(3),
            measurement: id(1),
            expected: id(10),
            actual: id(11)
        })
    );
    let mut replacement = table.definition().clone();
    replacement.entries[0] = MeasurementBinding::from(&fixture.measurements[0]);
    let rebound = MeasurementTable::new(replacement, &context).unwrap();
    assert_eq!(
        rebound
            .declaration(&token("body_waist"), &context)
            .unwrap()
            .id(),
        id(11)
    );
    assert_eq!(table.definition().entries[0].declaration, id(10));
}

#[test]
fn same_identity_state_source_and_document_edits_are_visible_without_a_table_value_cache() {
    let mut fixture = Fixture::new();
    let table = fixture.table();
    let old = fixture.declarations[0].clone();
    let mut changed = old.definition().clone();
    changed.source = id(95);
    changed.state = LengthState::Unknown {
        observation: id(96),
    };
    fixture.declarations[0] = LengthDeclaration::new(changed).unwrap();
    let mut metadata = fixture.measurements[0].definition().clone();
    metadata.name = "Revised display only".to_owned();
    metadata.entered_unit = Unit::Inch;
    metadata.landmarks.reverse();
    fixture.replace(0, metadata.clone());
    let mut procedure = fixture.procedures[0].definition().clone();
    procedure.documentation =
        "Current fixture-only instruction revision, awaiting physical review.".to_owned();
    fixture.procedures[0] = MeasurementProcedure::new(procedure.clone()).unwrap();
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    assert_eq!(table.validate_current(&context), Ok(()));
    let selected = table.measurement(&token("body_waist"), &context).unwrap();
    assert_eq!(selected.definition(), &metadata);
    assert_eq!(
        selected.procedure(context.records()).unwrap().definition(),
        &procedure
    );
    let declaration = table.declaration(&token("body_waist"), &context).unwrap();
    assert!(std::ptr::eq(declaration, &fixture.declarations[0]));
    assert_eq!(declaration.source(), id(95));
    assert_eq!(
        declaration.authored_value(),
        Err(LengthValueError::RequiresObservation {
            declaration: id(10),
            observation: id(96)
        })
    );
    assert_eq!(old.authored_value().unwrap().as_micrometres(), 740_000);
}

#[test]
fn every_authored_state_remains_canonical_and_unknown_or_derived_has_no_numeric_fallback() {
    let value = Length::from_micrometres(-1000).unwrap();
    for state in [
        LengthState::Known {
            value,
            evidence: vec![id(91)],
        },
        LengthState::Assumed {
            value,
            assumption: id(91),
        },
        LengthState::Unknown {
            observation: id(91),
        },
        LengthState::Preference {
            value,
            provenance: id(91),
        },
        LengthState::Derived { formula: id(91) },
    ] {
        let mut fixture = Fixture::new();
        let mut definition = fixture.declarations[0].definition().clone();
        definition.state = state.clone();
        fixture.declarations[0] = LengthDeclaration::new(definition).unwrap();
        let table = fixture.table();
        let records = fixture.records();
        let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
        let declaration = table.declaration(&token("body_waist"), &context).unwrap();
        assert_eq!(declaration.state(), &state);
        match state {
            LengthState::Unknown { observation } => assert_eq!(
                declaration.authored_value(),
                Err(LengthValueError::RequiresObservation {
                    declaration: id(10),
                    observation
                })
            ),
            LengthState::Derived { formula } => assert_eq!(
                declaration.authored_value(),
                Err(LengthValueError::RequiresEvaluation {
                    declaration: id(10),
                    formula
                })
            ),
            _ => assert_eq!(declaration.authored_value(), Ok(value)),
        }
    }
}

#[test]
fn required_current_target_failures_keep_table_measurement_and_underlying_identity_evidence() {
    for missing in [10, 20, 30] {
        let mut fixture = Fixture::new();
        let table = fixture.table();
        let issue = match missing {
            10 => {
                fixture.declarations.remove(0);
                MeasurementError::MissingDeclaration(id(10))
            }
            20 => {
                fixture.landmarks.remove(0);
                MeasurementError::MissingLandmark(id(20))
            }
            _ => {
                fixture.procedures.remove(0);
                MeasurementError::MissingProcedure(id(30))
            }
        };
        let records = fixture.records();
        let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
        let error = MeasurementTableError::InvalidMeasurement {
            table: id(3),
            measurement: id(1),
            issue: issue.clone(),
        };
        assert_eq!(
            table.declaration(&token("body_waist"), &context),
            Err(error.clone())
        );
        assert_eq!(table.validate_current(&context), Err(error.clone()));
        assert_eq!(error.source().unwrap().to_string(), issue.to_string());
        let display = error.to_string();
        assert!(display.contains(&id(3).to_string()));
        assert!(display.contains(&id(1).to_string()));
        assert!(display.contains(&id(missing).to_string()));
    }
}

#[test]
fn a_targeted_valid_input_query_does_not_claim_the_other_entries_or_entire_table_are_valid() {
    let mut fixture = Fixture::new();
    let table = fixture.table();
    fixture.measurements.remove(1);
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    assert!(table.declaration(&token("body_waist"), &context).is_ok());
    assert_eq!(
        table.validate_current(&context),
        Err(MeasurementTableError::MissingMeasurement {
            table: id(3),
            measurement: id(2)
        })
    );
    assert_eq!(
        table.measurement_by_id(id(99), &context),
        Err(MeasurementTableError::MissingBinding {
            table: id(3),
            measurement: id(99)
        })
    );
}

#[test]
fn reorder_and_shared_scalar_identities_preserve_binding_identity_and_authored_table_order() {
    let mut fixture = Fixture::new();
    let mut shared = fixture.measurements[1].definition().clone();
    shared.declaration = id(10);
    fixture.replace(1, shared);
    let table = fixture.table();
    let before = table.definition().clone();
    fixture.measurements.reverse();
    fixture.declarations.reverse();
    fixture.landmarks.reverse();
    fixture.procedures.reverse();
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    assert_eq!(table.validate_current(&context), Ok(()));
    assert_eq!(table.definition(), &before);
    assert_eq!(
        table
            .measurement(&token("body_waist"), &context)
            .unwrap()
            .id(),
        id(1)
    );
    assert!(std::ptr::eq(
        table.declaration(&token("body_waist"), &context).unwrap(),
        table
            .declaration(&token("garment_waist"), &context)
            .unwrap()
    ));
}

#[test]
fn current_context_collision_of_table_identity_is_not_cached_birth_approval() {
    let mut fixture = Fixture::new();
    let table = fixture.table();
    let mut extra = fixture.declarations[0].definition().clone();
    extra.id = id(3);
    fixture
        .declarations
        .push(LengthDeclaration::new(extra).unwrap());
    let records = fixture.records();
    let context = MeasurementTableContext::new(&fixture.measurements, &records).unwrap();
    assert_eq!(
        table.validate_current(&context),
        Err(MeasurementTableError::DuplicateIdentity(id(3)))
    );
    assert_eq!(
        table.measurement(&token("body_waist"), &context),
        Err(MeasurementTableError::DuplicateIdentity(id(3)))
    );
}
