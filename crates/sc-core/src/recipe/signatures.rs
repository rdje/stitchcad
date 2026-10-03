//! Immutable wanted-kind catalogs for contextual dimension diagnostics; no expression proof.
use super::{
    operators::arithmetic, FormulaBinaryOperator as B, FormulaBuiltin as F,
    FormulaBuiltinArity as A, FormulaBuiltinOperand as O, FormulaKind as K,
    FormulaUnaryOperator as U,
};

/// Typed requirement for one positional operand in a normative signature.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaOperandRequirement {
    /// Exactly this kind, without implicit promotion.
    Exact(K),
    /// Arithmetic type variable T, shared consistently across this signature.
    Arithmetic,
    /// A negatable kind N: length, angle, area or ratio.
    Negatable,
    /// One of the five symbolic tolerance-name nodes, not a computed length.
    ToleranceName,
}
/// Result-kind requirement; an operand position refers to this same signature's arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaResultRequirement {
    /// Fixed result kind.
    Exact(K),
    /// Preserve the kind of the operand at this zero-based position.
    Operand(usize),
}
use FormulaOperandRequirement as R;
use FormulaResultRequirement as V;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operands {
    One([R; 1]),
    Two([R; 2]),
    Three([R; 3]),
    Variadic(R),
}

/// Immutable catalog row. Construction is private, so positional result indices and arities
/// cannot be forged. Inspection or matching supplies metadata, not accepted source or values.
/// ```compile_fail
/// use sc_core::recipe::{FormulaBuiltin, FormulaResultRequirement, FormulaKind};
/// let mut row = *FormulaBuiltin::Hypot.signatures().first().unwrap();
/// row.result = FormulaResultRequirement::Exact(FormulaKind::Count);
/// ```
/// ```compile_fail
/// use sc_core::recipe::{FormulaBuiltin, FormulaOperandRequirement, FormulaKind};
/// let row = FormulaBuiltin::Hypot.signatures().first().unwrap();
/// row.operand_requirements()[0] = FormulaOperandRequirement::Exact(FormulaKind::Count);
/// ```
/// ```
/// use sc_core::recipe::{FormulaBuiltin, FormulaBuiltinOperand as O, FormulaKind as K};
/// let rows = FormulaBuiltin::Hypot.signatures();
/// assert_eq!(rows.first().unwrap().result_kind(&[O::Value(K::Length), O::Value(K::Length)]), Some(K::Length));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaKindSignature {
    operands: Operands,
    result: V,
}
impl FormulaKindSignature {
    const fn one(operand: R, result: V) -> Self {
        Self {
            operands: Operands::One([operand]),
            result,
        }
    }
    const fn two(left: R, right: R, result: V) -> Self {
        Self {
            operands: Operands::Two([left, right]),
            result,
        }
    }
    const fn three(first: R, second: R, third: R, result: V) -> Self {
        Self {
            operands: Operands::Three([first, second, third]),
            result,
        }
    }
    const fn variadic() -> Self {
        Self {
            operands: Operands::Variadic(R::Arithmetic),
            result: V::Operand(0),
        }
    }
    /// Ordered requirements; the sole variadic requirement repeats for every operand.
    #[must_use]
    pub fn operand_requirements(&self) -> &[R] {
        match &self.operands {
            Operands::One(args) => args,
            Operands::Two(args) => args,
            Operands::Three(args) => args,
            Operands::Variadic(arg) => core::slice::from_ref(arg),
        }
    }
    /// Result's fixed kind or actual same-signature operand position.
    #[must_use]
    pub const fn result_requirement(self) -> V {
        self.result
    }
    /// Exact fixed arity or one-or-more repetition, independent of syntax's structural bounds.
    #[must_use]
    pub const fn arity(self) -> A {
        match self.operands {
            Operands::One(_) => A::Fixed(1),
            Operands::Two(_) => A::Fixed(2),
            Operands::Three(_) => A::Fixed(3),
            Operands::Variadic(_) => A::OneOrMore,
        }
    }
    /// Match kinds and symbolic roles only, including consistent arithmetic T across operands.
    /// Reads no state, provider, numerical value or geometry, and produces no expression proof.
    #[must_use]
    pub fn result_kind(self, operands: &[O]) -> Option<K> {
        if !self.arity().accepts(operands.len()) {
            return None;
        }
        let mut arithmetic_kind = None;
        for (index, &operand) in operands.iter().enumerate() {
            let requirement = match &self.operands {
                Operands::Variadic(requirement) => requirement,
                _ => self.operand_requirements().get(index)?,
            };
            let kind = operand.kind();
            let accepts = match requirement {
                R::Exact(wanted) => kind == *wanted,
                R::Arithmetic => {
                    let same = arithmetic_kind.is_none_or(|prior| prior == kind);
                    arithmetic_kind = Some(kind);
                    arithmetic(kind) && same
                }
                R::Negatable => matches!(kind, K::Length | K::Angle | K::Area | K::Ratio),
                R::ToleranceName => matches!(operand, O::Tolerance(_)),
            };
            if !accepts {
                return None;
            }
        }
        match self.result {
            V::Exact(kind) => Some(kind),
            V::Operand(index) => operands.get(index).map(|operand| operand.kind()),
        }
    }
}
const fn pair(left: K, right: K, result: K) -> FormulaKindSignature {
    FormulaKindSignature::two(R::Exact(left), R::Exact(right), V::Exact(result))
}
impl U {
    /// Complete closed wanted-kind alternatives for this unary operator.
    #[must_use]
    pub const fn signatures(self) -> &'static [FormulaKindSignature] {
        match self {
            Self::Negate => const { &[FormulaKindSignature::one(R::Negatable, V::Operand(0))] },
            Self::Square => {
                const {
                    &[
                        FormulaKindSignature::one(R::Exact(K::Length), V::Exact(K::Area)),
                        FormulaKindSignature::one(R::Exact(K::Ratio), V::Exact(K::Ratio)),
                        FormulaKindSignature::one(R::Exact(K::Count), V::Exact(K::Count)),
                    ]
                }
            }
        }
    }
}
impl B {
    /// Complete ordered wanted-kind alternatives, including both product orders and directed quotients.
    #[must_use]
    pub const fn signatures(self) -> &'static [FormulaKindSignature] {
        match self {
            Self::Add | Self::Subtract => {
                const {
                    &[FormulaKindSignature::two(
                        R::Arithmetic,
                        R::Arithmetic,
                        V::Operand(0),
                    )]
                }
            }
            Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::LessEqual
            | Self::Greater
            | Self::GreaterEqual => {
                const {
                    &[FormulaKindSignature::two(
                        R::Arithmetic,
                        R::Arithmetic,
                        V::Exact(K::Boolean),
                    )]
                }
            }
            Self::Multiply => {
                const {
                    &[
                        pair(K::Length, K::Length, K::Area),
                        pair(K::Length, K::Ratio, K::Length),
                        pair(K::Ratio, K::Length, K::Length),
                        pair(K::Length, K::Count, K::Length),
                        pair(K::Count, K::Length, K::Length),
                        pair(K::Angle, K::Ratio, K::Angle),
                        pair(K::Ratio, K::Angle, K::Angle),
                        pair(K::Angle, K::Count, K::Angle),
                        pair(K::Count, K::Angle, K::Angle),
                        pair(K::Area, K::Ratio, K::Area),
                        pair(K::Ratio, K::Area, K::Area),
                        pair(K::Area, K::Count, K::Area),
                        pair(K::Count, K::Area, K::Area),
                        pair(K::Ratio, K::Ratio, K::Ratio),
                        pair(K::Ratio, K::Count, K::Ratio),
                        pair(K::Count, K::Ratio, K::Ratio),
                        pair(K::Count, K::Count, K::Count),
                    ]
                }
            }
            Self::Divide => {
                const {
                    &[
                        pair(K::Length, K::Length, K::Ratio),
                        pair(K::Length, K::Ratio, K::Length),
                        pair(K::Length, K::Count, K::Length),
                        pair(K::Angle, K::Angle, K::Ratio),
                        pair(K::Angle, K::Ratio, K::Angle),
                        pair(K::Angle, K::Count, K::Angle),
                        pair(K::Area, K::Length, K::Length),
                        pair(K::Area, K::Ratio, K::Area),
                        pair(K::Area, K::Count, K::Area),
                        pair(K::Area, K::Area, K::Ratio),
                        pair(K::Ratio, K::Ratio, K::Ratio),
                        pair(K::Ratio, K::Count, K::Ratio),
                        pair(K::Count, K::Count, K::Ratio),
                        pair(K::Count, K::Ratio, K::Count),
                    ]
                }
            }
        }
    }
}
impl F {
    /// Complete wanted-kind alternatives from grammar6/6.1/7, without reading operands or values.
    #[must_use]
    pub const fn signatures(self) -> &'static [FormulaKindSignature] {
        match self {
            Self::Sqrt => {
                const {
                    &[
                        FormulaKindSignature::one(R::Exact(K::Area), V::Exact(K::Length)),
                        FormulaKindSignature::one(R::Exact(K::Ratio), V::Exact(K::Ratio)),
                    ]
                }
            }
            Self::Hypot => const { &[pair(K::Length, K::Length, K::Length)] },
            Self::Abs => const { &[FormulaKindSignature::one(R::Arithmetic, V::Operand(0))] },
            Self::Min | Self::Max => const { &[FormulaKindSignature::variadic()] },
            Self::Clamp => {
                const {
                    &[FormulaKindSignature::three(
                        R::Arithmetic,
                        R::Arithmetic,
                        R::Arithmetic,
                        V::Operand(0),
                    )]
                }
            }
            Self::RoundTo => {
                const {
                    &[FormulaKindSignature::two(
                        R::Arithmetic,
                        R::Arithmetic,
                        V::Operand(0),
                    )]
                }
            }
            Self::Sin | Self::Cos | Self::Tan => {
                const {
                    &[FormulaKindSignature::one(
                        R::Exact(K::Angle),
                        V::Exact(K::Ratio),
                    )]
                }
            }
            Self::Atan => {
                const {
                    &[FormulaKindSignature::one(
                        R::Exact(K::Ratio),
                        V::Exact(K::Angle),
                    )]
                }
            }
            Self::Atan2 => {
                const {
                    &[
                        pair(K::Length, K::Length, K::Angle),
                        pair(K::Ratio, K::Ratio, K::Angle),
                    ]
                }
            }
            Self::ArcLength => const { &[pair(K::Angle, K::Length, K::Length)] },
            Self::If => {
                const {
                    &[FormulaKindSignature::three(
                        R::Exact(K::Boolean),
                        R::Arithmetic,
                        R::Arithmetic,
                        V::Operand(1),
                    )]
                }
            }
            Self::Within => {
                const {
                    &[FormulaKindSignature::three(
                        R::Arithmetic,
                        R::Arithmetic,
                        R::ToleranceName,
                        V::Exact(K::Boolean),
                    )]
                }
            }
            Self::X | Self::Y => {
                const {
                    &[FormulaKindSignature::one(
                        R::Exact(K::Point),
                        V::Exact(K::Length),
                    )]
                }
            }
            Self::Dist => const { &[pair(K::Point, K::Point, K::Length)] },
            Self::Dir => const { &[pair(K::Point, K::Point, K::Angle)] },
            Self::Len => {
                const {
                    &[FormulaKindSignature::one(
                        R::Exact(K::Edge),
                        V::Exact(K::Length),
                    )]
                }
            }
            Self::ParamAt => const { &[pair(K::Edge, K::Length, K::Ratio)] },
            Self::PointAt => const { &[pair(K::Edge, K::Ratio, K::Point)] },
        }
    }
}
