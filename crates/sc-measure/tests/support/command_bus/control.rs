//! Actual policy enforcement and cooperative pre-effect cancellation over native aggregate models.
use super::group_commit::{capacity, group, material, mutation};
use super::*;
use sc_core::{
    command::{
        AssignMaterial, CommandControlError as ControlError,
        CommandGroupCommitError as CommitError, CommandGroupPreviewError as GroupError,
        CommandHistoryDirection as Direction, CommandHistoryRequest,
        CommandOperationLimitError as LimitError, CommandOperationLimits, CommandPreview,
        CommandProgress, CommandProgressDecision as Decision, CommandProgressObserver,
        CommandProgressPhase as Phase, PieceSelection,
    },
    ontology::MaterialAssignment,
};
use std::{cell::RefCell, error::Error as StdError};

fn policy(members: usize, targets: usize, bytes: usize) -> CommandOperationLimits {
    CommandOperationLimits::new(capacity(members), capacity(targets), capacity(bytes))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Stop {
    phase: Phase,
    completed: usize,
    member: Option<(usize, CommandId)>,
}
impl Stop {
    fn new(phase: Phase, completed: usize, member: Option<(usize, CommandId)>) -> Self {
        Self {
            phase,
            completed,
            member,
        }
    }
}
#[derive(Default)]
struct Observer {
    events: RefCell<Vec<CommandProgress>>,
    stop: Option<Stop>,
    revoke: Option<Rc<Cell<bool>>>,
}
impl CommandProgressObserver for Observer {
    fn observe(&self, event: &CommandProgress) -> Decision {
        self.events.borrow_mut().push(*event);
        if event.phase() == Phase::ReadyToPublish {
            if let Some(revoked) = &self.revoke {
                revoked.set(true);
            }
        }
        if self.stop
            == Some(Stop {
                phase: event.phase(),
                completed: event.completed(),
                member: event.member(),
            })
        {
            Decision::Cancel
        } else {
            Decision::Continue
        }
    }
}
fn control_error<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a ControlError> {
    let mut current = Some(error);
    while let Some(error) = current {
        if let Some(control) = error.downcast_ref::<ControlError>() {
            return Some(control);
        }
        current = error.source();
    }
    None
}
fn history_request(
    original: &sc_core::command::CommandMutationGroup,
    revision: DesignRevision,
    key: u128,
    direction: Direction,
) -> CommandHistoryRequest {
    CommandHistoryRequest::new(
        CommandMutationContext::new(
            original.context().design(),
            actor(),
            revision,
            CommandIdempotencyKey::from_entity_id(id(key)),
            Some(original.id()),
        ),
        original.clone(),
        direction,
    )
    .expect("explicit original target/current operation")
}

#[test]
fn operation_policy_installs_once_and_missing_policy_never_blocks_authorized_piece_inspection() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let single = mutation(
        design,
        revision.clone(),
        800000,
        800001,
        MutationPayload::AddNotch(addition),
    );
    let mut bus = CommandBus::with_identities(owner, Host::new(design), notch_preview::reserve())
        .expect("actual reserved identities");
    bus.execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
        .expect("cheap authorized metadata inspection needs no preparation policy");
    assert!(!bus.is_preview_available(CommandId::AddNotch));
    let error = bus
        .preview(&CommandRequest::Mutation(single.clone()))
        .expect_err("no hidden operation defaults");
    assert_eq!(
        control_error(&error),
        Some(&ControlError::LimitsNotInstalled)
    );
    bus.install_operation_limits(policy(1, 2, 8))
        .expect("one trusted policy");
    assert!(bus.is_preview_available(CommandId::AddNotch));
    assert_eq!(
        bus.install_operation_limits(policy(9, 9, 99)),
        Err(ControlError::LimitsAlreadyInstalled)
    );
    let twice = group(800000, vec![single.clone(), single]);
    let error = bus
        .preview_group(&twice)
        .expect_err("original member limit cannot be widened");
    assert_eq!(
        control_error(&error),
        Some(&ControlError::Input(LimitError::Members {
            maximum: 1,
            requested: 2
        }))
    );
}

