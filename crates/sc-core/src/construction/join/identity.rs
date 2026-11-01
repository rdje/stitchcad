//! Scoped canonical result Piece identity declaration;trusted issuance/global nonreuse stay external.
use super::{BoundJoinEdgeRoles, JoinSourceOccurrenceLocation};
use crate::ontology::{EdgeRef, EntityId, MaterialAssignment, Mirroring, SeamSideId, StopLandmark};
use core::fmt;

/// Whole selected-span plus retained-ease-source stop-occurrence allowance before stop scans.
/// ```compile_fail
/// let limits:sc_core::construction::JoinPieceIdentityLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinPieceIdentityLimits {
    stops: usize,
}
impl JoinPieceIdentityLimits {
    /// Count both actual stop lists,including repeated references across them. Earlier full
    /// provided inventory/plan and original H/R admissions remain separate immutable obligations.
    #[must_use]
    pub const fn new(stops: usize) -> Self {
        Self { stops }
    }
    /// Complete selected-plus-retained stop-occurrence allowance.
    #[must_use]
    pub const fn stops(self) -> usize {
        self.stops
    }
}
/// Actual original stop list containing a refused known landmark reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinIdentityStopOrigin {
    /// Current selected span's original authored stop list.
    SelectedSpan,
    /// Complete original snapshot retained by a Portion ease source.
    RetainedEaseSource,
}
/// First whole stop quota or enumerated known local result-owner/reference contradiction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinPieceIdentityRefusal {
    /// Checked complete stop sum cannot be represented.
    StopOverflow,
    /// Complete actual selected/retained stop occurrences exceed policy.
    Stops {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Piece and its creating topology operation need separate owned identities.
    Operation,
    /// A supplied original inventory Piece already owns this identity.
    InventoryPiece {
        /// First original inventory position.
        index: usize,
    },
    /// A requested separate result pair member cannot be its own companion.
    SelfCompanion,
    /// The actual original SewingGraph already owns this identity.
    Graph,
    /// The actual selected span already owns this identity.
    Span,
    /// The retained original ease-source span already owns this identity.
    RetainedSpan,
    /// An original physical copy already owns this identity.
    Copy {
        /// First original plan position.
        index: usize,
    },
    /// An actual original A/B edge-producing operation owns this identity.
    SourceOperation {
        /// Original source role.
        side: SeamSideId,
        /// First original occurrence location.
        location: JoinSourceOccurrenceLocation,
    },
    /// A supplied inventory Piece requires this identity as its assigned Material.
    SourceMaterial {
        /// First original inventory position.
        index: usize,
    },
    /// The selected result metadata requires this identity as its Material.
    ResultMaterial,
    /// An actual selected/retained stop requires this identity as its landmark.
    Stop {
        /// Actual original stop list.
        origin: JoinIdentityStopOrigin,
        /// First position within that list.
        index: usize,
    },
}
/// Whole original roles/context,raw canonical id,policy and first cause;no identity is issued.
pub struct JoinPieceIdentityError<'a> {
    roles: &'a BoundJoinEdgeRoles<'a>,
    piece_id: &'a EntityId,
    limits: JoinPieceIdentityLimits,
    refusal: JoinPieceIdentityRefusal,
    source_edge: Option<&'a EdgeRef>,
}
impl<'a> JoinPieceIdentityError<'a> {
    /// Actual whole bank/catalogue/population/metadata/source context.
    #[must_use]
    pub const fn roles(&self) -> &'a BoundJoinEdgeRoles<'a> {
        self.roles
    }
    /// Actual caller-owned canonical declaration field,not an allocation request or issued id.
    #[must_use]
    pub const fn piece_id(&self) -> &'a EntityId {
        self.piece_id
    }
    /// Original complete stop policy.
    #[must_use]
    pub const fn limits(&self) -> JoinPieceIdentityLimits {
        self.limits
    }
    /// First whole quota or listed local alias cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinPieceIdentityRefusal {
        self.refusal
    }
    /// Actual refused edge field only for SourceOperation;other causes return None.
    #[must_use]
    pub const fn refused_source(&self) -> Option<&'a EdgeRef> {
        self.source_edge
    }
    /// Actual original stop field only for Stop;other causes return None. Constant work,no scan.
    #[must_use]
    pub fn refused_stop(&self) -> Option<&'a StopLandmark> {
        let JoinPieceIdentityRefusal::Stop { origin, index } = self.refusal else {
            return None;
        };
        let span = self
            .roles
            .banks()
            .sources()
            .population()
            .metadata()
            .splice()
            .source()
            .span();
        match origin {
            JoinIdentityStopOrigin::SelectedSpan => span.definition().stops.get(index),
            JoinIdentityStopOrigin::RetainedEaseSource => span
                .definition()
                .ease
                .portion()?
                .source()
                .stops()
                .get(index),
        }
    }
}
impl fmt::Debug for JoinPieceIdentityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinPieceIdentityError")
            .field("piece_id", &self.piece_id)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinPieceIdentityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join result Piece {}: {:?}", self.piece_id, self.refusal)
    }
}
impl std::error::Error for JoinPieceIdentityError<'_> {}
impl<'s> BoundJoinEdgeRoles<'s> {
    /// Declare one canonical native result Piece id under explicit enumerated local exclusions.
    /// Whole selected-plus-retained stop count/quota precedes candidate/stop field inspection.
    /// Reuse immutable complete C/I/H/R admission;work O(C+I+H+R+S+1),constant space,no allocation,
    /// string/registry/provider lookup or identity issuance. Known inventory/plan owners,actual
    /// graph/span/retained span,source producers,required materials and landmarks are checked.
    /// Formula/Profile source ids are not classified from equal external values. Unlisted/current/
    /// historical namespace,reference-kind/state truth,trusted issuance and authority remain required.
    /// # Errors
    /// Stop arithmetic/quota;operation,inventory Piece,self-companion,graph,span,retained span,
    /// original copy,source producer,input Material,result Material,selected then retained stop.
    pub fn declare_piece_identity<'a>(
        &'a self,
        piece_id: &'a EntityId,
        limits: JoinPieceIdentityLimits,
    ) -> Result<DeclaredJoinPieceIdentity<'a>, JoinPieceIdentityError<'a>>
    where
        's: 'a,
    {
        use JoinPieceIdentityRefusal as R;
        let refuse = |refusal, source_edge| JoinPieceIdentityError {
            roles: self,
            piece_id,
            limits,
            refusal,
            source_edge,
        };
        let population = self.banks().sources().population();
        let metadata = population.metadata();
        let splice = metadata.splice();
        let source = splice.source();
        let span = source.span();
        let retained = span
            .definition()
            .ease
            .portion()
            .map(|portion| portion.source());
        let stops = span
            .definition()
            .stops
            .len()
            .checked_add(retained.map_or(0, |s| s.stops().len()))
            .ok_or_else(|| refuse(R::StopOverflow, None))?;
        if stops > limits.stops {
            return Err(refuse(
                R::Stops {
                    required: stops,
                    allowed: limits.stops,
                },
                None,
            ));
        }
        if *piece_id == source.edit().operation() {
            return Err(refuse(R::Operation, None));
        }
        for (index, piece) in source.pieces().iter().enumerate() {
            if piece.id() == *piece_id {
                return Err(refuse(R::InventoryPiece { index }, None));
            }
        }
        if matches!(population.mirroring(),Mirroring::PairMember{companion,..} if companion==piece_id)
        {
            return Err(refuse(R::SelfCompanion, None));
        }
        if *piece_id == source.graph().id() {
            return Err(refuse(R::Graph, None));
        }
        if *piece_id == span.id() {
            return Err(refuse(R::Span, None));
        }
        if retained.is_some_and(|s| s.span() == *piece_id) {
            return Err(refuse(R::RetainedSpan, None));
        }
        for (index, copy) in source.plan().copies().iter().enumerate() {
            if copy.id() == *piece_id {
                return Err(refuse(R::Copy { index }, None));
            }
        }
        for occurrence in self.banks().sources().occurrences() {
            let edge = occurrence.field().edge();
            if edge.creator() == *piece_id {
                return Err(refuse(
                    R::SourceOperation {
                        side: occurrence.side(),
                        location: occurrence.location(),
                    },
                    Some(edge),
                ));
            }
        }
        for (index, piece) in source.pieces().iter().enumerate() {
            if matches!(&piece.definition().material,MaterialAssignment::Assigned(id) if id==piece_id)
            {
                return Err(refuse(R::SourceMaterial { index }, None));
            }
        }
        if matches!(metadata.material(),MaterialAssignment::Assigned(id) if id==piece_id) {
            return Err(refuse(R::ResultMaterial, None));
        }
        for (index, stop) in span.definition().stops.iter().enumerate() {
            if stop.landmark == *piece_id {
                return Err(refuse(
                    R::Stop {
                        origin: JoinIdentityStopOrigin::SelectedSpan,
                        index,
                    },
                    None,
                ));
            }
        }
        if let Some(retained) = retained {
            for (index, stop) in retained.stops().iter().enumerate() {
                if stop.landmark == *piece_id {
                    return Err(refuse(
                        R::Stop {
                            origin: JoinIdentityStopOrigin::RetainedEaseSource,
                            index,
                        },
                        None,
                    ));
                }
            }
        }
        Ok(DeclaredJoinPieceIdentity {
            roles: self,
            piece_id,
            limits,
            stops,
        })
    }
}
/// Authentic local canonical Piece-id declaration,not an issued/fresh/current result or authority.
/// ```compile_fail
/// fn forge<'a>(mut id:sc_core::construction::DeclaredJoinPieceIdentity<'a>,other:&'a sc_core::ontology::EntityId) {id.piece_id=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(roles:&'a sc_core::construction::BoundJoinEdgeRoles<'a>)
///     ->sc_core::construction::DeclaredJoinPieceIdentity<'a> {
///     let field=sc_core::ontology::EntityId::from_bits(5000);
///     roles.declare_piece_identity(&field,sc_core::construction::JoinPieceIdentityLimits::new(1)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct DeclaredJoinPieceIdentity<'a> {
    roles: &'a BoundJoinEdgeRoles<'a>,
    piece_id: &'a EntityId,
    limits: JoinPieceIdentityLimits,
    stops: usize,
}
impl<'a> DeclaredJoinPieceIdentity<'a> {
    /// Actual complete original role/catalogue/bank/source/metadata/population context.
    #[must_use]
    pub const fn roles(&self) -> &'a BoundJoinEdgeRoles<'a> {
        self.roles
    }
    /// Actual canonical declared result Piece field;no source/label/coordinate/ordinal derivation.
    #[must_use]
    pub const fn piece_id(&self) -> &'a EntityId {
        self.piece_id
    }
    /// Original explicit complete stop allowance.
    #[must_use]
    pub const fn limits(&self) -> JoinPieceIdentityLimits {
        self.limits
    }
    /// Complete selected-plus-retained stop occurrence count,including duplicate references.
    #[must_use]
    pub const fn stop_occurrences(&self) -> usize {
        self.stops
    }
}
