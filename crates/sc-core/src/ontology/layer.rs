//! Served layer intent; recipe owns offset dimensions and modelled lining remains outside v1 execution.
use super::range::range_is_owned;
use super::{
    DirectedRange, DirectedRangeResolution, EntityId, GeometricValidation, IdentityLedger,
    MaterialAssignment, Piece, RangeResolution,
};
use core::fmt;
use std::collections::BTreeMap;

/// Explicit link to the recipe offset operation and its owned geometric sources.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayerOffsetRelationship {
    /// Operation whose inputs own dimensions/states; recipe validates existence and offset kind.
    pub operation: EntityId,
    /// Nonempty directed source ranges in the served Piece's frame.
    pub sources: Vec<DirectedRange>,
}
/// Editable layer input, without generated contours or cached output-Piece material states.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayerDefinition {
    /// Stable semantic layer identity.
    pub id: EntityId,
    /// Piece this facing, lining or interfacing serves.
    pub served_piece: EntityId,
    /// Explicit recipe offset relationship, never inferred from line art.
    pub offset: LayerOffsetRelationship,
    /// Authored assignment or explicit unresolved reason; Design validates assigned material ids.
    pub material: MaterialAssignment,
}
/// Distinct semantic layer kinds, separate from a proof of execution support.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayerKind {
    /// Facing.
    Facing,
    /// Lining, modelled but deferred in the v1 envelope.
    Lining,
    /// Interfacing.
    Interfacing,
}
/// Typed scope refusal, distinct from structural input or geometric failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayerEnvelopeError {
    /// The v1 matrix defers lined garments to G7 rather than approximating them.
    LiningDeferred {
        /// Piece the requested lining serves.
        served_piece: EntityId,
    },
}
impl LayerEnvelopeError {
    /// Canonical feature-matrix diagnostic token.
    #[must_use]
    pub const fn diagnostic(self) -> &'static str {
        match self {
            Self::LiningDeferred { .. } => "env_lining",
        }
    }
    /// Gate that owns the supported-envelope limitation; not a promise of physical support.
    #[must_use]
    pub const fn proving_gate(self) -> &'static str {
        match self {
            Self::LiningDeferred { .. } => "G7",
        }
    }
}
impl fmt::Display for LayerEnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LiningDeferred { served_piece } => write!(
                f,
                "env_lining: lining for piece {served_piece} is deferred to G7"
            ),
        }
    }
}
impl std::error::Error for LayerEnvelopeError {}
/// Structural layer refusal; operation values and actual offset geometry remain later checks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LayerError {
    /// The supplied Piece is not the authored served target.
    WrongServedPiece {
        /// Held target identity.
        held: EntityId,
        /// Provided Piece identity.
        provided: EntityId,
    },
    /// The offset relationship requires owned geometric sources.
    NoOffsetSources,
    /// A source interval is repeated even if its authored direction differs.
    DuplicateOffsetSource {
        /// Earlier list position.
        first: usize,
        /// Duplicate position.
        second: usize,
    },
    /// An unresolved material requires a nonblank reason.
    UnexplainedMaterial,
    /// Some source interval or endpoint needs repair/choice.
    UnresolvedSource {
        /// Position in the semantic input list, not a geometry identity.
        index: usize,
        /// Exact raw evidence.
        evidence: Box<RangeResolution>,
    },
    /// Some current interval is outside the served Piece.
    SourceOutsidePiece {
        /// Position in the semantic input list.
        index: usize,
        /// Named served Piece.
        piece: EntityId,
    },
}
impl fmt::Display for LayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongServedPiece { held, provided } => write!(
                f,
                "layer served piece {held} differs from provided {provided}"
            ),
            Self::NoOffsetSources => {
                f.write_str("layer offset relationship requires source ranges")
            }
            Self::DuplicateOffsetSource { first, second } => write!(
                f,
                "layer offset source {second} repeats held interval {first}"
            ),
            Self::UnexplainedMaterial => {
                f.write_str("layer unresolved material needs a nonblank reason")
            }
            Self::UnresolvedSource { index, .. } => write!(
                f,
                "layer offset source {index} needs interval/endpoint repair or choice"
            ),
            Self::SourceOutsidePiece { index, piece } => write!(
                f,
                "layer offset source {index} includes geometry outside served piece {piece}"
            ),
        }
    }
}
impl std::error::Error for LayerError {}
#[derive(Clone, Debug, PartialEq, Eq)]
struct LayerContent {
    definition: LayerDefinition,
}
impl LayerContent {
    fn new(
        definition: LayerDefinition,
        served: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<Self, LayerError> {
        if definition.served_piece != served.id() {
            return Err(LayerError::WrongServedPiece {
                held: definition.served_piece,
                provided: served.id(),
            });
        }
        if !definition.material.has_required_reason() {
            return Err(LayerError::UnexplainedMaterial);
        }
        if definition.offset.sources.is_empty() {
            return Err(LayerError::NoOffsetSources);
        }
        let mut seen = BTreeMap::new();
        for (index, source) in definition.offset.sources.iter().enumerate() {
            let range = source.range;
            if let Some(first) = seen.insert((range.edge(), range.from(), range.to()), index) {
                return Err(LayerError::DuplicateOffsetSource {
                    first,
                    second: index,
                });
            }
            let evidence = ledger.resolve_range(range);
            if !evidence.has_full_coverage()
                || evidence.start().resolved().is_none()
                || evidence.end().resolved().is_none()
            {
                return Err(LayerError::UnresolvedSource {
                    index,
                    evidence: Box::new(evidence),
                });
            }
            if !range_is_owned(&evidence, served, ledger) {
                return Err(LayerError::SourceOutsidePiece {
                    index,
                    piece: served.id(),
                });
            }
        }
        Ok(Self { definition })
    }
}
/// Immutable facing intent, without generated offset geometry.
/// ```compile_fail
/// fn change_facing(facing: &mut sc_core::ontology::Facing) {
///     facing.0.definition.offset.sources.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Facing(LayerContent);
/// Immutable modelled lining; requested v1 execution returns the named envelope refusal.
/// ```compile_fail
/// fn change_lining(lining: &mut sc_core::ontology::Lining) {
///     lining.0.definition.offset.sources.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lining(LayerContent);
/// Immutable interfacing intent, without generated offset geometry.
/// ```compile_fail
/// fn change_interfacing(interfacing: &mut sc_core::ontology::Interfacing) {
///     interfacing.0.definition.offset.sources.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interfacing(LayerContent);

// Share structural content/interface without collapsing the three semantic recipe kinds.
macro_rules! layer_interface {
    ($name:ident, $kind:ident) => {
        impl $name {
            /// Validate modelled structural content; this does not execute or authorize construction.
            /// # Errors
            /// Returns [`LayerError`] for wrong Piece, missing/duplicate sources, invalid material
            /// reason or unresolved/foreign intervals. Recipe/material registries remain obligations.
            pub fn new(
                definition: LayerDefinition,
                served: &Piece,
                ledger: &IdentityLedger,
            ) -> Result<Self, LayerError> {
                LayerContent::new(definition, served, ledger).map(Self)
            }
            /// Stable semantic identity.
            #[must_use]
            pub const fn id(&self) -> EntityId {
                self.0.definition.id
            }
            /// Named served Piece.
            #[must_use]
            pub const fn served_piece(&self) -> EntityId {
                self.0.definition.served_piece
            }
            /// Distinct semantic kind.
            #[must_use]
            pub const fn kind(&self) -> LayerKind {
                LayerKind::$kind
            }
            /// Shared authored content, exposed without mutation.
            #[must_use]
            pub const fn definition(&self) -> &LayerDefinition {
                &self.0.definition
            }
            /// Current directed evidence in authored source-list order, without rewriting input.
            pub fn source_resolutions<'a>(
                &'a self,
                ledger: &'a IdentityLedger,
            ) -> impl Iterator<Item = (usize, DirectedRangeResolution)> + 'a {
                self.0
                    .definition
                    .offset
                    .sources
                    .iter()
                    .copied()
                    .enumerate()
                    .map(move |(index, source)| (index, source.resolve(ledger)))
            }
            /// Check the planned v1 execution envelope, separate from geometry/release certification.
            /// # Errors
            /// Modelled lining returns [`LayerEnvelopeError`] with env_lining, served Piece and G7.
            pub const fn require_in_scope(&self) -> Result<(), LayerEnvelopeError> {
                match self.kind() {
                    LayerKind::Lining => Err(LayerEnvelopeError::LiningDeferred {
                        served_piece: self.served_piece(),
                    }),
                    LayerKind::Facing | LayerKind::Interfacing => Ok(()),
                }
            }
            /// Actual offsets, generated shape and physical relationships remain unproved.
            #[must_use]
            pub const fn geometric_validation(&self) -> GeometricValidation {
                GeometricValidation::DeferredToG2
            }
        }
    };
}
layer_interface!(Facing, Facing);
layer_interface!(Lining, Lining);
layer_interface!(Interfacing, Interfacing);
