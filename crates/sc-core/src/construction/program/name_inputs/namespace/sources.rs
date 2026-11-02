//! Complete existing Root source checking through the actual bound mixed initial namespace.
use super::DraftingProgramInitialNamespace;
use crate::{
    construction::{BoundDraftingProgramInitials, DraftingInitialInputProvider},
    design::{DesignMetadataHost, DesignNormalizedFormulaSources},
    recipe::{FormulaCheckedSources, FormulaSourceCheckError, FormulaSourceCheckLimits},
};
use core::fmt;

/// Original native source-check cause and actual containing inputs/normalization owner.
/// The native allocator cause remains inline;the wrapper allocates no diagnostic box.
pub struct DraftingProgramSourceCheckError<
    'q,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    inputs: &'q BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    normalized: &'q DesignNormalizedFormulaSources<'a, H>,
    cause: FormulaSourceCheckError<'q>,
}
impl<
        'q,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramSourceCheckError<'q, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Original complete initial/output source associations and their actual programme/Root.
    #[must_use]
    pub const fn inputs(&self) -> &'q BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Exact actual normalized Root source owner,not another normalization of equal text.
    #[must_use]
    pub const fn normalized(&self) -> &'q DesignNormalizedFormulaSources<'a, H> {
        self.normalized
    }
    /// Original explicit native source policy,including on reservation/static refusal.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceCheckLimits {
        self.cause.limits()
    }
    /// Complete original quota/allocation/expression/canonical/consumer cause and witnesses.
    #[must_use]
    pub const fn cause(&self) -> &FormulaSourceCheckError<'q> {
        &self.cause
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramSourceCheckError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramSourceCheckError")
            .field("limits", &self.limits())
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Display
    for DraftingProgramSourceCheckError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("complete actual Root source checking refused")
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> std::error::Error
    for DraftingProgramSourceCheckError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.source()
    }
}

/// Complete original Root source kind/canonical-consumer proof under actual bound initial names.
/// Original declarations and normalized sources stay borrowed;the namespace allocation is not
/// borrowed by this owner. Mixed checking may subsequently consume the actual initial map.
/// This grants no source cycle,geometry/semantic dependency,value or whole-programme acceptance.
/// ```compile_fail,E0616
/// use sc_core::{construction::{CheckedDraftingProgramSources,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     proof:&mut CheckedDraftingProgramSources<'_, '_, '_, '_, '_, '_, '_,H,S>) {proof.proof=todo!();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::{CheckedDraftingProgramSources,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn escape<'q,'i,'r,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     proof:CheckedDraftingProgramSources<'q,'i,'r,'b,'n,'p,'a,H,S>)
///     ->CheckedDraftingProgramSources<'static,'i,'r,'b,'n,'p,'a,H,S> {proof}
/// ```
pub struct CheckedDraftingProgramSources<
    'q,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    inputs: &'q BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    normalized: &'q DesignNormalizedFormulaSources<'a, H>,
    proof: FormulaCheckedSources<'q>,
}
impl<
        'q,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > CheckedDraftingProgramSources<'q, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Exact complete original initial/output associations,programme/Root and name-work policy.
    #[must_use]
    pub const fn inputs(&self) -> &'q BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Exact actual Root normalization object containing all original source arenas.
    #[must_use]
    pub const fn normalized(&self) -> &'q DesignNormalizedFormulaSources<'a, H> {
        self.normalized
    }
    /// Complete native source proof,original explicit limits and actual canonical consumers.
    #[must_use]
    pub const fn proof(&self) -> &FormulaCheckedSources<'q> {
        &self.proof
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for CheckedDraftingProgramSources<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CheckedDraftingProgramSources")
            .field("source_count", &self.proof.sources().len())
            .finish_non_exhaustive()
    }
}
impl<
        's,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramInitialNamespace<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Check every actual Root formula source through the existing complete factory against this
    /// actual private initial map. Whole source/node/Derived-target/proof bounds precede allocation
    /// and source checking in the native factory. No supplied namespace,foreign source corpus,
    /// provider callback,value,metadata revalidation or cloned map is introduced.
    ///
    /// The result borrows actual initial declarations and Root source arenas,not this map owner;
    /// it can coexist with subsequent consuming mixed-order checking.
    /// ```
    /// use sc_core::{construction::{DraftingProgramInitialNamespace,DraftingInitialInputProvider,
    ///     DraftingProgramOrderLimits},design::DesignMetadataHost,recipe::FormulaSourceCheckLimits};
    /// fn source_and_step_counts<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     names:DraftingProgramInitialNamespace<'_, '_, '_, '_, '_, '_, '_,H,S>,
    ///     source_policy:FormulaSourceCheckLimits,order_policy:DraftingProgramOrderLimits)
    ///     ->Result<(usize,usize),String> {
    ///     let sources=names.check_root_sources(source_policy).map_err(|error|error.to_string())?;
    ///     let order=names.check_order(&order_policy).map_err(|error|error.to_string())?;
    ///     Ok((sources.proof().sources().len(),order.steps().len()))
    /// }
    /// ```
    /// # Errors
    /// Original complete native quota/allocator/expression/canonical/consumer-kind cause,retaining
    /// actual input/normalization owners and selected limits. No successful local prefix escapes.
    pub fn check_root_sources<'q>(
        &self,
        limits: FormulaSourceCheckLimits,
    ) -> Result<
        CheckedDraftingProgramSources<'q, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingProgramSourceCheckError<'q, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    >
    where
        's: 'q,
    {
        let inputs = self.inputs;
        let normalized = inputs
            .roles()
            .positions()
            .work()
            .inputs()
            .program()
            .sources();
        let proof = normalized
            .normalized()
            .check_sources(&self.namespace, limits)
            .map_err(|cause| DraftingProgramSourceCheckError {
                inputs,
                normalized,
                cause,
            })?;
        Ok(CheckedDraftingProgramSources {
            inputs,
            normalized,
            proof,
        })
    }
}
