//! Exact original Piece preservation under previously admitted whole field/companion contexts.
use super::{JoinCompanionRepair, RequiredJoinCompanionRepairs};
use crate::ontology::{EntityId, LabelField, LabelText, Piece, PieceDefinition};
use core::fmt;

/// First original Piece field differing from its permitted candidate value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinOriginalPieceField {
    /// Original stable Piece identity.
    Identity,
    /// Exact ordered outer edge identities and directions.
    Boundary,
    /// Exact ordered hole groups,edges and directions.
    Holes,
    /// Exact ordered construction edge identities and directions,including repeats.
    Construction,
    /// Quantity derived from the actual retained-original copy actions.
    Quantity,
    /// Exact original mode,hand and companion,except an admitted explicit repair.
    Mirroring,
    /// Original fold-presence declaration.
    CutOnFold,
    /// Exact separate ordered fold edge declarations.
    FoldEdges,
    /// Original cutting face.
    CuttingSide,
    /// Exact original assigned identity or unresolved reason.
    Material,
    /// Original assembly layer.
    Layer,
    /// Exact original UTF-8 label field,without normalization.
    Label(LabelField),
}
/// First original-order closed-parent invariant or actual candidate field cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinOriginalPieceRefusal {
    /// Internal fault:closed original retained-copy count is absent or not representable.
    CountInvariant {
        /// Actual original Piece exposing the fault.
        piece: EntityId,
    },
    /// Internal fault:closed candidate membership disagrees with zero/positive retained copies.
    MembershipInvariant {
        /// Actual original Piece exposing the fault.
        piece: EntityId,
    },
    /// Actual candidate field differs from its original or expressly permitted derived value.
    Field {
        /// Actual original Piece.
        piece: EntityId,
        /// First differing field in Piece-definition order,then label declaration order.
        field: JoinOriginalPieceField,
    },
}
impl JoinOriginalPieceRefusal {
    fn piece(self) -> EntityId {
        match self {
            Self::CountInvariant { piece }
            | Self::MembershipInvariant { piece }
            | Self::Field { piece, .. } => piece,
        }
    }
}
/// Actual whole resource/repair parents and first original/candidate field cause;no accepted prefix.
pub struct JoinOriginalPieceError<'a> {
    repairs: &'a RequiredJoinCompanionRepairs<'a>,
    refusal: JoinOriginalPieceRefusal,
}
impl<'a> JoinOriginalPieceError<'a> {
    /// Actual complete original repair/field/copy-plan contexts,including outstanding evidence ids.
    #[must_use]
    pub const fn repairs(&self) -> &'a RequiredJoinCompanionRepairs<'a> {
        self.repairs
    }
    /// First original-order internal invariant or actual field cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinOriginalPieceRefusal {
        self.refusal
    }
    /// Actual original Piece for this cause;None would itself expose a closed-parent fault.
    #[must_use]
    pub fn original_piece(&self) -> Option<&'a Piece> {
        self.repairs
            .fields()
            .plan()
            .correspondence()
            .sources()
            .results()
            .source_piece(self.refusal.piece())
    }
    /// Actual candidate if present;fully consumed originals have none.
    #[must_use]
    pub fn candidate_piece(&self) -> Option<&'a Piece> {
        self.repairs.fields().plan().piece(self.refusal.piece())
    }
}
impl fmt::Debug for JoinOriginalPieceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinOriginalPieceError")
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinOriginalPieceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join original Piece preservation: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinOriginalPieceError<'_> {}

