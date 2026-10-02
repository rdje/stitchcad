//! Borrowed lexical machine form, without numeric conversion or binding authority.
use crate::name::{grammar_keyword, valid_spelling, GrammarKeyword};
use core::fmt;
use std::{iter::FusedIterator, iter::Peekable, str::CharIndices};

/// Half-open byte positions within the exact source borrowed by a lexeme or lexer.
/// This is a source location, not a persistent topological/entity reference or canonical identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaSourceSpan {
    start: usize,
    end: usize,
}
impl FormulaSourceSpan {
    /// First byte, inclusive.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }
    /// Byte after the span, exclusive.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }
}

/// Lexical role only. Units, kinds, built-ins and reserved inputs remain identifiers here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaLexemeKind {
    /// Binding statement keyword.
    Let,
    /// Assertion statement keyword.
    Assert,
    /// Conditional special-form keyword.
    If,
    /// Shared lower-snake machine spelling; grants no binding or call authority.
    Identifier,
    /// Digits and an optional nonempty decimal fraction; no conversion or value implied.
    Number,
    /// `+`.
    Plus,
    /// `-`; unary versus binary is a parser distinction.
    Minus,
    /// `*`.
    Multiply,
    /// `/`.
    Divide,
    /// `^`; the parser must enforce the square-only contract.
    Power,
    /// `=` statement assignment.
    Assign,
    /// `==`.
    Equal,
    /// `!=`.
    NotEqual,
    /// `<`.
    Less,
    /// `<=`.
    LessEqual,
    /// `>`.
    Greater,
    /// `>=`.
    GreaterEqual,
    /// `:`.
    Colon,
    /// `,`.
    Comma,
    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
}

/// Immutable lexical role, original spelling and source span. Text is borrowed, never normalized.
/// ```compile_fail
/// fn retarget(token: &mut sc_core::recipe::FormulaLexeme<'_>) { token.text = "other"; }
/// ```
/// Borrowed text cannot outlive its source.
/// ```compile_fail
/// fn detached() -> sc_core::recipe::FormulaLexeme<'static> {
///     let source = String::from("waist_girth");
///     sc_core::recipe::FormulaLexer::new(&source).next().unwrap().unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaLexeme<'a> {
    kind: FormulaLexemeKind,
    text: &'a str,
    span: FormulaSourceSpan,
}
impl<'a> FormulaLexeme<'a> {
    /// Lexical role.
    #[must_use]
    pub const fn kind(self) -> FormulaLexemeKind {
        self.kind
    }
    /// Exact original source spelling.
    #[must_use]
    pub const fn text(self) -> &'a str {
        self.text
    }
    /// Location in the lexer source, not in an arbitrary replacement source.
    #[must_use]
    pub const fn span(self) -> FormulaSourceSpan {
        self.span
    }
}

/// Precise lexical rule refused, without embedding customer source in a diagnostic payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaLexicalRule {
    /// Machine syntax is ASCII; display glyphs and localized spellings are not parsed.
    MachineAscii,
    /// An identifier violates the shared lower-snake spelling.
    IdentifierSpelling,
    /// A decimal point has no required following digits.
    DecimalFraction,
    /// `!` is valid only as part of `!=`.
    ComparisonPair,
    /// A character is not part of the machine lexical alphabet.
    UnsupportedCharacter,
    /// Internal source-span invariant failed; no slice or fallback token is fabricated.
    SourceBoundary,
}

/// First lexical refusal. The later parser adds statement/canonical-expression context where available.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaLexicalError {
    span: FormulaSourceSpan,
    rule: FormulaLexicalRule,
}
impl FormulaLexicalError {
    /// Stable formula diagnostic family; localization belongs to the command/message layer.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        "formula_parse"
    }
    /// Full offending Unicode scalar, invalid identifier, decimal point or punctuation location.
    #[must_use]
    pub const fn span(self) -> FormulaSourceSpan {
        self.span
    }
    /// Refused lexical rule.
    #[must_use]
    pub const fn rule(self) -> FormulaLexicalRule {
        self.rule
    }
}
impl fmt::Display for FormulaLexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "formula machine syntax {:?} at bytes {}..{}",
            self.rule, self.span.start, self.span.end
        )
    }
}
impl std::error::Error for FormulaLexicalError {}

