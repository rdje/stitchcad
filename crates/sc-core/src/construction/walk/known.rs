//! Scoped known EntityId exclusions;unlisted is not globally fresh or reserved.
use super::BoundWalkExistingNotches;
use crate::ontology::{
    EntityId, MaterialAssignment, NotchDimensionBindings, NotchProfileBindings, PieceDefinition,
    SeamSideId,
};
use core::fmt;
use std::collections::BTreeSet;
/// Explicit whole copy/Piece/hole/edge/pair/Notch and new logical-workspace allowances.
/// ```compile_fail
/// let limits:sc_core::construction::WalkNotchIdentityExclusionLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkNotchIdentityExclusionLimits {
    copies: usize,
    pieces: usize,
    holes: usize,
    edges: usize,
    pairs: usize,
    notches: usize,
    workspace_entries: usize,
}
impl WalkNotchIdentityExclusionLimits {
    /// Select complete C/I/H/R/N/K and checked C+2I+R+14N+9K+3 allowances. Logical entries are not
    /// allocation bytes,prior model/index storage or actual current global namespace completeness.
    #[must_use]
    pub const fn new(
        copies: usize,
        pieces: usize,
        holes: usize,
        edges: usize,
        pairs: usize,
        notches: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            copies,
            pieces,
            holes,
            edges,
            pairs,
            notches,
            workspace_entries,
        }
    }
    /// Whole supplied CutPlan population,including unrelated copies.
    #[must_use]
    pub const fn copies(self) -> usize {
        self.copies
    }
    /// Whole actual supplied Piece inventory.
    #[must_use]
    pub const fn pieces(self) -> usize {
        self.pieces
    }
    /// Whole supplied Piece hole groups,before scanning child lengths.
    #[must_use]
    pub const fn holes(self) -> usize {
        self.holes
    }
    /// Whole supplied Piece boundary/hole/construction/fold occurrences,including repeats.
    #[must_use]
    pub const fn edges(self) -> usize {
        self.edges
    }
    /// Whole original walk pairs,including repeated requests.
    #[must_use]
    pub const fn pairs(self) -> usize {
        self.pairs
    }
    /// Whole supplied existing Notch collection,including unrelated objects.
    #[must_use]
    pub const fn notches(self) -> usize {
        self.notches
    }
    /// New known-id set upper bound,not bytes or earlier allocation.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// Whole resource/count refusal;no prefix identity set escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkNotchIdentityExclusionRefusal {
    /// Checked complete hole/edge/workspace populations cannot be represented.
    CountOverflow,
    /// Complete supplied copy population exceeds policy.
    Copies {
        /// Actual whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete supplied Piece population exceeds policy.
    Pieces {
        /// Actual whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original input pair population exceeds policy.
    Pairs {
        /// Actual whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete supplied Notch population exceeds policy.
    Notches {
        /// Actual whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete hole groups exceed policy before scanning child lengths.
    Holes {
        /// Actual whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete edge occurrences exceed policy before reading creator fields.
    Edges {
        /// Actual whole count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Checked new identity-set entry upper bound exceeds policy before allocation.
    Workspace {
        /// Actual complete bound.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
}
/// Actual whole existing-object/assignment/source context,policy and first refusal.
pub struct WalkNotchIdentityExclusionError<'a> {
    existing: &'a BoundWalkExistingNotches<'a>,
    limits: &'a WalkNotchIdentityExclusionLimits,
    refusal: WalkNotchIdentityExclusionRefusal,
}
impl<'a> WalkNotchIdentityExclusionError<'a> {
    /// Actual complete supplied objects,original assignments and copy/role/source parents.
    #[must_use]
    pub const fn existing(&self) -> &'a BoundWalkExistingNotches<'a> {
        self.existing
    }
    /// Original caller-owned complete resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a WalkNotchIdentityExclusionLimits {
        self.limits
    }
    /// First whole count or quota cause.
    #[must_use]
    pub const fn refusal(&self) -> WalkNotchIdentityExclusionRefusal {
        self.refusal
    }
}
impl fmt::Debug for WalkNotchIdentityExclusionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalkNotchIdentityExclusionError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for WalkNotchIdentityExclusionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "walk known identity exclusions: {:?}", self.refusal)
    }
}
impl std::error::Error for WalkNotchIdentityExclusionError<'_> {}
fn profile_ids(bindings: &NotchProfileBindings) -> [EntityId; 6] {
    // Exhaustive patterns keep the 14N/9K identity-field accounting coupled to these records.
    let NotchProfileBindings {
        style,
        sample,
        production,
        encoding,
    } = bindings;
    let NotchDimensionBindings {
        depth: sample_depth,
        width: sample_width,
    } = sample;
    let NotchDimensionBindings {
        depth: production_depth,
        width: production_width,
    } = production;
    [
        style.id(),
        sample_depth.id(),
        sample_width.id(),
        production_depth.id(),
        production_width.id(),
        encoding.id(),
    ]
}
fn edge_count(piece: &PieceDefinition) -> Option<usize> {
    piece.holes.iter().try_fold(
        piece
            .boundary
            .len()
            .checked_add(piece.construction_lines.len())?
            .checked_add(piece.fold_edges.len())?,
        |count, hole| count.checked_add(hole.len()),
    )
}
impl<'s> BoundWalkExistingNotches<'s> {
    /// Bind explicit scoped exclusions without generating or globally reserving an identity.
    /// Whole C/I/N/K quotas precede all scans. Whole H precedes child-length scans,whole R precedes
    /// creator reads,and checked C+2I+R+14N+9K+3 precedes all id scans/set allocation. Known fields:
    /// every supplied copy/Piece id,Piece edge creator and assigned Material;every original requested
    /// anchor creator/six profile ids;every supplied Notch id/Piece/anchor creator/six profile ids;
    /// actual walk operation,graph and selected span. Other graph/provider/history fields remain outside
    /// this scope. Work is O((C+I+H+R+N+K)log(C+I+R+N+K+2));no current namespace/kind proof follows.
    /// # Errors
    /// Complete C/I/N/K quotas;checked H/quota;checked R/quota;checked workspace/quota.
    pub fn bind_identity_exclusions<'a>(
        &'a self,
        limits: &'a WalkNotchIdentityExclusionLimits,
    ) -> Result<BoundWalkNotchIdentityExclusions<'a>, WalkNotchIdentityExclusionError<'a>>
    where
        's: 'a,
    {
        use WalkNotchIdentityExclusionRefusal as F;
        let refuse = |refusal| WalkNotchIdentityExclusionError {
            existing: self,
            limits,
            refusal,
        };
        let copies = self.assignments().roles().copies();
        let pairs = copies.source().admitted().edit().pairs();
        let c = copies.plan().copies().len();
        let i = copies.inventory().pieces().len();
        let n = pairs.len();
        let k = self.notches().len();
        if c > limits.copies {
            return Err(refuse(F::Copies {
                required: c,
                allowed: limits.copies,
            }));
        }
        if i > limits.pieces {
            return Err(refuse(F::Pieces {
                required: i,
                allowed: limits.pieces,
            }));
        }
        if n > limits.pairs {
            return Err(refuse(F::Pairs {
                required: n,
                allowed: limits.pairs,
            }));
        }
        if k > limits.notches {
            return Err(refuse(F::Notches {
                required: k,
                allowed: limits.notches,
            }));
        }
        let h = copies
            .inventory()
            .pieces()
            .iter()
            .try_fold(0usize, |count, piece| {
                count.checked_add(piece.definition().holes.len())
            })
            .ok_or_else(|| refuse(F::CountOverflow))?;
        if h > limits.holes {
            return Err(refuse(F::Holes {
                required: h,
                allowed: limits.holes,
            }));
        }
        let r = copies
            .inventory()
            .pieces()
            .iter()
            .try_fold(0usize, |count, piece| {
                count.checked_add(edge_count(piece.definition())?)
            })
            .ok_or_else(|| refuse(F::CountOverflow))?;
        if r > limits.edges {
            return Err(refuse(F::Edges {
                required: r,
                allowed: limits.edges,
            }));
        }
        let workspace = i
            .checked_mul(2)
            .and_then(|count| count.checked_add(c))
            .and_then(|count| count.checked_add(r))
            .and_then(|count| count.checked_add(n.checked_mul(14)?))
            .and_then(|count| count.checked_add(k.checked_mul(9)?))
            .and_then(|count| count.checked_add(3))
            .ok_or_else(|| refuse(F::CountOverflow))?;
        if workspace > limits.workspace_entries {
            return Err(refuse(F::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        let mut ids = BTreeSet::new();
        for copy in copies.plan().copies() {
            ids.insert(copy.id());
        }
        for piece in copies.inventory().pieces() {
            ids.insert(piece.id());
            let definition = piece.definition();
            match &definition.material {
                MaterialAssignment::Assigned(id) => {
                    ids.insert(*id);
                }
                MaterialAssignment::Unresolved(_) => {}
            }
            for edge in definition
                .boundary
                .iter()
                .chain(definition.holes.iter().flatten())
                .chain(definition.construction_lines.iter())
            {
                ids.insert(edge.edge.creator());
            }
            for edge in &definition.fold_edges {
                ids.insert(edge.creator());
            }
        }
        for pair in pairs {
            for side in [SeamSideId::A, SeamSideId::B] {
                let mark = pair.mark(side);
                ids.insert(mark.anchor().edge.creator());
                ids.extend(profile_ids(mark.representation()));
            }
        }
        for notch in self.notches() {
            ids.insert(notch.id());
            ids.insert(notch.piece());
            ids.insert(notch.definition().anchor.edge.creator());
            ids.extend(profile_ids(&notch.definition().representation));
        }
        ids.insert(copies.source().admitted().edit().operation());
        ids.insert(copies.source().graph().id());
        ids.insert(copies.source().span().id());
        Ok(BoundWalkNotchIdentityExclusions {
            existing: self,
            limits,
            workspace,
            holes: h,
            edges: r,
            ids,
        })
    }
}
/// Authentic scoped known-id set;false lookup is unlisted,never globally fresh/reserved.
/// ```compile_fail
/// fn reparent<'a>(mut exclusions:sc_core::construction::BoundWalkNotchIdentityExclusions<'a>,
///     other:&'a sc_core::construction::BoundWalkExistingNotches<'a>) {exclusions.existing=other;}
/// ```
/// ```compile_fail
/// fn escape(existing:sc_core::construction::BoundWalkExistingNotches<'_>)
///     ->sc_core::construction::BoundWalkNotchIdentityExclusions<'static> {
///     let limits=sc_core::construction::WalkNotchIdentityExclusionLimits::new(0,0,0,0,0,0,0);
///     existing.bind_identity_exclusions(&limits).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct BoundWalkNotchIdentityExclusions<'a> {
    existing: &'a BoundWalkExistingNotches<'a>,
    limits: &'a WalkNotchIdentityExclusionLimits,
    workspace: usize,
    holes: usize,
    edges: usize,
    ids: BTreeSet<EntityId>,
}
impl<'a> BoundWalkNotchIdentityExclusions<'a> {
    /// Actual complete supplied Notches,assignments and all original walk/copy/role/source parents.
    #[must_use]
    pub const fn existing(&self) -> &'a BoundWalkExistingNotches<'a> {
        self.existing
    }
    /// Original explicit caller-owned resource policy.
    #[must_use]
    pub const fn limits(&self) -> &'a WalkNotchIdentityExclusionLimits {
        self.limits
    }
    /// Checked C+2I+R+14N+9K+3 entry upper bound,not bytes or prior storage.
    #[must_use]
    pub const fn workspace_bound(&self) -> usize {
        self.workspace
    }
    /// Whole supplied Piece hole groups counted before child-length scans.
    #[must_use]
    pub const fn holes(&self) -> usize {
        self.holes
    }
    /// Whole supplied Piece edge occurrences,including repeated construction/fold references.
    #[must_use]
    pub const fn edges(&self) -> usize {
        self.edges
    }
    /// Actual distinct known-id set entries,not a current namespace census.
    #[must_use]
    pub fn indexed_count(&self) -> usize {
        self.ids.len()
    }
    /// Whether this id names one of the explicit inspected fields. False means unlisted in this
    /// local set,not globally absent,fresh,reserved or authorized. Query work is O(log(W+2)).
    #[must_use]
    pub fn excludes_new_notch(&self, id: EntityId) -> bool {
        self.ids.contains(&id)
    }
}
