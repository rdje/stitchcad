//! Original direct selection need classification,without actual row/source/target acceptance.
use super::{DraftingSemanticSelectionKey as Key, DraftingSemanticSelectionLocation};
use crate::{
    construction::{
        DraftingContextSource, DraftingGarmentSemanticField, DraftingGarmentSemanticUse,
        DraftingNativeSemanticField, DraftingProgramNativeSemanticUse, DraftingProgramSemanticUse,
        DraftingSemanticField,
    },
    design::DesignMetadataHost,
    ontology::{ClosureLength, NotionSize, PocketOpening},
    recipe::LogicalDerivationInputSource,
};

/// Original field syntax's direct need or unresolved native target context. Public construction
/// supplies no actual consumer,request coverage,source,Profile member or work proof.
/// No selected value,domain,Profile pin or implicit rule origin is manufactured.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingSemanticSelectionNeed {
    /// This field carries no direct Profile pin or rule-origin request. Its Root/numeric/geometry/
    /// component and other source obligations remain with their original binders.
    NoDirectSelection,
    /// This genuine field requires exactly its explicit direct Profile or rule-origin key.
    /// The key does not prove that a row exists or that its selected source is acceptable.
    Required(Key),
    /// A native canonical target must be authentically associated before its internal Profile
    /// needs are known. A context row here is merely potential intent,not consumed coverage.
    DeferredTargetProfile(DraftingSemanticSelectionLocation),
}
const fn notion_needs_profile(source: &NotionSize) -> bool {
    match source {
        NotionSize::Declaration(_) => false,
        NotionSize::Profile(_) => true,
    }
}
impl<H: DesignMetadataHost> DraftingGarmentSemanticUse<'_, H> {
    /// Classify original Pocket opening,Closure length/size variants without resolving members
    /// or reading values. Other garment fields retain their original non-selection obligations.
    /// This fixed syntax projection grants no row,source or fresh work acceptance.
    #[must_use]
    pub const fn selection_need(self) -> DraftingSemanticSelectionNeed {
        use DraftingGarmentSemanticField as F;
        let profile = match self.field() {
            F::PocketOpening(opening) => match opening {
                PocketOpening::Declaration(_) => false,
                PocketOpening::Profile(_) => true,
            },
            F::ClosureLength(length) => match length {
                ClosureLength::Explicit {
                    parameter: _,
                    value: _,
                }
                | ClosureLength::Formula(_) => false,
                ClosureLength::Profile(_) => true,
            },
            F::ClosureSize { role: _, source } => notion_needs_profile(source),
            F::PocketServed(_)
            | F::PocketPosition(_)
            | F::PocketOrientation(_)
            | F::PocketComponent {
                index: _,
                reference: _,
            }
            | F::ClosureButtonhole(_)
            | F::ClosureInstance {
                index: _,
                instance: _,
            }
            | F::ClosureUnsupportedFly(_) => false,
        };
        if profile {
            DraftingSemanticSelectionNeed::Required(Key::Profile(self.selection_location()))
        } else {
            DraftingSemanticSelectionNeed::NoDirectSelection
        }
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramSemanticUse<'_, '_, '_, H, S>
{
    /// Original direct Profile inputs/button and mandatory explicit rule/policy origin. Root
    /// logical/numeric inputs and Root output need no direct Profile selection at this field.
    /// No canonical/member/source lookup,value read,implicit origin or work grant occurs.
    #[must_use]
    pub const fn selection_need(self) -> DraftingSemanticSelectionNeed {
        use DraftingSemanticField as F;
        let profile = match self.field() {
            F::ButtonholeButton(source) => notion_needs_profile(source),
            F::LogicalInput { index: _, input } => match input.source {
                LogicalDerivationInputSource::Profile {
                    parameter: _,
                    kind: _,
                } => true,
                LogicalDerivationInputSource::Logical(_)
                | LogicalDerivationInputSource::Numeric(_) => false,
            },
            F::ButtonholePolicy(_) | F::LogicalRule(_) => {
                return DraftingSemanticSelectionNeed::Required(Key::Rule {
                    ordinal: self.ordinal(),
                });
            }
            F::ButtonholeInput { index: _, input: _ } | F::LogicalOutput(_) => false,
        };
        if profile {
            DraftingSemanticSelectionNeed::Required(Key::Profile(self.selection_location()))
        } else {
            DraftingSemanticSelectionNeed::NoDirectSelection
        }
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramNativeSemanticUse<'_, '_, '_, H, S>
{
    /// Original direct Notch/Walk Profile fields require their exact keys. Every native target
    /// defers its internal Profile need until authentic target association,even if a context row
    /// is supplied. This fixed projection queries no target,accepts no row and grants no work.
    #[must_use]
    pub const fn selection_need(self) -> DraftingSemanticSelectionNeed {
        let location = self.selection_location();
        match self.native().field() {
            DraftingNativeSemanticField::Target {
                role: _,
                reference: _,
            } => DraftingSemanticSelectionNeed::DeferredTargetProfile(location),
            DraftingNativeSemanticField::NotchProfile {
                role: _,
                reference: _,
            }
            | DraftingNativeSemanticField::WalkNotchProfile {
                pair_index: _,
                side: _,
                role: _,
                reference: _,
            } => DraftingSemanticSelectionNeed::Required(Key::Profile(location)),
        }
    }
}
