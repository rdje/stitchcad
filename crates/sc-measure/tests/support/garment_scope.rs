//! Requested explicit scope precedes current geometry while inspection retains exact held objects.
#![allow(clippy::expect_used, clippy::panic)]
use super::{garment_current, garment_ownership, id};
use sc_core::{
    design::{DesignGarment, DesignGarmentEnvelopeError as Error},
    ontology::*,
};

#[test]
fn modelled_lining_is_inspectable_but_root_execution_retains_original_named_layer_cause() {
    let definition = garment_ownership::definition();
    let saved = definition.clone();
    let garment = DesignGarment::new(definition).expect("actual immutable owner");
    garment
        .validate_current()
        .expect("current original sources");
    garment
        .declarations()
        .expect("original declarations remain inspectable");
    let original = garment
        .layers()
        .require_in_scope()
        .expect_err("original Lining scope");
    let error = garment
        .require_in_scope()
        .expect_err("execution refuses modelled Lining");
    assert_eq!(error, Error::Layers(Box::new(original.clone())));
    let cause = std::error::Error::source(&error)
        .expect("original layer inventory cause")
        .downcast_ref::<LayerInventoryError>()
        .expect("concrete original cause");
    assert_eq!(cause, &original);
    let LayerInventoryError::OutsideEnvelope {
        layer,
        position,
        issue,
    } = cause
    else {
        panic!("original named envelope payload")
    };
    assert_eq!(*layer, id(200301));
    assert_eq!(
        *position,
        LayerInventoryPosition {
            kind: LayerKind::Lining,
            index: 0
        }
    );
    assert_eq!(
        *issue,
        LayerEnvelopeError::LiningDeferred {
            served_piece: id(95000)
        }
    );
    assert_eq!(issue.diagnostic(), "env_lining");
    assert_eq!(issue.proving_gate(), "G7");
    assert!(core::ptr::eq(
        std::error::Error::source(cause)
            .expect("original named cause")
            .downcast_ref::<LayerEnvelopeError>()
            .expect("concrete named cause"),
        issue,
    ));
    let message = error.to_string();
    for expected in [
        "env_lining".to_owned(),
        "G7".to_owned(),
        id(95000).to_string(),
    ] {
        assert!(message.contains(&expected));
    }
    assert_eq!(garment.definition(), &saved);
}

#[test]
fn requested_scope_precedes_damaged_sources_and_success_does_not_certify_currentness() {
    let mut definition = garment_ownership::definition();
    garment_current::lose_edge(&mut definition, 0);
    let saved = definition.clone();
    let garment = DesignGarment::new(definition).expect("held source damage remains inspectable");
    assert!(garment.validate_current().is_err());
    assert!(garment.declarations().is_err());
    assert_eq!(
        garment.require_in_scope(),
        Err(Error::Layers(Box::new(
            garment
                .layers()
                .require_in_scope()
                .expect_err("named Lining guard")
        )))
    );
    assert_eq!(garment.definition(), &saved);
    let mut supported = saved;
    supported.layers = LayerInventory::new(
        supported.layers.facings().to_vec(),
        vec![],
        supported.layers.interfacings().to_vec(),
    )
    .expect("actual supported layer kinds only");
    let saved = supported.clone();
    let garment = DesignGarment::new(supported).expect("same held source damage");
    assert_eq!(garment.require_in_scope(), Ok(()));
    assert!(garment.validate_current().is_err());
    assert!(garment.declarations().is_err());
    assert_eq!(garment.definition(), &saved);
}

#[test]
fn fly_request_keeps_original_pre_birth_refusal_without_modelling_an_immutable_fly() {
    let garment = DesignGarment::new(garment_ownership::definition()).expect("actual owner");
    let saved = garment.definition().clone();
    let mut requested = garment
        .closures()
        .closures()
        .first()
        .expect("actual Closure")
        .definition()
        .clone();
    requested.kind = ClosureKind::Fly {
        trousers_gap: id(230000),
    };
    requested.instances.clear();
    let expected = ClosureEnvelopeError::FlyDeferred {
        closure: requested.id,
        trousers_gap: id(230000),
    };
    assert_eq!(requested.require_in_scope(), Err(expected));
    assert_eq!(expected.diagnostic(), "env_fly");
    assert_eq!(expected.proving_gate(), "G7");
    let structure = garment.inputs().structure();
    let refused = Closure::new(
        requested.clone(),
        &[],
        structure.cut_plan(),
        &[],
        structure.topology(),
    )
    .expect_err("scope precedes empty instances and missing geometry");
    assert_eq!(refused, ClosureError::OutsideEnvelope(expected));
    assert_eq!(
        std::error::Error::source(&refused)
            .expect("original Fly cause")
            .downcast_ref::<ClosureEnvelopeError>(),
        Some(&expected)
    );
    let message = refused.to_string();
    for text in [
        "env_fly".to_owned(),
        "G7".to_owned(),
        requested.id.to_string(),
        id(230000).to_string(),
    ] {
        assert!(message.contains(&text));
    }
    assert_eq!(garment.definition(), &saved);
}
