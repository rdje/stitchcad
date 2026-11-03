//! Direct original native fields under actual complete canonical and mixed order owners.
use super::BoundDraftingProgramCanonicalInputs;
use crate::{
    construction::{
        DraftingGeometryReference, DraftingInitialInputProvider, DraftingNativeGeometryUse,
        DraftingProgramOrderStep,
    },
    design::DesignMetadataHost,
};
use core::iter::FusedIterator;

/// One original native field pinned to its actual complete checked mixed consumer.
/// Private construction keeps the genuine programme position and original typed field together.
/// This is dependency evidence, without a geometry supplier, semantic source or execution grant.
/// ```compile_fail,E0616
/// fn alter(use_:&mut sc_core::construction::DraftingProgramNativeGeometryUse<'_, '_>) {use_.step=todo!();}
/// ```
/// ```compile_fail
/// fn escape<'v,'j>(use_:sc_core::construction::DraftingProgramNativeGeometryUse<'v,'j>)
///     ->sc_core::construction::DraftingProgramNativeGeometryUse<'static,'j> {use_}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct DraftingProgramNativeGeometryUse<'v, 'j> {
    step: &'v DraftingProgramOrderStep<'j>,
    native: DraftingNativeGeometryUse<'j>,
}
impl<'v, 'j> DraftingProgramNativeGeometryUse<'v, 'j> {
    /// Actual checked mixed consumer, borrowing the complete original order proof.
    #[must_use]
    pub const fn step(self) -> &'v DraftingProgramOrderStep<'j> {
        self.step
    }
    /// Genuine one-based mixed position; no scalar ordinal or source span is manufactured.
    #[must_use]
    pub const fn position(self) -> usize {
        self.step.position()
    }
    /// Original raw field witness, including its actual input, local index and typed selector.
    #[must_use]
    pub const fn native(self) -> DraftingNativeGeometryUse<'j> {
        self.native
    }
    /// Original persistent reference, without resolving or accepting a geometry supplier.
    #[must_use]
    pub fn reference(self) -> DraftingGeometryReference {
        self.native.reference()
    }
}
impl<
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Stream every direct native geometry field in genuine complete mixed order, then original
    /// local field order. The private checked order and normalized slots share the same admitted
    /// programme. Reuse each original raw field producer; no reconstructed family schema, names,
    /// spans, scalar ordinals, buffers, source/provider callbacks or value reads participate.
    /// Scalar and semantic-only slots have no direct fields. Formula/semantic uses, complete
    /// geometry bounds and supplier/frame/currentness binding remain separate obligations.
    ///
    /// ```
    /// use sc_core::{construction::*,design::DesignMetadataHost};
    /// fn native_uses<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     inputs:&BoundDraftingProgramCanonicalInputs<'_, '_, '_, '_, '_, '_, '_, '_,H,S>) -> usize {
    ///     inputs.native_geometry_dependencies().count()
    /// }
    /// ```
    pub fn native_geometry_dependencies(
        &self,
    ) -> impl FusedIterator<Item = DraftingProgramNativeGeometryUse<'_, 'j>> + '_ {
        let order = self.inputs().order();
        let program = order.inputs().roles().positions().work().inputs().program();
        order
            .steps()
            .iter()
            .flat_map(move |step| {
                program
                    .operations()
                    .get(step.position() - 1)
                    .unwrap_or_else(|| {
                        unreachable!("private complete mixed order/normalized slots")
                    })
                    .as_ref()
                    .into_iter()
                    .flat_map(|operation| operation.input().geometry_dependencies())
                    .map(move |native| DraftingProgramNativeGeometryUse { step, native })
            })
            .fuse()
    }
}
