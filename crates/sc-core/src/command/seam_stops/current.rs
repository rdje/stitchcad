//! Bounded immutable current registry/Piece and target/cut/range evidence,without stop rows/effects.
use super::CommandStopOwnershipAdmission;
use crate::{
    design::DesignReferenceHost,
    ontology::{
        CutCopy, EdgeRange, EntityId, Piece, RangeCoverage, RangePortion, ResolvedRef, SeamSideId,
        SewingLandmark, UniquePointResolution,
    },
};
use core::fmt;
use std::collections::BTreeMap;
mod build;
mod error;
pub use error::{
    CommandStopContextBoundary, CommandStopContextError, CommandStopContextIssue,
    CommandStopContextPopulation, CommandStopContextRange,
};

/// One complete actual Piece interval portion,with original query/ordinal provenance.
/// Working cache order is current edge/parameter order;source indices retain authored traversal.
/// No lost/unknown/numeric repair is filtered from this evidence.
#[derive(Debug, PartialEq, Eq)]
pub struct CommandStopPiecePortion {
    held: EdgeRange,
    source_edge_index: usize,
    portion_index: usize,
    portion: RangePortion,
}
impl CommandStopPiecePortion {
    /// Actual original whole-edge query,not a guessed current edge.
    #[must_use]
    pub const fn held(&self) -> EdgeRange {
        self.held
    }
    /// Original boundary/hole/construction ordinal in the actual Piece.
    #[must_use]
    pub const fn source_edge_index(&self) -> usize {
        self.source_edge_index
    }
    /// Original portion ordinal in that query's forward traversal.
    #[must_use]
    pub const fn portion_index(&self) -> usize {
        self.portion_index
    }
    /// Actual complete live or repair interval evidence.
    #[must_use]
    pub const fn portion(&self) -> &RangePortion {
        &self.portion
    }
}
struct PieceView<'a> {
    piece: &'a Piece,
    portions: Vec<CommandStopPiecePortion>,
}
impl PieceView<'_> {
    fn contains(&self, point: ResolvedRef) -> bool {
        self.portions.iter().any(|part| match &part.portion {
            RangePortion::Resolved(part) => {
                part.range().edge() == point.edge()
                    && part.range().from() <= point.param()
                    && point.param() <= part.range().to()
            }
            RangePortion::Unresolved(_) => false,
        })
    }
    fn covers(&self, requested: EdgeRange) -> bool {
        let mut covered = requested.from();
        for part in &self.portions {
            let RangePortion::Resolved(part) = &part.portion else {
                continue;
            };
            let range = part.range();
            if range.edge() != requested.edge() {
                continue;
            }
            if range.from() > covered {
                break;
            }
            covered = covered.max(range.to());
            if covered >= requested.to() {
                return true;
            }
        }
        false
    }
}
/// Actual semantic object and unique current point over the same immutable admitted input ledger.
/// This fixed view is not an original-preimage,physical representation or global readiness proof.
#[derive(Clone, Copy)]
pub struct CommandStopLandmarkEvidence<'a> {
    landmark: &'a SewingLandmark,
    resolution: UniquePointResolution,
    current: ResolvedRef,
}
impl fmt::Debug for CommandStopLandmarkEvidence<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            landmark,
            resolution,
            current,
        } = self;
        f.debug_struct("CommandStopLandmarkEvidence")
            .field("landmark", &landmark.id())
            .field("resolution", resolution)
            .field("current", current)
            .finish()
    }
}
impl<'a> CommandStopLandmarkEvidence<'a> {
    /// Exact actual semantic object;no cloned Notch/Turn metadata.
    #[must_use]
    pub const fn landmark(self) -> &'a SewingLandmark {
        self.landmark
    }
    /// Actual original held reference and successful bounded current query outcome.
    #[must_use]
    pub const fn resolution(self) -> UniquePointResolution {
        self.resolution
    }
    /// Actual unique current point in that same ledger frame.
    #[must_use]
    pub const fn current(self) -> ResolvedRef {
        self.current
    }
}

