//! Canonical result Piece identity association;trusted issuance/global nonreuse remain host checks.
use super::{BoundMirrorPieceEdgeRoles, MirrorPieceEdgeLocation};
use crate::ontology::{EdgeRef, EntityId, MaterialAssignment, Mirroring};
use core::fmt;

/// First intrinsic or known typed source-owner alias,without a whole namespace census.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceIdentityRefusal {
    /// A Piece and its creating topology operation are separate owned identities.
    Operation,
    /// Copy intent cannot reuse the actual original source Piece identity.
    SourcePiece,
    /// A separately handed result cannot be its own requested companion.
    SelfCompanion,
    /// A required original source-edge operation cannot also own the result Piece.
    SourceOperation {
        /// First original Piece field using that operation.
        location: MirrorPieceEdgeLocation,
        /// Exact original source edge identity and persistent tag.
        edge: EdgeRef,
    },
    /// A required axis-source operation cannot also own the result Piece.
    AxisOperation {
        /// Original point start/end or range dependency position.
        index: usize,
    },
    /// The source's explicitly assigned Material cannot also own the result Piece identity.
    SourceMaterial,
    /// The selected result Material cannot also own the result Piece identity.
    ResultMaterial,
}
/// Actual role/source/metadata parents and canonical identity field survive every refusal.
pub struct MirrorPieceIdentityError<'a> {
    roles: &'a BoundMirrorPieceEdgeRoles<'a>,
    piece_id: &'a EntityId,
    refusal: MirrorPieceIdentityRefusal,
}
impl<'a> MirrorPieceIdentityError<'a> {
    /// Actual original role/catalogue and full source/metadata association.
    #[must_use]
    pub const fn roles(&self) -> &'a BoundMirrorPieceEdgeRoles<'a> {
        self.roles
    }
    /// Actual caller-supplied canonical native declaration field,not an allocated identity.
    #[must_use]
    pub const fn piece_id(&self) -> &'a EntityId {
        self.piece_id
    }
    /// First local intrinsic or known typed-owner alias.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceIdentityRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceIdentityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceIdentityError")
            .field("piece_id", &self.piece_id)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceIdentityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "mirror result Piece {}: {:?}",
            self.piece_id, self.refusal
        )
    }
}
impl std::error::Error for MirrorPieceIdentityError<'_> {}

impl<'s> BoundMirrorPieceEdgeRoles<'s> {
    /// Associate a canonical result Piece id without generating,reserving or publishing identities.
    /// This is native declaration data,not a creation-request field. The trusted host/bus still
    /// supplies issued identities and checks the complete current/historical namespace before use.
    /// Source companion references may intentionally name this planned result;uses are not owners.
    /// Work is O(R+1) under the original whole source R admission,with constant auxiliary space.
    /// # Errors
    /// Operation,source Piece,self-companion,first original source-edge operation,first authored
    /// axis operation,then source and selected result Material aliases,in that order.
    pub fn declare_piece_identity<'a>(
        &'a self,
        piece_id: &'a EntityId,
    ) -> Result<DeclaredMirrorPieceIdentity<'a>, MirrorPieceIdentityError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| MirrorPieceIdentityError {
            roles: self,
            piece_id,
            refusal,
        };
        let metadata = self.metadata();
        let source = metadata.cut().source();
        let input = source.input();
        if *piece_id == input.operation() {
            return Err(refuse(MirrorPieceIdentityRefusal::Operation));
        }
        if *piece_id == source.piece().id() {
            return Err(refuse(MirrorPieceIdentityRefusal::SourcePiece));
        }
        if matches!(metadata.cut().mirroring(), Mirroring::PairMember { companion, .. } if companion == piece_id)
        {
            return Err(refuse(MirrorPieceIdentityRefusal::SelfCompanion));
        }
        for usage in source.edges() {
            let edge = *usage.source().edge();
            if edge.creator() == *piece_id {
                return Err(refuse(MirrorPieceIdentityRefusal::SourceOperation {
                    location: usage.location(),
                    edge,
                }));
            }
        }
        for (index, dependency) in input.dependencies().enumerate() {
            if dependency.source().creator() == *piece_id {
                return Err(refuse(MirrorPieceIdentityRefusal::AxisOperation { index }));
            }
        }
        if matches!(&source.piece().definition().material, MaterialAssignment::Assigned(id) if id == piece_id)
        {
            return Err(refuse(MirrorPieceIdentityRefusal::SourceMaterial));
        }
        if matches!(metadata.material(), MaterialAssignment::Assigned(id) if id == piece_id) {
            return Err(refuse(MirrorPieceIdentityRefusal::ResultMaterial));
        }
        Ok(DeclaredMirrorPieceIdentity {
            roles: self,
            piece_id,
        })
    }
}
/// Authentic locally checked canonical Piece-id declaration,not an issued/fresh/current result.
/// ```compile_fail
/// fn forge<'a>(mut declared:sc_core::construction::DeclaredMirrorPieceIdentity<'a>,
///     other:&'a sc_core::ontology::EntityId) {
///     declared.piece_id=other;
/// }
/// ```
/// ```compile_fail
/// fn escape(roles:sc_core::construction::BoundMirrorPieceEdgeRoles<'_>,id:sc_core::ontology::EntityId)
///     ->sc_core::construction::DeclaredMirrorPieceIdentity<'static> {
///     roles.declare_piece_identity(&id).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct DeclaredMirrorPieceIdentity<'a> {
    roles: &'a BoundMirrorPieceEdgeRoles<'a>,
    piece_id: &'a EntityId,
}
impl<'a> DeclaredMirrorPieceIdentity<'a> {
    /// Actual original role/catalogue/source/metadata parents.
    #[must_use]
    pub const fn roles(&self) -> &'a BoundMirrorPieceEdgeRoles<'a> {
        self.roles
    }
    /// Actual canonical result identity field;no ordinal,label,coordinate or source-id derivation.
    #[must_use]
    pub const fn piece_id(&self) -> &'a EntityId {
        self.piece_id
    }
}