#[test]
fn target_and_reason_admission_refuse_before_lookup_authority_progress_or_any_accepted_record() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let pieces: Vec<_> = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .iter()
        .take(2)
        .map(|piece| piece.id())
        .collect();
    let address = owner.inputs().structure().pieces().pieces().as_ptr();
    let host = Host::new(design);
    let reads = Rc::clone(&host.reads);
    let mut bus = CommandBus::with_edit_capacity(owner, host, vec![], capacity(1))
        .expect("one accepted group");
    bus.install_operation_limits(policy(2, 2, 3))
        .expect("explicit boundaries");
    let assignment = |selected, material| {
        MutationPayload::AssignMaterial(
            AssignMaterial::new(
                PieceSelection::new(selected).expect("distinct selection"),
                material,
            )
            .expect("nonblank intent"),
        )
    };
    let oversized = group(
        800010,
        vec![mutation(
            design,
            revision.clone(),
            800010,
            800011,
            assignment(
                vec![id(799991), id(799992), id(799993)],
                MaterialAssignment::Assigned(id(799994)),
            ),
        )],
    );
    let observer = Observer::default();
    let error = bus
        .preview_group_with_progress(&oversized, &observer)
        .expect_err("target count before missing target/source lookup");
    assert_eq!(
        control_error(&error),
        Some(&ControlError::Input(LimitError::Targets {
            maximum: 2,
            consumed: 0,
            requested: 3,
            index: 0,
            command: CommandId::AssignMaterial
        }))
    );
    let bytes = group(
        800012,
        vec![mutation(
            design,
            revision.clone(),
            800012,
            800013,
            assignment(pieces.clone(), MaterialAssignment::Unresolved("é".into())),
        )],
    );
    let error = bus
        .commit_group_with_progress(&bytes, &observer)
        .expect_err("two proposed two-byte fields exceed3");
    assert_eq!(
        control_error(&error),
        Some(&ControlError::Input(LimitError::ReasonBytes {
            maximum: 3,
            consumed: 0,
            copies: 2,
            bytes_per_copy: 2,
            index: 0,
            command: CommandId::AssignMaterial
        }))
    );
    assert_eq!(
        reads.get(),
        0,
        "pure input admission does not inspect private model/authority facts"
    );
    assert!(observer.events.borrow().is_empty());
    let CommandResult::PieceList(original) = bus
        .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
        .expect("original owner");
    assert_eq!(original.pieces().as_ptr(), address);
    let corrected = group(
        800012,
        vec![mutation(
            design,
            revision,
            800012,
            800013,
            assignment(pieces, MaterialAssignment::Assigned(id(220000))),
        )],
    );
    assert!(!bus
        .commit_group(&corrected)
        .expect("failed admission booked no key/audit capacity")
        .is_replay());
}

#[test]
fn cancelling_single_preview_after_actual_candidate_validation_preserves_original_identity() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let request = CommandRequest::Mutation(mutation(
        design,
        revision,
        800020,
        800021,
        MutationPayload::AddNotch(addition),
    ));
    let bus = controlled(
        CommandBus::with_identities(owner, Host::new(design), notch_preview::reserve())
            .expect("actual reserve"),
    );
    let observer = Observer {
        stop: Some(Stop::new(
            Phase::CandidateValidation,
            1,
            Some((0, CommandId::AddNotch)),
        )),
        ..Observer::default()
    };
    let error = bus
        .preview_with_progress(&request, &observer)
        .expect_err("candidate existed, but was never published");
    let Some(ControlError::Cancelled { progress }) = control_error(&error) else {
        unreachable!("typed cancellation")
    };
    assert_eq!(progress.phase(), Phase::CandidateValidation);
    assert_eq!(progress.completed(), 1);
    assert_eq!(progress.total(), capacity(1));
    let CommandPreview::Notch(next) = bus
        .preview(&request)
        .expect("same original reserve after cancellation")
    else {
        unreachable!("Notch report")
    };
    assert_eq!(next.notch().id(), id(720000));
}

