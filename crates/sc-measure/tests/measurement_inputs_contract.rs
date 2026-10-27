//! Whole owned measurement/Ease sources retain current amount, membership and repair obligations.
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
    CompressionPermission, Ease, EaseBinding, EaseDefinition, EaseError, EaseSet, EaseSetContext,
    EaseSetDefinition, EaseSetError, EaseSide, FitIntent, Landmark, LandmarkDefinition,
    Measurement, MeasurementContext, MeasurementDefinition, MeasurementInputs as Inputs,
    MeasurementInputsDefinition as Definition, MeasurementInputsError as Error,
    MeasurementInputsPosition as Position, MeasurementInventory, MeasurementInventoryDefinition,
    MeasurementInventoryPosition as MetadataPosition, MeasurementKind as Kind,
    MeasurementProcedure, MeasurementProcedureDefinition, MeasurementTable,
    MeasurementTableContext, MeasurementTableDefinition, MeasurementTableError,
};
use sc_units::{Length, Unit};

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn token(name: &str) -> MachineToken {
    MachineToken::new(name).expect("fixture token")
}
fn canonical(amount: LengthState) -> CanonicalInputs {
    let lengths = [1, 2]
        .into_iter()
        .map(|identity| {
            LengthDeclaration::new(LengthDeclarationDefinition {
                id: id(identity),
                source: id(900),
                state: LengthState::Unknown {
                    observation: id(901),
                },
            })
            .expect("explicit fixture body/garment state")
        })
        .chain(std::iter::once(
            LengthDeclaration::new(LengthDeclarationDefinition {
                id: id(3),
                source: id(902),
                state: amount,
            })
            .expect("explicit fixture Ease state"),
        ))
        .collect();
    CanonicalInputs::new(lengths, vec![]).expect("unique canonical fixture inputs")
}
fn unknown() -> CanonicalInputs {
    canonical(LengthState::Unknown {
        observation: id(903),
    })
}
fn fixture(inputs: &CanonicalInputs) -> Definition {
    let landmarks = [(10, Kind::Body), (11, Kind::Garment)]
        .into_iter()
        .map(|(identity, kind)| {
            Landmark::new(LandmarkDefinition {
                id: id(identity),
                name: "fixture level".into(),
                kind,
                source: id(904),
            })
            .expect("caller-authored landmark")
        })
        .collect::<Vec<_>>();
    let procedures = [(20, Kind::Body), (21, Kind::Garment)]
        .into_iter()
        .map(|(identity, kind)| {
            MeasurementProcedure::new(MeasurementProcedureDefinition {
                id: id(identity),
                name: "fixture procedure".into(),
                kind,
                documentation:
                    "Caller-authored fixture instructions; physical repeatability is unproved."
                        .into(),
                source: id(905),
            })
            .expect("documented caller-authored procedure")
        })
        .collect::<Vec<_>>();
    let records = MeasurementContext::new(inputs.lengths(), &landmarks, &procedures)
        .expect("fixture targets");
    let measurements = [
        (30, "body_girth", Kind::Body, 10, 20, 1),
        (31, "garment_girth", Kind::Garment, 11, 21, 2),
    ]
    .into_iter()
    .map(|(identity, name, kind, landmark, procedure, declaration)| {
        Measurement::new(
            MeasurementDefinition {
                id: id(identity),
                name: "fixture measurement".into(),
                token: token(name),
                entered_unit: Unit::Centimetre,
                kind,
                landmarks: [id(landmark), id(landmark)],
                procedure: id(procedure),
                declaration: id(declaration),
            },
            &records,
        )
        .expect("born fixture measurement")
    })
    .collect::<Vec<_>>();
    let context =
        MeasurementTableContext::new(&measurements, &records).expect("fixture measurement context");
    let tables = vec![MeasurementTable::new(
        MeasurementTableDefinition {
            id: id(40),
            name: "mixed fixture table".into(),
            entries: measurements.iter().map(Into::into).collect(),
        },
        &context,
    )
    .expect("explicit mixed table")];
    let body = measurements.first().expect("body fixture");
    let garment = measurements.last().expect("garment fixture");
    let eases = [50, 51]
        .into_iter()
        .map(|identity| {
            Ease::new(
                EaseDefinition {
                    id: id(identity),
                    body: body.into(),
                    garment: garment.into(),
                    declaration: id(3),
                    fit: FitIntent::Semi,
                    provenance: id(906),
                    compression: CompressionPermission::Forbidden,
                },
                &context,
            )
            .expect("explicit unresolved Ease mapping")
        })
        .collect::<Vec<_>>();
    let ease_context =
        EaseSetContext::new(&tables, &eases, &context).expect("fixture Ease context");
    let ease_sets = [60, 61]
        .into_iter()
        .map(|identity| {
            EaseSet::new(
                EaseSetDefinition {
                    id: id(identity),
                    body_table: id(40),
                    garment_table: id(40),
                    entries: vec![EaseBinding::new(
                        token("ease_girth"),
                        eases.first().expect("first mapping"),
                    )],
                },
                &ease_context,
            )
            .expect("same token in separate sets is legal")
        })
        .collect();
    let measurements = MeasurementInventory::new(
        MeasurementInventoryDefinition {
            landmarks,
            procedures,
            measurements,
            tables,
        },
        inputs,
    )
    .expect("owned actual metadata");
    Definition {
        measurements,
        eases,
        ease_sets,
    }
}

