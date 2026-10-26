//! Actual break/context refusals, preserving underlying measurement, axis and value evidence.
use crate::{
    MeasurementBinding, MeasurementTableError, SizeAxesError, SizeMembershipError, SizeSetReference,
};
use core::fmt;
use sc_core::{ontology::EntityId, value::LengthValueError};

/// Structural/current signed break refusal, never an inferred dimension or zero delta.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeBreakError {
    /// Current axes do not validate against the supplied actual membership/source records.
    Axes(SizeAxesError),
    /// Original table/dimension/measurement failure.
    Table(MeasurementTableError),
    /// Metadata reuses an identity from the actual membership/axes/table/canonical inventories.
    DuplicateIdentity(EntityId),
    /// Two supplied canonical inventories disagree about the content or record role of one identity.
    ConflictingSource(EntityId),
    /// Actual membership differs from the break's exact authored reference.
    MembershipMismatch {
        /// Authored reference.
        expected: SizeSetReference,
        /// Actual supplied reference.
        actual: SizeSetReference,
    },
    /// Requested axis is not part of the current complete axes aggregate.
    MissingAxis(EntityId),
    /// Original actual member lookup refusal.
    Member(SizeMembershipError),
    /// Requested ordered member pair is not immediately adjacent in the authored sequence.
    NotAdjacent {
        /// Selected set identity.
        set: EntityId,
        /// Authored earlier member.
        from: EntityId,
        /// Authored later member.
        to: EntityId,
    },
    /// Supplied table is not the break's explicitly named dimension table.
    TableMismatch {
        /// Authored table identity.
        expected: EntityId,
        /// Actual table identity.
        actual: EntityId,
    },
    /// Current selected dimension differs from its saved identity/token/domain/declaration binding.
    DimensionMismatch {
        /// Authored dimension binding.
        expected: Box<MeasurementBinding>,
        /// Actual current binding.
        actual: Box<MeasurementBinding>,
    },
    /// The actual shared canonical inventory lacks the delta declaration.
    MissingDelta(EntityId),
    /// A general scalar cannot silently substitute for the explicitly dedicated Length source role.
    DeltaRole(EntityId),
    /// Original required observation/formula refusal on an explicit numeric read.
    Value(LengthValueError),
}
impl fmt::Display for SizeBreakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size break: {self:?}")
    }
}
impl std::error::Error for SizeBreakError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Axes(issue) => Some(issue),
            Self::Table(issue) => Some(issue),
            Self::Member(issue) => Some(issue),
            Self::Value(issue) => Some(issue),
            _ => None,
        }
    }
}
