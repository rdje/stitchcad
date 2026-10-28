//! Exact actual owned report data, without model/authority lookups or retained runtime admission.
use super::{
    CommandAuditEntry, CommandGroupCommit, CommandGroupCommitResult, CommandGroupDifference,
    CommandGroupPreview, CommandHistoryAuditEntry, CommandHistoryResult, CommandLengthChange,
    CommandMaterialChange, CommandPreview,
};
use crate::{
    command::resource::fixed,
    design::{
        charge_length_declaration_resources, DesignResourceLimitError, DesignResourceLocation,
        DesignResourceScope, DesignResourceSink,
    },
    ontology::{EntityId, MaterialAssignment, Notch, NotchDefinition},
};

#[derive(Clone, Copy)]
struct Site {
    owner: EntityId,
    index: Option<usize>,
}
impl Site {
    fn location(self, field: &'static str) -> DesignResourceLocation {
        DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(self.owner),
            field,
            index: self.index,
        }
    }
}

/// Charge an actual single-command preview's owned revisions, changes and before/after snapshots.
/// The owner is the actual report Design. No model, authority, source or currentness call occurs.
/// This data-only result is not a publication token or runtime retained-state admission.
/// # Errors
/// Preserves the original first resource refusal with static report field and original location.
pub fn charge_command_preview_resources(
    report: &CommandPreview,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_preview(report, None, sink)
}

/// Charge an actual group report's common revision, ordered steps and complete owned net difference.
/// Aliases own another full report, even though they do not apply another edit. Per-step locations
/// retain the actual containing group and original step index; net-difference members use their own
/// original change index. Before/after evidence and text are separate owned copies. This borrows all
/// actual fields without a model clone, reconstructed source census, callback or supplied totals.
/// # Errors
/// Preserves the first refusal, including a prior failure already latched by the caller's sink.
pub fn charge_command_group_preview_resources(
    report: &CommandGroupPreview,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (group, revision, steps, difference) = report.resource_parts();
    let site = Site {
        owner: group.entity_id(),
        index: None,
    };
    sink.charge_counter_bits(
        revision.resource_bits(),
        site.location("reports.group.revision"),
    )?;
    sink.charge_entries(steps.len(), site.location("reports.steps"))?;
    for (index, step) in steps.iter().enumerate() {
        charge_preview(
            step.resource_report(),
            Some(Site {
                index: Some(index),
                ..site
            }),
            sink,
        )?;
    }
    charge_difference(difference, site, sink)
}

fn charge_preview(
    report: &CommandPreview,
    containing: Option<Site>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    match report {
        CommandPreview::Measurement(report) => {
            let (design, revision, measurement, changes) = report.resource_parts();
            fixed(&measurement);
            let site = containing.unwrap_or(Site {
                owner: design,
                index: None,
            });
            sink.charge_counter_bits(
                revision.resource_bits(),
                site.location("reports.measurement.revision"),
            )?;
            sink.charge_entries(changes.len(), site.location("reports.measurement.changes"))?;
            for (index, change) in changes.iter().enumerate() {
                let site = Site {
                    index: containing.map_or(Some(index), |outer| outer.index),
                    ..site
                };
                charge_length_change(
                    change,
                    site,
                    "reports.measurement.before.evidence",
                    "reports.measurement.after.evidence",
                    sink,
                )?;
            }
        }
        CommandPreview::Material(report) => {
            let (design, revision, changes) = report.resource_parts();
            let site = containing.unwrap_or(Site {
                owner: design,
                index: None,
            });
            sink.charge_counter_bits(
                revision.resource_bits(),
                site.location("reports.material.revision"),
            )?;
            sink.charge_entries(changes.len(), site.location("reports.material.changes"))?;
            for (index, change) in changes.iter().enumerate() {
                let site = Site {
                    index: containing.map_or(Some(index), |outer| outer.index),
                    ..site
                };
                charge_material_change(
                    change,
                    site,
                    "reports.material.before.reason",
                    "reports.material.after.reason",
                    sink,
                )?;
            }
        }
        CommandPreview::Notch(report) => {
            let (design, revision, notch) = report.resource_parts();
            let site = containing.unwrap_or(Site {
                owner: design,
                index: None,
            });
            sink.charge_counter_bits(
                revision.resource_bits(),
                site.location("reports.notch.revision"),
            )?;
            charge_notch(notch);
        }
    }
    Ok(())
}

