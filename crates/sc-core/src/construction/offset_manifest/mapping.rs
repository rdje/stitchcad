//! Normal correspondence requirements,not mapped positions or accepted geometry evidence.
use super::OffsetCandidateMatch;
use crate::ontology::EntityId;
use core::fmt;
use sc_units::{Length, Tolerance, ToleranceClass};

/// Distinct parameter laws;interval endpoints do not establish their equivalence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetCorrespondenceKind {
    /// Existing topology primitive's exact (t-from)/(to-from),without normal geometry proof.
    DeclaredAffinePrimitive,
    /// Point correspondence to the normal-offset geometry,requiring actual geometric evidence.
    NormalOffset,
}
/// Required evidence-source identities,not supplied evidence or registry/currentness proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetMappingEvidenceRequest {
    /// Requesting the old structural primitive cannot satisfy a normal correspondence obligation.
    DeclaredAffinePrimitive,
    /// Actual G2 evidence must bind original/current geometry,normal correspondence and both lengths.
    NormalOffset {
        /// Required actual source-edge arclength evidence,not a bounding-box surrogate.
        source_arclength: EntityId,
        /// Required actual output-edge arclength evidence,for output parameter conversion.
        output_arclength: EntityId,
        /// Required normal-correspondence and propagated approximation/quantization evidence.
        normal_correspondence: EntityId,
    },
}
impl OffsetMappingEvidenceRequest {
    /// Explicit requested law,without accepting any evidence.
    #[must_use]
    pub const fn kind(self) -> OffsetCorrespondenceKind {
        match self {
            Self::DeclaredAffinePrimitive => OffsetCorrespondenceKind::DeclaredAffinePrimitive,
            Self::NormalOffset { .. } => OffsetCorrespondenceKind::NormalOffset,
        }
    }
}
/// Error components in one common metric:propagated output-point positional deviation in µm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetMappingErrorPart {
    /// Approximate output curve's contribution at the corresponding point,not Hausdorff alone.
    CurveApproximation,
    /// Source-parameter conversion error after propagation through the actual correspondence.
    SourceParameter,
    /// Normal-correspondence computation's remaining output-position error.
    Correspondence,
    /// Output-parameter conversion's output-position error,based on the actual output length.
    OutputParameter,
}
const PARTS: [OffsetMappingErrorPart; 4] = [
    OffsetMappingErrorPart::CurveApproximation,
    OffsetMappingErrorPart::SourceParameter,
    OffsetMappingErrorPart::Correspondence,
    OffsetMappingErrorPart::OutputParameter,
];
/// Raw explicit ceilings;constructor retains negative inputs for typed validation at binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OffsetMappingErrorAllocation {
    curve: Length,
    source_parameter: Length,
    correspondence: Length,
    output_parameter: Length,
}
impl OffsetMappingErrorAllocation {
    /// Select all four ceilings in output-position µm;no implicit EPSgeo or unpropagated bound.
    #[must_use]
    pub const fn new(
        curve: Length,
        source_parameter: Length,
        correspondence: Length,
        output_parameter: Length,
    ) -> Self {
        Self {
            curve,
            source_parameter,
            correspondence,
            output_parameter,
        }
    }
    /// Actual original allocation field for the requested component.
    #[must_use]
    pub const fn allowance(&self, part: OffsetMappingErrorPart) -> &Length {
        match part {
            OffsetMappingErrorPart::CurveApproximation => &self.curve,
            OffsetMappingErrorPart::SourceParameter => &self.source_parameter,
            OffsetMappingErrorPart::Correspondence => &self.correspondence,
            OffsetMappingErrorPart::OutputParameter => &self.output_parameter,
        }
    }
}
/// Raw explicit evidence/budget requirement,without default policy or evidence acceptance.
/// ```compile_fail
/// let requirement:sc_core::construction::OffsetMappingRequirement<'_>=Default::default();
/// ```
#[derive(Clone, Copy, Debug)]
pub struct OffsetMappingRequirement<'a> {
    evidence: OffsetMappingEvidenceRequest,
    tolerance_source: EntityId,
    tolerance: &'a Tolerance,
    allocation: OffsetMappingErrorAllocation,
}
impl<'a> OffsetMappingRequirement<'a> {
    /// Retain the actual supplied tolerance and all raw requirement fields without accepting them.
    #[must_use]
    pub const fn new(
        evidence: OffsetMappingEvidenceRequest,
        tolerance_source: EntityId,
        tolerance: &'a Tolerance,
        allocation: OffsetMappingErrorAllocation,
    ) -> Self {
        Self {
            evidence,
            tolerance_source,
            tolerance,
            allocation,
        }
    }
    /// Actual original evidence request;these identities are not resolved provider certificates.
    #[must_use]
    pub const fn evidence(&self) -> &OffsetMappingEvidenceRequest {
        &self.evidence
    }
    /// Required tolerance's source identity,without source-registry authority.
    #[must_use]
    pub const fn tolerance_source(&self) -> EntityId {
        self.tolerance_source
    }
    /// Actual supplied quantitative tolerance,including its original derivation.
    #[must_use]
    pub const fn tolerance(&self) -> &'a Tolerance {
        self.tolerance
    }
    /// Actual original raw component ceilings.
    #[must_use]
    pub const fn allocation(&self) -> &OffsetMappingErrorAllocation {
        &self.allocation
    }
}
/// First class/domain/budget/law refusal;no guessed point or geometric fallback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetMappingRequirementRefusal {
    /// The actual tolerance is not the internal geometric approximation class.
    ToleranceClass,
    /// The actual requested total tolerance is negative.
    NegativeTolerance,
    /// First negative allocation in curve/source/correspondence/output order.
    NegativeAllocation {
        /// Actual negative original field's role.
        part: OffsetMappingErrorPart,
    },
    /// Exact sum exceeds the actual inclusive tolerance,without narrowing to a Length.
    Budget {
        /// Exact complete sum in µm,even when outside the individual Length domain.
        required_um: i128,
        /// Actual selected tolerance limit in µm.
        allowed_um: i64,
    },
    /// The affine structural primitive cannot discharge this normal-map requirement.
    CorrespondenceKind,
}
/// Actual authentic candidate and whole raw requirement remain inspectable after refusal.
pub struct OffsetMappingRequirementError<'a> {
    candidate: OffsetCandidateMatch<'a>,
    requirement: &'a OffsetMappingRequirement<'a>,
    refusal: OffsetMappingRequirementRefusal,
}
impl<'a> OffsetMappingRequirementError<'a> {
    /// Authentic original candidate occurrence and complete source query.
    #[must_use]
    pub const fn candidate(&self) -> OffsetCandidateMatch<'a> {
        self.candidate
    }
    /// Actual whole original policy and requested evidence sources.
    #[must_use]
    pub const fn requirement(&self) -> &'a OffsetMappingRequirement<'a> {
        self.requirement
    }
    /// First actual class/domain/budget/law cause.
    #[must_use]
    pub const fn refusal(&self) -> OffsetMappingRequirementRefusal {
        self.refusal
    }
}
impl fmt::Debug for OffsetMappingRequirementError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OffsetMappingRequirementError")
            .field("output", &self.candidate.logical_output())
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for OffsetMappingRequirementError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "offset mapping requirement: {:?}", self.refusal)
    }
}
impl std::error::Error for OffsetMappingRequirementError<'_> {}

