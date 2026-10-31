//! Boundary-specific and whole source-interval evidence requests,without geometric selection.
use super::super::SlashSpreadBoundaryPolicy;
use super::RequiredSlashSpreadConsumerEvidence;
use crate::ontology::EntityId;
use core::fmt;

/// Explicit policy-specific evidence sources,not accepted incidence,closure or trim geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadBoundaryEvidenceRequest {
    /// Require corresponding opened-mouth endpoints and a supported unique closed contour.
    StraightBridges {
        /// Required actual incidence/corresponding endpoint evidence for every declared bridge.
        incidence: EntityId,
        /// Required complete boundary/hole closure and supported topology evidence.
        closure: EntityId,
    },
    /// Require the complete arrangement and unique supported result,not an arbitrary loop index.
    TrimOverlaps {
        /// Required complete bank intersections,retained ranges and overlap arrangement evidence.
        arrangement: EntityId,
        /// Required unique selected boundary/holes matching the actual declared contours.
        unique_selection: EntityId,
    },
}
impl SlashSpreadBoundaryEvidenceRequest {
    /// Required boundary mode,without checking any provider evidence.
    #[must_use]
    pub const fn policy(&self) -> SlashSpreadBoundaryPolicy {
        match self {
            Self::StraightBridges { .. } => SlashSpreadBoundaryPolicy::StraightBridges,
            Self::TrimOverlaps { .. } => SlashSpreadBoundaryPolicy::TrimOverlaps,
        }
    }
}
/// Explicit common and policy-specific evidence-source requests;no defaults or supplied geometry.
/// ```compile_fail
/// let request:sc_core::construction::SlashSpreadBoundaryRequirement=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadBoundaryRequirement {
    supported_domain: EntityId,
    source_coverage: EntityId,
    boundary: SlashSpreadBoundaryEvidenceRequest,
}
impl SlashSpreadBoundaryRequirement {
    /// Retain all original requested sources. Whole source-interval coverage must include retained,
    /// derived and omitted/trimmed domains and their bank provenance,not only identity membership.
    /// Source identities are not provider-existence,currentness or geometric certificates.
    #[must_use]
    pub const fn new(
        supported_domain: EntityId,
        source_coverage: EntityId,
        boundary: SlashSpreadBoundaryEvidenceRequest,
    ) -> Self {
        Self {
            supported_domain,
            source_coverage,
            boundary,
        }
    }
    /// Actual required supported motion/region/pivot-or-axis/topology-domain evidence source.
    #[must_use]
    pub const fn supported_domain(&self) -> &EntityId {
        &self.supported_domain
    }
    /// Actual required whole source-interval/bank coverage evidence source.
    #[must_use]
    pub const fn source_coverage(&self) -> &EntityId {
        &self.source_coverage
    }
    /// Actual original policy-specific requested sources.
    #[must_use]
    pub const fn boundary(&self) -> &SlashSpreadBoundaryEvidenceRequest {
        &self.boundary
    }
}
/// Original boundary request and actual whole consumer context survive policy mismatch.
pub struct SlashSpreadBoundaryRequirementError<'a> {
    requirement: &'a SlashSpreadBoundaryRequirement,
    consumers: &'a RequiredSlashSpreadConsumerEvidence<'a>,
}
impl<'a> SlashSpreadBoundaryRequirementError<'a> {
    /// Actual original complete request.
    #[must_use]
    pub const fn requirement(&self) -> &'a SlashSpreadBoundaryRequirement {
        self.requirement
    }
    /// Actual whole registered/source/contour/ledger and mandatory consumer-evidence context.
    #[must_use]
    pub const fn consumers(&self) -> &'a RequiredSlashSpreadConsumerEvidence<'a> {
        self.consumers
    }
    /// Actual authored policy field that the evidence request must match.
    #[must_use]
    pub fn required_policy(&self) -> &'a SlashSpreadBoundaryPolicy {
        actual_policy(self.consumers)
    }
}
impl fmt::Debug for SlashSpreadBoundaryRequirementError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadBoundaryRequirementError")
            .field("requirement", &self.requirement)
            .field("required_policy", self.required_policy())
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadBoundaryRequirementError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "slash/spread boundary evidence mode does not match {:?}",
            self.required_policy()
        )
    }
}
impl std::error::Error for SlashSpreadBoundaryRequirementError<'_> {}
fn actual_policy<'a>(
    consumers: &RequiredSlashSpreadConsumerEvidence<'a>,
) -> &'a SlashSpreadBoundaryPolicy {
    consumers
        .registration_plan()
        .sources()
        .contours()
        .edges()
        .bound_roles()
        .intent()
        .boundary_policy()
}
impl<'s> RequiredSlashSpreadConsumerEvidence<'s> {
    /// Associate all explicit boundary/domain/coverage requests with this actual complete context.
    /// Fixed field comparison allocates nothing and scans no source,consumer or geometry. This
    /// accepts no evidence,computes no intersection/map,selects no loop and mutates no ledger.
    /// # Errors
    /// Requested bridge/trim evidence mode differs from the actual original boundary-policy field.
    pub fn require_boundary<'a>(
        &'a self,
        requirement: &'a SlashSpreadBoundaryRequirement,
    ) -> Result<RequiredSlashSpreadBoundary<'a>, SlashSpreadBoundaryRequirementError<'a>>
    where
        's: 'a,
    {
        if requirement.boundary.policy() != *actual_policy(self) {
            return Err(SlashSpreadBoundaryRequirementError {
                requirement,
                consumers: self,
            });
        }
        Ok(RequiredSlashSpreadBoundary {
            requirement,
            consumers: self,
        })
    }
}
/// Authentic policy-matched requests,not geometric validity or publication readiness.
/// ```compile_fail
/// fn forge<'a>(mut required:sc_core::construction::RequiredSlashSpreadBoundary<'a>,other:&'a sc_core::construction::SlashSpreadBoundaryRequirement) {
///     required.requirement=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredSlashSpreadBoundary<'a> {
    requirement: &'a SlashSpreadBoundaryRequirement,
    consumers: &'a RequiredSlashSpreadConsumerEvidence<'a>,
}
impl<'a> RequiredSlashSpreadBoundary<'a> {
    /// Actual original common and policy-specific source fields.
    #[must_use]
    pub const fn requirement(&self) -> &'a SlashSpreadBoundaryRequirement {
        self.requirement
    }
    /// Actual whole consumer/source/contour/ledger and evidence-request context.
    #[must_use]
    pub const fn consumers(&self) -> &'a RequiredSlashSpreadConsumerEvidence<'a> {
        self.consumers
    }
    /// Actual authored boundary-policy field,without accepting geometric evidence.
    #[must_use]
    pub fn policy(&self) -> &'a SlashSpreadBoundaryPolicy {
        actual_policy(self.consumers)
    }
}
