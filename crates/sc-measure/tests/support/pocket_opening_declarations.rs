//! Actual logical Pocket openings retain domains and current canonical owner without scalar coercion.
#![allow(clippy::expect_used, clippy::panic)]
use super::{garment_ownership, id};
use sc_core::{
    design::{DesignInputs, DesignStructure},
    ontology::*,
};
use sc_measure::DesignSources;

fn with_openings() -> (DesignInputs<DesignSources>, PocketInventory) {
    let definition = garment_ownership::definition();
    let root = definition.sewing.inputs().structure();
    let original = definition.pockets.pockets().first().expect("actual Pocket");
    let pockets = [
        PocketOpening::Declaration(id(10)),
        PocketOpening::Declaration(id(5001)),
        PocketOpening::Profile(ProfileParameterRef::new(id(10))),
        PocketOpening::Profile(ProfileParameterRef::new(id(2000))),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, opening)| {
        let mut input = original.definition().clone();
        input.id = id(214003 - u128::try_from(index).expect("descending authored ordinal"));
        input.opening = opening;
        Pocket::new(
            input,
            root.cut_plan(),
            root.pieces().pieces(),
            root.topology(),
        )
        .expect("actual logical opening")
    })
    .collect();
    (
        definition.sewing.inputs().clone(),
        PocketInventory::new(pockets).expect("exact authored order"),
    )
}

#[test]
fn opening_views_borrow_exact_objects_fields_and_canonical_owner_without_numeric_conversion() {
    let (inputs, pockets) = with_openings();
    let saved = (inputs.inputs().clone(), pockets.clone());
    let views = inputs
        .pocket_opening_declarations(&pockets)
        .expect("logical declarations");
    assert_eq!(views.len(), 4);
    for (index, view) in views.into_iter().enumerate() {
        let original = pockets
            .pockets()
            .get(index)
            .expect("original authored position");
        assert_eq!(view.index(), index);
        assert_eq!(
            view.pocket().id(),
            id(214003 - u128::try_from(index).expect("ordinal"))
        );
        assert!(core::ptr::eq(view.pocket(), original));
        assert!(core::ptr::eq(
            view.opening(),
            &original.definition().opening
        ));
        assert!(core::ptr::eq(view.canonical_inputs(), inputs.inputs()));
        assert_eq!(
            view.opening_validation(),
            PocketOpeningValidation::DeferredToG3
        );
        match view.opening() {
            PocketOpening::Declaration(identity) => {
                assert!(
                    inputs.inputs().length(*identity).is_some()
                        || inputs.parameters().parameter(*identity).is_some()
                );
                assert_eq!(view.profile_validation(), None);
            }
            PocketOpening::Profile(reference) => {
                assert!(
                    inputs.inputs().length(reference.id()).is_some()
                        || inputs.inputs().scalar(reference.id()).is_some()
                );
                assert_eq!(
                    view.profile_validation(),
                    Some(ProfileBindingValidation::DeferredToG4)
                );
            }
        }
        let debug = format!("{view:?}");
        assert!(
            !debug.contains("EntityId") && !debug.contains("Length") && !debug.contains("21400")
        );
    }
    assert_eq!((inputs.inputs(), &pockets), (&saved.0, &saved.1));
}

#[test]
fn metadata_borrowing_preserves_missing_source_repairs_and_explicit_empty_scope_without_readiness()
{
    let (inputs, pockets) = with_openings();
    let mut definition = inputs.definition().clone();
    let mut structure = definition.structure.definition().clone();
    structure.pieces = PieceInventory::new(vec![]).expect("actual current removal");
    structure.cut_plan = CutPlan::new(vec![], &[]).expect("empty copies");
    definition.structure = DesignStructure::new(structure).expect("held source frame retained");
    let current = DesignInputs::new(definition).expect("current input metadata scope");
    assert_eq!(
        current
            .pocket_opening_declarations(&pockets)
            .expect("metadata only")
            .len(),
        4
    );
    let root = current.structure();
    assert!(pockets
        .validate_current(root.cut_plan(), root.pieces().pieces(), root.topology())
        .is_err());
    let empty = PocketInventory::new(vec![]).expect("explicit draft scope");
    assert!(current
        .pocket_opening_declarations(&empty)
        .expect("empty logical list")
        .is_empty());
}
