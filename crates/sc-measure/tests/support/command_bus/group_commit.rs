//! Actual in-memory effects, exact result replay and fresh authorization at the publication boundary.
use super::*;
use sc_core::{
    command::{
        AssignMaterial, ChangeMeasurement, CommandGroupCommitError as CommitError, CommandGroupId,
        CommandGroupPreviewError as GroupError, CommandMutationGroup, CommandPreview,
        PieceSelection,
    },
    ontology::{MaterialAssignment, NotchError},
    value::{LengthDeclaration, LengthState},
};
use std::{error::Error as StdError, num::NonZeroUsize};

fn group_id(value: u128) -> CommandGroupId {
    CommandGroupId::from_entity_id(id(value))
}
fn mutation(
    design: EntityId,
    revision: DesignRevision,
    group: u128,
    key: u128,
    payload: MutationPayload,
) -> CommandMutation {
    CommandMutation::new(
        CommandMutationContext::new(
            design,
            actor(),
            revision,
            CommandIdempotencyKey::from_entity_id(id(key)),
            Some(group_id(group)),
        ),
        payload,
    )
}
fn group(group: u128, members: Vec<CommandMutation>) -> CommandMutationGroup {
    CommandMutationGroup::new(group_id(group), members).expect("actual exact common claims")
}
fn material(piece: EntityId, reason: &str) -> MutationPayload {
    MutationPayload::AssignMaterial(
        AssignMaterial::new(
            PieceSelection::new(vec![piece]).expect("original target"),
            MaterialAssignment::Unresolved(reason.into()),
        )
        .expect("actual explicit intent"),
    )
}
fn capacity(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("explicit finite host capacity")
}

#[test]
fn commit_publishes_actual_metadata_once_with_successor_revision_original_actor_and_used_ids() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let original = measurement_preview::original(&owner);
    let mut replacement = original.definition().clone();
    replacement.state = LengthState::Unknown {
        observation: id(760100),
    };
    let replacement = LengthDeclaration::new(replacement).expect("exact authored unknown");
    let notch = mutation(
        design,
        revision.clone(),
        760000,
        760003,
        MutationPayload::AddNotch(addition),
    );
    let request = group(
        760000,
        vec![
            mutation(
                design,
                revision.clone(),
                760000,
                760001,
                material(addition.piece(), "actual committed reason"),
            ),
            mutation(
                design,
                revision.clone(),
                760000,
                760002,
                MutationPayload::ChangeMeasurement(ChangeMeasurement::new(
                    id(1),
                    replacement.clone(),
                )),
            ),
            notch.clone(),
            mutation(
                design,
                revision.clone(),
                760000,
                760004,
                MutationPayload::AddNotch(addition),
            ),
            notch,
        ],
    );
    let host = Host::new(design);
    let reads = Rc::clone(&host.reads);
    let mut bus = CommandBus::with_edit_capacity(
        owner,
        host,
        vec![id(720000), id(720001), id(720002)],
        capacity(3),
    )
    .expect("explicit configured owner");
    assert!(bus.is_commit_available(CommandId::AddNotch));
    assert!(!bus.is_commit_available(CommandId::CloseDart));
    let committed = bus
        .commit_group(&request)
        .expect("actual atomic publication");
    assert!(!committed.is_replay());
    assert_eq!(
        reads.get(),
        2,
        "initial and effect-boundary fresh authority"
    );
    let audit = committed.entry();
    assert_eq!(audit.request(), &request);
    assert_eq!(audit.actor(), actor());
    assert_eq!(audit.grant(), id(700001));
    assert_eq!(
        audit.checked_at(),
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(5)
    );
    assert_eq!(audit.commit().changes().revision(), &revision);
    assert_eq!(audit.commit().revision(), &revision.successor());
    assert_eq!(
        audit
            .commit()
            .changes()
            .difference()
            .notches()
            .iter()
            .map(|n| n.id())
            .collect::<Vec<_>>(),
        vec![id(720000), id(720001)]
    );
    let current = revision.successor();
    let CommandResult::PieceList(actual) = bus
        .execute(&query(design, current.clone(), QueryPayload::GetPieceList))
        .expect("actual changed original owner");
    assert_eq!(
        actual
            .pieces()
            .first()
            .expect("actual target")
            .definition()
            .material,
        MaterialAssignment::Unresolved("actual committed reason".into())
    );
    let measure = CommandRequest::Mutation(CommandMutation::new(
        CommandMutationContext::new(
            design,
            actor(),
            current.clone(),
            CommandIdempotencyKey::from_entity_id(id(760010)),
            None,
        ),
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), original)),
    ));
    let CommandPreview::Measurement(report) = bus
        .preview(&measure)
        .expect("actual published canonical state")
    else {
        unreachable!("Measurement report")
    };
    assert_eq!(
        report
            .changes()
            .first()
            .expect("restoration proposal")
            .before(),
        &replacement
    );
    let next = group(
        760011,
        vec![mutation(
            design,
            current,
            760011,
            760012,
            MutationPayload::AddNotch(addition),
        )],
    );
    assert_eq!(
        bus.preview_group(&next)
            .expect("actual remaining reserve")
            .difference()
            .notches()
            .first()
            .expect("next staged ID")
            .id(),
        id(720002)
    );
    let replay = bus
        .commit_group(&request)
        .expect("exact retry after revision advance");
    assert!(replay.is_replay());
    assert_eq!(replay.entry(), committed.entry());
}

