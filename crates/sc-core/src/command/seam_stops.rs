//! Explicit actual stop partition inputs and conservative workspace admission before queries.
use super::{CommandSeamSplitBinding, CommandSeamSplitBindingError, SplitSeamSpan};
use crate::{
    design::{
        charge_design_sewing_resources, DesignReferenceHost, DesignResourceDemand,
        DesignResourceLimitError, DesignResourceLimits, DesignResourceMeter, DesignResourceSource,
        DesignSewing,
    },
    ontology::{EntityId, PointJournalAdmissionIssue, RangeCoverageLimits, UniquePointLimits},
};
use core::{fmt, num::NonZeroUsize};
mod current;
pub use current::{
    CommandStopContextBoundary, CommandStopContextError, CommandStopContextIssue,
    CommandStopContextPopulation, CommandStopContextRange, CommandStopCurrentContext,
    CommandStopLandmarkEvidence, CommandStopPiecePortion,
};
mod membership;
mod ownership;
mod partition;
mod source;
pub use membership::{
    CommandStopMembership, CommandStopMembershipError, CommandStopMembershipIssue,
    CommandStopMemberships, CommandStopPairMembership,
};
pub use ownership::{
    CommandStopOwnershipAdmission, CommandStopOwnershipAdmissionError, CommandStopOwnershipLimits,
};
pub use partition::{
    CommandStopChild, CommandStopChildView, CommandStopPartitionError, CommandStopPartitions,
};
pub use source::{CommandStopSourceBinding, CommandStopSourceBindingError};

/// Explicit logical workspace and point/range input limits;no defaults or currentness proof.
/// Workspace covers full registry index,target stop rows,two child references per stop and four
/// range caches at the selected per-query peak. It is not an allocator,RSS,CPU or operation cap.
/// ```compile_fail
/// let limits:sc_core::command::CommandStopPartitionLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandStopPartitionLimits {
    workspace_items: NonZeroUsize,
    current: UniquePointLimits,
    range_portions: NonZeroUsize,
}
impl CommandStopPartitionLimits {
    /// Choose positive workspace,complete journal/nested input and peak per-range allowances.
    #[must_use]
    pub const fn new(
        workspace_items: NonZeroUsize,
        current: UniquePointLimits,
        range_portions: NonZeroUsize,
    ) -> Self {
        Self {
            workspace_items,
            current,
            range_portions,
        }
    }
    /// Selected aggregate conservative workspace population allowance.
    #[must_use]
    pub const fn workspace_items(self) -> NonZeroUsize {
        self.workspace_items
    }
    /// Complete journal/nested input limits for later current-point queries.
    #[must_use]
    pub const fn current(self) -> UniquePointLimits {
        self.current
    }
    /// Peak portions per later range query,including active/transient work.
    #[must_use]
    pub const fn range_portions(self) -> NonZeroUsize {
        self.range_portions
    }
    /// Existing bounded coverage limits with the exact same complete journal allowance.
    #[must_use]
    pub const fn range_limits(self) -> RangeCoverageLimits {
        RangeCoverageLimits::new(self.current, self.range_portions)
    }
}
/// Original model/target/journal cause or conservative workspace refusal,before derived queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandStopPartitionAdmissionError {
    /// Actual complete represented model envelope refuses before target lookup.
    Model(DesignResourceLimitError),
    /// Actual target lookup or original A/B source-frame/interior cut refusal.
    Binding(CommandSeamSplitBindingError),
    /// Complete journal/nested admission refuses before workspace arithmetic or current queries.
    Journal {
        /// Actual consuming target.
        target_span: EntityId,
        /// Complete original typed journal cause;this evidence has no standard Error implementation.
        issue: PointJournalAdmissionIssue,
    },
    /// Conservative M+N+2N+4B arithmetic cannot be represented;no allocation/query occurs.
    BoundOverflow {
        /// Actual consuming target.
        target_span: EntityId,
        /// Actual target's complete authored stop count,not full ease-root stops.
        stops: usize,
        /// Complete actual semantic registry,including unused entries.
        landmarks: usize,
        /// Selected peak portions per one range query.
        range_portions: usize,
    },
    /// Conservative complete logical workspace exceeds the independently selected allowance.
    Workspace {
        /// Actual consuming target.
        target_span: EntityId,
        /// Actual target stop count N.
        stops: usize,
        /// Complete registry count M.
        landmarks: usize,
        /// Selected per-range peak B.
        range_portions: usize,
        /// Checked M+N+2N+4B reservation.
        required: usize,
        /// Actual caller-selected allowance.
        limit: usize,
    },
}
impl fmt::Display for CommandStopPartitionAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command stop partition input admission: {self:?}")
    }
}
impl std::error::Error for CommandStopPartitionAdmissionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Model(issue) => Some(issue),
            Self::Binding(issue) => Some(issue),
            Self::Journal {
                target_span: _,
                issue: _,
            }
            | Self::BoundOverflow {
                target_span: _,
                stops: _,
                landmarks: _,
                range_portions: _,
            }
            | Self::Workspace {
                target_span: _,
                stops: _,
                landmarks: _,
                range_portions: _,
                required: _,
                limit: _,
            } => None,
        }
    }
}

