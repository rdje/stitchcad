//! Declared metadata demand and phase fractions, without fabricated model/runtime effects.
#![allow(clippy::expect_used)]
use core::num::NonZeroUsize;
use sc_core::{
    command::{
        AssignMaterial, ChangeMeasurement, CommandActorClaim, CommandGroupId, CommandId,
        CommandIdempotencyKey, CommandMutation, CommandMutationContext, CommandMutationGroup,
        CommandOperationLimitError as Error, CommandOperationLimits as Limits, CommandProgress,
        CommandProgressError, CommandProgressPhase as Phase, DesignRevision, MutationPayload,
        PieceSelection,
    },
    ontology::{EntityId, MaterialAssignment},
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};

fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}
fn positive(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("explicit positive policy/count")
}
fn limits(members: usize, targets: usize, bytes: usize, evidence: usize) -> Limits {
    Limits::new(
        positive(members),
        positive(targets),
        positive(bytes),
        positive(evidence),
    )
}
fn mutation(key: u128, payload: MutationPayload) -> CommandMutation {
    CommandMutation::new(
        CommandMutationContext::new(
            id(1),
            CommandActorClaim::Human(id(2)),
            DesignRevision::initial(),
            CommandIdempotencyKey::from_entity_id(id(key)),
            Some(CommandGroupId::from_entity_id(id(3))),
        ),
        payload,
    )
}
fn original() -> CommandMutationGroup {
    let assignment = mutation(
        4,
        MutationPayload::AssignMaterial(
            AssignMaterial::new(
                PieceSelection::new(vec![id(9), id(7)]).expect("two original selections"),
                MaterialAssignment::Unresolved("é".into()),
            )
            .expect("explicit nonblank UTF-8 reason"),
        ),
    );
    let length = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(20),
        source: id(21),
        state: LengthState::Unknown {
            observation: id(22),
        },
    })
    .expect("actual immutable declaration shape");
    let measurement = mutation(
        5,
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(23), length)),
    );
    CommandMutationGroup::new(
        CommandGroupId::from_entity_id(id(3)),
        vec![assignment.clone(), measurement, assignment],
    )
    .expect("original group with an exact submitted alias")
}

#[test]
fn admission_counts_original_aliases_primary_targets_and_utf8_reason_fields_at_exact_bounds() {
    let original = original();
    let saved = original.clone();
    let policy = limits(3, 5, 8, 1);
    let demand = policy
        .check_group(&original)
        .expect("three submitted members, five target selections, eight reason bytes");
    assert_eq!(demand.members(), 3);
    assert_eq!(demand.targets(), 5);
    assert_eq!(demand.reason_bytes(), 8);
    assert_eq!(policy.members(), positive(3));
    assert_eq!(policy.targets(), positive(5));
    assert_eq!(policy.reason_bytes(), positive(8));
    assert_eq!(
        original, saved,
        "admission preserves contexts/order/alias and all payloads"
    );
    let first = original.mutations().first().expect("original assignment");
    let single = limits(1, 2, 4, 1)
        .check_mutation(first)
        .expect("single mutation demand uses the same exact units");
    assert_eq!(single.members(), 1);
    assert_eq!(single.targets(), 2);
    assert_eq!(single.reason_bytes(), 4);
}

#[test]
fn member_limit_precedes_per_member_target_and_reason_checks_without_deduplicating_aliases() {
    assert_eq!(
        limits(2, 1, 1, 1).check_group(&original()),
        Err(Error::Members {
            maximum: 2,
            requested: 3
        })
    );
}

