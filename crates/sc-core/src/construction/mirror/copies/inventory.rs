//! Actual candidate Piece/companion metadata association,not current Design or geometric validity.
use super::AdmittedMirrorPieceResultCopies;
use crate::ontology::{
    DirectedEdge, EntityId, LabelText, MaterialAssignment, Mirroring, Piece, PieceInventory,
};
use core::fmt;

/// Explicit whole inventory and inspected source/result four-form population allowances.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPieceCopyInventoryLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceCopyInventoryLimits {
    pieces: usize,
    holes: usize,
    edges: usize,
    text_bytes: usize,
}
impl MirrorPieceCopyInventoryLimits {
    /// Bound all I inventory members,then complete H hole groups,R edge uses and UTF-8 B across
    /// original source,candidate source,declared result and candidate result. Other inventory
    /// fields/prior storage retain their own resource policy;this stage compares these four forms.
    #[must_use]
    pub const fn new(pieces: usize, holes: usize, edges: usize, text_bytes: usize) -> Self {
        Self {
            pieces,
            holes,
            edges,
            text_bytes,
        }
    }
    /// Complete candidate inventory member allowance before identity lookup.
    #[must_use]
    pub const fn pieces(self) -> usize {
        self.pieces
    }
    /// Complete inspected four-form hole-group allowance before child-length scans.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// Complete inspected edge-occurrence allowance,including construction repeats and folds.
    #[must_use]
    pub const fn edges(self) -> usize {
        self.edges
    }
    /// Complete inspected label/material-reason UTF-8 byte allowance before field comparison.
    #[must_use]
    pub const fn text_bytes(self) -> usize {
        self.text_bytes
    }
}
/// Canonical result field whose actual candidate differs from the native declaration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceCopyInventoryField {
    /// Canonical cut quantity.
    Quantity,
    /// Canonical mirroring/hand/companion declaration.
    Mirroring,
    /// Canonical fold presence.
    CutOnFold,
    /// Canonical cutting face.
    CuttingSide,
    /// Exact selected material assignment/reason.
    Material,
    /// Exact selected assembly layer.
    Layer,
    /// Exact selected authored label fields.
    Label,
    /// Complete desired outer contour order/direction/identities.
    Boundary,
    /// Complete desired hole groups/order/directions/identities.
    Holes,
    /// Complete desired ordered construction occurrences.
    Construction,
    /// Desired cut-derived fold edge identity.
    FoldEdges,
}
/// Whole inventory/resource or original source/result association refusal;no partial binding escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceCopyInventoryRefusal {
    /// Whole actual inventory exceeds policy before any member lookup.
    Pieces {
        /// Complete actual members.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Actual candidate inventory lacks the canonical original source.
    MissingSource {
        /// Original required identity.
        id: EntityId,
    },
    /// Actual candidate inventory lacks the canonical declared result.
    MissingResult {
        /// Actual declared result identity.
        id: EntityId,
    },
    /// A complete inspected H/R/B population cannot be represented.
    CountOverflow,
    /// Whole four-form hole population exceeds policy before child scans.
    Holes {
        /// Complete actual inspected hole groups.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole four-form edge occurrence population exceeds policy before field comparisons.
    Edges {
        /// Complete actual inspected occurrences.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole four-form label/material reason text exceeds policy before field comparisons.
    TextBytes {
        /// Complete actual UTF-8 bytes.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Candidate source content differs from the actual original complete Piece definition.
    SourceChanged,
    /// Actual candidate result field differs from the actual native cut/metadata/contour declaration.
    ResultField {
        /// First mismatch in canonical scalar/text/contour order.
        field: MirrorPieceCopyInventoryField,
    },
}
/// Whole actual native plan,inventory,policy and first refusal remain inspectable.
pub struct MirrorPieceCopyInventoryError<'a> {
    copies: &'a AdmittedMirrorPieceResultCopies<'a>,
    inventory: &'a PieceInventory,
    limits: &'a MirrorPieceCopyInventoryLimits,
    refusal: MirrorPieceCopyInventoryRefusal,
}
impl<'a> MirrorPieceCopyInventoryError<'a> {
    /// Actual whole source/result/identity/copy-proposal context.
    #[must_use]
    pub const fn copies(&self) -> &'a AdmittedMirrorPieceResultCopies<'a> {
        self.copies
    }
    /// Actual complete structurally pair-consistent candidate inventory.
    #[must_use]
    pub const fn inventory(&self) -> &'a PieceInventory {
        self.inventory
    }
    /// Original explicit caller-owned whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceCopyInventoryLimits {
        self.limits
    }
    /// First complete resource or actual original-field cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceCopyInventoryRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceCopyInventoryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceCopyInventoryError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceCopyInventoryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror candidate Piece inventory: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceCopyInventoryError<'_> {}
fn edge_count(
    boundary: usize,
    holes: &[Vec<DirectedEdge>],
    construction: usize,
    folds: usize,
) -> Option<usize> {
    holes.iter().try_fold(
        boundary.checked_add(construction)?.checked_add(folds)?,
        |count, hole| count.checked_add(hole.len()),
    )
}
fn text_bytes(label: &LabelText, material: &MaterialAssignment) -> Option<usize> {
    let mut count = 0usize;
    for text in [&label.name, &label.size, &label.fabric, &label.colorway] {
        count = count.checked_add(text.len())?;
    }
    if let MaterialAssignment::Unresolved(reason) = material {
        count = count.checked_add(reason.len())?;
    }
    Some(count)
}
impl<'s> AdmittedMirrorPieceResultCopies<'s> {
    /// Bind the actual structurally complete candidate inventory to source/result declarations.
    /// I precedes identity lookup;whole four-form H precedes child-length scans;whole R and UTF-8 B
    /// precede complete source and desired result comparisons. Work is O(I+H+R+B),constant auxiliary
    /// space and no allocation. Prior inventory birth/storage retains its own bounds. Its structural
    /// companion proof supplies reciprocity,opposite hands and equal quantity,not physical geometry,
    /// current ownership,global issuance or unrelated Piece-content authorization.
    /// # Errors
    /// I quota,source/result absence,H/R/B count or quota;then changed source and first result field.
    pub fn bind_piece_inventory<'a>(
        &'a self,
        inventory: &'a PieceInventory,
        limits: &'a MirrorPieceCopyInventoryLimits,
    ) -> Result<BoundMirrorPieceCopyInventory<'a>, MirrorPieceCopyInventoryError<'a>>
    where
        's: 'a,
    {
        use MirrorPieceCopyInventoryField as F;
        use MirrorPieceCopyInventoryRefusal as R;
        let refuse = |refusal| MirrorPieceCopyInventoryError {
            copies: self,
            inventory,
            limits,
            refusal,
        };
        let i = inventory.pieces().len();
        if i > limits.pieces {
            return Err(refuse(R::Pieces {
                required: i,
                allowed: limits.pieces,
            }));
        }
        let contours = self.identities().source().context().contours();
        let identity = contours.coverage().edges().identity();
        let metadata = identity.roles().metadata();
        let cut = metadata.cut();
        let original = cut.source().piece();
        let source = inventory
            .piece(original.id())
            .ok_or_else(|| refuse(R::MissingSource { id: original.id() }))?;
        let result = inventory.piece(*identity.piece_id()).ok_or_else(|| {
            refuse(R::MissingResult {
                id: *identity.piece_id(),
            })
        })?;
        let definitions = [
            original.definition(),
            source.definition(),
            result.definition(),
        ];
        let h = definitions
            .iter()
            .try_fold(contours.holes().len(), |count, definition| {
                count.checked_add(definition.holes.len())
            })
            .ok_or_else(|| refuse(R::CountOverflow))?;
        if h > limits.holes {
            return Err(refuse(R::Holes {
                required: h,
                allowed: limits.holes,
            }));
        }
        let mut r = edge_count(
            contours.boundary().len(),
            contours.holes(),
            contours.construction_lines().len(),
            contours.fold_edges().len(),
        )
        .ok_or_else(|| refuse(R::CountOverflow))?;
        for definition in definitions {
            r = r
                .checked_add(
                    edge_count(
                        definition.boundary.len(),
                        &definition.holes,
                        definition.construction_lines.len(),
                        definition.fold_edges.len(),
                    )
                    .ok_or_else(|| refuse(R::CountOverflow))?,
                )
                .ok_or_else(|| refuse(R::CountOverflow))?;
        }
        if r > limits.edges {
            return Err(refuse(R::Edges {
                required: r,
                allowed: limits.edges,
            }));
        }
        let mut b = text_bytes(metadata.label(), metadata.material())
            .ok_or_else(|| refuse(R::CountOverflow))?;
        for definition in definitions {
            b = b
                .checked_add(
                    text_bytes(&definition.label, &definition.material)
                        .ok_or_else(|| refuse(R::CountOverflow))?,
                )
                .ok_or_else(|| refuse(R::CountOverflow))?;
        }
        if b > limits.text_bytes {
            return Err(refuse(R::TextBytes {
                required: b,
                allowed: limits.text_bytes,
            }));
        }
        if source.definition() != original.definition() {
            return Err(refuse(R::SourceChanged));
        }
        let actual = result.definition();
        for (field, matches) in [
            (F::Quantity, actual.quantity == *cut.quantity()),
            (F::Mirroring, actual.mirroring == *cut.mirroring()),
            (F::CutOnFold, actual.cut_on_fold == cut.cut_on_fold()),
            (F::CuttingSide, actual.cutting_side == *cut.cutting_side()),
            (F::Material, &actual.material == metadata.material()),
            (F::Layer, actual.layer_index == *metadata.layer_index()),
            (F::Label, &actual.label == metadata.label()),
            (F::Boundary, actual.boundary == contours.boundary()),
            (F::Holes, actual.holes == contours.holes()),
            (
                F::Construction,
                actual.construction_lines == contours.construction_lines(),
            ),
            (F::FoldEdges, actual.fold_edges == contours.fold_edges()),
        ] {
            if !matches {
                return Err(refuse(R::ResultField { field }));
            }
        }
        Ok(BoundMirrorPieceCopyInventory {
            copies: self,
            inventory,
            limits,
            source,
            result,
            holes: h,
            edges: r,
            text_bytes: b,
        })
    }
}
/// Authentic actual candidate inventory/source/result association,not published or geometric evidence.
/// ```compile_fail
/// fn reparent<'a>(mut bound:sc_core::construction::BoundMirrorPieceCopyInventory<'a>,
///     other:&'a sc_core::ontology::PieceInventory) {bound.inventory=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(copies:&'a sc_core::construction::AdmittedMirrorPieceResultCopies<'a>)
///     ->sc_core::construction::BoundMirrorPieceCopyInventory<'a> {
///     use sc_core::{construction::*,ontology::PieceInventory};
///     let inventory=PieceInventory::new(vec![]).unwrap();let limits=MirrorPieceCopyInventoryLimits::new(0,0,0,0);
///     copies.bind_piece_inventory(&inventory,&limits).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundMirrorPieceCopyInventory<'a> {
    copies: &'a AdmittedMirrorPieceResultCopies<'a>,
    inventory: &'a PieceInventory,
    limits: &'a MirrorPieceCopyInventoryLimits,
    source: &'a Piece,
    result: &'a Piece,
    holes: usize,
    edges: usize,
    text_bytes: usize,
}
impl<'a> BoundMirrorPieceCopyInventory<'a> {
    /// Actual complete native result-copy proposal and original source context.
    #[must_use]
    pub const fn copies(&self) -> &'a AdmittedMirrorPieceResultCopies<'a> {
        self.copies
    }
    /// Actual whole structurally complete candidate Piece collection,in original order.
    #[must_use]
    pub const fn inventory(&self) -> &'a PieceInventory {
        self.inventory
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a MirrorPieceCopyInventoryLimits {
        self.limits
    }
    /// Actual candidate source member,whose complete fields equal the actual original source.
    #[must_use]
    pub const fn source_piece(&self) -> &'a Piece {
        self.source
    }
    /// Actual candidate result member,matching desired canonical metadata/contours.
    #[must_use]
    pub const fn result_piece(&self) -> &'a Piece {
        self.result
    }
    /// Actual source's reciprocal separate companion,when declared;no ordinal pairing.
    #[must_use]
    pub fn source_companion(&self) -> Option<&'a Piece> {
        match self.source.definition().mirroring {
            Mirroring::PairMember { companion, .. } => self.inventory.piece(companion),
            _ => None,
        }
    }
    /// Actual result's reciprocal separate companion,when declared;no physical-copy neighbour transfer.
    #[must_use]
    pub fn result_companion(&self) -> Option<&'a Piece> {
        match self.result.definition().mirroring {
            Mirroring::PairMember { companion, .. } => self.inventory.piece(companion),
            _ => None,
        }
    }
    /// Complete admitted inspected four-form hole count.
    #[must_use]
    pub const fn holes(&self) -> usize {
        self.holes
    }
    /// Complete admitted inspected four-form edge-occurrence count.
    #[must_use]
    pub const fn edges(&self) -> usize {
        self.edges
    }
    /// Complete admitted inspected four-form UTF-8 label/material reason bytes.
    #[must_use]
    pub const fn text_bytes(&self) -> usize {
        self.text_bytes
    }
}