#[test]
fn exact_sources_order_intent_and_set_scoped_tokens_survive_with_canonical_borrows() {
    let canonical = unknown();
    let supplied = fixture(&canonical);
    let original = supplied.clone();
    let inputs = Inputs::new(supplied, &canonical).unwrap();
    assert_eq!(inputs.definition(), &original);
    assert!(core::ptr::eq(
        inputs.measurements(),
        &inputs.definition().measurements
    ));
    assert!(core::ptr::eq(
        inputs.ease(id(50)).unwrap(),
        inputs.definition().eases.first().unwrap()
    ));
    assert!(core::ptr::eq(
        inputs.ease_set(id(61)).unwrap(),
        inputs.definition().ease_sets.last().unwrap()
    ));
    assert!(inputs.ease(id(99)).is_none());
    assert!(inputs.ease_set(id(99)).is_none());
    let records = inputs.measurements().record_context(&canonical).unwrap();
    let measurements = inputs.measurements().definition();
    let tables = MeasurementTableContext::new(&measurements.measurements, &records).unwrap();
    let context =
        EaseSetContext::new(&measurements.tables, &inputs.definition().eases, &tables).unwrap();
    for set in &inputs.definition().ease_sets {
        let ease = set.ease_by_token(&token("ease_girth"), &context).unwrap();
        assert!(core::ptr::eq(ease, inputs.ease(id(50)).unwrap()));
        assert!(core::ptr::eq(
            ease.declaration(&tables).unwrap(),
            canonical.length(id(3)).unwrap()
        ));
        assert!(ease.authored_value(&tables).is_err());
    }
}

#[test]
fn duplicate_mapping_and_set_ids_report_authored_positions_before_other_failures() {
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    definition
        .eases
        .push(definition.eases.first().unwrap().clone());
    assert_eq!(
        Inputs::new(definition, &canonical),
        Err(Error::DuplicateIdentity {
            identity: id(50),
            first: Position::Ease { index: 0 },
            duplicate: Position::Ease { index: 2 }
        })
    );
    let mut definition = fixture(&canonical);
    definition
        .ease_sets
        .push(definition.ease_sets.first().unwrap().clone());
    assert_eq!(
        Inputs::new(definition, &canonical),
        Err(Error::DuplicateIdentity {
            identity: id(60),
            first: Position::EaseSet { index: 0 },
            duplicate: Position::EaseSet { index: 2 }
        })
    );
}

