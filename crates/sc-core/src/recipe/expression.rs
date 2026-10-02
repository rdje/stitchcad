//! Immutable, source-borrowing expression syntax; no numeric or binding authority.
use super::{FormulaLexicalRule, FormulaSourceSpan};
use core::fmt;

/// Closed literal-unit vocabulary. Conversion belongs to canonicalization, not syntax parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaUnit {
    /// Micrometres.
    Micrometre,
    /// Millimetres.
    Millimetre,
    /// Centimetres.
    Centimetre,
    /// Metres.
    Metre,
    /// Inches.
    Inch,
    /// Degrees.
    Degree,
    /// Percent.
    Percent,
}
impl FormulaUnit {
    /// Exact ASCII machine token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Micrometre => "um",
            Self::Millimetre => "mm",
            Self::Centimetre => "cm",
            Self::Metre => "m",
            Self::Inch => "in",
            Self::Degree => "deg",
            Self::Percent => "pct",
        }
    }
    pub(super) fn from_token(token: &str) -> Option<Self> {
        match token {
            "um" => Some(Self::Micrometre),
            "mm" => Some(Self::Millimetre),
            "cm" => Some(Self::Centimetre),
            "m" => Some(Self::Metre),
            "in" => Some(Self::Inch),
            "deg" => Some(Self::Degree),
            "pct" => Some(Self::Percent),
            _ => None,
        }
    }
}

/// Binary syntax; dimensional applicability is checked in a later layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaBinaryOperator {
    /// `+`.
    Add,
    /// `-`.
    Subtract,
    /// `*`.
    Multiply,
    /// `/`.
    Divide,
    /// `==`.
    Equal,
    /// `!=`.
    NotEqual,
    /// `<`.
    Less,
    /// `<=`.
    LessEqual,
    /// `>`.
    Greater,
    /// `>=`.
    GreaterEqual,
}

/// Structural expression limits fixed by the formula contract §4.3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaExpressionLimit {
    /// All semantic nodes, including ordinary calls and every conditional branch.
    Nodes,
    /// Nested conditionals; siblings do not add their depths.
    ConditionalDepth,
}
impl FormulaExpressionLimit {
    /// Normative maximum; callers cannot widen it.
    #[must_use]
    pub const fn bound(self) -> usize {
        match self {
            Self::Nodes => 256,
            Self::ConditionalDepth => 16,
        }
    }
    /// Canonical limit name for structured diagnostics.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Nodes => "max_expression_nodes",
            Self::ConditionalDepth => "max_if_depth",
        }
    }
}

/// Refused syntax rule or measured structural bound. No customer source is included.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaParseRule {
    /// The shared lexical scanner refused its precise rule.
    Lexical(FormulaLexicalRule),
    /// An operand is missing or a token cannot begin one.
    ExpectedOperand,
    /// A special-form keyword must be followed by `(`.
    ExpectedCallParenthesis,
    /// An atom follows a completed operand without a binary operator.
    UnexpectedToken,
    /// A closing delimiter or comma has no matching grammatical position.
    UnexpectedDelimiter,
    /// An opening delimiter has no closing parenthesis.
    UnclosedParenthesis,
    /// An ordinary call has no required first argument.
    EmptyArguments,
    /// A conditional does not have exactly three parts.
    ConditionalArity,
    /// A known unit does not follow its numeric token with exactly one ASCII space.
    UnitSeparator,
    /// Comparisons cannot chain within one grammatical expression.
    ChainedComparison,
    /// One postfix expression cannot carry two ungrouped square suffixes.
    RepeatedPower,
    /// An exponent is not the exact machine literal `2`.
    UnsupportedExponent,
    /// A semantic structural limit was exceeded on encounter.
    StructuralLimit {
        /// Limit whose bound was exceeded.
        limit: FormulaExpressionLimit,
        /// Measured size at the refusal point, not a fabricated final size.
        measured: usize,
    },
    /// An internal arena/stack invariant failed; no fallback tree is fabricated.
    InternalStructure,
}

