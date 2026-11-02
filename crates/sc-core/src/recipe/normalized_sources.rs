//! Complete source normalization after original canonical/source joins and retained-shape admission.
use super::{
    FormulaExpressionDocumentError, FormulaNormalizedExpression, FormulaSource, FormulaSourceInputs,
};
use core::fmt;
use std::collections::TryReserveError;

/// Explicit complete source/node and conservative retained-record allowances, without defaults.
/// Retained records are `sources + 2 * nodes`: one source owner-view, one normalized node and at
/// most one additional dynamic call-argument index per node. This is a logical allowance, not
/// allocator bytes/capacity, parser/literal scratch, backend work, CPU or a whole graph envelope.
/// ```compile_fail
/// let limits: sc_core::recipe::FormulaSourceNormalizationLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaSourceNormalizationLimits {
    sources: usize,
    nodes: usize,
    retained_records: usize,
}
impl FormulaSourceNormalizationLimits {
    /// Select all allowances explicitly; zero admits an empty source collection.
    #[must_use]
    pub const fn new(sources: usize, nodes: usize, retained_records: usize) -> Self {
        Self {
            sources,
            nodes,
            retained_records,
        }
    }
    /// Maximum complete expression-source population, including unused entities.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Maximum sum of every actual source's factory-derived semantic node count.
    #[must_use]
    pub const fn nodes(self) -> usize {
        self.nodes
    }
    /// Maximum conservative `sources + 2 * nodes` retained-record reservation.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.retained_records
    }
}

/// Source normalization refusal; no invented formula-language diagnostic or accepted prefix.
#[derive(Debug)]
pub enum FormulaSourceNormalizationRefusal {
    /// Complete source population fails before shape arithmetic or any reparse/allocation.
    Sources {
        /// Explicit selected allowance.
        allowed: usize,
        /// Actual complete population.
        required: usize,
    },
    /// Checked complete node sum or conservative retained reservation cannot be represented.
    PopulationOverflow,
    /// Complete actual node population exceeds its selected allowance.
    Nodes {
        /// Explicit selected allowance.
        allowed: usize,
        /// Sum across every source and every branch.
        required: usize,
    },
    /// Conservative complete retained reservation cannot fit, even if the node quota does.
    RetainedRecords {
        /// Explicit selected allowance.
        allowed: usize,
        /// Actual `sources + 2 * nodes` prospective reservation.
        required: usize,
    },
    /// Original fallible outer-vector reservation cause; no replacement language error.
    Allocation(TryReserveError),
    /// Original document factory failure at its original authored source index.
    /// A box compacts semantic diagnostics; allocator refusals remain inline and allocate nothing.
    Document {
        /// Zero-based original source position.
        source_index: usize,
        /// Original syntax or literal cause; no partial vector is returned.
        error: Box<FormulaExpressionDocumentError>,
    },
}

/// Refusal borrowing the actual canonical/source admission and retaining the selected limits.
#[derive(Debug)]
pub struct FormulaSourceNormalizationError<'a> {
    inputs: FormulaSourceInputs<'a>,
    limits: FormulaSourceNormalizationLimits,
    refusal: FormulaSourceNormalizationRefusal,
}
impl<'a> FormulaSourceNormalizationError<'a> {
    /// Original complete canonical/source admission over actual immutable owners.
    #[must_use]
    pub const fn inputs(&self) -> FormulaSourceInputs<'a> {
        self.inputs
    }
    /// Explicit selected allowances, independent of canonical content identity.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceNormalizationLimits {
        self.limits
    }
    /// Actual complete population/allocation or original document cause.
    #[must_use]
    pub const fn refusal(&self) -> &FormulaSourceNormalizationRefusal {
        &self.refusal
    }
}
impl fmt::Display for FormulaSourceNormalizationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "complete formula source normalization: {:?}",
            self.refusal
        )
    }
}
impl std::error::Error for FormulaSourceNormalizationError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            FormulaSourceNormalizationRefusal::Allocation(error) => Some(error),
            FormulaSourceNormalizationRefusal::Document { error, .. } => Some(error.as_ref()),
            FormulaSourceNormalizationRefusal::Sources { .. }
            | FormulaSourceNormalizationRefusal::PopulationOverflow
            | FormulaSourceNormalizationRefusal::Nodes { .. }
            | FormulaSourceNormalizationRefusal::RetainedRecords { .. } => None,
        }
    }
}

/// One actual original source and its separately owned, original-source-borrowed normalized arena.
/// No statement/header/name, namespace, result kind, value or dependency/cycle proof is invented.
#[derive(Debug)]
pub struct FormulaNormalizedSource<'a> {
    source: &'a FormulaSource,
    expression: FormulaNormalizedExpression<'a>,
}
impl<'a> FormulaNormalizedSource<'a> {
    /// Actual original expression entity; it is not reconstructed from canonical bytes.
    #[must_use]
    pub const fn source(&self) -> &'a FormulaSource {
        self.source
    }
    /// Actual normalized arena; its node views cannot outlive this owner.
    #[must_use]
    pub const fn expression(&self) -> &FormulaNormalizedExpression<'a> {
        &self.expression
    }
}

