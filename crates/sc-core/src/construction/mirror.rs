//! Explicit reflected Piece-copy inputs; result identities and geometry remain later obligations.
use crate::ontology::{DirectedRange, EntityId, PointRef};
use core::fmt;

mod source;
pub use source::{
    BoundMirrorPieceCopy, MirrorPieceEdge, MirrorPieceEdgeLocation, MirrorPieceEdgeSource,
    MirrorPieceSourceError, MirrorPieceSourceLimits, MirrorPieceSourceRefusal,
};
mod cut;
pub use cut::{
    BoundMirrorPieceCutPolicy, MirrorPieceCutPolicy, MirrorPieceCutPolicyError,
    MirrorPieceCutPolicyRefusal, MirrorPieceFoldIntent,
};
mod metadata;
pub use metadata::{
    BoundMirrorPieceMetadata, MirrorPieceFieldIntent, MirrorPieceMetadataError,
    MirrorPieceMetadataLimits, MirrorPieceMetadataPolicy, MirrorPieceMetadataRefusal,
};
mod roles;
pub use roles::{
    BoundMirrorPieceEdgeRoles, MirrorPieceRoleBindingError, MirrorPieceRoleBindingRefusal,
};
mod identity;
pub use identity::{
    DeclaredMirrorPieceIdentity, MirrorPieceIdentityError, MirrorPieceIdentityRefusal,
};

/// Deliberately authored reflection-axis mode,not inferred from selected geometry.
/// Actual nondegeneracy,straightness,common frame and current sources remain G2/program checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorAxis {
    /// Axis through two original point references;source order remains authored even though
    /// reversing a valid geometric axis would leave its reflection unchanged.
    ThroughPoints {
        /// Actual first point reference.
        start: PointRef,
        /// Actual second point reference.
        end: PointRef,
    },
    /// Axis along an explicitly selected directed range;no tangent/chord is guessed for a curve.
    AlongRange(DirectedRange),
}
/// Definite local self-supply,with the exact original field and no fake formula context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceCopyError {
    /// Required existing Piece cannot have this operation's own identity.
    SelfPiece(EntityId),
    /// Actual first axis point depends on this operation.
    SelfAxisStart {
        /// Original operation identity.
        operation: EntityId,
        /// Original first point and persistent tag.
        point: PointRef,
    },
    /// Actual second axis point depends on this operation.
    SelfAxisEnd {
        /// Original operation identity.
        operation: EntityId,
        /// Original second point and persistent tag.
        point: PointRef,
    },
    /// Actual range-axis source depends on this operation.
    SelfAxisRange {
        /// Original operation identity.
        operation: EntityId,
        /// Original directed range and exact source-parameter frame.
        range: DirectedRange,
    },
}
impl fmt::Display for MirrorPieceCopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (operation, role) = match self {
            Self::SelfPiece(operation) => (*operation, "Piece"),
            Self::SelfAxisStart { operation, .. } => (*operation, "axis start"),
            Self::SelfAxisEnd { operation, .. } => (*operation, "axis end"),
            Self::SelfAxisRange { operation, .. } => (*operation, "axis range"),
        };
        write!(
            f,
            "mirror copy {operation} depends on its own {role} source"
        )
    }
}
impl std::error::Error for MirrorPieceCopyError {}