#[test]
fn mapping_ids_are_disjoint_from_actual_tables_and_current_general_scalar_records() {
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    let mut alias = definition.eases.first().unwrap().definition().clone();
    alias.id = id(40);
    let records = definition.measurements.record_context(&canonical).unwrap();
    let tables =
        MeasurementTableContext::new(&definition.measurements.definition().measurements, &records)
            .unwrap();
    let replacement = Ease::new(alias, &tables).unwrap();
    definition.eases.insert(0, replacement);
    assert_eq!(
        Inputs::new(definition, &canonical),
        Err(Error::DuplicateIdentity {
            identity: id(40),
            first: Position::Metadata(MetadataPosition::Table { index: 0 }),
            duplicate: Position::Ease { index: 0 }
        })
    );
    let current = CanonicalInputs::new(
        canonical.lengths().to_vec(),
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(50),
                source: id(907),
                kind: ScalarInputKind::Boolean,
                state: ScalarInputState::Unknown {
                    observation: id(908),
                },
            })
            .unwrap(),
        ],
    )
    .unwrap();
    assert_eq!(
        Inputs::new(fixture(&canonical), &current),
        Err(Error::DuplicateIdentity {
            identity: id(50),
            first: Position::Metadata(MetadataPosition::Scalar { index: 0 }),
            duplicate: Position::Ease { index: 0 }
        })
    );
}

