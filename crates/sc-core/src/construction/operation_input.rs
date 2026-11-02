//! Borrowed native drafting inputs,without a containing program or context acceptance.
use super::{
    BezierConstruction, CenterArcConstruction, ConstrainedSeamTruing, GrainlinePlacement,
    JoinPiecesAtSeam, LineConstruction, MirrorPieceCopy, OffsetConstruction, PointConstruction,
    SlashSpreadConstruction, WalkSeamEdit,
};
use crate::{
    command::{AddNotch, CloseDart},
    ontology::EntityId,
};
use core::fmt;
pub(super) mod resources;

/// Closed v1 input families;this tag says nothing about geometric or program acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DraftingOperationFamily {
    /// Construct a point from two original Length argument documents.
    ConstructPoint,
    /// Draw a line from its original ordered point references.
    DrawLine,
    /// Draw the supported center-based arc declaration.
    DrawArc,
    /// Draw the original explicit quadratic/cubic Bézier declaration.
    DrawBezier,
    /// Offset a directed source range with its actual allowance intent.
    OffsetEdge,
    /// Slash/spread with explicit original spatial sources and amount.
    SlashSpread,
    /// Close the targeted canonical dart.
    CloseDart,
    /// Add the original semantic notch intent.
    AddNotch,
    /// Place the targeted canonical grainline.
    PlaceGrainline,
    /// Perform the deliberate seam-walking edit.
    WalkSeam,
    /// True the named seam with its original fit source.
    TrueSeam,
    /// Mirror the named Piece with its original axis intent.
    MirrorPiece,
    /// Join two named Pieces at the original seam correspondence.
    JoinPieces,
}

/// A view of one actual native input,without copying its documents,targets or dependencies.
/// Eleven schemas retain their own operation id. The two command intents instead require an
/// explicit authored history-operation id beside the borrowed request;it is never inferred from
/// the dart,Piece or anchor. Closing still requires the canonical dart's actual closing-operation
/// binding,and a fresh notch entity id remains the bus's responsibility.
///
/// This is raw input data,not an accepted operation/program. It issues no identity,binds no frame
/// or namespace,queries no registry,normalizes/evaluates no argument and publishes no result.
/// Copying the view copies only references and the explicit semantic-operation identity.
/// ```compile_fail
/// fn escape(point:&sc_core::construction::PointConstruction)
///     ->sc_core::construction::DraftingOperationInput<'static> {
///     sc_core::construction::DraftingOperationInput::ConstructPoint(point)
/// }
/// ```
/// ```compile_fail
/// use sc_core::construction::{DraftingOperationInput,PointConstruction};
/// fn replace(point:&mut PointConstruction,other:PointConstruction) {
///     let held=DraftingOperationInput::ConstructPoint(point);
///     *point=other;
///     let _=held.operation();
/// }
/// ```
#[derive(Clone, Copy)]
pub enum DraftingOperationInput<'a> {
    /// Original point declaration and both owned argument documents.
    ConstructPoint(&'a PointConstruction),
    /// Original ordered endpoint declaration.
    DrawLine(&'a LineConstruction),
    /// Original center and all three scalar argument documents.
    DrawArc(&'a CenterArcConstruction),
    /// Original degree and ordered control references.
    DrawBezier(&'a BezierConstruction),
    /// Original source,side and canonical allowance target.
    OffsetEdge(&'a OffsetConstruction),
    /// Original explicit cut,bank,motion sources and amount document.
    SlashSpread(&'a SlashSpreadConstruction),
    /// Authored history identity beside the actual canonical-target request.
    CloseDart {
        /// Requested history-operation identity;not an issued or registry-validated id.
        operation: EntityId,
        /// Original target intent;its dart id is not this operation id.
        request: &'a CloseDart,
    },
    /// Authored history identity beside the exact original notch intent.
    AddNotch {
        /// Requested history-operation identity;not a new semantic notch identity.
        operation: EntityId,
        /// Original Piece,anchor and symbolic representation bindings.
        request: &'a AddNotch,
    },
    /// Original canonical grainline-target declaration.
    PlaceGrainline(&'a GrainlinePlacement),
    /// Original seam target and ordered deliberate balance pairs.
    WalkSeam(&'a WalkSeamEdit),
    /// Original seam,side,Piece and sourced fit tolerance.
    TrueSeam(&'a ConstrainedSeamTruing),
    /// Original source Piece and explicit axis variant.
    MirrorPiece(&'a MirrorPieceCopy),
    /// Original seam correspondence and ordered Piece targets.
    JoinPieces(&'a JoinPiecesAtSeam),
}
impl DraftingOperationInput<'_> {
    /// Exact native family,without inspecting or accepting its original fields.
    #[must_use]
    pub const fn family(&self) -> DraftingOperationFamily {
        use DraftingOperationFamily as F;
        match self {
            Self::ConstructPoint(_) => F::ConstructPoint,
            Self::DrawLine(_) => F::DrawLine,
            Self::DrawArc(_) => F::DrawArc,
            Self::DrawBezier(_) => F::DrawBezier,
            Self::OffsetEdge(_) => F::OffsetEdge,
            Self::SlashSpread(_) => F::SlashSpread,
            Self::CloseDart { .. } => F::CloseDart,
            Self::AddNotch { .. } => F::AddNotch,
            Self::PlaceGrainline(_) => F::PlaceGrainline,
            Self::WalkSeam(_) => F::WalkSeam,
            Self::TrueSeam(_) => F::TrueSeam,
            Self::MirrorPiece(_) => F::MirrorPiece,
            Self::JoinPieces(_) => F::JoinPieces,
        }
    }
    /// Original schema operation id,or explicit requested history id beside a semantic intent.
    /// Identity freshness/uniqueness,currentness and canonical target matching remain unproved.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        match self {
            Self::ConstructPoint(input) => input.operation(),
            Self::DrawLine(input) => input.operation(),
            Self::DrawArc(input) => input.operation(),
            Self::DrawBezier(input) => input.operation(),
            Self::OffsetEdge(input) => input.operation(),
            Self::SlashSpread(input) => input.operation(),
            Self::CloseDart { operation, .. } | Self::AddNotch { operation, .. } => *operation,
            Self::PlaceGrainline(input) => input.operation(),
            Self::WalkSeam(input) => input.operation(),
            Self::TrueSeam(input) => input.operation(),
            Self::MirrorPiece(input) => input.operation(),
            Self::JoinPieces(input) => input.operation(),
        }
    }
}
impl fmt::Debug for DraftingOperationInput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingOperationInput")
            .field("family", &self.family())
            .finish_non_exhaustive()
    }
}
