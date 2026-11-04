//! Fresh complete Root/provider content and readonly actual semantic member loans.
use super::{DraftingContext, DraftingContextSource};
use crate::{
    design::{
        charge_design_input_resources, fixed_value, DesignMetadataHost, DesignResourceDemand,
        DesignResourceLimitError, DesignResourceLimits, DesignResourceLocation,
        DesignResourceMeter, DesignResourceScope, DesignResourceSink, DesignResourceSource,
    },
    ontology::EntityId,
    recipe::SemanticOperationKind,
    value::LogicalInputTarget,
};
use core::fmt;

/// Actual original context,selected complete content policy and first resource refusal.
/// Only refused content diagnostics allocate; no member/result collection is constructed.
pub struct DraftingDesignSemanticScopeError<
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    details: Box<ScopeFailure<'a, H, S>>,
}
struct ScopeFailure<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> {
    context: DraftingContext<'a, H, S>,
    limits: DesignResourceLimits,
    cause: DesignResourceLimitError,
}
impl<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingDesignSemanticScopeError<'a, H, S>
{
    /// Original actual Root,source provider,frame and snapshot loans.
    #[must_use]
    pub const fn context(&self) -> DraftingContext<'a, H, S> {
        self.details.context
    }
    /// Original explicit fresh complete content allowance.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.details.limits
    }
    /// First actual field refusal,including an ignored provider-sink failure.
    #[must_use]
    pub const fn cause(&self) -> &DesignResourceLimitError {
        &self.details.cause
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingDesignSemanticScopeError<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingDesignSemanticScopeError")
            .field("cause", &self.details.cause)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingDesignSemanticScopeError<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Design semantic source content: {}", self.details.cause)
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingDesignSemanticScopeError<'_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.details.cause)
    }
}

/// Fresh actual Root/provider content scope for individual original logical/operation member loans.
/// It retains the existing immutable context's Root/snapshot association; it performs no new
/// currentness query. This is not complete garment content,Profile/rule binding,aggregate lookup
/// work,semantic dependency order,evidence truth,value or whole-program/execution acceptance.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingContextSource<H>>(scope:&mut DraftingDesignSemanticScope<'_,H,S>) {
///     scope.context=todo!();
/// }
/// ```
pub struct DraftingDesignSemanticScope<
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    context: DraftingContext<'a, H, S>,
    limits: DesignResourceLimits,
    demand: DesignResourceDemand,
}
impl<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingDesignSemanticScope<'a, H, S>
{
    /// Original containing context,including its actual Root and snapshot record.
    #[must_use]
    pub const fn context(&self) -> DraftingContext<'a, H, S> {
        self.context
    }
    /// Explicit selected complete fresh Root/provider content allowance.
    #[must_use]
    pub const fn limits(&self) -> DesignResourceLimits {
        self.limits
    }
    /// Complete freshly walked demand; no earlier context or Root policy grants credit.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Loan the actual Root logical member and its original domain/state/provenance.
    /// This performs one allocation-free inventory lookup; it invokes no provider or value query.
    /// A containing complete checker must separately bound all its requests/comparisons before
    /// using this primitive. Unknown/Derived remain unchanged and grant no selected result.
    /// # Errors
    /// Original requested ID/kind and actual missing/kind cause under this exact source scope.
    pub fn logical_source(
        &self,
        target: LogicalInputTarget,
    ) -> Result<
        DraftingDesignLogicalSource<'_, 'a, H, S>,
        DraftingDesignSemanticLookupError<'_, 'a, H, S>,
    > {
        let binding = self
            .context
            .scope()
            .root()
            .inputs()
            .logicals()
            .resolve(target.declaration, target.kind)
            .map_err(|cause| DraftingDesignSemanticLookupError {
                scope: self,
                request: DraftingDesignSemanticRequest::Logical(target),
                cause: DraftingDesignSemanticLookupCause::Logical(cause),
            })?;
        Ok(DraftingDesignLogicalSource {
            scope: self,
            binding,
        })
    }
    /// Loan the actual Root semantic member and original semantic recipe position.
    /// This neither interprets rule/policy/input IDs nor invents a mixed/scalar/native position.
    /// No provider,currentness,value,rule lookup or result allocation occurs.
    /// # Errors
    /// Original requested ID/kind and actual missing/kind cause under this exact source scope.
    pub fn operation_source(
        &self,
        operation: EntityId,
        kind: SemanticOperationKind,
    ) -> Result<
        DraftingDesignOperationSource<'_, 'a, H, S>,
        DraftingDesignSemanticLookupError<'_, 'a, H, S>,
    > {
        let binding = self
            .context
            .scope()
            .root()
            .semantic_recipe()
            .resolve(operation, kind)
            .map_err(|cause| DraftingDesignSemanticLookupError {
                scope: self,
                request: DraftingDesignSemanticRequest::Operation { operation, kind },
                cause: DraftingDesignSemanticLookupCause::Operation(cause),
            })?;
        Ok(DraftingDesignOperationSource {
            scope: self,
            binding,
        })
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingDesignSemanticScope<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingDesignSemanticScope")
            .field("demand", &self.demand)
            .finish_non_exhaustive()
    }
}
pub(super) fn charge_context<H: DesignResourceSource, S: DraftingContextSource<H> + ?Sized>(
    context: DraftingContext<'_, H, S>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let root = context.scope().root();
    charge_design_input_resources(root, sink)?;
    let charged = context.source().charge_resources(sink);
    // Preserve an ignored first sink failure before any later returned provider refusal.
    sink.charge_entries(
        0,
        DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(root.id()),
            field: "drafting.semantic.sources",
            index: None,
        },
    )?;
    charged
}
impl<H: DesignResourceSource, S: DraftingContextSource<H> + ?Sized>
    DraftingDesignSemanticScope<'_, H, S>
{
    /// Append every actual Root/provider field to a cumulative sink again. Cached demand/limits
    /// never replace the fresh source walk. No member lookup,copy or validation occurs.
    /// # Errors
    /// Original first actual Root/provider field refusal,including previously latched failures.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            context,
            limits,
            demand,
        } = self;
        fixed_value(limits);
        fixed_value(demand);
        charge_context(*context, sink)
    }
}
impl<'a, H: DesignResourceSource, S: DraftingContextSource<H> + ?Sized> DraftingContext<'a, H, S> {
    /// Freshly charge all actual Root content,then all held provider fields in one cumulative meter,
    /// before lending any logical/operation member. Earlier admissions grant no aggregate credit.
    /// No source/root/currentness/member query,clone,result collection or value read occurs.
    /// # Errors
    /// Original complete first content refusal,retaining actual context and selected policy.
    pub fn prepare_design_semantic_sources(
        self,
        limits: DesignResourceLimits,
    ) -> Result<DraftingDesignSemanticScope<'a, H, S>, DraftingDesignSemanticScopeError<'a, H, S>>
    {
        let mut meter = DesignResourceMeter::new(limits);
        let charged = charge_context(self, &mut meter);
        let refuse = |cause| DraftingDesignSemanticScopeError {
            details: Box::new(ScopeFailure {
                context: self,
                limits,
                cause,
            }),
        };
        let demand = meter.finish().map_err(refuse)?;
        charged.map_err(refuse)?;
        Ok(DraftingDesignSemanticScope {
            context: self,
            limits,
            demand,
        })
    }
}

mod lookup;
mod members;
pub use lookup::{
    DraftingDesignSemanticLookupCause, DraftingDesignSemanticLookupError,
    DraftingDesignSemanticRequest,
};
pub use members::{DraftingDesignLogicalSource, DraftingDesignOperationSource};
