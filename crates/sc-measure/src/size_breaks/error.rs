//! Explicit requirement and adjacent-cell inventory/coverage refusals.
use super::SizeBreakCell;
use crate::{SizeBreakError, SizeSetReference};
use core::fmt;
use sc_core::ontology::EntityId;

/// Actual current inventory or grading-coverage refusal, without guessed dimensions or zero breaks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeBreaksError {
    /// Supplied membership differs from the authored collection pin.
    MembershipMismatch {
        /// Exact authored reference.
        expected: SizeSetReference,
        /// Actual current reference.
        actual: SizeSetReference,
    },
    /// Original requirement/context/pair refusal.
    Context(SizeBreakError),
    /// An actual available declaration no longer validates.
    Break {
        /// Original declaration identity.
        declaration: EntityId,
        /// Original current-source/key failure.
        issue: Box<SizeBreakError>,
    },
    /// Two requirements name the same axis and canonical measurement dimension.
    DuplicateDimension {
        /// Repeated axis.
        axis: EntityId,
        /// Repeated measurement.
        dimension: EntityId,
    },
    /// A declaration/query names an axis/dimension outside the explicit required inventory.
    UndeclaredDimension {
        /// Requested axis.
        axis: EntityId,
        /// Requested measurement.
        dimension: EntityId,
    },
    /// Two available declarations reuse one stable identity.
    DuplicateIdentity(EntityId),
    /// Distinct available declarations target the same exact axis/dimension/member-pair cell.
    DuplicateCell(SizeBreakCell),
    /// No explicit dimensions were authored; available inputs never choose them.
    MissingDimensions(EntityId),
    /// One exact required/queried cell has no authored declaration.
    MissingCell(SizeBreakCell),
    /// A one-member set cannot be graded, regardless of an empty gap list.
    Singleton {
        /// Actual set.
        set: EntityId,
        /// Sole actual member/base.
        member: EntityId,
    },
}
impl fmt::Display for SizeBreaksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size break inventory: {self:?}")
    }
}
impl std::error::Error for SizeBreaksError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Context(issue) => Some(issue),
            Self::Break { issue, .. } => Some(issue),
            _ => None,
        }
    }
}
