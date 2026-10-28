//! Owned semantic layer families and their complete current served-Piece/source scope.
use super::{
    EntityId, Facing, IdentityLedger, Interfacing, LayerDefinition, LayerEnvelopeError, LayerError,
    LayerKind, Lining, Material, MaterialAssignment, MaterialInventory, MaterialInventoryError,
    Piece,
};
use crate::value::CanonicalInputs;
use core::fmt;
use std::collections::BTreeMap;

/// Original owning family and zero-based position; a position never supplies an entity identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LayerInventoryPosition {
    /// Distinct semantic layer family.
    pub kind: LayerKind,
    /// Zero-based authored position within that family.
    pub index: usize,
}
/// Borrowed actual layer, preserving its distinct immutable semantic type.
/// Direct construction or inspection is not a current-validation certificate.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LayerRef<'a> {
    /// Original actual Facing.
    Facing(&'a Facing),
    /// Original modelled Lining, independently subject to env_lining.
    Lining(&'a Lining),
    /// Original actual Interfacing.
    Interfacing(&'a Interfacing),
}
impl fmt::Debug for LayerRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LayerRef")
            .field("kind", &self.kind())
            .finish_non_exhaustive()
    }
}
impl<'a> LayerRef<'a> {
    /// Actual semantic kind, never inferred from a label or numeric identity.
    #[must_use]
    pub const fn kind(self) -> LayerKind {
        match self {
            Self::Facing(value) => value.kind(),
            Self::Lining(value) => value.kind(),
            Self::Interfacing(value) => value.kind(),
        }
    }
    /// Exact original identity.
    #[must_use]
    pub const fn id(self) -> EntityId {
        self.definition().id
    }
    /// Original authored served Piece.
    #[must_use]
    pub const fn served_piece(self) -> EntityId {
        self.definition().served_piece
    }
    /// Exact original input; shared access grants no mutator.
    #[must_use]
    pub const fn definition(self) -> &'a LayerDefinition {
        match self {
            Self::Facing(value) => value.definition(),
            Self::Lining(value) => value.definition(),
            Self::Interfacing(value) => value.definition(),
        }
    }
    /// Recheck the actual layer's current source rules without rewriting references.
    /// # Errors
    /// Returns the original concrete [`LayerError`] from the distinct layer's validator.
    pub fn validate_current(
        self,
        served: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<(), LayerError> {
        match self {
            Self::Facing(value) => value.validate_current(served, ledger),
            Self::Lining(value) => value.validate_current(served, ledger),
            Self::Interfacing(value) => value.validate_current(served, ledger),
        }
    }
    /// Apply the original named execution envelope independently of source or geometry validation.
    /// # Errors
    /// Modelled Lining returns the original env_lining refusal with served Piece and G7.
    pub const fn require_in_scope(self) -> Result<(), LayerEnvelopeError> {
        match self {
            Self::Facing(value) => value.require_in_scope(),
            Self::Lining(value) => value.require_in_scope(),
            Self::Interfacing(value) => value.require_in_scope(),
        }
    }
}

/// Original assignment data; direct construction supplies no independent validation certificate.
/// Query results keep the actual canonical context borrowed while the returned view is used.
///
/// ```compile_fail
/// fn replace(layers: &sc_core::ontology::LayerInventory,
///     materials: &sc_core::ontology::MaterialInventory, mut canonical: sc_core::value::CanonicalInputs) {
///     let views = layers.material_assignments(materials, &canonical).unwrap();
///     canonical = sc_core::value::CanonicalInputs::new(vec![], vec![]).unwrap();
///     let _still_borrowed = views;
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LayerMaterialView<'a> {
    /// Actual original layer and its exact current Material target; properties may remain unknown.
    Assigned {
        /// Original distinct semantic layer.
        layer: LayerRef<'a>,
        /// Actual stored Material, without copied property values or state.
        material: &'a Material,
    },
    /// Explicit original unresolved intent, without choosing a Material or default property value.
    Unresolved {
        /// Original distinct semantic layer.
        layer: LayerRef<'a>,
        /// Original untrimmed reason borrowed from that layer's assignment.
        reason: &'a str,
    },
}
impl fmt::Debug for LayerMaterialView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (layer, state) = match self {
            Self::Assigned { layer, .. } => (layer, "assigned"),
            Self::Unresolved { layer, .. } => (layer, "unresolved"),
        };
        f.debug_struct("LayerMaterialView")
            .field("kind", &layer.kind())
            .field("state", &state)
            .finish_non_exhaustive()
    }
}

