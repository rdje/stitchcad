//! Bounded, value-free expression proof over one normalized owner and initial declaration metadata.
use super::{
    calls::lookup_call_name, normalized::NormalizedData, FormulaBinaryOperator, FormulaBuiltin,
    FormulaBuiltinOperand, FormulaCallRefusal, FormulaCanonicalExpression, FormulaDeclaration,
    FormulaDeclarationSource, FormulaKind, FormulaKindSignature, FormulaLiteralKind,
    FormulaNamespace, FormulaNormalizedExpression, FormulaReservedName, FormulaSourceSpan,
    FormulaUnaryOperator, FormulaUnboundName,
};
use core::fmt;

/// Actual closed operation at a dimension refusal, preserving unary/binary arity and call identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaCheckedOperation {
    /// Unary minus or fixed square.
    Unary(FormulaUnaryOperator),
    /// Binary arithmetic or comparison, with authored operand order.
    Binary(FormulaBinaryOperator),
    /// Known function, selector or the existing conditional special form.
    Builtin(FormulaBuiltin),
}
impl FormulaCheckedOperation {
    /// Exact canonical operator or callee spelling, with no localization.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Unary(operation) => operation.token(),
            Self::Binary(operation) => operation.token(),
            Self::Builtin(operation) => operation.token(),
        }
    }
    /// Complete immutable wanted-kind alternatives for this actual operation.
    #[must_use]
    pub const fn signatures(self) -> &'static [FormulaKindSignature] {
        match self {
            Self::Unary(operation) => operation.signatures(),
            Self::Binary(operation) => operation.signatures(),
            Self::Builtin(operation) => operation.signatures(),
        }
    }
}

/// Complete immediate dimension arguments, constructed only after every child has resolved.
/// Actual kinds and symbolic tolerance roles are available through the ordered operand slice.
/// This carries no values, guessed kinds or statement ordinal.
pub struct FormulaDimensionRefusal {
    operation: FormulaCheckedOperation,
    operands: Vec<FormulaBuiltinOperand>,
}
impl FormulaDimensionRefusal {
    /// Actual typed operator or known callee.
    #[must_use]
    pub const fn operation(&self) -> FormulaCheckedOperation {
        self.operation
    }
    /// Every immediate resolved operand in source order, including direct tolerance-name roles.
    #[must_use]
    pub fn operands(&self) -> &[FormulaBuiltinOperand] {
        &self.operands
    }
    /// Every normative alternative, without filtering away rejected or directed rows.
    #[must_use]
    pub const fn wanted_signatures(&self) -> &'static [FormulaKindSignature] {
        self.operation.signatures()
    }
    /// True only for a refused angle-times-length product, in either order.
    #[must_use]
    pub fn arc_length_hint(&self) -> bool {
        match (self.operation, self.operands.as_slice()) {
            (FormulaCheckedOperation::Binary(operation), [left, right]) => {
                operation.arc_length_hint(left.kind(), right.kind())
            }
            _ => false,
        }
    }
}
impl fmt::Debug for FormulaDimensionRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaDimensionRefusal")
            .field("operation", &self.operation)
            .field("operands", &self.operands)
            .finish_non_exhaustive()
    }
}

/// Structured reason for a static expression refusal; each nested payload retains its actual scope.
#[derive(Debug)]
pub enum FormulaExpressionCheckRefusal<'a> {
    /// A data-name read failed its existing declaration-domain search.
    UnboundName(FormulaUnboundName<'a>),
    /// A callee refused before its arguments, including envelope requests.
    Call(FormulaCallRefusal<'a>),
    /// Every child resolved, but no normative signature admits the complete operand tuple.
    Dimension(FormulaDimensionRefusal),
}
impl FormulaExpressionCheckRefusal<'_> {
    /// Existing stable diagnostic token; command presentation must localize it.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::UnboundName(error) => error.token(),
            Self::Call(error) => error.token(),
            Self::Dimension(_) => "formula_dimension",
        }
    }
}

