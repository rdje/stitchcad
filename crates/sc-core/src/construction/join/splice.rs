//! Exact nondeforming splice requirements;no pose,coincidence or zero-ease certificate.
use super::{BoundJoinCopyInventory, BoundJoinPiecesAtSeam};
use crate::ontology::{EdgeAnchor, SeamDirection, SeamEase, SeamSide, SeamSideId};

/// Required parity of the moving authored Piece's transform,independent of copy reflection.
/// ```compile_fail
/// let parity:sc_core::construction::JoinMovingOrientation=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinMovingOrientation {
    /// Require an orientation-preserving rigid transform of the authored moving Piece.
    Preserve,
    /// Require an orientation-reversing rigid transform of the authored moving Piece.
    Reflect,
}
/// Deliberate placement policy,not an evaluated transform or implicit seam fit.
/// ```compile_fail
/// let placement:sc_core::construction::JoinSplicePlacement=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinSplicePlacement {
    /// Require the original resolved placements to meet without moving either Piece.
    KeepOriginal,
    /// Keep one authored Piece fixed and require unique nondegenerate rigid alignment of the
    /// other from both actual endpoint pairs,plus complete selected-correspondence coincidence.
    RigidAlign {
        /// Original A/B role whose authored placement remains fixed.
        fixed: SeamSideId,
        /// Required authored moving-frame parity,not inferred from physical-copy reflection.
        orientation: JoinMovingOrientation,
    },
}
/// Exact nondeforming domain:require actually zero allocated ease,exact coincidence of both
/// complete selected banks after placement,and valid complete joined topology. No stretch,
/// gather,curve refit,snap,copy fork or epsilon. Partial spans/reflected copies remain actual
/// sources. Execution must retain unavailable/uncertified/unsupported/resource/cancel causes.
/// ```compile_fail
/// let request:sc_core::construction::JoinExactSpliceRequirement=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinExactSpliceRequirement {
    placement: JoinSplicePlacement,
}
impl JoinExactSpliceRequirement {
    /// Retain explicit placement intent without evaluating geometry or selecting a transform.
    #[must_use]
    pub const fn new(placement: JoinSplicePlacement) -> Self {
        Self { placement }
    }
    /// Actual original authored placement field.
    #[must_use]
    pub const fn placement(&self) -> &JoinSplicePlacement {
        &self.placement
    }
}
impl<'s> BoundJoinCopyInventory<'s> {
    /// Bind exact nondeforming requirements to this actual complete original inventory.
    /// Constant work/space,no scan,allocation,lookup,provider call or geometry evaluation.
    /// Zero ease,placement/curve equality,currentness,topology/observers and authority stay required.
    #[must_use]
    pub const fn require_exact_splice<'a>(
        &'a self,
        requirement: &'a JoinExactSpliceRequirement,
    ) -> RequiredJoinExactSplice<'a>
    where
        's: 'a,
    {
        RequiredJoinExactSplice {
            inventory: self,
            requirement,
        }
    }
}
/// Actual whole inventory and required policy,not an accepted geometric splice.
/// ```compile_fail
/// fn forge<'a>(mut splice:sc_core::construction::RequiredJoinExactSplice<'a>,
///     other:&'a sc_core::construction::JoinExactSpliceRequirement) { splice.requirement=other; }
/// ```
/// ```compile_fail
/// fn escape<'a>(inventory:&'a sc_core::construction::BoundJoinCopyInventory<'a>)
///     ->sc_core::construction::RequiredJoinExactSplice<'a> {
///     use sc_core::construction::*;
///     let request=JoinExactSpliceRequirement::new(JoinSplicePlacement::KeepOriginal);
///     inventory.require_exact_splice(&request)
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredJoinExactSplice<'a> {
    inventory: &'a BoundJoinCopyInventory<'a>,
    requirement: &'a JoinExactSpliceRequirement,
}
impl<'a> RequiredJoinExactSplice<'a> {
    /// Actual complete original inventory admission and its original policy/parents.
    #[must_use]
    pub const fn inventory(&self) -> &'a BoundJoinCopyInventory<'a> {
        self.inventory
    }
    /// Actual original deliberate splice requirement.
    #[must_use]
    pub const fn requirement(&self) -> &'a JoinExactSpliceRequirement {
        self.requirement
    }
    /// Actual original graph/span/plan/side/Piece association.
    #[must_use]
    pub const fn source(&self) -> &'a BoundJoinPiecesAtSeam<'a> {
        self.inventory.source()
    }
    /// Original full/portion ease that must actually resolve to zero. No copied amount,
    /// symbolic-source resolution or accepted zero-ease evidence is manufactured.
    #[must_use]
    pub const fn ease(&self) -> &'a SeamEase {
        &self.source().span().definition().ease
    }
    /// Both original endpoint pairs in ascending A-held parameter order/reverse.
    /// Correspondence remains independent of copy reflection/fixed-side policy. These anchors
    /// supply no current map,position,interior correspondence or coincidence certificate.
    pub fn endpoint_pairs(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinSpliceEndpointPair<'_>> + DoubleEndedIterator {
        [JoinSpliceEnd::AFrom, JoinSpliceEnd::ATo]
            .into_iter()
            .map(|end| JoinSpliceEndpointPair { context: self, end })
    }
}
/// Endpoint order in the original A-held frame,not physical left/right.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinSpliceEnd {
    /// Pair containing A's lower source parameter.
    AFrom,
    /// Pair containing A's upper source parameter.
    ATo,
}
/// Authentic original side-anchor pair with complete placement/copy/ease context.
/// ```compile_fail
/// fn forge<'a>(context:&'a sc_core::construction::RequiredJoinExactSplice<'a>)
///     ->sc_core::construction::JoinSpliceEndpointPair<'a> {
///     use sc_core::construction::*;JoinSpliceEndpointPair { context,end:JoinSpliceEnd::AFrom }
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinSpliceEndpointPair<'a> {
    context: &'a RequiredJoinExactSplice<'a>,
    end: JoinSpliceEnd,
}
impl<'a> JoinSpliceEndpointPair<'a> {
    /// Actual whole original source and placement requirement.
    #[must_use]
    pub const fn context(self) -> &'a RequiredJoinExactSplice<'a> {
        self.context
    }
    /// Original A-held endpoint determining this pair's position.
    #[must_use]
    pub const fn end(self) -> JoinSpliceEnd {
        self.end
    }
    /// Actual original span-side field,retaining its range frame and physical copy.
    #[must_use]
    pub const fn source_side(self, side: SeamSideId) -> &'a SeamSide {
        let span = self.context.source().span().definition();
        match side {
            SeamSideId::A => &span.a,
            SeamSideId::B => &span.b,
        }
    }
    /// Original bounded anchor under actual Same/Opposite endpoint correspondence.
    /// Copy reflection/fixed-side policy does not rewrite this source parameter frame.
    #[must_use]
    pub fn anchor(self, side: SeamSideId) -> EdgeAnchor {
        let range = &self.source_side(side).range;
        let from = match side {
            SeamSideId::A => self.end == JoinSpliceEnd::AFrom,
            SeamSideId::B => match self.context.source().span().definition().direction {
                SeamDirection::Same => self.end == JoinSpliceEnd::AFrom,
                SeamDirection::Opposite => self.end == JoinSpliceEnd::ATo,
            },
        };
        EdgeAnchor {
            edge: range.edge(),
            param: if from { range.from() } else { range.to() },
        }
    }
}
