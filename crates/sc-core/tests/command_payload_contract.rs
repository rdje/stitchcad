//! Canonical payloads preserve authored state and targets while determining their own identity.
#![allow(clippy::expect_used)]
use sc_core::{
    command::{
        AssignMaterial, ChangeMeasurement, CommandActorClaim as Actor, CommandId,
        CommandIdempotencyKey as Key, CommandMutation, CommandMutationContext as Context,
        DesignRevision, MutationPayload, PieceSelection, PieceSelectionError,
    },
    ontology::{EntityId as Id, MaterialAssignment},
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
use sc_units::Length;

fn context() -> Context {
    Context::new(
        Id::from_bits(15),
        Actor::Human(Id::from_bits(16)),
        DesignRevision::initial(),
        Key::from_entity_id(Id::from_bits(17)),
        None,
    )
}

#[test]
fn measurement_intent_retains_all_five_canonical_states_and_original_context() {
    let value = Length::from_micrometres(10).expect("length");
    let states = [
        LengthState::Known {
            value,
            evidence: vec![Id::from_bits(14)],
        },
        LengthState::Assumed {
            value,
            assumption: Id::from_bits(21),
        },
        LengthState::Unknown {
            observation: Id::from_bits(22),
        },
        LengthState::Preference {
            value: Length::ZERO,
            provenance: Id::from_bits(23),
        },
        LengthState::Derived {
            formula: Id::from_bits(24),
        },
    ];
    for state in states {
        let replacement = LengthDeclaration::new(LengthDeclarationDefinition {
            id: Id::from_bits(12),
            source: Id::from_bits(13),
            state: state.clone(),
        })
        .expect("canonical input");
        let original_read = replacement.authored_value();
        let payload = ChangeMeasurement::new(Id::from_bits(11), replacement.clone());
        assert_eq!(payload.measurement(), Id::from_bits(11));
        assert_eq!(payload.replacement(), &replacement);
        assert_eq!(payload.replacement().state(), &state);
        assert_eq!(payload.replacement().authored_value(), original_read);
        let metadata = context();
        let request = CommandMutation::new(
            metadata.clone(),
            MutationPayload::ChangeMeasurement(payload),
        );
        assert_eq!(request.context(), &metadata);
        assert_eq!(request.command_id(), CommandId::ChangeMeasurement);
        assert_eq!(request.payload().command_id().token(), "ChangeMeasurement");
    }
}

#[test]
fn target_inventory_keeps_authored_order_and_refuses_the_first_actual_duplicate() {
    assert_eq!(
        PieceSelection::new(vec![]),
        Err(PieceSelectionError::NoPieces)
    );
    let ids = vec![Id::from_bits(9), Id::from_bits(u128::MAX), Id::from_bits(1)];
    let selection = PieceSelection::new(ids.clone()).expect("unique pieces");
    assert_eq!(selection.pieces(), ids);
    assert_eq!(
        PieceSelection::new(vec![
            Id::from_bits(4),
            Id::from_bits(2),
            Id::from_bits(4),
            Id::from_bits(2)
        ]),
        Err(PieceSelectionError::DuplicatePiece {
            piece: Id::from_bits(4),
            first: 0,
            duplicate: 2,
        })
    );
}

#[test]
fn material_intent_reuses_the_canonical_reason_rule_without_resolving_identity() {
    let selection = || PieceSelection::new(vec![Id::from_bits(9)]).expect("one piece");
    for reason in ["", "  ", "\t\n", "\u{a0}"] {
        let error = AssignMaterial::new(
            selection(),
            MaterialAssignment::Unresolved(reason.to_owned()),
        )
        .expect_err("canonical material requires nonblank reason");
        assert_eq!(error.reason(), reason);
    }
    for material in [
        MaterialAssignment::Assigned(Id::from_bits(u128::MAX)),
        MaterialAssignment::Unresolved(" material pending ".to_owned()),
    ] {
        let intent = AssignMaterial::new(selection(), material.clone()).expect("explicit intent");
        assert_eq!(intent.pieces().pieces(), &[Id::from_bits(9)]);
        assert_eq!(intent.material(), &material);
        let request = CommandMutation::new(context(), MutationPayload::AssignMaterial(intent));
        assert_eq!(request.command_id(), CommandId::AssignMaterial);
        assert_eq!(request.payload().command_id().token(), "AssignMaterial");
    }
}
