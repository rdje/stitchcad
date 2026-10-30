//! Original requester/supplier causes;only semantic request context is boxed.
use crate::recipe::{
    FormulaCanonicalInput, FormulaCheckedNode, FormulaCheckedStatement,
    FormulaScalarExecutionError, FormulaScalarRecipeError, FormulaScalarSourceGraphExecutionCause,
    FormulaSourceSpan, FormulaStatementExpression,
};
use core::fmt;

pub(in crate::recipe::execution) struct SourceRequestFailure<'a> {
    pub(in crate::recipe::execution) node: FormulaCheckedNode<'a, 'a>,
    pub(in crate::recipe::execution) input: FormulaCanonicalInput<'a>,
    pub(in crate::recipe::execution) cause: FormulaScalarSourceGraphExecutionCause<'a>,
}
impl fmt::Debug for SourceRequestFailure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceRequestFailure")
            .field("node", &self.node)
            .field("kind", &self.input.metadata().kind())
            .field("cause", &self.cause)
            .finish()
    }
}
pub(in crate::recipe::execution) enum SourceOperandFailure<'a> {
    // Allocation keeps the original cause inline and the real requesting span,without a new box.
    Allocation {
        span: FormulaSourceSpan,
        cause: FormulaScalarSourceGraphExecutionCause<'a>,
    },
    Execution(Box<SourceRequestFailure<'a>>),
}
impl<'a> SourceOperandFailure<'a> {
    pub(super) fn new(
        node: FormulaCheckedNode<'a, 'a>,
        input: FormulaCanonicalInput<'a>,
        cause: FormulaScalarSourceGraphExecutionCause<'a>,
    ) -> Self {
        match cause {
            cause @ FormulaScalarSourceGraphExecutionCause::Allocation { .. } => Self::Allocation {
                span: node.span(),
                cause,
            },
            cause => Self::Execution(Box::new(SourceRequestFailure { node, input, cause })),
        }
    }
}
impl fmt::Debug for SourceOperandFailure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allocation { span, cause } => f
                .debug_struct("Allocation")
                .field("span", span)
                .field("cause", cause)
                .finish(),
            Self::Execution(error) => fmt::Debug::fmt(error, f),
        }
    }
}
pub(super) enum OperandFailure<'a> {
    Scalar(FormulaScalarExecutionError<'a>),
    Source(SourceOperandFailure<'a>),
}
pub(in crate::recipe::execution) enum RecipeRuntimeError<'a> {
    Recipe(FormulaScalarRecipeError<'a>),
    Source {
        statement: &'a FormulaCheckedStatement<'a>,
        part: FormulaStatementExpression,
        error: SourceOperandFailure<'a>,
    },
}
impl fmt::Debug for RecipeRuntimeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Recipe(error) => fmt::Debug::fmt(error, f),
            Self::Source {
                statement,
                part,
                error,
            } => f
                .debug_struct("Source")
                .field("statement", statement)
                .field("part", part)
                .field("error", error)
                .finish(),
        }
    }
}
