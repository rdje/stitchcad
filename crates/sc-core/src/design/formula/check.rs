//! Actual-root static checking uses existing factories and borrows original document owners.
use super::{DesignFormulaScope, DesignNormalizedFormulaRecipe, DesignNormalizedFormulaSources};
use crate::{
    design::{DesignFormulaError, DesignFormulaHost, DesignMetadataHost},
    recipe::{
        FormulaCheckedRecipe, FormulaCheckedSources, FormulaRecipeCheckError,
        FormulaSourceCheckError, FormulaSourceCheckLimits,
    },
};
use core::fmt;

/// Original actual-namespace or complete-source refusal,with its actual normalized Design owner.
/// Allocation causes stay in the original factory error;no additional diagnostic box is allocated.
pub enum DesignFormulaSourceCheckError<'a, H: DesignMetadataHost> {
    /// Current actual Design namespace construction failed before source checking.
    Namespace {
        /// Actual document owner;no caller namespace or foreign root was accepted.
        normalized: &'a DesignNormalizedFormulaSources<'a, H>,
        /// Explicit source-proof allowances selected for this attempt.
        limits: FormulaSourceCheckLimits,
        /// Original current/reference/declaration/namespace refusal.
        error: DesignFormulaError<'a, H::Error>,
    },
    /// The existing complete source checker refused without an accepted prefix.
    Sources {
        /// Actual containing Design normalization owner.
        normalized: &'a DesignNormalizedFormulaSources<'a, H>,
        /// Original complete quota,allocation,expression,canonical or consumer-kind cause.
        error: FormulaSourceCheckError<'a>,
    },
}
impl<'a, H: DesignMetadataHost> DesignFormulaSourceCheckError<'a, H> {
    /// Actual normalized Design source owner used by this attempt,including its scope/root.
    #[must_use]
    pub const fn normalized(&self) -> &'a DesignNormalizedFormulaSources<'a, H> {
        match self {
            Self::Namespace { normalized, .. } | Self::Sources { normalized, .. } => normalized,
        }
    }
    /// Original explicit source-proof allowances,including when namespace construction refused.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceCheckLimits {
        match self {
            Self::Namespace { limits, .. } => *limits,
            Self::Sources { error, .. } => error.limits(),
        }
    }
}
impl<H: DesignMetadataHost> fmt::Debug for DesignFormulaSourceCheckError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace {
                normalized,
                limits,
                error,
            } => f
                .debug_struct("Namespace")
                .field("normalized", normalized)
                .field("limits", limits)
                .field("error", error)
                .finish(),
            Self::Sources { normalized, error } => f
                .debug_struct("Sources")
                .field("normalized", normalized)
                .field("error", error)
                .finish(),
        }
    }
}
impl<H: DesignMetadataHost> fmt::Display for DesignFormulaSourceCheckError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace { error, .. } => fmt::Display::fmt(error, f),
            Self::Sources { error, .. } => fmt::Display::fmt(error, f),
        }
    }
}
impl<H: DesignMetadataHost> std::error::Error for DesignFormulaSourceCheckError<'_, H> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Borrowed typed causes remain in the variants;forward only actual static factory causes.
        match self {
            Self::Namespace { error, .. } => std::error::Error::source(error),
            Self::Sources { error, .. } => std::error::Error::source(error),
        }
    }
}
/// Complete source kind/canonical/consumer proofs from the actual containing Design namespace.
/// No cycle,provider,value,recipe execution or whole-Design resource certificate follows.
/// ```compile_fail
/// fn replace<H>(proof:&mut sc_core::design::DesignCheckedFormulaSources<'_,H>,
///     other:sc_core::recipe::FormulaCheckedSources<'_>) {proof.proof=other;}
/// ```
pub struct DesignCheckedFormulaSources<'a, H> {
    normalized: &'a DesignNormalizedFormulaSources<'a, H>,
    proof: FormulaCheckedSources<'a>,
}
impl<'a, H> DesignCheckedFormulaSources<'a, H> {
    /// Actual normalized Design owner,including its original containing scope/root.
    #[must_use]
    pub const fn normalized(&self) -> &'a DesignNormalizedFormulaSources<'a, H> {
        self.normalized
    }
    /// Complete existing source proof with original expressions/canonical targets and limits.
    #[must_use]
    pub const fn proof(&self) -> &FormulaCheckedSources<'a> {
        &self.proof
    }
    /// Actual containing admitted input scope;identity is the original owner reference.
    #[must_use]
    pub const fn scope(&self) -> DesignFormulaScope<'a, H> {
        self.normalized.scope()
    }
}
impl<H> fmt::Debug for DesignCheckedFormulaSources<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignCheckedFormulaSources")
            .field("normalized", &self.normalized)
            .field("proof", &self.proof)
            .finish_non_exhaustive()
    }
}
impl<'r, H: DesignFormulaHost> DesignNormalizedFormulaSources<'r, H> {
    /// Build the actual current Design namespace,then invoke complete existing source checking.
    /// Namespace creation/current checking precedes source-proof quota checks;namespace/checker
    /// scratch and allocator/CPU bounds remain separate from held content and retained-proof quotas.
    /// No namespace,metadata,canonical inventory,values or supplied proof is accepted from a caller.
    /// ```compile_fail
    /// fn supplied<H:sc_core::design::DesignFormulaHost>(
    ///     owner:&sc_core::design::DesignNormalizedFormulaSources<'_,H>,
    ///     foreign:&sc_core::recipe::FormulaNamespace<'_>,
    ///     limits:sc_core::recipe::FormulaSourceCheckLimits) {owner.check_sources(foreign,limits);}
    /// ```
    /// # Errors
    /// Keeps original actual namespace or complete source failure and original owner/selected limits.
    pub fn check_sources<'a>(
        &'a self,
        limits: FormulaSourceCheckLimits,
    ) -> Result<DesignCheckedFormulaSources<'a, H>, DesignFormulaSourceCheckError<'a, H>> {
        let namespace = self.scope.root.formula_namespace().map_err(|error| {
            DesignFormulaSourceCheckError::Namespace {
                normalized: self,
                limits,
                error,
            }
        })?;
        let proof = self
            .normalized
            .check_sources(&namespace, limits)
            .map_err(|error| DesignFormulaSourceCheckError::Sources {
                normalized: self,
                error,
            })?;
        Ok(DesignCheckedFormulaSources {
            normalized: self,
            proof,
        })
    }
}

