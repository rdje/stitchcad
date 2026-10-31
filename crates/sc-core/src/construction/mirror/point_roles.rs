//! Actual declared mirror Piece and point catalogue association;not point-source/frame evidence.
use super::{DeclaredMirrorPieceIdentity, MirrorPieceRoleBindingRefusal};
use crate::construction::{AdmittedPointOutputRoleCatalog, PointOutputRoleCatalogLimits};
use core::fmt;

/// Shared D/A/key-byte/operation cause data;this alias carries no catalogue membership proof.
pub type MirrorPiecePointRoleBindingRefusal = MirrorPieceRoleBindingRefusal;

/// Original declared Piece,actual point catalogue and selected limits survive every refusal.
pub struct MirrorPiecePointRoleBindingError<'a> {
    identity: &'a DeclaredMirrorPieceIdentity<'a>,
    catalog: &'a AdmittedPointOutputRoleCatalog<'a>,
    limits: PointOutputRoleCatalogLimits,
    refusal: MirrorPiecePointRoleBindingRefusal,
}
impl<'a> MirrorPiecePointRoleBindingError<'a> {
    /// Actual original canonical Piece-id/edge-role/source/metadata association.
    #[must_use]
    pub const fn identity(&self) -> &'a DeclaredMirrorPieceIdentity<'a> {
        self.identity
    }
    /// Actual complete admitted point catalogue,including inactive definitions.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedPointOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicit complete association policy.
    #[must_use]
    pub const fn limits(&self) -> PointOutputRoleCatalogLimits {
        self.limits
    }
    /// First whole quota or actual operation cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPiecePointRoleBindingRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPiecePointRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPiecePointRoleBindingError")
            .field("piece_id", &self.identity.piece_id())
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPiecePointRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror copy point-role association: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPiecePointRoleBindingError<'_> {}

impl<'s> DeclaredMirrorPieceIdentity<'s> {
    /// Associate the actual mirror operation and canonical Piece declaration with typed point roles.
    /// Whole D/A/key-byte quotas precede owner equality;cached original metadata makes this O(1)
    /// with no workspace allocation. Source existence/ownership/frame,current program and actual
    /// issued history/geometry/global freshness/publication remain separate obligations.
    /// # Errors
    /// Complete definitions,active ids,key-byte quota,then actual mirror/point operation mismatch.
    pub fn bind_point_roles<'a>(
        &'a self,
        catalog: &'a AdmittedPointOutputRoleCatalog<'a>,
        limits: PointOutputRoleCatalogLimits,
    ) -> Result<BoundMirrorPiecePointRoles<'a>, MirrorPiecePointRoleBindingError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| MirrorPiecePointRoleBindingError {
            identity: self,
            catalog,
            limits,
            refusal,
        };
        let definitions = catalog.catalog().definitions().len();
        let active = catalog.catalog().active_tags().len();
        if definitions > limits.definitions() {
            return Err(refuse(MirrorPiecePointRoleBindingRefusal::Definitions {
                required: definitions,
                allowed: limits.definitions(),
            }));
        }
        if active > limits.active() {
            return Err(refuse(MirrorPiecePointRoleBindingRefusal::Active {
                required: active,
                allowed: limits.active(),
            }));
        }
        if catalog.key_bytes() > limits.key_bytes() {
            return Err(refuse(MirrorPiecePointRoleBindingRefusal::KeyBytes {
                required: catalog.key_bytes(),
                allowed: limits.key_bytes(),
            }));
        }
        if self.roles().metadata().cut().source().input().operation()
            != catalog.catalog().operation()
        {
            return Err(refuse(MirrorPiecePointRoleBindingRefusal::Operation));
        }
        Ok(BoundMirrorPiecePointRoles {
            identity: self,
            catalog,
            limits,
        })
    }
}

/// Authentic declared Piece/point-catalogue association,without PointRef generation or source proof.
/// ```compile_fail
/// fn edge<'a>(identity:&'a sc_core::construction::DeclaredMirrorPieceIdentity<'a>,
///     edge:&'a sc_core::construction::AdmittedEdgeOutputRoleCatalog<'a>) {
///     identity.bind_point_roles(edge,sc_core::construction::PointOutputRoleCatalogLimits::new(0,0,0)).unwrap();
/// }
/// ```
/// ```compile_fail
/// fn replace<'a>(mut binding:sc_core::construction::BoundMirrorPiecePointRoles<'a>,
///     other:&'a sc_core::construction::DeclaredMirrorPieceIdentity<'a>) {
///     binding.identity=other;
/// }
/// ```
/// ```compile_fail
/// fn escape(identity:sc_core::construction::DeclaredMirrorPieceIdentity<'_>,
///     points:sc_core::construction::AdmittedPointOutputRoleCatalog<'_>)
///     ->sc_core::construction::BoundMirrorPiecePointRoles<'static> {
///     identity.bind_point_roles(&points,sc_core::construction::PointOutputRoleCatalogLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundMirrorPiecePointRoles<'a> {
    identity: &'a DeclaredMirrorPieceIdentity<'a>,
    catalog: &'a AdmittedPointOutputRoleCatalog<'a>,
    limits: PointOutputRoleCatalogLimits,
}
impl<'a> BoundMirrorPiecePointRoles<'a> {
    /// Actual original Piece-id/edge-role/source/metadata parents.
    #[must_use]
    pub const fn identity(&self) -> &'a DeclaredMirrorPieceIdentity<'a> {
        self.identity
    }
    /// Actual complete typed point catalogue,not an edge-role catalogue proof.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedPointOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicit association policy.
    #[must_use]
    pub const fn limits(&self) -> PointOutputRoleCatalogLimits {
        self.limits
    }
}
