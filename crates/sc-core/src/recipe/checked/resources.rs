//! Actual held successful proof vectors, separate from their borrowed syntax/declaration owners.
use super::FormulaCheckedExpression;
use crate::design::{
    fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation, DesignResourceSink,
};

impl FormulaCheckedExpression<'_, '_, '_> {
    /// Append the actual node-certificate vector, then the actual ordered dependency vector to
    /// one cumulative sink. Every repeated/untaken name occurrence counts. Borrowed normalized
    /// syntax, declaration/source bodies and names compose through their original owners; this
    /// component owns no extra text or counters. Fixed Copy guards expose added dynamic fields.
    ///
    /// No namespace lookup, expression evaluation, cloning, result allocation or use of an earlier
    /// shape/policy/demand occurs. The caller supplies the two actual containing field locations
    /// and owns/finishes the meter. Accounting grants no whole-content/work or source allowance.
    ///
    /// ```
    /// use sc_core::{recipe::FormulaCheckedExpression,design::*};
    /// fn proof_records(proof:&FormulaCheckedExpression<'_, '_, '_>,sink:&mut dyn DesignResourceSink,
    ///     certificates:DesignResourceLocation,dependencies:DesignResourceLocation)
    ///     ->Result<(),DesignResourceLimitError> {
    ///     proof.charge_retained_resources(sink,certificates,dependencies)
    /// }
    /// ```
    /// # Errors
    /// Original first cumulative certificate/dependency field refusal. Zero-length vectors still
    /// preserve any earlier sink failure, without translation or a successful partial demand.
    pub fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        certificates_location: DesignResourceLocation,
        dependencies_location: DesignResourceLocation,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            expression,
            kind,
            dependencies,
            certificates,
        } = self;
        fixed_value(expression);
        fixed_value(kind);
        fixed_slice(certificates);
        fixed_slice(dependencies);
        sink.charge_entries(certificates.len(), certificates_location)?;
        sink.charge_entries(dependencies.len(), dependencies_location)
    }
}
