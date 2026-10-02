//! Formula syntax/literal normalization and later ordered recipe validation/evaluation.
//!
//! The lexical stream and immutable expression trees borrow machine-form source with precise spans.
//! Expression parsing checks grammar and node/conditional limits, not numeric conversion,
//! canonical identity, statements, type/name validity or execution. Literal conversion is explicit
//! on parsed nodes or a separate whole normalized arena; neither evaluates operators. Normalized
//! expressions can produce owned canonical identity bytes without name/type validation or execution.
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
