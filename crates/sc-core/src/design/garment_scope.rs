//! Original explicit garment-envelope guards, independent of source and geometric validation.
use super::{DesignGarment, DesignReferenceHost};
use crate::ontology::{ClosureEnvelopeError, LayerInventoryError};
use core::fmt;

/// Original named refusal with authored context, without a supported replacement or approximation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignGarmentEnvelopeError {
    /// Original Closure definition guard, checked before layers in authored Closure order.
    Closure {
        /// Zero-based actual authored Closure position.
        index: usize,
        /// Original named refusal, including requested Closure and trousers-gap identities.
        issue: ClosureEnvelopeError,
    },
    /// Original complete layer scope refusal, retaining identity, family/position and named cause.
    Layers(Box<LayerInventoryError>),
}
impl fmt::Display for DesignGarmentEnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Closure { index, issue } => {
                write!(f, "garment Closure at {index} execution envelope: {issue}")
            }
            Self::Layers(issue) => write!(f, "garment layer execution envelope: {issue}"),
        }
    }
}
impl std::error::Error for DesignGarmentEnvelopeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Closure { issue, .. } => Some(issue),
            Self::Layers(issue) => Some(issue.as_ref()),
        }
    }
}
impl<H: DesignReferenceHost> DesignGarment<H> {
    /// Apply the implemented explicit Closure/layer envelope guards before requested execution.
    ///
    /// Checks authored Closure definitions, then original Facing/Lining/Interfacing scope. Modelled
    /// Lining refuses env_lining/G7 even with damaged sources. Fly already refuses env_fly/G7 in
    /// Closure::new before an immutable Closure can exist; this check retains that original guard.
    /// It does not validate ownership, host metadata, sources, logical domains or geometry, read
    /// canonical values, execute operations, or certify physical support/completeness/release.
    /// Inspect held content freely. Within authorized execution, apply this guard before geometry
    /// or operation execution; it does not replace authentication or effect-boundary authorization.
    /// # Errors
    /// Returns the first original named scope cause, preserving authored positions and source chains.
    pub fn require_in_scope(&self) -> Result<(), DesignGarmentEnvelopeError> {
        for (index, closure) in self.closures().closures().iter().enumerate() {
            closure
                .definition()
                .require_in_scope()
                .map_err(|issue| DesignGarmentEnvelopeError::Closure { index, issue })?;
        }
        self.layers()
            .require_in_scope()
            .map_err(|issue| DesignGarmentEnvelopeError::Layers(Box::new(issue)))
    }
}
