//! Native mixed-command progression, exact retries and discarded failure candidates.
use super::*;
use sc_core::{
    command::{
        AddNotch, AssignMaterial, ChangeMeasurement, CommandGroupId,
        CommandGroupPreviewError as GroupError, CommandMutationGroup, CommandPreview,
        CommandPreviewError, PieceSelection,
    },
    ontology::{IdError, MaterialAssignment, NotchError},
    value::{LengthDeclaration, LengthState},
};
use std::error::Error as StdError;

fn group_id() -> CommandGroupId {
    CommandGroupId::from_entity_id(id(750000))
}
fn mutation(
    owner: &DesignGarment<DesignSources>,
    key: u128,
    payload: MutationPayload,
) -> CommandMutation {
    CommandMutation::new(
        CommandMutationContext::new(
            owner.inputs().id(),
            actor(),
            owner.inputs().revision().clone(),
            CommandIdempotencyKey::from_entity_id(id(key)),
            Some(group_id()),
        ),
        payload,
    )
}
fn group(mutations: Vec<CommandMutation>) -> CommandMutationGroup {
    CommandMutationGroup::new(group_id(), mutations).expect("original common group claims")
}
fn material(piece: EntityId, assignment: MaterialAssignment) -> MutationPayload {
    MutationPayload::AssignMaterial(
        AssignMaterial::new(
            PieceSelection::new(vec![piece]).expect("one actual target"),
            assignment,
        )
        .expect("explicit reason"),
    )
}

#[test]
fn mixed_group_uses_actual_preceding_models_net_delta_and_one_cursor_with_explicit_retry() {
    let owner = notch_preview::owner();
    let addition = notch_preview::addition(&owner);
    let length = measurement_preview::original(&owner);
    let mut replacement = length.definition().clone();
    replacement.state = LengthState::Unknown {
        observation: id(750100),
    };
    let replacement = LengthDeclaration::new(replacement).expect("actual authored unknown");
    let pieces = owner.inputs().structure().pieces().pieces();
    let address = pieces.as_ptr();
    let originals = pieces.to_vec();
    let original_material = pieces
        .first()
        .expect("actual target")
        .definition()
        .material
        .clone();
    let temporary = MaterialAssignment::Unresolved("temporary group intent".into());
    let notch = mutation(&owner, 750012, MutationPayload::AddNotch(addition));
    let request = group(vec![
        mutation(
            &owner,
            750010,
            material(addition.piece(), temporary.clone()),
        ),
        mutation(
            &owner,
            750011,
            MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), replacement.clone())),
        ),
        notch.clone(),
        mutation(
            &owner,
            750013,
            material(addition.piece(), original_material.clone()),
        ),
        mutation(
            &owner,
            750014,
            MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), length)),
        ),
        mutation(&owner, 750015, MutationPayload::AddNotch(addition)),
        notch,
    ]);
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let host = Host::with_permissions(design, vec![Permission::Inspect]);
    let reads = Rc::clone(&host.reads);
    let bus = controlled(
        CommandBus::with_identities(owner, host, notch_preview::reserve()).expect("actual reserve"),
    );
    let report = bus
        .preview_group(&request)
        .expect("actual sequential candidates");
    assert_eq!(reads.get(), 1, "one authoritative snapshot for all members");
    assert_eq!(report.group(), group_id());
    assert_eq!(report.design(), design);
    assert_eq!(report.revision(), &revision);
    assert_eq!(report.steps().len(), 7);
    let Some(CommandPreview::Material(restored)) = report.steps().get(3).map(|s| s.report()) else {
        unreachable!("Material report")
    };
    assert_eq!(
        restored
            .changes()
            .first()
            .expect("restoration change")
            .before(),
        &temporary
    );
    assert_eq!(
        restored
            .changes()
            .first()
            .expect("restoration change")
            .after(),
        &original_material
    );
    let Some(CommandPreview::Measurement(restored)) = report.steps().get(4).map(|s| s.report())
    else {
        unreachable!("Measurement report")
    };
    assert_eq!(
        restored
            .changes()
            .first()
            .expect("restoration change")
            .before(),
        &replacement
    );
    assert!(report.difference().lengths().is_empty());
    assert!(report.difference().materials().is_empty());
    assert_eq!(
        report
            .difference()
            .notches()
            .iter()
            .map(|n| n.id())
            .collect::<Vec<_>>(),
        vec![id(720000), id(720001)]
    );
    let retry = report.steps().last().expect("original alias position");
    assert_eq!(retry.command(), CommandId::AddNotch);
    assert_eq!(
        retry.key(),
        CommandIdempotencyKey::from_entity_id(id(750012))
    );
    assert_eq!(retry.replay_of(), Some(2));
    assert_eq!(
        Some(retry.report()),
        report.steps().get(2).map(|s| s.report())
    );
    assert_eq!(
        bus.preview_group(&request).expect("independent repeat"),
        report
    );
    assert_eq!(reads.get(), 2);
    let CommandResult::PieceList(original) = bus
        .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
        .expect("original owner");
    assert_eq!(original.pieces(), originals);
    assert_eq!(original.pieces().as_ptr(), address);
    assert_eq!(original.revision(), &revision);
}

