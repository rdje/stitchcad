//! Actual snapshot roundtrips, original authors/current operators, finite history and branch identity.
use super::group_commit::{capacity, group, group_id, material, mutation};
use super::*;
use sc_core::{
    command::{
        ChangeMeasurement, CommandGroupCommitError as CommitError,
        CommandGroupPreviewError as GroupError, CommandHistoryDirection as Direction,
        CommandHistoryError as HistoryError, CommandHistoryRequest as Request,
        CommandMutationGroup, CommandPreview,
    },
    ontology::MaterialAssignment,
    value::{LengthDeclaration, LengthState},
};

fn operation(
    original: &CommandMutationGroup,
    operator: Actor,
    revision: DesignRevision,
    key: u128,
    direction: Direction,
) -> Request {
    Request::new(
        CommandMutationContext::new(
            original.context().design(),
            operator,
            revision,
            CommandIdempotencyKey::from_entity_id(id(key)),
            Some(original.id()),
        ),
        original.clone(),
        direction,
    )
    .expect("exact original group/current operator intent")
}
fn inspection(design: EntityId, operator: Actor, revision: DesignRevision) -> CommandRequest {
    CommandRequest::Query(CommandQuery::new(
        CommandSnapshotContext::new(design, operator, revision),
        QueryPayload::GetPieceList,
    ))
}
fn human() -> Actor {
    Actor::Human(id(700003))
}
fn operator_host(design: EntityId, commands: Vec<CommandId>) -> Host {
    let mut host = Host::new(design);
    for (entries, status) in [
        (&mut host.active, CommandGrantStatus::Active),
        (&mut host.inactive, CommandGrantStatus::Revoked),
    ] {
        entries.push(
            CommandGrant::new(
                id(790900),
                human(),
                vec![Permission::Inspect, Permission::Commit],
                CommandScope::new(
                    design,
                    commands.clone(),
                    SystemTime::UNIX_EPOCH,
                    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(10),
                )
                .expect("actual human operator scope"),
                status,
            )
            .expect("distinct real human grant"),
        );
    }
    host
}

