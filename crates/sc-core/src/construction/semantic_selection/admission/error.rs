//! Exact raw selection owner/local policy/fresh metadata and first local refusal;no prefix proof.
use super::{
    DraftingSemanticSelectionInputs, DraftingSemanticSelectionKey, DraftingSemanticSelectionLimits,
    DraftingSemanticSelectionShape,
};
use crate::design::{DesignResourceDemand, DesignResourceLimitError};
use core::fmt;
/// Original first complete local content/work or duplicate structural-key refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingSemanticSelectionRefusal {
    /// Original first cumulative record field refusal.
    Resources(DesignResourceLimitError),
    /// Complete N² conservative key-comparison bound cannot fit usize.
    PopulationOverflow,
    /// Complete conservative bound exceeds its explicitly selected allowance.
    KeyComparisons {
        /// Exact selected maximum.
        allowed: usize,
        /// Checked conservative complete requirement.
        required: usize,
    },
    /// First repeated requested structural key,independent of source-choice equality.
    DuplicateKey {
        /// Zero-based first original row position.
        first: usize,
        /// Zero-based first repeated original row position.
        duplicate: usize,
        /// Complete original requested structural key.
        key: DraftingSemanticSelectionKey,
    },
}
/// Boxed original raw owner,policy,fresh content/shape and first typed refusal. No accepted prefix.
pub struct DraftingSemanticSelectionAdmissionError<'r, 'a> {
    details: Box<Failure<'r, 'a>>,
}
struct Failure<'r, 'a> {
    inputs: &'r DraftingSemanticSelectionInputs<'a>,
    limits: DraftingSemanticSelectionLimits,
    demand: Option<DesignResourceDemand>,
    shape: Option<DraftingSemanticSelectionShape>,
    refusal: DraftingSemanticSelectionRefusal,
}
impl<'r, 'a> DraftingSemanticSelectionAdmissionError<'r, 'a> {
    pub(super) fn new(
        inputs: &'r DraftingSemanticSelectionInputs<'a>,
        limits: DraftingSemanticSelectionLimits,
        demand: Option<DesignResourceDemand>,
        shape: Option<DraftingSemanticSelectionShape>,
        refusal: DraftingSemanticSelectionRefusal,
    ) -> Self {
        Self {
            details: Box::new(Failure {
                inputs,
                limits,
                demand,
                shape,
                refusal,
            }),
        }
    }
    /// Exact actual raw owner,including every invalid/duplicate row and original pin/origin loan.
    #[must_use]
    pub const fn inputs(&self) -> &'r DraftingSemanticSelectionInputs<'a> {
        self.details.inputs
    }
    /// Exact selected explicit local content/comparison policy.
    #[must_use]
    pub const fn limits(&self) -> DraftingSemanticSelectionLimits {
        self.details.limits
    }
    /// Fresh complete local content demand when that stage succeeded.
    #[must_use]
    pub const fn demand(&self) -> Option<DesignResourceDemand> {
        self.details.demand
    }
    /// Checked conservative local shape when its arithmetic succeeded,even on a short work quota.
    #[must_use]
    pub const fn shape(&self) -> Option<DraftingSemanticSelectionShape> {
        self.details.shape
    }
    /// Original first local cause. Actual coordinate/source coverage is outside this admission.
    #[must_use]
    pub const fn refusal(&self) -> DraftingSemanticSelectionRefusal {
        self.details.refusal
    }
}
impl fmt::Debug for DraftingSemanticSelectionAdmissionError<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingSemanticSelectionAdmissionError")
            .field("refusal", &self.refusal())
            .finish_non_exhaustive()
    }
}
impl fmt::Display for DraftingSemanticSelectionAdmissionError<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "semantic selection local admission: {:?}",
            self.refusal()
        )
    }
}
impl std::error::Error for DraftingSemanticSelectionAdmissionError<'_, '_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.details.refusal {
            DraftingSemanticSelectionRefusal::Resources(cause) => Some(cause),
            _ => None,
        }
    }
}
