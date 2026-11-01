//! Explicit refit/source-preserving requests;neither is an achieved geometric outcome.
use super::{
    BoundTruingSourceDomain, CoveredTruingSourceDomain, TruingSourceCoverageLimits,
    TruingSourceCoverageRefusal,
};
use core::fmt;

/// Required outcome,never inferred from empty candidates,coverage or fitted geometry.
/// ```compile_fail
/// let intent:sc_core::construction::TruingOutcomeIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingOutcomeIntent {
    /// Require the exact original Piece/copies/span/ease/stops/history,with no new candidate edges.
    /// Actual unchanged geometry,fit and observer/currentness evidence remain required.
    PreserveSource,
    /// Require refitting with a complete exact selected-source candidate partition.
    /// Coverage neither proves physical change nor chooses a deterministic fitted curve.
    Refit,
}

/// First whole candidate quota,source-preservation contradiction or exact refit partition cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingOutcomeRefusal {
    /// Complete original candidates exceed the explicit allowance,before outcome inspection.
    Candidates {
        /// Actual whole population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Source preservation cannot declare new candidate edges,even with complete source coverage.
    NewEdges {
        /// Actual complete candidate population.
        count: usize,
    },
    /// Refit's original exact partition admission cause;no partial coverage escapes.
    Coverage(TruingSourceCoverageRefusal),
}

/// Complete actual membership context,explicit intent/resource policy and original first cause.
pub struct TruingOutcomeError<'a> {
    source_domain: &'a BoundTruingSourceDomain<'a>,
    intent: TruingOutcomeIntent,
    limits: TruingSourceCoverageLimits,
    refusal: TruingOutcomeRefusal,
}
impl<'a> TruingOutcomeError<'a> {
    /// Actual complete membership/candidate/role/copy/Piece/source parents.
    #[must_use]
    pub const fn source_domain(&self) -> &'a BoundTruingSourceDomain<'a> {
        self.source_domain
    }
    /// Original explicit requested outcome,not an achieved effect.
    #[must_use]
    pub const fn intent(&self) -> TruingOutcomeIntent {
        self.intent
    }
    /// Original whole candidate/index policy;PreserveSource creates no index.
    #[must_use]
    pub const fn limits(&self) -> TruingSourceCoverageLimits {
        self.limits
    }
    /// First whole resource,preservation or refit partition cause.
    #[must_use]
    pub const fn refusal(&self) -> TruingOutcomeRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingOutcomeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingOutcomeError")
            .field("intent", &self.intent)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingOutcomeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing outcome requirement: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingOutcomeError<'_> {}

impl<'s> BoundTruingSourceDomain<'s> {
    /// Require explicit source preservation or refitting in the actual complete original context.
    /// Whole N quota precedes either branch. PreserveSource requires N0,allocates nothing and
    /// requests exact original source retention,including active role/history and physical copies.
    /// Refit delegates complete N/W and exact partition admission to bind_source_coverage,retaining
    /// its original cause and authentic result. No new identities,evaluated fit,no-op verdict,
    /// current source/observer truth or editing authority is established by either branch.
    /// # Errors
    /// Whole candidate quota;then PreserveSource's new candidates,or Refit's original partition cause.
    pub fn require_outcome<'a>(
        &'a self,
        intent: TruingOutcomeIntent,
        limits: TruingSourceCoverageLimits,
    ) -> Result<RequiredTruingOutcome<'a>, TruingOutcomeError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| TruingOutcomeError {
            source_domain: self,
            intent,
            limits,
            refusal,
        };
        let n = self.edge_count();
        if n > limits.candidates() {
            return Err(refuse(TruingOutcomeRefusal::Candidates {
                required: n,
                allowed: limits.candidates(),
            }));
        }
        let coverage = match intent {
            TruingOutcomeIntent::PreserveSource => {
                if n != 0 {
                    return Err(refuse(TruingOutcomeRefusal::NewEdges { count: n }));
                }
                None
            }
            TruingOutcomeIntent::Refit => Some(
                self.bind_source_coverage(limits)
                    .map_err(|cause| refuse(TruingOutcomeRefusal::Coverage(cause.refusal())))?,
            ),
        };
        Ok(RequiredTruingOutcome {
            source_domain: self,
            intent,
            limits,
            coverage,
        })
    }
}

/// Authentic explicit outcome request,not an achieved unchanged/refitted or publishable result.
/// ```compile_fail
/// fn forge<'a>(mut outcome:sc_core::construction::RequiredTruingOutcome<'a>,
///     other:&'a sc_core::construction::BoundTruingSourceDomain<'a>) {outcome.source_domain=other;}
/// ```
/// ```compile_fail
/// fn escape(source:sc_core::construction::BoundTruingSourceDomain<'_>)
///     ->sc_core::construction::RequiredTruingOutcome<'static> {
///     source.require_outcome(sc_core::construction::TruingOutcomeIntent::PreserveSource,
///         sc_core::construction::TruingSourceCoverageLimits::new(0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct RequiredTruingOutcome<'a> {
    source_domain: &'a BoundTruingSourceDomain<'a>,
    intent: TruingOutcomeIntent,
    limits: TruingSourceCoverageLimits,
    coverage: Option<CoveredTruingSourceDomain<'a>>,
}
impl<'a> RequiredTruingOutcome<'a> {
    /// Actual complete membership/candidate/role/copy/Piece/source parents.
    #[must_use]
    pub const fn source_domain(&self) -> &'a BoundTruingSourceDomain<'a> {
        self.source_domain
    }
    /// Original explicit requested outcome,never a physical-effect verdict.
    #[must_use]
    pub const fn intent(&self) -> TruingOutcomeIntent {
        self.intent
    }
    /// Original explicit whole candidate/index policy.
    #[must_use]
    pub const fn limits(&self) -> TruingSourceCoverageLimits {
        self.limits
    }
    /// Authentic exact refit partition,or None only for explicit N0 source preservation.
    #[must_use]
    pub const fn coverage(&self) -> Option<&CoveredTruingSourceDomain<'a>> {
        self.coverage.as_ref()
    }
}
