//! Actual Length/Angle scalar proofs for the explicit center-based variant, without values.
use super::{CenterArcArgument, NormalizedCenterArcConstruction};
use crate::{
    construction::arguments::{check_expression, CheckRefusal},
    recipe::{
        FormulaCheckedExpression, FormulaExpressionCheckError, FormulaKind, FormulaNamespace,
        FormulaSourceSpan,
    },
};
use core::fmt;

/// Original scalar expression refusal or an actual completed root of the wrong field kind.
#[derive(Debug)]
pub enum CenterArcCheckRefusal<'a> {
    /// Original normalized owner,node/span and typed name/operator/call failure.
    Expression(FormulaExpressionCheckError<'a, 'a>),
    /// Actual statically inferred root kind;the original role supplies the required kind.
    Kind {
        /// Actual root kind, not caller metadata.
        actual: FormulaKind,
    },
}
/// Original normalized arc and first failed scalar argument, with no accepted proof prefix.
pub struct CenterArcCheckError<'a> {
    normalized: &'a NormalizedCenterArcConstruction<'a>,
    argument: CenterArcArgument,
    refusal: CenterArcCheckRefusal<'a>,
}
impl<'a> CenterArcCheckError<'a> {
    /// Actual complete normalized owner and original arc/center/documents.
    #[must_use]
    pub const fn normalized(&self) -> &'a NormalizedCenterArcConstruction<'a> {
        self.normalized
    }
    /// Actual original scalar field, not a fabricated statement ordinal.
    #[must_use]
    pub const fn argument(&self) -> CenterArcArgument {
        self.argument
    }
    /// Required root kind from that actual closed argument role.
    #[must_use]
    pub const fn required_kind(&self) -> FormulaKind {
        self.argument.required_kind()
    }
    /// Original typed expression or actual-root kind refusal.
    #[must_use]
    pub const fn refusal(&self) -> &CenterArcCheckRefusal<'a> {
        &self.refusal
    }
    /// Actual failed node span or original whole root span for a root-kind mismatch.
    #[must_use]
    pub fn span(&self) -> FormulaSourceSpan {
        match &self.refusal {
            CenterArcCheckRefusal::Expression(error) => error.span(),
            CenterArcCheckRefusal::Kind { .. } => {
                self.normalized.argument(self.argument).root().span()
            }
        }
    }
    /// Original canonical formula family, or formula_dimension for the actual wrong root kind.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match &self.refusal {
            CenterArcCheckRefusal::Expression(error) => error.token(),
            CenterArcCheckRefusal::Kind { .. } => "formula_dimension",
        }
    }
}
impl fmt::Debug for CenterArcCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CenterArcCheckError")
            .field("argument", &self.argument)
            .field("token", &self.token())
            .field("span", &self.span())
            .finish_non_exhaustive()
    }
}
impl fmt::Display for CenterArcCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "arc {:?} argument: {}", self.argument, self.token())
    }
}
impl std::error::Error for CenterArcCheckError<'_> {}

/// Complete actual Radius Length and Start/Sweep Angle proofs, pinned to the original owner.
/// No positive radius,nonzero/supported sweep,center/frame/currentness or geometric result follows.
/// ```compile_fail
/// fn replace(arc:&mut sc_core::construction::CheckedCenterArcConstruction<'_>) { arc.radius=todo!(); }
/// ```
/// ```compile_fail
/// fn escape(arc:&sc_core::construction::NormalizedCenterArcConstruction<'_>)
///     ->sc_core::construction::CheckedCenterArcConstruction<'static> {
///     arc.check_arguments(&sc_core::recipe::FormulaNamespace::new([]).unwrap()).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct CheckedCenterArcConstruction<'a> {
    normalized: &'a NormalizedCenterArcConstruction<'a>,
    radius: FormulaCheckedExpression<'a, 'a, 'a>,
    start_angle: FormulaCheckedExpression<'a, 'a, 'a>,
    sweep_angle: FormulaCheckedExpression<'a, 'a, 'a>,
}
impl<'a> CheckedCenterArcConstruction<'a> {
    /// Actual normalized arc,admitted policy and original declaration/center/documents.
    #[must_use]
    pub const fn normalized(&self) -> &'a NormalizedCenterArcConstruction<'a> {
        self.normalized
    }
    /// Actual complete proof for this original scalar field,without substitution or detached nodes.
    #[must_use]
    pub const fn argument(
        &self,
        argument: CenterArcArgument,
    ) -> &FormulaCheckedExpression<'a, 'a, 'a> {
        match argument {
            CenterArcArgument::Radius => &self.radius,
            CenterArcArgument::StartAngle => &self.start_angle,
            CenterArcArgument::SweepAngle => &self.sweep_angle,
        }
    }
}
fn check<'a>(
    normalized: &'a NormalizedCenterArcConstruction<'a>,
    namespace: &FormulaNamespace<'a>,
    argument: CenterArcArgument,
) -> Result<FormulaCheckedExpression<'a, 'a, 'a>, CenterArcCheckError<'a>> {
    check_expression(
        normalized.argument(argument),
        namespace,
        argument.required_kind(),
    )
    .map_err(|cause| CenterArcCheckError {
        normalized,
        argument,
        refusal: match cause {
            CheckRefusal::Expression(error) => CenterArcCheckRefusal::Expression(error),
            CheckRefusal::Kind { actual } => CenterArcCheckRefusal::Kind { actual },
        },
    })
}
impl<'s> NormalizedCenterArcConstruction<'s> {
    /// Check all actual scalar fields in Radius/StartAngle/SweepAngle order,before any value.
    /// Each proof and its required root kind must succeed before the complete private result exists.
    /// Source names/branches retain their original static refusals;center validation stays separate.
    /// ```
    /// use sc_core::{construction::*,ontology::{EntityId,PointRef,LocalTag},recipe::*};
    /// let arc=CenterArcConstruction::new(EntityId::from_bits(10),
    ///     PointRef::new(EntityId::from_bits(1),LocalTag::FIRST),
    ///     FormulaExpressionDocument::new("1 mm/0".into())?,
    ///     FormulaExpressionDocument::new("90 deg".into())?,
    ///     FormulaExpressionDocument::new("-450 deg".into())?)?;
    /// let normalized=arc.normalized(CenterArcNormalizationLimits::new(6,15)).expect("complete shapes");
    /// let checked=normalized.check_arguments(&FormulaNamespace::new([])?).expect("actual field kinds");
    /// assert_eq!(checked.argument(CenterArcArgument::Radius).kind(),FormulaKind::Length);
    /// // Radius division and sweep geometry remain unevaluated.
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Original first expression refusal or actual root-kind mismatch,retaining original owner,
    /// role/span and cause. No accepted scalar/geometry prefix or fake program position escapes.
    pub fn check_arguments<'a>(
        &'a self,
        namespace: &FormulaNamespace<'a>,
    ) -> Result<CheckedCenterArcConstruction<'a>, CenterArcCheckError<'a>>
    where
        's: 'a,
    {
        let normalized: &NormalizedCenterArcConstruction<'a> = self;
        let radius = check(normalized, namespace, CenterArcArgument::Radius)?;
        let start_angle = check(normalized, namespace, CenterArcArgument::StartAngle)?;
        let sweep_angle = check(normalized, namespace, CenterArcArgument::SweepAngle)?;
        Ok(CheckedCenterArcConstruction {
            normalized,
            radius,
            start_angle,
            sweep_angle,
        })
    }
}
