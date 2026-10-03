//! Formula syntax/literal normalization and later ordered recipe validation/evaluation.
//!
//! The lexical stream and immutable expression trees borrow machine-form source with precise spans.
//! Expression parsing checks grammar and node/conditional limits, not numeric conversion,
//! canonical identity, statements, type/name validity or execution. Literal conversion is explicit
//! on parsed nodes or a separate whole normalized arena; neither evaluates operators. Normalized
//! expressions can produce owned canonical identity bytes without name/type validation or execution.
//! Standalone let/assert statements preserve closed annotations, full-source spans and operand
//! refusals without binding or evaluating. Complete recipes retain authored order, global spans and
//! the fixed statement bound. Whole statement/recipe input normalization preserves metadata and
//! contextual literal refusals. Owned expression/statement/recipe bytes preserve typed identity;
//! Closed kind/origin/reserved-context metadata and immutable source locators are available;
//! initial namespaces retain sources, reject collisions and resolve exact names; ordered metadata scopes
//! retain actual prior let declarations; closed operator kind signatures are available.
//! Function/expression/whole-recipe type validation and evaluation follow.
mod lexer;
pub use lexer::{
    FormulaLexeme, FormulaLexemeKind, FormulaLexer, FormulaLexicalError, FormulaLexicalRule,
    FormulaSourceSpan,
};

mod expression;
mod parser;
pub use expression::{
    FormulaArguments, FormulaBinaryOperator, FormulaExpression, FormulaExpressionLimit,
    FormulaNode, FormulaNodeKind, FormulaParseError, FormulaParseRule, FormulaUnit,
};

mod literal;
pub use literal::{
    FormulaLiteral, FormulaLiteralError, FormulaLiteralKind, FormulaLiteralRule,
    FormulaRationalComponent,
};

mod normalized;
pub use normalized::{
    FormulaNormalizedArguments, FormulaNormalizedExpression, FormulaNormalizedNode,
    FormulaNormalizedNodeKind,
};

mod canonical;
pub use canonical::FormulaCanonicalExpression;

mod statement;
pub use statement::{
    FormulaBindingKind, FormulaStatement, FormulaStatementError, FormulaStatementExpression,
    FormulaStatementKind, FormulaStatementRule, FormulaToleranceName,
};

mod ordered;
pub use ordered::{FormulaRecipe, FormulaRecipeError, FormulaRecipeRule};

mod normalized_recipe;
pub use normalized_recipe::{
    FormulaNormalizedRecipe, FormulaNormalizedStatement, FormulaNormalizedStatementKind,
    FormulaRecipeLiteralError, FormulaStatementLiteralError,
};

mod canonical_recipe;
pub use canonical_recipe::{FormulaCanonicalRecipe, FormulaCanonicalStatement};

mod semantic;
pub use semantic::{FormulaKind, FormulaOrigin, FormulaReservedContext, FormulaReservedName};

mod operators;
pub use operators::FormulaUnaryOperator;

mod declaration;
pub use declaration::{
    FormulaDeclaration, FormulaDeclarationSource, FormulaInputOrigin, FormulaScalarInputOrigin,
};

mod namespace;
pub use namespace::{
    FormulaInitialDeclaration, FormulaNameCursor, FormulaNamespace, FormulaNamespaceError,
    FormulaStatementNameScope, FormulaUnboundName,
};
