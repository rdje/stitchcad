//! Exact owned sewing/input components; held reference inspection is distinct from current readiness.
use super::{
    DesignInputs, DesignInputsError, DesignInputsPosition, DesignReferenceHost, DesignSewingAmount,
    DesignSewingDeclarationError,
};
use crate::ontology::{
    anchor::validate_current_anchor, AnchorError, EntityId, SewingError, SewingGraph,
    SewingLandmark,
};
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

/// Original ownership/current-source refusal, without rewriting held sewing references.
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
    /// An actual landmark's original source Piece is absent from the current root.
    MissingLandmarkPiece {
        /// Actual semantic landmark identity.
        landmark: EntityId,
        /// Exact held source Piece identity.
        piece: EntityId,
    },
    /// Original current anchor/ownership refusal, including unused landmarks.
    Landmark {
        /// Actual semantic landmark identity.
        landmark: EntityId,
        /// Original typed current anchor cause.
        issue: Box<AnchorError>,
    },
    /// Original current graph copy/range/stop/distribution refusal.
    Graph(Box<SewingError>),
    /// Original actual amount-declaration metadata refusal.
    Amount(Box<DesignSewingDeclarationError<E>>),
}
impl<E: fmt::Display> fmt::Display for DesignSewingError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f,
                "Design sewing identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::Inputs(issue) => write!(f, "Design sewing inputs: {issue}"),
            Self::MissingLandmarkPiece { landmark, piece } => write!(f, "sewing landmark {landmark} names missing Piece {piece}"),
            Self::Landmark { landmark, issue } => write!(f, "sewing landmark {landmark}: {issue}"),
            Self::Graph(issue) => write!(f, "Design sewing graph: {issue}"),
            Self::Amount(issue) => write!(f, "Design sewing amount: {issue}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignSewingError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } => None,
            Self::Inputs(issue) => Some(issue.as_ref()),
            Self::MissingLandmarkPiece { .. } => None,
            Self::Landmark { issue, .. } => Some(issue.as_ref()),
            Self::Graph(issue) => Some(issue.as_ref()),
            Self::Amount(issue) => Some(issue.as_ref()),
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
/// ```compile_fail
/// fn extract<H: sc_core::design::DesignReferenceHost>(owned: sc_core::design::DesignSewing<H>) {
///     let _ = owned.into_definition();
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct DesignSewing<H> {
    definition: DesignSewingDefinition<H>,
}
impl<H> DesignSewing<H> {
    // Exhaustive private ownership view: a new root field requires an accounting decision.
    pub(crate) const fn resource_definition(&self) -> &DesignSewingDefinition<H> {
        let Self { definition } = self;
        definition
    }
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
    // Consume an already owned component during private candidate reconstruction. Callers still
    // rebuild through validating constructors; this cannot extract from a borrowed bus owner.
    pub(crate) fn into_definition(self) -> DesignSewingDefinition<H> {
        self.definition
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
    /// Validate complete actual current root, every landmark and graph, and amount metadata.
    ///
    /// Current anchor/graph validators retain their raw choice and repair rules. No saved input
    /// success or missing-id census replaces these current checks. Profile refs stay deferred to
    /// G4; numeric amount availability, whole Design topology and execution/release stay separate.
    ///
    /// # Errors
    /// Returns first original ownership/root, landmark, graph or amount metadata cause.
    pub fn validate_current(&self) -> Result<(), DesignSewingError<H::Error>> {
        self.amount_declarations().map(|_| ())
    }
    /// Borrow actual amount declarations only after complete current sewing source validation.
    ///
    /// Every owned landmark is checked, even when no span selects it. Original graph ranges and
    /// stops use current root copies/Pieces/ledger; declarations borrow the same canonical owner.
    /// Logical Profile roles remain visibly DeferredToG4 rather than becoming numeric inputs.
    ///
    /// ```compile_fail
    /// fn escape<H: sc_core::design::DesignReferenceHost>(state: sc_core::design::DesignSewing<H>)
    ///     -> Vec<sc_core::design::DesignSewingAmount<'static>> {
    ///     state.amount_declarations().unwrap()
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns the first original current-source cause without an accepted declaration prefix.
    pub fn amount_declarations(
        &self,
    ) -> Result<Vec<DesignSewingAmount<'_>>, DesignSewingError<H::Error>> {
        self.validate_owned_inputs()?;
        let structure = self.inputs().structure();
        for landmark in self.landmarks() {
            let piece = structure.pieces().piece(landmark.piece()).ok_or(
                DesignSewingError::MissingLandmarkPiece {
                    landmark: landmark.id(),
                    piece: landmark.piece(),
                },
            )?;
            let anchor = match landmark {
                SewingLandmark::Notch(mark) => mark.definition().anchor,
                SewingLandmark::Turn(mark) => mark.anchor(),
            };
            validate_current_anchor(anchor, piece, structure.topology()).map_err(|issue| {
                DesignSewingError::Landmark {
                    landmark: landmark.id(),
                    issue: Box::new(issue),
                }
            })?;
        }
        self.graph()
            .validate_current(
                structure.cut_plan(),
                structure.pieces().pieces(),
                self.landmarks(),
                structure.topology(),
            )
            .map_err(|issue| DesignSewingError::Graph(Box::new(issue)))?;
        self.inputs()
            .sewing_amount_declarations(self.graph())
            .map_err(|issue| DesignSewingError::Amount(Box::new(issue)))
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
