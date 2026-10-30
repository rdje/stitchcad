//! Complete prospective canonical Derived-source relation; no cycle, operation or execution proof.
use super::{
    FormulaCanonicalInput, FormulaCheckedSource, FormulaCheckedSources, FormulaDeclarationSource,
    FormulaNameDependency,
};
use crate::ontology::EntityId;
use core::fmt;
use std::collections::TryReserveError;

mod cycle;
pub use cycle::{
    FormulaAcyclicSourceDependencies, FormulaSourceCycle, FormulaSourceCycleError,
    FormulaSourceCycleLimits, FormulaSourceCycleRefusal,
};

/// Explicit complete source/Derived-edge and retained-record allowances, without defaults.
/// `sources + edges + 1` charges one source offset per vertex plus its terminal sentinel and
/// every distinct original Derived name use. Existing proofs and future cycle scratch are separate;
/// this is not an allocator-byte,CPU,whole geometry/operation graph or runtime envelope.
/// ```compile_fail
/// let limits: sc_core::recipe::FormulaSourceDependencyLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaSourceDependencyLimits {
    sources: usize,
    edges: usize,
    retained_records: usize,
}
impl FormulaSourceDependencyLimits {
    /// Select all allowances explicitly; an empty relation retains one terminal offset record.
    #[must_use]
    pub const fn new(sources: usize, edges: usize, retained_records: usize) -> Self {
        Self {
            sources,
            edges,
            retained_records,
        }
    }
    /// Maximum complete source population, including unused/constant vertices.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Maximum complete original Derived name-use edge population, without deduplication.
    #[must_use]
    pub const fn edges(self) -> usize {
        self.edges
    }
    /// Maximum complete `sources + edges + 1` logical retained-record reservation.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.retained_records
    }
}

/// Complete dependency admission failure, distinct from cycle or formula-language diagnostics.
#[derive(Debug)]
pub enum FormulaSourceDependencyRefusal {
    /// Complete source population exceeds its allowance; checked before edge traversal.
    Sources {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual population.
        required: usize,
    },
    /// Complete edge/offset count or retained reservation cannot be represented.
    PopulationOverflow,
    /// All original canonical Derived reads exceed their selected allowance.
    Edges {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual edge population.
        required: usize,
    },
    /// Complete source-offset/edge reservation exceeds its selected allowance.
    RetainedRecords {
        /// Selected allowance.
        allowed: usize,
        /// Complete prospective reservation.
        required: usize,
    },
    /// Original source-offset vector reservation cause, without a new diagnostic allocation.
    OffsetAllocation(TryReserveError),
    /// Original edge-vector reservation cause; no accepted offset/edge prefix escapes.
    EdgeAllocation(TryReserveError),
}

/// Refusal pinned to the actual complete source proofs and selected explicit allowances.
#[derive(Debug)]
pub struct FormulaSourceDependencyError<'a> {
    checked: &'a FormulaCheckedSources<'a>,
    limits: FormulaSourceDependencyLimits,
    refusal: FormulaSourceDependencyRefusal,
}
impl<'a> FormulaSourceDependencyError<'a> {
    /// Actual complete source proofs, including original normalized/canonical owners.
    #[must_use]
    pub const fn checked(&self) -> &'a FormulaCheckedSources<'a> {
        self.checked
    }
    /// Selected allowances, separate from canonical content.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceDependencyLimits {
        self.limits
    }
    /// Complete resource/allocator cause; no guessed failing node or statement ordinal.
    #[must_use]
    pub const fn refusal(&self) -> &FormulaSourceDependencyRefusal {
        &self.refusal
    }
}
impl fmt::Display for FormulaSourceDependencyError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "canonical Derived-source dependencies: {:?}",
            self.refusal
        )
    }
}
impl std::error::Error for FormulaSourceDependencyError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            FormulaSourceDependencyRefusal::OffsetAllocation(error)
            | FormulaSourceDependencyRefusal::EdgeAllocation(error) => Some(error),
            FormulaSourceDependencyRefusal::Sources { .. }
            | FormulaSourceDependencyRefusal::PopulationOverflow
            | FormulaSourceDependencyRefusal::Edges { .. }
            | FormulaSourceDependencyRefusal::RetainedRecords { .. } => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    consumer: usize,
    name_use: usize,
    supplier: usize,
}

