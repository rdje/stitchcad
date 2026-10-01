//! Born-live and current semantic edge-anchor validation, shared by marks and construction placements.
use super::{EdgeRef, EntityId, IdentityLedger, Param, Piece, RangePortion, Resolution};
use core::fmt;

/// A held point along a stable edge reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeAnchor {
    /// Persistent edge identity.
    pub edge: EdgeRef,
    /// Exact parameter in the edge's original frame.
    pub param: Param,
}

/// Structural semantic-anchor refusal, independent of physical representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnchorError {
    /// The requested anchor's edge does not exist now; creation never invents a repairable reference.
    MissingEdge {
        /// Requested edge identity.
        edge: EdgeRef,
    },
    /// The anchor resolves outside every surviving edge interval of the named piece.
    AnchorOutsidePiece {
        /// Named owner.
        piece: EntityId,
        /// Requested held anchor.
        anchor: EdgeAnchor,
    },
    /// A supplied edge/parameter is not uniquely born-resolved.
    BornUnresolved {
        /// Requested held anchor.
        anchor: EdgeAnchor,
        /// The exact resolution requiring repair or an explicit consumer choice.
        resolution: Box<Resolution>,
    },
    /// A previously valid held anchor now needs repair or an explicit consumer choice.
    CurrentUnresolved {
        /// Held anchor.
        anchor: EdgeAnchor,
        /// Current raw resolution evidence.
        resolution: Box<Resolution>,
    },
}

impl fmt::Display for AnchorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEdge { edge } => {
                write!(f, "semantic anchor edge {edge} must exist at construction")
            }
            Self::AnchorOutsidePiece { piece, anchor } => {
                write!(
                    f,
                    "semantic anchor {} at {} must belong to piece {piece}",
                    anchor.edge, anchor.param
                )
            }
            Self::CurrentUnresolved { anchor, .. } => write!(
                f,
                "semantic anchor {} at {} currently needs repair or an explicit choice",
                anchor.edge, anchor.param
            ),
            Self::BornUnresolved { anchor, .. } => {
                write!(
                    f,
                    "semantic anchor {} at {} must be born uniquely resolved",
                    anchor.edge, anchor.param
                )
            }
        }
    }
}

impl std::error::Error for AnchorError {}

/// Validate a new semantic anchor; physical representation and geometry are separate obligations.
pub(crate) fn validate_anchor(
    anchor: EdgeAnchor,
    piece: &Piece,
    ledger: &IdentityLedger,
) -> Result<(), AnchorError> {
    if !ledger.is_live(anchor.edge) {
        return Err(AnchorError::MissingEdge { edge: anchor.edge });
    }
    validate_current_anchor(anchor, piece, ledger).map_err(|error| match error {
        AnchorError::CurrentUnresolved { anchor, resolution } => {
            AnchorError::BornUnresolved { anchor, resolution }
        }
        other => other,
    })
}

/// Validate current resolved ownership; a held historical edge need not still be live itself.
/// Unlike birth validation, this follows the journal and preserves choices or repair evidence.
pub(crate) fn validate_current_anchor(
    anchor: EdgeAnchor,
    piece: &Piece,
    ledger: &IdentityLedger,
) -> Result<(), AnchorError> {
    let resolution = ledger.resolve(anchor.edge, anchor.param);
    let Some(point) = resolution.resolved() else {
        return Err(AnchorError::CurrentUnresolved {
            anchor,
            resolution: Box::new(resolution),
        });
    };
    let belongs = piece.range_resolutions(ledger).any(|(_, range)| {
        range.portions().iter().any(|portion| match portion {
            RangePortion::Resolved(part) => {
                part.range().edge() == point.edge()
                    && part.range().from() <= point.param()
                    && point.param() <= part.range().to()
            }
            RangePortion::Unresolved(_) => false,
        })
    });
    if !belongs {
        return Err(AnchorError::AnchorOutsidePiece {
            piece: piece.id(),
            anchor,
        });
    }
    Ok(())
}
