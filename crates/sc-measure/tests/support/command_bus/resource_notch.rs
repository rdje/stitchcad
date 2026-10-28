//! Real reserved identities, fixed Notch payloads and exact rebuilt-model resource oracle.
use super::group_commit::{capacity, group, mutation};
use super::resource_admission::{control, counts, demand, reset, without_model_limits};
use super::resource_copies::CountedSources;
use super::*;
use sc_core::{
    command::{
        AddNotch, ChangeMeasurement, CommandControlError as Control, CommandPreview,
        CommandPreviewError,
    },
    design::{
        charge_design_garment_resources, DesignResourceLimits, DesignResourceMeter, DesignSewing,
    },
    ontology::{EdgeAnchor, IdError, Notch, NotchDefinition, Param, SewingLandmark},
    value::{LengthDeclaration, LengthState},
};
use std::{error::Error as StdError, num::NonZeroU64};
fn addition(owner: &DesignGarment<CountedSources>) -> AddNotch {
    let piece = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual target Piece");
    AddNotch::new(
        piece.id(),
        EdgeAnchor {
            edge: piece.edges().next().expect("actual edge").edge,
            param: Param::START,
        },
        notch_preview::bindings(),
    )
}
fn limits(owner: &DesignGarment<CountedSources>, extra: usize) -> DesignResourceLimits {
    let actual = demand(owner);
    DesignResourceLimits::new(
        capacity(actual.collection_entries() + extra),
        capacity(actual.text_bytes()),
        NonZeroU64::MAX,
    )
}
fn rebuilt(
    before: &DesignGarment<CountedSources>,
    intent: AddNotch,
    identity: EntityId,
) -> DesignGarment<CountedSources> {
    let structure = before.inputs().structure();
    let piece = structure
        .pieces()
        .pieces()
        .iter()
        .find(|piece| piece.id() == intent.piece())
        .expect("actual target");
    let notch = Notch::new(
        NotchDefinition {
            id: identity,
            anchor: intent.anchor(),
            representation: intent.representation(),
        },
        piece,
        structure.topology(),
    )
    .expect("actual reserved identity and semantic anchor");
    let mut garment = before.definition().clone();
    let mut sewing = garment.sewing.definition().clone();
    sewing.landmarks.push(SewingLandmark::Notch(notch));
    garment.sewing = DesignSewing::new(sewing).expect("actual appended registry");
    let actual = DesignGarment::new(garment).expect("actual appended garment");
    actual
        .validate_current()
        .expect("independently validated actual model");
    actual
}
fn notch_report(report: &CommandPreview) -> &sc_core::command::CommandNotchPreview {
    match report {
        CommandPreview::Notch(notch) => Some(notch),
        _ => None,
    }
    .expect("typed actual Notch result")
}
#[test]
fn actual_append_refuses_before_copy_and_retry_can_free_evidence_without_losing_reserved_identity()
{
    let owner = super::resource_measurement::fixture();
    let intent = addition(&owner);
    let policy = limits(&owner, 0);
    let reserve = notch_preview::reserve();
    let first = *reserve.first().expect("actual first reserved ID");
    let oracle = rebuilt(&owner, intent, first);
    let before = demand(&owner);
    let after = demand(&oracle);
    assert_eq!(after.collection_entries(), before.collection_entries() + 1);
    assert_eq!(after.text_bytes(), before.text_bytes());
    assert_eq!(after.counter_bits(), before.counter_bits());
    let mut meter = DesignResourceMeter::new(policy);
    let expected = charge_design_garment_resources(&oracle, &mut meter)
        .expect_err("actual one-entry append exceeds current maximum");
    assert_eq!(meter.finish(), Err(expected));
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let address = owner.inputs().structure().pieces().pieces().as_ptr();
    let mut replacement = owner
        .inputs()
        .sources()
        .measurement_declaration(id(1), owner.inputs().inputs())
        .expect("actual two-evidence canonical binding")
        .definition()
        .clone();
    replacement.state = LengthState::Unknown {
        observation: id(806010),
    };
    let replacement = LengthDeclaration::new(replacement).expect("actual smaller replacement");
    let member = mutation(
        design,
        revision.clone(),
        806000,
        806001,
        MutationPayload::AddNotch(intent),
    );
    let request = group(806000, vec![member.clone()]);
    let mut bus = without_model_limits(
        CommandBus::with_edit_capacity(owner, Host::new(design), reserve, capacity(1))
            .expect("actual finite reserved owner"),
    );
    bus.install_model_limits(policy)
        .expect("exact original-size policy");
    for path in 0..3 {
        reset(&counters);
        let error: Box<dyn StdError> = match path {
            0 => Box::new(
                bus.preview(&CommandRequest::Mutation(member.clone()))
                    .expect_err("single append quota"),
            ),
            1 => Box::new(bus.preview_group(&request).expect_err("group append quota")),
            _ => Box::new(bus.commit_group(&request).expect_err("commit append quota")),
        };
        assert_eq!(control(error.as_ref()), Some(&Control::Model(expected)));
        assert_eq!(counts(&counters)[0], 0, "no candidate host copy");
        let CommandResult::PieceList(result) = bus
            .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
            .expect("no revision effect");
        assert_eq!(result.pieces().as_ptr(), address);
    }
    let valid = group(
        806000,
        vec![
            mutation(
                design,
                revision,
                806000,
                806002,
                MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), replacement)),
            ),
            member,
        ],
    );
    let accepted = bus
        .commit_group(&valid)
        .expect("free2 evidence entries then append1; keys/capacity retained");
    let notch = notch_report(
        accepted
            .entry()
            .commit()
            .changes()
            .steps()
            .last()
            .expect("actual final step")
            .report(),
    );
    assert_eq!(
        notch.notch().id(),
        first,
        "all refusals preserved the original next reserved identity"
    );
    assert_eq!(
        notch.notch().definition().representation,
        intent.representation()
    );
}
#[test]
fn exact_one_entry_room_admits_repeated_preview_alias_and_rolls_back_later_group_overflow() {
    let owner = super::resource_measurement::fixture();
    let intent = addition(&owner);
    let policy = limits(&owner, 1);
    let reserve = notch_preview::reserve();
    let first = *reserve.first().expect("actual first reserved ID");
    let second = *reserve.get(1).expect("actual second reserved ID");
    let oracle = rebuilt(&rebuilt(&owner, intent, first), intent, second);
    let mut meter = DesignResourceMeter::new(policy);
    let expected = charge_design_garment_resources(&oracle, &mut meter)
        .expect_err("actual two appends exceed one-entry room");
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let member = mutation(
        design,
        revision.clone(),
        806020,
        806021,
        MutationPayload::AddNotch(intent),
    );
    let alias = group(806020, vec![member.clone(), member.clone()]);
    let too_many = group(
        806020,
        vec![
            member.clone(),
            mutation(
                design,
                revision.clone(),
                806020,
                806022,
                MutationPayload::AddNotch(intent),
            ),
        ],
    );
    let mut bus = without_model_limits(
        CommandBus::with_edit_capacity(owner, Host::new(design), reserve, capacity(1))
            .expect("actual one-group finite owner"),
    );
    bus.install_model_limits(policy)
        .expect("exactly one new registry entry");
    for _ in 0..2 {
        reset(&counters);
        let preview = bus
            .preview(&CommandRequest::Mutation(member.clone()))
            .expect("exact one-entry boundary");
        assert_eq!(notch_report(&preview).notch().id(), first);
        assert_eq!(counts(&counters)[0], 1);
        assert_eq!(counts(&counters)[1], 2);
    }
    reset(&counters);
    let report = bus.preview_group(&alias).expect("alias appends only once");
    assert_eq!(report.steps().last().expect("alias").replay_of(), Some(0));
    assert_eq!(counts(&counters)[0], 1);
    assert_eq!(counts(&counters)[1], 2);
    reset(&counters);
    let error = bus
        .commit_group(&too_many)
        .expect_err("second staged append exceeds actual model maximum");
    assert_eq!(control(&error), Some(&Control::Model(expected)));
    assert_eq!(
        counts(&counters)[0],
        1,
        "only first admitted staged candidate was copied"
    );
    let valid = group(806020, vec![member]);
    let accepted = bus
        .commit_group(&valid)
        .expect("failed later member preserved original reserve and accepted-group capacity");
    assert_eq!(
        notch_report(
            accepted
                .entry()
                .commit()
                .changes()
                .steps()
                .first()
                .expect("actual first step")
                .report()
        )
        .notch()
        .id(),
        first
    );
}
#[test]
fn original_missing_target_exhaustion_and_anchor_causes_precede_prospective_quota() {
    for case in 0..3 {
        let owner = super::resource_measurement::fixture();
        let policy = limits(&owner, 0);
        let valid = addition(&owner);
        let structure = owner.inputs().structure();
        let other = structure
            .pieces()
            .pieces()
            .iter()
            .find(|piece| piece.id() != valid.piece())
            .expect("actual other Piece");
        let wrong_anchor = EdgeAnchor {
            edge: other.edges().next().expect("actual other boundary").edge,
            param: valid.anchor().param,
        };
        let intent = if case == 0 {
            AddNotch::new(id(806999), valid.anchor(), valid.representation())
        } else {
            AddNotch::new(valid.piece(), wrong_anchor, valid.representation())
        };
        let reserve = if case == 2 {
            notch_preview::reserve()
        } else {
            vec![]
        };
        let expected_anchor = if let Some(identity) = reserve.first() {
            let target = structure
                .pieces()
                .pieces()
                .iter()
                .find(|piece| piece.id() == valid.piece())
                .expect("actual target");
            Some(
                Notch::new(
                    NotchDefinition {
                        id: *identity,
                        anchor: wrong_anchor,
                        representation: intent.representation(),
                    },
                    target,
                    structure.topology(),
                )
                .expect_err("actual original wrong-Piece anchor refusal"),
            )
        } else {
            None
        };
        let counters = owner.inputs().sources().host().telemetry();
        let design = owner.inputs().id();
        let revision = owner.inputs().revision().clone();
        let request = CommandRequest::Mutation(mutation(
            design,
            revision,
            806030,
            806031,
            MutationPayload::AddNotch(intent),
        ));
        let mut bus = without_model_limits(
            CommandBus::with_identities(owner, Host::new(design), reserve)
                .expect("actual reserved owner"),
        );
        bus.install_model_limits(policy)
            .expect("current fits but append never fits");
        reset(&counters);
        let error = bus
            .preview(&request)
            .expect_err("preserve original target/ID/anchor cause before model growth");
        assert_eq!(control(&error), None);
        let original_cause = match (case, error, expected_anchor) {
            (0, CommandPreviewError::MissingPiece { piece, index: 0 }, _) => piece == id(806999),
            (1, CommandPreviewError::Identity(error), _) => error == IdError::Exhausted,
            (2, CommandPreviewError::Notch(error), Some(expected)) => error == expected,
            _ => false,
        };
        assert!(
            original_cause,
            "original ordered Notch refusal was not preserved"
        );
        assert_eq!(counts(&counters)[0], 0);
        assert_eq!(
            counts(&counters)[1],
            1,
            "no prospective walk before original cause"
        );
    }
}
