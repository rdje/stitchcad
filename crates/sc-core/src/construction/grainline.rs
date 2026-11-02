//! Placement inputs refer to actual semantic grain intent; no arrow geometry is generated here.
use crate::ontology::{DirectedRange, EntityId, GrainAlignment, GrainReferenceRole, Grainline};
use core::fmt;

/// Original target or first local source refusal, without an invented formula location.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrainlinePlacementError {
    /// An operation cannot require its own identity as a pre-existing semantic target.
    SelfTarget(EntityId),
    /// Actual supplied semantic record differs from the named target.
    TargetIdentity {
        /// Original required target.
        expected: EntityId,
        /// Actual provided target.
        actual: EntityId,
    },
    /// The operation cannot consume a source edge it would itself create.
    SelfSource {
        /// Actual placement operation.
        operation: EntityId,
        /// First actual role in arrow/alignment/stripe/plaid order.
        role: GrainReferenceRole,
        /// Exact original directed range, retaining bounds and persistent edge tag.
        reference: DirectedRange,
    },
}
impl fmt::Display for GrainlinePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelfTarget(operation) => {
                write!(
                    f,
                    "grainline placement {operation} depends on its own target identity"
                )
            }
            Self::TargetIdentity { expected, actual } => {
                write!(
                    f,
                    "grainline target {actual} differs from required {expected}"
                )
            }
            Self::SelfSource {
                operation,
                role,
                reference,
            } => {
                write!(
                    f,
                    "grainline placement {operation} {role:?} depends on its own source {}",
                    reference.range.edge()
                )
            }
        }
    }
}
impl std::error::Error for GrainlinePlacementError {}

/// Placement of a named existing semantic grainline using its actual arrow/alignment/print intent.
/// The target supplies its Piece owner and directed references. No copied direction,default angle,
/// new identity,generated arrow,current source proof or physical alignment certificate is provided.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GrainlinePlacement {
    operation: EntityId,
    grainline: EntityId,
}
impl GrainlinePlacement {
    // Classify every original field independently of the held-content resource walk.
    pub(crate) fn geometry_input_guard(&self) {
        let Self {
            operation,
            grainline,
        } = self;
        let _: [&EntityId; 2] = [operation, grainline];
    }

    // Exhaustive owner view: new fields must be classified before resource accounting compiles.
    pub(crate) fn resource_fields(&self) {
        use super::operation_input::resources::fixed;
        let Self {
            operation,
            grainline,
        } = self;
        fixed(operation);
        fixed(grainline);
    }

    /// Retain the original target identity; source association follows through bind_grainline.
    /// ```
    /// use sc_core::{construction::GrainlinePlacement,ontology::EntityId};
    /// let placement=GrainlinePlacement::new(EntityId::from_bits(10),EntityId::from_bits(99))?;
    /// assert_eq!(placement.grainline_id(),EntityId::from_bits(99));
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Refuses the same identity for this operation and its required pre-existing target.
    pub fn new(operation: EntityId, grainline: EntityId) -> Result<Self, GrainlinePlacementError> {
        if operation == grainline {
            return Err(GrainlinePlacementError::SelfTarget(operation));
        }
        Ok(Self {
            operation,
            grainline,
        })
    }
    /// Actual operation identity, without issuance or publication authority.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Required existing semantic grainline, not an allocated result identity.
    #[must_use]
    pub const fn grainline_id(&self) -> EntityId {
        self.grainline
    }

    /// Bind the actual supplied record and inspect every original reference role for self-supply.
    /// Optional print references stay optional; repeated sources remain distinct role occurrences.
    /// This proves target association and excludes definite local self-supply only. Current root
    /// membership,Piece/ledger coverage,global order/frame,angle resolution and geometry stay owned.
    /// # Errors
    /// First the actual target identity,then the first same-operation source in the actual grainline's
    /// arrow/alignment/stripe/plaid order. No accepted prefix or substituted direction escapes.
    pub fn bind_grainline<'a>(
        &'a self,
        grainline: &'a Grainline,
    ) -> Result<BoundGrainlinePlacement<'a>, GrainlinePlacementError> {
        if grainline.id() != self.grainline {
            return Err(GrainlinePlacementError::TargetIdentity {
                expected: self.grainline,
                actual: grainline.id(),
            });
        }
        for (role, reference) in grainline.references() {
            if reference.range.edge().creator() == self.operation {
                return Err(GrainlinePlacementError::SelfSource {
                    operation: self.operation,
                    role,
                    reference,
                });
            }
        }
        Ok(BoundGrainlinePlacement {
            placement: self,
            grainline,
        })
    }
}

/// Authentic original placement/semantic-source association; neither values nor mark geometry.
/// ```compile_fail
/// fn forge<'a>(placement:&'a sc_core::construction::GrainlinePlacement,
///     grainline:&'a sc_core::ontology::Grainline)->sc_core::construction::BoundGrainlinePlacement<'a> {
///     sc_core::construction::BoundGrainlinePlacement { placement,grainline }
/// }
/// ```
/// ```compile_fail
/// fn escape(placement:sc_core::construction::GrainlinePlacement,grainline:sc_core::ontology::Grainline)
///     ->sc_core::construction::BoundGrainlinePlacement<'static> {
///     placement.bind_grainline(&grainline).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundGrainlinePlacement<'a> {
    placement: &'a GrainlinePlacement,
    grainline: &'a Grainline,
}
impl<'a> BoundGrainlinePlacement<'a> {
    /// Actual placement input that requested this target.
    #[must_use]
    pub const fn placement(self) -> &'a GrainlinePlacement {
        self.placement
    }
    /// Actual supplied semantic record;its Piece/arrow/print references remain attached.
    #[must_use]
    pub const fn grainline(self) -> &'a Grainline {
        self.grainline
    }
    /// Actual alignment field, preserving ParallelTo versus explicit/Formula/Profile angle intent.
    #[must_use]
    pub const fn alignment(self) -> &'a GrainAlignment {
        &self.grainline.definition().alignment
    }
    /// Actual directed source occurrences in original arrow/alignment/stripe/plaid role order.
    /// This delegates to the semantic record; no default optional reference or angle is inferred.
    pub fn references(&self) -> impl Iterator<Item = (GrainReferenceRole, DirectedRange)> + '_ {
        self.grainline.references()
    }
}
