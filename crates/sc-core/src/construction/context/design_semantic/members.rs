//! Actual canonical/semantic member loans pinned to their original fresh source scope.
use super::{DraftingContextSource, DraftingDesignSemanticScope};
use crate::{
    design::DesignMetadataHost,
    recipe::{SemanticOperation, SemanticOperationBinding},
    value::{
        CanonicalNumericInputBinding, CanonicalNumericInputMember, LogicalInputBinding,
        LogicalInputDeclaration,
    },
};
use core::fmt;

/// Actual original numeric member pinned to the exact freshly admitted Root/provider scope.
/// Original representation,kind,state and provenance stay in their canonical owner. This lends
/// no value,source truth,Derived supplier/order or complete aggregate semantic acceptance.
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost,value::CanonicalInputTarget};
/// fn escape<'s,'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     scope:&'s DraftingDesignSemanticScope<'a,H,S>,target:CanonicalInputTarget)
///     ->DraftingDesignNumericSource<'static,'a,H,S> {scope.numeric_source(target).unwrap()}
/// ```
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingContextSource<H>>(p:&mut DraftingDesignNumericSource<'_,'_,H,S>) {p.scope=todo!();}
/// ```
pub struct DraftingDesignNumericSource<
    's,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    pub(super) scope: &'s DraftingDesignSemanticScope<'a, H, S>,
    pub(super) binding: CanonicalNumericInputBinding<'a>,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingDesignNumericSource<'_, '_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingDesignNumericSource<'_, '_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<'s, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingDesignNumericSource<'s, 'a, H, S>
{
    /// Exact original fresh source scope and its Root/provider/frame/snapshot/policy loans.
    #[must_use]
    pub const fn scope(self) -> &'s DraftingDesignSemanticScope<'a, H, S> {
        self.scope
    }
    /// Exact original canonical inventory,target,kind and actual authored member position.
    #[must_use]
    pub const fn binding(self) -> CanonicalNumericInputBinding<'a> {
        self.binding
    }
    /// Actual original numeric declaration view,without state or numeric-value copying.
    #[must_use]
    pub const fn member(self) -> CanonicalNumericInputMember<'a> {
        self.binding.member()
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingDesignNumericSource<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingDesignNumericSource")
            .field("target", &self.binding.target())
            .field("position", &self.binding.position())
            .finish_non_exhaustive()
    }
}

/// Actual original kind-checked logical member pinned to its complete fresh Root/source scope.
/// Domain/state/provenance stay in the canonical owner;no selected value or evidence is created.
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost,value::LogicalInputTarget};
/// fn escape<'s,'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     scope:&'s DraftingDesignSemanticScope<'a,H,S>,target:LogicalInputTarget)
///     ->DraftingDesignLogicalSource<'static,'a,H,S> {scope.logical_source(target).unwrap()}
/// ```
pub struct DraftingDesignLogicalSource<
    's,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    pub(super) scope: &'s DraftingDesignSemanticScope<'a, H, S>,
    pub(super) binding: LogicalInputBinding<'a>,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingDesignLogicalSource<'_, '_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingDesignLogicalSource<'_, '_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<'s, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingDesignLogicalSource<'s, 'a, H, S>
{
    /// Actual original fresh scope;this member cannot outlive its scope loan.
    #[must_use]
    pub const fn scope(self) -> &'s DraftingDesignSemanticScope<'a, H, S> {
        self.scope
    }
    /// Actual canonical inventory/member/original index and kind join.
    #[must_use]
    pub const fn binding(self) -> LogicalInputBinding<'a> {
        self.binding
    }
    /// Actual canonical record,without copying its domain or uncertainty state.
    #[must_use]
    pub const fn declaration(self) -> &'a LogicalInputDeclaration {
        self.binding.declaration()
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingDesignLogicalSource<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingDesignLogicalSource")
            .finish_non_exhaustive()
    }
}

/// Actual original kind-checked semantic member under the same complete fresh Root/source scope.
/// Its original semantic index is neither a scalar ordinal nor a native/mixed program position.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingContextSource<H>>(source:&mut DraftingDesignOperationSource<'_,'_,H,S>) {
///     source.binding=todo!();
/// }
/// ```
pub struct DraftingDesignOperationSource<
    's,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    pub(super) scope: &'s DraftingDesignSemanticScope<'a, H, S>,
    pub(super) binding: SemanticOperationBinding<'a>,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingDesignOperationSource<'_, '_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingDesignOperationSource<'_, '_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<'s, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingDesignOperationSource<'s, 'a, H, S>
{
    /// Exact original fresh scope and actual Root/snapshot/policy loans.
    #[must_use]
    pub const fn scope(self) -> &'s DraftingDesignSemanticScope<'a, H, S> {
        self.scope
    }
    /// Actual original recipe/member/semantic-index join.
    #[must_use]
    pub const fn binding(self) -> SemanticOperationBinding<'a> {
        self.binding
    }
    /// Actual immutable operation body,without source/rule/dependency or execution acceptance.
    #[must_use]
    pub const fn operation(self) -> &'a SemanticOperation {
        self.binding.operation()
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingDesignOperationSource<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingDesignOperationSource")
            .finish_non_exhaustive()
    }
}
