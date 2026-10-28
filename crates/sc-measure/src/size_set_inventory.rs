//! Exact external set inventory and Design-reference checks over actual current shared inputs.
use crate::{
    MeasurementInputs, MeasurementInputsError, MeasurementInputsPosition, SizeSet, SizeSetError,
    SizeSetPosition, SizeSetReference,
};
use core::fmt;
use sc_core::{design::DesignSizeSetReference, ontology::EntityId, value::CanonicalInputs};
use std::collections::BTreeMap;

/// Actual original position across shared inputs and complete external set source owners.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeSetInventoryPosition {
    /// Actual shared canonical/measurement/Ease ownership location.
    Inputs(MeasurementInputsPosition),
    /// Actual complete owned set source location.
    Set {
        /// Zero-based authored set position.
        index: usize,
        /// Original family/source location inside this set.
        position: SizeSetPosition,
    },
}

/// Current complete inventory or exact selected reference refusal, without label/latest fallback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeSetInventoryError {
    /// Actual owned entities collide, including cross-set entities with different set identities.
    DuplicateIdentity {
        /// Repeated entity identity.
        identity: EntityId,
        /// First original position; boxed to keep the common Result small.
        first: Box<SizeSetInventoryPosition>,
        /// First repeated original position.
        duplicate: Box<SizeSetInventoryPosition>,
    },
    /// Actual shared measurement/Ease inputs fail current validation, even for an empty set inventory.
    Inputs(Box<MeasurementInputsError>),
    /// One actual owned set fails its original family current validator.
    InvalidSet {
        /// Actual offending set identity/revision.
        set: SizeSetReference,
        /// Original concrete family cause.
        issue: Box<SizeSetError>,
    },
    /// The exact requested set id is absent; no similarly labelled/default set is selected.
    MissingSet {
        /// Exact Design-authored reference.
        expected: DesignSizeSetReference,
    },
    /// Actual named set has a different revision; no latest/current revision is substituted.
    RevisionMismatch {
        /// Exact Design-authored reference.
        expected: DesignSizeSetReference,
        /// Actual named set's identity/revision.
        actual: DesignSizeSetReference,
    },
}
impl fmt::Display for SizeSetInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity {identity,first,duplicate} => write!(f,"SizeSet inventory identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::Inputs(issue) => write!(f,"SizeSet inventory inputs: {issue}"),
            Self::InvalidSet {set,issue} => write!(f,"SizeSet inventory {set:?}: {issue}"),
            Self::MissingSet {expected} => write!(f,"SizeSet inventory has no set for {expected:?}"),
            Self::RevisionMismatch {expected,actual} => write!(f,"SizeSet inventory reference {expected:?} differs from {actual:?}"),
        }
    }
}
impl std::error::Error for SizeSetInventoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Inputs(issue) => Some(issue.as_ref()),
            Self::InvalidSet { issue, .. } => Some(issue.as_ref()),
            _ => None,
        }
    }
}

