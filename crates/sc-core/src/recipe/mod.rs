//! Formula syntax, individual literal conversion and later ordered recipe validation/evaluation.
//!
//! The lexical stream and immutable expression trees borrow machine-form source with precise spans.
//! Expression parsing checks grammar and node/conditional limits, not numeric conversion,
//! canonical identity, statements, type/name validity or execution. Individual literal conversion
//! is explicit on parsed node views and does not evaluate or normalize a whole expression.
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