#[test]
fn undo_redo_moves_complete_actual_snapshots_and_preserves_agent_author_under_human_operator() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let originals = owner.inputs().structure().pieces().pieces().to_vec();
    let original_address = owner.inputs().structure().pieces().pieces().as_ptr();
    let addition = notch_preview::addition(&owner);
    let length = measurement_preview::original(&owner);
    let mut replacement = length.definition().clone();
    replacement.state = LengthState::Unknown {
        observation: id(790100),
    };
    let replacement = LengthDeclaration::new(replacement).expect("exact authored replacement");
    let original = group(
        790000,
        vec![
            mutation(
                design,
                revision.clone(),
                790000,
                790001,
                material(addition.piece(), "accepted full metadata"),
            ),
            mutation(
                design,
                revision.clone(),
                790000,
                790002,
                MutationPayload::ChangeMeasurement(ChangeMeasurement::new(
                    id(1),
                    replacement.clone(),
                )),
            ),
            mutation(
                design,
                revision.clone(),
                790000,
                790003,
                MutationPayload::AddNotch(addition),
            ),
        ],
    );
    let host = operator_host(design, CommandId::ALL.to_vec());
    let session = Rc::clone(&host.actor);
    let mut bus = controlled(
        CommandBus::with_edit_history(
            owner,
            host,
            notch_preview::reserve(),
            capacity(4),
            capacity(2),
            capacity(6),
        )
        .expect("explicit finite author/history/action bounds"),
    );
    let accepted = bus
        .commit_group(&original)
        .expect("actual metadata publication");
    let CommandResult::PieceList(after) = bus
        .execute(&inspection(design, actor(), revision.successor()))
        .expect("actual committed snapshot");
    let accepted_address = after.pieces().as_ptr();
    session.set(Some(human()));
    let undo = operation(
        &original,
        human(),
        revision.successor(),
        790010,
        Direction::Undo,
    );
    let undone = bus
        .apply_history(&undo)
        .expect("human is freshly authorized for all original command scopes");
    assert!(!undone.is_replay());
    assert_eq!(accepted.entry().sequence().to_string(), "1");
    assert_eq!(undone.entry().sequence().to_string(), "2");
    assert_eq!(accepted.entry().checked_at(), undone.entry().checked_at());
    assert_eq!(
        undone.entry().commands(),
        &[
            CommandId::AssignMaterial,
            CommandId::ChangeMeasurement,
            CommandId::AddNotch
        ]
    );
    assert_eq!(undone.entry().request(), &undo);
    assert_eq!(undone.entry().operator(), human());
    assert_eq!(undone.entry().original_actor(), actor());
    assert_eq!(undone.entry().grant(), id(790900));
    assert_eq!(undone.entry().before_revision(), &revision.successor());
    assert_eq!(undone.entry().revision(), &revision);
    let CommandResult::PieceList(before) = bus
        .execute(&inspection(design, human(), revision.clone()))
        .expect("restored actual predecessor");
    assert_eq!(before.pieces(), originals);
    assert_eq!(before.pieces().as_ptr(), original_address);
    let restore_measurement = |revision| {
        CommandRequest::Mutation(CommandMutation::new(
            CommandMutationContext::new(
                design,
                human(),
                revision,
                CommandIdempotencyKey::from_entity_id(id(790011)),
                None,
            ),
            MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), length.clone())),
        ))
    };
    let CommandPreview::Measurement(before) = bus
        .preview(&restore_measurement(revision.clone()))
        .expect("exact original declaration")
    else {
        unreachable!("Measurement report")
    };
    assert!(before.changes().is_empty());
    let redo = operation(
        &original,
        human(),
        revision.clone(),
        790012,
        Direction::Redo,
    );
    let redone = bus
        .apply_history(&redo)
        .expect("same actual accepted snapshot");
    assert_eq!(redone.entry().original_actor(), actor());
    assert_eq!(redone.entry().sequence().to_string(), "3");
    assert_eq!(redone.entry().operator(), human());
    assert_eq!(redone.entry().revision(), &revision.successor());
    let CommandResult::PieceList(after) = bus
        .execute(&inspection(design, human(), revision.successor()))
        .expect("restored actual successor");
    assert_eq!(after.pieces().as_ptr(), accepted_address);
    assert_eq!(
        after
            .pieces()
            .first()
            .expect("actual target")
            .definition()
            .material,
        MaterialAssignment::Unresolved("accepted full metadata".into())
    );
    let CommandPreview::Measurement(after) = bus
        .preview(&restore_measurement(revision.successor()))
        .expect("actual accepted canonical state")
    else {
        unreachable!("Measurement report")
    };
    assert_eq!(
        after
            .changes()
            .first()
            .expect("actual restoration proposal")
            .before(),
        &replacement
    );
    session.set(Some(actor()));
    let replay = bus
        .commit_group(&original)
        .expect("original authoring audit survived both traversals");
    assert!(replay.is_replay());
    assert_eq!(replay.entry(), accepted.entry());
    let next = group(
        790013,
        vec![mutation(
            design,
            revision.successor(),
            790013,
            790014,
            MutationPayload::AddNotch(addition),
        )],
    );
    assert_eq!(
        bus.preview_group(&next)
            .expect("issued IDs never rewound")
            .difference()
            .notches()
            .first()
            .expect("next identity")
            .id(),
        id(720001)
    );
}

