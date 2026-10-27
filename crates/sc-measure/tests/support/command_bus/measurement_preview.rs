//! Real canonical Measurement binding and whole candidate validation, without recipe execution.
#![allow(clippy::panic)]
use super::*;
use sc_core::{
    command::{ChangeMeasurement, CommandMeasurementPreview, CommandPreview, CommandPreviewError},
    design::DesignMetadataError,
    ontology::MaterialError,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
use std::error::Error as StdError;

fn owner() -> DesignGarment<DesignSources> {
    DesignGarment::new(material_preview::definition())
        .expect("actual supported garment/material/source host")
}
pub(super) fn original(owner: &DesignGarment<DesignSources>) -> LengthDeclaration {
    owner
        .inputs()
        .sources()
        .measurement_declaration(id(1), owner.inputs().inputs())
        .expect("actual Measurement binding")
        .clone()
}
fn request(
    design: EntityId,
    revision: DesignRevision,
    measurement: EntityId,
    replacement: LengthDeclaration,
) -> CommandRequest {
    CommandRequest::Mutation(CommandMutation::new(
        CommandMutationContext::new(
            design,
            actor(),
            revision,
            CommandIdempotencyKey::from_entity_id(id(740010)),
            None,
        ),
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(measurement, replacement)),
    ))
}
fn report(
    bus: &CommandBus<DesignSources, Host>,
    request: &CommandRequest,
) -> CommandMeasurementPreview {
    match bus.preview(request).expect("actual Measurement candidate") {
        CommandPreview::Measurement(report) => report,
        other => panic!("wrong preview result {other:?}"),
    }
}

#[test]
fn inspect_only_measurement_preview_preserves_exact_source_unknown_derived_and_original_owner() {
    let owner = owner();
    let original = original(&owner);
    let pieces = owner.inputs().structure().pieces().pieces().to_vec();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let bus = controlled(CommandBus::new(
        owner,
        Host::with_permissions(design, vec![Permission::Inspect]),
    ));
    for state in [
        LengthState::Unknown {
            observation: id(740001),
        },
        LengthState::Derived {
            formula: id(740002),
        },
    ] {
        let replacement = LengthDeclaration::new(LengthDeclarationDefinition {
            id: original.id(),
            source: id(740000),
            state,
        })
        .expect("explicit canonical state/provenance");
        let request = request(design, revision.clone(), id(1), replacement.clone());
        let actual = report(&bus, &request);
        assert_eq!(actual.design(), design);
        assert_eq!(actual.revision(), &revision);
        assert_eq!(actual.measurement(), id(1));
        assert_eq!(actual.changes().len(), 1);
        let change = actual
            .changes()
            .first()
            .expect("one actual canonical change");
        assert_eq!(change.before(), &original);
        assert_eq!(change.after(), &replacement);
        assert!(
            change.after().authored_value().is_err(),
            "no evaluated or default numeric result"
        );
        assert!(matches!(
            bus.execute(&request),
            Err(Error::Authorization(_))
        ));
    }
    let noop = request(design, revision.clone(), id(1), original);
    assert!(
        report(&bus, &noop).changes().is_empty(),
        "both previews retained original canonical owner"
    );
    let CommandResult::PieceList(result) = bus
        .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
        .expect("original Piece metadata");
    assert_eq!(result.revision(), &revision);
    assert_eq!(result.pieces(), pieces);
}

#[test]
fn measurement_preview_refuses_rebinding_a_different_canonical_declaration() {
    let owner = owner();
    let original = original(&owner);
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let mut replacement = original.definition().clone();
    replacement.id = id(799998);
    let replacement =
        LengthDeclaration::new(replacement).expect("structurally valid wrong identity");
    let bus = controlled(CommandBus::new(owner, Host::new(design)));
    assert!(
        matches!(bus.preview(&request(design, revision.clone(), id(1), replacement)),
        Err(CommandPreviewError::MeasurementDeclarationMismatch { measurement, expected, requested })
        if measurement == id(1) && expected == original.id() && requested == id(799998))
    );
    assert!(report(&bus, &request(design, revision, id(1), original))
        .changes()
        .is_empty());
}

#[test]
fn missing_measurement_preserves_original_metadata_role_error_before_replacement() {
    let owner = owner();
    let original = original(&owner);
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let bus = controlled(CommandBus::new(owner, Host::new(design)));
    let error = bus
        .preview(&request(design, revision, id(799999), original))
        .expect_err("no invented metadata binding");
    assert_eq!(
        error
            .source()
            .expect("original binding error")
            .downcast_ref::<DesignMetadataError<sc_measure::DesignSourcesError>>(),
        Some(&DesignMetadataError::MissingMeasurement {
            measurement: id(799999)
        })
    );
}

#[test]
fn candidate_revalidates_actual_shared_material_width_and_preserves_failed_owner() {
    let owner = owner();
    let original = original(&owner);
    assert_eq!(
        original.id(),
        id(10),
        "actual shared Measurement/Material canonical declaration"
    );
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let replacement = LengthDeclaration::new(LengthDeclarationDefinition {
        id: original.id(),
        source: original.source(),
        state: LengthState::Known {
            value: sc_units::Length::ZERO,
            evidence: vec![id(740003)],
        },
    })
    .expect("structural evidence inventory only");
    let bus = controlled(CommandBus::new(owner, Host::new(design)));
    let error = bus
        .preview(&request(design, revision.clone(), id(1), replacement))
        .expect_err("actual Material width must remain positive");
    let mut cause: Option<&dyn StdError> = Some(&error);
    let mut material_cause = false;
    while let Some(current) = cause {
        if let Some(MaterialError::NonPositiveWidth { material, .. }) =
            current.downcast_ref::<MaterialError>()
        {
            assert_eq!(*material, id(220000));
            material_cause = true;
        }
        cause = current.source();
    }
    assert!(
        material_cause,
        "original actual Material error remains in standard source chain"
    );
    assert!(
        report(&bus, &request(design, revision, id(1), original))
            .changes()
            .is_empty(),
        "failed candidate retained original canonical value"
    );
}
