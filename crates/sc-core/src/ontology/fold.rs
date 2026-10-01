//! Distinct tuck/pleat intent with shared structural validation; executed folding remains G2/G3.
use super::range::range_is_owned;
use super::{
    DirectedRange, DirectedRangeResolution, EntityId, GeometricValidation, IdentityLedger,
    IntakeAmount, IntakeValidation, Piece, ProfileBindingValidation, RangeResolution,
};
use core::fmt;
use sc_units::Length;
use std::collections::BTreeMap;

/// Editable input shared by tucks and pleats; the validated object preserves its distinct kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldDefinition {
    /// Stable construction identity.
    pub id: EntityId,
    /// Authored intake source, without cached symbolic values/states.
    pub intake: IntakeAmount,
    /// Nonempty authored list of directed fold-line intervals.
    pub fold_lines: Vec<DirectedRange>,
    /// Explicit directed geometry carrying folding direction.
    pub direction: DirectedRange,
    /// Required closing operation; recipe/Design validates its existence, kind and dependencies.
    pub closing_operation: EntityId,
}
/// Directed field role named by validation and evidence queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FoldReferenceRole {
    /// Fold line's index in the authored semantic list, not a geometry/tessellation identity.
    FoldLine(usize),
    /// Authored folding-direction reference.
    Direction,
}
/// Shared structural refusal; physical shape/count rules require G2/G3 closing operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FoldError {
    /// Authored intake cannot be negative.
    NegativeIntake(Length),
    /// At least one fold-line interval is required.
    NoFoldLines,
    /// Identical held intervals cannot silently represent two fold lines.
    DuplicateFoldLine {
        /// Earlier line index.
        first: usize,
        /// Later duplicate index.
        second: usize,
    },
    /// Some positive interval or endpoint needs repair/choice.
    UnresolvedReference {
        /// Affected semantic field.
        role: FoldReferenceRole,
        /// Raw evidence, without an implicit endpoint choice.
        evidence: Box<RangeResolution>,
    },
    /// Some current interval belongs to another Piece.
    OutsidePiece {
        /// Affected semantic field.
        role: FoldReferenceRole,
        /// Named owner.
        piece: EntityId,
    },
}
impl fmt::Display for FoldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeIntake(value) => {
                write!(f, "negative fold intake {} µm", value.as_micrometres())
            }
            Self::NoFoldLines => f.write_str("fold construction requires a fold-line interval"),
            Self::DuplicateFoldLine { first, second } => {
                write!(f, "fold line {second} repeats held interval {first}")
            }
            Self::UnresolvedReference { role, .. } => {
                write!(f, "fold {role:?} needs interval/endpoint repair or choice")
            }
            Self::OutsidePiece { role, piece } => {
                write!(f, "fold {role:?} includes geometry outside piece {piece}")
            }
        }
    }
}
impl std::error::Error for FoldError {}
#[derive(Clone, Debug, PartialEq, Eq)]
struct FoldContent {
    piece: EntityId,
    definition: FoldDefinition,
}
impl FoldContent {
    fn new(
        definition: FoldDefinition,
        piece: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<Self, FoldError> {
        if let IntakeAmount::Explicit { value, .. } = definition.intake {
            if value.as_micrometres() < 0 {
                return Err(FoldError::NegativeIntake(value));
            }
        }
        if definition.fold_lines.is_empty() {
            return Err(FoldError::NoFoldLines);
        }
        let mut seen = BTreeMap::new();
        for (index, line) in definition.fold_lines.iter().enumerate() {
            let range = line.range;
            if let Some(first) = seen.insert((range.edge(), range.from(), range.to()), index) {
                return Err(FoldError::DuplicateFoldLine {
                    first,
                    second: index,
                });
            }
        }
        for (role, reference) in references(&definition) {
            let evidence = ledger.resolve_range(reference.range);
            if !evidence.has_full_coverage()
                || evidence.start().resolved().is_none()
                || evidence.end().resolved().is_none()
            {
                return Err(FoldError::UnresolvedReference {
                    role,
                    evidence: Box::new(evidence),
                });
            }
            if !range_is_owned(&evidence, piece, ledger) {
                return Err(FoldError::OutsidePiece {
                    role,
                    piece: piece.id(),
                });
            }
        }
        Ok(Self {
            piece: piece.id(),
            definition,
        })
    }
}
/// Immutable tuck, distinct from a pleat even with identical structural input.
/// ```compile_fail
/// fn change_tuck(tuck: &mut sc_core::ontology::Tuck) {
///     tuck.0.definition.fold_lines.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tuck(FoldContent);
/// Immutable pleat; fold shape/count and conserved intake remain physical obligations.
/// ```compile_fail
/// fn change_pleat(pleat: &mut sc_core::ontology::Pleat) {
///     pleat.0.definition.fold_lines.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pleat(FoldContent);

// Keep the two semantic types distinct while sharing their identical immutable structural interface.
macro_rules! fold_interface {
    ($kind:ident) => {
        impl $kind {
            /// Validate owned structural intent without executing the named closing operation.
            /// # Errors
            /// Returns [`FoldError`] for negative explicit intake, missing/duplicate fold lines or
            /// invalid ranges. Symbolic parameter and operation validation remain registry obligations.
            pub fn new(
                definition: FoldDefinition,
                piece: &Piece,
                ledger: &IdentityLedger,
            ) -> Result<Self, FoldError> {
                FoldContent::new(definition, piece, ledger).map(Self)
            }
            /// Stable construction identity.
            #[must_use]
            pub const fn id(&self) -> EntityId {
                self.0.definition.id
            }
            /// Named source Piece.
            #[must_use]
            pub const fn piece(&self) -> EntityId {
                self.0.piece
            }
            /// Shared authored content, exposed without mutation.
            #[must_use]
            pub const fn definition(&self) -> &FoldDefinition {
                &self.0.definition
            }
            /// Declared intake provenance, without physical conservation claims.
            #[must_use]
            pub const fn intake(&self) -> IntakeAmount {
                self.0.definition.intake
            }
            /// Fold lines in authored order, followed by the direction reference.
            pub fn references(
                &self,
            ) -> impl Iterator<Item = (FoldReferenceRole, DirectedRange)> + '_ {
                references(&self.0.definition)
            }
            /// Current directed traversal/repair evidence, retaining raw point/interval answers.
            pub fn reference_resolutions<'a>(
                &'a self,
                ledger: &'a IdentityLedger,
            ) -> impl Iterator<Item = (FoldReferenceRole, DirectedRangeResolution)> + 'a {
                self.references()
                    .map(move |(role, reference)| (role, reference.resolve(ledger)))
            }
            /// Actual fold shape and geometry remain unproved.
            #[must_use]
            pub const fn geometric_validation(&self) -> GeometricValidation {
                GeometricValidation::DeferredToG2
            }
            /// Removed boundary length has not been compared with the declared intake.
            #[must_use]
            pub const fn intake_validation(&self) -> IntakeValidation {
                IntakeValidation::DeferredToG2AndG3
            }
            /// A profile intake binding stays unresolved; None means no profile intake field.
            #[must_use]
            pub const fn profile_binding_validation(&self) -> Option<ProfileBindingValidation> {
                match self.0.definition.intake {
                    IntakeAmount::Profile(_) => Some(ProfileBindingValidation::DeferredToG4),
                    _ => None,
                }
            }
        }
    };
}
fold_interface!(Tuck);
fold_interface!(Pleat);
fn references(
    definition: &FoldDefinition,
) -> impl Iterator<Item = (FoldReferenceRole, DirectedRange)> + '_ {
    definition
        .fold_lines
        .iter()
        .copied()
        .enumerate()
        .map(|(index, line)| (FoldReferenceRole::FoldLine(index), line))
        .chain(std::iter::once((
            FoldReferenceRole::Direction,
            definition.direction,
        )))
}