#[test]
fn later_candidate_failure_leaves_model_reserve_and_audit_capacity_unchanged() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let other = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .get(1)
        .expect("second Piece");
    let invalid =
        sc_core::command::AddNotch::new(other.id(), addition.anchor(), addition.representation());
    let expected = sc_core::ontology::Notch::new(
        invalid.definition_with_id(id(720001)),
        other,
        owner.inputs().structure().topology(),
    )
    .expect_err("native cause");
    let first = mutation(
        design,
        revision.clone(),
        760020,
        760021,
        MutationPayload::AddNotch(addition),
    );
    let bad = group(
        760020,
        vec![
            first.clone(),
            mutation(
                design,
                revision.clone(),
                760020,
                760022,
                MutationPayload::AddNotch(invalid),
            ),
        ],
    );
    let corrected = group(
        760020,
        vec![
            first,
            mutation(
                design,
                revision.clone(),
                760020,
                760022,
                MutationPayload::AddNotch(addition),
            ),
        ],
    );
    let mut bus = CommandBus::with_edit_capacity(
        owner,
        Host::new(design),
        notch_preview::reserve(),
        capacity(1),
    )
    .expect("one accepted group fits");
    let error = bus
        .commit_group(&bad)
        .expect_err("later failure refuses whole group");
    let preparation = error
        .source()
        .expect("original group error")
        .downcast_ref::<GroupError<sc_measure::DesignSourcesError>>()
        .expect("typed original");
    assert!(matches!(
        preparation,
        GroupError::Mutation {
            index: 1,
            command: CommandId::AddNotch,
            ..
        }
    ));
    let native = preparation
        .source()
        .expect("original preview error")
        .source()
        .expect("original native cause");
    assert_eq!(native.downcast_ref::<NotchError>(), Some(&expected));
    let CommandResult::PieceList(actual) = bus
        .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
        .expect("unchanged actual revision");
    assert_eq!(actual.revision(), &revision);
    let result = bus
        .commit_group(&corrected)
        .expect("failed request left no record/reserve effect");
    assert!(!result.is_replay());
    assert_eq!(
        result
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
}

#[test]
fn exact_replay_after_later_commits_returns_first_result_without_rewinding_or_consuming_capacity() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let first = group(
        760030,
        vec![mutation(
            design,
            revision.clone(),
            760030,
            760031,
            material(addition.piece(), "first"),
        )],
    );
    let second = group(
        760032,
        vec![mutation(
            design,
            revision.successor(),
            760032,
            760033,
            material(addition.piece(), "second"),
        )],
    );
    let third = group(
        760034,
        vec![mutation(
            design,
            revision.successor().successor(),
            760034,
            760035,
            material(addition.piece(), "third"),
        )],
    );
    let host = Host::new(design);
    let revoked = Rc::clone(&host.revoked);
    let mut bus = CommandBus::with_edit_capacity(owner, host, vec![], capacity(2))
        .expect("two immutable authoring records");
    let original = bus.commit_group(&first).expect("first edit");
    for _ in 0..3 {
        let replay = bus.commit_group(&first).expect("repeat uses no capacity");
        assert!(replay.is_replay());
        assert_eq!(replay.entry(), original.entry());
    }
    bus.commit_group(&second)
        .expect("second capacity slot still free");
    let replay = bus
        .commit_group(&first)
        .expect("original replay at full capacity and later revision");
    assert!(replay.is_replay());
    assert_eq!(replay.entry(), original.entry());
    assert!(matches!(
        bus.commit_group(&third),
        Err(CommitError::JournalFull { limit: 2, .. })
    ));
    let CommandResult::PieceList(actual) = bus
        .execute(&query(
            design,
            revision.successor().successor(),
            QueryPayload::GetPieceList,
        ))
        .expect("latest model untouched by replay/full refusal");
    assert_eq!(
        actual
            .pieces()
            .first()
            .expect("actual target")
            .definition()
            .material,
        MaterialAssignment::Unresolved("second".into())
    );
    revoked.set(true);
    assert!(
        matches!(bus.commit_group(&first),Err(CommitError::Preparation(issue)) if matches!(*issue,GroupError::Authorization(_)))
    );
}

