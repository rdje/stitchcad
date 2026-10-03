//! Closed call signatures and symbolic operand roles; no value or geometry execution.
use super::{operators::arithmetic, FormulaKind as K, FormulaToleranceName};

/// Signature metadata retains a tolerance symbol separately from its ordinary length kind.
/// This descriptor is not proof that an expression resolves to that symbol; static checking
/// must obtain the role from the actual reserved-name node, not from a computed length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaBuiltinOperand {
    /// Ordinary operand kind, including lengths that are not tolerance-name nodes.
    Value(K),
    /// Exact existing reserved tolerance class, without a provider or numeric value.
    Tolerance(FormulaToleranceName),
}
impl FormulaBuiltinOperand {
    /// Ordinary value kind; a tolerance symbol is length outside its special operand role.
    #[must_use]
    pub const fn kind(self) -> K {
        match self {
            Self::Value(kind) => kind,
            Self::Tolerance(_) => K::Length,
        }
    }
}

/// Closed syntactic role; registry membership adds no keyword or accepted-expression proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaBuiltinCategory {
    /// Numeric function, with value/domain rules checked separately during execution.
    Function,
    /// Read-only selector on previously constructed geometry.
    Selector,
    /// The existing if special form; both branches are checked before either is evaluated.
    Conditional,
    /// Within compares arithmetic values at a symbolic tolerance class.
    ToleranceComparison,
}

/// Signature arity, independent of the parser's structural argument limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaBuiltinArity {
    /// Exactly this many positional operands.
    Fixed(usize),
    /// One or more homogeneous arithmetic operands; zero is not a signature.
    OneOrMore,
}
impl FormulaBuiltinArity {
    /// Whether the count fits the signature; does not grant a structural-limit exception.
    #[must_use]
    pub const fn accepts(self, count: usize) -> bool {
        match self {
            Self::Fixed(wanted) => count == wanted,
            Self::OneOrMore => count >= 1,
        }
    }
}

/// Every normative function, selector and special form; metadata never invokes a provider.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaBuiltin {
    /// Square root of area or ratio.
    Sqrt,
    /// Hypotenuse from two lengths.
    Hypot,
    /// Absolute arithmetic value.
    Abs,
    /// Minimum of one or more values of the same arithmetic kind.
    Min,
    /// Maximum of one or more values of the same arithmetic kind.
    Max,
    /// Arithmetic value limited by two bounds of its own kind.
    Clamp,
    /// Arithmetic value rounded to a step of its own kind.
    RoundTo,
    /// Sine of an angle.
    Sin,
    /// Cosine of an angle.
    Cos,
    /// Tangent of an angle.
    Tan,
    /// Signed principal angle of a ratio.
    Atan,
    /// Signed principal angle of positional y, x components, both length or both ratio.
    Atan2,
    /// Length of an arc from an angle followed by a radius.
    ArcLength,
    /// Existing special form with Boolean condition and two matching arithmetic branches.
    If,
    /// Arithmetic comparison with an exact symbolic tolerance as its third operand.
    Within,
    /// Point x coordinate.
    X,
    /// Point y coordinate.
    Y,
    /// Distance between points.
    Dist,
    /// Direction from the first point to the second.
    Dir,
    /// Arc length of an edge.
    Len,
    /// Edge parameter at the supplied arc length.
    ParamAt,
    /// Existing edge's point at the supplied parameter.
    PointAt,
}
impl FormulaBuiltin {
    /// Complete closed population in normative table order, with grouped tokens expanded.
    pub const ALL: [Self; 22] = [
        Self::Sqrt,
        Self::Hypot,
        Self::Abs,
        Self::Min,
        Self::Max,
        Self::Clamp,
        Self::RoundTo,
        Self::Sin,
        Self::Cos,
        Self::Tan,
        Self::Atan,
        Self::Atan2,
        Self::ArcLength,
        Self::If,
        Self::Within,
        Self::X,
        Self::Y,
        Self::Dist,
        Self::Dir,
        Self::Len,
        Self::ParamAt,
        Self::PointAt,
    ];

