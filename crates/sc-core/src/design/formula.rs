//! Staged actual-Design formula ownership;logical held-content admission precedes semantic checks.
use super::{
    charge_design_input_resources, DesignInputs, DesignInputsError, DesignMetadataHost,
    DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits, DesignResourceMeter,
    DesignResourceSource,
};
use crate::recipe::{
    FormulaNormalizedRecipe, FormulaNormalizedSources, FormulaRecipeDocumentError,
    FormulaSourceInputs, FormulaSourceNormalizationError, FormulaSourceNormalizationLimits,
};
use core::fmt;

/// Original held-content or complete current-input refusal;no formula-language token is invented.
#[derive(Debug)]
pub enum DesignFormulaScopeRefusal<E> {
    /// Original first cumulative actual-input content refusal with its original location.
    Resources(DesignResourceLimitError),
    /// Original global/current input or actual source-reference refusal,after content admission.
    Inputs(DesignInputsError<E>),
}
/// Failed preparation pins the actual root/limits and original typed semantic/content refusal.
/// Boxing keeps this uncommon semantic error compact;normalization allocation errors stay in
/// their existing separate factory return types and are not boxed by this wrapper.
pub struct DesignFormulaScopeError<'a, H: DesignMetadataHost> {
    root: &'a DesignInputs<H>,
    limits: DesignResourceLimits,
    refusal: Box<DesignFormulaScopeRefusal<H::Error>>,
}
impl<H: DesignMetadataHost> fmt::Debug for DesignFormulaScopeError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignFormulaScopeError")
            .field("root", &self.root)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl<'a, H: DesignMetadataHost> DesignFormulaScopeError<'a, H> {
    /// Actual original Design input owner,including its canonical inventory and formula documents.
    #[must_use]
    pub const fn root(&self) -> &'a DesignInputs<H> {
        self.root
    }
    /// Actual explicit logical input-content allowances.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.limits
    }
    /// Original complete content/current-input cause;no accepted scope or normalized prefix escapes.
    #[must_use]
    pub fn refusal(&self) -> &DesignFormulaScopeRefusal<H::Error> {
        &self.refusal
    }
}
impl<H: DesignMetadataHost> fmt::Display for DesignFormulaScopeError<'_, H>
where
    H::Error: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.refusal.as_ref() {
            DesignFormulaScopeRefusal::Resources(error) => {
                write!(f, "Design formula content: {error}")
            }
            DesignFormulaScopeRefusal::Inputs(error) => write!(f, "Design formula inputs: {error}"),
        }
    }
}
impl<H: DesignMetadataHost> std::error::Error for DesignFormulaScopeError<'_, H>
where
    H::Error: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.refusal.as_ref() {
            DesignFormulaScopeRefusal::Resources(error) => Some(error),
            DesignFormulaScopeRefusal::Inputs(error) => Some(error),
        }
    }
}

