//! Complete reflection/map evidence requests on authentic contour and point parents;no evidence acceptance.
use super::{AdmittedMirrorPiecePoints, MirrorAxis, MirrorPieceContours};
use crate::ontology::EntityId;
use core::fmt;

/// Required axis-specific evidence sources,not a supplied or certified reflection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceAxisEvidenceRequest {
    /// Actual original point-axis nondegeneracy and supported reflection over the required domain.
    ThroughPoints {
        /// Required provider identity,not point-coordinate evidence.
        supported_axis: EntityId,
    },
    /// Actual original range must establish a straight,nondegenerate supported axis;no tangent/chord.
    AlongRange {
        /// Whole selected range's actual straightness evidence source.
        straight_axis: EntityId,
        /// Required nondegenerate supported reflection-domain source.
        supported_axis: EntityId,
    },
}
/// Six mandatory complete-context obligations;shared provider identities do not merge their roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceCorrespondenceEvidenceKind {
    /// Current source ownership/liveness and original/common frame association for every dependency.
    SourceFrames,
    /// Whole original edge-domain maps matching every explicit output parameter direction.
    EdgeCorrespondence,
    /// Complete actual point attribution/maps,including endpoints and dependencies absent from proposals.
    PointCorrespondence,
    /// Actual reflected boundary/hole winding,closure,containment and construction/fold correspondence.
    Contours,
    /// Actual sourced T2 and propagated output-position errors,including axis and parameter conversions.
    Accuracy,
    /// Copy preserves the original source and never rewrites held references or retires source identities.
    SourcePreservation,
}
const KINDS: [MirrorPieceCorrespondenceEvidenceKind; 6] = [
    MirrorPieceCorrespondenceEvidenceKind::SourceFrames,
    MirrorPieceCorrespondenceEvidenceKind::EdgeCorrespondence,
    MirrorPieceCorrespondenceEvidenceKind::PointCorrespondence,
    MirrorPieceCorrespondenceEvidenceKind::Contours,
    MirrorPieceCorrespondenceEvidenceKind::Accuracy,
    MirrorPieceCorrespondenceEvidenceKind::SourcePreservation,
];
/// Explicit whole axis/map/source/accuracy requests,without default evidence or provider acceptance.
/// ```compile_fail
/// let request:sc_core::construction::MirrorPieceCorrespondenceRequirement=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceCorrespondenceRequirement {
    axis: MirrorPieceAxisEvidenceRequest,
    source_frames: EntityId,
    edge_correspondence: EntityId,
    point_correspondence: EntityId,
    contours: EntityId,
    accuracy: EntityId,
    source_preservation: EntityId,
}
impl MirrorPieceCorrespondenceRequirement {
    /// Retain every explicit source. Identities request evidence;they supply no registry resolution,
    /// quantitative budget,achieved error,geometry,source retirement or held-object repair authority.
    #[must_use]
    pub const fn new(
        axis: MirrorPieceAxisEvidenceRequest,
        source_frames: EntityId,
        edge_correspondence: EntityId,
        point_correspondence: EntityId,
        contours: EntityId,
        accuracy: EntityId,
        source_preservation: EntityId,
    ) -> Self {
        Self {
            axis,
            source_frames,
            edge_correspondence,
            point_correspondence,
            contours,
            accuracy,
            source_preservation,
        }
    }
    /// Actual complete original axis-specific request,including distinct range straightness obligation.
    #[must_use]
    pub const fn axis(&self) -> &MirrorPieceAxisEvidenceRequest {
        &self.axis
    }
    /// Actual original source field for each mandatory whole-context obligation.
    #[must_use]
    pub const fn source(&self, kind: MirrorPieceCorrespondenceEvidenceKind) -> &EntityId {
        match kind {
            MirrorPieceCorrespondenceEvidenceKind::SourceFrames => &self.source_frames,
            MirrorPieceCorrespondenceEvidenceKind::EdgeCorrespondence => &self.edge_correspondence,
            MirrorPieceCorrespondenceEvidenceKind::PointCorrespondence => {
                &self.point_correspondence
            }
            MirrorPieceCorrespondenceEvidenceKind::Contours => &self.contours,
            MirrorPieceCorrespondenceEvidenceKind::Accuracy => &self.accuracy,
            MirrorPieceCorrespondenceEvidenceKind::SourcePreservation => &self.source_preservation,
        }
    }
}
/// Explicit whole edge/point/source occurrence limits before parent or axis comparison.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPieceCorrespondenceLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceCorrespondenceLimits {
    edges: usize,
    points: usize,
    source_occurrences: usize,
}
impl MirrorPieceCorrespondenceLimits {
    /// Bound complete actual edge candidates,point candidates and original source occurrences.
    #[must_use]
    pub const fn new(edges: usize, points: usize, source_occurrences: usize) -> Self {
        Self {
            edges,
            points,
            source_occurrences,
        }
    }
    /// Complete edge-candidate allowance.
    #[must_use]
    pub const fn edges(self) -> usize {
        self.edges
    }
    /// Complete point-candidate allowance,including repeated original sources.
    #[must_use]
    pub const fn points(self) -> usize {
        self.points
    }
    /// Complete original edge-occurrence allowance,including construction repeats and fold.
    #[must_use]
    pub const fn source_occurrences(self) -> usize {
        self.source_occurrences
    }
}
/// First whole-resource,parent-lineage or actual axis-mode refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceCorrespondenceRefusal {
    /// Whole actual edge population exceeds policy.
    Edges {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole actual point population exceeds policy.
    Points {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole actual source occurrence population exceeds policy.
    SourceOccurrences {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Point and contour proposals do not borrow the same actual canonical Piece declaration.
    PieceContext,
    /// Requested axis-evidence mode differs from the actual original input's mode.
    AxisKind,
}
/// Actual complete proposals,request and limits survive refusal without mutation or cloning.
pub struct MirrorPieceCorrespondenceError<'a> {
    contours: &'a MirrorPieceContours<'a>,
    points: &'a AdmittedMirrorPiecePoints<'a>,
    requirement: &'a MirrorPieceCorrespondenceRequirement,
    limits: MirrorPieceCorrespondenceLimits,
    refusal: MirrorPieceCorrespondenceRefusal,
}
impl<'a> MirrorPieceCorrespondenceError<'a> {
    /// Actual whole contour/source/identity context.
    #[must_use]
    pub const fn contours(&self) -> &'a MirrorPieceContours<'a> {
        self.contours
    }
    /// Actual whole original point-candidate context,including its original identity parent.
    #[must_use]
    pub const fn points(&self) -> &'a AdmittedMirrorPiecePoints<'a> {
        self.points
    }
    /// Actual original complete evidence-source request.
    #[must_use]
    pub const fn requirement(&self) -> &'a MirrorPieceCorrespondenceRequirement {
        self.requirement
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPieceCorrespondenceLimits {
        self.limits
    }
    /// First whole-resource or actual lineage/mode cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceCorrespondenceRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceCorrespondenceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceCorrespondenceError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceCorrespondenceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror correspondence request: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceCorrespondenceError<'_> {}
impl<'s> MirrorPieceContours<'s> {
    /// Bind complete evidence requests to actual contour/point proposals from the same Piece parent.
    /// Cached whole counts precede parent identity and actual axis-mode comparison. This uses constant
    /// work with no allocation,provider lookup,source scan,coordinate mapping or mutation. Empty point
    /// metadata does not discharge the mandatory complete point correspondence requirement.
    /// # Errors
    /// Complete edges,points,source occurrences;then actual canonical Piece parent and axis mode.
    pub fn require_correspondence<'a>(
        &'a self,
        points: &'a AdmittedMirrorPiecePoints<'a>,
        requirement: &'a MirrorPieceCorrespondenceRequirement,
        limits: MirrorPieceCorrespondenceLimits,
    ) -> Result<RequiredMirrorPieceCorrespondence<'a>, MirrorPieceCorrespondenceError<'a>>
    where
        's: 'a,
    {
        use MirrorPieceCorrespondenceRefusal as R;
        let refuse = |refusal| MirrorPieceCorrespondenceError {
            contours: self,
            points,
            requirement,
            limits,
            refusal,
        };
        let edges = self.coverage().edges().edge_count();
        let n = points.point_count();
        let r = self.coverage().occurrence_count();
        if edges > limits.edges {
            return Err(refuse(R::Edges {
                required: edges,
                allowed: limits.edges,
            }));
        }
        if n > limits.points {
            return Err(refuse(R::Points {
                required: n,
                allowed: limits.points,
            }));
        }
        if r > limits.source_occurrences {
            return Err(refuse(R::SourceOccurrences {
                required: r,
                allowed: limits.source_occurrences,
            }));
        }
        let identity = self.coverage().edges().identity();
        if !core::ptr::eq(identity, points.roles().identity()) {
            return Err(refuse(R::PieceContext));
        }
        let axis = identity.roles().metadata().cut().source().input().axis();
        if !matches!(
            (axis, requirement.axis()),
            (
                MirrorAxis::ThroughPoints { .. },
                MirrorPieceAxisEvidenceRequest::ThroughPoints { .. }
            ) | (
                MirrorAxis::AlongRange(_),
                MirrorPieceAxisEvidenceRequest::AlongRange { .. }
            )
        ) {
            return Err(refuse(R::AxisKind));
        }
        Ok(RequiredMirrorPieceCorrespondence {
            contours: self,
            points,
            requirement,
            limits,
        })
    }
}
/// Authentic whole requests,not accepted source/frame/map/accuracy evidence or publication readiness.
/// ```compile_fail
/// fn reparent<'a>(mut request:sc_core::construction::RequiredMirrorPieceCorrespondence<'a>,
///     other:&'a sc_core::construction::MirrorPieceCorrespondenceRequirement) { request.requirement=other; }
/// ```
/// ```compile_fail
/// fn escape<'a>(contours:&'a sc_core::construction::MirrorPieceContours<'a>,points:&'a sc_core::construction::AdmittedMirrorPiecePoints<'a>)
///     ->sc_core::construction::RequiredMirrorPieceCorrespondence<'a> {
///     use sc_core::{construction::*,ontology::EntityId};
///     let id=EntityId::from_bits(0);
///     let raw=MirrorPieceCorrespondenceRequirement::new(MirrorPieceAxisEvidenceRequest::ThroughPoints {supported_axis:id},id,id,id,id,id,id);
///     contours.require_correspondence(points,&raw,MirrorPieceCorrespondenceLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredMirrorPieceCorrespondence<'a> {
    contours: &'a MirrorPieceContours<'a>,
    points: &'a AdmittedMirrorPiecePoints<'a>,
    requirement: &'a MirrorPieceCorrespondenceRequirement,
    limits: MirrorPieceCorrespondenceLimits,
}
impl<'a> RequiredMirrorPieceCorrespondence<'a> {
    /// Actual whole contour/source/Piece declaration owner.
    #[must_use]
    pub const fn contours(&self) -> &'a MirrorPieceContours<'a> {
        self.contours
    }
    /// Actual whole original point-candidate proposal and its same actual Piece parent.
    #[must_use]
    pub const fn points(&self) -> &'a AdmittedMirrorPiecePoints<'a> {
        self.points
    }
    /// Actual complete raw evidence-source request.
    #[must_use]
    pub const fn requirement(&self) -> &'a MirrorPieceCorrespondenceRequirement {
        self.requirement
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPieceCorrespondenceLimits {
        self.limits
    }
    /// Actual original axis field,not a reconstructed or resolved geometric axis.
    #[must_use]
    pub fn axis(&self) -> &'a MirrorAxis {
        self.contours
            .coverage()
            .edges()
            .identity()
            .roles()
            .metadata()
            .cut()
            .source()
            .input()
            .axis()
    }
    /// All six mandatory whole-context obligations in stable policy order,even at zero point candidates.
    pub fn requirements(
        &self,
    ) -> impl ExactSizeIterator<Item = RequiredMirrorPieceCorrespondenceSource<'_>> + DoubleEndedIterator
    {
        KINDS
            .into_iter()
            .map(move |kind| RequiredMirrorPieceCorrespondenceSource { owner: self, kind })
    }
}
/// One authentic original required source field,without provider resolution or acceptance authority.
#[derive(Clone, Copy, Debug)]
pub struct RequiredMirrorPieceCorrespondenceSource<'a> {
    owner: &'a RequiredMirrorPieceCorrespondence<'a>,
    kind: MirrorPieceCorrespondenceEvidenceKind,
}
impl<'a> RequiredMirrorPieceCorrespondenceSource<'a> {
    /// Actual whole point/contour/request owner.
    #[must_use]
    pub const fn owner(self) -> &'a RequiredMirrorPieceCorrespondence<'a> {
        self.owner
    }
    /// Actual mandatory obligation,not inferred from source identity equality.
    #[must_use]
    pub const fn kind(self) -> MirrorPieceCorrespondenceEvidenceKind {
        self.kind
    }
    /// Actual original required source field;shared identities retain separate role occurrences.
    #[must_use]
    pub const fn source(self) -> &'a EntityId {
        self.owner.requirement.source(self.kind)
    }
}
