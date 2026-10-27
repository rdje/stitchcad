//! Exact owned sewing/input components; held reference inspection is distinct from current readiness.
use super::{DesignInputs, DesignInputsError, DesignInputsPosition, DesignReferenceHost};
use crate::ontology::{EntityId, SewingGraph, SewingLandmark};
use core::fmt;
use std::collections::BTreeMap;

/// Actual implemented root, original graph and complete authored semantic landmark inventory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignSewingDefinition<H> {
    /// Single actual canonical/input owner, with its original metadata sources.
    pub inputs: DesignInputs<H>,
    /// Original immutable sewing correspondences, including held references after later edits.
    pub graph: SewingGraph,
    /// Every owned semantic landmark, including those not selected by a span.
    pub landmarks: Vec<SewingLandmark>,
}

/// Original location in the complete input/sewing ownership census; references remain uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignSewingPosition {
    /// Original actual root ownership position, including retired creating operations.
    Inputs(DesignInputsPosition),
    /// First-class sewing graph identity.
    Graph,
    /// Authored semantic landmark position.
    Landmark {
        /// Zero-based original inventory position.
        index: usize,
    },
    /// Authored correspondence position.
    Span {
        /// Zero-based original correspondence position.
        index: usize,
    },
}

/// Original ownership/current-input refusal, without rewriting held sewing source references.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignSewingError<E> {
    /// Independently valid components claim one actual owned identity.
    DuplicateIdentity {
        /// Exact repeated identity.
        identity: EntityId,
        /// Original first owning position.
        first: DesignSewingPosition,
        /// Original first repeated owning position.
        duplicate: DesignSewingPosition,
    },
    /// Complete current root input cause, preserving the original concrete host refusal.
    Inputs(Box<DesignInputsError<E>>),
}
impl<E: fmt::Display> fmt::Display for DesignSewingError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f,
                "Design sewing identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::Inputs(issue) => write!(f, "Design sewing inputs: {issue}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignSewingError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } => None,
            Self::Inputs(issue) => Some(issue.as_ref()),
        }
    }
}

/// Private immutable owner of the implemented root and actual held sewing components.
///
/// Construction checks complete owned identities and current root inputs. It preserves original
/// graph/landmark references, including damaged held references needed for inspection and explicit
/// repair. It supplies no current sewing, geometric, numeric, Profile or release readiness proof.
///
/// ```compile_fail
/// fn edit<H>(state: &mut sc_core::design::DesignSewing<H>) {
///     state.definition.landmarks.clear();
/// }
/// ```
/// ```compile_fail
/// fn edit<H: sc_core::design::DesignReferenceHost>(state: &mut sc_core::design::DesignSewing<H>) {
///     state.definition().landmarks.clear();
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct DesignSewing<H> {
    definition: DesignSewingDefinition<H>,
}
impl<H> fmt::Debug for DesignSewing<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignSewing")
            .field("landmark_count", &self.definition.landmarks.len())
            .finish_non_exhaustive()
    }
}
impl<H: DesignReferenceHost> DesignSewing<H> {
    /// Own exact components without reconstructing born-valid objects against a later ledger.
    ///
    /// # Errors
    /// Returns first complete ownership collision, then original current root input refusal.
    pub fn new(definition: DesignSewingDefinition<H>) -> Result<Self, DesignSewingError<H::Error>> {
        let state = Self { definition };
        state.validate_owned_inputs()?;
        Ok(state)
    }
    /// Exact original definition; shared access grants no replacement or mutator.
    #[must_use]
    pub const fn definition(&self) -> &DesignSewingDefinition<H> {
        &self.definition
    }
    /// Exact single owned input/canonical root.
    #[must_use]
    pub const fn inputs(&self) -> &DesignInputs<H> {
        &self.definition.inputs
    }
    /// Original actual graph, without fresh current-reference or numeric proof.
    #[must_use]
    pub const fn graph(&self) -> &SewingGraph {
        &self.definition.graph
    }
    /// Complete original landmark inventory in authored order.
    #[must_use]
    pub fn landmarks(&self) -> &[SewingLandmark] {
        &self.definition.landmarks
    }
    /// Complete current actual ownership positions, excluding referenced parameter/copy/anchor ids.
    #[must_use]
    pub fn identities(&self) -> Vec<(EntityId, DesignSewingPosition)> {
        self.inputs()
            .identities()
            .into_iter()
            .map(|(id, position)| (id, DesignSewingPosition::Inputs(position)))
            .chain(core::iter::once((
                self.graph().id(),
                DesignSewingPosition::Graph,
            )))
            .chain(
                self.landmarks()
                    .iter()
                    .enumerate()
                    .map(|(index, landmark)| {
                        (landmark.id(), DesignSewingPosition::Landmark { index })
                    }),
            )
            .chain(
                self.graph()
                    .spans()
                    .iter()
                    .enumerate()
                    .map(|(index, span)| (span.id(), DesignSewingPosition::Span { index })),
            )
            .collect()
    }
    fn validate_owned_inputs(&self) -> Result<(), DesignSewingError<H::Error>> {
        let mut seen = BTreeMap::new();
        for (identity, position) in self.identities() {
            if let Some(first) = seen.insert(identity, position) {
                return Err(DesignSewingError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate: position,
                });
            }
        }
        self.inputs()
            .validate_current()
            .map_err(|issue| DesignSewingError::Inputs(Box::new(issue)))
    }
}