#[test]
fn later_failure_retains_exact_command_and_original_cause_and_discards_all_staged_identities() {
    let owner = notch_preview::owner();
    let addition = notch_preview::addition(&owner);
    let other = owner
        .inputs()
        .structure()
        .pieces()
        .pieces()
        .get(1)
        .expect("actual second target");
    let invalid = AddNotch::new(other.id(), addition.anchor(), addition.representation());
    let expected = sc_core::ontology::Notch::new(
        invalid.definition_with_id(id(720001)),
        other,
        owner.inputs().structure().topology(),
    )
    .expect_err("native anchor refusal");
    let first = mutation(&owner, 750020, MutationPayload::AddNotch(addition));
    let bad = group(vec![
        first.clone(),
        mutation(&owner, 750021, MutationPayload::AddNotch(invalid)),
    ]);
    let exhausted = group(vec![
        first.clone(),
        mutation(&owner, 750022, MutationPayload::AddNotch(addition)),
        mutation(&owner, 750023, MutationPayload::AddNotch(addition)),
    ]);
    let valid = group(vec![first]);
    let design = owner.inputs().id();
    let bus = controlled(
        CommandBus::with_identities(owner, Host::new(design), notch_preview::reserve())
            .expect("actual reserve"),
    );
    let error = bus
        .preview_group(&bad)
        .expect_err("later anchor fails whole group");
    assert!(
        matches!(&error,GroupError::Mutation { group, index:1, command:CommandId::AddNotch, .. } if *group==group_id())
    );
    let original = error
        .source()
        .expect("original preview cause")
        .downcast_ref::<CommandPreviewError<sc_measure::DesignSourcesError>>()
        .expect("original typed preview cause");
    assert_eq!(
        original
            .source()
            .expect("original native anchor cause")
            .downcast_ref::<NotchError>(),
        Some(&expected)
    );
    assert!(
        matches!(bus.preview_group(&exhausted),Err(GroupError::Mutation { index:2, command:CommandId::AddNotch, issue, .. }) if matches!(*issue,CommandPreviewError::Identity(IdError::Exhausted)))
    );
    let report = bus
        .preview_group(&valid)
        .expect("original reserve after two failures");
    assert_eq!(
        report
            .difference()
            .notches()
            .first()
            .expect("original first available identity")
            .id(),
        id(720000)
    );
}

#[test]
fn conflicting_retry_key_refuses_instead_of_applying_a_different_payload() {
    let owner = notch_preview::owner();
    let addition = notch_preview::addition(&owner);
    let first = mutation(&owner, 750030, MutationPayload::AddNotch(addition));
    let bad = group(vec![
        first.clone(),
        mutation(
            &owner,
            750030,
            material(id(799999), MaterialAssignment::Assigned(id(799998))),
        ),
    ]);
    let valid = group(vec![first]);
    let design = owner.inputs().id();
    let bus = controlled(
        CommandBus::with_identities(owner, Host::new(design), notch_preview::reserve())
            .expect("actual reserve"),
    );
    assert!(
        matches!(bus.preview_group(&bad),Err(GroupError::RetryConflict { group, key, first:0,index:1,command:CommandId::AssignMaterial, .. }) if group==group_id() && key==CommandIdempotencyKey::from_entity_id(id(750030)))
    );
    assert_eq!(
        bus.preview_group(&valid)
            .expect("unchanged original reserve")
            .difference()
            .notches()
            .first()
            .expect("first identity")
            .id(),
        id(720000)
    );
}

#[test]
fn group_preview_refreshes_whole_authority_before_actual_binding_or_member_preparation() {
    let owner = notch_preview::owner();
    let addition = notch_preview::addition(&owner);
    let request = group(vec![mutation(
        &owner,
        750040,
        MutationPayload::AddNotch(addition),
    )]);
    let design = owner.inputs().id();
    let host = Host::new(design);
    let session = Rc::clone(&host.actor);
    let revoked = Rc::clone(&host.revoked);
    let reads = Rc::clone(&host.reads);
    let bus = controlled(
        CommandBus::with_identities(owner, host, notch_preview::reserve()).expect("actual reserve"),
    );
    bus.preview_group(&request)
        .expect("first live source observation");
    revoked.set(true);
    assert!(matches!(
        bus.preview_group(&request),
        Err(GroupError::Authorization(_))
    ));
    session.set(None);
    let wrong = group(vec![CommandMutation::new(
        CommandMutationContext::new(
            id(799999),
            actor(),
            DesignRevision::from_counter(99),
            CommandIdempotencyKey::from_entity_id(id(750041)),
            Some(group_id()),
        ),
        MutationPayload::CloseDart(CloseDart::new(id(799998))),
    )]);
    let error = bus
        .preview_group(&wrong)
        .expect_err("authenticate before wrong Design/revision/handler");
    assert!(matches!(error, GroupError::Authorization(_)));
    assert_eq!(reads.get(), 3);
}

