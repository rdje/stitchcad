//! Authentic planned family fields;no existing-geometry lookup or synthesized family proof.
use super::{DraftingContextSource, DraftingGeometryReference, DraftingSourceRecord};
use crate::{
    construction::{
        ActiveOffsetCandidate, DraftingGeometryNameSource, DraftingGeometryStepRequest,
        DraftingOperationInput, DraftingOutputPosition, JoinOutputEdge, MirrorPieceOutputEdge,
        MirrorPiecePoint, SlashSpreadEdge, TruingOutputEdge,
    },
    design::DesignMetadataHost,
};
use core::fmt;

/// Authentic original variable-output field from its existing published family factory.
/// Every variant retains the actual admitted manifest/source/copy/identity parents and original
/// active catalogue membership. A catalogue alone cannot construct this field. Logical outputs
/// remain planned/unissued;this adds no numeric geometry,frame,currentness or program approval.
/// ```compile_fail,E0308
/// use sc_core::construction::{ActiveOffsetRole,DraftingPlannedOutput};
/// fn fake<'a>(role:ActiveOffsetRole<'a>)->DraftingPlannedOutput<'a> {
///     DraftingPlannedOutput::Offset(role)
/// }
/// ```
/// ```compile_fail
/// fn escape<'a>(output:sc_core::construction::DraftingPlannedOutput<'a>)
///     ->sc_core::construction::DraftingPlannedOutput<'static> {output}
/// ```
#[derive(Clone, Copy)]
pub enum DraftingPlannedOutput<'a> {
    /// Actual original active Offset candidate,including bound input and interval/role association.
    Offset(ActiveOffsetCandidate<'a>),
    /// Actual original admitted slash/spread edge and full output intent/role/source parents.
    SlashSpread(SlashSpreadEdge<'a>),
    /// Actual original admitted truing edge and copy/source/role parents.
    Truing(TruingOutputEdge<'a>),
    /// Actual original admitted Mirror edge and identity/metadata/cut/source parents.
    MirrorEdge(MirrorPieceOutputEdge<'a>),
    /// Actual original admitted Mirror point and its distinct point-role/identity parents.
    MirrorPoint(MirrorPiecePoint<'a>),
    /// Actual original admitted Join edge and identity/bank/population/source parents.
    Join(JoinOutputEdge<'a>),
}
impl<'a> DraftingPlannedOutput<'a> {
    /// Actual original creating input borrowed through the authentic public family parent chain.
    /// Equal-looking copied inputs are not substituted and no binding is reconstructed from ids.
    #[must_use]
    pub fn input(self) -> DraftingOperationInput<'a> {
        match self {
            Self::Offset(p) => DraftingOperationInput::OffsetEdge(
                p.presence().admitted().bound_input().construction(),
            ),
            Self::SlashSpread(p) => {
                DraftingOperationInput::SlashSpread(p.admitted().bound_roles().intent().input())
            }
            Self::Truing(p) => {
                DraftingOperationInput::TrueSeam(p.owner().roles().copies().source().edit())
            }
            Self::MirrorEdge(p) => DraftingOperationInput::MirrorPiece(
                p.owner()
                    .identity()
                    .roles()
                    .metadata()
                    .cut()
                    .source()
                    .input(),
            ),
            Self::MirrorPoint(p) => DraftingOperationInput::MirrorPiece(
                p.owner()
                    .roles()
                    .identity()
                    .roles()
                    .metadata()
                    .cut()
                    .source()
                    .input(),
            ),
            Self::Join(p) => DraftingOperationInput::JoinPieces(
                p.owner()
                    .identity()
                    .roles()
                    .banks()
                    .sources()
                    .population()
                    .metadata()
                    .splice()
                    .source()
                    .edit(),
            ),
        }
    }
    /// Actual typed active role and original catalogue/definition/tag loan from this family field.
    /// Offset uses the intentional shared Edge aliases;Mirror points retain the distinct domain.
    #[must_use]
    pub fn name_source(self) -> DraftingGeometryNameSource<'a> {
        match self {
            Self::Offset(p) => DraftingGeometryNameSource::EdgeRole(p.role()),
            Self::SlashSpread(p) => DraftingGeometryNameSource::EdgeRole(p.role()),
            Self::Truing(p) => DraftingGeometryNameSource::EdgeRole(p.role()),
            Self::MirrorEdge(p) => DraftingGeometryNameSource::EdgeRole(p.role()),
            Self::MirrorPoint(p) => DraftingGeometryNameSource::PointRole(p.role()),
            Self::Join(p) => DraftingGeometryNameSource::EdgeRole(p.role()),
        }
    }
    /// Original logical output from the actual family field;no identity is issued or made live.
    #[must_use]
    pub fn reference(self) -> DraftingGeometryReference {
        match self {
            Self::Offset(p) => DraftingGeometryReference::Edge(p.logical_output()),
            Self::SlashSpread(p) => DraftingGeometryReference::Edge(p.logical_output()),
            Self::Truing(p) => DraftingGeometryReference::Edge(p.logical_output()),
            Self::MirrorEdge(p) => DraftingGeometryReference::Edge(p.logical_output()),
            Self::MirrorPoint(p) => DraftingGeometryReference::Point(p.logical_output()),
            Self::Join(p) => DraftingGeometryReference::Edge(p.logical_output()),
        }
    }
}
impl fmt::Debug for DraftingPlannedOutput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Offset(_) => "DraftingPlannedOutput::Offset(..)",
            Self::SlashSpread(_) => "DraftingPlannedOutput::SlashSpread(..)",
            Self::Truing(_) => "DraftingPlannedOutput::Truing(..)",
            Self::MirrorEdge(_) => "DraftingPlannedOutput::MirrorEdge(..)",
            Self::MirrorPoint(_) => "DraftingPlannedOutput::MirrorPoint(..)",
            Self::Join(_) => "DraftingPlannedOutput::Join(..)",
        })
    }
}

