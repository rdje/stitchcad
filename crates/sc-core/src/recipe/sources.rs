//! Owned formula expression entities; no canonical-input join or graph/value proof.
use super::FormulaExpressionDocument;
use crate::ontology::EntityId;
use core::fmt;
use std::collections::TryReserveError;

/// An explicitly identified original expression, without an inferred name, kind or stored value.
/// The id is supplied by the entity-creation boundary, never derived from text or list position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormulaSource {
    id: EntityId,
    expression: FormulaExpressionDocument,
}
impl FormulaSource {
    /// Retain the exact supplied identity and already validated original expression document.
    #[must_use]
    pub const fn new(id: EntityId, expression: FormulaExpressionDocument) -> Self {
        Self { id, expression }
    }
    /// Stable formula-source entity identity; distinct from a parameter using this source.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.id
    }
    /// Actual authored source owner; canonical aliases do not replace its original text.
    #[must_use]
    pub const fn expression(&self) -> &FormulaExpressionDocument {
        &self.expression
    }
}

/// Explicit complete retained-content allowances. No defaults or language-limit changes.
/// Source count also bounds the single sorted index to one `(EntityId, usize)` record per source.
/// Caller-owned document creation, allocator overhead, parsing scratch and graph work are separate.
/// ```compile_fail
/// let limits: sc_core::recipe::FormulaSourceLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaSourceLimits {
    sources: usize,
    original_bytes: usize,
    canonical_bytes: usize,
}
impl FormulaSourceLimits {
    /// Select all allowances explicitly, including zero for an empty registry.
    #[must_use]
    pub const fn new(sources: usize, original_bytes: usize, canonical_bytes: usize) -> Self {
        Self {
            sources,
            original_bytes,
            canonical_bytes,
        }
    }
    /// Maximum complete source population and index-record population.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Maximum sum of exact original UTF-8 source bytes, including whitespace.
    #[must_use]
    pub const fn original_bytes(self) -> usize {
        self.original_bytes
    }
    /// Maximum sum of existing typed canonical expression bytes, counted separately.
    #[must_use]
    pub const fn canonical_bytes(self) -> usize {
        self.canonical_bytes
    }
}

/// Complete source-content population; not a formula-language diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaSourceResource {
    /// All supplied source entities, including unused ones.
    Sources,
    /// All original source bytes, without trimming or canonical substitution.
    OriginalBytes,
    /// All separately retained canonical expression bytes.
    CanonicalBytes,
}

/// No partial source registry escapes any content, identity or index-allocation refusal.
#[derive(Debug)]
pub enum FormulaSourcesError {
    /// Complete population exceeds its explicit allowance.
    Limit {
        /// Actual retained-content population.
        resource: FormulaSourceResource,
        /// Selected allowance.
        allowed: usize,
        /// Complete measured population.
        required: usize,
    },
    /// Defensive checked-arithmetic refusal before any index allocation.
    PopulationOverflow {
        /// Population whose sum cannot be represented.
        resource: FormulaSourceResource,
    },
    /// One source identity occurs more than once, regardless of body equality.
    DuplicateIdentity {
        /// Actual repeated identity.
        identity: EntityId,
        /// First authored position for this identity.
        first: usize,
        /// Earliest authored duplicate position across the complete registry.
        duplicate: usize,
    },
    /// Original fallible index reservation cause; no body is copied to build the index.
    Allocation(TryReserveError),
}
impl fmt::Display for FormulaSourcesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "formula source registry: {self:?}")
    }
}
impl std::error::Error for FormulaSourcesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Allocation(error) => Some(error),
            _ => None,
        }
    }
}

