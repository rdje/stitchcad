//! Actual accepted authoring/history/result owners, including replay, eviction and branch loss.
use super::group_commit::{capacity, group, material, mutation};
use super::resource_admission::{counts, reset};
use super::resource_copies::CountedSources;
use super::*;
use sc_core::{
    command::{
        charge_command_audit_entry_resources as charge_authoring,
        charge_command_group_commit_resources as charge_commit,
        charge_command_group_commit_result_resources as charge_result,
        charge_command_history_audit_entry_resources as charge_history,
        charge_command_history_request_resources as charge_history_request,
        charge_command_history_result_resources as charge_history_result, AddNotch, AssignMaterial,
        ChangeMeasurement, CommandGroupCommitResult, CommandHistoryDirection as Direction,
        CommandHistoryRequest, CommandHistoryResult, CommandMutationGroup, PieceSelection,
    },
    design::{
        DesignResourceCount as Unit, DesignResourceLimitError as LimitError, DesignResourceLimits,
        DesignResourceLocation as Location, DesignResourceMeter as Meter,
        DesignResourceScope as Scope, DesignResourceSink,
    },
    ontology::{EdgeAnchor, MaterialAssignment, Param},
    value::{LengthDeclaration, LengthState},
};
use std::num::{NonZeroU64, NonZeroUsize};
fn limits(entries: usize, text: usize, bits: u64) -> DesignResourceLimits {
    DesignResourceLimits::new(
        NonZeroUsize::new(entries).expect("positive entries"),
        NonZeroUsize::new(text).expect("positive bytes"),
        NonZeroU64::new(bits).expect("positive bits"),
    )
}
fn location(field: &'static str, index: Option<usize>) -> Location {
    Location {
        scope: Scope::CommandState,
        owner: Some(id(810000)),
        field,
        index,
    }
}
fn units(meter: Meter) -> (usize, usize, u64) {
    let d = meter.finish().expect("complete actual demand");
    (d.collection_entries(), d.text_bytes(), d.counter_bits())
}
struct Actual {
    bus: CommandBus<CountedSources, Host>,
    request: CommandMutationGroup,
    accepted: CommandGroupCommitResult,
    counters: [Rc<Cell<usize>>; 3],
    reads: Rc<Cell<usize>>,
    actor: Rc<Cell<Option<Actor>>>,
    piece: EntityId,
}
impl Actual {
    fn reset(&self) {
        reset(&self.counters);
        self.reads.set(0);
    }
    fn untouched(&self) {
        assert_eq!(counts(&self.counters), [0, 0, 0]);
        assert_eq!(self.reads.get(), 0);
    }
}
fn fixture() -> Actual {
    let owner = super::resource_measurement::fixture();
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let canonical = owner
        .inputs()
        .sources()
        .measurement_declaration(id(1), owner.inputs().inputs())
        .expect("actual Known2");
    let mut definition = canonical.definition().clone();
    let value = match definition.state {
        LengthState::Known { value, .. } => Some(value),
        _ => None,
    }
    .expect("actual positive width");
    definition.state = LengthState::Known {
        value,
        evidence: vec![id(810100), id(810101), id(810102)],
    };
    let replacement = LengthDeclaration::new(definition).expect("actual Known3");
    let pieces = owner.inputs().structure().pieces().pieces();
    assert_eq!(pieces.len(), 2);
    for p in pieces {
        assert_eq!(
            p.definition().material,
            MaterialAssignment::Unresolved("explicit selection pending".into())
        );
    }
    let first = pieces.first().expect("actual first Piece");
    let piece = first.id();
    let selection = pieces.iter().rev().map(|p| p.id()).collect();
    let addition = AddNotch::new(
        piece,
        EdgeAnchor {
            edge: first.edges().next().expect("actual edge").edge,
            param: Param::START,
        },
        notch_preview::bindings(),
    );
    let measure = mutation(
        design,
        revision.clone(),
        810000,
        810001,
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), replacement)),
    );
    let request = group(
        810000,
        vec![
            measure.clone(),
            mutation(
                design,
                revision.clone(),
                810000,
                810002,
                MutationPayload::AssignMaterial(
                    AssignMaterial::new(
                        PieceSelection::new(selection).expect("actual reversed Piece selection"),
                        MaterialAssignment::Unresolved(" é ".into()),
                    )
                    .expect("actual exact text"),
                ),
            ),
            mutation(
                design,
                revision,
                810000,
                810003,
                MutationPayload::AddNotch(addition),
            ),
            measure,
        ],
    );
    let host = super::history::operator_host(design, CommandId::ALL.to_vec());
    let reads = Rc::clone(&host.reads);
    let actor = Rc::clone(&host.actor);
    let mut bus = controlled(
        CommandBus::with_edit_history(
            owner,
            host,
            vec![id(810200)],
            capacity(6),
            capacity(2),
            capacity(6),
        )
        .expect("finite actual journal/history"),
    );
    let accepted = bus
        .commit_group(&request)
        .expect("real mixed accepted authoring");
    let actual = Actual {
        bus,
        request,
        accepted,
        counters,
        reads,
        actor,
        piece,
    };
    actual.reset();
    actual
}
fn operation(
    original: &CommandMutationGroup,
    revision: DesignRevision,
    key: u128,
    direction: Direction,
) -> CommandHistoryRequest {
    CommandHistoryRequest::new(
        CommandMutationContext::new(
            original.context().design(),
            super::history::human(),
            revision,
            CommandIdempotencyKey::from_entity_id(id(key)),
            Some(original.id()),
        ),
        original.clone(),
        direction,
    )
    .expect("actual distinct operator and original author intent")
}
fn roundtrip(actual: &mut Actual) -> (CommandHistoryResult, CommandHistoryResult) {
    actual.actor.set(Some(super::history::human()));
    let undo = operation(
        &actual.request,
        DesignRevision::from_counter(1),
        810010,
        Direction::Undo,
    );
    let undone = actual.bus.apply_history(&undo).expect("actual human undo");
    let redone = actual
        .bus
        .apply_history(&operation(
            &actual.request,
            DesignRevision::initial(),
            810011,
            Direction::Redo,
        ))
        .expect("actual human redo");
    let replay = actual
        .bus
        .apply_history(&undo)
        .expect("exact undo replay after redo");
    assert!(replay.is_replay());
    assert_eq!(replay.entry(), undone.entry());
    actual.reset();
    (undone, redone)
}
#[test]
fn actual_authoring_commit_history_and_replayed_result_copies_have_exact_owned_demands() {
    let mut actual = fixture();
    let original = actual.accepted.clone();
    let mut meter = Meter::new(limits(39, 124, 2));
    charge_authoring(actual.accepted.entry(), &mut meter).expect("actual record");
    assert_eq!(units(meter), (39, 124, 2));
    let mut meter = Meter::new(limits(27, 120, 1));
    charge_commit(actual.accepted.entry().commit(), &mut meter).expect("actual commit");
    assert_eq!(units(meter), (27, 120, 1));
    let mut meter = Meter::new(limits(39, 124, 2));
    charge_result(&actual.accepted, &mut meter).expect("returned owned record");
    assert_eq!(units(meter), (39, 124, 2));
    actual.untouched();
    let (undo, redo) = roundtrip(&mut actual);
    assert_eq!(
        undo.entry().commands(),
        &[
            CommandId::ChangeMeasurement,
            CommandId::AssignMaterial,
            CommandId::AddNotch
        ]
    );
    assert_eq!(undo.entry().original_actor(), actor());
    assert_eq!(undo.entry().operator(), super::history::human());
    for (result, expected) in [(&undo, (15, 4, 4)), (&redo, (15, 4, 3))] {
        let mut meter = Meter::new(limits(15, 4, 4));
        charge_history(result.entry(), &mut meter).expect("actual action");
        assert_eq!(units(meter), expected);
        let mut meter = Meter::new(limits(15, 4, 4));
        charge_history_result(result, &mut meter).expect("returned action copy");
        assert_eq!(units(meter), expected);
    }
    let mut meter = Meter::new(limits(12, 4, 1));
    charge_history_request(undo.entry().request(), &mut meter)
        .expect("operator context plus original request");
    assert_eq!(units(meter), (12, 4, 1));
    actual.untouched();
    actual.actor.set(Some(actor()));
    let replay = actual
        .bus
        .commit_group(&actual.request)
        .expect("exact authoring replay after roundtrip");
    assert!(replay.is_replay());
    assert_eq!(replay.entry(), original.entry());
    actual.reset();
    let mut meter = Meter::new(limits(39, 124, 2));
    charge_result(&replay, &mut meter).expect("actual replay copy still owns fields");
    assert_eq!(units(meter), (39, 124, 2));
    assert_eq!(actual.accepted, original);
    actual.untouched();
}
#[test]
fn actual_record_refusals_preserve_original_sequence_request_report_and_history_locations() {
    let mut actual = fixture();
    for (maximum, consumed, requested, field, index) in [
        (3, 0, 4, "requests.mutations", None),
        (11, 9, 3, "requests.mutations.measurement.evidence", Some(3)),
        (21, 19, 3, "reports.measurement.after.evidence", Some(0)),
    ] {
        let expected = LimitError::Count {
            unit: Unit::CollectionEntries,
            maximum,
            consumed,
            requested,
            location: location(field, index),
        };
        let mut meter = Meter::new(limits(maximum, 124, 2));
        assert_eq!(
            charge_authoring(actual.accepted.entry(), &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
    }
    for (maximum, consumed, requested, field, index) in [
        (3, 0, 4, "requests.mutations.material.reason", Some(1)),
        (59, 34, 26, "reports.material.before.reason", Some(1)),
        (
            123,
            120,
            4,
            "reports.difference.materials.after.reason",
            Some(1),
        ),
    ] {
        let expected = LimitError::Count {
            unit: Unit::TextBytes,
            maximum,
            consumed,
            requested,
            location: location(field, index),
        };
        let mut meter = Meter::new(limits(39, maximum, 2));
        assert_eq!(
            charge_authoring(actual.accepted.entry(), &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
    }
    let expected = LimitError::CounterBits {
        maximum: 1,
        consumed: 1,
        requested: 1,
        location: location("records.commit.revision", None),
    };
    let mut meter = Meter::new(limits(39, 124, 1));
    assert_eq!(
        charge_authoring(actual.accepted.entry(), &mut meter),
        Err(expected)
    );
    assert_eq!(meter.finish(), Err(expected));
    actual.untouched();
    let (undo, redo) = roundtrip(&mut actual);
    for (maximum, consumed, requested, field) in [
        (1, 0, 2, "records.history.sequence"),
        (2, 2, 1, "requests.history.context.revision"),
        (3, 3, 1, "records.history.before_revision"),
    ] {
        let expected = LimitError::CounterBits {
            maximum,
            consumed,
            requested,
            location: location(field, None),
        };
        let mut meter = Meter::new(limits(15, 4, maximum));
        assert_eq!(charge_history(undo.entry(), &mut meter), Err(expected));
        assert_eq!(meter.finish(), Err(expected));
    }
    let expected = LimitError::CounterBits {
        maximum: 2,
        consumed: 2,
        requested: 1,
        location: location("records.history.revision", None),
    };
    let mut meter = Meter::new(limits(15, 4, 2));
    assert_eq!(charge_history(redo.entry(), &mut meter), Err(expected));
    assert_eq!(meter.finish(), Err(expected));
    let expected = LimitError::Count {
        unit: Unit::CollectionEntries,
        maximum: 2,
        consumed: 0,
        requested: 3,
        location: location("records.history.commands", None),
    };
    let mut meter = Meter::new(limits(2, 4, 4));
    assert_eq!(charge_history(undo.entry(), &mut meter), Err(expected));
    assert_eq!(meter.finish(), Err(expected));
    actual.untouched();
}
#[test]
fn accepted_records_compose_prefix_and_preserve_a_prior_failure_through_every_public_reader() {
    let mut actual = fixture();
    let (undo, _) = roundtrip(&mut actual);
    let prior = location("prior", None);
    let mut meter = Meter::new(limits(41, 126, 3));
    meter.charge_entries(2, prior).expect("prior entries");
    meter.charge_text("é", prior).expect("prior bytes");
    meter.charge_counter_bits(1, prior).expect("prior bit");
    charge_authoring(actual.accepted.entry(), &mut meter).expect("same sink");
    assert_eq!(units(meter), (41, 126, 3));
    let mut refused = Meter::new(limits(1, 1, 1));
    let first = refused
        .charge_entries(2, prior)
        .expect_err("prior original failure");
    assert_eq!(
        charge_authoring(actual.accepted.entry(), &mut refused),
        Err(first)
    );
    assert_eq!(
        charge_commit(actual.accepted.entry().commit(), &mut refused),
        Err(first)
    );
    assert_eq!(charge_result(&actual.accepted, &mut refused), Err(first));
    assert_eq!(charge_history(undo.entry(), &mut refused), Err(first));
    assert_eq!(charge_history_result(&undo, &mut refused), Err(first));
    assert_eq!(
        charge_history_request(undo.entry().request(), &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
    actual.untouched();
}
#[test]
fn actual_redo_discard_vector_counts_entries_while_oldest_undo_identity_remains_fixed() {
    let mut actual = fixture();
    let design = actual.request.context().design();
    let next = |group_id, revision, reason| {
        group(
            group_id,
            vec![mutation(
                design,
                DesignRevision::from_counter(revision),
                group_id,
                group_id + 1,
                material(actual.piece, reason),
            )],
        )
    };
    let second = next(810020, 1, " β ");
    actual
        .bus
        .commit_group(&second)
        .expect("actual second group");
    actual.actor.set(Some(super::history::human()));
    actual
        .bus
        .apply_history(&operation(
            &second,
            DesignRevision::from_counter(2),
            810030,
            Direction::Undo,
        ))
        .expect("undo second");
    actual
        .bus
        .apply_history(&operation(
            &actual.request,
            DesignRevision::from_counter(1),
            810031,
            Direction::Undo,
        ))
        .expect("undo first");
    actual.actor.set(Some(actor()));
    let branch = next(810040, 0, " γ ");
    let branched = actual
        .bus
        .commit_group(&branch)
        .expect("actual branch discards two redo snapshots");
    assert_eq!(
        branched.entry().commit().history().discarded_redo(),
        &[actual.request.id(), second.id()]
    );
    assert_eq!(branched.entry().sequence().to_string(), "5");
    assert_eq!(
        branched.entry().commit().revision(),
        &DesignRevision::from_counter(3)
    );
    actual.reset();
    let mut meter = Meter::new(limits(7, 64, 5));
    charge_result(&branched, &mut meter).expect("actual two discarded IDs");
    assert_eq!(units(meter), (7, 64, 5));
    let mut meter = Meter::new(limits(7, 64, 2));
    let expected = LimitError::CounterBits {
        maximum: 2,
        consumed: 0,
        requested: 3,
        location: Location {
            owner: Some(branch.id().entity_id()),
            ..location("records.authoring.sequence", None)
        },
    };
    assert_eq!(charge_result(&branched, &mut meter), Err(expected));
    assert_eq!(meter.finish(), Err(expected));
    let mut meter = Meter::new(limits(6, 64, 5));
    let expected = LimitError::Count {
        unit: Unit::CollectionEntries,
        maximum: 6,
        consumed: 5,
        requested: 2,
        location: Location {
            owner: Some(branch.id().entity_id()),
            ..location("records.commit.discarded_redo", None)
        },
    };
    assert_eq!(charge_result(&branched, &mut meter), Err(expected));
    assert_eq!(meter.finish(), Err(expected));
    actual.untouched();
    let fourth = next(810050, 3, " γ ");
    actual
        .bus
        .commit_group(&fourth)
        .expect("actual no-op fourth");
    let fifth = next(810060, 4, " γ ");
    let evicted = actual
        .bus
        .commit_group(&fifth)
        .expect("actual oldest undo eviction");
    assert_eq!(
        evicted.entry().commit().history().dropped_undo(),
        Some(branch.id())
    );
    assert!(evicted
        .entry()
        .commit()
        .history()
        .discarded_redo()
        .is_empty());
    actual.reset();
    let mut meter = Meter::new(limits(3, 4, 18));
    charge_result(&evicted, &mut meter).expect("fixed oldest-undo ID adds no dynamic count");
    assert_eq!(units(meter), (3, 4, 18));
    actual.untouched();
}
