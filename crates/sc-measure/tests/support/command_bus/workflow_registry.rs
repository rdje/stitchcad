//! Execute the builtin declarations themselves against real source/model ownership and live grants.
use super::group_commit::{capacity, group, material, mutation};
use super::*;
use sc_core::{
    command::{
        ChangeMeasurement, CommandGroupCommitError as CommitError, CommandGroupCommitResult,
        CommandGroupPreviewError as GroupError, CommandHistoryDirection as Direction,
        CommandHistoryRequest, CommandMutationGroup, CommandWorkflowId as Workflow,
        CommandWorkflowStep as Step,
    },
    ontology::MaterialAssignment,
    value::{LengthDeclaration, LengthState},
};

fn request(
    owner: &DesignGarment<DesignSources>,
    workflow: Workflow,
) -> Option<CommandMutationGroup> {
    let stage = workflow
        .steps()
        .iter()
        .copied()
        .find(|step| *step != Step::GetPieceList)?;
    let design = owner.inputs().id();
    let revision = owner.inputs().revision().clone();
    let addition = notch_preview::addition(owner);
    let mut members = Vec::new();
    for command in stage.commands() {
        let (key, payload) = match command {
            CommandId::AssignMaterial => (
                818001,
                material(addition.piece(), "workflow metadata intent"),
            ),
            CommandId::ChangeMeasurement => {
                let mut definition = measurement_preview::original(owner).definition().clone();
                definition.state = LengthState::Unknown {
                    observation: id(818100),
                };
                let length = LengthDeclaration::new(definition).expect("real authored Unknown");
                (
                    818002,
                    MutationPayload::ChangeMeasurement(ChangeMeasurement::new(id(1), length)),
                )
            }
            CommandId::AddNotch => (818003, MutationPayload::AddNotch(addition)),
            command => unreachable!(
                "a new declared command needs actual native workflow evidence: {command:?}"
            ),
        };
        members.push(mutation(design, revision.clone(), 818000, key, payload));
    }
    Some(group(818000, members))
}

