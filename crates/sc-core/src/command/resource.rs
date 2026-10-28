//! Exact logical owned command data; no authorization, model lookup or runtime admission capability.
use super::{CommandHistoryRequest, CommandMutationGroup, MutationPayload};
use crate::{
    design::{
        charge_length_declaration_resources, DesignResourceLimitError, DesignResourceLocation,
        DesignResourceScope, DesignResourceSink,
    },
    ontology::{EntityId, MaterialAssignment},
};
// Fixed contexts/intent references own no dynamic field. Losing Copy requires explicit accounting.
pub(super) fn fixed<T: Copy>(_: &T) {}
fn fixed_slice<T: Copy>(_: &[T]) {}
fn location(owner: EntityId, field: &'static str, index: Option<usize>) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::CommandState,
        owner: Some(owner),
        field,
        index,
    }
}
/// Charge the actual typed mutation group's owned contexts and five payload forms in schema order.
/// The common snapshot context and every member's context each retain their own revision value;
/// exact retry aliases own their original payload/vector/text copies and are not deduplicated here.
/// Piece selection and Known evidence vectors are precharged before their contents. An unresolved
/// reason is owned once per request payload, independent of its selected Piece count. Proposed model
/// copies and reports are separate owners. No registry/currentness, authority or numeric lookup occurs.
/// This borrows actual immutable data, without cloning, rebinding or accepting source-supplied totals.
/// Raw decoding/allocation and runtime retained-state enforcement remain separate work.
/// # Errors
/// Returns the original first refusal, with original group owner, static field and member index.
pub fn charge_command_mutation_group_resources(
    group: &CommandMutationGroup,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (id, context, mutations) = group.resource_parts();
    let owner = id.entity_id();
    sink.charge_counter_bits(
        context.resource_revision().resource_bits(),
        location(owner, "requests.group.context.revision", None),
    )?;
    sink.charge_entries(mutations.len(), location(owner, "requests.mutations", None))?;
    for (index, mutation) in mutations.iter().enumerate() {
        let (context, payload) = mutation.resource_parts();
        sink.charge_counter_bits(
            context.resource_revision().resource_bits(),
            location(owner, "requests.mutations.context.revision", Some(index)),
        )?;
        match payload {
            MutationPayload::SplitSeamSpan(intent) => fixed(intent),
            MutationPayload::CloseDart(intent) => fixed(intent),
            MutationPayload::AddNotch(intent) => fixed(intent),
            MutationPayload::ChangeMeasurement(change) => {
                let (measurement, replacement) = change.resource_parts();
                fixed(&measurement);
                charge_length_declaration_resources(
                    replacement,
                    location(
                        owner,
                        "requests.mutations.measurement.evidence",
                        Some(index),
                    ),
                    sink,
                )?;
            }
            MutationPayload::AssignMaterial(assignment) => {
                let (selection, material) = assignment.resource_parts();
                let pieces = selection.resource_pieces();
                sink.charge_entries(
                    pieces.len(),
                    location(owner, "requests.mutations.material.pieces", Some(index)),
                )?;
                fixed_slice(pieces);
                match material {
                    MaterialAssignment::Assigned(identity) => fixed(identity),
                    MaterialAssignment::Unresolved(reason) => sink.charge_text(
                        reason,
                        location(owner, "requests.mutations.material.reason", Some(index)),
                    )?,
                }
            }
        }
    }
    Ok(())
}

/// Charge the actual history intent's operator context and complete original authoring-group copy.
/// The direction is fixed data; original contexts, keys, payloads and aliases stay unchanged.
/// No accepted-record lookup, authority, model/currentness check or history traversal occurs.
/// # Errors
/// Returns the original first refusal, with the original containing group and request location.
pub fn charge_command_history_request_resources(
    request: &CommandHistoryRequest,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (context, original) = request.resource_parts();
    sink.charge_counter_bits(
        context.resource_revision().resource_bits(),
        location(
            original.id().entity_id(),
            "requests.history.context.revision",
            None,
        ),
    )?;
    charge_command_mutation_group_resources(original, sink)
}
