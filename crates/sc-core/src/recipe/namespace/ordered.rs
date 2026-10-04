//! Ordered declaration metadata; a cursor never grants expression or whole-recipe acceptance.
use super::{
    FormulaDeclaration, FormulaDeclarationSource, FormulaNamespace, FormulaNamespaceError,
};
use crate::{
    name::MachineToken,
    recipe::{FormulaNormalizedRecipe, FormulaNormalizedStatement, FormulaUnboundName},
};
use core::fmt;

/// Metadata staging cursor over one actual normalized recipe in its authored order.
/// Only initial names and earlier let annotations are visible. Advancing performs no expression
/// checking or execution and produces no accepted recipe or final initial namespace.
/// ```compile_fail
/// fn skip(c: &mut sc_core::recipe::FormulaNameCursor<'_>) { c.position = 100; }
/// ```
/// ```compile_fail
/// fn initial(c: sc_core::recipe::FormulaNameCursor<'_>) -> sc_core::recipe::FormulaNamespace<'_> {
///     c.namespace
/// }
/// ```
/// ```compile_fail
/// fn escaped(source: &str) -> sc_core::recipe::FormulaNameCursor<'_> {
///     let recipe = sc_core::recipe::FormulaRecipe::parse(source).unwrap().normalize_literals().unwrap();
///     sc_core::recipe::FormulaNameCursor::new(sc_core::recipe::FormulaNamespace::new([]).unwrap(), &recipe)
/// }
/// ```
pub struct FormulaNameCursor<'a> {
    namespace: FormulaNamespace<'a>,
    recipe: &'a FormulaNormalizedRecipe<'a>,
    position: usize,
}
impl<'a> FormulaNameCursor<'a> {
    /// Begin before statement one with a checked initial namespace and one immutable recipe owner.
    #[must_use]
    pub const fn new(
        namespace: FormulaNamespace<'a>,
        recipe: &'a FormulaNormalizedRecipe<'a>,
    ) -> Self {
        Self {
            namespace,
            recipe,
            position: 0,
        }
    }

    /// Current actual statement and its prior-name scope, or None at the end.
    /// # Errors
    /// Refuses a colliding let header before exposing its scope. Both binding sources are retained.
    pub fn current(
        &self,
    ) -> Result<Option<FormulaStatementNameScope<'_, 'a>>, FormulaNamespaceError<'a>> {
        let Some(statement) = self.recipe.statements().get(self.position) else {
            return Ok(None);
        };
        // Position starts at zero and advances only while an actual statement exists (at most4096).
        let statement_index = self.position + 1;
        if let Some(attempted) = FormulaDeclaration::recipe(self.recipe, statement_index) {
            if let Some(prior) = self.namespace.entries.get(attempted.name()) {
                return Err(match prior.source() {
                    FormulaDeclarationSource::Reserved(reserved) => {
                        FormulaNamespaceError::ReservedBinding {
                            reserved,
                            attempted,
                        }
                    }
                    FormulaDeclarationSource::Recipe { .. } => {
                        FormulaNamespaceError::RecipeRebinding {
                            sources: Box::new([*prior, attempted]),
                        }
                    }
                    _ => FormulaNamespaceError::AmbiguousName {
                        sources: Box::new([*prior, attempted]),
                    },
                });
            }
        }
        Ok(Some(FormulaStatementNameScope {
            namespace: &self.namespace,
            statement,
            statement_index,
        }))
    }

    /// Record only the current let's authored metadata, then advance one actual statement.
    /// Returns false at the fused end. Assertion labels introduce no declaration.
    /// Expression/type/whole-recipe validators must check every operand before advancing this stage.
    /// # Errors
    /// A colliding let refuses without changing the position or prior namespace; it cannot be skipped.
    pub fn advance_metadata(&mut self) -> Result<bool, FormulaNamespaceError<'a>> {
        let Some(current) = self.current()? else {
            return Ok(false);
        };
        let statement_index = current.statement_index;
        if let Some(declaration) = FormulaDeclaration::recipe(self.recipe, statement_index) {
            self.namespace
                .entries
                .insert(declaration.name(), declaration);
        }
        self.position += 1;
        Ok(true)
    }
}
impl fmt::Debug for FormulaNameCursor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNameCursor")
            .field("position", &self.position)
            .finish_non_exhaustive()
    }
}

