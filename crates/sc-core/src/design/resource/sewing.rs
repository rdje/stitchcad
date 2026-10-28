//! Actual sewing intent/semantic landmarks, without context lookup or geometric realization.
use super::{
    fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation,
    DesignResourceScope, DesignResourceSink,
};
use crate::ontology::{
    DeclaredEase, EaseDistribution, EntityId, NotchDefinition, SeamSpanDefinition, SewingGraph,
    SewingLandmark,
};
fn location(
    field: &'static str,
    owner: Option<EntityId>,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Sewing,
        owner,
        field,
        index,
    }
}
/// Charge the actual graph/span/weighted-region/stop collections in authored/schema order.
/// Fixed side/ease/profile/stop references remain held metadata; no copy, landmark, value or anchor
/// lookup, current validation, resampling, ease realization or geometry is performed.
/// Exhaustive private owner/definition/ease views and Copy payload guards expose owned-field changes.
/// # Errors
/// Returns the original first resource refusal with actual graph/span identity and original index.
pub fn charge_sewing_graph_resources(
    graph: &SewingGraph,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (graph_id, spans) = graph.resource_spans();
    sink.charge_entries(spans.len(), location("sewing.spans", Some(graph_id), None))?;
    for (index, span) in spans.iter().enumerate() {
        let SeamSpanDefinition {
            id,
            a,
            b,
            direction,
            ease,
            stops,
        } = span.resource_definition();
        fixed_value(a);
        fixed_value(b);
        fixed_value(direction);
        let DeclaredEase {
            amount,
            distribution,
        } = ease;
        fixed_value(amount);
        match distribution {
            EaseDistribution::Uniform => {}
            EaseDistribution::Weighted { side, regions } => {
                fixed_value(side);
                fixed_slice(regions);
                sink.charge_entries(
                    regions.len(),
                    location("sewing.spans.ease.regions", Some(*id), Some(index)),
                )?;
            }
            EaseDistribution::BetweenNotches { side, start, end } => {
                fixed_value(side);
                fixed_value(start);
                fixed_value(end);
            }
        }
        fixed_slice(stops);
        sink.charge_entries(
            stops.len(),
            location("sewing.spans.stops", Some(*id), Some(index)),
        )?;
    }
    Ok(())
}
/// Charge the provided actual semantic registry slice without resolving or rebirthing any anchor.
/// Callers compose this with the root's exact owned landmark vector. Notch/Turn private views guard
/// their fixed reference payloads; profile bindings stay symbolic, without inferred style/physical values.
/// # Errors
/// Returns the original first resource refusal for sewing.landmarks (or an earlier latched error).
pub fn charge_sewing_landmark_resources(
    landmarks: &[SewingLandmark],
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    sink.charge_entries(landmarks.len(), location("sewing.landmarks", None, None))?;
    for landmark in landmarks {
        match landmark {
            SewingLandmark::Notch(notch) => {
                let (piece, definition) = notch.resource_parts();
                fixed_value(&piece);
                let NotchDefinition {
                    id,
                    anchor,
                    representation,
                } = definition;
                fixed_value(id);
                fixed_value(anchor);
                fixed_value(representation);
            }
            SewingLandmark::Turn(turn) => fixed_value(&turn.resource_fields()),
        }
    }
    Ok(())
}
