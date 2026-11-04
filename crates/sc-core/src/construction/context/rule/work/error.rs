//! First original complete work or lending Root operation cause,without source lookup diagnostics.
use super::{
    DraftingContext, DraftingRuleBindingLimits, DraftingRuleProvider, DraftingRuleSelection,
};
use crate::{
    design::{DesignMetadataHost, DesignResourceLimitError},
    ontology::EntityId,
    recipe::{SemanticOperationKind, SemanticOperationLookupError},
};
use core::fmt;
/// First fresh whole content,prospective work or actual Root-member refusal.
#[derive(Clone, Copy, Debug)]
pub enum DraftingRuleWorkRefusal<'a> {
    /// Original first complete Root/provider/request field refusal.
    Resources(DesignResourceLimitError),
    /// Complete E+8 or (E+8)² cannot fit usize.
    PopulationOverflow,
    /// Complete conservative matching bound exceeds its explicit selected allowance.
    MatchingInspections {
        /// Exact selected maximum.
        allowed: usize,
        /// Complete conservative original requirement.
        required: usize,
    },
    /// Original requested ID/kind and actual lending missing/kind cause.
    Operation(SemanticOperationLookupError<'a>),
}
/// Exact original context/request/selection/policy and first refusal;no accepted prefix exists.
pub struct DraftingRuleWorkError<'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> {
    details: Box<Failure<'a, H, S>>,
}
struct Failure<'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> {
    context: DraftingContext<'a, H, S>,
    operation: EntityId,
    kind: SemanticOperationKind,
    selection: DraftingRuleSelection<'a>,
    limits: DraftingRuleBindingLimits,
    refusal: DraftingRuleWorkRefusal<'a>,
}
impl<'a, H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized>
    DraftingRuleWorkError<'a, H, S>
{
    pub(super) fn new(
        context: DraftingContext<'a, H, S>,
        operation: EntityId,
        kind: SemanticOperationKind,
        selection: DraftingRuleSelection<'a>,
        limits: DraftingRuleBindingLimits,
        refusal: DraftingRuleWorkRefusal<'a>,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                context,
                operation,
                kind,
                selection,
                limits,
                refusal,
            }),
        }
    }
    /// Exact original actual Root/provider/frame/snapshot context.
    #[must_use]
    pub const fn context(&self) -> DraftingContext<'a, H, S> {
        self.details.context
    }
    /// Original requested operation identity,even when no member exists.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.details.operation
    }
    /// Original requested operation family.
    #[must_use]
    pub const fn kind(&self) -> SemanticOperationKind {
        self.details.kind
    }
    /// Original explicit Design/Profile selection intent,without normalization.
    #[must_use]
    pub const fn selection(&self) -> DraftingRuleSelection<'a> {
        self.details.selection
    }
    /// Exact selected complete content/work policy.
    #[must_use]
    pub const fn limits(&self) -> DraftingRuleBindingLimits {
        self.details.limits
    }
    /// First actual field/population/quota or original lending member cause.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingRuleWorkRefusal<'a> {
        &self.details.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Debug
    for DraftingRuleWorkError<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingRuleWorkError")
            .field("operation", &self.details.operation)
            .field("refusal", &self.details.refusal)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> fmt::Display
    for DraftingRuleWorkError<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting rule work: {:?}", self.details.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingRuleProvider<H> + ?Sized> std::error::Error
    for DraftingRuleWorkError<'_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingRuleWorkRefusal::Resources(cause) => Some(cause),
            _ => None,
        }
    }
}