/// Borrowed current statement and visible prior declaration metadata.
/// A view cannot escape its cursor or survive a metadata advance.
/// ```compile_fail
/// fn detach(source: &str) -> sc_core::recipe::FormulaStatementNameScope<'_, '_> {
///     let recipe = sc_core::recipe::FormulaRecipe::parse(source).unwrap().normalize_literals().unwrap();
///     let cursor = sc_core::recipe::FormulaNameCursor::new(sc_core::recipe::FormulaNamespace::new([]).unwrap(), &recipe);
///     cursor.current().unwrap().unwrap()
/// }
/// ```
/// ```compile_fail
/// use sc_core::recipe::{FormulaNameCursor, FormulaNamespace, FormulaRecipe};
/// let recipe = FormulaRecipe::parse("let first:length=1 cm").unwrap().normalize_literals().unwrap();
/// let mut cursor = FormulaNameCursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
/// let scope = cursor.current().unwrap().unwrap();
/// cursor.advance_metadata().unwrap();
/// let _ = scope.statement();
/// ```
#[derive(Clone, Copy)]
pub struct FormulaStatementNameScope<'s, 'a> {
    namespace: &'s FormulaNamespace<'a>,
    statement: &'a FormulaNormalizedStatement<'a>,
    statement_index: usize,
}
impl<'s, 'a> FormulaStatementNameScope<'s, 'a> {
    /// Exact one-based ordinal in this cursor's actual recipe owner, including assertion positions.
    #[must_use]
    pub const fn statement_index(self) -> usize {
        self.statement_index
    }

    /// Original normalized statement, with its global source/name/annotation spans and operands.
    #[must_use]
    pub const fn statement(self) -> &'a FormulaNormalizedStatement<'a> {
        self.statement
    }

    /// Check this actual statement against its initial and prior-let declaration metadata.
    /// Checks every operand before its annotation/comparison, without advancing the cursor.
    /// The result borrows the recipe and declaration owners, not the cursor allocation.
    /// Earlier annotations are metadata; this alone grants no whole-recipe or runtime acceptance.
    /// ```
    /// use sc_core::recipe::{FormulaNameCursor, FormulaNamespace, FormulaRecipe};
    /// let recipe = FormulaRecipe::parse("let width:length=1 mm").unwrap().normalize_literals().unwrap();
    /// let mut cursor = FormulaNameCursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
    /// let proof = cursor.current().unwrap().unwrap().check_kinds().unwrap();
    /// cursor.advance_metadata().unwrap();
    /// drop(cursor);
    /// assert_eq!(proof.statement_index(), 1);
    /// assert_eq!(proof.canonical_statement().as_str(), "(bind width length length:1000)");
    /// ```
    /// # Errors
    /// Retains the actual statement, ordinal, operand part and complete available refusal arguments.
    pub fn check_kinds(
        self,
    ) -> Result<
        crate::recipe::FormulaCheckedStatement<'a>,
        crate::recipe::FormulaStatementCheckError<'a>,
    > {
        crate::recipe::checked_statement::check_statement(
            self.statement,
            self.statement_index,
            self.namespace,
        )
    }

    /// Exact initial or earlier-let declaration metadata, independent of source value availability.
    /// # Errors
    /// An absent, self or forward name retains its exact query and complete searched domains.
    pub fn resolve<'n>(
        self,
        name: &'n MachineToken,
    ) -> Result<FormulaDeclaration<'a>, FormulaUnboundName<'n>> {
        self.resolve_name(name.as_str())
    }

    /// Exact parser-validated lookup for later expression checking inside the recipe module.
    pub(in crate::recipe) fn resolve_name<'n>(
        self,
        name: &'n str,
    ) -> Result<FormulaDeclaration<'a>, FormulaUnboundName<'n>> {
        self.namespace.resolve_name(name)
    }
}
impl fmt::Debug for FormulaStatementNameScope<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaStatementNameScope")
            .field("statement_index", &self.statement_index)
            .finish_non_exhaustive()
    }
}
