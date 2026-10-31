//! Ordered retained/output edge uses and actual source Piece association;not geometric closure.
use super::{AdmittedSlashSpreadEdges, SlashSpreadEdge};
use crate::ontology::{DirectedEdge, Direction, EdgeRef, EntityId, LocalTag, Piece};
use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

/// An explicitly directed new logical output or retained original edge identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadContourEdge {
    /// Use a candidate from the actual admitted edge manifest.
    Output {
        /// Explicit output tag,not a candidate position.
        tag: LocalTag,
        /// Traversal relative to that output edge's own original parameter frame.
        direction: Direction,
    },
    /// Preserve this original source edge identity,with explicit contour traversal.
    Retained(DirectedEdge),
}
impl SlashSpreadContourEdge {
    /// Actual authored traversal;no source/motion/parameter normalization is performed.
    #[must_use]
    pub const fn direction(&self) -> Direction {
        match self {
            Self::Output { direction, .. } => *direction,
            Self::Retained(edge) => edge.direction,
        }
    }
}
/// Original location of one contour occurrence;positions are diagnostics,not persistent identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadContourLocation {
    /// Zero-based occurrence in the original boundary list.
    Boundary(usize),
    /// Original hole and occurrence within it.
    Hole {
        /// Zero-based original hole position.
        hole: usize,
        /// Zero-based original occurrence position.
        edge: usize,
    },
    /// Zero-based occurrence in the original construction list.
    Construction(usize),
}
/// Explicit complete group/candidate/source/use allowances;no default or prior-storage guarantee.
/// ```compile_fail
/// let limits:sc_core::construction::SlashSpreadContourLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadContourLimits {
    candidates: usize,
    source_edges: usize,
    source_holes: usize,
    holes: usize,
    uses: usize,
}
impl SlashSpreadContourLimits {
    /// Select complete N candidates,R source occurrences,source holes,target holes and M uses.
    #[must_use]
    pub const fn new(
        candidates: usize,
        source_edges: usize,
        source_holes: usize,
        holes: usize,
        uses: usize,
    ) -> Self {
        Self {
            candidates,
            source_edges,
            source_holes,
            holes,
            uses,
        }
    }
    /// Complete admitted candidate allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Complete original source edge occurrences,including construction repetitions.
    #[must_use]
    pub const fn source_edges(self) -> usize {
        self.source_edges
    }
    /// Complete original source hole-group allowance,before traversing those groups.
    #[must_use]
    pub const fn source_holes(self) -> usize {
        self.source_holes
    }
    /// Complete target hole-group allowance,before traversing those groups.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// Complete original target edge-use allowance,including repeated construction uses.
    #[must_use]
    pub const fn uses(self) -> usize {
        self.uses
    }
}
/// Whole-resource,source association or original contour/candidate refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadContourRefusal {
    /// A complete occurrence or workspace population cannot be counted.
    CountOverflow,
    /// Complete target hole groups exceed policy.
    Holes {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete source hole groups exceed policy.
    SourceHoles {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete admitted candidate population exceeds policy.
    Candidates {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete source occurrence population exceeds policy.
    SourceEdges {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete target occurrence population exceeds policy.
    Uses {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Actual provided Piece identity differs from the original slash target.
    SourcePiece {
        /// Required original target identity.
        required: EntityId,
        /// Actual provided Piece identity.
        provided: EntityId,
    },
    /// A cyclic boundary declaration requires at least one edge use.
    EmptyBoundary,
    /// Original target hole declaration is empty.
    EmptyHole {
        /// Actual original hole position.
        hole: usize,
    },
    /// Actual source Piece contains an edge supplied by this same operation.
    SelfSource {
        /// Original flattened source boundary/hole/construction occurrence index.
        source_edge: usize,
    },
    /// Target use names no actual admitted candidate.
    MissingOutput {
        /// Actual original target position.
        location: SlashSpreadContourLocation,
        /// Original explicit tag.
        tag: LocalTag,
    },
    /// A retained use cannot bypass output admission by naming this same operation.
    SelfRetained {
        /// Actual original target position.
        location: SlashSpreadContourLocation,
    },
    /// Retained identity occurs nowhere in the original source Piece.
    ForeignRetained {
        /// Actual original target position.
        location: SlashSpreadContourLocation,
        /// Original retained identity.
        edge: EdgeRef,
    },
    /// Boundary/hole cut identity repeats;construction occurrences remain independently allowed.
    RepeatedCut {
        /// Actual original first cut occurrence.
        first: SlashSpreadContourLocation,
        /// Actual original repeated cut occurrence.
        second: SlashSpreadContourLocation,
    },
    /// Actual admitted output has no declared boundary/hole/construction use.
    UnusedCandidate {
        /// Actual original candidate index.
        candidate: usize,
        /// Actual explicit candidate tag.
        tag: LocalTag,
    },
}
/// Actual raw contours,admitted edges,source Piece,limits and original cause survive refusal.
pub struct SlashSpreadContourError<'a> {
    contours: &'a SlashSpreadContours,
    edges: &'a AdmittedSlashSpreadEdges<'a>,
    source: &'a Piece,
    limits: &'a SlashSpreadContourLimits,
    refusal: SlashSpreadContourRefusal,
}
impl<'a> SlashSpreadContourError<'a> {
    /// Actual whole original contour declaration.
    #[must_use]
    pub const fn contours(&self) -> &'a SlashSpreadContours {
        self.contours
    }
    /// Actual complete admitted candidate source.
    #[must_use]
    pub const fn edges(&self) -> &'a AdmittedSlashSpreadEdges<'a> {
        self.edges
    }
    /// Actual provided immutable source Piece,without current-root authority.
    #[must_use]
    pub const fn source_piece(&self) -> &'a Piece {
        self.source
    }
    /// Original selected admission policy.
    #[must_use]
    pub const fn limits(&self) -> &'a SlashSpreadContourLimits {
        self.limits
    }
    /// First complete-resource,source or original target/candidate cause.
    #[must_use]
    pub const fn refusal(&self) -> SlashSpreadContourRefusal {
        self.refusal
    }
    /// Repeated cut identity read from its actual original first occurrence,without duplicating it
    /// in the refusal payload. Other refusal kinds return None.
    #[must_use]
    pub fn repeated_edge(&self) -> Option<EdgeRef> {
        let SlashSpreadContourRefusal::RepeatedCut { first, .. } = self.refusal else {
            return None;
        };
        let declared = match first {
            SlashSpreadContourLocation::Boundary(edge) => self.contours.boundary.get(edge),
            SlashSpreadContourLocation::Hole { hole, edge } => {
                self.contours.holes.get(hole)?.get(edge)
            }
            SlashSpreadContourLocation::Construction(edge) => self.contours.construction.get(edge),
        }?;
        Some(match declared {
            SlashSpreadContourEdge::Output { tag, .. } => {
                EdgeRef::new(self.edges.manifest().operation(), *tag)
            }
            SlashSpreadContourEdge::Retained(directed) => directed.edge,
        })
    }
}
impl fmt::Debug for SlashSpreadContourError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadContourError")
            .field("source_piece", &self.source.definition().id)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadContourError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slash/spread contour admission: {:?}", self.refusal)
    }
}
impl std::error::Error for SlashSpreadContourError<'_> {}

