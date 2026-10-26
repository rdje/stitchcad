//! Canonical authored/evaluated sources remain exact, without synthesized report inputs or instances.
use sc_core::{
    command::{
        ArtifactPayload, CommandActorClaim, CommandArtifact, CommandAuthority, CommandClass,
        CommandId, CommandQuery, CommandSnapshotContext, DesignRevision, EvaluationProfile,
        GenerateTechPack, InstanceSelection, InstanceSelectionError, QueryPayload,
    },
    ontology::{EntityId as Id, SelectedProfilePin, SelectedProfilePinDefinition},
};

#[test]
fn evaluated_source_selection_retains_authored_order_and_first_duplicate_context() {
    let inputs = vec![Id::from_bits(u128::MAX), Id::from_bits(3)];
    let selection = InstanceSelection::new(inputs.clone()).expect("actual source claims");
    assert_eq!(selection.instances(), inputs);
    assert_eq!(
        InstanceSelection::new(vec![]),
        Err(InstanceSelectionError::NoInstances)
    );
    assert_eq!(
        InstanceSelection::new(vec![
            Id::from_bits(3),
            Id::from_bits(4),
            Id::from_bits(3),
            Id::from_bits(4)
        ]),
        Err(InstanceSelectionError::DuplicateInstance {
            instance: Id::from_bits(3),
            first: 0,
            duplicate: 2
        })
    );
}

#[test]
fn actual_design_instances_profile_and_report_order_are_identical_in_generation_and_closure() {
    let profiles = [
        EvaluationProfile::Logical,
        EvaluationProfile::FactoryResolved {
            selected: SelectedProfilePin::new(SelectedProfilePinDefinition {
                profile_id: " House/É ".to_owned(),
                schema_version: "02.0".to_owned(),
                content_version: "content+001".to_owned(),
            })
            .expect("authored selected profile"),
            parameters: Id::from_bits(4),
        },
    ];
    let revision = DesignRevision::from_counter(u128::MAX).successor();
    let snapshot = CommandSnapshotContext::new(
        Id::from_bits(1),
        CommandActorClaim::Agent {
            identity: Id::from_bits(7),
            principal: Id::from_bits(2),
        },
        revision.clone(),
    );
    for profile in profiles {
        let selection = InstanceSelection::new(vec![Id::from_bits(30), Id::from_bits(20)])
            .expect("explicit evaluated sources");
        let input = GenerateTechPack::new(selection.clone(), profile.clone());
        assert_eq!(input.instances(), &selection);
        assert_eq!(input.profile(), &profile);
        let reordered = GenerateTechPack::new(
            InstanceSelection::new(vec![Id::from_bits(20), Id::from_bits(30)])
                .expect("reordered sources"),
            profile,
        );
        assert_ne!(input, reordered);
        let generation =
            CommandArtifact::new(snapshot.clone(), ArtifactPayload::GenerateTechPack(input));
        let inspection = CommandQuery::new(
            snapshot.clone(),
            QueryPayload::GetUncertaintyClosure {
                artifact: generation.payload().clone(),
            },
        );
        assert_eq!(generation.context(), &snapshot);
        assert_eq!(inspection.context(), generation.context());
        assert_eq!(inspection.context().expected_revision(), &revision);
        assert!(
            matches!(inspection.payload(), QueryPayload::GetUncertaintyClosure { artifact } if artifact == generation.payload())
        );
        assert_eq!(generation.command_id(), CommandId::GenerateTechPack);
        assert_eq!(generation.command_id().class(), CommandClass::Artifact);
        assert_eq!(
            generation.command_id().required_authority(),
            CommandAuthority::Generate
        );
        assert_eq!(
            inspection.command_id().required_authority(),
            CommandAuthority::Inspect
        );
    }
}
