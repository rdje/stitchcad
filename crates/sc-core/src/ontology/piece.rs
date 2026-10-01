//! Structurally validated pattern pieces (ontology §4.1).
//!
//! A loop is a cyclic sequence of directed edge identities, without a repeated closing edge.
//! Endpoint coincidence, winding, simplicity and hole containment require G2 geometry. This module
//! deliberately cannot manufacture a geometrically valid state. Objects expose no public mutation;
//! command-bus edits will construct a replacement through the same validation boundary.

use core::fmt;
use std::collections::BTreeSet;

use super::{
    Direction, EdgeRange, EdgeRef, EntityId, IdentityLedger, Param, RangeResolution, Resolution,
};

/// The geometric obligation carried by G1 ontology objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometricValidation {
    /// G2 must check the object's physical geometry: closure/winding, angles, offsets and error bounds.
    DeferredToG2,
}

/// An edge traversed in an authored direction, independent of subsequent topology edits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectedEdge {
    /// The persistent edge identity.
    pub edge: EdgeRef,
    /// Traversal relative to the edge's original direction.
    pub direction: Direction,
}

/// Location of a boundary or cutout loop in a piece definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopLocation {
    /// The outer boundary.
    Boundary,
    /// A cutout, numbered in the definition's authored order.
    Hole(usize),
}

/// How cut pieces are mirrored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mirroring {
    /// Each copy retains its authored handedness.
    Single,
    /// Copies are cut in left/right pairs; the total quantity must be even.
    MirroredPairs,
    /// A separately identified left or right member, paired with a distinct companion piece.
    /// Its quantity counts copies of this member, not both members together.
    PairMember {
        /// The handedness printed on this member's label.
        handedness: Handedness,
        /// The companion's stable piece identity; the design validates reciprocity and quantity.
        companion: EntityId,
    },
}

/// The handedness of a separately identified mirrored-pair member.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handedness {
    /// Left member.
    Left,
    /// Right member.
    Right,
}

/// The face presented when cutting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CuttingSide {
    /// Fabric face up.
    Face,
    /// Fabric wrong side up.
    WrongSide,
}

/// Material assignment with no silent default for an unresolved material.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaterialAssignment {
    /// Identity of the assigned material; the design resolves that identity.
    Assigned(EntityId),
    /// No material has been assigned; the reason is visible and must be nonblank.
    Unresolved(String),
}

impl MaterialAssignment {
    // Shared local invariant only: assigned material existence/kind/state belongs to Design.
    pub(crate) fn has_required_reason(&self) -> bool {
        !matches!(self, Self::Unresolved(reason) if reason.trim().is_empty())
    }
}

/// The authored text required on a printed piece label.
///
/// Quantity, pair L/R and fold indication are derived from the piece's cut plan, never entered twice.
/// Fabric and colorway are required print text, not a substitute for material assignment evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabelText {
    /// Pattern-piece name.
    pub name: String,
    /// Printable size label; the eventual size-set reference remains a separate design concern.
    pub size: String,
    /// Printable fabric description (including an explicit unresolved description if needed).
    pub fabric: String,
    /// Printable colorway (an explicit "undetermined" is permitted).
    pub colorway: String,
}

/// Label field named by an incomplete-label diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelField {
    /// Pattern-piece name.
    Name,
    /// Size label.
    Size,
    /// Fabric text.
    Fabric,
    /// Colorway text.
    Colorway,
}

/// Complete label view, using the cut plan's authoritative quantity, mirroring and fold state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrintedLabel<'a> {
    /// Authored label text.
    pub text: &'a LabelText,
    /// Physical copies represented by this piece definition. `MirroredPairs` includes both hands;
    /// `PairMember` counts this member's copies only.
    pub quantity: u32,
    /// Whether the label must indicate pair L/R.
    pub mirroring: Mirroring,
    /// Whether the label must indicate cut on fold.
    pub cut_on_fold: bool,
}

/// Unvalidated piece input. Creating this value does not create a [`Piece`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PieceDefinition {
    /// Stable identity supplied by the caller's injected id generator.
    pub id: EntityId,
    /// Cyclic outer boundary; omit a repeated terminal edge.
    pub boundary: Vec<DirectedEdge>,
    /// Cyclic cutout loops, each ordered independently.
    pub holes: Vec<Vec<DirectedEdge>>,
    /// Non-cutting construction geometry, in authored order.
    pub construction_lines: Vec<DirectedEdge>,
    /// Total cut quantity; at least one.
    pub quantity: u32,
    /// Single handedness or mirrored pairs.
    pub mirroring: Mirroring,
    /// Whether the boundary includes exactly one declared fold edge.
    pub cut_on_fold: bool,
    /// Fold-edge declarations; exactly one on the boundary when cutting on fold, otherwise none.
    pub fold_edges: Vec<EdgeRef>,
    /// Which fabric side is presented when cutting.
    pub cutting_side: CuttingSide,
    /// Material identity or explicit unresolved state.
    pub material: MaterialAssignment,
    /// Assembly layering order; it does not imply geometric placement.
    pub layer_index: i32,
    /// Required printed label text.
    pub label: LabelText,
}

