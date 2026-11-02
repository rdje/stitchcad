//! Explicit authored control variants and original dependencies; exact geometry promotion is G2.
use crate::ontology::{EdgeRef, EntityId, LocalTag, PointRef};
use core::fmt;

/// Explicit authored input degree, not an evaluated curve or a replacement geometry primitive.
/// G2's canonical geometry is cubic; quadratic source intent requires exact degree elevation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BezierDegree {
    /// Three original controls; no synthetic cubic controls are inferred here.
    Quadratic,
    /// Four original controls in their actual authored order.
    Cubic,
}
impl BezierDegree {
    /// Complete original point-source population for the explicit input variant.
    #[must_use]
    pub const fn control_count(self) -> usize {
        match self {
            Self::Quadratic => 3,
            Self::Cubic => 4,
        }
    }
}

/// Closed original control layout; no integer degree, arbitrary vector or implicit inference.
/// Higher-order curves/NURBS remain outside the supported contract.
/// ```compile_fail
/// fn incomplete(start:sc_core::ontology::PointRef,end:sc_core::ontology::PointRef) {
///     let controls=sc_core::construction::BezierControlPoints::Cubic { start,end };
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BezierControlPoints {
    /// Actual three-control quadratic source declaration.
    Quadratic {
        /// Original first endpoint reference.
        start: PointRef,
        /// Original single interior control reference.
        control: PointRef,
        /// Original final endpoint reference.
        end: PointRef,
    },
    /// Actual four-control cubic source declaration.
    Cubic {
        /// Original first endpoint reference.
        start: PointRef,
        /// Original first interior control reference.
        first_control: PointRef,
        /// Original second interior control reference.
        second_control: PointRef,
        /// Original final endpoint reference.
        end: PointRef,
    },
}
impl BezierControlPoints {
    /// Explicit original variant, independent of coordinates or coincident references.
    #[must_use]
    pub const fn degree(&self) -> BezierDegree {
        match self {
            Self::Quadratic { .. } => BezierDegree::Quadratic,
            Self::Cubic { .. } => BezierDegree::Cubic,
        }
    }
}
/// Actual original field role; a quadratic has no SecondControl occurrence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BezierPointRole {
    /// Original first endpoint.
    Start,
    /// Quadratic's only control or cubic's first control.
    FirstControl,
    /// Cubic's actual second control, never synthesized for a quadratic.
    SecondControl,
    /// Original final endpoint.
    End,
}
/// Definite local self-supply with the original operation,first actual control role and reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BezierConstructionError {
    operation: EntityId,
    role: BezierPointRole,
    point: PointRef,
}
impl BezierConstructionError {
    /// Original supplied creating-operation identity.
    #[must_use]
    pub const fn operation(self) -> EntityId {
        self.operation
    }
    /// Original first self-dependent field role in this explicit variant.
    #[must_use]
    pub const fn role(self) -> BezierPointRole {
        self.role
    }
    /// Exact refused PointRef, including its persistent local tag.
    #[must_use]
    pub const fn point(self) -> PointRef {
        self.point
    }
}
impl fmt::Display for BezierConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bezier {:?} point depends on its own creating operation {}",
            self.role, self.operation
        )
    }
}
impl std::error::Error for BezierConstructionError {}

/// Authored quadratic/cubic control declaration with one persistent logical edge role.
/// Source references,degree and order survive unchanged. No coordinates,exact promotion,curve,
/// nondegeneracy,frame,current/prior source proof or identity issuance/publication is granted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BezierConstruction {
    operation: EntityId,
    controls: BezierControlPoints,
}
impl BezierConstruction {
    // Exhaustive owner view: new fields must be classified before resource accounting compiles.
    pub(crate) fn resource_fields(&self) {
        use super::operation_input::resources::fixed;
        let Self {
            operation,
            controls,
        } = self;
        fixed(operation);
        fixed(controls);
    }

