//! Explicit whole held-edge identity and non-silent reference requirements;no journal application.
use super::{
    RequiredTruingEvidence, RequiredTruingFold, TruingDirectionalOutput, TruingEvidenceKind,
    TruingOutcomeIntent, TruingPlacedContourEdge,
};
use crate::ontology::{EdgeRef, EntityId};
use core::fmt;

/// Explicit requested disposition of the entire original held edge,not just the selected subrange.
/// ```compile_fail
/// let intent:sc_core::construction::TruingSourceIdentityIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingSourceIdentityIntent {
    /// Require the original edge to remain live;this does not retarget any semantic consumer.
    RetainOriginal,
    /// Require retirement with actual history/correspondence and mapped-or-visible repairs.
    /// This request grants no global retirement authority or current-source proof.
    RetireOriginal,
}
/// Complete actual output and generated-contour allowances;no default or filtered population.
/// ```compile_fail
/// let limits:sc_core::construction::TruingLineageLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingLineageLimits {
    outputs: usize,
    occurrences: usize,
}
impl TruingLineageLimits {
    /// Choose all T outputs and G generated contour uses before intent/field inspection.
    #[must_use]
    pub const fn new(outputs: usize, occurrences: usize) -> Self {
        Self {
            outputs,
            occurrences,
        }
    }
    /// All actual selected/outside logical outputs.
    #[must_use]
    pub const fn outputs(self) -> usize {
        self.outputs
    }
    /// All generated boundary,hole and construction uses,including unrelated fields.
    #[must_use]
    pub const fn occurrences(self) -> usize {
        self.occurrences
    }
}
/// First complete quota or local whole-edge disposition contradiction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingLineageRefusal {
    /// Whole actual output population exceeds policy.
    Outputs {
        /// Complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole generated contour population exceeds policy.
    Occurrences {
        /// Complete count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Explicit preservation requires the original Piece/copies/span/ease/stops/history unchanged.
    PreserveSource,
}
/// Actual whole fold/contour/source parents,original explicit intent,policy and first cause.
pub struct TruingLineageError<'a> {
    fold: &'a RequiredTruingFold<'a>,
    intent: &'a TruingSourceIdentityIntent,
    limits: TruingLineageLimits,
    refusal: TruingLineageRefusal,
}
impl<'a> TruingLineageError<'a> {
    /// Actual complete fold/contour/Piece/output/source context.
    #[must_use]
    pub const fn fold(&self) -> &'a RequiredTruingFold<'a> {
        self.fold
    }
    /// Actual original whole-edge disposition request.
    #[must_use]
    pub const fn intent(&self) -> &'a TruingSourceIdentityIntent {
        self.intent
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> TruingLineageLimits {
        self.limits
    }
    /// First whole quota or local disposition contradiction.
    #[must_use]
    pub const fn refusal(&self) -> TruingLineageRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingLineageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingLineageError")
            .field("intent", &self.intent)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingLineageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing whole-edge lineage: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingLineageError<'_> {}
impl<'s> RequiredTruingFold<'s> {
    /// Require explicit whole held-edge disposition and actual mapped-or-visible semantic repairs.
    /// Whole T then G quotas precede outcome/disposition inspection.
    /// RetireOriginal is impossible during PreserveSource. For Refit,the authentic candidate
    /// admission forbids self-sources;complete contours replace every original target use and
    /// fold admission requires outer membership. Those parents already exclude a result use of
    /// the retired identity. Admission reuses these actual parent guarantees.
    /// Constant work/space,no allocation/provider call,field scan or history mutation.
    /// Global observers,liveness,nonreuse,numeric maps and repair truth remain required evidence.
    /// # Errors
    /// Whole T/G quotas,then retirement during explicit preservation.
    pub fn require_lineage<'a>(
        &'a self,
        intent: &'a TruingSourceIdentityIntent,
        limits: TruingLineageLimits,
    ) -> Result<RequiredTruingLineage<'a>, TruingLineageError<'a>>
    where
        's: 'a,
    {
        use TruingLineageRefusal as R;
        let refuse = |refusal| TruingLineageError {
            fold: self,
            intent,
            limits,
            refusal,
        };
        let contours = self.contours();
        let t = contours.directions().manifest().assignments().len();
        let g = contours.occurrence_count();
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
        if *intent == TruingSourceIdentityIntent::RetireOriginal
            && contours
                .directions()
                .outside()
                .evidence()
                .locks()
                .outcome()
                .intent()
                == TruingOutcomeIntent::PreserveSource
        {
            return Err(refuse(R::PreserveSource));
        }
        Ok(RequiredTruingLineage {
            fold: self,
            intent,
            limits,
        })
    }
}
/// Authentic explicit identity/reference requests,not accepted mappings,global liveness or authority.
/// ```compile_fail
/// fn forge<'a>(mut lineage:sc_core::construction::RequiredTruingLineage<'a>,
///     other:&'a sc_core::construction::RequiredTruingFold<'a>) {lineage.fold=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(fold:&'a sc_core::construction::RequiredTruingFold<'a>)
///     ->sc_core::construction::RequiredTruingLineage<'a> {
///     use sc_core::construction::*;let intent=TruingSourceIdentityIntent::RetainOriginal;
///     fold.require_lineage(&intent,TruingLineageLimits::new(0,0)).unwrap()
/// }
/// ```
pub struct RequiredTruingLineage<'a> {
    fold: &'a RequiredTruingFold<'a>,
    intent: &'a TruingSourceIdentityIntent,
    limits: TruingLineageLimits,
}
impl fmt::Debug for RequiredTruingLineage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequiredTruingLineage")
            .field("intent", &self.intent)
            .field("limits", &self.limits)
            .field("source_edge", &self.source_edge())
            .finish_non_exhaustive()
    }
}
impl<'a> RequiredTruingLineage<'a> {
    /// Actual complete fold/contour/Piece/source proposal.
    #[must_use]
    pub const fn fold(&self) -> &'a RequiredTruingFold<'a> {
        self.fold
    }
    /// Actual explicit requested disposition of the whole original edge.
    #[must_use]
    pub const fn intent(&self) -> &'a TruingSourceIdentityIntent {
        self.intent
    }
    /// Original whole output/generated-use policy.
    #[must_use]
    pub const fn limits(&self) -> TruingLineageLimits {
        self.limits
    }
    /// Actual original whole held edge identity,not a current resolution or selected-domain-only edge.
    #[must_use]
    pub const fn source_edge(&self) -> EdgeRef {
        self.fold.contours().source().target()
    }
    /// Original canonical pattern Piece identity;this declares neither a copy fork nor a new Piece id.
    #[must_use]
    pub fn piece_id(&self) -> &'a EntityId {
        &self.fold.source_piece().definition().id
    }
    /// Actual original complete mandatory evidence policy,unchanged for either disposition.
    #[must_use]
    pub const fn evidence(&self) -> &'a RequiredTruingEvidence<'a> {
        self.fold.contours().directions().outside().evidence()
    }
    /// Actual original IdentityHistory source;numeric whole-edge mappings still need real providers.
    #[must_use]
    pub const fn history_source(&self) -> &'a EntityId {
        self.evidence()
            .policy()
            .source(TruingEvidenceKind::IdentityHistory)
    }
    /// Actual original VisibleRepairs source;all semantic consumers need mapping or visible repair.
    #[must_use]
    pub const fn repair_source(&self) -> &'a EntityId {
        self.evidence()
            .policy()
            .source(TruingEvidenceKind::VisibleRepairs)
    }
    /// Every actual new selected/outside attribution/direction in original declaration order/reverse.
    /// Held source bounds and requested orientation are not numeric or affine output maps.
    pub fn outputs(
        &self,
    ) -> impl ExactSizeIterator<Item = TruingDirectionalOutput<'_>> + DoubleEndedIterator {
        self.fold.contours().directions().assignments()
    }
    /// Every unchanged original contour use,in original output order/reverse,including repeats.
    /// O(G) traversal,no allocation;unrelated original identities are not implicitly retired.
    pub fn retained_contours(
        &self,
    ) -> impl DoubleEndedIterator<Item = TruingPlacedContourEdge<'_>> {
        self.fold
            .contours()
            .occurrences()
            .filter(|placed| placed.output().is_none())
    }
}
