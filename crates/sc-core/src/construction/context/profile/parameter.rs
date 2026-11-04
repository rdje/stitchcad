//! Fresh selected-source content and exact original parameter/header/canonical association.
use super::{
    BoundDraftingProfileSource, DraftingProfileLogicalProvider, DraftingProfileLogicalResponse,
    DraftingProfileLogicalSource,
};
use crate::{
    design::{
        fixed_value, DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError,
        DesignResourceLimits, DesignResourceMeter, DesignResourceSink, DesignResourceSource,
    },
    ontology::ProfileParameterRef,
    value::{LogicalInputBinding, LogicalInputDeclaration, LogicalInputKind},
};
use core::fmt;
mod check;
mod error;
pub use error::{
    DraftingProfileLogicalError, DraftingProfileLogicalIssue, DraftingProfileLogicalRefusal,
};

/// Actual original logical parameter metadata and kind-checked canonical member under one exact
/// selected Profile parent and fresh content policy. Domain/state/provenance stay in the original
/// source. This proves no aggregate lookup work,G4 schema/evidence/physical truth,value resolution,
/// semantic dependency graph,currentness revalidation or whole-program/execution acceptance.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>>(
///     source:&mut BoundDraftingProfileLogicalSource<'_,'_,H,S>) {source.binding=todo!();}
/// ```
/// A parameter proof cannot outlive its exact original selected-source parent loan:
/// ```compile_fail
/// use sc_core::{construction::*,design::{DesignResourceSource,DesignResourceLimits},
///     ontology::ProfileParameterRef,value::LogicalInputKind};
/// fn escape<'b,'a,H:DesignResourceSource,S:DraftingProfileLogicalProvider<H>>(
///     parent:&'b BoundDraftingProfileSource<'a,H,S>,parameter:ProfileParameterRef,
///     kind:LogicalInputKind,limits:DesignResourceLimits)
///     ->Result<BoundDraftingProfileLogicalSource<'static,'a,H,S>,String> {
///     parent.bind_logical_parameter(parameter,kind,limits).map_err(|e|e.to_string())
/// }
/// ```
pub struct BoundDraftingProfileLogicalSource<
    'b,
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + ?Sized,
> {
    parent: &'b BoundDraftingProfileSource<'a, H, S>,
    parameter: ProfileParameterRef,
    kind: LogicalInputKind,
    source: DraftingProfileLogicalResponse<'a, H, S>,
    binding: LogicalInputBinding<'a>,
    limits: DesignResourceLimits,
    demand: DesignResourceDemand,
}
impl<'b, 'a, H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized>
    BoundDraftingProfileLogicalSource<'b, 'a, H, S>
{
    /// Exact original selected Profile parent,not an equal-looking independent binding.
    #[must_use]
    pub const fn parent(&self) -> &'b BoundDraftingProfileSource<'a, H, S> {
        self.parent
    }
    /// Original requested parameter identity,distinct from canonical declaration identity.
    #[must_use]
    pub const fn parameter(&self) -> ProfileParameterRef {
        self.parameter
    }
    /// Original requested logical role,checked against the actual canonical declaration.
    #[must_use]
    pub const fn kind(&self) -> LogicalInputKind {
        self.kind
    }
    /// Actual original metadata and matched selected-source header;no copied body/domain/state.
    #[must_use]
    pub const fn source(&self) -> DraftingProfileLogicalResponse<'a, H, S> {
        self.source
    }
    /// Actual original Profile canonical inventory/member/kind/index join.
    #[must_use]
    pub const fn binding(&self) -> LogicalInputBinding<'a> {
        self.binding
    }
    /// Actual kind-checked canonical member,without a selected value or evidence grant.
    #[must_use]
    pub const fn declaration(&self) -> &'a LogicalInputDeclaration {
        self.binding.declaration()
    }
    /// Explicit selected fresh complete content allowance.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.limits
    }
    /// Fresh whole Root/provider/request content demand,not aggregate lookup/comparison work.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProfileLogicalSource<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProfileLogicalSource")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
impl<H: DesignResourceSource, S: DraftingProfileLogicalProvider<H> + ?Sized>
    BoundDraftingProfileLogicalSource<'_, '_, H, S>
{
    /// Freshly append actual complete parent/provider content again;member/metadata are original
    /// borrowed views,not duplicated owned bodies. No parameter/source/value query or clone occurs.
    /// # Errors
    /// Original first cumulative resource cause,including earlier or ignored sink failures.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            parent,
            parameter,
            kind,
            source,
            binding,
            limits,
            demand,
        } = self;
        fixed_value(parameter);
        fixed_value(kind);
        fixed_value(source);
        fixed_value(binding);
        fixed_value(limits);
        fixed_value(demand);
        parent.charge_resources(sink)
    }
}
impl<'a, H: DesignResourceSource, S: DraftingProfileLogicalProvider<H> + ?Sized>
    BoundDraftingProfileSource<'a, H, S>
{
    /// Freshly charge complete Root/provider/request content before one parameter query. Require
    /// the same original returned Root/pin/inventory/Profile/snapshot header,then parameter ID,
    /// canonical membership/kind and exact declaration owner. No source/default substitution,
    /// value read,query-count grant or accepted partial parameter escapes failure.
    /// # Errors
    /// Original first resource/provider/header/parameter/canonical cause and returned witnesses.
    pub fn bind_logical_parameter(
        &self,
        parameter: ProfileParameterRef,
        kind: LogicalInputKind,
        limits: DesignResourceLimits,
    ) -> Result<
        BoundDraftingProfileLogicalSource<'_, 'a, H, S>,
        DraftingProfileLogicalError<'_, 'a, H, S>,
    > {
        let refuse = |returned, refusal| {
            DraftingProfileLogicalError::new(self, parameter, kind, limits, returned, refusal)
        };
        let mut meter = DesignResourceMeter::new(limits);
        let charged = self.charge_resources(&mut meter);
        let demand = meter
            .finish()
            .map_err(|cause| refuse(None, DraftingProfileLogicalRefusal::Resources(cause)))?;
        charged.map_err(|cause| refuse(None, DraftingProfileLogicalRefusal::Resources(cause)))?;
        let source: DraftingProfileLogicalResponse<'a, H, S> = self
            .context()
            .source()
            .profile_logical_source(self.source(), parameter)
            .map_err(|cause| refuse(None, DraftingProfileLogicalRefusal::Source(cause)))?;
        if let Some(issue) = check::issue(self.source(), parameter, source) {
            return Err(refuse(
                Some(source),
                DraftingProfileLogicalRefusal::Mismatch(issue),
            ));
        }
        let binding = self
            .source()
            .inputs
            .logicals()
            .resolve(source.declaration.id(), kind)
            .map_err(|cause| {
                refuse(
                    Some(source),
                    DraftingProfileLogicalRefusal::Canonical(cause),
                )
            })?;
        if !core::ptr::eq(binding.declaration(), source.declaration) {
            return Err(refuse(
                Some(source),
                DraftingProfileLogicalRefusal::Mismatch(DraftingProfileLogicalIssue::Declaration),
            ));
        }
        Ok(BoundDraftingProfileLogicalSource {
            parent: self,
            parameter,
            kind,
            source,
            binding,
            limits,
            demand,
        })
    }
}
