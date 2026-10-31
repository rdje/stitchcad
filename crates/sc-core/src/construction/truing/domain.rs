//! Membership in the actual selected held source range,not current geometry or complete coverage.
use super::{AdmittedTruingEdges, TruingOutputEdge};
use crate::ontology::{EdgeRange, EdgeRef, LocalTag};
use core::fmt;
/// Explicit whole original candidate allowance;no workspace is allocated by domain binding.
/// ```compile_fail
/// let limits:sc_core::construction::TruingSourceDomainLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingSourceDomainLimits {
    candidates: usize,
}
impl TruingSourceDomainLimits {
    /// Admit every original candidate before inspecting any range or parameter.
    #[must_use]
    pub const fn new(candidates: usize) -> Self {
        Self { candidates }
    }
    /// Complete original candidate allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
}
/// Whole quota or first original range disagreement,with exact held parameters and no epsilon.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingSourceDomainRefusal {
    /// Complete original candidates exceed policy.
    Candidates {
        /// Actual complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Exact held edge creator/tag differs from the selected original span side.
    Edge {
        /// Original candidate position.
        index: usize,
        /// Actual offending held edge;the whole range remains in the original manifest.
        edge: EdgeRef,
    },
    /// Original lower bound precedes the actual selected held domain.
    BeforeStart {
        /// Original candidate position.
        index: usize,
    },
    /// Original upper bound follows the actual selected held domain.
    AfterEnd {
        /// Original candidate position.
        index: usize,
    },
}
/// Actual complete candidates,selected original domain,policy and first refusal;no accepted prefix.
pub struct TruingSourceDomainError<'a> {
    edges: &'a AdmittedTruingEdges<'a>,
    domain: &'a EdgeRange,
    limits: TruingSourceDomainLimits,
    refusal: TruingSourceDomainRefusal,
}
impl<'a> TruingSourceDomainError<'a> {
    /// Actual complete candidate/role/copy/Piece/source parents.
    #[must_use]
    pub const fn edges(&self) -> &'a AdmittedTruingEdges<'a> {
        self.edges
    }
    /// Actual original selected span-side range field,not a supplied replacement range.
    #[must_use]
    pub const fn domain(&self) -> &'a EdgeRange {
        self.domain
    }
    /// Original explicit whole-candidate policy.
    #[must_use]
    pub const fn limits(&self) -> TruingSourceDomainLimits {
        self.limits
    }
    /// First complete quota or original range cause.
    #[must_use]
    pub const fn refusal(&self) -> TruingSourceDomainRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingSourceDomainError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingSourceDomainError")
            .field("domain", self.domain)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingSourceDomainError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing selected source domain: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingSourceDomainError<'_> {}
