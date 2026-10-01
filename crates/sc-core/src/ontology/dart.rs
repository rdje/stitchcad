//! Semantic dart intent; actual closing operations and conserved boundary length remain G2/G3.
use super::anchor::validate_anchor;
use super::range::range_is_owned;
use super::{
    AnchorError, DirectedRange, DirectedRangeResolution, EdgeAnchor, EntityId, GeometricValidation,
    IdentityLedger, Piece, ProfileBindingValidation, ProfileParameterRef, RangeResolution,
    Resolution,
};
use core::fmt;
use sc_units::Length;

/// Intake provenance without copied formula/profile values or uncertainty flags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntakeAmount {
    /// Authored nonnegative amount, including a deliberately authored zero.
    Explicit {
        /// Stable authored parameter identity.
        parameter: EntityId,
        /// Authored amount, never a fallback for unread input.
        value: Length,
    },
    /// Formula declaration expected to resolve to a nonnegative length.
    Formula(EntityId),
    /// Target-profile declaration expected to resolve to a nonnegative length.
    Profile(ProfileParameterRef),
}
/// Physical conservation is separate from carrying a declared intake.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntakeValidation {
    /// G2/G3 must execute the closure and compare removed boundary length with declared intake.
    DeferredToG2AndG3,
}
/// Editable dart input; references are semantic content rather than tessellation indices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DartDefinition {
    /// Stable construction identity.
    pub id: EntityId,
    /// Declared intake source, not an executed closure's length.
    pub intake: IntakeAmount,
    /// Interior apex on owned construction geometry, not necessarily the cut boundary.
    pub apex: EdgeAnchor,
    /// First directed leg, with its own persistent range.
    pub first_leg: DirectedRange,
    /// Second directed leg.
    pub second_leg: DirectedRange,
    /// Explicit directed geometry carrying the authored closing/folding direction.
    pub direction: DirectedRange,
    /// Operation whose implementation will close this dart; recipe validates existence/kind.
    pub closing_operation: EntityId,
}
/// Directed reference roles in deterministic validation/query order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DartReferenceRole {
    /// First leg.
    FirstLeg,
    /// Second leg.
    SecondLeg,
    /// Authored closing/folding direction.
    Direction,
}
/// Immutable structurally validated dart, without executed closing geometry.
/// ```compile_fail
/// fn move_dart(dart: &mut sc_core::ontology::Dart) {
///     dart.definition.apex.param = sc_core::ontology::Param::START;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dart {
    piece: EntityId,
    definition: DartDefinition,
}
/// Structural dart refusal; geometric leg/apex coincidence and intake conservation are later checks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DartError {
    /// Explicit intake cannot remove a negative amount.
    NegativeIntake(Length),
    /// The two legs name the same held interval, even if their authored directions differ.
    IdenticalLegs,
    /// Apex is missing, outside the Piece, or needs repair/choice at birth.
    Apex(AnchorError),
    /// Some directed reference needs interval/endpoint repair or choice.
    UnresolvedReference {
        /// Affected field.
        role: DartReferenceRole,
        /// Raw evidence; no consumer choice made implicitly.
        evidence: Box<RangeResolution>,
    },
    /// Some current positive interval is outside the named Piece.
    OutsidePiece {
        /// Affected field.
        role: DartReferenceRole,
        /// Named owner.
        piece: EntityId,
    },
}
impl fmt::Display for DartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeIntake(value) => {
                write!(f, "negative dart intake {} µm", value.as_micrometres())
            }
            Self::IdenticalLegs => f.write_str("dart legs must name distinct held intervals"),
            Self::Apex(error) => write!(f, "dart apex: {error}"),
            Self::UnresolvedReference { role, .. } => {
                write!(f, "dart {role:?} needs interval/endpoint repair or choice")
            }
            Self::OutsidePiece { role, piece } => {
                write!(f, "dart {role:?} includes geometry outside piece {piece}")
            }
        }
    }
}
impl std::error::Error for DartError {}
impl Dart {
    /// Validate born-owned geometry and explicit intake domain, without executing a closure.
    /// # Errors
    /// Returns [`DartError`] for negative authored intake, duplicate leg interval or invalid geometry
    /// references. Symbolic intake and closing operation existence/kinds remain registry obligations.
    pub fn new(
        definition: DartDefinition,
        piece: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<Self, DartError> {
        if let IntakeAmount::Explicit { value, .. } = definition.intake {
            if value.as_micrometres() < 0 {
                return Err(DartError::NegativeIntake(value));
            }
        }
        if definition.first_leg.range == definition.second_leg.range {
            return Err(DartError::IdenticalLegs);
        }
        validate_anchor(definition.apex, piece, ledger).map_err(DartError::Apex)?;
        for (role, reference) in references(&definition) {
            let evidence = ledger.resolve_range(reference.range);
            if !evidence.has_full_coverage()
                || evidence.start().resolved().is_none()
                || evidence.end().resolved().is_none()
            {
                return Err(DartError::UnresolvedReference {
                    role,
                    evidence: Box::new(evidence),
                });
            }
            if !range_is_owned(&evidence, piece, ledger) {
                return Err(DartError::OutsidePiece {
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
    /// Stable construction identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Source Piece owner.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        self.piece
    }
    /// Held authored content, exposed without mutation.
    #[must_use]
    pub const fn definition(&self) -> &DartDefinition {
        &self.definition
    }
    /// Declared intake provenance, queryable without claiming physical conservation.
    #[must_use]
    pub const fn intake(&self) -> IntakeAmount {
        self.definition.intake
    }
    /// Current apex evidence; split choices/repairs remain visible.
    #[must_use]
    pub fn apex_resolution(&self, ledger: &IdentityLedger) -> Resolution {
        ledger.resolve(self.definition.apex.edge, self.definition.apex.param)
    }
    /// Directed references in fixed first-leg/second-leg/direction order.
    pub fn references(&self) -> impl Iterator<Item = (DartReferenceRole, DirectedRange)> {
        references(&self.definition)
    }
    /// Directed post-edit evidence, retaining each raw interval/endpoint answer.
    pub fn reference_resolutions<'a>(
        &'a self,
        ledger: &'a IdentityLedger,
    ) -> impl Iterator<Item = (DartReferenceRole, DirectedRangeResolution)> + 'a {
        self.references()
            .map(move |(role, reference)| (role, reference.resolve(ledger)))
    }
    /// Straightness and geometric leg/apex coincidence remain unproved.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }
    /// Declared intake has not been compared with an executed closure's removed boundary length.
    #[must_use]
    pub const fn intake_validation(&self) -> IntakeValidation {
        IntakeValidation::DeferredToG2AndG3
    }
    /// A present profile intake binding remains unresolved; None only means no profile intake field.
    #[must_use]
    pub const fn profile_binding_validation(&self) -> Option<ProfileBindingValidation> {
        match self.definition.intake {
            IntakeAmount::Profile(_) => Some(ProfileBindingValidation::DeferredToG4),
            _ => None,
        }
    }
}
fn references(
    definition: &DartDefinition,
) -> impl Iterator<Item = (DartReferenceRole, DirectedRange)> {
    [
        (DartReferenceRole::FirstLeg, definition.first_leg),
        (DartReferenceRole::SecondLeg, definition.second_leg),
        (DartReferenceRole::Direction, definition.direction),
    ]
    .into_iter()
}
