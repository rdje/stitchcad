//! Real private candidate preparation under inspect-only authority, without publication.
#![allow(clippy::panic)]
use super::*;
use crate::garment_declarations;
use sc_core::{
    command::{
        AssignMaterial, CommandMaterialPreview, CommandPreview, CommandPreviewError, PieceSelection,
    },
    design::{DesignGarmentDefinition, DesignGarmentError, DesignInputsError},
    ontology::{LayerInventory, MaterialAssignment, MaterialInventoryError, Piece},
};
use std::error::Error as StdError;

pub(super) fn definition() -> DesignGarmentDefinition<DesignSources> {
    let mut definition = garment_declarations::assigned_definition();
    definition.layers = LayerInventory::new(
        definition.layers.facings().to_vec(),
        vec![],
        definition.layers.interfacings().to_vec(),
    )
    .expect("supported envelope retains actual Facing and Interfacing");
    definition
}
fn material_report(report: CommandPreview) -> CommandMaterialPreview {
    match report {
        CommandPreview::Material(report) => report,
        other => panic!("actual Material preview expected, got {other:?}"),
    }
}
fn assignment(
    design: EntityId,
    revision: DesignRevision,
    pieces: Vec<EntityId>,
    material: MaterialAssignment,
) -> CommandRequest {
    CommandRequest::Mutation(CommandMutation::new(
        CommandMutationContext::new(
            design,
            actor(),
            revision,
            CommandIdempotencyKey::from_entity_id(id(700004)),
            None,
        ),
        MutationPayload::AssignMaterial(
            AssignMaterial::new(
                PieceSelection::new(pieces).expect("actual target order"),
                material,
            )
            .expect("explicit assignment intent"),
        ),
    ))
}
fn unchanged(bus: &CommandBus<DesignSources, Host>, request: &CommandRequest, pieces: &[Piece]) {
    let CommandResult::PieceList(result) = bus.execute(request).expect("original metadata read");
    assert_eq!(result.pieces(), pieces);
    assert_eq!(result.revision(), request.expected_revision());
}

#[test]
fn inspect_only_preview_reports_actual_changes_in_selection_order_without_publication() {
    let owner = DesignGarment::new(definition()).expect("actual complete garment");
    let pieces = owner.inputs().structure().pieces().pieces().to_vec();
    let address = owner.inputs().structure().pieces().pieces().as_ptr();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let selected: Vec<_> = pieces.iter().rev().map(Piece::id).collect();
    let host = Host::with_permissions(design, vec![Permission::Inspect]);
    let bus = CommandBus::new(owner, host);
    let request = assignment(
        design,
        revision.clone(),
        selected.clone(),
        MaterialAssignment::Assigned(id(220000)),
    );
    let preview = material_report(
        bus.preview(&request)
            .expect("inspect alone prepares a real candidate with unknown material flags"),
    );
    assert_eq!(preview.design(), design);
    assert_eq!(preview.revision(), &revision);
    assert_eq!(preview.changes().len(), selected.len());
    for (change, piece) in preview.changes().iter().zip(pieces.iter().rev()) {
        assert_eq!(change.piece(), piece.id());
        assert_eq!(change.before(), &piece.definition().material);
        assert_eq!(change.after(), &MaterialAssignment::Assigned(id(220000)));
    }
    assert!(
        matches!(bus.execute(&request), Err(Error::Authorization(_))),
        "successful preview grants no commit permission"
    );
    let query = query(design, revision, QueryPayload::GetPieceList);
    unchanged(&bus, &query, &pieces);
    let CommandResult::PieceList(result) = bus.execute(&query).expect("original owner");
    assert_eq!(result.pieces().as_ptr(), address);
    for command in CommandId::ALL {
        assert_eq!(
            bus.is_preview_available(command),
            matches!(command, CommandId::AssignMaterial | CommandId::AddNotch)
        );
        assert_eq!(
            bus.is_available(command),
            command == CommandId::GetPieceList
        );
    }
}

#[test]
fn exact_noop_is_omitted_and_unresolved_reason_text_is_preserved() {
    let owner = DesignGarment::new(definition()).expect("real supported garment");
    let pieces = owner.inputs().structure().pieces().pieces().to_vec();
    let first = pieces.first().expect("target");
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let bus = CommandBus::new(owner, Host::new(design));
    let noop = assignment(
        design,
        revision.clone(),
        vec![first.id()],
        first.definition().material.clone(),
    );
    assert!(
        material_report(bus.preview(&noop).expect("same exact assignment"))
            .changes()
            .is_empty()
    );
    let proposed = MaterialAssignment::Unresolved("  observe grain and nap\n".into());
    let change = assignment(design, revision.clone(), vec![first.id()], proposed.clone());
    let report = material_report(
        bus.preview(&change)
            .expect("explicit unresolved material intent"),
    );
    assert_eq!(report.changes().len(), 1);
    assert_eq!(
        report
            .changes()
            .first()
            .expect("actual single change")
            .before(),
        &first.definition().material
    );
    assert_eq!(
        report
            .changes()
            .first()
            .expect("actual single change")
            .after(),
        &proposed
    );
    assert_eq!(
        bus.execute(&change),
        Err(Error::HandlerUnavailable {
            command: CommandId::AssignMaterial
        })
    );
    unchanged(
        &bus,
        &query(design, revision, QueryPayload::GetPieceList),
        &pieces,
    );
}

