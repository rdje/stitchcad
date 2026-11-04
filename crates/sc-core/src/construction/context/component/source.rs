//! Complete original component metadata association, without creation/history or geometry proof.
use super::{
    AdmittedDraftingComponentWork, DraftingComponentBindingShape, DraftingComponentProvider,
    DraftingComponentResponse,
};
use crate::design::{
    fixed_value, DesignReferenceHost, DesignResourceDemand, DesignResourceLimitError,
    DesignResourceSink, DesignResourceSource,
};
use core::fmt;
mod check;
mod error;
pub use error::{
    DraftingComponentSourceError, DraftingComponentSourceIssue, DraftingComponentSourceRefusal,
};
type Admission = (DesignResourceDemand, DraftingComponentBindingShape);

/// Complete original import or recipe/creator association with the current copy/Piece and actual
/// frame/snapshot records. A recipe retains its captured context and genuine unique mixed position.
/// This proves no executed creation, history, source truth, geometry or whole-program acceptance.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn alter<H:DesignReferenceHost,S:DraftingComponentProvider<H>>(source:&mut BoundDraftingComponentSource<'_,'_,'_,H,S>) {
///     source.source=todo!();
/// }
/// ```
/// The complete binding cannot outlive its original work parent:
/// ```compile_fail
/// use sc_core::{construction::*,design::{DesignReferenceHost,DesignResourceSource}};
/// fn escape<'w,'g,'a,H:DesignReferenceHost+DesignResourceSource,S:DraftingComponentProvider<H>>(
///     work:&'w AdmittedDraftingComponentWork<'g,'a,H,S>)
///     ->Result<BoundDraftingComponentSource<'static,'g,'a,H,S>,String> {
///     work.bind_source().map_err(|e|e.to_string())
/// }
/// ```
pub struct BoundDraftingComponentSource<
    'w,
    'g,
    'a,
    H: DesignReferenceHost,
    S: DraftingComponentProvider<H> + ?Sized,
> {
    work: &'w AdmittedDraftingComponentWork<'g, 'a, H, S>,
    source: DraftingComponentResponse<'a, H, S>,
    creator_position: Option<usize>,
    demand: DesignResourceDemand,
    shape: DraftingComponentBindingShape,
}
impl<'w, 'g, 'a, H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized>
    BoundDraftingComponentSource<'w, 'g, 'a, H, S>
{
    /// Exact original current copy/Piece work parent and explicit policy.
    #[must_use]
    pub const fn work(&self) -> &'w AdmittedDraftingComponentWork<'g, 'a, H, S> {
        self.work
    }
    /// Complete original associated response. Captured recipe context/snapshot is not rewritten.
    #[must_use]
    pub const fn source(&self) -> DraftingComponentResponse<'a, H, S> {
        self.source
    }
    /// Genuine one-based position in the captured recipe's original mixed sequence;None for import.
    #[must_use]
    pub const fn creator_position(&self) -> Option<usize> {
        self.creator_position
    }
    /// Fresh complete content demand from this binding's repeated work admission.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Fresh conservative prospective matching shape,without cached parent credit.
    #[must_use]
    pub const fn shape(&self) -> DraftingComponentBindingShape {
        self.shape
    }
}
impl<H: DesignReferenceHost, S: DraftingComponentProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingComponentSource<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingComponentSource")
            .field("creator_position", &self.creator_position)
            .finish_non_exhaustive()
    }
}
impl<H: DesignReferenceHost + DesignResourceSource, S: DraftingComponentProvider<H> + ?Sized>
    BoundDraftingComponentSource<'_, '_, '_, H, S>
{
    /// Rewalk all original parent garment/provider fields. Response and metadata are borrowed/fixed
    /// views,not additional body copies;stored demand grants no credit. No query or association scan.
    /// # Errors
    /// First original cumulative content cause,including earlier or ignored provider failures.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            work,
            source,
            creator_position,
            demand,
            shape,
        } = self;
        fixed_value(source);
        fixed_value(creator_position);
        fixed_value(demand);
        fixed_value(shape);
        work.charge_resources(sink)
    }
}
impl<
        'g,
        'a,
        H: DesignReferenceHost + DesignResourceSource,
        S: DraftingComponentProvider<H> + ?Sized,
    > AdmittedDraftingComponentWork<'g, 'a, H, S>
{
    /// Repeat fresh complete content/work/current-member admission before one original query.
    /// Match actual Piece,frame,snapshot,then complete original recipe owner/creator association.
    /// Preserve the captured context and genuine unique mixed position;never infer history from IDs.
    /// No currentness/value query,body clone,result vector,transform or successful prefix is created.
    /// # Errors
    /// Original fresh work/provider cause or first actual complete association mismatch.
    pub fn bind_source(
        &self,
    ) -> Result<
        BoundDraftingComponentSource<'_, 'g, 'a, H, S>,
        DraftingComponentSourceError<'_, 'g, 'a, H, S>,
    > {
        let fresh = self
            .parent()
            .prepare_component_binding(self.target(), self.limits())
            .map_err(|cause| {
                DraftingComponentSourceError::new(
                    self,
                    None,
                    None,
                    DraftingComponentSourceRefusal::Work(cause),
                )
            })?;
        let admission = Some((fresh.demand(), fresh.shape()));
        let source = fresh
            .parent()
            .context()
            .source()
            .component_source(fresh.target())
            .map_err(|cause| {
                DraftingComponentSourceError::new(
                    self,
                    admission,
                    None,
                    DraftingComponentSourceRefusal::Source(cause),
                )
            })?;
        let creator_position = check::association(&fresh, source).map_err(|issue| {
            DraftingComponentSourceError::new(
                self,
                admission,
                Some(source),
                DraftingComponentSourceRefusal::Mismatch(issue),
            )
        })?;
        Ok(BoundDraftingComponentSource {
            work: self,
            source,
            creator_position,
            demand: fresh.demand(),
            shape: fresh.shape(),
        })
    }
}
