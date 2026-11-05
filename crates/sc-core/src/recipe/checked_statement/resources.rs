//! Fresh held expression buffers of one original checked statement; no source-body duplication.
use super::{FormulaCheckedStatement, FormulaCheckedStatementKind};
use crate::{
    design::{
        fixed_value, DesignResourceLimitError, DesignResourceLocation, DesignResourceScope,
        DesignResourceSink,
    },
    ontology::EntityId,
};
impl FormulaCheckedStatement<'_> {
    /// Charge every actual held operand proof, in let or assertion-left/assertion-right order.
    /// Fixed statement metadata and borrowed normalized syntax own no extra dynamic content.
    /// Scope/owner identify the containing caller; each field index is the genuine one-based
    /// original statement ordinal. The caller separately accounts any outer statement vector.
    /// This freshly counts actual vectors, not constructor policies or earlier demands. It reads
    /// no values, searches no names, allocates nothing and leaves the cumulative meter unfinished.
    /// # Errors
    /// Original first operand certificate/dependency field refusal or an earlier sink latch.
    pub fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            statement,
            statement_index,
            kind,
        } = self;
        fixed_value(statement);
        fixed_value(statement_index);
        let at = |field| DesignResourceLocation {
            scope,
            owner,
            field,
            index: Some(*statement_index),
        };
        match kind {
            FormulaCheckedStatementKind::Let { expression } => expression
                .charge_retained_resources(
                    sink,
                    at("checked.statement.let.certificates"),
                    at("checked.statement.let.dependencies"),
                ),
            FormulaCheckedStatementKind::Assert { left, right } => {
                left.charge_retained_resources(
                    sink,
                    at("checked.statement.assert.left.certificates"),
                    at("checked.statement.assert.left.dependencies"),
                )?;
                right.charge_retained_resources(
                    sink,
                    at("checked.statement.assert.right.certificates"),
                    at("checked.statement.assert.right.dependencies"),
                )
            }
        }
    }
}