/// Private immutable ordered external set inventory, separate from a Design's references to it.
///
/// All actual set sources remain owned exactly once here. Shared canonical declarations and
/// measurement/Ease inputs are borrowed on current queries, never cached or copied as values.
///
/// ```compile_fail
/// fn clear(inventory: &mut sc_measure::SizeSetInventory) { inventory.sets.clear(); }
/// ```
/// ```compile_fail
/// fn edit(inventory: &mut sc_measure::SizeSetInventory) {
///     let _: &mut [sc_measure::SizeSet] = inventory.sets();
/// }
/// ```
/// ```compile_fail
/// fn replace(inventory: &sc_measure::SizeSetInventory, reference: sc_core::design::DesignSizeSetReference,
///     canonical: &mut sc_core::value::CanonicalInputs, replacement: &mut sc_core::value::CanonicalInputs,
///     inputs: &sc_measure::MeasurementInputs) {
///     if let Ok(set) = inventory.validate_reference(reference, canonical, inputs) {
///         core::mem::swap(canonical, replacement);
///         let _ = set.reference();
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct SizeSetInventory {
    sets: Vec<SizeSet>,
}
impl fmt::Debug for SizeSetInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SizeSetInventory")
            .field("set_count", &self.sets.len())
            .finish_non_exhaustive()
    }
}
impl SizeSetInventory {
    pub(crate) fn resource_sets(&self) -> &[SizeSet] {
        let Self { sets } = self;
        sets
    }
    /// Own exact authored sets after complete source identity/current validation. Empty drafts are legal.
    ///
    /// # Errors
    /// Returns the first whole inventory refusal described by validate_current.
    pub fn new(
        sets: Vec<SizeSet>,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<Self, SizeSetInventoryError> {
        let inventory = Self { sets };
        inventory.validate_current(canonical, inputs)?;
        Ok(inventory)
    }
    /// Exact original order; shared access supplies no mutable inventory.
    #[must_use]
    pub fn sets(&self) -> &[SizeSet] {
        &self.sets
    }
    /// Inspect an actual owned set by identity; this raw shared query grants no current-context proof.
    #[must_use]
    pub fn size_set(&self, identity: EntityId) -> Option<&SizeSet> {
        self.sets.iter().find(|set| set.reference().id == identity)
    }
    /// Check all shared/current sources, then every complete actual set in authored order.
    ///
    /// Census order is shared canonical/measurement/Ease records then each set's complete sources.
    /// References are uses, not repeated declarations. Identity checks precede reference validation.
    /// No numeric evaluation, chart coverage or release readiness is inferred.
    ///
    /// # Errors
    /// Returns first owned-id collision, shared input cause, then first actual invalid set cause.
    pub fn validate_current(
        &self,
        canonical: &CanonicalInputs,
        inputs: &MeasurementInputs,
    ) -> Result<(), SizeSetInventoryError> {
        let mut seen = BTreeMap::new();
        let positions = inputs
            .identity_positions(canonical)
            .map(|(id, position)| (id, SizeSetInventoryPosition::Inputs(position)))
            .chain(self.sets.iter().enumerate().flat_map(|(index, set)| {
                set.identities().into_iter().map(move |(id, position)| {
                    (id, SizeSetInventoryPosition::Set { index, position })
                })
            }));
        for (identity, position) in positions {
            if let Some(first) = seen.insert(identity, position) {
                return Err(SizeSetInventoryError::DuplicateIdentity {
                    identity,
                    first: Box::new(first),
                    duplicate: Box::new(position),
                });
            }
        }
        inputs
            .validate_current(canonical)
            .map_err(|issue| SizeSetInventoryError::Inputs(Box::new(issue)))?;
        for set in &self.sets {
            set.validate_current(canonical, inputs).map_err(|issue| {
                SizeSetInventoryError::InvalidSet {
                    set: set.reference(),
                    issue: Box::new(issue),
                }
            })?;
        }
        Ok(())
    }
    /// Validate this whole current inventory, then borrow the exact Design-named set/revision.
    /// Returned access retains the inventory and both current input borrows; presence is no path proof.
    ///
    /// # Errors
    /// Returns whole current refusal before missing-id/revision mismatch. No label or latest fallback.
    pub fn validate_reference<'a>(
        &'a self,
        expected: DesignSizeSetReference,
        canonical: &'a CanonicalInputs,
        inputs: &'a MeasurementInputs,
    ) -> Result<&'a SizeSet, SizeSetInventoryError> {
        self.validate_current(canonical, inputs)?;
        let set = self
            .size_set(expected.id())
            .ok_or(SizeSetInventoryError::MissingSet { expected })?;
        let actual = set.reference().into();
        if actual != expected {
            return Err(SizeSetInventoryError::RevisionMismatch { expected, actual });
        }
        Ok(set)
    }
}
