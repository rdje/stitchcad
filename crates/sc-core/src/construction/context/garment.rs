//! Actual garment source identity and complete fresh content, without semantic query acceptance.
use super::{DraftingContext, DraftingContextSource, DraftingSourceRecord};
use crate::design::{
    charge_design_garment_resources, fixed_value, DesignGarment, DesignReferenceHost,
    DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits, DesignResourceLocation,
    DesignResourceMeter, DesignResourceScope, DesignResourceSink, DesignResourceSource,
};
use core::fmt;
mod occurrences;
pub use occurrences::{
    DraftingClosureSizeRole, DraftingGarmentSemanticField, DraftingGarmentSemanticOwner,
    DraftingGarmentSemanticUse,
};

/// Raw original garment and its containing snapshot. Public construction grants no Root binding,
/// resource allowance, currentness, declaration/domain or operation-execution certificate.
/// ```compile_fail
/// use sc_core::{construction::DraftingGarmentSource,design::DesignReferenceHost};
/// fn escape<'a,H:DesignReferenceHost>(source:DraftingGarmentSource<'a,H,()>)
///     ->DraftingGarmentSource<'static,H,()> {source}
/// ```
pub struct DraftingGarmentSource<'a, H: DesignReferenceHost, S: ?Sized> {
    /// Actual owner of sewing, Pocket, notion, Closure, layer, Hem and construction inventories.
    pub garment: &'a DesignGarment<H>,
    /// Original immutable snapshot containing these semantic source inventories.
    pub snapshot: &'a DraftingSourceRecord<S>,
}
impl<H: DesignReferenceHost, S: ?Sized> Copy for DraftingGarmentSource<'_, H, S> {}
impl<H: DesignReferenceHost, S: ?Sized> Clone for DraftingGarmentSource<'_, H, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignReferenceHost, S: ?Sized> fmt::Debug for DraftingGarmentSource<'_, H, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DraftingGarmentSource(..)")
    }
}
/// Readonly facet borrowing an actual already-held garment under the original context.
///
/// This getter performs no validation, allocation, cloning, declaration-vector construction,
/// source lookup or value read. It returns original immutable owners/records only. A source without
/// an actual garment must not implement this facet by manufacturing an empty substitute. The
/// complete garment is separately charged by core; all additional provider-held logical/Profile/
/// operation records, duplicated inventories and caches belong in inherited charge_resources.
/// Admission does not call currentness or logical declaration queries. Those require later explicit
/// complete semantic work policies. Like the context port, this honest/exhaustive integration
/// obligation is not a security boundary against invented or omitted source content.
pub trait DraftingGarmentProvider<H: DesignReferenceHost>: DraftingContextSource<H> {
    /// Borrow the original actual semantic owner and snapshot, without a query or acceptance claim.
    fn garment_source(&self) -> DraftingGarmentSource<'_, H, Self::Snapshot>;
}
/// First actual owner, complete content or original snapshot refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingGarmentSourceIssue {
    /// Actual garment inputs differ from the context's original Root, even if IDs/revisions agree.
    Root,
    /// Original complete actual garment/provider content refusal.
    Resources(DesignResourceLimitError),
    /// Raw garment source uses another actual snapshot record.
    Snapshot,
}
/// Boxed original context/raw garment/policy/first-refusal witnesses keep the Result compact.
/// Only a refused semantic diagnostic allocates; no admitted prefix or result vector is created.
pub struct DraftingGarmentSourceError<
    'a,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    details: Box<GarmentSourceFailure<'a, H, S>>,
}
struct GarmentSourceFailure<'a, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> {
    context: DraftingContext<'a, H, S>,
    source: DraftingGarmentSource<'a, H, S::Snapshot>,
    limits: DesignResourceLimits,
    issue: DraftingGarmentSourceIssue,
}
impl<'a, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized>
    DraftingGarmentSourceError<'a, H, S>
{
    /// Actual context and original Root/frame/snapshot/provider.
    #[must_use]
    pub const fn context(&self) -> DraftingContext<'a, H, S> {
        self.details.context
    }
    /// Complete original raw response, including any foreign garment or snapshot.
    #[must_use]
    pub const fn returned(&self) -> DraftingGarmentSource<'a, H, S::Snapshot> {
        self.details.source
    }
    /// Explicit selected fresh complete content allowance.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.details.limits
    }
    /// First Root, content or snapshot cause, in that order.
    #[must_use]
    pub const fn issue(&self) -> DraftingGarmentSourceIssue {
        self.details.issue
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Debug
    for DraftingGarmentSourceError<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingGarmentSourceError")
            .field("issue", &self.details.issue)
            .finish_non_exhaustive()
    }
}
fn charge<H: DesignReferenceHost + DesignResourceSource, S: DraftingGarmentProvider<H> + ?Sized>(
    context: DraftingContext<'_, H, S>,
    source: DraftingGarmentSource<'_, H, S::Snapshot>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let DraftingGarmentSource { garment, snapshot } = source;
    fixed_value(&snapshot);
    charge_design_garment_resources(garment, sink)?;
    let result = context.source().charge_resources(sink);
    sink.charge_entries(
        0,
        DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(garment.inputs().id()),
            field: "drafting.garment.source",
            index: None,
        },
    )?;
    result
}
impl<H: DesignReferenceHost + DesignResourceSource, S: DraftingGarmentProvider<H> + ?Sized>
    AdmittedDraftingGarmentSource<'_, H, S>
{
    /// Freshly append every actual garment/provider field in one cumulative sink. Original source
    /// and context are borrowed views;stored policy/demand gives no credit. Preserve first ignored
    /// or earlier sink failure before a later returned cause. No garment/source/currentness/member
    /// or value query,clone or result collection occurs. Caller owns/finishes the meter.
    /// # Errors
    /// Original first cumulative logical-content cause with actual owning fields.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            context,
            source,
            limits,
            demand,
        } = self;
        fixed_value(source);
        fixed_value(limits);
        fixed_value(demand);
        charge(*context, *source, sink)
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Display
    for DraftingGarmentSourceError<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting garment source: {:?}", self.details.issue)
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> std::error::Error
    for DraftingGarmentSourceError<'_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.issue {
            DraftingGarmentSourceIssue::Resources(cause) => Some(cause),
            _ => None,
        }
    }
}
/// Original Root-bound garment and snapshot under a fresh complete logical content allowance.
/// This private admission grants no current semantic targets, declaration vectors, logical domains,
/// operation existence, values, complete semantic dependency acceptance or whole-programme proof.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn alter<H:DesignReferenceHost,S:DraftingGarmentProvider<H>>(
///     proof:&mut AdmittedDraftingGarmentSource<'_,H,S>) {proof.source=todo!();}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignReferenceHost};
/// fn escape<'a,H:DesignReferenceHost,S:DraftingGarmentProvider<H>>(
///     proof:AdmittedDraftingGarmentSource<'a,H,S>)->AdmittedDraftingGarmentSource<'static,H,S> {proof}
/// ```
pub struct AdmittedDraftingGarmentSource<
    'a,
    H: DesignReferenceHost,
    S: DraftingGarmentProvider<H> + ?Sized,
