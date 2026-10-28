//! Executable queries use the actual aggregate/source schemas and a separately controlled authority.
#![allow(clippy::expect_used)]
use super::{garment_current, garment_ownership, id};
use sc_core::{
    command::{
        CloseDart, CommandActorClaim as Actor, CommandAuthority as Permission,
        CommandAuthoritySnapshot, CommandAuthoritySource, CommandAuthorizationError as Denial,
        CommandBus, CommandBusError as Error, CommandGrant, CommandGrantStatus, CommandId,
        CommandIdempotencyKey, CommandMutation, CommandMutationContext, CommandQuery,
        CommandRequest, CommandResult, CommandScope, CommandSnapshotContext, DesignRevision,
        MutationPayload, QueryPayload,
    },
    design::DesignGarment,
    ontology::EntityId,
};
use sc_measure::DesignSources;
use std::{cell::Cell, rc::Rc, time::SystemTime};

struct Host {
    actor: Rc<Cell<Option<Actor>>>,
    revoked: Rc<Cell<bool>>,
    reads: Rc<Cell<usize>>,
    active: Vec<CommandGrant>,
    inactive: Vec<CommandGrant>,
}
impl Host {
    fn new(design: EntityId) -> Self {
        Self::with_permissions(design, vec![Permission::Inspect, Permission::Commit])
    }
    fn with_permissions(design: EntityId, permissions: Vec<Permission>) -> Self {
        let grant = |status| {
            CommandGrant::new(
                id(700001),
                actor(),
                permissions.clone(),
                CommandScope::new(
                    design,
                    CommandId::ALL.to_vec(),
                    SystemTime::UNIX_EPOCH,
                    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(10),
                )
                .expect("actual exact Design/command/time scope"),
                status,
            )
            .expect("human-issued agent grant without approval")
        };
        Self {
            actor: Rc::new(Cell::new(Some(actor()))),
            revoked: Rc::new(Cell::new(false)),
            reads: Rc::new(Cell::new(0)),
            active: vec![grant(CommandGrantStatus::Active)],
            inactive: vec![grant(CommandGrantStatus::Revoked)],
        }
    }
}

