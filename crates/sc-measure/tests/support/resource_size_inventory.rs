//! Actual mixed source inventory with explicit singleton family provenance and authored refusal order.
use super::owned_size_sets as owned;
use super::{limits, location, Error, Meter, Unit};
use sc_core::{design::DesignResourceSink, value::CanonicalInputs};
use sc_measure::{
    charge_size_set_inventory_resources, MeasurementInputs, SizeSet, SizeSetInventory,
};
fn fixture(reverse: bool) -> (SizeSetInventory, CanonicalInputs, MeasurementInputs) {
    let (canonical, inputs, mtm, garment) = owned::inventory_fixture();
    let sets = if reverse {
        vec![SizeSet::Garment(garment), SizeSet::Mtm(mtm)]
    } else {
        vec![SizeSet::Mtm(mtm), SizeSet::Garment(garment)]
    };
    let inventory = SizeSetInventory::new(sets, &canonical, &inputs)
        .expect("actual complete mixed source ownership");
    (inventory, canonical, inputs)
}
#[test]
fn actual_mixed_inventory_counts_each_family_once_and_preserves_singleton_garment_origin() {
    let (inventory, canonical, inputs) = fixture(false);
    let original = (inventory.clone(), canonical.clone(), inputs.clone());
    let first = inventory.sets().first().expect("actual first set");
    let second = inventory.sets().last().expect("actual second set");
    let mtm = match first {
        SizeSet::Mtm(value) => Some(value),
        SizeSet::Garment(_) => None,
    }
    .expect("authored MTM family must stay MTM");
    let garment = match second {
        SizeSet::Garment(value) => Some(value),
        SizeSet::Mtm(_) => None,
    }
    .expect("singleton garment family must stay Garment");
    assert_eq!(
        mtm.definition()
            .metadata
            .definition()
            .membership
            .definition()
            .members
            .first()
            .expect("actual singleton")
            .label
            .as_str(),
        "Client A"
    );
    assert_eq!(
        mtm.definition()
            .metadata
            .definition()
            .axes
            .definition()
            .axes
            .first()
            .expect("actual MTM axis")
            .definition()
            .name,
        "Client axis"
    );
    assert_eq!(
        garment
            .definition()
            .metadata
            .definition()
            .membership
            .definition()
            .members
            .len(),
        1
    );
    assert_eq!(
        garment
            .definition()
            .metadata
            .definition()
            .axes
            .definition()
            .axes
            .first()
            .expect("actual garment axis")
            .definition()
            .name,
        "Explicit count axis"
    );
    let mut meter = Meter::new(limits(20, 239));
    charge_size_set_inventory_resources(&inventory, &mut meter)
        .expect("outer2+MTM7+Garment11, text102+137");
    let demand = meter.finish().expect("actual complete external set scope");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (20, 239, 0)
    );
    assert_eq!((inventory, canonical, inputs), original);
}
#[test]
fn actual_inventory_precharge_and_last_saved_token_refusals_keep_original_locations() {
    let (inventory, _, _) = fixture(false);
    for (policy, expected) in [
        (
            limits(1, 239),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 1,
                consumed: 0,
                requested: 2,
                location: location("size_sets", None, None),
            },
        ),
        (
            limits(19, 239),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 19,
                consumed: 18,
                requested: 2,
                location: location("garment_sets.chart.observations", Some(1006), None),
            },
        ),
        (
            limits(20, 238),
            Error::Count {
                unit: Unit::TextBytes,
                maximum: 238,
                consumed: 230,
                requested: 9,
                location: location(
                    "garment_sets.chart.observations.measurement.token",
                    Some(1006),
                    Some(1),
                ),
            },
        ),
    ] {
        let original = inventory.clone();
        let mut meter = Meter::new(policy);
        assert_eq!(
            charge_size_set_inventory_resources(&inventory, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(inventory, original);
    }
}
#[test]
fn reversing_actual_family_order_changes_first_refusal_without_changing_total_content() {
    let (inventory, canonical, inputs) = fixture(true);
    let original = (inventory.clone(), canonical.clone(), inputs.clone());
    let mut meter = Meter::new(limits(20, 239));
    charge_size_set_inventory_resources(&inventory, &mut meter)
        .expect("same owned content in opposite authored order");
    let demand = meter.finish().expect("same aggregate units");
    assert_eq!(
        (demand.collection_entries(), demand.text_bytes()),
        (20, 239)
    );
    let mut refused = Meter::new(limits(19, 239));
    let expected = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 19,
        consumed: 18,
        requested: 2,
        location: location("mtm_sets.chart.ease_snapshot.entries", Some(70), None),
    };
    assert_eq!(
        charge_size_set_inventory_resources(&inventory, &mut refused),
        Err(expected)
    );
    assert_eq!(refused.finish(), Err(expected));
    assert_eq!((inventory, canonical, inputs), original);
}
#[test]
fn actual_inventory_composes_prefix_and_keeps_held_content_when_current_canonical_validation_refuses(
) {
    let (inventory, canonical, inputs) = fixture(false);
    let missing =
        CanonicalInputs::new(vec![], vec![]).expect("actual missing replacement source context");
    assert!(inventory.validate_current(&missing, &inputs).is_err());
    let original = (
        inventory.clone(),
        canonical.clone(),
        inputs.clone(),
        missing.clone(),
    );
    let mut meter = Meter::new(limits(22, 241));
    meter
        .charge_entries(2, location("earlier.entries", None, None))
        .expect("prior entries");
    meter
        .charge_text("é", location("earlier.text", None, None))
        .expect("prior UTF8 bytes");
    charge_size_set_inventory_resources(&inventory, &mut meter)
        .expect("held complete source fields without current lookup");
    let demand = meter.finish().expect("one cumulative sink");
    assert_eq!(
        (demand.collection_entries(), demand.text_bytes()),
        (22, 241)
    );
    assert_eq!((inventory, canonical, inputs, missing), original);
}
#[test]
fn explicit_empty_inventory_counts_zero_and_both_empty_and_nonempty_keep_prior_refusal() {
    let (inventory, canonical, inputs) = fixture(false);
    let empty =
        SizeSetInventory::new(vec![], &canonical, &inputs).expect("actual explicit external draft");
    let mut meter = Meter::new(limits(1, 1));
    charge_size_set_inventory_resources(&empty, &mut meter)
        .expect("zero sets, shared inputs remain separately owned");
    let demand = meter.finish().expect("zero inventory component demand");
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
        charge_size_set_inventory_resources(&empty, &mut refused),
        Err(first)
    );
    assert_eq!(
        charge_size_set_inventory_resources(&inventory, &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
}