#[test]
fn d285_new_branch_never_reuses_old_snapshot_revision_or_issued_identity_and_old_request_refuses() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let first = group(
        790020,
        vec![mutation(
            design,
            revision.clone(),
            790020,
            790021,
            MutationPayload::AddNotch(addition),
        )],
    );
    let old_request = group(
        790022,
        vec![mutation(
            design,
            revision.successor(),
            790022,
            790023,
            material(addition.piece(), "authored against old branch"),
        )],
    );
    let branch = group(
        790024,
        vec![mutation(
            design,
            revision.clone(),
            790024,
            790025,
            MutationPayload::AddNotch(addition),
        )],
    );
    let mut bus = controlled(
        CommandBus::with_edit_history(
            owner,
            Host::new(design),
            vec![id(720000), id(720001), id(720002)],
            capacity(4),
            capacity(3),
            capacity(6),
        )
        .expect("explicit bounds"),
    );
    let original = bus
        .commit_group(&first)
        .expect("first distinct snapshot revision1");
    bus.apply_history(&operation(
        &first,
        actor(),
        revision.successor(),
        790026,
        Direction::Undo,
    ))
    .expect("restore original snapshot/revision0");
    let fresh = bus
        .commit_group(&branch)
        .expect("new authoring branch uses maximum-issued revision");
    assert_eq!(
        fresh.entry().commit().revision(),
        &revision.successor().successor()
    );
    assert_eq!(fresh.entry().commit().changes().revision(), &revision);
    assert_eq!(
        fresh
            .entry()
            .commit()
            .changes()
            .difference()
            .notches()
            .first()
            .expect("new branch identity")
            .id(),
        id(720001)
    );
    assert_eq!(
        fresh.entry().commit().history().discarded_redo(),
        &[first.id()]
    );
    let CommitError::Precondition { issue, changes, .. } = bus
        .commit_group(&old_request)
        .expect_err("old branch precondition refuses")
    else {
        unreachable!("actual stale precondition")
    };
    assert!(
        matches!(&*issue,GroupError::RevisionMismatch {requested,current,..} if requested==&revision.successor() && current==&revision.successor().successor())
    );
    assert_eq!(changes.len(), 2);
    let undo_change = changes
        .first()
        .expect("actual undo transition after old snapshot");
    assert_eq!(undo_change.sequence.to_string(), "2");
    assert_eq!(undo_change.direction, Some(Direction::Undo));
    assert_eq!(undo_change.revision, revision);
    let branch_change = changes
        .last()
        .expect("actual branch transition follows undo");
    assert_eq!(branch_change.sequence.to_string(), "3");
    assert_eq!(branch_change.direction, None);
    assert_eq!(branch_change.revision, revision.successor().successor());
    assert!(matches!(
        bus.apply_history(&operation(
            &first,
            actor(),
            revision.successor().successor(),
            790027,
            Direction::Redo
        )),
        Err(HistoryError::Empty {
            direction: Direction::Redo
        })
    ));
    let replay = bus
        .commit_group(&first)
        .expect("original transport retry returns first result without restoring discarded branch");
    assert!(replay.is_replay());
    assert_eq!(replay.entry(), original.entry());
    bus.execute(&inspection(
        design,
        actor(),
        revision.successor().successor(),
    ))
    .expect("replay left current branch at2");
    let next = group(
        790028,
        vec![mutation(
            design,
            revision.successor().successor(),
            790028,
            790029,
            MutationPayload::AddNotch(addition),
        )],
    );
    assert_eq!(
        bus.preview_group(&next)
            .expect("neither undo nor stale refusal rewinds IDs")
            .difference()
            .notches()
            .first()
            .expect("remaining identity")
            .id(),
        id(720002)
    );
}

#[test]
fn bounded_depth_reports_oldest_drop_without_erasing_authoring_audit_or_partial_traversal() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let first = group(
        790030,
        vec![mutation(
            design,
            revision.clone(),
            790030,
            790031,
            material(addition.piece(), "first"),
        )],
    );
    let second = group(
        790032,
        vec![mutation(
            design,
            revision.successor(),
            790032,
            790033,
            material(addition.piece(), "second"),
        )],
    );
    let mut bus = controlled(
        CommandBus::with_edit_history(
            owner,
            Host::new(design),
            vec![],
            capacity(4),
            capacity(1),
            capacity(4),
        )
        .expect("one reversible group"),
    );
    let accepted = bus.commit_group(&first).expect("first");
    assert_eq!(accepted.entry().commit().history().dropped_undo(), None);
    let latest = bus.commit_group(&second).expect("second");
    assert_eq!(
        latest.entry().commit().history().dropped_undo(),
        Some(first.id())
    );
    assert!(
        matches!(bus.apply_history(&operation(&first,actor(),revision.successor().successor(),790034,Direction::Undo)),Err(HistoryError::NotCurrent {group,direction:Direction::Undo}) if group==first.id())
    );
    bus.apply_history(&operation(
        &second,
        actor(),
        revision.successor().successor(),
        790035,
        Direction::Undo,
    ))
    .expect("latest complete group restores exact predecessor");
    let CommandResult::PieceList(actual) = bus
        .execute(&inspection(design, actor(), revision.successor()))
        .expect("actual first snapshot");
    assert_eq!(
        actual
            .pieces()
            .first()
            .expect("target")
            .definition()
            .material,
        MaterialAssignment::Unresolved("first".into())
    );
    assert!(matches!(
        bus.apply_history(&operation(
            &first,
            actor(),
            revision.successor(),
            790036,
            Direction::Undo
        )),
        Err(HistoryError::Empty {
            direction: Direction::Undo
        })
    ));
    assert_eq!(
        bus.commit_group(&first)
            .expect("oldest original audit/replay remains")
            .entry(),
        accepted.entry()
    );
}

