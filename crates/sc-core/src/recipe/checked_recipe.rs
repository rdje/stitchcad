//! Complete value-free recipe proof; no checked prefix escapes a later static refusal.
use super::{
    FormulaCanonicalRecipe, FormulaCanonicalStatement, FormulaCheckedStatement,
    FormulaCheckedStatementKind, FormulaDeclaration, FormulaNameCursor, FormulaNameDependency,
    FormulaNamespace, FormulaNamespaceError, FormulaNormalizedRecipe, FormulaNormalizedStatement,
    FormulaNormalizedStatementKind, FormulaReservedName, FormulaSourceSpan,
    FormulaStatementCheckError,
};
use core::fmt;

/// Static whole-recipe failure; the private enclosing error retains its real owner and position.
#[derive(Debug)]
pub enum FormulaRecipeCheckRefusal<'a> {
    /// Actual initial/reserved/prior-let collision; no expression was checked for this header.
    Namespace(FormulaNamespaceError<'a>),
    /// Actual operand, annotation or comparison failure from this statement's scope.
    Statement(FormulaStatementCheckError<'a>),
}
impl FormulaRecipeCheckRefusal<'_> {
    /// Existing diagnostic family, without authored source text or runtime values.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::Namespace(error) => error.token(),
            Self::Statement(error) => error.token(),
        }
    }
}

/// Privately constructed first refusal in one actual normalized recipe; no partial proof is exposed.
/// ```compile_fail
/// fn forge<'a>(recipe: &'a sc_core::recipe::FormulaNormalizedRecipe<'a>,
///     statement: &'a sc_core::recipe::FormulaNormalizedStatement<'a>,
///     refusal: Box<sc_core::recipe::FormulaRecipeCheckRefusal<'a>>)
///     -> sc_core::recipe::FormulaRecipeCheckError<'a> {
///     sc_core::recipe::FormulaRecipeCheckError { recipe, statement, statement_index: 1, refusal }
/// }
/// ```
/// ```compile_fail
/// fn escape(source: &str) -> sc_core::recipe::FormulaRecipeCheckError<'_> {
///     use sc_core::recipe::*;
///     let recipe = FormulaRecipe::parse(source).unwrap().normalize_literals().unwrap();
///     recipe.check_kinds(FormulaNamespace::new([]).unwrap()).unwrap_err()
/// }
/// ```
pub struct FormulaRecipeCheckError<'a> {
    recipe: &'a FormulaNormalizedRecipe<'a>,
    statement: &'a FormulaNormalizedStatement<'a>,
    statement_index: usize,
    refusal: Box<FormulaRecipeCheckRefusal<'a>>,
}
impl<'a> FormulaRecipeCheckError<'a> {
    /// Original complete normalized owner, including the statements following the refusal.
    #[must_use]
    pub const fn recipe(&self) -> &'a FormulaNormalizedRecipe<'a> {
        self.recipe
    }
    /// Exact original statement that refused; no synthetic operand or header is attached.
    #[must_use]
    pub const fn statement(&self) -> &'a FormulaNormalizedStatement<'a> {
        self.statement
    }
    /// Genuine one-based position in this recipe, including assertion positions.
    #[must_use]
    pub const fn statement_index(&self) -> usize {
        self.statement_index
    }
    /// First header collision or nested statement refusal, retaining its typed source arguments.
    #[must_use]
    pub fn refusal(&self) -> &FormulaRecipeCheckRefusal<'a> {
        self.refusal.as_ref()
    }
    /// Header collision uses the original binding name; other failures retain their existing span.
    #[must_use]
    pub fn span(&self) -> FormulaSourceSpan {
        match self.refusal.as_ref() {
            FormulaRecipeCheckRefusal::Namespace(_) => self.statement.name_span(),
            FormulaRecipeCheckRefusal::Statement(error) => error.span(),
        }
    }
    /// Stable existing diagnostic family, without a numerical or provider query.
    #[must_use]
    pub fn token(&self) -> &'static str {
        self.refusal.token()
    }
    /// Explicit owned identity of the actual refused statement, even after static rejection.
    #[must_use]
    pub fn canonical_statement(&self) -> FormulaCanonicalStatement {
        self.statement.canonical_form()
    }
    /// Explicit identity of the complete normalized source, without granting acceptance.
    #[must_use]
    pub fn canonical_recipe(&self) -> FormulaCanonicalRecipe {
        self.recipe.canonical_form()
    }
}
impl fmt::Debug for FormulaRecipeCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaRecipeCheckError")
            .field("token", &self.token())
            .field("statement_index", &self.statement_index)
            .field("span", &self.span())
            .finish_non_exhaustive()
    }
}
impl fmt::Display for FormulaRecipeCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}
impl std::error::Error for FormulaRecipeCheckError<'_> {}

