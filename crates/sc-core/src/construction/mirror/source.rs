//! Actual source-Piece association before result metadata, identities or reflected geometry.
use super::MirrorPieceCopy;
use crate::ontology::{DirectedEdge, EdgeRef, EntityId, Piece};
use core::fmt;

/// Explicit complete source population limits, without a default allowance.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPieceSourceLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceSourceLimits {
    holes: usize,
    edges: usize,
}
impl MirrorPieceSourceLimits {
    /// Bound all hole groups and all boundary/hole/construction/fold edge occurrences.
    /// A repeated construction use or separately declared fold counts again.
    #[must_use]
    pub const fn new(holes: usize, edges: usize) -> Self {
        Self { holes, edges }
    }
    /// Complete hole-group allowance, before reading individual hole lengths.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// Complete original edge-occurrence allowance, before inspecting source identities.
    #[must_use]
    pub const fn edges(self) -> usize {
        self.edges
    }
}
/// An original source field's location, without replacing its stable identity with an index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceEdgeLocation {
    /// Outer-boundary position.
    Boundary(usize),
    /// Original hole and position within that hole.
    Hole {
        /// Original hole-group position.
        hole: usize,
        /// Original edge position.
        edge: usize,
    },
    /// Non-cutting construction-line occurrence.
    Construction(usize),
    /// Separate fold declaration, even when the same edge also occurs on the boundary.
    Fold(usize),
}
/// Original borrowed directed contour/construction field or undirected fold declaration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceEdgeSource<'a> {
    /// Actual directed field; its original traversal is unchanged.
    Directed(&'a DirectedEdge),
    /// Actual fold-edge field; no direction is manufactured for this declaration.
    Fold(&'a EdgeRef),
}
impl<'a> MirrorPieceEdgeSource<'a> {
    /// Actual original persistent edge field.
    #[must_use]
    pub const fn edge(self) -> &'a EdgeRef {
        match self {
            Self::Directed(edge) => &edge.edge,
            Self::Fold(edge) => edge,
        }
    }
}
/// First source association, whole-population or definite local self-supply failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceSourceRefusal {
    /// The actual supplied Piece differs from the original required identity.
    PieceIdentity {
        /// Original required Piece identity.
        expected: EntityId,
        /// Actual supplied Piece identity.
        actual: EntityId,
    },
    /// Whole hole-group quota exceeded before individual hole lengths are scanned.
    Holes {
        /// Complete actual group count.
        required: usize,
        /// Explicit caller allowance.
        allowed: usize,
    },
    /// Complete edge-occurrence count cannot be represented by usize.
    CountOverflow,
    /// Whole edge-occurrence quota exceeded before any edge-source scan.
    Edges {
        /// Complete actual occurrence count.
        required: usize,
        /// Explicit caller allowance.
        allowed: usize,
    },
    /// An original required source edge would be supplied by this operation itself.
    SelfSource {
        /// Actual mirror operation identity.
        operation: EntityId,
        /// First original field in boundary/hole/construction/fold order.
        location: MirrorPieceEdgeLocation,
        /// Exact original persistent edge identity and tag.
        edge: EdgeRef,
    },
}
/// The actual input, supplied source and explicit limits survive every refusal.
pub struct MirrorPieceSourceError<'a> {
    input: &'a MirrorPieceCopy,
    piece: &'a Piece,
    limits: &'a MirrorPieceSourceLimits,
    refusal: MirrorPieceSourceRefusal,
}
impl<'a> MirrorPieceSourceError<'a> {
    /// Actual original mirror input, including its authored axis.
    #[must_use]
    pub const fn input(&self) -> &'a MirrorPieceCopy {
        self.input
    }
    /// Actual supplied immutable Piece, including all original metadata.
    #[must_use]
    pub const fn piece(&self) -> &'a Piece {
        self.piece
    }
    /// Actual caller-owned complete population policy.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceSourceLimits {
        self.limits
    }
    /// First complete association or admission cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceSourceRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceSourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceSourceError")
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceSourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror Piece source: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceSourceError<'_> {}

