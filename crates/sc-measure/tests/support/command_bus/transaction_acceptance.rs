//! Public-interface transaction acceptance over real mixed metadata, aliases and finite ownership.
use super::group_commit::{capacity, group, material, mutation};
use super::resource_admission::control;
use super::*;
use sc_core::{
    command::{
        AddNotch, ChangeMeasurement, CommandControlError as Control,
        CommandGroupPreviewError as GroupError, CommandHistoryDirection, CommandHistoryRequest,
        CommandMutationGroup, CommandProgress, CommandProgressDecision as Decision,
        CommandProgressObserver, CommandProgressPhase as Phase,
    },
    ontology::{Notch, NotchError},
    value::{LengthDeclaration, LengthState},
};
use std::{cell::RefCell, error::Error as StdError};

fn fixture() -> (DesignGarment<DesignSources>, CommandMutationGroup) {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let mut length = measurement_preview::original(&owner).definition().clone();
    length.state = LengthState::Unknown {
        observation: id(817100),
    };
    let length = LengthDeclaration::new(length).expect("actual authored Unknown");
    let first = mutation(
        design,
        revision.clone(),
        817000,
        817001,
        MutationPayload::AddNotch(addition),
    );
    let request = group(
        817000,
        vec![
            first.clone(),
            mutation(
                design,
                revision.clone(),
                817000,
                817002,
                material(addition.piece(), "exact mixed transaction intent"),
            ),
            mutation(
                design,
                revision.clone(),
                817000,
                817003,
                MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), length)),
            ),
            mutation(
                design,
                revision,
                817000,
                817004,
                MutationPayload::AddNotch(addition),
            ),
            first,
        ],
    );
    (owner, request)
}

fn configured(owner: DesignGarment<DesignSources>) -> CommandBus<DesignSources, Host> {
    let design = owner.inputs().id();
    controlled(
        CommandBus::with_edit_history(
            owner,
            Host::new(design),
            notch_preview::reserve(),
            capacity(1),
            capacity(1),
            capacity(1),
        )
        .expect("one original authoring slot, undo snapshot and history action"),
    )
}

#[derive(Default)]
struct Trace {
    cancel_at: Option<usize>,
    events: RefCell<Vec<CommandProgress>>,
}
impl CommandProgressObserver for Trace {
    fn observe(&self, event: &CommandProgress) -> Decision {
        let mut events = self.events.borrow_mut();
        let position = events.len();
        events.push(*event);
        if self.cancel_at == Some(position) {
            Decision::Cancel
        } else {
            Decision::Continue
        }
    }
}

