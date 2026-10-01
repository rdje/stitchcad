//! Pocket placement/composition intent; opening execution and physical construction remain G3.
use super::anchor::{validate_anchor, validate_current_anchor};
use super::range::range_is_owned;
use super::{
    AnchorError, CutPlan, DirectedRange, DirectedRangeResolution, EdgeAnchor, EntityId,
    GeometricValidation, IdentityLedger, Piece, ProfileBindingValidation, ProfileParameterRef,
    RangeResolution, Resolution,
};
use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

/// Explicit physical-copy/source-Piece binding, without duplicated pattern geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PocketPieceRef {
    /// Stable physical cut-copy identity.
    pub copy: EntityId,
    /// Required source pattern Piece identity; reassignment cannot silently satisfy this reference.
    pub piece: EntityId,
}
/// Required logical opening-type binding; its owner retains the domain, selection and state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PocketOpening {
    /// Recipe/Design declaration; no physical opening vocabulary is invented at G1.
    Declaration(EntityId),
    /// Target-profile declaration, with no fallback opening type.
    Profile(ProfileParameterRef),
}
/// Carrying a logical opening binding does not resolve its construction or supported scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PocketOpeningValidation {
    /// G3 must resolve/execute the opening recipe and refuse unsupported scope explicitly.
    DeferredToG3,
}
/// Editable pocket intent, with required placement and explicit composition references.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PocketDefinition {
    /// Stable semantic pocket identity.
    pub id: EntityId,
    /// Physical copy/source Piece this pocket serves.
    pub served: PocketPieceRef,
    /// Held attachment position in the served Piece's source frame.
    pub position: EdgeAnchor,
    /// Directed orientation reference in that source frame, independent of copy reflection.
    pub orientation: DirectedRange,
    /// Required logical opening-type declaration.
    pub opening: PocketOpening,
    /// Nonempty explicit component copies/Pieces, each copy appearing at most once.
    pub components: Vec<PocketPieceRef>,
}
/// Immutable structural pocket, without generated opening or composition geometry.
/// ```compile_fail
/// fn change_pocket(pocket: &mut sc_core::ontology::Pocket) {
///     pocket.definition.components.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pocket {
    definition: PocketDefinition,
}
/// Structural Pocket refusal, retaining stable targets and current geometric evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PocketError {
    /// Composition requires component references.
    NoComponents,
    /// A physical component copy is repeated, even with a different claimed source Piece.
    DuplicateComponent(EntityId),
    /// Supplied current Piece identities are ambiguous.
    DuplicatePiece(EntityId),
    /// A required physical copy is missing from the current plan.
    MissingCopy(EntityId),
    /// A current physical copy names another source Piece.
    WrongCopyPiece {
        /// Held copy identity.
        copy: EntityId,
        /// Authored expected source.
        expected: EntityId,
        /// Actual current source.
        actual: EntityId,
    },
    /// The explicit source Piece is absent from the supplied registry.
    MissingPiece {
        /// Physical copy whose source is missing.
        copy: EntityId,
        /// Required source Piece.
        piece: EntityId,
    },
    /// A component query names no authored component copy.
    MissingComponent(EntityId),
    /// Position needs owned unique resolution.
    Position(Box<AnchorError>),
    /// Orientation includes missing intervals or ambiguous/unresolved endpoints.
    UnresolvedOrientation(Box<RangeResolution>),
    /// Some current orientation interval lies outside the served Piece.
    OrientationOutsidePiece(EntityId),
}
impl fmt::Display for PocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoComponents => f.write_str("pocket requires component copies/Pieces"),
            Self::DuplicateComponent(id) => write!(f, "pocket component copy {id} is repeated"),
            Self::DuplicatePiece(id) => write!(f, "duplicate pocket context piece {id}"),
            Self::MissingCopy(id) => write!(f, "pocket physical copy {id} is missing"),
            Self::WrongCopyPiece {
                copy,
                expected,
                actual,
            } => write!(f, "pocket copy {copy} names {actual}, expected {expected}"),
            Self::MissingPiece { copy, piece } => {
                write!(f, "pocket copy {copy} source piece {piece} is missing")
            }
            Self::MissingComponent(id) => write!(f, "pocket component copy {id} is absent"),
            Self::Position(error) => write!(f, "pocket position: {error}"),
            Self::UnresolvedOrientation(_) => {
                f.write_str("pocket orientation needs interval/endpoint repair or choice")
            }
            Self::OrientationOutsidePiece(piece) => write!(
                f,
                "pocket orientation includes geometry outside served piece {piece}"
            ),
        }
    }
}
impl std::error::Error for PocketError {}
impl Pocket {
    /// Validate explicit composition targets, live owned birth position and whole orientation.
    /// # Errors
    /// Returns [`PocketError`] for empty/duplicate components, ambiguous/missing/mismatched targets
    /// or invalid placement references. Component contour repairs, geometry and opening scope remain
    /// Design/G2/G3 checks; this constructor does not infer support from a logical opening id.
    pub fn new(
        definition: PocketDefinition,
        plan: &CutPlan,
        pieces: &[Piece],
        ledger: &IdentityLedger,
    ) -> Result<Self, PocketError> {
        if definition.components.is_empty() {
            return Err(PocketError::NoComponents);
        }
        let mut seen = BTreeSet::new();
        for component in &definition.components {
            if !seen.insert(component.copy) {
                return Err(PocketError::DuplicateComponent(component.copy));
            }
        }
        let pocket = Self { definition };
        let served = pocket.validate_targets(plan, pieces)?;
        validate_anchor(pocket.definition.position, served, ledger)
            .map_err(|error| PocketError::Position(Box::new(error)))?;
        pocket.validate_orientation(served, ledger)?;
        Ok(pocket)
    }
    /// Stable pocket identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Authored input, exposed without mutation.
    #[must_use]
    pub const fn definition(&self) -> &PocketDefinition {
        &self.definition
    }
    /// Validate current targets and placement references, including historical anchor resolution.
    /// # Errors
    /// Returns [`PocketError`] for ambiguous/missing/reassigned targets or placement repairs.
    /// Component contour correctness and resolved opening scope still require their own checks.
    pub fn validate_current(
        &self,
        plan: &CutPlan,
        pieces: &[Piece],
        ledger: &IdentityLedger,
    ) -> Result<(), PocketError> {
        let served = self.validate_targets(plan, pieces)?;
        validate_current_anchor(self.definition.position, served, ledger)
            .map_err(|error| PocketError::Position(Box::new(error)))?;
        self.validate_orientation(served, ledger)
    }
    /// Borrow current component Piece metadata after checking its explicit copy/source binding.
    /// This validates only that target; callers must inspect contour repairs and all other contexts.
    /// # Errors
    /// Returns [`PocketError`] for absent component or ambiguous/missing/reassigned target context.
    pub fn component<'a>(
        &self,
        copy: EntityId,
        plan: &CutPlan,
        pieces: &'a [Piece],
    ) -> Result<&'a Piece, PocketError> {
        let reference = self
            .definition
            .components
            .iter()
            .find(|held| held.copy == copy)
            .ok_or(PocketError::MissingComponent(copy))?;
        let registry = piece_registry(pieces)?;
        resolve_piece(*reference, plan, &registry)
    }
    /// Raw current position evidence, independent of composition or scope checks.
    #[must_use]
    pub fn position_resolution(&self, ledger: &IdentityLedger) -> Resolution {
        ledger.resolve(
            self.definition.position.edge,
            self.definition.position.param,
        )
    }
    /// Directed current orientation evidence, retaining input order, endpoint choices and repairs.
    #[must_use]
    pub fn orientation_resolution(&self, ledger: &IdentityLedger) -> DirectedRangeResolution {
        self.definition.orientation.resolve(ledger)
    }
    /// Actual placement, opening/contour relationships and reflected geometry remain unproved.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }
    /// Logical opening type does not prove resolved or supported physical construction.
    #[must_use]
    pub const fn opening_validation(&self) -> PocketOpeningValidation {
        PocketOpeningValidation::DeferredToG3
    }
    /// Present target-profile opening remains unresolved; None only means no profile field authored.
    #[must_use]
    pub const fn profile_binding_validation(&self) -> Option<ProfileBindingValidation> {
        match self.definition.opening {
            PocketOpening::Profile(_) => Some(ProfileBindingValidation::DeferredToG4),
            PocketOpening::Declaration(_) => None,
        }
    }
    fn validate_targets<'a>(
        &self,
        plan: &CutPlan,
        pieces: &'a [Piece],
    ) -> Result<&'a Piece, PocketError> {
        let registry = piece_registry(pieces)?;
        let served = resolve_piece(self.definition.served, plan, &registry)?;
        for component in &self.definition.components {
            resolve_piece(*component, plan, &registry)?;
        }
        Ok(served)
    }
    fn validate_orientation(
        &self,
        served: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<(), PocketError> {
        let evidence = ledger.resolve_range(self.definition.orientation.range);
        if !evidence.has_full_coverage()
            || evidence.start().resolved().is_none()
            || evidence.end().resolved().is_none()
        {
            return Err(PocketError::UnresolvedOrientation(Box::new(evidence)));
        }
        if !range_is_owned(&evidence, served, ledger) {
            return Err(PocketError::OrientationOutsidePiece(served.id()));
        }
        Ok(())
    }
}
fn piece_registry(pieces: &[Piece]) -> Result<BTreeMap<EntityId, &Piece>, PocketError> {
    let mut registry = BTreeMap::new();
    for piece in pieces {
        if registry.insert(piece.id(), piece).is_some() {
            return Err(PocketError::DuplicatePiece(piece.id()));
        }
    }
    Ok(registry)
}
fn resolve_piece<'a>(
    reference: PocketPieceRef,
    plan: &CutPlan,
    registry: &BTreeMap<EntityId, &'a Piece>,
) -> Result<&'a Piece, PocketError> {
    let copy = plan
        .copy(reference.copy)
        .ok_or(PocketError::MissingCopy(reference.copy))?;
    if copy.piece() != reference.piece {
        return Err(PocketError::WrongCopyPiece {
            copy: reference.copy,
            expected: reference.piece,
            actual: copy.piece(),
        });
    }
    registry
        .get(&reference.piece)
        .copied()
        .ok_or(PocketError::MissingPiece {
            copy: reference.copy,
            piece: reference.piece,
        })
}
