//! Bounded structural graph candidates over exact actual source and ordered stop owners.
use super::{CommandSeamSplitSourceAdmission, CommandStopSourceBinding};
use crate::{
    design::{
        charge_sewing_graph_resources, DesignReferenceHost, DesignResourceDemand,
        DesignResourceLimitError, DesignResourceLimits, DesignResourceLocation,
        DesignResourceMeter, DesignResourceScope, DesignResourceSink,
    },
    ontology::{
        EntityId, SeamEasePortionError, SeamSpanHistory, SeamSpanHistoryBindingError,
        SeamSpanHistoryError, SeamSpanHistoryLimits, SewingError,
    },
};
use core::fmt;
mod build;

/// Explicit graph payload reservation and complete next-history allowances, with no defaults.
/// These cover logical output/copy populations, not allocator bytes, temporary indices or CPU.
/// ```compile_fail
/// let limits:sc_core::command::CommandSeamSplitGraphLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandSeamSplitGraphLimits {
    resources: DesignResourceLimits,
    history: SeamSpanHistoryLimits,
}
impl CommandSeamSplitGraphLimits {
    /// Select both independent complete allowances explicitly.
    #[must_use]
    pub const fn new(resources: DesignResourceLimits, history: SeamSpanHistoryLimits) -> Self {
        Self { resources, history }
    }
    /// Selected logical graph/output envelope; no Design or operation envelope is implied.
    #[must_use]
    pub const fn resources(self) -> DesignResourceLimits {
        self.resources
    }
    /// Selected complete next history, including all previously issued identities.
    #[must_use]
    pub const fn history(self) -> SeamSpanHistoryLimits {
        self.history
    }
}

/// Exact owner, quota, identity or current structural refusal; no partial graph escapes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandSeamSplitGraphError {
    /// Equal IDs/content cannot substitute a different actual complete source state.
    SourceOwner {
        /// Graph owned by the stop pipeline's actual state.
        expected_graph: EntityId,
        /// Graph owned by the supplied source admission.
        provided_graph: EntityId,
    },
    /// The supplied complete source admission belongs to another actual request object.
    RequestOwner {
        /// Exact target retained by the stop/capture pipeline.
        target_span: EntityId,
    },
    /// The admitted target is not the actual graph member used by the stop/capture pipeline.
    TargetOwner {
        /// Exact target retained by the stop/capture pipeline.
        target_span: EntityId,
    },
    /// A legacy graph has no actual attached history; past edits are never invented.
    MissingHistory {
        /// Actual containing graph identity.
        graph: EntityId,
    },
    /// The complete next span population cannot be represented.
    SpanCountOverflow {
        /// Actual containing graph identity.
        graph: EntityId,
    },
    /// Exact complete graph reservation or final output refusal.
    Resources(DesignResourceLimitError),
    /// Exact complete next-history quota, parent or child nonreuse refusal.
    History(SeamSpanHistoryError),
    /// Exact captured portion's original frame/interior refusal.
    Portion(SeamEasePortionError),
    /// Original current graph copy, range, namespace, source, stop or distribution refusal.
    Graph(Box<SewingError>),
    /// Original attached-history/live-leaf/source-ancestry refusal.
    HistoryBinding(SeamSpanHistoryBindingError),
    /// Actual output exceeded the pre-copy conservative reservation; no accepted output escapes.
    ReservationExceeded {
        /// Successfully admitted complete logical reservation.
        reserved: usize,
        /// Actual final graph collection population.
        actual: usize,
    },
}
impl fmt::Display for CommandSeamSplitGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "seam split graph candidate: {self:?}")
    }
}
impl std::error::Error for CommandSeamSplitGraphError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Resources(issue) => Some(issue),
            Self::History(issue) => Some(issue),
            Self::Portion(issue) => Some(issue),
            Self::Graph(issue) => Some(issue.as_ref()),
            Self::HistoryBinding(issue) => Some(issue),
            Self::SourceOwner { .. }
            | Self::RequestOwner { .. }
            | Self::TargetOwner { .. }
            | Self::MissingHistory { .. }
            | Self::SpanCountOverflow { .. }
            | Self::ReservationExceeded { .. } => None,
        }
    }
}