    /// Retain the explicit original control layout and reject definite local self-supply.
    /// Controls are never sorted,deduplicated,degree-elevated or inferred from coincident geometry.
    /// ```
    /// use sc_core::{construction::*,ontology::{EntityId,PointRef,LocalTag}};
    /// let point=|id|PointRef::new(EntityId::from_bits(id),LocalTag::FIRST);
    /// let curve=BezierConstruction::new(EntityId::from_bits(10),BezierControlPoints::Quadratic {
    ///     start:point(99),control:point(80),end:point(70) })?;
    /// assert_eq!(curve.degree(),BezierDegree::Quadratic);
    /// assert_eq!(curve.dependencies().count(),3);
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Original first same-operation supplier in actual control order;no accepted declaration,
    /// invented formula name/span or global cycle/currentness proof is returned on refusal.
    pub fn new(
        operation: EntityId,
        controls: BezierControlPoints,
    ) -> Result<Self, BezierConstructionError> {
        let construction = Self {
            operation,
            controls,
        };
        for dependency in construction.dependencies() {
            let point = dependency.point();
            if point.creator() == operation {
                return Err(BezierConstructionError {
                    operation,
                    role: dependency.role(),
                    point,
                });
            }
        }
        Ok(construction)
    }
    /// Original creating-operation identity, not an issued/current registry fact.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Exact original named control fields;no equal-looking replacement is attached.
    #[must_use]
    pub const fn controls(&self) -> &BezierControlPoints {
        &self.controls
    }
    /// Explicit authored input degree; canonical cubic geometry remains separate.
    #[must_use]
    pub const fn degree(&self) -> BezierDegree {
        self.controls.degree()
    }
    /// Complete declared original source occurrences, including repeated PointRefs.
    #[must_use]
    pub const fn dependency_count(&self) -> usize {
        self.degree().control_count()
    }
    /// Single stable logical edge output;no point entities are issued or inferred.
    #[must_use]
    pub const fn output(&self) -> EdgeRef {
        EdgeRef::new(self.operation, LocalTag::FIRST)
    }
    /// All actual fields in their explicit variant's order,including repetitions.
    /// Views borrow both this exact declaration and the actual underlying PointRef field.
    pub fn dependencies(&self) -> impl DoubleEndedIterator<Item = BezierPointDependency<'_>> + '_ {
        use BezierPointRole as R;
        let fields = match &self.controls {
            BezierControlPoints::Quadratic {
                start,
                control,
                end,
            } => [
                Some((R::Start, start)),
                Some((R::FirstControl, control)),
                None,
                Some((R::End, end)),
            ],
            BezierControlPoints::Cubic {
                start,
                first_control,
                second_control,
                end,
            } => [
                Some((R::Start, start)),
                Some((R::FirstControl, first_control)),
                Some((R::SecondControl, second_control)),
                Some((R::End, end)),
            ],
        };
        fields
            .into_iter()
            .flatten()
            .map(move |(role, point)| BezierPointDependency {
                construction: self,
                role,
                point,
            })
    }
}

/// One authentic original field occurrence; no resolved-source or executed-read certificate.
/// ```compile_fail
/// fn forge<'a>(curve:&'a sc_core::construction::BezierConstruction,point:&'a sc_core::ontology::PointRef)
///     ->sc_core::construction::BezierPointDependency<'a> {
///     sc_core::construction::BezierPointDependency { construction:curve,
///         role:sc_core::construction::BezierPointRole::Start,point }
/// }
/// ```
/// ```compile_fail
/// fn escape(curve:sc_core::construction::BezierConstruction)->sc_core::construction::BezierPointDependency<'static> {
///     curve.dependencies().next().unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BezierPointDependency<'a> {
    construction: &'a BezierConstruction,
    role: BezierPointRole,
    point: &'a PointRef,
}
impl<'a> BezierPointDependency<'a> {
    /// Exact original declaration that supplied this field.
    #[must_use]
    pub const fn construction(self) -> &'a BezierConstruction {
        self.construction
    }
    /// Actual original role in that declaration's explicit control layout.
    #[must_use]
    pub const fn role(self) -> BezierPointRole {
        self.role
    }
    /// Exact original PointRef value, including creator and persistent local tag.
    #[must_use]
    pub const fn point(self) -> PointRef {
        *self.point
    }
    /// Actual underlying immutable field borrow;no caller-supplied reference can substitute.
    #[must_use]
    pub const fn point_ref(self) -> &'a PointRef {
        self.point
    }
}
