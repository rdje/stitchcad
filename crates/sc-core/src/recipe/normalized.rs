//! Immutable flat normalized syntax: literal conversion only, with every structural edge retained.
use super::expression::NodeData;
use super::{
    FormulaBinaryOperator, FormulaExpression, FormulaLiteral, FormulaLiteralError,
    FormulaSourceSpan,
};
use core::fmt;

#[derive(Clone, Debug)]
pub(super) struct NormalizedNode<'a> {
    pub span: FormulaSourceSpan,
    pub data: NormalizedData<'a>,
}
#[derive(Clone, Debug)]
pub(super) enum NormalizedData<'a> {
    Literal(FormulaLiteral<'a>),
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

/// Separately owned, immutable normalized arena borrowing the original source.
/// Literal inputs are converted; no operator/name/type validation or evaluation is implied.
/// ```compile_fail
/// fn mutate(e: &mut sc_core::recipe::FormulaNormalizedExpression<'_>) { e.nodes.clear(); }
/// ```
/// ```compile_fail
/// fn detached() -> sc_core::recipe::FormulaNormalizedExpression<'static> {
///     let source = String::from("2.5 cm + ease_waist");
///     sc_core::recipe::FormulaExpression::parse(&source).unwrap().normalize_literals().unwrap()
/// }
/// ```
/// Views cannot outlive their normalized arena, even when source remains available.
/// ```compile_fail
/// fn escaped(source: &str) -> sc_core::recipe::FormulaNormalizedNode<'_> {
///     let syntax = sc_core::recipe::FormulaExpression::parse(source).unwrap();
///     let normalized = syntax.normalize_literals().unwrap();
///     normalized.root()
/// }
/// ```
#[derive(Clone)]
pub struct FormulaNormalizedExpression<'a> {
    pub(super) nodes: Vec<NormalizedNode<'a>>,
    pub(super) root: usize,
    pub(super) if_depth: usize,
}
impl fmt::Debug for FormulaNormalizedExpression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNormalizedExpression")
            .field("node_count", &self.nodes.len())
            .field("conditional_depth", &self.if_depth)
            .finish_non_exhaustive()
    }
}
impl FormulaNormalizedExpression<'_> {
    /// Read-only root; callers cannot supply an index or attach a different arena.
    #[must_use]
    #[allow(clippy::indexing_slicing)] // Conversion retains the validated parser root and all indices.
    pub fn root(&self) -> FormulaNormalizedNode<'_> {
        FormulaNormalizedNode {
            node: &self.nodes[self.root],
            arena: &self.nodes,
        }
    }
    /// Same semantic node count as syntax, including both branches and every call argument.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    /// Same maximum conditional depth as the validated syntax arena.
    #[must_use]
    pub const fn conditional_depth(&self) -> usize {
        self.if_depth
    }
}
impl<'a> FormulaExpression<'a> {
    /// Convert every literal without evaluating any operator or modifying this syntax arena.
    /// Any literal refusal aborts construction with its original span/rule; no partial result escapes.
    /// Conversion and destruction are flat, independent of source grouping/unary depth.
    /// The returned arena borrows source rather than this syntax allocation.
    /// ```
    /// use sc_core::recipe::{FormulaExpression, FormulaNormalizedNodeKind, FormulaLiteralKind};
    /// let syntax = FormulaExpression::parse("720 deg")?;
    /// let normalized = syntax.normalize_literals()?;
    /// drop(syntax);
    /// if let FormulaNormalizedNodeKind::Literal(literal) = normalized.root().kind() {
    ///     assert_eq!(literal.kind(), FormulaLiteralKind::Angle);
    ///     assert_eq!(literal.magnitude(), 720_000_000);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn normalize_literals(
        &self,
    ) -> Result<FormulaNormalizedExpression<'a>, FormulaLiteralError> {
        let mut nodes = Vec::with_capacity(self.nodes.len());
        for node in &self.nodes {
            let data = match &node.data {
                NodeData::Literal { number, unit } => {
                    NormalizedData::Literal(super::literal::normalize(number, *unit, node.span)?)
                }
                NodeData::Name(name) => NormalizedData::Name(name),
                NodeData::Negate(child) => NormalizedData::Negate(*child),
                NodeData::Square(child) => NormalizedData::Square(*child),
                NodeData::Binary {
                    operator,
                    left,
                    right,
                } => NormalizedData::Binary {
                    operator: *operator,
                    left: *left,
                    right: *right,
                },
                NodeData::Call { name, arguments } => NormalizedData::Call {
                    name,
                    arguments: arguments.clone(),
                },
                NodeData::Conditional {
                    condition,
                    then_branch,
                    else_branch,
                } => NormalizedData::Conditional {
                    condition: *condition,
                    then_branch: *then_branch,
                    else_branch: *else_branch,
                },
            };
            nodes.push(NormalizedNode {
                span: node.span,
                data,
            });
        }
        Ok(FormulaNormalizedExpression {
            nodes,
            root: self.root,
            if_depth: self.if_depth,
        })
    }
}
/// Read-only semantic node view. Child views retain their own arena; indices cannot be forged.
#[derive(Clone, Copy)]
pub struct FormulaNormalizedNode<'a> {
    node: &'a NormalizedNode<'a>,
    arena: &'a [NormalizedNode<'a>],
}
impl fmt::Debug for FormulaNormalizedNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNormalizedNode")
            .field("span", &self.node.span)
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaNormalizedNode<'a> {
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
    pub fn kind(self) -> FormulaNormalizedNodeKind<'a> {
        match &self.node.data {
            NormalizedData::Literal(literal) => FormulaNormalizedNodeKind::Literal(*literal),
            NormalizedData::Name(name) => FormulaNormalizedNodeKind::Name(name),
            NormalizedData::Negate(child) => FormulaNormalizedNodeKind::Negate(self.child(*child)),
            NormalizedData::Square(child) => FormulaNormalizedNodeKind::Square(self.child(*child)),
            NormalizedData::Binary {
                operator,
                left,
                right,
            } => FormulaNormalizedNodeKind::Binary {
                operator: *operator,
                left: self.child(*left),
                right: self.child(*right),
            },
            NormalizedData::Call { name, arguments } => FormulaNormalizedNodeKind::Call {
                name,
                arguments: FormulaNormalizedArguments {
                    arena: self.arena,
                    indices: arguments.iter(),
                },
            },
            NormalizedData::Conditional {
                condition,
                then_branch,
                else_branch,
            } => FormulaNormalizedNodeKind::Conditional {
                condition: self.child(*condition),
                then_branch: self.child(*then_branch),
                else_branch: self.child(*else_branch),
            },
        }
    }
}

