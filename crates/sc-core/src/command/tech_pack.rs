//! Canonical tech-pack source requests, without copied report content or fabricated Order quantities.
use super::EvaluationProfile;
use crate::ontology::EntityId;
use core::fmt;
use std::collections::BTreeMap;

/// Nonempty, distinct immutable Instance identities in authored report order, before provenance lookup.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstanceSelection(Vec<EntityId>);
/// Invalid source inventory, retaining the original repeated identity and positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceSelectionError {
    /// No evaluated source was supplied.
    NoInstances,
    /// First repeated Instance in the authored vector, with zero-based positions.
    DuplicateInstance {
        /// Actual repeated identity.
        instance: EntityId,
        /// Original position.
        first: usize,
        /// First repeated position.
        duplicate: usize,
    },
}
impl fmt::Display for InstanceSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInstances => f.write_str("instance selection requires at least one instance"),
            Self::DuplicateInstance {
                instance,
                first,
                duplicate,
            } => {
                write!(f,
                "instance {instance} repeats at position {duplicate}, first supplied at {first}")
            }
        }
    }
}
impl std::error::Error for InstanceSelectionError {}
impl InstanceSelection {
    /// Preserve explicit identities and order; do not evaluate a Design or select its latest instances.
    /// # Errors
    /// Returns an empty-inventory refusal or the first actual repeated Instance identity.
    pub fn new(instances: Vec<EntityId>) -> Result<Self, InstanceSelectionError> {
        if instances.is_empty() {
            return Err(InstanceSelectionError::NoInstances);
        }
        let mut positions = BTreeMap::new();
        for (duplicate, instance) in instances.iter().copied().enumerate() {
            if let Some(first) = positions.insert(instance, duplicate) {
                return Err(InstanceSelectionError::DuplicateInstance {
                    instance,
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self(instances))
    }
    /// Original Instance identities; actual snapshot/size/profile provenance must agree at execution.
    #[must_use]
    pub fn instances(&self) -> &[EntityId] {
        &self.0
    }
}

/// Generate structured tech-pack content from the pinned Design and actual evaluated Instances.
/// Design owns authored materials/trims/notes/stitch/POM knowledge; this request carries no report edits.
/// Piece cut quantities come from cut plans. Order size-run quantities are unavailable in v1 and must
/// remain unresolved when needed, never inferred from membership, report order or piece multiplicity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenerateTechPack {
    instances: InstanceSelection,
    profile: EvaluationProfile,
}
impl GenerateTechPack {
    /// Preserve actual source selections. Execution must validate provenance and required content.
    /// Constructing this request neither supplies missing knowledge nor allocates a report or Order.
    #[must_use]
    pub const fn new(instances: InstanceSelection, profile: EvaluationProfile) -> Self {
        Self { instances, profile }
    }
    /// Explicit immutable evaluated sources, in the requested report order.
    #[must_use]
    pub const fn instances(&self) -> &InstanceSelection {
        &self.instances
    }
    /// Explicit logical/factory input selection, without a default profile or authority grant.
    #[must_use]
    pub const fn profile(&self) -> &EvaluationProfile {
        &self.profile
    }
}