/// One original prospective read of a real Derived canonical declaration's actual formula source.
/// Direction is consumer-expression→supplier-source, not chronological execution or a read trace.
#[derive(Clone, Copy)]
pub struct FormulaSourceDependency<'a> {
    checked: &'a FormulaCheckedSources<'a>,
    edge: Edge,
}
impl fmt::Debug for FormulaSourceDependency<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaSourceDependency")
            .field("consumer", &self.consumer().normalized().source().id())
            .field("supplier", &self.supplier().normalized().source().id())
            .field("span", &self.name_use().span())
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaSourceDependency<'a> {
    /// Actual original source expression containing this prospective canonical read.
    #[must_use]
    pub fn consumer(self) -> &'a FormulaCheckedSource<'a> {
        source(self.checked, self.edge.consumer)
    }
    /// Actual source selected solely by that canonical declaration's Derived formula field.
    #[must_use]
    pub fn supplier(self) -> &'a FormulaCheckedSource<'a> {
        source(self.checked, self.edge.supplier)
    }
    /// Actual original checked name occurrence, retaining metadata and its local source span.
    #[must_use]
    pub fn name_use(self) -> FormulaNameDependency<'a> {
        let Some(dependency) = self
            .consumer()
            .proof()
            .dependencies()
            .get(self.edge.name_use)
        else {
            unreachable!("private edge retains its actual complete proof dependency index");
        };
        *dependency
    }
    /// Existing actual canonical inventory certificate for this original read, without values.
    #[must_use]
    pub fn canonical_input(self) -> FormulaCanonicalInput<'a> {
        let Some((_, canonical)) = derived(self.checked, self.name_use()) else {
            unreachable!("private edge retains a real Derived input in the same immutable owners");
        };
        canonical
    }
}

