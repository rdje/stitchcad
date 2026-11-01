//! Explicit original-state/outer-contour fold attribution;not a geometric axis or current proof.
use super::{TruingDirectionalOutput, TruingPieceContours, TruingPieceOccurrenceLocation};
use crate::ontology::{EdgeRef, EntityId, LocalTag, Piece};
use core::fmt;

/// Explicit preservation of original cut-on-fold state and result fold attribution;no default.
/// ```compile_fail
/// let intent:sc_core::construction::TruingFoldIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingFoldIntent {
    /// Original Piece is not cut on fold;this cannot remove an existing fold.
    NoFold,
    /// Original fold identity remains as an unchanged outer-contour use.
    RetainOriginal,
    /// An actual outer output is attributed to the original fold edge.
    Derived {
        /// Persistent actual output tag,not a contour ordinal.
        tag: LocalTag,
    },
}
/// Original explicit fold choice and required geometric correspondence evidence source.
/// ```compile_fail
/// let request:sc_core::construction::TruingFoldRequirement=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingFoldRequirement {
    intent: TruingFoldIntent,
    correspondence: EntityId,
}
impl TruingFoldRequirement {
    /// Retain both original fields,including any still-unresolved evidence source identity.
    /// Original Piece metadata stays authoritative;no provider existence or evidence is accepted.
    #[must_use]
    pub const fn new(intent: TruingFoldIntent, correspondence: EntityId) -> Self {
        Self {
            intent,
            correspondence,
        }
    }
    /// Actual original explicit fold choice.
    #[must_use]
    pub const fn intent(&self) -> &TruingFoldIntent {
        &self.intent
    }
    /// Actual required geometric correspondence source,not accepted evidence.
    #[must_use]
    pub const fn correspondence(&self) -> &EntityId {
        &self.correspondence
    }
}
/// Whole actual output and generated-contour allowances;earlier parent bounds remain separate.
/// ```compile_fail
/// let limits:sc_core::construction::TruingFoldLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingFoldLimits {
    outputs: usize,
    occurrences: usize,
}
impl TruingFoldLimits {
    /// Choose all T outputs and G generated contour uses,including unrelated uses.
    #[must_use]
    pub const fn new(outputs: usize, occurrences: usize) -> Self {
        Self {
            outputs,
            occurrences,
        }
    }
    /// Every actual selected/outside logical output.
    #[must_use]
    pub const fn outputs(self) -> usize {
        self.outputs
    }
    /// Every generated boundary,hole and construction use.
    #[must_use]
    pub const fn occurrences(self) -> usize {
        self.occurrences
    }
}
/// First whole quota,original-state or exact structural fold-attribution cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingFoldRefusal {
    /// Complete actual output count exceeds policy before any fold inspection.
    Outputs {
        /// Whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete generated contour count exceeds policy before any fold inspection.
    Occurrences {
        /// Whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Requested fold presence differs from original authoritative cutting state.
    FoldStatus,
    /// Required original fold field is unavailable;no substitute is manufactured.
    SourceFold,
    /// Exact original fold is not an unchanged proposed outer-contour use.
    FoldNotRetained,
    /// Tag names no actual selected/outside output,even if active in metadata.
    Output,
    /// Actual output has no proposed outer-contour use.
    OutputNotBoundary,
    /// Actual output's attributed held source is not the exact original fold edge.
    OutputNotFoldSource,
}
/// Actual complete original requirement/contour/source parents,policy and first cause.
pub struct TruingFoldError<'a> {
    contours: &'a TruingPieceContours<'a>,
    requirement: &'a TruingFoldRequirement,
    limits: TruingFoldLimits,
    refusal: TruingFoldRefusal,
}
impl<'a> TruingFoldError<'a> {
    /// Actual complete original contour/Piece/source context.
    #[must_use]
    pub const fn contours(&self) -> &'a TruingPieceContours<'a> {
        self.contours
    }
    /// Actual entire original fold/evidence request.
    #[must_use]
    pub const fn requirement(&self) -> &'a TruingFoldRequirement {
        self.requirement
    }
    /// Original explicit complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> TruingFoldLimits {
        self.limits
    }
    /// First whole count,original-state or attribution refusal.
    #[must_use]
    pub const fn refusal(&self) -> TruingFoldRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingFoldError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingFoldError")
            .field("requirement", &self.requirement)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingFoldError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing fold requirement: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingFoldError<'_> {}