/// Private actual-root scope after complete logical input-content and current ownership checks.
/// Copying this borrowed view copies no model,host,namespace,arena or result. It is not a source
/// body kind/cycle/value,namespace/worker scratch,allocator/CPU,whole-Design or approval certificate.
/// ```compile_fail
/// fn substitute<'a,H>(scope:&mut sc_core::design::DesignFormulaScope<'a,H>,
///     other:&'a sc_core::design::DesignInputs<H>) {scope.root=other;}
/// ```
/// ```compile_fail
/// fn escape<H:sc_core::design::DesignResourceSource>(root:sc_core::design::DesignInputs<H>,
///     limits:sc_core::design::DesignResourceLimits)->sc_core::design::DesignFormulaScope<'static,H> {
///     root.prepare_formula_scope(limits).unwrap()
/// }
/// ```
pub struct DesignFormulaScope<'a, H> {
    root: &'a DesignInputs<H>,
    limits: DesignResourceLimits,
    demand: DesignResourceDemand,
    joined: FormulaSourceInputs<'a>,
}
impl<H> Copy for DesignFormulaScope<'_, H> {}
impl<H> Clone for DesignFormulaScope<'_, H> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<H> fmt::Debug for DesignFormulaScope<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignFormulaScope")
            .field("root", &self.root)
            .field("demand", &self.demand)
            .finish_non_exhaustive()
    }
}
impl<'a, H> DesignFormulaScope<'a, H> {
    /// Actual same immutable containing Design input owner;not an equal-looking copy or id-only pin.
    #[must_use]
    pub const fn root(self) -> &'a DesignInputs<H> {
        self.root
    }
    /// Original selected logical input-content limits,not a source/runtime default policy.
    #[must_use]
    pub const fn limits(self) -> DesignResourceLimits {
        self.limits
    }
    /// Actual complete charged held input content under the honest/exhaustive source-port contract.
    #[must_use]
    pub const fn demand(self) -> DesignResourceDemand {
        self.demand
    }
    /// Existing actual canonical/source join over exactly this root's original owners.
    #[must_use]
    pub const fn source_inputs(self) -> FormulaSourceInputs<'a> {
        self.joined
    }
    /// Normalize the complete actual source collection through its existing bounded factory.
    /// No foreign source set,namespace,canonical owner or values can be supplied to this method.
    /// # Errors
    /// Returns the original complete source normalization refusal without wrapping allocator causes.
    pub fn normalize_sources(
        self,
        limits: FormulaSourceNormalizationLimits,
    ) -> Result<DesignNormalizedFormulaSources<'a, H>, FormulaSourceNormalizationError<'a>> {
        let normalized = self.joined.normalize_sources(limits)?;
        Ok(DesignNormalizedFormulaSources {
            scope: self,
            normalized,
        })
    }
}
impl<'a, H: DesignMetadataHost> DesignFormulaScope<'a, H> {
    /// Normalize this actual Design's original ordered recipe through its existing document factory.
    /// Its fixed syntax/input limits remain authoritative;this does not check names,kinds or values.
    /// # Errors
    /// Returns the original syntax/literal error without substituting another source or recipe.
    pub fn normalize_recipe(
        self,
    ) -> Result<DesignNormalizedFormulaRecipe<'a, H>, FormulaRecipeDocumentError> {
        let normalized = self.root.recipe().normalized()?;
        Ok(DesignNormalizedFormulaRecipe {
            scope: self,
            normalized,
        })
    }
}
/// Complete original source arenas with their actual admitted containing Design scope.
/// No self-reference or caller-supplied arena is stored;later proofs borrow this owner.
/// ```compile_fail
/// fn replace<H>(sources:&mut sc_core::design::DesignNormalizedFormulaSources<'_,H>) {
///     sources.normalized.sources.clear();
/// }
/// ```
pub struct DesignNormalizedFormulaSources<'a, H> {
    scope: DesignFormulaScope<'a, H>,
    normalized: FormulaNormalizedSources<'a>,
}
impl<'a, H> DesignNormalizedFormulaSources<'a, H> {
    /// Actual root/current-content scope used by this complete normalization.
    #[must_use]
    pub const fn scope(&self) -> DesignFormulaScope<'a, H> {
        self.scope
    }
    /// Actual complete normalized source owners;shared access grants no Design-backed kind proof.
    #[must_use]
    pub const fn normalized(&self) -> &FormulaNormalizedSources<'a> {
        &self.normalized
    }
}
impl<H> fmt::Debug for DesignNormalizedFormulaSources<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignNormalizedFormulaSources")
            .field("scope", &self.scope)
            .field("normalized", &self.normalized)
            .finish_non_exhaustive()
    }
}
/// Actual normalized ordered recipe with its actual admitted containing Design scope.
/// No checked statements,previous-let values or source results are manufactured.
pub struct DesignNormalizedFormulaRecipe<'a, H> {
    scope: DesignFormulaScope<'a, H>,
    normalized: FormulaNormalizedRecipe<'a>,
}
impl<'a, H> DesignNormalizedFormulaRecipe<'a, H> {
    /// Actual root/current-content scope;an empty recipe still pins its real containing owner.
    #[must_use]
    pub const fn scope(&self) -> DesignFormulaScope<'a, H> {
        self.scope
    }
    /// Actual normalized recipe in authored order;later whole static proofs borrow this owner.
    #[must_use]
    pub const fn normalized(&self) -> &FormulaNormalizedRecipe<'a> {
        &self.normalized
    }
}
impl<H> fmt::Debug for DesignNormalizedFormulaRecipe<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignNormalizedFormulaRecipe")
            .field("scope", &self.scope)
            .field("normalized", &self.normalized)
            .finish_non_exhaustive()
    }
}
impl<H: DesignResourceSource> DesignInputs<H> {
    /// Admit every actually held input field,then recheck complete current ownership/source links.
    /// Source resource walking precedes current semantic callbacks;no namespace or numerical value
    /// is read/created. The source port must honestly/exhaustively charge its immutable held data.
    /// Parsing/namespace/provider/worker/kernel scratch and allocator/CPU bounds remain separate.
    /// ```
    /// use sc_core::design::{DesignInputs,DesignResourceLimits,DesignResourceSource};
    /// use sc_core::recipe::FormulaSourceNormalizationLimits;
    /// fn inspect<H:DesignResourceSource>(root:&DesignInputs<H>,content:DesignResourceLimits,
    ///     source_syntax:FormulaSourceNormalizationLimits) {
    ///     let scope=root.prepare_formula_scope(content).expect("actual complete input admission");
    ///     let sources=scope.normalize_sources(source_syntax).expect("complete source syntax");
    ///     let recipe=scope.normalize_recipe().expect("actual recipe syntax");
    ///     assert!(std::ptr::eq(sources.scope().root(),root));
    ///     assert!(std::ptr::eq(recipe.scope().root(),root));
    ///     assert!(std::ptr::eq(sources.normalized().inputs().inputs(),root.inputs()));
    /// }
    /// ```
    /// # Errors
    /// Preserves the first content refusal before semantic validation,then the original complete
    /// input refusal. No accepted scope,cloned model,normalized prefix or implicit policy escapes.
    pub fn prepare_formula_scope(
        &self,
        limits: DesignResourceLimits,
    ) -> Result<DesignFormulaScope<'_, H>, DesignFormulaScopeError<'_, H>> {
        let refuse = |refusal| DesignFormulaScopeError {
            root: self,
            limits,
            refusal: Box::new(refusal),
        };
        let mut meter = DesignResourceMeter::new(limits);
        charge_design_input_resources(self, &mut meter)
            .map_err(|e| refuse(DesignFormulaScopeRefusal::Resources(e)))?;
        let demand = meter
            .finish()
            .map_err(|e| refuse(DesignFormulaScopeRefusal::Resources(e)))?;
        self.validate_current()
            .map_err(|e| refuse(DesignFormulaScopeRefusal::Inputs(e)))?;
        let Ok(joined) = FormulaSourceInputs::new(self.inputs(), self.formula_sources()) else {
            unreachable!(
                "complete current inputs checked these same immutable canonical/source owners"
            );
        };
        Ok(DesignFormulaScope {
            root: self,
            limits,
            demand,
            joined,
        })
    }
}