fn charge_difference(
    difference: &CommandGroupDifference,
    site: Site,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (lengths, materials, notches) = difference.resource_parts();
    sink.charge_entries(lengths.len(), site.location("reports.difference.lengths"))?;
    for (index, change) in lengths.iter().enumerate() {
        charge_length_change(
            change,
            Site {
                index: Some(index),
                ..site
            },
            "reports.difference.lengths.before.evidence",
            "reports.difference.lengths.after.evidence",
            sink,
        )?;
    }
    sink.charge_entries(
        materials.len(),
        site.location("reports.difference.materials"),
    )?;
    for (index, change) in materials.iter().enumerate() {
        charge_material_change(
            change,
            Site {
                index: Some(index),
                ..site
            },
            "reports.difference.materials.before.reason",
            "reports.difference.materials.after.reason",
            sink,
        )?;
    }
    sink.charge_entries(notches.len(), site.location("reports.difference.notches"))?;
    for notch in notches {
        charge_notch(notch);
    }
    Ok(())
}
fn charge_length_change(
    change: &CommandLengthChange,
    site: Site,
    before_field: &'static str,
    after_field: &'static str,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let CommandLengthChange { before, after } = change;
    charge_length_declaration_resources(before, site.location(before_field), sink)?;
    charge_length_declaration_resources(after, site.location(after_field), sink)
}
fn charge_material_change(
    change: &CommandMaterialChange,
    site: Site,
    before_field: &'static str,
    after_field: &'static str,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let CommandMaterialChange {
        piece,
        before,
        after,
    } = change;
    fixed(piece);
    charge_assignment(before, site.location(before_field), sink)?;
    charge_assignment(after, site.location(after_field), sink)
}
fn charge_assignment(
    assignment: &MaterialAssignment,
    location: DesignResourceLocation,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    match assignment {
        MaterialAssignment::Assigned(identity) => {
            fixed(identity);
            Ok(())
        }
        MaterialAssignment::Unresolved(reason) => sink.charge_text(reason, location),
    }
}
fn charge_notch(notch: &Notch) {
    let (piece, definition) = notch.resource_parts();
    let NotchDefinition {
        id,
        anchor,
        representation,
    } = definition;
    fixed(&piece);
    fixed(id);
    fixed(anchor);
    fixed(representation);
}

/// Charge the actual accepted commit's original report, issued revision and redo-discard vector.
/// Fixed oldest-undo identities add no dynamic entry. This borrows data without executing an effect.
/// # Errors
/// Preserves the first refusal at its actual containing group, static field and original index.
pub fn charge_command_group_commit_resources(
    commit: &CommandGroupCommit,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (changes, revision, history) = commit.resource_parts();
    let site = Site {
        owner: changes.group().entity_id(),
        index: None,
    };
    charge_command_group_preview_resources(changes, sink)?;
    sink.charge_counter_bits(
        revision.resource_bits(),
        site.location("records.commit.revision"),
    )?;
    let discarded = history.resource_discarded();
    sink.charge_entries(
        discarded.len(),
        site.location("records.commit.discarded_redo"),
    )?;
    for group in discarded {
        fixed(group);
    }
    Ok(())
}

/// Charge one actual immutable accepted authoring record in stored schema order.
/// Its sequence, original request copy and complete accepted commit are independent owned fields.
/// Trusted final grant/time are fixed values; no fresh authority, model or replay lookup occurs.
/// # Errors
/// Preserves the original first resource refusal; this is not runtime retained-state admission.
pub fn charge_command_audit_entry_resources(
    entry: &CommandAuditEntry,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (sequence, request, commit) = entry.resource_parts();
    let site = Site {
        owner: request.id().entity_id(),
        index: None,
    };
    sink.charge_counter_bits(
        sequence.resource_bits(),
        site.location("records.authoring.sequence"),
    )?;
    crate::command::charge_command_mutation_group_resources(request, sink)?;
    charge_command_group_commit_resources(commit, sink)
}

/// Charge an actual returned authoring result's owned accepted-record copy, including exact replay.
/// Its fixed replay flag neither deduplicates the returned data nor appends to the private journal.
/// # Errors
/// Preserves the original first refusal without authority, currentness or publication effects.
pub fn charge_command_group_commit_result_resources(
    result: &CommandGroupCommitResult,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_command_audit_entry_resources(result.resource_entry(), sink)
}

/// Charge one actual accepted history action, retaining its operator/original-author distinction.
/// Schema order is sequence, effective-command vector, full history request, before/restored revision.
/// Original aliases remain in the copied authoring request; effective commands are the actual stored
/// vector with execution aliases omitted. No reconstruction, model traversal or history effect occurs.
/// # Errors
/// Preserves the original first refusal under the actual original containing group.
pub fn charge_command_history_audit_entry_resources(
    entry: &CommandHistoryAuditEntry,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (sequence, commands, request, before, revision) = entry.resource_parts();
    let site = Site {
        owner: request.original().id().entity_id(),
        index: None,
    };
    sink.charge_counter_bits(
        sequence.resource_bits(),
        site.location("records.history.sequence"),
    )?;
    sink.charge_entries(commands.len(), site.location("records.history.commands"))?;
    for command in commands {
        fixed(command);
    }
    crate::command::charge_command_history_request_resources(request, sink)?;
    sink.charge_counter_bits(
        before.resource_bits(),
        site.location("records.history.before_revision"),
    )?;
    sink.charge_counter_bits(
        revision.resource_bits(),
        site.location("records.history.revision"),
    )
}

/// Charge an actual returned history result's owned action copy, including exact replay.
/// The replay flag is fixed; it grants no traversal, currentness or reset capability.
/// # Errors
/// Preserves the original first refusal without model/source/authority callbacks.
pub fn charge_command_history_result_resources(
    result: &CommandHistoryResult,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_command_history_audit_entry_resources(result.resource_entry(), sink)
}
