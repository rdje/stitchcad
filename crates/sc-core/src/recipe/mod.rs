//! Formula syntax/literal normalization and later ordered recipe validation/evaluation.
//!
//! The lexical stream and immutable expression trees borrow machine-form source with precise spans.
//! Expression parsing checks grammar and node/conditional limits, not numeric conversion,
//! canonical identity, statements, type/name validity or execution. Literal conversion is explicit
//! on parsed nodes or a separate whole normalized arena; neither evaluates operators. Normalized
//! expressions can produce owned canonical identity bytes without name/type validation or execution.
//! Standalone let/assert statements preserve closed annotations, full-source spans and operand
//! refusals without binding or evaluating; ordered recipe composition and statement identity follow.
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
