//! Actual containing Design execution owns fresh logical admission and complete native results.
use super::{DesignFormulaPlanInputs, DesignFormulaScope, DesignFormulaScopeError};
use crate::{
    design::DesignMetadataHost,
    recipe::{FormulaExecutedScalarSourceRecipe, FormulaScalarSourceRecipeError},
};
use core::fmt;

/// First actual-root admission or complete native execution refusal;no accepted prefix.
// Keep native allocator failures and fresh logical scope inline rather than allocating a wrapper
// while reporting allocation failure. The admission factory already compacts semantic refusals.
#[allow(clippy::large_enum_variant)]
pub enum DesignFormulaExecutionError<'a, H: DesignMetadataHost> {
    /// Fresh complete actual held-input/current admission refused before runtime preparation/values.
    Admission {
        /// Original private same-root proof/policy join.
        inputs: &'a DesignFormulaPlanInputs<'a, H>,
        /// Original root/model limits and actual first resource/current cause.
        error: DesignFormulaScopeError<'a, H>,
    },
    /// Fresh actual input admission succeeded;the complete native operation refused atomically.
    Execution {
        /// Original actual source/recipe proof/policy join.
        inputs: &'a DesignFormulaPlanInputs<'a, H>,
        /// Actual fresh logical held-input/current admission,not a whole allocator/CPU certificate.
        scope: DesignFormulaScope<'a, H>,
        /// Original paired native owners/policies/requester/supplier/value or allocator cause.
        error: FormulaScalarSourceRecipeError<'a>,
    },
}
impl<'a, H: DesignMetadataHost> DesignFormulaExecutionError<'a, H> {
    /// Original exact-root proof owners and selected native execution policies.
    #[must_use]
    pub const fn inputs(&self) -> &'a DesignFormulaPlanInputs<'a, H> {
        match self {
            Self::Admission { inputs, .. } | Self::Execution { inputs, .. } => inputs,
        }
    }
    /// Fresh accepted input scope only after actual complete admission;None for admission failure.
    #[must_use]
    pub const fn scope(&self) -> Option<DesignFormulaScope<'a, H>> {
        match self {
            Self::Admission { .. } => None,
            Self::Execution { scope, .. } => Some(*scope),
        }
    }
}
impl<H: DesignMetadataHost> fmt::Debug for DesignFormulaExecutionError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admission { inputs, error } => f
                .debug_struct("Admission")
                .field("inputs", inputs)
                .field("error", error)
                .finish(),
            Self::Execution {
                inputs,
                scope,
                error,
            } => f
                .debug_struct("Execution")
                .field("inputs", inputs)
                .field("scope", scope)
                .field("error", error)
                .finish(),
        }
    }
}
impl<H: DesignMetadataHost> fmt::Display for DesignFormulaExecutionError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admission { error, .. } => fmt::Display::fmt(error, f),
            Self::Execution { error, .. } => fmt::Display::fmt(error, f),
        }
    }
}
impl<H: DesignMetadataHost> std::error::Error for DesignFormulaExecutionError<'_, H> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Admission { error, .. } => std::error::Error::source(error),
            Self::Execution { error, .. } => std::error::Error::source(error),
        }
    }
}
/// Complete actual Design-backed scalar recipe execution,with fresh input scope and original plans.
/// No model state is mutated;unbound source roots and real once-stored recipe bindings stay distinct.
/// ```compile_fail
/// fn seed<H>(result:&mut sc_core::design::DesignExecutedFormulaRecipe<'_,H>) {result.execution.recipe.statements.clear();}
/// ```
/// ```compile_fail
/// fn escape<H>(result:sc_core::design::DesignExecutedFormulaRecipe<'_,H>)
///     ->sc_core::design::DesignFormulaScope<'static,H> {result.scope()}
/// ```
pub struct DesignExecutedFormulaRecipe<'a, H> {
    pub(super) inputs: &'a DesignFormulaPlanInputs<'a, H>,
    pub(super) scope: DesignFormulaScope<'a, H>,
    pub(super) execution: FormulaExecutedScalarSourceRecipe<'a>,
}
impl<'a, H> DesignExecutedFormulaRecipe<'a, H> {
    /// Original exact actual root/source/recipe proof owners and selected native policies.
    #[must_use]
    pub const fn inputs(&self) -> &'a DesignFormulaPlanInputs<'a, H> {
        self.inputs
    }
    /// Fresh complete actual logical held-input/current admission under the original model limits.
    #[must_use]
    pub const fn scope(&self) -> DesignFormulaScope<'a, H> {
        self.scope
    }
    /// Complete original source roots,recipe outcomes/read witnesses,context and native plans.
    #[must_use]
    pub const fn execution(&self) -> &FormulaExecutedScalarSourceRecipe<'a> {
        &self.execution
    }
}
impl<H> fmt::Debug for DesignExecutedFormulaRecipe<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignExecutedFormulaRecipe")
            .field("scope", &self.scope)
            .field("execution", &self.execution)
            .finish_non_exhaustive()
    }
}
