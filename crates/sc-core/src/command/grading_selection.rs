//! Explicit ordered member selection against one canonical SizeSet snapshot, before membership lookup.
use crate::ontology::EntityId;
use core::fmt;
use sc_units::Count;
use std::collections::BTreeMap;

/// Nonempty distinct member claims under one exact SizeSet identity/revision; no labels or quantities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GradedSizeSelection {
    size_set: EntityId,
    revision: Count,
    members: Vec<EntityId>,
}
/// Invalid explicit member inventory, retaining the actual authored positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GradedSizeSelectionError {
    /// No target member was supplied.
    NoMembers,
    /// First repeated identity in the original vector; positions are zero-based.
    DuplicateMember {
        /// Actual repeated member.
        member: EntityId,
        /// First supplied position.
        first: usize,
        /// First repeated position.
        duplicate: usize,
    },
}
impl fmt::Display for GradedSizeSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMembers => f.write_str("graded size selection requires at least one member"),
            Self::DuplicateMember {
                member,
                first,
                duplicate,
            } => {
                write!(f,
                "graded member {member} repeats at position {duplicate}, first supplied at {first}")
            }
        }
    }
}
impl std::error::Error for GradedSizeSelectionError {}
impl GradedSizeSelection {
    /// Preserve the exact shared set pin and member order; do not infer/sort labels or a base member.
    /// # Errors
    /// Refuses an empty inventory or its first actual repeated member. Registry membership is separate.
    pub fn new(
        size_set: EntityId,
        revision: Count,
        members: Vec<EntityId>,
    ) -> Result<Self, GradedSizeSelectionError> {
        if members.is_empty() {
            return Err(GradedSizeSelectionError::NoMembers);
        }
        let mut positions = BTreeMap::new();
        for (duplicate, member) in members.iter().copied().enumerate() {
            if let Some(first) = positions.insert(member, duplicate) {
                return Err(GradedSizeSelectionError::DuplicateMember {
                    member,
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self {
            size_set,
            revision,
            members,
        })
    }
    /// Exact set identity whose membership/breaks execution must resolve.
    #[must_use]
    pub const fn size_set(&self) -> EntityId {
        self.size_set
    }
    /// Exact canonical Count revision, separate from the Design revision.
    #[must_use]
    pub const fn revision(&self) -> Count {
        self.revision
    }
    /// Authored member order without inferred completeness, quantities or base selection.
    #[must_use]
    pub fn members(&self) -> &[EntityId] {
        &self.members
    }
}
