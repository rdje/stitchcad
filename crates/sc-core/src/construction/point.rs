//! Owned point formulas and complete value-free argument normalization.
use crate::{
    ontology::{EntityId, LocalTag, PointRef},
    recipe::{
        FormulaExpressionDocument, FormulaExpressionDocumentError, FormulaKind,
        FormulaNormalizedExpression,
    },
};
use core::fmt;
mod check;
pub use check::{
    CheckedPointConstruction, PointConstructionCheckError, PointConstructionCheckRefusal,
};

/// Original coordinate argument role, not an inferred frame or evaluated point component.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PointCoordinate {
    /// Horizontal Length in the containing drafting frame.
    X,
    /// Vertical Length in the containing drafting frame.
    Y,
}
impl PointCoordinate {
    /// Required static result kind for either actual coordinate argument.
    #[must_use]
    pub const fn required_kind(self) -> FormulaKind {
        match self {
            Self::X | Self::Y => FormulaKind::Length,
        }
    }
}

/// Authored point-from-formulas declaration; no coordinate value, frame or registry proof.
/// Original documents remain distinct even when unit aliases have equal canonical syntax.
/// The creating operation owns one persistent point role at LocalTag::FIRST. Constructing this
/// record neither issues that id nor publishes geometry; command and containing-program checks
/// remain required. The containing ordered program must supply the actual drafting frame.
/// ```compile_fail
/// fn rewrite(point:&mut sc_core::construction::PointConstruction) { point.x=todo!(); }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointConstruction {
    operation: EntityId,
    x: FormulaExpressionDocument,
    y: FormulaExpressionDocument,
}
impl PointConstruction {
    // Exhaustive actual fields; dynamic documents stay borrowed from this original owner.
    pub(crate) fn resource_documents(&self) -> [&FormulaExpressionDocument; 2] {
        use super::operation_input::resources::fixed;
        let Self { operation, x, y } = self;
        fixed(operation);
        [x, y]
    }

    /// Retain two already syntax/input-validated documents, without kind checking or values.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        x: FormulaExpressionDocument,
        y: FormulaExpressionDocument,
    ) -> Self {
        Self { operation, x, y }
    }
    /// Actual supplied creating-operation identity; no allocation or currentness is implied.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Stable logical point role, independent of formula text or geometry-vector positions.
    #[must_use]
    pub const fn output(&self) -> PointRef {
        PointRef::new(self.operation, LocalTag::FIRST)
    }
    /// Original owned coordinate document, including source spelling and typed syntax identity.
    #[must_use]
    pub const fn coordinate(&self, coordinate: PointCoordinate) -> &FormulaExpressionDocument {
        match coordinate {
            PointCoordinate::X => &self.x,
            PointCoordinate::Y => &self.y,
        }
    }
    /// Admit complete node/retained populations before reparsing either original document.
    /// For N nodes across both arguments, retained records are `2 + 2N`: two argument owners,
    /// N normalized nodes and at most N call-argument indices. Earlier document creation and
    /// parser/checker scratch, allocator capacity, geometry/runtime and CPU remain separate.
    /// # Errors
    /// Complete population/quota refusal precedes either original document factory. A document
    /// refusal retains its actual coordinate and original cause; no normalized prefix escapes.
    pub fn normalized(
        &self,
        limits: PointConstructionNormalizationLimits,
    ) -> Result<NormalizedPointConstruction<'_>, PointConstructionNormalizationError<'_>> {
        use PointConstructionNormalizationRefusal as R;
        let refuse = |refusal| PointConstructionNormalizationError {
            point: self,
            limits,
            refusal,
        };
        use super::arguments::{measure, PopulationRefusal as P};
        let (nodes, records) = measure([&self.x, &self.y], limits.nodes, limits.retained_records)
            .map_err(|cause| {
            refuse(match cause {
                P::Overflow => R::PopulationOverflow,
                P::Nodes { allowed, required } => R::Nodes { allowed, required },
                P::RetainedRecords { allowed, required } => {
                    R::RetainedRecords { allowed, required }
                }
            })
        })?;
        let x = self.x.normalized().map_err(|error| {
            refuse(R::Document {
                coordinate: PointCoordinate::X,
                error: Box::new(error),
            })
        })?;
        let y = self.y.normalized().map_err(|error| {
            refuse(R::Document {
                coordinate: PointCoordinate::Y,
                error: Box::new(error),
            })
        })?;
        Ok(NormalizedPointConstruction {
            point: self,
            limits,
            nodes,
            retained_records: records,
            x,
            y,
        })
    }
}