/// Original actual-namespace or ordered-recipe refusal,with its actual normalized Design owner.
pub enum DesignFormulaRecipeCheckError<'a, H: DesignMetadataHost> {
    /// The actual Design namespace failed before recipe checking.
    Namespace {
        /// Actual normalized ordered recipe and containing Design scope.
        normalized: &'a DesignNormalizedFormulaRecipe<'a, H>,
        /// Original current/reference/declaration/namespace cause.
        error: DesignFormulaError<'a, H::Error>,
    },
    /// The existing whole ordered checker failed;no accepted prefix escapes.
    Recipe {
        /// Actual normalized ordered recipe and containing Design scope.
        normalized: &'a DesignNormalizedFormulaRecipe<'a, H>,
        /// Original actual statement/index/span/token/nested static cause.
        error: FormulaRecipeCheckError<'a>,
    },
}
impl<'a, H: DesignMetadataHost> DesignFormulaRecipeCheckError<'a, H> {
    /// Original actual normalized recipe owner,including its root even for an empty recipe.
    #[must_use]
    pub const fn normalized(&self) -> &'a DesignNormalizedFormulaRecipe<'a, H> {
        match self {
            Self::Namespace { normalized, .. } | Self::Recipe { normalized, .. } => normalized,
        }
    }
}
impl<H: DesignMetadataHost> fmt::Debug for DesignFormulaRecipeCheckError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace { normalized, error } => f
                .debug_struct("Namespace")
                .field("normalized", normalized)
                .field("error", error)
                .finish(),
            Self::Recipe { normalized, error } => f
                .debug_struct("Recipe")
                .field("normalized", normalized)
                .field("error", error)
                .finish(),
        }
    }
}
impl<H: DesignMetadataHost> fmt::Display for DesignFormulaRecipeCheckError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace { error, .. } => fmt::Display::fmt(error, f),
            Self::Recipe { error, .. } => fmt::Display::fmt(error, f),
        }
    }
}
impl<H: DesignMetadataHost> std::error::Error for DesignFormulaRecipeCheckError<'_, H> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Namespace { error, .. } => std::error::Error::source(error),
            Self::Recipe { error, .. } => std::error::Error::source(error),
        }
    }
}
/// Complete actual ordered recipe proof from this Design's real initial metadata.
/// Prior-let visibility comes only from actual earlier statements;no values or stored bindings exist.
/// ```compile_fail
/// fn escape<H:sc_core::design::DesignFormulaHost>(owner:sc_core::design::DesignNormalizedFormulaRecipe<'_,H>)
///     ->sc_core::design::DesignCheckedFormulaRecipe<'static,H> {owner.check_kinds().unwrap()}
/// ```
pub struct DesignCheckedFormulaRecipe<'a, H> {
    normalized: &'a DesignNormalizedFormulaRecipe<'a, H>,
    proof: FormulaCheckedRecipe<'a>,
}
impl<'a, H> DesignCheckedFormulaRecipe<'a, H> {
    /// Actual complete normalized Design recipe owner.
    #[must_use]
    pub const fn normalized(&self) -> &'a DesignNormalizedFormulaRecipe<'a, H> {
        self.normalized
    }
    /// Original whole recipe proof in authored order,with actual prior-let dependencies.
    #[must_use]
    pub const fn proof(&self) -> &FormulaCheckedRecipe<'a> {
        &self.proof
    }
    /// Actual original containing admitted input scope.
    #[must_use]
    pub const fn scope(&self) -> DesignFormulaScope<'a, H> {
        self.normalized.scope()
    }
}
impl<H> fmt::Debug for DesignCheckedFormulaRecipe<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignCheckedFormulaRecipe")
            .field("normalized", &self.normalized)
            .field("proof", &self.proof)
            .finish_non_exhaustive()
    }
}
impl<'r, H: DesignFormulaHost> DesignNormalizedFormulaRecipe<'r, H> {
    /// Build this actual current Design namespace,then check the whole original ordered recipe.
    /// Existing current/name/header/kind ordering and all-branch checking remain authoritative.
    /// This performs no numerical reads/storage and grants no whole runtime resource certificate.
    /// ```
    /// use sc_core::{design::{DesignInputs,DesignResourceLimits,DesignResourceSource,DesignFormulaHost},
    ///     recipe::{FormulaSourceNormalizationLimits,FormulaSourceCheckLimits}};
    /// fn inspect<H:DesignResourceSource+DesignFormulaHost>(root:&DesignInputs<H>,content:DesignResourceLimits,
    ///     syntax:FormulaSourceNormalizationLimits,source_checks:FormulaSourceCheckLimits) {
    ///     let scope=root.prepare_formula_scope(content).unwrap();
    ///     let sources=scope.normalize_sources(syntax).unwrap();
    ///     let checked_sources=sources.check_sources(source_checks).unwrap();
    ///     let recipe=scope.normalize_recipe().unwrap();
    ///     let checked_recipe=recipe.check_kinds().unwrap();
    ///     assert!(std::ptr::eq(checked_sources.scope().root(),checked_recipe.scope().root()));
    /// }
    /// ```
    /// ```compile_fail
    /// fn supplied<H:sc_core::design::DesignFormulaHost>(
    ///     owner:&sc_core::design::DesignNormalizedFormulaRecipe<'_,H>,
    ///     foreign:sc_core::recipe::FormulaNamespace<'_>) {owner.check_kinds(foreign);}
    /// ```
    /// # Errors
    /// Keeps the original actual namespace or whole recipe error with actual owner/statement causes.
    pub fn check_kinds<'a>(
        &'a self,
    ) -> Result<DesignCheckedFormulaRecipe<'a, H>, DesignFormulaRecipeCheckError<'a, H>> {
        let namespace = self.scope.root.formula_namespace().map_err(|error| {
            DesignFormulaRecipeCheckError::Namespace {
                normalized: self,
                error,
            }
        })?;
        let proof = self.normalized.check_kinds(namespace).map_err(|error| {
            DesignFormulaRecipeCheckError::Recipe {
                normalized: self,
                error,
            }
        })?;
        Ok(DesignCheckedFormulaRecipe {
            normalized: self,
            proof,
        })
    }
}