#[test]
fn every_observed_pre_effect_checkpoint_preserves_the_complete_mixed_transaction_for_retry() {
    let (owner, request) = fixture();
    let design = owner.inputs().id();
    let base = owner.inputs().revision().clone();
    let original_pieces = owner.inputs().structure().pieces().pieces().to_vec();
    let trace = Trace::default();
    let accepted = configured(owner.clone())
        .commit_group_with_progress(&request, &trace)
        .expect("actual mixed transaction establishes the public progress trace");
    let events = trace.events.into_inner();
    for phase in [
        Phase::Authorized,
        Phase::SnapshotBound,
        Phase::RetainedValidation,
        Phase::RetainedPublication,
        Phase::Preparing,
        Phase::CurrentValidation,
        Phase::CandidateValidation,
        Phase::AllocationPreflight,
        Phase::ReadyToPublish,
    ] {
        assert!(events.iter().any(|event| event.phase() == phase));
    }
    assert_eq!(
        events.last().map(|e| e.phase()),
        Some(Phase::ReadyToPublish)
    );
    assert_eq!(
        accepted
            .entry()
            .commit()
            .changes()
            .steps()
            .get(4)
            .expect("fifth original alias")
            .replay_of(),
        Some(0)
    );
    for (position, expected_event) in events.iter().enumerate() {
        let mut bus = configured(owner.clone());
        let original_address = {
            let CommandResult::PieceList(original) = bus
                .execute(&query(design, base.clone(), QueryPayload::GetPieceList))
                .expect("actual owner before cancellation");
            original.pieces().as_ptr()
        };
        let trace = Trace {
            cancel_at: Some(position),
            ..Trace::default()
        };
        let error = bus
            .commit_group_with_progress(&request, &trace)
            .expect_err("cancellation at every actual pre-effect checkpoint");
        assert_eq!(
            control(&error),
            Some(&Control::Cancelled {
                progress: *expected_event
            })
        );
        assert_eq!(
            &*trace.events.borrow(),
            events
                .get(..=position)
                .expect("prefix of the actual observed trace")
        );
        let CommandResult::PieceList(current) = bus
            .execute(&query(design, base.clone(), QueryPayload::GetPieceList))
            .expect("cancelled transaction preserves exact original revision");
        assert_eq!(current.pieces().as_ptr(), original_address);
        assert_eq!(current.pieces(), original_pieces);
        let retry = bus
            .commit_group(&request)
            .expect("same group/keys, both original IDs and sole audit slot remain available");
        assert!(!retry.is_replay());
        assert_eq!(retry.entry().commit(), accepted.entry().commit());
        assert_eq!(
            retry
                .entry()
                .commit()
                .changes()
                .difference()
                .notches()
                .iter()
                .map(|n| n.id())
                .collect::<Vec<_>>(),
            vec![id(720000), id(720001)]
        );
        assert!(bus
            .commit_group(&request)
            .expect("exact replay at capacity")
            .is_replay());
        let undo = CommandHistoryRequest::new(
            CommandMutationContext::new(
                design,
                actor(),
                retry.entry().commit().revision().clone(),
                CommandIdempotencyKey::from_entity_id(id(817010)),
                Some(request.id()),
            ),
            request.clone(),
            CommandHistoryDirection::Undo,
        )
        .expect("actual accepted group under current operator");
        bus.apply_history(&undo)
            .expect("only the accepted predecessor was retained");
        let CommandResult::PieceList(restored) = bus
            .execute(&query(design, base.clone(), QueryPayload::GetPieceList))
            .expect("original revision restored after actual Undo");
        assert_eq!(restored.pieces().as_ptr(), original_address);
    }
    eprintln!("transaction acceptance: {} actual pre-effect checkpoints across9 required phases; every cancellation preserves same-key/two-ID/one-slot retry, replay and original snapshot Undo", events.len());
}

#[test]
fn later_native_failure_after_all_three_metadata_handlers_preserves_exact_context_and_retry() {
    let (owner, request) = fixture();
    let design = owner.inputs().id();
    let base = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let other = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .get(1)
        .expect("actual second Piece");
    let invalid = AddNotch::new(other.id(), addition.anchor(), addition.representation());
    let expected = Notch::new(
        invalid.definition_with_id(id(720001)),
        other,
        owner.inputs().structure().topology(),
    )
    .expect_err("independent native Notch witness after the first staged identity");
    let accepted = configured(owner.clone())
        .commit_group(&request)
        .expect("actual corrected baseline");
    let mut members = request.mutations().to_vec();
    *members.get_mut(3).expect("fourth original member") = mutation(
        design,
        base.clone(),
        817000,
        817004,
        MutationPayload::AddNotch(invalid),
    );
    let bad = group(817000, members);
    let mut bus = configured(owner);
    let error = bus
        .commit_group(&bad)
        .expect_err("fourth member refuses the whole mixed transaction");
    let preparation = error
        .source()
        .expect("original group cause")
        .downcast_ref::<GroupError<sc_measure::DesignSourcesError>>()
        .expect("original typed group refusal");
    assert!(
        matches!(preparation, GroupError::Mutation { group, index:3, command:CommandId::AddNotch, .. } if *group==request.id())
    );
    let native = preparation
        .source()
        .expect("original preparation cause")
        .source()
        .expect("original native cause");
    assert_eq!(native.downcast_ref::<NotchError>(), Some(&expected));
    bus.execute(&query(design, base, QueryPayload::GetPieceList))
        .expect("original snapshot remains current");
    let retry = bus
        .commit_group(&request)
        .expect("corrected same group/keys retains actual original metadata and both IDs");
    assert!(!retry.is_replay());
    assert_eq!(retry.entry().commit(), accepted.entry().commit());
}
