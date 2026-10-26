//! Structural size-value frame; canonical membership derivation belongs to its host adapter.
use super::super::{FormulaKind, FormulaNumericValue, FormulaSizeName};
use crate::ontology::EntityId;
use core::fmt;
use sc_units::Count;

/// Supplied pinned size source, without claiming current-registry or membership validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FormulaSizeContextSource {
    /// Stable owning set identity.
    pub set: EntityId,
    /// Exact authored revision, including zero.
    pub revision: Count,
    /// Selected stable member identity, never a label or ordinal-derived identity.
    pub member: EntityId,
    /// Explicit base member identity.
    pub base: EntityId,
}

/// Structural inconsistency in the caller-supplied size-value frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaSizeContextError {
    /// Selected/base identity aliases its owning set.
    IdentityAlias(FormulaSizeContextSource),
    /// A nonempty one-based ordinal/count range was not supplied.
    OrdinalRange {
        /// Actual supplied source.
        source: FormulaSizeContextSource,
        /// Actual supplied one-based index.
        index: u128,
        /// Actual supplied member count.
        count: u128,
    },
    /// In a singleton, the selected member must be its explicit base.
    SingletonBase(FormulaSizeContextSource),
}

impl fmt::Display for FormulaSizeContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdentityAlias(source) => {
                write!(f, "size set {} aliases selected/base identity", source.set)
            }
            Self::OrdinalRange {
                source,
                index,
                count,
            } => write!(
                f,
                "size set {} member {} index {index} is outside 1..={count}",
                source.set, source.member
            ),
            Self::SingletonBase(source) => write!(
                f,
                "size set {} singleton {} differs from base {}",
                source.set, source.member, source.base
            ),
        }
    }
}
impl std::error::Error for FormulaSizeContextError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SizeFrame {
    source: FormulaSizeContextSource,
    index: u128,
    count: u128,
}

/// Optional immutable size-value frame. Validates supplied structural facts, not authored membership.
/// The host membership adapter must derive actual order, count, pin and base from its canonical record.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FormulaSizeContext(Option<SizeFrame>);

impl FormulaSizeContext {
    /// No size context; reads refuse rather than returning zero or false.
    #[must_use]
    pub const fn empty() -> Self {
        Self(None)
    }

    /// Validates source aliases, the one-based range, then singleton/base consistency in that order.
    /// # Errors
    /// Retains the supplied source and actual bad range/identity. This does not establish actual
    /// membership, revision currentness, label order, axes, charts or instantiation-path readiness.
    pub fn new(
        source: FormulaSizeContextSource,
        index: u128,
        count: u128,
    ) -> Result<Self, FormulaSizeContextError> {
        if source.set == source.member || source.set == source.base {
            return Err(FormulaSizeContextError::IdentityAlias(source));
        }
        if count == 0 || index == 0 || index > count {
            return Err(FormulaSizeContextError::OrdinalRange {
                source,
                index,
                count,
            });
        }
        if count == 1 && source.member != source.base {
            return Err(FormulaSizeContextError::SingletonBase(source));
        }
        Ok(Self(Some(SizeFrame {
            source,
            index,
            count,
        })))
    }

    /// Actual supplied pin/identities, or None for an absent context.
    #[must_use]
    pub fn source(self) -> Option<FormulaSizeContextSource> {
        self.0.map(|frame| frame.source)
    }

    /// Exact Count/Boolean reserved read with source retained. No i64 binding narrowing occurs.
    /// # Errors
    /// Returns the actual missing reserved name as formula_unbound_name. No namespace search,
    /// statement ordinal, member identity, default zero or false is invented.
    pub fn read(self, name: FormulaSizeName) -> Result<FormulaSizeValue, FormulaSizeReadError> {
        let frame = self.0.ok_or(FormulaSizeReadError { name })?;
        let value = match name {
            FormulaSizeName::Index => SizeValue::Count(frame.index),
            FormulaSizeName::Count => SizeValue::Count(frame.count),
            FormulaSizeName::IsBaseSize => {
                SizeValue::Boolean(frame.source.member == frame.source.base)
            }
        };
        Ok(FormulaSizeValue {
            name,
            source: frame.source,
            value,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SizeValue {
    Count(u128),
    Boolean(bool),
}

/// Present exact size value beside its requested role and supplied pinned source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaSizeValue {
    name: FormulaSizeName,
    source: FormulaSizeContextSource,
    value: SizeValue,
}

impl FormulaSizeValue {
    /// Actual size role; no label value is produced.
    #[must_use]
    pub const fn name(self) -> FormulaSizeName {
        self.name
    }

    /// Retained supplied pin/selected/base identities.
    #[must_use]
    pub const fn source(self) -> FormulaSizeContextSource {
        self.source
    }

    /// Count or Boolean, without magnitude-based kind inference.
    #[must_use]
    pub const fn kind(self) -> FormulaKind {
        match self.value {
            SizeValue::Count(_) => FormulaKind::Count,
            SizeValue::Boolean(_) => FormulaKind::Boolean,
        }
    }

    /// Exact wide Count, or None for Boolean. Binding storage remains a later operation.
    #[must_use]
    pub const fn numeric_value(self) -> Option<FormulaNumericValue> {
        match self.value {
            SizeValue::Count(value) => Some(FormulaNumericValue::from_unsigned_count(value)),
            SizeValue::Boolean(_) => None,
        }
    }

    /// Actual Boolean, including present false; None for Count.
    #[must_use]
    pub const fn boolean(self) -> Option<bool> {
        match self.value {
            SizeValue::Boolean(value) => Some(value),
            SizeValue::Count(_) => None,
        }
    }
}

/// Size role was read without a size-value context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaSizeReadError {
    /// Actual reserved size role, with no invented source/currentness/search scope.
    pub name: FormulaSizeName,
}

impl FormulaSizeReadError {
    /// The missing size value has the language's unbound-name family.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        "formula_unbound_name"
    }
}
impl fmt::Display for FormulaSizeReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "formula_unbound_name: {} needs a size context",
            self.name.token()
        )
    }
}
impl std::error::Error for FormulaSizeReadError {}
