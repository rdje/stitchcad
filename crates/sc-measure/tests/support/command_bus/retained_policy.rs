//! Immutable explicit retained-policy configuration, without claiming private-state admission.
use super::group_commit::{capacity, group, mutation};
use super::resource_admission::{control, counts, reset};
use super::*;
use sc_core::{
    command::{
        CommandControlError as Control, CommandHistoryDirection, CommandHistoryRequest,
        CommandOperationLimits,
    },
    design::DesignResourceLimits,
};
use std::{
    error::Error as StdError,
    num::{NonZeroU64, NonZeroUsize},
};
fn install_other_policies<H, A>(bus: &mut CommandBus<H, A>) {
    bus.install_operation_limits(CommandOperationLimits::new(
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
    ))
    .expect("explicit operation policy");
    bus.install_model_limits(model_policy())
        .expect("explicit model policy");
}
#[test]
fn retained_policy_is_explicit_once_only_and_never_walks_or_blocks_cheap_authorized_reads() {
    for constructor in 0..3 {
        let (owner, addition) = resource_copies::owner(Rc::new(Cell::new(0)));
        let counters = owner.inputs().sources().host().telemetry();
        let design = owner.inputs().id();
        let revision = owner.inputs().revision().clone();
        let host = Host::new(design);
        let reads = Rc::clone(&host.reads);
        let mut bus = match constructor {
            0 => CommandBus::new(owner, host),
            1 => CommandBus::with_identities(owner, host, notch_preview::reserve())
                .expect("actual reserve"),
            _ => CommandBus::with_edit_history(
                owner,
                host,
                notch_preview::reserve(),
                capacity(1),
                capacity(1),
                capacity(1),
            )
            .expect("actual edit/history capacities"),
        };
        let member = mutation(
            design,
            revision.clone(),
            811000,
            811001,
            MutationPayload::AddNotch(addition),
        );
        let request = CommandRequest::Mutation(member.clone());
        reset(&counters);
        reads.set(0);
        bus.execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
            .expect("policy-free actual read");
        assert_eq!(counts(&counters), [0, 0, 0]);
        install_other_policies(&mut bus);
        reads.set(0);
        assert!(!bus.is_preview_available(CommandId::AddNotch));
        assert!(!bus.is_commit_available(CommandId::AddNotch));
        let error = bus
            .preview(&request)
            .expect_err("no implicit retained policy");
        assert_eq!(control(&error), Some(&Control::RetainedLimitsNotInstalled));
        assert_eq!(counts(&counters), [0, 0, 0]);
        assert_eq!(reads.get(), 0);
        let small = DesignResourceLimits::new(capacity(1), capacity(1), NonZeroU64::MIN);
        bus.install_retained_limits(small)
            .expect("one explicit immutable retained policy");
        assert!(bus.is_preview_available(CommandId::AddNotch));
        assert_eq!(
            bus.is_commit_available(CommandId::AddNotch),
            constructor == 2
        );
        assert_eq!(
            bus.install_retained_limits(model_policy()),
            Err(Control::RetainedLimitsAlreadyInstalled)
        );
        assert_eq!(
            bus.install_retained_limits(small),
            Err(Control::RetainedLimitsAlreadyInstalled)
        );
        assert_eq!(counts(&counters), [0, 0, 0]);
        assert_eq!(reads.get(), 0);
        bus.execute(&query(design, revision, QueryPayload::GetPieceList))
            .expect("cheap read after tiny retained configuration");
        assert_eq!(counts(&counters), [0, 0, 0]);
    }
}
#[test]
fn every_costly_entry_point_requires_retained_configuration_before_authority_or_private_work() {
    let (owner, addition) = resource_copies::owner(Rc::new(Cell::new(0)));
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let host = Host::new(design);
    let reads = Rc::clone(&host.reads);
    let mut bus = CommandBus::with_edit_history(
        owner,
        host,
        notch_preview::reserve(),
        capacity(1),
        capacity(1),
        capacity(1),
    )
    .expect("actual owner");
    install_other_policies(&mut bus);
    let member = mutation(
        design,
        revision.clone(),
        811010,
        811011,
        MutationPayload::AddNotch(addition),
    );
    let group = group(811010, vec![member.clone()]);
    let history = CommandHistoryRequest::new(
        CommandMutationContext::new(
            design,
            actor(),
            revision,
            CommandIdempotencyKey::from_entity_id(id(811012)),
            Some(group.id()),
        ),
        group.clone(),
        CommandHistoryDirection::Undo,
    )
    .expect("original typed history intent");
    reset(&counters);
    reads.set(0);
    let errors: Vec<Box<dyn StdError>> = vec![
        Box::new(
            bus.preview(&CommandRequest::Mutation(member))
                .expect_err("single setup"),
        ),
        Box::new(bus.preview_group(&group).expect_err("group setup")),
        Box::new(bus.commit_group(&group).expect_err("commit setup")),
        Box::new(bus.apply_history(&history).expect_err("history setup")),
    ];
    for error in errors {
        assert_eq!(
            control(error.as_ref()),
            Some(&Control::RetainedLimitsNotInstalled)
        );
    }
    assert_eq!(counts(&counters), [0, 0, 0]);
    assert_eq!(reads.get(), 0);
}
