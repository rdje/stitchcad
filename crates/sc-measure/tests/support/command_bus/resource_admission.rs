//! Real model/resource port telemetry: authority/binding precede private walks, validation and copies.
use super::group_commit::{capacity, group, material, mutation};
use super::*;
use sc_core::{
    command::{
        CommandControlError as Control, CommandHistoryDirection, CommandHistoryRequest,
        CommandOperationLimits, CommandProgress, CommandProgressDecision, CommandProgressObserver,
    },
    design::{
        charge_design_garment_resources, DesignResourceDemand, DesignResourceLimits,
        DesignResourceMeter,
    },
};
use std::{cell::RefCell, error::Error as StdError, num::NonZeroU64};
pub(super) fn control<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a Control> {
    let mut current = Some(error);
    while let Some(error) = current {
        if let Some(control) = error.downcast_ref::<Control>() {
            return Some(control);
        }
        current = error.source();
    }
    None
}
pub(super) fn reset(counters: &[Rc<Cell<usize>>; 3]) {
    for counter in counters {
        counter.set(0);
    }
}
pub(super) fn counts(counters: &[Rc<Cell<usize>>; 3]) -> [usize; 3] {
    counters.each_ref().map(|counter| counter.get())
}
pub(super) fn demand(
    owner: &DesignGarment<resource_copies::CountedSources>,
) -> DesignResourceDemand {
    let mut meter = DesignResourceMeter::new(model_policy());
    charge_design_garment_resources(owner, &mut meter).expect("actual complete logical model");
    meter
        .finish()
        .expect("actual demand without semantic callbacks")
}
pub(super) fn without_model_limits<H, A>(mut bus: CommandBus<H, A>) -> CommandBus<H, A> {
    bus.install_operation_limits(CommandOperationLimits::new(
        capacity(10),
        capacity(10),
        capacity(1000),
        capacity(1000),
    ))
    .expect("explicit operation admission");
    bus.install_retained_limits(model_policy())
        .expect("explicit retained policy; only model limits deliberately absent");
    bus
}
#[derive(Default)]
struct Observer(RefCell<Vec<CommandProgress>>);
impl CommandProgressObserver for Observer {
    fn observe(&self, event: &CommandProgress) -> CommandProgressDecision {
        self.0.borrow_mut().push(*event);
        CommandProgressDecision::Continue
    }
}
#[test]
fn model_policy_has_no_default_installs_once_and_does_not_walk_or_block_piece_inspection() {
    let (owner, addition) = resource_copies::owner(Rc::new(Cell::new(0)));
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let request = CommandRequest::Mutation(mutation(
        design,
        revision.clone(),
        803000,
        803001,
        MutationPayload::AddNotch(addition),
    ));
    let mut bus = without_model_limits(
        CommandBus::with_edit_capacity(
            owner,
            Host::new(design),
            notch_preview::reserve(),
            capacity(1),
        )
        .expect("actual finite unused IDs"),
    );
    reset(&counters);
    assert!(!bus.is_preview_available(CommandId::AddNotch));
    assert!(!bus.is_commit_available(CommandId::AddNotch));
    let error = bus.preview(&request).expect_err("no implicit model policy");
    assert_eq!(control(&error), Some(&Control::ModelLimitsNotInstalled));
    bus.execute(&query(design, revision, QueryPayload::GetPieceList))
        .expect("authorized cheap read");
    let small = DesignResourceLimits::new(capacity(1), capacity(1), NonZeroU64::MIN);
    bus.install_model_limits(small)
        .expect("one explicit immutable model policy");
    assert!(bus.is_preview_available(CommandId::AddNotch));
    assert!(bus.is_commit_available(CommandId::AddNotch));
    assert_eq!(
        bus.install_model_limits(model_policy()),
        Err(Control::ModelLimitsAlreadyInstalled)
    );
    assert_eq!(
        counts(&counters),
        [0, 0, 0],
        "setup/read invoke no private resource/semantic callbacks"
    );
    let error = bus
        .preview(&request)
        .expect_err("original tiny policy cannot be widened");
    assert!(matches!(control(&error), Some(Control::Model(_))));
    assert_eq!(
        counts(&counters),
        [0, 1, 0],
        "retained admission precedes the first per-model quota; no validation/copy"
    );
}
#[test]
fn authority_and_exact_snapshot_binding_precede_private_walks_on_single_group_and_commit() {
    for case in 0..3 {
        let (owner, addition) = resource_copies::owner(Rc::new(Cell::new(0)));
        let counters = owner.inputs().sources().host().telemetry();
        let actual_design = owner.inputs().id();
        let actual_revision = owner.inputs().revision().clone();
        let design = if case == 1 { id(803099) } else { actual_design };
        let revision = if case == 2 {
            actual_revision.successor()
        } else {
            actual_revision.clone()
        };
        let host = Host::new(design);
        if case == 0 {
            host.revoked.set(true);
        }
        let mut bus = controlled(
            CommandBus::with_edit_capacity(owner, host, notch_preview::reserve(), capacity(1))
                .expect("actual finite bus"),
        );
        let member = mutation(
            design,
            revision,
            803010,
            803011,
            MutationPayload::AddNotch(addition),
        );
        let request = group(803010, vec![member.clone()]);
        let observer = Observer::default();
        reset(&counters);
        let single_error = bus
            .preview_with_progress(&CommandRequest::Mutation(member), &observer)
            .expect_err("authority or exact binding refusal");
        let group_error = bus
            .preview_group_with_progress(&request, &observer)
            .expect_err("group authority or exact binding refusal");
        let commit_error = bus
            .commit_group_with_progress(&request, &observer)
            .expect_err("commit authority or exact binding refusal");
        assert_eq!(control(&single_error), None);
        assert_eq!(control(&group_error), None);
        assert_eq!(control(&commit_error), None);
        assert_eq!(
            counts(&counters),
            [0, 0, 0],
            "no private quota witness/census/validation/copy"
        );
        if case == 0 {
            assert!(
                observer.0.borrow().is_empty(),
                "unauthorized has no progress"
            );
        }
    }
}
#[test]
fn exact_current_model_refusal_precedes_validation_copy_and_effect_on_all_preparation_entries() {
    let (owner, addition) = resource_copies::owner(Rc::new(Cell::new(0)));
    let counters = owner.inputs().sources().host().telemetry();
    let actual = demand(&owner);
    let limits = DesignResourceLimits::new(
        capacity(actual.collection_entries()),
        capacity(actual.text_bytes() - 1),
        NonZeroU64::MAX,
    );
    let mut oracle = DesignResourceMeter::new(limits);
    let expected = charge_design_garment_resources(&owner, &mut oracle)
        .expect_err("one byte below actual model");
    assert_eq!(oracle.finish(), Err(expected));
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let address = owner.inputs().structure().pieces().pieces().as_ptr();
    let member = mutation(
        design,
        revision.clone(),
        803020,
        803021,
        MutationPayload::AddNotch(addition),
    );
    let request = group(803020, vec![member.clone()]);
    let mut bus = without_model_limits(
        CommandBus::with_edit_capacity(
            owner,
            Host::new(design),
            notch_preview::reserve(),
            capacity(1),
        )
        .expect("actual reserved model bus"),
    );
    bus.install_model_limits(limits)
        .expect("exact immutable original policy");
    for path in 0..3 {
        reset(&counters);
        let error: Box<dyn StdError> = match path {
            0 => Box::new(
                bus.preview(&CommandRequest::Mutation(member.clone()))
                    .expect_err("bounded single"),
            ),
            1 => Box::new(bus.preview_group(&request).expect_err("bounded group")),
            _ => Box::new(bus.commit_group(&request).expect_err("bounded commit")),
        };
        assert_eq!(control(error.as_ref()), Some(&Control::Model(expected)));
        assert_eq!(
            counts(&counters),
            [0, 2, 0],
            "retained plus per-model H walks, no semantic callback or clone"
        );
        let CommandResult::PieceList(result) = bus
            .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
            .expect("no effect changed original revision");
        assert_eq!(result.pieces().as_ptr(), address);
    }
}
#[test]
fn history_admits_both_actual_snapshots_and_exact_replays_skip_model_walks() {
    let (owner, addition) = resource_copies::owner(Rc::new(Cell::new(0)));
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let request = group(
        803030,
        vec![mutation(
            design,
            revision.clone(),
            803030,
            803031,
            material(addition.piece(), "bounded"),
        )],
    );
    let host = Host::new(design);
    let revoked = Rc::clone(&host.revoked);
    let mut bus = controlled(
        CommandBus::with_edit_history(owner, host, vec![], capacity(1), capacity(1), capacity(2))
            .expect("actual reversible finite owner"),
    );
    bus.commit_group(&request)
        .expect("actual accepted metadata act");
    let undo = CommandHistoryRequest::new(
        CommandMutationContext::new(
            design,
            actor(),
            revision.successor(),
            CommandIdempotencyKey::from_entity_id(id(803032)),
            Some(request.id()),
        ),
        request.clone(),
        CommandHistoryDirection::Undo,
    )
    .expect("exact actual history action");
    revoked.set(true);
    reset(&counters);
    bus.apply_history(&undo)
        .expect_err("unauthorized history cannot inspect private model");
    assert_eq!(counts(&counters), [0, 0, 0]);
    revoked.set(false);
    reset(&counters);
    let stale = CommandHistoryRequest::new(
        CommandMutationContext::new(
            design,
            actor(),
            revision.successor().successor(),
            CommandIdempotencyKey::from_entity_id(id(803033)),
            Some(request.id()),
        ),
        request.clone(),
        CommandHistoryDirection::Undo,
    )
    .expect("explicit stale history snapshot");
    let error = bus
        .apply_history(&stale)
        .expect_err("bind history snapshot before resource walk");
    assert_eq!(control(&error), None);
    assert_eq!(counts(&counters), [0, 0, 0]);
    let accepted = bus
        .apply_history(&undo)
        .expect("actual source/current/alternative checks");
    let observed = counts(&counters);
    assert_eq!(observed[0], 0, "undo moves existing owners without cloning");
    assert_eq!(
        observed[1], 6,
        "current retained2 + individual current/alternative2 + prospective retained2"
    );
    assert!(
        observed[2] > 0,
        "existing actual current validation remains in force"
    );
    reset(&counters);
    let replay = bus
        .apply_history(&undo)
        .expect("exact accepted history replay");
    assert!(replay.is_replay());
    assert_eq!(replay.entry(), accepted.entry());
    assert_eq!(counts(&counters), [0, 0, 0]);
    let replay = bus
        .commit_group(&request)
        .expect("exact original authoring replay after undo");
    assert!(replay.is_replay());
    assert_eq!(counts(&counters), [0, 0, 0]);
}
