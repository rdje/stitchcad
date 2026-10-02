//! Typed diagnostics for every way a quantity can fail.
//!
//! The numerical contract (`docs/book/src/spec/units-and-tolerances.md`) forbids silent failure: an
//! out-of-domain value, a division by zero, a non-finite float at a boundary and an arithmetic
//! overflow are each a *named* condition carrying the operation and the value that produced it. None
//! of them panics, clamps, wraps or saturates.

use core::fmt;

/// Every way a quantity or conversion in this crate can fail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitError {
    /// A value lies outside the declared domain (§1.1 of the spec), which is deliberately far
    /// tighter than the integer type. The diagnostic retains the producing operation and the
    /// declared boundary without inferring why the value is invalid.
    DomainExceeded {
        /// The operation that produced the out-of-domain value.
        operation: &'static str,
        /// What was being measured, for the diagnostic ("length", "area", "angle").
        kind: &'static str,
        /// The offending signed value, in the quantity's internal unit, widened so the report cannot
        /// itself overflow.
        value: i128,
        /// The declared limit for that quantity.
        limit: i128,
    },
    /// A division by zero was requested.
    DivisionByZero {
        /// The operation that requested it.
        operation: &'static str,
    },
    /// A non-finite float (NaN or an infinity) reached a boundary where a quantity is constructed.
    NonFinite {
        /// The operation that received it.
        operation: &'static str,
    },
    /// An intermediate or final value does not fit the internal representation.
    Overflow {
        /// The operation that overflowed.
        operation: &'static str,
    },
    /// A tolerance was constructed without stating the requirement it was derived from. The spec
    /// (§3) requires every tolerance value to carry its derivation: a value with no derivation is
    /// not adopted.
    EmptyDerivation,
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DomainExceeded { operation, kind, value, limit } => write!(
                f,
                "{operation}: {kind} {value} is outside the declared domain (limit {limit})"
            ),
            Self::DivisionByZero { operation } => {
                write!(f, "{operation} divided by zero")
            }
            Self::NonFinite { operation } => {
                write!(f, "{operation} received a non-finite value (NaN or infinity)")
            }
            Self::Overflow { operation } => {
                write!(f, "{operation} overflowed the internal representation")
            }
            Self::EmptyDerivation => write!(
                f,
                "a tolerance was constructed without a derivation; every tolerance value states the \
                 requirement it was derived from (spec §3)"
            ),
        }
    }
}

impl std::error::Error for UnitError {}