/// Input declaration for creating a reflected copy of a named existing Piece.
/// This explicitly requests copying rather than replacing the source. It supplies no new Piece
/// identity or edge/point tags. Metadata/cut-plan/sewing/handedness policy,result manifest,current
/// root/frame/geometry,stable source mappings and command publication remain separate obligations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceCopy {
    operation: EntityId,
    source_piece: EntityId,
    axis: MirrorAxis,
}
impl MirrorPieceCopy {
    /// Retain the actual source and explicit axis mode,excluding definite local self-supply.
    /// Repeated points and a range's authored traversal remain unchanged;this grants no valid axis.
    /// ```
    /// use sc_core::{construction::*,ontology::*};
    /// let point=|id|PointRef::new(EntityId::from_bits(id),LocalTag::FIRST);
    /// let input=MirrorPieceCopy::new(EntityId::from_bits(10),EntityId::from_bits(50),
    ///     MirrorAxis::ThroughPoints { start:point(99),end:point(80) })?;
    /// assert_eq!(input.dependencies().count(),2);
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// First the source Piece,then the actual axis fields in start/end or range order.
    pub fn new(
        operation: EntityId,
        source_piece: EntityId,
        axis: MirrorAxis,
    ) -> Result<Self, MirrorPieceCopyError> {
        if source_piece == operation {
            return Err(MirrorPieceCopyError::SelfPiece(operation));
        }
        let input = Self {
            operation,
            source_piece,
            axis,
        };
        for dependency in input.dependencies() {
            let source = dependency.source();
            if source.creator() == operation {
                return Err(match source {
                    MirrorAxisSource::Start(point) => MirrorPieceCopyError::SelfAxisStart {
                        operation,
                        point: *point,
                    },
                    MirrorAxisSource::End(point) => MirrorPieceCopyError::SelfAxisEnd {
                        operation,
                        point: *point,
                    },
                    MirrorAxisSource::Range(range) => MirrorPieceCopyError::SelfAxisRange {
                        operation,
                        range: *range,
                    },
                });
            }
        }
        Ok(input)
    }
    /// Original operation identity,not an allocated copy/result identity.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Actual required existing Piece;no registry or semantic copying proof follows.
    #[must_use]
    pub const fn source_piece(&self) -> EntityId {
        self.source_piece
    }
    /// Actual original axis variant and fields.
    #[must_use]
    pub const fn axis(&self) -> &MirrorAxis {
        &self.axis
    }
    /// Complete original axis-source occurrences;the source Piece remains a separate dependency.
    #[must_use]
    pub const fn dependency_count(&self) -> usize {
        match self.axis {
            MirrorAxis::ThroughPoints { .. } => 2,
            MirrorAxis::AlongRange(_) => 1,
        }
    }
    /// Authentic axis-source fields in original mode order,including repeated point uses.
    /// This enumerates axis inputs only,not the whole containing program dependency graph.
    pub fn dependencies(&self) -> impl DoubleEndedIterator<Item = MirrorAxisDependency<'_>> + '_ {
        let fields = match &self.axis {
            MirrorAxis::ThroughPoints { start, end } => [
                Some(MirrorAxisSource::Start(start)),
                Some(MirrorAxisSource::End(end)),
            ],
            MirrorAxis::AlongRange(range) => [Some(MirrorAxisSource::Range(range)), None],
        };
        fields
            .into_iter()
            .flatten()
            .map(move |source| MirrorAxisDependency {
                input: self,
                source,
            })
    }
}

/// Axis inspection data;only a private dependency wrapper certifies its original owning input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorAxisSource<'a> {
    /// Original first point field.
    Start(&'a PointRef),
    /// Original second point field.
    End(&'a PointRef),
    /// Original directed-range field.
    Range(&'a DirectedRange),
}
impl MirrorAxisSource<'_> {
    /// Creating operation of this inspected source,without currentness or prior-order proof.
    #[must_use]
    pub const fn creator(self) -> EntityId {
        match self {
            Self::Start(point) | Self::End(point) => point.creator(),
            Self::Range(range) => range.range.edge().creator(),
        }
    }
}
/// Actual original copy/axis-field occurrence,without evaluated geometry or read authority.
/// ```compile_fail
/// fn forge<'a>(input:&'a sc_core::construction::MirrorPieceCopy,
///     source:sc_core::construction::MirrorAxisSource<'a>)->sc_core::construction::MirrorAxisDependency<'a> {
///     sc_core::construction::MirrorAxisDependency { input,source }
/// }
/// ```
/// ```compile_fail
/// fn escape(input:sc_core::construction::MirrorPieceCopy)->sc_core::construction::MirrorAxisDependency<'static> {
///     input.dependencies().next().unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct MirrorAxisDependency<'a> {
    input: &'a MirrorPieceCopy,
    source: MirrorAxisSource<'a>,
}
impl<'a> MirrorAxisDependency<'a> {
    /// Actual input whose original axis supplied this field.
    #[must_use]
    pub const fn input(self) -> &'a MirrorPieceCopy {
        self.input
    }
    /// Actual original field borrow and role;no foreign axis source can replace this view.
    #[must_use]
    pub const fn source(self) -> MirrorAxisSource<'a> {
        self.source
    }
}
