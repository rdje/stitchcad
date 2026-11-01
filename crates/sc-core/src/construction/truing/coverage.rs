//! Exact partition of the selected held parameter domain;no fitted output or currentness proof.
use super::{BoundTruingSourceDomain, TruingSourceDomainEdge};
use crate::ontology::{EdgeRange, Param};
use core::fmt;

/// Explicit complete candidate and source-order index allowances;no defaults or allocation-byte claim.
/// ```compile_fail
/// let limits:sc_core::construction::TruingSourceCoverageLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingSourceCoverageLimits {
    candidates: usize,
    workspace_entries: usize,
}
impl TruingSourceCoverageLimits {
    /// Choose whole N and W before any source-order scan or index allocation;required W is N.
    #[must_use]
    pub const fn new(candidates: usize, workspace_entries: usize) -> Self {
        Self {
            candidates,
            workspace_entries,
        }
    }
    /// Complete original candidate allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Complete additional source-order index allowance,not allocator bytes or earlier owned storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}

/// First whole-resource cause or first exact discontinuity in ascending source-parameter order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingSourceCoverageRefusal {
    /// Complete original candidates exceed policy.
    Candidates {
        /// Actual whole population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete additional source-order index exceeds policy.
    Workspace {
        /// Actual whole population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// A positive source interval is unassigned;an empty manifest leaves the whole domain unassigned.
    Gap {
        /// Exact first uncovered lower parameter.
        from: Param,
        /// Exact first uncovered upper parameter.
        to: Param,
        /// Original index of the next candidate,or None for a trailing/whole-domain gap.
        next: Option<usize>,
    },
    /// Two candidates assign a positive source interval more than once;endpoint touches are legal.
    Overlap {
        /// Original index of the preceding candidate in source order.
        previous: usize,
        /// Original index of the current candidate in source order.
        index: usize,
        /// Exact overlap lower parameter.
        from: Param,
        /// Exact overlap upper parameter.
        to: Param,
    },
}

/// Complete actual membership parent,explicit limits and first cause;no accepted prefix escapes.
pub struct TruingSourceCoverageError<'a> {
    source_domain: &'a BoundTruingSourceDomain<'a>,
    limits: TruingSourceCoverageLimits,
    refusal: TruingSourceCoverageRefusal,
}
impl<'a> TruingSourceCoverageError<'a> {
    /// Actual whole membership/candidate/role/copy/Piece/source context.
    #[must_use]
    pub const fn source_domain(&self) -> &'a BoundTruingSourceDomain<'a> {
        self.source_domain
    }
    /// Original explicit whole-resource limits.
    #[must_use]
    pub const fn limits(&self) -> TruingSourceCoverageLimits {
        self.limits
    }
    /// First quota or exact source-order discontinuity.
    #[must_use]
    pub const fn refusal(&self) -> TruingSourceCoverageRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingSourceCoverageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingSourceCoverageError")
            .field("domain", self.source_domain.domain())
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingSourceCoverageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing selected source coverage: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingSourceCoverageError<'_> {}

impl<'s> BoundTruingSourceDomain<'s> {
    /// Require an exact nonoverlapping partition of the actual selected held source domain.
    /// Whole N quota,then W quota precede source scans/index allocation. Required W is N additional
    /// logical entries;earlier admission/storage and allocation bytes retain their separate owners.
    /// Original fields/order remain unchanged. The borrowed index sorts by exact from,to,original
    /// index,then reports the first gap or positive overlap from the selected lower bound onward.
    /// Endpoint touches pass;empty input refuses the whole positive domain. Sorting uses O(N log N)
    /// work and one N-entry index sorted in place. No ledger/geometry query,parameter
    /// arithmetic,epsilon,refit/no-op decision or output mapping is introduced.
    /// # Errors
    /// Whole candidate quota,whole workspace quota,then first exact source-order gap/overlap.
    pub fn bind_source_coverage<'a>(
        &'a self,
        limits: TruingSourceCoverageLimits,
    ) -> Result<CoveredTruingSourceDomain<'a>, TruingSourceCoverageError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| TruingSourceCoverageError {
            source_domain: self,
            limits,
            refusal,
        };
        let n = self.edge_count();
        if n > limits.candidates {
            return Err(refuse(TruingSourceCoverageRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if n > limits.workspace_entries {
            return Err(refuse(TruingSourceCoverageRefusal::Workspace {
                required: n,
                allowed: limits.workspace_entries,
            }));
        }
        let mut source_order: Vec<_> = self.edges().collect();
        source_order.sort_unstable_by_key(|edge| {
            (
                edge.source().from(),
                edge.source().to(),
                edge.candidate().index(),
            )
        });
        let mut end = self.domain().from();
        let mut previous = None;
        for edge in &source_order {
            let range = edge.source();
            let index = edge.candidate().index();
            if range.from() > end {
                return Err(refuse(TruingSourceCoverageRefusal::Gap {
                    from: end,
                    to: range.from(),
                    next: Some(index),
                }));
            }
            if let Some(previous) = previous {
                if range.from() < end {
                    return Err(refuse(TruingSourceCoverageRefusal::Overlap {
                        previous,
                        index,
                        from: range.from(),
                        to: end.min(range.to()),
                    }));
                }
            }
            end = range.to();
            previous = Some(index);
        }
        if end < self.domain().to() {
            return Err(refuse(TruingSourceCoverageRefusal::Gap {
                from: end,
                to: self.domain().to(),
                next: None,
            }));
        }
        Ok(CoveredTruingSourceDomain {
            source_domain: self,
            limits,
            source_order,
        })
    }
}

/// Authentic complete partition of the selected held domain;no output/currentness/fit certificate.
/// ```compile_fail
/// fn forge<'a>(mut covered:sc_core::construction::CoveredTruingSourceDomain<'a>,
///     other:&'a sc_core::construction::BoundTruingSourceDomain<'a>) {covered.source_domain=other;}
/// ```
/// ```compile_fail
/// fn escape(domain:sc_core::construction::BoundTruingSourceDomain<'_>)
///     ->sc_core::construction::CoveredTruingSourceDomain<'static> {
///     domain.bind_source_coverage(sc_core::construction::TruingSourceCoverageLimits::new(0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct CoveredTruingSourceDomain<'a> {
    source_domain: &'a BoundTruingSourceDomain<'a>,
    limits: TruingSourceCoverageLimits,
    source_order: Vec<TruingSourceDomainEdge<'a>>,
}
impl<'a> CoveredTruingSourceDomain<'a> {
    /// Actual complete original membership and all candidate/role/copy/Piece/source parents.
    #[must_use]
    pub const fn source_domain(&self) -> &'a BoundTruingSourceDomain<'a> {
        self.source_domain
    }
    /// Actual original selected held range field,without rescaling or normalization.
    #[must_use]
    pub const fn domain(&self) -> &'a EdgeRange {
        self.source_domain.domain()
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> TruingSourceCoverageLimits {
        self.limits
    }
    /// Complete additional logical index entries,not allocation bytes or whole execution resources.
    #[must_use]
    pub fn workspace_entries(&self) -> usize {
        self.source_order.len()
    }
    /// Every original candidate in exact ascending source order,including reverse iteration.
    /// Members retain the actual original membership context;original declaration order remains
    /// available through source_domain().edges(). This order is not output traversal direction.
    pub fn source_order(
        &self,
    ) -> impl ExactSizeIterator<Item = TruingSourceDomainEdge<'a>> + DoubleEndedIterator + '_ {
        self.source_order.iter().copied()
    }
}
