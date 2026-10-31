//! Fresh containing-Design admission precedes every borrowed-plan/result replay attempt.
use super::{
    DesignExecutedFormulaRecipe, DesignFormulaPlanInputs, DesignFormulaScalarPlan,
    DesignFormulaScope, DesignFormulaScopeError,
};
use crate::{
    design::{DesignMetadataHost, DesignResourceSource},
    recipe::{
        FormulaScalarExecutionContext, FormulaScalarRecipePlan, FormulaScalarSourceGraphPlan,
        FormulaScalarSourceRecipeReplayError,
    },
};
use core::fmt;

/// Actual containing-root admission or fresh native replay refusal; no accepted partial result.
// Preserve original allocator causes and fresh logical scope inline during error reporting.
#[allow(clippy::large_enum_variant)]
pub enum DesignFormulaReplayError<'a, H: DesignMetadataHost> {
    /// Current complete held-input admission refused before native preparation or values.
    Admission {
        /// Original private same-root proof and policy join.
        inputs: &'a DesignFormulaPlanInputs<'a, H>,
        /// Original root, model limits and actual first content/current refusal.
        error: DesignFormulaScopeError<'a, H>,
    },
    /// Fresh containing-root admission succeeded; fresh native preparation/execution refused.
    Replay {
        /// Original actual source/recipe proof owners and both selected policies.
        inputs: &'a DesignFormulaPlanInputs<'a, H>,
        /// Actual fresh logical held-input/current admission, not whole Design approval.
        scope: DesignFormulaScope<'a, H>,
        /// Original native replay stage and its typed owner/policy/requester/supplier cause.
        error: FormulaScalarSourceRecipeReplayError<'a>,
    },
}
impl<'a, H: DesignMetadataHost> DesignFormulaReplayError<'a, H> {
    /// Original exact-root source/recipe proof owners and selected native policies.
    #[must_use]
    pub const fn inputs(&self) -> &'a DesignFormulaPlanInputs<'a, H> {
        match self {
            Self::Admission { inputs, .. } | Self::Replay { inputs, .. } => inputs,
        }
    }
    /// Fresh accepted input scope only after current complete admission succeeds.
    #[must_use]
    pub const fn scope(&self) -> Option<DesignFormulaScope<'a, H>> {
        match self {
            Self::Admission { .. } => None,
            Self::Replay { scope, .. } => Some(*scope),
        }
    }
}
impl<H: DesignMetadataHost> fmt::Debug for DesignFormulaReplayError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admission { inputs, error } => f
                .debug_struct("Admission")
                .field("inputs", inputs)
                .field("error", error)
                .finish(),
            Self::Replay {
                inputs,
                scope,
                error,
            } => f
                .debug_struct("Replay")
                .field("inputs", inputs)
                .field("scope", scope)
                .field("error", error)
                .finish(),
        }
    }
}
impl<H: DesignMetadataHost> fmt::Display for DesignFormulaReplayError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admission { error, .. } => fmt::Display::fmt(error, f),
            Self::Replay { error, .. } => fmt::Display::fmt(error, f),
        }
    }
}
impl<H: DesignMetadataHost> std::error::Error for DesignFormulaReplayError<'_, H> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Admission { error, .. } => std::error::Error::source(error),
            Self::Replay { error, .. } => std::error::Error::source(error),
        }
    }
}

// Both callers own a private same-root plan/result; no caller can replace these joins.
#[allow(clippy::result_large_err)]
fn replay<'a, H: DesignResourceSource>(
    inputs: &'a DesignFormulaPlanInputs<'a, H>,
    recipe: &FormulaScalarRecipePlan<'a>,
    sources: &FormulaScalarSourceGraphPlan<'a>,
    context: FormulaScalarExecutionContext<'a>,
) -> Result<DesignExecutedFormulaRecipe<'a, H>, DesignFormulaReplayError<'a, H>> {
    let original = inputs.scope();
    let scope = original
        .root()
        .prepare_formula_scope(original.limits())
        .map_err(|error| DesignFormulaReplayError::Admission { inputs, error })?;
    let execution = recipe
        .replay_with_sources(sources, context)
        .map_err(|error| DesignFormulaReplayError::Replay {
            inputs,
            scope,
            error,
        })?;
    Ok(DesignExecutedFormulaRecipe {
        inputs,
        scope,
        execution,
    })
}

impl<'p, H: DesignResourceSource> DesignFormulaScalarPlan<'p, H> {
    /// Replay this borrowed actual Design plan after fresh complete input/current admission.
    /// Original model limits, proof/graph owners and both native policies remain pinned. Every
    /// attempt starts at statement one with fresh source/read state. Model and host are not cloned.
    /// # Errors
    /// Admission precedes native preparation/values. Original typed causes survive; failure leaves
    /// this plan and every earlier accepted result intact, with no accepted source/statement prefix.
    #[allow(clippy::result_large_err)]
    pub fn replay<'a>(
        &self,
        context: FormulaScalarExecutionContext<'a>,
    ) -> Result<DesignExecutedFormulaRecipe<'a, H>, DesignFormulaReplayError<'a, H>>
    where
        'p: 'a,
    {
        replay(self.inputs(), self.recipe(), self.sources(), context)
    }
}
impl<'p, H: DesignResourceSource> DesignExecutedFormulaRecipe<'p, H> {
    /// Replay an accepted result's original plans after fresh complete input/current admission.
    /// The earlier result stays available; its old admission and cached values do not authorize
    /// this attempt. Original model/native limits and actual owner/policy identity stay pinned.
    /// ```
    /// use sc_core::{design::*,recipe::FormulaScalarExecutionContext};
    /// fn again<'a,H:DesignResourceSource>(prior:&DesignExecutedFormulaRecipe<'a,H>,
    ///     context:FormulaScalarExecutionContext<'a>)
    ///     ->Result<DesignExecutedFormulaRecipe<'a,H>,DesignFormulaReplayError<'a,H>> {
    ///     prior.replay(context)
    /// }
    /// ```
    /// # Errors
    /// Original admission or native replay failure exposes no accepted prefix and cannot change
    /// this earlier result. Whole-Design readiness, geometry/publication and approval stay separate.
    #[allow(clippy::result_large_err)]
    pub fn replay<'a>(
        &self,
        context: FormulaScalarExecutionContext<'a>,
    ) -> Result<DesignExecutedFormulaRecipe<'a, H>, DesignFormulaReplayError<'a, H>>
    where
        'p: 'a,
    {
        replay(
            self.inputs(),
            self.execution().recipe_plan(),
            self.execution().source_plan(),
            context,
        )
    }
}
