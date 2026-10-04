//! Value-free current-statement proof, available only through a real ordered name scope.
use super::{
    normalized_recipe::NormalizedStatementData as Data, FormulaBinaryOperator, FormulaBindingKind,
    FormulaCanonicalStatement, FormulaCheckedExpression, FormulaDimensionRefusal,
    FormulaExpressionCheckError, FormulaKind, FormulaNamespace, FormulaNormalizedStatement,
    FormulaSourceSpan, FormulaStatementExpression,
};
use core::fmt;

/// Actual declared and expression kinds at a bindable let's header mismatch.
/// Raw unbindable annotations belong to syntax parsing and never enter this payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaBindingDimensionRefusal {
    declared_kind: FormulaBindingKind,
    expression_kind: FormulaKind,
}
impl FormulaBindingDimensionRefusal {
    /// Genuine checked annotation, without an implicit kind conversion.
    #[must_use]
    pub const fn declared_kind(self) -> FormulaBindingKind {
        self.declared_kind
    }
    /// Fully checked RHS kind; no absent or invalid child kind is guessed.
    #[must_use]
    pub const fn expression_kind(self) -> FormulaKind {
        self.expression_kind
    }
    /// Only the declared kind satisfies this header.
    #[must_use]
    pub fn wanted_kind(self) -> FormulaKind {
        self.declared_kind.into()
    }
}

/// Static current-statement refusal; nested expressions retain their own normalized identity.
#[derive(Debug)]
pub enum FormulaStatementCheckRefusal<'a> {
    /// First refused operand, including the complete original node/name/call/dimension arguments.
    Expression {
        /// Binding RHS, assertion left or assertion right, in actual checking order.
        part: FormulaStatementExpression,
        /// Existing bounded-expression refusal, without a fabricated combined assertion AST.
        error: FormulaExpressionCheckError<'a, 'a>,
    },
    /// The valid annotation differs from its fully checked RHS kind.
    BindingDimension(FormulaBindingDimensionRefusal),
    /// Both assertion operands resolve but their complete tuple has no equality signature.
    AssertionDimension(FormulaDimensionRefusal),
}
impl FormulaStatementCheckRefusal<'_> {
    /// Stable existing token; command presentation supplies localization.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::Expression { error, .. } => error.token(),
            Self::BindingDimension(_) | Self::AssertionDimension(_) => "formula_dimension",
        }
    }
}

/// Privately constructed error tied to one actual statement and its genuine recipe ordinal.
/// ```compile_fail
/// fn forge<'a>(statement: &'a sc_core::recipe::FormulaNormalizedStatement<'a>,
///     error: sc_core::recipe::FormulaExpressionCheckError<'a, 'a>)
///     -> sc_core::recipe::FormulaStatementCheckError<'a> {
///     use sc_core::recipe::*;
///     FormulaStatementCheckError { statement, statement_index: 1,
///         refusal: FormulaStatementCheckRefusal::Expression {
///             part: FormulaStatementExpression::Binding, error } }
/// }
/// ```
/// ```compile_fail
/// fn escape(source: &str) -> sc_core::recipe::FormulaStatementCheckError<'_> {
///     use sc_core::recipe::*;
///     let recipe = FormulaRecipe::parse(source).unwrap().normalize_literals().unwrap();
///     FormulaNameCursor::new(FormulaNamespace::new([]).unwrap(), &recipe)
///         .current().unwrap().unwrap().check_kinds().unwrap_err()
/// }
/// ```
pub struct FormulaStatementCheckError<'a> {
    statement: &'a FormulaNormalizedStatement<'a>,
    statement_index: usize,
    refusal: FormulaStatementCheckRefusal<'a>,
}
impl<'a> FormulaStatementCheckError<'a> {
    /// Original normalized statement, including global spans and actual operands.
    #[must_use]
    pub const fn statement(&self) -> &'a FormulaNormalizedStatement<'a> {
        self.statement
    }
    /// Actual one-based position supplied by the checked scope, never by a public caller.
    #[must_use]
    pub const fn statement_index(&self) -> usize {
        self.statement_index
    }
    /// Nested node, let annotation or complete assertion statement span, as available.
    /// The assertion separator has no retained independent span and none is fabricated here.
    #[must_use]
    pub fn span(&self) -> FormulaSourceSpan {
        match &self.refusal {
            FormulaStatementCheckRefusal::Expression { error, .. } => error.span(),
            FormulaStatementCheckRefusal::BindingDimension(_) => self.statement.annotation_span(),
            FormulaStatementCheckRefusal::AssertionDimension(_) => self.statement.span(),
        }
    }
    /// Complete case-specific metadata, with no values or provider availability.
    #[must_use]
    pub const fn refusal(&self) -> &FormulaStatementCheckRefusal<'a> {
        &self.refusal
    }
    /// Existing stable token without authored source text.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        self.refusal.token()
    }
    /// Explicit owned bytes from this exact normalized statement, even when static checking refused.
    #[must_use]
    pub fn canonical_statement(&self) -> FormulaCanonicalStatement {
        self.statement.canonical_form()
    }
}
impl fmt::Debug for FormulaStatementCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaStatementCheckError")
            .field("token", &self.token())
            .field("statement_index", &self.statement_index)
            .field("span", &self.span())
            .finish_non_exhaustive()
    }
}
impl fmt::Display for FormulaStatementCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}
impl std::error::Error for FormulaStatementCheckError<'_> {}