#[test]
fn group_preview_binds_actual_design_then_exact_revision_then_original_handler_failure() {
    for (wrong_design, wrong_revision) in [(true, true), (false, true), (false, false)] {
        let owner = notch_preview::owner();
        let actual_design = owner.inputs().id();
        let requested = if wrong_design {
            id(799999)
        } else {
            actual_design
        };
        let revision = if wrong_revision {
            owner.inputs().revision().successor()
        } else {
            owner.inputs().revision().clone()
        };
        let request = group(vec![CommandMutation::new(
            CommandMutationContext::new(
                requested,
                actor(),
                revision.clone(),
                CommandIdempotencyKey::from_entity_id(id(750050)),
                Some(group_id()),
            ),
            MutationPayload::CloseDart(CloseDart::new(id(799998))),
        )]);
        let current = owner.inputs().revision().clone();
        let bus = controlled(CommandBus::new(owner, Host::new(requested)));
        let error = bus
            .preview_group(&request)
            .expect_err("exact binding then actual availability");
        match (wrong_design, wrong_revision, error) {
            (
                true,
                _,
                GroupError::DesignMismatch {
                    group,
                    requested: observed,
                    current,
                },
            ) => {
                assert_eq!(group, group_id());
                assert_eq!(observed, requested);
                assert_eq!(current, actual_design);
            }
            (
                false,
                true,
                GroupError::RevisionMismatch {
                    group,
                    requested,
                    current: observed,
                },
            ) => {
                assert_eq!(group, group_id());
                assert_eq!(requested, revision);
                assert_eq!(observed, current);
            }
            (
                false,
                false,
                GroupError::Mutation {
                    index: 0,
                    command: CommandId::CloseDart,
                    issue,
                    ..
                },
            ) => assert!(
                matches!(*issue,CommandPreviewError::Command(cause) if matches!(*cause,Error::HandlerUnavailable {command:CommandId::CloseDart}))
            ),
            _ => unreachable!("original structured binding priority"),
        }
    }
}

#[test]
fn final_delta_reads_actual_declarations_and_assignments_in_original_inventory_order() {
    let owner = notch_preview::owner();
    let length = measurement_preview::original(&owner);
    let mut definition = length.definition().clone();
    definition.source = id(750200);
    definition.state = LengthState::Derived {
        formula: id(750201),
    };
    let replacement = LengthDeclaration::new(definition).expect("explicit derived declaration");
    let originals = owner.inputs().structure().pieces().pieces().to_vec();
    let intent = MaterialAssignment::Unresolved("  exact final reason\n".into());
    let selection = originals.iter().rev().map(|p| p.id()).collect();
    let request = group(vec![
        mutation(
            &owner,
            750060,
            MutationPayload::AssignMaterial(
                AssignMaterial::new(
                    PieceSelection::new(selection).expect("reverse submitted order"),
                    intent.clone(),
                )
                .expect("explicit reason"),
            ),
        ),
        mutation(
            &owner,
            750061,
            MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), replacement.clone())),
        ),
    ]);
    let design = owner.inputs().id();
    let original_request = CommandRequest::Mutation(CommandMutation::new(
        CommandMutationContext::new(
            design,
            actor(),
            owner.inputs().revision().clone(),
            CommandIdempotencyKey::from_entity_id(id(750062)),
            None,
        ),
        MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), length.clone())),
    ));
    let bus = controlled(CommandBus::new(owner, Host::new(design)));
    let report = bus
        .preview_group(&request)
        .expect("actual nonempty final delta");
    let delta = report.difference();
    assert_eq!(delta.lengths().len(), 1);
    let change = delta.lengths().first().expect("one actual declaration");
    assert_eq!(change.before(), &length);
    assert_eq!(change.after(), &replacement);
    assert!(delta.notches().is_empty());
    assert_eq!(delta.materials().len(), originals.len());
    for (change, original) in delta.materials().iter().zip(&originals) {
        assert_eq!(change.piece(), original.id());
        assert_eq!(change.before(), &original.definition().material);
        assert_eq!(change.after(), &intent);
    }
    let CommandPreview::Measurement(original) = bus
        .preview(&original_request)
        .expect("original canonical state retained")
    else {
        unreachable!("Measurement report")
    };
    assert!(original.changes().is_empty());
}