#[test]
fn every_registered_workflow_executes_its_actual_declared_stages_over_real_metadata() {
    for workflow in Workflow::ALL {
        let owner = notch_preview::owner();
        let design = owner.inputs().id();
        let base = owner.inputs().revision().clone();
        let original_address = owner.inputs().structure().pieces().pieces().as_ptr();
        let original_pieces = owner.inputs().structure().pieces().pieces().to_vec();
        let request = request(&owner, workflow);
        let host = Host::with_permissions(design, workflow.required_authorities());
        let mut bus = match workflow {
            Workflow::InspectPieces => CommandBus::new(owner, host),
            Workflow::EditMetadata => controlled(
                CommandBus::with_edit_capacity(owner, host, notch_preview::reserve(), capacity(1))
                    .expect("one actual authoring slot; history is unnecessary"),
            ),
            Workflow::UndoRedoMetadata => controlled(
                CommandBus::with_edit_history(
                    owner,
                    host,
                    notch_preview::reserve(),
                    capacity(1),
                    capacity(1),
                    capacity(2),
                )
                .expect("one authoring/snapshot slot and two history actions"),
            ),
            workflow => {
                unreachable!("a new workflow needs actual native prerequisites: {workflow:?}")
            }
        };
        let mut revision = base.clone();
        let mut accepted: Option<CommandGroupCommitResult> = None;
        let mut published_address = None;
        let mut traversals = 0;
        for stage in workflow.steps() {
            match stage {
                Step::GetPieceList => {
                    let CommandResult::PieceList(actual) = bus
                        .execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
                        .expect("declared actual current inspection");
                    assert_eq!(actual.revision(), &revision);
                    if revision == base {
                        assert_eq!(actual.pieces().as_ptr(), original_address);
                        assert_eq!(actual.pieces(), original_pieces);
                    } else {
                        let expected =
                            MaterialAssignment::Unresolved("workflow metadata intent".into());
                        assert_eq!(
                            actual
                                .pieces()
                                .first()
                                .expect("actual target")
                                .definition()
                                .material,
                            expected
                        );
                        if let Some(address) = published_address {
                            assert_eq!(
                                actual.pieces().as_ptr(),
                                address,
                                "Redo restores the actual published snapshot"
                            );
                        } else {
                            published_address = Some(actual.pieces().as_ptr());
                        }
                    }
                }
                Step::PreviewGroup => {
                    let request = request.as_ref().expect("declared original group");
                    let preview = bus.preview_group(request).expect("actual Inspect preview");
                    assert_eq!(preview.revision(), &base);
                    assert_eq!(
                        preview
                            .steps()
                            .iter()
                            .map(|step| step.command())
                            .collect::<Vec<_>>(),
                        stage.commands()
                    );
                }
                Step::CommitGroup => {
                    let request = request.as_ref().expect("declared original group");
                    let result = bus
                        .commit_group(request)
                        .expect("actual Commit publication");
                    assert!(!result.is_replay());
                    assert_eq!(result.entry().request(), request);
                    let difference = result.entry().commit().changes().difference();
                    assert_eq!(difference.lengths().len(), 1);
                    assert_eq!(difference.materials().len(), 1);
                    assert_eq!(difference.notches().len(), 1);
                    assert_eq!(
                        difference
                            .notches()
                            .first()
                            .expect("actual appended Notch")
                            .id(),
                        id(720000)
                    );
                    revision = result.entry().commit().revision().clone();
                    assert_eq!(revision, base.successor());
                    accepted = Some(result);
                }
                Step::ReplayGroup => {
                    let result = bus
                        .commit_group(request.as_ref().expect("original accepted group"))
                        .expect("exact accepted replay at sole slot capacity");
                    assert!(result.is_replay());
                    assert_eq!(
                        result.entry(),
                        accepted.as_ref().expect("actual prior acceptance").entry()
                    );
                }
                Step::UndoGroup | Step::RedoGroup => {
                    let direction = if *stage == Step::UndoGroup {
                        Direction::Undo
                    } else {
                        Direction::Redo
                    };
                    let key = if direction == Direction::Undo {
                        818010
                    } else {
                        818011
                    };
                    let request = request.as_ref().expect("original accepted group");
                    let operation = CommandHistoryRequest::new(
                        CommandMutationContext::new(
                            design,
                            actor(),
                            revision.clone(),
                            CommandIdempotencyKey::from_entity_id(id(key)),
                            Some(request.id()),
                        ),
                        request.clone(),
                        direction,
                    )
                    .expect("actual bound operator and original group");
                    let result = bus
                        .apply_history(&operation)
                        .expect("actual complete snapshot transition");
                    assert!(!result.is_replay());
                    revision = result.entry().revision().clone();
                    traversals += 1;
                }
                stage => {
                    unreachable!("a new declared stage needs native execution evidence: {stage:?}")
                }
            }
        }
        match workflow {
            Workflow::InspectPieces => {
                assert!(accepted.is_none());
                assert_eq!(revision, base);
                assert_eq!(traversals, 0);
            }
            Workflow::EditMetadata => {
                assert_eq!(
                    revision,
                    accepted
                        .as_ref()
                        .expect("actual accepted edit")
                        .entry()
                        .commit()
                        .revision()
                        .clone()
                );
                assert_eq!(traversals, 0);
            }
            Workflow::UndoRedoMetadata => {
                assert_eq!(
                    revision,
                    accepted
                        .as_ref()
                        .expect("actual accepted snapshot")
                        .entry()
                        .commit()
                        .revision()
                        .clone()
                );
                assert_eq!(traversals, 2);
            }
            workflow => {
                unreachable!("a new registered outcome needs actual evidence: {workflow:?}")
            }
        }
        eprintln!(
            "native workflow {}: {} declared stages executed; actual rights {:?}",
            workflow.token(),
            workflow.steps().len(),
            workflow.required_authorities()
        );
    }
}

#[test]
fn registry_permission_sets_match_actual_independent_read_preview_and_commit_refusals() {
    assert_eq!(
        Workflow::EditMetadata.required_authorities(),
        vec![Permission::Inspect, Permission::Commit]
    );
    for permission in [Permission::Inspect, Permission::Commit] {
        let owner = notch_preview::owner();
        let design = owner.inputs().id();
        let revision = owner.inputs().revision().clone();
        let request = request(&owner, Workflow::EditMetadata).expect("actual registered group");
        let host = Host::with_permissions(design, vec![permission]);
        let mut bus = controlled(
            CommandBus::with_edit_capacity(owner, host, notch_preview::reserve(), capacity(1))
                .expect("actual finite owner with one exact permission"),
        );
        if permission == Permission::Inspect {
            bus.execute(&query(design, revision.clone(), QueryPayload::GetPieceList))
                .expect("Inspect reads");
            bus.preview_group(&request).expect("Inspect previews");
            assert!(
                matches!(bus.commit_group(&request),Err(CommitError::Preparation(issue)) if matches!(*issue,GroupError::Authorization(_)))
            );
            bus.execute(&query(design, revision, QueryPayload::GetPieceList))
                .expect("refusal retains original revision");
        } else {
            assert!(matches!(
                bus.execute(&query(design, revision, QueryPayload::GetPieceList)),
                Err(Error::Authorization(_))
            ));
            assert!(matches!(
                bus.preview_group(&request),
                Err(GroupError::Authorization(_))
            ));
            bus.commit_group(&request)
                .expect("Commit remains independently sufficient for the actual mutation");
        }
    }
}