/// Complete normalized source collection, preserving the original source order and both owners.
/// Admission is complete before any source is reparsed or the outer collection is allocated.
/// Earlier caller-owned document creation and existing per-expression scratch are not charged as
/// retained normalized records. Existing source-byte admission and fixed language limits apply.
/// ```compile_fail
/// fn clear(sources: &mut sc_core::recipe::FormulaNormalizedSources<'_>) {
///     sources.sources.clear();
/// }
/// ```
/// ```compile_fail
/// fn escape(source: &sc_core::recipe::FormulaNormalizedSource<'_>)
///     -> sc_core::recipe::FormulaNormalizedNode<'static> {
///     source.expression().root()
/// }
/// ```
#[derive(Debug)]
pub struct FormulaNormalizedSources<'a> {
    inputs: FormulaSourceInputs<'a>,
    limits: FormulaSourceNormalizationLimits,
    node_count: usize,
    retained_records: usize,
    sources: Vec<FormulaNormalizedSource<'a>>,
}
impl<'a> FormulaNormalizedSources<'a> {
    pub(crate) fn charge_retained_resources(
        &self,
        sink: &mut dyn crate::design::DesignResourceSink,
        location: crate::design::DesignResourceLocation,
    ) -> Result<(), crate::design::DesignResourceLimitError> {
        use crate::design::{fixed_value, DesignResourceLocation};
        let Self {
            inputs,
            limits,
            node_count,
            retained_records,
            sources,
        } = self;
        fixed_value(inputs);
        fixed_value(limits);
        fixed_value(node_count);
        fixed_value(retained_records);
        sink.charge_entries(sources.len(), location)?;
        for (index, item) in sources.iter().enumerate() {
            let FormulaNormalizedSource { source, expression } = item;
            fixed_value(source);
            let at = |field| DesignResourceLocation {
                owner: Some(source.id()),
                field,
                index: Some(index),
                ..location
            };
            expression.charge_retained_resources(
                sink,
                at("drafting.normalized.sources.nodes"),
                at("drafting.normalized.sources.arguments"),
            )?;
        }
        Ok(())
    }

    /// Original canonical/source admission; this adds no current Design or execution proof.
    #[must_use]
    pub const fn inputs(&self) -> FormulaSourceInputs<'a> {
        self.inputs
    }
    /// Explicit retained-population policy, not stored canonical content.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceNormalizationLimits {
        self.limits
    }
    /// Complete actual node count, including all repeated/untaken/unused sources.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }
    /// Complete conservative reservation `sources + 2 * nodes`; not a measured allocator size.
    #[must_use]
    pub const fn retained_records(&self) -> usize {
        self.retained_records
    }
    /// Complete actual source/arena pairs in original authored order; no partial prefix is exposed.
    #[must_use]
    pub fn sources(&self) -> &[FormulaNormalizedSource<'a>] {
        &self.sources
    }
}
impl<'a> FormulaSourceInputs<'a> {
    /// Normalize the complete actual original-source inventory after explicit retained admission.
    /// Count comes from immutable factory metadata, not a caller hint or reparsed prefix. Every
    /// original expression is then processed through its existing parser/literal converter.
    /// # Errors
    /// Complete source quota precedes checked population arithmetic, node and retained quotas;
    /// only then reserve the outer vector and derive original source arenas in authored order.
    /// Original allocation/document causes are retained; no accepted partial owner escapes.
    pub fn normalize_sources(
        self,
        limits: FormulaSourceNormalizationLimits,
    ) -> Result<FormulaNormalizedSources<'a>, FormulaSourceNormalizationError<'a>> {
        use FormulaSourceNormalizationRefusal as Refusal;
        let refuse = |refusal| FormulaSourceNormalizationError {
            inputs: self,
            limits,
            refusal,
        };
        let originals = self.sources().sources();
        if originals.len() > limits.sources {
            return Err(refuse(Refusal::Sources {
                allowed: limits.sources,
                required: originals.len(),
            }));
        }
        let node_count = originals.iter().try_fold(0usize, |total, source| {
            total
                .checked_add(source.expression().node_count())
                .ok_or_else(|| refuse(Refusal::PopulationOverflow))
        })?;
        let retained_records = node_count
            .checked_mul(2)
            .and_then(|nodes| nodes.checked_add(originals.len()))
            .ok_or_else(|| refuse(Refusal::PopulationOverflow))?;
        if node_count > limits.nodes {
            return Err(refuse(Refusal::Nodes {
                allowed: limits.nodes,
                required: node_count,
            }));
        }
        if retained_records > limits.retained_records {
            return Err(refuse(Refusal::RetainedRecords {
                allowed: limits.retained_records,
                required: retained_records,
            }));
        }
        let mut sources = Vec::new();
        sources
            .try_reserve_exact(originals.len())
            .map_err(|error| refuse(Refusal::Allocation(error)))?;
        for (source_index, source) in originals.iter().enumerate() {
            let expression = source.expression().normalized().map_err(|error| {
                refuse(Refusal::Document {
                    source_index,
                    error: Box::new(error),
                })
            })?;
            sources.push(FormulaNormalizedSource { source, expression });
        }
        Ok(FormulaNormalizedSources {
            inputs: self,
            limits,
            node_count,
            retained_records,
            sources,
        })
    }
}