    /// Exact existing machine spelling; no display aliases or new reservations.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Sqrt => "sqrt",
            Self::Hypot => "hypot",
            Self::Abs => "abs",
            Self::Min => "min",
            Self::Max => "max",
            Self::Clamp => "clamp",
            Self::RoundTo => "round_to",
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Atan => "atan",
            Self::Atan2 => "atan2",
            Self::ArcLength => "arc_length",
            Self::If => "if",
            Self::Within => "within",
            Self::X => "x",
            Self::Y => "y",
            Self::Dist => "dist",
            Self::Dir => "dir",
            Self::Len => "len",
            Self::ParamAt => "param_at",
            Self::PointAt => "point_at",
        }
    }

    /// Recognize only an exact registry spelling, without trimming or case conversion.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|builtin| builtin.token() == token)
    }

    /// Existing call or special-form role, independent of value availability.
    #[must_use]
    pub const fn category(self) -> FormulaBuiltinCategory {
        match self {
            Self::X
            | Self::Y
            | Self::Dist
            | Self::Dir
            | Self::Len
            | Self::ParamAt
            | Self::PointAt => FormulaBuiltinCategory::Selector,
            Self::If => FormulaBuiltinCategory::Conditional,
            Self::Within => FormulaBuiltinCategory::ToleranceComparison,
            _ => FormulaBuiltinCategory::Function,
        }
    }

    /// Exact positional arity; min/max alone are variadic from one operand.
    #[must_use]
    pub const fn arity(self) -> FormulaBuiltinArity {
        match self {
            Self::Min | Self::Max => FormulaBuiltinArity::OneOrMore,
            Self::Clamp | Self::If | Self::Within => FormulaBuiltinArity::Fixed(3),
            Self::Hypot
            | Self::RoundTo
            | Self::Atan2
            | Self::ArcLength
            | Self::Dist
            | Self::Dir
            | Self::ParamAt
            | Self::PointAt => FormulaBuiltinArity::Fixed(2),
            _ => FormulaBuiltinArity::Fixed(1),
        }
    }

    /// Result of the closed signature, or None when arity, kind or symbolic role does not fit.
    /// Reads metadata only; domains, numeric rounding, context availability, geometry and
    /// whole-expression acceptance remain separate obligations.
    #[must_use]
    pub fn result_kind(self, operands: &[FormulaBuiltinOperand]) -> Option<K> {
        if !self.arity().accepts(operands.len()) {
            return None;
        }
        match (self, operands) {
            (Self::Abs | Self::Min | Self::Max | Self::Clamp | Self::RoundTo, args) => {
                same_arithmetic(args)
            }
            (Self::Sqrt, [a]) => match a.kind() {
                K::Area => Some(K::Length),
                K::Ratio => Some(K::Ratio),
                _ => None,
            },
            (Self::Hypot, [a, b]) if a.kind() == K::Length && b.kind() == K::Length => {
                Some(K::Length)
            }
            (Self::Sin | Self::Cos | Self::Tan, [a]) if a.kind() == K::Angle => Some(K::Ratio),
            (Self::Atan, [a]) if a.kind() == K::Ratio => Some(K::Angle),
            (Self::Atan2, [a, b])
                if a.kind() == b.kind() && matches!(a.kind(), K::Length | K::Ratio) =>
            {
                Some(K::Angle)
            }
            (Self::ArcLength, [a, b]) if a.kind() == K::Angle && b.kind() == K::Length => {
                Some(K::Length)
            }
            (Self::If, [condition, left, right]) if condition.kind() == K::Boolean => {
                same_arithmetic(&[*left, *right])
            }
            (Self::Within, [left, right, FormulaBuiltinOperand::Tolerance(_)]) => {
                same_arithmetic(&[*left, *right]).map(|_| K::Boolean)
            }
            (Self::X | Self::Y, [point]) if point.kind() == K::Point => Some(K::Length),
            (Self::Dist, [a, b]) if a.kind() == K::Point && b.kind() == K::Point => Some(K::Length),
            (Self::Dir, [a, b]) if a.kind() == K::Point && b.kind() == K::Point => Some(K::Angle),
            (Self::Len, [edge]) if edge.kind() == K::Edge => Some(K::Length),
            (Self::ParamAt, [edge, length])
                if edge.kind() == K::Edge && length.kind() == K::Length =>
            {
                Some(K::Ratio)
            }
            (Self::PointAt, [edge, ratio])
                if edge.kind() == K::Edge && ratio.kind() == K::Ratio =>
            {
                Some(K::Point)
            }
            _ => None,
        }
    }
}

fn same_arithmetic(operands: &[FormulaBuiltinOperand]) -> Option<K> {
    let first = operands.first()?.kind();
    (arithmetic(first) && operands.iter().all(|operand| operand.kind() == first)).then_some(first)
}
