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

#[test]
fn package_identity_preserves_opaque_label_and_every_digest_byte_without_aliases(
) -> Result<(), Box<dyn std::error::Error>> {
    use sc_core::ontology::PackageIdentity;
    let label = " Custom-Hash/未来 ".to_owned();
    let bytes = vec![0, 255, 1, 0];
    let identity = PackageIdentity::new(label.clone(), bytes.clone())?;
    assert_eq!(identity.algorithm(), label);
    assert_eq!(identity.digest(), bytes);
    assert_ne!(
        identity,
        PackageIdentity::new(label.trim().to_owned(), bytes.clone())?
    );
    assert_ne!(identity, PackageIdentity::new(label, vec![0, 255, 1])?);
    assert_eq!(identity, identity.clone());
    Ok(())
}

#[test]
fn missing_identity_fields_refuse_original_values_without_guessing_algorithm_or_digest() {
    use sc_core::ontology::{PackageIdentity, PackageIdentityError};
    for blank in ["", " \t\n", "\u{2003}"] {
        assert_eq!(
            PackageIdentity::new(blank.to_owned(), vec![]),
            Err(PackageIdentityError::BlankAlgorithm(blank.to_owned()))
        );
    }
    assert_eq!(
        PackageIdentity::new("opaque_algorithm".to_owned(), vec![]),
        Err(PackageIdentityError::EmptyDigest {
            algorithm: "opaque_algorithm".to_owned()
        })
    );
}

#[test]
fn approval_intent_binds_exact_candidate_scope_and_human_only_metadata_without_trusting_actor_claims(
) -> Result<(), Box<dyn std::error::Error>> {
    use sc_core::{
        command::{ApprovalScopeTarget, ApprovePackage},
        ontology::PackageIdentity,
    };
    let identity = PackageIdentity::new("fixture_algorithm".to_owned(), vec![0, 255, 1, 0])?;
    let scope = ApprovalScopeTarget::from_entity_id(Id::from_bits(50));
    let input = ApprovePackage::new(identity.clone(), scope);
    assert_eq!(input.package_identity(), &identity);
    assert_eq!(input.scope().entity_id(), Id::from_bits(50));
    assert_ne!(
        input,
        ApprovePackage::new(
            identity.clone(),
            ApprovalScopeTarget::from_entity_id(Id::from_bits(51))
        )
    );
    let revision = DesignRevision::from_counter(u128::MAX).successor();
    for actor in [
        CommandActorClaim::Human(Id::from_bits(2)),
        CommandActorClaim::Agent {
            identity: Id::from_bits(7),
            principal: Id::from_bits(2),
        },
    ] {
        let context = CommandSnapshotContext::new(Id::from_bits(1), actor, revision.clone());
        let payload = ReleasePayload::ApprovePackage(input.clone());
        let request = CommandRelease::new(context.clone(), payload.clone());
        assert_eq!(request.context(), &context);
        assert_eq!(request.payload(), &payload);
        assert_eq!(request.command_id(), CommandId::ApprovePackage);
        assert_eq!(request.command_id().class(), CommandClass::Release);
        assert_eq!(
            request.command_id().required_authority(),
            CommandAuthority::Approve
        );
        assert_ne!(
            request.command_id().required_authority(),
            CommandAuthority::Generate
        );
        assert_eq!(
            request.command_id().reversibility(),
            CommandReversibility::No
        );
        assert_eq!(
            request.command_id().undo_granularity(),
            CommandUndoGranularity::NotApplicable
        );
    }
    Ok(())
}