> {
    context: DraftingContext<'a, H, S>,
    source: DraftingGarmentSource<'a, H, S::Snapshot>,
    limits: DesignResourceLimits,
    demand: DesignResourceDemand,
}
impl<'a, H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized>
    AdmittedDraftingGarmentSource<'a, H, S>
{
    /// Original actual containing context, not a rebuilt context or Root.
    #[must_use]
    pub const fn context(&self) -> DraftingContext<'a, H, S> {
        self.context
    }
    /// Actual original complete garment and matched snapshot record.
    #[must_use]
    pub const fn source(&self) -> DraftingGarmentSource<'a, H, S::Snapshot> {
        self.source
    }
    /// Explicit selected complete fresh content allowance.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.limits
    }
    /// Complete freshly walked garment plus provider demand; prior admissions supply no credit.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
}
impl<H: DesignReferenceHost, S: DraftingGarmentProvider<H> + ?Sized> fmt::Debug
    for AdmittedDraftingGarmentSource<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingGarmentSource")
            .field("demand", &self.demand)
            .finish_non_exhaustive()
    }
}
impl<'a, H: DesignReferenceHost + DesignResourceSource, S: DraftingGarmentProvider<H> + ?Sized>
    DraftingContext<'a, H, S>
{
    /// Bind the raw actual garment Root before freshly charging complete garment/provider content
    /// in one meter, then require the original snapshot record. The final meter preserves its first
    /// refusal even when a provider ignores a sink error. No currentness, declarations, values,
    /// geometry or semantic query work is performed. Earlier context demands supply no credit.
    ///
    /// ```
    /// use sc_core::{construction::*,design::{DesignReferenceHost,DesignResourceSource,DesignResourceLimits}};
    /// fn garment<'a,H:DesignReferenceHost+DesignResourceSource,S:DraftingGarmentProvider<H>>(
    ///     context:DraftingContext<'a,H,S>,limits:DesignResourceLimits)
    ///     ->Result<AdmittedDraftingGarmentSource<'a,H,S>,DraftingGarmentSourceError<'a,H,S>> {
    ///     context.admit_garment_source(limits)
    /// }
    /// ```
    /// # Errors
    /// First actual Root mismatch, original full content refusal, then actual snapshot mismatch.
    /// Every refusal retains the actual context/raw source and explicit selected content policy.
    pub fn admit_garment_source(
        self,
        limits: DesignResourceLimits,
    ) -> Result<AdmittedDraftingGarmentSource<'a, H, S>, DraftingGarmentSourceError<'a, H, S>> {
        let source = self.source().garment_source();
        let refuse = |issue| DraftingGarmentSourceError {
            details: Box::new(GarmentSourceFailure {
                context: self,
                source,
                limits,
                issue,
            }),
        };
        if !core::ptr::eq(source.garment.inputs(), self.scope().root()) {
            return Err(refuse(DraftingGarmentSourceIssue::Root));
        }
        let mut meter = DesignResourceMeter::new(limits);
        let charged = charge(self, source, &mut meter);
        let demand = meter
            .finish()
            .map_err(|cause| refuse(DraftingGarmentSourceIssue::Resources(cause)))?;
        charged.map_err(|cause| refuse(DraftingGarmentSourceIssue::Resources(cause)))?;
        if !self.snapshot().is_same_record(source.snapshot) {
            return Err(refuse(DraftingGarmentSourceIssue::Snapshot));
        }
        Ok(AdmittedDraftingGarmentSource {
            context: self,
            source,
            limits,
            demand,
        })
    }
}

mod buttonhole;
pub use buttonhole::{
    BoundDraftingClosureButtonhole, DraftingClosureButtonholeError,
    DraftingClosureButtonholeLimits, DraftingClosureButtonholeRefusal,
    DraftingClosureButtonholeShape,
};

mod placement;
pub use placement::{
    BoundDraftingClosurePlacementMember, DraftingClosurePlacementLimits,
    DraftingClosurePlacementMemberError, DraftingClosurePlacementMemberRefusal,
    DraftingClosurePlacementOwner, DraftingClosurePlacementRequest, DraftingClosurePlacementShape,
};
