//! Actual publication revisions and independent maximum-issued counter after undo, without fake hosts.
use super::group_commit::{capacity, group, material, mutation};
use super::resource_admission::{control, counts, demand, operation_only, reset};
use super::resource_copies::CountedSources;
use super::*;
use sc_core::{
    command::{
        AssignMaterial, CommandControlError as Control, CommandHistoryDirection as Direction,
        CommandHistoryRequest, PieceSelection,
    },
    design::{
        charge_design_garment_resources, DesignInputs, DesignResourceLimitError as LimitError,
        DesignResourceLimits, DesignResourceLocation, DesignResourceMeter, DesignResourceScope,
        DesignSewing, DesignStructure,
    },
};
use std::num::{NonZeroU64, NonZeroUsize};
fn with_revision(
    before: &DesignGarment<CountedSources>,
    revision: DesignRevision,
) -> DesignGarment<CountedSources> {
    let mut garment = before.definition().clone();
    let mut sewing = garment.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.revision = revision;
    inputs.structure = DesignStructure::new(structure).expect("actual exact-revision structure");
    sewing.inputs = DesignInputs::new(inputs).expect("actual exact-revision inputs");
    garment.sewing = DesignSewing::new(sewing).expect("actual exact-revision sewing");
    let actual = DesignGarment::new(garment).expect("actual exact-revision garment");
    actual
        .validate_current()
        .expect("independently validated real counter model");
    actual
}
fn policy(bits: u64) -> DesignResourceLimits {
    DesignResourceLimits::new(
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroU64::new(bits).expect("explicit positive model bit maximum"),
    )
}
fn noop(owner: &DesignGarment<CountedSources>) -> MutationPayload {
    let piece = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual Piece");
    MutationPayload::AssignMaterial(
        AssignMaterial::new(
            PieceSelection::new(vec![piece.id()]).expect("actual exact selection"),
            piece.definition().material.clone(),
        )
        .expect("exact original assignment"),
    )
}
#[test]
fn exact_successor_bits_admit_zero_sparse_carry_and_beyond_u128_real_models() {
    for value in [0_u128, 1, 2, 3, u128::MAX] {
        let base = DesignRevision::from_counter(value);
        let owner = with_revision(&super::resource_measurement::fixture(), base.clone());
        let next = base.successor();
        let oracle = with_revision(&owner, next.clone());
        let actual = demand(&oracle);
        let limits = policy(actual.counter_bits());
        let counters = owner.inputs().sources().host().telemetry();
        let design = owner.inputs().id();
        let payload = noop(&owner);
        let request = group(
            807000,
            vec![mutation(design, base, 807000, 807001, payload)],
        );
        let mut bus = operation_only(
            CommandBus::with_edit_capacity(owner, Host::new(design), vec![], capacity(1))
                .expect("actual finite counter owner"),
        );
        bus.install_model_limits(limits)
            .expect("exact actual successor bit budget");
        reset(&counters);
        let accepted = bus
            .commit_group(&request)
            .expect("exact forecast matches actual constructed model");
        assert_eq!(accepted.entry().commit().revision(), &next);
        assert_eq!(
            counts(&counters)[0],
            2,
            "metadata candidate and admitted final revision copies only"
        );
        assert_eq!(
            counts(&counters)[1],
            3,
            "current, metadata and final whole revision projections"
        );
        let CommandResult::PieceList(result) = bus
            .execute(&query(design, next, QueryPayload::GetPieceList))
            .expect("actual published revision");
        assert!(!result.pieces().is_empty());
    }
}
#[test]
fn carry_quota_matches_actual_model_before_revision_copy_and_keeps_revision_keys_and_capacity() {
    for value in [1_u128, 3, u128::MAX] {
        let base = DesignRevision::from_counter(value);
        let owner = with_revision(&super::resource_measurement::fixture(), base.clone());
        let next = base.successor();
        let oracle = with_revision(&owner, next);
        let limits = policy(demand(&owner).counter_bits());
        let mut meter = DesignResourceMeter::new(limits);
        let expected = charge_design_garment_resources(&oracle, &mut meter)
            .expect_err("actual carry exceeds current bit maximum");
        assert_eq!(meter.finish(), Err(expected));
        let counters = owner.inputs().sources().host().telemetry();
        let design = owner.inputs().id();
        let payload = noop(&owner);
        let piece = owner
            .inputs()
            .structure()
            .pieces()
            .pieces()
            .first()
            .expect("actual Piece")
            .id();
        let address = owner.inputs().structure().pieces().pieces().as_ptr();
        let request = group(
            807010,
            vec![mutation(design, base.clone(), 807010, 807011, payload)],
        );
        let alternate = group(
            807010,
            vec![mutation(
                design,
                base.clone(),
                807010,
                807011,
                material(piece, "other valid metadata"),
            )],
        );
        let mut bus = operation_only(
            CommandBus::with_edit_capacity(owner, Host::new(design), vec![], capacity(1))
                .expect("actual finite counter owner"),
        );
        bus.install_model_limits(limits)
            .expect("current fits; next carry does not");
        for attempt in [&request, &alternate, &request] {
            reset(&counters);
            let error = bus
                .commit_group(attempt)
                .expect_err("prospective revision refused without acceptance");
            assert_eq!(control(&error), Some(&Control::Model(expected)));
            assert_eq!(
                counts(&counters)[0],
                1,
                "bounded metadata prepared; no final revision model copy"
            );
            assert_eq!(
                counts(&counters)[1],
                2,
                "carry refuses before projected host traversal"
            );
            let CommandResult::PieceList(result) = bus
                .execute(&query(design, base.clone(), QueryPayload::GetPieceList))
                .expect("original revision survives every refusal");
            assert_eq!(result.pieces().as_ptr(), address);
        }
    }
}
fn undo(
    original: &sc_core::command::CommandMutationGroup,
    current: DesignRevision,
    key: u128,
) -> CommandHistoryRequest {
    CommandHistoryRequest::new(
        CommandMutationContext::new(
            original.context().design(),
            actor(),
            current,
            CommandIdempotencyKey::from_entity_id(id(key)),
            Some(original.id()),
        ),
        original.clone(),
        Direction::Undo,
    )
    .expect("exact original accepted group and current operator context")
}
#[test]
fn undo_does_not_lower_maximum_issued_budget_and_exact_replays_survive_next_counter_exhaustion() {
    let owner = super::resource_measurement::fixture();
    let counters = owner.inputs().sources().host().telemetry();
    let design = owner.inputs().id();
    let payload = noop(&owner);
    let address = owner.inputs().structure().pieces().pieces().as_ptr();
    let mut bus = operation_only(
        CommandBus::with_edit_history(
            owner,
            Host::new(design),
            vec![],
            capacity(4),
            capacity(3),
            capacity(3),
        )
        .expect("actual finite snapshots and action records"),
    );
    bus.install_model_limits(policy(2))
        .expect("revisions0..3 fit; revision4 does not");
    let groups = [
        (0_u128, 807020, 807021),
        (1, 807022, 807023),
        (2, 807024, 807025),
    ]
    .map(|(before, group_id, key)| {
        group(
            group_id,
            vec![mutation(
                design,
                DesignRevision::from_counter(before),
                group_id,
                key,
                payload.clone(),
            )],
        )
    });
    for request in &groups {
        bus.commit_group(request)
            .expect("actual accepted revisions1/2/3");
    }
    let mut current = DesignRevision::from_counter(3);
    let mut last = None;
    for (original, key) in groups.iter().rev().zip(807030_u128..) {
        let request = undo(original, current, key);
        let accepted = bus
            .apply_history(&request)
            .expect("actual undo under fixed model policy");
        current = accepted.entry().revision().clone();
        last = Some((request, accepted));
    }
    assert_eq!(current, DesignRevision::initial());
    let branch = group(
        807040,
        vec![mutation(design, current.clone(), 807040, 807041, payload)],
    );
    let expected = LimitError::CounterBits {
        maximum: 2,
        consumed: 0,
        requested: 3,
        location: DesignResourceLocation {
            scope: DesignResourceScope::Structure,
            owner: Some(design),
            field: "design.revision",
            index: None,
        },
    };
    reset(&counters);
    let error = bus
        .commit_group(&branch)
        .expect_err("maximum-issued3 forecasts4/3bits even after restoring0");
    assert_eq!(control(&error), Some(&Control::Model(expected)));
    assert_eq!(
        counts(&counters)[0],
        1,
        "no final revision reconstruction before refusal"
    );
    let CommandResult::PieceList(result) = bus
        .execute(&query(design, current, QueryPayload::GetPieceList))
        .expect("no new authoring effect");
    assert_eq!(result.pieces().as_ptr(), address);
    reset(&counters);
    let replay = bus
        .commit_group(groups.first().expect("actual first accepted group"))
        .expect("exact authoring replay needs no new counter");
    assert!(replay.is_replay());
    assert_eq!(counts(&counters), [0, 0, 0]);
    let (request, accepted) = last.expect("actual last accepted undo");
    let replay = bus
        .apply_history(&request)
        .expect("exact history replay at action bound");
    assert!(replay.is_replay());
    assert_eq!(replay.entry(), accepted.entry());
    assert_eq!(counts(&counters), [0, 0, 0]);
}
