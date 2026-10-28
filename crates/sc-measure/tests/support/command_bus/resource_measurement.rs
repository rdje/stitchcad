//! Prospective borrowed evidence versus independently reconstructed actual complete model accounting.
use super::group_commit::{capacity, group, mutation};
use super::resource_admission::{control, counts, demand, reset, without_model_limits};
use super::resource_copies::{owner, CountedSources};
use super::*;
use sc_core::{
    command::{ChangeMeasurement, CommandControlError as Control, CommandPreview},
    design::{
        charge_design_garment_resources, DesignInputs, DesignResourceLimits, DesignResourceMeter,
        DesignSewing, DesignStructure,
    },
    value::{LengthDeclaration, LengthState},
};
use std::{error::Error as StdError, num::NonZeroU64};
fn replacement(owner: &DesignGarment<CountedSources>, state: LengthState) -> LengthDeclaration {
    let mut definition = owner
        .inputs()
        .sources()
        .measurement_declaration(id(1), owner.inputs().inputs())
        .expect("actual Measurement binding")
        .definition()
        .clone();
    definition.state = state;
    LengthDeclaration::new(definition).expect("structurally valid actual replacement")
}
fn known(evidence: &[u128]) -> LengthState {
    LengthState::Known {
        value: sc_units::Length::from_micrometres(1000)
            .expect("actual positive shared Material width"),
        evidence: evidence.iter().copied().map(id).collect(),
    }
}
// This oracle constructs and validates a real complete model through its public contracts. Runtime
// projection must refuse before this work, but the test can independently inspect the actual result.
fn rebuilt(
    before: &DesignGarment<CountedSources>,
    replacement: LengthDeclaration,
) -> DesignGarment<CountedSources> {
    let mut garment = before.definition().clone();
    let mut sewing = garment.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.inputs = structure
        .inputs
        .with_length_declaration(replacement)
        .expect("actual target replacement");
    inputs.structure = DesignStructure::new(structure).expect("actual replacement structure");
    sewing.inputs = DesignInputs::new(inputs).expect("actual replacement input owner");
    garment.sewing = DesignSewing::new(sewing).expect("actual replacement sewing owner");
    let actual = DesignGarment::new(garment).expect("actual replacement garment");
    actual
        .validate_current()
        .expect("independent actual current-model oracle");
    actual
}
pub(super) fn fixture() -> DesignGarment<CountedSources> {
    let (original, _) = owner(Rc::new(Cell::new(0)));
    rebuilt(&original, replacement(&original, known(&[804100, 804101])))
}
fn exact_limits(owner: &DesignGarment<CountedSources>) -> DesignResourceLimits {
    let actual = demand(owner);
    DesignResourceLimits::new(
        capacity(actual.collection_entries()),
        capacity(actual.text_bytes()),
        NonZeroU64::MAX,
    )
}
#[test]
fn growing_evidence_refuses_before_model_copy_and_failed_group_keys_remain_available() {
    let owner = fixture();
    let limits = exact_limits(&owner);
    let base_demand = demand(&owner);
    let growing = replacement(&owner, known(&[804110, 804111, 804112]));
    let oracle = rebuilt(&owner, growing.clone());
    assert_eq!(
        demand(&oracle).collection_entries(),
        base_demand.collection_entries() + 1
    );
    let mut meter = DesignResourceMeter::new(limits);
    let expected = charge_design_garment_resources(&oracle, &mut meter)
        .expect_err("actual projected +1 entry");
    assert_eq!(meter.finish(), Err(expected));
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let address = owner.inputs().structure().pieces().pieces().as_ptr();
    let original = replacement(&owner, known(&[804100, 804101]));
    let member = mutation(
        design,
        revision.clone(),
        804000,
        804001,
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), growing)),
    );
    let request = group(804000, vec![member.clone()]);
    let mut bus = without_model_limits(
        CommandBus::with_edit_capacity(owner, Host::new(design), vec![], capacity(1))
            .expect("actual finite owner"),
    );
    bus.install_model_limits(limits)
        .expect("exact finite current-model budget");
    for path in 0..3 {
        reset(&counters);
        let error: Box<dyn StdError> = match path {
            0 => Box::new(
                bus.preview(&CommandRequest::Mutation(member.clone()))
                    .expect_err("single prospective growth"),
            ),
            1 => Box::new(
                bus.preview_group(&request)
                    .expect_err("group prospective growth"),
            ),
            _ => Box::new(
                bus.commit_group(&request)
                    .expect_err("commit prospective growth"),
            ),
        };
        assert_eq!(control(error.as_ref()), Some(&Control::Model(expected)));
        let telemetry = counts(&counters);
        assert_eq!(
            telemetry[0], 0,
            "no candidate host clone before projected refusal"
        );
        assert_eq!(
            telemetry[1], 3,
            "retained, current and exact prospective resource walks"
        );
        assert!(
            telemetry[2] > 0,
            "original current validation and actual binding precede projection"
        );
        let CommandResult::PieceList(result) = bus
            .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
            .expect("unchanged original revision");
        assert_eq!(result.pieces().as_ptr(), address);
    }
    let changed = replacement_from(&original, known(&[804120, 804121]));
    let valid = group(
        804000,
        vec![mutation(
            design,
            revision,
            804000,
            804001,
            MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), changed)),
        )],
    );
    let accepted = bus
        .commit_group(&valid)
        .expect("refused group/key/capacity remain available");
    assert!(!accepted.is_replay());
}
fn replacement_from(original: &LengthDeclaration, state: LengthState) -> LengthDeclaration {
    let mut definition = original.definition().clone();
    definition.state = state;
    LengthDeclaration::new(definition).expect("actual alternate state")
}
#[test]
fn exact_noop_same_size_replacement_and_unknown_shrink_use_resulting_owned_evidence_only() {
    for (state, entries_delta, changes) in [
        (known(&[804100, 804101]), 0, 0),
        (known(&[804130, 804131]), 0, 1),
        (
            LengthState::Unknown {
                observation: id(804132),
            },
            2,
            1,
        ),
    ] {
        let owner = fixture();
        let original_demand = demand(&owner);
        let limits = exact_limits(&owner);
        let after = replacement(&owner, state);
        let oracle = rebuilt(&owner, after.clone());
        let projected = demand(&oracle);
        assert_eq!(
            projected.collection_entries(),
            original_demand.collection_entries() - entries_delta
        );
        assert_eq!(projected.text_bytes(), original_demand.text_bytes());
        assert_eq!(projected.counter_bits(), original_demand.counter_bits());
        let counters = owner.inputs().sources().host().telemetry();
        let design = owner.inputs().id();
        let revision = owner.inputs().revision().clone();
        let request = CommandRequest::Mutation(mutation(
            design,
            revision.clone(),
            804010,
            804011,
            MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), after.clone())),
        ));
        let mut bus = without_model_limits(CommandBus::new(owner, Host::new(design)));
        bus.install_model_limits(limits)
            .expect("exact original-size policy");
        reset(&counters);
        let report = match bus.preview(&request).expect("no old+new double charge") {
            CommandPreview::Measurement(report) => Some(report),
            _ => None,
        }
        .expect("typed Measurement report");
        assert_eq!(report.changes().len(), changes);
        if let Some(change) = report.changes().first() {
            assert_eq!(change.after(), &after);
        }
        let telemetry = counts(&counters);
        assert_eq!(telemetry[0], 1, "one admitted actual candidate source copy");
        assert_eq!(
            telemetry[1], 3,
            "retained, current and prospective source resource walks"
        );
        bus.execute(&query(design, revision, QueryPayload::GetPieceList))
            .expect("preview leaves original revision");
    }
}
#[test]
fn sequential_evidence_projection_uses_actual_staged_state_and_aliases_do_not_repeat_work() {
    let owner = fixture();
    let limits = exact_limits(&owner);
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let original = replacement(&owner, known(&[804100, 804101]));
    let unknown = replacement_from(
        &original,
        LengthState::Unknown {
            observation: id(804140),
        },
    );
    let first = mutation(
        design,
        revision.clone(),
        804020,
        804021,
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), unknown)),
    );
    let restore = mutation(
        design,
        revision.clone(),
        804020,
        804022,
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), original)),
    );
    let request = group(804020, vec![first, restore.clone(), restore]);
    let mut bus = without_model_limits(CommandBus::new(owner, Host::new(design)));
    bus.install_model_limits(limits)
        .expect("exact original-size policy");
    reset(&counters);
    let report = bus
        .preview_group(&request)
        .expect("shrink then restore exact actual owned evidence");
    assert!(report.difference().lengths().is_empty());
    assert!(report.difference().materials().is_empty());
    assert!(report.difference().notches().is_empty());
    assert_eq!(
        report.steps().last().expect("original alias").replay_of(),
        Some(1)
    );
    let telemetry = counts(&counters);
    assert_eq!(
        telemetry[0], 2,
        "one admitted candidate copy for each distinct member"
    );
    assert_eq!(
        telemetry[1], 5,
        "one retained walk plus current/prospective for each distinct staged base"
    );
    bus.execute(&query(design, revision, QueryPayload::GetPieceList))
        .expect("group preview leaves original revision");
}
