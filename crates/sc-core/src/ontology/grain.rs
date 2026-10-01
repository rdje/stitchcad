//! Directed grain and print references; actual straightness/angles remain G2 obligations.
use super::range::range_is_owned;
use super::{
    Direction, EdgeRange, EntityId, GeometricValidation, IdentityLedger, Piece,
    ProfileBindingValidation, ProfileParameterRef, RangePortion, RangeResolution, Resolution,
};
use core::fmt;
use sc_units::Angle;

/// A positive source-frame interval traversed in an explicit authored direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectedRange {
    /// Exact held range; never rewritten automatically.
    pub range: EdgeRange,
    /// Authored traversal, independent of journal reversal.
    pub direction: Direction,
}
impl DirectedRange {
    /// Resolve all intervals and endpoints, retaining authored direction separately.
    #[must_use]
    pub fn resolve(self, ledger: &IdentityLedger) -> DirectedRangeResolution {
        DirectedRangeResolution {
            held: self,
            evidence: ledger.resolve_range(self.range),
        }
    }
}
/// One directed view of an unchanged live portion or repair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectedRangePortion<'a> {
    /// Raw journal evidence, including interval-repair payloads.
    pub portion: &'a RangePortion,
    /// Composed traversal on a live portion, or traversal known at the orphaning edit.
    pub direction: Direction,
}
/// Raw range evidence plus the authored directed traversal; no cached geometric verdict.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectedRangeResolution {
    held: DirectedRange,
    evidence: RangeResolution,
}
impl DirectedRangeResolution {
    /// Original held content.
    #[must_use]
    pub const fn held(&self) -> DirectedRange {
        self.held
    }
    /// Unmodified positive-interval and separate point-endpoint evidence.
    #[must_use]
    pub const fn evidence(&self) -> &RangeResolution {
        &self.evidence
    }
    /// Authored traversal's start point; split choices/repairs remain explicit.
    #[must_use]
    pub const fn start(&self) -> &Resolution {
        match self.held.direction {
            Direction::Original => self.evidence.start(),
            Direction::Reversed => self.evidence.end(),
        }
    }
    /// Authored traversal's end point.
    #[must_use]
    pub const fn end(&self) -> &Resolution {
        match self.held.direction {
            Direction::Original => self.evidence.end(),
            Direction::Reversed => self.evidence.start(),
        }
    }
    /// Portions in authored traversal order, with direction composed through the journal.
    pub fn portions(&self) -> impl Iterator<Item = DirectedRangePortion<'_>> {
        let mut portions = self.evidence.portions().iter();
        let authored = self.held.direction;
        std::iter::from_fn(move || {
            let portion = match authored {
                Direction::Original => portions.next()?,
                Direction::Reversed => portions.next_back()?,
            };
            let current = match portion {
                RangePortion::Resolved(part) => part.direction(),
                RangePortion::Unresolved(task) => task.affected().direction(),
            };
            let direction = match authored {
                Direction::Original => current,
                Direction::Reversed => current.reversed(),
            };
            Some(DirectedRangePortion { portion, direction })
        })
    }
}
/// An authored angular relation or a declaration to resolve later; no implicit value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrainAngle {
    /// Explicit counter-clockwise angle from the directed reference to the arrow.
    Explicit(Angle),
    /// Formula declaration expected to resolve to an angle.
    Formula(EntityId),
    /// Target-profile declaration expected to resolve to an angle.
    Profile(ProfileParameterRef),
}
/// Explicit alignment intent, distinct from a proved geometric relationship.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrainAlignment {
    /// Codirected alignment, stated explicitly rather than defaulted.
    ParallelTo(DirectedRange),
    /// Declared angle from an explicit directed reference, including bias or antiparallel placement.
    AtAngle {
        /// Reference axis in the Piece's source frame.
        reference: DirectedRange,
        /// Explicit or symbolic angle; never evaluated by this object.
        angle: GrainAngle,
    },
}
impl GrainAlignment {
    /// The held axis used by the relation.
    #[must_use]
    pub const fn reference(self) -> DirectedRange {
        match self {
            Self::ParallelTo(reference) | Self::AtAngle { reference, .. } => reference,
        }
    }
}
/// Editable grainline input; print references do not replace or constrain the grain by convention.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrainlineDefinition {
    /// Stable semantic grainline identity.
    pub id: EntityId,
    /// Directed semantic grain arrow, referencing owned geometry.
    pub arrow: DirectedRange,
    /// Explicit reference and relation to that reference.
    pub alignment: GrainAlignment,
    /// Independent stripe reference, if declared.
    pub stripe: Option<DirectedRange>,
    /// Independent plaid reference, if declared.
    pub plaid: Option<DirectedRange>,
}
/// Reference role named by validation and query results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrainReferenceRole {
    /// Semantic grain arrow.
    Arrow,
    /// Alignment reference axis.
    Alignment,
    /// Optional stripe reference.
    Stripe,
    /// Optional plaid reference.
    Plaid,
}
/// Immutable structurally validated directed grain intent.
/// ```compile_fail
/// fn reverse_grain(grain: &mut sc_core::ontology::Grainline) {
///     grain.definition.arrow.direction = sc_core::ontology::Direction::Reversed;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grainline {
    piece: EntityId,
    definition: GrainlineDefinition,
}
/// Structural refusal, independent of actual geometry/angle evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrainlineError {
    /// A born reference has missing intervals or unresolved endpoints.
    UnresolvedReference {
        /// Field carrying the reference.
        role: GrainReferenceRole,
        /// Exact range, point-choice and interval-repair evidence.
        evidence: Box<RangeResolution>,
    },
    /// Some current positive interval is outside the named Piece.
    OutsidePiece {
        /// Field carrying the reference.
        role: GrainReferenceRole,
        /// Named owner.
        piece: EntityId,
    },
}
impl fmt::Display for GrainlineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnresolvedReference { role, .. } => write!(
                f,
                "grainline {role:?} needs interval/endpoint repair or an explicit choice"
            ),
            Self::OutsidePiece { role, piece } => write!(
                f,
                "grainline {role:?} includes geometry outside piece {piece}"
            ),
        }
    }
}
impl std::error::Error for GrainlineError {}
impl Grainline {
    /// Validate complete owned references and unique endpoints; do not infer physical alignment.
    /// # Errors
    /// Returns [`GrainlineError`] for unknown/orphaned/ambiguous or foreign references. Formula/profile
    /// declarations retain their owners' validation obligations and supply no values here.
    pub fn new(
        definition: GrainlineDefinition,
        piece: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<Self, GrainlineError> {
        for (role, reference) in references(&definition) {
            let evidence = ledger.resolve_range(reference.range);
            if !evidence.has_full_coverage()
                || evidence.start().resolved().is_none()
                || evidence.end().resolved().is_none()
            {
                return Err(GrainlineError::UnresolvedReference {
                    role,
                    evidence: Box::new(evidence),
                });
            }
            if !range_is_owned(&evidence, piece, ledger) {
                return Err(GrainlineError::OutsidePiece {
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
    /// Stable grainline identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Source Piece owner.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        self.piece
    }
    /// Shared authored content; no public mutation.
    #[must_use]
    pub const fn definition(&self) -> &GrainlineDefinition {
        &self.definition
    }
    /// Reference roles and directed content in fixed arrow/alignment/stripe/plaid order.
    pub fn references(&self) -> impl Iterator<Item = (GrainReferenceRole, DirectedRange)> {
        references(&self.definition)
    }
    /// Current directed traversal/repair evidence, without changing any held reference.
    pub fn reference_resolutions<'a>(
        &'a self,
        ledger: &'a IdentityLedger,
    ) -> impl Iterator<Item = (GrainReferenceRole, DirectedRangeResolution)> + 'a {
        self.references()
            .map(move |(role, reference)| (role, reference.resolve(ledger)))
    }
    /// Actual straightness and angular relations remain unproved until the G2 geometry kernel.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }
    /// Present profile angle bindings remain unresolved; None means this definition has no such binding.
    #[must_use]
    pub const fn profile_binding_validation(&self) -> Option<ProfileBindingValidation> {
        match self.definition.alignment {
            GrainAlignment::AtAngle {
                angle: GrainAngle::Profile(_),
                ..
            } => Some(ProfileBindingValidation::DeferredToG4),
            _ => None,
        }
    }
}
fn references(
    definition: &GrainlineDefinition,
) -> impl Iterator<Item = (GrainReferenceRole, DirectedRange)> {
    [
        (GrainReferenceRole::Arrow, Some(definition.arrow)),
        (
            GrainReferenceRole::Alignment,
            Some(definition.alignment.reference()),
        ),
        (GrainReferenceRole::Stripe, definition.stripe),
        (GrainReferenceRole::Plaid, definition.plaid),
    ]
    .into_iter()
    .filter_map(|(role, reference)| reference.map(|reference| (role, reference)))
}
