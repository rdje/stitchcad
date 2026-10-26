//! Exact authored print choices, independent of physical scale verification or a PDF renderer.
use core::fmt;
use sc_units::Length;

/// Explicit page orientation; the writer must not rotate or fit a piece silently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfOrientation {
    /// Portrait page.
    Portrait,
    /// Landscape page.
    Landscape,
}
/// Admitted tiled paper sizes; A0 uses the separate single-page layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfTilePaper {
    /// A3 paper.
    A3,
    /// A4 paper.
    A4,
}
/// Declared deterministic tile traversal, recorded with the artifact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfTileOrder {
    /// Complete each row before advancing to the next row.
    RowMajor,
    /// Complete each column before advancing to the next column.
    ColumnMajor,
}
/// Authored layout before positive-length validation by PdfPrintSettings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfLayout {
    /// One A0 page; a nonfitting piece must refuse rather than rescale or silently change layout.
    A0,
    /// A3/A4 tiling with explicit overlap and page order; registration marks remain required output.
    Tiled {
        /// Authored paper choice.
        paper: PdfTilePaper,
        /// Positive physical overlap, not an inferred fraction of the page.
        overlap: Length,
        /// Authored traversal order.
        order: PdfTileOrder,
    },
}
/// Actual nonpositive authored print length; no clamping or invented replacement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfPrintSettingsError {
    /// A printed scale square needs a positive side length.
    NonPositiveScaleSquare(Length),
    /// Adjacent tiled pages need positive overlap.
    NonPositiveOverlap(Length),
}
impl fmt::Display for PdfPrintSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonPositiveScaleSquare(size) => write!(
                f,
                "PDF scale square must be positive: {} um",
                size.as_micrometres()
            ),
            Self::NonPositiveOverlap(overlap) => write!(
                f,
                "PDF tile overlap must be positive: {} um",
                overlap.as_micrometres()
            ),
        }
    }
}
impl std::error::Error for PdfPrintSettingsError {}

/// Immutable validated print choices, at physical 1:1 scale; no fit-to-page scale is represented.
/// These authored output choices do not certify printer calibration, fit or physical measurement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PdfPrintSettings {
    layout: PdfLayout,
    orientation: PdfOrientation,
    scale_square: Length,
}
impl PdfPrintSettings {
    /// Preserve all explicit print choices and require positive square/overlap lengths.
    /// # Errors
    /// Returns the actual nonpositive scale square, then a nonpositive tiled overlap if present.
    /// Page fit and whether a square fits on the selected page are checked by the eventual writer.
    pub const fn new(
        layout: PdfLayout,
        orientation: PdfOrientation,
        scale_square: Length,
    ) -> Result<Self, PdfPrintSettingsError> {
        if scale_square.as_micrometres() <= 0 {
            return Err(PdfPrintSettingsError::NonPositiveScaleSquare(scale_square));
        }
        if let PdfLayout::Tiled { overlap, .. } = layout {
            if overlap.as_micrometres() <= 0 {
                return Err(PdfPrintSettingsError::NonPositiveOverlap(overlap));
            }
        }
        Ok(Self {
            layout,
            orientation,
            scale_square,
        })
    }
    /// Actual authored layout, including exact overlap/order for tiled output.
    #[must_use]
    pub const fn layout(self) -> PdfLayout {
        self.layout
    }
    /// Authored orientation, never an automatic fitting decision.
    #[must_use]
    pub const fn orientation(self) -> PdfOrientation {
        self.orientation
    }
    /// Exact declared side of the required printed scale square.
    #[must_use]
    pub const fn scale_square(self) -> Length {
        self.scale_square
    }
}
