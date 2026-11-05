//! Fresh selected-source content and exact original numeric metadata/header/canonical association.
use super::{
    BoundDraftingProfileSource, DraftingProfileNumericProvider, DraftingProfileNumericResponse,
    DraftingProfileNumericSource,
};
use crate::{
    design::{
        fixed_value, DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError,
        DesignResourceLimits, DesignResourceMeter, DesignResourceSink, DesignResourceSource,
    },
    ontology::ProfileParameterRef,
    value::{CanonicalNumericInputBinding, CanonicalNumericInputMember, ScalarInputKind},
};
use core::fmt;
mod check;
mod error;
pub use error::{
    DraftingProfileNumericError, DraftingProfileNumericIssue, DraftingProfileNumericRefusal,
};

/// Actual original numeric parameter metadata and kind-checked canonical member under one exact
/// selected Profile parent and fresh content policy. Domain/state/provenance stay in the original
/// source. This proves no aggregate lookup work,G4 schema/evidence/physical truth,value resolution,
/// semantic dependency graph,currentness revalidation or whole-program/execution acceptance.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingProfileNumericProvider<H>>(
///     source:&mut BoundDraftingProfileNumericSource<'_,'_,H,S>) {source.binding=todo!();}
/// ```
/// A parameter proof cannot outlive its exact original selected-source parent loan:
/// ```compile_fail
/// use sc_core::{construction::*,design::{DesignResourceSource,DesignResourceLimits},
///     ontology::ProfileParameterRef,value::ScalarInputKind};
/// fn escape<'b,'a,H:DesignResourceSource,S:DraftingProfileNumericProvider<H>>(
///     parent:&'b BoundDraftingProfileSource<'a,H,S>,parameter:ProfileParameterRef,
///     kind:ScalarInputKind,limits:DesignResourceLimits)
///     ->Result<BoundDraftingProfileNumericSource<'static,'a,H,S>,String> {
///     parent.bind_numeric_parameter(parameter,kind,limits).map_err(|e|e.to_string())
/// }
/// ```
pub struct BoundDraftingProfileNumericSource<
    'b,
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileNumericProvider<H> + ?Sized,
> {
    parent: &'b BoundDraftingProfileSource<'a, H, S>,
    parameter: ProfileParameterRef,
    kind: ScalarInputKind,
    source: DraftingProfileNumericResponse<'a, H, S>,
    binding: CanonicalNumericInputBinding<'a>,
    limits: DesignResourceLimits,
    demand: DesignResourceDemand,
}
impl<'b, 'a, H: DesignMetadataHost, S: DraftingProfileNumericProvider<H> + ?Sized>
    BoundDraftingProfileNumericSource<'b, 'a, H, S>
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
    /// Original requested numeric role,checked against the actual canonical declaration.
    #[must_use]
    pub const fn kind(&self) -> ScalarInputKind {
        self.kind
    }
    /// Actual original metadata and matched selected-source header;no copied body/domain/state.
    #[must_use]
    pub const fn source(&self) -> DraftingProfileNumericResponse<'a, H, S> {
        self.source
    }
    /// Actual original Profile canonical inventory/member/kind/index join.
    #[must_use]
    pub const fn binding(&self) -> CanonicalNumericInputBinding<'a> {
        self.binding
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
impl<H: DesignMetadataHost, S: DraftingProfileNumericProvider<H> + ?Sized> fmt::Debug
    for BoundDraftingProfileNumericSource<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundDraftingProfileNumericSource")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}
impl<H: DesignResourceSource, S: DraftingProfileNumericProvider<H> + ?Sized>
    BoundDraftingProfileNumericSource<'_, '_, H, S>
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
impl<'a, H: DesignResourceSource, S: DraftingProfileNumericProvider<H> + ?Sized>
    BoundDraftingProfileSource<'a, H, S>
{
    /// Freshly charge complete Root/provider/request content before one parameter query. Require
    /// the same original returned Root/pin/inventory/Profile/snapshot header,then parameter ID,
    /// exact numeric representation membership/annotation and actual expected kind. Original target
    /// family remains unchanged;no source/default substitution,
    /// value read,query-count grant or accepted partial parameter escapes failure.
    /// # Errors
    /// Original first resource/provider/header/parameter/canonical/expected-kind cause and returned witnesses.
    pub fn bind_numeric_parameter(
        &self,
        parameter: ProfileParameterRef,
        kind: ScalarInputKind,
        limits: DesignResourceLimits,
    ) -> Result<
        BoundDraftingProfileNumericSource<'_, 'a, H, S>,
        DraftingProfileNumericError<'_, 'a, H, S>,
    > {
        let refuse = |returned, refusal| {
            DraftingProfileNumericError::new(self, parameter, kind, limits, returned, refusal)
        };
        let mut meter = DesignResourceMeter::new(limits);
        let charged = self.charge_resources(&mut meter);
        let demand = meter
            .finish()
            .map_err(|cause| refuse(None, DraftingProfileNumericRefusal::Resources(cause)))?;
        charged.map_err(|cause| refuse(None, DraftingProfileNumericRefusal::Resources(cause)))?;
        let source: DraftingProfileNumericResponse<'a, H, S> = self
            .context()
            .source()
            .profile_numeric_source(self.source(), parameter)
            .map_err(|cause| refuse(None, DraftingProfileNumericRefusal::Source(cause)))?;
        if let Some(issue) = check::issue(self.source(), parameter, source) {
            return Err(refuse(
                Some(source),
                DraftingProfileNumericRefusal::Mismatch(issue),
            ));
        }
        let binding = self
            .source()
            .inputs
            .resolve_numeric(*source.target)
            .map_err(|cause| {
                refuse(
                    Some(source),
                    DraftingProfileNumericRefusal::Canonical(cause),
                )
            })?;
        let actual = match binding.member() {
            CanonicalNumericInputMember::Length { .. } => ScalarInputKind::Length,
            CanonicalNumericInputMember::Scalar { declaration, .. } => declaration.kind(),
        };
        if actual != kind {
            return Err(refuse(
                Some(source),
                DraftingProfileNumericRefusal::Kind { actual },
            ));
        }
        Ok(BoundDraftingProfileNumericSource {
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