#[test]
fn group_and_actor_key_conflicts_never_reapply_or_accept_part_of_another_group() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let first = group(
        760040,
        vec![mutation(
            design,
            revision.clone(),
            760040,
            760041,
            material(addition.piece(), "accepted"),
        )],
    );
    let changed = group(
        760040,
        vec![mutation(
            design,
            revision.clone(),
            760040,
            760041,
            material(addition.piece(), "conflict"),
        )],
    );
    let overlap = group(
        760042,
        vec![
            mutation(
                design,
                revision.successor(),
                760042,
                760043,
                material(addition.piece(), "must not apply"),
            ),
            mutation(
                design,
                revision.successor(),
                760042,
                760041,
                material(addition.piece(), "accepted"),
            ),
        ],
    );
    let stale = group(
        760044,
        vec![mutation(
            design,
            revision.clone(),
            760044,
            760045,
            material(addition.piece(), "stale"),
        )],
    );
    let mut bus = CommandBus::with_edit_capacity(owner, Host::new(design), vec![], capacity(3))
        .expect("actual owner");
    bus.commit_group(&first).expect("accepted original");
    assert!(
        matches!(bus.commit_group(&changed),Err(CommitError::GroupConflict {group}) if group==group_id(760040))
    );
    assert!(
        matches!(bus.commit_group(&overlap),Err(CommitError::KeyConflict {group,index:1,key,previous}) if group==group_id(760042)&&previous==group_id(760040)&&key==CommandIdempotencyKey::from_entity_id(id(760041)))
    );
    let CommitError::Precondition {
        issue,
        history_from,
        changes,
    } = bus
        .commit_group(&stale)
        .expect_err("actual revision witness")
    else {
        unreachable!("revision precondition")
    };
    assert!(
        matches!(&*issue,GroupError::RevisionMismatch {requested,current,..} if requested==&revision && current==&revision.successor())
    );
    assert_eq!(history_from, revision);
    assert_eq!(changes.len(), 1);
    let moved = changes.first().expect("original authoring group");
    assert_eq!(moved.group, group_id(760040));
    assert_eq!(moved.revision, revision.successor());
    assert_eq!(moved.commands, vec![CommandId::AssignMaterial]);
    let CommandResult::PieceList(actual) = bus
        .execute(&query(
            design,
            revision.successor(),
            QueryPayload::GetPieceList,
        ))
        .expect("only original whole group published");
    assert_eq!(
        actual
            .pieces()
            .first()
            .expect("actual target")
            .definition()
            .material,
        MaterialAssignment::Unresolved("accepted".into())
    );
}

#[test]
fn commit_requires_explicit_capacity_and_commit_permission_independent_of_inspect() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let request = group(
        760050,
        vec![mutation(
            design,
            revision,
            760050,
            760051,
            material(addition.piece(), "actual intent"),
        )],
    );
    let mut unconfigured = CommandBus::new(owner.clone(), Host::new(design));
    assert!(!unconfigured.is_commit_available(CommandId::AssignMaterial));
    assert!(matches!(
        unconfigured.commit_group(&request),
        Err(CommitError::EditsNotConfigured { .. })
    ));
    let mut inspect = CommandBus::with_edit_capacity(
        owner.clone(),
        Host::with_permissions(design, vec![Permission::Inspect]),
        vec![],
        capacity(1),
    )
    .expect("configured");
    inspect.preview_group(&request).expect("inspect preview");
    assert!(
        matches!(inspect.commit_group(&request),Err(CommitError::Preparation(issue)) if matches!(*issue,GroupError::Authorization(_)))
    );
    let mut commit = CommandBus::with_edit_capacity(
        owner,
        Host::with_permissions(design, vec![Permission::Commit]),
        vec![],
        capacity(1),
    )
    .expect("configured");
    assert!(matches!(
        commit.preview_group(&request),
        Err(GroupError::Authorization(_))
    ));
    commit
        .commit_group(&request)
        .expect("explicit Commit is sufficient for actual edit");
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
fn effect_boundary_revocation_refuses_after_preparation_without_any_publication_or_audit_entry() {
    let owner = notch_preview::owner();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(&owner);
    let originals = owner.inputs().structure().pieces().pieces().to_vec();
    let address = owner.inputs().structure().pieces().pieces().as_ptr();
    let request = group(
        760060,
        vec![mutation(
            design,
            revision.clone(),
            760060,
            760061,
            MutationPayload::AddNotch(addition),
        )],
    );
    let host = Host::new(design);
    let revoked = Rc::clone(&host.revoked);
    let reads = Rc::clone(&host.reads);
    let revoke_at = Rc::new(Cell::new(Some(2)));
    let mut bus = CommandBus::with_edit_capacity(
        owner,
        BoundaryHost {
            host,
            revoke_at: Rc::clone(&revoke_at),
        },
        notch_preview::reserve(),
        capacity(1),
    )
    .expect("explicit capacity");
    assert!(
        matches!(bus.commit_group(&request),Err(CommitError::Preparation(issue)) if matches!(*issue,GroupError::Authorization(_)))
    );
    assert_eq!(reads.get(), 2);
    revoke_at.set(None);
    revoked.set(false);
    let CommandResult::PieceList(actual) = bus
        .execute(&query(design, revision, QueryPayload::GetPieceList))
        .expect("same original after final refusal");
    assert_eq!(actual.pieces(), originals);
    assert_eq!(actual.pieces().as_ptr(), address);
    let committed = bus
        .commit_group(&request)
        .expect("same group/key still unused and capacity free");
    assert!(!committed.is_replay());
    assert_eq!(
        committed
            .entry()
            .commit()
            .changes()
            .difference()
            .notches()
            .first()
            .expect("original identity")
            .id(),
        id(720000)
    );
}
