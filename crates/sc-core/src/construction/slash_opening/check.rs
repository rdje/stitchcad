//! Required Angle/Length proof from the actual original mode,without numerical execution.
use super::{NormalizedSlashSpreadOpening, SlashSpreadOpeningMode};
use crate::{
    construction::arguments::{check_expression, CheckRefusal},
    recipe::{
        FormulaCheckedExpression, FormulaExpressionCheckError, FormulaKind, FormulaNameScope,
        FormulaNamespace, FormulaSourceSpan,
    },
};
use core::fmt;

/// Original expression refusal or actual completed root of the wrong explicit mode's kind.
#[derive(Debug)]
pub enum SlashSpreadCheckRefusal<'a> {
    /// Original arena,node/span and typed language cause.
    Expression(FormulaExpressionCheckError<'a, 'a>),
    /// Actual statically inferred root kind;required kind comes from the original mode.
    Kind {
        /// Actual completed root kind.
        actual: FormulaKind,
    },
}
/// Actual normalized owner and complete original refusal; no fabricated operation ordinal.
pub struct SlashSpreadCheckError<'a> {
    normalized: &'a NormalizedSlashSpreadOpening<'a>,
    refusal: SlashSpreadCheckRefusal<'a>,
}
impl<'a> SlashSpreadCheckError<'a> {
    /// Actual complete normalized owner and original explicit opening.
    #[must_use]
    pub const fn normalized(&self) -> &'a NormalizedSlashSpreadOpening<'a> {
        self.normalized
    }
    /// Actual explicit authored mode,even when the document's root kind differs.
    #[must_use]
    pub const fn mode(&self) -> SlashSpreadOpeningMode {
        self.normalized.opening().mode()
    }
    /// Required kind from that original mode,not caller metadata or inferred intent.
    #[must_use]
    pub const fn required_kind(&self) -> FormulaKind {
        self.mode().required_kind()
    }
    /// Original actual expression or root-kind cause.
    #[must_use]
    pub const fn refusal(&self) -> &SlashSpreadCheckRefusal<'a> {
        &self.refusal
    }
    /// Actual failed-node span,or actual whole-root span for a root-kind mismatch.
    #[must_use]
    pub fn span(&self) -> FormulaSourceSpan {
        match &self.refusal {
            SlashSpreadCheckRefusal::Expression(error) => error.span(),
            SlashSpreadCheckRefusal::Kind { .. } => self.normalized.expression().root().span(),
        }
    }
    /// Original language token,or formula_dimension for the actual wrong root kind.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match &self.refusal {
            SlashSpreadCheckRefusal::Expression(error) => error.token(),
            SlashSpreadCheckRefusal::Kind { .. } => "formula_dimension",
        }
    }
}
impl fmt::Debug for SlashSpreadCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadCheckError")
            .field("mode", &self.mode())
            .field("token", &self.token())
            .field("span", &self.span())
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slash/spread {:?} amount: {}", self.mode(), self.token())
    }
}
impl std::error::Error for SlashSpreadCheckError<'_> {}

/// Actual complete amount proof tied to its original normalized owner and explicit mode.
/// No value,cut/pivot/moving-region,direction,output or geometry follows from a kind proof.
/// ```compile_fail
/// fn rewrite(checked:&mut sc_core::construction::CheckedSlashSpreadOpening<'_>) { checked.expression=todo!(); }
/// ```
/// ```compile_fail
/// fn escape(opening:&sc_core::construction::NormalizedSlashSpreadOpening<'_>)
///     ->sc_core::construction::CheckedSlashSpreadOpening<'static> {
///     opening.check_argument(&sc_core::recipe::FormulaNamespace::new([]).unwrap()).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct CheckedSlashSpreadOpening<'a> {
    normalized: &'a NormalizedSlashSpreadOpening<'a>,
    expression: FormulaCheckedExpression<'a, 'a, 'a>,
}
impl<'a> CheckedSlashSpreadOpening<'a> {
    /// Actual normalized owner,explicit mode and original document.
    #[must_use]
    pub const fn normalized(&self) -> &'a NormalizedSlashSpreadOpening<'a> {
        self.normalized
    }
    /// Actual amount proof,without detached or caller-substituted nodes.
    #[must_use]
    pub const fn expression(&self) -> &FormulaCheckedExpression<'a, 'a, 'a> {
        &self.expression
    }
}
impl<'s> NormalizedSlashSpreadOpening<'s> {
    /// Invoke the same complete argument checker through an actual borrowed visible-name scope.
    /// No map clone,source substitution,values or containing-program acceptance is introduced.
    /// # Errors
    /// Original actual argument/role/span and typed expression or required-kind refusal.
    pub fn check_argument_in_scope<'a>(
        &'a self,
        names: FormulaNameScope<'_, 'a>,
    ) -> Result<CheckedSlashSpreadOpening<'a>, SlashSpreadCheckError<'a>>
    where
        's: 'a,
    {
        self.check_argument(names.namespace())
    }

    /// Check the actual whole amount against its explicit mode's required kind before any value.
    /// ```
    /// use sc_core::{construction::*,recipe::*};
    /// let opening=SlashSpreadOpening::separation(FormulaExpressionDocument::new("1 mm/0".into())?);
    /// let normalized=opening.normalized(SlashSpreadNormalizationLimits::new(3,7)).expect("complete shape");
    /// let checked=normalized.check_argument(&FormulaNamespace::new([])?).expect("Length without division");
    /// assert_eq!(checked.expression().kind(),FormulaKind::Length);
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Original expression error or actual wrong root kind,retaining mode,owner,span and cause.
    /// Unknown values and geometric/numerical domains remain unevaluated,not defaulted.
    pub fn check_argument<'a>(
        &'a self,
        namespace: &FormulaNamespace<'a>,
    ) -> Result<CheckedSlashSpreadOpening<'a>, SlashSpreadCheckError<'a>>
    where
        's: 'a,
    {
        let normalized: &NormalizedSlashSpreadOpening<'a> = self;
        let expression = check_expression(
            normalized.expression(),
            namespace,
            normalized.opening().mode().required_kind(),
        )
        .map_err(|cause| SlashSpreadCheckError {
            normalized,
            refusal: match cause {
                CheckRefusal::Expression(error) => SlashSpreadCheckRefusal::Expression(error),
                CheckRefusal::Kind { actual } => SlashSpreadCheckRefusal::Kind { actual },
            },
        })?;
        Ok(CheckedSlashSpreadOpening {
            normalized,
            expression,
        })
    }
}
