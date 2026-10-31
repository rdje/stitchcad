//! Explicit spatial source intent; region classification and transformed topology remain G2/G3.
use super::{SlashSpreadOpening, SlashSpreadOpeningMode};
use crate::{
    ontology::{DirectedRange, EntityId, PointRef},
    recipe::FormulaExpressionDocument,
};
use core::fmt;
mod contours;
mod references;
pub use references::{
    BoundSlashSpreadRegistrationPlan, BoundSlashSpreadSourcePlan, SlashSpreadBridgeEndpoint,
    SlashSpreadGlobalSourceIntent, SlashSpreadReferenceMatch, SlashSpreadReferenceMatchKind,
    SlashSpreadReferenceOrigin, SlashSpreadReferenceQuery, SlashSpreadReferenceQueryError,
    SlashSpreadReferenceQueryLimits, SlashSpreadReferenceQueryRefusal,
    SlashSpreadRegisteredSourceRequest, SlashSpreadRegistrationDisposition,
    SlashSpreadRegistrationIntent, SlashSpreadRegistrationPlan, SlashSpreadRegistrationPlanError,
    SlashSpreadRegistrationPlanLimits, SlashSpreadRegistrationPlanRefusal,
    SlashSpreadSourceDisposition, SlashSpreadSourcePlan, SlashSpreadSourcePlanError,
    SlashSpreadSourcePlanLimits, SlashSpreadSourcePlanRefusal, SlashSpreadSourceRequest,
};
mod edges;
pub use contours::{
    AdmittedSlashSpreadContours, SlashSpreadContourEdge, SlashSpreadContourError,
    SlashSpreadContourLimits, SlashSpreadContourLocation, SlashSpreadContourRefusal,
    SlashSpreadContourUse, SlashSpreadContours,
};
mod output;
mod roles;
pub use edges::{
    AdmittedSlashSpreadEdges, SlashSpreadBridgeDirection, SlashSpreadCutEnd, SlashSpreadEdge,
    SlashSpreadEdgeCandidate, SlashSpreadEdgeManifest, SlashSpreadEdgeManifestError,
    SlashSpreadEdgeManifestLimits, SlashSpreadEdgeManifestRefusal, SlashSpreadEdgeOrigin,
    SlashSpreadSourceMotion,
};
pub use output::{SlashSpreadBoundaryPolicy, SlashSpreadFullnessIntent, SlashSpreadOutputIntent};
pub use roles::{
    BoundSlashSpreadEdgeRoles, SlashSpreadRoleBindingError, SlashSpreadRoleBindingRefusal,
};

