//! Owned measurement/Ease composition over explicit borrowed canonical values.
use crate::{
    Ease, EaseError, EaseSet, EaseSetContext, EaseSetError, MeasurementError, MeasurementInventory,
    MeasurementInventoryError, MeasurementInventoryPosition, MeasurementTableContext,
    MeasurementTableError,
};
use core::fmt;
use sc_core::{ontology::EntityId, value::CanonicalInputs};
use std::collections::BTreeMap;

/// Exact metadata/Ease source objects; canonical declarations remain singly owned elsewhere.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementInputsDefinition {
    /// Actual owned landmark/procedure/measurement/table inventory.
    pub measurements: MeasurementInventory,
    /// Actual immutable mappings, including mappings not selected by any Ease set.
    pub eases: Vec<Ease>,
    /// Actual immutable set bindings and explicit table identities.
    pub ease_sets: Vec<EaseSet>,
}

/// Original position in the combined census, with no invented owning record for references.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeasurementInputsPosition {
    /// Inherited exact canonical-input or owned metadata position.
    Metadata(MeasurementInventoryPosition),
    /// Owned Ease mapping.
    Ease {
        /// Zero-based authored mapping position.
        index: usize,
    },
    /// Owned Ease set.
    EaseSet {
        /// Zero-based authored set position.
        index: usize,
    },
}

/// Actual current composition refusal; nested source objects retain their original typed causes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MeasurementInputsError {
    /// Repeated identity, checked before validating current targets or permission.
    DuplicateIdentity {
        /// Repeated entity id.
        identity: EntityId,
        /// First occurrence.
        first: MeasurementInputsPosition,
        /// First repeated occurrence.
        duplicate: MeasurementInputsPosition,
    },
    /// Actual current metadata inventory is invalid.
    Measurements(Box<MeasurementInventoryError>),
    /// Existing target-context constructor refused.
    RecordContext(Box<MeasurementError>),
    /// Existing measurement-context constructor refused.
    TableContext(Box<MeasurementTableError>),
    /// Actual mapping's current targets or present amount/permission are invalid.
    Ease {
        /// Mapping carrying the failed obligation.
        ease: EntityId,
        /// Original typed cause.
        issue: Box<EaseError>,
    },
    /// Existing mapping/table context constructor refused.
    EaseContext(Box<EaseSetError>),
    /// Actual set's current mapping/table targets or membership are invalid.
    EaseSet {
        /// Set carrying the failed obligation.
        set: EntityId,
        /// Original typed cause.
        issue: Box<EaseSetError>,
    },
}
impl fmt::Display for MeasurementInputsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f, "measurement input identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::Measurements(issue) => write!(f, "measurement inputs: {issue}"),
            Self::RecordContext(issue) => write!(f, "measurement input records: {issue}"),
            Self::TableContext(issue) => write!(f, "measurement input tables: {issue}"),
            Self::Ease { ease, issue } => write!(f, "measurement input Ease {ease}: {issue}"),
            Self::EaseContext(issue) => write!(f, "measurement input Ease context: {issue}"),
            Self::EaseSet { set, issue } => write!(f, "measurement input Ease set {set}: {issue}"),
        }
    }
}
impl std::error::Error for MeasurementInputsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } => None,
            Self::Measurements(issue) => Some(issue.as_ref()),
            Self::RecordContext(issue) => Some(issue.as_ref()),
            Self::TableContext(issue) => Some(issue.as_ref()),
            Self::Ease { issue, .. } => Some(issue.as_ref()),
            Self::EaseContext(issue) | Self::EaseSet { issue, .. } => Some(issue.as_ref()),
        }
    }
}

/// Immutable actual metadata, mapping and set inventory validated against canonical current values.
///
/// This composes structural sources without selecting a Design, SizeSet or profile, computing an
/// Ease, evaluating a derived amount, proving evidence truth or authorizing geometry/release.
///
/// ```compile_fail
/// fn remove(inputs: &mut sc_measure::MeasurementInputs) { inputs.definition.eases.clear(); }
/// ```
/// ```compile_fail
/// fn edit(inputs: &mut sc_measure::MeasurementInputs) { inputs.definition().ease_sets.clear(); }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct MeasurementInputs {
    definition: MeasurementInputsDefinition,
}
impl fmt::Debug for MeasurementInputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MeasurementInputs")
            .field("measurements", &self.definition.measurements)
            .field("ease_count", &self.definition.eases.len())
            .field("ease_set_count", &self.definition.ease_sets.len())
            .finish_non_exhaustive()
    }
}
impl MeasurementInputs {
    /// Own exact source objects after complete identity/metadata/mapping/set validation.
    /// Empty inventories are legal; authored empty Ease sets still require their explicit tables.
    ///
    /// # Errors
    /// Returns the first actual combined refusal described by validate_current, without partial state.
    pub fn new(
        definition: MeasurementInputsDefinition,
        canonical: &CanonicalInputs,
    ) -> Result<Self, MeasurementInputsError> {
        let inputs = Self { definition };
        inputs.validate_current(canonical)?;
        Ok(inputs)
    }

