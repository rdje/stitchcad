//! Authored signed break declarations over current axis, dimension and adjacent member identities.
use crate::{Measurement, MeasurementBinding, SizeSetReference};
use sc_core::{ontology::EntityId, value::LengthDeclaration};
use sc_units::Length;

mod context;
mod error;
pub use context::SizeBreakContext;
pub use error::SizeBreakError;

/// Explicit axis/POM/adjacent-pair break input; no chart subtraction or numeric/state cache.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeBreakDefinition {
    /// Stable authored break identity.
    pub id: EntityId,
    /// Exact owning SizeSet identity and revision.
    pub reference: SizeSetReference,
    /// Explicit axis of the size step.
    pub axis: EntityId,
    /// Earlier member in the actual authored membership sequence.
    pub from: EntityId,
    /// Immediately following member, never inferred from label or axis-value sorting.
    pub to: EntityId,
    /// Named table owning the explicit measurement dimension.
    pub dimension_table: EntityId,
    /// Exact canonical measurement identity/token/domain/declaration, not an axis-name guess.
    pub dimension: MeasurementBinding,
    /// Sole canonical signed Length declaration for the delta, retaining its state/source/provenance.
    pub delta: EntityId,
}

/// Immutable locally validated signed break; coverage, grading and equivalence remain separate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeBreak {
    definition: SizeBreakDefinition,
}
impl SizeBreak {
    pub(crate) fn resource_definition(&self) -> &SizeBreakDefinition {
        let Self { definition } = self;
        definition
    }
    /// Validate the explicit key, actual dimension binding and canonical delta without reading its value.
    /// # Errors
    /// Returns exact current pin, identity, adjacency, dimension or delta-source evidence.
    pub fn new(
        definition: SizeBreakDefinition,
        context: &SizeBreakContext<'_>,
    ) -> Result<Self, SizeBreakError> {
        let value = Self { definition };
        value.validate_current(context)?;
        Ok(value)
    }

    /// Original authored key/source references, without unchecked mutation.
    #[must_use]
    pub const fn definition(&self) -> &SizeBreakDefinition {
        &self.definition
    }

    /// Check the saved key against this actual immutable current snapshot.
    /// # Errors
    /// Returns the first actual pin/reference/adjacency/dimension/source refusal.
    pub fn validate_current(&self, context: &SizeBreakContext<'_>) -> Result<(), SizeBreakError> {
        self.delta(context).map(|_| ())
    }

    /// Borrow the original canonical signed delta declaration, including Unknown/Derived state.
    /// # Errors
    /// Returns current metadata/source refusals; does not evaluate a formula or require a numeric value.
    pub fn delta<'a>(
        &self,
        context: &SizeBreakContext<'a>,
    ) -> Result<&'a LengthDeclaration, SizeBreakError> {
        context.resolve(&self.definition)
    }

    /// Borrow the actual current named measurement after validating this complete break declaration.
    /// # Errors
    /// Returns current break or original canonical table/measurement evidence.
    pub fn dimension<'a>(
        &self,
        context: &SizeBreakContext<'a>,
    ) -> Result<&'a Measurement, SizeBreakError> {
        self.validate_current(context)?;
        context.dimension(self.definition.dimension.measurement)
    }

    /// Read a present authored signed delta, without replacing Unknown/Derived by zero or chart arithmetic.
    /// # Errors
    /// Preserves the original observation/evaluation refusal as a standard error source.
    pub fn authored_value(&self, context: &SizeBreakContext<'_>) -> Result<Length, SizeBreakError> {
        self.delta(context)?
            .authored_value()
            .map_err(SizeBreakError::Value)
    }
}