/// Moving bank relative to the explicitly directed straight cut in the Piece's drafting frame.
/// A geometry executor must prove a unique applicable region; no fabric-face or winding default.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SlashSpreadSide {
    /// Move the cut's left bank;the other bank remains stationary.
    Left,
    /// Move the cut's right bank;the other bank remains stationary.
    Right,
}
/// Definite local self-supply; original actual fields survive without a fake formula location.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadConstructionError {
    /// Target Piece identity cannot be this same operation's required prior input.
    SelfPiece(EntityId),
    /// Original cut cannot be supplied by this operation.
    SelfCut {
        /// Actual operation.
        operation: EntityId,
        /// Original directed cut and exact parameter frame.
        cut: DirectedRange,
    },
    /// Original rotation pivot cannot be supplied by this operation.
    SelfPivot {
        /// Actual operation.
        operation: EntityId,
        /// Original pivot and persistent local tag.
        pivot: PointRef,
    },
    /// Original separation axis cannot be supplied by this operation.
    SelfDirection {
        /// Actual operation.
        operation: EntityId,
        /// Original directed movement axis and exact parameter frame.
        direction: DirectedRange,
    },
}
impl fmt::Display for SlashSpreadConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (operation, role) = match self {
            Self::SelfPiece(operation) => (*operation, "Piece"),
            Self::SelfCut { operation, .. } => (*operation, "cut"),
            Self::SelfPivot { operation, .. } => (*operation, "pivot"),
            Self::SelfDirection { operation, .. } => (*operation, "direction"),
        };
        write!(
            f,
            "slash/spread {operation} depends on its own {role} source"
        )
    }
}
impl std::error::Error for SlashSpreadConstructionError {}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Motion {
    Rotation {
        pivot: PointRef,
        opening: SlashSpreadOpening,
    },
    Separation {
        direction: DirectedRange,
        opening: SlashSpreadOpening,
    },
}
/// Authored slash/spread spatial inputs and their actual explicit amount component.
/// Rotation has a required pivot;separation has a required directed movement axis. Both select
/// the target Piece,straight cut and moving bank explicitly. No default region/pivot/normal axis.
/// Current Piece ownership/frame/coverage,straightness,unique bank,pivot incidence,geometric
/// domain,bridging/fullness policy and result topology remain unproved. No output tags are issued.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlashSpreadConstruction {
    operation: EntityId,
    piece: EntityId,
    cut: DirectedRange,
    moving_side: SlashSpreadSide,
    motion: Motion,
}
impl SlashSpreadConstruction {
    /// Retain explicit rotation sources and construct the actual Rotation Angle amount component.
    /// The geometry executor must validate pivot incidence and the unique supported moving bank.
    /// # Errors
    /// First local self-supply in Piece,cut,pivot order;no partial accepted declaration escapes.
    pub fn rotation(
        operation: EntityId,
        piece: EntityId,
        cut: DirectedRange,
        moving_side: SlashSpreadSide,
        pivot: PointRef,
        angle: FormulaExpressionDocument,
    ) -> Result<Self, SlashSpreadConstructionError> {
        Self {
            operation,
            piece,
            cut,
            moving_side,
            motion: Motion::Rotation {
                pivot,
                opening: SlashSpreadOpening::rotation(angle),
            },
        }
        .admit()
    }
    /// Retain explicit separation sources and construct the actual Separation Length component.
    /// Positive amount follows the supplied directed axis;no cut normal is inferred. Values and
    /// supported signs/domains remain numerical/geometric execution obligations.
    /// # Errors
    /// First local self-supply in Piece,cut,direction order;no partial accepted declaration escapes.
    /// ```compile_fail
    /// use sc_core::{construction::*,ontology::*,recipe::FormulaExpressionDocument};
    /// fn wrong(op:EntityId,piece:EntityId,cut:DirectedRange,pivot:PointRef,amount:FormulaExpressionDocument) {
    ///     let input=SlashSpreadConstruction::separation(op,piece,cut,SlashSpreadSide::Left,pivot,amount);
    /// }
    /// ```
    pub fn separation(
        operation: EntityId,
        piece: EntityId,
        cut: DirectedRange,
        moving_side: SlashSpreadSide,
        direction: DirectedRange,
        distance: FormulaExpressionDocument,
    ) -> Result<Self, SlashSpreadConstructionError> {
        Self {
            operation,
            piece,
            cut,
            moving_side,
            motion: Motion::Separation {
                direction,
                opening: SlashSpreadOpening::separation(distance),
            },
        }
        .admit()
    }
    fn admit(self) -> Result<Self, SlashSpreadConstructionError> {
        if self.piece == self.operation {
            return Err(SlashSpreadConstructionError::SelfPiece(self.operation));
        }
        for dependency in self.dependencies() {
            let source = dependency.source();
            if source.creator() == self.operation {
                let operation = self.operation;
                return Err(match source {
                    SlashSpreadSource::Cut(cut) => SlashSpreadConstructionError::SelfCut {
                        operation,
                        cut: *cut,
                    },
                    SlashSpreadSource::Pivot(pivot) => SlashSpreadConstructionError::SelfPivot {
                        operation,
                        pivot: *pivot,
                    },
                    SlashSpreadSource::Direction(direction) => {
                        SlashSpreadConstructionError::SelfDirection {
                            operation,
                            direction: *direction,
                        }
                    }
                });
            }
        }
        Ok(self)
    }
    /// Actual operation identity,without identity issuance or publication authority.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Original target Piece; no current registry,frame or containment proof follows.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        self.piece
    }
    /// Actual original directed cut field and exact source-parameter bounds.
    #[must_use]
    pub const fn cut(&self) -> &DirectedRange {
        &self.cut
    }
    /// Explicit moving bank relative to that cut traversal,not a fabric cutting side.
    #[must_use]
    pub const fn moving_side(&self) -> SlashSpreadSide {
        self.moving_side
    }
    /// Explicit amount mode,selected by the actual construction variant.
    #[must_use]
    pub const fn mode(&self) -> SlashSpreadOpeningMode {
        self.opening().mode()
    }
    /// Actual embedded amount component;its normalized proof remains tied to this original field.
    #[must_use]
    pub const fn opening(&self) -> &SlashSpreadOpening {
        match &self.motion {
            Motion::Rotation { opening, .. } | Motion::Separation { opening, .. } => opening,
        }
    }
    /// Actual pivot field for rotation;separation invents no pivot.
    #[must_use]
    pub const fn pivot(&self) -> Option<&PointRef> {
        match &self.motion {
            Motion::Rotation { pivot, .. } => Some(pivot),
            Motion::Separation { .. } => None,
        }
    }
    /// Actual movement-axis field for separation;rotation invents no translation direction.
    #[must_use]
    pub const fn direction(&self) -> Option<&DirectedRange> {
        match &self.motion {
            Motion::Separation { direction, .. } => Some(direction),
            Motion::Rotation { .. } => None,
        }
    }
    /// Both actual geometry-source occurrences:cut first,then pivot or movement axis.
    /// Repeated edge uses remain distinct roles. Target Piece and amount formula dependencies
    /// remain available separately;this iterator alone is not the whole operation dependency graph.
    pub fn dependencies(
        &self,
    ) -> impl ExactSizeIterator<Item = SlashSpreadDependency<'_>> + DoubleEndedIterator + '_ {
        let motion = match &self.motion {
            Motion::Rotation { pivot, .. } => SlashSpreadSource::Pivot(pivot),
            Motion::Separation { direction, .. } => SlashSpreadSource::Direction(direction),
        };
        [SlashSpreadSource::Cut(&self.cut), motion]
            .into_iter()
            .map(move |source| SlashSpreadDependency {
                construction: self,
                source,
            })
    }
}