/// An immutable, structurally valid piece whose geometric obligations are still deferred.
///
/// The validated content is private. A command must validate a replacement rather than change it:
///
/// ```compile_fail
/// fn erase_quantity(piece: &mut sc_core::ontology::Piece) {
///     piece.definition.quantity = 0;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    definition: PieceDefinition,
}

/// A piece-construction refusal naming the failed structural invariant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PieceError {
    /// A boundary or hole has no edges.
    EmptyLoop {
        /// The offending loop.
        location: LoopLocation,
    },
    /// An edge is repeated in one loop or shared by distinct cut loops.
    RepeatedCutEdge {
        /// The loop at which repetition was found.
        location: LoopLocation,
        /// The repeated identity.
        edge: EdgeRef,
    },
    /// An authored reference is not live at construction.
    MissingEdge {
        /// The absent or retired identity.
        edge: EdgeRef,
    },
    /// No copies would be cut.
    ZeroQuantity,
    /// A total cut quantity cannot form complete left/right pairs.
    UnpairedQuantity {
        /// The odd cut quantity.
        quantity: u32,
    },
    /// A separately identified pair member cannot be its own companion.
    SelfCompanion {
        /// The piece incorrectly named as both members.
        piece: EntityId,
    },
    /// Fold declarations do not agree with the cut plan.
    FoldCount {
        /// Required declaration count (zero or one).
        expected: usize,
        /// Actual declaration count.
        actual: usize,
    },
    /// The fold edge is not an outer-boundary edge.
    FoldNotOnBoundary {
        /// The offending identity.
        edge: EdgeRef,
    },
    /// A printed text field is empty or whitespace-only.
    IncompleteLabel {
        /// The missing field.
        field: LabelField,
    },
    /// An unresolved material lacks an explanation.
    UnexplainedMaterial,
}

impl fmt::Display for PieceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLoop { location } => {
                write!(f, "{location:?} must be a non-empty cyclic loop")
            }
            Self::RepeatedCutEdge { location, edge } => {
                write!(f, "{location:?} repeats cut edge {edge}")
            }
            Self::MissingEdge { edge } => {
                write!(f, "piece reference {edge} must exist at construction")
            }
            Self::ZeroQuantity => f.write_str("piece cut quantity must be at least one"),
            Self::UnpairedQuantity { quantity } => {
                write!(
                    f,
                    "mirrored pairs require an even total cut quantity, got {quantity}"
                )
            }
            Self::SelfCompanion { piece } => write!(
                f,
                "mirrored-pair member {piece} must name a distinct companion"
            ),
            Self::FoldCount { expected, actual } => {
                write!(f, "cut plan requires {expected} fold edges, got {actual}")
            }
            Self::FoldNotOnBoundary { edge } => {
                write!(f, "fold edge {edge} must lie on the outer boundary")
            }
            Self::IncompleteLabel { field } => {
                write!(f, "printed label field {field:?} must be nonblank")
            }
            Self::UnexplainedMaterial => {
                f.write_str("unresolved material must carry a nonblank reason")
            }
        }
    }
}

impl std::error::Error for PieceError {}

impl Piece {
    /// Validates structural content against the current topology, without modifying the ledger.
    ///
    /// # Errors
    /// Returns a [`PieceError`] naming the first invalid structural invariant. Geometric checks are
    /// always [`GeometricValidation::DeferredToG2`], including for a structurally valid loop.
    pub fn new(definition: PieceDefinition, ledger: &IdentityLedger) -> Result<Self, PieceError> {
        let mut cut_edges = BTreeSet::new();
        validate_loop(
            &definition.boundary,
            LoopLocation::Boundary,
            ledger,
            &mut cut_edges,
        )?;
        for (index, hole) in definition.holes.iter().enumerate() {
            validate_loop(hole, LoopLocation::Hole(index), ledger, &mut cut_edges)?;
        }
        for line in &definition.construction_lines {
            require_live(line.edge, ledger)?;
        }
        if definition.quantity == 0 {
            return Err(PieceError::ZeroQuantity);
        }
        if definition.mirroring == Mirroring::MirroredPairs
            && !definition.quantity.is_multiple_of(2)
        {
            return Err(PieceError::UnpairedQuantity {
                quantity: definition.quantity,
            });
        }
        if matches!(definition.mirroring, Mirroring::PairMember { companion, .. } if companion == definition.id)
        {
            return Err(PieceError::SelfCompanion {
                piece: definition.id,
            });
        }
        let expected = usize::from(definition.cut_on_fold);
        if definition.fold_edges.len() != expected {
            return Err(PieceError::FoldCount {
                expected,
                actual: definition.fold_edges.len(),
            });
        }
        for edge in &definition.fold_edges {
            if !definition.boundary.iter().any(|item| item.edge == *edge) {
                return Err(PieceError::FoldNotOnBoundary { edge: *edge });
            }
        }
        for (field, text) in [
            (LabelField::Name, &definition.label.name),
            (LabelField::Size, &definition.label.size),
            (LabelField::Fabric, &definition.label.fabric),
            (LabelField::Colorway, &definition.label.colorway),
        ] {
            if text.trim().is_empty() {
                return Err(PieceError::IncompleteLabel { field });
            }
        }
        if !definition.material.has_required_reason() {
            return Err(PieceError::UnexplainedMaterial);
        }
        Ok(Self { definition })
    }

