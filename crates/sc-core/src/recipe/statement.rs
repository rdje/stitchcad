//! Immutable single-statement syntax; annotation vocabulary is not binding or execution.
use super::{
    FormulaExpression, FormulaLexeme, FormulaLexemeKind as K, FormulaLexer, FormulaLexicalRule,
    FormulaParseError, FormulaSourceSpan as Span,
};
use core::fmt;
use core::iter::Peekable;

/// The six kinds a formula statement may declare, without inferring its expression's kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaBindingKind {
    /// Distance or coordinate.
    Length,
    /// Signed angle or sweep.
    Angle,
    /// Derived area; there is no area literal.
    Area,
    /// Scale factor.
    Ratio,
    /// Nonnegative repetition count at the later binding boundary.
    Count,
    /// Boolean condition; there is no Boolean literal.
    Boolean,
}
impl FormulaBindingKind {
    /// Exact declared machine token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Length => "length",
            Self::Angle => "angle",
            Self::Area => "area",
            Self::Ratio => "ratio",
            Self::Count => "count",
            Self::Boolean => "boolean",
        }
    }
    fn from_token(token: &str) -> Option<Self> {
        match token {
            "length" => Some(Self::Length),
            "angle" => Some(Self::Angle),
            "area" => Some(Self::Area),
            "ratio" => Some(Self::Ratio),
            "count" => Some(Self::Count),
            "boolean" => Some(Self::Boolean),
            _ => None,
        }
    }
}

/// Symbolic assertion tolerance names. Parsing supplies no numeric tolerance or factory context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaToleranceName {
    /// T1 numerical.
    Numerical,
    /// T2 internal geometric approximation; the chordal sibling is not a formula name.
    Geometric,
    /// T3 target-format quantum.
    Format,
    /// T4 receiver comparison.
    Importer,
    /// T5 physical acceptance.
    Physical,
}
impl FormulaToleranceName {
    /// Exact reserved machine token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Numerical => "eps_num",
            Self::Geometric => "eps_geo",
            Self::Format => "eps_fmt",
            Self::Importer => "eps_imp",
            Self::Physical => "eps_phys",
        }
    }
    fn from_token(token: &str) -> Option<Self> {
        match token {
            "eps_num" => Some(Self::Numerical),
            "eps_geo" => Some(Self::Geometric),
            "eps_fmt" => Some(Self::Format),
            "eps_imp" => Some(Self::Importer),
            "eps_phys" => Some(Self::Physical),
            _ => None,
        }
    }
}

/// Which unevaluated expression in the statement was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaStatementExpression {
    /// Right-hand side of a let declaration.
    Binding,
    /// Left assertion operand.
    AssertionLeft,
    /// Right assertion operand.
    AssertionRight,
}

/// Single-statement refusal, without customer names or source text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaStatementRule {
    /// Shared lexical refusal.
    Lexical(FormulaLexicalRule),
    /// Requires let or assert, rather than an implicit expression statement.
    ExpectedKeyword,
    /// Requires a non-keyword machine identifier.
    ExpectedName,
    /// Requires the annotation colon.
    ExpectedColon,
    /// Requires an identifier in the annotation position.
    ExpectedAnnotation,
    /// Annotation is not one of the six bindable kinds.
    UnbindableKind,
    /// Annotation is outside the five-name grammar; this is a syntax refusal, not missing context.
    UnknownTolerance,
    /// Requires a single assignment sign.
    ExpectedAssignment,
    /// Requires exactly one top-level equality separator between assertion operands.
    AssertionSeparator,
    /// Existing expression parser refusal with global source span and operand role.
    Expression {
        /// Refused operand.
        part: FormulaStatementExpression,
        /// Original rule with its span rebased to the whole statement source.
        error: FormulaParseError,
    },
}

