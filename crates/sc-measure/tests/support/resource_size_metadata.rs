//! Actual common SizeSet owner, no chart/host substitute or numeric coordinate reads.
use super::{fixture as measurement_fixture, id, limits, location, Error, Meter, Unit};
use sc_core::design::DesignResourceSink;
use sc_core::value::{
    CanonicalInputs, ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
    ScalarInputState,
};
use sc_measure::{
    charge_size_set_metadata_resources, MeasurementBinding, MeasurementContext, MeasurementInputs,
    MeasurementTableContext, SizeAxes, SizeAxesDefinition, SizeAxis, SizeAxisCoordinate,
    SizeAxisDefinition, SizeAxisKind, SizeAxisSourceContext, SizeAxisValueDefinition,
    SizeAxisValueSource, SizeBreak, SizeBreakContext, SizeBreakDefinition, SizeBreakDimension,
    SizeBreaks, SizeBreaksDefinition, SizeLabel, SizeMember, SizeMemberPoint, SizeMembership,
    SizeMembershipDefinition, SizeSetMetadata, SizeSetMetadataDefinition, SizeSetReference,
    SizeSystem,
};
use sc_units::Count;
fn label(text: &str) -> SizeLabel {
    SizeLabel::new(text.into()).expect("exact nonblank authored label")
}
fn fixture(partial: bool) -> (SizeSetMetadata, CanonicalInputs, MeasurementInputs) {
    let (inputs, original) = measurement_fixture();
    let scalars = (1100_u128..1102)
        .map(|identity| {
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(identity),
                source: id(905),
                kind: ScalarInputKind::Length,
                state: ScalarInputState::Unknown {
                    observation: id(906),
                },
            })
            .expect("actual unknown scalar coordinate")
        })
        .collect();
    let canonical = CanonicalInputs::new(original.lengths().to_vec(), scalars)
        .expect("actual singly owned current source inventory");
    let reference = SizeSetReference {
        id: id(2000),
        revision: Count::new(u32::MAX),
    };
    let membership = SizeMembership::new(SizeMembershipDefinition {
        reference,
        system: SizeSystem::Custom,
        members: vec![
            SizeMember {
                id: id(2001),
                label: label(" é "),
            },
            SizeMember {
                id: id(2002),
                label: label("猫"),
            },
        ],
        base: id(2002),
    })
    .expect("actual ordered members and explicit second-member base");
    let lengths = canonical.lengths().iter().collect::<Vec<_>>();
    let scalars = canonical.scalars().iter().collect::<Vec<_>>();
    let sources = SizeAxisSourceContext::new(&lengths, &scalars)
        .expect("actual borrowed unknown coordinate/delta sources");
    let axis = |identity, name: &str, kind, values| {
        SizeAxis::new(
            SizeAxisDefinition {
                reference,
                id: id(identity),
                name: name.into(),
                kind,
                values,
            },
            &sources,
        )
        .expect("actual named ordered axis")
    };
    let value = |identity, source| SizeAxisValueDefinition {
        id: id(identity),
        source,
    };
    let axes = vec![
        axis(
            2100,
            "a",
            SizeAxisKind::Label,
            vec![
                value(2110, SizeAxisValueSource::Label(label("S"))),
                value(2111, SizeAxisValueSource::Label(label("é"))),
            ],
        ),
        axis(
            2200,
            "é",
            SizeAxisKind::Scalar(ScalarInputKind::Length),
            vec![
                value(2210, SizeAxisValueSource::Length(id(1000))),
                value(2211, SizeAxisValueSource::Length(id(1001))),
            ],
        ),
        axis(
            2300,
            "猫",
            SizeAxisKind::Scalar(ScalarInputKind::Length),
            vec![
                value(2310, SizeAxisValueSource::Scalar(id(1100))),
                value(2311, SizeAxisValueSource::Scalar(id(1101))),
            ],
        ),
    ];
    let coordinate = |axis, value| SizeAxisCoordinate {
        axis: id(axis),
        value: id(value),
    };
    let axes = SizeAxes::new(
        SizeAxesDefinition {
            reference,
            axes,
            points: vec![
                SizeMemberPoint {
                    member: id(2001),
                    coordinates: vec![
                        coordinate(2100, 2110),
                        coordinate(2200, 2210),
                        coordinate(2300, 2310),
                    ],
                },
                SizeMemberPoint {
                    member: id(2002),
                    coordinates: vec![
                        coordinate(2300, 2310),
                        coordinate(2200, 2211),
                        coordinate(2100, 2110),
                    ],
                },
            ],
        },
        &membership,
        &sources,
    )
    .expect("actual complete points with independent coordinate order and unused catalog values");
    let records = inputs.measurements().definition();
    let records_context =
        MeasurementContext::new(canonical.lengths(), &records.landmarks, &records.procedures)
            .expect("actual canonical measurement targets");
    let measurements = MeasurementTableContext::new(&records.measurements, &records_context)
        .expect("actual metadata binding context");
    let table = records.tables.first().expect("actual mixed Design table");
    let context = SizeBreakContext::new(&membership, &axes, &sources, table, &measurements)
        .expect("actual current break scope");
    let dimension =
        MeasurementBinding::from(records.measurements.last().expect("actual garment POM"));
    let breaks = if partial {
        vec![]
    } else {
        vec![SizeBreak::new(
            SizeBreakDefinition {
                id: id(2400),
                reference,
                axis: id(2200),
                from: id(2001),
                to: id(2002),
                dimension_table: table.id(),
                dimension: dimension.clone(),
                delta: id(1002),
            },
            &context,
        )
        .expect("actual adjacent unknown signed delta")]
    };
    let breaks = SizeBreaks::new(
        SizeBreaksDefinition {
            reference,
            dimensions: vec![SizeBreakDimension {
                axis: id(2200),
                dimension_table: table.id(),
                dimension,
            }],
            breaks,
        },
        &context,
    )
    .expect("actual independent requirements and available records");
    let metadata = SizeSetMetadata::new(
        SizeSetMetadataDefinition {
            membership,
            axes,
            breaks,
            design_table: table.id(),
        },
        &canonical,
        &inputs,
    )
    .expect("actual common source owner");
    (metadata, canonical, inputs)
}
#[test]
fn actual_common_size_metadata_counts_members_axes_points_and_saved_dimension_tokens() {
    let (metadata, canonical, inputs) = fixture(false);
    let original = (metadata.clone(), canonical.clone(), inputs.clone());
    assert!(metadata.reference().next_revision().is_err());
    let mut meter = Meter::new(limits(21, 20));
    charge_size_set_metadata_resources(&metadata, &mut meter).expect(
        "members2,axes3,values6,points2,coordinates6,dimension1,break1; exact20 text bytes",
    );
    let demand = meter.finish().expect("exact common source content");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (21, 20, 0)
    );
    assert_eq!((metadata, canonical, inputs), original);
}
#[test]
fn all_common_vectors_precharge_and_keep_actual_owners_and_original_point_indices() {
    let (metadata, _, _) = fixture(false);
    for (maximum, consumed, requested, field, owner, index) in [
        (1, 0, 2, "size_sets.members", 2000, None),
        (4, 2, 3, "size_sets.axes", 2000, None),
        (6, 5, 2, "size_sets.axes.values", 2100, Some(0)),
        (8, 7, 2, "size_sets.axes.values", 2200, Some(1)),
        (10, 9, 2, "size_sets.axes.values", 2300, Some(2)),
        (12, 11, 2, "size_sets.points", 2000, None),
        (15, 13, 3, "size_sets.points.coordinates", 2000, Some(0)),
        (18, 16, 3, "size_sets.points.coordinates", 2000, Some(1)),
        (19, 19, 1, "size_sets.break_dimensions", 2000, None),
        (20, 20, 1, "size_sets.breaks", 2000, None),
    ] {
        let original = metadata.clone();
        let mut meter = Meter::new(limits(maximum, 20));
        let expected = Error::Count {
            unit: Unit::CollectionEntries,
            maximum,
            consumed,
            requested,
            location: location(field, Some(owner), index),
        };
        assert_eq!(
            charge_size_set_metadata_resources(&metadata, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(metadata, original);
    }
}
#[test]
fn exact_unicode_labels_axis_names_and_both_dimension_binding_owners_refuse_without_normalization()
{
    let (metadata, _, _) = fixture(false);
    for (maximum, consumed, requested, field, owner, index) in [
        (3, 0, 4, "size_sets.members.label", 2001, 0),
        (6, 4, 3, "size_sets.members.label", 2002, 1),
        (10, 9, 2, "size_sets.axes.values.label", 2111, 1),
        (12, 11, 2, "size_sets.axes.name", 2200, 1),
        (15, 13, 3, "size_sets.axes.name", 2300, 2),
        (17, 16, 2, "size_sets.break_dimensions.token", 2000, 0),
        (19, 18, 2, "size_sets.breaks.dimension.token", 2400, 0),
    ] {
        let original = metadata.clone();
        let mut meter = Meter::new(limits(21, maximum));
        let expected = Error::Count {
            unit: Unit::TextBytes,
            maximum,
            consumed,
            requested,
            location: location(field, Some(owner), Some(index)),
        };
        assert_eq!(
            charge_size_set_metadata_resources(&metadata, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(metadata, original);
    }
}
#[test]
fn partial_available_breaks_keep_required_dimensions_without_fabricating_deltas() {
    let (metadata, canonical, inputs) = fixture(true);
    assert!(metadata.definition().breaks.definition().breaks.is_empty());
    let original = (metadata.clone(), canonical.clone(), inputs.clone());
    let mut meter = Meter::new(limits(20, 18));
    charge_size_set_metadata_resources(&metadata, &mut meter)
        .expect("required dimension retained, no available break or duplicate binding fabricated");
    let demand = meter.finish().expect("actual partial source");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (20, 18, 0)
    );
    assert_eq!((metadata, canonical, inputs), original);
}
#[test]
fn held_common_metadata_needs_no_new_canonical_context_or_source_resolution() {
    let (metadata, canonical, inputs) = fixture(false);
    let missing = CanonicalInputs::new(vec![], vec![]).expect("actual replacement draft");
    assert!(metadata.validate_current(&missing, &inputs).is_err());
    let original = (
        metadata.clone(),
        canonical.clone(),
        inputs.clone(),
        missing.clone(),
    );
    let mut meter = Meter::new(limits(21, 20));
    charge_size_set_metadata_resources(&metadata, &mut meter)
        .expect("held accounting creates no current context");
    assert_eq!(
        meter
            .finish()
            .expect("held resource demand")
            .collection_entries(),
        21
    );
    assert_eq!((metadata, canonical, inputs, missing), original);
}
#[test]
fn actual_common_metadata_composes_with_prior_charges_and_preserves_first_refusal() {
    let (metadata, _, _) = fixture(false);
    let mut meter = Meter::new(limits(23, 22));
    meter
        .charge_entries(2, location("earlier.entries", None, None))
        .expect("prior entries");
    meter
        .charge_text("é", location("earlier.text", None, None))
        .expect("prior UTF8");
    charge_size_set_metadata_resources(&metadata, &mut meter).expect("one cumulative meter");
    let demand = meter.finish().expect("prior plus actual content");
    assert_eq!((demand.collection_entries(), demand.text_bytes()), (23, 22));
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
        charge_size_set_metadata_resources(&metadata, &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
}
