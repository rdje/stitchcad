//! Complete declared source-identity requests;binding does not authorize a global edit.
use super::super::{
    AdmittedSlashSpreadContours, SlashSpreadContourUse, SlashSpreadEdge, SlashSpreadEdgeOrigin,
};
use crate::ontology::{DirectedEdge, DirectedRange, EdgeRef};
use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

/// Explicit global source treatment,independent of membership in one proposed Piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadGlobalSourceIntent {
    /// Keep the global source;local semantic membership still requires reconciliation.
    Preserve,
    /// Request whole affected-source/consumer and command-authority review before retirement.
    /// This is neither permission nor a ledger edit;other Pieces may still need this source.
    RequestRetirementReview,
}
/// One original source identity and its explicit global treatment request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadSourceDisposition {
    edge: EdgeRef,
    intent: SlashSpreadGlobalSourceIntent,
}
impl SlashSpreadSourceDisposition {
    /// Retain both actual authored fields;no source existence or authority is inferred.
    #[must_use]
    pub const fn new(edge: EdgeRef, intent: SlashSpreadGlobalSourceIntent) -> Self {
        Self { edge, intent }
    }
    /// Exact original identity;aliases are not silently followed.
    #[must_use]
    pub const fn edge(&self) -> EdgeRef {
        self.edge
    }
    /// Actual explicit request,not achieved disposition.
    #[must_use]
    pub const fn intent(&self) -> &SlashSpreadGlobalSourceIntent {
        &self.intent
    }
}
/// Raw complete source-identity requests in authored order;empty construction accepts no result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlashSpreadSourcePlan {
    sources: Vec<SlashSpreadSourceDisposition>,
}
impl SlashSpreadSourcePlan {
    /// Retain every original request,including malformed duplicates or omissions.
    #[must_use]
    pub const fn new(sources: Vec<SlashSpreadSourceDisposition>) -> Self {
        Self { sources }
    }
    /// Original complete request list.
    #[must_use]
    pub fn sources(&self) -> &[SlashSpreadSourceDisposition] {
        &self.sources
    }
}
/// Explicit complete N candidates,R source occurrences,M uses and P requests;no default.
/// ```compile_fail
/// let policy:sc_core::construction::SlashSpreadSourcePlanLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadSourcePlanLimits {
    candidates: usize,
    source_edges: usize,
    uses: usize,
    requests: usize,
}
impl SlashSpreadSourcePlanLimits {
    /// Select all complete populations,including repeated source and contour occurrences.
    #[must_use]
    pub const fn new(candidates: usize, source_edges: usize, uses: usize, requests: usize) -> Self {
        Self {
            candidates,
            source_edges,
            uses,
            requests,
        }
    }
    /// Complete candidate allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Complete original Piece occurrence allowance.
    #[must_use]
    pub const fn source_edges(self) -> usize {
        self.source_edges
    }
    /// Complete contour occurrence allowance.
    #[must_use]
    pub const fn uses(self) -> usize {
        self.uses
    }
    /// Complete authored disposition request allowance.
    #[must_use]
    pub const fn requests(self) -> usize {
        self.requests
    }
}
/// Whole-resource refusal,then original-order request errors,then first uncovered source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadSourcePlanRefusal {
    /// Conservative R+1+N+P index entries cannot be counted.
    CountOverflow,
    /// Complete population exceeds explicit policy.
    Candidates {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original source occurrences exceed policy.
    SourceEdges {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete contour occurrences exceed policy.
    Uses {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete request population exceeds policy.
    Requests {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Original request names no original Piece,cut or candidate source identity.
    Source {
        /// Zero-based original request position.
        request: usize,
    },
    /// Two original requests name the same source,regardless of their intents.
    Duplicate {
        /// First original request.
        first: usize,
        /// Repeated original request.
        second: usize,
    },
    /// A retained contour use requires the source to remain globally available.
    RetainedRetirement {
        /// Original contradictory retirement-review request.
        request: usize,
    },
    /// A declared original source has no explicit request;no treatment is defaulted.
    Missing {
        /// Exact first uncovered source in Piece,cut,candidate order.
        edge: EdgeRef,
    },
}
/// Original plan,complete contour context and borrowed policy survive every refusal.
pub struct SlashSpreadSourcePlanError<'a> {
    plan: &'a SlashSpreadSourcePlan,
    contours: &'a AdmittedSlashSpreadContours<'a>,
    limits: &'a SlashSpreadSourcePlanLimits,
    refusal: SlashSpreadSourcePlanRefusal,
}
impl<'a> SlashSpreadSourcePlanError<'a> {
    /// Actual complete raw request owner.
    #[must_use]
    pub const fn plan(&self) -> &'a SlashSpreadSourcePlan {
        self.plan
    }
    /// Actual source and target declaration context.
    #[must_use]
    pub const fn contours(&self) -> &'a AdmittedSlashSpreadContours<'a> {
        self.contours
    }
    /// Actual original caller-owned policy.
    #[must_use]
    pub const fn limits(&self) -> &'a SlashSpreadSourcePlanLimits {
        self.limits
    }
    /// First whole-resource or authored-field refusal.
    #[must_use]
    pub const fn refusal(&self) -> SlashSpreadSourcePlanRefusal {
        self.refusal
    }
}
impl fmt::Debug for SlashSpreadSourcePlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadSourcePlanError")
            .field("plan", &self.plan)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadSourcePlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slash/spread source plan: {:?}", self.refusal)
    }
}
impl std::error::Error for SlashSpreadSourcePlanError<'_> {}

