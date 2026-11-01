//! Complete proposed construction-use reconciliation under explicit additional-content requests.
use super::{
    DeclaredJoinConstructionTargets, JoinAdditionalConstructionIntent, JoinPlacedConstructionUse,
};
use crate::ontology::EntityId;
use core::fmt;

/// Complete actual mapped-reference T and proposed-use U scan allowances;no new workspace.
/// ```compile_fail
/// let limits:sc_core::construction::JoinConstructionTargetCoverageLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinConstructionTargetCoverageLimits {
    mappings: usize,
    uses: usize,
}
impl JoinConstructionTargetCoverageLimits {
    /// Admit full T/U before intent or occurrence fields;earlier immutable resource policies remain.
    #[must_use]
    pub const fn new(mappings: usize, uses: usize) -> Self {
        Self { mappings, uses }
    }
    /// Complete actual retained-reference mappings.
    #[must_use]
    pub const fn mappings(self) -> usize {
        self.mappings
    }
    /// Every actual proposed occurrence,including repeated/unmapped additional content.
    #[must_use]
    pub const fn uses(self) -> usize {
        self.uses
    }
}
/// First whole resource or actual unrequested additional occurrence cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinConstructionTargetCoverageRefusal {
    /// Complete actual mappings exceed policy.
    Mappings {
        /// Complete actual T.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete actual proposed uses exceed policy.
    Uses {
        /// Complete actual U.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// SourceMappedOnly forbids this actual unmapped occurrence,even when its tag repeats a mapped use.
    AdditionalUse {
        /// First original proposed occurrence with no retained-reference assignment.
        index: usize,
    },
}
/// Whole actual mappings,policy and original first cause survive refusal;no partial result escapes.
pub struct JoinConstructionTargetCoverageError<'a> {
    targets: &'a DeclaredJoinConstructionTargets<'a>,
    limits: JoinConstructionTargetCoverageLimits,
    refusal: JoinConstructionTargetCoverageRefusal,
}
impl<'a> JoinConstructionTargetCoverageError<'a> {
    /// Actual whole declared mappings/source partitions/actions/proposed uses.
    #[must_use]
    pub const fn targets(&self) -> &'a DeclaredJoinConstructionTargets<'a> {
        self.targets
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionTargetCoverageLimits {
        self.limits
    }
    /// First whole resource or original additional occurrence cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinConstructionTargetCoverageRefusal {
        self.refusal
    }
    /// Actual first unrequested additional occurrence;whole quota causes return None.
    #[must_use]
    pub fn refused_use(&self) -> Option<JoinPlacedConstructionUse<'a>> {
        let JoinConstructionTargetCoverageRefusal::AdditionalUse { index } = self.refusal else {
            return None;
        };
        self.targets
            .coverage()
            .groups()
            .sources()
            .uses()
            .uses()
            .nth(index)
    }
}
impl fmt::Debug for JoinConstructionTargetCoverageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinConstructionTargetCoverageError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinConstructionTargetCoverageError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join construction target coverage: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinConstructionTargetCoverageError<'_> {}
impl<'a> DeclaredJoinConstructionTargets<'a> {
    /// Reconcile every actual proposed occurrence against distinct retained mappings and explicit
    /// additional-content intent. Admit full T/U before intent or occurrence fields. SourceMappedOnly
    /// refuses the first original unmapped occurrence;AllowAdditional retains each such occurrence
    /// and its authored evidence request without accepting evidence kind/state/currentness/truth.
    /// Work O(U log(T+2)+1),constant count state,no allocation/provider/geometry/evidence call.
    /// Earlier complete source/action/group/target policies and storage remain. All-Omit/empty Retain
    /// scopes still require actual omission/consumer justification;this proof does not discharge them.
    /// Source ranges/target traversal are declared requirements,not achieved maps or final Piece
    /// geometry. Fold/copy/identity/currentness/retirement and atomic authority remain separate.
    /// # Errors
    /// Complete T,U quotas,then first original additional occurrence under SourceMappedOnly.
    pub fn require_target_coverage(
        &'a self,
        limits: JoinConstructionTargetCoverageLimits,
    ) -> Result<RequiredJoinConstructionTargets<'a>, JoinConstructionTargetCoverageError<'a>> {
        use JoinConstructionTargetCoverageRefusal as R;
        let refuse = |refusal| JoinConstructionTargetCoverageError {
            targets: self,
            limits,
            refusal,
        };
        let t = self.mapping_count();
        let uses = self.coverage().groups().sources().uses();
        let u = uses.use_count();
        if t > limits.mappings {
            return Err(refuse(R::Mappings {
                required: t,
                allowed: limits.mappings,
            }));
        }
        if u > limits.uses {
            return Err(refuse(R::Uses {
                required: u,
                allowed: limits.uses,
            }));
        }
        let mut additional = 0;
        for use_ in uses.uses() {
            if self.mapping_for_use(use_.index()).is_none() {
                if matches!(
                    self.manifest().additional(),
                    JoinAdditionalConstructionIntent::SourceMappedOnly
                ) {
                    return Err(refuse(R::AdditionalUse {
                        index: use_.index(),
                    }));
                }
                additional += 1;
            }
        }
        Ok(RequiredJoinConstructionTargets {
            targets: self,
            limits,
            additional,
        })
    }
}
/// Whole native proposed-use reconciliation with explicit outstanding additional/omission obligations.
/// ```compile_fail
/// fn forge<'a>(mut proof:sc_core::construction::RequiredJoinConstructionTargets<'a>,other:&'a sc_core::construction::DeclaredJoinConstructionTargets<'a>) {proof.targets=other;}
/// ```
/// ```compile_fail
/// fn escape(targets:sc_core::construction::DeclaredJoinConstructionTargets<'_>)->sc_core::construction::RequiredJoinConstructionTargets<'static> {
///     targets.require_target_coverage(sc_core::construction::JoinConstructionTargetCoverageLimits::new(0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct RequiredJoinConstructionTargets<'a> {
    targets: &'a DeclaredJoinConstructionTargets<'a>,
    limits: JoinConstructionTargetCoverageLimits,
    additional: usize,
}
impl<'a> RequiredJoinConstructionTargets<'a> {
    /// Actual complete original target/source/action/use parents.
    #[must_use]
    pub const fn targets(&self) -> &'a DeclaredJoinConstructionTargets<'a> {
        self.targets
    }
    /// Original complete scan policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionTargetCoverageLimits {
        self.limits
    }
    /// Complete proposed construction occurrence population,including retained and additional uses.
    #[must_use]
    pub fn use_count(&self) -> usize {
        self.targets
            .coverage()
            .groups()
            .sources()
            .uses()
            .use_count()
    }
    /// Complete actual occurrences explicitly assigned retained source references.
    #[must_use]
    pub fn mapped_count(&self) -> usize {
        self.targets.mapping_count()
    }
    /// Complete actual additional occurrences under the explicit request,not accepted authored truth.
    #[must_use]
    pub const fn additional_count(&self) -> usize {
        self.additional
    }
    /// Actual authored additional-content evidence id field,if explicitly requested,even for zero extras.
    /// This supplies neither accepted evidence nor permission to omit original source/consumer fields.
    #[must_use]
    pub const fn additional_evidence_source(&self) -> Option<&'a EntityId> {
        match self.targets.manifest().additional() {
            JoinAdditionalConstructionIntent::SourceMappedOnly => None,
            JoinAdditionalConstructionIntent::AllowAdditional { evidence_source } => {
                Some(evidence_source)
            }
        }
    }
    /// Actual original additional occurrences in proposed order,and reverse;repeated tags stay separate.
    /// Work O(U log(T+2)+1),no new index/allocation;full admitted T/U applies to this immutable view.
    pub fn additional_uses(
        &self,
    ) -> impl DoubleEndedIterator<Item = JoinPlacedConstructionUse<'a>> {
        let targets = self.targets;
        targets
            .coverage()
            .groups()
            .sources()
            .uses()
            .uses()
            .filter(move |use_| targets.mapping_for_use(use_.index()).is_none())
    }
}
