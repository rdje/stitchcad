//! Actual selected Piece text versus independently rebuilt validated whole-model resource oracle.
use super::group_commit::{capacity, group, mutation};
use super::resource_admission::{control, counts, demand, reset, without_model_limits};
use super::resource_copies::{owner, CountedSources};
use super::*;
use sc_core::{
    command::{
        AssignMaterial, CommandControlError as Control, CommandPreview, CommandPreviewError,
        PieceSelection,
    },
    design::{
        charge_design_garment_resources, DesignInputs, DesignResourceLimits, DesignResourceMeter,
        DesignSewing, DesignStructure,
    },
    ontology::{MaterialAssignment, PieceInventory},
};
use std::{error::Error as StdError, num::NonZeroU64};
fn rebuilt(
    before: &DesignGarment<CountedSources>,
    selected: &[EntityId],
    assignment: &MaterialAssignment,
) -> DesignGarment<CountedSources> {
    let mut garment = before.definition().clone();
    let mut sewing = garment.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.pieces = PieceInventory::new(
        structure
            .pieces
            .pieces()
            .iter()
            .map(|piece| {
                if selected.contains(&piece.id()) {
                    piece
                        .with_material_assignment(assignment.clone())
                        .expect("actual selected assignment")
                } else {
                    piece.clone()
                }
            })
            .collect(),
    )
    .expect("actual assigned Piece inventory");
    inputs.structure = DesignStructure::new(structure).expect("actual assignment structure");
    sewing.inputs = DesignInputs::new(inputs).expect("actual assignment input owner");
    garment.sewing = DesignSewing::new(sewing).expect("actual assignment sewing owner");
    let actual = DesignGarment::new(garment).expect("actual assignment garment");
    actual
        .validate_current()
        .expect("independent actual full-model oracle");
    actual
}
fn fixture() -> (DesignGarment<CountedSources>, [EntityId; 2]) {
    let (original, _) = owner(Rc::new(Cell::new(0)));
    let pieces = original.inputs().structure().pieces().pieces();
    let first = pieces.first().expect("actual first Piece").id();
    let second = pieces.get(1).expect("actual second Piece").id();
    let first_owner = rebuilt(
        &original,
        &[first],
        &MaterialAssignment::Unresolved(" é ".into()),
    );
    (
        rebuilt(
            &first_owner,
            &[second],
            &MaterialAssignment::Unresolved("猫".into()),
        ),
        [first, second],
    )
}
fn exact_limits(owner: &DesignGarment<CountedSources>) -> DesignResourceLimits {
    let actual = demand(owner);
    DesignResourceLimits::new(
        capacity(actual.collection_entries()),
        capacity(actual.text_bytes()),
        NonZeroU64::MAX,
    )
}
fn payload(selected: Vec<EntityId>, assignment: MaterialAssignment) -> MutationPayload {
    MutationPayload::AssignMaterial(
        AssignMaterial::new(
            PieceSelection::new(selected).expect("explicit distinct authored selection"),
            assignment,
        )
        .expect("explicit nonblank assignment"),
    )
}
#[test]
fn selected_utf8_growth_refuses_before_any_candidate_copy_and_keeps_keys_capacity_and_originals() {
    let (owner, [first, second]) = fixture();
    let limits = exact_limits(&owner);
    let original_demand = demand(&owner);
    let proposed = MaterialAssignment::Unresolved("猫猫".into());
    assert_eq!(" é ".len() + "猫".len(), 7);
    assert_eq!("猫猫".len() * 2, 12);
    let oracle = rebuilt(&owner, &[second, first], &proposed);
    assert_eq!(
        demand(&oracle).text_bytes(),
        original_demand.text_bytes() + 5
    );
    assert_eq!(
        demand(&oracle).collection_entries(),
        original_demand.collection_entries()
    );
    let mut meter = DesignResourceMeter::new(limits);
    let expected = charge_design_garment_resources(&oracle, &mut meter)
        .expect_err("actual +5-byte projected model");
    assert_eq!(meter.finish(), Err(expected));
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let address = owner.inputs().structure().pieces().pieces().as_ptr();
    let originals = owner.inputs().structure().pieces().pieces().to_vec();
    let member = mutation(
        design,
        revision.clone(),
        805000,
        805001,
        payload(vec![second, first], proposed),
    );
    let request = group(805000, vec![member.clone()]);
    let mut bus = without_model_limits(
        CommandBus::with_edit_capacity(owner, Host::new(design), vec![], capacity(1))
            .expect("actual finite original owner"),
    );
    bus.install_model_limits(limits)
        .expect("exact finite original model budget");
    for path in 0..3 {
        reset(&counters);
        let error: Box<dyn StdError> = match path {
            0 => Box::new(
                bus.preview(&CommandRequest::Mutation(member.clone()))
                    .expect_err("single text growth"),
            ),
            1 => Box::new(bus.preview_group(&request).expect_err("group text growth")),
            _ => Box::new(bus.commit_group(&request).expect_err("commit text growth")),
        };
        assert_eq!(control(error.as_ref()), Some(&Control::Model(expected)));
        assert_eq!(
            counts(&counters)[0],
            0,
            "no candidate host copy before refusal"
        );
        let CommandResult::PieceList(result) = bus
            .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
            .expect("original revision survives refusal");
        assert_eq!(result.pieces().as_ptr(), address);
        assert_eq!(result.pieces(), originals);
    }
    let valid = group(
        805000,
        vec![mutation(
            design,
            revision,
            805000,
            805001,
            payload(
                vec![second, first],
                MaterialAssignment::Unresolved("猫".into()),
            ),
        )],
    );
    let accepted = bus
        .commit_group(&valid)
        .expect("failed group/key/capacity remain available");
    assert!(!accepted.is_replay());
}
#[test]
fn only_selected_resulting_reasons_count_for_noop_same_byte_shrink_and_assigned_material() {
    for case in 0..4 {
        let (owner, [first, second]) = fixture();
        let actual = demand(&owner);
        let limits = exact_limits(&owner);
        let assigned = owner
            .inputs()
            .materials()
            .materials()
            .first()
            .expect("actual registry Material")
            .id();
        let (selected, assignment, removed, changes) = match case {
            0 => (
                vec![first],
                MaterialAssignment::Unresolved(" é ".into()),
                0,
                0,
            ),
            1 => (
                vec![first],
                MaterialAssignment::Unresolved("éé".into()),
                0,
                1,
            ),
            2 => (
                vec![first],
                MaterialAssignment::Unresolved("猫".into()),
                1,
                1,
            ),
            _ => (
                vec![second, first],
                MaterialAssignment::Assigned(assigned),
                7,
                2,
            ),
        };
        let oracle = rebuilt(&owner, &selected, &assignment);
        let projected = demand(&oracle);
        assert_eq!(projected.text_bytes(), actual.text_bytes() - removed);
        assert_eq!(projected.collection_entries(), actual.collection_entries());
        assert_eq!(projected.counter_bits(), actual.counter_bits());
        let counters = owner.inputs().sources().host().telemetry();
        let design = owner.inputs().id();
        let revision = owner.inputs().revision().clone();
        let originals = owner.inputs().structure().pieces().pieces().to_vec();
        let request = CommandRequest::Mutation(mutation(
            design,
            revision.clone(),
            805010,
            805011,
            payload(selected.clone(), assignment.clone()),
        ));
        let mut bus = without_model_limits(CommandBus::new(owner, Host::new(design)));
        bus.install_model_limits(limits)
            .expect("exact finite original model budget");
        reset(&counters);
        let report = match bus
            .preview(&request)
            .expect("no old+new or unselected double count")
        {
            CommandPreview::Material(report) => Some(report),
            _ => None,
        }
        .expect("typed actual Material preview");
        assert_eq!(report.changes().len(), changes);
        for (change, target) in report.changes().iter().zip(selected) {
            assert_eq!(change.piece(), target);
            assert_eq!(change.after(), &assignment);
        }
        let telemetry = counts(&counters);
        assert_eq!(telemetry[0], 1);
        assert_eq!(telemetry[1], 3);
        let CommandResult::PieceList(result) = bus
            .execute(&query(design, revision, QueryPayload::GetPieceList))
            .expect("preview keeps original model");
        assert_eq!(result.pieces(), originals);
    }
}
#[test]
fn later_missing_selected_piece_keeps_original_index_before_any_prospective_walk_or_copy() {
    let (owner, [first, _]) = fixture();
    let limits = exact_limits(&owner);
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let request = CommandRequest::Mutation(mutation(
        design,
        revision,
        805020,
        805021,
        payload(
            vec![first, id(805999)],
            MaterialAssignment::Unresolved("猫猫".into()),
        ),
    ));
    let mut bus = without_model_limits(CommandBus::new(owner, Host::new(design)));
    bus.install_model_limits(limits)
        .expect("exact finite original model budget");
    reset(&counters);
    let error = bus
        .preview(&request)
        .expect_err("actual later missing target precedes projection");
    assert!(
        matches!(error,CommandPreviewError::MissingPiece { piece,index:1 } if piece==id(805999))
    );
    let telemetry = counts(&counters);
    assert_eq!(telemetry[0], 0);
    assert_eq!(telemetry[1], 2);
}
#[test]
fn sequential_material_projection_uses_actual_staged_reason_and_aliases_do_not_repeat_work() {
    let (owner, [first, _]) = fixture();
    let limits = exact_limits(&owner);
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let shorter = mutation(
        design,
        revision.clone(),
        805030,
        805031,
        payload(vec![first], MaterialAssignment::Unresolved("猫".into())),
    );
    let restore = mutation(
        design,
        revision.clone(),
        805030,
        805032,
        payload(vec![first], MaterialAssignment::Unresolved(" é ".into())),
    );
    let request = group(805030, vec![shorter, restore.clone(), restore]);
    let mut bus = without_model_limits(CommandBus::new(owner, Host::new(design)));
    bus.install_model_limits(limits)
        .expect("exact finite original model budget");
    reset(&counters);
    let report = bus
        .preview_group(&request)
        .expect("stage actual shorter then restored owned reason");
    assert!(report.difference().lengths().is_empty());
    assert!(report.difference().materials().is_empty());
    assert!(report.difference().notches().is_empty());
    assert_eq!(
        report.steps().last().expect("original alias").replay_of(),
        Some(1)
    );
    let telemetry = counts(&counters);
    assert_eq!(telemetry[0], 2);
    assert_eq!(telemetry[1], 5);
    bus.execute(&query(design, revision, QueryPayload::GetPieceList))
        .expect("preview keeps original revision");
}