impl MirrorPieceCopy {
    /// Bind the actual original Piece without allocating, copying metadata or changing any source.
    /// H bounds the hole-length scan; R counts every original edge field, including repeated uses
    /// and the fold declaration. Admission uses O(H+R) work and constant auxiliary space.
    /// This is snapshot association only: current Design/order/ledger/frame, geometric validity,
    /// reflection, semantic consumers, result identities and publication remain separate checks.
    /// # Errors
    /// Piece identity, whole H quota, checked R sum, whole R quota, then first local self-source
    /// in original boundary/hole/construction/fold order. No partial bound view escapes.
    pub fn bind_piece<'a>(
        &'a self,
        piece: &'a Piece,
        limits: &'a MirrorPieceSourceLimits,
    ) -> Result<BoundMirrorPieceCopy<'a>, MirrorPieceSourceError<'a>> {
        let refuse = |refusal| MirrorPieceSourceError {
            input: self,
            piece,
            limits,
            refusal,
        };
        if piece.id() != self.source_piece() {
            return Err(refuse(MirrorPieceSourceRefusal::PieceIdentity {
                expected: self.source_piece(),
                actual: piece.id(),
            }));
        }
        let source = piece.definition();
        if source.holes.len() > limits.holes {
            return Err(refuse(MirrorPieceSourceRefusal::Holes {
                required: source.holes.len(),
                allowed: limits.holes,
            }));
        }
        let edge_count = [
            source.boundary.len(),
            source.construction_lines.len(),
            source.fold_edges.len(),
        ]
        .into_iter()
        .chain(source.holes.iter().map(Vec::len))
        .try_fold(0usize, usize::checked_add)
        .ok_or_else(|| refuse(MirrorPieceSourceRefusal::CountOverflow))?;
        if edge_count > limits.edges {
            return Err(refuse(MirrorPieceSourceRefusal::Edges {
                required: edge_count,
                allowed: limits.edges,
            }));
        }
        for (location, source) in edge_fields(piece) {
            let edge = *source.edge();
            if edge.creator() == self.operation() {
                return Err(refuse(MirrorPieceSourceRefusal::SelfSource {
                    operation: self.operation(),
                    location,
                    edge,
                }));
            }
        }
        Ok(BoundMirrorPieceCopy {
            input: self,
            piece,
            limits,
            edge_count,
        })
    }
}

fn edge_fields(
    piece: &Piece,
) -> impl DoubleEndedIterator<Item = (MirrorPieceEdgeLocation, MirrorPieceEdgeSource<'_>)> {
    let source = piece.definition();
    let boundary = source.boundary.iter().enumerate().map(|(index, edge)| {
        (
            MirrorPieceEdgeLocation::Boundary(index),
            MirrorPieceEdgeSource::Directed(edge),
        )
    });
    let holes = source.holes.iter().enumerate().flat_map(|(hole, edges)| {
        edges.iter().enumerate().map(move |(index, edge)| {
            (
                MirrorPieceEdgeLocation::Hole { hole, edge: index },
                MirrorPieceEdgeSource::Directed(edge),
            )
        })
    });
    let construction = source
        .construction_lines
        .iter()
        .enumerate()
        .map(|(index, edge)| {
            (
                MirrorPieceEdgeLocation::Construction(index),
                MirrorPieceEdgeSource::Directed(edge),
            )
        });
    let folds = source.fold_edges.iter().enumerate().map(|(index, edge)| {
        (
            MirrorPieceEdgeLocation::Fold(index),
            MirrorPieceEdgeSource::Fold(edge),
        )
    });
    boundary.chain(holes).chain(construction).chain(folds)
}

/// Authentic actual Piece/input association, without result metadata or geometry authority.
/// ```compile_fail
/// fn forge<'a>(mut bound:sc_core::construction::BoundMirrorPieceCopy<'a>,other:&'a sc_core::ontology::Piece) {
///     bound.piece=other;
/// }
/// ```
/// ```compile_fail
/// fn escape(input:sc_core::construction::MirrorPieceCopy,piece:sc_core::ontology::Piece,
///     limits:sc_core::construction::MirrorPieceSourceLimits)->sc_core::construction::BoundMirrorPieceCopy<'static> {
///     input.bind_piece(&piece,&limits).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundMirrorPieceCopy<'a> {
    input: &'a MirrorPieceCopy,
    piece: &'a Piece,
    limits: &'a MirrorPieceSourceLimits,
    edge_count: usize,
}
impl<'a> BoundMirrorPieceCopy<'a> {
    /// Actual original mirror input, retaining both explicit axis modes unchanged.
    #[must_use]
    pub const fn input(&self) -> &'a MirrorPieceCopy {
        self.input
    }
    /// Actual complete immutable source Piece, not a copied or reflected replacement.
    #[must_use]
    pub const fn piece(&self) -> &'a Piece {
        self.piece
    }
    /// Actual complete source-population policy.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceSourceLimits {
        self.limits
    }
    /// Complete actual original occurrence count, including repeated fields.
    #[must_use]
    pub const fn edge_count(&self) -> usize {
        self.edge_count
    }
    /// Authentic fields in boundary/hole/construction/fold order; reverse retains original indices.
    pub fn edges(&self) -> impl DoubleEndedIterator<Item = MirrorPieceEdge<'_>> + '_ {
        edge_fields(self.piece).map(move |(location, source)| MirrorPieceEdge {
            copy: self,
            location,
            source,
        })
    }
}
/// One original field occurrence from an actual admitted Piece/input association.
#[derive(Clone, Copy, Debug)]
pub struct MirrorPieceEdge<'a> {
    copy: &'a BoundMirrorPieceCopy<'a>,
    location: MirrorPieceEdgeLocation,
    source: MirrorPieceEdgeSource<'a>,
}
impl<'a> MirrorPieceEdge<'a> {
    /// Actual owning source association; the occurrence cannot be reparented by callers.
    #[must_use]
    pub const fn copy(self) -> &'a BoundMirrorPieceCopy<'a> {
        self.copy
    }
    /// Actual original field position and role.
    #[must_use]
    pub const fn location(self) -> MirrorPieceEdgeLocation {
        self.location
    }
    /// Exact underlying source field borrow and original direction where supplied.
    #[must_use]
    pub const fn source(self) -> MirrorPieceEdgeSource<'a> {
        self.source
    }
}
