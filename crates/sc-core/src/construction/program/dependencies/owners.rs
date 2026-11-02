//! Exact privately certified source/order/graph owners;no canonical work or whole programme grant.
use crate::{
    construction::{
        CheckedDraftingProgramOrder, CheckedDraftingProgramSources, DraftingInitialInputProvider,
    },
    design::DesignMetadataHost,
    recipe::FormulaAcyclicSourceDependencies,
};
use core::fmt;

/// First selected proof-owner mismatch,before canonical work or allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramDependencyOwnerIssue {
    /// Order refers to a different actual complete initial-association object,even over equal data.
    OrderInputs,
    /// Graph was built from a different actual native source proof,even over the same source corpus.
    SourceGraph,
}
/// Every actual provided proof survives a refused join;no accepted source/order/graph prefix.
pub struct DraftingProgramDependencyOwnerError<
    'j,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    sources: &'j CheckedDraftingProgramSources<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    order: &'j CheckedDraftingProgramOrder<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    graph: &'j FormulaAcyclicSourceDependencies<'j>,
    issue: DraftingProgramDependencyOwnerIssue,
}
impl<
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramDependencyOwnerError<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Exact selected source proof owner and all original Root/input/normalization fields.
    #[must_use]
    pub const fn sources(
        &self,
    ) -> &'j CheckedDraftingProgramSources<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.sources
    }
    /// Exact provided order proof,including its original associations and selected policy.
    #[must_use]
    pub const fn order(&self) -> &'j CheckedDraftingProgramOrder<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.order
    }
    /// Exact provided native acyclic proof,including original dependency/source proof identities.
    #[must_use]
    pub const fn graph(&self) -> &'j FormulaAcyclicSourceDependencies<'j> {
        self.graph
    }
    /// First mismatch in order-input then selected native source-graph identity order.
    #[must_use]
    pub const fn issue(&self) -> DraftingProgramDependencyOwnerIssue {
        self.issue
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramDependencyOwnerError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramDependencyOwnerError")
            .field("issue", &self.issue)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Display
    for DraftingProgramDependencyOwnerError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting dependency owner: {:?}", self.issue)
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> std::error::Error
    for DraftingProgramDependencyOwnerError<'_, '_, '_, '_, '_, '_, '_, H, S>
{
}

/// Exact selected source/order/acyclic owners,without cloned graph,metadata or proof buffers.
/// This constant-work join grants no mixed canonical bindings,geometry/semantic dependencies,
/// values,execution order or whole-programme acceptance. Native graph policies remain original.
/// ```compile_fail,E0616
/// use sc_core::{construction::{DraftingProgramDependencyInputs,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     inputs:&mut DraftingProgramDependencyInputs<'_, '_, '_, '_, '_, '_, '_,H,S>) {inputs.graph=todo!();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::{DraftingProgramDependencyInputs,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn escape<'j,'i,'r,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     inputs:DraftingProgramDependencyInputs<'j,'i,'r,'b,'n,'p,'a,H,S>)
///     ->DraftingProgramDependencyInputs<'static,'i,'r,'b,'n,'p,'a,H,S> {inputs}
/// ```
pub struct DraftingProgramDependencyInputs<
    'j,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    sources: &'j CheckedDraftingProgramSources<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    order: &'j CheckedDraftingProgramOrder<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    graph: &'j FormulaAcyclicSourceDependencies<'j>,
}
impl<
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramDependencyInputs<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Original exact checked source/normalization/input owner,with original native allowances.
    #[must_use]
    pub const fn sources(
        &self,
    ) -> &'j CheckedDraftingProgramSources<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.sources
    }
    /// Original exact checked mixed order and its original source associations/policy.
    #[must_use]
    pub const fn order(&self) -> &'j CheckedDraftingProgramOrder<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.order
    }
    /// Original native complete acyclic relation and selected dependency/cycle policies.
    #[must_use]
    pub const fn graph(&self) -> &'j FormulaAcyclicSourceDependencies<'j> {
        self.graph
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramDependencyInputs<'_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramDependencyInputs")
            .finish_non_exhaustive()
    }
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
    /// Join the exact selected original proof owners. Order must use this source owner's actual
    /// BoundInitials object;graph.dependencies().checked() must be this exact native source proof.
    /// Independently recomputed equal-looking proofs cannot substitute. The checks allocate/query
    /// nothing and read no values;order-input mismatch precedes source-graph mismatch.
    ///
    /// ```
    /// use sc_core::{construction::{CheckedDraftingProgramSources,CheckedDraftingProgramOrder,
    ///     DraftingInitialInputProvider},design::DesignMetadataHost,recipe::FormulaAcyclicSourceDependencies};
    /// fn selected_sources<'j,'i,'r,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     sources:&'j CheckedDraftingProgramSources<'j,'i,'r,'b,'n,'p,'a,H,S>,
    ///     order:&'j CheckedDraftingProgramOrder<'j,'i,'r,'b,'n,'p,'a,H,S>,
    ///     graph:&'j FormulaAcyclicSourceDependencies<'j>)->Result<usize,String> {
    ///     let inputs=sources.dependency_inputs(order,graph).map_err(|error|error.to_string())?;
    ///     Ok(inputs.sources().proof().sources().len())
    /// }
    /// ```
    /// # Errors
    /// First foreign order-input/source-graph identity,retaining all actual provided proof objects.
    pub fn dependency_inputs<'j>(
        &'j self,
        order: &'j CheckedDraftingProgramOrder<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        graph: &'j FormulaAcyclicSourceDependencies<'j>,
    ) -> Result<
        DraftingProgramDependencyInputs<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingProgramDependencyOwnerError<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    >
    where
        'q: 'j,
    {
        let sources: &'j CheckedDraftingProgramSources<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> = self;
        let issue = if !core::ptr::eq(sources.inputs(), order.inputs()) {
            Some(DraftingProgramDependencyOwnerIssue::OrderInputs)
        } else if !core::ptr::eq(sources.proof(), graph.dependencies().checked()) {
            Some(DraftingProgramDependencyOwnerIssue::SourceGraph)
        } else {
            None
        };
        if let Some(issue) = issue {
            return Err(DraftingProgramDependencyOwnerError {
                sources,
                order,
                graph,
                issue,
            });
        }
        Ok(DraftingProgramDependencyInputs {
            sources,
            order,
            graph,
        })
    }
}
