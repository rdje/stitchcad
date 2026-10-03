//! Closed operator kind signatures; no operands are read or evaluated by this metadata layer.
use super::{FormulaBinaryOperator as B, FormulaKind as K};

/// Unary normalized syntax operators, with the exact canonical node tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaUnaryOperator {
    /// Negation of length, angle, area or ratio; count cannot be negated.
    Negate,
    /// Square of length, ratio or count; the exponent is fixed syntax, not another operand.
    Square,
}
impl FormulaUnaryOperator {
    /// Canonical operator token, independent of display presentation or literal value.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Negate => "-",
            Self::Square => "^2",
        }
    }

    /// Normative result kind, or None when no unary signature accepts this kind.
    /// A permitted kind supplies no numeric-domain or accepted-expression proof.
    #[must_use]
    pub const fn result_kind(self, operand: K) -> Option<K> {
        match (self, operand) {
            (Self::Negate, K::Length | K::Angle | K::Area | K::Ratio) => Some(operand),
            (Self::Square, K::Length) => Some(K::Area),
            (Self::Square, K::Ratio | K::Count) => Some(operand),
            _ => None,
        }
    }
}

pub(super) fn arithmetic(kind: K) -> bool {
    matches!(kind, K::Length | K::Angle | K::Area | K::Ratio | K::Count)
}
impl B {
    /// Exact existing machine operator symbol, independent of display presentation.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
        }
    }

    /// Normative kind signature, preserving operand order for division.
    /// Returns None for an absent pair; reads no divisor, value, state, provider or geometry.
    /// Signature acceptance supplies no numeric-domain, expression or recipe acceptance.
    #[must_use]
    pub fn result_kind(self, left: K, right: K) -> Option<K> {
        match self {
            Self::Add | Self::Subtract if left == right && arithmetic(left) => Some(left),
            Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::LessEqual
            | Self::Greater
            | Self::GreaterEqual
                if left == right && arithmetic(left) =>
            {
                Some(K::Boolean)
            }
            Self::Multiply => match (left, right) {
                (K::Length, K::Length) => Some(K::Area),
                (K::Length, K::Ratio | K::Count) | (K::Ratio | K::Count, K::Length) => {
                    Some(K::Length)
                }
                (K::Angle, K::Ratio | K::Count) | (K::Ratio | K::Count, K::Angle) => Some(K::Angle),
                (K::Area, K::Ratio | K::Count) | (K::Ratio | K::Count, K::Area) => Some(K::Area),
                (K::Ratio, K::Ratio | K::Count) | (K::Count, K::Ratio) => Some(K::Ratio),
                (K::Count, K::Count) => Some(K::Count),
                _ => None,
            },
            Self::Divide => match (left, right) {
                (K::Length, K::Length)
                | (K::Angle, K::Angle)
                | (K::Area, K::Area)
                | (K::Ratio, K::Ratio | K::Count)
                | (K::Count, K::Count) => Some(K::Ratio),
                (K::Length, K::Ratio | K::Count) | (K::Area, K::Length) => Some(K::Length),
                (K::Angle, K::Ratio | K::Count) => Some(K::Angle),
                (K::Area, K::Ratio | K::Count) => Some(K::Area),
                (K::Count, K::Ratio) => Some(K::Count),
                _ => None,
            },
            _ => None,
        }
    }

    /// Whether the absent signature is an angle-times-length product needing the arc_length hint.
    #[must_use]
    pub const fn arc_length_hint(self, left: K, right: K) -> bool {
        matches!(
            (self, left, right),
            (Self::Multiply, K::Angle, K::Length) | (Self::Multiply, K::Length, K::Angle)
        )
    }
}
