//! Per-edge allowance intent; target policy and bounded offset geometry remain later obligations.
use super::range::range_is_owned;
use super::{
    EdgeRange, EdgeRef, EntityId, GeometricValidation, IdentityLedger, Piece,
    ProfileBindingValidation, ProfileParameterRef, RangeResolution,
};
use core::fmt;
use sc_units::Length;

/// Width origin, retaining authored parameter identity without copying symbolic values/states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllowanceWidth {
    /// Explicit nonnegative authored width; zero is never a fallback for an unread binding.
    Explicit {
        /// Authored parameter identity; the Design validates its declaration.
        parameter: EntityId,
        /// Authored length value, not an evaluated formula/profile cache.
        value: Length,
    },
    /// Formula declaration expected to resolve to a nonnegative length.
    Formula(EntityId),
    /// Target-profile declaration expected to resolve to a nonnegative length.
    Profile(ProfileParameterRef),
}
/// Explicit corner vocabulary from ontology §4.4; no automatic join choice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CornerTreatment {
    /// Miter join.
    Miter,
    /// Slanted corner.
    Slant,
    /// Envelope corner.
    Envelope,
    /// Trimmed corner.
    Trim,
    /// Stepped corner.
    Step,
}
/// Possible resolved target-profile policies; a descriptor does not select one implicitly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllowanceInclusion {
    /// The stored/exported contour includes the allowance.
    IncludedInContour,
    /// The downstream receiver generates the allowance from the net contour.
    GeneratedDownstream,
}
/// Editable allowance input, independent of generated offset geometry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeamAllowanceDefinition {
    /// Stable descriptor identity.
    pub id: EntityId,
    /// Whole owned source edge; not an exporter-specific tessellation index.
    pub edge: EdgeRef,
    /// Explicit authored width or symbolic width declaration.
    pub width: AllowanceWidth,
    /// Authored corner treatment.
    pub corner: CornerTreatment,
    /// Logical declaration resolved per target profile, never a project-wide boolean.
    pub inclusion: ProfileParameterRef,
}
/// Immutable per-edge derived-object intent, with no generated/certified geometry.
/// ```compile_fail
/// fn change_allowance(sa: &mut sc_core::ontology::SeamAllowance) {
///     sa.definition.corner = sc_core::ontology::CornerTreatment::Trim;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeamAllowance {
    piece: EntityId,
    definition: SeamAllowanceDefinition,
}
/// Structural refusal of a descriptor; offset failure belongs to the G2 geometry engine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SeamAllowanceError {
    /// An authored width is negative; symbolic widths remain their owner's validation obligation.
    NegativeWidth(Length),
    /// Some positive interval is missing or an endpoint needs repair/choice.
    UnresolvedEdge(Box<RangeResolution>),
    /// Some current interval is outside the named Piece.
    OutsidePiece(EntityId),
}
impl fmt::Display for SeamAllowanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeWidth(value) => {
                write!(f, "negative allowance width {} µm", value.as_micrometres())
            }
            Self::UnresolvedEdge(_) => {
                f.write_str("allowance edge needs interval/endpoint repair or an explicit choice")
            }
            Self::OutsidePiece(piece) => {
                write!(f, "allowance edge includes geometry outside piece {piece}")
            }
        }
    }
}
impl std::error::Error for SeamAllowanceError {}
impl SeamAllowance {
    /// Validate complete owned edge coverage, unique endpoints and an explicit width's domain.
    /// # Errors
    /// Returns [`SeamAllowanceError`] for negative authored width or missing/ambiguous/foreign edges.
    /// Symbolic parameter existence, kinds, values and evidence remain recipe/Design/profile checks.
    pub fn new(
        definition: SeamAllowanceDefinition,
        piece: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<Self, SeamAllowanceError> {
        if let AllowanceWidth::Explicit { value, .. } = definition.width {
            if value.as_micrometres() < 0 {
                return Err(SeamAllowanceError::NegativeWidth(value));
            }
        }
        let evidence = ledger.resolve_range(EdgeRange::whole(definition.edge));
        if !evidence.has_full_coverage()
            || evidence.start().resolved().is_none()
            || evidence.end().resolved().is_none()
        {
            return Err(SeamAllowanceError::UnresolvedEdge(Box::new(evidence)));
        }
        if !range_is_owned(&evidence, piece, ledger) {
            return Err(SeamAllowanceError::OutsidePiece(piece.id()));
        }
        Ok(Self {
            piece: piece.id(),
            definition,
        })
    }
    /// Stable descriptor identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Source Piece owner.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        self.piece
    }
    /// Authored input, exposed without mutation.
    #[must_use]
    pub const fn definition(&self) -> &SeamAllowanceDefinition {
        &self.definition
    }
    /// Current whole-edge interval and endpoint evidence; does not rewrite held content.
    #[must_use]
    pub fn edge_resolution(&self, ledger: &IdentityLedger) -> RangeResolution {
        ledger.resolve_range(EdgeRange::whole(self.definition.edge))
    }
    /// Actual bounded offsets, corner construction and topology repair remain G2 obligations.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }
    /// Every descriptor holds target-profile inclusion; neither inclusion nor width is defaulted.
    #[must_use]
    pub const fn profile_binding_validation(&self) -> ProfileBindingValidation {
        ProfileBindingValidation::DeferredToG4
    }
}
