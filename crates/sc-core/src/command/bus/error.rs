//! Original structured preparation refusals shared by every implemented metadata candidate.
use super::CommandBusError;
use crate::{
    design::{
        DesignGarmentEnvelopeError, DesignGarmentError, DesignInputsError, DesignMetadataError,
        DesignSewingError, DesignStructureError,
    },
    ontology::{EntityId, IdError, NotchError, PieceError, PieceInventoryError},
    value::CanonicalInputsError,
};
use core::fmt;

/// Original preparation failures, without partial publication or a substitute model.
#[derive(Debug)]
pub enum CommandPreviewError<E> {
    /// Original trusted operation-policy/admission/cancellation refusal.
    Control(Box<super::CommandControlError>),
    /// Authentication, request binding or unavailable-handler cause.
    Command(Box<CommandBusError>),
    /// Original explicit execution-envelope cause before source/operation work.
    Envelope(Box<DesignGarmentEnvelopeError>),
    /// Original complete current source cause before copying a candidate.
    Current(Box<DesignGarmentError<E>>),
    /// Original actual Measurement-role/binding/current-source refusal.
    MeasurementBinding(Box<DesignMetadataError<E>>),
    /// Replacement attempts to change the actual bound canonical declaration identity.
    MeasurementDeclarationMismatch {
        /// Original requested Measurement metadata identity.
        measurement: EntityId,
        /// Actual bound canonical declaration identity.
        expected: EntityId,
        /// Requested replacement declaration identity.
        requested: EntityId,
    },
    /// Original immutable canonical replacement cause.
    CanonicalInputs(Box<CanonicalInputsError>),
    /// Actual selection names no Piece; index is zero-based in the request.
    MissingPiece {
        /// Original requested identity.
        piece: EntityId,
        /// Original selection index.
        index: usize,
    },
    /// The private staged reserve contains no unused identity.
    Identity(IdError),
    /// Original born-valid anchor/ownership cause for the proposed semantic Notch.
    Notch(NotchError),
    /// Exact actual candidate comparison found an unpermitted semantic change.
    Difference(super::CommandDifferenceError),
    /// Original local immutable Piece-replacement cause.
    Piece {
        /// Original selected identity.
        piece: EntityId,
        /// Original local validation cause.
        issue: PieceError,
    },
    /// Original whole pair-consistent inventory cause.
    Pieces(Box<PieceInventoryError>),
    /// Original structural ownership/copy-plan cause.
    Structure(Box<DesignStructureError>),
    /// Original current registry/input cause, including a missing assigned Material.
    Inputs(Box<DesignInputsError<E>>),
    /// Original sewing/root ownership cause.
    Sewing(Box<DesignSewingError<E>>),
    /// Original candidate garment/current-source cause.
    Candidate(Box<DesignGarmentError<E>>),
}
impl<E: fmt::Display> fmt::Display for CommandPreviewError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Control(e) => write!(f,"preview control: {e}"),
            Self::Command(e) => write!(f, "preview request: {e}"),
            Self::Envelope(e) => write!(f, "preview envelope: {e}"),
            Self::Current(e) => write!(f, "preview current sources: {e}"),
            Self::MeasurementBinding(e) => write!(f, "preview Measurement binding: {e}"),
            Self::MeasurementDeclarationMismatch { measurement, expected, requested } => write!(f,
                "preview Measurement {measurement} binds declaration {expected}, replacement names {requested}"),
            Self::CanonicalInputs(e) => write!(f, "preview canonical inputs: {e}"),
            Self::MissingPiece { piece, index } => {
                write!(f, "preview Piece {piece} absent at selection {index}")
            }
            Self::Identity(e) => write!(f, "preview identity: {e}"),
            Self::Notch(e) => write!(f, "preview Notch: {e}"),
            Self::Difference(e) => write!(f, "preview difference: {e}"),
            Self::Piece { piece, issue } => write!(f, "preview Piece {piece}: {issue}"),
            Self::Pieces(e) => write!(f, "preview Piece inventory: {e}"),
            Self::Structure(e) => write!(f, "preview structure: {e}"),
            Self::Inputs(e) => write!(f, "preview inputs: {e}"),
            Self::Sewing(e) => write!(f, "preview sewing: {e}"),
            Self::Candidate(e) => write!(f, "preview candidate: {e}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for CommandPreviewError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Control(e) => Some(e.as_ref()),
            Self::Command(e) => Some(e.as_ref()),
            Self::Envelope(e) => Some(e.as_ref()),
            Self::Current(e) | Self::Candidate(e) => Some(e.as_ref()),
            Self::MeasurementBinding(e) => Some(e.as_ref()),
            Self::MeasurementDeclarationMismatch { .. } => None,
            Self::CanonicalInputs(e) => Some(e.as_ref()),
            Self::MissingPiece { .. } => None,
            Self::Identity(e) => Some(e),
            Self::Notch(e) => Some(e),
            Self::Difference(e) => Some(e),
            Self::Piece { issue, .. } => Some(issue),
            Self::Pieces(e) => Some(e.as_ref()),
            Self::Structure(e) => Some(e.as_ref()),
            Self::Inputs(e) => Some(e.as_ref()),
            Self::Sewing(e) => Some(e.as_ref()),
        }
    }
}
impl<E> From<super::CommandControlError> for CommandPreviewError<E> {
    fn from(issue: super::CommandControlError) -> Self {
        Self::Control(Box::new(issue))
    }
}
