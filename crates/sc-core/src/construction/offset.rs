//! Exact source/allowance association; G2 owns fragment outputs and bounded offset geometry.
use crate::ontology::{
    AllowanceWidth, CornerTreatment, DirectedRange, Direction, EdgeRef, EntityId,
    ProfileParameterRef, SeamAllowance,
};
use core::fmt;

/// Side relative to the explicitly declared traversal of the original source range.
/// Neither variant implies a Piece's inside/outside, fabric face or cutting orientation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OffsetSide {
    /// Left of the declared traversal.
    Left,
    /// Right of the declared traversal.
    Right,
}

/// Definite local self-supply in the original input, before any allowance binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetConstructionError {
    /// The operation cannot read an edge it would itself create.
    SelfSource {
        /// Actual operation identity.
        operation: EntityId,
        /// Original range, including edge tag and exact bounds.
        source: DirectedRange,
    },
    /// The same identity cannot name both this operation and its required allowance input.
    SelfAllowance(EntityId),
}
impl fmt::Display for OffsetConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelfSource { operation, source } => {
                write!(
                    f,
                    "offset {operation} depends on its own source {}",
                    source.range.edge()
                )
            }
            Self::SelfAllowance(operation) => {
                write!(
                    f,
                    "offset {operation} depends on its own allowance identity"
                )
            }
        }
    }
}
impl std::error::Error for OffsetConstructionError {}

/// Exact original-identity association refusal; no alias or topology repair is inferred.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetBindingError {
    /// The provided descriptor is not the declaration's named allowance.
    AllowanceIdentity {
        /// Original required descriptor identity.
        expected: EntityId,
        /// Actual provided descriptor identity.
        actual: EntityId,
    },
    /// Even an equal descriptor identity cannot substitute a different authored source edge.
    SourceEdge {
        /// Original selected range and parameter frame.
        source: DirectedRange,
        /// Actual edge held by the supplied allowance.
        allowance_edge: EdgeRef,
    },
}
impl fmt::Display for OffsetBindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllowanceIdentity { expected, actual } => {
                write!(
                    f,
                    "offset allowance {actual} differs from required {expected}"
                )
            }
            Self::SourceEdge {
                source,
                allowance_edge,
            } => {
                write!(
                    f,
                    "offset source {} differs from allowance edge {allowance_edge}",
                    source.range.edge()
                )
            }
        }
    }
}
impl std::error::Error for OffsetBindingError {}

/// Authored offset inputs: the canonical DirectedRange, explicit side and allowance id.
/// Width, corner and inclusion are read from the bound descriptor, never copied into this record.
/// This input stage declares no fixed edge count or output tags. Actual fragment intervals,
/// current registries, containing frame, width evaluation, error budget and publication stay owned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OffsetConstruction {
    operation: EntityId,
    source: DirectedRange,
    side: OffsetSide,
    allowance: EntityId,
}
impl OffsetConstruction {
    // Exhaustive owner view: new fields must be classified before resource accounting compiles.
    pub(crate) fn resource_fields(&self) {
        use super::operation_input::resources::fixed;
        let Self {
            operation,
            source,
            side,
            allowance,
        } = self;
        fixed(operation);
        fixed(source);
        fixed(side);
        fixed(allowance);
    }

    /// Retain exact original intent; numeric identity ordering is not recipe ordering.
    /// # Errors
    /// Refuses a same-operation source first, then a same-identity allowance dependency.
    pub fn new(
        operation: EntityId,
        source: DirectedRange,
        side: OffsetSide,
        allowance: EntityId,
    ) -> Result<Self, OffsetConstructionError> {
        if source.range.edge().creator() == operation {
            return Err(OffsetConstructionError::SelfSource { operation, source });
        }
        if allowance == operation {
            return Err(OffsetConstructionError::SelfAllowance(operation));
        }
        Ok(Self {
            operation,
            source,
            side,
            allowance,
        })
    }
    /// Original operation identity; no identities or outputs are issued.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Actual original range field, with its ascending exact bounds unchanged.
    #[must_use]
    pub const fn source(&self) -> &DirectedRange {
        &self.source
    }
    /// Explicit traversal relative to the original source parameterization.
    #[must_use]
    pub const fn direction(&self) -> Direction {
        self.source.direction
    }
    /// Explicit side relative to that traversal; no winding inference.
    #[must_use]
    pub const fn side(&self) -> OffsetSide {
        self.side
    }
    /// Original required descriptor identity, without a cached width or policy.
    #[must_use]
    pub const fn allowance_id(&self) -> EntityId {
        self.allowance
    }