#[path = "command_bus/control.rs"]
mod control;
#[path = "command_bus/group_commit.rs"]
mod group_commit;
#[path = "command_bus/group_preview.rs"]
mod group_preview;
#[path = "command_bus/history.rs"]
mod history;
#[path = "command_bus/material_preview.rs"]
mod material_preview;
#[path = "command_bus/measurement_preview.rs"]
mod measurement_preview;
#[path = "command_bus/notch_preview.rs"]
mod notch_preview;
#[path = "command_bus/resource_admission.rs"]
mod resource_admission;
#[path = "command_bus/resource_copies.rs"]
mod resource_copies;
#[path = "command_bus/resource_material.rs"]
mod resource_material;
#[path = "command_bus/resource_measurement.rs"]
mod resource_measurement;
#[path = "command_bus/resource_notch.rs"]
mod resource_notch;
#[path = "command_bus/resource_records.rs"]
mod resource_records;
#[path = "command_bus/resource_reports.rs"]
mod resource_reports;
#[path = "command_bus/resource_requests.rs"]
mod resource_requests;
#[path = "command_bus/resource_revision.rs"]
mod resource_revision;
#[path = "command_bus/retained_authoring.rs"]
mod retained_authoring;
#[path = "command_bus/retained_current.rs"]
mod retained_current;
#[path = "command_bus/retained_history.rs"]
mod retained_history;
#[path = "command_bus/retained_policy.rs"]
mod retained_policy;
#[path = "command_bus/source_clone_footprint.rs"]
mod source_clone_footprint;
impl CommandAuthoritySource for Host {
    fn snapshot(&self) -> CommandAuthoritySnapshot<'_> {
        self.reads.set(self.reads.get() + 1);
        CommandAuthoritySnapshot {
            authenticated_actor: self.actor.get(),
            observed_at: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(5),
            grants: if self.revoked.get() {
                &self.inactive
            } else {
                &self.active
            },
        }
    }
}
fn actor() -> Actor {
    Actor::Agent {
        identity: id(700002),
        principal: id(700003),
    }
}
fn garment() -> DesignGarment<DesignSources> {
    DesignGarment::new(garment_ownership::definition()).expect("actual canonical aggregate")
}
// Explicit permissive test-host policy preserves the prior semantic cases. Runtime control tests
// install their own small boundaries; production code has no default operation policy.
fn controlled<H, A>(mut bus: CommandBus<H, A>) -> CommandBus<H, A> {
    bus.install_operation_limits(sc_core::command::CommandOperationLimits::new(
        std::num::NonZeroUsize::MAX,
        std::num::NonZeroUsize::MAX,
        std::num::NonZeroUsize::MAX,
        std::num::NonZeroUsize::MAX,
    ))
    .expect("one explicit test-host policy installation");
    model_controlled(bus)
}
fn model_policy() -> sc_core::design::DesignResourceLimits {
    sc_core::design::DesignResourceLimits::new(
        std::num::NonZeroUsize::MAX,
        std::num::NonZeroUsize::MAX,
        std::num::NonZeroU64::MAX,
    )
}
fn model_controlled<H, A>(mut bus: CommandBus<H, A>) -> CommandBus<H, A> {
    bus.install_model_limits(model_policy())
        .expect("one explicit test-host model policy");
    bus.install_retained_limits(model_policy())
        .expect("one explicit test-host retained policy");
    bus
}
fn query(design: EntityId, revision: DesignRevision, payload: QueryPayload) -> CommandRequest {
    CommandRequest::Query(CommandQuery::new(
        CommandSnapshotContext::new(design, actor(), revision),
        payload,
    ))
}

#[test]
fn command_bus_reads_the_exact_original_piece_inventory_without_reconstruction_or_changes() {
    let owner = garment();
    let pieces = owner.inputs().structure().pieces().pieces();
    let address = pieces.as_ptr();
    let expected = pieces.to_vec();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let request = query(design, revision.clone(), QueryPayload::GetPieceList);
    let host = Host::new(design);
    let reads = Rc::clone(&host.reads);
    let bus = CommandBus::new(owner, host);
    for _ in 0..2 {
        let CommandResult::PieceList(result) =
            bus.execute(&request).expect("authorized exact read");
        assert_eq!(result.design(), design);
        assert_eq!(result.revision(), &revision);
        assert_eq!(result.pieces(), expected);
        assert_eq!(result.pieces().as_ptr(), address);
    }
    assert_eq!(
        reads.get(),
        2,
        "each read takes a fresh authoritative snapshot"
    );
    assert_eq!(format!("{bus:?}"), "CommandBus { .. }");
}

#[test]
fn command_bus_authenticates_before_owned_state_checks_and_preserves_original_cause() {
    let owner = garment();
    let design = owner.inputs().id();
    let host = Host::new(design);
    let session = Rc::clone(&host.actor);
    session.set(None);
    let reads = Rc::clone(&host.reads);
    let bus = CommandBus::new(owner, host);
    // Wrong Design, future revision and missing handler must not mask authentication failure.
    let bad = query(
        id(799999),
        DesignRevision::from_counter(99),
        QueryPayload::WalkSeamSpan { span: id(12) },
    );
    let error = bus.execute(&bad).expect_err("authenticate first");
    assert_eq!(error, Error::Authorization(Denial::Unauthenticated));
    assert!(std::error::Error::source(&error)
        .expect("exact original cause")
        .is::<Denial>());
    assert_eq!(reads.get(), 1);
    session.set(Some(Actor::Human(id(700002))));
    assert_eq!(
        bus.execute(&bad),
        Err(Error::Authorization(Denial::ActorMismatch {
            claimed: actor(),
            authenticated: Actor::Human(id(700002)),
        }))
    );
    assert_eq!(reads.get(), 2);
}

