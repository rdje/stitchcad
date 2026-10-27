//! Exact implemented garment ownership over one actual sewing/input root; currentness is separate.
use super::{
    DesignInputs, DesignInputsError, DesignReferenceHost, DesignSewing, DesignSewingError,
    DesignSewingPosition,
};
use crate::ontology::{
    ClosureInventory, ClosureInventoryError, ClosureInventoryPosition, ConstructionInventory,
    ConstructionInventoryError, ConstructionInventoryPosition, EntityId, HemInventory,
    HemInventoryError, LayerInventory, LayerInventoryError, LayerInventoryPosition,
    NotionPlacementInventory, NotionPlacementInventoryError, PocketInventory, PocketInventoryError,
};
use core::fmt;
use std::collections::BTreeMap;

/// Exact implemented garment components; references are never reconstructed against a later ledger.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignGarmentDefinition<H> {
    /// Actual sole sewing/input/canonical owner, with complete semantic landmark inventory.
    pub sewing: DesignSewing<H>,
    /// Original physical Pocket targets, placements and composition intent.
    pub pockets: PocketInventory,
    /// Complete original physical notion placements, including unused records.
    pub notions: NotionPlacementInventory,
    /// Complete original Closure/instance ownership and logical sizing intent.
    pub closures: ClosureInventory,
    /// Distinct original Facing/modelled Lining/Interfacing objects.
    pub layers: LayerInventory,
    /// Original Hem finish and current Facing intent.
    pub hems: HemInventory,
    /// Distinct original Dart/Tuck/Pleat/Gather/Grainline/SeamAllowance objects.
    pub constructions: ConstructionInventory,
}
/// Actual original ownership location; referenced Piece/copy/parameter/Profile/operation ids are uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignGarmentPosition {
    /// Original complete input/sewing position, including retired creating operations.
    Sewing(DesignSewingPosition),
    /// Original authored Pocket position.
    Pocket {
        /// Zero-based original index.
        index: usize,
    },
    /// Original authored physical notion-placement position.
    NotionPlacement {
        /// Zero-based original index.
        index: usize,
    },
    /// Original Closure or nested instance position; instances own identities independently.
    Closure(ClosureInventoryPosition),
    /// Original distinct layer family/authored position.
    Layer(LayerInventoryPosition),
    /// Original authored Hem position.
    Hem {
        /// Zero-based original index.
        index: usize,
    },
    /// Original distinct construction family/authored position.
    Construction(ConstructionInventoryPosition),
}
/// Original complete ownership/current-input/source cause, preserving unmodified typed evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignGarmentError<E> {
    /// Independently valid components claim the same actual owned identity.
    DuplicateIdentity {
        /// Repeated owned identity.
        identity: EntityId,
        /// First actual owning location.
        first: DesignGarmentPosition,
        /// First repeated original owning location.
        duplicate: DesignGarmentPosition,
    },
    /// Original complete current root-input cause, preserving the concrete host chain.
    Inputs(Box<DesignInputsError<E>>),
    /// Complete current sewing/root/landmark/graph/amount cause, including unused sources.
    Sewing(Box<DesignSewingError<E>>),
    /// Original complete Pocket/current physical component interval cause.
    Pockets(Box<PocketInventoryError>),
    /// Original complete physical notion-placement cause, including unused placements.
    Notions(Box<NotionPlacementInventoryError>),
    /// Original complete Closure/instance/placement-target cause.
    Closures(Box<ClosureInventoryError>),
    /// Original complete layer/source/Material assignment cause, including unused layers.
    Layers(Box<LayerInventoryError>),
    /// Original complete Hem finish/current Facing cause.
    Hems(Box<HemInventoryError>),
    /// Original complete current six-family construction source cause.
    Constructions(Box<ConstructionInventoryError>),
}
impl<E: fmt::Display> fmt::Display for DesignGarmentError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity {
                identity,
                first,
                duplicate,
            } => write!(
                f,
                "garment identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"
            ),
            Self::Inputs(issue) => write!(f, "garment current root inputs: {issue}"),
            Self::Sewing(issue) => write!(f, "garment current sewing: {issue}"),
            Self::Pockets(issue) => write!(f, "garment current Pockets: {issue}"),
            Self::Notions(issue) => write!(f, "garment current notions: {issue}"),
            Self::Closures(issue) => write!(f, "garment current Closures: {issue}"),
            Self::Layers(issue) => write!(f, "garment current layers: {issue}"),
            Self::Hems(issue) => write!(f, "garment current Hems: {issue}"),
            Self::Constructions(issue) => write!(f, "garment current constructions: {issue}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignGarmentError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } => None,
            Self::Inputs(issue) => Some(issue.as_ref()),
            Self::Sewing(issue) => Some(issue.as_ref()),
            Self::Pockets(issue) => Some(issue.as_ref()),
            Self::Notions(issue) => Some(issue.as_ref()),
            Self::Closures(issue) => Some(issue.as_ref()),
            Self::Layers(issue) => Some(issue.as_ref()),
            Self::Hems(issue) => Some(issue.as_ref()),
            Self::Constructions(issue) => Some(issue.as_ref()),
        }
    }
}
/// Private exact garment ownership; held damage/empty draft scope stays available for inspection.
///
/// Admission checks every owned id, then current root inputs. It does not certify current garment
/// targets/sources; use `validate_current` for structural source/assignment checks. Declaration roles,
/// requested execution scope, geometry, global completeness and physical/release readiness stay
/// separate. Modelled Lining/Fly retain their execution deferrals.
/// ```compile_fail
/// fn edit<H>(value: &mut sc_core::design::DesignGarment<H>, hems: sc_core::ontology::HemInventory) {
///     value.definition.hems = hems;
/// }
/// ```
/// ```compile_fail
/// fn edit<H: sc_core::design::DesignReferenceHost>(value: &mut sc_core::design::DesignGarment<H>) {
///     let _: &mut sc_core::ontology::ConstructionInventory = value.constructions();
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct DesignGarment<H> {
    definition: DesignGarmentDefinition<H>,
}
impl<H> fmt::Debug for DesignGarment<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignGarment")
            .field("pocket_count", &self.definition.pockets.pockets().len())
            .field("notion_count", &self.definition.notions.placements().len())
            .field("closure_count", &self.definition.closures.closures().len())
            .field("layer_count", &self.definition.layers.entries().count())
            .field("hem_count", &self.definition.hems.hems().len())
            .field(
                "construction_count",
                &self.definition.constructions.entries().count(),
            )
            .finish_non_exhaustive()
    }
}
impl<H: DesignReferenceHost> DesignGarment<H> {
    /// Own exact components without repairing, sorting or reconstructing held semantic objects.
    /// # Errors
    /// Returns the first complete ownership collision, then original current root-input cause.
    pub fn new(
        definition: DesignGarmentDefinition<H>,
    ) -> Result<Self, DesignGarmentError<H::Error>> {
        let state = Self { definition };
        state.validate_owned_inputs()?;
        Ok(state)
    }
    /// Exact original definition; shared access grants no replacement or mutator.
    #[must_use]
    pub const fn definition(&self) -> &DesignGarmentDefinition<H> {
        &self.definition
    }
    /// Actual sole sewing owner, without a current-source certificate.
    #[must_use]
    pub const fn sewing(&self) -> &DesignSewing<H> {
        &self.definition.sewing
    }
    /// Actual original single input/canonical owner, never another stored copy.
    #[must_use]
    pub const fn inputs(&self) -> &DesignInputs<H> {
        self.sewing().inputs()
    }
    /// Complete original Pocket inventory.
    #[must_use]
    pub const fn pockets(&self) -> &PocketInventory {
        &self.definition.pockets
    }
    /// Complete original notion-placement inventory, including unused placements.
    #[must_use]
    pub const fn notions(&self) -> &NotionPlacementInventory {
        &self.definition.notions
    }
    /// Complete original Closure/instance inventory.
    #[must_use]
    pub const fn closures(&self) -> &ClosureInventory {
        &self.definition.closures
    }
    /// Complete original distinct layer families, including modelled Lining.
    #[must_use]
    pub const fn layers(&self) -> &LayerInventory {
        &self.definition.layers
    }
    /// Complete original Hem inventory.
    #[must_use]
    pub const fn hems(&self) -> &HemInventory {
        &self.definition.hems
    }
    /// Complete original distinct construction families.
    #[must_use]
    pub const fn constructions(&self) -> &ConstructionInventory {
        &self.definition.constructions
    }
    /// Complete ownership in sewing, Pocket, notion, Closure/instance, layer, Hem, construction order.
    /// Each family keeps its original authored order; uses never become additional owned identities.
    #[must_use]
    pub fn identities(&self) -> Vec<(EntityId, DesignGarmentPosition)> {
        self.sewing()
            .identities()
            .into_iter()
            .map(|(identity, position)| (identity, DesignGarmentPosition::Sewing(position)))
            .chain(
                self.pockets()
                    .pockets()
                    .iter()
                    .enumerate()
                    .map(|(index, value)| (value.id(), DesignGarmentPosition::Pocket { index })),
            )
            .chain(
                self.notions()
                    .placements()
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        (value.id(), DesignGarmentPosition::NotionPlacement { index })
                    }),
            )
            .chain(
                self.closures()
                    .closures()
                    .iter()
                    .enumerate()
                    .flat_map(|(index, value)| {
                        core::iter::once((
                            value.id(),
                            DesignGarmentPosition::Closure(ClosureInventoryPosition::Closure {
                                index,
                            }),
                        ))
                        .chain(
                            value.definition().instances.iter().enumerate().map(
                                move |(instance, value)| {
                                    (
                                        value.id,
                                        DesignGarmentPosition::Closure(
                                            ClosureInventoryPosition::Instance {
                                                closure: index,
                                                index: instance,
                                            },
                                        ),
                                    )
                                },
                            ),
                        )
                    }),
            )
            .chain(
                self.layers()
                    .entries()
                    .map(|(position, value)| (value.id(), DesignGarmentPosition::Layer(position))),
            )
            .chain(
                self.hems()
                    .hems()
                    .iter()
                    .enumerate()
                    .map(|(index, value)| (value.id(), DesignGarmentPosition::Hem { index })),
            )
            .chain(self.constructions().entries().map(|(position, value)| {
                (value.id(), DesignGarmentPosition::Construction(position))
            }))
            .collect()
    }
    /// Revalidate all owned identities/current inputs, sewing and every actual garment source.
    ///
    /// Order is complete ownership/inputs, sewing, Pockets, notions, Closures, layers and Material
    /// assignments, Hems, then constructions. Original validators include unused objects and complete
    /// supplied contexts even for empty families. Raw repair/choice evidence stays unchanged;
    /// held objects are never repaired, retargeted or cached as valid. Unresolved Material assignments
    /// stay unresolved and logical Profile roles stay deferred. This supplies current structural
    /// source/assignment validity only: complete declaration roles, operation execution, requested
    /// envelope, geometry, resolved physical values and release readiness remain separate.
    /// # Errors
    /// Returns the first original contextual cause in documented order without partial success.
    pub fn validate_current(&self) -> Result<(), DesignGarmentError<H::Error>> {
        self.validate_owned_inputs()?;
        self.sewing()
            .validate_current()
            .map_err(|issue| DesignGarmentError::Sewing(Box::new(issue)))?;
        let structure = self.inputs().structure();
        let pieces = structure.pieces().pieces();
        let plan = structure.cut_plan();
        let ledger = structure.topology();
        self.pockets()
            .validate_current(plan, pieces, ledger)
            .map_err(|issue| DesignGarmentError::Pockets(Box::new(issue)))?;
        self.notions()
            .validate_current(plan, pieces, ledger)
            .map_err(|issue| DesignGarmentError::Notions(Box::new(issue)))?;
        self.closures()
            .validate_current(self.notions(), plan, pieces, ledger)
            .map_err(|issue| DesignGarmentError::Closures(Box::new(issue)))?;
        self.layers()
            .validate_current(pieces, ledger)
            .map_err(|issue| DesignGarmentError::Layers(Box::new(issue)))?;
        self.layers()
            .material_assignments(self.inputs().materials(), self.inputs().inputs())
            .map_err(|issue| DesignGarmentError::Layers(Box::new(issue)))?;
        self.hems()
            .validate_current(self.layers(), pieces, ledger)
            .map_err(|issue| DesignGarmentError::Hems(Box::new(issue)))?;
        self.constructions()
            .validate_current(
                pieces,
                plan,
                self.sewing().graph(),
                self.sewing().landmarks(),
                ledger,
            )
            .map_err(|issue| DesignGarmentError::Constructions(Box::new(issue)))?;
        Ok(())
    }
    fn validate_owned_inputs(&self) -> Result<(), DesignGarmentError<H::Error>> {
        let mut seen = BTreeMap::new();
        for (identity, duplicate) in self.identities() {
            if let Some(first) = seen.insert(identity, duplicate) {
                return Err(DesignGarmentError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate,
                });
            }
        }
        self.inputs()
            .validate_current()
            .map_err(|issue| DesignGarmentError::Inputs(Box::new(issue)))
    }
}
