//! Gather intent binds one physical sewing side and borrows its existing ease declaration.
use super::range::range_is_owned;
use super::{
    CutCopy, CutPlan, DeclaredEase, DirectedRange, DirectedRangeResolution, EaseAmount, EntityId,
    GeometricValidation, IdentityLedger, IntakeValidation, Piece, ProfileBindingValidation,
    RangeResolution, SeamSide, SeamSideId, SeamSpan, SewingGraph,
};
use core::fmt;
use sc_units::Length;

/// Editable gather input; attachment/intake/allocation are provided by the named sewing span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GatherDefinition {
    /// Stable construction identity.
    pub id: EntityId,
    /// Sewing graph identity.
    pub graph: EntityId,
    /// Span identity inside that graph.
    pub span: EntityId,
    /// Side whose material is gathered.
    pub side: SeamSideId,
    /// Held physical-copy binding; changing a span's copy must not retarget this gather silently.
    pub copy: EntityId,
    /// Explicit directed geometry carrying gathering direction.
    pub direction: DirectedRange,
    /// Required closing operation; recipe/Design validates its existence, kind and dependencies.
    pub closing_operation: EntityId,
}
/// Borrowed canonical intake/allocation source, retaining the signed A-minus-B convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GatherIntakeSource<'a> {
    /// Existing span declaration, never an independently authored/cached gather distribution.
    pub declaration: &'a DeclaredEase,
    /// Gathering A interprets A-minus-B directly; gathering B interprets its negation.
    pub gathered_side: SeamSideId,
}
/// Structural reference role named by constructor refusals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GatherReferenceRole {
    /// Selected sewing-side material interval.
    Attachment,
    /// Authored directed gathering reference.
    Direction,
}
/// Immutable gather binding, without executed shortening or certified physical intake.
/// ```compile_fail
/// fn retarget_gather(gather: &mut sc_core::ontology::Gather) {
///     gather.definition.side = sc_core::ontology::SeamSideId::B;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gather {
    piece: EntityId,
    definition: GatherDefinition,
}
/// Structural/target refusal; symbolic sign, walking and conserved intake remain G2/G3/G4.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GatherError {
    /// The provided graph is not the held graph.
    WrongGraph {
        /// Held identity.
        held: EntityId,
        /// Provided identity.
        provided: EntityId,
    },
    /// The original named span is absent; another span is not a replacement by convention.
    MissingSpan(EntityId),
    /// The span side now names different physical material.
    CopyBindingChanged {
        /// Held physical-copy id.
        held: EntityId,
        /// Span's current physical-copy id.
        current: EntityId,
    },
    /// The held physical copy is absent from the supplied plan.
    MissingCopy(EntityId),
    /// The copy belongs to another Piece.
    CopyOutsidePiece {
        /// Held physical copy.
        copy: EntityId,
        /// Expected source Piece.
        expected: EntityId,
        /// Actual source Piece.
        actual: EntityId,
    },
    /// An explicit signed differential makes the selected side shorter than its partner.
    IncompatibleEaseSign {
        /// Selected gathered side.
        side: SeamSideId,
        /// Authored A-minus-B differential.
        differential: Length,
    },
    /// A positive interval or endpoint needs repair/choice.
    UnresolvedReference {
        /// Affected field.
        role: GatherReferenceRole,
        /// Raw evidence.
        evidence: Box<RangeResolution>,
    },
    /// A current interval is outside the named Piece.
    OutsidePiece {
        /// Affected field.
        role: GatherReferenceRole,
        /// Named owner.
        piece: EntityId,
    },
}
impl fmt::Display for GatherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongGraph { held, provided } => {
                write!(f, "gather graph {held} does not match provided {provided}")
            }
            Self::MissingSpan(id) => write!(f, "gather span {id} is missing"),
            Self::CopyBindingChanged { held, current } => {
                write!(f, "gather copy {held} differs from span copy {current}")
            }
            Self::MissingCopy(id) => write!(f, "gather physical copy {id} is missing"),
            Self::CopyOutsidePiece {
                copy,
                expected,
                actual,
            } => write!(
                f,
                "gather copy {copy} belongs to {actual}, expected {expected}"
            ),
            Self::IncompatibleEaseSign { side, differential } => write!(
                f,
                "gather {side:?} has incompatible A-minus-B ease {} µm",
                differential.as_micrometres()
            ),
            Self::UnresolvedReference { role, .. } => write!(
                f,
                "gather {role:?} needs interval/endpoint repair or choice"
            ),
            Self::OutsidePiece { role, piece } => {
                write!(f, "gather {role:?} includes geometry outside piece {piece}")
            }
        }
    }
}
impl std::error::Error for GatherError {}
impl Gather {
    /// Validate held graph/span/copy binding, explicit ease sign and born-owned geometry.
    /// # Errors
    /// Returns [`GatherError`] for missing/changed targets, a wrong Piece/ease sign or invalid ranges.
    /// Symbolic values and operation existence/kinds remain recipe/Design/profile obligations.
    pub fn new(
        definition: GatherDefinition,
        piece: &Piece,
        plan: &CutPlan,
        graph: &SewingGraph,
        ledger: &IdentityLedger,
    ) -> Result<Self, GatherError> {
        let span = held_span(&definition, graph)?;
        held_copy(definition.copy, piece.id(), plan)?;
        let attachment = side(span, definition.side);
        for (role, range) in [
            (GatherReferenceRole::Attachment, attachment.range),
            (GatherReferenceRole::Direction, definition.direction.range),
        ] {
            let evidence = ledger.resolve_range(range);
            if !evidence.has_full_coverage()
                || evidence.start().resolved().is_none()
                || evidence.end().resolved().is_none()
            {
                return Err(GatherError::UnresolvedReference {
                    role,
                    evidence: Box::new(evidence),
                });
            }
            if !range_is_owned(&evidence, piece, ledger) {
                return Err(GatherError::OutsidePiece {
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
    /// Authored content, exposed without mutation.
    #[must_use]
    pub const fn definition(&self) -> &GatherDefinition {
        &self.definition
    }
    /// Original physical copy, never reassigned to a replacement.
    /// # Errors
    /// Returns missing-copy or wrong-Piece evidence from the supplied current plan.
    pub fn copy<'a>(&self, plan: &'a CutPlan) -> Result<&'a CutCopy, GatherError> {
        held_copy(self.definition.copy, self.piece, plan)
    }
    /// Borrow canonical intake/allocation from the current named span, without copying values/states.
    /// # Errors
    /// Returns missing/changed graph/span/copy binding or incompatible explicit ease sign.
    pub fn intake_source<'a>(
        &self,
        graph: &'a SewingGraph,
    ) -> Result<GatherIntakeSource<'a>, GatherError> {
        let span = held_span(&self.definition, graph)?;
        Ok(GatherIntakeSource {
            declaration: &span.definition().ease,
            gathered_side: self.definition.side,
        })
    }
    /// Current attachment range evidence; target binding is checked, raw repairs remain visible.
    /// # Errors
    /// Returns missing/changed graph/span/copy binding or incompatible explicit ease sign.
    pub fn attachment_resolution(
        &self,
        graph: &SewingGraph,
        ledger: &IdentityLedger,
    ) -> Result<RangeResolution, GatherError> {
        let span = held_span(&self.definition, graph)?;
        Ok(span.resolve_side(self.definition.side, ledger))
    }
    /// Current direction evidence, retaining ordered portions and raw point choices/repairs.
    #[must_use]
    pub fn direction_resolution(&self, ledger: &IdentityLedger) -> DirectedRangeResolution {
        self.definition.direction.resolve(ledger)
    }
    /// Actual gathering geometry has not been constructed or certified.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }
    /// Walking/resolved sign/shortening and conserved intake remain G2/G3 obligations.
    #[must_use]
    pub const fn intake_validation(&self) -> IntakeValidation {
        IntakeValidation::DeferredToG2AndG3
    }
    /// A present profile ease source remains unresolved; absence means no such field.
    /// # Errors
    /// Returns missing/changed graph/span/copy binding or incompatible explicit ease sign.
    pub fn profile_binding_validation(
        &self,
        graph: &SewingGraph,
    ) -> Result<Option<ProfileBindingValidation>, GatherError> {
        let source = self.intake_source(graph)?;
        Ok(match source.declaration.amount {
            EaseAmount::Profile(_) => Some(ProfileBindingValidation::DeferredToG4),
            _ => None,
        })
    }
}
fn held_copy(copy: EntityId, piece: EntityId, plan: &CutPlan) -> Result<&CutCopy, GatherError> {
    let value = plan.copy(copy).ok_or(GatherError::MissingCopy(copy))?;
    if value.piece() != piece {
        return Err(GatherError::CopyOutsidePiece {
            copy,
            expected: piece,
            actual: value.piece(),
        });
    }
    Ok(value)
}
fn side(span: &SeamSpan, selected: SeamSideId) -> SeamSide {
    match selected {
        SeamSideId::A => span.definition().a,
        SeamSideId::B => span.definition().b,
    }
}
fn held_span<'a>(
    definition: &GatherDefinition,
    graph: &'a SewingGraph,
) -> Result<&'a SeamSpan, GatherError> {
    if graph.id() != definition.graph {
        return Err(GatherError::WrongGraph {
            held: definition.graph,
            provided: graph.id(),
        });
    }
    let span = graph
        .spans()
        .iter()
        .find(|span| span.id() == definition.span)
        .ok_or(GatherError::MissingSpan(definition.span))?;
    let current = side(span, definition.side).copy;
    if current != definition.copy {
        return Err(GatherError::CopyBindingChanged {
            held: definition.copy,
            current,
        });
    }
    if let EaseAmount::Explicit(value) = span.definition().ease.amount {
        let wrong_sign = match definition.side {
            SeamSideId::A => value.as_micrometres() < 0,
            SeamSideId::B => value.as_micrometres() > 0,
        };
        if wrong_sign {
            return Err(GatherError::IncompatibleEaseSign {
                side: definition.side,
                differential: value,
            });
        }
    }
    Ok(span)
}
