//! Node certificates borrow their successful proof, original arena and resolved declarations.
use super::{FormulaNameDependency, NodeCertificate};
use crate::recipe::{
    normalized::NormalizedData, FormulaBinaryOperator, FormulaBuiltin, FormulaBuiltinOperand,
    FormulaDeclaration, FormulaKind, FormulaLiteral, FormulaNormalizedExpression,
    FormulaSourceSpan,
};
use core::fmt;

/// Typed node view pinned to one successful expression proof; callers cannot attach foreign syntax.
/// It certifies static metadata, not value availability, numeric domains or recipe acceptance.
/// ```compile_fail
/// fn forge<'p>(expression: &'p sc_core::recipe::FormulaNormalizedExpression<'p>)
///     -> sc_core::recipe::FormulaCheckedNode<'p, 'static> {
///     sc_core::recipe::FormulaCheckedNode { expression, certificates: &[], dependencies: &[], index: 0 }
/// }
/// ```
/// The normalized source may outlive a proof; a certified child cannot escape that proof.
/// ```compile_fail
/// fn escape<'e, 's>(expression: &'e sc_core::recipe::FormulaNormalizedExpression<'s>)
///     -> sc_core::recipe::FormulaCheckedNode<'e, 'static> {
///     use sc_core::recipe::*;
///     let namespace = FormulaNamespace::new([]).unwrap();
///     let proof = expression.check_kinds(&namespace).unwrap();
///     proof.root()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FormulaCheckedNode<'p, 'd> {
    expression: &'p FormulaNormalizedExpression<'p>,
    certificates: &'p [NodeCertificate],
    dependencies: &'p [FormulaNameDependency<'d>],
    index: usize,
}
#[allow(clippy::indexing_slicing)] // Only parser-private root/child edges enter a fully certified arena.
impl<'p, 'd> FormulaCheckedNode<'p, 'd> {
    pub(super) fn root(
        expression: &'p FormulaNormalizedExpression<'p>,
        certificates: &'p [NodeCertificate],
        dependencies: &'p [FormulaNameDependency<'d>],
    ) -> Self {
        Self {
            expression,
            certificates,
            dependencies,
            index: expression.root,
        }
    }
    fn child(self, index: usize) -> Self {
        Self { index, ..self }
    }
    /// Original normalized arena owner; no copied or substitutable expression is attached.
    #[must_use]
    pub const fn expression(self) -> &'p FormulaNormalizedExpression<'p> {
        self.expression
    }
    /// Actual normalized source extent, including the node's grouping parentheses.
    #[must_use]
    pub fn span(self) -> FormulaSourceSpan {
        self.expression.nodes[self.index].span
    }
    /// Statically inferred value kind of this actual node, without evaluating it.
    #[must_use]
    pub fn kind(self) -> FormulaKind {
        self.operand().kind()
    }
    /// Actual direct tolerance-name role, or ordinary inferred value role for computed results.
    #[must_use]
    pub fn operand(self) -> FormulaBuiltinOperand {
        self.certificates[self.index].operand
    }
    /// Actual immutable syntax shape with certified child views and original resolved name sources.
    /// Explicit inspection exposes authored name/literal content; Debug omits it.
    #[must_use]
    pub fn form(self) -> FormulaCheckedNodeForm<'p, 'd> {
        match &self.expression.nodes[self.index].data {
            NormalizedData::Literal(literal) => FormulaCheckedNodeForm::Literal(*literal),
            NormalizedData::Name(name) => {
                let Some(dependency) = self.certificates[self.index].dependency else {
                    unreachable!(
                        "private successful name certificate retains its actual dependency"
                    );
                };
                FormulaCheckedNodeForm::Name {
                    name,
                    declaration: self.dependencies[dependency].declaration(),
                }
            }
            NormalizedData::Negate(child) => FormulaCheckedNodeForm::Negate(self.child(*child)),
            NormalizedData::Square(child) => FormulaCheckedNodeForm::Square(self.child(*child)),
            NormalizedData::Binary {
                operator,
                left,
                right,
            } => FormulaCheckedNodeForm::Binary {
                operator: *operator,
                left: self.child(*left),
                right: self.child(*right),
            },
            NormalizedData::Call { arguments, .. } => {
                let Some(builtin) = self.certificates[self.index].builtin else {
                    unreachable!(
                        "private successful call certificate retains its resolved builtin"
                    );
                };
                FormulaCheckedNodeForm::Call {
                    builtin,
                    arguments: FormulaCheckedArguments {
                        parent: self,
                        indices: arguments,
                    },
                }
            }
            NormalizedData::Conditional {
                condition,
                then_branch,
                else_branch,
            } => FormulaCheckedNodeForm::Conditional {
                condition: self.child(*condition),
                then_branch: self.child(*then_branch),
                else_branch: self.child(*else_branch),
            },
        }
    }
}
impl fmt::Debug for FormulaCheckedNode<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaCheckedNode")
            .field("kind", &self.kind())
            .field("span", &self.span())
            .finish_non_exhaustive()
    }
}

