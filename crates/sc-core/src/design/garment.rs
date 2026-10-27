//! Exact implemented garment ownership over one actual sewing/input root; currentness is separate.
use super::{
    DesignInputs, DesignInputsError, DesignReferenceHost, DesignSewing, DesignSewingPosition,
};
use crate::ontology::{
    ClosureInventory, ClosureInventoryPosition, ConstructionInventory,
    ConstructionInventoryPosition, EntityId, HemInventory, LayerInventory, LayerInventoryPosition,
    NotionPlacementInventory, PocketInventory,
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
/// Original complete ownership or current root-input refusal; admission supplies no source proof.
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
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for DesignGarmentError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } => None,
            Self::Inputs(issue) => Some(issue.as_ref()),
        }
    }
}
/// Private exact garment ownership; held damage/empty draft scope stays available for inspection.
///
/// Admission checks every owned id, then current root inputs. It does not certify current garment
/// targets/sources/declarations, requested execution scope, geometry, global feature completeness or
/// physical/release readiness. Modelled Lining/Fly retain their separate execution deferrals.
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