/// Owned raw ordered cyclic cut lists and construction uses;no coordinates or source metadata copy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlashSpreadContours {
    boundary: Vec<SlashSpreadContourEdge>,
    holes: Vec<Vec<SlashSpreadContourEdge>>,
    construction: Vec<SlashSpreadContourEdge>,
}
fn counts<T>(boundary: &[T], holes: &[Vec<T>], construction: &[T]) -> Option<(usize, usize)> {
    let cut = holes
        .iter()
        .try_fold(boundary.len(), |sum, hole| sum.checked_add(hole.len()))?;
    Some((cut, cut.checked_add(construction.len())?))
}
impl SlashSpreadContours {
    /// Own all original lists without sorting,deduplication or accepting geometry.
    #[must_use]
    pub const fn new(
        boundary: Vec<SlashSpreadContourEdge>,
        holes: Vec<Vec<SlashSpreadContourEdge>>,
        construction: Vec<SlashSpreadContourEdge>,
    ) -> Self {
        Self {
            boundary,
            holes,
            construction,
        }
    }
    /// Actual original boundary uses,without repeating a terminal cut edge.
    #[must_use]
    pub fn boundary(&self) -> &[SlashSpreadContourEdge] {
        &self.boundary
    }
    /// Actual original hole groups and their original orders.
    #[must_use]
    pub fn holes(&self) -> &[Vec<SlashSpreadContourEdge>] {
        &self.holes
    }
    /// Actual original construction occurrences,repetitions included.
    #[must_use]
    pub fn construction_lines(&self) -> &[SlashSpreadContourEdge] {
        &self.construction
    }
    /// Admit whole group/population counts before source/content/index allocation. Then require the
    /// actual target Piece,nonempty boundary,no source self-cycle and original ordered cut/use rules.
    /// Conservative peak entry bound is 2N+R+K+M:output index/used set,source set,cut index,retained uses.
    /// These counts grant no prior Vec/String,whole allocation-byte,currentness or geometry proof.
    /// # Errors
    /// Target/source hole quota,count/workspace,N/R/M quota,source id,empty boundary,source self-cycle,
    /// first original boundary/hole/construction cause,then first unused original candidate.
    pub fn admit<'a>(
        &'a self,
        edges: &'a AdmittedSlashSpreadEdges<'a>,
        source: &'a Piece,
        limits: &'a SlashSpreadContourLimits,
    ) -> Result<AdmittedSlashSpreadContours<'a>, SlashSpreadContourError<'a>> {
        let refuse = |refusal| SlashSpreadContourError {
            contours: self,
            edges,
            source,
            limits,
            refusal,
        };
        let definition = source.definition();
        if self.holes.len() > limits.holes {
            return Err(refuse(SlashSpreadContourRefusal::Holes {
                required: self.holes.len(),
                allowed: limits.holes,
            }));
        }
        if definition.holes.len() > limits.source_holes {
            return Err(refuse(SlashSpreadContourRefusal::SourceHoles {
                required: definition.holes.len(),
                allowed: limits.source_holes,
            }));
        }
        let overflow = || refuse(SlashSpreadContourRefusal::CountOverflow);
        let (k, m) =
            counts(&self.boundary, &self.holes, &self.construction).ok_or_else(overflow)?;
        let (_, r) = counts(
            &definition.boundary,
            &definition.holes,
            &definition.construction_lines,
        )
        .ok_or_else(overflow)?;
        let n = edges.manifest().candidates().len();
        let workspace_entries = n
            .checked_mul(2)
            .and_then(|sum| sum.checked_add(r))
            .and_then(|sum| sum.checked_add(k))
            .and_then(|sum| sum.checked_add(m))
            .ok_or_else(overflow)?;
        if n > limits.candidates {
            return Err(refuse(SlashSpreadContourRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if r > limits.source_edges {
            return Err(refuse(SlashSpreadContourRefusal::SourceEdges {
                required: r,
                allowed: limits.source_edges,
            }));
        }
        if m > limits.uses {
            return Err(refuse(SlashSpreadContourRefusal::Uses {
                required: m,
                allowed: limits.uses,
            }));
        }
        let input = edges.bound_roles().intent().input();
        if definition.id != input.piece() {
            return Err(refuse(SlashSpreadContourRefusal::SourcePiece {
                required: input.piece(),
                provided: definition.id,
            }));
        }
        if self.boundary.is_empty() {
            return Err(refuse(SlashSpreadContourRefusal::EmptyBoundary));
        }
        let mut source_edges = BTreeSet::new();
        for (source_edge, directed) in source.edges().enumerate() {
            if directed.edge.creator() == input.operation() {
                return Err(refuse(SlashSpreadContourRefusal::SelfSource {
                    source_edge,
                }));
            }
            source_edges.insert(directed.edge);
        }
        let outputs = edges
            .outputs()
            .map(|output| (output.candidate().tag(), output))
            .collect::<BTreeMap<_, _>>();
        let mut used = BTreeSet::new();
        let mut cut = BTreeMap::new();
        let mut uses = Vec::with_capacity(m);
        {
            let mut accept = |location, declared: &'a SlashSpreadContourEdge, is_cut| {
                let target = match declared {
                    SlashSpreadContourEdge::Output { tag, .. } => {
                        let output = outputs.get(tag).copied().ok_or_else(|| {
                            refuse(SlashSpreadContourRefusal::MissingOutput {
                                location,
                                tag: *tag,
                            })
                        })?;
                        used.insert(*tag);
                        Target::Output(output)
                    }
                    SlashSpreadContourEdge::Retained(directed) => {
                        if directed.edge.creator() == input.operation() {
                            return Err(refuse(SlashSpreadContourRefusal::SelfRetained {
                                location,
                            }));
                        }
                        if !source_edges.contains(&directed.edge) {
                            return Err(refuse(SlashSpreadContourRefusal::ForeignRetained {
                                location,
                                edge: directed.edge,
                            }));
                        }
                        Target::Retained(directed)
                    }
                };
                let edge = target.edge();
                if is_cut {
                    if let Some(first) = cut.insert(edge, location) {
                        return Err(refuse(SlashSpreadContourRefusal::RepeatedCut {
                            first,
                            second: location,
                        }));
                    }
                }
                uses.push(Use {
                    location,
                    declared,
                    target,
                });
                Ok(())
            };
            for (edge, declared) in self.boundary.iter().enumerate() {
                accept(SlashSpreadContourLocation::Boundary(edge), declared, true)?;
            }
            for (hole, entries) in self.holes.iter().enumerate() {
                if entries.is_empty() {
                    return Err(refuse(SlashSpreadContourRefusal::EmptyHole { hole }));
                }
                for (edge, declared) in entries.iter().enumerate() {
                    accept(
                        SlashSpreadContourLocation::Hole { hole, edge },
                        declared,
                        true,
                    )?;
                }
            }
            for (edge, declared) in self.construction.iter().enumerate() {
                accept(
                    SlashSpreadContourLocation::Construction(edge),
                    declared,
                    false,
                )?;
            }
        }
        for output in edges.outputs() {
            if !used.contains(&output.candidate().tag()) {
                return Err(refuse(SlashSpreadContourRefusal::UnusedCandidate {
                    candidate: output.index(),
                    tag: output.candidate().tag(),
                }));
            }
        }
        Ok(AdmittedSlashSpreadContours {
            contours: self,
            edges,
            source,
            limits,
            workspace_entries,
            source_edge_count: r,
            uses,
        })
    }
}
#[derive(Clone, Copy, Debug)]
enum Target<'a> {
    Output(SlashSpreadEdge<'a>),
    Retained(&'a DirectedEdge),
}
impl Target<'_> {
    fn edge(self) -> EdgeRef {
        match self {
            Self::Output(output) => output.logical_output(),
            Self::Retained(directed) => directed.edge,
        }
    }
}
#[derive(Debug)]
struct Use<'a> {
    location: SlashSpreadContourLocation,
    declared: &'a SlashSpreadContourEdge,
    target: Target<'a>,
}

