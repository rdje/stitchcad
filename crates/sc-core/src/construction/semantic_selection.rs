//! Original relative semantic source-selection intent;no actual consumer or source acceptance.
use super::{
    DraftingClosureSizeRole, DraftingNativeSemanticTargetRole, DraftingNotchProfileRole,
    DraftingRuleSelection,
};
use crate::ontology::{SeamSideId, SelectedProfilePin};
use core::fmt;
mod admission;
mod needs;
mod resources;
pub use admission::{
    AdmittedDraftingSemanticSelections, DraftingSemanticSelectionAdmissionError,
    DraftingSemanticSelectionLimits, DraftingSemanticSelectionRefusal,
    DraftingSemanticSelectionShape,
};
pub use needs::DraftingSemanticSelectionNeed;
/// Requested original Pocket field;locations are data,not member/source proofs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingPocketSelectionField {
    /// Served copy/expected-Piece source context.
    Served,
    /// Original attachment-position context.
    Position,
    /// Original directed-orientation context.
    Orientation,
    /// Original logical opening source.
    Opening,
    /// Requested original component position.
    Component {
        /// Zero-based requested component index,not an issued identity.
        index: usize,
    },
}
/// Requested original Closure field;actual kind/member/need is checked later.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingClosureSelectionField {
    /// Original zipper-length source context.
    Length,
    /// Original logical hardware-size role.
    Size(DraftingClosureSizeRole),
    /// Original buttonhole-operation context.
    Buttonhole,
    /// Requested original physical instance position.
    Instance {
        /// Zero-based requested instance index,not its stable identity.
        index: usize,
    },
    /// Original unsupported Fly gap context;normal Closure construction already refuses it.
    UnsupportedFly,
}
/// Requested semantic operation field,without policy/rule/member/source existence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingSemanticSelectionField {
    /// Original canonical button source.
    ButtonholeButton,
    /// Original physical-policy context;its rule-origin selection remains an explicit rule row.
    ButtonholePolicy,
    /// Requested original named numeric policy input.
    ButtonholeInput {
        /// Zero-based requested input index.
        index: usize,
    },
    /// Original typed logical output context.
    LogicalOutput,
    /// Original logical-rule context;its origin selection remains an explicit rule row.
    LogicalRule,
    /// Requested original named logical/Profile/numeric input.
    LogicalInput {
        /// Zero-based requested input index.
        index: usize,
    },
}
/// Requested native semantic field or authentic target Profile context,without a lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingNativeSelectionField {
    /// Profile context for actual references reached through this original canonical target.
    Target(DraftingNativeSemanticTargetRole),
    /// Original direct Notch Profile field.
    NotchProfile(DraftingNotchProfileRole),
    /// Requested original Walk pair/A/B/Profile field.
    WalkNotchProfile {
        /// Zero-based requested pair index.
        pair_index: usize,
        /// Explicit original canonical side.
        side: SeamSideId,
        /// Requested original Profile field role.
        role: DraftingNotchProfileRole,
    },
}
/// Fixed typed relative coordinates for source-selection intent. Zero/missing/wrong-kind data is
/// retained;only a later actual-scope factory can join it to genuine owners/positions/fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingSemanticSelectionLocation {
    /// Requested original Pocket and field,without a fabricated mixed position.
    Pocket {
        /// Zero-based requested Pocket inventory index.
        pocket: usize,
        /// Explicit requested original field.
        field: DraftingPocketSelectionField,
    },
    /// Requested original Closure and field,without a fabricated mixed position.
    Closure {
        /// Zero-based requested Closure inventory index.
        closure: usize,
        /// Explicit requested original field.
        field: DraftingClosureSelectionField,
    },
    /// Requested semantic recipe ordinal and field;not a scalar/native ordinal or mixed position.
    Semantic {
        /// Requested one-based original semantic ordinal;zero remains invalid data.
        ordinal: usize,
        /// Explicit requested original field/input index.
        field: DraftingSemanticSelectionField,
    },
    /// Requested genuine mixed native position and original field/local roles.
    Native {
        /// Requested one-based mixed position;zero remains invalid data.
        position: usize,
        /// Explicit requested original field/pair/A/B/Profile role.
        field: DraftingNativeSelectionField,
    },
}
/// Structural selection identity,without pin/origin equality or actual source membership.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingSemanticSelectionKey {
    /// Requested Profile source/context location.
    Profile(DraftingSemanticSelectionLocation),
    /// Requested semantic operation's explicit rule/policy origin.
    Rule {
        /// Requested one-based semantic ordinal,independent of a mixed position.
        ordinal: usize,
    },
}
/// One original authored selection row. Every source choice is explicit;there is no default origin.
#[derive(Clone, Copy)]
pub enum DraftingSemanticSelectionRow<'a> {
    /// Original requested Profile source/context and exact selected pin loan.
    Profile {
        /// Fixed requested coordinates,without accepted location/need.
        location: DraftingSemanticSelectionLocation,
        /// Original explicit profile/schema/content pin.
        selected: &'a SelectedProfilePin,
    },
    /// Original requested rule/policy selection;identity/family comes from the actual operation later.
    Rule {
        /// Requested one-based semantic ordinal;zero remains data.
        ordinal: usize,
        /// Explicit Design or original Profile pin intent.
        origin: DraftingRuleSelection<'a>,
    },
}
impl DraftingSemanticSelectionRow<'_> {
    /// Fixed structural key. Different pins/origins do not create another key for the same target.
    /// This constant-time projection performs no lookup,matching,validation or source selection.
    #[must_use]
    pub const fn key(self) -> DraftingSemanticSelectionKey {
        match self {
            Self::Profile { location, .. } => DraftingSemanticSelectionKey::Profile(location),
            Self::Rule { ordinal, .. } => DraftingSemanticSelectionKey::Rule { ordinal },
        }
    }
}
impl fmt::Debug for DraftingSemanticSelectionRow<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Profile { location, .. } => f
                .debug_struct("Profile")
                .field("location", location)
                .finish_non_exhaustive(),
            Self::Rule { ordinal, origin } => f
                .debug_struct("Rule")
                .field("ordinal", ordinal)
                .field("origin", origin)
                .finish(),
        }
    }
}
/// Exact original ordered request rows,without validation,lookup,local/whole admission or source
/// proof. Construction moves the caller's existing Vec unchanged;invalid/duplicate/extra requests
/// remain inspectable. No source owner/body,selected value,domain or fallback is manufactured.
/// ```compile_fail,E0616
/// fn alter(inputs:&mut sc_core::construction::DraftingSemanticSelectionInputs<'_>) {inputs.rows.clear();}
/// ```
/// The original selected pin must outlive its request rows:
/// ```compile_fail
/// use sc_core::{construction::*,ontology::SelectedProfilePin};
/// fn escape<'a>(pin:&'a SelectedProfilePin)->DraftingSemanticSelectionInputs<'static> {
///     DraftingSemanticSelectionInputs::new(vec![DraftingSemanticSelectionRow::Rule{ordinal:1,origin:DraftingRuleSelection::Profile(pin)}])
/// }
/// ```
pub struct DraftingSemanticSelectionInputs<'a> {
    rows: Vec<DraftingSemanticSelectionRow<'a>>,
}
impl<'a> DraftingSemanticSelectionInputs<'a> {
    /// Own the actual supplied row vector in its original order,without clone,validation or query.
    /// ```
    /// use sc_core::construction::{DraftingSemanticSelectionInputs,DraftingSemanticSelectionRow};
    /// fn original(rows:Vec<DraftingSemanticSelectionRow<'_>>)->DraftingSemanticSelectionInputs<'_> {
    ///     DraftingSemanticSelectionInputs::new(rows)
    /// }
    /// ```
    #[must_use]
    pub const fn new(rows: Vec<DraftingSemanticSelectionRow<'a>>) -> Self {
        Self { rows }
    }
    /// Every original row,including invalid coordinates,duplicates and extra source intents.
    #[must_use]
    pub fn rows(&self) -> &[DraftingSemanticSelectionRow<'a>] {
        &self.rows
    }
}
impl fmt::Debug for DraftingSemanticSelectionInputs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingSemanticSelectionInputs")
            .field("row_count", &self.rows.len())
            .finish_non_exhaustive()
    }
}
