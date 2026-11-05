//! Complete original Root member/supplier projection after both private supplier factories.
use super::{BoundDraftingProgramRootNumericSuppliers, DraftingProgramRootNumericSupplier};
use crate::{
    construction::{
        DraftingContextSource, DraftingProgramRootInputMember, DraftingProgramRootLogicalSupplier,
    },
    design::DesignMetadataHost,
};
use core::{fmt, iter::FusedIterator};

/// Original logical or numeric Derived relation. The two supplier scopes remain distinct;
/// non-Derived members have no supplier. This is a loan,not another source/order validation.
pub enum DraftingProgramRootInputSupplier<
    'r,
    'v,
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    /// Actual Root operation/output and genuine strict earlier mixed supplier position.
    Logical(&'r DraftingProgramRootLogicalSupplier<'v, 'a, H, S>),
    /// Original FormulaDerivedInput,checked source and acyclic owner;no mixed supplier position.
    Numeric(&'r DraftingProgramRootNumericSupplier<'w>),
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingProgramRootInputSupplier<'_, '_, '_, '_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingProgramRootInputSupplier<'_, '_, '_, '_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramRootInputSupplier<'_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logical(supplier) => f.debug_tuple("Logical").field(supplier).finish(),
            Self::Numeric(supplier) => f.debug_tuple("Numeric").field(supplier).finish(),
        }
    }
}
/// One complete original Root operation-input member and its supplier when actually Derived.
/// Private fields keep the member index,source family and actual previously accepted relation
/// aligned. Original bodies/states remain borrowed;Profile/rule/whole-cohort acceptance is separate.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingContextSource<H>>(p:&mut DraftingProgramRootInputBinding<'_,'_,'_,'_,H,S>) {p.supplier=None;}
/// ```
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'r,'v,'w,'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(p:DraftingProgramRootInputBinding<'r,'v,'w,'a,H,S>)->DraftingProgramRootInputBinding<'static,'v,'w,'a,H,S> {p}
/// ```
pub struct DraftingProgramRootInputBinding<
    'r,
    'v,
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    member_index: usize,
    member: &'r DraftingProgramRootInputMember<'v, 'a, H, S>,
    supplier: Option<DraftingProgramRootInputSupplier<'r, 'v, 'w, 'a, H, S>>,
}
impl<'r, 'v, 'w, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramRootInputBinding<'r, 'v, 'w, 'a, H, S>
{
    /// Original field position in the complete retained member sequence,without deduplication.
    #[must_use]
    pub const fn member_index(&self) -> usize {
        self.member_index
    }
    /// Actual original operation/field/consumer ordinal/mixed position and canonical source loan.
    #[must_use]
    pub const fn member(&self) -> DraftingProgramRootInputMember<'v, 'a, H, S> {
        *self.member
    }
    /// Actual retained Derived supplier,or None for the member's original non-Derived state.
    #[must_use]
    pub const fn supplier(&self) -> Option<DraftingProgramRootInputSupplier<'r, 'v, 'w, 'a, H, S>> {
        self.supplier
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramRootInputBinding<'_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramRootInputBinding")
            .field("member_index", &self.member_index)
            .field("supplier", &self.supplier)
            .finish_non_exhaustive()
    }
}
impl<'v, 'w, 'a, I: ?Sized, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    BoundDraftingProgramRootNumericSuppliers<'v, 'w, 'a, I, H, S>
{
    /// Lend every complete original Root member with its actual accepted supplier relation when
    /// Derived. The preceding consuming factories already retain one member owner,one unique
    /// cursor and both complete disjoint supplier classes. This constant-storage linear projection
    /// visits their private sorted indices once;it performs no source lookup,content walk,quota
    /// reservation,allocation,value read or new acceptance. Repeated consumers stay separate.
    /// Profile/rule/garment/native and whole semantic acceptance remain outside this class.
    pub fn bindings(
        &self,
    ) -> impl ExactSizeIterator<Item = DraftingProgramRootInputBinding<'_, 'v, 'w, 'a, H, S>>
           + FusedIterator
           + '_ {
        let mut logical = self.logical.records.iter().peekable();
        let mut numeric = self.records.iter().peekable();
        self.logical
            .members
            .records
            .iter()
            .enumerate()
            .map(move |(member_index, member)| {
                let supplier = if let Some(record) =
                    logical.next_if(|record| record.member_index() == member_index)
                {
                    Some(DraftingProgramRootInputSupplier::Logical(record))
                } else {
                    numeric
                        .next_if(|record| record.member_index() == member_index)
                        .map(DraftingProgramRootInputSupplier::Numeric)
                };
                DraftingProgramRootInputBinding {
                    member_index,
                    member,
                    supplier,
                }
            })
    }
}