/// Complete structural contour/use/source association,not an actual validated replacement Piece.
/// ```compile_fail
/// fn escape(contours:sc_core::construction::SlashSpreadContours,edges:sc_core::construction::AdmittedSlashSpreadEdges<'_>,piece:sc_core::ontology::Piece)
///     ->sc_core::construction::AdmittedSlashSpreadContours<'static> {
///     contours.admit(&edges,&piece,&sc_core::construction::SlashSpreadContourLimits::new(0,0,0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedSlashSpreadContours<'a> {
    contours: &'a SlashSpreadContours,
    edges: &'a AdmittedSlashSpreadEdges<'a>,
    source: &'a Piece,
    limits: &'a SlashSpreadContourLimits,
    workspace_entries: usize,
    source_edge_count: usize,
    uses: Vec<Use<'a>>,
}
impl<'a> AdmittedSlashSpreadContours<'a> {
    /// Actual whole original contour declaration.
    #[must_use]
    pub const fn contours(&self) -> &'a SlashSpreadContours {
        self.contours
    }
    /// Actual admitted candidate/provenance source.
    #[must_use]
    pub const fn edges(&self) -> &'a AdmittedSlashSpreadEdges<'a> {
        self.edges
    }
    /// Actual immutable source Piece;source association is not current Design authority.
    #[must_use]
    pub const fn source_piece(&self) -> &'a Piece {
        self.source
    }
    /// Original selected admission policy.
    #[must_use]
    pub const fn limits(&self) -> &'a SlashSpreadContourLimits {
        self.limits
    }
    /// Conservative complete 2N+R+K+M entry bound,not measured entries or allocation bytes.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Complete actual original source occurrences,including repetitions;not unique index entries.
    #[must_use]
    pub const fn source_edge_count(&self) -> usize {
        self.source_edge_count
    }
    /// All original occurrences:boundary,holes in order,then construction;no deduplication.
    pub fn uses(
        &self,
    ) -> impl ExactSizeIterator<Item = SlashSpreadContourUse<'_>> + DoubleEndedIterator {
        self.uses.iter().map(move |usage| SlashSpreadContourUse {
            contours: self,
            usage,
        })
    }
}
/// Authentic original contour occurrence and its actual output or retained-reference association.
/// ```compile_fail
/// fn reparent<'a>(mut usage:sc_core::construction::SlashSpreadContourUse<'a>,other:&'a sc_core::construction::AdmittedSlashSpreadContours<'a>) {
///     usage.contours=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SlashSpreadContourUse<'a> {
    contours: &'a AdmittedSlashSpreadContours<'a>,
    usage: &'a Use<'a>,
}
impl<'a> SlashSpreadContourUse<'a> {
    /// Actual complete admitted contour owner.
    #[must_use]
    pub const fn contours(self) -> &'a AdmittedSlashSpreadContours<'a> {
        self.contours
    }
    /// Actual original list position,not persistent identity.
    #[must_use]
    pub const fn location(self) -> SlashSpreadContourLocation {
        self.usage.location
    }
    /// Actual original field in the raw boundary/hole/construction list.
    #[must_use]
    pub const fn declared(self) -> &'a SlashSpreadContourEdge {
        self.usage.declared
    }
    /// Authentic candidate view for a new logical output,without coordinate/currentness proof.
    #[must_use]
    pub const fn output(self) -> Option<SlashSpreadEdge<'a>> {
        match self.usage.target {
            Target::Output(output) => Some(output),
            Target::Retained(_) => None,
        }
    }
    /// Actual raw retained field in this contour plan;membership is by exact source identity.
    #[must_use]
    pub const fn retained(self) -> Option<&'a DirectedEdge> {
        match self.usage.target {
            Target::Retained(directed) => Some(directed),
            Target::Output(_) => None,
        }
    }
    /// Logical output reference or unchanged original retained identity,without issuing anything.
    #[must_use]
    pub fn edge(self) -> EdgeRef {
        self.usage.target.edge()
    }
    /// Actual authored traversal in that edge's own parameter frame.
    #[must_use]
    pub const fn direction(self) -> Direction {
        self.usage.declared.direction()
    }
}
