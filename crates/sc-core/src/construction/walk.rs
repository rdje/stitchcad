//! Deliberate balance-notch requests,distinct from a seam-differential read query.
use crate::ontology::{EdgeAnchor, EntityId, NotchProfileBindings, SeamSideId};
use core::fmt;
mod roles;
pub use roles::{BoundWalkNotchRoles, WalkNotchRoleBindingError, WalkNotchRoleBindingRefusal};
mod copies;
pub use copies::{
    BoundWalkCopyInventory, WalkCopyInventoryError, WalkCopyInventoryLimits,
    WalkCopyInventoryRefusal,
};
mod binding;
pub use binding::{
    BoundWalkBalancePair, BoundWalkSeamEdit, WalkEditBindingError, WalkEditBindingRefusal,
};

/// One requested balance notch,without a client-supplied new identity or resolved profile values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkBalanceNotch {
    anchor: EdgeAnchor,
    representation: NotchProfileBindings,
}
impl WalkBalanceNotch {
    /// Retain the original source-frame anchor and all sample/production/profile bindings.
    #[must_use]
    pub const fn new(anchor: EdgeAnchor, representation: NotchProfileBindings) -> Self {
        Self {
            anchor,
            representation,
        }
    }
    /// Actual original anchor field;no counterpart position is inferred.
    #[must_use]
    pub const fn anchor(&self) -> &EdgeAnchor {
        &self.anchor
    }
    /// Actual symbolic bindings;no style,dimension or encoding is supplied by this record.
    #[must_use]
    pub const fn representation(&self) -> &NotchProfileBindings {
        &self.representation
    }
}
/// Both explicitly requested sides of one balance pair;equal parameters are not correspondence proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkBalancePair {
    a: WalkBalanceNotch,
    b: WalkBalanceNotch,
}
impl WalkBalancePair {
    /// Both original requests are required;neither side nor profile context receives a default.
    #[must_use]
    pub const fn new(a: WalkBalanceNotch, b: WalkBalanceNotch) -> Self {
        Self { a, b }
    }
    /// Actual field for the named canonical seam side,not an adjacency or traversal inference.
    #[must_use]
    pub const fn mark(&self, side: SeamSideId) -> &WalkBalanceNotch {
        match side {
            SeamSideId::A => &self.a,
            SeamSideId::B => &self.b,
        }
    }
}
/// Explicit complete balance-pair admission allowance;earlier owned inputs/graph/workspace are separate.
/// ```compile_fail
/// let limits:sc_core::construction::WalkEditLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkEditLimits {
    pairs: usize,
}
impl WalkEditLimits {
    /// Select the maximum complete authored pair population,not an accepted prefix.
    #[must_use]
    pub const fn new(pairs: usize) -> Self {
        Self { pairs }
    }
    /// Explicit complete pair allowance;at most twice this population can be requested.
    #[must_use]
    pub const fn pairs(self) -> usize {
        self.pairs
    }
}
/// Complete quota or original local dependency refusal;no program statement ordinal is invented.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkEditAdmissionRefusal {
    /// The complete two-requests-per-pair population cannot be represented.
    PopulationOverflow,
    /// Complete actual pair population exceeds its explicit allowance.
    Pairs {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual population.
        required: usize,
    },
    /// A deliberate mark edit requires at least one pair;report-only walking has its read query.
    Empty,
    /// Required existing span has this operation's own identity.
    SelfSpan,
    /// First actual requested anchor depends on this same creating operation.
    SelfAnchor {
        /// Actual zero-based authored pair index,not a recipe statement position.
        pair: usize,
        /// Actual A/B field role within that original pair.
        side: SeamSideId,
        /// Original source-frame anchor,including persistent tag and exact parameter.
        anchor: EdgeAnchor,
    },
}
/// Actual original input,selected policy and complete refusal;input inspection survives failure.
pub struct WalkEditAdmissionError<'a> {
    edit: &'a WalkSeamEdit,
    limits: WalkEditLimits,
    refusal: WalkEditAdmissionRefusal,
}
impl<'a> WalkEditAdmissionError<'a> {
    /// Original operation,span and every authored pair,without a cloned replacement.
    #[must_use]
    pub const fn edit(&self) -> &'a WalkSeamEdit {
        self.edit
    }
    /// Original explicitly selected admission allowance.
    #[must_use]
    pub const fn limits(&self) -> WalkEditLimits {
        self.limits
    }
    /// Complete population or first actual local source cause.
    #[must_use]
    pub const fn refusal(&self) -> WalkEditAdmissionRefusal {
        self.refusal
    }
}
impl fmt::Debug for WalkEditAdmissionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalkEditAdmissionError")
            .field("operation", &self.edit.operation)
            .field("span", &self.edit.span)
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for WalkEditAdmissionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.refusal {
            WalkEditAdmissionRefusal::PopulationOverflow => {
                f.write_str("walk edit request population overflow")
            }
            WalkEditAdmissionRefusal::Pairs { allowed, required } => {
                write!(f, "walk edit needs {required} pairs;allowance is {allowed}")
            }
            WalkEditAdmissionRefusal::Empty => {
                f.write_str("walk editing requires at least one balance pair")
            }
            WalkEditAdmissionRefusal::SelfSpan => {
                f.write_str("walk edit span depends on its own operation identity")
            }
            WalkEditAdmissionRefusal::SelfAnchor { pair, side, .. } => write!(
                f,
                "walk edit pair {pair} {side:?} anchor depends on its own operation"
            ),
        }
    }
}
impl std::error::Error for WalkEditAdmissionError<'_> {}