#[test]
fn cancelling_after_first_real_group_member_keeps_all_model_reserve_audit_and_alias_work_atomic() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let first = mutation(
        design,
        revision.clone(),
        800030,
        800031,
        MutationPayload::AddNotch(addition),
    );
    let request = group(
        800030,
        vec![
            first.clone(),
            mutation(
                design,
                revision.clone(),
                800030,
                800032,
                MutationPayload::AddNotch(addition),
            ),
            first,
        ],
    );
    let mut bus = controlled(
        CommandBus::with_edit_history(
            owner,
            Host::new(design),
            notch_preview::reserve(),
            capacity(1),
            capacity(1),
            capacity(1),
        )
        .expect("finite actual owner"),
    );
    let stop = Observer {
        stop: Some(Stop::new(
            Phase::Preparing,
            1,
            Some((0, CommandId::AddNotch)),
        )),
        ..Observer::default()
    };
    let preview_error = bus
        .preview_group_with_progress(&request, &stop)
        .expect_err("group inspect-preview cancellation discards actual candidates");
    assert!(matches!(
        control_error(&preview_error),
        Some(ControlError::Cancelled { .. })
    ));
    let error = bus
        .commit_group_with_progress(&request, &stop)
        .expect_err("first actual staged model/report is discarded");
    assert!(
        matches!(control_error(&error),Some(ControlError::Cancelled {progress}) if progress.member()==Some((0,CommandId::AddNotch)))
    );
    bus.execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
        .expect("no revision publication");
    let continued = Observer::default();
    let result = bus
        .commit_group_with_progress(&request, &continued)
        .expect("same group/keys/IDs remain unused");
    assert!(!result.is_replay());
    assert_eq!(result.entry().sequence().to_string(), "1");
    assert_eq!(
        result
            .entry()
            .commit()
            .changes()
            .difference()
            .notches()
            .iter()
            .map(|notch| notch.id())
            .collect::<Vec<_>>(),
        vec![id(720000), id(720001)]
    );
    let events = continued.events.borrow();
    let prepared: Vec<_> = events
        .iter()
        .filter(|event| event.phase() == Phase::Preparing)
        .map(|event| {
            (
                event.completed(),
                event.total().get(),
                event.member().map(|(index, _)| index),
            )
        })
        .collect();
    assert_eq!(
        prepared,
        vec![
            (0, 3, Some(0)),
            (1, 3, Some(0)),
            (1, 3, Some(1)),
            (2, 3, Some(1)),
            (2, 3, Some(2)),
            (3, 3, Some(2))
        ]
    );
    assert_eq!(
        events.last().expect("last pre-effect event").phase(),
        Phase::ReadyToPublish
    );
}

#[test]
fn ready_cancellation_and_ready_observer_revocation_both_leave_publication_and_keys_unused() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let request = group(
        800040,
        vec![mutation(
            design,
            revision.clone(),
            800040,
            800041,
            MutationPayload::AddNotch(addition),
        )],
    );
    let host = Host::new(design);
    let reads = Rc::clone(&host.reads);
    let revoked = Rc::clone(&host.revoked);
    let mut bus = controlled(
        CommandBus::with_edit_capacity(owner, host, notch_preview::reserve(), capacity(1))
            .expect("finite owner"),
    );
    let cancelled = Observer {
        stop: Some(Stop::new(Phase::ReadyToPublish, 1, None)),
        ..Observer::default()
    };
    let error = bus
        .commit_group_with_progress(&request, &cancelled)
        .expect_err("all preparation done, no effect");
    assert!(
        matches!(control_error(&error),Some(ControlError::Cancelled {progress}) if progress.phase()==Phase::ReadyToPublish)
    );
    assert_eq!(reads.get(), 1);
    let revoke = Observer {
        revoke: Some(Rc::clone(&revoked)),
        ..Observer::default()
    };
    assert!(
        matches!(bus.commit_group_with_progress(&request,&revoke),Err(CommitError::Preparation(issue)) if matches!(*issue,GroupError::Authorization(_)))
    );
    assert_eq!(
        reads.get(),
        3,
        "final authority check follows the last observer"
    );
    revoked.set(false);
    bus.execute(&query(design, revision, QueryPayload::GetPieceList))
        .expect("original actual revision");
    let result = bus
        .commit_group(&request)
        .expect("same group/key and capacity still unused");
    assert!(!result.is_replay());
    assert_eq!(result.entry().sequence().to_string(), "1");
    assert_eq!(
        result
            .entry()
            .commit()
            .changes()
            .difference()
            .notches()
            .first()
            .expect("original ID")
            .id(),
        id(720000)
    );
}