/// Same normalized shape with actual child certificates; no index-based construction is public.
#[derive(Clone, Copy)]
pub enum FormulaCheckedNodeForm<'p, 'd> {
    /// Original once-normalized literal; kind/domain input checks already passed.
    Literal(FormulaLiteral<'p>),
    /// Actual authored name and the exact source resolved during static checking.
    Name {
        /// Original source token, borrowing the normalized owner.
        name: &'p str,
        /// Original resolved metadata; no runtime value or registry proof is inferred.
        declaration: FormulaDeclaration<'d>,
    },
    /// Exact unary negation with its certified operand.
    Negate(FormulaCheckedNode<'p, 'd>),
    /// Exact fixed square with its certified operand.
    Square(FormulaCheckedNode<'p, 'd>),
    /// Actual ordered binary operation and its certified children.
    Binary {
        /// Parsed closed operator whose immediate signature passed.
        operator: FormulaBinaryOperator,
        /// Original left operand.
        left: FormulaCheckedNode<'p, 'd>,
        /// Original right operand.
        right: FormulaCheckedNode<'p, 'd>,
    },
    /// Statically resolved built-in and all certified arguments in original source order.
    Call {
        /// Actual closed callee resolved before its arguments.
        builtin: FormulaBuiltin,
        /// Bounded borrowed arguments; no new signature lookup or provider query is performed.
        arguments: FormulaCheckedArguments<'p, 'd>,
    },
    /// Both branches and condition passed static checks; this view executes none of them.
    Conditional {
        /// Actual Boolean condition certificate.
        condition: FormulaCheckedNode<'p, 'd>,
        /// Actual numeric then-branch certificate.
        then_branch: FormulaCheckedNode<'p, 'd>,
        /// Actual mandatory matching else-branch certificate.
        else_branch: FormulaCheckedNode<'p, 'd>,
    },
}
impl fmt::Debug for FormulaCheckedNodeForm<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(value) => f.debug_tuple("Literal").field(&value.kind()).finish(),
            Self::Name { declaration, .. } => f
                .debug_struct("Name")
                .field("kind", &declaration.kind())
                .field("origin", &declaration.origin())
                .finish_non_exhaustive(),
            Self::Negate(value) => f.debug_tuple("Negate").field(value).finish(),
            Self::Square(value) => f.debug_tuple("Square").field(value).finish(),
            Self::Binary {
                operator,
                left,
                right,
            } => f
                .debug_struct("Binary")
                .field("operator", operator)
                .field("left", left)
                .field("right", right)
                .finish(),
            Self::Call { builtin, arguments } => f
                .debug_struct("Call")
                .field("builtin", builtin)
                .field("argument_count", &arguments.len())
                .finish_non_exhaustive(),
            Self::Conditional {
                condition,
                then_branch,
                else_branch,
            } => f
                .debug_struct("Conditional")
                .field("condition", condition)
                .field("then_branch", then_branch)
                .field("else_branch", else_branch)
                .finish(),
        }
    }
}

/// Ordered borrowed call arguments; every yielded node retains the same proof and arena.
#[derive(Clone, Copy)]
pub struct FormulaCheckedArguments<'p, 'd> {
    parent: FormulaCheckedNode<'p, 'd>,
    indices: &'p [usize],
}
impl<'p, 'd> FormulaCheckedArguments<'p, 'd> {
    /// Actual argument count, already validated against this call's static signature.
    #[must_use]
    pub const fn len(self) -> usize {
        self.indices.len()
    }
    /// Whether this actual argument list is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.indices.is_empty()
    }
    /// Original left-to-right arguments; reversing retains their exact nodes rather than rechecking.
    pub fn iter(
        self,
    ) -> impl ExactSizeIterator<Item = FormulaCheckedNode<'p, 'd>> + DoubleEndedIterator + 'p {
        self.indices
            .iter()
            .copied()
            .map(move |index| self.parent.child(index))
    }
}
impl fmt::Debug for FormulaCheckedArguments<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaCheckedArguments")
            .field("len", &self.len())
            .finish_non_exhaustive()
    }
}
