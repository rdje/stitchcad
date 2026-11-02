//! Real complete argument proofs; no coordinate values or constructed point are manufactured.
use super::{NormalizedPointConstruction, PointCoordinate};
use crate::recipe::{
    FormulaCheckedExpression, FormulaExpressionCheckError, FormulaKind, FormulaNameScope,
    FormulaNamespace, FormulaSourceSpan,
};
use core::fmt;

/// Original argument expression refusal or an actual completed root of the wrong coordinate kind.
#[derive(Debug)]
pub enum PointConstructionCheckRefusal<'a> {
    /// Original expression owner,failed node/span and typed name/operator/call cause.
    Expression(FormulaExpressionCheckError<'a, 'a>),
    /// Real checked root kind;the coordinate role requires Length.
    Kind {
        /// Actual statically inferred argument kind, not a caller annotation.
        actual: FormulaKind,
    },
}
/// Actual normalized point owner and first static argument failure, without partial accepted proof.
pub struct PointConstructionCheckError<'a> {
    normalized: &'a NormalizedPointConstruction<'a>,
    coordinate: PointCoordinate,
    refusal: PointConstructionCheckRefusal<'a>,
}
impl<'a> PointConstructionCheckError<'a> {
    /// Actual two-argument owner and original declaration.
    #[must_use]
    pub const fn normalized(&self) -> &'a NormalizedPointConstruction<'a> {
        self.normalized
    }
    /// Actual original coordinate argument role, without a fabricated recipe ordinal.
    #[must_use]
    pub const fn coordinate(&self) -> PointCoordinate {
        self.coordinate
    }
    /// Required kind from the real coordinate role.
    #[must_use]
    pub const fn required_kind(&self) -> FormulaKind {
        self.coordinate.required_kind()
    }
    /// Original typed expression or actual-root kind cause.
    #[must_use]
    pub const fn refusal(&self) -> &PointConstructionCheckRefusal<'a> {
        &self.refusal
    }
    /// Actual failed node span, or complete argument root span for a root-kind refusal.
    #[must_use]
    pub fn span(&self) -> FormulaSourceSpan {
        match &self.refusal {
            PointConstructionCheckRefusal::Expression(error) => error.span(),
            PointConstructionCheckRefusal::Kind { .. } => {
                self.normalized.coordinate(self.coordinate).root().span()
            }
        }
    }
    /// Existing canonical formula family; actual operation/argument context remains separate.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match &self.refusal {
            PointConstructionCheckRefusal::Expression(error) => error.token(),
            PointConstructionCheckRefusal::Kind { .. } => "formula_dimension",
        }
    }
}
impl fmt::Debug for PointConstructionCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointConstructionCheckError")
            .field("coordinate", &self.coordinate)
            .field("token", &self.token())
            .field("span", &self.span())
            .finish_non_exhaustive()
    }
}
impl fmt::Display for PointConstructionCheckError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "point {:?} argument: {}", self.coordinate, self.token())
    }
}
impl std::error::Error for PointConstructionCheckError<'_> {}

/// Both real Length argument proofs, pinned to the actual declaration/normalized arenas.
/// Static names/kinds only: no numeric coordinate, current reference, frame or geometry is certified.
/// ```compile_fail
/// fn replace(point:&mut sc_core::construction::CheckedPointConstruction<'_>) { point.x=todo!(); }
/// ```
/// ```compile_fail
/// fn escape(point:&sc_core::construction::NormalizedPointConstruction<'_>)
///     ->sc_core::construction::CheckedPointConstruction<'static> {
///     point.check_arguments(&sc_core::recipe::FormulaNamespace::new([]).unwrap()).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct CheckedPointConstruction<'a> {
    normalized: &'a NormalizedPointConstruction<'a>,
    x: FormulaCheckedExpression<'a, 'a, 'a>,
    y: FormulaCheckedExpression<'a, 'a, 'a>,
}
impl<'a> CheckedPointConstruction<'a> {
    /// Actual declaration owner,admitted policy and both original normalized arenas.
    #[must_use]
    pub const fn normalized(&self) -> &'a NormalizedPointConstruction<'a> {
        self.normalized
    }
    /// Actual complete coordinate proof; neither caller kinds nor a supplied node can substitute.
    #[must_use]
    pub const fn coordinate(
        &self,
        coordinate: PointCoordinate,
    ) -> &FormulaCheckedExpression<'a, 'a, 'a> {
        match coordinate {
            PointCoordinate::X => &self.x,
            PointCoordinate::Y => &self.y,
        }
    }
}
fn check<'a>(
    normalized: &'a NormalizedPointConstruction<'a>,
    namespace: &FormulaNamespace<'a>,
    coordinate: PointCoordinate,
) -> Result<FormulaCheckedExpression<'a, 'a, 'a>, PointConstructionCheckError<'a>> {
    use crate::construction::arguments::{check_expression, CheckRefusal as R};
    check_expression(
        normalized.coordinate(coordinate),
        namespace,
        coordinate.required_kind(),
    )
    .map_err(|cause| PointConstructionCheckError {
        normalized,
        coordinate,
        refusal: match cause {
            R::Expression(error) => PointConstructionCheckRefusal::Expression(error),
            R::Kind { actual } => PointConstructionCheckRefusal::Kind { actual },
        },
    })
}
impl<'s> NormalizedPointConstruction<'s> {
    /// Invoke the same complete argument checker through an actual borrowed visible-name scope.
    /// No map clone,source substitution,values or containing-program acceptance is introduced.
    /// # Errors
    /// Original actual argument/role/span and typed expression or required-kind refusal.
    pub fn check_arguments_in_scope<'a>(
        &'a self,
        names: FormulaNameScope<'_, 'a>,
    ) -> Result<CheckedPointConstruction<'a>, PointConstructionCheckError<'a>>
    where
        's: 'a,
    {
        self.check_arguments(names.namespace())
    }

    /// Check x then y against the actual supplied metadata namespace before any value read.
    /// Both complete expression proofs and Length roots must succeed before the paired proof
    /// exists. An earlier runtime fault cannot hide a later static argument refusal.
    /// ```
    /// use sc_core::{construction::*,ontology::EntityId,recipe::*};
    /// let point=PointConstruction::new(EntityId::from_bits(1),
    ///     FormulaExpressionDocument::new("1 mm/0".into())?,
    ///     FormulaExpressionDocument::new("2 mm".into())?);
    /// let normalized=point.normalized(PointConstructionNormalizationLimits::new(4,10)).expect("complete shapes");
    /// let checked=normalized.check_arguments(&FormulaNamespace::new([])?).expect("actual Length proofs");
    /// assert_eq!(checked.coordinate(PointCoordinate::X).kind(),FormulaKind::Length);
    /// // No division or geometry construction occurred.
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Original first expression/name/call refusal or actual root-kind mismatch. No accepted
    /// argument/proof prefix escapes and no recipe statement ordinal is invented.
    pub fn check_arguments<'a>(
        &'a self,
        namespace: &FormulaNamespace<'a>,
    ) -> Result<CheckedPointConstruction<'a>, PointConstructionCheckError<'a>>
    where
        's: 'a,
    {
        let normalized: &NormalizedPointConstruction<'a> = self;
        let x = check(normalized, namespace, PointCoordinate::X)?;
        let y = check(normalized, namespace, PointCoordinate::Y)?;
        Ok(CheckedPointConstruction { normalized, x, y })
    }
}