/// Precise standalone statement refusal. A containing recipe supplies its ordinal later.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaStatementError {
    span: Span,
    rule: FormulaStatementRule,
}
impl FormulaStatementError {
    /// Stable diagnostic family; header annotation families agree with the curated reference.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        match self.rule {
            FormulaStatementRule::UnbindableKind => "formula_dimension",
            FormulaStatementRule::UnknownTolerance => "formula_parse",
            FormulaStatementRule::Expression { error, .. } => error.diagnostic_code(),
            _ => "formula_parse",
        }
    }
    /// Offending token, gap, opener or zero-width end in the original whole source.
    #[must_use]
    pub const fn span(self) -> Span {
        self.span
    }
    /// Typed header or expression refusal. No canonical expression is fabricated for bad syntax.
    #[must_use]
    pub const fn rule(self) -> FormulaStatementRule {
        self.rule
    }
}
impl fmt::Display for FormulaStatementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?} at bytes {}..{}",
            self.diagnostic_code(),
            self.rule,
            self.span.start(),
            self.span.end()
        )
    }
}
impl std::error::Error for FormulaStatementError {}

#[derive(Clone)]
enum StatementData<'a> {
    Let(FormulaBindingKind, FormulaExpression<'a>),
    Assert(
        FormulaToleranceName,
        FormulaExpression<'a>,
        FormulaExpression<'a>,
    ),
}

/// Privately constructed, immutable statement borrowing its original machine source.
/// ```compile_fail
/// fn mutate(s: &mut sc_core::recipe::FormulaStatement<'_>) { s.name = "other"; }
/// ```
/// ```compile_fail
/// fn detached() -> sc_core::recipe::FormulaStatement<'static> {
///     let source = String::from("let waist: length = 25 mm");
///     sc_core::recipe::FormulaStatement::parse(&source).unwrap()
/// }
/// ```
#[derive(Clone)]
pub struct FormulaStatement<'a> {
    span: Span,
    name_span: Span,
    annotation_span: Span,
    name: &'a str,
    data: StatementData<'a>,
}
impl fmt::Debug for FormulaStatement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaStatement")
            .field("span", &self.span)
            .field("kind", &self.kind())
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaStatement<'a> {
    /// Parse exactly one whole let/assert statement, with no normalization, binding or evaluation.
    /// Each operand retains the existing expression limits; grouping/traversal/drop do not recurse.
    /// ```
    /// use sc_core::recipe::{FormulaStatement, FormulaStatementKind};
    /// let statement = FormulaStatement::parse(
    ///     "let garment_waist: length = waist_girth + ease_waist"
    /// )?;
    /// assert_eq!(statement.name(), "garment_waist");
    /// if let FormulaStatementKind::Let { expression, .. } = statement.kind() {
    ///     let canonical = expression.normalize_literals()?.canonical_form();
    ///     assert_eq!(canonical.as_str(), "(+ waist_girth ease_waist)");
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn parse(source: &'a str) -> Result<Self, FormulaStatementError> {
        parse(source)
    }
    /// Convert every input literal in source operand order, preserving this syntax unchanged.
    /// The returned owner borrows original source, not this statement allocation. Declared kinds,
    /// names and tolerances are retained without static validation, binding or evaluation.
    /// ```
    /// use sc_core::recipe::{FormulaStatement, FormulaNormalizedStatementKind};
    /// let syntax = FormulaStatement::parse("let width: length = 2.5 cm")?;
    /// let normalized = syntax.normalize_literals()?;
    /// drop(syntax);
    /// if let FormulaNormalizedStatementKind::Let { expression, .. } = normalized.kind() {
    ///     assert_eq!(expression.canonical_form().as_str(), "length:25000");
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn normalize_literals(
        &self,
    ) -> Result<super::FormulaNormalizedStatement<'a>, super::FormulaStatementLiteralError> {
        use super::normalized_recipe::NormalizedStatementData as D;
        let convert = |expression: &FormulaExpression<'a>, part| {
            expression
                .normalize_literals()
                .map_err(|error| super::FormulaStatementLiteralError { part, error })
        };
        let data = match &self.data {
            StatementData::Let(kind, expression) => D::Let(
                *kind,
                convert(expression, FormulaStatementExpression::Binding)?,
            ),
            StatementData::Assert(tolerance, left, right) => D::Assert(
                *tolerance,
                convert(left, FormulaStatementExpression::AssertionLeft)?,
                convert(right, FormulaStatementExpression::AssertionRight)?,
            ),
        };
        Ok(super::FormulaNormalizedStatement {
            span: self.span,
            name_span: self.name_span,
            annotation_span: self.annotation_span,
            name: self.name,
            data,
        })
    }
    /// Complete statement span, excluding outer whitespace.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }
    /// Explicit access to the authored customer name; no namespace authority is granted.
    #[must_use]
    pub const fn name(&self) -> &'a str {
        self.name
    }
    /// Name's exact location in the original source.
    #[must_use]
    pub const fn name_span(&self) -> Span {
        self.name_span
    }
    /// Declared kind or symbolic tolerance location in the original source.
    #[must_use]
    pub const fn annotation_span(&self) -> Span {
        self.annotation_span
    }
    /// Read-only statement role and its unevaluated expression arenas.
    #[must_use]
    pub fn kind(&self) -> FormulaStatementKind<'_> {
        match &self.data {
            StatementData::Let(kind, expression) => FormulaStatementKind::Let {
                declared_kind: *kind,
                expression,
            },
            StatementData::Assert(tolerance, left, right) => FormulaStatementKind::Assert {
                tolerance: *tolerance,
                left,
                right,
            },
        }
    }
}

