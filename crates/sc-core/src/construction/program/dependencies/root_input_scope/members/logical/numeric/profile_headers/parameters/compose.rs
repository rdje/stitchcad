//! Complete original Root/Profile operation-input projection,without a new acceptance pass.
use super::{BoundDraftingProgramProfileParameters, DraftingProgramProfileInputParameter};
use crate::{
    construction::{
        DraftingProfileLogicalProvider, DraftingProgramRootInputBinding, DraftingSemanticField,
        DraftingSemanticSelectionLocation,
    },
    design::DesignMetadataHost,
    recipe::SemanticOperation,
};
use core::{cmp::Ordering, fmt, iter::FusedIterator};

/// One actual original operation-input field under its accepted Root or Profile source class.
/// The containing factory chain supplies completeness;this individual borrowed view creates no
/// source/member/order/value grant and supplies no Profile-internal Root supplier position.
/// ```compile_fail
/// use sc_core::{construction::*,design::DesignMetadataHost};
/// fn escape<'o,'h,'v,'w,'a,H:DesignMetadataHost,S:DraftingProfileLogicalProvider<H>>(p:DraftingProgramOperationInput<'o,'h,'v,'w,'a,H,S>)->DraftingProgramOperationInput<'static,'h,'v,'w,'a,H,S> {p}
/// ```
pub enum DraftingProgramOperationInput<
    'o,
    'h,
    'v,
    'w,
    'a,
    H: DesignMetadataHost,
    S: DraftingProfileLogicalProvider<H> + ?Sized,
> {
    /// Original Root canonical member and its actual logical/numeric supplier when Derived.
    Root(DraftingProgramRootInputBinding<'o, 'v, 'w, 'a, H, S>),
    /// Actual original Profile input/header/metadata/canonical member,with unchanged external state.
    Profile(&'o DraftingProgramProfileInputParameter<'h, 'w, 'a, H, S>),
}
impl<'a, H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized>
    DraftingProgramOperationInput<'_, '_, '_, '_, 'a, H, S>
{
    /// Exact original Root consumer operation,never a source operation substituted by matching ID.
    #[must_use]
    pub fn operation(&self) -> &'a SemanticOperation {
        match self {
            Self::Root(input) => input.member().operation(),
            Self::Profile(input) => input.header().request().operation(),
        }
    }
    /// Genuine original semantic consumer ordinal,independent of mixed position.
    #[must_use]
    pub fn ordinal(&self) -> usize {
        match self {
            Self::Root(input) => input.member().ordinal(),
            Self::Profile(input) => input.header().request().ordinal(),
        }
    }
    /// Genuine original mixed consumer position,not a supplier or canonical family position.
    #[must_use]
    pub fn position(&self) -> usize {
        match self {
            Self::Root(input) => input.member().position(),
            Self::Profile(input) => input.header().request().position(),
        }
    }
    /// Exact original field coordinates under the complete retained program owner.
    #[must_use]
    pub fn location(&self) -> DraftingSemanticSelectionLocation {
        match self {
            Self::Root(input) => input.member().location(),
            Self::Profile(input) => input.header().request().location(),
        }
    }
    /// Actual original typed button/named-input field loan,without copying its body or role text.
    #[must_use]
    pub fn field(&self) -> DraftingSemanticField<'a> {
        match self {
            Self::Root(input) => input.member().field(),
            Self::Profile(input) => input.header().request().field(),
        }
    }
}
impl<H: DesignMetadataHost, S: DraftingProfileLogicalProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramOperationInput<'_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Root(input) => f.debug_tuple("Root").field(input).finish(),
            Self::Profile(input) => f.debug_tuple("Profile").field(input).finish(),
        }
    }
}
fn key(position: usize, field: DraftingSemanticField<'_>) -> (usize, usize) {
    let local = match field {
        DraftingSemanticField::ButtonholeButton(_) => 0,
        DraftingSemanticField::ButtonholeInput { index, .. } => {
            index.checked_add(1).unwrap_or_else(|| {
                unreachable!(
                    "actual nonzero-size input vector indices fit their one-based local rank"
                )
            })
        }
        DraftingSemanticField::LogicalInput { index, .. } => index,
        DraftingSemanticField::ButtonholePolicy(_)
        | DraftingSemanticField::LogicalRule(_)
        | DraftingSemanticField::LogicalOutput(_) => {
            unreachable!("private complete input classes exclude rule/policy/output headers")
        }
    };
    // Local rank only compares actual fields of the same original operation. It is never exposed
    // as a mixed/supplier/canonical position; the genuine mixed consumer position stays unchanged.
    (position, local)
}
impl<
        'h,
        'v,
        'w,
        'a,
        I: ?Sized,
        H: DesignMetadataHost,
        S: DraftingProfileLogicalProvider<H> + ?Sized,
    > BoundDraftingProgramProfileParameters<'h, 'v, 'w, 'a, I, H, S>
{
    /// Complete actual Root+Profile operation-input population. These disjoint original classes
    /// partition the already checked finite semantic field population,so their sum fits usize.
    #[must_use]
    pub fn operation_input_count(&self) -> usize {
        self.root
            .logical()
            .members()
            .records()
            .len()
            .checked_add(self.records.len())
            .unwrap_or_else(|| {
                unreachable!(
                    "complete disjoint original input classes fit admitted occurrence bounds"
                )
            })
    }
    /// Lend every original Root/Profile operation input once,in genuine mixed/local schema order.
    /// Merge the complete private sorted classes using actual consumer positions/field indices;
    /// a Buttonhole button precedes its named numeric policy inputs. No sort buffer,allocation,
    /// source query,resource walk,reservation,value read or new acceptance pass occurs. Previous
    /// source/supplier scopes and the same unique borrowed cursor remain original and unchanged.
    /// Rule/signature/output,garment/native targets,external Profile recipe/G4 truth,physical
    /// execution and whole semantic acceptance stay separate. Prefix reads view an already
    /// complete owner;refused factories never expose a successful construction prefix.
    pub fn operation_inputs(
        &self,
    ) -> impl FusedIterator<Item = DraftingProgramOperationInput<'_, 'h, 'v, 'w, 'a, H, S>> + '_
    {
        let mut root = self.root.bindings().peekable();
        let mut profile = self.records.iter().peekable();
        core::iter::from_fn(move || {
            let take_root = match (root.peek(), profile.peek()) {
                (Some(left), Some(right)) => {
                    let left = left.member();
                    let right = right.header().request();
                    match key(left.position(), left.field())
                        .cmp(&key(right.position(), right.field()))
                    {
                        Ordering::Less => true,
                        Ordering::Greater => false,
                        Ordering::Equal => {
                            unreachable!("original Root/Profile input origins are disjoint fields")
                        }
                    }
                }
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (None, None) => return None,
            };
            if take_root {
                root.next().map(DraftingProgramOperationInput::Root)
            } else {
                profile.next().map(DraftingProgramOperationInput::Profile)
            }
        })
        .fuse()
    }
}