/// Inspected normalized syntax; operators/names have not been validated or evaluated.
#[derive(Clone, Debug)]
pub enum FormulaNormalizedNodeKind<'a> {
    /// Once-rounded typed literal input; source spelling/span are retained.
    Literal(FormulaLiteral<'a>),
    /// Borrowed machine name; existence/currentness not checked.
    Name(&'a str),
    /// Unary minus.
    Negate(FormulaNormalizedNode<'a>),
    /// Square, whose `2` payload is not an expression child.
    Square(FormulaNormalizedNode<'a>),
    /// Binary syntax.
    Binary {
        /// Operator.
        operator: FormulaBinaryOperator,
        /// Left operand.
        left: FormulaNormalizedNode<'a>,
        /// Right operand.
        right: FormulaNormalizedNode<'a>,
    },
    /// Ordinary call; later checks enforce closed vocabulary/signatures.
    Call {
        /// Borrowed call name.
        name: &'a str,
        /// Nonempty ordered arguments.
        arguments: FormulaNormalizedArguments<'a>,
    },
    /// Mandatory three-part special form; every branch is present statically.
    Conditional {
        /// Condition, not yet required to be boolean by syntax alone.
        condition: FormulaNormalizedNode<'a>,
        /// Then branch.
        then_branch: FormulaNormalizedNode<'a>,
        /// Else branch.
        else_branch: FormulaNormalizedNode<'a>,
    },
}

/// Read-only, exact-size/fused call-argument iteration, with no forged arena handles.
#[derive(Clone)]
pub struct FormulaNormalizedArguments<'a> {
    arena: &'a [NormalizedNode<'a>],
    indices: std::slice::Iter<'a, usize>,
}
impl fmt::Debug for FormulaNormalizedArguments<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNormalizedArguments")
            .field("remaining", &self.indices.len())
            .finish_non_exhaustive()
    }
}
impl<'a> Iterator for FormulaNormalizedArguments<'a> {
    type Item = FormulaNormalizedNode<'a>;
    #[allow(clippy::indexing_slicing)] // The parser stores only existing child indices in private calls.
    fn next(&mut self) -> Option<Self::Item> {
        self.indices.next().map(|index| FormulaNormalizedNode {
            node: &self.arena[*index],
            arena: self.arena,
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}
impl ExactSizeIterator for FormulaNormalizedArguments<'_> {}
impl std::iter::FusedIterator for FormulaNormalizedArguments<'_> {}
