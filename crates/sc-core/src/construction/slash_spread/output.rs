//! Explicit authored boundary/fullness choices;actual contours and effects remain unproved.
use super::SlashSpreadConstruction;

/// Required treatment request for the changed cut banks,without a default connector or loop choice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadBoundaryPolicy {
    /// Request straight connectors between corresponding opened cut mouths. Actual output roles,
    /// endpoints/incidence and a supported unique closed contour must be proved by G2/G3.
    StraightBridges,
    /// Request trimming of overlapping banks. Actual retained ranges,intersections and unique
    /// boundary selection must be explicit in the result;this name does not choose a loop.
    TrimOverlaps,
}
/// Intended fullness effect,not a measured length/area,intake declaration or conservation proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadFullnessIntent {
    /// Intend added fullness;the actual attributable effect must be derived and verified.
    Added,
    /// Intend removed fullness;this neither closes a Dart nor copies its canonical intake.
    Removed,
    /// Intend no fullness change;zero amount or full turns alone do not prove this effect.
    Unchanged,
}
/// Owned actual slash inputs plus required authored boundary/fullness choices.
/// This is output intent only,not an accepted manifest,executed transform or authority certificate.
/// ```compile_fail
/// let output:sc_core::construction::SlashSpreadOutputIntent=Default::default();
/// ```
/// ```compile_fail
/// fn infer(input:sc_core::construction::SlashSpreadConstruction)->sc_core::construction::SlashSpreadOutputIntent {
///     input.into()
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlashSpreadOutputIntent {
    input: SlashSpreadConstruction,
    boundary: SlashSpreadBoundaryPolicy,
    fullness: SlashSpreadFullnessIntent,
}
impl SlashSpreadOutputIntent {
    /// Move the original input intact and require both explicit choices. Every combination remains
    /// raw intent;actual supported mode/domain/region/closure/effect validation stays G2/G3.
    /// No new allocation,evaluation,identity issuance,retirement or metadata mutation occurs.
    #[must_use]
    pub const fn new(
        input: SlashSpreadConstruction,
        boundary: SlashSpreadBoundaryPolicy,
        fullness: SlashSpreadFullnessIntent,
    ) -> Self {
        Self {
            input,
            boundary,
            fullness,
        }
    }
    /// Actual owned original spatial inputs,including the embedded unevaluated amount document.
    /// ```compile_fail
    /// fn escape(output:sc_core::construction::SlashSpreadOutputIntent)->sc_core::construction::SlashSpreadDependency<'static> {
    ///     output.input().dependencies().next().unwrap()
    /// }
    /// ```
    #[must_use]
    pub const fn input(&self) -> &SlashSpreadConstruction {
        &self.input
    }
    /// Actual authored boundary-treatment field,without generated connector geometry.
    #[must_use]
    pub const fn boundary_policy(&self) -> &SlashSpreadBoundaryPolicy {
        &self.boundary
    }
    /// Actual intended effect field,without a second canonical intake/solved amount.
    #[must_use]
    pub const fn fullness(&self) -> &SlashSpreadFullnessIntent {
        &self.fullness
    }
}
