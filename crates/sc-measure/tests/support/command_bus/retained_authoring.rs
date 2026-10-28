//! Exact prospective publication versus independently reconstructed real models/accepted records.
use super::group_commit::{capacity, group, material, mutation};
use super::resource_admission::{control, counts, demand, reset};
use super::retained_current::{history, policy, request, with_assignment, Model, Observed};
use super::*;
use sc_core::{
    command::{
        charge_command_audit_entry_resources, ChangeMeasurement, CommandControlError as Control,
        CommandHistoryDirection as Direction, CommandOperationLimits, CommandProgress,
        CommandProgressDecision as Decision, CommandProgressObserver,
        CommandProgressPhase as Phase,
    },
    design::{
        DesignInputs, DesignResourceLimitError, DesignResourceLimits, DesignResourceMeter as Meter,
        DesignResourceScope, DesignSewing, DesignStructure,
    },
    value::{LengthDeclaration, LengthState},
};
use std::num::NonZeroUsize;
fn configured(
    owner: Model,
    reserve: Vec<EntityId>,
    limits: DesignResourceLimits,
    depth: usize,
) -> CommandBus<resource_copies::CountedSources, Host> {
    let host = Host::new(owner.inputs().id());
    let mut bus = CommandBus::with_edit_history(
        owner,
        host,
        reserve,
        capacity(8),
        capacity(depth),
        capacity(8),
    )
    .expect("actual finite owner");
    install(&mut bus, limits);
    bus
}
fn install<H, A>(bus: &mut CommandBus<H, A>, limits: DesignResourceLimits) {
    bus.install_operation_limits(CommandOperationLimits::new(
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
    ))
    .expect("input policy");
    bus.install_model_limits(model_policy())
        .expect("model policy");
    bus.install_retained_limits(limits)
        .expect("retained policy");
}
#[test]
fn prospective_alias_record_and_history_fit_exact_limits_or_refuse_before_publication_copies() {
    let original = super::resource_measurement::fixture();
    let counters = original.inputs().sources().host().telemetry();
    let design = original.inputs().id();
    let piece = original
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual Piece")
        .id();
    let candidate = with_assignment(&original, 1, piece, " é ");
    let member = mutation(
        design,
        DesignRevision::initial(),
        813000,
        813001,
        material(piece, " é "),
    );
    let request = group(813000, vec![member.clone(), member]);
    let reserve = vec![id(813900), id(813901)];
    let mut reference = configured(original.clone(), reserve.clone(), model_policy(), 2);
    let accepted = reference.commit_group(&request).expect("real alias group");
    assert_eq!(
        accepted
            .entry()
            .commit()
            .changes()
            .steps()
            .get(1)
            .expect("alias")
            .replay_of(),
        Some(0)
    );
    let oracle = Observed {
        initial: &original,
        current: &candidate,
        reserve: &reserve,
        accepted: &[accepted.entry()],
        undo: &[&original],
        redo: &[],
        actions: &[],
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
            2 * base.collection_entries() + 13,
            2 * base.text_bytes() + 76,
            5
        )
    );
    for limits in [
        policy(expected.collection_entries(), expected.text_bytes(), 5),
        policy(expected.collection_entries() - 1, expected.text_bytes(), 5),
        policy(expected.collection_entries(), expected.text_bytes() - 1, 5),
        policy(expected.collection_entries(), expected.text_bytes(), 4),
    ] {
        let mut actual = configured(original.clone(), reserve.clone(), limits, 2);
        let mut meter = Meter::new(limits);
        let expected_result = oracle.charge(&mut meter);
        reset(&counters);
        let result = actual.commit_group(&request);
        match expected_result {
            Ok(()) => {
                assert_eq!(
                    result.expect("exact retained projection").entry(),
                    accepted.entry()
                );
                assert_eq!(
                    counts(&counters)[0],
                    0,
                    "shared source: no foreign H::clone"
                );
            }
            Err(expected) => {
                let error = result.expect_err("exact prospective retained refusal");
                assert_eq!(control(&error), Some(&Control::Retained(expected)));
                assert_eq!(meter.finish(), Err(expected));
                assert_eq!(
                    counts(&counters)[0],
                    0,
                    "shared source: no foreign H::clone"
                );
                let CommandResult::PieceList(current) = actual
                    .execute(&query(
                        design,
                        DesignRevision::initial(),
                        QueryPayload::GetPieceList,
                    ))
                    .expect("original snapshot preserved");
                assert_eq!(
                    current.pieces(),
                    original.inputs().structure().pieces().pieces()
                );
                if matches!(expected, DesignResourceLimitError::Count { .. }) {
                    let noop = mutation(
                        design,
                        DesignRevision::initial(),
                        813000,
                        813001,
                        material(piece, "explicit selection pending"),
                    );
                    let retry = group(813000, vec![noop.clone(), noop]);
                    assert!(!actual
                        .commit_group(&retry)
                        .expect("refused group/key/capacities remain available for a smaller no-op")
                        .is_replay());
                }
            }
        }
    }
}
#[test]
fn exact_oldest_undo_eviction_does_not_add_dropped_models_to_the_future_budget() {
    let original = super::resource_measurement::fixture();
    let counters = original.inputs().sources().host().telemetry();
    let piece = original
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual Piece")
        .id();
    let one = with_assignment(&original, 1, piece, " é ");
    let two = with_assignment(&one, 2, piece, " β ");
    let three = with_assignment(&two, 3, piece, " γ ");
    let a = request(&original, 813100, piece, " é ");
    let b = request(&one, 813110, piece, " β ");
    let c = request(&two, 813120, piece, " γ ");
    let reserve = vec![id(813910), id(813911)];
    let mut reference = configured(original.clone(), reserve.clone(), model_policy(), 1);
    let first = reference.commit_group(&a).expect("first");
    let second = reference
        .commit_group(&b)
        .expect("second evicts first undo");
    let third = reference
        .commit_group(&c)
        .expect("third evicts second undo");
    assert_eq!(
        third.entry().commit().history().dropped_undo(),
        Some(b.id())
    );
    let oracle = Observed {
        initial: &original,
        current: &three,
        reserve: &reserve,
        accepted: &[first.entry(), second.entry(), third.entry()],
        undo: &[&two],
        redo: &[],
        actions: &[],
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
            2 * base.collection_entries() + 21,
            2 * base.text_bytes() + 60,
            30
        )
    );
    let mut actual = configured(
        original.clone(),
        reserve,
        policy(expected.collection_entries(), expected.text_bytes(), 30),
        1,
    );
    actual
        .commit_group(&a)
        .expect("first under exact final budget");
    actual.commit_group(&b).expect("second");
    reset(&counters);
    let accepted = actual
        .commit_group(&c)
        .expect("oldest snapshot removed before exact future composition");
    assert_eq!(accepted.entry(), third.entry());
    assert_eq!(
        counts(&counters)[0],
        0,
        "shared source: no foreign H::clone"
    );
    assert_eq!(
        counts(&counters)[1],
        7,
        "current2 + model current/metadata/revision3 + final retained2"
    );
}
fn with_evidence(before: &Model, count: usize) -> Model {
    let bound = before
        .inputs()
        .sources()
        .measurement_declaration(id(1), before.inputs().inputs())
        .expect("real width");
    let mut definition = bound.definition().clone();
    let value = match definition.state {
        LengthState::Known { value, .. } => Some(value),
        _ => None,
    }
    .expect("actual positive value");
    definition.state = LengthState::Known {
        value,
        evidence: (0..count).map(|i| id(814000 + i as u128)).collect(),
    };
    let replacement = LengthDeclaration::new(definition).expect("actual large evidence");
    let mut garment = before.definition().clone();
    let mut sewing = garment.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.inputs = structure
        .inputs
        .with_length_declaration(replacement)
        .expect("actual canonical replacement");
    inputs.structure = DesignStructure::new(structure).expect("structure");
    sewing.inputs = DesignInputs::new(inputs).expect("inputs");
    garment.sewing = DesignSewing::new(sewing).expect("sewing");
    let owner = DesignGarment::new(garment).expect("actual evidence model");
    owner
        .validate_current()
        .expect("independently validated oracle");
    owner
}
fn unknown_model(before: &Model) -> (Model, LengthDeclaration) {
    let mut definition = before
        .inputs()
        .sources()
        .measurement_declaration(id(1), before.inputs().inputs())
        .expect("real bound Measurement")
        .definition()
        .clone();
    definition.state = LengthState::Unknown {
        observation: id(813299),
    };
    let replacement = LengthDeclaration::new(definition).expect("actual explicit Unknown");
    let mut garment = before.definition().clone();
    let mut sewing = garment.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.revision = DesignRevision::from_counter(1);
    structure.inputs = structure
        .inputs
        .with_length_declaration(replacement.clone())
        .expect("actual Unknown canonical slot");
    inputs.structure = DesignStructure::new(structure).expect("structure");
    sewing.inputs = DesignInputs::new(inputs).expect("inputs");
    garment.sewing = DesignSewing::new(sewing).expect("sewing");
    let owner = DesignGarment::new(garment).expect("actual Unknown model");
    owner
        .validate_current()
        .expect("independent Unknown oracle");
    (owner, replacement)
}
#[test]
fn exact_branch_discards_redo_models_but_retains_ids_actions_and_maximum_issued_revision() {
    const EVIDENCE: usize = 512;
    let original = with_evidence(&super::resource_measurement::fixture(), EVIDENCE);
    let counters = original.inputs().sources().host().telemetry();
    let piece = original
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("Piece")
        .id();
    let (one, replacement) = unknown_model(&original);
    let two = with_assignment(&one, 2, piece, " é ");
    let branch_model = with_assignment(&original, 3, piece, " γ ");
    let a = group(
        813200,
        vec![mutation(
            original.inputs().id(),
            DesignRevision::initial(),
            813200,
            813201,
            MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), replacement)),
        )],
    );
    let b = request(&one, 813210, piece, " é ");
    let c = request(&original, 813220, piece, " γ ");
    let undo_b = history(&b, 2, 813230, Direction::Undo);
    let undo_a = history(&a, 1, 813231, Direction::Undo);
    let reserve = vec![id(813920), id(813921)];
    let mut reference = configured(original.clone(), reserve.clone(), model_policy(), 2);
    let first = reference.commit_group(&a).expect("actual evidence shrink");
    let second = reference.commit_group(&b).expect("actual Material");
    let h1 = reference.apply_history(&undo_b).expect("undo second");
    let h2 = reference.apply_history(&undo_a).expect("undo first");
    let third = reference.commit_group(&c).expect("actual new branch");
    assert_eq!(
        third.entry().commit().history().discarded_redo(),
        &[a.id(), b.id()]
    );
    assert_eq!(
        third.entry().commit().revision(),
        &DesignRevision::from_counter(3)
    );
    assert_eq!(third.entry().sequence().to_string(), "5");
    let oracle = Observed {
        initial: &original,
        current: &branch_model,
        reserve: &reserve,
        accepted: &[first.entry(), second.entry(), third.entry()],
        undo: &[&original],
        redo: &[],
        actions: &[h1.entry(), h2.entry()],
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
            2 * base.collection_entries() + 2 * EVIDENCE + 29,
            2 * base.text_bytes() + 110,
            36
        )
    );
    let before = Observed {
        initial: &original,
        current: &original,
        reserve: &reserve,
        accepted: &[first.entry(), second.entry()],
        undo: &[],
        redo: &[&two, &one],
        actions: &[h1.entry(), h2.entry()],
    }
    .demand();
    assert!(
        expected.collection_entries() > before.collection_entries(),
        "large real evidence makes final exact entry maximum cover the earlier redo state"
    );
    for (entry_limit, bit_limit) in [
        (expected.collection_entries(), 36),
        (expected.collection_entries() - 1, 36),
        (expected.collection_entries(), 35),
    ] {
        let mut actual = configured(
            original.clone(),
            reserve.clone(),
            policy(
                entry_limit,
                expected.text_bytes().max(before.text_bytes()),
                bit_limit,
            ),
            2,
        );
        actual.commit_group(&a).expect("first");
        actual.commit_group(&b).expect("second");
        actual.apply_history(&undo_b).expect("undo second");
        actual.apply_history(&undo_a).expect("undo first");
        let limits = policy(
            entry_limit,
            expected.text_bytes().max(before.text_bytes()),
            bit_limit,
        );
        let mut meter = Meter::new(limits);
        let projected = oracle.charge(&mut meter);
        reset(&counters);
        let result = actual.commit_group(&c);
        match projected {
            Ok(()) => {
                assert_eq!(
                    result
                        .expect("exact branch without discarded model payloads")
                        .entry(),
                    third.entry()
                );
                assert_eq!(
                    counts(&counters)[0],
                    0,
                    "shared source: no foreign H::clone"
                );
                assert_eq!(
                    counts(&counters)[1],
                    8,
                    "current3 + individual model3 + retained future2"
                );
            }
            Err(expected) => {
                assert_eq!(
                    control(&result.expect_err("one-short original branch refusal")),
                    Some(&Control::Retained(expected))
                );
                assert_eq!(
                    counts(&counters)[0],
                    0,
                    "shared source: no foreign H::clone"
                );
            }
        }
    }
}
#[test]
fn prospective_counter_carry_beyond_u128_is_exact_and_refuses_before_final_revision_allocation() {
    let initial = super::resource_revision::with_revision(
        &super::resource_measurement::fixture(),
        DesignRevision::from_counter(u128::MAX),
    );
    let counters = initial.inputs().sources().host().telemetry();
    let design = initial.inputs().id();
    let piece = initial
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("Piece")
        .id();
    assert_eq!(demand(&initial).counter_bits(), 128);
    let request = request(&initial, 813300, piece, "explicit selection pending");
    let base = demand(&initial);
    for bits in [1029, 1028] {
        let mut actual =
            CommandBus::with_edit_capacity(initial.clone(), Host::new(design), vec![], capacity(1))
                .expect("actual no-history owner");
        install(
            &mut actual,
            policy(base.collection_entries() + 4, base.text_bytes() + 26, bits),
        );
        reset(&counters);
        let result = actual.commit_group(&request);
        if bits == 1029 {
            let result = result.expect("exact future bits");
            assert_eq!(
                result.entry().commit().revision(),
                &DesignRevision::from_counter(u128::MAX).successor()
            );
            let mut meter = Meter::new(model_policy());
            charge_command_audit_entry_resources(result.entry(), &mut meter)
                .expect("actual public record oracle");
            assert_eq!(
                meter.finish().expect("actual record demand").counter_bits(),
                642
            );
            assert_eq!(
                counts(&counters)[0],
                0,
                "shared source: no foreign H::clone"
            );
        } else {
            let error = result.expect_err("one bit short");
            let expected = DesignResourceLimitError::CounterBits {
                maximum: 1028,
                consumed: 900,
                requested: 129,
                location: sc_core::design::DesignResourceLocation {
                    scope: DesignResourceScope::CommandState,
                    owner: Some(request.id().entity_id()),
                    field: "records.commit.revision",
                    index: None,
                },
            };
            assert_eq!(control(&error), Some(&Control::Retained(expected)));
            assert_eq!(
                counts(&counters)[0],
                0,
                "shared source: no foreign H::clone"
            );
            actual
                .execute(&query(
                    design,
                    DesignRevision::from_counter(u128::MAX),
                    QueryPayload::GetPieceList,
                ))
                .expect("original large revision unchanged");
        }
    }
}
struct Revoke(Rc<Cell<bool>>);
impl CommandProgressObserver for Revoke {
    fn observe(&self, p: &CommandProgress) -> Decision {
        if p.phase() == Phase::RetainedPublication && p.completed() == 1 {
            self.0.set(true);
        }
        Decision::Continue
    }
}
struct Stop(usize);
impl CommandProgressObserver for Stop {
    fn observe(&self, p: &CommandProgress) -> Decision {
        if p.phase() == Phase::RetainedPublication && p.completed() == self.0 {
            Decision::Cancel
        } else {
            Decision::Continue
        }
    }
}
#[test]
fn prospective_checkpoints_cancel_before_publication_construction_and_leave_group_key_available() {
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
    let group = request(&original, 813400, piece, " é ");
    let host = Host::new(design);
    let revoked = Rc::clone(&host.revoked);
    let reads = Rc::clone(&host.reads);
    let mut actual = CommandBus::with_edit_history(
        original,
        host,
        vec![id(813930)],
        capacity(8),
        capacity(2),
        capacity(8),
    )
    .expect("actual fresh-grant owner");
    install(&mut actual, model_policy());
    for completed in [0, 1] {
        reset(&counters);
        let error = actual
            .commit_group_with_progress(&group, &Stop(completed))
            .expect_err("prospective checkpoint cancellation");
        assert!(
            matches!(control(&error),Some(Control::Cancelled {progress}) if progress.phase()==Phase::RetainedPublication && progress.completed()==completed)
        );
        assert_eq!(
            counts(&counters)[0],
            0,
            "shared source: no foreign H::clone"
        );
        actual
            .execute(&query(
                design,
                DesignRevision::initial(),
                QueryPayload::GetPieceList,
            ))
            .expect("original state remains");
    }
    reset(&counters);
    reads.set(0);
    let error = actual
        .commit_group_with_progress(&group, &Revoke(Rc::clone(&revoked)))
        .expect_err("post-resource admission revocation still requires fresh final grant");
    assert!(
        matches!(error,sc_core::command::CommandGroupCommitError::Preparation(issue) if matches!(*issue,sc_core::command::CommandGroupPreviewError::Authorization(_)))
    );
    assert_eq!(reads.get(), 2, "initial and fresh final authorization");
    assert_eq!(
        counts(&counters)[0],
        0,
        "shared source: no foreign H::clone"
    );
    revoked.set(false);
    actual
        .execute(&query(
            design,
            DesignRevision::initial(),
            QueryPayload::GetPieceList,
        ))
        .expect("original snapshot after revocation");
    assert!(!actual
        .commit_group(&group)
        .expect("original key/group/capacity preserved")
        .is_replay());
}