#[test]
fn history_operation_retry_and_shared_actor_key_namespace_preserve_first_result_at_action_limit() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let original = group(
        790040,
        vec![mutation(
            design,
            revision.clone(),
            790040,
            790041,
            material(addition.piece(), "first"),
        )],
    );
    let host = Host::new(design);
    let revoked = Rc::clone(&host.revoked);
    let mut bus = controlled(
        CommandBus::with_edit_history(owner, host, vec![], capacity(4), capacity(2), capacity(1))
            .expect("one immutable history action"),
    );
    bus.commit_group(&original).expect("original accepted");
    let undo = operation(
        &original,
        actor(),
        revision.successor(),
        790042,
        Direction::Undo,
    );
    let accepted = bus.apply_history(&undo).expect("one action accepted");
    let repeated = bus
        .apply_history(&undo)
        .expect("exact retry before current revision/stack/limit");
    assert!(repeated.is_replay());
    assert_eq!(repeated.entry(), accepted.entry());
    assert!(
        matches!(bus.apply_history(&operation(&original,actor(),revision.clone(),790042,Direction::Redo)),Err(HistoryError::KeyConflict {key}) if key==CommandIdempotencyKey::from_entity_id(id(790042)))
    );
    assert!(matches!(
        bus.apply_history(&operation(
            &original,
            actor(),
            revision.clone(),
            790043,
            Direction::Redo
        )),
        Err(HistoryError::ActionLimit { limit: 1 })
    ));
    assert!(
        matches!(bus.apply_history(&operation(&original,actor(),revision.clone(),790041,Direction::Redo)),Err(HistoryError::KeyConflict {key}) if key==CommandIdempotencyKey::from_entity_id(id(790041)))
    );
    let branch = group(
        790044,
        vec![mutation(
            design,
            revision.clone(),
            790044,
            790045,
            material(addition.piece(), "new branch"),
        )],
    );
    bus.commit_group(&branch)
        .expect("action capacity does not consume authoring capacity");
    let repeated = bus
        .apply_history(&undo)
        .expect("original action replay survives a discarded redo branch");
    assert!(repeated.is_replay());
    assert_eq!(repeated.entry(), accepted.entry());
    let reuse = group(
        790046,
        vec![mutation(
            design,
            revision.successor().successor(),
            790046,
            790042,
            material(addition.piece(), "must not publish"),
        )],
    );
    assert!(
        matches!(bus.commit_group(&reuse),Err(CommitError::KeyConflict {key,..}) if key==CommandIdempotencyKey::from_entity_id(id(790042)))
    );
    let CommandResult::PieceList(actual) = bus
        .execute(&inspection(
            design,
            actor(),
            revision.successor().successor(),
        ))
        .expect("replay never moved current branch");
    assert_eq!(
        actual
            .pieces()
            .first()
            .expect("target")
            .definition()
            .material,
        MaterialAssignment::Unresolved("new branch".into())
    );
    revoked.set(true);
    assert!(
        matches!(bus.apply_history(&undo),Err(HistoryError::Preparation(issue)) if matches!(*issue,GroupError::Authorization(_)))
    );
}

#[test]
fn history_binds_complete_original_record_and_current_revision_before_any_stack_transition() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let original = group(
        790050,
        vec![mutation(
            design,
            revision.clone(),
            790050,
            790051,
            material(addition.piece(), "original"),
        )],
    );
    let forged = group(
        790050,
        vec![mutation(
            design,
            revision.clone(),
            790050,
            790051,
            material(addition.piece(), "forged"),
        )],
    );
    let unknown = group(
        790052,
        vec![mutation(
            design,
            revision.clone(),
            790052,
            790053,
            material(addition.piece(), "not accepted"),
        )],
    );
    let mut bus = controlled(
        CommandBus::with_edit_history(
            owner,
            Host::new(design),
            vec![],
            capacity(2),
            capacity(1),
            capacity(1),
        )
        .expect("one actual reversible group/action"),
    );
    bus.commit_group(&original).expect("accepted");
    assert!(
        matches!(bus.apply_history(&operation(&forged,actor(),revision.successor(),790054,Direction::Undo)),Err(HistoryError::GroupConflict {group}) if group==group_id(790050))
    );
    assert!(
        matches!(bus.apply_history(&operation(&unknown,actor(),revision.successor(),790054,Direction::Undo)),Err(HistoryError::GroupNotAccepted {group}) if group==group_id(790052))
    );
    assert!(
        matches!(bus.apply_history(&operation(&original,actor(),revision.successor().successor(),790054,Direction::Undo)),Err(HistoryError::Precondition {issue,..}) if matches!(&*issue,GroupError::RevisionMismatch {requested,current,..} if requested==&revision.successor().successor() && current==&revision.successor()))
    );
    let result = bus
        .apply_history(&operation(
            &original,
            actor(),
            revision.successor(),
            790054,
            Direction::Undo,
        ))
        .expect("failed attempts booked neither key nor action nor moved stack");
    assert!(!result.is_replay());
    assert_eq!(result.entry().revision(), &revision);
}