#[test]
fn per_member_refusals_keep_exact_prior_counts_index_identity_and_unmultiplied_byte_components() {
    let original = original();
    assert_eq!(
        limits(3, 2, 8, 1).check_group(&original),
        Err(Error::Targets {
            maximum: 2,
            consumed: 2,
            requested: 1,
            index: 1,
            command: CommandId::ChangeMeasurement
        })
    );
    assert_eq!(
        limits(3, 5, 7, 1).check_group(&original),
        Err(Error::ReasonBytes {
            maximum: 7,
            consumed: 4,
            copies: 2,
            bytes_per_copy: 2,
            index: 2,
            command: CommandId::AssignMaterial
        })
    );
    let big = Limits::new(
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
        NonZeroUsize::MAX,
    )
    .check_group(&original)
    .expect("explicit platform-wide policy is preserved without narrowing");
    assert_eq!(big.reason_bytes(), 8);
}

#[test]
fn progress_preserves_phase_local_integer_fraction_and_original_member_without_clipping() {
    for completed in [0, 3] {
        let progress = CommandProgress::new(
            Phase::Preparing,
            completed,
            positive(3),
            Some((2, CommandId::AssignMaterial)),
        )
        .expect("valid inclusive phase endpoints");
        assert_eq!(progress.phase(), Phase::Preparing);
        assert_eq!(progress.completed(), completed);
        assert_eq!(progress.total(), positive(3));
        assert_eq!(progress.member(), Some((2, CommandId::AssignMaterial)));
    }
    assert_eq!(
        CommandProgress::new(Phase::Preparing, 4, positive(3), None),
        Err(CommandProgressError {
            completed: 4,
            total: positive(3)
        })
    );
    let ready = CommandProgress::new(Phase::ReadyToPublish, 1, positive(1), None)
        .expect("pre-effect phase data is not publication");
    assert_eq!(ready.member(), None);
}

#[test]
fn known_measurement_evidence_cannot_bypass_the_explicit_admission_policy() {
    let known = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(20),
        source: id(21),
        state: LengthState::Known {
            value: sc_units::Length::ZERO,
            evidence: vec![id(22), id(23)],
        },
    })
    .expect("two distinct actual submitted evidence references");
    let request = mutation(
        5,
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(24), known)),
    );
    let saved = request.clone();
    assert_eq!(
        limits(1, 1, 1, 1).check_mutation(&request),
        Err(Error::EvidenceReferences {
            maximum: 1,
            consumed: 0,
            requested: 2,
            index: 0,
            command: CommandId::ChangeMeasurement,
        })
    );
    let policy = limits(1, 1, 1, 2);
    let demand = policy
        .check_mutation(&request)
        .expect("exact independent evidence bound");
    assert_eq!(
        (demand.members(), demand.targets(), demand.reason_bytes()),
        (1, 1, 0)
    );
    assert_eq!(demand.evidence_references(), 2);
    assert_eq!(policy.evidence_references(), positive(2));
    assert_eq!(
        request, saved,
        "admission preserves the complete submitted declaration"
    );
}

#[test]
fn evidence_demand_counts_original_aliases_and_preserves_first_cumulative_excess() {
    let known = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(20),
        source: id(21),
        state: LengthState::Known {
            value: sc_units::Length::ZERO,
            evidence: vec![id(22), id(23)],
        },
    })
    .expect("explicit distinct references");
    let request = mutation(
        5,
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(24), known)),
    );
    let group = CommandMutationGroup::new(
        CommandGroupId::from_entity_id(id(3)),
        vec![request.clone(), request],
    )
    .expect("exact original alias is still submitted input");
    assert_eq!(
        limits(2, 2, 1, 3).check_group(&group),
        Err(Error::EvidenceReferences {
            maximum: 3,
            consumed: 2,
            requested: 2,
            index: 1,
            command: CommandId::ChangeMeasurement,
        })
    );
    assert_eq!(
        limits(2, 2, 1, 4)
            .check_group(&group)
            .expect("exact cumulative bound")
            .evidence_references(),
        4
    );
    assert_eq!(
        limits(1, 1, 1, 1).check_group(&original()),
        Err(Error::Members {
            maximum: 1,
            requested: 3
        })
    );
    assert_eq!(
        limits(3, 5, 8, 1)
            .check_group(&original())
            .expect("Unknown has no evidence vector")
            .evidence_references(),
        0
    );
}
