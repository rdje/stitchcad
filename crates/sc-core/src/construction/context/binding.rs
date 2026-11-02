//! Bounded actual-owner context binding; no accepted program or geometry values.
use super::{DraftingContextSource, DraftingSourceRecord};
use crate::{
    command::DesignRevision,
    design::{
        DesignFormulaScope, DesignInputs, DesignMetadataHost, DesignResourceDemand,
        DesignResourceLimitError, DesignResourceLimits, DesignResourceMeter,
    },
};
use core::fmt;

/// Original context-admission refusal, retaining actual owners/revisions or the typed source cause.
pub enum DraftingContextError<'a, H: DesignMetadataHost, E> {
    /// The source belongs to a different actual owner, even if its ids/revisions compare equal.
    ForeignOwner {
        /// Actual owner pinned by the containing scalar/input scope.
        expected: &'a DesignInputs<H>,
        /// Actual owner returned by the provided source port.
        actual: &'a DesignInputs<H>,
    },
    /// Complete held context content exceeds the explicit selected allowance.
    Resources(DesignResourceLimitError),
    /// Snapshot revision differs from the exact actual containing Design revision.
    RevisionMismatch {
        /// Actual containing Design revision, borrowed without copying its counter.
        expected: &'a DesignRevision,
        /// Actual snapshot revision, borrowed after its content has been charged.
        actual: &'a DesignRevision,
    },
    /// Original current-source refusal after exact ownership/content/revision checks.
    Source(E),
}
impl<H: DesignMetadataHost, E: fmt::Debug> fmt::Debug for DraftingContextError<'_, H, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForeignOwner { expected, actual } => f
                .debug_struct("ForeignOwner")
                .field("expected", &expected.id())
                .field("actual", &actual.id())
                .finish(),
            Self::Resources(error) => f.debug_tuple("Resources").field(error).finish(),
            Self::RevisionMismatch { expected, actual } => f
                .debug_struct("RevisionMismatch")
                .field("expected", expected)
                .field("actual", actual)
                .finish(),
            Self::Source(error) => f.debug_tuple("Source").field(error).finish(),
        }
    }
}
impl<H: DesignMetadataHost, E: fmt::Display> fmt::Display for DraftingContextError<'_, H, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForeignOwner { .. } => {
                f.write_str("drafting source belongs to another Design owner")
            }
            Self::Resources(error) => write!(f, "drafting context content: {error}"),
            Self::RevisionMismatch { .. } => {
                f.write_str("drafting snapshot revision differs from Design")
            }
            Self::Source(error) => write!(f, "drafting context source: {error}"),
        }
    }
}
impl<H: DesignMetadataHost, E: std::error::Error + 'static> std::error::Error
    for DraftingContextError<'_, H, E>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Resources(error) => Some(error),
            Self::Source(error) => Some(error),
            Self::ForeignOwner { .. } | Self::RevisionMismatch { .. } => None,
        }
    }
}

/// Actual-root binding under complete context-content limits and the immutable source contract.
///
/// This pins the original scalar/input scope, source provider, authored Cartesian frame and current
/// snapshot. Provider completeness/currentness relies on the honest immutable source-port contract.
/// It grants no accepted program, existing-output binding, geometry values, numeric/physical
/// feasibility, runtime budget or release approval. The root and context demands remain separate;
/// whole-program admission must compose them with its own complete retained content.
///
/// Public callers cannot replace the held source after binding:
/// ```compile_fail
/// use sc_core::{construction::{DraftingContext, DraftingContextSource}, design::DesignMetadataHost};
/// fn replace<'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     context:&mut DraftingContext<'a,H,S>, other:&'a S) { context.source=other; }
/// ```
/// A bound context cannot outlive its actual input scope and source provider:
/// ```compile_fail
/// use sc_core::{construction::{DraftingContext, DraftingContextError, DraftingContextSource},
///     design::{DesignFormulaScope, DesignMetadataHost, DesignResourceLimits}};
/// fn escape<'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     scope:DesignFormulaScope<'a,H>, source:&'a S, limits:DesignResourceLimits)
///     ->Result<DraftingContext<'static,H,S>,DraftingContextError<'a,H,S::Error>> {
///     scope.bind_drafting_context(source,limits)
/// }
/// ```
pub struct DraftingContext<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> {
    scope: DesignFormulaScope<'a, H>,
    source: &'a S,
    frame: &'a DraftingSourceRecord<S::Frame>,
    snapshot: &'a DraftingSourceRecord<S::Snapshot>,
    limits: DesignResourceLimits,
    demand: DesignResourceDemand,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingContext<'_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingContext<'_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingContext<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingContext")
            .field("root", &self.scope.root().id())
            .field("limits", &self.limits)
            .field("demand", &self.demand)
            .finish_non_exhaustive()
    }
}
impl<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> DraftingContext<'a, H, S> {
    /// Original actual scalar/input scope, not an equal-looking replacement model.
    #[must_use]
    pub const fn scope(self) -> DesignFormulaScope<'a, H> {
        self.scope
    }
    /// Original immutable source provider; this accessor grants no mutation.
    #[must_use]
    pub const fn source(self) -> &'a S {
        self.source
    }
    /// Actual explicitly selected authored Cartesian frame source.
    #[must_use]
    pub const fn frame(self) -> &'a DraftingSourceRecord<S::Frame> {
        self.frame
    }
    /// Actual immutable current snapshot under the source-port contract.
    #[must_use]
    pub const fn snapshot(self) -> &'a DraftingSourceRecord<S::Snapshot> {
        self.snapshot
    }
    /// Explicit context-content allowance, separate from the original Design scope's allowance.
    #[must_use]
    pub const fn limits(self) -> DesignResourceLimits {
        self.limits
    }
    /// Complete successfully charged context demand under the exhaustive provider contract.
    #[must_use]
    pub const fn demand(self) -> DesignResourceDemand {
        self.demand
    }
}

impl<'a, H: DesignMetadataHost> DesignFormulaScope<'a, H> {
    /// Bind an actual immutable drafting source after complete context logical-content admission.
    ///
    /// Compare the actual owner first, then charge complete content and finish the first-failure
    /// meter before comparing revisions or validating sources. Even a provider that ignores a sink
    /// refusal cannot clear it by returning success or a later error. No accepted context prefix,
    /// program, registry copy, argument normalization or geometry/numeric query escapes a refusal.
    /// # Errors
    /// Returns the actual first owner/content/revision/current-source refusal and original witnesses.
    pub fn bind_drafting_context<S: DraftingContextSource<H> + ?Sized>(
        self,
        source: &'a S,
        limits: DesignResourceLimits,
    ) -> Result<DraftingContext<'a, H, S>, DraftingContextError<'a, H, S::Error>> {
        let actual = source.root();
        if !core::ptr::eq(self.root(), actual) {
            return Err(DraftingContextError::ForeignOwner {
                expected: self.root(),
                actual,
            });
        }
        let mut meter = DesignResourceMeter::new(limits);
        let charged = source.charge_resources(&mut meter);
        let demand = meter.finish().map_err(DraftingContextError::Resources)?;
        charged.map_err(DraftingContextError::Resources)?;
        let actual = source.snapshot_revision();
        if actual != self.root().revision() {
            return Err(DraftingContextError::RevisionMismatch {
                expected: self.root().revision(),
                actual,
            });
        }
        source
            .validate_current()
            .map_err(DraftingContextError::Source)?;
        Ok(DraftingContext {
            scope: self,
            source,
            frame: source.frame(),
            snapshot: source.snapshot(),
            limits,
            demand,
        })
    }
}