/// Actual bounded current registry/Piece ownership and target/cut/four-pair-range context.
/// Preserves all Piece repairs,validates every semantic mark,actual target copies,endpoints/cuts,
/// full target interval ownership and same-copy nonoverlap. Paired coverage uses original A order.
/// Stop-reference rows and ordered child lists remain a later stage. This local immutable proof
/// is not whole Piece geometry,whole Design/ease-root,value/Profile,physical,authority or effects.
///
/// Private fields prevent fabricated success:
/// ```compile_fail
/// use sc_core::command::{CommandStopCurrentContext,CommandStopOwnershipAdmission};
/// fn forge<'a,H>(ownership:&'a CommandStopOwnershipAdmission<'a,H>)->CommandStopCurrentContext<'a,H>{
///     CommandStopCurrentContext {ownership,copies:Default::default(),pieces:Default::default(),
///         marks:Default::default(),points:todo!(),paired:todo!(),owned_items:0}
/// }
/// ```
/// Its actual admitted source owner cannot escape:
/// ```compile_fail
/// use sc_core::command::{CommandStopCurrentContext,CommandStopOwnershipAdmission};
/// use sc_core::design::DesignReferenceHost;
/// fn escape<H:DesignReferenceHost>(ownership:&CommandStopOwnershipAdmission<'_,H>)
///     ->CommandStopCurrentContext<'static,H>{ownership.validate_current_context().unwrap()}
/// ```
pub struct CommandStopCurrentContext<'a, H> {
    ownership: &'a CommandStopOwnershipAdmission<'a, H>,
    copies: BTreeMap<EntityId, &'a CutCopy>,
    pieces: BTreeMap<EntityId, PieceView<'a>>,
    marks: BTreeMap<EntityId, CommandStopLandmarkEvidence<'a>>,
    points: [UniquePointResolution; 6],
    paired: [RangeCoverage; 4],
    owned_items: usize,
}
impl<H> fmt::Debug for CommandStopCurrentContext<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ownership,
            copies,
            pieces,
            marks,
            points: _,
            paired: _,
            owned_items,
        } = self;
        f.debug_struct("CommandStopCurrentContext")
            .field("target", &ownership.inputs().binding().span().id())
            .field("copies", &copies.len())
            .field("pieces", &pieces.len())
            .field("landmarks", &marks.len())
            .field("owned_items", owned_items)
            .finish()
    }
}
impl<'a, H> CommandStopCurrentContext<'a, H> {
    /// Exact actual ownership/input/limit evidence;no same-ID source reconstruction.
    #[must_use]
    pub const fn ownership(&self) -> &'a CommandStopOwnershipAdmission<'a, H> {
        self.ownership
    }
    /// Actual complete copy lookup,including unused entries.
    #[must_use]
    pub fn copy(&self, id: EntityId) -> Option<&'a CutCopy> {
        self.copies.get(&id).copied()
    }
    /// All actual Piece live/repair rows,in current working order with original provenance.
    #[must_use]
    pub fn piece_portions(&self, id: EntityId) -> Option<&[CommandStopPiecePortion]> {
        self.pieces.get(&id).map(|piece| piece.portions.as_slice())
    }
    /// Exact actual Piece associated with those cached interval rows.
    #[must_use]
    pub fn piece(&self, id: EntityId) -> Option<&'a Piece> {
        self.pieces.get(&id).map(|piece| piece.piece)
    }
    /// Complete actual semantic lookup,including unused marks,without a cloned object.
    #[must_use]
    pub fn landmark(&self, id: EntityId) -> Option<CommandStopLandmarkEvidence<'a>> {
        self.marks.get(&id).copied()
    }
    /// Actual complete copy index population.
    #[must_use]
    pub fn copy_count(&self) -> usize {
        self.copies.len()
    }
    /// Actual complete Piece index population.
    #[must_use]
    pub fn piece_count(&self) -> usize {
        self.pieces.len()
    }
    /// Actual complete semantic index population.
    #[must_use]
    pub fn landmark_count(&self) -> usize {
        self.marks.len()
    }
    /// Actual retained live plus repair Piece portions,not allocator capacity.
    #[must_use]
    pub const fn owned_item_count(&self) -> usize {
        self.owned_items
    }
    /// Actual bounded lower/cut/upper query on the named source side.
    #[must_use]
    pub const fn boundary(
        &self,
        side: SeamSideId,
        boundary: CommandStopContextBoundary,
    ) -> UniquePointResolution {
        let [a_start, a_cut, a_end, b_start, b_cut, b_end] = self.points;
        match (side, boundary) {
            (SeamSideId::A, CommandStopContextBoundary::Start) => a_start,
            (SeamSideId::A, CommandStopContextBoundary::Cut) => a_cut,
            (SeamSideId::A, CommandStopContextBoundary::End) => a_end,
            (SeamSideId::B, CommandStopContextBoundary::Start) => b_start,
            (SeamSideId::B, CommandStopContextBoundary::Cut) => b_cut,
            (SeamSideId::B, CommandStopContextBoundary::End) => b_end,
        }
    }
    /// Complete current first A-ordered pair's named side coverage.
    #[must_use]
    pub const fn first_pair_coverage(&self, side: SeamSideId) -> &RangeCoverage {
        let [a, b, _, _] = &self.paired;
        match side {
            SeamSideId::A => a,
            SeamSideId::B => b,
        }
    }
    /// Complete current second A-ordered pair's named side coverage.
    #[must_use]
    pub const fn second_pair_coverage(&self, side: SeamSideId) -> &RangeCoverage {
        let [_, _, a, b] = &self.paired;
        match side {
            SeamSideId::A => a,
            SeamSideId::B => b,
        }
    }
}
impl<H: DesignReferenceHost> CommandStopOwnershipAdmission<'_, H> {
    /// Validate actual local current context through bounded ports after both input admissions.
    /// Index real copies and require named targets,cache every Piece live/repair portion,validate
    /// every actual semantic mark,then target lower/cut/upper points and full A/B range ownership/
    /// self-copy nonoverlap. Drop temporary full-side evidence before caching four pair coverages.
    /// Uses no legacy choice tree,synthetic graph,source coordinate narrowing or numeric provider.
    /// The immutable structural owner already checked CutPlan/Piece metadata at birth;this stage
    /// rechecks held graph references against those actual current objects without cloning a plan.
    /// ```
    /// use sc_core::command::{CommandStopOwnershipAdmission,CommandStopCurrentContext,CommandStopContextError};
    /// use sc_core::design::DesignReferenceHost;
    /// fn validate<'a,H:DesignReferenceHost>(ownership:&'a CommandStopOwnershipAdmission<'a,H>)
    ///     ->Result<CommandStopCurrentContext<'a,H>,CommandStopContextError>{
    ///     ownership.validate_current_context()
    /// }
    /// ```
    /// # Errors
    /// Retains original target/copy/Piece/mark identity and full bounded point/interval evidence,
    /// including concrete quota,repair,choice and numeric standard causes. No partial successful
    /// context or implicit stop assignment,physical correspondence,authority or effect is returned.
    pub fn validate_current_context(
        &self,
    ) -> Result<CommandStopCurrentContext<'_, H>, CommandStopContextError> {
        build::validate(self)
    }
}