impl<'a> OffsetCandidateMatch<'a> {
    /// Bind a normal correspondence obligation to this authentic match;do not resolve ambiguity.
    /// Fixed four-component arithmetic allocates nothing;whole query/execution resources stay owned.
    /// Every ceiling is propagated output-point deviation. Actual engines must prove those ceilings
    /// using current source/output geometry and arclengths;identities and arithmetic do not prove it.
    /// # Errors
    /// T2 class,negative total tolerance,first negative component,exact total budget,then wrong law.
    pub fn require_mapping(
        self,
        requirement: &'a OffsetMappingRequirement<'a>,
    ) -> Result<RequiredOffsetMapping<'a>, OffsetMappingRequirementError<'a>> {
        let refuse = |refusal| OffsetMappingRequirementError {
            candidate: self,
            requirement,
            refusal,
        };
        let tolerance = requirement.tolerance;
        if tolerance.class != ToleranceClass::GeometricApproximation {
            return Err(refuse(OffsetMappingRequirementRefusal::ToleranceClass));
        }
        if tolerance.limit.as_micrometres() < 0 {
            return Err(refuse(OffsetMappingRequirementRefusal::NegativeTolerance));
        }
        let mut total_um = 0i128;
        for part in PARTS {
            let amount = requirement.allocation.allowance(part).as_micrometres();
            if amount < 0 {
                return Err(refuse(
                    OffsetMappingRequirementRefusal::NegativeAllocation { part },
                ));
            }
            // Four i64 values fit exactly in i128,including future wider Length domains.
            total_um += i128::from(amount);
        }
        if total_um > i128::from(tolerance.limit.as_micrometres()) {
            return Err(refuse(OffsetMappingRequirementRefusal::Budget {
                required_um: total_um,
                allowed_um: tolerance.limit.as_micrometres(),
            }));
        }
        if requirement.evidence.kind() != OffsetCorrespondenceKind::NormalOffset {
            return Err(refuse(OffsetMappingRequirementRefusal::CorrespondenceKind));
        }
        Ok(RequiredOffsetMapping {
            candidate: self,
            requirement,
            total_um,
        })
    }
}
/// Admitted requirement arithmetic on an authentic match;no mapped Param or evidence certificate.
/// ```compile_fail
/// fn escape<'a>(candidate:sc_core::construction::OffsetCandidateMatch<'a>)
///     ->sc_core::construction::RequiredOffsetMapping<'a> {
///     use sc_core::{construction::*,ontology::*};
///     let tolerance=sc_units::Tolerance::geometric_internal().unwrap();
///     let zero=sc_units::Length::from_micrometres(0).unwrap();
///     let request=OffsetMappingRequirement::new(OffsetMappingEvidenceRequest::DeclaredAffinePrimitive,
///         EntityId::from_bits(1),&tolerance,OffsetMappingErrorAllocation::new(zero,zero,zero,zero));
///     candidate.require_mapping(&request).unwrap()
/// }
/// ```
/// ```compile_fail
/// fn reparent<'a>(mut request:sc_core::construction::RequiredOffsetMapping<'a>,candidate:sc_core::construction::OffsetCandidateMatch<'a>) {
///     request.candidate=candidate;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredOffsetMapping<'a> {
    candidate: OffsetCandidateMatch<'a>,
    requirement: &'a OffsetMappingRequirement<'a>,
    total_um: i128,
}
impl<'a> RequiredOffsetMapping<'a> {
    /// Authentic candidate;the whole query still retains every ambiguous candidate or gap.
    #[must_use]
    pub const fn candidate(self) -> OffsetCandidateMatch<'a> {
        self.candidate
    }
    /// Actual raw evidence/budget requirement admitted by this view.
    #[must_use]
    pub const fn requirement(self) -> &'a OffsetMappingRequirement<'a> {
        self.requirement
    }
    /// Complete exact allocated ceiling sum,not an observed or certified achieved deviation.
    #[must_use]
    pub const fn allocated_um(self) -> i128 {
        self.total_um
    }
}
