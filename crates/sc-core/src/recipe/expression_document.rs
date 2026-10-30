//! Immutable original expression text and canonical syntax, without source-entity or value claims.
use super::{
    FormulaCanonicalExpression, FormulaExpression, FormulaLiteralError,
    FormulaNormalizedExpression, FormulaParseError,
};
use core::fmt;

/// Original complete syntax or input-literal refusal; no partial document is returned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormulaExpressionDocumentError {
    /// Existing expression parser rule and original source span.
    Syntax(FormulaParseError),
    /// Existing literal width/domain rule and actual original literal span.
    Literal(FormulaLiteralError),
}
impl fmt::Display for FormulaExpressionDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(error) => error.fmt(f),
            Self::Literal(error) => error.fmt(f),
        }
    }
}
impl std::error::Error for FormulaExpressionDocumentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Syntax(error) => Some(error),
            Self::Literal(error) => Some(error),
        }
    }
}

/// Owned exact expression source and its existing typed canonical syntax identity.
/// Equality includes original authored spelling/whitespace even when canonical identities agree.
/// No let/header/name, entity id, namespace/kind proof, graph, value or execution is manufactured.
/// Original source survives independently of the caller's buffer; derived views borrow this owner.
/// ```compile_fail
/// fn rewrite(document: &mut sc_core::recipe::FormulaExpressionDocument) {
///     document.source.clear();
/// }
/// ```
/// ```compile_fail
/// fn detached() -> sc_core::recipe::FormulaNormalizedExpression<'static> {
///     let document = sc_core::recipe::FormulaExpressionDocument::new("2.5 cm".into()).unwrap();
///     document.normalized().unwrap()
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct FormulaExpressionDocument {
    source: String,
    canonical: FormulaCanonicalExpression,
    node_count: usize,
}
impl fmt::Debug for FormulaExpressionDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaExpressionDocument")
            .field("source_byte_count", &self.source.len())
            .field("canonical_byte_count", &self.canonical.as_str().len())
            .field("node_count", &self.node_count)
            .finish_non_exhaustive()
    }
}
impl FormulaExpressionDocument {
    /// Own original text after existing complete parsing and input-literal conversion.
    /// Operators remain unevaluated; all branch literals still pass normalization.
    /// An empty source is not an expression and retains its original syntax refusal.
    /// ```
    /// use sc_core::recipe::FormulaExpressionDocument;
    /// let document = FormulaExpressionDocument::new(" (2.5 cm + eps_num) ".into())?;
    /// let view = document.normalized()?;
    /// assert_eq!(&view.canonical_form(), document.canonical());
    /// let span = view.root().span();
    /// assert_eq!(document.source().get(span.start()..span.end()), Some("(2.5 cm + eps_num)"));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Original syntax failure precedes literal refusal. Neither publishes partial source identity.
    pub fn new(source: String) -> Result<Self, FormulaExpressionDocumentError> {
        let (canonical, node_count) = {
            let expression = normalized(&source)?;
            (expression.canonical_form(), expression.node_count())
        };
        Ok(Self {
            source,
            canonical,
            node_count,
        })
    }
    /// Exact original authored source, deliberately exposed without rewriting to canonical text.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }
    /// Existing typed expression identity; this supplies no stable entity or result identity.
    #[must_use]
    pub const fn canonical(&self) -> &FormulaCanonicalExpression {
        &self.canonical
    }
    /// Exact semantic node count from the original successful parser/converter, including all
    /// branches and arguments. Immutable factory metadata enables preallocation admission;
    /// it is not a caller annotation, name/kind proof, allocator byte count or evaluated state.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }
    /// Reuse the original parser/converter to derive immutable arenas borrowing actual source.
    /// No cached/self-referential arena or spans reconstructed from identity bytes are stored.
    /// # Errors
    /// Propagates the same original parser/literal causes as construction, without fallback.
    pub fn normalized(
        &self,
    ) -> Result<FormulaNormalizedExpression<'_>, FormulaExpressionDocumentError> {
        normalized(&self.source)
    }
}
fn normalized(
    source: &str,
) -> Result<FormulaNormalizedExpression<'_>, FormulaExpressionDocumentError> {
    FormulaExpression::parse(source)
        .map_err(FormulaExpressionDocumentError::Syntax)?
        .normalize_literals()
        .map_err(FormulaExpressionDocumentError::Literal)
}
