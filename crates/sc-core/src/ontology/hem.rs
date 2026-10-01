//! Hem intent with current Facing composition checks; physical folding remains G2/G3.
use super::range::range_is_owned;
use super::{
    EdgeRange, EdgeRef, EntityId, Facing, GeometricValidation, IdentityLedger, LayerError, Piece,
    ProfileBindingValidation, ProfileParameterRef, RangeResolution,
};
use core::fmt;
use sc_units::Length;

/// Authored depth origin, without cached symbolic values or uncertainty states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HemDepth {
    /// Explicit nonnegative depth; authored zero is permitted and never an unread-input fallback.
    Explicit {
        /// Stable authored parameter identity.
        parameter: EntityId,
        /// Authored depth.
        value: Length,
    },
    /// Formula declaration expected to resolve to a nonnegative length.
    Formula(EntityId),
    /// Target-profile declaration expected to resolve to a nonnegative length.
    Profile(ProfileParameterRef),
}
/// Required fold-type origin; its logical declaration owns domain and uncertainty state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HemFoldType {
    /// Recipe/Design declaration; G1 does not invent a physical fold vocabulary.
    Declaration(EntityId),
    /// Target-profile declaration, without a default physical choice.
    Profile(ProfileParameterRef),
}
/// Explicit finishing method, separate from fold-type and allowance corner treatment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HemMethod {
    /// Turned finish.
    Turned,
    /// Faced finish bound to one stable existing Facing identity.
    Faced {
        /// Facing serving the same Piece.
        facing: EntityId,
    },
}
/// Editable Hem input, without executed folding geometry or a duplicated Facing definition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HemDefinition {
    /// Stable construction identity.
    pub id: EntityId,
    /// Whole Piece-owned edge this hem finishes.
    pub edge: EdgeRef,
    /// Required depth provenance.
    pub depth: HemDepth,
    /// Required fold-type binding.
    pub fold_type: HemFoldType,
    /// Explicit turned/faced intent.
    pub method: HemMethod,
}
/// Immutable structural Hem, without physical fold certification.
/// ```compile_fail
/// fn change_hem(hem: &mut sc_core::ontology::Hem) {
///     hem.definition.method = sc_core::ontology::HemMethod::Turned;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hem {
    piece: EntityId,
    definition: HemDefinition,
}
/// Typed Hem refusal, including stable composition targets and current source evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HemError {
    /// Negative authored depth is invalid; symbolic domains remain registry obligations.
    NegativeDepth(Length),
    /// Finish edge has missing intervals or ambiguous/unresolved endpoints.
    UnresolvedEdge(Box<RangeResolution>),
    /// Some current finish interval belongs outside the named Piece.
    OutsidePiece(EntityId),
    /// A current-target query was given a different owner Piece.
    WrongPiece {
        /// Held owner.
        held: EntityId,
        /// Provided owner.
        provided: EntityId,
    },
    /// The faced method's held target is absent; no substitute is generated.
    MissingFacing(EntityId),
    /// A provided Facing is not the original target identity.
    WrongFacing {
        /// Held target.
        held: EntityId,
        /// Provided target.
        provided: EntityId,
    },
    /// The target no longer serves this Hem's Piece.
    FacingOutsidePiece {
        /// Target identity.
        facing: EntityId,
        /// Required served owner.
        expected: EntityId,
        /// Actual served owner.
        served: EntityId,
    },
    /// The target's current source references require repair or fail structural validation.
    InvalidFacing {
        /// Held target identity.
        facing: EntityId,
        /// Layer refusal with current source evidence.
        error: LayerError,
    },
}
impl fmt::Display for HemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeDepth(value) => {
                write!(f, "negative hem depth {} µm", value.as_micrometres())
            }
            Self::UnresolvedEdge(_) => {
                f.write_str("hem finish edge needs interval/endpoint repair or choice")
            }
            Self::OutsidePiece(piece) => {
                write!(f, "hem finish edge includes geometry outside piece {piece}")
            }
            Self::WrongPiece { held, provided } => {
                write!(f, "hem owner {held} differs from provided {provided}")
            }
            Self::MissingFacing(facing) => write!(f, "hem facing {facing} is missing"),
            Self::WrongFacing { held, provided } => {
                write!(f, "hem facing {held} differs from provided {provided}")
            }
            Self::FacingOutsidePiece {
                facing,
                expected,
                served,
            } => write!(
                f,
                "hem facing {facing} serves {served}, expected {expected}"
            ),
            Self::InvalidFacing { facing, error } => write!(f, "hem facing {facing}: {error}"),
        }
    }
}
impl std::error::Error for HemError {}
impl Hem {
    /// Validate a whole owned finish edge, depth domain and current faced composition target.
    /// # Errors
    /// Returns [`HemError`] for invalid references, negative authored depth or missing/wrong/invalid
    /// Facing. Symbolic domains and every declaration/operation/material registry remain obligations.
    pub fn new(
        definition: HemDefinition,
        piece: &Piece,
        ledger: &IdentityLedger,
        facing: Option<&Facing>,
    ) -> Result<Self, HemError> {
        if let HemDepth::Explicit { value, .. } = definition.depth {
            if value.as_micrometres() < 0 {
                return Err(HemError::NegativeDepth(value));
            }
        }
        let evidence = ledger.resolve_range(EdgeRange::whole(definition.edge));
        if !evidence.has_full_coverage()
            || evidence.start().resolved().is_none()
            || evidence.end().resolved().is_none()
        {
            return Err(HemError::UnresolvedEdge(Box::new(evidence)));
        }
        if !range_is_owned(&evidence, piece, ledger) {
            return Err(HemError::OutsidePiece(piece.id()));
        }
        let hem = Self {
            piece: piece.id(),
            definition,
        };
        hem.facing(piece, ledger, facing)?;
        Ok(hem)
    }
    /// Stable construction identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Original owner Piece.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        self.piece
    }
    /// Authored definition, exposed without mutation.
    #[must_use]
    pub const fn definition(&self) -> &HemDefinition {
        &self.definition
    }
    /// Current whole-edge evidence, without rewriting input or picking endpoint choices.
    #[must_use]
    pub fn edge_resolution(&self, ledger: &IdentityLedger) -> RangeResolution {
        ledger.resolve_range(EdgeRange::whole(self.definition.edge))
    }
    /// Borrow the original current Facing after checking its identity, served Piece and sources.
    /// Turned methods return None; this query does not certify the Hem's edge or physical geometry.
    /// # Errors
    /// Returns [`HemError`] for wrong owner, missing/replaced/reassigned target or invalid sources.
    pub fn facing<'a>(
        &self,
        piece: &Piece,
        ledger: &IdentityLedger,
        facing: Option<&'a Facing>,
    ) -> Result<Option<&'a Facing>, HemError> {
        if piece.id() != self.piece {
            return Err(HemError::WrongPiece {
                held: self.piece,
                provided: piece.id(),
            });
        }
        let HemMethod::Faced { facing: held } = self.definition.method else {
            return Ok(None);
        };
        let current = facing.ok_or(HemError::MissingFacing(held))?;
        if current.id() != held {
            return Err(HemError::WrongFacing {
                held,
                provided: current.id(),
            });
        }
        if current.served_piece() != self.piece {
            return Err(HemError::FacingOutsidePiece {
                facing: held,
                expected: self.piece,
                served: current.served_piece(),
            });
        }
        Facing::new(current.definition().clone(), piece, ledger).map_err(|error| {
            HemError::InvalidFacing {
                facing: held,
                error,
            }
        })?;
        Ok(Some(current))
    }
    /// Actual edge/fold geometry remains unproved; no depth or fold is executed by this descriptor.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }
    /// Present target-profile fields remain unresolved; None means only no profile field is present.
    #[must_use]
    pub const fn profile_binding_validation(&self) -> Option<ProfileBindingValidation> {
        match (self.definition.depth, self.definition.fold_type) {
            (HemDepth::Profile(_), _) | (_, HemFoldType::Profile(_)) => {
                Some(ProfileBindingValidation::DeferredToG4)
            }
            _ => None,
        }
    }
}
