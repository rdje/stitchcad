//! Complete external source inventory preserves explicit families, global identities and exact pins.
#![allow(clippy::expect_used)]
#[path = "support/owned_size_set_fixture.rs"]
mod support;
use sc_core::{design::DesignSizeSetReference, value::CanonicalInputs};
use sc_measure::*;
use sc_units::Count;
use support::{id, inventory_fixture};

#[test]
fn actual_singleton_families_share_canonical_inputs_without_inferred_provenance() {
    let (canonical, inputs, mtm, garment) = inventory_fixture();
    let mtm_reference = mtm.reference();
    let garment_reference = garment.reference();
    let inventory = SizeSetInventory::new(vec![mtm.into(), garment.into()], &canonical, &inputs)
        .expect("distinct actual sources");
    for (reference, is_garment) in [(mtm_reference, false), (garment_reference, true)] {
        let set = inventory
            .validate_reference(reference.into(), &canonical, &inputs)
            .expect("exact current source");
        assert!(core::ptr::eq(
            set,
            inventory
                .size_set(reference.id)
                .expect("actual owned object")
        ));
        set.with_inputs(&canonical, &inputs, |view| {
            assert_eq!(view.membership().definition().members.len(), 1);
            assert_eq!(matches!(view, SizeSetInputs::Garment(_)), is_garment);
            let declaration = match view {
                SizeSetInputs::Garment(view) => view
                    .chart()
                    .declaration(id(1001), id(2), view.chart_context())
                    .expect("actual garment observation"),
                SizeSetInputs::Mtm(view) => view
                    .chart()
                    .body_declaration(id(2), view.chart_context())
                    .expect("actual body"),
            };
            let target = if is_garment { 11 } else { 10 };
            assert!(core::ptr::eq(
                declaration,
                canonical
                    .length(id(target))
                    .expect("actual canonical target")
            ));
        })
        .expect("explicit original family view");
        assert_eq!(
            set.validate_inputs_for(SizeInstantiationPath::Regeneration, &canonical, &inputs),
            Ok(())
        );
    }
    assert!(matches!(
        inventory.sets(),
        [SizeSet::Mtm(_), SizeSet::Garment(_)]
    ));
    assert!(!format!("{inventory:?}").contains("Client"));
}

#[test]
fn missing_id_and_wrong_revision_never_select_a_label_peer_or_latest_revision() {
    let (canonical, inputs, mtm, garment) = inventory_fixture();
    let actual = DesignSizeSetReference::from(mtm.reference());
    let inventory = SizeSetInventory::new(vec![mtm.into(), garment.into()], &canonical, &inputs)
        .expect("actual sources");
    let missing = DesignSizeSetReference::new(id(9999), actual.revision());
    assert_eq!(
        inventory.validate_reference(missing, &canonical, &inputs),
        Err(SizeSetInventoryError::MissingSet { expected: missing })
    );
    let stale = DesignSizeSetReference::new(actual.id(), Count::new(actual.revision().get() + 1));
    assert_eq!(
        inventory.validate_reference(stale, &canonical, &inputs),
        Err(SizeSetInventoryError::RevisionMismatch {
            expected: stale,
            actual
        })
    );
    assert!(inventory
        .validate_reference(actual, &canonical, &inputs)
        .is_ok());
}

#[test]
fn repeated_actual_set_is_not_deduplicated_or_selected() {
    let (canonical, inputs, mtm, _) = inventory_fixture();
    let original = SizeSet::from(mtm);
    let position = SizeSetPosition::Mtm(MtmSizeSetPosition::Common(SizeSetMetadataPosition::Set));
    assert_eq!(
        SizeSetInventory::new(vec![original.clone(), original], &canonical, &inputs)
            .expect_err("same source supplied twice"),
        SizeSetInventoryError::DuplicateIdentity {
            identity: id(40),
            first: Box::new(SizeSetInventoryPosition::Set { index: 0, position }),
            duplicate: Box::new(SizeSetInventoryPosition::Set { index: 1, position })
        }
    );
}

#[test]
fn different_set_ids_cannot_reuse_the_same_owned_axis() {
    let (canonical, inputs, mtm, _) = inventory_fixture();
    let garment = support::garment_singleton(&canonical, &inputs, 80);
    assert_ne!(mtm.reference().id, garment.reference().id);
    assert_eq!(
        SizeSetInventory::new(vec![mtm.into(), garment.into()], &canonical, &inputs)
            .expect_err("cross-set axis alias"),
        SizeSetInventoryError::DuplicateIdentity {
            identity: id(80),
            first: Box::new(SizeSetInventoryPosition::Set {
                index: 0,
                position: SizeSetPosition::Mtm(MtmSizeSetPosition::Common(
                    SizeSetMetadataPosition::Axis { index: 0 }
                ))
            }),
            duplicate: Box::new(SizeSetInventoryPosition::Set {
                index: 1,
                position: SizeSetPosition::Garment(GarmentSizeSetPosition::Common(
                    SizeSetMetadataPosition::Axis { index: 0 }
                ))
            })
        }
    );
}

#[test]
fn exact_reference_query_rechecks_even_unselected_set_sources() {
    let (canonical, inputs, mtm, garment) = inventory_fixture();
    let reference = mtm.reference();
    let inventory = SizeSetInventory::new(vec![mtm.into(), garment.into()], &canonical, &inputs)
        .expect("actual complete sources");
    let replacement = CanonicalInputs::new(canonical.lengths().to_vec(), vec![])
        .expect("removed garment-only coordinate");
    assert!(inventory
        .size_set(reference.id)
        .expect("actual MTM source")
        .validate_current(&replacement, &inputs)
        .is_ok());
    assert!(
        matches!(inventory.validate_reference(reference.into(),&replacement,&inputs),
        Err(SizeSetInventoryError::InvalidSet {set,..}) if set.id==id(1000))
    );
    assert!(inventory
        .validate_reference(reference.into(), &canonical, &inputs)
        .is_ok());
}

#[test]
fn empty_explicit_inventory_mints_no_size_set() {
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
    .expect("empty metadata");
    let inputs = MeasurementInputs::new(
        MeasurementInputsDefinition {
            measurements,
            eases: vec![],
            ease_sets: vec![],
        },
        &canonical,
    )
    .expect("empty source draft");
    let inventory =
        SizeSetInventory::new(vec![], &canonical, &inputs).expect("explicit empty inventory");
    let expected = DesignSizeSetReference::new(id(1), Count::new(0));
    assert!(inventory.sets().is_empty());
    assert_eq!(
        inventory.validate_reference(expected, &canonical, &inputs),
        Err(SizeSetInventoryError::MissingSet { expected })
    );
}
