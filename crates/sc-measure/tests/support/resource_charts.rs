//! Actual source-family chart ownership, including unused records and MTM saved token copies.
use super::{id, limits, location, Error, Meter, Unit};
use sc_core::{design::DesignResourceSink, value::CanonicalInputs};
use sc_measure::{
    charge_garment_size_set_chart_resources, charge_mtm_size_set_chart_resources,
    charge_size_set_metadata_resources, GarmentSizeSet, GarmentSizeSetDefinition,
    MeasurementBinding, MeasurementInputs, MeasurementTableContext, MtmSizeSet, SizeChart,
    SizeChartCollectionContext, SizeChartContext, SizeChartDefinition, SizeChartObservation,
    SizeChartObservationDefinition,
};
#[path = "owned_size_set_fixture.rs"]
mod owned;
fn garment(draft: bool) -> (GarmentSizeSet, CanonicalInputs, MeasurementInputs) {
    let (metadata, canonical, inputs) = super::size_metadata::fixture(false);
    let inventory = inputs.measurements().definition();
    let records = inputs
        .measurements()
        .record_context(&canonical)
        .expect("actual current measurement records");
    let measurements = MeasurementTableContext::new(&inventory.measurements, &records)
        .expect("actual binding context");
    let context = SizeChartContext::new(
        &metadata.definition().membership,
        &inventory.tables,
        &measurements,
    )
    .expect("actual member/table context");
    let pom = MeasurementBinding::from(inventory.measurements.last().expect("actual garment POM"));
    let observations = if draft {
        vec![]
    } else {
        metadata
            .definition()
            .membership
            .definition()
            .members
            .iter()
            .zip(3000_u128..)
            .map(|(member, identity)| {
                SizeChartObservation::new(
                    SizeChartObservationDefinition {
                        id: id(identity),
                        membership: metadata.reference(),
                        member: member.id,
                        design_table: id(7),
                        chart_table: id(7),
                        pom: pom.clone(),
                        measurement: pom.clone(),
                        provenance: id(907),
                    },
                    &context,
                )
                .expect("actual original correspondence")
            })
            .collect()
    };
    let catalog = SizeChartCollectionContext::new(&observations, &context)
        .expect("actual complete observation registry");
    let chart = SizeChart::new(
        SizeChartDefinition {
            id: id(3100),
            membership: metadata.reference(),
            design_table: id(7),
            poms: if draft { vec![] } else { vec![pom] },
            observations: observations.first().map(Into::into).into_iter().collect(),
        },
        &catalog,
    )
    .expect("actual partial chart; second observation deliberately unselected");
    let set = GarmentSizeSet::new(
        GarmentSizeSetDefinition {
            metadata,
            observations,
            chart,
        },
        &canonical,
        &inputs,
    )
    .expect("actual owned garment source family");
    (set, canonical, inputs)
}
fn mtm(partial: bool) -> (MtmSizeSet, CanonicalInputs, MeasurementInputs) {
    let (canonical, inputs, definition) = owned::mtm_fixture(partial, 3200);
    let set =
        MtmSizeSet::new(definition, &canonical, &inputs).expect("actual owned MTM source family");
    (set, canonical, inputs)
}
#[test]
fn garment_chart_accounts_for_unused_observations_saved_bindings_and_separate_common_owner() {
    let (set, canonical, inputs) = garment(false);
    let original = (set.clone(), canonical.clone(), inputs.clone());
    let mut meter = Meter::new(limits(4, 14));
    charge_garment_size_set_chart_resources(&set, &mut meter)
        .expect("2 actual observations,1 target,1 saved binding;7 distinct owned token fields");
    let demand = meter.finish().expect("chart component demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (4, 14, 0)
    );
    let mut complete = Meter::new(limits(25, 34));
    charge_size_set_metadata_resources(&set.definition().metadata, &mut complete)
        .expect("actual common owner21/20");
    charge_garment_size_set_chart_resources(&set, &mut complete)
        .expect("retained chart content4/14");
    let demand = complete.finish().expect("two actual family components");
    assert_eq!((demand.collection_entries(), demand.text_bytes()), (25, 34));
    assert_eq!((set, canonical, inputs), original);
}
#[test]
fn garment_chart_limits_preserve_actual_set_observation_chart_and_original_registry_entry_indices()
{
    let (set, _, _) = garment(false);
    for (unit, maximum, consumed, requested, field, owner, index) in [
        (
            Unit::CollectionEntries,
            1,
            0,
            2,
            "garment_sets.observations",
            2000,
            None,
        ),
        (
            Unit::CollectionEntries,
            2,
            2,
            1,
            "garment_sets.chart.poms",
            3100,
            None,
        ),
        (
            Unit::CollectionEntries,
            3,
            3,
            1,
            "garment_sets.chart.observations",
            3100,
            None,
        ),
        (
            Unit::TextBytes,
            1,
            0,
            2,
            "garment_sets.observations.pom.token",
            3000,
            Some(0),
        ),
        (
            Unit::TextBytes,
            3,
            2,
            2,
            "garment_sets.observations.measurement.token",
            3000,
            Some(0),
        ),
        (
            Unit::TextBytes,
            5,
            4,
            2,
            "garment_sets.observations.pom.token",
            3001,
            Some(1),
        ),
        (
            Unit::TextBytes,
            7,
            6,
            2,
            "garment_sets.observations.measurement.token",
            3001,
            Some(1),
        ),
        (
            Unit::TextBytes,
            9,
            8,
            2,
            "garment_sets.chart.poms.token",
            3100,
            Some(0),
        ),
        (
            Unit::TextBytes,
            11,
            10,
            2,
            "garment_sets.chart.observations.pom.token",
            3100,
            Some(0),
        ),
        (
            Unit::TextBytes,
            13,
            12,
            2,
            "garment_sets.chart.observations.measurement.token",
            3100,
            Some(0),
        ),
    ] {
        let original = set.clone();
        let mut meter = Meter::new(if unit == Unit::CollectionEntries {
            limits(maximum, 14)
        } else {
            limits(4, maximum)
        });
        let expected = Error::Count {
            unit,
            maximum,
            consumed,
            requested,
            location: location(field, Some(owner), index),
        };
        assert_eq!(
            charge_garment_size_set_chart_resources(&set, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(set, original);
    }
}
#[test]
fn actual_mtm_chart_saved_ease_snapshot_owns_its_entries_and_three_tokens_per_entry() {
    for (partial, entries, text) in [(false, 2, 66), (true, 1, 33)] {
        let (set, canonical, inputs) = mtm(partial);
        let spellings: Vec<_> = set
            .definition()
            .chart
            .definition()
            .ease_set
            .entries
            .iter()
            .map(|entry| {
                (
                    entry.token.as_str(),
                    entry.body.token.as_str(),
                    entry.garment.token.as_str(),
                )
            })
            .collect();
        let expected = [
            ("waist_ease", "body_waist", "garment_waist"),
            ("waistband_ease", "body_waist", "waistband"),
        ];
        assert_eq!(
            spellings.as_slice(),
            if partial {
                &expected[..1]
            } else {
                &expected[..]
            }
        );
        let original = (set.clone(), canonical.clone(), inputs.clone());
        let mut meter = Meter::new(limits(entries, text));
        charge_mtm_size_set_chart_resources(&set, &mut meter)
            .expect("held Ease snapshot, not authoritative lookup or numeric cache");
        let demand = meter.finish().expect("actual retained MTM chart component");
        assert_eq!(
            (
                demand.collection_entries(),
                demand.text_bytes(),
                demand.counter_bits()
            ),
            (entries, text, 0)
        );
        assert_eq!((set, canonical, inputs), original);
    }
}
#[test]
fn mtm_snapshot_limits_keep_chart_owner_and_original_entry_for_each_saved_token_role() {
    let (set, _, _) = mtm(false);
    for (unit, maximum, consumed, requested, field, index) in [
        (
            Unit::CollectionEntries,
            1,
            0,
            2,
            "mtm_sets.chart.ease_snapshot.entries",
            None,
        ),
        (
            Unit::TextBytes,
            9,
            0,
            10,
            "mtm_sets.chart.ease_snapshot.entries.token",
            Some(0),
        ),
        (
            Unit::TextBytes,
            19,
            10,
            10,
            "mtm_sets.chart.ease_snapshot.entries.body.token",
            Some(0),
        ),
        (
            Unit::TextBytes,
            32,
            20,
            13,
            "mtm_sets.chart.ease_snapshot.entries.garment.token",
            Some(0),
        ),
        (
            Unit::TextBytes,
            46,
            33,
            14,
            "mtm_sets.chart.ease_snapshot.entries.token",
            Some(1),
        ),
        (
            Unit::TextBytes,
            56,
            47,
            10,
            "mtm_sets.chart.ease_snapshot.entries.body.token",
            Some(1),
        ),
        (
            Unit::TextBytes,
            65,
            57,
            9,
            "mtm_sets.chart.ease_snapshot.entries.garment.token",
            Some(1),
        ),
    ] {
        let original = set.clone();
        let mut meter = Meter::new(if unit == Unit::CollectionEntries {
            limits(maximum, 66)
        } else {
            limits(2, maximum)
        });
        let expected = Error::Count {
            unit,
            maximum,
            consumed,
            requested,
            location: location(field, Some(3200), index),
        };
        assert_eq!(
            charge_mtm_size_set_chart_resources(&set, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(set, original);
    }
}
#[test]
fn actual_chart_components_remain_held_when_replacement_canonical_contexts_refuse() {
    let (garment, garment_canonical, garment_inputs) = garment(false);
    let (mtm, mtm_canonical, mtm_inputs) = mtm(false);
    let missing = CanonicalInputs::new(vec![], vec![]).expect("actual empty replacement draft");
    assert!(garment.validate_current(&missing, &garment_inputs).is_err());
    assert!(mtm.validate_current(&missing, &mtm_inputs).is_err());
    let original = (
        garment.clone(),
        garment_canonical.clone(),
        garment_inputs.clone(),
        mtm.clone(),
        mtm_canonical.clone(),
        mtm_inputs.clone(),
    );
    let mut meter = Meter::new(limits(6, 80));
    charge_garment_size_set_chart_resources(&garment, &mut meter).expect("held garment fields");
    charge_mtm_size_set_chart_resources(&mtm, &mut meter).expect("held MTM snapshot");
    let demand = meter
        .finish()
        .expect("actual components in one cumulative meter");
    assert_eq!((demand.collection_entries(), demand.text_bytes()), (6, 80));
    assert_eq!(
        (
            garment,
            garment_canonical,
            garment_inputs,
            mtm,
            mtm_canonical,
            mtm_inputs
        ),
        original
    );
}
#[test]
fn both_actual_chart_producers_keep_the_original_prior_refusal() {
    let (garment, _, _) = garment(false);
    let (mtm, _, _) = mtm(false);
    let mut meter = Meter::new(limits(1, 1));
    let first = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 1,
        consumed: 0,
        requested: 2,
        location: location("original", None, None),
    };
    assert_eq!(
        meter.charge_entries(2, location("original", None, None)),
        Err(first)
    );
    assert_eq!(
        charge_garment_size_set_chart_resources(&garment, &mut meter),
        Err(first)
    );
    assert_eq!(
        charge_mtm_size_set_chart_resources(&mtm, &mut meter),
        Err(first)
    );
    assert_eq!(meter.finish(), Err(first));
}
#[test]
fn actual_empty_garment_chart_component_has_zero_demand_and_preserves_prior_error() {
    let (set, _, _) = garment(true);
    assert_eq!(
        set.definition()
            .metadata
            .definition()
            .membership
            .definition()
            .members
            .len(),
        2
    );
    let mut meter = Meter::new(limits(1, 1));
    charge_garment_size_set_chart_resources(&set, &mut meter)
        .expect("empty observations/target/binding vectors are an explicit draft");
    let demand = meter
        .finish()
        .expect("chart component only, common owner separate");
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
        charge_garment_size_set_chart_resources(&set, &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
}
