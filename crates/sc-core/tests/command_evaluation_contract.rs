//! Canonical path inputs remain distinct, exact and unexecuted under shared snapshot claims.
use sc_core::{
    command::{
        CommandActorClaim as Actor, CommandAuthority, CommandClass, CommandEvaluation, CommandId,
        CommandQueryContext, CommandReversibility, CommandSnapshotContext, CommandUndoGranularity,
        DesignRevision, EvaluateInstance, EvaluationPayload, EvaluationProfile, GradeSize,
        RegenerationChart, ResolveProfile, SizeMemberTarget,
    },
    ontology::{EntityId as Id, SelectedProfilePin, SelectedProfilePinDefinition},
};
use sc_units::Count;

#[test]
fn reads_and_evaluations_share_exact_snapshot_type_without_losing_actor_or_large_revision() {
    let revision = DesignRevision::from_counter(u128::MAX)
        .successor()
        .successor();
    let actor = Actor::Agent {
        identity: Id::from_bits(2),
        principal: Id::from_bits(3),
    };
    let shared = CommandSnapshotContext::new(Id::from_bits(1), actor, revision.clone());
    let legacy: &CommandQueryContext = &shared;
    assert!(std::ptr::eq(legacy, &shared));
    assert_eq!(shared.expected_revision(), &revision);
    assert_ne!(shared.expected_revision(), &revision.successor());
    assert_eq!(shared.actor().initiator(), Id::from_bits(2));
    assert_eq!(shared.actor().principal(), Id::from_bits(3));
    let target = SizeMemberTarget::new(Id::from_bits(4), Count::new(u32::MAX), Id::from_bits(5));
    let payload =
        EvaluationPayload::GradeSize(GradeSize::new(Id::from_bits(6), Id::from_bits(7), target));
    let request = CommandEvaluation::new(shared.clone(), payload);
    assert_eq!(request.context(), &shared);
    assert_eq!(request.context().design(), Id::from_bits(1));
    assert_eq!(target.revision(), Count::new(u32::MAX));
}

#[test]
fn regeneration_keeps_explicit_chart_family_and_exact_logical_or_resolved_factory_selection() {
    let selected = SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: " House/É ".to_owned(),
        schema_version: "02.0".to_owned(),
        content_version: "v1.3+authored".to_owned(),
    })
    .expect("opaque explicit profile labels");
    let resolved = EvaluationProfile::FactoryResolved {
        selected: selected.clone(),
        parameters: Id::from_bits(30),
    };
    for profile in [EvaluationProfile::Logical, resolved] {
        for chart in [
            RegenerationChart::Garment(Id::from_bits(20)),
            RegenerationChart::Mtm(Id::from_bits(20)),
        ] {
            let target = SizeMemberTarget::new(Id::from_bits(10), Count::new(0), Id::from_bits(11));
            let input = EvaluateInstance::new(target, chart, profile.clone());
            assert_eq!(input.target().size_set(), Id::from_bits(10));
            assert_eq!(input.target().member(), Id::from_bits(11));
            assert_eq!(input.chart(), chart);
            assert_eq!(input.profile(), &profile);
            if let EvaluationProfile::FactoryResolved {
                selected: actual,
                parameters,
            } = input.profile()
            {
                assert_eq!(actual, &selected);
                assert_eq!(*parameters, Id::from_bits(30));
            }
            assert_evaluation(
                EvaluationPayload::EvaluateInstance(input),
                CommandId::EvaluateInstance,
            );
        }
    }
}

#[test]
fn grading_preserves_imported_base_rules_and_target_break_snapshot_without_recipe_inputs() {
    let target = SizeMemberTarget::new(Id::from_bits(40), Count::new(7), Id::from_bits(41));
    let input = GradeSize::new(Id::from_bits(50), Id::from_bits(60), target);
    assert_eq!(input.base_instance(), Id::from_bits(50));
    assert_eq!(input.rule_table(), Id::from_bits(60));
    assert_eq!(input.target(), target);
    let changed = GradeSize::new(
        Id::from_bits(50),
        Id::from_bits(60),
        SizeMemberTarget::new(Id::from_bits(40), Count::new(8), Id::from_bits(41)),
    );
    assert_ne!(input, changed);
    assert_evaluation(EvaluationPayload::GradeSize(input), CommandId::GradeSize);
}

fn assert_evaluation(payload: EvaluationPayload, id: CommandId) {
    let context = CommandSnapshotContext::new(
        Id::from_bits(1),
        Actor::Human(Id::from_bits(3)),
        DesignRevision::initial(),
    );
    let request = CommandEvaluation::new(context.clone(), payload.clone());
    assert_eq!(request.payload(), &payload);
    assert_eq!(request.context(), &context);
    assert_eq!(request.command_id(), id);
    assert_eq!(id.class(), CommandClass::Evaluation);
    assert_eq!(id.required_authority(), CommandAuthority::Generate);
    assert_eq!(id.reversibility(), CommandReversibility::Discarded);
    assert_eq!(id.undo_granularity(), CommandUndoGranularity::NotApplicable);
}

#[test]
fn profile_resolution_retains_exact_selection_and_uses_evaluation_snapshot_metadata() {
    let authored = SelectedProfilePinDefinition {
        profile_id: " House/É ".to_owned(),
        schema_version: "02.0".to_owned(),
        content_version: "content+001".to_owned(),
    };
    let pin = SelectedProfilePin::new(authored.clone()).expect("authored profile labels");
    let input = ResolveProfile::new(pin.clone());
    assert_eq!(input.selected(), &pin);
    assert_eq!(input.selected().definition(), &authored);
    let changed = ResolveProfile::new(
        SelectedProfilePin::new(SelectedProfilePinDefinition {
            content_version: "content+002".to_owned(),
            ..authored
        })
        .expect("next explicit selection"),
    );
    assert_ne!(input, changed);
    assert_evaluation(
        EvaluationPayload::ResolveProfile(input),
        CommandId::ResolveProfile,
    );
}
