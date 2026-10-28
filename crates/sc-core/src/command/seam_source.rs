//! Complete actual owned sewing source preflight before command source capture.
use super::{
    CommandEaseCaptureAdmission, CommandEaseCaptureError, CommandEaseCaptureLimits,
    CommandSeamSplitBinding, CommandSeamSplitBindingError, SplitSeamSpan,
};
use crate::design::{
    charge_design_sewing_resources, DesignReferenceHost, DesignResourceDemand,
    DesignResourceLimitError, DesignResourceLimits, DesignResourceMeter, DesignResourceSource,
    DesignSewing, DesignSewingAmount, DesignSewingError,
};
use core::fmt;

/// Original complete resource/current-source/target/capture refusal,in that order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandSeamSplitSourceError<E> {
    /// Complete actual model envelope refuses before semantic preflight or capture cloning.
    Resources(DesignResourceLimitError),
    /// Actual complete ownership/root/landmark/graph/amount metadata refusal.
    Current(Box<DesignSewingError<E>>),
    /// Actual target lookup or original source-frame/interior cut refusal.
    Binding(CommandSeamSplitBindingError),
    /// Complete original capture population refusal,without materializing that source.
    Capture(CommandEaseCaptureError),
}
impl<E: fmt::Display> fmt::Display for CommandSeamSplitSourceError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resources(issue) => write!(f, "seam split source resources: {issue}"),
            Self::Current(issue) => write!(f, "seam split current source: {issue}"),
            Self::Binding(issue) => write!(f, "seam split source binding: {issue}"),
            Self::Capture(issue) => write!(f, "seam split source capture: {issue}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for CommandSeamSplitSourceError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Resources(issue) => Some(issue),
            Self::Current(issue) => Some(issue.as_ref()),
            Self::Binding(issue) => Some(issue),
            Self::Capture(issue) => Some(issue),
        }
    }
}

