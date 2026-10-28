//! Actual semantic creation with core-owned staged allocation and no model publication.
#![allow(clippy::panic)]
use super::*;
use sc_core::{
    command::{AddNotch, CommandIdentityError, CommandPreview, CommandPreviewError},
    ontology::{
        DeterministicIdGenerator, EdgeAnchor, IdError, IdGenerator, NotchDimensionBindings,
        NotchError, NotchProfileBindings, Param, ProfileBindingValidation, ProfileParameterRef,
    },
};
use std::error::Error as StdError;

pub(super) fn bindings() -> NotchProfileBindings {
    let reference = |identity| ProfileParameterRef::new(id(identity));
    NotchProfileBindings {
        style: reference(710001),
        sample: NotchDimensionBindings {
            depth: reference(710002),
            width: reference(710003),
        },
        production: NotchDimensionBindings {
            depth: reference(710004),
            width: reference(710005),
        },
        encoding: reference(710006),
    }
}
pub(super) fn owner() -> DesignGarment<DesignSources> {
    DesignGarment::new(material_preview::definition()).expect("actual supported complete garment")
}
pub(super) fn addition(owner: &DesignGarment<DesignSources>) -> AddNotch {
    let piece = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual target");
    AddNotch::new(
        piece.id(),
        EdgeAnchor {
            edge: piece.edges().next().expect("actual owned edge").edge,
            param: Param::START,
        },
        bindings(),
    )
}
fn request(design: EntityId, revision: DesignRevision, addition: AddNotch) -> CommandRequest {
    CommandRequest::Mutation(CommandMutation::new(
        CommandMutationContext::new(
            design,
            actor(),
            revision,
            CommandIdempotencyKey::from_entity_id(id(710007)),
            None,
        ),
        MutationPayload::AddNotch(addition),
    ))
}
pub(super) fn reserve() -> Vec<EntityId> {
    let mut generator = DeterministicIdGenerator::from_start(720000);
    vec![
        generator.next_id().expect("reserved identity"),
        generator.next_id().expect("reserved identity"),
    ]
}

#[test]
fn repeated_inspect_only_notch_preview_keeps_actual_identity_reserve_and_garment_unchanged() {
    let owner = owner();
    let intent = addition(&owner);
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let pieces = owner.inputs().structure().pieces().pieces().to_vec();
    let address = owner.inputs().structure().pieces().pieces().as_ptr();
    let host = Host::with_permissions(design, vec![Permission::Inspect]);
    let bus = controlled(
        CommandBus::with_identities(owner, host, reserve())
            .expect("trusted actual generator reserve"),
    );
    let request = request(design, revision.clone(), intent);
    let mut previous = None;
    for _ in 0..3 {
        let report = match bus
            .preview(&request)
            .expect("actual private Notch and complete candidate")
        {
            CommandPreview::Notch(report) => report,
            other => panic!("wrong preview result {other:?}"),
        };
        assert_eq!(report.design(), design);
        assert_eq!(report.revision(), &revision);
        assert_eq!(report.notch().id(), id(720000));
        assert_eq!(report.notch().piece(), intent.piece());
        assert_eq!(
            report.notch().definition(),
            &intent.definition_with_id(id(720000))
        );
        assert_eq!(
            report.notch().profile_binding_validation(),
            ProfileBindingValidation::DeferredToG4
        );
        if let Some(previous) = previous {
            assert_eq!(report, previous);
        }
        previous = Some(report);
    }
    assert!(
        matches!(bus.execute(&request), Err(Error::Authorization(_))),
        "preview is no commit permission"
    );
    let CommandResult::PieceList(result) = bus
        .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
        .expect("same original owner");
    assert_eq!(result.pieces(), pieces);
    assert_eq!(result.pieces().as_ptr(), address);
    assert_eq!(result.revision(), &revision);
}

#[test]
fn reserve_configuration_rejects_duplicates_reverse_order_and_actual_owned_identities() {
    for (identities, previous, identity) in [
        (vec![id(720000), id(720000)], id(720000), id(720000)),
        (vec![id(720001), id(720000)], id(720001), id(720000)),
    ] {
        let owner = owner();
        let design = owner.inputs().id();
        assert_eq!(
            CommandBus::with_identities(owner, Host::new(design), identities)
                .expect_err("no sorting/reuse"),
            CommandIdentityError::NonIncreasing {
                previous,
                identity,
                index: 1
            }
        );
    }
    let owner = owner();
    let design = owner.inputs().id();
    let identity = owner
        .layers()
        .facings()
        .first()
        .expect("actual unrelated family owner")
        .id();
    let position = owner
        .identities()
        .into_iter()
        .find(|(id, _)| *id == identity)
        .expect("canonical owning position")
        .1;
    assert_eq!(
        CommandBus::with_identities(owner, Host::new(design), vec![identity])
            .expect_err("reserve cannot reuse another family identity"),
        CommandIdentityError::AlreadyOwned {
            identity,
            index: 0,
            owner: position
        }
    );
}

#[test]
fn exhausted_reserve_preserves_original_id_error_after_actual_target_lookup() {
    let owner = owner();
    let intent = addition(&owner);
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let bus = controlled(CommandBus::new(owner, Host::new(design)));
    let error = bus
        .preview(&request(design, revision.clone(), intent))
        .expect_err("no hidden fallback source");
    assert!(matches!(
        error,
        CommandPreviewError::Identity(IdError::Exhausted)
    ));
    assert_eq!(
        error
            .source()
            .expect("original allocation cause")
            .downcast_ref::<IdError>(),
        Some(&IdError::Exhausted)
    );
    let absent = AddNotch::new(id(799999), intent.anchor(), intent.representation());
    assert!(
        matches!(bus.preview(&request(design, revision, absent)), Err(CommandPreviewError::MissingPiece { piece, index: 0 }) if piece == id(799999))
    );
}

#[test]
fn refused_anchor_does_not_consume_an_identity_before_a_later_valid_preview() {
    let owner = owner();
    let intent = addition(&owner);
    let other = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .get(1)
        .expect("second actual Piece");
    let invalid = AddNotch::new(other.id(), intent.anchor(), intent.representation());
    let original = sc_core::ontology::Notch::new(
        invalid.definition_with_id(id(720000)),
        other,
        owner.inputs().structure().topology(),
    )
    .expect_err("actual anchor ownership refusal");
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let bus = controlled(
        CommandBus::with_identities(owner, Host::new(design), reserve()).expect("actual reserve"),
    );
    let error = bus
        .preview(&request(design, revision.clone(), invalid))
        .expect_err("bad candidate anchor");
    assert_eq!(
        error
            .source()
            .expect("original semantic anchor error")
            .downcast_ref::<NotchError>(),
        Some(&original)
    );
    let CommandPreview::Notch(report) = bus
        .preview(&request(design, revision, intent))
        .expect("valid after failed candidate")
    else {
        panic!("actual Notch report")
    };
    assert_eq!(report.notch().id(), id(720000));
}
