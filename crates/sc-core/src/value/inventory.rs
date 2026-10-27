//! Owned canonical declarations; metadata keeps borrowing these records rather than copying state.
use super::{LengthDeclaration, ScalarInputDeclaration, ScalarInputKind};
use crate::ontology::EntityId;
use core::fmt;
use std::collections::BTreeMap;

/// Original position of an owned declaration, preserving its canonical representation family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalInputPosition {
    /// Dedicated length declaration, used by measurement/Ease metadata.
    Length {
        /// Zero-based index in the authored length vector.
        index: usize,
    },
    /// General scalar declaration, even when its declared kind is Length.
    Scalar {
        /// Zero-based index in the authored scalar vector.
        index: usize,
    },
}

/// Canonical ownership cannot select an unambiguous declaration for construction or replacement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalInputsError {
    /// Replacement names no existing dedicated length; general Length-kind scalars remain distinct.
    MissingLength {
        /// Exact requested canonical declaration identity.
        declaration: EntityId,
    },
    /// An owned identity occurs twice; neither copy is silently selected or deduplicated.
    DuplicateIdentity {
        /// Actual repeated declaration identity.
        identity: EntityId,
        /// First original family/position.
        first: CanonicalInputPosition,
        /// First repeated family/position.
        duplicate: CanonicalInputPosition,
    },
}
impl fmt::Display for CanonicalInputsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLength { declaration } => {
                write!(f, "canonical dedicated length {declaration} is absent")
            }
            Self::DuplicateIdentity {
                identity,
                first,
                duplicate,
            } => {
                write!(f,
                "canonical input {identity} repeats at {duplicate:?}, first supplied at {first:?}")
            }
        }
    }
}
impl std::error::Error for CanonicalInputsError {}

/// An immutable owner of dedicated length and general-scalar declarations, in authored family order.
///
/// This is a component for canonical Design state, not a complete Design, namespace or evidence
/// registry. Referenced source/evidence/formula identities are retained without claiming their
/// existence or truth. Queries do not evaluate formulas or replace unknown inputs with defaults.
///
/// ```compile_fail
/// fn remove(inputs: &mut sc_core::value::CanonicalInputs) {
///     inputs.lengths.clear();
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct CanonicalInputs {
    lengths: Vec<LengthDeclaration>,
    scalars: Vec<ScalarInputDeclaration>,
}
impl fmt::Debug for CanonicalInputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CanonicalInputs")
            .field("length_count", &self.lengths.len())
            .field("scalar_count", &self.scalars.len())
            .finish_non_exhaustive()
    }
}
impl CanonicalInputs {
    /// Own exact validated records without conversion, sorting, numeric reads or inferred sources.
    ///
    /// Empty drafts are legal. Uniqueness covers owned ids across both vectors; referenced ids are
    /// not mistaken for declarations owned here. Lengths are checked first, then general scalars,
    /// with each original order retained in duplicate diagnostics and shared views.
    ///
    /// # Errors
    /// Returns the first [`CanonicalInputsError::DuplicateIdentity`], including identical duplicates.
    pub fn new(
        lengths: Vec<LengthDeclaration>,
        scalars: Vec<ScalarInputDeclaration>,
    ) -> Result<Self, CanonicalInputsError> {
        let mut seen = BTreeMap::new();
        let positions = lengths
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                (declaration.id(), CanonicalInputPosition::Length { index })
            })
            .chain(scalars.iter().enumerate().map(|(index, declaration)| {
                (declaration.id(), CanonicalInputPosition::Scalar { index })
            }));
        for (identity, position) in positions {
            if let Some(first) = seen.insert(identity, position) {
                return Err(CanonicalInputsError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate: position,
                });
            }
        }
        Ok(Self { lengths, scalars })
    }

    /// Return a detached copy with one existing dedicated length replaced at its original position.
    /// The replacement's identity selects that original record; this never appends, renames, sorts or
    /// changes general Length-kind scalars. All source/state/provenance bytes remain exactly supplied.
    /// Original inputs stay unchanged. This validates no measurement binding, source truth, numeric
    /// evaluation, command authority or publication; those require the actual containing context.
    /// # Errors
    /// Returns MissingLength for an absent dedicated identity, including one owned only as a scalar.
    pub fn with_length_declaration(
        &self,
        replacement: LengthDeclaration,
    ) -> Result<Self, CanonicalInputsError> {
        let mut candidate = self.clone();
        let target = candidate
            .lengths
            .iter_mut()
            .find(|declaration| declaration.id() == replacement.id())
            .ok_or(CanonicalInputsError::MissingLength {
                declaration: replacement.id(),
            })?;
        *target = replacement;
        Ok(candidate)
    }

    // Exhaustive borrowed view for Design resource accounting. New owned fields must extend the
    // accounting contract rather than silently disappearing behind the existing public getters.
    pub(crate) fn resource_collections(&self) -> (&[LengthDeclaration], &[ScalarInputDeclaration]) {
        let Self { lengths, scalars } = self;
        (lengths, scalars)
    }

    /// Exact dedicated length declarations in authored order.
    #[must_use]
    pub fn lengths(&self) -> &[LengthDeclaration] {
        &self.lengths
    }
    /// Exact general-scalar declarations in authored order, without collapsing Length-kind records.
    #[must_use]
    pub fn scalars(&self) -> &[ScalarInputDeclaration] {
        &self.scalars
    }
    /// Borrow the actual dedicated length record; a general Length-kind scalar does not match.
    #[must_use]
    pub fn length(&self, identity: EntityId) -> Option<&LengthDeclaration> {
        self.lengths
            .iter()
            .find(|declaration| declaration.id() == identity)
    }
    /// Borrow the actual general-scalar record; a dedicated length does not become this representation.
    #[must_use]
    pub fn scalar(&self, identity: EntityId) -> Option<&ScalarInputDeclaration> {
        self.scalars
            .iter()
            .find(|declaration| declaration.id() == identity)
    }
}

/// Pinned canonical target representation; a general Length scalar never becomes a dedicated length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalInputTarget {
    /// Existing dedicated length declaration.
    Length(EntityId),
    /// Existing general scalar declaration with an explicit expected kind.
    Scalar {
        /// Stable target identity.
        declaration: EntityId,
        /// Kind expected by this metadata binding, including when no authored value exists.
        kind: ScalarInputKind,
    },
}