/// Owned unadmitted deliberate-edit input,retaining original pair order,repeats and profile contexts.
/// This constructor grants no quota,source,correspondence,currentness,notch identity or geometry proof.
/// Only the admitted stage can bind a graph. No query payload or existing data is mutated here.
/// ```compile_fail
/// fn bypass(edit:&sc_core::construction::WalkSeamEdit,graph:&sc_core::ontology::SewingGraph) {
///     edit.bind_source(graph);
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct WalkSeamEdit {
    operation: EntityId,
    span: EntityId,
    pairs: Vec<WalkBalancePair>,
}
impl fmt::Debug for WalkSeamEdit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalkSeamEdit")
            .field("operation", &self.operation)
            .field("span", &self.span)
            .field("pairs", &self.pairs.len())
            .finish_non_exhaustive()
    }
}
impl WalkSeamEdit {
    /// Own exact authored inputs without evaluating or accepting them. Earlier Vec allocation and
    /// source/profile authoring are separate from the subsequent explicit admission allowance.
    #[must_use]
    pub const fn new(operation: EntityId, span: EntityId, pairs: Vec<WalkBalancePair>) -> Self {
        Self {
            operation,
            span,
            pairs,
        }
    }
    /// Original creating operation identity,without issuance or authority.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Actual required canonical correspondence identity,not a Piece or physical-copy id.
    #[must_use]
    pub const fn span_id(&self) -> EntityId {
        self.span
    }
    /// Every original pair in authored order;no sorting,deduplication or counterpart inference.
    #[must_use]
    pub fn pairs(&self) -> &[WalkBalancePair] {
        &self.pairs
    }
    /// Admit complete N pairs and checked2N requests before inspecting local source dependencies.
    /// These are requested occurrences,not an assertion of2N new semantic notch identities.
    /// Whole graph lookup/current validation,correspondence,profile/geometry/execution resources
    /// and earlier owned input allocations are outside this pair allowance.
    /// # Errors
    /// Complete population/quota,then empty edit,then span and pair/A/B local self-supply.
    /// The error borrows every original input;no admitted or bound prefix escapes.
    pub fn admit(
        &self,
        limits: WalkEditLimits,
    ) -> Result<AdmittedWalkSeamEdit<'_>, WalkEditAdmissionError<'_>> {
        let refuse = |refusal| WalkEditAdmissionError {
            edit: self,
            limits,
            refusal,
        };
        let pairs = self.pairs.len();
        let requests = pairs
            .checked_mul(2)
            .ok_or_else(|| refuse(WalkEditAdmissionRefusal::PopulationOverflow))?;
        if pairs > limits.pairs {
            return Err(refuse(WalkEditAdmissionRefusal::Pairs {
                allowed: limits.pairs,
                required: pairs,
            }));
        }
        if pairs == 0 {
            return Err(refuse(WalkEditAdmissionRefusal::Empty));
        }
        if self.span == self.operation {
            return Err(refuse(WalkEditAdmissionRefusal::SelfSpan));
        }
        for (pair_index, pair) in self.pairs.iter().enumerate() {
            for side in [SeamSideId::A, SeamSideId::B] {
                let anchor = *pair.mark(side).anchor();
                if anchor.edge.creator() == self.operation {
                    return Err(refuse(WalkEditAdmissionRefusal::SelfAnchor {
                        pair: pair_index,
                        side,
                        anchor,
                    }));
                }
            }
        }
        Ok(AdmittedWalkSeamEdit {
            edit: self,
            limits,
            requests,
        })
    }
}
/// Complete admitted original edit;actual source binding follows through this stage only.
/// ```compile_fail
/// fn escape(edit:&sc_core::construction::WalkSeamEdit)->sc_core::construction::AdmittedWalkSeamEdit<'static> {
///     edit.admit(sc_core::construction::WalkEditLimits::new(1)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct AdmittedWalkSeamEdit<'a> {
    edit: &'a WalkSeamEdit,
    limits: WalkEditLimits,
    requests: usize,
}
impl<'a> AdmittedWalkSeamEdit<'a> {
    /// Actual original owned operation and all request fields.
    #[must_use]
    pub const fn edit(&self) -> &'a WalkSeamEdit {
        self.edit
    }
    /// Original selected complete pair allowance.
    #[must_use]
    pub const fn limits(&self) -> WalkEditLimits {
        self.limits
    }
    /// Complete actual requested occurrences;not generated outputs or fresh identity count.
    #[must_use]
    pub const fn notch_request_count(&self) -> usize {
        self.requests
    }
}
