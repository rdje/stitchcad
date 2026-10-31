//! Explicit logical fragment candidates and original source domains;no geometric position map.
use super::BoundOffsetConstruction;
use crate::ontology::{DirectedRange, EdgeRef, EntityId, LocalTag, OffsetInterval};
use core::fmt;
use std::collections::BTreeMap;
mod declaration;
mod presence;
mod query;
pub use declaration::{
    DeclaredOffsetCandidate, DeclaredOffsetCandidates, OffsetDeclarationError,
    OffsetDeclarationLimits, OffsetDeclarationRefusal,
};
pub use presence::{
    ActiveOffsetCandidate, ActiveOffsetCandidates, OffsetRolePresenceError,
    OffsetRolePresenceLimits, OffsetRolePresenceRefusal,
};
pub use query::{
    OffsetCandidateMatch, OffsetCandidateMatchKind, OffsetCandidateQuery,
    OffsetCandidateQueryError, OffsetCandidateQueryRefusal,
};

/// One declared logical output role and positive original-source domain,not a computed curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OffsetCandidateFragment {
    tag: LocalTag,
    source: OffsetInterval,
}
impl OffsetCandidateFragment {
    /// Retain explicit persistent tag and original domain;neither comes from vector position.
    #[must_use]
    pub const fn new(tag: LocalTag, source: OffsetInterval) -> Self {
        Self { tag, source }
    }
    /// Actual explicit logical tag,without identity issuance or replay matching proof.
    #[must_use]
    pub const fn tag(&self) -> LocalTag {
        self.tag
    }
    /// Actual positive source interval;this does not specify a mapped output Param.
    #[must_use]
    pub const fn source_interval(&self) -> &OffsetInterval {
        &self.source
    }
}
/// Explicit complete candidate allowance;earlier Vec allocation and later execution are separate.
/// ```compile_fail
/// let policy:sc_core::construction::OffsetCandidateLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OffsetCandidateLimits {
    fragments: usize,
}
impl OffsetCandidateLimits {
    /// Select the complete maximum population,not an accepted prefix.
    #[must_use]
    pub const fn new(fragments: usize) -> Self {
        Self { fragments }
    }
    /// Explicit candidate population allowance.
    #[must_use]
    pub const fn fragments(self) -> usize {
        self.fragments
    }
}
/// Complete quota,actual bound-input association or first original fragment refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetCandidateRefusal {
    /// Complete population exceeds the selected allowance.
    Fragments {
        /// Complete original population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// This candidate manifest requires a nonempty result;total collapse is not one fabricated edge.
    Empty,
    /// Original manifest operation differs from the actual bound input operation.
    Operation,
    /// Original edge,bounds or traversal differ from the actual bound input.
    Source,
    /// Explicit tag repeated;equal domains or content do not permit identity reuse.
    DuplicateTag {
        /// Original first occurrence index.
        first: usize,
        /// Original repeated occurrence index.
        second: usize,
    },
    /// Fragment domain leaves the actual selected source range.
    SourceBounds {
        /// Actual original fragment index.
        fragment: usize,
    },
    /// Domains overlap or descend in original source-frame order;shared endpoints remain allowed.
    SourceOrder {
        /// Actual previous original index.
        previous: usize,
        /// Actual next original index.
        next: usize,
    },
}
/// Original complete manifest,bound input and explicit policy survive refusal without cloning.
pub struct OffsetCandidateError<'a> {
    manifest: &'a OffsetCandidateManifest,
    bound: &'a BoundOffsetConstruction<'a>,
    limits: OffsetCandidateLimits,
    refusal: OffsetCandidateRefusal,
}
impl<'a> OffsetCandidateError<'a> {
    /// Complete original raw manifest,including every unaccepted candidate.
    #[must_use]
    pub const fn manifest(&self) -> &'a OffsetCandidateManifest {
        self.manifest
    }
    /// Actual bound input/allowance association.
    #[must_use]
    pub const fn bound_input(&self) -> &'a BoundOffsetConstruction<'a> {
        self.bound
    }
    /// Original explicit admission allowance.
    #[must_use]
    pub const fn limits(&self) -> OffsetCandidateLimits {
        self.limits
    }
    /// First complete-quota or actual original source/fragment cause.
    #[must_use]
    pub const fn refusal(&self) -> OffsetCandidateRefusal {
        self.refusal
    }
}
impl fmt::Debug for OffsetCandidateError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OffsetCandidateError")
            .field("operation", &self.manifest.operation)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for OffsetCandidateError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "offset {} candidate admission: {:?}",
            self.manifest.operation, self.refusal
        )
    }
}
impl std::error::Error for OffsetCandidateError<'_> {}