    /// Original exact definition; shared access grants no mutator.
    #[must_use]
    pub const fn definition(&self) -> &MeasurementInputsDefinition {
        &self.definition
    }

    /// Actual owned measurement metadata, without another copied inventory.
    #[must_use]
    pub const fn measurements(&self) -> &MeasurementInventory {
        &self.definition.measurements
    }

    /// Borrow actual canonical mapping metadata by identity.
    #[must_use]
    pub fn ease(&self, identity: EntityId) -> Option<&Ease> {
        self.definition
            .eases
            .iter()
            .find(|ease| ease.id() == identity)
    }

    /// Borrow actual canonical set metadata by identity.
    #[must_use]
    pub fn ease_set(&self, identity: EntityId) -> Option<&EaseSet> {
        self.definition
            .ease_sets
            .iter()
            .find(|set| set.id() == identity)
    }

    pub(crate) fn identity_positions<'a>(
        &'a self,
        canonical: &'a CanonicalInputs,
    ) -> impl Iterator<Item = (EntityId, MeasurementInputsPosition)> + 'a {
        self.measurements()
            .identity_positions(canonical)
            .map(|(id, position)| (id, MeasurementInputsPosition::Metadata(position)))
            .chain(
                self.definition
                    .eases
                    .iter()
                    .enumerate()
                    .map(|(index, ease)| (ease.id(), MeasurementInputsPosition::Ease { index })),
            )
            .chain(
                self.definition
                    .ease_sets
                    .iter()
                    .enumerate()
                    .map(|(index, set)| (set.id(), MeasurementInputsPosition::EaseSet { index })),
            )
    }

    /// Validate the whole actual inventory against these explicit current canonical declarations.
    ///
    /// Census order extends MeasurementInventory's exact positions with mappings then sets, each
    /// authored. Then all metadata, every mapping (even unused), and every set are checked in order
    /// by their existing validators. Context owners are local borrows, never stored self-references.
    /// Unknown/Derived amounts gain no fallback. Present negative amounts need explicit permission;
    /// future evaluated results still require the mapping's separate amount check and v1 envelope.
    ///
    /// # Errors
    /// Returns the first identity collision, then original metadata/mapping/set or context cause.
    pub fn validate_current(
        &self,
        canonical: &CanonicalInputs,
    ) -> Result<(), MeasurementInputsError> {
        let mut seen = BTreeMap::new();
        for (identity, position) in self.identity_positions(canonical) {
            if let Some(first) = seen.insert(identity, position) {
                return Err(MeasurementInputsError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate: position,
                });
            }
        }
        self.measurements()
            .validate_current(canonical)
            .map_err(|issue| MeasurementInputsError::Measurements(Box::new(issue)))?;
        let records = self
            .measurements()
            .record_context(canonical)
            .map_err(|issue| MeasurementInputsError::RecordContext(Box::new(issue)))?;
        let measurements = self.measurements().definition();
        let tables = MeasurementTableContext::new(&measurements.measurements, &records)
            .map_err(|issue| MeasurementInputsError::TableContext(Box::new(issue)))?;
        for ease in &self.definition.eases {
            ease.validate_current(&tables)
                .map_err(|issue| MeasurementInputsError::Ease {
                    ease: ease.id(),
                    issue: Box::new(issue),
                })?;
        }
        let context = EaseSetContext::new(&measurements.tables, &self.definition.eases, &tables)
            .map_err(|issue| MeasurementInputsError::EaseContext(Box::new(issue)))?;
        for set in &self.definition.ease_sets {
            set.validate_current(&context)
                .map_err(|issue| MeasurementInputsError::EaseSet {
                    set: set.id(),
                    issue: Box::new(issue),
                })?;
        }
        Ok(())
    }
}