fn first_label_field(original: &LabelText, candidate: &LabelText) -> Option<LabelField> {
    let LabelText {
        name,
        size,
        fabric,
        colorway,
    } = original; // Exhaustive:future owned label fields require an explicit comparison.
    [
        (LabelField::Name, name, &candidate.name),
        (LabelField::Size, size, &candidate.size),
        (LabelField::Fabric, fabric, &candidate.fabric),
        (LabelField::Colorway, colorway, &candidate.colorway),
    ]
    .into_iter()
    .find_map(|(field, expected, actual)| (expected != actual).then_some(field))
}
fn first_field(
    repairs: &RequiredJoinCompanionRepairs<'_>,
    original: &PieceDefinition,
    candidate: &PieceDefinition,
    retained: u32,
) -> Option<JoinOriginalPieceField> {
    use JoinOriginalPieceField as F;
    // Exhaustive:future owned Piece fields cannot silently escape original preservation.
    let PieceDefinition {
        id,
        boundary,
        holes,
        construction_lines,
        quantity: _,
        mirroring,
        cut_on_fold,
        fold_edges,
        cutting_side,
        material,
        layer_index,
        label,
    } = original;
    if *id != candidate.id {
        return Some(F::Identity);
    }
    if *boundary != candidate.boundary {
        return Some(F::Boundary);
    }
    if *holes != candidate.holes {
        return Some(F::Holes);
    }
    if *construction_lines != candidate.construction_lines {
        return Some(F::Construction);
    }
    if retained != candidate.quantity {
        return Some(F::Quantity);
    }
    // An authentic repair was checked against these same immutable actual members and requested
    // result population. It permits only that exact explicit companion field,with original hand.
    let expected_mirroring = repairs
        .repair(*id)
        .map_or(mirroring, JoinCompanionRepair::candidate_mirroring);
    if *expected_mirroring != candidate.mirroring {
        return Some(F::Mirroring);
    }
    if *cut_on_fold != candidate.cut_on_fold {
        return Some(F::CutOnFold);
    }
    if *fold_edges != candidate.fold_edges {
        return Some(F::FoldEdges);
    }
    if *cutting_side != candidate.cutting_side {
        return Some(F::CuttingSide);
    }
    if *material != candidate.material {
        return Some(F::Material);
    }
    if *layer_index != candidate.layer_index {
        return Some(F::Layer);
    }
    first_label_field(label, &candidate.label).map(F::Label)
}

