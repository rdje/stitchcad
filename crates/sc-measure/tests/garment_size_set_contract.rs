//! Actual owned garment sources preserve current canonical quantities and original draft boundaries.
#![allow(clippy::expect_used)]
#[path = "support/size_break_fixture.rs"]
mod support;
use core::cell::Cell;
use sc_core::value::CanonicalInputs;
use sc_measure::*;
use support::{id, Fixture};

fn fixture(
    partial: bool,
    extra: Option<(u128, u128)>,
) -> (CanonicalInputs, MeasurementInputs, GarmentSizeSetDefinition) {
    let f = Fixture::new(MeasurementKind::Garment);
    let records =
        MeasurementContext::new(&f.lengths, &f.landmarks, &f.procedures).expect("actual records");
    let measurements =
        MeasurementTableContext::new(&f.measurements, &records).expect("actual metadata");
    let mut birth_tables = vec![f.table.clone()];
    if extra.is_some_and(|(_, table)| table == 501) {
        let mut definition = f.table.definition().clone();
        definition.id = id(501);
        birth_tables
            .push(MeasurementTable::new(definition, &measurements).expect("original second table"));
    }
    let context = SizeChartContext::new(&f.membership, &birth_tables, &measurements)
        .expect("actual birth context");
    let mut observations = vec![];
    for member in &f.membership.definition().members {
        for measurement in &f.measurements {
            let observation = SizeChartObservation::new(
                SizeChartObservationDefinition {
                    id: id(800 + observations.len() as u128),
                    membership: f.membership.reference(),
                    member: member.id,
                    design_table: f.table.id(),
                    chart_table: f.table.id(),
                    pom: measurement.into(),
                    measurement: measurement.into(),
                    provenance: id(9000),
                },
                &context,
            )
            .expect("authored actual correspondence");
            observations.push(observation);
        }
    }
    let count = observations.len() - usize::from(partial);
    let bindings = observations.iter().take(count).map(Into::into).collect();
    if let Some((identity, table)) = extra {
        let pom = f.measurements.first().expect("actual POM");
        observations.push(
            SizeChartObservation::new(
                SizeChartObservationDefinition {
                    id: id(identity),
                    membership: f.membership.reference(),
                    member: id(2),
                    design_table: f.table.id(),
                    chart_table: id(table),
                    pom: pom.into(),
                    measurement: pom.into(),
                    provenance: id(9001),
                },
                &context,
            )
            .expect("valid at birth, not selected by chart"),
        );
    }
    let catalog =
        SizeChartCollectionContext::new(&observations, &context).expect("actual original catalog");
    let chart = SizeChart::new(
        SizeChartDefinition {
            id: id(850),
            membership: f.membership.reference(),
            design_table: f.table.id(),
            poms: f.measurements.iter().map(Into::into).collect(),
            observations: bindings,
        },
        &catalog,
    )
    .expect("authored possibly partial chart");
    let breaks = f.with_context(|context| {
        SizeBreaks::new(
            SizeBreaksDefinition {
                reference: f.membership.reference(),
                dimensions: vec![SizeBreakDimension {
                    axis: id(10),
                    dimension_table: f.table.id(),
                    dimension: f.measurements.first().expect("dimension").into(),
                }],
                breaks: vec![
                    SizeBreak::new(f.definition(), context).expect("actual available break")
                ],
            },
            context,
        )
        .expect("explicit partial break inventory")
    });
    let canonical = CanonicalInputs::new(f.lengths.clone(), f.scalars.clone())
        .expect("actual canonical values");
    let inputs = MeasurementInputs::new(
        MeasurementInputsDefinition {
            measurements: MeasurementInventory::new(
                MeasurementInventoryDefinition {
                    landmarks: f.landmarks.clone(),
                    procedures: f.procedures.clone(),
                    measurements: f.measurements.clone(),
                    tables: vec![f.table.clone()],
                },
                &canonical,
            )
            .expect("current actual records"),
            eases: vec![],
            ease_sets: vec![],
        },
        &canonical,
    )
    .expect("actual current inputs");
    let metadata = SizeSetMetadata::new(
        SizeSetMetadataDefinition {
            membership: f.membership.clone(),
            axes: f.axes.clone(),
            breaks,
            design_table: f.table.id(),
        },
        &canonical,
        &inputs,
    )
    .expect("owned actual common metadata");
    (
        canonical,
        inputs,
        GarmentSizeSetDefinition {
            metadata,
            observations,
            chart,
        },
    )
}

