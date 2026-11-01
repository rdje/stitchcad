//! Explicit result fold attribution to actual outer cut use,not complete source fold reconciliation.
use super::{JoinCutContourUseLocation, JoinPlacedCutUse, RequiredJoinConstructionTargets};
use crate::ontology::{EdgeRef, EntityId, LocalTag};
use core::fmt;

/// Explicit desired result cutting state;no inferred axis,source preservation or default.
/// ```compile_fail
/// let intent:sc_core::construction::JoinFoldIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinFoldIntent {
    /// Explicit desired result without a fold;original fold actions still require reconciliation.
    NoFold,
    /// Exact actual outer cut output requested as the one result fold edge.
    CutOnFold {
        /// Persistent actual output tag,not a contour/candidate ordinal.
        tag: LocalTag,
    },
}
/// Explicit operation,result fold choice and required geometric correspondence evidence source.
/// ```compile_fail
/// let request:sc_core::construction::JoinFoldRequirement=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinFoldRequirement {
    operation: EntityId,
    intent: JoinFoldIntent,
    correspondence: EntityId,
}
impl JoinFoldRequirement {
    /// Retain all original fields,including unresolved correspondence ids;no evidence is accepted.
    /// A correspondence request is explicit even for NoFold;source actions must still account for
    /// every original A/B fold field before any result/effect can be accepted.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        intent: JoinFoldIntent,
        correspondence: EntityId,
    ) -> Self {
        Self {
            operation,
            intent,
            correspondence,
        }
    }
    /// Actual original operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Actual original result cutting-state request.
    #[must_use]
    pub const fn intent(&self) -> &JoinFoldIntent {
        &self.intent
    }
    /// Actual original required correspondence source field,not accepted evidence or authority.
    #[must_use]
    pub const fn correspondence(&self) -> &EntityId {
        &self.correspondence
    }
}
/// Whole original source R,actual candidate N and actual cut-use C allowances before fold fields.
/// ```compile_fail
/// let limits:sc_core::construction::JoinFoldLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinFoldLimits {
    sources: usize,
    outputs: usize,
    cut_uses: usize,
}
impl JoinFoldLimits {
    /// Admit all actual original fields/candidates/cut uses,including unrelated and nonfold fields.
    /// Earlier complete construction/source/direction/path/mapping resources remain separate.
    #[must_use]
    pub const fn new(sources: usize, outputs: usize, cut_uses: usize) -> Self {
        Self {
            sources,
            outputs,
            cut_uses,
        }
    }
    /// Every original A/B source field,including separate fold declarations.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Every actual cut/noncut candidate.
    #[must_use]
    pub const fn outputs(self) -> usize {
        self.outputs
    }
    /// Every actual declared cut use,including all holes.
    #[must_use]
    pub const fn cut_uses(self) -> usize {
        self.cut_uses
    }
}
/// First complete resource,operation or exact actual outer-fold attribution cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinFoldRefusal {
    /// Complete original source fields exceed policy.
    Sources {
        /// Actual whole R.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete actual candidate population exceeds policy.
    Outputs {
        /// Actual whole N.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete actual cut uses exceed policy.
    CutUses {
        /// Actual whole C.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Raw request names another operation.
    Operation,
    /// Requested tag names no actual directional candidate,even if active in role metadata.
    Output,
    /// Actual candidate has no outer cut use;hole/construction/fold-only attribution is insufficient.
    OutputNotBoundary,
}
/// Whole original construction/source context,raw request,policy and first refusal remain borrowed.
pub struct JoinFoldError<'a> {
    construction: &'a RequiredJoinConstructionTargets<'a>,
    requirement: &'a JoinFoldRequirement,
    limits: JoinFoldLimits,
    refusal: JoinFoldRefusal,
}
impl<'a> JoinFoldError<'a> {
    /// Actual complete original construction/cut-path/source context.
    #[must_use]
    pub const fn construction(&self) -> &'a RequiredJoinConstructionTargets<'a> {
        self.construction
    }
    /// Actual complete original fold/correspondence request.
    #[must_use]
    pub const fn requirement(&self) -> &'a JoinFoldRequirement {
        self.requirement
    }
    /// Original explicit complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinFoldLimits {
        self.limits
    }
    /// First whole resource,owner or output attribution cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinFoldRefusal {
        self.refusal
    }
    /// Actual requested tag field only for output-specific causes;other causes None.
    #[must_use]
    pub fn refused_tag(&self) -> Option<&'a LocalTag> {
        if !matches!(
            self.refusal,
            JoinFoldRefusal::Output | JoinFoldRefusal::OutputNotBoundary
        ) {
            return None;
        }
        match self.requirement.intent() {
            JoinFoldIntent::NoFold => None,
            JoinFoldIntent::CutOnFold { tag } => Some(tag),
        }
    }
}
impl fmt::Debug for JoinFoldError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinFoldError")
            .field("requirement", &self.requirement)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinFoldError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join result fold request: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinFoldError<'_> {}
