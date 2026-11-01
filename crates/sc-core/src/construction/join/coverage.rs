//! Complete exact-held surviving cut-source partitions,not whole joined contours or geometry.
use super::{
    AdmittedJoinEdges, JoinEdgeCandidate, JoinOutputEdge, JoinSourceOccurrence,
    JoinSourceOccurrenceLocation,
};
use crate::ontology::{Param, SeamSideId};
use core::fmt;
/// Complete proposal N,original occurrence R and conservative N-entry sorted-index allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinCutCoverageLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCutCoverageLimits {
    candidates: usize,
    source_occurrences: usize,
    workspace_entries: usize,
}
impl JoinCutCoverageLimits {
    /// Bound all candidates/all original occurrences before fields or allocation,including noncut
    /// fields. Workspace admits N entries before filtering;it does not measure allocator bytes.
    #[must_use]
    pub const fn new(
        candidates: usize,
        source_occurrences: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            candidates,
            source_occurrences,
            workspace_entries,
        }
    }
    /// Whole original proposal allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Whole original A+B cut/construction/fold occurrence allowance,not only cut fields.
    #[must_use]
    pub const fn source_occurrences(self) -> usize {
        self.source_occurrences
    }
    /// Conservative whole N-entry sorted-index allowance before filtering.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First complete quota,consumed-bank intersection or exact required cut-domain partition cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinCutCoverageRefusal {
    /// Whole original N exceeds policy.
    Candidates {
        /// Complete original population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole original R,including noncut fields,exceeds policy.
    SourceOccurrences {
        /// Complete original count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Conservative N-entry index exceeds policy before filtering/allocation.
    Workspace {
        /// Complete required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// A cut candidate has positive intersection with this side's selected bank domain.
    ConsumedBank {
        /// First original candidate index.
        index: usize,
    },
    /// The first uncovered positive interval on an original required cut domain.
    Gap {
        /// Original A/B source role.
        side: SeamSideId,
        /// Original cut field location.
        location: JoinSourceOccurrenceLocation,
        /// Exact first missing lower bound.
        from: Param,
        /// Exact first missing upper bound.
        to: Param,
    },
    /// A candidate begins before the required domain's already covered cursor.
    Overlap {
        /// Original offending candidate index after source-range ordering.
        index: usize,
        /// Previously covered upper bound in the same source frame.
        covered_to: Param,
    },
}
/// Whole actual attributed manifest,policy and first original failure;no accepted prefix escapes.
pub struct JoinCutCoverageError<'a> {
    edges: &'a AdmittedJoinEdges<'a>,
    limits: JoinCutCoverageLimits,
    refusal: JoinCutCoverageRefusal,
}
impl<'a> JoinCutCoverageError<'a> {
    /// Actual complete attributed proposal and original identity/catalogue/bank/source parents.
    #[must_use]
    pub const fn edges(&self) -> &'a AdmittedJoinEdges<'a> {
        self.edges
    }
    /// Original explicit complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCutCoverageLimits {
        self.limits
    }
    /// First complete quota,consumed-bank or exact interval-partition cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinCutCoverageRefusal {
        self.refusal
    }
    /// Actual original candidate only for ConsumedBank/Overlap;other causes return None.
    #[must_use]
    pub fn refused_candidate(&self) -> Option<&'a JoinEdgeCandidate> {
        let index = match self.refusal {
            JoinCutCoverageRefusal::ConsumedBank { index }
            | JoinCutCoverageRefusal::Overlap { index, .. } => index,
            _ => return None,
        };
        self.edges.manifest().candidates().get(index)
    }
    /// Actual original occurrence for domain causes;whole quota causes return None.
    #[must_use]
    pub fn source_occurrence(&self) -> Option<JoinSourceOccurrence<'a>> {
        let (side, location) = match self.refusal {
            JoinCutCoverageRefusal::Gap { side, location, .. } => (side, location),
            JoinCutCoverageRefusal::ConsumedBank { .. }
            | JoinCutCoverageRefusal::Overlap { .. } => {
                let candidate = self.refused_candidate()?;
                (candidate.side(), candidate.location())
            }
            _ => return None,
        };
        self.edges
            .identity()
            .roles()
            .banks()
            .sources()
            .occurrence(side, location)
    }
}
impl fmt::Debug for JoinCutCoverageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinCutCoverageError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinCutCoverageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join cut-source coverage: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinCutCoverageError<'_> {}
// Transient source-field lookup only;never a persisted/output identity. Cut family order matches
// original boundary then hole occurrences;A then B is explicit,not sorted entity-id inference.
type SourceKey = (u8, u8, usize, usize);
fn source_key(side: SeamSideId, location: JoinSourceOccurrenceLocation) -> Option<SourceKey> {
    let side = match side {
        SeamSideId::A => 0,
        SeamSideId::B => 1,
    };
    match location {
        JoinSourceOccurrenceLocation::Boundary(index) => Some((side, 0, 0, index)),
        JoinSourceOccurrenceLocation::Hole { hole, index } => Some((side, 1, hole, index)),
        JoinSourceOccurrenceLocation::Construction(_) | JoinSourceOccurrenceLocation::Fold(_) => {
            None
        }
    }
}
#[derive(Debug)]
struct IndexedCut<'a> {
    key: SourceKey,
    edge: JoinOutputEdge<'a>,
}
fn group<'a, 'b>(
    index: &'b [IndexedCut<'a>],
    key: Option<SourceKey>,
) -> impl ExactSizeIterator<Item = &'b IndexedCut<'a>> + DoubleEndedIterator {
    let (start, end) = key.map_or((0, 0), |key| {
        (
            index.partition_point(|row| row.key < key),
            index.partition_point(|row| row.key <= key),
        )
    });
    // Sorted keys guarantee start <= end <= len. Slice-iterator skip/take advances by
    // bounded offsets without slicing or scanning each skipped candidate.
    index.iter().skip(start).take(end - start)
}
impl<'s> AdmittedJoinEdges<'s> {
    /// Require exactly the original cut domains that survive removal of both selected banks.
    /// Whole N/R/N-entry quotas precede all scans/allocation;prior C/I/H/R/S/catalogue resources
    /// remain. Cut candidates with positive bank overlap refuse in original candidate order.
    /// Then required domains run original A/B/boundary/hole order,prefix before suffix,with
    /// candidates ordered by source from,to,original index. Endpoint-only touching is permitted.
    /// Original construction/fold candidates are not cut-domain obligations or silently discarded.
    /// Work O(N log(N+2)+(H+R)log(N+2)),N index entries,no registry/provider/geometry calls.
    /// This is exact-held attribution/partition data,not currentness,valid output topology,
    /// traversal/map/closure/fold/consumer proof,retirement,geometry or publication authority.
    /// # Errors
    /// Complete N,R,index quotas;first original cut candidate touching a positive consumed domain;
    /// then first required-domain gap/overlap under the documented original-source/range order.
    pub fn require_cut_coverage<'a>(
        &'a self,
        limits: JoinCutCoverageLimits,
    ) -> Result<CoveredJoinCutSources<'a>, JoinCutCoverageError<'a>>
    where
        's: 'a,
    {
        use JoinCutCoverageRefusal as R;
        let refuse = |refusal| JoinCutCoverageError {
            edges: self,
            limits,
            refusal,
        };
        let n = self.edge_count();
        let sources = self.identity().roles().banks().sources();
        let r = sources.occurrence_count();
        if n > limits.candidates {
            return Err(refuse(R::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if r > limits.source_occurrences {
            return Err(refuse(R::SourceOccurrences {
                required: r,
                allowed: limits.source_occurrences,
            }));
        }
        if n > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: n,
                allowed: limits.workspace_entries,
            }));
        }
        let banks = self.identity().roles().banks();
        let mut index = Vec::with_capacity(n);
        for edge in self.edges() {
            let occurrence = edge.occurrence();
            let Some(key) = source_key(occurrence.side(), occurrence.location()) else {
                continue;
            };
            let bank = banks.bank(occurrence.side());
            if occurrence.location() == bank.occurrence().location()
                && edge.source().from() < bank.range().to()
                && edge.source().to() > bank.range().from()
            {
                return Err(refuse(R::ConsumedBank {
                    index: edge.index(),
                }));
            }
            index.push(IndexedCut { key, edge });
        }
        index.sort_unstable_by_key(|row| {
            (
                row.key,
                row.edge.source().from(),
                row.edge.source().to(),
                row.edge.index(),
            )
        });
        let mut cut_occurrences = 0;
        for occurrence in sources.occurrences() {
            let Some(key) = source_key(occurrence.side(), occurrence.location()) else {
                continue;
            };
            cut_occurrences += 1; // Complete admitted R already bounds this subset.
            let bank = banks.bank(occurrence.side());
            let domains = if occurrence.location() == bank.occurrence().location() {
                [
                    (Param::START, bank.range().from()),
                    (bank.range().to(), Param::END),
                ]
            } else {
                [(Param::START, Param::END), (Param::END, Param::END)]
            };
            let mut next = group(&index, Some(key)).peekable();
            for (from, to) in domains {
                if from == to {
                    continue;
                }
                let mut cursor = from;
                while let Some(row) = next.peek().filter(|row| row.edge.source().from() < to) {
                    let range = row.edge.source();
                    if range.from() < cursor {
                        return Err(refuse(R::Overlap {
                            index: row.edge.index(),
                            covered_to: cursor,
                        }));
                    }
                    if range.from() > cursor {
                        return Err(refuse(R::Gap {
                            side: occurrence.side(),
                            location: occurrence.location(),
                            from: cursor,
                            to: range.from(),
                        }));
                    }
                    cursor = range.to();
                    next.next();
                }
                if cursor < to {
                    return Err(refuse(R::Gap {
                        side: occurrence.side(),
                        location: occurrence.location(),
                        from: cursor,
                        to,
                    }));
                }
            }
        }
        Ok(CoveredJoinCutSources {
            edges: self,
            limits,
            workspace_entries: n,
            cut_occurrences,
            index,
        })
    }
}
/// Complete original surviving cut partitions,not a complete joined Piece or currentness proof.
/// ```compile_fail
/// fn forge<'a>(mut coverage:sc_core::construction::CoveredJoinCutSources<'a>,other:&'a sc_core::construction::AdmittedJoinEdges<'a>) {coverage.edges=other;}
/// ```
/// ```compile_fail
/// fn escape(edges:sc_core::construction::AdmittedJoinEdges<'_>)->sc_core::construction::CoveredJoinCutSources<'static> {
///     edges.require_cut_coverage(sc_core::construction::JoinCutCoverageLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct CoveredJoinCutSources<'a> {
    edges: &'a AdmittedJoinEdges<'a>,
    limits: JoinCutCoverageLimits,
    workspace_entries: usize,
    cut_occurrences: usize,
    index: Vec<IndexedCut<'a>>,
}
impl<'a> CoveredJoinCutSources<'a> {
    /// Actual complete original attributed manifest/identity/catalogue/bank/source context.
    #[must_use]
    pub const fn edges(&self) -> &'a AdmittedJoinEdges<'a> {
        self.edges
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCutCoverageLimits {
        self.limits
    }
    /// Complete conservative N-entry index admission,including unindexed noncut fields.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Complete original A+B boundary/hole occurrence count,including wholly consumed banks.
    #[must_use]
    pub const fn cut_occurrence_count(&self) -> usize {
        self.cut_occurrences
    }
    /// Actual indexed cut candidates;construction/fold candidates remain on the whole parent.
    #[must_use]
    pub fn cut_edge_count(&self) -> usize {
        self.index.len()
    }
    /// Authentic cut candidates in original source-field then ascending range/index order,and reverse.
    pub fn cut_edges(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinOutputEdge<'_>> + DoubleEndedIterator {
        self.index.iter().map(|row| row.edge)
    }
    /// Authentic cut outputs for this original field in ascending held range order,and reverse.
    /// Missing/noncut locations return an empty iterator;no ordinal becomes a persistent identity.
    pub fn outputs(
        &self,
        side: SeamSideId,
        location: JoinSourceOccurrenceLocation,
    ) -> impl ExactSizeIterator<Item = JoinOutputEdge<'_>> + DoubleEndedIterator {
        group(&self.index, source_key(side, location)).map(|row| row.edge)
    }
}