/// Low-level expression refusal with exact source location. Statement/canonical context is later work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaParseError {
    pub(super) span: FormulaSourceSpan,
    pub(super) rule: FormulaParseRule,
}
impl FormulaParseError {
    /// Stable diagnostic family; raw tokens are localized by the command layer.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        match self.rule {
            FormulaParseRule::StructuralLimit { .. } => "formula_domain",
            FormulaParseRule::UnsupportedExponent => "formula_unsupported",
            _ => "formula_parse",
        }
    }
    /// Offending token/gap, opener, or zero-width end-of-source location.
    #[must_use]
    pub const fn span(self) -> FormulaSourceSpan {
        self.span
    }
    /// Typed rule, including measured size and the limit's fixed bound where applicable.
    #[must_use]
    pub const fn rule(self) -> FormulaParseRule {
        self.rule
    }
}
impl fmt::Display for FormulaParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?} at bytes {}..{}",
            self.diagnostic_code(),
            self.rule,
            self.span.start(),
            self.span.end()
        )
    }
}
impl std::error::Error for FormulaParseError {}

#[derive(Clone, Debug)]
pub(super) struct Node<'a> {
    pub span: FormulaSourceSpan,
    pub data: NodeData<'a>,
}
#[derive(Clone, Debug)]
pub(super) enum NodeData<'a> {
    Literal {
        number: &'a str,
        unit: Option<FormulaUnit>,
    },
    Name(&'a str),
    Negate(usize),
    Square(usize),
    Binary {
        operator: FormulaBinaryOperator,
        left: usize,
        right: usize,
    },
    Call {
        name: &'a str,
        arguments: Vec<usize>,
    },
    Conditional {
        condition: usize,
        then_branch: usize,
        else_branch: usize,
    },
}

/// Privately built flat syntax arena borrowing source spellings. No canonical identity/value implied.
/// ```compile_fail
/// fn mutate(e: &mut sc_core::recipe::FormulaExpression<'_>) { e.nodes.clear(); }
/// ```
/// ```compile_fail
/// fn detached() -> sc_core::recipe::FormulaExpression<'static> {
///     let input = String::from("waist_girth + 1 cm");
///     sc_core::recipe::FormulaExpression::parse(&input).unwrap()
/// }
/// ```
/// A node view cannot outlive the parsed arena even if its source still exists.
/// ```compile_fail
/// fn escaped(source: &str) -> sc_core::recipe::FormulaNode<'_> {
///     let tree = sc_core::recipe::FormulaExpression::parse(source).unwrap();
///     tree.root()
/// }
/// ```
#[derive(Clone)]
pub struct FormulaExpression<'a> {
    pub(super) nodes: Vec<Node<'a>>,
    pub(super) root: usize,
    pub(super) if_depth: usize,
}
impl fmt::Debug for FormulaExpression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaExpression")
            .field("node_count", &self.nodes.len())
            .field("conditional_depth", &self.if_depth)
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaExpression<'a> {
    /// Parse one whole machine expression, without statements, conversion, binding or evaluation.
    /// Operator/delimiter/value stacks and arena destruction do not recurse with input nesting.
    pub fn parse(source: &'a str) -> Result<Self, FormulaParseError> {
        super::parser::parse(source)
    }
    /// Read-only root, inseparable from this arena; no cross-tree index can be supplied.
    #[must_use]
    #[allow(clippy::indexing_slicing)] // Successful parser construction proves the private root exists.
    pub fn root(&self) -> FormulaNode<'_> {
        FormulaNode {
            node: &self.nodes[self.root],
            arena: &self.nodes,
        }
    }
    /// Semantic count; grouping and square exponent payload are not extra nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    /// Maximum conditional nesting through all branches and call arguments.
    #[must_use]
    pub const fn conditional_depth(&self) -> usize {
        self.if_depth
    }
}

