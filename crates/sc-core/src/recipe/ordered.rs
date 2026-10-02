//! Whole ordered formula syntax; syntax acceptance grants no execution or storage authority.
use super::{
    statement, FormulaLexemeKind as K, FormulaLexer, FormulaLexicalRule, FormulaSourceSpan as Span,
    FormulaStatement, FormulaStatementError, FormulaStatementRule,
};
use core::fmt;

/// Whole-recipe refusal, preserving a nested statement rule or the measured statement bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaRecipeRule {
    /// Precise original-source statement refusal, including its expression role where known.
    Statement(FormulaStatementError),
    /// First recognized statement beyond the fixed language bound.
    StatementLimit {
        /// Normative maximum; callers cannot widen it.
        bound: usize,
        /// First excess statement count, rather than an invented total for unparsed input.
        measured: usize,
    },
}

/// Whole-recipe error without a partially accepted recipe or customer-bearing diagnostic text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaRecipeError {
    statement_index: Option<usize>,
    span: Span,
    rule: FormulaRecipeRule,
}
impl FormulaRecipeError {
    /// One-based statement index. Global ASCII preflight occurs before boundaries are known.
    #[must_use]
    pub const fn statement_index(self) -> Option<usize> {
        self.statement_index
    }
    /// Offending span in the original complete recipe source.
    #[must_use]
    pub const fn span(self) -> Span {
        self.span
    }
    /// Typed nested syntax refusal or measured recipe bound.
    #[must_use]
    pub const fn rule(self) -> FormulaRecipeRule {
        self.rule
    }
    /// Stable formula diagnostic family; rendering/localization belongs to the command layer.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        match self.rule {
            FormulaRecipeRule::Statement(error) => error.diagnostic_code(),
            FormulaRecipeRule::StatementLimit { .. } => "formula_domain",
        }
    }
}
impl fmt::Display for FormulaRecipeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?}, statement {:?}, at bytes {}..{}",
            self.diagnostic_code(),
            self.rule,
            self.statement_index,
            self.span.start(),
            self.span.end()
        )
    }
}
impl std::error::Error for FormulaRecipeError {}

/// Privately constructed ordered statement arenas borrowing the original machine source.
/// ```compile_fail
/// fn clear(r: &mut sc_core::recipe::FormulaRecipe<'_>) { r.statements.clear(); }
/// ```
/// ```compile_fail
/// fn detached() -> sc_core::recipe::FormulaRecipe<'static> {
///     let source = String::from("let waist: length = 25 mm");
///     sc_core::recipe::FormulaRecipe::parse(&source).unwrap()
/// }
/// ```
#[derive(Clone)]
pub struct FormulaRecipe<'a> {
    statements: Vec<FormulaStatement<'a>>,
}
impl fmt::Debug for FormulaRecipe<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaRecipe")
            .field("statement_count", &self.statements.len())
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaRecipe<'a> {
    /// Language contract §4.3 `max_recipe_statements`; reaching it is valid, exceeding it is refused.
    pub const MAX_STATEMENTS: usize = 4096;

    /// Parse the complete ordered machine source, including an empty recipe, without evaluation.
    /// Newlines are whitespace; top-level let/assert keywords after a header delimit statements.
    /// All headers, nodes and errors retain spans in this original whole source.
    /// ```
    /// use sc_core::recipe::FormulaRecipe;
    /// let recipe = FormulaRecipe::parse(
    ///     "let waist: length = waist_girth + ease_waist\n\
    ///      assert closure: eps_num = waist == target_waist"
    /// )?;
    /// assert_eq!(recipe.statements().len(), 2);
    /// assert_eq!(recipe.statements()[1].name(), "closure");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn parse(source: &'a str) -> Result<Self, FormulaRecipeError> {
        let mut lexer = FormulaLexer::new(source).peekable();
        let mut statements = Vec::new();
        loop {
            let index = statements.len() + 1;
            match lexer.peek() {
                Some(Ok(next))
                    if statements.len() == Self::MAX_STATEMENTS
                        && matches!(next.kind(), K::Let | K::Assert) =>
                {
                    return Err(FormulaRecipeError {
                        statement_index: Some(index),
                        span: next.span(),
                        rule: FormulaRecipeRule::StatementLimit {
                            bound: Self::MAX_STATEMENTS,
                            measured: index,
                        },
                    });
                }
                None => break,
                _ => {}
            }
            let statement = statement::parse_next(source, &mut lexer, true).map_err(|error| {
                let statement_index = if error.rule()
                    == FormulaStatementRule::Lexical(FormulaLexicalRule::MachineAscii)
                {
                    None
                } else {
                    Some(index)
                };
                FormulaRecipeError {
                    statement_index,
                    span: error.span(),
                    rule: FormulaRecipeRule::Statement(error),
                }
            })?;
            statements.push(statement);
        }
        Ok(Self { statements })
    }

    /// Immutable authored order. A view cannot outlive the recipe that owns its arenas.
    /// ```compile_fail
    /// fn escaped(source: &str) -> &[sc_core::recipe::FormulaStatement<'_>] {
    ///     let recipe = sc_core::recipe::FormulaRecipe::parse(source).unwrap();
    ///     recipe.statements()
    /// }
    /// ```
    #[must_use]
    pub fn statements(&self) -> &[FormulaStatement<'a>] {
        &self.statements
    }
}
