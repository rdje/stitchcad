//! Whole original/candidate Piece-field resource admission,not semantic field equality or geometry.
use super::BoundJoinCandidatePlan;
use crate::ontology::{LabelText, MaterialAssignment, Piece, PieceDefinition};
use core::fmt;

/// Complete original I,candidate J,hole-group H,edge-field R and UTF-8 byte B allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinInventoryFieldLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinInventoryFieldLimits {
    sources: usize,
    pieces: usize,
    holes: usize,
    occurrences: usize,
    text_bytes: usize,
}
impl JoinInventoryFieldLimits {
    /// Admit all original/candidate headers before hole lengths,all H before child lengths,
    /// all R before text lengths,and all B before later content/field comparisons. No workspace.
    #[must_use]
    pub const fn new(
        sources: usize,
        pieces: usize,
        holes: usize,
        occurrences: usize,
        text_bytes: usize,
    ) -> Self {
        Self {
            sources,
            pieces,
            holes,
            occurrences,
            text_bytes,
        }
    }
    /// Every original Piece,including fully consumed/unrelated/companion members.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Every actual candidate Piece.
    #[must_use]
    pub const fn pieces(self) -> usize {
        self.pieces
    }
    /// Every original and candidate hole group,including empty child groups if ever present.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// Every original/candidate boundary,hole,construction and separate fold field.
    #[must_use]
    pub const fn occurrences(self) -> usize {
        self.occurrences
    }
    /// Every UTF-8 label byte and Unresolved material-reason byte across both whole inventories.
    #[must_use]
    pub const fn text_bytes(self) -> usize {
        self.text_bytes
    }
}
/// First complete header,nested-count arithmetic or whole data allowance cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinInventoryFieldRefusal {
    /// Whole original Piece population exceeds policy.
    Sources {
        /// Actual I.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole candidate Piece population exceeds policy.
    Pieces {
        /// Actual J.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original+candidate hole count cannot be represented.
    HoleOverflow,
    /// Complete hole groups exceed policy before child-length scans.
    Holes {
        /// Whole actual H.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original+candidate edge-field count cannot be represented.
    OccurrenceOverflow,
    /// Complete original+candidate fields exceed policy before text-length/content scans.
    Occurrences {
        /// Whole actual R.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original+candidate label/reason byte count cannot be represented.
    TextOverflow,
    /// Complete UTF-8 label/reason bytes exceed policy before content/semantic comparisons.
    TextBytes {
        /// Whole actual B.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
}
/// Whole actual candidate-plan/original context,policy and first resource cause;no accepted prefix.
pub struct JoinInventoryFieldError<'a> {
    plan: &'a BoundJoinCandidatePlan<'a>,
    limits: JoinInventoryFieldLimits,
    refusal: JoinInventoryFieldRefusal,
}
impl<'a> JoinInventoryFieldError<'a> {
    /// Actual complete original/candidate inventory/copy-plan/correspondence parents.
    #[must_use]
    pub const fn plan(&self) -> &'a BoundJoinCandidatePlan<'a> {
        self.plan
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinInventoryFieldLimits {
        self.limits
    }
    /// First complete resource/arithmetic cause;no guessed field incompatibility.
    #[must_use]
    pub const fn refusal(&self) -> JoinInventoryFieldRefusal {
        self.refusal
    }
}
impl fmt::Debug for JoinInventoryFieldError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinInventoryFieldError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinInventoryFieldError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join whole inventory fields: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinInventoryFieldError<'_> {}
fn definition(piece: &Piece) -> &PieceDefinition {
    let definition = piece.definition();
    // Exhaustive owned fields:fixed scalar fields are charged by whole Piece headers. Any new
    // field forces an accounting review;no `..` can silently omit future owned data.
    let PieceDefinition {
        id: _,
        boundary: _,
        holes: _,
        construction_lines: _,
        quantity: _,
        mirroring: _,
        cut_on_fold: _,
        fold_edges: _,
        cutting_side: _,
        material: _,
        layer_index: _,
        label: _,
    } = definition;
    definition
}
fn pieces<'a>(plan: &'a BoundJoinCandidatePlan<'a>) -> impl Iterator<Item = &'a Piece> {
    plan.original_inventory()
        .pieces()
        .iter()
        .chain(plan.inventory().pieces())
}
fn text_bytes(definition: &PieceDefinition) -> Option<usize> {
    let LabelText {
        name,
        size,
        fabric,
        colorway,
    } = &definition.label; // Exhaustive label fields.
    let labels = [name, size, fabric, colorway]
        .into_iter()
        .map(String::len)
        .try_fold(0usize, usize::checked_add)?;
    let reason = match &definition.material {
        MaterialAssignment::Assigned(_) => 0,
        MaterialAssignment::Unresolved(reason) => reason.len(),
    };
    labels.checked_add(reason)
}
impl<'a> BoundJoinCandidatePlan<'a> {
    /// Admit complete original/candidate body resources before any later semantic comparison.
    /// Whole I/J precede full H counting;H quota precedes any hole-child length scan;full R quota
    /// precedes label/reason lengths;full B precedes all label/reason content and edge-field reads.
    /// R counts separate fold declarations again,even when equal to boundary fields. B counts each
    /// original/candidate label/reason occurrence independently,even for equal or repeated content.
    /// Work O(I+J+H+1),constant counters,no allocation/provider/registry/evidence/geometry/content
    /// call. Counts use cached lengths only;no Unicode normalization or blank scan occurs. Earlier
    /// immutable copy-plan/context resources remain. Semantic original preservation,joined fields,
    /// companion/consumer repair,per-pair physical/currentness/identity and atomic effects stay required.
    /// # Errors
    /// Whole I/J;H arithmetic/quota;R arithmetic/quota;B arithmetic/quota. No accepted partial inventory.
    pub fn admit_piece_fields(
        &'a self,
        limits: JoinInventoryFieldLimits,
    ) -> Result<BoundJoinInventoryFields<'a>, JoinInventoryFieldError<'a>> {
        use JoinInventoryFieldRefusal as R;
        let refuse = |refusal| JoinInventoryFieldError {
            plan: self,
            limits,
            refusal,
        };
        let i = self.original_inventory().pieces().len();
        let j = self.inventory().pieces().len();
        if i > limits.sources {
            return Err(refuse(R::Sources {
                required: i,
                allowed: limits.sources,
            }));
        }
        if j > limits.pieces {
            return Err(refuse(R::Pieces {
                required: j,
                allowed: limits.pieces,
            }));
        }
        let holes = pieces(self)
            .map(|p| definition(p).holes.len())
            .try_fold(0usize, usize::checked_add)
            .ok_or_else(|| refuse(R::HoleOverflow))?;
        if holes > limits.holes {
            return Err(refuse(R::Holes {
                required: holes,
                allowed: limits.holes,
            }));
        }
        let mut occurrences = 0usize;
        for p in pieces(self) {
            let d = definition(p);
            for n in [
                d.boundary.len(),
                d.construction_lines.len(),
                d.fold_edges.len(),
            ]
            .into_iter()
            .chain(d.holes.iter().map(Vec::len))
            {
                occurrences = occurrences
                    .checked_add(n)
                    .ok_or_else(|| refuse(R::OccurrenceOverflow))?;
            }
        }
        if occurrences > limits.occurrences {
            return Err(refuse(R::Occurrences {
                required: occurrences,
                allowed: limits.occurrences,
            }));
        }
        let mut bytes = 0usize;
        for p in pieces(self) {
            bytes = bytes
                .checked_add(text_bytes(definition(p)).ok_or_else(|| refuse(R::TextOverflow))?)
                .ok_or_else(|| refuse(R::TextOverflow))?;
        }
        if bytes > limits.text_bytes {
            return Err(refuse(R::TextBytes {
                required: bytes,
                allowed: limits.text_bytes,
            }));
        }
        Ok(BoundJoinInventoryFields {
            plan: self,
            limits,
            holes,
            occurrences,
            text_bytes: bytes,
        })
    }
}
/// Authentic whole body resource admission only,not original/result semantic field matching.
/// ```compile_fail
/// fn forge<'a>(mut fields:sc_core::construction::BoundJoinInventoryFields<'a>,other:&'a sc_core::construction::BoundJoinCandidatePlan<'a>) {fields.plan=other;}
/// ```
/// ```compile_fail
/// fn escape(plan:sc_core::construction::BoundJoinCandidatePlan<'_>)->sc_core::construction::BoundJoinInventoryFields<'static> {
///     plan.admit_piece_fields(sc_core::construction::JoinInventoryFieldLimits::new(0,0,0,0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct BoundJoinInventoryFields<'a> {
    plan: &'a BoundJoinCandidatePlan<'a>,
    limits: JoinInventoryFieldLimits,
    holes: usize,
    occurrences: usize,
    text_bytes: usize,
}
impl<'a> BoundJoinInventoryFields<'a> {
    /// Actual whole original/candidate inventories,exact copy plan and native correspondence.
    #[must_use]
    pub const fn plan(&self) -> &'a BoundJoinCandidatePlan<'a> {
        self.plan
    }
    /// Original whole body resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinInventoryFieldLimits {
        self.limits
    }
    /// Whole admitted original+candidate hole groups,including unrelated members.
    #[must_use]
    pub const fn hole_count(&self) -> usize {
        self.holes
    }
    /// Whole admitted original+candidate edge fields,including separate fold declarations.
    #[must_use]
    pub const fn occurrence_count(&self) -> usize {
        self.occurrences
    }
    /// Whole admitted UTF-8 label/reason bytes;no content normalization or truth accepted.
    #[must_use]
    pub const fn text_bytes(&self) -> usize {
        self.text_bytes
    }
    /// Every actual original Piece in authored order,and reverse;fully consumed members remain.
    pub fn original_pieces(
        &self,
    ) -> impl ExactSizeIterator<Item = &'a Piece> + DoubleEndedIterator {
        self.plan.original_inventory().pieces().iter()
    }
    /// Every actual candidate Piece in supplied order,and reverse;no filtering or replacement.
    pub fn candidate_pieces(
        &self,
    ) -> impl ExactSizeIterator<Item = &'a Piece> + DoubleEndedIterator {
        self.plan.inventory().pieces().iter()
    }
}