/// Exact actual owners, admitted graph payload and one newly extended immutable history.
/// This is structural candidate preparation, not trusted identity allocation, global Design
/// ownership, numeric/physical/provider readiness, authorization or atomic publication.
/// Private fields prevent replacing the admitted child IDs:
/// ```compile_fail
/// fn forge<H>(admitted:&mut sc_core::command::CommandSeamSplitGraphAdmission<'_,H>) {
///     admitted.children = [sc_core::ontology::EntityId::from_bits(1);2];
/// }
/// ```
/// Original source ownership cannot escape:
/// ```compile_fail
/// use sc_core::command::{CommandSeamSplitGraphAdmission,CommandSeamSplitSourceAdmission};
/// fn escape<H>(admitted:&CommandSeamSplitGraphAdmission<'_,H>)
///     ->&'static CommandSeamSplitSourceAdmission<'static,H>{admitted.source()}
/// ```
pub struct CommandSeamSplitGraphAdmission<'a, H> {
    source: &'a CommandSeamSplitSourceAdmission<'a, H>,
    joined: &'a CommandStopSourceBinding<'a, H>,
    children: [EntityId; 2],
    limits: CommandSeamSplitGraphLimits,
    reservation: DesignResourceDemand,
    target_index: usize,
    span_count: usize,
    history: SeamSpanHistory,
}
impl<H> fmt::Debug for CommandSeamSplitGraphAdmission<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            source,
            joined: _,
            children,
            limits,
            reservation,
            target_index,
            span_count,
            history: _,
        } = self;
        f.debug_struct("CommandSeamSplitGraphAdmission")
            .field("target", &source.binding().span().id())
            .field("children", children)
            .field("limits", limits)
            .field("reservation", reservation)
            .field("target_index", target_index)
            .field("span_count", span_count)
            .finish()
    }
}
impl<'a, H> CommandSeamSplitGraphAdmission<'a, H> {
    /// Exact original complete source admission; no same-ID reconstruction.
    #[must_use]
    pub const fn source(&self) -> &'a CommandSeamSplitSourceAdmission<'a, H> {
        self.source
    }
    /// Exact externally owned captured source and authored ordered stop evidence.
    #[must_use]
    pub const fn stop_source(&self) -> &'a CommandStopSourceBinding<'a, H> {
        self.joined
    }
    /// Tentative supplied children in A order, without a reserve or publication capability.
    #[must_use]
    pub const fn children(&self) -> [EntityId; 2] {
        self.children
    }
    /// Actual explicit graph and history allowances.
    #[must_use]
    pub const fn limits(&self) -> CommandSeamSplitGraphLimits {
        self.limits
    }
    /// Conservative graph payload reservation admitted before any history/definition copy.
    #[must_use]
    pub const fn reservation(&self) -> DesignResourceDemand {
        self.reservation
    }
}
impl<H: DesignReferenceHost> CommandStopSourceBinding<'_, H> {
    /// Join exact complete source ownership, admit the whole graph payload and extend history.
    /// State pointer precedes request/target pointers; attached history is mandatory. Charge the
    /// actual complete source graph plus full captured-root population, child stop references,
    /// two child slots and one next-history record before copying any payload. Then admit and
    /// validate the complete next history under its independent explicit allowances.
    ///
    /// No input/landmark clone, generator, value/provider call, Design reconstruction or effect
    /// occurs. Supplied IDs are tentative metadata; complete global ownership and trusted reserve
    /// admission remain required before any production effect. Legacy histories are not inferred.
    /// ```
    /// use sc_core::command::{CommandStopSourceBinding,CommandSeamSplitSourceAdmission,
    ///     CommandSeamSplitGraphError,CommandSeamSplitGraphLimits};
    /// use sc_core::{design::DesignReferenceHost,ontology::{EntityId,SewingGraph}};
    /// fn candidate<'a,H:DesignReferenceHost>(joined:&'a CommandStopSourceBinding<'a,H>,
    ///     source:&'a CommandSeamSplitSourceAdmission<'a,H>,children:[EntityId;2],
    ///     limits:CommandSeamSplitGraphLimits)->Result<SewingGraph,CommandSeamSplitGraphError>{
    ///     joined.admit_graph_candidate(source,children,limits)?.build()
    /// }
    /// ```
    /// # Errors
    /// Preserves actual owner mismatch, missing attached history, complete resource/history
    /// counts and original parent/child evidence. Neither a partial token nor a graph escapes.
    pub fn admit_graph_candidate<'a>(
        &'a self,
        source: &'a CommandSeamSplitSourceAdmission<'a, H>,
        children: [EntityId; 2],
        limits: CommandSeamSplitGraphLimits,
    ) -> Result<CommandSeamSplitGraphAdmission<'a, H>, CommandSeamSplitGraphError> {
        use CommandSeamSplitGraphError as Error;
        let inputs = self
            .partitions()
            .memberships()
            .context()
            .ownership()
            .inputs();
        let state = source.state();
        if !core::ptr::eq(inputs.state(), state) {
            return Err(Error::SourceOwner {
                expected_graph: inputs.state().graph().id(),
                provided_graph: state.graph().id(),
            });
        }
        let binding = self.capture().binding();
        if !core::ptr::eq(binding.request(), source.binding().request()) {
            return Err(Error::RequestOwner {
                target_span: binding.span().id(),
            });
        }
        if !core::ptr::eq(binding.span(), source.binding().span()) {
            return Err(Error::TargetOwner {
                target_span: binding.span().id(),
            });
        }
        let graph = state.graph();
        let target_index = graph
            .spans()
            .iter()
            .position(|span| core::ptr::eq(span, binding.span()))
            .ok_or(Error::TargetOwner {
                target_span: binding.span().id(),
            })?;
        let history = graph
            .span_history()
            .ok_or(Error::MissingHistory { graph: graph.id() })?;
        let span_count = graph
            .spans()
            .len()
            .checked_add(1)
            .ok_or(Error::SpanCountOverflow { graph: graph.id() })?;
        let mut meter = DesignResourceMeter::new(limits.resources);
        charge_sewing_graph_resources(graph, &mut meter).map_err(Error::Resources)?;
        for (field, count) in [
            ("candidate.span_split.source", self.capture().item_count()),
            (
                "candidate.span_split.stops",
                self.partitions().reference_count(),
            ),
            ("candidate.span_split.children_and_history", 3),
        ] {
            meter
                .charge_entries(
                    count,
                    DesignResourceLocation {
                        scope: DesignResourceScope::Sewing,
                        owner: Some(graph.id()),
                        field,
                        index: None,
                    },
                )
                .map_err(Error::Resources)?;
        }
        let reservation = meter.finish().map_err(Error::Resources)?;
        let history = history
            .with_split(binding.span().id(), children, limits.history)
            .map_err(Error::History)?;
        Ok(CommandSeamSplitGraphAdmission {
            source,
            joined: self,
            children,
            limits,
            reservation,
            target_index,
            span_count,
            history,
        })
    }
}
