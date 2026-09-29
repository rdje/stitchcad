//! The five tolerance classes (spec §3). There is no global epsilon.
//!
//! Every comparison in the product names its class, and every class carries the requirement it was
//! derived from. The derivation is a **required, non-empty** field: a tolerance whose author cannot
//! say why the number is what it is cannot be constructed. That is the mechanical form of "a value
//! chosen to make a test pass is not a tolerance".

use crate::error::UnitError;
use crate::length::Length;

/// Which question a comparison is asking. Using the wrong class is a defect, not a style choice:
/// T1 on an approximated curve reports noise as failure, and T2 on values that must be identical by
/// construction hides real bugs (spec §3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToleranceClass {
    /// Values identical by construction: the same length computed twice, a mirrored piece against its
    /// original, save → load → save. A violation is an internal invariant failure — a bug — not a
    /// user-facing condition.
    Numerical,
    /// Deviation introduced by approximating one curve with another inside the model: arc → Bézier,
    /// a Bézier offset, tessellation for internal use.
    GeometricApproximation,
    /// Chordal deviation of a tessellation destined for a polyline-only receiver.
    GeometricChordal,
    /// The quantum of a target format. Not a failure: it is the resolution the artifact carries, and
    /// it is published with the artifact.
    FormatQuantization,
    /// Differences between our artifact and the same artifact after a receiver imported and
    /// re-exported it. Declared per receiver, recorded with the evidence.
    ImporterComparison,
    /// Deviation of a printed, plotted or cut artifact from its canonical geometry. The value comes
    /// from the factory, never from this project.
    PhysicalAcceptance,
}

impl ToleranceClass {
    /// A short human-readable name, for diagnostics that must name the class they used.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Numerical => "T1 numerical",
            Self::GeometricApproximation => "T2 geometric approximation",
            Self::GeometricChordal => "T2 geometric approximation (chordal)",
            Self::FormatQuantization => "T3 format quantization",
            Self::ImporterComparison => "T4 importer comparison",
            Self::PhysicalAcceptance => "T5 physical acceptance",
        }
    }
}

/// A tolerance: a class, a limit, and the requirement the limit was derived from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tolerance {
    /// The class of question this tolerance answers.
    pub class: ToleranceClass,
    /// The inclusive limit: a deviation equal to the limit passes, greater fails.
    pub limit: Length,
    derivation: &'static str,
}

impl Tolerance {
    /// The default T1 limit: one quantum of the internal representation (spec §3).
    pub const NUMERICAL_UM: i64 = 1;
    /// The default internal geometric-approximation limit (spec §3).
    pub const GEOMETRIC_INTERNAL_UM: i64 = 10;
    /// The default chordal limit for polyline-only receivers (spec §3, §6).
    pub const GEOMETRIC_CHORDAL_UM: i64 = 100;

    /// Constructs a tolerance, requiring a non-empty derivation.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::EmptyDerivation`] when `derivation` is empty or whitespace, because a
    /// tolerance nobody can justify is a tolerance nobody can review.
    pub fn new(
        class: ToleranceClass,
        limit: Length,
        derivation: &'static str,
    ) -> Result<Self, UnitError> {
        if derivation.trim().is_empty() {
            return Err(UnitError::EmptyDerivation);
        }
        Ok(Self {
            class,
            limit,
            derivation,
        })
    }

    /// The requirement this limit was derived from.
    #[must_use]
    pub const fn derivation(&self) -> &'static str {
        self.derivation
    }

    /// The T1 numerical tolerance: one quantum, derived from the representation itself.
    ///
    /// # Errors
    ///
    /// Only if the constant is outside the domain, which it is not; the `Result` keeps that a fact
    /// checked by a test rather than an assumption.
    pub fn numerical() -> Result<Self, UnitError> {
        Self::new(
            ToleranceClass::Numerical,
            Length::from_micrometres(Self::NUMERICAL_UM)?,
            "one quantum of the internal fixed-point representation (spec §3, T1)",
        )
    }

    /// The T2 internal geometric-approximation tolerance.
    ///
    /// # Errors
    ///
    /// As [`Tolerance::numerical`].
    pub fn geometric_internal() -> Result<Self, UnitError> {
        Self::new(
            ToleranceClass::GeometricApproximation,
            Length::from_micrometres(Self::GEOMETRIC_INTERNAL_UM)?,
            "tightest internal consumer of approximated curve geometry (spec §3, T2)",
        )
    }

    /// The T2 chordal tolerance for polyline-only receivers.
    ///
    /// # Errors
    ///
    /// As [`Tolerance::numerical`].
    pub fn geometric_chordal() -> Result<Self, UnitError> {
        Self::new(
            ToleranceClass::GeometricChordal,
            Length::from_micrometres(Self::GEOMETRIC_CHORDAL_UM)?,
            "≤ 0.1 mm chordal deviation for legacy polyline-only importers (spec §3, §6)",
        )
    }

    /// Whether the deviation between two lengths is inside this tolerance, inclusively.
    ///
    /// The deviation is computed exactly in integers, so the comparison itself introduces no error —
    /// which is the point of the internal representation.
    #[must_use]
    pub fn accepts(&self, a: Length, b: Length) -> bool {
        let deviation = (a.as_micrometres() - b.as_micrometres()).abs();
        deviation <= self.limit.as_micrometres()
    }

    /// Whether a single measured deviation is inside this tolerance, inclusively.
    ///
    /// Used where the deviation is already known — an offset engine's achieved error, a printed
    /// measurement against its canonical value.
    #[must_use]
    pub fn accepts_deviation(&self, deviation: Length) -> bool {
        deviation.abs().as_micrometres() <= self.limit.as_micrometres()
    }
}