/// Refusal borrowing the actual normalized owner and failing-node span; no detached context can be forged.
/// Grouping remains part of the node span. Canonical identity is the whole available expression;
/// it is produced only on explicit inspection, never by reading values during checking.
/// ```compile_fail
/// fn forge<'e, 's>(expression: &'e sc_core::recipe::FormulaNormalizedExpression<'s>,
///     name: &'s sc_core::name::MachineToken) -> sc_core::recipe::FormulaExpressionCheckError<'e, 's> {
///     use sc_core::recipe::*;
///     let missing = FormulaNamespace::new([]).unwrap().resolve(name).unwrap_err();
///     FormulaExpressionCheckError { expression, span: expression.root().span(),
///         refusal: FormulaExpressionCheckRefusal::UnboundName(missing) }
/// }
/// ```
/// ```compile_fail
/// fn escape(source: &str) -> sc_core::recipe::FormulaExpressionCheckError<'_, '_> {
///     let expression = sc_core::recipe::FormulaExpression::parse(source).unwrap().normalize_literals().unwrap();
///     expression.check_kinds(&sc_core::recipe::FormulaNamespace::new([]).unwrap()).unwrap_err()
/// }
/// ```
pub struct FormulaExpressionCheckError<'e, 's> {
    expression: &'e FormulaNormalizedExpression<'s>,
    span: FormulaSourceSpan,
    refusal: FormulaExpressionCheckRefusal<'s>,
}
impl<'e, 's> FormulaExpressionCheckError<'e, 's> {
    /// Exact normalized expression that was checked; syntax/literal normalization had succeeded.
    #[must_use]
    pub const fn expression(&self) -> &'e FormulaNormalizedExpression<'s> {
        self.expression
    }
    /// Actual failing normalized node's original source location, with no guessed callee subspan.
    #[must_use]
    pub const fn span(&self) -> FormulaSourceSpan {
        self.span
    }
    /// Typed complete refusal arguments, with data reads and callees retaining distinct domains.
    #[must_use]
    pub const fn refusal(&self) -> &FormulaExpressionCheckRefusal<'s> {
        &self.refusal
    }
    /// Existing stable token, without customer source text.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        self.refusal.token()
    }
    /// Explicit whole-expression identity from this same normalized owner, without evaluation.
    #[must_use]
    pub fn canonical_expression(&self) -> FormulaCanonicalExpression {
        self.expression.canonical_form()
    }
}
impl fmt::Debug for FormulaExpressionCheckError<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaExpressionCheckError")
            .field("token", &self.token())
            .field("span", &self.span)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for FormulaExpressionCheckError<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}
impl std::error::Error for FormulaExpressionCheckError<'_, '_> {}

/// One actual name occurrence and its immutable resolved declaration, including untaken branches.
/// Repeated names retain separate use spans; this is dependency evidence, not a numerical value.
#[derive(Clone, Copy)]
pub struct FormulaNameDependency<'d> {
    span: FormulaSourceSpan,
    declaration: FormulaDeclaration<'d>,
}
impl<'d> FormulaNameDependency<'d> {
    /// Original normalized name-node span; grouping is transparent to kind and symbolic role.
    #[must_use]
    pub const fn span(self) -> FormulaSourceSpan {
        self.span
    }
    /// Actual copied source locator, preserving canonical borrows rather than copying input state.
    #[must_use]
    pub const fn declaration(self) -> FormulaDeclaration<'d> {
        self.declaration
    }
}
impl fmt::Debug for FormulaNameDependency<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaNameDependency")
            .field("span", &self.span)
            .field("declaration", &self.declaration)
            .finish_non_exhaustive()
    }
}

