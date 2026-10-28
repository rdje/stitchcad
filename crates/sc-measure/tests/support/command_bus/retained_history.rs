//! Real human-operated history preflight: exact moved owners and action copies, no model clone.
use super::group_commit::{capacity, group, material, mutation};
use super::resource_admission::{control, counts, demand, reset};
use super::retained_current::{policy, request, with_assignment, Model, Observed};
use super::*;
use sc_core::{
    command::{
        charge_command_history_audit_entry_resources, CommandControlError as Control,
        CommandHistoryDirection as Direction, CommandHistoryRequest, CommandMutationGroup,
        CommandOperationLimits, CommandProgress, CommandProgressDecision as Decision,
        CommandProgressObserver, CommandProgressPhase as Phase,
    },
    design::{DesignResourceLimitError, DesignResourceLimits, DesignResourceMeter as Meter},
};
use std::num::NonZeroUsize;
struct Owner {
    bus: CommandBus<resource_copies::CountedSources, Host>,
    actor: Rc<Cell<Option<Actor>>>,
    revoked: Rc<Cell<bool>>,
    reads: Rc<Cell<usize>>,
}
impl Owner {
    fn human(&self) {
        self.actor.set(Some(super::history::human()));
    }
}
fn configured(
    initial: Model,
    reserve: Vec<EntityId>,
    limits: DesignResourceLimits,
    actions: usize,
) -> Owner {
    let host = super::history::operator_host(initial.inputs().id(), CommandId::ALL.to_vec());
    let actor = Rc::clone(&host.actor);
    let revoked = Rc::clone(&host.revoked);
    let reads = Rc::clone(&host.reads);
    let mut bus = CommandBus::with_edit_history(
        initial,
        host,
        reserve,
        capacity(6),
        capacity(2),
        capacity(actions),
    )
    .expect("actual finite owner");
    bus.install_operation_limits(CommandOperationLimits::new(
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
    ))
    .expect("input limits");
    bus.install_model_limits(model_policy())
        .expect("model limits");
    bus.install_retained_limits(limits)
        .expect("retained limits");
    Owner {
        bus,
        actor,
        revoked,
        reads,
    }
}
fn operation(
    original: &CommandMutationGroup,
    revision: &DesignRevision,
    key: u128,
    direction: Direction,
) -> CommandHistoryRequest {
    CommandHistoryRequest::new(
        CommandMutationContext::new(
            original.context().design(),
            super::history::human(),
            revision.clone(),
            CommandIdempotencyKey::from_entity_id(id(key)),
            Some(original.id()),
        ),
        original.clone(),
        direction,
    )
    .expect("actual original agent/current human intent")
}
fn inspect(design: EntityId, revision: DesignRevision) -> CommandRequest {
    CommandRequest::Query(CommandQuery::new(
        CommandSnapshotContext::new(design, super::history::human(), revision),
        QueryPayload::GetPieceList,
    ))
}
#[test]
fn exact_undo_action_counts_original_alias_payloads_but_only_effective_commands_before_any_copy() {
    let original = super::resource_measurement::fixture();
    let counters = original.inputs().sources().host().telemetry();
    let design = original.inputs().id();
    let piece = original
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("Piece")
        .id();
    let after = with_assignment(&original, 1, piece, " é ");
    let member = mutation(
        design,
        DesignRevision::initial(),
        815000,
        815001,
        material(piece, " é "),
    );
    let author = group(815000, vec![member.clone(), member]);
    let reserve = vec![id(815900), id(815901)];
    let undo = operation(
        &author,
        &DesignRevision::from_counter(1),
        815010,
        Direction::Undo,
    );
    let mut reference = configured(original.clone(), reserve.clone(), model_policy(), 1);
    let accepted = reference
        .bus
        .commit_group(&author)
        .expect("real agent publication");
    reference.human();
    let undone = reference.bus.apply_history(&undo).expect("real human undo");
    assert_eq!(undone.entry().operator(), super::history::human());
    assert_eq!(undone.entry().original_actor(), actor());
    assert_eq!(undone.entry().commands(), &[CommandId::AssignMaterial]);
    let oracle = Observed {
        initial: &original,
        current: &original,
        reserve: &reserve,
        accepted: &[accepted.entry()],
        undo: &[],
        redo: &[&after],
        actions: &[undone.entry()],
    };
    let expected = oracle.demand();
    let base = demand(&original);
    assert_eq!(
        (
            expected.collection_entries(),
            expected.text_bytes(),
            expected.counter_bits()
        ),
        (
            2 * base.collection_entries() + 19,
            2 * base.text_bytes() + 84,
            10
        )
    );
    for limits in [
        policy(expected.collection_entries(), expected.text_bytes(), 10),
        policy(expected.collection_entries() - 1, expected.text_bytes(), 10),
        policy(expected.collection_entries(), expected.text_bytes() - 1, 10),
        policy(expected.collection_entries(), expected.text_bytes(), 9),
    ] {
        let mut actual = configured(original.clone(), reserve.clone(), limits, 1);
        actual.bus.commit_group(&author).expect("first fits");
        actual.human();
        let address = match actual
            .bus
            .execute(&inspect(design, DesignRevision::from_counter(1)))
            .expect("original snapshot")
        {
            CommandResult::PieceList(list) => list.pieces().as_ptr(),
        };
        let mut meter = Meter::new(limits);
        let projected = oracle.charge(&mut meter);
        reset(&counters);
        actual.reads.set(0);
        let result = actual.bus.apply_history(&undo);
        match projected {
            Ok(()) => {
                assert_eq!(result.expect("exact action fits").entry(), undone.entry());
                assert_eq!(counts(&counters)[1], 6);
                assert_eq!(actual.reads.get(), 2);
                reset(&counters);
                let replay = actual
                    .bus
                    .apply_history(&undo)
                    .expect("exact replay at action-count limit");
                assert!(replay.is_replay());
                assert_eq!(replay.entry(), undone.entry());
                assert_eq!(counts(&counters), [0, 0, 0]);
            }
            Err(expected) => {
                let error = result.expect_err("one-short history refusal");
                assert_eq!(control(&error), Some(&Control::Retained(expected)));
                assert_eq!(
                    actual.reads.get(),
                    1,
                    "no final authority before resource admission"
                );
                assert_eq!(
                    std::error::Error::source(control(&error).expect("retained cause"))
                        .and_then(|e| e.downcast_ref::<DesignResourceLimitError>()),
                    Some(&expected)
                );
                let error = actual
                    .bus
                    .apply_history(&undo)
                    .expect_err("same original refusal, no key/action/counter consumed");
                assert_eq!(control(&error), Some(&Control::Retained(expected)));
                let CommandResult::PieceList(current) = actual
                    .bus
                    .execute(&inspect(design, DesignRevision::from_counter(1)))
                    .expect("unchanged snapshot");
                assert_eq!(current.pieces().as_ptr(), address);
                assert_eq!(
                    current.pieces(),
                    after.inputs().structure().pieces().pieces()
                );
            }
        }
        assert_eq!(
            counts(&counters)[0],
            0,
            "history moves actual owners; no H clone"
        );
    }
}
#[test]
fn exact_redo_keeps_other_alternatives_in_stored_order_and_does_not_double_count_the_target() {
    let original = super::resource_measurement::fixture();
    let counters = original.inputs().sources().host().telemetry();
    let design = original.inputs().id();
    let piece = original
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("Piece")
        .id();
    let large = " β ".repeat(16);
    assert_eq!(large.len(), 64);
    let one = with_assignment(&original, 1, piece, " é ");
    let two = with_assignment(&one, 2, piece, &large);
    let a = request(&original, 815100, piece, " é ");
    let b = request(&one, 815110, piece, &large);
    let reserve = vec![id(815910), id(815911)];
    let undo_b = operation(
        &b,
        &DesignRevision::from_counter(2),
        815120,
        Direction::Undo,
    );
    let undo_a = operation(
        &a,
        &DesignRevision::from_counter(1),
        815121,
        Direction::Undo,
    );
    let redo_a = operation(&a, &DesignRevision::initial(), 815122, Direction::Redo);
    let mut reference = configured(original.clone(), reserve.clone(), model_policy(), 8);
    let first = reference.bus.commit_group(&a).expect("first");
    let second = reference.bus.commit_group(&b).expect("second");
    reference.human();
    let h1 = reference.bus.apply_history(&undo_b).expect("undo second");
    let h2 = reference.bus.apply_history(&undo_a).expect("undo first");
    let h3 = reference.bus.apply_history(&redo_a).expect("redo first");
    let oracle = Observed {
        initial: &original,
        current: &one,
        reserve: &reserve,
        accepted: &[first.entry(), second.entry()],
        undo: &[&original],
        redo: &[&two],
        actions: &[h1.entry(), h2.entry(), h3.entry()],
    };
    let expected = oracle.demand();
    let base = demand(&original);
    assert_eq!(
        (
            expected.collection_entries(),
            expected.text_bytes(),
            expected.counter_bits()
        ),
        (
            3 * base.collection_entries() + 28,
            3 * base.text_bytes() + 352,
            36
        )
    );
    for limits in [
        policy(expected.collection_entries(), expected.text_bytes(), 36),
        policy(expected.collection_entries(), expected.text_bytes() - 1, 36),
        policy(expected.collection_entries(), expected.text_bytes(), 35),
    ] {
        let mut actual = configured(original.clone(), reserve.clone(), limits, 8);
        actual.bus.commit_group(&a).expect("first");
        actual.bus.commit_group(&b).expect("second");
        actual.human();
        actual.bus.apply_history(&undo_b).expect("undo second");
        actual.bus.apply_history(&undo_a).expect("undo first");
        let mut meter = Meter::new(limits);
        let projected = oracle.charge(&mut meter);
        reset(&counters);
        let result = actual.bus.apply_history(&redo_a);
        match projected {
            Ok(()) => {
                assert_eq!(
                    result.expect("exact reciprocal stacks and action").entry(),
                    h3.entry()
                );
                assert_eq!(
                    counts(&counters)[1],
                    8,
                    "current3 + individual2 + prospective3"
                );
            }
            Err(expected) => {
                assert_eq!(
                    control(&result.expect_err("one-short original redo refusal")),
                    Some(&Control::Retained(expected))
                );
                actual
                    .bus
                    .execute(&inspect(design, DesignRevision::initial()))
                    .expect("current original revision remains");
            }
        }
        assert_eq!(counts(&counters)[0], 0);
    }
}
#[test]
fn history_keeps_all_one_maximum_issued_counter_without_forecasting_another_revision() {
    let original = super::resource_revision::with_revision(
        &super::resource_measurement::fixture(),
        DesignRevision::from_counter(u128::MAX - 1),
    );
    let counters = original.inputs().sources().host().telemetry();
    let piece = original
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("Piece")
        .id();
    let after =
        super::resource_revision::with_revision(&original, DesignRevision::from_counter(u128::MAX));
    let author = request(&original, 815200, piece, "explicit selection pending");
    let undo = operation(
        &author,
        &DesignRevision::from_counter(u128::MAX),
        815210,
        Direction::Undo,
    );
    let mut reference = configured(original.clone(), vec![], model_policy(), 1);
    let accepted = reference
        .bus
        .commit_group(&author)
        .expect("actual all-one maximum issued");
    reference.human();
    let undone = reference
        .bus
        .apply_history(&undo)
        .expect("actual restore previous large revision");
    let oracle = Observed {
        initial: &original,
        current: &original,
        reserve: &[],
        accepted: &[accepted.entry()],
        undo: &[],
        redo: &[&after],
        actions: &[undone.entry()],
    };
    let expected = oracle.demand();
    let base = demand(&original);
    assert_eq!(
        (
            expected.collection_entries(),
            expected.text_bytes(),
            expected.counter_bits()
        ),
        (
            2 * base.collection_entries() + 9,
            2 * base.text_bytes() + 52,
            1797
        )
    );
    let mut meter = Meter::new(model_policy());
    charge_command_history_audit_entry_resources(undone.entry(), &mut meter)
        .expect("actual original record");
    assert_eq!(meter.finish().expect("action oracle").counter_bits(), 642);
    for bits in [1797, 1796] {
        let limits = policy(expected.collection_entries(), expected.text_bytes(), bits);
        let mut actual = configured(original.clone(), vec![], limits, 1);
        actual.bus.commit_group(&author).expect("first fits");
        actual.human();
        let mut meter = Meter::new(limits);
        let projected = oracle.charge(&mut meter);
        reset(&counters);
        let result = actual.bus.apply_history(&undo);
        match projected {
            Ok(()) => assert_eq!(
                result.expect("maximum stays fixed at128 bits").entry(),
                undone.entry()
            ),
            Err(expected) => assert_eq!(
                control(&result.expect_err("one bit short")),
                Some(&Control::Retained(expected))
            ),
        }
        assert_eq!(counts(&counters)[0], 0);
    }
}
struct Observe {
    stop: Option<usize>,
    revoke: Option<Rc<Cell<bool>>>,
}
impl CommandProgressObserver for Observe {
    fn observe(&self, p: &CommandProgress) -> Decision {
        if p.phase() == Phase::RetainedPublication {
            if p.completed() == 1 {
                if let Some(revoke) = &self.revoke {
                    revoke.set(true);
                }
            }
            if self.stop == Some(p.completed()) {
                return Decision::Cancel;
            }
        }
        Decision::Continue
    }
}
#[test]
fn prospective_history_cancellation_and_late_revocation_leave_model_stacks_and_original_key_intact()
{
    let original = super::resource_measurement::fixture();
    let counters = original.inputs().sources().host().telemetry();
    let design = original.inputs().id();
    let piece = original
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("Piece")
        .id();
    let author = request(&original, 815300, piece, " é ");
    let undo = operation(
        &author,
        &DesignRevision::from_counter(1),
        815310,
        Direction::Undo,
    );
    let mut actual = configured(original, vec![id(815920)], model_policy(), 1);
    actual
        .bus
        .commit_group(&author)
        .expect("real initial snapshot");
    actual.human();
    let address = match actual
        .bus
        .execute(&inspect(design, DesignRevision::from_counter(1)))
        .expect("current snapshot")
    {
        CommandResult::PieceList(list) => list.pieces().as_ptr(),
    };
    for completed in [0, 1] {
        reset(&counters);
        let error = actual
            .bus
            .apply_history_with_progress(
                &undo,
                &Observe {
                    stop: Some(completed),
                    revoke: None,
                },
            )
            .expect_err("prospective cancellation");
        assert!(
            matches!(control(&error),Some(Control::Cancelled {progress}) if progress.phase()==Phase::RetainedPublication && progress.completed()==completed)
        );
        assert_eq!(counts(&counters)[0], 0);
        let CommandResult::PieceList(current) = actual
            .bus
            .execute(&inspect(design, DesignRevision::from_counter(1)))
            .expect("same snapshot after cancellation");
        assert_eq!(current.pieces().as_ptr(), address);
    }
    reset(&counters);
    actual.reads.set(0);
    let error = actual
        .bus
        .apply_history_with_progress(
            &undo,
            &Observe {
                stop: None,
                revoke: Some(Rc::clone(&actual.revoked)),
            },
        )
        .expect_err("fresh final history grant after admission");
    assert!(
        matches!(error,sc_core::command::CommandHistoryError::Preparation(issue) if matches!(*issue,sc_core::command::CommandGroupPreviewError::Authorization(_)))
    );
    assert_eq!(actual.reads.get(), 2);
    assert_eq!(counts(&counters)[0], 0);
    actual.revoked.set(false);
    let CommandResult::PieceList(current) = actual
        .bus
        .execute(&inspect(design, DesignRevision::from_counter(1)))
        .expect("unchanged after revocation");
    assert_eq!(current.pieces().as_ptr(), address);
    let accepted = actual
        .bus
        .apply_history(&undo)
        .expect("original request/key and sole action room intact");
    assert!(!accepted.is_replay());
    assert_eq!(accepted.entry().revision(), &DesignRevision::initial());
}
