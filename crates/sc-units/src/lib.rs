//! `sc-units` — StitchCAD's numerical contract, as code.
//!
//! The normative specification is `docs/book/src/spec/units-and-tolerances.md`; the coupled decisions
//! and their rationale are in `docs/decisions/decision_numerical-contract-fixed-point.md`. This crate
//! is the implementation of both, and it is deliberately **dependency-free**: it is the foundation of
//! the `wasm-viewer` runtime profile and of byte-deterministic golden files, so a dependency here is
//! a dependency everywhere.
//!
//! # The contract in five lines
//!
//! 1. Lengths are `i64` **micrometres** ([`Length`]); angles are `i64` **microdegrees** ([`Angle`]);
//!    areas, ratios and counts are their own types and cannot be confused with one another.
//! 2. The declared domain is far tighter than the type ([`length::MAX_LENGTH_UM`]), so intermediate
//!    arithmetic cannot overflow and an out-of-domain value is a diagnostic, never a clamp.
//! 3. Rounding is **half away from zero** and every conversion is a **single multiply then divide**
//!    on an exact integer ratio ([`round`], [`Unit`]).
//! 4. There is **no global epsilon**: five named tolerance classes, each carrying the requirement it
//!    was derived from ([`Tolerance`], [`ToleranceClass`]).
//! 5. Nothing panics. Every failure is a [`UnitError`] naming the operation and the value.
//!
//! # Example
//!
//! ```
//! use sc_units::{Angle, Length, Ratio, Tolerance, Unit};
//!
//! // A 1 cm seam allowance and a 3 cm hem, as the reference skirt declares them.
//! let side_allowance = Length::from_rational(1, 1, Unit::Centimetre)?;
//! let hem_allowance = Length::from_rational(3, 1, Unit::Centimetre)?;
//! assert_eq!(side_allowance.as_micrometres(), 10_000);
//! assert_eq!(hem_allowance.as_micrometres(), 30_000);
//!
//! // Three quarters of an inch, exactly: 19 050 µm, with one rounding step.
//! let fraction = Length::from_rational(3, 4, Unit::Inch)?;
//! assert_eq!(fraction.as_micrometres(), 19_050);
//!
//! // A 2 % fabric shrinkage is the *multiplier* 1.02 — not the percentage 0.02. Applying it is one
//! // explicit, rounded operation, never a float folded into a coordinate.
//! let shrinkage = Ratio::from_rational(102, 100)?;
//! assert_eq!(shrinkage.scale(side_allowance)?.as_micrometres(), 10_200);
//! // And the percentage form is a different number, kept apart by a different constructor:
//! assert_eq!(Ratio::from_percent(2, 1)?.as_parts_per_million(), 20_000);
//!
//! // Bias grain is exactly 45°: microdegrees store rational degrees without approximation.
//! let bias = Angle::from_degrees_rational(45, 1)?;
//! assert_eq!(bias.as_microdegrees(), 45_000_000);
//!
//! // Two values identical by construction agree within T1 — one quantum. Note that arithmetic which
//! // can leave the domain is `checked_*`, not an operator: this crate never panics.
//! let doubled = side_allowance.checked_add(side_allowance)?;
//! let t1 = Tolerance::numerical()?;
//! assert!(side_allowance.eq_within(hem_allowance - doubled, t1));
//! # Ok::<(), sc_units::UnitError>(())
//! ```
//!
//! [`length::MAX_LENGTH_UM`]: crate::length::MAX_LENGTH_UM

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod angle;
pub mod error;
pub mod length;
pub mod ratio;
pub mod round;
pub mod tolerance;
pub mod unit;

pub use angle::{Angle, MICRODEGREES_PER_DEGREE, MICRODEGREES_PER_TURN};
pub use error::UnitError;
pub use length::{Area, Length, MAX_AREA_UM2, MAX_LENGTH_UM};
pub use ratio::{Count, Ratio, PPM_UNITY};
pub use tolerance::{Tolerance, ToleranceClass};
pub use unit::Unit;
