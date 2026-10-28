//! Actual owned measurement/Ease content; no fake host or canonical value copies.
#![allow(clippy::expect_used)]
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::{
        DesignResourceCount as Unit, DesignResourceLimitError as Error,
        DesignResourceLimits as Limits, DesignResourceLocation as Location,
        DesignResourceMeter as Meter, DesignResourceScope as Scope, DesignResourceSink,
    },
    name::MachineToken,
    ontology::EntityId,
    value::{CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
use sc_measure::{
    charge_measurement_input_resources, CompressionPermission, Ease, EaseBinding, EaseDefinition,
    EaseSet, EaseSetContext, EaseSetDefinition, FitIntent, Landmark, LandmarkDefinition,
    Measurement, MeasurementContext, MeasurementDefinition, MeasurementInputs,
    MeasurementInputsDefinition, MeasurementInventory, MeasurementInventoryDefinition,
    MeasurementKind as Kind, MeasurementProcedure, MeasurementProcedureDefinition,
    MeasurementTable, MeasurementTableContext, MeasurementTableDefinition,
};
use sc_units::Unit as EnteredUnit;
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn token(text: &str) -> MachineToken {
    MachineToken::new(text).expect("explicit exact machine token")
}
fn limits(entries: usize, text: usize) -> Limits {
    Limits::new(
        NonZeroUsize::new(entries).expect("positive entries"),
        NonZeroUsize::new(text).expect("positive bytes"),
        NonZeroU64::new(1).expect("positive bits"),
    )
}
fn location(field: &'static str, owner: Option<u128>, index: Option<usize>) -> Location {
    Location {
        scope: Scope::SourceHost,
        owner: owner.map(id),
        field,
        index,
    }
}
fn fixture() -> (MeasurementInputs, CanonicalInputs) {
    let canonical = CanonicalInputs::new(
        (1000_u128..1003)
            .map(|identity| {
                LengthDeclaration::new(LengthDeclarationDefinition {
                    id: id(identity),
                    source: id(900),
                    state: LengthState::Unknown {
                        observation: id(901),
                    },
                })
                .expect("actual unknown canonical length")
            })
            .collect(),
        vec![],
    )
    .expect("sole actual canonical owner");
    let landmarks = [(1, "é", Kind::Body), (2, "猫", Kind::Garment)]
        .into_iter()
        .map(|(identity, name, kind)| {
            Landmark::new(LandmarkDefinition {
                id: id(identity),
                name: name.into(),
                kind,
                source: id(902),
            })
            .expect("actual landmark metadata")
        })
        .collect::<Vec<_>>();
    let procedures = [(3, "p", " é ", Kind::Body), (4, "q", "猫", Kind::Garment)]
        .into_iter()
        .map(|(identity, name, documentation, kind)| {
            MeasurementProcedure::new(MeasurementProcedureDefinition {
                id: id(identity),
                name: name.into(),
                kind,
                documentation: documentation.into(),
                source: id(903),
            })
            .expect("actual exact procedure text")
        })
        .collect::<Vec<_>>();
    let records = MeasurementContext::new(canonical.lengths(), &landmarks, &procedures)
        .expect("actual borrowed canonical/metadata context");
    let measurements = [
        (5, "B", "b", Kind::Body, 1, 3, 1000),
        (6, "G", "gg", Kind::Garment, 2, 4, 1001),
    ]
    .into_iter()
    .map(
        |(identity, name, spelling, kind, landmark, procedure, declaration)| {
            Measurement::new(
                MeasurementDefinition {
                    id: id(identity),
                    name: name.into(),
                    token: token(spelling),
                    entered_unit: EnteredUnit::Centimetre,
                    kind,
                    landmarks: [id(landmark), id(landmark)],
                    procedure: id(procedure),
                    declaration: id(declaration),
                },
                &records,
            )
            .expect("actual measurement binds sole canonical declaration")
        },
    )
    .collect::<Vec<_>>();
    let context =
        MeasurementTableContext::new(&measurements, &records).expect("actual binding context");
    let tables = vec![MeasurementTable::new(
        MeasurementTableDefinition {
            id: id(7),
            name: "T".into(),
            entries: measurements.iter().map(Into::into).collect(),
        },
        &context,
    )
    .expect("actual mixed table with held tokens")];
    let body = measurements.first().expect("actual body input");
    let garment = measurements.last().expect("actual garment input");
    let eases = vec![Ease::new(
        EaseDefinition {
            id: id(8),
            body: body.into(),
            garment: garment.into(),
            declaration: id(1002),
            fit: FitIntent::Semi,
            provenance: id(904),
            compression: CompressionPermission::Forbidden,
        },
        &context,
    )
    .expect("actual unresolved canonical Ease amount")];
    let sets =
        EaseSetContext::new(&tables, &eases, &context).expect("actual mixed-table mapping context");
    let ease_sets = vec![EaseSet::new(
        EaseSetDefinition {
            id: id(9),
            body_table: id(7),
            garment_table: id(7),
            entries: vec![EaseBinding::new(
                token("e"),
                eases.first().expect("actual Ease"),
            )],
        },
        &sets,
    )
    .expect("actual set owns token and both saved bindings")];
    let measurements = MeasurementInventory::new(
        MeasurementInventoryDefinition {
            landmarks,
            procedures,
            measurements,
            tables,
        },
        &canonical,
    )
    .expect("actual complete metadata owner");
    let inputs = MeasurementInputs::new(
        MeasurementInputsDefinition {
            measurements,
            eases,
            ease_sets,
        },
        &canonical,
    )
    .expect("actual measurement/Ease owner");
    (inputs, canonical)
}
#[test]
fn actual_measurement_ease_collections_count_every_owned_text_binding_without_deduplication() {
    let (inputs, canonical) = fixture();
    let original = (inputs.clone(), canonical.clone());
    let mut meter = Meter::new(limits(12, 30));
    charge_measurement_input_resources(&inputs, &mut meter)
        .expect("12 owned entries/30 exact UTF8 bytes, repeated token owners counted");
    let demand = meter.finish().expect("actual source component demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (12, 30, 0)
    );
    assert_eq!((inputs, canonical), original);
}
#[test]
fn actual_source_vectors_precharge_in_schema_order_and_preserve_nested_table_set_owners() {
    let (inputs, _) = fixture();
    for (maximum, consumed, requested, field, owner, index) in [
        (1, 0, 2, "measurements.landmarks", None, None),
        (3, 2, 2, "measurements.procedures", None, None),
        (5, 4, 2, "measurements.records", None, None),
        (6, 6, 1, "measurements.tables", None, None),
        (8, 7, 2, "measurements.tables.entries", Some(7), Some(0)),
        (9, 9, 1, "measurements.eases", None, None),
        (10, 10, 1, "measurements.ease_sets", None, None),
        (
            11,
            11,
            1,
            "measurements.ease_sets.entries",
            Some(9),
            Some(0),
        ),
    ] {
        let original = inputs.clone();
        let mut meter = Meter::new(limits(maximum, 30));
        let expected = Error::Count {
            unit: Unit::CollectionEntries,
            maximum,
            consumed,
            requested,
            location: location(field, owner, index),
        };
        assert_eq!(
            charge_measurement_input_resources(&inputs, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(inputs, original);
    }
}
#[test]
fn exact_utf8_documentation_and_all_saved_token_roles_keep_original_first_owner_and_index() {
    let (inputs, _) = fixture();
    for (maximum, consumed, requested, field, owner, index) in [
        (1, 0, 2, "measurements.landmarks.name", 1, 0),
        (4, 2, 3, "measurements.landmarks.name", 2, 1),
        (9, 6, 4, "measurements.procedures.documentation", 3, 0),
        (13, 11, 3, "measurements.procedures.documentation", 4, 1),
        (18, 17, 2, "measurements.records.token", 6, 1),
        (19, 19, 1, "measurements.tables.name", 7, 0),
        (22, 21, 2, "measurements.tables.entries.token", 7, 1),
        (25, 24, 2, "measurements.eases.garment.token", 8, 0),
        (
            29,
            28,
            2,
            "measurements.ease_sets.entries.garment.token",
            9,
            0,
        ),
    ] {
        let original = inputs.clone();
        let mut meter = Meter::new(limits(12, maximum));
        let expected = Error::Count {
            unit: Unit::TextBytes,
            maximum,
            consumed,
            requested,
            location: location(field, Some(owner), Some(index)),
        };
        assert_eq!(
            charge_measurement_input_resources(&inputs, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(inputs, original);
    }
}
#[test]
fn held_source_content_accounting_needs_no_rebinding_to_a_missing_canonical_context() {
    let (inputs, canonical) = fixture();
    let missing = CanonicalInputs::new(vec![], vec![])
        .expect("actual replacement draft without old canonical targets");
    assert!(inputs.validate_current(&missing).is_err());
    let original = (inputs.clone(), canonical.clone(), missing.clone());
    let mut meter = Meter::new(limits(12, 30));
    charge_measurement_input_resources(&inputs, &mut meter)
        .expect("held metadata contains no canonical value/evidence copy and needs no lookup");
    assert_eq!(
        meter
            .finish()
            .expect("held source demand")
            .collection_entries(),
        12
    );
    assert_eq!((inputs, canonical, missing), original);
}
#[test]
fn actual_source_accounting_composes_prior_counts_and_cannot_clear_a_first_refusal() {
    let (inputs, _) = fixture();
    let mut meter = Meter::new(limits(14, 32));
    meter
        .charge_entries(2, location("earlier.entries", None, None))
        .expect("prior entries");
    meter
        .charge_text("é", location("earlier.text", None, None))
        .expect("prior bytes");
    charge_measurement_input_resources(&inputs, &mut meter).expect("same cumulative sink");
    let demand = meter.finish().expect("composed demand");
    assert_eq!((demand.collection_entries(), demand.text_bytes()), (14, 32));
    let mut refused = Meter::new(limits(1, 1));
    let first = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 1,
        consumed: 0,
        requested: 2,
        location: location("original", None, None),
    };
    assert_eq!(
        refused.charge_entries(2, location("original", None, None)),
        Err(first)
    );
    assert_eq!(
        charge_measurement_input_resources(&inputs, &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
}
#[test]
fn empty_actual_measurement_sources_contribute_zero_and_preserve_prior_error_on_zero_charges() {
    let canonical = CanonicalInputs::new(vec![], vec![]).expect("empty canonical draft");
    let measurements = MeasurementInventory::new(
        MeasurementInventoryDefinition {
            landmarks: vec![],
            procedures: vec![],
            measurements: vec![],
            tables: vec![],
        },
        &canonical,
    )
    .expect("empty metadata inventory");
    let empty = MeasurementInputs::new(
        MeasurementInputsDefinition {
            measurements,
            eases: vec![],
            ease_sets: vec![],
        },
        &canonical,
    )
    .expect("empty actual source draft");
    let mut meter = Meter::new(limits(1, 1));
    charge_measurement_input_resources(&empty, &mut meter).expect("all actual collections empty");
    let demand = meter.finish().expect("zero demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 0, 0)
    );
    let mut refused = Meter::new(limits(1, 1));
    let first = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 1,
        consumed: 0,
        requested: 2,
        location: location("original", None, None),
    };
    assert_eq!(
        refused.charge_entries(2, location("original", None, None)),
        Err(first)
    );
    assert_eq!(
        charge_measurement_input_resources(&empty, &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
}

#[path = "support/resource_size_metadata.rs"]
mod size_metadata;
