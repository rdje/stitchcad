//! One core input surface preserves original family data and derives identity without granting authority.
use sc_core::{
    command::{
        ApprovalScopeTarget, ApprovePackage, ArtifactPayload, CloseDart, CommandActorClaim,
        CommandArtifact, CommandEvaluation, CommandGroupId, CommandId, CommandIdempotencyKey,
        CommandMutation, CommandMutationContext, CommandQuery, CommandRelease, CommandRequest,
        CommandSnapshotContext, DesignRevision, EvaluationPayload, EvaluationProfile,
        GenerateTechPack, InstanceSelection, MutationPayload, PrepareRelease, QueryPayload,
        ReleasePayload, ResolveProfile,
    },
    ontology::{EntityId as Id, PackageIdentity, SelectedProfilePin, SelectedProfilePinDefinition},
};

#[test]
fn every_family_keeps_exact_context_and_payload_without_reclassifying_permissions(
) -> Result<(), Box<dyn std::error::Error>> {
    let snapshot = |index: u128| {
        CommandSnapshotContext::new(
            Id::from_bits(100 + index),
            CommandActorClaim::Agent {
                identity: Id::from_bits(200 + index),
                principal: Id::from_bits(300 + index),
            },
            DesignRevision::from_counter(u128::MAX - index).successor(),
        )
    };
    let mutation_snapshot = snapshot(1);
    let mutation_context = CommandMutationContext::new(
        mutation_snapshot.design(),
        mutation_snapshot.actor(),
        mutation_snapshot.expected_revision().clone(),
        CommandIdempotencyKey::from_entity_id(Id::from_bits(401)),
        Some(CommandGroupId::from_entity_id(Id::from_bits(501))),
    );
    let artifact = ArtifactPayload::GenerateTechPack(GenerateTechPack::new(
        InstanceSelection::new(vec![Id::from_bits(17), Id::from_bits(9)])?,
        EvaluationProfile::Logical,
    ));
    let requests = [
        CommandRequest::Query(CommandQuery::new(snapshot(0), QueryPayload::GetPieceList)),
        CommandRequest::Mutation(CommandMutation::new(
            mutation_context.clone(),
            MutationPayload::CloseDart(CloseDart::new(Id::from_bits(7))),
        )),
        CommandRequest::Evaluation(CommandEvaluation::new(
            snapshot(2),
            EvaluationPayload::ResolveProfile(ResolveProfile::new(SelectedProfilePin::new(
                SelectedProfilePinDefinition {
                    profile_id: " House/É ".to_owned(),
                    schema_version: "02".to_owned(),
                    content_version: "001+review".to_owned(),
                },
            )?)),
        )),
        CommandRequest::Artifact(CommandArtifact::new(snapshot(3), artifact.clone())),
        CommandRequest::Release(CommandRelease::new(
            snapshot(4),
            ReleasePayload::PrepareRelease(PrepareRelease::new(vec![artifact.clone()])?),
        )),
        CommandRequest::Release(CommandRelease::new(
            snapshot(5),
            ReleasePayload::ApprovePackage(ApprovePackage::new(
                PackageIdentity::new(" opaque/É ".to_owned(), vec![0, 255, 0])?,
                ApprovalScopeTarget::from_entity_id(Id::from_bits(8)),
            )),
        )),
    ];
    let expected = [
        (CommandId::GetPieceList, "query", "inspect"),
        (CommandId::CloseDart, "mutation", "commit"),
        (CommandId::ResolveProfile, "evaluation", "generate"),
        (CommandId::GenerateTechPack, "artifact", "generate"),
        (CommandId::PrepareRelease, "release", "generate"),
        (CommandId::ApprovePackage, "release", "approve"),
    ];
    for (index, (request, (identity, class, authority))) in
        requests.iter().zip(expected).enumerate()
    {
        let original = snapshot(u128::try_from(index)?);
        assert_eq!(request.command_id(), identity);
        assert_eq!(request.command_id().class().token(), class);
        assert_eq!(request.command_id().required_authority().token(), authority);
        assert_eq!(request.design(), original.design());
        assert_eq!(request.actor(), original.actor());
        assert_eq!(request.expected_revision(), original.expected_revision());
        let original_revision = match request {
            CommandRequest::Query(command) => command.context().expected_revision(),
            CommandRequest::Mutation(command) => command.context().expected_revision(),
            CommandRequest::Evaluation(command) => command.context().expected_revision(),
            CommandRequest::Artifact(command) => command.context().expected_revision(),
            CommandRequest::Release(command) => command.context().expected_revision(),
        };
        assert!(std::ptr::eq(request.expected_revision(), original_revision));
        if let CommandRequest::Mutation(command) = request {
            let context = request
                .mutation_context()
                .ok_or("mutation metadata missing")?;
            assert!(std::ptr::eq(context, command.context()));
            assert_eq!(context, &mutation_context);
        } else {
            assert_eq!(request.mutation_context(), None);
        }
    }
    let Some(CommandRequest::Artifact(command)) = requests.get(3) else {
        return Err("artifact family changed".into());
    };
    assert_eq!(command.payload(), &artifact);
    // Agent approval is representable untrusted input; catalog metadata cannot authenticate it.
    assert!(matches!(
        requests.last().map(CommandRequest::actor),
        Some(CommandActorClaim::Agent { .. })
    ));
    Ok(())
}

#[test]
fn human_initial_snapshot_remains_zero_and_carries_no_mutation_metadata(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = CommandRequest::Query(CommandQuery::new(
        CommandSnapshotContext::new(
            Id::from_bits(1),
            CommandActorClaim::Human(Id::from_bits(2)),
            DesignRevision::initial(),
        ),
        QueryPayload::WalkSeamSpan {
            span: Id::from_bits(3),
        },
    ));
    assert_eq!(request.actor().initiator(), request.actor().principal());
    assert_eq!(request.expected_revision(), &DesignRevision::initial());
    assert_eq!(request.mutation_context(), None);
    let CommandRequest::Query(command) = request else {
        return Err("query family changed".into());
    };
    assert_eq!(
        command.payload(),
        &QueryPayload::WalkSeamSpan {
            span: Id::from_bits(3)
        }
    );
    Ok(())
}
