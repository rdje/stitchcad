//! Fresh Root/provider/request content,then original selected Profile association without values.
use super::{DraftingProfileProvider, DraftingProfileSource};
use crate::{
    construction::DraftingContext,
    design::{
        charge_selected_profile_pin_resources, fixed_value, DesignMetadataHost,
        DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits, DesignResourceMeter,
        DesignResourceScope, DesignResourceSink, DesignResourceSource,
    },
    ontology::SelectedProfilePin,
};
use core::fmt;
mod check;
mod error;
pub use error::{
    DraftingProfileSourceError, DraftingProfileSourceIssue, DraftingProfileSourceRefusal,
};

/// Original selected Profile under exact containing Root/requested labels/snapshot and fresh
/// complete Root/provider/request content. Profile/body/canonical association relies on the
/// original honest immutable provider contract;G4 registry/schema/evidence/value validation is
/// separate. This is one source binding,not aggregate query work,parameter/member,dependency
/// graph,garment content,physical resolution or whole-program acceptance.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingProfileProvider<H>>(bound:&mut BoundDraftingProfileSource<'_,H,S>) {
///     bound.source=todo!();
/// }
/// ```
/// A binding cannot survive its explicitly requested pin:
/// ```compile_fail,E0515
/// use sc_core::{construction::*,design::{DesignResourceSource,DesignResourceLimits},
///     ontology::{SelectedProfilePin,SelectedProfilePinDefinition}};
/// fn escape<'a,H:DesignResourceSource,S:DraftingProfileProvider<H>>(
///     context:DraftingContext<'a,H,S>,limits:DesignResourceLimits)
///     ->Result<BoundDraftingProfileSource<'a,H,S>,String> {
///     let requested=SelectedProfilePin::new(SelectedProfilePinDefinition {
///         profile_id:"atelier".into(),schema_version:"2".into(),content_version:"7".into(),
///     }).unwrap();
///     context.bind_profile_source(&requested,limits).map_err(|e|e.to_string())
/// }
/// ```
pub struct BoundDraftingProfileSource<
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileProvider<H> + ?Sized,
> {
    context: DraftingContext<'a, H, S>,
    selected: &'a SelectedProfilePin,
    source: DraftingProfileSource<'a, H, S::Snapshot, S::Profile>,
    limits: DesignResourceLimits,
    demand: DesignResourceDemand,
}
impl<'a, H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized>
    BoundDraftingProfileSource<'a, H, S>
{
    /// Exact original containing context,Root,provider,frame and snapshot.
    #[must_use]
    pub const fn context(&self) -> DraftingContext<'a, H, S> {
        self.context
    }
    /// Original explicit requested profile/schema/content selection,without normalization.
    #[must_use]
    pub const fn selected(&self) -> &'a SelectedProfilePin {
        self.selected
    }
    /// Actual provider-owned source with matched Root,exact labels and original snapshot.
    /// This raw view alone cannot construct another private binding or certify parameter data.
    #[must_use]
    pub const fn source(&self) -> DraftingProfileSource<'a, H, S::Snapshot, S::Profile> {
        self.source
    }
    /// Explicit selected fresh complete Root/provider/request content allowance.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.limits
    }
    /// Fresh complete demand,including the requested pin and every provider-held source.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProfileSource<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProfileSource")
            .field("demand", &self.demand)
            .finish_non_exhaustive()
    }
}
fn charge<H: DesignResourceSource, S: DraftingProfileProvider<H> + ?Sized>(
    context: DraftingContext<'_, H, S>,
    selected: &SelectedProfilePin,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    super::super::design_semantic::charge_context(context, sink)?;
    charge_selected_profile_pin_resources(
        selected,
        sink,
        DesignResourceScope::CommandState,
        Some(context.scope().root().id()),
        None,
    )
}
impl<H: DesignResourceSource, S: DraftingProfileProvider<H> + ?Sized>
    BoundDraftingProfileSource<'_, H, S>
{
    /// Freshly append every actual Root/provider/request field again;cached demand supplies no
    /// content credit. The returned source loans those original provider-owned records.
    /// No Profile lookup,currentness/member/value query or body cloning occurs.
    /// # Errors
    /// Original first cumulative resource cause,including an earlier or ignored sink refusal.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            context,
            selected,
            source,
            limits,
            demand,
        } = self;
        fixed_value(source);
        fixed_value(limits);
        fixed_value(demand);
        charge(*context, selected, sink)
    }
}
impl<'a, H: DesignResourceSource, S: DraftingProfileProvider<H> + ?Sized>
    DraftingContext<'a, H, S>
{
    /// Charge complete Root,all held provider data,then requested pin strings before one readonly
    /// Profile lookup. Preserve original provider cause,or compare Root,profile/schema/content
    /// labels and snapshot in that order. No default/body substitution,revalidation,value/member
    /// query or accepted partial source escapes failure. The pin borrow may be shorter than context.
    /// # Errors
    /// First original content/provider/Root/pin/snapshot refusal and complete original witnesses.
    pub fn bind_profile_source<'p>(
        self,
        selected: &'p SelectedProfilePin,
        limits: DesignResourceLimits,
    ) -> Result<BoundDraftingProfileSource<'p, H, S>, DraftingProfileSourceError<'p, H, S>>
    where
        'a: 'p,
    {
        let context: DraftingContext<'p, H, S> = self;
        let refuse = |returned, refusal| {
            DraftingProfileSourceError::new(context, selected, limits, returned, refusal)
        };
        let mut meter = DesignResourceMeter::new(limits);
        let charged = charge(context, selected, &mut meter);
        let demand = meter
            .finish()
            .map_err(|cause| refuse(None, DraftingProfileSourceRefusal::Resources(cause)))?;
        charged.map_err(|cause| refuse(None, DraftingProfileSourceRefusal::Resources(cause)))?;
        let source: DraftingProfileSource<'p, H, S::Snapshot, S::Profile> = context
            .source()
            .profile_source(selected)
            .map_err(|cause| refuse(None, DraftingProfileSourceRefusal::Source(cause)))?;
        if let Some(issue) = check::issue(context, selected, source) {
            return Err(refuse(
                Some(source),
                DraftingProfileSourceRefusal::Mismatch(issue),
            ));
        }
        Ok(BoundDraftingProfileSource {
            context,
            selected,
            source,
            limits,
            demand,
        })
    }
}