impl<'a> RequiredJoinConstructionTargets<'a> {
    /// Bind explicit result fold/correspondence intent to actual whole construction/source/cut parents.
    /// Whole R/N/C quotas precede operation and fold/tag fields. CutOnFold must name an actual
    /// directional candidate with an actual outer cut use;hole/noncut/merely catalogued roles refuse.
    /// NoFold explicitly requests result state only;it cannot silently discard either source fold.
    /// Complete original fold actions,source attribution/omission requirements and actual geometric
    /// correspondence remain mandatory next. A positive source subrange is not a whole physical axis.
    /// Work O(log(N+2)+log(C+2)+1),constant additional state,no allocation/provider/evidence/geometry
    /// call. No original state compatibility,new axis,currentness,final Piece or effect is certified.
    /// # Errors
    /// Whole R/N/C,operation,actual output membership,then actual outer cut membership.
    pub fn require_fold(
        &'a self,
        requirement: &'a JoinFoldRequirement,
        limits: JoinFoldLimits,
    ) -> Result<RequiredJoinFold<'a>, JoinFoldError<'a>> {
        use JoinFoldRefusal as R;
        let refuse = |refusal| JoinFoldError {
            construction: self,
            requirement,
            limits,
            refusal,
        };
        let uses = self.targets().coverage().groups().sources().uses();
        let contours = uses.connectivity().contours();
        let directions = contours.directions();
        let edges = directions.coverage().edges();
        let r = edges
            .identity()
            .roles()
            .banks()
            .sources()
            .occurrence_count();
        let n = edges.edge_count();
        let c = contours.use_count();
        if r > limits.sources {
            return Err(refuse(R::Sources {
                required: r,
                allowed: limits.sources,
            }));
        }
        if n > limits.outputs {
            return Err(refuse(R::Outputs {
                required: n,
                allowed: limits.outputs,
            }));
        }
        if c > limits.cut_uses {
            return Err(refuse(R::CutUses {
                required: c,
                allowed: limits.cut_uses,
            }));
        }
        if requirement.operation != uses.manifest().operation() {
            return Err(refuse(R::Operation));
        }
        let boundary = match requirement.intent {
            JoinFoldIntent::NoFold => None,
            JoinFoldIntent::CutOnFold { tag } => {
                directions
                    .assignment(tag)
                    .ok_or_else(|| refuse(R::Output))?;
                let placed = contours
                    .use_for(tag)
                    .filter(|u| matches!(u.location(), JoinCutContourUseLocation::Boundary(_)))
                    .ok_or_else(|| refuse(R::OutputNotBoundary))?;
                Some(placed)
            }
        };
        Ok(RequiredJoinFold {
            construction: self,
            requirement,
            limits,
            boundary,
        })
    }
}
/// Authentic explicit desired result fold/outer attribution;source actions and geometric proof pending.
/// ```compile_fail
/// fn forge<'a>(mut fold:sc_core::construction::RequiredJoinFold<'a>,other:&'a sc_core::construction::JoinFoldRequirement) {fold.requirement=other;}
/// ```
/// ```compile_fail
/// fn escape(construction:sc_core::construction::RequiredJoinConstructionTargets<'_>,request:sc_core::construction::JoinFoldRequirement)->sc_core::construction::RequiredJoinFold<'static> {
///     construction.require_fold(&request,sc_core::construction::JoinFoldLimits::new(0,0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct RequiredJoinFold<'a> {
    construction: &'a RequiredJoinConstructionTargets<'a>,
    requirement: &'a JoinFoldRequirement,
    limits: JoinFoldLimits,
    boundary: Option<JoinPlacedCutUse<'a>>,
}
impl<'a> RequiredJoinFold<'a> {
    /// Actual whole construction/cut-path/source parents.
    #[must_use]
    pub const fn construction(&self) -> &'a RequiredJoinConstructionTargets<'a> {
        self.construction
    }
    /// Actual original explicit result/correspondence request.
    #[must_use]
    pub const fn requirement(&self) -> &'a JoinFoldRequirement {
        self.requirement
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinFoldLimits {
        self.limits
    }
    /// Actual requested outer cut-use field,or None for explicit NoFold.
    #[must_use]
    pub const fn boundary(&self) -> Option<JoinPlacedCutUse<'a>> {
        self.boundary
    }
    /// Desired result cutting state derives only from explicit intent,not an independent flag.
    /// It is not accepted until complete original fold/source/physical-copy requirements hold.
    #[must_use]
    pub const fn cut_on_fold(&self) -> bool {
        self.boundary.is_some()
    }
    /// Desired actual logical fold output tuple,unissued;None for explicit NoFold.
    #[must_use]
    pub fn edge(&self) -> Option<EdgeRef> {
        self.boundary.map(|u| u.output().source().logical_output())
    }
}
