//! Formula syntax and, in later slices, ordered recipe validation/evaluation.
//!
//! The lexical stream borrows machine-form source and reports precise byte spans. Its success
//! certifies individual tokens only, not expression grammar, numeric/type/name validity or execution.
mod lexer;
pub use lexer::{
    FormulaLexeme, FormulaLexemeKind, FormulaLexer, FormulaLexicalError, FormulaLexicalRule,
    FormulaSourceSpan,
};
