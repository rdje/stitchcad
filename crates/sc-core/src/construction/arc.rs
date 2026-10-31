//! Explicit center/radius/start/sweep declaration; no implicit alternate arc or endpoint entities.
use crate::{
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{FormulaExpressionDocument, FormulaKind},
};
use core::fmt;
mod normalized;
pub use normalized::{
    CenterArcNormalizationError, CenterArcNormalizationLimits, CenterArcNormalizationRefusal,
    NormalizedCenterArcConstruction,
};
mod check;
pub use check::{CenterArcCheckError, CenterArcCheckRefusal, CheckedCenterArcConstruction};

/// Actual scalar field in the explicit center-based arc variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CenterArcArgument {
    /// Radius formula; Length kind does not prove a positive geometric radius.
    Radius,
    /// Start-direction formula, preserving authored Angle sign and full turns before geometry.
    StartAngle,
    /// Signed sweep formula; no modulo, nonzero or supported-sweep proof is inferred.
    SweepAngle,
}
impl CenterArcArgument {
    /// Required root kind from this actual closed argument role.
    #[must_use]
    pub const fn required_kind(self) -> FormulaKind {
        match self {
            Self::Radius => FormulaKind::Length,
            Self::StartAngle | Self::SweepAngle => FormulaKind::Angle,
        }
    }
}
/// Definite local self-supply for the actual center reference; no global graph path is invented.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CenterArcConstructionError {
    operation: EntityId,
    center: PointRef,
}
impl CenterArcConstructionError {
    /// Original supplied creating-operation identity.
    #[must_use]
    pub const fn operation(self) -> EntityId {
        self.operation
    }
    /// Original center whose creator is this same operation, including its persistent tag.
    #[must_use]
    pub const fn center(self) -> PointRef {
        self.center
    }
}
impl fmt::Display for CenterArcConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "arc center depends on its own creating operation {}",
            self.operation
        )
    }
}
impl std::error::Error for CenterArcConstructionError {}

/// Authored center-based arc with original scalar documents and one persistent logical edge role.
/// No endpoint PointRefs are declared or inferred. This type cannot be mistaken for an arc through
/// points. Radius/sweep domain,center currentness,frame,geometry,error budgets and publication stay
/// separate. Original formula sign/turns remain intact; construction evaluates no values.
/// ```compile_fail
/// fn rewrite(arc:&mut sc_core::construction::CenterArcConstruction) { arc.radius=todo!(); }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CenterArcConstruction {
    operation: EntityId,
    center: PointRef,
    radius: FormulaExpressionDocument,
    start_angle: FormulaExpressionDocument,
    sweep_angle: FormulaExpressionDocument,
}
impl CenterArcConstruction {
    /// Retain original center and syntax/input-validated documents; reject definite self-supply.
    /// Scalar kinds are checked in the later actual normalized owner, not inferred from text.
    /// # Errors
    /// Original operation and center reference when that center cannot be a prior supplier.
    pub fn new(
        operation: EntityId,
        center: PointRef,
        radius: FormulaExpressionDocument,
        start_angle: FormulaExpressionDocument,
        sweep_angle: FormulaExpressionDocument,
    ) -> Result<Self, CenterArcConstructionError> {
        if center.creator() == operation {
            return Err(CenterArcConstructionError { operation, center });
        }
        Ok(Self {
            operation,
            center,
            radius,
            start_angle,
            sweep_angle,
        })
    }
    /// Original creating-operation identity, without issuance/currentness authority.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Actual supplied center reference; no coordinate/frame/registry proof follows.
    #[must_use]
    pub const fn center(&self) -> PointRef {
        self.center
    }
    /// Single stable logical edge output; no endpoint point identity is manufactured.
    #[must_use]
    pub const fn output(&self) -> EdgeRef {
        EdgeRef::new(self.operation, LocalTag::FIRST)
    }
    /// Actual original document for this scalar argument role.
    #[must_use]
    pub const fn argument(&self, argument: CenterArcArgument) -> &FormulaExpressionDocument {
        match argument {
            CenterArcArgument::Radius => &self.radius,
            CenterArcArgument::StartAngle => &self.start_angle,
            CenterArcArgument::SweepAngle => &self.sweep_angle,
        }
    }
    /// Authentic declared center dependency tied to this exact original field and owner.
    #[must_use]
    pub const fn center_dependency(&self) -> CenterArcPointDependency<'_> {
        CenterArcPointDependency { arc: self }
    }
}
/// Original center-field dependency; no resolved point or executed-read certificate.
/// ```compile_fail
/// fn forge(arc:&sc_core::construction::CenterArcConstruction)->sc_core::construction::CenterArcPointDependency<'_> {
///     sc_core::construction::CenterArcPointDependency { arc }
/// }
/// ```
/// ```compile_fail
/// fn escape(arc:sc_core::construction::CenterArcConstruction)->sc_core::construction::CenterArcPointDependency<'static> {
///     arc.center_dependency()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct CenterArcPointDependency<'a> {
    arc: &'a CenterArcConstruction,
}
impl<'a> CenterArcPointDependency<'a> {
    /// Actual original declaration, not a supplied equal-looking replacement.
    #[must_use]
    pub const fn arc(self) -> &'a CenterArcConstruction {
        self.arc
    }
    /// Exact original center field, retaining its operation and persistent local tag.
    #[must_use]
    pub const fn point(self) -> PointRef {
        self.arc.center()
    }
}