/// Owned unadmitted logical roles and source domains,no curves or issued output identities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OffsetCandidateManifest {
    operation: EntityId,
    source: DirectedRange,
    fragments: Vec<OffsetCandidateFragment>,
}
impl OffsetCandidateManifest {
    /// Retain every original candidate without evaluating or accepting it.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        source: DirectedRange,
        fragments: Vec<OffsetCandidateFragment>,
    ) -> Self {
        Self {
            operation,
            source,
            fragments,
        }
    }
    /// Original required creating operation,not an allocated ledger operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Original source edge,bounds and traversal;no current-fragment alias.
    #[must_use]
    pub const fn source(&self) -> &DirectedRange {
        &self.source
    }
    /// All original roles in declared source order,without sorting or deduplication.
    #[must_use]
    pub fn fragments(&self) -> &[OffsetCandidateFragment] {
        &self.fragments
    }
    /// Complete quota precedes all content checks and the temporary tag index allocation.
    /// Then check operation/source,followed by each original fragment's tag,bounds and order.
    /// Count bounds this metadata admission/tag index only;earlier Vec allocation,ledger capacity,
    /// replay matching,currentness,geometry/error evidence and publication resources stay separate.
    /// # Errors
    /// Quota,empty,operation,source,then first fragment duplicate tag/bounds/order. No accepted prefix.
    pub fn admit<'a>(
        &'a self,
        bound: &'a BoundOffsetConstruction<'a>,
        limits: OffsetCandidateLimits,
    ) -> Result<AdmittedOffsetCandidates<'a>, OffsetCandidateError<'a>> {
        let refuse = |refusal| OffsetCandidateError {
            manifest: self,
            bound,
            limits,
            refusal,
        };
        if self.fragments.len() > limits.fragments {
            return Err(refuse(OffsetCandidateRefusal::Fragments {
                required: self.fragments.len(),
                allowed: limits.fragments,
            }));
        }
        if self.fragments.is_empty() {
            return Err(refuse(OffsetCandidateRefusal::Empty));
        }
        if self.operation != bound.construction().operation() {
            return Err(refuse(OffsetCandidateRefusal::Operation));
        }
        if &self.source != bound.construction().source() {
            return Err(refuse(OffsetCandidateRefusal::Source));
        }
        let source = self.source.range;
        let mut tags = BTreeMap::new();
        let mut previous = None;
        for (index, fragment) in self.fragments.iter().enumerate() {
            if let Some(first) = tags.insert(fragment.tag, index) {
                return Err(refuse(OffsetCandidateRefusal::DuplicateTag {
                    first,
                    second: index,
                }));
            }
            if fragment.source.from() < source.from() || fragment.source.to() > source.to() {
                return Err(refuse(OffsetCandidateRefusal::SourceBounds {
                    fragment: index,
                }));
            }
            if let Some((previous_index, previous_to)) = previous {
                if fragment.source.from() < previous_to {
                    return Err(refuse(OffsetCandidateRefusal::SourceOrder {
                        previous: previous_index,
                        next: index,
                    }));
                }
            }
            previous = Some((index, fragment.source.to()));
        }
        Ok(AdmittedOffsetCandidates {
            manifest: self,
            bound,
            limits,
        })
    }
}
/// Complete immutable metadata admission,not stable-role replay reconciliation or a G2 certificate.
/// ```compile_fail
/// fn forge<'a>(manifest:&'a sc_core::construction::OffsetCandidateManifest,
///     bound:&'a sc_core::construction::BoundOffsetConstruction<'a>,limits:sc_core::construction::OffsetCandidateLimits)
///     ->sc_core::construction::AdmittedOffsetCandidates<'a> {
///     sc_core::construction::AdmittedOffsetCandidates { manifest,bound,limits }
/// }
/// ```
/// ```compile_fail
/// fn escape(manifest:sc_core::construction::OffsetCandidateManifest,
///     bound:sc_core::construction::BoundOffsetConstruction<'_>)->sc_core::construction::AdmittedOffsetCandidates<'static> {
///     manifest.admit(&bound,sc_core::construction::OffsetCandidateLimits::new(1)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct AdmittedOffsetCandidates<'a> {
    manifest: &'a OffsetCandidateManifest,
    bound: &'a BoundOffsetConstruction<'a>,
    limits: OffsetCandidateLimits,
}
impl<'a> AdmittedOffsetCandidates<'a> {
    /// Actual complete original manifest.
    #[must_use]
    pub const fn manifest(self) -> &'a OffsetCandidateManifest {
        self.manifest
    }
    /// Actual original bound input and provided allowance descriptor.
    #[must_use]
    pub const fn bound_input(self) -> &'a BoundOffsetConstruction<'a> {
        self.bound
    }
    /// Selected original metadata-admission policy.
    #[must_use]
    pub const fn limits(self) -> OffsetCandidateLimits {
        self.limits
    }
    /// All actual original candidate fields,in source order,including gaps/shared boundaries.
    #[must_use]
    pub fn fragments(self) -> &'a [OffsetCandidateFragment] {
        self.manifest.fragments()
    }
    /// Logical unissued edge roles paired with their authentic original candidate fields.
    /// Tag values stay explicit even when unrelated to source order or vector positions.
    /// These are not current registered edges,computed curves or mapped output positions.
    pub fn outputs(
        self,
    ) -> impl ExactSizeIterator<Item = (EdgeRef, &'a OffsetCandidateFragment)> + DoubleEndedIterator + 'a
    {
        self.manifest.fragments.iter().map(move |fragment| {
            (
                EdgeRef::new(self.manifest.operation, fragment.tag),
                fragment,
            )
        })
    }
}