/// Raw planned-output response with authentic family field and original actual context records.
/// Public construction proves no containing Root/input/catalogue/frame/snapshot association.
/// It carries no geometry payload or arbitrary substituted output reference. Copying this view
/// copies only the authentic field and loans,without Clone/Debug bounds on provider records.
/// ```
/// use sc_core::construction::{ActiveOffsetCandidate, DraftingPlannedOutput,
///     DraftingPlannedOutputSource, DraftingSourceRecord};
/// fn raw_planned_source<'a,F:?Sized,S:?Sized>(field:ActiveOffsetCandidate<'a>,
///     frame:&'a DraftingSourceRecord<F>,snapshot:&'a DraftingSourceRecord<S>)
///     ->DraftingPlannedOutputSource<'a,F,S> {
///     let source=DraftingPlannedOutputSource {output:DraftingPlannedOutput::Offset(field),frame,snapshot};
///     let copied=source;
///     assert!(copied.frame.is_same_record(frame));
///     copied
/// }
/// ```
/// ```compile_fail
/// use sc_core::construction::DraftingPlannedOutputSource;
/// fn escape<'a>(source:DraftingPlannedOutputSource<'a,(),()>)
///     ->DraftingPlannedOutputSource<'static,(),()> {source}
/// ```
pub struct DraftingPlannedOutputSource<'a, F: ?Sized, S: ?Sized> {
    /// Authentic original family field;metadata is not accepted physical geometry.
    pub output: DraftingPlannedOutput<'a>,
    /// Actual selected Cartesian frame source record for this planned field and its source parents.
    pub frame: &'a DraftingSourceRecord<F>,
    /// Actual immutable context snapshot supplying those original source/copy/identity parents.
    pub snapshot: &'a DraftingSourceRecord<S>,
}
impl<F: ?Sized, S: ?Sized> Copy for DraftingPlannedOutputSource<'_, F, S> {}
impl<F: ?Sized, S: ?Sized> Clone for DraftingPlannedOutputSource<'_, F, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<F: ?Sized, S: ?Sized> fmt::Debug for DraftingPlannedOutputSource<'_, F, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingPlannedOutputSource")
            .field("output", &self.output)
            .finish_non_exhaustive()
    }
}
/// Original typed source refusal or raw planned field/record loans;no binding validation implied.
pub type DraftingPlannedOutputSourceResult<'a, F, S, E> =
    Result<DraftingPlannedOutputSource<'a, F, S>, E>;