/// Inspected component proofs. Only the privately constructed enclosing owner certifies a statement.
#[derive(Debug)]
pub enum FormulaCheckedStatementKind<'a> {
    /// The full RHS passed expression checking and matches the actual annotation.
    Let {
        /// Exact RHS proof with ordered sourced dependencies.
        expression: FormulaCheckedExpression<'a, 'a, 'a>,
    },
    /// Both operands passed expression checking and share one admitted arithmetic kind.
    Assert {
        /// Actual left operand proof; the tolerance remains statement metadata.
        left: FormulaCheckedExpression<'a, 'a, 'a>,
        /// Actual right operand proof, including all static dependencies.
        right: FormulaCheckedExpression<'a, 'a, 'a>,
    },
}

/// Immutable proof for one actual current statement; prior annotations are not whole-recipe proof.
/// Its expression/declaration borrows outlive cursor allocation but remain pinned to their owners.
/// ```compile_fail
/// fn forge<'a>(statement: &'a sc_core::recipe::FormulaNormalizedStatement<'a>,
///     expression: sc_core::recipe::FormulaCheckedExpression<'a, 'a, 'a>)
///     -> sc_core::recipe::FormulaCheckedStatement<'a> {
///     use sc_core::recipe::*;
///     FormulaCheckedStatement { statement, statement_index: 1,
///         kind: FormulaCheckedStatementKind::Let { expression } }
/// }
/// ```
/// ```compile_fail
/// fn escape(source: &str) -> sc_core::recipe::FormulaCheckedStatement<'_> {
///     use sc_core::recipe::*;
///     let recipe = FormulaRecipe::parse(source).unwrap().normalize_literals().unwrap();
///     FormulaNameCursor::new(FormulaNamespace::new([]).unwrap(), &recipe)
///         .current().unwrap().unwrap().check_kinds().unwrap()
/// }
/// ```
/// ```compile_fail
/// fn escape_record<'a>(recipe: &'a sc_core::recipe::FormulaNormalizedRecipe<'a>,
///     name: &'a sc_core::name::MachineToken) -> sc_core::recipe::FormulaCheckedStatement<'a> {
///     use sc_core::{ontology::EntityId, recipe::*, value::*};
///     let record = LengthDeclaration::new(LengthDeclarationDefinition {
///         id: EntityId::from_bits(1), source: EntityId::from_bits(2),
///         state: LengthState::Unknown { observation: EntityId::from_bits(3) },
///     }).unwrap();
///     let input = FormulaDeclaration::length_input(name, FormulaInputOrigin::Measurement,
///         EntityId::from_bits(4), &record);
///     let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(input).unwrap()]).unwrap();
///     FormulaNameCursor::new(namespace, recipe).current().unwrap().unwrap().check_kinds().unwrap()
/// }
/// ```
pub struct FormulaCheckedStatement<'a> {
    statement: &'a FormulaNormalizedStatement<'a>,
    statement_index: usize,
    kind: FormulaCheckedStatementKind<'a>,
}
impl<'a> FormulaCheckedStatement<'a> {
    /// Exact statement owner certified locally by this proof.
    #[must_use]
    pub const fn statement(&self) -> &'a FormulaNormalizedStatement<'a> {
        self.statement
    }
    /// Genuine one-based ordinal in the cursor's actual recipe.
    #[must_use]
    pub const fn statement_index(&self) -> usize {
        self.statement_index
    }
    /// Read-only component proofs; no statement or expression can be substituted.
    #[must_use]
    pub const fn kind(&self) -> &FormulaCheckedStatementKind<'a> {
        &self.kind
    }
    /// Explicit owned identity from this same normalized owner, without evaluation.
    #[must_use]
    pub fn canonical_statement(&self) -> FormulaCanonicalStatement {
        self.statement.canonical_form()
    }
}
impl fmt::Debug for FormulaCheckedStatement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaCheckedStatement")
            .field("statement_index", &self.statement_index)
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

/// Scope-private inputs: original statement, actual ordinal and that scope's declaration metadata.
pub(super) fn check_statement<'a>(
    statement: &'a FormulaNormalizedStatement<'a>,
    statement_index: usize,
    namespace: &FormulaNamespace<'a>,
) -> Result<FormulaCheckedStatement<'a>, FormulaStatementCheckError<'a>> {
    use FormulaStatementCheckRefusal as R;
    let refuse = |refusal| FormulaStatementCheckError {
        statement,
        statement_index,
        refusal,
    };
    let check = |expression: &'a super::FormulaNormalizedExpression<'a>, part| {
        expression
            .check_kinds(namespace)
            .map_err(|error| refuse(R::Expression { part, error }))
    };
    let kind = match &statement.data {
        Data::Let(declared_kind, expression) => {
            let expression = check(expression, FormulaStatementExpression::Binding)?;
            if expression.kind() != FormulaKind::from(*declared_kind) {
                return Err(refuse(R::BindingDimension(
                    FormulaBindingDimensionRefusal {
                        declared_kind: *declared_kind,
                        expression_kind: expression.kind(),
                    },
                )));
            }
            FormulaCheckedStatementKind::Let { expression }
        }
        Data::Assert(_, left, right) => {
            let left = check(left, FormulaStatementExpression::AssertionLeft)?;
            let right = check(right, FormulaStatementExpression::AssertionRight)?;
            let operands = [left.root_operand(), right.root_operand()];
            if !FormulaBinaryOperator::Equal
                .signatures()
                .iter()
                .any(|row| row.result_kind(&operands) == Some(FormulaKind::Boolean))
            {
                return Err(refuse(R::AssertionDimension(
                    FormulaDimensionRefusal::comparison(operands),
                )));
            }
            FormulaCheckedStatementKind::Assert { left, right }
        }
    };
    Ok(FormulaCheckedStatement {
        statement,
        statement_index,
        kind,
    })
}
