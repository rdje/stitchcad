//! Concrete local membership, axis and coordinate failures.
use crate::{SizeAxisError, SizeMembershipError, SizeSetReference};
use core::fmt;
use sc_core::ontology::EntityId;

/// Complete explicit axis/member-point refusal, without defaults or a replacement record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeAxesError {
    /// Supplied membership differs from the authored aggregate pin.
    MembershipMismatch {
        /// Authored set/revision.
        expected: SizeSetReference,
        /// Actual supplied membership pin.
        actual: SizeSetReference,
    },
    /// Every complete axes aggregate, including a singleton, requires an authored axis.
    EmptyAxes(EntityId),
    /// An axis belongs to another set or revision.
    AxisMismatch {
        /// Actual axis identity.
        axis: EntityId,
        /// Aggregate's authored pin.
        expected: SizeSetReference,
        /// Axis's authored pin.
        actual: SizeSetReference,
    },
    /// Metadata identities alias another axis/value/member/set or an actual canonical source.
    DuplicateIdentity(EntityId),
    /// Two axes use the same exact human name.
    DuplicateName {
        /// First authored axis.
        first: EntityId,
        /// Repeating axis.
        duplicate: EntityId,
        /// Exact repeated name, without trimming/normalization.
        name: String,
    },
    /// Original local axis/current-source refusal.
    Axis(SizeAxisError),
    /// Original actual membership lookup refusal.
    Member(SizeMembershipError),
    /// Two point records name the same member.
    DuplicateMember(EntityId),
    /// An actual member has no authored point.
    MissingMember(EntityId),
    /// A point names an axis outside this aggregate.
    UnknownAxis {
        /// Authored member.
        member: EntityId,
        /// Foreign axis.
        axis: EntityId,
    },
    /// A point names one axis twice.
    DuplicateCoordinate {
        /// Authored member.
        member: EntityId,
        /// Repeated axis.
        axis: EntityId,
    },
    /// A member has no value for an actual axis.
    MissingCoordinate {
        /// Authored member.
        member: EntityId,
        /// Axis needing an explicit value.
        axis: EntityId,
    },
    /// Distinct members claim the same tuple of axis/value identities.
    DuplicatePoint {
        /// First authored member at this point.
        first: EntityId,
        /// Repeating member, independent of coordinate vector order.
        duplicate: EntityId,
    },
    /// A queried axis is absent; no first-axis fallback.
    MissingAxis(EntityId),
}
impl fmt::Display for SizeAxesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size member axes: {self:?}")
    }
}
impl std::error::Error for SizeAxesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Axis(issue) => Some(issue),
            Self::Member(issue) => Some(issue),
            _ => None,
        }
    }
}