/// Readonly actual planned-family field facet of the immutable containing drafting source.
/// Core calls only after complete held-content/prospective work and original position admission,
/// for requests whose typed role candidate requires binding. Fixed-schema names need no lookup.
///
/// Implementations return actual existing family-factory fields,with their complete original
/// parent chain rooted in this source's actual Root/frame/snapshot. All held parent inventories,
/// duplicated snapshots and retained indexes belong in the inherited exhaustive resource walk.
/// No lazy allocation,cloning,manufactured parent proof,currentness mutation,values or existing-
/// geometry lookup may create a response. Missing/retired/wrong-family sources retain original
/// typed causes. The immutable honest/exhaustive provider obligation is not a security boundary.
///
/// A raw response grants no acceptance:core must check actual input and catalogue/definition/tag
/// identity plus original frame/snapshot source records. Planned output stays outside the existing
/// geometry registry until execution creates it. Providers need neither Clone nor Debug.
pub trait DraftingPlannedOutputProvider<H: DesignMetadataHost>: DraftingContextSource<H> {
    /// Resolve this exact original named-output occurrence to an authentic actual family field.
    /// # Errors
    /// Original missing/retired/wrong-family/current-source refusal;never a fabricated output.
    fn planned_output<'v, 'n, 'i>(
        &'v self,
        request: DraftingOutputPosition<'v, 'n, 'i>,
    ) -> DraftingPlannedOutputSourceResult<'v, Self::Frame, Self::Snapshot, Self::Error>;
}

/// Readonly authentic planned-family source facet for an actual strictly earlier creating step
/// and reference, including outputs with no global name. This supplements the named-output port;
/// callers never manufacture a name, catalogue entry or named-output occurrence to use it.
///
/// Core may call only after complete geometry content/work admission, for variable-output
/// requests. Return an actual already-held `DraftingPlannedOutputSource` with original family
/// parents, frame and snapshot. The inherited exhaustive resource walk covers all those records.
/// No lazy allocation, cloning, parent-proof manufacture, values, existing-geometry lookup or
/// currentness mutation may create a response. Missing/unsupported sources retain original causes.
///
/// The raw response grants no source acceptance. The later core controller must check original
/// creating-input identity, exact reference and original frame/snapshot records before retaining
/// any complete result. A fixed request needs no source callback. This interface supplies no
/// geometry work allowance, existing/live planned output or whole-programme proof. Providers and
/// opaque records need neither Clone nor Debug; the honest immutable port is no security boundary.
///
/// ```
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn original_planned_field<'v,'i,H:DesignMetadataHost,S:DraftingGeometryStepProvider<H> + ?Sized>(
///     source:&'v S,request:DraftingGeometryStepRequest<'v,'i>)
///     ->DraftingPlannedOutputSourceResult<'v,S::Frame,S::Snapshot,S::Error> {
///     source.planned_geometry(request)
/// }
/// ```
pub trait DraftingGeometryStepProvider<H: DesignMetadataHost>: DraftingContextSource<H> {
    /// Resolve the exact genuine earlier variable-output request to an authentic held family field.
    /// # Errors
    /// Original missing, retired, unsupported or current-source cause; no fabricated output.
    fn planned_geometry<'v, 'i>(
        &'v self,
        request: DraftingGeometryStepRequest<'v, 'i>,
    ) -> DraftingPlannedOutputSourceResult<'v, Self::Frame, Self::Snapshot, Self::Error>;
}