#[test]
fn current_negative_amount_refuses_without_permission_even_for_unused_mappings() {
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    definition.ease_sets.clear();
    let amount = Length::from_micrometres(-1).unwrap();
    let current = self::canonical(LengthState::Assumed {
        value: amount,
        assumption: id(909),
    });
    let error = Inputs::new(definition, &current).unwrap_err();
    assert_eq!(
        error,
        Error::Ease {
            ease: id(50),
            issue: Box::new(EaseError::UndeclaredCompression {
                ease: id(50),
                declaration: id(3),
                amount
            })
        }
    );
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn explicit_permission_and_fit_replacement_are_current_without_rebinding_set_targets() {
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    let original_sets = definition.ease_sets.clone();
    let current = self::canonical(LengthState::Preference {
        value: Length::from_micrometres(-2).unwrap(),
        provenance: id(910),
    });
    let records = definition.measurements.record_context(&current).unwrap();
    let tables =
        MeasurementTableContext::new(&definition.measurements.definition().measurements, &records)
            .unwrap();
    definition.eases = definition
        .eases
        .iter()
        .map(|ease| {
            let mut replacement = ease.definition().clone();
            replacement.fit = FitIntent::Loose;
            replacement.compression = CompressionPermission::Declared {
                provenance: id(911),
            };
            Ease::new(replacement, &tables).unwrap()
        })
        .collect();
    let inputs = Inputs::new(definition, &current).unwrap();
    assert_eq!(inputs.definition().ease_sets, original_sets);
    assert_eq!(
        inputs.ease(id(50)).unwrap().definition().fit,
        FitIntent::Loose
    );
    assert_eq!(
        inputs.ease(id(50)).unwrap().definition().compression,
        CompressionPermission::Declared {
            provenance: id(911)
        }
    );
}

#[test]
fn deleted_mappings_or_tables_do_not_transfer_to_surviving_peers() {
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    definition.eases.remove(0);
    assert_eq!(
        Inputs::new(definition, &canonical),
        Err(Error::EaseSet {
            set: id(60),
            issue: Box::new(EaseSetError::MissingEase {
                set: id(60),
                ease: id(50)
            })
        })
    );
    let mut definition = fixture(&canonical);
    let mut metadata = definition.measurements.definition().clone();
    metadata.tables.clear();
    definition.measurements = MeasurementInventory::new(metadata, &canonical).unwrap();
    assert_eq!(
        Inputs::new(definition, &canonical),
        Err(Error::EaseSet {
            set: id(60),
            issue: Box::new(EaseSetError::MissingTable {
                set: id(60),
                side: EaseSide::Body,
                table: id(40)
            })
        })
    );
}

#[test]
fn current_table_membership_is_required_even_when_mapping_metadata_still_exists() {
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    let records = definition.measurements.record_context(&canonical).unwrap();
    let tables =
        MeasurementTableContext::new(&definition.measurements.definition().measurements, &records)
            .unwrap();
    let replacement = MeasurementTable::new(
        MeasurementTableDefinition {
            id: id(40),
            name: "now empty".into(),
            entries: vec![],
        },
        &tables,
    )
    .unwrap();
    let mut metadata = definition.measurements.definition().clone();
    metadata.tables = vec![replacement];
    definition.measurements = MeasurementInventory::new(metadata, &canonical).unwrap();
    let error = Inputs::new(definition, &canonical).unwrap_err();
    assert_eq!(
        error,
        Error::EaseSet {
            set: id(60),
            issue: Box::new(EaseSetError::InvalidTableMember {
                set: id(60),
                ease: id(50),
                side: EaseSide::Body,
                issue: Box::new(MeasurementTableError::MissingBinding {
                    table: id(40),
                    measurement: id(30)
                })
            })
        }
    );
}

#[test]
fn current_state_edits_remain_borrowed_and_detached_edits_do_not_mutate_sources() {
    let canonical = unknown();
    let inputs = Inputs::new(fixture(&canonical), &canonical).unwrap();
    for state in [
        LengthState::Assumed {
            value: Length::from_micrometres(4_000).unwrap(),
            assumption: id(912),
        },
        LengthState::Derived { formula: id(913) },
    ] {
        let current = self::canonical(state.clone());
        inputs.validate_current(&current).unwrap();
        let records = inputs.measurements().record_context(&current).unwrap();
        let tables = MeasurementTableContext::new(
            &inputs.measurements().definition().measurements,
            &records,
        )
        .unwrap();
        let declaration = inputs.ease(id(50)).unwrap().declaration(&tables).unwrap();
        assert!(core::ptr::eq(declaration, current.length(id(3)).unwrap()));
        assert_eq!(declaration.state(), &state);
    }
    let mut detached = inputs.definition().clone();
    detached.ease_sets.clear();
    assert_eq!(inputs.definition().ease_sets.len(), 2);
    assert_ne!(&detached, inputs.definition());
}

#[test]
fn empty_inventories_mint_nothing_but_authored_empty_sets_still_need_their_tables() {
    let canonical = CanonicalInputs::new(vec![], vec![]).unwrap();
    let measurements = MeasurementInventory::new(
        MeasurementInventoryDefinition {
            landmarks: vec![],
            procedures: vec![],
            measurements: vec![],
            tables: vec![],
        },
        &canonical,
    )
    .unwrap();
    let definition = Definition {
        measurements,
        eases: vec![],
        ease_sets: vec![],
    };
    assert_eq!(
        Inputs::new(definition.clone(), &canonical)
            .unwrap()
            .definition(),
        &definition
    );
    let canonical = unknown();
    let mut definition = fixture(&canonical);
    let records = definition.measurements.record_context(&canonical).unwrap();
    let metadata = definition.measurements.definition();
    let tables = MeasurementTableContext::new(&metadata.measurements, &records).unwrap();
    let context = EaseSetContext::new(&metadata.tables, &definition.eases, &tables).unwrap();
    let set = EaseSet::new(
        EaseSetDefinition {
            id: id(62),
            body_table: id(40),
            garment_table: id(40),
            entries: vec![],
        },
        &context,
    )
    .unwrap();
    let mut metadata = definition.measurements.definition().clone();
    metadata.tables.clear();
    definition.measurements = MeasurementInventory::new(metadata, &canonical).unwrap();
    definition.ease_sets = vec![set];
    assert_eq!(
        Inputs::new(definition, &canonical),
        Err(Error::EaseSet {
            set: id(62),
            issue: Box::new(EaseSetError::MissingTable {
                set: id(62),
                side: EaseSide::Body,
                table: id(40)
            })
        })
    );
}