/// Explicit complete normalization populations; no runtime or allocator-byte allowance is inferred.
/// ```compile_fail
/// let limits:sc_core::construction::PointConstructionNormalizationLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PointConstructionNormalizationLimits {
    nodes: usize,
    retained_records: usize,
}
impl PointConstructionNormalizationLimits {
    /// Select complete N node and conservative `2 + 2N` retained-record allowances.
    #[must_use]
    pub const fn new(nodes: usize, retained_records: usize) -> Self {
        Self {
            nodes,
            retained_records,
        }
    }
    /// Maximum complete nodes, including every branch and both coordinate arguments.
    #[must_use]
    pub const fn nodes(self) -> usize {
        self.nodes
    }
    /// Maximum conservative normalized retained records, excluding parser/checker/runtime scratch.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.retained_records
    }
}
/// Complete normalization refusal, separate from kind checking or numerical diagnostics.
#[derive(Debug)]
pub enum PointConstructionNormalizationRefusal {
    /// Complete count or retained-record arithmetic cannot be represented.
    PopulationOverflow,
    /// Complete actual node quota, checked first.
    Nodes {
        /// Explicit selected allowance.
        allowed: usize,
        /// Complete actual node population.
        required: usize,
    },
    /// Complete conservative retained-record quota, checked before either reparse.
    RetainedRecords {
        /// Explicit selected allowance.
        allowed: usize,
        /// Complete prospective retained records.
        required: usize,
    },
    /// Original document refusal, without a replacement formula diagnostic.
    Document {
        /// Actual original argument role.
        coordinate: PointCoordinate,
        /// Original semantic syntax/input-literal cause, compacted without erasing its type.
        error: Box<FormulaExpressionDocumentError>,
    },
}
/// Original point owner, selected normalization policy and complete refusal; no accepted prefix.
pub struct PointConstructionNormalizationError<'a> {
    point: &'a PointConstruction,
    limits: PointConstructionNormalizationLimits,
    refusal: PointConstructionNormalizationRefusal,
}
impl<'a> PointConstructionNormalizationError<'a> {
    /// Actual original declaration and its documents.
    #[must_use]
    pub const fn point(&self) -> &'a PointConstruction {
        self.point
    }
    /// Original explicit normalization policy.
    #[must_use]
    pub const fn limits(&self) -> PointConstructionNormalizationLimits {
        self.limits
    }
    /// Original complete typed cause, including a real argument role when available.
    #[must_use]
    pub const fn refusal(&self) -> &PointConstructionNormalizationRefusal {
        &self.refusal
    }
}
impl fmt::Debug for PointConstructionNormalizationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointConstructionNormalizationError")
            .field("operation", &self.point.operation())
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for PointConstructionNormalizationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("point construction normalization refused")
    }
}
impl std::error::Error for PointConstructionNormalizationError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            PointConstructionNormalizationRefusal::Document { error, .. } => Some(error.as_ref()),
            _ => None,
        }
    }
}
/// Both actual normalized argument arenas, pinned to their original authored declaration.
/// No caller-created arena, partial argument list, frame or geometric result can be substituted.
/// ```compile_fail
/// fn escape(point:&sc_core::construction::PointConstruction)
///     ->sc_core::construction::NormalizedPointConstruction<'static> {
///     point.normalized(sc_core::construction::PointConstructionNormalizationLimits::new(8,18)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct NormalizedPointConstruction<'a> {
    point: &'a PointConstruction,
    limits: PointConstructionNormalizationLimits,
    nodes: usize,
    retained_records: usize,
    x: FormulaNormalizedExpression<'a>,
    y: FormulaNormalizedExpression<'a>,
}
impl<'a> NormalizedPointConstruction<'a> {
    /// Actual original declaration and persistent output role.
    #[must_use]
    pub const fn point(&self) -> &'a PointConstruction {
        self.point
    }
    /// Original explicitly admitted normalization policy.
    #[must_use]
    pub const fn limits(&self) -> PointConstructionNormalizationLimits {
        self.limits
    }
    /// Complete actual node population across both arguments.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.nodes
    }
    /// Complete admitted conservative normalized retained records.
    #[must_use]
    pub const fn retained_records(&self) -> usize {
        self.retained_records
    }
    /// Actual coordinate's normalized arena borrowing its original document text.
    #[must_use]
    pub const fn coordinate(
        &self,
        coordinate: PointCoordinate,
    ) -> &FormulaNormalizedExpression<'a> {
        match coordinate {
            PointCoordinate::X => &self.x,
            PointCoordinate::Y => &self.y,
        }
    }
}