/// Original identity/current-source or explicit execution-envelope refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LayerInventoryError {
    /// Actual layers across all three families claim one owned identity.
    DuplicateIdentity {
        /// Exact repeated identity.
        identity: EntityId,
        /// Original first owning position.
        first: LayerInventoryPosition,
        /// Original first repeated owning position.
        duplicate: LayerInventoryPosition,
    },
    /// Current Piece context has ambiguous identities; no first match is selected.
    DuplicatePiece(EntityId),
    /// An actual layer's original served Piece is absent from the current context.
    MissingServedPiece {
        /// Actual layer identity.
        layer: EntityId,
        /// Original owning family/position.
        position: LayerInventoryPosition,
        /// Exact original served Piece target.
        piece: EntityId,
    },
    /// Original actual layer current-source refusal.
    Layer {
        /// Actual layer identity.
        layer: EntityId,
        /// Original owning family/position.
        position: LayerInventoryPosition,
        /// Original concrete source cause with raw interval/endpoint evidence.
        issue: Box<LayerError>,
    },
    /// Original explicit scope refusal, without selecting a supported replacement.
    OutsideEnvelope {
        /// Actual modelled layer identity.
        layer: EntityId,
        /// Original owning family/position.
        position: LayerInventoryPosition,
        /// Original named envelope cause.
        issue: LayerEnvelopeError,
    },
    /// Original complete current Material inventory/property cause, including unused Materials.
    Materials(Box<MaterialInventoryError>),
    /// A layer identity aliases an actual Material owner; neither role is inferred from its label.
    MaterialIdentityCollision {
        /// Exact aliased owned identity.
        layer: EntityId,
        /// Original owning family/position.
        position: LayerInventoryPosition,
    },
    /// An assigned Material is absent; peers and numeric canonical inputs never substitute.
    MissingMaterial {
        /// Actual layer carrying the assignment.
        layer: EntityId,
        /// Original owning family/position.
        position: LayerInventoryPosition,
        /// Exact missing Material target.
        material: EntityId,
    },
}
impl fmt::Display for LayerInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity {
                identity,
                first,
                duplicate,
            } => write!(
                f,
                "layer identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"
            ),
            Self::DuplicatePiece(piece) => {
                write!(f, "layer inventory current Piece {piece} is repeated")
            }
            Self::MissingServedPiece {
                layer,
                position,
                piece,
            } => write!(
                f,
                "layer {layer} at {position:?} lacks original served Piece {piece}"
            ),
            Self::Layer {
                layer,
                position,
                issue,
            } => write!(f, "layer {layer} at {position:?}: {issue}"),
            Self::OutsideEnvelope {
                layer,
                position,
                issue,
            } => write!(f, "layer {layer} at {position:?}: {issue}"),
            Self::Materials(issue) => write!(f, "layer assignments current Materials: {issue}"),
            Self::MaterialIdentityCollision { layer, position } => write!(
                f,
                "layer {layer} at {position:?} aliases a Material identity"
            ),
            Self::MissingMaterial {
                layer,
                position,
                material,
            } => write!(
                f,
                "layer {layer} at {position:?} names absent Material {material}"
            ),
        }
    }
}
impl std::error::Error for LayerInventoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Layer { issue, .. } => Some(issue.as_ref()),
            Self::OutsideEnvelope { issue, .. } => Some(issue),
            Self::Materials(issue) => Some(issue.as_ref()),
            _ => None,
        }
    }
}

