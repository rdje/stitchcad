//! Exact original Root source scope before aggregate operation-input membership or Derived order.
use super::{
    CheckedDraftingProgramDirectSelections, DraftingProgramSemanticInputs,
    DraftingProgramSemanticOwnerIssue,
};
use crate::{
    construction::{
        DraftingContextSource, DraftingDesignSemanticScope, DraftingGarmentProvider,
        DraftingInitialInputProvider,
    },
    design::{DesignMetadataHost, DesignReferenceHost},
};
use core::fmt;
mod members;
pub use members::{
    BoundDraftingProgramRootInputMembers, DraftingProgramRootInputMember,
    DraftingProgramRootInputMemberError, DraftingProgramRootInputMemberRefusal,
    DraftingProgramRootInputMembersResult, DraftingProgramRootInputSource,
};

/// Exact owner-join result,retaining original consumer and supplied source-scope witnesses on
/// either branch. The alias supplies no constructor,copy,source or fresh-work acceptance.
/// ```
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn original<'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>+?Sized>(
///     result:DraftingProgramRootInputScopeResult<'v,'w,'a,I,H,S>)->Result<(),String> {
///     let owner=result.map_err(|e|e.to_string())?;
///     assert!(std::ptr::eq(owner.inputs(),owner.selections().inputs()));
///     Ok(())
/// }
/// ```
pub type DraftingProgramRootInputScopeResult<'v, 'w, 'a, I, H, S> = Result<
    DraftingProgramRootInputScope<'v, 'w, 'a, I, H, S>,
    DraftingProgramRootInputScopeError<'v, 'w, 'a, I, H, S>,
>;

/// Exact original consumer/direct-selection and fresh Design source-scope owners. The successful
/// join consumes and retains the unique original child cursor without resetting or reserving it.
/// This identity association supplies no fresh content/work credit,Root operation-input member,
/// Derived supplier/order,Profile source,value or whole semantic cohort acceptance.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:&mut DraftingProgramRootInputScope<'_,'_,'_,I,H,S>) {p.scope=todo!();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'v,'w,'a,I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     p:DraftingProgramRootInputScope<'v,'w,'a,I,H,S>)->DraftingProgramRootInputScope<'static,'w,'a,I,H,S> {p}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn duplicate<I:?Sized,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:DraftingProgramRootInputScope<'_,'_,'_,I,H,S>) {let _=p.clone();}
/// ```
pub struct DraftingProgramRootInputScope<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    selections: CheckedDraftingProgramDirectSelections<'w, I>,
    scope: &'v DraftingDesignSemanticScope<'a, H, S>,
}
impl<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramRootInputScope<'v, 'w, 'a, I, H, S>
{
    /// Exact original joined semantic owner,including actual program/garment and raw requests.
    #[must_use]
    pub const fn inputs(&self) -> &'w I {
        self.selections.inputs()
    }
    /// Original checked keys,complete fresh work and the same unique child-pass cursor.
    #[must_use]
    pub const fn selections(&self) -> &CheckedDraftingProgramDirectSelections<'w, I> {
        &self.selections
    }
    /// Exact original supplied Design source scope,with its actual context and original policy.
    /// Its earlier demand supplies no fresh content or later lookup-work allowance.
    #[must_use]
    pub const fn scope(&self) -> &'v DraftingDesignSemanticScope<'a, H, S> {
        self.scope
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramRootInputScope<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRootInputScope")
            .finish_non_exhaustive()
    }
}
/// Every original supplied owner and first Root/provider/frame/snapshot mismatch. The consumed
/// direct-key proof and its original unmodified policy/budget remain inspectable without copying
/// source bodies or accepting a Root member/prefix. Only a refused diagnostic is boxed.
pub struct DraftingProgramRootInputScopeError<
    'v,
    'w,
    'a,
    I: ?Sized,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    details: Box<Failure<'v, 'w, 'a, I, H, S>>,
}
struct Failure<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> {
    selections: CheckedDraftingProgramDirectSelections<'w, I>,
    scope: &'v DraftingDesignSemanticScope<'a, H, S>,
    issue: DraftingProgramSemanticOwnerIssue,
}
impl<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramRootInputScopeError<'v, 'w, 'a, I, H, S>
{
    /// Exact original semantic source/programme/garment/request owner.
    #[must_use]
    pub const fn inputs(&self) -> &'w I {
        self.details.selections.inputs()
    }
    /// Exact original checked selection/work owner and unchanged child cursor.
    #[must_use]
    pub const fn selections(&self) -> &CheckedDraftingProgramDirectSelections<'w, I> {
        &self.details.selections
    }
    /// Exact original supplied source scope,including every mismatched actual context witness.
    #[must_use]
    pub const fn scope(&self) -> &'v DraftingDesignSemanticScope<'a, H, S> {
        self.details.scope
    }
    /// First original held context mismatch;no provider callback reselects a record.
    #[must_use]
    pub const fn issue(&self) -> DraftingProgramSemanticOwnerIssue {
        self.details.issue
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramRootInputScopeError<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRootInputScopeError")
            .field("issue", &self.details.issue)
            .finish_non_exhaustive()
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingProgramRootInputScopeError<'_, '_, '_, I, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting Root input scope: {:?}", self.details.issue)
    }
}
impl<I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingProgramRootInputScopeError<'_, '_, '_, I, H, S>
{
}

impl<
        'w,
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
    >
    CheckedDraftingProgramDirectSelections<
        'w,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    >
{
    /// Consume the original checked keys/work and join the exact actual Root source scope. Check
    /// Root identity,provider storage address,held frame and held snapshot in that order,without
    /// callbacks,content walks,member lookups,allocations on success or child-budget mutation.
    /// Prior scope policy/demand supplies no fresh credit;later member stages must reserve their
    /// complete original costs and freshly charge source content before queries/results.
    /// # Errors
    /// Every original supplied owner/selected policy and unchanged cursor,with the first mismatch.
    pub fn root_input_scope<'v>(
        self,
        scope: &'v DraftingDesignSemanticScope<'a, H, S>,
    ) -> DraftingProgramRootInputScopeResult<
        'v,
        'w,
        'a,
        DraftingProgramSemanticInputs<'s, 'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        H,
        S,
    > {
        let actual = self.inputs().garment().context();
        let supplied = scope.context();
        let issue = if !core::ptr::eq(actual.scope().root(), supplied.scope().root()) {
            Some(DraftingProgramSemanticOwnerIssue::Root)
        } else if !core::ptr::addr_eq(actual.source(), supplied.source()) {
            Some(DraftingProgramSemanticOwnerIssue::Provider)
        } else if !actual.frame().is_same_record(supplied.frame()) {
            Some(DraftingProgramSemanticOwnerIssue::Frame)
        } else if !actual.snapshot().is_same_record(supplied.snapshot()) {
            Some(DraftingProgramSemanticOwnerIssue::Snapshot)
        } else {
            None
        };
        if let Some(issue) = issue {
            return Err(DraftingProgramRootInputScopeError {
                details: Box::new(Failure {
                    selections: self,
                    scope,
                    issue,
                }),
            });
        }
        Ok(DraftingProgramRootInputScope {
            selections: self,
            scope,
        })
    }
}
