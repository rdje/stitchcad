//! Complete actual content and conservative matching work before original operation lookup.
use super::{DraftingRuleProvider, DraftingRuleRequest, DraftingRuleSelection};
use crate::{
    construction::DraftingContext,
    design::{
        charge_selected_profile_pin_resources, fixed_value, DesignMetadataHost,
        DesignResourceDemand, DesignResourceLimitError, DesignResourceMeter, DesignResourceScope,
        DesignResourceSink, DesignResourceSource,
    },
    ontology::EntityId,
    recipe::{SemanticOperation, SemanticOperationBinding, SemanticOperationKind},
};
use core::fmt;
mod error;
mod limits;
pub use error::{DraftingRuleWorkError, DraftingRuleWorkRefusal};
pub use limits::{DraftingRuleBindingLimits, DraftingRuleBindingShape};

/// Actual original Root operation after fresh whole content and prospective matching bounds.
/// It retains the exact selection intent,not an accepted Profile/rule/body/signature association.
/// No source,canonical input/output,domain/state,value,physical result or program is accepted.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingRuleProvider<H>>(work:&mut AdmittedDraftingRuleWork<'_,H,S>) {
///     work.operation=todo!();
/// }
/// ```
/// The scope cannot outlive a shorter requested Profile pin:
/// ```compile_fail,E0515
/// use sc_core::{construction::*,design::DesignResourceSource,ontology::*,recipe::SemanticOperationKind};
/// fn escape<'a,H:DesignResourceSource,S:DraftingRuleProvider<H>>(context:DraftingContext<'a,H,S>,
///     limits:DraftingRuleBindingLimits)->Result<AdmittedDraftingRuleWork<'a,H,S>,String> {
///     let selected=SelectedProfilePin::new(SelectedProfilePinDefinition {
///         profile_id:"atelier".into(),schema_version:"2".into(),content_version:"7".into(),
///     }).unwrap();
///     context.prepare_rule_binding(EntityId::from_bits(80),SemanticOperationKind::LogicalDerivation,
///         DraftingRuleSelection::Profile(&selected),limits).map_err(|e|e.to_string())
/// }
/// ```
pub struct AdmittedDraftingRuleWork<'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
{
    context: DraftingContext<'a, H, S>,
    operation: SemanticOperationBinding<'a>,
    selection: DraftingRuleSelection<'a>,
    limits: DraftingRuleBindingLimits,
    demand: DesignResourceDemand,
    shape: DraftingRuleBindingShape,
}
impl<'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    AdmittedDraftingRuleWork<'a, H, S>
{
    /// Exact original immutable Root/provider/frame/snapshot context.
    #[must_use]
    pub const fn context(&self) -> DraftingContext<'a, H, S> {
        self.context
    }
    /// Actual original semantic recipe/member/index,not a mixed/scalar/native position.
    #[must_use]
    pub const fn operation(&self) -> SemanticOperationBinding<'a> {
        self.operation
    }
    /// Original explicit Design/Profile selection intent,without accepted source association.
    #[must_use]
    pub const fn selection(&self) -> DraftingRuleSelection<'a> {
        self.selection
    }
    /// Exact selected fresh content and complete prospective inspection policy.
    #[must_use]
    pub const fn limits(&self) -> DraftingRuleBindingLimits {
        self.limits
    }
    /// Fresh complete Root/provider/request-pin demand,without earlier admission credit.
    #[must_use]
    pub const fn demand(&self) -> DesignResourceDemand {
        self.demand
    }
    /// Complete actual held population and conservative prospective matching bound.
    #[must_use]
    pub const fn shape(&self) -> DraftingRuleBindingShape {
        self.shape
    }
    /// Exact original rule/policy ID and family derived from the actual bound Root operation.
    /// This names an intended source;it performs no lookup or accepted association.
    #[must_use]
    pub const fn request(&self) -> DraftingRuleRequest<'a> {
        let operation = self.operation.operation();
        let rule = match operation {
            SemanticOperation::Buttonhole(operation) => operation.definition().policy,
            SemanticOperation::LogicalDerivation(operation) => operation.definition().rule,
        };
        DraftingRuleRequest {
            rule,
            kind: operation.kind(),
            origin: self.selection,
        }
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for AdmittedDraftingRuleWork<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingRuleWork")
            .field("operation", &self.operation.operation().id())
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
fn charge<H: DesignResourceSource, S: DraftingRuleProvider<H> + ?Sized>(
    context: DraftingContext<'_, H, S>,
    selection: DraftingRuleSelection<'_>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    super::super::design_semantic::charge_context(context, sink)?;
    match selection {
        DraftingRuleSelection::Design => Ok(()),
        DraftingRuleSelection::Profile(selected) => charge_selected_profile_pin_resources(
            selected,
            sink,
            DesignResourceScope::CommandState,
            Some(context.scope().root().id()),
            None,
        ),
    }
}
impl<H: DesignResourceSource, S: DraftingRuleProvider<H> + ?Sized>
    AdmittedDraftingRuleWork<'_, H, S>
{
    /// Freshly append every actual Root/provider/request-pin field in the caller's cumulative sink.
    /// Original operation/selection are borrowed views;policy/demand/shape grant no cached credit.
    /// No source/currentness/member/value query,matching scan or result buffer occurs.
    /// # Errors
    /// Original first cumulative resource refusal,including earlier or ignored sink failures.
    pub fn charge_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            context,
            operation,
            selection,
            limits,
            demand,
            shape,
        } = self;
        fixed_value(operation);
        fixed_value(selection);
        fixed_value(limits);
        fixed_value(demand);
        fixed_value(shape);
        charge(*context, *selection, sink)
    }
}
impl<'a, H: DesignResourceSource, S: DraftingRuleProvider<H> + ?Sized> DraftingContext<'a, H, S> {
    /// Freshly walk complete Root/provider/request-pin content,derive checked (E+8)² and refuse
    /// inspection shortage before original Root operation lookup. E counts all actual held dynamic
    /// collection entries,including unused signatures/caches;no supplied demand or prior proof gives
    /// credit. This conservative bound covers planned linear/member/header and quadratic role work,
    /// not string-comparison bytes,provider-internal CPU,allocator capacity or execution.
    /// No Profile/rule/currentness/value query or result vector allocation occurs. A refused
    /// diagnostic is boxed. The selection pin may be borrowed for less time than the context.
    /// # Errors
    /// First whole content,arithmetic,inspection or original lending operation lookup cause.
    pub fn prepare_rule_binding<'p>(
        self,
        operation: EntityId,
        kind: SemanticOperationKind,
        selection: DraftingRuleSelection<'p>,
        limits: DraftingRuleBindingLimits,
    ) -> Result<AdmittedDraftingRuleWork<'p, H, S>, DraftingRuleWorkError<'p, H, S>>
    where
        'a: 'p,
    {
        let context: DraftingContext<'p, H, S> = self;
        let refuse = |refusal| {
            DraftingRuleWorkError::new(context, operation, kind, selection, limits, refusal)
        };
        let mut meter = DesignResourceMeter::new(limits.content());
        let charged = charge(context, selection, &mut meter);
        let demand = meter
            .finish()
            .map_err(|cause| refuse(DraftingRuleWorkRefusal::Resources(cause)))?;
        charged.map_err(|cause| refuse(DraftingRuleWorkRefusal::Resources(cause)))?;
        let shape =
            DraftingRuleBindingShape::admit(demand.collection_entries(), limits).map_err(refuse)?;
        let operation = context
            .scope()
            .root()
            .semantic_recipe()
            .resolve(operation, kind)
            .map_err(|cause| refuse(DraftingRuleWorkRefusal::Operation(cause)))?;
        Ok(AdmittedDraftingRuleWork {
            context,
            operation,
            selection,
            limits,
            demand,
            shape,
        })
    }
}
