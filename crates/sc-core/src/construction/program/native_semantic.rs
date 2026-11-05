//! Original native semantic fields under the exact normalized program and genuine mixed step.
use super::{DraftingProgramStepKind, DraftingProgramStepView, NormalizedDraftingProgram};
use crate::{
    construction::{
        DraftingContextSource, DraftingNativeSelectionField, DraftingNativeSemanticField,
        DraftingNativeSemanticUse, DraftingSemanticSelectionLocation,
    },
    design::DesignMetadataHost,
};
use core::{fmt, iter::FusedIterator};
/// Actual normalized program/step and original raw native semantic field. This proves original
/// coverage/normalization provenance only;source/order/kind/work/currentness and values stay separate.
/// ```compile_fail,E0616
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingContextSource<H>>(use_:&mut DraftingProgramNativeSemanticUse<'_,'_,'_,H,S>) {use_.native=todo!();}
/// ```
/// The field cannot outlive its actual normalized owner:
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'n,'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(program:&'n NormalizedDraftingProgram<'p,'a,H,S>)
///     ->Option<DraftingProgramNativeSemanticUse<'static,'p,'a,H,S>> {program.native_semantic_dependencies().next()}
/// ```
pub struct DraftingProgramNativeSemanticUse<
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    program: &'n NormalizedDraftingProgram<'p, 'a, H, S>,
    step: DraftingProgramStepView<'n, 'a>,
    native: DraftingNativeSemanticUse<'a>,
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Copy
    for DraftingProgramNativeSemanticUse<'_, '_, '_, H, S>
{
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> Clone
    for DraftingProgramNativeSemanticUse<'_, '_, '_, H, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramNativeSemanticUse<'_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramNativeSemanticUse")
            .field("position", &self.step.position())
            .field("native", &self.native)
            .finish_non_exhaustive()
    }
}
impl<'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramNativeSemanticUse<'n, 'p, 'a, H, S>
{
    /// Exact original complete normalized program owner,not an equal-ID/recomputed substitute.
    #[must_use]
    pub const fn program(self) -> &'n NormalizedDraftingProgram<'p, 'a, H, S> {
        self.program
    }
    /// Actual original mixed native step,without scalar/semantic ordinal reconstruction.
    #[must_use]
    pub const fn step(self) -> DraftingProgramStepView<'n, 'a> {
        self.step
    }
    /// Genuine original one-based mixed position.
    #[must_use]
    pub const fn position(self) -> usize {
        self.step.position()
    }
    /// Exact original native input/field and original pair/side/Profile-role metadata.
    #[must_use]
    pub const fn native(self) -> DraftingNativeSemanticUse<'a> {
        self.native
    }
    /// Fixed original native location at this genuine mixed position,including actual target,
    /// Notch role or Walk pair/A/B/Profile role. Repeated raw loans in different mixed slots stay
    /// distinct. This syntax projection proves no row coverage or target-internal Profile need.
    #[must_use]
    pub const fn selection_location(self) -> DraftingSemanticSelectionLocation {
        use DraftingNativeSelectionField as N;
        let field = match self.native.field() {
            DraftingNativeSemanticField::Target { role, reference: _ } => N::Target(role),
            DraftingNativeSemanticField::NotchProfile { role, reference: _ } => {
                N::NotchProfile(role)
            }
            DraftingNativeSemanticField::WalkNotchProfile {
                pair_index,
                side,
                role,
                reference: _,
            } => N::WalkNotchProfile {
                pair_index,
                side,
                role,
            },
        };
        DraftingSemanticSelectionLocation::Native {
            position: self.position(),
            field,
        }
    }
}
impl<'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    NormalizedDraftingProgram<'p, 'a, H, S>
{
    /// Stream every raw native semantic field in genuine mixed order and original local field order.
    /// Reuse the exhaustive all13 raw producer;scalar/semantic slots and other dependency classes
    /// retain their own obligations. No callbacks,values,new arenas,clones or result allocation.
    /// This syntax/coverage view grants no source/order/kind or fresh aggregate-work acceptance.
    /// ```
    /// use sc_core::{construction::*,design::DesignMetadataHost};
    /// fn count<H:DesignMetadataHost,S:DraftingContextSource<H>+?Sized>(program:&NormalizedDraftingProgram<'_,'_,H,S>)->usize {
    ///     program.native_semantic_dependencies().count()
    /// }
    /// ```
    pub fn native_semantic_dependencies(
        &self,
    ) -> impl FusedIterator<Item = DraftingProgramNativeSemanticUse<'_, 'p, 'a, H, S>> + '_ {
        self.steps()
            .filter_map(|step| match step.kind() {
                DraftingProgramStepKind::Drafting(operation) => Some((step, operation.input())),
                DraftingProgramStepKind::Scalar { .. }
                | DraftingProgramStepKind::Semantic { .. } => None,
            })
            .flat_map(move |(step, input)| {
                input
                    .semantic_dependencies()
                    .map(move |native| DraftingProgramNativeSemanticUse {
                        program: self,
                        step,
                        native,
                    })
            })
            .fuse()
    }
}
