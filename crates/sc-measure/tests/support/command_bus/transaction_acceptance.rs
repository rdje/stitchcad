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

#[test]
fn failed_group_diagnostic_identifies_discarded_preparations_without_repeating_aliases() {
    let (owner, request) = fixture();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let other = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .get(1)
        .expect("actual second Piece");
    let invalid = AddNotch::new(other.id(), addition.anchor(), addition.representation());
    let expected_native = Notch::new(
        invalid.definition_with_id(id(720001)),
        other,
        owner.inputs().structure().topology(),
    )
    .expect_err("independent second-ID native witness");
    let reference = configured(owner.clone())
        .commit_group(&request)
        .expect("actual corrected reference");
    for conflict in [false, true] {
        let mut members = request.mutations().to_vec();
        let alias = members.first().expect("original first Notch").clone();
        members.insert(1, alias);
        *members
            .get_mut(4)
            .expect("original fourth member after alias") = mutation(
            design,
            revision.clone(),
            817000,
            if conflict { 817001 } else { 817004 },
            MutationPayload::AddNotch(invalid),
        );
        let bad = group(817000, members);
        let mut bus = configured(owner.clone());
        let preview_error = bus
            .preview_group(&bad)
            .expect_err("failed actual mixed preview");
        let commit_error = bus
            .commit_group(&bad)
            .expect_err("failed actual mixed commit");
        let commit_preparation = commit_error
            .source()
            .expect("original group source")
            .downcast_ref::<GroupError<sc_measure::DesignSourcesError>>()
            .expect("same original group error type");
        for error in [&preview_error, commit_preparation] {
            let discarded = match error {
                GroupError::Mutation {
                    group,
                    index: 4,
                    command: CommandId::AddNotch,
                    discarded,
                    ..
                } if !conflict => {
                    assert_eq!(*group, request.id());
                    let native = error
                        .source()
                        .expect("original preview source")
                        .source()
                        .expect("original Notch source");
                    assert_eq!(native.downcast_ref::<NotchError>(), Some(&expected_native));
                    discarded
                }
                GroupError::RetryConflict {
                    group,
                    key,
                    first: 0,
                    index: 4,
                    command: CommandId::AddNotch,
                    discarded,
                } if conflict => {
                    assert_eq!(*group, request.id());
                    assert_eq!(*key, CommandIdempotencyKey::from_entity_id(id(817001)));
                    assert!(
                        error.source().is_none(),
                        "a key conflict invents no native preparation cause"
                    );
                    discarded
                }
                error => {
                    unreachable!("original native/conflicting fourth-member context: {error:?}")
                }
            };
            let contexts: Vec<_> = discarded
                .iter()
                .map(|step| (step.index(), step.command(), step.key()))
                .collect();
            assert_eq!(
                contexts,
                vec![
                    (
                        0,
                        CommandId::AddNotch,
                        CommandIdempotencyKey::from_entity_id(id(817001))
                    ),
                    (
                        2,
                        CommandId::AssignMaterial,
                        CommandIdempotencyKey::from_entity_id(id(817002))
                    ),
                    (
                        3,
                        CommandId::ChangeMeasurement,
                        CommandIdempotencyKey::from_entity_id(id(817003))
                    ),
                ],
                "only actual completed first occurrences; no alias1, failing4 or unexecuted5"
            );
            let diagnostic = error.to_string();
            assert!(
                diagnostic.contains("AssignMaterial") && diagnostic.contains("ChangeMeasurement")
            );
        }
        bus.execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
            .expect("original revision after both failures");
        let corrected = bus
            .commit_group(&request)
            .expect("same group/keys and both IDs retain sole publication slot");
        assert!(!corrected.is_replay());
        assert_eq!(corrected.entry().commit(), reference.entry().commit());
    }
    let failed_first = group(
        817000,
        vec![mutation(
            design,
            revision.clone(),
            817000,
            817004,
            MutationPayload::AddNotch(invalid),
        )],
    );
    let mut bus = configured(owner);
    let error = bus
        .preview_group(&failed_first)
        .expect_err("no successful predecessor exists");
    assert!(
        matches!(error, GroupError::Mutation { index:0, discarded, .. } if discarded.is_empty())
    );
    assert!(!bus
        .commit_group(&request)
        .expect("empty-prefix failure also leaves original slot/keys/IDs")
        .is_replay());
}