/// Where a whole-recipe dependency occurs; assertion tolerance is header metadata, not an AST node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaRecipeDependencyRole {
    /// Name occurrence in a let's RHS.
    Binding,
    /// Original symbolic tolerance annotation of an assertion.
    AssertionTolerance,
    /// Name occurrence in the left assertion operand.
    AssertionLeft,
    /// Name occurrence in the right assertion operand.
    AssertionRight,
}

/// One immutable dependency occurrence, streaming from a privately certified whole recipe.
/// Repetitions and untaken branches retain distinct original spans and actual declaration sources.
#[derive(Clone, Copy, Debug)]
pub struct FormulaRecipeNameDependency<'a> {
    statement_index: usize,
    role: FormulaRecipeDependencyRole,
    span: FormulaSourceSpan,
    declaration: FormulaDeclaration<'a>,
}
impl<'a> FormulaRecipeNameDependency<'a> {
    /// Genuine consumer position, independent of a supplier's optional recipe ordinal.
    #[must_use]
    pub const fn statement_index(self) -> usize {
        self.statement_index
    }
    /// Original header or operand location, without inventing a combined expression.
    #[must_use]
    pub const fn role(self) -> FormulaRecipeDependencyRole {
        self.role
    }
    /// Original use location; grouping spans come from the existing expression proof.
    #[must_use]
    pub const fn span(self) -> FormulaSourceSpan {
        self.span
    }
    /// Actual immutable supplier metadata; canonical input state is neither copied nor queried.
    #[must_use]
    pub const fn declaration(self) -> FormulaDeclaration<'a> {
        self.declaration
    }
}

/// Immutable complete static proof; it grants no numeric result, geometry or runtime readiness.
/// Only this privately constructed owner certifies that every prior binding was checked.
/// ```compile_fail
/// fn forge<'a>(recipe: &'a sc_core::recipe::FormulaNormalizedRecipe<'a>)
///     -> sc_core::recipe::FormulaCheckedRecipe<'a> {
///     sc_core::recipe::FormulaCheckedRecipe { recipe, statements: vec![] }
/// }
/// ```
/// ```compile_fail
/// fn escape(source: &str) -> sc_core::recipe::FormulaCheckedRecipe<'_> {
///     use sc_core::recipe::*;
///     let recipe = FormulaRecipe::parse(source).unwrap().normalize_literals().unwrap();
///     recipe.check_kinds(FormulaNamespace::new([]).unwrap()).unwrap()
/// }
/// ```
/// ```compile_fail
/// fn escape_record<'a>(recipe: &'a sc_core::recipe::FormulaNormalizedRecipe<'a>,
///     name: &'a sc_core::name::MachineToken) -> sc_core::recipe::FormulaCheckedRecipe<'a> {
///     use sc_core::{ontology::EntityId, recipe::*, value::*};
///     let record = LengthDeclaration::new(LengthDeclarationDefinition {
///         id: EntityId::from_bits(1), source: EntityId::from_bits(2),
///         state: LengthState::Unknown { observation: EntityId::from_bits(3) },
///     }).unwrap();
///     let input = FormulaDeclaration::length_input(name, FormulaInputOrigin::Measurement,
///         EntityId::from_bits(4), &record);
///     let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(input).unwrap()]).unwrap();
///     recipe.check_kinds(namespace).unwrap()
/// }
/// ```
pub struct FormulaCheckedRecipe<'a> {
    recipe: &'a FormulaNormalizedRecipe<'a>,
    statements: Vec<FormulaCheckedStatement<'a>>,
}
impl<'a> FormulaCheckedRecipe<'a> {
    /// Exact complete normalized owner certified by this proof.
    #[must_use]
    pub const fn recipe(&self) -> &'a FormulaNormalizedRecipe<'a> {
        self.recipe
    }
    /// Complete checked statements in authored order; no mutable or substitutable view is exposed.
    #[must_use]
    pub fn statements(&self) -> &[FormulaCheckedStatement<'a>] {
        &self.statements
    }
    /// Every dependency in statement and source order, including assertion-header classes.
    /// Streams existing operand proofs without allocating a duplicate whole edge vector.
    pub fn dependencies(&self) -> impl Iterator<Item = FormulaRecipeNameDependency<'a>> + '_ {
        self.statements.iter().flat_map(statement_dependencies)
    }
    /// Explicit owned bytes from the same complete normalized recipe, without execution.
    #[must_use]
    pub fn canonical_recipe(&self) -> FormulaCanonicalRecipe {
        self.recipe.canonical_form()
    }
}
impl fmt::Debug for FormulaCheckedRecipe<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaCheckedRecipe")
            .field("statement_count", &self.statements.len())
            .finish_non_exhaustive()
    }
}