/// Actual borrowed command/owned sewing inputs and separately admitted fixed workspace bounds.
/// This stage performs no semantic/current validation,point/range query,index,projection or source
/// clone. Held damaged references remain inspectable. Subsequent bounded validation must establish
/// actual copy/point/range membership before a stop partition or effect can be accepted.
///
/// Private fields prevent fabricated admitted inputs:
/// ```compile_fail
/// use sc_core::command::{CommandStopPartitionAdmission,CommandStopPartitionLimits,CommandSeamSplitBinding};
/// use sc_core::design::{DesignSewing,DesignResourceDemand,DesignResourceLimits};
/// fn forge<'a,H>(state:&'a DesignSewing<H>,binding:CommandSeamSplitBinding<'a>,
///     demand:DesignResourceDemand,model_limits:DesignResourceLimits,limits:CommandStopPartitionLimits)
///     ->CommandStopPartitionAdmission<'a,H>{
///     CommandStopPartitionAdmission {state,binding,demand,model_limits,limits,stops:0,landmarks:0,
///         range_cache_items:0,child_reference_items:0,workspace_items:0}
/// }
/// ```
/// The actual input owner's borrow cannot escape:
/// ```compile_fail
/// use sc_core::command::{SplitSeamSpan,CommandStopPartitionAdmission,CommandStopPartitionLimits};
/// use sc_core::design::{DesignSewing,DesignReferenceHost,DesignResourceSource,DesignResourceLimits};
/// fn escape<H:DesignReferenceHost+DesignResourceSource>(request:&SplitSeamSpan,state:&DesignSewing<H>,
///     model:DesignResourceLimits,limits:CommandStopPartitionLimits)->CommandStopPartitionAdmission<'static,H>{
///     request.admit_stop_partition_inputs(state,model,limits).unwrap()
/// }
/// ```
pub struct CommandStopPartitionAdmission<'a, H> {
    state: &'a DesignSewing<H>,
    binding: CommandSeamSplitBinding<'a>,
    demand: DesignResourceDemand,
    model_limits: DesignResourceLimits,
    limits: CommandStopPartitionLimits,
    stops: usize,
    landmarks: usize,
    range_cache_items: usize,
    child_reference_items: usize,
    workspace_items: usize,
}
impl<H> fmt::Debug for CommandStopPartitionAdmission<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            state: _,
            binding,
            demand,
            model_limits,
            limits,
            stops,
            landmarks,
            range_cache_items,
            child_reference_items,
            workspace_items,
        } = self;
        f.debug_struct("CommandStopPartitionAdmission")
            .field("target", &binding.span().id())
            .field("demand", demand)
            .field("model_limits", model_limits)
            .field("limits", limits)
            .field("stops", stops)
            .field("landmarks", landmarks)
            .field("range_cache_items", range_cache_items)
            .field("child_reference_items", child_reference_items)
            .field("workspace_items", workspace_items)
            .finish()
    }
}
impl<'a, H> CommandStopPartitionAdmission<'a, H> {
    /// Actual immutable owned sewing/input/semantic registry owner.
    #[must_use]
    pub const fn state(&self) -> &'a DesignSewing<H> {
        self.state
    }
    /// Actual original request/target/cuts,without currentness certification.
    #[must_use]
    pub const fn binding(&self) -> CommandSeamSplitBinding<'a> {
        self.binding
    }
    /// Exact complete represented model demand derived before lookup or semantic work.
    #[must_use]
    pub const fn model_demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Actual caller-selected complete model envelope.
    #[must_use]
    pub const fn model_limits(&self) -> DesignResourceLimits {
        self.model_limits
    }
    /// Actual separately chosen logical workspace and point/range limits.
    #[must_use]
    pub const fn limits(&self) -> CommandStopPartitionLimits {
        self.limits
    }
    /// Complete authored target stop population N;original ease-root stops stay separate.
    #[must_use]
    pub const fn stop_count(&self) -> usize {
        self.stops
    }
    /// Complete semantic registry population M,including unused entries.
    #[must_use]
    pub const fn landmark_count(&self) -> usize {
        self.landmarks
    }
    /// Four actual pair-side range caches at selected per-query peak B.
    #[must_use]
    pub const fn range_cache_item_bound(&self) -> usize {
        self.range_cache_items
    }
    /// One metadata/membership row per actual target stop N.
    #[must_use]
    pub const fn membership_item_bound(&self) -> usize {
        self.stops
    }
    /// Conservative total child references2N,including a shared cut on both closed children.
    #[must_use]
    pub const fn child_reference_item_bound(&self) -> usize {
        self.child_reference_items
    }
    /// Complete conservative M+N+2N+4B workspace reservation.
    #[must_use]
    pub const fn workspace_item_bound(&self) -> usize {
        self.workspace_items
    }
}
impl SplitSeamSpan {
    /// Admit actual stop partition inputs before semantic/current work or derived allocation.
    /// Charge the complete actual model,bind real held target/cuts,admit complete journal/nested
    /// input,then check conservative M+N+2N+4B workspace. M is the complete semantic registry,N
    /// the actual target stop list and B the selected range peak. Full original ease-root stops
    /// cannot be substituted for a retained target's authored list. No current callback,point/range
    /// query,index,source clone or numeric/provider lookup is performed. Later validation remains
    /// necessary even when the held model's resources and original cuts admit.
    /// ```
    /// use sc_core::command::{SplitSeamSpan,CommandStopPartitionAdmission,
    ///     CommandStopPartitionAdmissionError,CommandStopPartitionLimits};
    /// use sc_core::design::{DesignSewing,DesignReferenceHost,DesignResourceSource,DesignResourceLimits};
    /// fn admit<'a,H:DesignReferenceHost+DesignResourceSource>(request:&'a SplitSeamSpan,
    ///     state:&'a DesignSewing<H>,model:DesignResourceLimits,limits:CommandStopPartitionLimits)
    ///     ->Result<CommandStopPartitionAdmission<'a,H>,CommandStopPartitionAdmissionError>{
    ///     request.admit_stop_partition_inputs(state,model,limits)
    /// }
    /// ```
    /// # Errors
    /// Preserves original complete model refusal,then target/A/B cuts,then original complete
    /// journal/nested cause,then checked workspace overflow or selected-limit refusal. No partial
    /// accepted input token,derived workspace or currentness proof escapes on refusal.
    pub fn admit_stop_partition_inputs<'a, H: DesignReferenceHost + DesignResourceSource>(
        &'a self,
        state: &'a DesignSewing<H>,
        model_limits: DesignResourceLimits,
        limits: CommandStopPartitionLimits,
    ) -> Result<CommandStopPartitionAdmission<'a, H>, CommandStopPartitionAdmissionError> {
        let mut meter = DesignResourceMeter::new(model_limits);
        charge_design_sewing_resources(state, &mut meter)
            .map_err(CommandStopPartitionAdmissionError::Model)?;
        let demand = meter
            .finish()
            .map_err(CommandStopPartitionAdmissionError::Model)?;
        let binding = self
            .bind_source_cuts(state.graph())
            .map_err(CommandStopPartitionAdmissionError::Binding)?;
        let target_span = binding.span().id();
        limits
            .current
            .admit_journal(state.inputs().structure().topology())
            .map_err(|issue| CommandStopPartitionAdmissionError::Journal { target_span, issue })?;
        let stops = binding.span().definition().stops.len();
        let landmarks = state.landmarks().len();
        let overflow = || CommandStopPartitionAdmissionError::BoundOverflow {
            target_span,
            stops,
            landmarks,
            range_portions: limits.range_portions.get(),
        };
        let range_cache_items = limits
            .range_portions
            .get()
            .checked_mul(4)
            .ok_or_else(overflow)?;
        let child_reference_items = stops.checked_mul(2).ok_or_else(overflow)?;
        let workspace_items = landmarks
            .checked_add(stops)
            .and_then(|count| count.checked_add(child_reference_items))
            .and_then(|count| count.checked_add(range_cache_items))
            .ok_or_else(overflow)?;
        if workspace_items > limits.workspace_items.get() {
            return Err(CommandStopPartitionAdmissionError::Workspace {
                target_span,
                stops,
                landmarks,
                range_portions: limits.range_portions.get(),
                required: workspace_items,
                limit: limits.workspace_items.get(),
            });
        }
        Ok(CommandStopPartitionAdmission {
            state,
            binding,
            demand,
            model_limits,
            limits,
            stops,
            landmarks,
            range_cache_items,
            child_reference_items,
            workspace_items,
        })
    }
}