/// Immutable proof that all names and kinds of this exact expression passed initial-scope checking.
/// Borrows the normalized owner and canonical declaration sources, but not the namespace allocation.
/// It grants no input availability, numeric-domain, statement, operation-graph or recipe acceptance.
/// ```compile_fail
/// fn forge<'e, 's>(expression: &'e sc_core::recipe::FormulaNormalizedExpression<'s>)
///     -> sc_core::recipe::FormulaCheckedExpression<'e, 's, 'static> {
///     use sc_core::recipe::*;
///     FormulaCheckedExpression { expression, kind: FormulaKind::Length, dependencies: vec![] }
/// }
/// ```
/// ```compile_fail
/// fn escape(source: &str) -> sc_core::recipe::FormulaCheckedExpression<'_, '_, 'static> {
///     let expression = sc_core::recipe::FormulaExpression::parse(source).unwrap().normalize_literals().unwrap();
///     expression.check_kinds(&sc_core::recipe::FormulaNamespace::new([]).unwrap()).unwrap()
/// }
/// ```
/// ```compile_fail
/// fn escape_record<'e, 's>(expression: &'e sc_core::recipe::FormulaNormalizedExpression<'s>,
///     name: &'static sc_core::name::MachineToken)
///     -> sc_core::recipe::FormulaCheckedExpression<'e, 's, 'static> {
///     use sc_core::{ontology::EntityId, recipe::*, value::*};
///     let record = LengthDeclaration::new(LengthDeclarationDefinition {
///         id: EntityId::from_bits(1), source: EntityId::from_bits(2),
///         state: LengthState::Unknown { observation: EntityId::from_bits(3) },
///     }).unwrap();
///     let declaration = FormulaDeclaration::length_input(name, FormulaInputOrigin::Measurement,
///         EntityId::from_bits(4), &record);
///     let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(declaration).unwrap()]).unwrap();
///     expression.check_kinds(&namespace).unwrap()
/// }
/// ```
pub struct FormulaCheckedExpression<'e, 's, 'd> {
    expression: &'e FormulaNormalizedExpression<'s>,
    kind: FormulaKind,
    dependencies: Vec<FormulaNameDependency<'d>>,
}
impl<'e, 's, 'd> FormulaCheckedExpression<'e, 's, 'd> {
    /// Exact expression owner certified by this proof; no replacement syntax can be attached.
    #[must_use]
    pub const fn expression(&self) -> &'e FormulaNormalizedExpression<'s> {
        self.expression
    }
    /// Complete expression's statically inferred kind, independently of numeric availability.
    #[must_use]
    pub const fn kind(&self) -> FormulaKind {
        self.kind
    }
    /// Every ordered name use and source, with repetitions and untaken branches retained.
    #[must_use]
    pub fn dependencies(&self) -> &[FormulaNameDependency<'d>] {
        &self.dependencies
    }
    /// Explicit owned identity from this proof's exact normalized expression.
    #[must_use]
    pub fn canonical_expression(&self) -> FormulaCanonicalExpression {
        self.expression.canonical_form()
    }
}
impl fmt::Debug for FormulaCheckedExpression<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaCheckedExpression")
            .field("kind", &self.kind)
            .field("dependency_count", &self.dependencies.len())
            .finish_non_exhaustive()
    }
}

enum Action {
    Enter(usize),
    Apply {
        index: usize,
        operation: FormulaCheckedOperation,
        arity: usize,
    },
}