/// Borrowed views cannot detach an expression from its owning statement arena.
/// ```compile_fail
/// fn escaped(source: &str) -> sc_core::recipe::FormulaStatementKind<'_> {
///     let statement = sc_core::recipe::FormulaStatement::parse(source).unwrap();
///     statement.kind()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub enum FormulaStatementKind<'a> {
    /// Binding syntax, without comparing declared and inferred kinds.
    Let {
        /// Authored annotation.
        declared_kind: FormulaBindingKind,
        /// Unevaluated expression.
        expression: &'a FormulaExpression<'a>,
    },
    /// Assertion syntax, without resolving a tolerance or checking whether it holds.
    Assert {
        /// Symbolic tolerance annotation.
        tolerance: FormulaToleranceName,
        /// Left expression, with its own node/depth limits.
        left: &'a FormulaExpression<'a>,
        /// Right expression, with its own node/depth limits.
        right: &'a FormulaExpression<'a>,
    },
}

fn refusal(rule: FormulaStatementRule, span: Span) -> FormulaStatementError {
    FormulaStatementError { rule, span }
}
fn header<'a>(
    lexer: &mut Peekable<FormulaLexer<'a>>,
    source_end: usize,
    kind: K,
    rule: FormulaStatementRule,
) -> Result<FormulaLexeme<'a>, FormulaStatementError> {
    match lexer.next().transpose() {
        Ok(Some(token)) if token.kind() == kind => Ok(token),
        Ok(token) => Err(refusal(
            rule,
            token.map_or(Span::new(source_end, source_end), |t| t.span()),
        )),
        Err(error) => Err(refusal(
            FormulaStatementRule::Lexical(error.rule()),
            error.span(),
        )),
    }
}

fn expression(
    source: &str,
    range: Span,
    part: FormulaStatementExpression,
) -> Result<FormulaExpression<'_>, FormulaStatementError> {
    // Only lexer-produced boundaries and source.len() enter these private ranges.
    let Some(input) = source.get(range.start()..range.end()) else {
        return Err(refusal(
            FormulaStatementRule::Lexical(FormulaLexicalRule::SourceBoundary),
            range,
        ));
    };
    let mut parsed = FormulaExpression::parse(input).map_err(|mut error| {
        error.span = Span::new(
            range.start() + error.span.start(),
            range.start() + error.span.end(),
        );
        refusal(FormulaStatementRule::Expression { part, error }, error.span)
    })?;
    for node in &mut parsed.nodes {
        node.span = Span::new(
            range.start() + node.span.start(),
            range.start() + node.span.end(),
        );
    }
    Ok(parsed)
}

fn record_separator(current: &mut Option<Span>, span: Span) -> Result<(), FormulaStatementError> {
    if current.is_some() {
        return Err(refusal(FormulaStatementRule::AssertionSeparator, span));
    }
    *current = Some(span);
    Ok(())
}