impl<'s> AdmittedTruingEdges<'s> {
    /// Bind all candidate ranges to the actual original selected span-side held parameter domain.
    /// Complete N quota precedes all range/parameter inspection. Original order checks exact edge,
    /// lower bound,then upper bound. Param ordering is exact bounded rational ordering;no epsilon,
    /// normalization,arithmetic result,ledger/geometry query or allocation is introduced. Work O(N),
    /// auxiliary space O(1). Empty membership is legal and proves no coverage,no-op or retirement.
    /// This certifies neither current source/Piece ownership nor an output parameter map or fit.
    /// # Errors
    /// Whole quota;then first original candidate's edge,lower or upper disagreement.
    pub fn bind_source_domain<'a>(
        &'a self,
        limits: TruingSourceDomainLimits,
    ) -> Result<BoundTruingSourceDomain<'a>, TruingSourceDomainError<'a>>
    where
        's: 'a,
    {
        let domain = &self.roles().copies().source().source_side().range;
        let refuse = |refusal| TruingSourceDomainError {
            edges: self,
            domain,
            limits,
            refusal,
        };
        let n = self.edge_count();
        if n > limits.candidates {
            return Err(refuse(TruingSourceDomainRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        for edge in self.edges() {
            let range = edge.source();
            let index = edge.index();
            if range.edge() != domain.edge() {
                return Err(refuse(TruingSourceDomainRefusal::Edge {
                    index,
                    edge: range.edge(),
                }));
            }
            if range.from() < domain.from() {
                return Err(refuse(TruingSourceDomainRefusal::BeforeStart { index }));
            }
            if range.to() > domain.to() {
                return Err(refuse(TruingSourceDomainRefusal::AfterEnd { index }));
            }
        }
        Ok(BoundTruingSourceDomain {
            edges: self,
            domain,
            limits,
        })
    }
}
/// Complete original candidate membership in the actual held domain,not coverage/currentness/fit.
/// ```compile_fail
/// fn reparent<'a>(mut bound:sc_core::construction::BoundTruingSourceDomain<'a>,
///     other:&'a sc_core::construction::AdmittedTruingEdges<'a>) {bound.edges=other;}
/// ```
/// ```compile_fail
/// fn escape(edges:sc_core::construction::AdmittedTruingEdges<'_>)
///     ->sc_core::construction::BoundTruingSourceDomain<'static> {
///     edges.bind_source_domain(sc_core::construction::TruingSourceDomainLimits::new(0)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundTruingSourceDomain<'a> {
    edges: &'a AdmittedTruingEdges<'a>,
    domain: &'a EdgeRange,
    limits: TruingSourceDomainLimits,
}
impl<'a> BoundTruingSourceDomain<'a> {
    /// Actual complete original candidate/role/copy/Piece/source parents.
    #[must_use]
    pub const fn candidates(&self) -> &'a AdmittedTruingEdges<'a> {
        self.edges
    }
    /// Actual original selected span-side range,not a copied or caller-supplied domain.
    #[must_use]
    pub const fn domain(&self) -> &'a EdgeRange {
        self.domain
    }
    /// Original explicit whole-candidate allowance.
    #[must_use]
    pub const fn limits(&self) -> TruingSourceDomainLimits {
        self.limits
    }
    /// Complete original candidates,not a coverage or output-geometry count.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.edge_count()
    }
    /// Authentic whole-domain candidate views in original order,including reverse traversal.
    pub fn edges(
        &self,
    ) -> impl ExactSizeIterator<Item = TruingSourceDomainEdge<'_>> + DoubleEndedIterator {
        self.edges.edges().map(|edge| TruingSourceDomainEdge {
            context: self,
            edge,
        })
    }
    /// Authentic candidate for a present role,in O(log(N+2)) work.
    #[must_use]
    pub fn edge(&self, tag: LocalTag) -> Option<TruingSourceDomainEdge<'_>> {
        self.edges.edge(tag).map(|edge| TruingSourceDomainEdge {
            context: self,
            edge,
        })
    }
}
/// Original candidate with authentic actual whole-domain context;no geometric map is inferred.
/// ```compile_fail
/// fn reparent<'a>(mut edge:sc_core::construction::TruingSourceDomainEdge<'a>,
///     other:&'a sc_core::construction::BoundTruingSourceDomain<'a>) {edge.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingSourceDomainEdge<'a> {
    context: &'a BoundTruingSourceDomain<'a>,
    edge: TruingOutputEdge<'a>,
}
impl<'a> TruingSourceDomainEdge<'a> {
    /// Actual complete domain/candidate/source owner.
    #[must_use]
    pub const fn context(self) -> &'a BoundTruingSourceDomain<'a> {
        self.context
    }
    /// Authentic original candidate and active role in its original attribution owner.
    #[must_use]
    pub const fn candidate(self) -> TruingOutputEdge<'a> {
        self.edge
    }
    /// Actual original candidate source field,without rescaling or normalization.
    #[must_use]
    pub const fn source(self) -> &'a EdgeRange {
        self.edge.source()
    }
    /// Actual selected original domain field from this whole context.
    #[must_use]
    pub const fn domain(self) -> &'a EdgeRange {
        self.context.domain
    }
}