    /// Authored content, available by shared reference only.
    #[must_use]
    pub const fn definition(&self) -> &PieceDefinition {
        &self.definition
    }

    /// Stable piece identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }

    /// G1 cannot certify geometric validity.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }

    /// A complete print view with cut information derived from the authoritative cut plan.
    #[must_use]
    pub fn printed_label(&self) -> PrintedLabel<'_> {
        PrintedLabel {
            text: &self.definition.label,
            quantity: self.definition.quantity,
            mirroring: self.definition.mirroring,
            cut_on_fold: self.definition.cut_on_fold,
        }
    }

    /// All held edge identities, including holes and construction lines, in authored order.
    ///
    /// Edits never rewrite these identities. Consumers resolve them through the ledger on demand.
    pub fn edges(&self) -> impl Iterator<Item = DirectedEdge> + '_ {
        self.definition
            .boundary
            .iter()
            .chain(self.definition.holes.iter().flatten())
            .chain(self.definition.construction_lines.iter())
            .copied()
    }

    /// Full-edge interval evidence for every held edge, including lost interior fragments.
    ///
    /// Range portions are ordered in the stored edge's forward frame; the accompanying directed edge
    /// tells a contour consumer whether to reverse that traversal. This evidence does not certify
    /// geometric continuity or release readiness.
    pub fn range_resolutions<'a>(
        &'a self,
        ledger: &'a IdentityLedger,
    ) -> impl Iterator<Item = (DirectedEdge, RangeResolution)> + 'a {
        self.edges()
            .map(|item| (item, ledger.resolve_range(EdgeRange::whole(item.edge))))
    }

    /// Both authored traversal endpoints for each edge, suitable for reference registration.
    ///
    /// The command bus must register these under [`Self::id`] as part of its atomic creation step;
    /// this constructor does not mutate a separate ledger. This is an endpoint-reference inventory,
    /// not geometric reconstruction of a contour (split fragments require the G2 kernel). Endpoint
    /// registrations alone cannot certify full-edge integrity after deletion of an interior fragment;
    /// release validation must also evaluate the complete recipe and its geometry.
    pub fn endpoint_references(&self) -> impl Iterator<Item = (EdgeRef, Param)> + '_ {
        self.edges().flat_map(|item| {
            let ends = match item.direction {
                Direction::Original => [Param::START, Param::END],
                Direction::Reversed => [Param::END, Param::START],
            };
            ends.map(move |param| (item.edge, param))
        })
    }

    /// Resolve every held endpoint, exposing ambiguity or repair tasks without rewriting the piece.
    pub fn endpoint_resolutions<'a>(
        &'a self,
        ledger: &'a IdentityLedger,
    ) -> impl Iterator<Item = (EdgeRef, Param, Resolution)> + 'a {
        self.endpoint_references()
            .map(|(edge, param)| (edge, param, ledger.resolve(edge, param)))
    }
}

fn require_live(edge: EdgeRef, ledger: &IdentityLedger) -> Result<(), PieceError> {
    if ledger.is_live(edge) {
        Ok(())
    } else {
        Err(PieceError::MissingEdge { edge })
    }
}

fn validate_loop(
    edges: &[DirectedEdge],
    location: LoopLocation,
    ledger: &IdentityLedger,
    seen: &mut BTreeSet<EdgeRef>,
) -> Result<(), PieceError> {
    if edges.is_empty() {
        return Err(PieceError::EmptyLoop { location });
    }
    for item in edges {
        require_live(item.edge, ledger)?;
        if !seen.insert(item.edge) {
            return Err(PieceError::RepeatedCutEdge {
                location,
                edge: item.edge,
            });
        }
    }
    Ok(())
}