fn parse(source: &str) -> Result<FormulaStatement<'_>, FormulaStatementError> {
    parse_next(source, &mut FormulaLexer::new(source).peekable(), false)
}

/// Shared original-source parser; only the containing recipe permits another statement boundary.
pub(super) fn parse_next<'a>(
    source: &'a str,
    lexer: &mut Peekable<FormulaLexer<'a>>,
    recipe_boundary: bool,
) -> Result<FormulaStatement<'a>, FormulaStatementError> {
    use FormulaStatementRule as R;
    let head = match lexer.next().transpose() {
        Ok(Some(token)) if matches!(token.kind(), K::Let | K::Assert) => token,
        Ok(token) => {
            return Err(refusal(
                R::ExpectedKeyword,
                token.map_or(Span::new(source.len(), source.len()), |t| t.span()),
            ))
        }
        Err(error) => return Err(refusal(R::Lexical(error.rule()), error.span())),
    };
    let name = header(lexer, source.len(), K::Identifier, R::ExpectedName)?;
    let _ = header(lexer, source.len(), K::Colon, R::ExpectedColon)?;
    let annotation = header(lexer, source.len(), K::Identifier, R::ExpectedAnnotation)?;
    // Refuse closed annotation vocabulary before parsing an expression, as the reference does.
    let binding_kind = if head.kind() == K::Let {
        Some(
            FormulaBindingKind::from_token(annotation.text())
                .ok_or_else(|| refusal(R::UnbindableKind, annotation.span()))?,
        )
    } else {
        None
    };
    let tolerance = if head.kind() == K::Assert {
        Some(
            FormulaToleranceName::from_token(annotation.text())
                .ok_or_else(|| refusal(R::UnknownTolerance, annotation.span()))?,
        )
    } else {
        None
    };
    let assignment = header(lexer, source.len(), K::Assign, R::ExpectedAssignment)?;
    let (mut depth, mut separator, mut end) = (0_usize, None, assignment.span().end());
    let mut range_end = source.len();
    loop {
        match lexer.peek() {
            Some(Ok(next))
                if recipe_boundary && depth == 0 && matches!(next.kind(), K::Let | K::Assert) =>
            {
                range_end = next.span().start();
                break;
            }
            _ => {}
        }
        let Some(token) = lexer.next() else { break };
        let token = token.map_err(|error| {
            let part = if binding_kind.is_some() {
                FormulaStatementExpression::Binding
            } else if separator.is_none() {
                FormulaStatementExpression::AssertionLeft
            } else {
                FormulaStatementExpression::AssertionRight
            };
            let nested = FormulaParseError {
                span: error.span(),
                rule: super::FormulaParseRule::Lexical(error.rule()),
            };
            refusal(
                R::Expression {
                    part,
                    error: nested,
                },
                error.span(),
            )
        })?;
        end = token.span().end();
        match token.kind() {
            K::LeftParen => depth += 1,
            // This scanner only locates the separator. Expression parsing validates all delimiters.
            K::RightParen => depth = depth.saturating_sub(1),
            K::Equal if head.kind() == K::Assert && depth == 0 => {
                record_separator(&mut separator, token.span())?;
            }
            _ => {}
        }
    }
    let data = if let Some(kind) = binding_kind {
        StatementData::Let(
            kind,
            expression(
                source,
                Span::new(assignment.span().end(), range_end),
                FormulaStatementExpression::Binding,
            )?,
        )
    } else if let Some(tolerance) = tolerance {
        let separator = separator
            .ok_or_else(|| refusal(R::AssertionSeparator, Span::new(range_end, range_end)))?;
        let left = expression(
            source,
            Span::new(assignment.span().end(), separator.start()),
            FormulaStatementExpression::AssertionLeft,
        )?;
        let right = expression(
            source,
            Span::new(separator.end(), range_end),
            FormulaStatementExpression::AssertionRight,
        )?;
        StatementData::Assert(tolerance, left, right)
    } else {
        return Err(refusal(R::ExpectedAnnotation, annotation.span()));
    };
    Ok(FormulaStatement {
        span: Span::new(head.span().start(), end),
        name_span: name.span(),
        annotation_span: annotation.span(),
        name: name.text(),
        data,
    })
}
