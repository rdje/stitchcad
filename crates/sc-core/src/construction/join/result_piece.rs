//! Complete candidate Piece-field reconciliation,not geometry,consumer repair or publication authority.
use super::{BoundJoinCandidatePlan, MatchedJoinOriginalPieces};
use crate::ontology::{LabelField, Piece, PieceDefinition};
use core::fmt;

/// First actual joined Piece field differing from the explicit native request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinResultPieceField {
    /// Explicit joined Piece identity;scoped association does not issue it globally.
    Identity,
    /// Complete ordered outer output identities and traversal directions.
    Boundary,
    /// Complete ordered hole groups,output identities and traversal directions.
    Holes,
    /// Complete ordered construction occurrences,including repeats and additional requests.
    Construction,
    /// One canonical requested total,also enforced by the exact candidate copy plan.
    Quantity,
    /// Explicit mode,hand and companion;no copy-parity or name inference.
    Mirroring,
    /// Fold presence derived from explicit joined fold intent.
    CutOnFold,
    /// Exactly the explicitly selected fold output,or empty for NoFold.
    FoldEdges,
    /// Actual independently selected cutting face.
    CuttingSide,
    /// Actual independently selected material id or unresolved reason.
    Material,
    /// Actual independently selected assembly layer.
    Layer,
    /// Exact independently selected UTF-8 label field.
    Label(LabelField),
}
/// Actual whole original match/request/resource contexts and first joined-field cause survive.
pub struct JoinResultPieceError<'a> {
    originals: &'a MatchedJoinOriginalPieces<'a>,
    field: JoinResultPieceField,
}
impl<'a> JoinResultPieceError<'a> {
    /// Actual complete original-field proof and all outstanding companion/evidence requests.
    #[must_use]
    pub const fn originals(&self) -> &'a MatchedJoinOriginalPieces<'a> {
        self.originals
    }
    /// First joined-field mismatch in Piece-definition order,then label declaration order.
    #[must_use]
    pub const fn field(&self) -> JoinResultPieceField {
        self.field
    }
    /// Actual supplied candidate result,not reconstructed expected content.
    #[must_use]
    pub fn candidate_piece(&self) -> &'a Piece {
        self.originals.repairs().fields().plan().joined_piece()
    }
}
impl fmt::Debug for JoinResultPieceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinResultPieceError")
            .field("field", &self.field)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinResultPieceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join result Piece fields: {:?}", self.field)
    }
}
impl std::error::Error for JoinResultPieceError<'_> {}

fn first_field(originals: &MatchedJoinOriginalPieces<'_>) -> Option<JoinResultPieceField> {
    use JoinResultPieceField as F;
    let plan = originals.repairs().fields().plan();
    let fold = plan.correspondence().sources().results().folds().fold();
    let construction = fold
        .construction()
        .targets()
        .coverage()
        .groups()
        .sources()
        .uses();
    let contours = construction.connectivity().contours();
    let identity = contours.directions().coverage().edges().identity();
    let population = identity.roles().banks().sources().population();
    let metadata = population.metadata();
    // Exhaustive:future owned Piece fields require an explicit result reconciliation update.
    let PieceDefinition {
        id,
        boundary,
        holes,
        construction_lines,
        quantity,
        mirroring,
        cut_on_fold,
        fold_edges,
        cutting_side,
        material,
        layer_index,
        label,
    } = plan.joined_piece().definition();
    if id != identity.piece_id() {
        return Some(F::Identity);
    }
    if boundary.len() != contours.manifest().boundary().len()
        || contours.boundary().zip(boundary).any(|(expected, actual)| {
            actual.edge != expected.logical_output() || actual.direction != *expected.direction()
        })
    {
        return Some(F::Boundary);
    }
    if holes.len() != contours.hole_count()
        || contours.holes().zip(holes).any(|(expected, actual)| {
            actual.len() != expected.declared().len()
                || expected.uses().zip(actual).any(|(expected, actual)| {
                    actual.edge != expected.logical_output()
                        || actual.direction != *expected.direction()
                })
        })
    {
        return Some(F::Holes);
    }
    if construction_lines.len() != construction.use_count()
        || construction
            .uses()
            .zip(construction_lines)
            .any(|(expected, actual)| {
                actual.edge != expected.logical_output()
                    || actual.direction != *expected.direction()
            })
    {
        return Some(F::Construction);
    }
    if quantity != population.quantity() {
        return Some(F::Quantity);
    }
    if mirroring != population.mirroring() {
        return Some(F::Mirroring);
    }
    if *cut_on_fold != fold.cut_on_fold() {
        return Some(F::CutOnFold);
    }
    let matches_fold = match fold.edge() {
        None => fold_edges.is_empty(),
        Some(edge) => fold_edges.as_slice() == [edge],
    };
    if !matches_fold {
        return Some(F::FoldEdges);
    }
    if cutting_side != metadata.cutting_side() {
        return Some(F::CuttingSide);
    }
    if material != metadata.material() {
        return Some(F::Material);
    }
    if layer_index != metadata.layer_index() {
        return Some(F::Layer);
    }
    super::original_pieces::first_label_field(metadata.label(), label).map(F::Label)
}