fn source_frames<'a>(
    contours: &'a AdmittedSlashSpreadContours<'a>,
) -> impl Iterator<Item = EdgeRef> + 'a {
    contours
        .source_piece()
        .edges()
        .map(|edge| edge.edge)
        .chain(core::iter::once(
            contours
                .edges()
                .bound_roles()
                .intent()
                .input()
                .cut()
                .range
                .edge(),
        ))
        .chain(
            contours
                .edges()
                .outputs()
                .filter_map(|output| match output.candidate().origin() {
                    SlashSpreadEdgeOrigin::Source { range, .. } => Some(range.range.edge()),
                    SlashSpreadEdgeOrigin::StraightBridge { .. } => None,
                }),
        )
}
impl SlashSpreadSourcePlan {
    /// Bind complete exact identity coverage under N/R/M/P quotas before indexing or field scans.
    /// Identity coverage is not interval coverage,current liveness,consumer census or authority.
    /// # Errors
    /// Count overflow,N/R/M/P quotas,then original-order unknown/duplicate/retained-retirement
    /// requests,then the first missing Piece/cut/candidate source. No partial bound plan escapes.
    pub fn bind<'a>(
        &'a self,
        contours: &'a AdmittedSlashSpreadContours<'a>,
        limits: &'a SlashSpreadSourcePlanLimits,
    ) -> Result<BoundSlashSpreadSourcePlan<'a>, SlashSpreadSourcePlanError<'a>> {
        let refuse = |refusal| SlashSpreadSourcePlanError {
            plan: self,
            contours,
            limits,
            refusal,
        };
        let n = contours.edges().manifest().candidates().len();
        let r = contours.source_edge_count();
        let m = contours.uses().len();
        let p = self.sources.len();
        let workspace_entries = r
            .checked_add(1)
            .and_then(|x| x.checked_add(n))
            .and_then(|x| x.checked_add(p))
            .ok_or_else(|| refuse(SlashSpreadSourcePlanRefusal::CountOverflow))?;
        if n > limits.candidates {
            return Err(refuse(SlashSpreadSourcePlanRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if r > limits.source_edges {
            return Err(refuse(SlashSpreadSourcePlanRefusal::SourceEdges {
                required: r,
                allowed: limits.source_edges,
            }));
        }
        if m > limits.uses {
            return Err(refuse(SlashSpreadSourcePlanRefusal::Uses {
                required: m,
                allowed: limits.uses,
            }));
        }
        if p > limits.requests {
            return Err(refuse(SlashSpreadSourcePlanRefusal::Requests {
                required: p,
                allowed: limits.requests,
            }));
        }
        let sources: BTreeSet<_> = source_frames(contours).collect();
        let mut requests = BTreeMap::new();
        for (request, source) in self.sources.iter().enumerate() {
            if !sources.contains(&source.edge) {
                return Err(refuse(SlashSpreadSourcePlanRefusal::Source { request }));
            }
            if let Some(first) = requests.insert(source.edge, request) {
                return Err(refuse(SlashSpreadSourcePlanRefusal::Duplicate {
                    first,
                    second: request,
                }));
            }
            if source.intent == SlashSpreadGlobalSourceIntent::RequestRetirementReview
                && contours.uses().any(|usage| {
                    usage
                        .retained()
                        .is_some_and(|edge| edge.edge == source.edge)
                })
            {
                return Err(refuse(SlashSpreadSourcePlanRefusal::RetainedRetirement {
                    request,
                }));
            }
        }
        for edge in source_frames(contours) {
            if !requests.contains_key(&edge) {
                return Err(refuse(SlashSpreadSourcePlanRefusal::Missing { edge }));
            }
        }
        Ok(BoundSlashSpreadSourcePlan {
            plan: self,
            contours,
            limits,
            workspace_entries,
        })
    }
}
/// Authentic complete identity/request association;no ledger mutation or retirement permission.
#[derive(Debug)]
pub struct BoundSlashSpreadSourcePlan<'a> {
    plan: &'a SlashSpreadSourcePlan,
    contours: &'a AdmittedSlashSpreadContours<'a>,
    limits: &'a SlashSpreadSourcePlanLimits,
    workspace_entries: usize,
}
impl<'a> BoundSlashSpreadSourcePlan<'a> {
    /// Actual original raw plan.
    #[must_use]
    pub const fn plan(&self) -> &'a SlashSpreadSourcePlan {
        self.plan
    }
    /// Actual whole contour/source declaration.
    #[must_use]
    pub const fn contours(&self) -> &'a AdmittedSlashSpreadContours<'a> {
        self.contours
    }
    /// Actual caller-owned policy.
    #[must_use]
    pub const fn limits(&self) -> &'a SlashSpreadSourcePlanLimits {
        self.limits
    }
    /// Conservative temporary R+1+N+P index entries,not byte capacity or a consumer census.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Every actual request in original authored order,with all occurrence evidence inspectable.
    pub fn sources(
        &self,
    ) -> impl DoubleEndedIterator<Item = SlashSpreadSourceRequest<'_>> + ExactSizeIterator {
        self.plan
            .sources
            .iter()
            .enumerate()
            .map(|(index, request)| SlashSpreadSourceRequest {
                plan: self,
                index,
                request,
            })
    }
}
/// Private association of an original request with its complete bound source/contour context.
/// ```compile_fail
/// fn reparent<'a>(mut request:sc_core::construction::SlashSpreadSourceRequest<'a>,other:&'a sc_core::construction::BoundSlashSpreadSourcePlan<'a>) {
///     request.plan=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SlashSpreadSourceRequest<'a> {
    plan: &'a BoundSlashSpreadSourcePlan<'a>,
    index: usize,
    request: &'a SlashSpreadSourceDisposition,
}
impl<'a> SlashSpreadSourceRequest<'a> {
    /// Actual whole bound plan.
    #[must_use]
    pub const fn plan(self) -> &'a BoundSlashSpreadSourcePlan<'a> {
        self.plan
    }
    /// Original request position,not persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Actual original authored request fields.
    #[must_use]
    pub const fn request(self) -> &'a SlashSpreadSourceDisposition {
        self.request
    }
    /// All original Piece occurrences,including repeated construction uses and directions.
    pub fn piece_uses(self) -> impl Iterator<Item = DirectedEdge> + 'a {
        self.plan
            .contours
            .source_piece()
            .edges()
            .filter(move |edge| edge.edge == self.request.edge)
    }
    /// Actual original cut field when this source is the cut;its domain need not be the whole edge.
    #[must_use]
    pub fn cut(self) -> Option<&'a DirectedRange> {
        let cut = self
            .plan
            .contours
            .edges()
            .bound_roles()
            .intent()
            .input()
            .cut();
        (cut.range.edge() == self.request.edge).then_some(cut)
    }
    /// All authentic source-provenance candidates,without choosing one or inventing interval maps.
    pub fn derived_outputs(self) -> impl DoubleEndedIterator<Item = SlashSpreadEdge<'a>> {
        self.plan.contours.edges().outputs().filter(move |output| {
            matches!(
                output.candidate().origin(), SlashSpreadEdgeOrigin::Source { range, .. }
                    if range.range.edge() == self.request.edge
            )
        })
    }
    /// All original retained contour occurrences;absence implies no global disposition.
    pub fn retained_uses(self) -> impl DoubleEndedIterator<Item = SlashSpreadContourUse<'a>> {
        self.plan.contours.uses().filter(move |usage| {
            usage
                .retained()
                .is_some_and(|edge| edge.edge == self.request.edge)
        })
    }
}
