//! Owned original recipe text and identity; borrowed normalized views retain original locations.
use core::fmt;

use super::{
    FormulaCanonicalRecipe, FormulaNormalizedRecipe, FormulaRecipe, FormulaRecipeError,
    FormulaRecipeLiteralError,
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
}
impl fmt::Debug for FormulaRecipeDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaRecipeDocument")
            .field("source_byte_count", &self.source.len())
            .field("canonical_byte_count", &self.canonical.as_str().len())
            .finish_non_exhaustive()
    }
}
impl FormulaRecipeDocument {
    /// Own exact authored source after complete syntax and input-literal acceptance.
    ///
    /// Reuses the existing ordered parser, conversion, limits and identity serializer. Source is
    /// never replaced with canonical text: that would discard formatting and original spans.
    /// Empty/whitespace-only drafts accept. No statement, id, value or history is invented.
    ///
    /// # Errors
    /// Returns the original syntax refusal first, then the original ordered literal refusal.
    pub fn new(source: String) -> Result<Self, FormulaRecipeDocumentError> {
        let canonical = normalized(&source)?.canonical_form();
        Ok(Self { source, canonical })
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