/// Complete admitted canonical Derived-source relation, preserving original source/name-use order.
/// Repeated and untaken occurrences are distinct edges;constant/unused sources remain vertices.
/// It proves no acyclicity,current Design/host/provider,whole geometry/operation graph,binding,
/// evaluation order,rounding or executed value. Cycle admission is a separate required phase.
/// ```compile_fail
/// fn clear(deps: &mut sc_core::recipe::FormulaSourceDependencies<'_>) { deps.edges.clear(); }
/// ```
/// ```compile_fail
/// fn escape(deps: &sc_core::recipe::FormulaSourceDependencies<'_>)
///     -> sc_core::recipe::FormulaSourceDependency<'static> { deps.edges().next().unwrap() }
/// ```
#[derive(Debug)]
pub struct FormulaSourceDependencies<'a> {
    checked: &'a FormulaCheckedSources<'a>,
    limits: FormulaSourceDependencyLimits,
    retained_records: usize,
    edges: Vec<Edge>,
    offsets: Vec<usize>,
}
impl<'a> FormulaSourceDependencies<'a> {
    /// Actual complete source proofs and their original normalized/canonical owners.
    #[must_use]
    pub const fn checked(&self) -> &'a FormulaCheckedSources<'a> {
        self.checked
    }
    /// Selected complete allowances, independent of canonical content.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceDependencyLimits {
        self.limits
    }
    /// Complete admitted `sources + edges + 1` retained-record reservation.
    #[must_use]
    pub const fn retained_records(&self) -> usize {
        self.retained_records
    }
    /// All original Derived read edges in source/dependency order, not executed-read order.
    pub fn edges(
        &self,
    ) -> impl ExactSizeIterator<Item = FormulaSourceDependency<'a>> + DoubleEndedIterator + '_ {
        self.edges.iter().map(|edge| FormulaSourceDependency {
            checked: self.checked,
            edge: *edge,
        })
    }
    /// Exact source-id outgoing edges, including an empty iterator for a constant source.
    /// None denotes only an absent source identity; no canonical-id or name fallback occurs.
    pub fn outgoing(
        &self,
        id: EntityId,
    ) -> Option<impl ExactSizeIterator<Item = FormulaSourceDependency<'a>> + DoubleEndedIterator + '_>
    {
        let index = self
            .checked
            .normalized()
            .inputs()
            .sources()
            .source_index(id)?;
        let (Some(start), Some(end)) = (self.offsets.get(index), self.offsets.get(index + 1))
        else {
            unreachable!("complete private offsets retain every original source and terminal end");
        };
        let Some(edges) = self.edges.get(*start..*end) else {
            unreachable!("complete private offsets bound original grouped edge ranges");
        };
        Some(edges.iter().map(|edge| FormulaSourceDependency {
            checked: self.checked,
            edge: *edge,
        }))
    }
}
impl<'s> FormulaCheckedSources<'s> {
    /// Prepare the complete actual canonical Derived relation after retained admission.
    /// State tags/formula ids are inspected without numeric reads or provider callbacks.
    /// Original source index and actual canonical adapter certificates are reused, not cloned.
    /// # Errors
    /// Complete source quota,checked edge/offset/retained arithmetic,edge quota and retained quota
    /// precede either vector allocation. Original allocation causes are retained;no prefix escapes.
    pub fn prepare_dependencies<'a>(
        &'a self,
        limits: FormulaSourceDependencyLimits,
    ) -> Result<FormulaSourceDependencies<'a>, FormulaSourceDependencyError<'a>>
    where
        's: 'a,
    {
        use FormulaSourceDependencyRefusal as Refusal;
        let checked: &'a FormulaCheckedSources<'a> = self;
        let refuse = |refusal| FormulaSourceDependencyError {
            checked,
            limits,
            refusal,
        };
        let source_count = checked.sources().len();
        if source_count > limits.sources {
            return Err(refuse(Refusal::Sources {
                allowed: limits.sources,
                required: source_count,
            }));
        }
        let mut edge_count = 0usize;
        for source in checked.sources() {
            for dependency in source.proof().dependencies() {
                if derived(checked, *dependency).is_some() {
                    edge_count = edge_count
                        .checked_add(1)
                        .ok_or_else(|| refuse(Refusal::PopulationOverflow))?;
                }
            }
        }
        let offset_count = source_count
            .checked_add(1)
            .ok_or_else(|| refuse(Refusal::PopulationOverflow))?;
        let retained_records = offset_count
            .checked_add(edge_count)
            .ok_or_else(|| refuse(Refusal::PopulationOverflow))?;
        if edge_count > limits.edges {
            return Err(refuse(Refusal::Edges {
                allowed: limits.edges,
                required: edge_count,
            }));
        }
        if retained_records > limits.retained_records {
            return Err(refuse(Refusal::RetainedRecords {
                allowed: limits.retained_records,
                required: retained_records,
            }));
        }
        let mut offsets = Vec::new();
        offsets
            .try_reserve_exact(offset_count)
            .map_err(|error| refuse(Refusal::OffsetAllocation(error)))?;
        let mut edges = Vec::new();
        edges
            .try_reserve_exact(edge_count)
            .map_err(|error| refuse(Refusal::EdgeAllocation(error)))?;
        offsets.push(0);
        for (consumer, source) in checked.sources().iter().enumerate() {
            for (name_use, dependency) in source.proof().dependencies().iter().enumerate() {
                if let Some((formula, _)) = derived(checked, *dependency) {
                    let Some(supplier) = checked
                        .normalized()
                        .inputs()
                        .sources()
                        .source_index(formula)
                    else {
                        unreachable!("actual complete canonical/source admission retains every Derived formula");
                    };
                    edges.push(Edge {
                        consumer,
                        name_use,
                        supplier,
                    });
                }
            }
            offsets.push(edges.len());
        }
        Ok(FormulaSourceDependencies {
            checked,
            limits,
            retained_records,
            edges,
            offsets,
        })
    }
}
fn source<'a>(
    checked: &'a FormulaCheckedSources<'a>,
    index: usize,
) -> &'a FormulaCheckedSource<'a> {
    let Some(source) = checked.sources().get(index) else {
        unreachable!("private admitted edge retains an original complete source proof index");
    };
    source
}
fn derived<'a>(
    checked: &FormulaCheckedSources<'a>,
    dependency: FormulaNameDependency<'a>,
) -> Option<(EntityId, FormulaCanonicalInput<'a>)> {
    match dependency.declaration().source() {
        FormulaDeclarationSource::Reserved(_)
        | FormulaDeclarationSource::Point(_)
        | FormulaDeclarationSource::Edge(_) => return None,
        FormulaDeclarationSource::Recipe { .. } => {
            unreachable!("actual initial source proofs prohibit prior-let seeds")
        }
        FormulaDeclarationSource::Input { .. } | FormulaDeclarationSource::LengthInput { .. } => {}
    }
    let Ok(canonical) = FormulaCanonicalInput::resolve(
        dependency.declaration(),
        checked.normalized().inputs().inputs(),
    ) else {
        unreachable!(
            "complete source proofs already joined this same immutable canonical dependency"
        );
    };
    canonical
        .derived_formula()
        .map(|formula| (formula, canonical))
}
