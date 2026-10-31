//! Actual mirror metadata and shared admitted edge-role catalogue association;no geometry or issuance.
use super::BoundMirrorPieceMetadata;
use crate::construction::{AdmittedEdgeOutputRoleCatalog, EdgeOutputRoleCatalogLimits};
use core::fmt;

/// Complete catalogue quota or actual operation association refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceRoleBindingRefusal {
    /// All definitions,including inactive roles,exceed policy.
    Definitions {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete active-id population exceeds policy.
    Active {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original key bytes exceed policy.
    KeyBytes {
        /// Complete required byte count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Catalogue operation differs from the actual mirror input's operation.
    Operation,
}
/// Actual whole metadata/catalogue and selected limits survive refusal without cloning.
pub struct MirrorPieceRoleBindingError<'a> {
    metadata: &'a BoundMirrorPieceMetadata<'a>,
    catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits: EdgeOutputRoleCatalogLimits,
    refusal: MirrorPieceRoleBindingRefusal,
}
impl<'a> MirrorPieceRoleBindingError<'a> {
    /// Actual original complete mirror source/cut/field metadata.
    #[must_use]
    pub const fn metadata(&self) -> &'a BoundMirrorPieceMetadata<'a> {
        self.metadata
    }
    /// Actual original admitted edge catalogue,including inactive definitions.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedEdgeOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicit association limits.
    #[must_use]
    pub const fn limits(&self) -> EdgeOutputRoleCatalogLimits {
        self.limits
    }
    /// First complete-quota or operation cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceRoleBindingRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceRoleBindingError")
            .field(
                "operation",
                &self.metadata.cut().source().input().operation(),
            )
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror copy edge-role association: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceRoleBindingError<'_> {}

impl<'s> BoundMirrorPieceMetadata<'s> {
    /// Bind the actual mirror metadata operation identity to an actual admitted shared edge-role catalogue.
    /// Check complete D definitions,A active ids and key bytes before owner association. This stage
    /// allocates no workspace;the earlier catalogue admission/owned storage keep their own limits.
    /// Empty/inactive metadata remains valid here and proves no executable or complete output result.
    /// # Errors
    /// Definition,active-id,key-byte quota,then operation mismatch. No accepted prefix or mutation.
    pub fn bind_edge_roles<'a>(
        &'a self,
        catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
        limits: EdgeOutputRoleCatalogLimits,
    ) -> Result<BoundMirrorPieceEdgeRoles<'a>, MirrorPieceRoleBindingError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| MirrorPieceRoleBindingError {
            metadata: self,
            catalog,
            limits,
            refusal,
        };
        let definitions = catalog.catalog().definitions().len();
        let active = catalog.catalog().active_tags().len();
        if definitions > limits.definitions() {
            return Err(refuse(MirrorPieceRoleBindingRefusal::Definitions {
                required: definitions,
                allowed: limits.definitions(),
            }));
        }
        if active > limits.active() {
            return Err(refuse(MirrorPieceRoleBindingRefusal::Active {
                required: active,
                allowed: limits.active(),
            }));
        }
        if catalog.key_bytes() > limits.key_bytes() {
            return Err(refuse(MirrorPieceRoleBindingRefusal::KeyBytes {
                required: catalog.key_bytes(),
                allowed: limits.key_bytes(),
            }));
        }
        if self.cut().source().input().operation() != catalog.catalog().operation() {
            return Err(refuse(MirrorPieceRoleBindingRefusal::Operation));
        }
        Ok(BoundMirrorPieceEdgeRoles {
            metadata: self,
            catalog,
            limits,
        })
    }
}
/// Authentic operation/catalogue association,not geometric role matching,issuance or currentness.
/// ```compile_fail
/// fn escape(catalog:sc_core::construction::AdmittedEdgeOutputRoleCatalog<'_>,metadata:sc_core::construction::BoundMirrorPieceMetadata<'_>)
///     ->sc_core::construction::BoundMirrorPieceEdgeRoles<'static> {
///     metadata.bind_edge_roles(&catalog,sc_core::construction::EdgeOutputRoleCatalogLimits::new(0,0,0)).unwrap()
/// }
/// ```
/// ```compile_fail
/// fn reparent<'a>(mut binding:sc_core::construction::BoundMirrorPieceEdgeRoles<'a>,other:&'a sc_core::construction::BoundMirrorPieceMetadata<'a>) {
///     binding.metadata=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundMirrorPieceEdgeRoles<'a> {
    metadata: &'a BoundMirrorPieceMetadata<'a>,
    catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits: EdgeOutputRoleCatalogLimits,
}
impl<'a> BoundMirrorPieceEdgeRoles<'a> {
    /// Actual original mirror metadata supplying this operation and selected result fields.
    #[must_use]
    pub const fn metadata(self) -> &'a BoundMirrorPieceMetadata<'a> {
        self.metadata
    }
    /// Actual complete admitted edge catalogue;its field views remain catalogue membership proofs.
    #[must_use]
    pub const fn catalog(self) -> &'a AdmittedEdgeOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Explicit association-stage limits,without prior allocation or whole execution guarantees.
    #[must_use]
    pub const fn limits(self) -> EdgeOutputRoleCatalogLimits {
        self.limits
    }
}
