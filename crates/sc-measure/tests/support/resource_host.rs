//! Actual complete immutable host through the required core port, without invented host statistics.
use super::{limits, location, Error, Meter, Unit};
use sc_core::{
    design::{DesignResourceSink, DesignResourceSource},
    value::CanonicalInputs,
};
use sc_measure::{
    DesignSources, DesignSourcesDefinition, MeasurementInputs, MeasurementInputsDefinition,
    MeasurementInventory, MeasurementInventoryDefinition, SizeSet, SizeSetInventory,
};
fn fixture() -> (DesignSources, CanonicalInputs) {
    let (canonical, measurements, mtm, garment) = super::owned_size_sets::inventory_fixture();
    let size_sets = SizeSetInventory::new(
        vec![SizeSet::Mtm(mtm), SizeSet::Garment(garment)],
        &canonical,
        &measurements,
    )
    .expect("actual complete ordered set inventory");
    let host = DesignSources::new(
        DesignSourcesDefinition {
            measurements,
            size_sets,
        },
        &canonical,
    )
    .expect("actual whole semantic/source owner");
    (host, canonical)
}
fn charge_actual<H: DesignResourceSource>(
    host: &H,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), Error> {
    host.charge_resources(sink)
}
#[test]
fn actual_complete_host_composes_both_roots_through_the_required_core_port() {
    let (host, canonical) = fixture();
    let original = (host.clone(), canonical.clone());
    let records = host.definition().measurements.measurements().definition();
    assert_eq!(
        records
            .procedures
            .first()
            .expect("actual procedure")
            .definition()
            .documentation,
        "Authored fixture, not physical certification."
    );
    assert_eq!(records.landmarks.len(), 4);
    assert_eq!(records.procedures.len(), 2);
    assert_eq!(records.measurements.len(), 3);
    assert_eq!(records.tables.len(), 2);
    let mut meter = Meter::new(limits(39, 628));
    charge_actual(&host, &mut meter).expect("actual measurements19/389 + sets20/239");
    let demand = meter.finish().expect("actual complete source content");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (39, 628, 0)
    );
    assert_eq!((host, canonical), original);
}
#[test]
fn whole_host_first_refusals_preserve_original_cross_component_counts_and_locations() {
    let (host, _) = fixture();
    for (policy, expected) in [
        (
            limits(3, 628),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 3,
                consumed: 0,
                requested: 4,
                location: location("measurements.landmarks", None, None),
            },
        ),
        (
            limits(20, 628),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 20,
                consumed: 19,
                requested: 2,
                location: location("size_sets", None, None),
            },
        ),
        (
            limits(38, 628),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 38,
                consumed: 37,
                requested: 2,
                location: location("garment_sets.chart.observations", Some(1006), None),
            },
        ),
        (
            limits(39, 9),
            Error::Count {
                unit: Unit::TextBytes,
                maximum: 9,
                consumed: 0,
                requested: 10,
                location: location("measurements.landmarks.name", Some(20), Some(0)),
            },
        ),
        (
            limits(39, 627),
            Error::Count {
                unit: Unit::TextBytes,
                maximum: 627,
                consumed: 619,
                requested: 9,
                location: location(
                    "garment_sets.chart.observations.measurement.token",
                    Some(1006),
                    Some(1),
                ),
            },
        ),
    ] {
        let original = host.clone();
        let mut meter = Meter::new(policy);
        assert_eq!(charge_actual(&host, &mut meter), Err(expected));
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(host, original);
    }
}
#[test]
fn actual_host_stays_readonly_with_missing_replacement_sources_and_prior_mixed_units() {
    let (host, canonical) = fixture();
    let missing = CanonicalInputs::new(vec![], vec![])
        .expect("actual replacement draft without original targets");
    assert!(host.validate_current(&missing).is_err());
    let original = (host.clone(), canonical.clone(), missing.clone());
    let mut meter = Meter::new(limits(41, 630));
    meter
        .charge_entries(2, location("earlier.entries", None, None))
        .expect("prior entries");
    meter
        .charge_text("é", location("earlier.text", None, None))
        .expect("prior bytes");
    meter
        .charge_counter_bits(1, location("earlier.counter", None, None))
        .expect("prior represented counter");
    charge_actual(&host, &mut meter).expect("no current lookup or canonical read");
    let demand = meter.finish().expect("all cumulative units");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (41, 630, 1)
    );
    assert_eq!(
        (&host, &canonical, &missing),
        (&original.0, &original.1, &original.2)
    );
    let mut refused = Meter::new(limits(39, 628));
    let first = Error::CounterBits {
        maximum: 1,
        consumed: 0,
        requested: 2,
        location: location("original.counter", None, None),
    };
    assert_eq!(
        refused.charge_counter_bits(2, location("original.counter", None, None)),
        Err(first)
    );
    let _ignored = charge_actual(&host, &mut refused);
    assert_eq!(refused.finish(), Err(first));
}
#[test]
fn actual_empty_host_has_zero_content_and_cannot_clear_prior_sink_refusal() {
    let canonical = CanonicalInputs::new(vec![], vec![]).expect("explicit empty canonical owner");
    let measurements = MeasurementInventory::new(
        MeasurementInventoryDefinition {
            landmarks: vec![],
            procedures: vec![],
            measurements: vec![],
            tables: vec![],
        },
        &canonical,
    )
    .expect("actual empty metadata inventory");
    let measurements = MeasurementInputs::new(
        MeasurementInputsDefinition {
            measurements,
            eases: vec![],
            ease_sets: vec![],
        },
        &canonical,
    )
    .expect("actual empty source component");
    let size_sets = SizeSetInventory::new(vec![], &canonical, &measurements)
        .expect("actual empty set inventory");
    let host = DesignSources::new(
        DesignSourcesDefinition {
            measurements,
            size_sets,
        },
        &canonical,
    )
    .expect("actual explicit empty host");
    let mut meter = Meter::new(limits(1, 1));
    charge_actual(&host, &mut meter).expect("empty source owner");
    let demand = meter.finish().expect("source content only");
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
    assert_eq!(charge_actual(&host, &mut refused), Err(first));
    assert_eq!(refused.finish(), Err(first));
}
