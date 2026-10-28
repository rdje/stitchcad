//! Exact private composition checked through real authorized command paths and independent models.
use super::group_commit::{capacity, group, material, mutation};
use super::resource_admission::{control, counts, demand, reset};
use super::resource_copies::CountedSources;
use super::*;
use sc_core::{
    command::{
        charge_command_audit_entry_resources, charge_command_history_audit_entry_resources,
        CommandAuditEntry, CommandControlError as Control, CommandHistoryAuditEntry,
        CommandHistoryDirection as Direction, CommandHistoryRequest, CommandMutationGroup,
        CommandOperationLimits, CommandProgress, CommandProgressDecision as Decision,
        CommandProgressObserver, CommandProgressPhase as Phase,
    },
    design::{
        charge_design_garment_resources, DesignInputs, DesignResourceDemand,
        DesignResourceLimitError as LimitError, DesignResourceLimits,
        DesignResourceLocation as Location, DesignResourceMeter as Meter,
        DesignResourceScope as Scope, DesignResourceSink, DesignSewing, DesignStructure,
    },
    ontology::{MaterialAssignment, PieceInventory},
};
use std::num::{NonZeroU64, NonZeroUsize};
type Model = DesignGarment<CountedSources>;
fn location(owner: EntityId, field: &'static str) -> Location {
    Location {
        scope: Scope::CommandState,
        owner: Some(owner),
        field,
        index: None,
    }
}
fn policy(entries: usize, text: usize, bits: u64) -> DesignResourceLimits {
    DesignResourceLimits::new(
        capacity(entries.max(1)),
        capacity(text.max(1)),
        NonZeroU64::new(bits.max(1)).expect("positive bits"),
    )
}
struct Stop(usize);
impl CommandProgressObserver for Stop {
    fn observe(&self, p: &CommandProgress) -> Decision {
        if p.phase() == Phase::RetainedValidation && p.completed() == self.0 {
            Decision::Cancel
        } else {
            Decision::Continue
        }
    }
}
fn bus(
    owner: Model,
    reserve: Vec<EntityId>,
    limits: DesignResourceLimits,
) -> CommandBus<CountedSources, Host> {
    let design = owner.inputs().id();
    let mut bus = CommandBus::with_edit_history(
        owner,
        Host::new(design),
        reserve,
        capacity(6),
        capacity(2),
        capacity(6),
    )
    .expect("actual finite owner");
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
        .expect("exact retained policy");
    bus
}
fn with_assignment(before: &Model, revision: u128, piece: EntityId, reason: &str) -> Model {
    let mut garment = before.definition().clone();
    let mut sewing = garment.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.revision = DesignRevision::from_counter(revision);
    structure.pieces = PieceInventory::new(
        structure
            .pieces
            .pieces()
            .iter()
            .map(|p| {
                if p.id() == piece {
                    p.with_material_assignment(MaterialAssignment::Unresolved(reason.into()))
                        .expect("actual assignment")
                } else {
                    p.clone()
                }
            })
            .collect(),
    )
    .expect("actual inventory");
    inputs.structure = DesignStructure::new(structure).expect("actual structure");
    sewing.inputs = DesignInputs::new(inputs).expect("actual inputs");
    garment.sewing = DesignSewing::new(sewing).expect("actual sewing");
    let model = DesignGarment::new(garment).expect("actual independently reconstructed model");
    model
        .validate_current()
        .expect("actual complete independent model oracle");
    model
}
fn request(model: &Model, number: u128, piece: EntityId, reason: &str) -> CommandMutationGroup {
    group(
        number,
        vec![mutation(
            model.inputs().id(),
            model.inputs().revision().clone(),
            number,
            number + 1,
            material(piece, reason),
        )],
    )
}
fn history(
    original: &CommandMutationGroup,
    revision: u128,
    key: u128,
    direction: Direction,
) -> CommandHistoryRequest {
    CommandHistoryRequest::new(
        CommandMutationContext::new(
            original.context().design(),
            actor(),
            DesignRevision::from_counter(revision),
            CommandIdempotencyKey::from_entity_id(id(key)),
            Some(original.id()),
        ),
        original.clone(),
        direction,
    )
    .expect("actual history intent")
}
// Independent small-fixture oracle uses public exact decimal values, never a private counter reader.
fn bits(value: impl ToString) -> u64 {
    value
        .to_string()
        .parse::<u128>()
        .expect("this fixture uses only counters0..5")
        .checked_ilog2()
        .map_or(0, |n| u64::from(n) + 1)
}
struct Observed<'a> {
    initial: &'a Model,
    current: &'a Model,
    reserve: &'a [EntityId],
    accepted: &'a [&'a CommandAuditEntry],
    undo: &'a [&'a Model],
    redo: &'a [&'a Model],
    actions: &'a [&'a CommandHistoryAuditEntry],
}
impl Observed<'_> {
    fn charge(&self, meter: &mut Meter) -> Result<(), LimitError> {
        let owner = self.current.inputs().id();
        charge_design_garment_resources(self.current, meter)?;
        meter.charge_entries(self.reserve.len(), location(owner, "retained.identities"))?;
        meter.charge_counter_bits(
            bits(self.initial.inputs().revision()),
            location(owner, "retained.journal.initial_revision"),
        )?;
        let maximum = self
            .accepted
            .last()
            .map_or(self.initial.inputs().revision(), |e| e.commit().revision());
        meter.charge_counter_bits(
            bits(maximum),
            location(owner, "retained.journal.maximum_revision"),
        )?;
        let sequence = self
            .accepted
            .iter()
            .map(|e| e.sequence())
            .chain(self.actions.iter().map(|e| e.sequence()))
            .max();
        meter.charge_counter_bits(
            sequence.map_or(0, bits),
            location(owner, "retained.journal.sequence"),
        )?;
        meter.charge_entries(
            self.accepted.len(),
            location(owner, "retained.journal.entries"),
        )?;
        for entry in self.accepted {
            charge_command_audit_entry_resources(entry, meter)?;
        }
        meter.charge_entries(self.undo.len(), location(owner, "retained.history.undo"))?;
        for model in self.undo {
            charge_design_garment_resources(model, meter)?;
        }
        meter.charge_entries(self.redo.len(), location(owner, "retained.history.redo"))?;
        for model in self.redo {
            charge_design_garment_resources(model, meter)?;
        }
        meter.charge_entries(
            self.actions.len(),
            location(owner, "retained.history.actions"),
        )?;
        for action in self.actions {
            charge_command_history_audit_entry_resources(action, meter)?;
        }
        Ok(())
    }
    fn demand(&self) -> DesignResourceDemand {
        let mut meter = Meter::new(model_policy());
        self.charge(&mut meter)
            .expect("actual independent owner composition");
        meter.finish().expect("complete oracle")
    }
}
#[test]
fn exact_current_private_composition_includes_all_models_records_actions_and_independent_counters()
{
    let original = super::resource_measurement::fixture();
    let counters = original.inputs().sources().host().telemetry();
    let first = original
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual first Piece")
        .id();
    let second = original
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .get(1)
        .expect("actual second Piece")
        .id();
    let after_first = with_assignment(&original, 1, first, " é ");
    let after_second = with_assignment(&after_first, 2, second, " β ");
    let first_request = request(&original, 812000, first, " é ");
    let second_request = request(&after_first, 812010, second, " β ");
    let reserve = vec![id(812900), id(812901), id(812902)];
    // First real owner produces immutable accepted records; no supplied resource totals/model stand-in.
    let mut first_bus = bus(original.clone(), reserve.clone(), model_policy());
    let a = first_bus
        .commit_group(&first_request)
        .expect("actual first");
    let b = first_bus
        .commit_group(&second_request)
        .expect("actual second");
    let undo_b = history(&second_request, 2, 812020, Direction::Undo);
    let undo_a = history(&first_request, 1, 812021, Direction::Undo);
    let redo_a = history(&first_request, 0, 812022, Direction::Redo);
    let h1 = first_bus
        .apply_history(&undo_b)
        .expect("actual undo second");
    let h2 = first_bus.apply_history(&undo_a).expect("actual undo first");
    let h3 = first_bus.apply_history(&redo_a).expect("actual redo first");
    assert_eq!(h3.entry().sequence().to_string(), "5");
    assert_eq!(
        b.entry().commit().revision(),
        &DesignRevision::from_counter(2)
    );
    let oracle = Observed {
        initial: &original,
        current: &after_first,
        reserve: &reserve,
        accepted: &[a.entry(), b.entry()],
        undo: &[&original],
        redo: &[&after_second],
        actions: &[h1.entry(), h2.entry(), h3.entry()],
    };
    let expected = oracle.demand();
    let base = demand(&original);
    assert_eq!(
        expected.collection_entries(),
        3 * base.collection_entries() + 29
    );
    assert_eq!(expected.text_bytes(), 3 * base.text_bytes() + 74);
    assert_eq!(expected.counter_bits(), 36);
    for limit in [
        policy(expected.collection_entries(), expected.text_bytes(), 36),
        policy(expected.collection_entries() - 1, expected.text_bytes(), 36),
        policy(expected.collection_entries(), expected.text_bytes() - 1, 36),
        policy(expected.collection_entries(), expected.text_bytes(), 35),
    ] {
        let mut actual = bus(original.clone(), reserve.clone(), limit);
        assert_eq!(
            actual
                .commit_group(&first_request)
                .expect("first under final current budget")
                .entry(),
            a.entry()
        );
        assert_eq!(
            actual
                .commit_group(&second_request)
                .expect("second under final current budget")
                .entry(),
            b.entry()
        );
        actual.apply_history(&undo_b).expect("undo second");
        actual.apply_history(&undo_a).expect("undo first");
        actual.apply_history(&redo_a).expect("redo first");
        let mut meter = Meter::new(limit);
        let oracle_result = oracle.charge(&mut meter);
        reset(&counters);
        let probe = request(&after_first, 812030, first, " é ");
        let error = actual
            .preview_group_with_progress(&probe, &Stop(1))
            .expect_err("stop after current admission or original quota");
        match oracle_result {
            Ok(()) => {
                assert!(
                    matches!(control(&error),Some(Control::Cancelled { progress }) if progress.phase()==Phase::RetainedValidation && progress.completed()==1)
                );
                assert_eq!(
                    counts(&counters),
                    [0, 3, 0],
                    "three actual retained models; no semantic validation/copies"
                );
            }
            Err(expected) => {
                assert_eq!(control(&error), Some(&Control::Retained(expected)));
                assert_eq!(
                    std::error::Error::source(control(&error).expect("typed retained cause"))
                        .and_then(|issue| issue.downcast_ref::<LimitError>()),
                    Some(&expected)
                );
                assert_eq!(meter.finish(), Err(expected));
                assert_eq!(counts(&counters)[0], 0);
                assert_eq!(counts(&counters)[2], 0);
            }
        }
        reset(&counters);
        let replay = actual
            .commit_group(&first_request)
            .expect("accepted authoring replay skips current resource admission");
        assert!(replay.is_replay());
        let replay = actual
            .apply_history(&undo_a)
            .expect("accepted history replay skips current resource admission");
        assert!(replay.is_replay());
        assert_eq!(counts(&counters), [0, 0, 0]);
    }
}
#[test]
fn retained_reserve_counts_the_entire_backing_after_a_real_notch_consumes_its_prefix() {
    let (owner, addition) = resource_copies::owner(Rc::new(Cell::new(0)));
    let base = demand(&owner);
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let reserve = vec![id(812800), id(812801)];
    let mut actual = bus(
        owner,
        reserve,
        policy(base.collection_entries() + 2, usize::MAX, u64::MAX),
    );
    let original = group(
        812100,
        vec![mutation(
            design,
            revision,
            812100,
            812101,
            MutationPayload::AddNotch(addition),
        )],
    );
    actual
        .commit_group(&original)
        .expect("actual Notch consumes one reserve identity");
    let next = group(
        812110,
        vec![mutation(
            design,
            DesignRevision::from_counter(1),
            812110,
            812111,
            material(addition.piece(), "next"),
        )],
    );
    reset(&counters);
    let error = actual
        .preview_group(&next)
        .expect_err("new current model plus full backing exceeds initial bound");
    let expected = LimitError::Count {
        unit: sc_core::design::DesignResourceCount::CollectionEntries,
        maximum: base.collection_entries() + 2,
        consumed: base.collection_entries() + 1,
        requested: 2,
        location: location(design, "retained.identities"),
    };
    assert_eq!(control(&error), Some(&Control::Retained(expected)));
    assert_eq!(counts(&counters), [0, 1, 0]);
    reset(&counters);
    assert!(actual
        .commit_group(&original)
        .expect("exact replay still works over current bound")
        .is_replay());
    assert_eq!(counts(&counters), [0, 0, 0]);
}
#[test]
fn retained_cancellation_and_original_quota_fail_before_private_validation_or_effects() {
    let owner = super::resource_measurement::fixture();
    let counters = owner.inputs().sources().host().telemetry();
    let first = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual first Piece")
        .id();
    let probe = request(&owner, 812200, first, "next");
    let mut actual = bus(owner, vec![id(812700)], model_policy());
    for completed in [0, 1] {
        reset(&counters);
        let error = actual
            .commit_group_with_progress(&probe, &Stop(completed))
            .expect_err("cooperative retained cancellation");
        assert!(
            matches!(control(&error),Some(Control::Cancelled { progress }) if progress.phase()==Phase::RetainedValidation && progress.completed()==completed)
        );
        assert_eq!(counts(&counters), [0, completed, 0]);
    }
    let accepted = actual
        .commit_group(&probe)
        .expect("original group/key and capacities survive cancellation");
    assert!(!accepted.is_replay());
    assert_eq!(
        accepted.entry().commit().revision(),
        &DesignRevision::from_counter(1)
    );
}
