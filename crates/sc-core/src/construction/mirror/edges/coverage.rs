//! Complete identity attribution preserves every actual source field occurrence;no geometry proof.
use super::{AdmittedMirrorPieceEdges, MirrorPieceOutputEdge};
use crate::{
    construction::{MirrorPieceEdge, MirrorPieceEdgeLocation},
    ontology::EdgeRef,
};
use core::fmt;
use std::collections::{btree_map::Entry, BTreeMap};

/// Explicit whole candidate,original occurrence and workspace-entry policy;no default allowance.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPieceEdgeCoverageLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceEdgeCoverageLimits {
    candidates: usize,
    occurrences: usize,
    workspace_entries: usize,
}
impl MirrorPieceEdgeCoverageLimits {
    /// Bound complete N candidates,R source occurrences and checked N+R workspace entries.
    #[must_use]
    pub const fn new(candidates: usize, occurrences: usize, workspace_entries: usize) -> Self {
        Self {
            candidates,
            occurrences,
            workspace_entries,
        }
    }
    /// Complete admitted candidate allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Complete actual boundary/hole/construction/fold occurrence allowance.
    #[must_use]
    pub const fn occurrences(self) -> usize {
        self.occurrences
    }
    /// Source index plus complete occurrence associations;entries,not bytes.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// Complete resource or original source attribution failure;no partial copy escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceEdgeCoverageRefusal {
    /// Complete N+R count cannot be represented.
    WorkspaceOverflow,
    /// Complete candidate population exceeds policy before source inspection.
    Candidates {
        /// Complete actual candidates.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete source occurrence population exceeds policy before source inspection.
    Occurrences {
        /// Complete actual occurrences,including repeats and fold declarations.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete index/association population exceeds policy before allocation.
    Workspace {
        /// Complete N+R entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Two distinct candidate roles propose the same original edge;neither is chosen.
    AmbiguousSource {
        /// First original candidate position.
        first: usize,
        /// Repeated original candidate position.
        second: usize,
        /// Exact original source identity.
        source: EdgeRef,
    },
    /// First actual original source occurrence has no candidate.
    MissingSource {
        /// Original boundary/hole/construction/fold location.
        location: MirrorPieceEdgeLocation,
        /// Original source identity,not a rewritten output.
        source: EdgeRef,
    },
    /// Candidate names no actual source occurrence;current/global liveness is a separate question.
    UnusedCandidate {
        /// Original proposal position.
        index: usize,
        /// Original unreferenced source field.
        source: EdgeRef,
    },
}
/// Actual whole candidate/source/Piece owners and policy survive refusal.
pub struct MirrorPieceEdgeCoverageError<'a> {
    edges: &'a AdmittedMirrorPieceEdges<'a>,
    limits: MirrorPieceEdgeCoverageLimits,
    refusal: MirrorPieceEdgeCoverageRefusal,
}
impl<'a> MirrorPieceEdgeCoverageError<'a> {
    /// Actual admitted candidate owner and all canonical Piece/source/metadata parents.
    #[must_use]
    pub const fn edges(&self) -> &'a AdmittedMirrorPieceEdges<'a> {
        self.edges
    }
    /// Original explicit complete resource policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPieceEdgeCoverageLimits {
        self.limits
    }
    /// First whole-resource or original-source failure.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceEdgeCoverageRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceEdgeCoverageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceEdgeCoverageError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceEdgeCoverageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror source-edge coverage: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceEdgeCoverageError<'_> {}

#[derive(Debug)]
struct SourceMatch<'a> {
    output: MirrorPieceOutputEdge<'a>,
    uses: usize,
}
#[derive(Clone, Copy, Debug)]
struct Occurrence<'a> {
    source: MirrorPieceEdge<'a>,
    output: MirrorPieceOutputEdge<'a>,
}
impl<'s> AdmittedMirrorPieceEdges<'s> {
    /// Require exactly one proposed role per distinct actual source edge,and retain every occurrence.
    /// Whole N/R/N+R bounds precede source inspection and allocation. Work is O((N+R)log(N+2))
    /// with an N-entry index and R-entry association vector. Earlier input/catalogue storage retains
    /// its own policy. This is identity attribution,not currentness,reflection or contour validity.
    /// # Errors
    /// Checked N+R,N quota,R quota,workspace quota;then first duplicate source in proposal order,
    /// first missing actual source occurrence,and first unused candidate in proposal order.
    pub fn admit_source_coverage<'a>(
        &'a self,
        limits: MirrorPieceEdgeCoverageLimits,
    ) -> Result<MirrorPieceEdgeCoverage<'a>, MirrorPieceEdgeCoverageError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| MirrorPieceEdgeCoverageError {
            edges: self,
            limits,
            refusal,
        };
        let source = self.identity().roles().metadata().cut().source();
        let n = self.edge_count();
        let r = source.edge_count();
        let workspace_entries = n
            .checked_add(r)
            .ok_or_else(|| refuse(MirrorPieceEdgeCoverageRefusal::WorkspaceOverflow))?;
        if n > limits.candidates {
            return Err(refuse(MirrorPieceEdgeCoverageRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if r > limits.occurrences {
            return Err(refuse(MirrorPieceEdgeCoverageRefusal::Occurrences {
                required: r,
                allowed: limits.occurrences,
            }));
        }
        if workspace_entries > limits.workspace_entries {
            return Err(refuse(MirrorPieceEdgeCoverageRefusal::Workspace {
                required: workspace_entries,
                allowed: limits.workspace_entries,
            }));
        }
        let mut by_source: BTreeMap<EdgeRef, SourceMatch<'a>> = BTreeMap::new();
        for output in self.edges() {
            match by_source.entry(*output.source()) {
                Entry::Occupied(first) => {
                    return Err(refuse(MirrorPieceEdgeCoverageRefusal::AmbiguousSource {
                        first: first.get().output.index(),
                        second: output.index(),
                        source: *output.source(),
                    }))
                }
                Entry::Vacant(slot) => {
                    slot.insert(SourceMatch { output, uses: 0 });
                }
            }
        }
        let mut occurrences = Vec::with_capacity(r);
        for original in source.edges() {
            let edge = *original.source().edge();
            let matched = by_source.get_mut(&edge).ok_or_else(|| {
                refuse(MirrorPieceEdgeCoverageRefusal::MissingSource {
                    location: original.location(),
                    source: edge,
                })
            })?;
            // One increment per actual source occurrence;the full R count already fits usize.
            matched.uses += 1;
            occurrences.push(Occurrence {
                source: original,
                output: matched.output,
            });
        }
        // Choose the first unused candidate in original proposal order,never source-key sort order.
        if let Some(matched) = by_source
            .values()
            .filter(|entry| entry.uses == 0)
            .min_by_key(|entry| entry.output.index())
        {
            return Err(refuse(MirrorPieceEdgeCoverageRefusal::UnusedCandidate {
                index: matched.output.index(),
                source: *matched.output.source(),
            }));
        }
        Ok(MirrorPieceEdgeCoverage {
            edges: self,
            limits,
            workspace_entries,
            by_source,
            occurrences,
        })
    }
}

/// Complete source identity/occurrence attribution;not an evaluated reflected Piece.
/// ```compile_fail
/// fn reparent<'a>(mut coverage:sc_core::construction::MirrorPieceEdgeCoverage<'a>,
///     other:&'a sc_core::construction::AdmittedMirrorPieceEdges<'a>) { coverage.edges=other; }
/// ```
/// ```compile_fail
/// fn escape(edges:sc_core::construction::AdmittedMirrorPieceEdges<'_>)
///     ->sc_core::construction::MirrorPieceEdgeCoverage<'static> {
///     edges.admit_source_coverage(sc_core::construction::MirrorPieceEdgeCoverageLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct MirrorPieceEdgeCoverage<'a> {
    edges: &'a AdmittedMirrorPieceEdges<'a>,
    limits: MirrorPieceEdgeCoverageLimits,
    workspace_entries: usize,
    by_source: BTreeMap<EdgeRef, SourceMatch<'a>>,
    occurrences: Vec<Occurrence<'a>>,
}
impl<'a> MirrorPieceEdgeCoverage<'a> {
    /// Actual admitted edge candidates and complete parent chain.
    #[must_use]
    pub const fn edges(&self) -> &'a AdmittedMirrorPieceEdges<'a> {
        self.edges
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPieceEdgeCoverageLimits {
        self.limits
    }
    /// Actual N+R index/association entries,not allocated bytes.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Distinct original source edge identities;one actual candidate each.
    #[must_use]
    pub fn source_count(&self) -> usize {
        self.by_source.len()
    }
    /// Every original source occurrence,including repeated uses and separate fold declarations.
    #[must_use]
    pub fn occurrence_count(&self) -> usize {
        self.occurrences.len()
    }
    /// The actual candidate for an exact original source identity;no coordinate or tag-only lookup.
    #[must_use]
    pub fn edge_for_source(&self, source: &EdgeRef) -> Option<MirrorPieceOutputEdge<'a>> {
        self.by_source.get(source).map(|entry| entry.output)
    }
    /// Every original source occurrence in original boundary/hole/construction/fold order.
    pub fn occurrences(
        &self,
    ) -> impl ExactSizeIterator<Item = MirrorPieceMappedEdgeOccurrence<'_>> + DoubleEndedIterator
    {
        self.occurrences
            .iter()
            .map(move |occurrence| MirrorPieceMappedEdgeOccurrence {
                coverage: self,
                occurrence,
            })
    }
}
/// Actual original source field and unique admitted output role at one original occurrence.
#[derive(Clone, Copy, Debug)]
pub struct MirrorPieceMappedEdgeOccurrence<'a> {
    coverage: &'a MirrorPieceEdgeCoverage<'a>,
    occurrence: &'a Occurrence<'a>,
}
impl<'a> MirrorPieceMappedEdgeOccurrence<'a> {
    /// Actual complete coverage owner;not a caller-supplied association.
    #[must_use]
    pub const fn coverage(self) -> &'a MirrorPieceEdgeCoverage<'a> {
        self.coverage
    }
    /// Actual original source occurrence,retaining location,direction and exact field borrow.
    #[must_use]
    pub const fn source(self) -> MirrorPieceEdge<'a> {
        self.occurrence.source
    }
    /// Actual unique admitted candidate for the source identity.
    #[must_use]
    pub const fn output(self) -> MirrorPieceOutputEdge<'a> {
        self.occurrence.output
    }
}
