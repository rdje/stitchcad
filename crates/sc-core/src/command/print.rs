//! Concrete vector-print request, without rendering, scaling or physical verification.
use super::{EvaluationProfile, PdfPrintSettings};
use crate::ontology::EntityId;

/// One immutable Instance, explicit validated print choices and logical/factory input selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportPdf {
    instance: EntityId,
    settings: PdfPrintSettings,
    profile: EvaluationProfile,
}
impl ExportPdf {
    /// Retain exact authored selections; execution validates provenance/unknowns and page feasibility.
    #[must_use]
    pub const fn new(
        instance: EntityId,
        settings: PdfPrintSettings,
        profile: EvaluationProfile,
    ) -> Self {
        Self {
            instance,
            settings,
            profile,
        }
    }
    /// Explicit immutable source Instance, never a latest-snapshot lookup.
    #[must_use]
    pub const fn instance(&self) -> EntityId {
        self.instance
    }
    /// Original 1:1 print choices; no observed printer calibration is asserted.
    #[must_use]
    pub const fn settings(&self) -> PdfPrintSettings {
        self.settings
    }
    /// Explicit logical/factory selection; logical mode supplies no missing physical defaults.
    #[must_use]
    pub const fn profile(&self) -> &EvaluationProfile {
        &self.profile
    }
    /// Registered PDF identity, whose contract includes A0 and tiled A3/A4 layouts.
    #[must_use]
    pub const fn target_token(&self) -> &'static str {
        "pdf-a0"
    }
}
