//! Typed owned statement/recipe identity; serialization does not bind or execute formulas.
use super::{FormulaNormalizedRecipe, FormulaNormalizedStatement, FormulaNormalizedStatementKind};
use core::fmt;

/// Owned exact statement bytes, produced only from a successfully normalized statement.
///
/// Equality compares identity bytes, including the name, annotation and ordered operands.
/// Debug omits customer-bearing text. This is a distinct identity domain from an expression.
/// ```compile_fail
/// fn forge() -> sc_core::recipe::FormulaCanonicalStatement {
///     sc_core::recipe::FormulaCanonicalStatement { text: String::from("(bind n count count:1)") }
/// }
/// ```
/// ```compile_fail
/// fn compare(expression: sc_core::recipe::FormulaCanonicalExpression,
///            statement: sc_core::recipe::FormulaCanonicalStatement) -> bool {
///     expression == statement
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct FormulaCanonicalStatement {
    text: String,
}
impl FormulaCanonicalStatement {
    /// Explicit access to customer-bearing ASCII identity bytes, without a terminal newline.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Transfer the owned bytes; edits to the returned string cannot change another identity.
    #[must_use]
    pub fn into_string(self) -> String {
        self.text
    }
}
impl fmt::Debug for FormulaCanonicalStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaCanonicalStatement")
            .field("byte_count", &self.text.len())
            .finish_non_exhaustive()
    }
}

/// Owned exact bytes of a complete recipe, retaining authored statement order and empty identity.
///
/// Equality compares bytes, never evaluated results. A recipe is a distinct identity domain even
/// when an ordinary expression call happens to serialize to identical text. Typed project fields
/// and digest framing belong to persistence; these bytes define neither a reader nor a hash scheme.
/// ```compile_fail
/// fn forge() -> sc_core::recipe::FormulaCanonicalRecipe {
///     sc_core::recipe::FormulaCanonicalRecipe { text: String::from("(recipe)") }
/// }
/// ```
/// ```compile_fail
/// fn compare(expression: sc_core::recipe::FormulaCanonicalExpression,
///            recipe: sc_core::recipe::FormulaCanonicalRecipe) -> bool {
///     expression == recipe
/// }
/// ```
/// ```compile_fail
/// fn compare(statement: sc_core::recipe::FormulaCanonicalStatement,
///            recipe: sc_core::recipe::FormulaCanonicalRecipe) -> bool {
///     statement == recipe
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct FormulaCanonicalRecipe {
    text: String,
}
impl FormulaCanonicalRecipe {
    /// Explicit access to customer-bearing ASCII identity bytes, without a terminal newline.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Transfer the owned bytes; edits to the returned string cannot change another identity.
    #[must_use]
    pub fn into_string(self) -> String {
        self.text
    }
}
impl fmt::Debug for FormulaCanonicalRecipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaCanonicalRecipe")
            .field("byte_count", &self.text.len())
            .finish_non_exhaustive()
    }
}

impl FormulaNormalizedStatement<'_> {
    /// Serialize the exact canonical statement byte contract without binding or execution.
    ///
    /// The result owns its text and can outlive the source and every syntax/normalized arena.
    /// Declared kinds and symbolic tolerances are preserved without inference or resolution.
    /// ```
    /// use sc_core::recipe::FormulaStatement;
    /// let identity = {
    ///     let source = String::from("let width: length = 2.5 cm");
    ///     FormulaStatement::parse(&source)?.normalize_literals()?.canonical_form()
    /// };
    /// assert_eq!(identity.as_str(), "(bind width length length:25000)");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn canonical_form(&self) -> FormulaCanonicalStatement {
        let mut text = String::new();
        append_statement(&mut text, self);
        FormulaCanonicalStatement { text }
    }
}

impl FormulaNormalizedRecipe<'_> {
    /// Serialize the complete authored sequence; an empty recipe produces `(recipe)`.
    ///
    /// Flat iteration reuses the existing iterative expression serializer. No sorting, merging,
    /// source spans, numerical evaluation or project envelope are introduced.
    /// ```
    /// use sc_core::recipe::FormulaRecipe;
    /// let identity = {
    ///     let source = String::from("let n: count = 1\nassert check: eps_num = n == 1");
    ///     FormulaRecipe::parse(&source)?.normalize_literals()?.canonical_form()
    /// };
    /// assert_eq!(identity.as_str(),
    ///     "(recipe (bind n count count:1) (assert check eps_num n count:1))");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn canonical_form(&self) -> FormulaCanonicalRecipe {
        let mut text = String::from("(recipe");
        for statement in self.statements() {
            text.push(' ');
            append_statement(&mut text, statement);
        }
        text.push(')');
        FormulaCanonicalRecipe { text }
    }
}

fn append_statement(text: &mut String, statement: &FormulaNormalizedStatement<'_>) {
    match statement.kind() {
        FormulaNormalizedStatementKind::Let {
            declared_kind,
            expression,
        } => {
            text.push_str("(bind ");
            text.push_str(statement.name());
            text.push(' ');
            text.push_str(declared_kind.token());
            text.push(' ');
            text.push_str(expression.canonical_form().as_str());
        }
        FormulaNormalizedStatementKind::Assert {
            tolerance,
            left,
            right,
        } => {
            text.push_str("(assert ");
            text.push_str(statement.name());
            text.push(' ');
            text.push_str(tolerance.token());
            text.push(' ');
            text.push_str(left.canonical_form().as_str());
            text.push(' ');
            text.push_str(right.canonical_form().as_str());
        }
    }
    text.push(')');
}
