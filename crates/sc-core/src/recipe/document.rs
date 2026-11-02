//! Owned original recipe text and identity; borrowed normalized views retain original locations.
use core::fmt;

use super::{
    FormulaCanonicalRecipe, FormulaNormalizedRecipe, FormulaNormalizedStatementKind, FormulaRecipe,
    FormulaRecipeError, FormulaRecipeLiteralError,
};

/// Original syntax or input-literal refusal; no partially accepted document escapes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormulaRecipeDocumentError {
    /// Complete original-source syntax refusal, preserving its optional ordinal and typed rule.
    Syntax(FormulaRecipeError),
    /// Original statement/operand/literal context after complete syntax acceptance.
    Literal(FormulaRecipeLiteralError),
}
impl fmt::Display for FormulaRecipeDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(error) => fmt::Display::fmt(error, f),
            Self::Literal(error) => fmt::Display::fmt(error, f),
        }
    }
}
impl std::error::Error for FormulaRecipeDocumentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Syntax(error) => Some(error),
            Self::Literal(error) => Some(error),
        }
    }
}

/// Exact complete syntax population cached by the original successful recipe document factory.
/// Counts include all statements, both assertion operands and every conditional branch. This is
/// immutable source metadata, not a caller annotation, kind/value proof or allocator-byte budget.
/// ```compile_fail,E0451
/// let forged = sc_core::recipe::FormulaRecipeShape { statements: 1, expressions: 1, nodes: 1 };
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaRecipeShape {
    statements: usize,
    expressions: usize,
    nodes: usize,
}
impl FormulaRecipeShape {
    fn from_normalized(recipe: &FormulaNormalizedRecipe<'_>) -> Self {
        let mut expressions = 0;
        let mut nodes = 0;
        // Only the successful original document factory calls this. Existing language bounds
        // permit at most 4096 statements, two operands each and 256 nodes per operand: 2097152
        // nodes fit supported 32/64-bit usize. No new aggregate language cap is introduced.
        for statement in recipe.statements() {
            match statement.kind() {
                FormulaNormalizedStatementKind::Let { expression, .. } => {
                    expressions += 1;
                    nodes += expression.node_count();
                }
                FormulaNormalizedStatementKind::Assert { left, right, .. } => {
                    expressions += 2;
                    nodes += left.node_count() + right.node_count();
                }
            }
        }
        Self {
            statements: recipe.statements().len(),
            expressions,
            nodes,
        }
    }
    /// Complete actual statement count, including every assertion and repeated declaration.
    #[must_use]
    pub const fn statement_count(self) -> usize {
        self.statements
    }
    /// Complete actual operand count: one per binding and two per assertion.
    #[must_use]
    pub const fn expression_count(self) -> usize {
        self.expressions
    }
    /// Complete actual semantic nodes, including call arguments and every conditional branch.
    #[must_use]
    pub const fn node_count(self) -> usize {
        self.nodes
    }
}

/// Immutable original recipe source with its typed canonical syntax identity.
///
/// Equality compares exact authored text as well as identity: unit aliases or formatting changes
/// can have equal canonical identities while their documents remain distinct. This is syntax/input
/// ownership, not namespace, static-kind, source/evidence, numeric-execution or readiness proof.
/// Debug reports byte counts only; requesting source or canonical bytes exposes customer content.
///
/// ```compile_fail
/// fn rewrite(document: &mut sc_core::recipe::FormulaRecipeDocument) {
///     document.source.clear();
/// }
/// ```
/// ```compile_fail
/// fn detached() -> sc_core::recipe::FormulaNormalizedRecipe<'static> {
///     let document = sc_core::recipe::FormulaRecipeDocument::new(
///         String::from("let width: length = 25 mm")).unwrap();
///     document.normalized().unwrap()
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct FormulaRecipeDocument {
    source: String,
    canonical: FormulaCanonicalRecipe,
    shape: FormulaRecipeShape,
}
impl fmt::Debug for FormulaRecipeDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaRecipeDocument")
            .field("source_byte_count", &self.source.len())
            .field("canonical_byte_count", &self.canonical.as_str().len())
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
impl FormulaRecipeDocument {
    // Both held strings count; no parser, normalization or regenerated identity is needed.
    pub(crate) fn resource_texts(&self) -> (&str, &str) {
        let Self {
            source,
            canonical,
            shape,
        } = self;
        // Exhaustive owner and fixed Copy metadata views keep future dynamic fields accountable.
        let FormulaRecipeShape {
            statements: _,
            expressions: _,
            nodes: _,
        } = *shape;
        (source, canonical.resource_text())
    }

    /// Own exact authored source after complete syntax and input-literal acceptance.
    ///
    /// Reuses the existing ordered parser, conversion, limits and identity serializer. Source is
    /// never replaced with canonical text: that would discard formatting and original spans.
    /// Empty/whitespace-only drafts accept. No statement, id, value or history is invented.
    ///
    /// # Errors
    /// Returns the original syntax refusal first, then the original ordered literal refusal.
    pub fn new(source: String) -> Result<Self, FormulaRecipeDocumentError> {
        let (canonical, shape) = {
            let recipe = normalized(&source)?;
            (
                recipe.canonical_form(),
                FormulaRecipeShape::from_normalized(&recipe),
            )
        };
        Ok(Self {
            source,
            canonical,
            shape,
        })
    }

    /// Exact original authored text, including outer whitespace; explicit customer-data access.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Borrow the actual typed canonical identity; it is not a reader, hash or execution certificate.
    #[must_use]
    pub const fn canonical(&self) -> &FormulaCanonicalRecipe {
        &self.canonical
    }

    /// Original factory-derived complete population, with no reparse, normalization or value read.
    /// Empty recipes have zero counts; metadata alone grants no program or resource admission.
    /// ```
    /// use sc_core::recipe::FormulaRecipeDocument;
    /// let document = FormulaRecipeDocument::new(
    ///     "let width:length=if(1==2,1 mm,2 mm)\nassert closure:eps_geo=width==2 mm".into(),
    /// )?;
    /// let shape = document.shape();
    /// assert_eq!((shape.statement_count(), shape.expression_count(), shape.node_count()), (2, 3, 8));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn shape(&self) -> FormulaRecipeShape {
        self.shape
    }

    /// Derive actual normalized arenas borrowing this document's original text.
    ///
    /// This repeats the existing parser/converter rather than storing self-referential arenas or
    /// inventing source locations from identity bytes. Dropping a view does not change the document;
    /// a view cannot outlive it. No name/kind validation or operator computation is performed.
    ///
    /// # Errors
    /// Propagates the same typed parser/converter refusals as construction, without panic or fallback.
    pub fn normalized(&self) -> Result<FormulaNormalizedRecipe<'_>, FormulaRecipeDocumentError> {
        normalized(&self.source)
    }
}

fn normalized(source: &str) -> Result<FormulaNormalizedRecipe<'_>, FormulaRecipeDocumentError> {
    FormulaRecipe::parse(source)
        .map_err(FormulaRecipeDocumentError::Syntax)?
        .normalize_literals()
        .map_err(FormulaRecipeDocumentError::Literal)
}