/// Private immutable Facing/Lining/Interfacing families in their exact authored family orders.
///
/// Construction checks ids and preserves original objects, including held damage. Current validation
/// visits Facing, Lining, then Interfacing and checks every original served Piece and source. Modelled
/// Lining remains inspectable; require_in_scope is a separate refusal before requested execution.
/// Material assignment queries are separate from source checks; global Design joins and G2/G3/G4
/// physical/operation proof remain separate.
/// Explicit empty families supply no whole Design or assembly readiness claim.
///
/// ```compile_fail
/// fn edit(value: &mut sc_core::ontology::LayerInventory) { value.facings.clear(); }
/// ```
/// ```compile_fail
/// fn edit(value: &mut sc_core::ontology::LayerInventory) { value.linings().get_mut(0); }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct LayerInventory {
    facings: Vec<Facing>,
    linings: Vec<Lining>,
    interfacings: Vec<Interfacing>,
}
impl fmt::Debug for LayerInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LayerInventory")
            .field("facing_count", &self.facings.len())
            .field("lining_count", &self.linings.len())
            .field("interfacing_count", &self.interfacings.len())
            .finish_non_exhaustive()
    }
}
impl LayerInventory {
    pub(crate) fn resource_layers(&self) -> (&[Facing], &[Lining], &[Interfacing]) {
        let Self {
            facings,
            linings,
            interfacings,
        } = self;
        (facings, linings, interfacings)
    }
    /// Own exact family orders without sorting, choosing or repairing held sources.
    /// # Errors
    /// Returns the first duplicate layer id across the three families with both original positions.
    pub fn new(
        facings: Vec<Facing>,
        linings: Vec<Lining>,
        interfacings: Vec<Interfacing>,
    ) -> Result<Self, LayerInventoryError> {
        let layers = Self {
            facings,
            linings,
            interfacings,
        };
        let mut seen = BTreeMap::new();
        for (duplicate, layer) in layers.entries() {
            if let Some(first) = seen.insert(layer.id(), duplicate) {
                return Err(LayerInventoryError::DuplicateIdentity {
                    identity: layer.id(),
                    first,
                    duplicate,
                });
            }
        }
        Ok(layers)
    }
    /// Exact original Facing order, without current-source certification.
    #[must_use]
    pub fn facings(&self) -> &[Facing] {
        &self.facings
    }
    /// Exact original modelled Lining order, without execution approval.
    #[must_use]
    pub fn linings(&self) -> &[Lining] {
        &self.linings
    }
    /// Exact original Interfacing order, without current-source certification.
    #[must_use]
    pub fn interfacings(&self) -> &[Interfacing] {
        &self.interfacings
    }
    /// Borrow the exact original Facing target; another layer kind never substitutes for it.
    #[must_use]
    pub fn facing(&self, identity: EntityId) -> Option<&Facing> {
        self.facings.iter().find(|value| value.id() == identity)
    }
    /// Borrow original typed entries in Facing/Lining/Interfacing order, then authored family order.
    pub fn entries(&self) -> impl Iterator<Item = (LayerInventoryPosition, LayerRef<'_>)> {
        self.facings
            .iter()
            .enumerate()
            .map(|(index, value)| {
                (
                    LayerInventoryPosition {
                        kind: LayerKind::Facing,
                        index,
                    },
                    LayerRef::Facing(value),
                )
            })
            .chain(self.linings.iter().enumerate().map(|(index, value)| {
                (
                    LayerInventoryPosition {
                        kind: LayerKind::Lining,
                        index,
                    },
                    LayerRef::Lining(value),
                )
            }))
            .chain(self.interfacings.iter().enumerate().map(|(index, value)| {
                (
                    LayerInventoryPosition {
                        kind: LayerKind::Interfacing,
                        index,
                    },
                    LayerRef::Interfacing(value),
                )
            }))
    }
    /// Check unambiguous current Pieces and every layer's original served target/full sources.
    /// Modelled Lining is validated as data; this method supplies no execution-scope approval.
    /// # Errors
    /// Returns the first duplicate Piece, then first original layer refusal in documented order.
    pub fn validate_current(
        &self,
        pieces: &[Piece],
        ledger: &IdentityLedger,
    ) -> Result<(), LayerInventoryError> {
        let mut current = BTreeMap::new();
        for piece in pieces {
            if current.insert(piece.id(), piece).is_some() {
                return Err(LayerInventoryError::DuplicatePiece(piece.id()));
            }
        }
        for (position, layer) in self.entries() {
            let served = current.get(&layer.served_piece()).ok_or(
                LayerInventoryError::MissingServedPiece {
                    layer: layer.id(),
                    position,
                    piece: layer.served_piece(),
                },
            )?;
            layer
                .validate_current(served, ledger)
                .map_err(|issue| LayerInventoryError::Layer {
                    layer: layer.id(),
                    position,
                    issue: Box::new(issue),
                })?;
        }
        Ok(())
    }
    /// Borrow every actual layer's exact Material target or original unresolved reason.
    ///
    /// Checks complete current Materials first, then all layer/Material owner-id collisions, then
    /// assignments in documented family order. Unknown/Derived properties retain their canonical
    /// states without defaulting. This checks assignment metadata, not current source geometry,
    /// lining execution scope, other global Design identities or physical/release readiness.
    ///
    /// # Errors
    /// Returns the original whole-Material cause, then owner-id collision or exact missing target.
    /// No partial view collection escapes a refusal.
    pub fn material_assignments<'a>(
        &'a self,
        materials: &'a MaterialInventory,
        canonical: &'a CanonicalInputs,
    ) -> Result<Vec<LayerMaterialView<'a>>, LayerInventoryError> {
        materials
            .validate_current(canonical)
            .map_err(|issue| LayerInventoryError::Materials(Box::new(issue)))?;
        for (position, layer) in self.entries() {
            if materials.material(layer.id()).is_some() {
                return Err(LayerInventoryError::MaterialIdentityCollision {
                    layer: layer.id(),
                    position,
                });
            }
        }
        self.entries()
            .map(|(position, layer)| match &layer.definition().material {
                MaterialAssignment::Assigned(identity) => {
                    let material = materials.material(*identity).ok_or(
                        LayerInventoryError::MissingMaterial {
                            layer: layer.id(),
                            position,
                            material: *identity,
                        },
                    )?;
                    Ok(LayerMaterialView::Assigned { layer, material })
                }
                MaterialAssignment::Unresolved(reason) => {
                    Ok(LayerMaterialView::Unresolved { layer, reason })
                }
            })
            .collect()
    }
    /// Apply original named scope checks before requested execution, independently of geometry.
    /// Modelled content remains available through shared getters after this refusal.
    /// # Errors
    /// Returns the first modelled Lining's original env_lining cause, identity and position.
    pub fn require_in_scope(&self) -> Result<(), LayerInventoryError> {
        for (position, layer) in self.entries() {
            layer
                .require_in_scope()
                .map_err(|issue| LayerInventoryError::OutsideEnvelope {
                    layer: layer.id(),
                    position,
                    issue,
                })?;
        }
        Ok(())
    }
}