/// Structural source inspection data;only the private dependency wrapper certifies its owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadSource<'a> {
    /// Actual directed cut field.
    Cut(&'a DirectedRange),
    /// Actual rotation pivot field.
    Pivot(&'a PointRef),
    /// Actual separation movement-axis field.
    Direction(&'a DirectedRange),
}
impl SlashSpreadSource<'_> {
    /// Creating operation of this inspected source;not a currentness or prior-order proof.
    #[must_use]
    pub const fn creator(self) -> EntityId {
        match self {
            Self::Cut(range) | Self::Direction(range) => range.range.edge().creator(),
            Self::Pivot(point) => point.creator(),
        }
    }
}
/// Authentic original declaration/field occurrence;not a resolved source or executed-read proof.
/// ```compile_fail
/// fn forge<'a>(construction:&'a sc_core::construction::SlashSpreadConstruction,
///     source:sc_core::construction::SlashSpreadSource<'a>)->sc_core::construction::SlashSpreadDependency<'a> {
///     sc_core::construction::SlashSpreadDependency { construction,source }
/// }
/// ```
/// ```compile_fail
/// fn escape(input:sc_core::construction::SlashSpreadConstruction)
///     ->sc_core::construction::SlashSpreadDependency<'static> {
///     input.dependencies().next().unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SlashSpreadDependency<'a> {
    construction: &'a SlashSpreadConstruction,
    source: SlashSpreadSource<'a>,
}
impl<'a> SlashSpreadDependency<'a> {
    /// Actual original construction that supplied this field occurrence.
    #[must_use]
    pub const fn construction(self) -> &'a SlashSpreadConstruction {
        self.construction
    }
    /// Actual underlying field borrow and role;no foreign source can be attached to this view.
    #[must_use]
    pub const fn source(self) -> SlashSpreadSource<'a> {
        self.source
    }
}