impl<'s> RequiredJoinCompanionRepairs<'s> {
    /// Match every actual original in authored inventory order. Fully consumed originals must be
    /// absent;retained originals preserve every field except actual derived quantity and the exact
    /// admitted companion requirement. Whole immutable I/J/H/R/B/resources were admitted before any
    /// comparison. No new quota,allocation,normalization,provider,evidence or geometry call occurs.
    /// Work O(I log(I+J+M+2)+H+R+B+1),constant auxiliary storage;earlier owned storage stays separate.
    /// Outstanding repair/evidence requirements remain visible and unaccepted. Joined-result fields,
    /// consumer repair,currentness,global identity,physical validity and atomic effects stay pending.
    /// # Errors
    /// First original count/membership invariant,then first actual original field mismatch in
    /// Piece-definition order and label name/size/fabric/colorway order. No accepted prefix escapes.
    pub fn match_original_pieces<'a>(
        &'a self,
    ) -> Result<MatchedJoinOriginalPieces<'a>, JoinOriginalPieceError<'a>>
    where
        's: 'a,
    {
        use JoinOriginalPieceRefusal as R;
        let refuse = |refusal| JoinOriginalPieceError {
            repairs: self,
            refusal,
        };
        let plan = self.fields().plan();
        let mut retained_pieces = 0;
        for original in self.fields().original_pieces() {
            let piece = original.id();
            let count = plan
                .original_retained_count(piece)
                .ok_or_else(|| refuse(R::CountInvariant { piece }))?;
            let quantity = u32::try_from(count).map_err(|_| refuse(R::CountInvariant { piece }))?;
            let candidate = plan.piece(piece);
            if quantity == 0 {
                if candidate.is_some() {
                    return Err(refuse(R::MembershipInvariant { piece }));
                }
                continue;
            }
            let candidate = candidate.ok_or_else(|| refuse(R::MembershipInvariant { piece }))?;
            if let Some(field) = first_field(
                self,
                original.definition(),
                candidate.definition(),
                quantity,
            ) {
                return Err(refuse(R::Field { piece, field }));
            }
            retained_pieces += 1; // Bounded by the already admitted original header population I.
        }
        Ok(MatchedJoinOriginalPieces {
            repairs: self,
            retained_pieces,
        })
    }
}
/// Complete exact original-field matching,not joined-result/consumer/physical acceptance.
/// ```compile_fail
/// fn forge<'a>(mut proof:sc_core::construction::MatchedJoinOriginalPieces<'a>,other:&'a sc_core::construction::RequiredJoinCompanionRepairs<'a>) {proof.repairs=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::RequiredJoinCompanionRepairs<'_>)->sc_core::construction::MatchedJoinOriginalPieces<'static> {
///     raw.match_original_pieces().unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct MatchedJoinOriginalPieces<'a> {
    repairs: &'a RequiredJoinCompanionRepairs<'a>,
    retained_pieces: usize,
}
impl<'a> MatchedJoinOriginalPieces<'a> {
    /// Actual whole original/candidate resources and explicit unaccepted companion/evidence requests.
    #[must_use]
    pub const fn repairs(&self) -> &'a RequiredJoinCompanionRepairs<'a> {
        self.repairs
    }
    /// Every original member,including fully consumed and unrelated originals.
    #[must_use]
    pub fn original_piece_count(&self) -> usize {
        self.repairs.fields().original_pieces().len()
    }
    /// Original members with positive derived retained quantity,not physical-copy count.
    #[must_use]
    pub const fn retained_piece_count(&self) -> usize {
        self.retained_pieces
    }
    /// Fully consumed original members;this proof performs no retirement.
    #[must_use]
    pub fn consumed_piece_count(&self) -> usize {
        self.original_piece_count() - self.retained_pieces
    }
    /// Authentic original members in their inventory's authored order,and reverse.
    pub fn originals(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinOriginalPiece<'_>> + DoubleEndedIterator {
        let plan = self.repairs.fields().plan();
        self.repairs
            .fields()
            .original_pieces()
            .map(move |original| JoinOriginalPiece {
                context: self,
                original,
                candidate: plan.piece(original.id()),
            })
    }
    /// Authentic scope by actual original Piece identity;unknown and new-result ids return None.
    #[must_use]
    pub fn original(&self, piece: EntityId) -> Option<JoinOriginalPiece<'_>> {
        let plan = self.repairs.fields().plan();
        let original = plan
            .correspondence()
            .sources()
            .results()
            .source_piece(piece)?;
        Some(JoinOriginalPiece {
            context: self,
            original,
            candidate: plan.piece(piece),
        })
    }
}
/// One authentic original and its exact retained candidate,or explicit fully consumed absence.
#[derive(Clone, Copy, Debug)]
pub struct JoinOriginalPiece<'a> {
    context: &'a MatchedJoinOriginalPieces<'a>,
    original: &'a Piece,
    candidate: Option<&'a Piece>,
}
impl<'a> JoinOriginalPiece<'a> {
    /// Actual complete original-field matching owner.
    #[must_use]
    pub const fn context(self) -> &'a MatchedJoinOriginalPieces<'a> {
        self.context
    }
    /// Actual original Piece,including original quantity/hand/companion/geometry declarations.
    #[must_use]
    pub const fn original(self) -> &'a Piece {
        self.original
    }
    /// Actual matched retained candidate;None means known zero retained copies,not unknown.
    #[must_use]
    pub const fn candidate(self) -> Option<&'a Piece> {
        self.candidate
    }
    /// Actual canonical quantity matched to derived retained copies;zero for a fully consumed member.
    #[must_use]
    pub fn retained_quantity(self) -> u32 {
        self.candidate
            .map_or(0, |piece| piece.definition().quantity)
    }
    /// Actual explicit companion exception and outstanding evidence request,if authored for this member.
    #[must_use]
    pub fn companion_repair(self) -> Option<JoinCompanionRepair<'a>> {
        self.context.repairs.repair(self.original.id())
    }
}