/// Borrowed lexical stream; first error and end both permanently stop iteration.
/// ASCII preflight occurs before any token is returned. No number/name string is cloned.
/// General ASCII whitespace is skipped; original span gaps let the parser require exactly one
/// space before a literal unit. Adjacent atoms, comments, unsupported calls/exponents, expression
/// limits and numeric/name/type rules remain the parser/checker's responsibility.
#[derive(Clone)]
pub struct FormulaLexer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    ascii_checked: bool,
    stopped: bool,
}
impl fmt::Debug for FormulaLexer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaLexer")
            .field("source_bytes", &self.source.len())
            .field("ascii_checked", &self.ascii_checked)
            .field("stopped", &self.stopped)
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaLexer<'a> {
    /// Borrow machine-form source. Construction itself makes no syntax/currentness claim.
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            ascii_checked: false,
            stopped: false,
        }
    }
    fn error(start: usize, end: usize, rule: FormulaLexicalRule) -> FormulaLexicalError {
        FormulaLexicalError {
            span: FormulaSourceSpan { start, end },
            rule,
        }
    }
    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) {
        while self.chars.peek().is_some_and(|(_, c)| predicate(*c)) {
            let _ = self.chars.next();
        }
    }
    fn consume_equals(&mut self) -> bool {
        if self.chars.peek().is_some_and(|(_, c)| *c == '=') {
            let _ = self.chars.next();
            true
        } else {
            false
        }
    }
    fn scan(&mut self) -> Result<Option<FormulaLexeme<'a>>, FormulaLexicalError> {
        if !self.ascii_checked {
            self.ascii_checked = true;
            if let Some((start, c)) = self.source.char_indices().find(|(_, c)| !c.is_ascii()) {
                return Err(Self::error(
                    start,
                    start + c.len_utf8(),
                    FormulaLexicalRule::MachineAscii,
                ));
            }
        }
        self.consume_while(|c| c.is_ascii_whitespace() || c == '\x0b');
        let Some((start, first)) = self.chars.next() else {
            return Ok(None);
        };
        let kind = if first.is_ascii_digit() {
            self.consume_while(|c| c.is_ascii_digit());
            if let Some(&(dot, '.')) = self.chars.peek() {
                let _ = self.chars.next();
                if !self.chars.peek().is_some_and(|(_, c)| c.is_ascii_digit()) {
                    return Err(Self::error(
                        dot,
                        dot + 1,
                        FormulaLexicalRule::DecimalFraction,
                    ));
                }
                self.consume_while(|c| c.is_ascii_digit());
            }
            FormulaLexemeKind::Number
        } else if first.is_ascii_alphabetic() || first == '_' {
            self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
            // Word spelling and keyword role are checked after obtaining the exact borrowed span.
            FormulaLexemeKind::Identifier
        } else {
            match first {
                '+' => FormulaLexemeKind::Plus,
                '-' => FormulaLexemeKind::Minus,
                '*' => FormulaLexemeKind::Multiply,
                '/' => FormulaLexemeKind::Divide,
                '^' => FormulaLexemeKind::Power,
                '=' if self.consume_equals() => FormulaLexemeKind::Equal,
                '=' => FormulaLexemeKind::Assign,
                '!' if self.consume_equals() => FormulaLexemeKind::NotEqual,
                '!' => {
                    return Err(Self::error(
                        start,
                        start + 1,
                        FormulaLexicalRule::ComparisonPair,
                    ))
                }
                '<' if self.consume_equals() => FormulaLexemeKind::LessEqual,
                '<' => FormulaLexemeKind::Less,
                '>' if self.consume_equals() => FormulaLexemeKind::GreaterEqual,
                '>' => FormulaLexemeKind::Greater,
                ':' => FormulaLexemeKind::Colon,
                ',' => FormulaLexemeKind::Comma,
                '(' => FormulaLexemeKind::LeftParen,
                ')' => FormulaLexemeKind::RightParen,
                _ => {
                    return Err(Self::error(
                        start,
                        start + 1,
                        FormulaLexicalRule::UnsupportedCharacter,
                    ))
                }
            }
        };
        let end = self
            .chars
            .peek()
            .map_or(self.source.len(), |(position, _)| *position);
        let text = self
            .source
            .get(start..end)
            .ok_or_else(|| Self::error(start, end, FormulaLexicalRule::SourceBoundary))?;
        let kind = if kind == FormulaLexemeKind::Identifier {
            if !valid_spelling(text) {
                return Err(Self::error(
                    start,
                    end,
                    FormulaLexicalRule::IdentifierSpelling,
                ));
            }
            match grammar_keyword(text) {
                Some(GrammarKeyword::Let) => FormulaLexemeKind::Let,
                Some(GrammarKeyword::Assert) => FormulaLexemeKind::Assert,
                Some(GrammarKeyword::If) => FormulaLexemeKind::If,
                None => FormulaLexemeKind::Identifier,
            }
        } else {
            kind
        };
        Ok(Some(FormulaLexeme {
            kind,
            text,
            span: FormulaSourceSpan { start, end },
        }))
    }
}
impl<'a> Iterator for FormulaLexer<'a> {
    type Item = Result<FormulaLexeme<'a>, FormulaLexicalError>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }
        match self.scan() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => {
                self.stopped = true;
                None
            }
            Err(error) => {
                self.stopped = true;
                Some(Err(error))
            }
        }
    }
}
impl FusedIterator for FormulaLexer<'_> {}