struct BoundaryHost {
    host: Host,
    revoke_at: Rc<Cell<Option<usize>>>,
}
impl CommandAuthoritySource for BoundaryHost {
    fn snapshot(&self) -> CommandAuthoritySnapshot<'_> {
        if self
            .revoke_at
            .get()
            .is_some_and(|threshold| self.host.reads.get() + 1 >= threshold)
        {
            self.host.revoked.set(true);
        }
        self.host.snapshot()
    }
}
#[test]
fn history_effect_boundary_revocation_preserves_actual_model_stacks_action_capacity_and_unused_key()
{
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let original = group(
        790060,
        vec![mutation(
            design,
            revision.clone(),
            790060,
            790061,
            material(addition.piece(), "actual accepted"),
        )],
    );
    let host = Host::new(design);
    let reads = Rc::clone(&host.reads);
    let revoked = Rc::clone(&host.revoked);
    let revoke_at = Rc::new(Cell::new(None));
    let mut bus = controlled(
        CommandBus::with_edit_history(
            owner,
            BoundaryHost {
                host,
                revoke_at: Rc::clone(&revoke_at),
            },
            vec![],
            capacity(2),
            capacity(1),
            capacity(1),
        )
        .expect("finite history/action capacity"),
    );
    bus.commit_group(&original).expect("accepted");
    let CommandResult::PieceList(actual) = bus
        .execute(&inspection(design, actor(), revision.successor()))
        .expect("actual represented snapshot");
    let address = actual.pieces().as_ptr();
    let operation = operation(
        &original,
        actor(),
        revision.successor(),
        790062,
        Direction::Undo,
    );
    revoke_at.set(Some(reads.get() + 2));
    assert!(
        matches!(bus.apply_history(&operation),Err(HistoryError::Preparation(issue)) if matches!(*issue,GroupError::Authorization(_)))
    );
    revoke_at.set(None);
    revoked.set(false);
    let CommandResult::PieceList(actual) = bus
        .execute(&inspection(design, actor(), revision.successor()))
        .expect("same actual model after final authority refusal");
    assert_eq!(actual.pieces().as_ptr(), address);
    assert_eq!(
        actual
            .pieces()
            .first()
            .expect("target")
            .definition()
            .material,
        MaterialAssignment::Unresolved("actual accepted".into())
    );
    let accepted = bus
        .apply_history(&operation)
        .expect("original action key, stack and capacity remain available");
    assert!(!accepted.is_replay());
    assert_eq!(accepted.entry().revision(), &revision);
}

#[test]
fn current_operator_needs_one_complete_grant_over_every_original_command_scope() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let original = group(
        790070,
        vec![
            mutation(
                design,
                revision.clone(),
                790070,
                790071,
                material(addition.piece(), "actual"),
            ),
            mutation(
                design,
                revision.clone(),
                790070,
                790072,
                MutationPayload::AddNotch(addition),
            ),
        ],
    );
    let host = operator_host(design, vec![CommandId::AssignMaterial]);
    let session = Rc::clone(&host.actor);
    let mut bus = controlled(
        CommandBus::with_edit_history(
            owner,
            host,
            notch_preview::reserve(),
            capacity(2),
            capacity(1),
            capacity(2),
        )
        .expect("finite actual history"),
    );
    bus.commit_group(&original)
        .expect("original agent covers both commands");
    session.set(Some(human()));
    let error = bus
        .apply_history(&operation(
            &original,
            human(),
            revision.successor(),
            790073,
            Direction::Undo,
        ))
        .expect_err("human Material scope cannot undo the original Notch too");
    let HistoryError::Preparation(issue) = error else {
        unreachable!("original authority refusal")
    };
    let GroupError::Authorization(
        sc_core::command::CommandGroupAuthorizationError::NoMatchingGrant { rejections, .. },
    ) = *issue
    else {
        unreachable!("no complete original scope grant")
    };
    let human_rejection = rejections.last().expect("original human grant refusal");
    assert_eq!(human_rejection.grant, id(790900));
    assert_eq!(human_rejection.index, 1);
    assert_eq!(human_rejection.command, CommandId::AddNotch);
    session.set(Some(actor()));
    bus.apply_history(&operation(
        &original,
        actor(),
        revision.successor(),
        790074,
        Direction::Undo,
    ))
    .expect("original authorized actor can still traverse the unchanged complete group");
}