impl<'s> TruingPieceContours<'s> {
    /// Bind explicit source cutting-state preservation and exact proposed outer fold attribution.
    /// Whole T then G quotas precede original fold-state/field and proposed outer-use inspection.
    /// O(log(T+2)+G) work,constant additional space,no allocation/provider call or string scan.
    /// Actual whole-fold domain/position correspondence remains the named evidence requirement;
    /// even a source-subrange attribution does not prove it. All other source metadata stays exact.
    /// # Errors
    /// Whole T/G;original fold state/field;retained outer use or derived output,outer use,source.
    pub fn require_fold<'a>(
        &'a self,
        requirement: &'a TruingFoldRequirement,
        limits: TruingFoldLimits,
    ) -> Result<RequiredTruingFold<'a>, TruingFoldError<'a>>
    where
        's: 'a,
    {
        use TruingFoldRefusal as R;
        let refuse = |refusal| TruingFoldError {
            contours: self,
            requirement,
            limits,
            refusal,
        };
        let t = self.directions().manifest().assignments().len();
        let g = self.occurrence_count();
        if t > limits.outputs {
            return Err(refuse(R::Outputs {
                required: t,
                allowed: limits.outputs,
            }));
        }
        if g > limits.occurrences {
            return Err(refuse(R::Occurrences {
                required: g,
                allowed: limits.occurrences,
            }));
        }
        let piece = self.source().piece();
        let requested_fold = !matches!(requirement.intent, TruingFoldIntent::NoFold);
        if piece.definition().cut_on_fold != requested_fold {
            return Err(refuse(R::FoldStatus));
        }
        let mut output = None;
        if !matches!(requirement.intent, TruingFoldIntent::NoFold) {
            let original = piece
                .definition()
                .fold_edges
                .first()
                .ok_or_else(|| refuse(R::SourceFold))?;
            match requirement.intent {
                TruingFoldIntent::RetainOriginal => {
                    if !self.occurrences().any(|placed| {
                        matches!(
                            placed.location(),
                            TruingPieceOccurrenceLocation::Boundary(_)
                        ) && placed.output().is_none()
                            && placed.directed().edge == *original
                    }) {
                        return Err(refuse(R::FoldNotRetained));
                    }
                }
                TruingFoldIntent::Derived { tag } => {
                    let actual = self
                        .directions()
                        .assignment(tag)
                        .ok_or_else(|| refuse(R::Output))?;
                    if !self.occurrences().any(|placed| {
                        matches!(
                            placed.location(),
                            TruingPieceOccurrenceLocation::Boundary(_)
                        ) && placed
                            .output()
                            .is_some_and(|member| *member.assignment().tag() == tag)
                    }) {
                        return Err(refuse(R::OutputNotBoundary));
                    }
                    if actual.source().range().edge() != *original {
                        return Err(refuse(R::OutputNotFoldSource));
                    }
                    output = Some(actual);
                }
                TruingFoldIntent::NoFold => {}
            }
        }
        Ok(RequiredTruingFold {
            contours: self,
            requirement,
            limits,
            output,
        })
    }
}
/// Authentic original-state/outer fold request;not a new Piece,geometric axis or accepted evidence.
/// ```compile_fail
/// fn forge<'a>(mut fold:sc_core::construction::RequiredTruingFold<'a>,
///     other:&'a sc_core::construction::TruingPieceContours<'a>) {fold.contours=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(contours:&'a sc_core::construction::TruingPieceContours<'a>)
///     ->sc_core::construction::RequiredTruingFold<'a> {
///     use sc_core::{construction::*,ontology::EntityId};
///     let request=TruingFoldRequirement::new(TruingFoldIntent::NoFold,EntityId::from_bits(0));
///     contours.require_fold(&request,TruingFoldLimits::new(0,0)).unwrap()
/// }
/// ```
pub struct RequiredTruingFold<'a> {
    contours: &'a TruingPieceContours<'a>,
    requirement: &'a TruingFoldRequirement,
    limits: TruingFoldLimits,
    output: Option<TruingDirectionalOutput<'a>>,
}
impl fmt::Debug for RequiredTruingFold<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequiredTruingFold")
            .field("requirement", &self.requirement)
            .field("limits", &self.limits)
            .field("edge", &self.edge())
            .finish_non_exhaustive()
    }
}
impl<'a> RequiredTruingFold<'a> {
    /// Actual complete proposed contours and held source parents.
    #[must_use]
    pub const fn contours(&self) -> &'a TruingPieceContours<'a> {
        self.contours
    }
    /// Actual original explicit intent and geometric evidence request.
    #[must_use]
    pub const fn requirement(&self) -> &'a TruingFoldRequirement {
        self.requirement
    }
    /// Original whole output/contour policy.
    #[must_use]
    pub const fn limits(&self) -> TruingFoldLimits {
        self.limits
    }
    /// Actual original Piece;all canonical non-contour metadata remains authoritative.
    #[must_use]
    pub fn source_piece(&self) -> &'a Piece {
        self.contours.source().piece()
    }
    /// Actual original fold field when cutting on fold;not an inferred geometric axis.
    #[must_use]
    pub fn source_fold(&self) -> Option<&'a EdgeRef> {
        self.source_piece().definition().fold_edges.first()
    }
    /// Authentic explicit derived fold attribution,or None for retained/no-fold treatment.
    #[must_use]
    pub const fn output(&self) -> Option<TruingDirectionalOutput<'a>> {
        self.output
    }
    /// Requested original/new fold identity;new logical output remains unissued.
    #[must_use]
    pub fn edge(&self) -> Option<EdgeRef> {
        self.output
            .map(|o| o.source().logical_output())
            .or_else(|| self.source_fold().copied())
    }
}
