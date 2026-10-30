//! Complete paired scope and original failures;allocator causes require no extra box.
use super::{
    recipe_runtime::failure::SourceRequestFailure, source_failure::cause_source,
    FormulaRecipeExecutionLimits, FormulaScalarRecipeError, FormulaScalarSourceGraphExecutionCause,
    FormulaSourceGraphExecutionLimits,
};
use crate::{
    recipe::{
        FormulaAcyclicSourceDependencies, FormulaCanonicalInput, FormulaCheckedNode,
        FormulaCheckedRecipe, FormulaCheckedStatement, FormulaSourceSpan,
        FormulaStatementExpression,
    },
    value::CanonicalInputs,
};
use core::fmt;

/// Actual requesting recipe operand and original failing supplier execution.
/// Its private request was earned by the machine,not supplied metadata or an invented header.
pub struct FormulaScalarSourceRecipeSourceError<'a> {
    pub(super) statement: &'a FormulaCheckedStatement<'a>,
    pub(super) part: FormulaStatementExpression,
    pub(super) request: Box<SourceRequestFailure<'a>>,
}
impl<'a> FormulaScalarSourceRecipeSourceError<'a> {
    /// Original whole-proof requesting statement,including its actual ordinal/name/spans.
    #[must_use]
    pub const fn statement(&self) -> &'a FormulaCheckedStatement<'a> {
        self.statement
    }
    /// Actual binding RHS or assertion left/right operand.
    #[must_use]
    pub const fn part(&self) -> FormulaStatementExpression {
        self.part
    }
    /// Actual requesting name node in that original recipe operand.
    #[must_use]
    pub const fn node(&self) -> FormulaCheckedNode<'a, 'a> {
        self.request.node
    }
    /// Original canonical Derived join and sole supplier reference;no authored value is invented.
    #[must_use]
    pub const fn input(&self) -> FormulaCanonicalInput<'a> {
        self.request.input
    }
    /// Original supplier index/node/value/provider/frame cause.
    #[must_use]
    pub const fn cause(&self) -> &FormulaScalarSourceGraphExecutionCause<'a> {
        &self.request.cause
    }
}
impl fmt::Debug for FormulaScalarSourceRecipeSourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaScalarSourceRecipeSourceError")
            .field("statement", &self.statement)
            .field("part", &self.part)
            .field("request", &self.request)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for FormulaScalarSourceRecipeSourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "recipe statement {} {:?} source request: {:?}",
            self.statement.statement_index(),
            self.part,
            self.request.cause
        )
    }
}
impl std::error::Error for FormulaScalarSourceRecipeSourceError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        cause_source(&self.request.cause)
    }
}
/// Original first paired-mode refusal;no partial accepted source roots or recipe outcomes.
#[derive(Debug)]
pub enum FormulaScalarSourceRecipeErrorCause<'a> {
    /// Source graph and recipe plan use different actual canonical inventories.
    ForeignInputs {
        /// Actual rejected source inventory;the enclosing error retains the recipe inventory.
        sources: &'a CanonicalInputs,
    },
    /// Original source runtime preparation cause,including inline allocator failure before values.
    Sources(FormulaScalarSourceGraphExecutionCause<'a>),
    /// Original recipe reservation,operand,storage,tolerance/comparison or assertion failure.
    Recipe(FormulaScalarRecipeError<'a>),
    /// Original actual requesting operand and executed supplier refusal;semantic context is boxed.
    RequestedSource(FormulaScalarSourceRecipeSourceError<'a>),
    /// Original allocation cause encountered while demanding a real supplier;no new box is made.
    RequestedSourceAllocation {
        /// Actual original requesting statement.
        statement: &'a FormulaCheckedStatement<'a>,
        /// Actual requesting operand role.
        part: FormulaStatementExpression,
        /// Real requesting name span;no allocation-error AST node is fabricated.
        span: FormulaSourceSpan,
        /// Original inline allocation workspace/index/count/error.
        cause: FormulaScalarSourceGraphExecutionCause<'a>,
    },
}
/// Original paired proof/inventory/graph and both selected policies survive whole execution failure.
/// All fields are private;borrowed typed causes stay inspectable instead of being erased to tokens.
pub struct FormulaScalarSourceRecipeError<'a> {
    pub(super) recipe: &'a FormulaCheckedRecipe<'a>,
    pub(super) inputs: &'a CanonicalInputs,
    pub(super) graph: &'a FormulaAcyclicSourceDependencies<'a>,
    pub(super) source_limits: FormulaSourceGraphExecutionLimits,
    pub(super) source_tangent_limits: Option<crate::recipe::FormulaTangentLimits>,
    pub(super) recipe_limits: FormulaRecipeExecutionLimits,
    pub(super) recipe_tangent_limits: Option<crate::recipe::FormulaTangentLimits>,
    pub(super) cause: FormulaScalarSourceRecipeErrorCause<'a>,
}
impl<'a> FormulaScalarSourceRecipeError<'a> {
    /// Actual original complete ordered recipe proof.
    #[must_use]
    pub const fn recipe(&self) -> &'a FormulaCheckedRecipe<'a> {
        self.recipe
    }
    /// Actual recipe canonical inventory;accepted execution requires the same source owner.
    #[must_use]
    pub const fn inputs(&self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Actual complete acyclic source relation,including original source proofs and owners.
    #[must_use]
    pub const fn graph(&self) -> &'a FormulaAcyclicSourceDependencies<'a> {
        self.graph
    }
    /// Explicit original source graph execution allowances.
    #[must_use]
    pub const fn source_limits(&self) -> FormulaSourceGraphExecutionLimits {
        self.source_limits
    }
    /// Original source-graph per-call tangent policy,including pre-value ownership failure.
    #[must_use]
    pub const fn source_tangent_limits(&self) -> Option<crate::recipe::FormulaTangentLimits> {
        self.source_tangent_limits
    }
    /// Explicit original whole recipe execution allowances.
    #[must_use]
    pub const fn recipe_limits(&self) -> FormulaRecipeExecutionLimits {
        self.recipe_limits
    }
    /// Original recipe-owned per-call policy,independent of the source graph's selected limits.
    #[must_use]
    pub const fn recipe_tangent_limits(&self) -> Option<crate::recipe::FormulaTangentLimits> {
        self.recipe_tangent_limits
    }
    /// Original first ownership/preparation/recipe/requester/supplier cause.
    #[must_use]
    pub const fn cause(&self) -> &FormulaScalarSourceRecipeErrorCause<'a> {
        &self.cause
    }
}
impl fmt::Debug for FormulaScalarSourceRecipeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaScalarSourceRecipeError")
            .field("recipe", &self.recipe)
            .field("inputs", &self.inputs)
            .field("graph", &self.graph)
            .field("source_limits", &self.source_limits)
            .field("source_tangent_limits", &self.source_tangent_limits)
            .field("recipe_limits", &self.recipe_limits)
            .field("recipe_tangent_limits", &self.recipe_tangent_limits)
            .field("cause", &self.cause)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for FormulaScalarSourceRecipeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.cause {
            FormulaScalarSourceRecipeErrorCause::ForeignInputs { .. } => {
                f.write_str("recipe and source graph use different actual canonical owners")
            }
            FormulaScalarSourceRecipeErrorCause::Sources(cause) => {
                write!(f, "source runtime preparation: {cause:?}")
            }
            FormulaScalarSourceRecipeErrorCause::Recipe(error) => fmt::Display::fmt(error, f),
            FormulaScalarSourceRecipeErrorCause::RequestedSource(error) => {
                fmt::Display::fmt(error, f)
            }
            FormulaScalarSourceRecipeErrorCause::RequestedSourceAllocation {
                statement,
                part,
                cause,
                ..
            } => write!(
                f,
                "recipe statement {} {part:?} source allocation: {cause:?}",
                statement.statement_index()
            ),
        }
    }
}
impl std::error::Error for FormulaScalarSourceRecipeError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.cause {
            FormulaScalarSourceRecipeErrorCause::ForeignInputs { .. } => None,
            FormulaScalarSourceRecipeErrorCause::Sources(cause)
            | FormulaScalarSourceRecipeErrorCause::RequestedSourceAllocation { cause, .. } => {
                cause_source(cause)
            }
            FormulaScalarSourceRecipeErrorCause::Recipe(error) => std::error::Error::source(error),
            FormulaScalarSourceRecipeErrorCause::RequestedSource(error) => {
                std::error::Error::source(error)
            }
        }
    }
}