/// Borrowed actual owned sewing state,selected declaration and admitted complete source capture.
/// This is immutable source preflight,not a bus authorization,numeric/physical certificate or
/// prepared effect. The model resource walk relies on the actual source host's exhaustive readonly
/// accounting contract;its logical bounds are distinct from aggregate runtime/allocator/CPU caps.
///
/// Private fields prevent fabricated successful source admission:
/// ```compile_fail
/// use sc_core::command::{CommandSeamSplitSourceAdmission,CommandEaseCaptureAdmission};
/// use sc_core::design::{DesignSewing,DesignSewingAmount,DesignResourceDemand,DesignResourceLimits};
/// fn forge<'a,H>(state:&'a DesignSewing<H>,amount:DesignSewingAmount<'a>,
///     capture:CommandEaseCaptureAdmission<'a>,demand:DesignResourceDemand,limits:DesignResourceLimits)
///     ->CommandSeamSplitSourceAdmission<'a,H>{
///     CommandSeamSplitSourceAdmission {state,amount,capture,demand,limits}
/// }
/// ```
/// The actual source owner borrow cannot escape:
/// ```compile_fail
/// use sc_core::command::{SplitSeamSpan,CommandSeamSplitSourceAdmission,CommandEaseCaptureLimits};
/// use sc_core::design::{DesignReferenceHost,DesignResourceSource,DesignResourceLimits,DesignSewing};
/// fn escape<H:DesignReferenceHost+DesignResourceSource>(request:&SplitSeamSpan,
///     state:&DesignSewing<H>,limits:DesignResourceLimits,capture:CommandEaseCaptureLimits)
///     ->CommandSeamSplitSourceAdmission<'static,H>{
///     request.admit_sewing_source(state,limits,capture).unwrap()
/// }
/// ```
pub struct CommandSeamSplitSourceAdmission<'a, H> {
    state: &'a DesignSewing<H>,
    amount: DesignSewingAmount<'a>,
    capture: CommandEaseCaptureAdmission<'a>,
    demand: DesignResourceDemand,
    limits: DesignResourceLimits,
}
impl<H> fmt::Debug for CommandSeamSplitSourceAdmission<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            state: _,
            amount: _,
            capture,
            demand,
            limits,
        } = self;
        f.debug_struct("CommandSeamSplitSourceAdmission")
            .field("target", &capture.binding().span().id())
            .field("source", &capture.source_span())
            .field("demand", demand)
            .field("limits", limits)
            .finish()
    }
}
impl<'a, H> CommandSeamSplitSourceAdmission<'a, H> {
    /// Actual immutable owned input/graph/landmark state,not a same-ID reconstruction.
    #[must_use]
    pub const fn state(&self) -> &'a DesignSewing<H> {
        self.state
    }
    /// Actual target's full original amount metadata over its canonical owner.
    /// Formula stays an actual Length-kind declaration;Profile stays DeferredToG4.
    #[must_use]
    pub const fn amount_declaration(&self) -> DesignSewingAmount<'a> {
        self.amount
    }
    /// Actual request/span and fixed original projected pairs.
    #[must_use]
    pub const fn binding(&self) -> CommandSeamSplitBinding<'a> {
        self.capture.binding()
    }
    /// Complete original metadata capture admission;materialize only after this stage.
    /// Its per-capture population cap does not replace aggregate operation/snapshot accounting.
    #[must_use]
    pub const fn capture_admission(&self) -> CommandEaseCaptureAdmission<'a> {
        self.capture
    }
    /// Exact actual model demand derived before semantic source validation.
    #[must_use]
    pub const fn model_demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Actual caller-selected complete model envelope.
    #[must_use]
    pub const fn model_limits(&self) -> DesignResourceLimits {
        self.limits
    }
}
impl SplitSeamSpan {
    /// Admit complete actual model resources,current sewing sources,target cuts and source capture.
    /// Walk the actual immutable source host and all owned model populations under explicit limits
    /// before semantic preflight. Preserve complete current root/unused-landmark/graph/amount errors
    /// before target selection. Then bind actual cuts and admit full original capture before clone.
    ///
    /// The returned token lends the actual selected amount declaration and capture admission.
    /// Materialize the latter externally,then use existing weighted/original-notch quantity stages.
    /// Valid formula metadata is not a numeric value;logical Profiles remain deferred. No authority,
    /// whole garment/physical geometry,aggregate runtime or paired effect readiness is implied.
    /// ```
    /// use sc_core::command::{SplitSeamSpan,CommandSeamSplitSourceAdmission,
    ///     CommandSeamSplitSourceError,CommandEaseCaptureLimits};
    /// use sc_core::design::{DesignReferenceHost,DesignResourceSource,DesignResourceLimits,DesignSewing};
    /// fn admit<'a,H:DesignReferenceHost+DesignResourceSource>(request:&'a SplitSeamSpan,
    ///     state:&'a DesignSewing<H>,limits:DesignResourceLimits,capture:CommandEaseCaptureLimits)
    ///     ->Result<CommandSeamSplitSourceAdmission<'a,H>,CommandSeamSplitSourceError<H::Error>>{
    ///     request.admit_sewing_source(state,limits,capture)
    /// }
    /// ```
    /// # Errors
    /// Preserves original resource cause,then complete current source cause,then actual target/cut
    /// and capture refusal. No partial successful token or owned source clone on refusal.
    pub fn admit_sewing_source<'a, H: DesignReferenceHost + DesignResourceSource>(
        &'a self,
        state: &'a DesignSewing<H>,
        limits: DesignResourceLimits,
        capture_limits: CommandEaseCaptureLimits,
    ) -> Result<CommandSeamSplitSourceAdmission<'a, H>, CommandSeamSplitSourceError<H::Error>> {
        let mut meter = DesignResourceMeter::new(limits);
        charge_design_sewing_resources(state, &mut meter)
            .map_err(CommandSeamSplitSourceError::Resources)?;
        let demand = meter
            .finish()
            .map_err(CommandSeamSplitSourceError::Resources)?;
        let amount = state
            .amount_declarations()
            .map_err(|issue| CommandSeamSplitSourceError::Current(Box::new(issue)))?
            .into_iter()
            .find(|amount| amount.span().id() == self.span())
            .ok_or(CommandSeamSplitSourceError::Binding(
                CommandSeamSplitBindingError::MissingSpan { span: self.span() },
            ))?;
        let binding = self
            .bind_source_cuts(state.graph())
            .map_err(CommandSeamSplitSourceError::Binding)?;
        let capture = binding
            .admit_ease_capture(capture_limits)
            .map_err(CommandSeamSplitSourceError::Capture)?;
        Ok(CommandSeamSplitSourceAdmission {
            state,
            amount,
            capture,
            demand,
            limits,
        })
    }
}
