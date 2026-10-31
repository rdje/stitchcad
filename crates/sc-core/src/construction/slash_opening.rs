//! Explicit numerical opening intent; cut/pivot/moving-region and geometry remain separate.
use crate::recipe::{FormulaExpressionDocument, FormulaKind};
mod normalized;
pub use normalized::{
    NormalizedSlashSpreadOpening, SlashSpreadNormalizationError, SlashSpreadNormalizationLimits,
    SlashSpreadNormalizationRefusal,
};
mod check;
pub use check::{CheckedSlashSpreadOpening, SlashSpreadCheckError, SlashSpreadCheckRefusal};

/// Explicit authored numerical opening mode, never inferred from a formula's resulting kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SlashSpreadOpeningMode {
    /// Signed rotation amount; actual pivot/region and supported geometric domain stay separate.
    Rotation,
    /// Signed separation amount; actual movement direction/region stay separate.
    Separation,
}
impl SlashSpreadOpeningMode {
    /// Required root kind selected by the actual explicit mode.
    #[must_use]
    pub const fn required_kind(self) -> FormulaKind {
        match self {
            Self::Rotation => FormulaKind::Angle,
            Self::Separation => FormulaKind::Length,
        }
    }
}

/// Numerical opening component of a slash/spread declaration, not a complete operation.
/// Retains one actual owned document and explicit mode. No cut,pivot,moving-region,translation
/// direction,output manifest or geometry is fabricated by this component or its static proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlashSpreadOpening {
    mode: SlashSpreadOpeningMode,
    document: FormulaExpressionDocument,
}
impl SlashSpreadOpening {
    /// Retain a deliberately authored rotation amount; no kind/value or angle normalization yet.
    #[must_use]
    pub const fn rotation(angle: FormulaExpressionDocument) -> Self {
        Self {
            mode: SlashSpreadOpeningMode::Rotation,
            document: angle,
        }
    }
    /// Retain a deliberately authored separation amount; no direction,kind or value inferred.
    #[must_use]
    pub const fn separation(distance: FormulaExpressionDocument) -> Self {
        Self {
            mode: SlashSpreadOpeningMode::Separation,
            document: distance,
        }
    }
    /// Actual explicit mode, retained even if its document's eventual root kind is wrong.
    #[must_use]
    pub const fn mode(&self) -> SlashSpreadOpeningMode {
        self.mode
    }
    /// Actual original amount document, including authored text and canonical expression identity.
    #[must_use]
    pub const fn document(&self) -> &FormulaExpressionDocument {
        &self.document
    }
}
