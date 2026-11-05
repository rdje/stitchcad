//! Exact semantic source/context owners before fresh aggregate work or any semantic query.
use super::BoundDraftingProgramGeometry;
use crate::{
    construction::{
        AdmittedDraftingGarmentSource, AdmittedDraftingSemanticSelections, DraftingGarmentProvider,
        DraftingInitialInputProvider,
    },
    design::DesignReferenceHost,
};
use core::fmt;

/// First actual held context mismatch, in Root/provider/frame/snapshot order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramSemanticOwnerIssue {
    /// Garment and programme borrow different actual input Roots, even with equal IDs/revisions.
    Root,
    /// The garment scope borrows another actual immutable provider.
    Provider,
    /// Actual held Cartesian frame records differ, even when their data looks equal.
    Frame,
    /// Actual held immutable snapshot records differ, even when their data looks equal.
    Snapshot,
}

/// Exact actual garment/context, complete geometry proof chain and local selection owners.
/// This constant-work join grants no aggregate allowance, actual selection coverage, semantic
/// source binding, value, physical execution or whole-programme acceptance.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn alter<H:DesignReferenceHost,S:DraftingInitialInputProvider<H>+DraftingGarmentProvider<H>>(
///     inputs:&mut DraftingProgramSemanticInputs<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_,H,S>) {
///     inputs.garment=todo!();
/// }
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn escape<'s,'g,'k,'j,'i,'r,'b,'n,'p,'a,H:DesignReferenceHost,S:DraftingInitialInputProvider<H>+DraftingGarmentProvider<H>>(
///     inputs:DraftingProgramSemanticInputs<'s,'g,'k,'j,'i,'r,'b,'n,'p,'a,H,S>)
///     ->DraftingProgramSemanticInputs<'static,'g,'k,'j,'i,'r,'b,'n,'p,'a,H,S> {inputs}
/// ```
pub struct DraftingProgramSemanticInputs<
    's,
    'g,
    'k,
    'j,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignReferenceHost,
    S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
> {
    geometry: &'s BoundDraftingProgramGeometry<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    garment: &'s AdmittedDraftingGarmentSource<'a, H, S>,
    selections: &'s AdmittedDraftingSemanticSelections<'s, 's>,
}
impl<
        's,
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignReferenceHost,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Original exact complete geometry owner, retaining canonical/source/order/acyclic proofs.
    #[must_use]
    pub const fn geometry(
        &self,
    ) -> &'s BoundDraftingProgramGeometry<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.geometry
    }
    /// Original exact admitted garment and its held Root/provider/frame/snapshot context.
    #[must_use]
    pub const fn garment(&self) -> &'s AdmittedDraftingGarmentSource<'a, H, S> {
        self.garment
    }
    /// Original locally admitted request owner and every raw row/pin, without a copied table.
    #[must_use]
    pub const fn selections(&self) -> &'s AdmittedDraftingSemanticSelections<'s, 's> {
        self.selections
    }
}
impl<
        H: DesignReferenceHost,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > fmt::Debug for DraftingProgramSemanticInputs<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramSemanticInputs")
            .finish_non_exhaustive()
    }
}

/// All supplied original owners and the first mismatch; no successful semantic prefix.
pub struct DraftingProgramSemanticOwnerError<
    's,
    'g,
    'k,
    'j,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignReferenceHost,
    S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
> {
    geometry: &'s BoundDraftingProgramGeometry<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    garment: &'s AdmittedDraftingGarmentSource<'a, H, S>,
    selections: &'s AdmittedDraftingSemanticSelections<'s, 's>,
    issue: DraftingProgramSemanticOwnerIssue,
}
impl<
        's,
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignReferenceHost,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > DraftingProgramSemanticOwnerError<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Original exact complete geometry owner, retaining canonical/source/order/acyclic proofs.
    #[must_use]
    pub const fn geometry(
        &self,
    ) -> &'s BoundDraftingProgramGeometry<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.geometry
    }
    /// Original exact admitted garment and its held Root/provider/frame/snapshot context.
    #[must_use]
    pub const fn garment(&self) -> &'s AdmittedDraftingGarmentSource<'a, H, S> {
        self.garment
    }
    /// Original locally admitted request owner and every raw row/pin, without a copied table.
    #[must_use]
    pub const fn selections(&self) -> &'s AdmittedDraftingSemanticSelections<'s, 's> {
        self.selections
    }
    /// First actual Root/provider/frame/snapshot mismatch.
    #[must_use]
    pub const fn issue(&self) -> DraftingProgramSemanticOwnerIssue {
        self.issue
    }
}
impl<
        H: DesignReferenceHost,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > fmt::Debug
    for DraftingProgramSemanticOwnerError<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramSemanticOwnerError")
            .field("issue", &self.issue)
            .finish_non_exhaustive()
    }
}
impl<
        H: DesignReferenceHost,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > fmt::Display
    for DraftingProgramSemanticOwnerError<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting semantic owner: {:?}", self.issue)
    }
}
impl<
        H: DesignReferenceHost,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > std::error::Error
    for DraftingProgramSemanticOwnerError<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
}
impl<
        's,
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignReferenceHost,
        S: DraftingInitialInputProvider<H> + DraftingGarmentProvider<H> + ?Sized,
    > BoundDraftingProgramGeometry<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Join exact original owners without callbacks, allocation, content walks or member lookups.
    /// Root, provider, held frame and held snapshot identities are checked in that order. The
    /// geometry proof already retains its exact canonical/source/order/acyclic proof chain.
    /// Local selection admission supplies structural keys only; invalid coordinates remain data.
    /// Fresh whole-content/work admission and actual source/selection coverage remain separate.
    /// # Errors
    /// First context identity mismatch, retaining every original supplied owner.
    pub fn semantic_inputs(
        &'s self,
        garment: &'s AdmittedDraftingGarmentSource<'a, H, S>,
        selections: &'s AdmittedDraftingSemanticSelections<'s, 's>,
    ) -> Result<
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingProgramSemanticOwnerError<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    > {
        let context = self
            .work()
            .inputs()
            .inputs()
            .order()
            .inputs()
            .roles()
            .positions()
            .work()
            .inputs()
            .program()
            .admitted()
            .inputs()
            .context();
        let supplied = garment.context();
        let issue = if !core::ptr::eq(context.scope().root(), supplied.scope().root()) {
            Some(DraftingProgramSemanticOwnerIssue::Root)
        } else if !core::ptr::addr_eq(context.source(), supplied.source()) {
            Some(DraftingProgramSemanticOwnerIssue::Provider)
        } else if !context.frame().is_same_record(supplied.frame()) {
            Some(DraftingProgramSemanticOwnerIssue::Frame)
        } else if !context.snapshot().is_same_record(supplied.snapshot()) {
            Some(DraftingProgramSemanticOwnerIssue::Snapshot)
        } else {
            None
        };
        if let Some(issue) = issue {
            return Err(DraftingProgramSemanticOwnerError {
                geometry: self,
                garment,
                selections,
                issue,
            });
        }
        Ok(DraftingProgramSemanticInputs {
            geometry: self,
            garment,
            selections,
        })
    }
}

mod resources;