fn statement_dependencies<'g, 'a>(
    proof: &'g FormulaCheckedStatement<'a>,
) -> impl Iterator<Item = FormulaRecipeNameDependency<'a>> + 'g {
    use FormulaRecipeDependencyRole as R;
    let (tolerance, operands) = match proof.kind() {
        FormulaCheckedStatementKind::Let { expression } => (
            None,
            [
                (R::Binding, expression.dependencies()),
                (R::Binding, &[][..]),
            ],
        ),
        FormulaCheckedStatementKind::Assert { left, right } => {
            let FormulaNormalizedStatementKind::Assert { tolerance, .. } = proof.statement().kind()
            else {
                unreachable!("private statement proof must retain its actual role");
            };
            let header = FormulaRecipeNameDependency {
                statement_index: proof.statement_index(),
                role: R::AssertionTolerance,
                span: proof.statement().annotation_span(),
                declaration: FormulaDeclaration::reserved(FormulaReservedName::Tolerance(
                    tolerance,
                )),
            };
            (
                Some(header),
                [
                    (R::AssertionLeft, left.dependencies()),
                    (R::AssertionRight, right.dependencies()),
                ],
            )
        }
    };
    tolerance
        .into_iter()
        .chain(operands.into_iter().flat_map(move |(role, dependencies)| {
            dependencies
                .iter()
                .copied()
                .map(
                    move |dependency: FormulaNameDependency<'a>| FormulaRecipeNameDependency {
                        statement_index: proof.statement_index(),
                        role,
                        span: dependency.span(),
                        declaration: dependency.declaration(),
                    },
                )
        }))
}

impl<'a> FormulaNormalizedRecipe<'a> {
    /// Check every actual statement before advancing any of its metadata; accept only the whole recipe.
    /// The checked initial namespace is consumed into a private staging cursor. Clone it beforehand
    /// if the caller needs to reuse that immutable initial metadata allocation.
    /// ```
    /// use sc_core::recipe::{FormulaNamespace, FormulaRecipe};
    /// let recipe = FormulaRecipe::parse("let n:count=1\nassert check:eps_num=n==n")?
    ///     .normalize_literals()?;
    /// let proof = recipe.check_kinds(FormulaNamespace::new([]).unwrap()).unwrap();
    /// assert_eq!(proof.statements().len(), 2);
    /// assert_eq!(proof.dependencies().count(), 3);
    /// assert_eq!(proof.canonical_recipe(), recipe.canonical_form());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// First header/name/kind failure retains this recipe's genuine owner, ordinal and nested source.
    /// No prior checked prefix or final namespace escapes; no values or providers are read.
    pub fn check_kinds(
        &'a self,
        namespace: FormulaNamespace<'a>,
    ) -> Result<FormulaCheckedRecipe<'a>, FormulaRecipeCheckError<'a>> {
        let mut cursor = FormulaNameCursor::new(namespace, self);
        let mut statements = Vec::with_capacity(self.statements().len());
        for (position, statement) in self.statements().iter().enumerate() {
            let statement_index = position + 1;
            let refuse = |refusal| FormulaRecipeCheckError {
                recipe: self,
                statement,
                statement_index,
                refusal: Box::new(refusal),
            };
            let Some(scope) = cursor
                .current()
                .map_err(|error| refuse(FormulaRecipeCheckRefusal::Namespace(error)))?
            else {
                unreachable!("private cursor has the next actual recipe statement");
            };
            let proof = scope
                .check_kinds()
                .map_err(|error| refuse(FormulaRecipeCheckRefusal::Statement(error)))?;
            cursor
                .advance_metadata()
                .map_err(|error| refuse(FormulaRecipeCheckRefusal::Namespace(error)))?;
            statements.push(proof);
        }
        Ok(FormulaCheckedRecipe {
            recipe: self,
            statements,
        })
    }
}