impl<'s> FormulaNormalizedExpression<'s> {
    /// Check every name/kind in this bounded expression against a checked initial namespace.
    /// Callees resolve first; known children check left-to-right before the immediate signature.
    /// Both conditional branches and all within operands are checked without reading any value.
    /// Explicit heap stacks preserve structural bounds without recursive calls or destruction.
    /// ```
    /// use sc_core::recipe::{FormulaExpression, FormulaNamespace, FormulaKind};
    /// let expression = FormulaExpression::parse("if(is_base_size,1 mm,2 mm)")?.normalize_literals()?;
    /// let namespace = FormulaNamespace::new([])?;
    /// let proof = expression.check_kinds(&namespace).unwrap();
    /// assert_eq!(proof.kind(), FormulaKind::Length);
    /// assert_eq!(proof.dependencies()[0].declaration().name(), "is_base_size");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Returns the first static refusal with its actual node span and this expression owner.
    /// A late error exposes no accepted expression or dependency prefix. Syntax and literal-input
    /// refusals belong to the preceding constructors; statements/recipes require separate checks.
    #[allow(clippy::indexing_slicing)] // Parser-private indices and enter/apply stack arities are invariant.
    pub fn check_kinds<'e, 'd>(
        &'e self,
        namespace: &FormulaNamespace<'d>,
    ) -> Result<FormulaCheckedExpression<'e, 's, 'd>, FormulaExpressionCheckError<'e, 's>> {
        use FormulaBuiltinOperand as O;
        use FormulaCheckedOperation as Op;
        let mut pending = vec![Action::Enter(self.root)];
        let mut kinds = Vec::with_capacity(self.nodes.len());
        let mut dependencies = Vec::new();
        let refuse = |index: usize, refusal| FormulaExpressionCheckError {
            expression: self,
            span: self.nodes[index].span,
            refusal,
        };
        while let Some(action) = pending.pop() {
            match action {
                Action::Apply {
                    index,
                    operation,
                    arity,
                } => {
                    // Each entered child leaves exactly one kind; this consumes only this node's children.
                    let operands = kinds.split_off(kinds.len() - arity);
                    let result = operation
                        .signatures()
                        .iter()
                        .find_map(|row| row.result_kind(&operands));
                    let Some(kind) = result else {
                        return Err(refuse(
                            index,
                            FormulaExpressionCheckRefusal::Dimension(FormulaDimensionRefusal {
                                operation,
                                operands,
                            }),
                        ));
                    };
                    // Computed results never acquire the direct-name tolerance role.
                    kinds.push(O::Value(kind));
                }
                Action::Enter(index) => match &self.nodes[index].data {
                    NormalizedData::Literal(literal) => {
                        let kind = match literal.kind() {
                            FormulaLiteralKind::Length => FormulaKind::Length,
                            FormulaLiteralKind::Angle => FormulaKind::Angle,
                            FormulaLiteralKind::Ratio => FormulaKind::Ratio,
                            FormulaLiteralKind::Count => FormulaKind::Count,
                        };
                        kinds.push(O::Value(kind));
                    }
                    NormalizedData::Name(name) => {
                        let declaration = namespace.resolve_name(name).map_err(|error| {
                            refuse(index, FormulaExpressionCheckRefusal::UnboundName(error))
                        })?;
                        let operand = match declaration.source() {
                            FormulaDeclarationSource::Reserved(FormulaReservedName::Tolerance(
                                class,
                            )) => O::Tolerance(class),
                            _ => O::Value(declaration.kind()),
                        };
                        dependencies.push(FormulaNameDependency {
                            span: self.nodes[index].span,
                            declaration,
                        });
                        kinds.push(operand);
                    }
                    NormalizedData::Negate(child) | NormalizedData::Square(child) => {
                        let operation =
                            if matches!(self.nodes[index].data, NormalizedData::Negate(_)) {
                                FormulaUnaryOperator::Negate
                            } else {
                                FormulaUnaryOperator::Square
                            };
                        pending.push(Action::Apply {
                            index,
                            operation: Op::Unary(operation),
                            arity: 1,
                        });
                        pending.push(Action::Enter(*child));
                    }
                    NormalizedData::Binary {
                        operator,
                        left,
                        right,
                    } => {
                        pending.push(Action::Apply {
                            index,
                            operation: Op::Binary(*operator),
                            arity: 2,
                        });
                        pending.push(Action::Enter(*right));
                        pending.push(Action::Enter(*left));
                    }
                    NormalizedData::Call { name, arguments } => {
                        let builtin = lookup_call_name(name).map_err(|error| {
                            refuse(index, FormulaExpressionCheckRefusal::Call(error))
                        })?;
                        pending.push(Action::Apply {
                            index,
                            operation: Op::Builtin(builtin),
                            arity: arguments.len(),
                        });
                        for child in arguments.iter().rev() {
                            pending.push(Action::Enter(*child));
                        }
                    }
                    NormalizedData::Conditional {
                        condition,
                        then_branch,
                        else_branch,
                    } => {
                        pending.push(Action::Apply {
                            index,
                            operation: Op::Builtin(FormulaBuiltin::If),
                            arity: 3,
                        });
                        pending.push(Action::Enter(*else_branch));
                        pending.push(Action::Enter(*then_branch));
                        pending.push(Action::Enter(*condition));
                    }
                },
            }
        }
        // Successful traversal of the validated connected arena leaves exactly the root kind.
        Ok(FormulaCheckedExpression {
            expression: self,
            kind: kinds[0].kind(),
            dependencies,
        })
    }
}
