//! Consume one admitted structural candidate; preserve unrelated authored graph members.
use super::{CommandSeamSplitGraphAdmission, CommandSeamSplitGraphError};
use crate::{
    command::CommandStopChild,
    design::{charge_sewing_graph_resources, DesignReferenceHost, DesignResourceMeter},
    ontology::{SeamSideId, SeamSpanDefinition, SewingGraph, SewingGraphDefinition},
};

impl<H: DesignReferenceHost> CommandSeamSplitGraphAdmission<'_, H> {
    /// Build the current structural graph, replacing only the actual target in authored order.
    /// Children use exact captured portion pairs and ordered closed stop memberships; shared
    /// boundary stops retain their semantic IDs. Unrelated definitions remain exact. Full original
    /// ease bodies are shared, not replaced by child literals or populated from missing stops.
    ///
    /// Existing graph and history remain unchanged on success/refusal. The graph constructor,
    /// exact live/history ancestry binding and complete current namespace validation all run.
    /// Finally measure the actual graph under the original envelope and conservative reservation.
    /// This does not construct a complete Design, repair held Gathers, mint/reserve IDs, evaluate
    /// amounts/correspondence or grant authority. Those checks still precede production effects.
    /// # Errors
    /// Retains original captured portion, current graph, history binding or resource cause.
    /// No partial graph escapes any refusal.
    pub fn build(self) -> Result<SewingGraph, CommandSeamSplitGraphError> {
        use CommandSeamSplitGraphError as Error;
        let Self {
            source,
            joined,
            children,
            limits,
            reservation,
            target_index,
            span_count,
            history,
        } = self;
        let binding = source.binding();
        let portions = joined
            .capture()
            .portion()
            .split_at(
                binding.request().cut(SeamSideId::A),
                binding.request().cut(SeamSideId::B),
            )
            .map_err(Error::Portion)?;
        let definitions = portions
            .into_iter()
            .zip([CommandStopChild::First, CommandStopChild::Second])
            .zip(children)
            .map(|((portion, child), id)| SeamSpanDefinition {
                id,
                a: portion.side(SeamSideId::A),
                b: portion.side(SeamSideId::B),
                direction: portion.source().direction(),
                ease: portion.into(),
                stops: joined
                    .child(child)
                    .rows()
                    .iter()
                    .map(|row| *row.stop())
                    .collect(),
            });
        let state = source.state();
        let original = state.graph();
        let mut spans = Vec::with_capacity(span_count);
        spans.extend(
            original
                .spans()
                .iter()
                .take(target_index)
                .map(|span| span.definition().clone()),
        );
        spans.extend(definitions);
        spans.extend(
            original
                .spans()
                .iter()
                .skip(target_index + 1)
                .map(|span| span.definition().clone()),
        );
        let structure = state.inputs().structure();
        let graph = SewingGraph::new(
            SewingGraphDefinition {
                id: original.id(),
                spans,
            },
            structure.cut_plan(),
            structure.pieces().pieces(),
            state.landmarks(),
            structure.topology(),
        )
        .map_err(|issue| Error::Graph(Box::new(issue)))?
        .with_span_history(history)
        .map_err(Error::HistoryBinding)?;
        graph
            .validate_current(
                structure.cut_plan(),
                structure.pieces().pieces(),
                state.landmarks(),
                structure.topology(),
            )
            .map_err(|issue| Error::Graph(Box::new(issue)))?;
        let mut meter = DesignResourceMeter::new(limits.resources());
        charge_sewing_graph_resources(&graph, &mut meter).map_err(Error::Resources)?;
        let actual = meter
            .finish()
            .map_err(Error::Resources)?
            .collection_entries();
        let reserved = reservation.collection_entries();
        if actual > reserved {
            return Err(Error::ReservationExceeded { reserved, actual });
        }
        Ok(graph)
    }
}