#[test]
fn missing_later_piece_and_missing_material_keep_original_causes_and_owner() {
    let owner = DesignGarment::new(definition()).expect("real supported garment");
    let pieces = owner.inputs().structure().pieces().pieces().to_vec();
    let first = pieces.first().expect("target").id();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let bus = CommandBus::new(owner, Host::new(design));
    let absent = assignment(
        design,
        revision.clone(),
        vec![first, id(799999)],
        MaterialAssignment::Assigned(id(220000)),
    );
    assert!(
        matches!(bus.preview(&absent), Err(CommandPreviewError::MissingPiece { piece, index: 1 }) if piece == id(799999))
    );
    let request = assignment(
        design,
        revision.clone(),
        vec![first],
        MaterialAssignment::Assigned(id(799998)),
    );
    let issue = bus.preview(&request).expect_err("no fabricated Material");
    let original = DesignInputsError::<sc_measure::DesignSourcesError>::Materials(Box::new(
        MaterialInventoryError::MissingMaterial {
            piece: first,
            material: id(799998),
        },
    ));
    assert_eq!(
        issue
            .source()
            .expect("original input source")
            .downcast_ref::<DesignInputsError<sc_measure::DesignSourcesError>>(),
        Some(&original)
    );
    unchanged(
        &bus,
        &query(design, revision, QueryPayload::GetPieceList),
        &pieces,
    );
}

#[test]
fn preview_reauthorizes_before_binding_or_envelope_and_reports_unsupported_handler() {
    let owner = garment(); // Original modelled Lining must not be inspected before authentication.
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let piece = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual selected Piece")
        .id();
    let host = Host::with_permissions(design, vec![Permission::Inspect]);
    let authenticated = Rc::clone(&host.actor);
    let revoked = Rc::clone(&host.revoked);
    let bus = CommandBus::new(owner, host);
    let request = assignment(
        design,
        revision.clone(),
        vec![piece],
        MaterialAssignment::Assigned(id(799998)),
    );
    authenticated.set(None);
    let error = bus
        .preview(&request)
        .expect_err("authority before modelled Lining");
    assert!(
        matches!(error, CommandPreviewError::Command(ref e) if matches!(e.as_ref(), Error::Authorization(_)))
    );
    authenticated.set(Some(actor()));
    assert!(matches!(
        bus.preview(&request),
        Err(CommandPreviewError::Envelope(_))
    ));
    revoked.set(true);
    assert!(
        matches!(bus.preview(&request), Err(CommandPreviewError::Command(ref e)) if matches!(e.as_ref(), Error::Authorization(_)))
    );
    revoked.set(false);
    let unsupported = CommandRequest::Mutation(CommandMutation::new(
        CommandMutationContext::new(
            design,
            actor(),
            revision.clone(),
            CommandIdempotencyKey::from_entity_id(id(700004)),
            None,
        ),
        MutationPayload::CloseDart(CloseDart::new(id(700005))),
    ));
    assert!(
        matches!(bus.preview(&unsupported), Err(CommandPreviewError::Command(ref e)) if matches!(e.as_ref(), Error::HandlerUnavailable { command: CommandId::CloseDart }))
    );
    let stale = assignment(
        design,
        revision.successor(),
        vec![piece],
        MaterialAssignment::Assigned(id(220000)),
    );
    assert!(
        matches!(bus.preview(&stale), Err(CommandPreviewError::Command(ref e)) if matches!(e.as_ref(), Error::RevisionMismatch { .. }))
    );
}

#[test]
fn envelope_and_current_source_failures_preserve_actual_standard_error_chains() {
    let owner = garment();
    let expected = owner
        .require_in_scope()
        .expect_err("original modelled Lining");
    let request = assignment(
        owner.inputs().id(),
        owner.inputs().revision().clone(),
        vec![owner
            .inputs()
            .structure()
            .pieces()
            .pieces()
            .first()
            .expect("actual selected Piece")
            .id()],
        MaterialAssignment::Assigned(id(799998)),
    );
    let bus = CommandBus::new(owner, Host::new(request.design()));
    let issue = bus
        .preview(&request)
        .expect_err("envelope precedes missing Material");
    assert_eq!(
        issue.source().expect("original envelope").downcast_ref(),
        Some(&expected)
    );
    let mut damaged = definition();
    garment_current::lose_edge(&mut damaged, 1);
    let owner = DesignGarment::new(damaged).expect("actual held repair state");
    let expected = owner
        .validate_current()
        .expect_err("actual later source loss");
    let pieces = owner.inputs().structure().pieces().pieces().to_vec();
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let bus = CommandBus::new(owner, Host::new(design));
    let request = assignment(
        design,
        revision.clone(),
        vec![pieces.first().expect("actual selected Piece").id()],
        MaterialAssignment::Assigned(id(799998)),
    );
    let issue = bus
        .preview(&request)
        .expect_err("whole source preflight before candidate registry");
    assert_eq!(
        issue
            .source()
            .expect("original complete source")
            .downcast_ref::<DesignGarmentError<sc_measure::DesignSourcesError>>(),
        Some(&expected)
    );
    unchanged(
        &bus,
        &query(design, revision, QueryPayload::GetPieceList),
        &pieces,
    );
}
