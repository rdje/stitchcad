//! Shared actual owned source fixtures; the inventory adds a distinct explicit singleton garment set.
#![allow(clippy::expect_used)]
#[path = "mtm_fixture.rs"]
mod mtm;
pub(crate) use mtm::id;
use mtm::Fixture;
use sc_core::value::{
    CanonicalInputs, ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
    ScalarInputState,
};
use sc_measure::*;
use sc_units::Count;

pub(crate) fn mtm_fixture(
    partial: bool,
    chart_id: u128,
) -> (CanonicalInputs, MeasurementInputs, MtmSizeSetDefinition) {
    let mut f = Fixture::new();
    if partial {
        let mut definition = f.sets.first().expect("actual set").definition().clone();
        definition.entries.truncate(1);
        let set = f.make_set(definition);
        f.sets = vec![set];
    }
    let chart = f.with_context(|context| {
        let mut definition = f.definition();
        definition.id = id(chart_id);
        MtmChart::new(definition, context).expect("actual authored MTM chart")
    });
    let canonical =
        CanonicalInputs::new(f.declarations.clone(), vec![]).expect("actual canonical values");
    let inputs = MeasurementInputs::new(
        MeasurementInputsDefinition {
            measurements: MeasurementInventory::new(
                MeasurementInventoryDefinition {
                    landmarks: f.landmarks.clone(),
                    procedures: f.procedures.clone(),
                    measurements: f.measurements.clone(),
                    tables: f.tables.clone(),
                },
                &canonical,
            )
            .expect("actual metadata"),
            eases: f.eases.clone(),
            ease_sets: f.sets.clone(),
        },
        &canonical,
    )
    .expect("actual body/Ease sources");
    let lengths = canonical.lengths().iter().collect::<Vec<_>>();
    let sources = SizeAxisSourceContext::new(&lengths, &[]).expect("actual canonical sources");
    let axis = SizeAxis::new(
        SizeAxisDefinition {
            reference: f.membership.reference(),
            id: id(80),
            name: "Client axis".to_owned(),
            kind: SizeAxisKind::Label,
            values: vec![SizeAxisValueDefinition {
                id: id(81),
                source: SizeAxisValueSource::Label(
                    SizeLabel::new("Client coordinate".to_owned()).expect("authored coordinate"),
                ),
            }],
        },
        &sources,
    )
    .expect("explicit authored singleton axis");
    let axes = SizeAxes::new(
        SizeAxesDefinition {
            reference: f.membership.reference(),
            axes: vec![axis],
            points: vec![SizeMemberPoint {
                member: id(41),
                coordinates: vec![SizeAxisCoordinate {
                    axis: id(80),
                    value: id(81),
                }],
            }],
        },
        &f.membership,
        &sources,
    )
    .expect("explicit member point");
    let breaks = {
        let records = inputs
            .measurements()
            .record_context(&canonical)
            .expect("actual current records");
        let measurements = MeasurementTableContext::new(
            &inputs.measurements().definition().measurements,
            &records,
        )
        .expect("actual measurement context");
        let table = inputs
            .measurements()
            .table(id(51))
            .expect("exact logical Design table");
        let context = SizeBreakContext::new(&f.membership, &axes, &sources, table, &measurements)
            .expect("actual break context");
        SizeBreaks::new(
            SizeBreaksDefinition {
                reference: f.membership.reference(),
                dimensions: vec![],
                breaks: vec![],
            },
            &context,
        )
        .expect("explicit singleton breaks")
    };
    let metadata = SizeSetMetadata::new(
        SizeSetMetadataDefinition {
            membership: f.membership.clone(),
            axes,
            breaks,
            design_table: id(51),
        },
        &canonical,
        &inputs,
    )
    .expect("owned actual common metadata");
    (canonical, inputs, MtmSizeSetDefinition { metadata, chart })
}

