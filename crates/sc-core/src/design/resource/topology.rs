//! Actual topology journal and maintained indices, without replay, resolution or mutation.
use super::{
    fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation,
    DesignResourceScope, DesignResourceSink,
};
use crate::ontology::{EntityId, IdentityLedger, TopologyEdit};
fn location(
    field: &'static str,
    owner: Option<EntityId>,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Structure,
        owner,
        field,
        index,
    }
}
/// Charge the actual held journal, per-edit vectors, live index and registered-reference map.
/// Precharge the journal before edit traversal. Declared-edge/Offset-fragment locations retain actual
/// operation owner and original journal index. Exhaustive owner/edit matches and Copy-payload guards
/// expose new owned fields. Maintained indices use exact stored lengths, without replay/resolution.
/// No journal/edge/registration/choice is changed; this proves no integrity, readiness or authority.
/// # Errors
/// Returns the original first resource refusal with actual field/operation/index, including prior failure.
pub fn charge_topology_resources(
    ledger: &IdentityLedger,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let view = ledger.resource_view();
    sink.charge_entries(view.journal.len(), location("topology.journal", None, None))?;
    for (index, entry) in view.journal.iter().enumerate() {
        let (operation, edit) = entry.resource_edit();
        match edit {
            TopologyEdit::Declared { edges } => {
                fixed_slice(edges);
                sink.charge_entries(
                    edges.len(),
                    location(
                        "topology.journal.declared.edges",
                        Some(operation),
                        Some(index),
                    ),
                )?;
            }
            TopologyEdit::Split {
                source,
                at,
                first,
                second,
            } => {
                fixed_value(source);
                fixed_value(at);
                fixed_value(first);
                fixed_value(second);
            }
            TopologyEdit::Merge {
                first,
                first_length,
                second,
                second_length,
                merged,
            } => {
                fixed_value(first);
                fixed_value(first_length);
                fixed_value(second);
                fixed_value(second_length);
                fixed_value(merged);
            }
            TopologyEdit::Reverse { edge } | TopologyEdit::Delete { edge } => fixed_value(edge),
            TopologyEdit::Offset { source, fragments } => {
                fixed_value(source);
                fixed_slice(fragments);
                sink.charge_entries(
                    fragments.len(),
                    location(
                        "topology.journal.offset.fragments",
                        Some(operation),
                        Some(index),
                    ),
                )?;
            }
        }
    }
    sink.charge_entries(view.live_edges, location("topology.live_edges", None, None))?;
    sink.charge_entries(
        view.registrations,
        location("topology.registrations", None, None),
    )
}
