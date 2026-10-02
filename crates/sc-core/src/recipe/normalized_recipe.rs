//! Owned normalized statement arenas; input conversion grants no binding or execution authority.
use super::{
    FormulaBindingKind, FormulaLiteralError, FormulaNormalizedExpression,
    FormulaSourceSpan as Span, FormulaStatementExpression, FormulaToleranceName,
};
use core::fmt;

/// Exact input-literal refusal with the statement operand that contains it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaStatementLiteralError {
    pub(super) part: FormulaStatementExpression,
    pub(super) error: FormulaLiteralError,
}
impl FormulaStatementLiteralError {
    /// Which expression failed conversion; assertion operands are kept distinct.
    #[must_use]
    pub const fn expression_part(self) -> FormulaStatementExpression {
        self.part
    }
    /// Original literal rule, bound witness and whole-source span.
    #[must_use]
    pub const fn literal_error(self) -> FormulaLiteralError {
        self.error
    }
    /// Original offending literal span, including its grouping.
    #[must_use]
    pub const fn span(self) -> Span {
        self.error.span()
    }
    /// Stable formula-domain family; localized rendering belongs to the command boundary.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        self.error.diagnostic_code()
    }
}
impl fmt::Display for FormulaStatementLiteralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} in {:?}", self.error, self.part)
    }
}
impl std::error::Error for FormulaStatementLiteralError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// First failed input conversion in authored recipe order; no partial normalized recipe escapes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormulaRecipeLiteralError {
    pub(super) statement_index: usize,
    pub(super) error: Box<FormulaStatementLiteralError>,
}
impl FormulaRecipeLiteralError {
    /// Known one-based ordinal in an already accepted syntax recipe.
    #[must_use]
    pub const fn statement_index(&self) -> usize {
        self.statement_index
    }
    /// Original operand role and literal refusal.
    #[must_use]
    pub fn statement_error(&self) -> FormulaStatementLiteralError {
        *self.error
    }
    /// Exact offending literal span in the complete source.
    #[must_use]
    pub fn span(&self) -> Span {
        self.error.span()
    }
    /// Stable formula-domain family.
    #[must_use]
    pub fn diagnostic_code(&self) -> &'static str {
        self.error.diagnostic_code()
    }
}
impl fmt::Display for FormulaRecipeLiteralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, statement {}", self.error, self.statement_index)
    }
}
impl std::error::Error for FormulaRecipeLiteralError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.error.as_ref())
    }
}

#[derive(Clone)]
pub(super) enum NormalizedStatementData<'a> {
    Let(FormulaBindingKind, FormulaNormalizedExpression<'a>),
    Assert(
        FormulaToleranceName,
        FormulaNormalizedExpression<'a>,
        FormulaNormalizedExpression<'a>,
    ),
}

/// Privately constructed normalized statement, owning arenas and borrowing original source.
/// ```compile_fail
/// fn mutate(s: &mut sc_core::recipe::FormulaNormalizedStatement<'_>) { s.name = "other"; }
/// ```
/// ```compile_fail
/// fn detached() -> sc_core::recipe::FormulaNormalizedStatement<'static> {
///     let source = String::from("let width: length = 25 mm");
///     sc_core::recipe::FormulaStatement::parse(&source).unwrap().normalize_literals().unwrap()
/// }
/// ```
#[derive(Clone)]
pub struct FormulaNormalizedStatement<'a> {
    pub(super) span: Span,
    pub(super) name_span: Span,
    pub(super) annotation_span: Span,
    pub(super) name: &'a str,
    pub(super) data: NormalizedStatementData<'a>,
}
impl fmt::Debug for FormulaNormalizedStatement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNormalizedStatement")
            .field("span", &self.span)
            .field("kind", &self.kind())
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaNormalizedStatement<'a> {
    /// Original statement span, excluding its outer whitespace.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }
    /// Explicit customer-bearing name; normalization grants no namespace authority.
    #[must_use]
    pub const fn name(&self) -> &'a str {
        self.name
    }
    /// Original name location.
    #[must_use]
    pub const fn name_span(&self) -> Span {
        self.name_span
    }
    /// Original declared kind or symbolic tolerance location.
    #[must_use]
    pub const fn annotation_span(&self) -> Span {
        self.annotation_span
    }
    /// Immutable role/operand view, tied to the owner of the normalized arenas.
    /// ```compile_fail
    /// fn escaped(source: &str) -> sc_core::recipe::FormulaNormalizedStatementKind<'_> {
    ///     let syntax = sc_core::recipe::FormulaStatement::parse(source).unwrap();
    ///     let normalized = syntax.normalize_literals().unwrap();
    ///     normalized.kind()
    /// }
    /// ```
    #[must_use]
    pub fn kind(&self) -> FormulaNormalizedStatementKind<'_> {
        match &self.data {
            NormalizedStatementData::Let(kind, expression) => FormulaNormalizedStatementKind::Let {
                declared_kind: *kind,
                expression,
            },
            NormalizedStatementData::Assert(tolerance, left, right) => {
                FormulaNormalizedStatementKind::Assert {
                    tolerance: *tolerance,
                    left,
                    right,
                }
            }
        }
    }
}

/// Read-only normalized operands; annotations are preserved without dimensional inference.
#[derive(Clone, Copy, Debug)]
pub enum FormulaNormalizedStatementKind<'a> {
    /// Binding input, without evaluating or narrowing to the declared bound kind.
    Let {
        /// Authored binding kind.
        declared_kind: FormulaBindingKind,
        /// Every input literal converted; all operators remain unevaluated.
        expression: &'a FormulaNormalizedExpression<'a>,
    },
    /// Assertion inputs, without resolving or executing the tolerance comparison.
    Assert {
        /// Authored symbolic tolerance class.
        tolerance: FormulaToleranceName,
        /// Left input arena.
        left: &'a FormulaNormalizedExpression<'a>,
        /// Right input arena.
        right: &'a FormulaNormalizedExpression<'a>,
    },
}

/// Privately constructed complete normalized recipe in authored statement order.
/// ```compile_fail
/// fn clear(r: &mut sc_core::recipe::FormulaNormalizedRecipe<'_>) { r.statements.clear(); }
/// ```
/// ```compile_fail
/// fn detached() -> sc_core::recipe::FormulaNormalizedRecipe<'static> {
///     let source = String::from("let width: length = 25 mm");
///     sc_core::recipe::FormulaRecipe::parse(&source).unwrap().normalize_literals().unwrap()
/// }
/// ```
#[derive(Clone)]
pub struct FormulaNormalizedRecipe<'a> {
    pub(super) statements: Vec<FormulaNormalizedStatement<'a>>,
}
impl fmt::Debug for FormulaNormalizedRecipe<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNormalizedRecipe")
            .field("statement_count", &self.statements.len())
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaNormalizedRecipe<'a> {
    /// Immutable authored order; the slice and every operand view borrow this arena owner.
    /// ```compile_fail
    /// fn escaped(source: &str) -> &[sc_core::recipe::FormulaNormalizedStatement<'_>] {
    ///     let syntax = sc_core::recipe::FormulaRecipe::parse(source).unwrap();
    ///     let normalized = syntax.normalize_literals().unwrap();
    ///     normalized.statements()
    /// }
    /// ```
    #[must_use]
    pub fn statements(&self) -> &[FormulaNormalizedStatement<'a>] {
        &self.statements
    }
}
