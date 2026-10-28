//! Real mixed preview reports; owned step/alias/difference copies are not deduplicated.
use super::group_commit::{group, mutation};
use super::resource_admission::{counts, reset};
use super::*;
use sc_core::{
    command::{
        charge_command_group_preview_resources, charge_command_preview_resources, AddNotch,
        AssignMaterial, ChangeMeasurement, CommandGroupPreview, PieceSelection,
    },
    design::{
        DesignInputs, DesignResourceCount as Unit, DesignResourceLimitError as LimitError,
        DesignResourceLimits, DesignResourceLocation as Location, DesignResourceMeter as Meter,
        DesignResourceScope as Scope, DesignResourceSink, DesignSewing, DesignStructure,
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
fn location(owner: EntityId, field: &'static str, index: Option<usize>) -> Location {
    Location {
        scope: Scope::CommandState,
        owner: Some(owner),
        field,
        index,
    }
}
fn units(meter: Meter) -> (usize, usize, u64) {
    let demand = meter.finish().expect("complete actual demand");
    (
        demand.collection_entries(),
        demand.text_bytes(),
        demand.counter_bits(),
    )
}
struct Actual {
    report: CommandGroupPreview,
    counters: [Rc<Cell<usize>>; 3],
    authority_reads: Rc<Cell<usize>>,
}
fn fixture(restore: bool) -> Actual {
    let original = super::resource_measurement::fixture();
    let mut garment = original.definition().clone();
    let mut sewing = garment.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.revision = DesignRevision::from_counter(3);
    assert!(
        inputs.materials.material(id(220000)).is_some(),
        "actual available Material"
    );
    structure.pieces = sc_core::ontology::PieceInventory::new(
        structure
            .pieces
            .pieces()
            .iter()
            .map(|piece| {
                piece
                    .with_material_assignment(MaterialAssignment::Assigned(id(220000)))
                    .expect("actual assigned Piece")
            })
            .collect(),
    )
    .expect("actual ordered assigned inventory");
    inputs.structure = DesignStructure::new(structure).expect("actual revision3 structure");
    sewing.inputs = DesignInputs::new(inputs).expect("actual inputs");
    garment.sewing = DesignSewing::new(sewing).expect("actual sewing");
    let owner = DesignGarment::new(garment).expect("actual revision3 model");
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let canonical = owner
        .inputs()
        .sources()
        .measurement_declaration(id(1), owner.inputs().inputs())
        .expect("actual bound Known2")
        .clone();
    let mut definition = canonical.definition().clone();
    let known = match definition.state {
        LengthState::Known { value, .. } => Some(value),
        _ => None,
    }
    .expect("actual Known width");
    definition.state = LengthState::Known {
        value: known,
        evidence: vec![id(809100), id(809101), id(809102)],
    };
    let replacement = LengthDeclaration::new(definition).expect("actual Known3 snapshot");
    let pieces = owner.inputs().structure().pieces().pieces();
    let address = pieces.as_ptr();
    assert_eq!(pieces.len(), 2);
    let first = pieces.first().expect("actual Piece");
    let original_assignment = first.definition().material.clone();
    assert!(matches!(
        original_assignment,
        MaterialAssignment::Assigned(_)
    ));
    assert!(pieces
        .iter()
        .all(|p| p.definition().material == original_assignment));
    let selected = pieces.iter().rev().map(|p| p.id()).collect::<Vec<_>>();
    let addition = AddNotch::new(
        first.id(),
        EdgeAnchor {
            edge: first.edges().next().expect("actual edge").edge,
            param: Param::START,
        },
        notch_preview::bindings(),
    );
    let measure = mutation(
        design,
        revision.clone(),
        809000,
        809001,
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), replacement)),
    );
    let assignment = |material| {
        MutationPayload::AssignMaterial(
            AssignMaterial::new(
                PieceSelection::new(selected.clone()).expect("actual reversed selection"),
                material,
            )
            .expect("actual explicit assignment"),
        )
    };
    let mut members = vec![
        measure.clone(),
        mutation(
            design,
            revision.clone(),
            809000,
            809002,
            assignment(MaterialAssignment::Unresolved(" é ".into())),
        ),
        mutation(
            design,
            revision.clone(),
            809000,
            809003,
            MutationPayload::AddNotch(addition),
        ),
        measure,
        mutation(
            design,
            revision.clone(),
            809000,
            809004,
            assignment(MaterialAssignment::Unresolved(" β ".into())),
        ),
    ];
    if restore {
        members.push(mutation(
            design,
            revision.clone(),
            809000,
            809005,
            MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), canonical)),
        ));
        members.push(mutation(
            design,
            revision.clone(),
            809000,
            809006,
            assignment(original_assignment.clone()),
        ));
    }
    let request = group(809000, members);
    let host = Host::with_permissions(design, vec![Permission::Inspect]);
    let authority_reads = Rc::clone(&host.reads);
    let bus = controlled(
        CommandBus::with_identities(owner, host, vec![id(809200)]).expect("actual private reserve"),
    );
    let report = bus
        .preview_group(&request)
        .expect("real validated mixed sequential report");
    let CommandResult::PieceList(current) = bus
        .execute(&query(design, revision, QueryPayload::GetPieceList))
        .expect("actual unchanged model");
    assert_eq!(current.pieces().as_ptr(), address);
    assert!(current
        .pieces()
        .iter()
        .all(|p| p.definition().material == original_assignment));
    reset(&counters);
    authority_reads.set(0);
    Actual {
        report,
        counters,
        authority_reads,
    }
}
#[test]
fn exact_reports_charge_before_after_alias_and_net_copies_without_model_or_authority_calls() {
    let actual = fixture(false);
    let original = actual.report.clone();
    let mut meter = Meter::new(limits(30, 32, 12));
    charge_command_group_preview_resources(&actual.report, &mut meter)
        .expect("actual full group report");
    assert_eq!(units(meter), (30, 32, 12));
    assert_eq!(
        actual.report.steps().get(3).expect("alias").replay_of(),
        Some(0)
    );
    let expected = [(6, 0, 2), (2, 8, 2), (0, 0, 2), (6, 0, 2), (2, 16, 2)];
    for (step, expected) in actual.report.steps().iter().zip(expected) {
        let mut meter = Meter::new(limits(30, 32, 12));
        charge_command_preview_resources(step.report(), &mut meter)
            .expect("actual owned single report");
        assert_eq!(units(meter), expected);
    }
    assert_eq!(actual.report, original);
    assert_eq!(counts(&actual.counters), [0, 0, 0]);
    assert_eq!(actual.authority_reads.get(), 0);
}
#[test]
fn report_first_failures_keep_actual_group_member_and_difference_positions() {
    let actual = fixture(false);
    let owner = actual.report.group().entity_id();
    for (maximum, consumed, requested, field, index) in [
        (4, 0, 5, "reports.steps", None),
        (7, 6, 2, "reports.measurement.before.evidence", Some(0)),
        (10, 8, 3, "reports.measurement.after.evidence", Some(0)),
        (18, 16, 3, "reports.measurement.after.evidence", Some(3)),
        (
            26,
            24,
            3,
            "reports.difference.lengths.after.evidence",
            Some(0),
        ),
        (28, 27, 2, "reports.difference.materials", None),
        (29, 29, 1, "reports.difference.notches", None),
    ] {
        let expected = LimitError::Count {
            unit: Unit::CollectionEntries,
            maximum,
            consumed,
            requested,
            location: location(owner, field, index),
        };
        let mut meter = Meter::new(limits(maximum, 32, 12));
        assert_eq!(
            charge_command_group_preview_resources(&actual.report, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
    }
    for (maximum, consumed, field, index) in [
        (3, 0, "reports.material.after.reason", Some(1)),
        (15, 12, "reports.material.after.reason", Some(4)),
        (27, 24, "reports.difference.materials.after.reason", Some(0)),
    ] {
        let expected = LimitError::Count {
            unit: Unit::TextBytes,
            maximum,
            consumed,
            requested: 4,
            location: location(owner, field, index),
        };
        let mut meter = Meter::new(limits(30, maximum, 12));
        assert_eq!(
            charge_command_group_preview_resources(&actual.report, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
    }
    let expected = LimitError::CounterBits {
        maximum: 11,
        consumed: 10,
        requested: 2,
        location: location(owner, "reports.material.revision", Some(4)),
    };
    let mut meter = Meter::new(limits(30, 32, 11));
    assert_eq!(
        charge_command_group_preview_resources(&actual.report, &mut meter),
        Err(expected)
    );
    assert_eq!(meter.finish(), Err(expected));
    let step = actual
        .report
        .steps()
        .get(4)
        .expect("two actual before/after reasons");
    let mut meter = Meter::new(limits(2, 11, 2));
    let expected = LimitError::Count {
        unit: Unit::TextBytes,
        maximum: 11,
        consumed: 8,
        requested: 4,
        location: location(
            actual.report.design(),
            "reports.material.before.reason",
            Some(1),
        ),
    };
    assert_eq!(
        charge_command_preview_resources(step.report(), &mut meter),
        Err(expected)
    );
    assert_eq!(meter.finish(), Err(expected));
    assert_eq!(counts(&actual.counters), [0, 0, 0]);
    assert_eq!(actual.authority_reads.get(), 0);
}
#[test]
fn report_prefix_and_latched_errors_compose_without_clearing_or_restarting_the_sink() {
    let actual = fixture(false);
    let prior = location(actual.report.design(), "prior", None);
    let mut meter = Meter::new(limits(32, 34, 13));
    meter.charge_entries(2, prior).expect("prior entries");
    meter.charge_text("é", prior).expect("prior bytes");
    meter.charge_counter_bits(1, prior).expect("prior bit");
    charge_command_group_preview_resources(&actual.report, &mut meter).expect("same sink");
    assert_eq!(units(meter), (32, 34, 13));
    let mut refused = Meter::new(limits(1, 1, 1));
    let first = refused
        .charge_entries(2, prior)
        .expect_err("original prior refusal");
    assert_eq!(
        charge_command_group_preview_resources(&actual.report, &mut refused),
        Err(first)
    );
    for step in actual.report.steps() {
        assert_eq!(
            charge_command_preview_resources(step.report(), &mut refused),
            Err(first)
        );
    }
    assert_eq!(refused.finish(), Err(first));
    assert_eq!(counts(&actual.counters), [0, 0, 0]);
    assert_eq!(actual.authority_reads.get(), 0);
}
#[test]
fn restored_net_values_do_not_remove_owned_step_evidence_or_reason_snapshots() {
    let actual = fixture(true);
    assert!(actual.report.difference().lengths().is_empty());
    assert!(actual.report.difference().materials().is_empty());
    assert_eq!(actual.report.difference().notches().len(), 1);
    let mut meter = Meter::new(limits(32, 32, 16));
    charge_command_group_preview_resources(&actual.report, &mut meter)
        .expect("real restored net report");
    assert_eq!(units(meter), (32, 32, 16));
    assert_eq!(counts(&actual.counters), [0, 0, 0]);
    assert_eq!(actual.authority_reads.get(), 0);
}
#[test]
fn actual_noop_and_unknown_reports_keep_empty_vectors_and_zero_revision_without_inferred_data() {
    let owner = super::resource_measurement::fixture();
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let canonical = owner
        .inputs()
        .sources()
        .measurement_declaration(id(1), owner.inputs().inputs())
        .expect("actual Known2")
        .clone();
    let mut definition = canonical.definition().clone();
    definition.state = LengthState::Unknown {
        observation: id(809399),
    };
    let unknown = LengthDeclaration::new(definition).expect("actual explicit Unknown");
    let request = group(
        809300,
        vec![
            mutation(
                design,
                revision.clone(),
                809300,
                809301,
                MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), canonical)),
            ),
            mutation(
                design,
                revision,
                809300,
                809302,
                MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), unknown)),
            ),
        ],
    );
    let host = Host::with_permissions(design, vec![Permission::Inspect]);
    let reads = Rc::clone(&host.reads);
    let bus = controlled(CommandBus::new(owner, host));
    let report = bus
        .preview_group(&request)
        .expect("actual no-op then Unknown group");
    reset(&counters);
    reads.set(0);
    let mut meter = Meter::new(limits(8, 1, 1));
    charge_command_group_preview_resources(&report, &mut meter)
        .expect("only actual Known2 before copies");
    assert_eq!(units(meter), (8, 0, 0));
    for (step, expected) in report.steps().iter().zip([(0, 0, 0), (3, 0, 0)]) {
        let mut meter = Meter::new(limits(3, 1, 1));
        charge_command_preview_resources(step.report(), &mut meter)
            .expect("actual empty/Unknown report");
        assert_eq!(units(meter), expected);
    }
    assert_eq!(counts(&counters), [0, 0, 0]);
    assert_eq!(reads.get(), 0);
}