/// Read-only semantic node view. Child views retain their own arena; indices cannot be forged.
#[derive(Clone, Copy)]
pub struct FormulaNode<'a> {
    node: &'a Node<'a>,
    arena: &'a [Node<'a>],
}
impl fmt::Debug for FormulaNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNode")
            .field("span", &self.node.span)
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaNode<'a> {
    #[allow(clippy::indexing_slicing)] // Private arena edges always target previously created nodes.
    fn child(self, index: usize) -> Self {
        Self {
            node: &self.arena[index],
            arena: self.arena,
        }
    }
    /// Semantic source extent, including grouping parentheses around this node.
    #[must_use]
    pub const fn span(self) -> FormulaSourceSpan {
        self.node.span
    }
    /// Explicit inspection exposes borrowed name/literal text. Debug of the tree/view omits it.
    #[must_use]
    pub fn kind(self) -> FormulaNodeKind<'a> {
        match &self.node.data {
            NodeData::Literal { number, unit } => FormulaNodeKind::Literal {
                number,
                unit: *unit,
            },
            NodeData::Name(name) => FormulaNodeKind::Name(name),
            NodeData::Negate(child) => FormulaNodeKind::Negate(self.child(*child)),
            NodeData::Square(child) => FormulaNodeKind::Square(self.child(*child)),
            NodeData::Binary {
                operator,
                left,
                right,
            } => FormulaNodeKind::Binary {
                operator: *operator,
                left: self.child(*left),
                right: self.child(*right),
            },
            NodeData::Call { name, arguments } => FormulaNodeKind::Call {
                name,
                arguments: FormulaArguments {
                    arena: self.arena,
                    indices: arguments.iter(),
                },
            },
            NodeData::Conditional {
                condition,
                then_branch,
                else_branch,
            } => FormulaNodeKind::Conditional {
                condition: self.child(*condition),
                then_branch: self.child(*then_branch),
                else_branch: self.child(*else_branch),
            },
        }
    }
}

/// Inspected syntax; no operand has been dimensioned or evaluated.
#[derive(Clone, Debug)]
pub enum FormulaNodeKind<'a> {
    /// Exact numeric spelling and optional unit; no conversion/rounding yet.
    Literal {
        /// Borrowed digits/fraction.
        number: &'a str,
        /// Closed unit token.
        unit: Option<FormulaUnit>,
    },
    /// Borrowed machine name; existence/currentness not checked.
    Name(&'a str),
    /// Unary minus.
    Negate(FormulaNode<'a>),
    /// Square, whose `2` payload is not an expression child.
    Square(FormulaNode<'a>),
    /// Binary syntax.
    Binary {
        /// Operator.
        operator: FormulaBinaryOperator,
        /// Left operand.
        left: FormulaNode<'a>,
        /// Right operand.
        right: FormulaNode<'a>,
    },
    /// Ordinary call; later checks enforce closed vocabulary/signatures.
    Call {
        /// Borrowed call name.
        name: &'a str,
        /// Nonempty ordered arguments.
        arguments: FormulaArguments<'a>,
    },
    /// Mandatory three-part special form; every branch is present statically.
    Conditional {
        /// Condition, not yet required to be boolean by syntax alone.
        condition: FormulaNode<'a>,
        /// Then branch.
        then_branch: FormulaNode<'a>,
        /// Else branch.
        else_branch: FormulaNode<'a>,
    },
}

/// Read-only, exact-size/fused call-argument iteration, with no forged arena handles.
#[derive(Clone)]
pub struct FormulaArguments<'a> {
    arena: &'a [Node<'a>],
    indices: std::slice::Iter<'a, usize>,
}
impl fmt::Debug for FormulaArguments<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaArguments")
            .field("remaining", &self.indices.len())
            .finish_non_exhaustive()
    }
}
impl<'a> Iterator for FormulaArguments<'a> {
    type Item = FormulaNode<'a>;
    #[allow(clippy::indexing_slicing)] // The parser stores only existing child indices in private calls.
    fn next(&mut self) -> Option<Self::Item> {
        self.indices.next().map(|index| FormulaNode {
            node: &self.arena[*index],
            arena: self.arena,
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}
impl ExactSizeIterator for FormulaArguments<'_> {}
impl std::iter::FusedIterator for FormulaArguments<'_> {}
