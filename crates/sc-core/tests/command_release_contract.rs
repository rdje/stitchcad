//! Candidate preparation retains canonical artifact claims, exact snapshot and generate-only metadata.
use sc_core::{
    command::{
        ArtifactPayload, CommandActorClaim, CommandAuthority, CommandClass, CommandId,
        CommandRelease, CommandReversibility, CommandSnapshotContext, CommandUndoGranularity,
        DesignRevision, DxfTarget, EvaluationProfile, ExportDxf, ExportPdf, ExportPlt,
        GenerateTechPack, InstanceSelection, PdfLayout, PdfOrientation, PdfPrintSettings,
        PrepareRelease, PrepareReleaseError, ReleasePayload,
    },
    ontology::{EntityId as Id, SelectedProfilePin, SelectedProfilePinDefinition},
};
use sc_units::Length;

fn artifacts() -> Result<Vec<ArtifactPayload>, Box<dyn std::error::Error>> {
    let pin = SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: " House/É ".to_owned(),
        schema_version: "02.0".to_owned(),
        content_version: "content+001".to_owned(),
    })?;
    let source = Id::from_bits(3);
    let parameters = Id::from_bits(4);
    Ok(vec![
        ArtifactPayload::GenerateTechPack(GenerateTechPack::new(
            InstanceSelection::new(vec![Id::from_bits(5), source])?,
            EvaluationProfile::Logical,
        )),
        ArtifactPayload::ExportPdf(ExportPdf::new(
            source,
            PdfPrintSettings::new(
                PdfLayout::A0,
                PdfOrientation::Portrait,
                Length::from_micrometres(100_000)?,
            )?,
            EvaluationProfile::Logical,
        )),
        ArtifactPayload::ExportPlt(ExportPlt::new(source, pin.clone(), parameters)),
        ArtifactPayload::ExportDxf(ExportDxf::single_instance(
            source,
            DxfTarget::AstmNumberedR13,
            pin,
            parameters,
        )),
    ])
}

#[test]
fn candidate_inputs_refuse_empty_and_preserve_every_authored_request_order_and_repetition(
) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        PrepareRelease::new(vec![]),
        Err(PrepareReleaseError::NoArtifacts)
    );
    let mut original = artifacts()?;
    let repeated = original.first().ok_or("fixture request absent")?.clone();
    original.push(repeated);
    let candidate = PrepareRelease::new(original.clone())?;
    assert_eq!(candidate.artifacts(), original);
    assert_eq!(candidate.artifacts().len(), 5);
    let mut reversed = original;
    reversed.reverse();
    assert_ne!(candidate, PrepareRelease::new(reversed)?);
    Ok(())
}

#[test]
fn prepare_release_derives_generate_metadata_and_retains_exact_human_or_agent_snapshot(
) -> Result<(), Box<dyn std::error::Error>> {
    let revision = DesignRevision::from_counter(u128::MAX).successor();
    for actor in [
        CommandActorClaim::Human(Id::from_bits(2)),
        CommandActorClaim::Agent {
            identity: Id::from_bits(7),
            principal: Id::from_bits(2),
        },
    ] {
        let snapshot = CommandSnapshotContext::new(Id::from_bits(1), actor, revision.clone());
        let payload = ReleasePayload::PrepareRelease(PrepareRelease::new(artifacts()?)?);
        let request = CommandRelease::new(snapshot.clone(), payload.clone());
        assert_eq!(request.context(), &snapshot);
        assert_eq!(request.context().expected_revision(), &revision);
        assert_eq!(request.context().actor(), actor);
        assert_eq!(request.payload(), &payload);
        assert_eq!(request.command_id(), CommandId::PrepareRelease);
        assert_eq!(request.command_id().class(), CommandClass::Release);
        assert_eq!(
            request.command_id().required_authority(),
            CommandAuthority::Generate
        );
        assert_ne!(
            request.command_id().required_authority(),
            CommandAuthority::Approve
        );
        assert_eq!(
            request.command_id().reversibility(),
            CommandReversibility::NotApplicable
        );
        assert_eq!(
            request.command_id().undo_granularity(),
            CommandUndoGranularity::NotApplicable
        );
    }
    Ok(())
}