#[test]
fn history_ready_cancellation_preserves_model_stack_action_budget_and_exact_retry_event_order() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let original = group(
        800050,
        vec![mutation(
            design,
            revision.clone(),
            800050,
            800051,
            material(addition.piece(), "accepted"),
        )],
    );
    let mut bus = controlled(
        CommandBus::with_edit_history(
            owner,
            Host::new(design),
            vec![],
            capacity(1),
            capacity(1),
            capacity(2),
        )
        .expect("finite owner"),
    );
    bus.commit_group(&original).expect("authoring effect1");
    let undo = history_request(&original, revision.successor(), 800052, Direction::Undo);
    let stop = Observer {
        stop: Some(Stop::new(Phase::ReadyToPublish, 1, None)),
        ..Observer::default()
    };
    let error = bus
        .apply_history_with_progress(&undo, &stop)
        .expect_err("do not pop stack/book key at cancellation");
    assert!(matches!(
        control_error(&error),
        Some(ControlError::Cancelled { .. })
    ));
    bus.execute(&query(
        design,
        revision.successor(),
        QueryPayload::GetPieceList,
    ))
    .expect("same represented model");
    let undone = bus
        .apply_history(&undo)
        .expect("same action key/capacity remains");
    assert!(!undone.is_replay());
    assert_eq!(undone.entry().sequence().to_string(), "2");
    assert_eq!(undone.entry().revision(), &revision);
    let redo = history_request(&original, revision.clone(), 800053, Direction::Redo);
    let redone = bus
        .apply_history(&redo)
        .expect("original reciprocal snapshot still exists");
    assert_eq!(redone.entry().sequence().to_string(), "3");
    let replay = Observer::default();
    let result = bus
        .apply_history_with_progress(&undo, &replay)
        .expect("exact action retry after later traversal/full capacity");
    assert!(result.is_replay());
    assert_eq!(result.entry(), undone.entry());
    assert_eq!(
        replay
            .events
            .borrow()
            .last()
            .expect("exact replay phase")
            .phase(),
        Phase::Replaying
    );
    bus.execute(&query(
        design,
        revision.successor(),
        QueryPayload::GetPieceList,
    ))
    .expect("replay did not traverse again");
}

#[test]
fn unauthorized_requests_emit_no_progress_and_original_target_errors_keep_their_cause() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let host = Host::new(design);
    let session = Rc::clone(&host.actor);
    session.set(None);
    let bus = controlled(CommandBus::new(owner, host));
    let request = group(
        800060,
        vec![mutation(
            design,
            revision,
            800060,
            800061,
            material(id(799999), "intent"),
        )],
    );
    let observer = Observer::default();
    assert!(matches!(
        bus.preview_group_with_progress(&request, &observer),
        Err(GroupError::Authorization(_))
    ));
    assert!(observer.events.borrow().is_empty());
    session.set(Some(actor()));
    let error = bus
        .preview_group_with_progress(&request, &observer)
        .expect_err("original missing target");
    assert!(
        matches!(error,GroupError::Mutation {index:0,command:CommandId::AssignMaterial,issue,..} if matches!(*issue,sc_core::command::CommandPreviewError::MissingPiece {piece,index:0} if piece==id(799999)))
    );
}
