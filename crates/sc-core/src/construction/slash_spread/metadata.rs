//! Borrow canonical source metadata;explicit fold association is not geometric fold proof.
use super::{
    RequiredSlashSpreadBoundary, SlashSpreadContourLocation, SlashSpreadEdge, SlashSpreadEdgeOrigin,
};
use crate::ontology::{
    CuttingSide, EdgeRef, EntityId, LabelText, LocalTag, MaterialAssignment, Mirroring, Piece,
    PrintedLabel,
};
use core::fmt;

/// Explicit preservation of the source's cut-on-fold state and attribution of its result fold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadFoldIntent {
    /// Source is not cut on fold;this does not silently remove an existing fold.
    NoFold,
    /// Keep the original fold identity,which must be retained on the proposed outer boundary.
    RetainOriginal,
    /// Attribute the fold to an actual outer-boundary Source candidate from the original fold.
    Derived {
        /// Persistent admitted candidate tag,not a contour ordinal.
        tag: LocalTag,
    },
}
/// Explicit source-metadata preservation and required fold/all-copy evidence sources.
/// ```compile_fail
/// let request:sc_core::construction::SlashSpreadPieceMetadataRequirement=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadPieceMetadataRequirement {
    fold: SlashSpreadFoldIntent,
    fold_correspondence: EntityId,
    all_copy_effects: EntityId,
}
impl SlashSpreadPieceMetadataRequirement {
    /// Request preservation of canonical source identity,quantity,mirroring,cutting side,material,
    /// layer and label. Fold state is preserved with an explicit result-edge association.
    /// Source names request actual fold/copy evidence;no provider existence or evidence is accepted.
    #[must_use]
    pub const fn preserve_source(
        fold: SlashSpreadFoldIntent,
        fold_correspondence: EntityId,
        all_copy_effects: EntityId,
    ) -> Self {
        Self {
            fold,
            fold_correspondence,
            all_copy_effects,
        }
    }
    /// Original authored fold treatment.
    #[must_use]
    pub const fn fold(&self) -> &SlashSpreadFoldIntent {
        &self.fold
    }
    /// Original required actual fold geometry/source correspondence evidence provider.
    #[must_use]
    pub const fn fold_correspondence(&self) -> &EntityId {
        &self.fold_correspondence
    }
    /// Original required effects on every actual physical copy/semantic/sewing consumer.
    #[must_use]
    pub const fn all_copy_effects(&self) -> &EntityId {
        &self.all_copy_effects
    }
}
/// Exact source-state or structural fold attribution refusal;no geometric fallback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadPieceMetadataRefusal {
    /// NoFold versus retained/derived intent contradicts the source's cut-on-fold state.
    FoldStatus,
    /// Source does not expose its required original fold field;no substitute is manufactured.
    SourceFold,
    /// Original fold identity is not retained on the actual proposed outer boundary.
    FoldNotRetained,
    /// Derived tag identifies no actual admitted candidate.
    Output,
    /// Candidate exists but is not used on the proposed outer boundary.
    OutputNotBoundary,
    /// Candidate is not Source provenance from the exact original fold identity.
    OutputNotFoldSource,
}
/// Original preservation request and whole boundary/source context survive every refusal.
pub struct SlashSpreadPieceMetadataError<'a> {
    requirement: &'a SlashSpreadPieceMetadataRequirement,
    boundary: &'a RequiredSlashSpreadBoundary<'a>,
    refusal: SlashSpreadPieceMetadataRefusal,
}
impl<'a> SlashSpreadPieceMetadataError<'a> {
    /// Actual original complete request.
    #[must_use]
    pub const fn requirement(&self) -> &'a SlashSpreadPieceMetadataRequirement {
        self.requirement
    }
    /// Actual whole original boundary/consumer/source/contour context.
    #[must_use]
    pub const fn boundary(&self) -> &'a RequiredSlashSpreadBoundary<'a> {
        self.boundary
    }
    /// First original-state or field-association cause.
    #[must_use]
    pub const fn refusal(&self) -> SlashSpreadPieceMetadataRefusal {
        self.refusal
    }
}
impl fmt::Debug for SlashSpreadPieceMetadataError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadPieceMetadataError")
            .field("requirement", &self.requirement)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadPieceMetadataError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slash/spread Piece metadata: {:?}", self.refusal)
    }
}
impl std::error::Error for SlashSpreadPieceMetadataError<'_> {}
impl<'s> RequiredSlashSpreadBoundary<'s> {
    /// Bind explicit canonical source-metadata preservation and structural fold attribution.
    /// Uses the actual admitted N candidates/M contour uses without allocation;their original
    /// complete bounds apply,not a whole-operation CPU/memory or currentness guarantee.
    /// # Errors
    /// Source fold-state mismatch,missing original fold,then retained-fold absence or derived
    /// candidate identity,outer-boundary membership and original-fold provenance,in that order.
    pub fn preserve_piece_metadata<'a>(
        &'a self,
        requirement: &'a SlashSpreadPieceMetadataRequirement,
    ) -> Result<BoundSlashSpreadPieceMetadata<'a>, SlashSpreadPieceMetadataError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| SlashSpreadPieceMetadataError {
            requirement,
            boundary: self,
            refusal,
        };
        let contours = self.consumers().registration_plan().sources().contours();
        let source = contours.source_piece();
        let requested_fold = !matches!(requirement.fold, SlashSpreadFoldIntent::NoFold);
        if source.definition().cut_on_fold != requested_fold {
            return Err(refuse(SlashSpreadPieceMetadataRefusal::FoldStatus));
        }
        let mut fold_output = None;
        if !matches!(requirement.fold, SlashSpreadFoldIntent::NoFold) {
            let source_fold = source
                .definition()
                .fold_edges
                .first()
                .ok_or_else(|| refuse(SlashSpreadPieceMetadataRefusal::SourceFold))?;
            match requirement.fold {
                SlashSpreadFoldIntent::RetainOriginal => {
                    if !contours.uses().any(|usage| {
                        matches!(usage.location(), SlashSpreadContourLocation::Boundary(_))
                            && usage
                                .retained()
                                .is_some_and(|edge| edge.edge == *source_fold)
                    }) {
                        return Err(refuse(SlashSpreadPieceMetadataRefusal::FoldNotRetained));
                    }
                }
                SlashSpreadFoldIntent::Derived { tag } => {
                    let output = contours
                        .edges()
                        .outputs()
                        .find(|output| output.candidate().tag() == tag)
                        .ok_or_else(|| refuse(SlashSpreadPieceMetadataRefusal::Output))?;
                    if !contours.uses().any(|usage| {
                        matches!(usage.location(), SlashSpreadContourLocation::Boundary(_))
                            && usage
                                .output()
                                .is_some_and(|output| output.candidate().tag() == tag)
                    }) {
                        return Err(refuse(SlashSpreadPieceMetadataRefusal::OutputNotBoundary));
                    }
                    if !matches!(output.candidate().origin(), SlashSpreadEdgeOrigin::Source { range, .. } if range.range.edge() == *source_fold)
                    {
                        return Err(refuse(SlashSpreadPieceMetadataRefusal::OutputNotFoldSource));
                    }
                    fold_output = Some(output);
                }
                SlashSpreadFoldIntent::NoFold => {}
            }
        }
        Ok(BoundSlashSpreadPieceMetadata {
            requirement,
            boundary: self,
            source,
            fold_output,
        })
    }
}
/// Authentic borrowed source metadata and declared result fold;not a new Piece or achieved effect.
/// ```compile_fail
/// fn forge<'a>(mut metadata:sc_core::construction::BoundSlashSpreadPieceMetadata<'a>,other:&'a sc_core::ontology::Piece) {
///     metadata.source=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundSlashSpreadPieceMetadata<'a> {
    requirement: &'a SlashSpreadPieceMetadataRequirement,
    boundary: &'a RequiredSlashSpreadBoundary<'a>,
    source: &'a Piece,
    fold_output: Option<SlashSpreadEdge<'a>>,
}
impl<'a> BoundSlashSpreadPieceMetadata<'a> {
    /// Actual original metadata/evidence request.
    #[must_use]
    pub const fn requirement(&self) -> &'a SlashSpreadPieceMetadataRequirement {
        self.requirement
    }
    /// Actual whole boundary/consumer/source/contour context.
    #[must_use]
    pub const fn boundary(&self) -> &'a RequiredSlashSpreadBoundary<'a> {
        self.boundary
    }
    /// Actual original immutable source Piece;its contours are not proposed output contours.
    #[must_use]
    pub const fn source_piece(&self) -> &'a Piece {
        self.source
    }
    /// Canonical original Piece identity,without creating another Piece or physical copy.
    #[must_use]
    pub fn piece_id(&self) -> &'a EntityId {
        &self.source.definition().id
    }
    /// Actual original authoritative total cut quantity.
    #[must_use]
    pub fn quantity(&self) -> &'a u32 {
        &self.source.definition().quantity
    }
    /// Actual original handedness/companion metadata,without synthesizing a companion.
    #[must_use]
    pub fn mirroring(&self) -> &'a Mirroring {
        &self.source.definition().mirroring
    }
    /// Actual original cut-on-fold state;result edge association is separate.
    #[must_use]
    pub fn cut_on_fold(&self) -> &'a bool {
        &self.source.definition().cut_on_fold
    }
    /// Actual original fabric cutting side.
    #[must_use]
    pub fn cutting_side(&self) -> &'a CuttingSide {
        &self.source.definition().cutting_side
    }
    /// Actual canonical material assignment,including exact unresolved reason text.
    #[must_use]
    pub fn material(&self) -> &'a MaterialAssignment {
        &self.source.definition().material
    }
    /// Actual original assembly layer;no geometric placement is inferred.
    #[must_use]
    pub fn layer_index(&self) -> &'a i32 {
        &self.source.definition().layer_index
    }
    /// Actual original authored label,without a second quantity/hand/fold authority.
    #[must_use]
    pub fn label(&self) -> &'a LabelText {
        &self.source.definition().label
    }
    /// Original label view derives cut indicators from the same canonical source cut metadata.
    #[must_use]
    pub fn printed_label(&self) -> PrintedLabel<'a> {
        self.source.printed_label()
    }
    /// Actual original fold field when present,not an inferred geometric axis.
    #[must_use]
    pub fn source_fold(&self) -> Option<&'a EdgeRef> {
        self.source.definition().fold_edges.first()
    }
    /// Authentic source-provenance output when the fold is explicitly derived.
    #[must_use]
    pub const fn fold_output(&self) -> Option<SlashSpreadEdge<'a>> {
        self.fold_output
    }
    /// Declared retained/new fold identity,without geometric fold correspondence or issuance proof.
    #[must_use]
    pub fn fold_edge(&self) -> Option<EdgeRef> {
        self.fold_output
            .map(SlashSpreadEdge::logical_output)
            .or_else(|| self.source_fold().copied())
    }
}