    /// Associate the actual provided descriptor by identity and original source edge.
    /// The returned view borrows both original owners. This proves association only: root registry
    /// membership, current Piece/ledger coverage, formula/profile resolution and geometry remain
    /// separate checks. Historical edges are not silently retargeted to current fragments.
    /// ```
    /// use sc_core::{construction::*,ontology::*};
    /// let edge=EdgeRef::new(EntityId::from_bits(99),LocalTag::new(7));
    /// let input=OffsetConstruction::new(EntityId::from_bits(10),DirectedRange { range:EdgeRange::whole(edge),direction:Direction::Reversed },
    ///     OffsetSide::Left,EntityId::from_bits(80))?;
    /// assert_eq!(input.source().range.edge(),edge);
    /// assert_eq!(input.direction(),Direction::Reversed);
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// First the actual descriptor identity, then the original exact edge association; no bound
    /// prefix, guessed alias, substituted width or partial geometry escapes a refusal.
    pub fn bind_allowance<'a>(
        &'a self,
        allowance: &'a SeamAllowance,
    ) -> Result<BoundOffsetConstruction<'a>, OffsetBindingError> {
        if allowance.id() != self.allowance {
            return Err(OffsetBindingError::AllowanceIdentity {
                expected: self.allowance,
                actual: allowance.id(),
            });
        }
        if allowance.definition().edge != self.source.range.edge() {
            return Err(OffsetBindingError::SourceEdge {
                source: self.source,
                allowance_edge: allowance.definition().edge,
            });
        }
        Ok(BoundOffsetConstruction {
            construction: self,
            allowance,
        })
    }
}

/// Authentic association of the supplied input and descriptor, without registry/currentness proof.
/// ```compile_fail
/// fn forge<'a>(construction:&'a sc_core::construction::OffsetConstruction,
///     allowance:&'a sc_core::ontology::SeamAllowance)->sc_core::construction::BoundOffsetConstruction<'a> {
///     sc_core::construction::BoundOffsetConstruction { construction,allowance }
/// }
/// ```
/// ```compile_fail
/// fn escape(input:sc_core::construction::OffsetConstruction,allowance:sc_core::ontology::SeamAllowance)
///     ->sc_core::construction::BoundOffsetConstruction<'static> {
///     input.bind_allowance(&allowance).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundOffsetConstruction<'a> {
    construction: &'a OffsetConstruction,
    allowance: &'a SeamAllowance,
}
impl<'a> BoundOffsetConstruction<'a> {
    /// Actual immutable declaration that supplied the range, traversal and side.
    #[must_use]
    pub const fn construction(self) -> &'a OffsetConstruction {
        self.construction
    }
    /// Actual provided descriptor; its owner, corner and inclusion remain attached.
    #[must_use]
    pub const fn allowance(self) -> &'a SeamAllowance {
        self.allowance
    }
    /// Original authored width field; symbolic origins are not read or treated as zero.
    #[must_use]
    pub const fn width(self) -> &'a AllowanceWidth {
        &self.allowance.definition().width
    }
    /// Original authored corner field; no separate client override or default join.
    #[must_use]
    pub const fn corner(self) -> &'a CornerTreatment {
        &self.allowance.definition().corner
    }
    /// Original logical target-profile inclusion field, without resolved policy.
    #[must_use]
    pub const fn inclusion(self) -> &'a ProfileParameterRef {
        &self.allowance.definition().inclusion
    }
}