impl<'s> MatchedJoinOriginalPieces<'s> {
    /// Match every actual joined Piece field to its actual explicit identity,population,metadata,
    /// cut contours,construction occurrences and fold intent. Complete candidate/source I/J/H/R/B,
    /// copy and proposal resources were admitted by the immutable parents before comparisons.
    /// Ordered/grouped field equality includes persistent identities,directions,repetitions and
    /// exact UTF-8/reason content. Work O(H+R+B+1),constant auxiliary storage,no allocation or new
    /// quota. Expected lengths precede traversal;earlier declaration bounds/storage stay separate.
    /// No geometry/provider/evidence/currentness/global issuance/consumer repair/atomic effect is
    /// supplied. All earlier explicit omissions,additional content and repair requests stay visible.
    /// # Errors
    /// First joined-field mismatch in Piece-definition order,then label name/size/fabric/colorway.
    /// Failure retains the actual whole original proof and actual supplied result;no accepted prefix.
    pub fn match_result_piece<'a>(
        &'a self,
    ) -> Result<MatchedJoinPieceInventory<'a>, JoinResultPieceError<'a>>
    where
        's: 'a,
    {
        if let Some(field) = first_field(self) {
            return Err(JoinResultPieceError {
                originals: self,
                field,
            });
        }
        Ok(MatchedJoinPieceInventory { originals: self })
    }
}
/// Complete actual candidate copy/Piece-field reconciliation,not full join acceptance.
/// ```compile_fail
/// fn forge<'a>(mut proof:sc_core::construction::MatchedJoinPieceInventory<'a>,other:&'a sc_core::construction::MatchedJoinOriginalPieces<'a>) {proof.originals=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::MatchedJoinOriginalPieces<'_>)->sc_core::construction::MatchedJoinPieceInventory<'static> {
///     raw.match_result_piece().unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct MatchedJoinPieceInventory<'a> {
    originals: &'a MatchedJoinOriginalPieces<'a>,
}
impl<'a> MatchedJoinPieceInventory<'a> {
    /// Actual complete original-field proof and outstanding authored consumer/evidence requirements.
    #[must_use]
    pub const fn originals(&self) -> &'a MatchedJoinOriginalPieces<'a> {
        self.originals
    }
    /// Actual complete supplied candidate inventory/copy-plan and correspondence/resource context.
    #[must_use]
    pub fn plan(&self) -> &'a BoundJoinCandidatePlan<'a> {
        self.originals.repairs().fields().plan()
    }
    /// Actual supplied result Piece whose complete fields match the explicit native request.
    #[must_use]
    pub fn result(&self) -> &'a Piece {
        self.plan().joined_piece()
    }
}