#[test]
fn command_bus_rechecks_revocation_and_never_treats_an_earlier_read_as_authority() {
    let owner = garment();
    let design = owner.inputs().id();
    let request = query(
        design,
        owner.inputs().revision().clone(),
        QueryPayload::GetPieceList,
    );
    let host = Host::new(design);
    let revoked = Rc::clone(&host.revoked);
    let reads = Rc::clone(&host.reads);
    let bus = CommandBus::new(owner, host);
    let CommandResult::PieceList(first) = bus.execute(&request).expect("initial live grant");
    let expected = first.pieces().to_vec();
    revoked.set(true);
    assert!(matches!(
        bus.execute(&request),
        Err(Error::Authorization(Denial::NoMatchingGrant { .. }))
    ));
    revoked.set(false);
    let CommandResult::PieceList(next) =
        bus.execute(&request).expect("new live source observation");
    assert_eq!(next, first);
    assert_eq!(next.pieces(), expected);
    assert_eq!(reads.get(), 3);
}

#[test]
fn command_bus_binds_actual_design_and_exact_revision_and_reports_unavailable_handlers() {
    let owner = garment();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let other = id(799999);
    let other_bus = CommandBus::new(owner.clone(), Host::new(other));
    assert_eq!(
        other_bus.execute(&query(other, revision.clone(), QueryPayload::GetPieceList)),
        Err(Error::DesignMismatch {
            command: CommandId::GetPieceList,
            requested: other,
            current: design
        })
    );
    let bus = CommandBus::new(owner, Host::new(design));
    let future = DesignRevision::from_counter(u128::MAX).successor();
    assert_eq!(
        bus.execute(&query(design, future.clone(), QueryPayload::GetPieceList)),
        Err(Error::RevisionMismatch {
            command: CommandId::GetPieceList,
            requested: future,
            current: revision.clone()
        })
    );
    assert_eq!(
        bus.execute(&query(
            design,
            revision.clone(),
            QueryPayload::WalkSeamSpan { span: id(12) }
        )),
        Err(Error::HandlerUnavailable {
            command: CommandId::WalkSeamSpan
        })
    );
    let mutation = CommandRequest::Mutation(CommandMutation::new(
        CommandMutationContext::new(
            design,
            actor(),
            revision.clone(),
            CommandIdempotencyKey::from_entity_id(id(700004)),
            None,
        ),
        MutationPayload::CloseDart(CloseDart::new(id(700005))),
    ));
    assert_eq!(
        bus.execute(&mutation),
        Err(Error::HandlerUnavailable {
            command: CommandId::CloseDart
        })
    );
    for command in CommandId::ALL {
        assert_eq!(
            bus.is_available(command),
            command == CommandId::GetPieceList
        );
    }
    let CommandResult::PieceList(result) = bus
        .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
        .expect("refusals preserve exact owned revision");
    assert_eq!(result.revision(), &revision);
}

#[test]
fn command_bus_metadata_inspection_preserves_repair_and_unresolved_physical_intent() {
    let mut definition = garment_ownership::definition();
    garment_current::lose_edge(&mut definition, 0);
    let owner = DesignGarment::new(definition).expect("actual held repair aggregate");
    assert!(owner.validate_current().is_err());
    assert!(
        owner.require_in_scope().is_err(),
        "modelled Lining remains deferred"
    );
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let pieces = owner.inputs().structure().pieces().pieces().to_vec();
    let bus = CommandBus::new(owner, Host::new(design));
    let CommandResult::PieceList(result) = bus
        .execute(&query(design, revision, QueryPayload::GetPieceList))
        .expect("authorized authored metadata remains inspectable during repair");
    assert_eq!(result.pieces(), pieces);
}