#[test]
fn owned_sources_yield_actual_shared_views_and_canonical_declarations() {
    let (canonical, inputs, definition) = fixture(false, None);
    let original = definition.clone();
    let set = GarmentSizeSet::new(definition, &canonical, &inputs).expect("actual owned set");
    assert_eq!(set.definition(), &original);
    assert_eq!(
        set.identities().last(),
        Some(&(id(850), GarmentSizeSetPosition::Chart))
    );
    set.with_inputs(&canonical, &inputs, |view| {
        assert!(core::ptr::eq(
            view.membership(),
            &set.definition().metadata.definition().membership
        ));
        assert!(core::ptr::eq(view.chart(), &set.definition().chart));
        let declaration = view
            .chart()
            .declaration(id(2), id(200), view.chart_context())
            .expect("actual explicit chart binding");
        assert!(core::ptr::eq(
            declaration,
            canonical
                .length(id(150))
                .expect("original canonical record")
        ));
        let delta = view
            .breaks()
            .definition()
            .breaks
            .first()
            .expect("actual available break")
            .delta(view.break_context())
            .expect("actual signed delta");
        assert!(core::ptr::eq(
            delta,
            canonical.length(id(100)).expect("actual canonical delta")
        ));
        assert_eq!(
            SizeSetInputs::from(view).validate_design_reference(set.reference().into()),
            Ok(())
        );
    })
    .expect("current nonescaping actual view");
    assert_eq!(
        set.validate_inputs_for(SizeInstantiationPath::Regeneration, &canonical, &inputs),
        Ok(())
    );
    assert!(matches!(
        set.validate_inputs_for(SizeInstantiationPath::GradeRules, &canonical, &inputs),
        Err(GarmentSizeSetError::Inputs(_))
    ));
    assert!(!format!("{set:?}").contains("2XL"));
}

#[test]
fn partial_chart_draft_does_not_acquire_regeneration_coverage() {
    let (canonical, inputs, definition) = fixture(true, None);
    let set =
        GarmentSizeSet::new(definition, &canonical, &inputs).expect("explicit valid partial draft");
    assert_eq!(set.validate_current(&canonical, &inputs), Ok(()));
    assert!(matches!(
        set.validate_inputs_for(SizeInstantiationPath::Regeneration, &canonical, &inputs),
        Err(GarmentSizeSetError::Inputs(_))
    ));
    assert_eq!(set.definition().observations.len(), 6);
    assert_eq!(set.definition().chart.definition().observations.len(), 5);
}

#[test]
fn unused_observation_still_requires_its_actual_current_chart_table() {
    let (canonical, inputs, definition) = fixture(false, Some((900, 501)));
    assert!(!definition
        .chart
        .definition()
        .observations
        .iter()
        .any(|binding| binding.observation == id(900)));
    let error = GarmentSizeSet::new(definition, &canonical, &inputs)
        .expect_err("unused current target missing");
    assert!(
        matches!(error, GarmentSizeSetError::Observation { observation, .. } if observation == id(900))
    );
    assert!(std::error::Error::source(&error)
        .and_then(|cause| cause.downcast_ref::<SizeChartError>())
        .is_some());
}

#[test]
fn observation_identity_cannot_alias_an_owned_axis() {
    let (canonical, inputs, definition) = fixture(false, Some((10, 500)));
    assert_eq!(
        GarmentSizeSet::new(definition, &canonical, &inputs).expect_err("cross-source alias"),
        GarmentSizeSetError::DuplicateIdentity {
            identity: id(10),
            first: GarmentSizeSetPosition::Common(SizeSetMetadataPosition::Axis { index: 0 }),
            duplicate: GarmentSizeSetPosition::Observation { index: 6 }
        }
    );
}

#[test]
fn replacement_inputs_are_revalidated_before_a_view_is_exposed() {
    let (original, inputs, definition) = fixture(false, None);
    let set = GarmentSizeSet::new(definition, &original, &inputs).expect("original current set");
    let replacement = CanonicalInputs::new(
        original
            .lengths()
            .iter()
            .filter(|record| record.id() != id(150))
            .cloned()
            .collect(),
        vec![],
    )
    .expect("removed required source");
    let visited = Cell::new(false);
    let error = set
        .with_inputs(&replacement, &inputs, |_| visited.set(true))
        .expect_err("current source missing");
    assert!(!visited.get());
    assert!(matches!(error, GarmentSizeSetError::Metadata(_)));
    assert_eq!(set.validate_current(&original, &inputs), Ok(()));
}