/// Immutable complete source inventory in authored order, with unique explicit entity ids.
/// This is a source owner, not a current Design, canonical-state, namespace, kind or graph proof.
/// In particular, `Formula(id)` quantity metadata still selects a Design parameter; it does not
/// bypass that parameter to look up an expression here. A Derived state needs a separate real join.
/// ```compile_fail
/// fn clear(sources: &mut sc_core::recipe::FormulaSources) { sources.sources.clear(); }
/// ```
/// ```compile_fail
/// fn rebind(sources: &sc_core::recipe::FormulaSources) {
///     sources.sources()[0].id = sc_core::ontology::EntityId::from_bits(2);
/// }
/// ```
#[derive(Debug)]
pub struct FormulaSources {
    sources: Vec<FormulaSource>,
    index: Vec<(EntityId, usize)>,
}
impl FormulaSources {
    /// Consume complete original entities without cloning their source or canonical buffers.
    /// Limits are admission policy, not stored canonical content. Content checks precede index
    /// allocation and identity checks: count, checked byte sums, original bytes, canonical bytes.
    /// Distinct ids with equal bodies are legal. Equal ids always refuse, even with equal bodies.
    ///
    /// # Errors
    /// Returns complete resource totals, the original reservation error, or the earliest authored
    /// duplicate and its first occurrence. No accepted prefix or replacement id is returned.
    pub fn new(
        sources: Vec<FormulaSource>,
        limits: FormulaSourceLimits,
    ) -> Result<Self, FormulaSourcesError> {
        use FormulaSourceResource::{CanonicalBytes, OriginalBytes, Sources};
        check(Sources, sources.len(), limits.sources)?;
        let mut original_bytes = 0usize;
        let mut canonical_bytes = 0usize;
        for source in &sources {
            original_bytes = add(
                OriginalBytes,
                original_bytes,
                source.expression.source().len(),
            )?;
            canonical_bytes = add(
                CanonicalBytes,
                canonical_bytes,
                source.expression.canonical().as_str().len(),
            )?;
        }
        check(OriginalBytes, original_bytes, limits.original_bytes)?;
        check(CanonicalBytes, canonical_bytes, limits.canonical_bytes)?;
        let mut index = Vec::new();
        index
            .try_reserve_exact(sources.len())
            .map_err(FormulaSourcesError::Allocation)?;
        index.extend(sources.iter().enumerate().map(|(i, source)| (source.id, i)));
        index.sort_unstable();
        let duplicate = index
            .windows(2)
            .filter_map(|pair| match pair {
                [(id, first), (same, duplicate)] if id == same => Some((*id, *first, *duplicate)),
                _ => None,
            })
            .min_by_key(|(_, _, duplicate)| *duplicate);
        if let Some((identity, first, duplicate)) = duplicate {
            return Err(FormulaSourcesError::DuplicateIdentity {
                identity,
                first,
                duplicate,
            });
        }
        Ok(Self { sources, index })
    }
    /// Borrow the exact complete authored source order; lookups never reorder the owners.
    #[must_use]
    pub fn sources(&self) -> &[FormulaSource] {
        &self.sources
    }
    /// Borrow the actual source with the exact supplied identity; no alias, label or fallback.
    #[must_use]
    pub fn source(&self, id: EntityId) -> Option<&FormulaSource> {
        self.sources.get(self.source_index(id)?)
    }
    // Reuse this exact private id→authored-position certificate for parallel derived arenas/proofs.
    // No caller index, duplicate index allocation or lookup through canonical text is admitted.
    pub(super) fn source_index(&self, id: EntityId) -> Option<usize> {
        let position = self.index.binary_search_by_key(&id, |(id, _)| *id).ok()?;
        let (_, authored) = self.index.get(position)?;
        Some(*authored)
    }
}
fn add(
    resource: FormulaSourceResource,
    total: usize,
    next: usize,
) -> Result<usize, FormulaSourcesError> {
    total
        .checked_add(next)
        .ok_or(FormulaSourcesError::PopulationOverflow { resource })
}
fn check(
    resource: FormulaSourceResource,
    required: usize,
    allowed: usize,
) -> Result<(), FormulaSourcesError> {
    if required > allowed {
        return Err(FormulaSourcesError::Limit {
            resource,
            allowed,
            required,
        });
    }
    Ok(())
}