// The MTM target reuses only the base fixture; the inventory target exercises these compositions.
#[allow(dead_code)]
pub(crate) fn garment_singleton(
    canonical: &CanonicalInputs,
    inputs: &MeasurementInputs,
    axis_id: u128,
) -> GarmentSizeSet {
    let reference = SizeSetReference {
        id: id(1000),
        revision: Count::new(0),
    };
    let membership = SizeMembership::new(SizeMembershipDefinition {
        reference,
        system: SizeSystem::Custom,
        members: vec![SizeMember {
            id: id(1001),
            label: SizeLabel::new("Client A".to_owned()).expect("explicit label"),
        }],
        base: id(1001),
    })
    .expect("explicit singleton garment membership");
    let lengths = canonical.lengths().iter().collect::<Vec<_>>();
    let scalars = canonical.scalars().iter().collect::<Vec<_>>();
    let sources = SizeAxisSourceContext::new(&lengths, &scalars).expect("actual canonical sources");
    let axis = SizeAxis::new(
        SizeAxisDefinition {
            reference,
            id: id(axis_id),
            name: "Explicit count axis".to_owned(),
            kind: SizeAxisKind::Scalar(ScalarInputKind::Count),
            values: vec![SizeAxisValueDefinition {
                id: id(1003),
                source: SizeAxisValueSource::Scalar(id(2000)),
            }],
        },
        &sources,
    )
    .expect("actual unknown canonical Count coordinate");
    let axes = SizeAxes::new(
        SizeAxesDefinition {
            reference,
            axes: vec![axis],
            points: vec![SizeMemberPoint {
                member: id(1001),
                coordinates: vec![SizeAxisCoordinate {
                    axis: id(axis_id),
                    value: id(1003),
                }],
            }],
        },
        &membership,
        &sources,
    )
    .expect("actual member point");
    let records = inputs
        .measurements()
        .record_context(canonical)
        .expect("actual source records");
    let measurements =
        MeasurementTableContext::new(&inputs.measurements().definition().measurements, &records)
            .expect("actual metadata context");
    let table = inputs
        .measurements()
        .table(id(51))
        .expect("actual garment Design table");
    let break_context = SizeBreakContext::new(&membership, &axes, &sources, table, &measurements)
        .expect("actual break context");
    let breaks = SizeBreaks::new(
        SizeBreaksDefinition {
            reference,
            dimensions: vec![],
            breaks: vec![],
        },
        &break_context,
    )
    .expect("explicit singleton break draft");
    let chart_records = SizeChartContext::new(
        &membership,
        &inputs.measurements().definition().tables,
        &measurements,
    )
    .expect("actual chart record context");
    let observations = [2, 3]
        .into_iter()
        .enumerate()
        .map(|(index, identity)| {
            let measurement = inputs
                .measurements()
                .measurement(id(identity))
                .expect("actual garment POM");
            SizeChartObservation::new(
                SizeChartObservationDefinition {
                    id: id(1004 + index as u128),
                    membership: reference,
                    member: id(1001),
                    design_table: id(51),
                    chart_table: id(51),
                    pom: measurement.into(),
                    measurement: measurement.into(),
                    provenance: id(9000),
                },
                &chart_records,
            )
            .expect("actual explicit correspondence")
        })
        .collect::<Vec<_>>();
    let catalog = SizeChartCollectionContext::new(&observations, &chart_records)
        .expect("actual observation catalog");
    let chart = SizeChart::new(
        SizeChartDefinition {
            id: id(1006),
            membership: reference,
            design_table: id(51),
            poms: observations
                .iter()
                .map(|record| record.definition().pom.clone())
                .collect(),
            observations: observations.iter().map(Into::into).collect(),
        },
        &catalog,
    )
    .expect("actual singleton garment chart");
    let metadata = SizeSetMetadata::new(
        SizeSetMetadataDefinition {
            membership,
            axes,
            breaks,
            design_table: id(51),
        },
        canonical,
        inputs,
    )
    .expect("actual common metadata");
    GarmentSizeSet::new(
        GarmentSizeSetDefinition {
            metadata,
            observations,
            chart,
        },
        canonical,
        inputs,
    )
    .expect("actual complete singleton garment sources")
}
#[allow(dead_code)]
pub(crate) fn inventory_fixture() -> (
    CanonicalInputs,
    MeasurementInputs,
    MtmSizeSet,
    GarmentSizeSet,
) {
    let (original, inputs, definition) = mtm_fixture(false, 70);
    let coordinate = ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(2000),
        source: id(2002),
        kind: ScalarInputKind::Count,
        state: ScalarInputState::Unknown {
            observation: id(2001),
        },
    })
    .expect("explicit unknown axis declaration");
    let canonical = CanonicalInputs::new(original.lengths().to_vec(), vec![coordinate])
        .expect("actual shared canonical inputs");
    let mtm = MtmSizeSet::new(definition, &canonical, &inputs).expect("actual current MTM sources");
    let garment = garment_singleton(&canonical, &inputs, 1002);
    (canonical, inputs, mtm, garment)
}
