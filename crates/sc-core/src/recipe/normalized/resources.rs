//! Actual retained node/call-index vectors; borrowed source and fixed literals own no new buffers.
use super::{FormulaNormalizedExpression, NormalizedData, NormalizedNode};
use crate::design::{
    fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation, DesignResourceSink,
};
impl FormulaNormalizedExpression<'_> {
    pub(crate) fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        nodes_location: DesignResourceLocation,
        arguments_location: DesignResourceLocation,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            nodes,
            root,
            if_depth,
        } = self;
        fixed_value(root);
        fixed_value(if_depth);
        sink.charge_entries(nodes.len(), nodes_location)?;
        for (index, node) in nodes.iter().enumerate() {
            let NormalizedNode { span, data } = node;
            fixed_value(span);
            match data {
                NormalizedData::Literal(literal) => fixed_value(literal),
                NormalizedData::Name(name) => fixed_value(name),
                NormalizedData::Negate(child) | NormalizedData::Square(child) => fixed_value(child),
                NormalizedData::Binary {
                    operator,
                    left,
                    right,
                } => {
                    fixed_value(operator);
                    fixed_value(left);
                    fixed_value(right);
                }
                NormalizedData::Conditional {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    fixed_value(condition);
                    fixed_value(then_branch);
                    fixed_value(else_branch);
                }
                NormalizedData::Call { name, arguments } => {
                    fixed_value(name);
                    fixed_slice(arguments);
                    sink.charge_entries(
                        arguments.len(),
                        DesignResourceLocation {
                            index: Some(index),
                            ..arguments_location
                        },
                    )?;
                }
            }
        }
        Ok(())
    }
}
