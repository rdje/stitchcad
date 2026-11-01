//! Actual full join/edge-role association;metadata grants no output shape,geometry or issuance.
use super::BoundJoinCutBanks;
use crate::construction::{AdmittedEdgeOutputRoleCatalog, EdgeOutputRoleCatalogLimits};
use core::fmt;

/// First whole catalogue quota or actual operation-owner refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinEdgeRoleBindingRefusal {
    /// Whole definitions,including inactive/unrelated roles,exceed policy.
    Definitions {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Every actual active occurrence exceeds policy.
    Active {
        /// Complete original count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original cached machine-key bytes exceed policy.
    KeyBytes {
        /// Complete actual bytes.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Actual catalogue operation differs from this original join operation.
    Operation,
}
/// Whole actual banks,catalogue,policy and first cause survive every refusal.
pub struct JoinEdgeRoleBindingError<'a> {
    banks: &'a BoundJoinCutBanks<'a>,
    catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits: EdgeOutputRoleCatalogLimits,
    refusal: JoinEdgeRoleBindingRefusal,
}
impl<'a> JoinEdgeRoleBindingError<'a> {
    /// Actual original complete A/B bank/source/population/metadata context.
    #[must_use]
    pub const fn banks(&self) -> &'a BoundJoinCutBanks<'a> {
        self.banks
    }
    /// Actual whole typed admitted Edge catalogue,including inactive definitions.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedEdgeOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicit full association policy.
    #[must_use]
    pub const fn limits(&self) -> EdgeOutputRoleCatalogLimits {
        self.limits
    }
    /// First complete quota or actual operation-owner cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinEdgeRoleBindingRefusal {
        self.refusal
    }
}
impl fmt::Debug for JoinEdgeRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinEdgeRoleBindingError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinEdgeRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join edge-role association: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinEdgeRoleBindingError<'_> {}
impl<'s> BoundJoinCutBanks<'s> {
    /// Bind distinct actual Edge role metadata to this original join operation and both banks.
    /// Whole D/A/key-byte quotas precede owner equality. Original admitted counts/key bytes are
    /// cached,so this stage performs constant work/space with no key scan or allocation.
    /// Earlier catalogue admission/storage and source/context resources remain separate.
    /// Empty/inactive metadata can bind;no empty result,no-op,source/output attribution,stable-role
    /// history,geometry,currentness,global freshness,identity issuance or authority follows.
    /// # Errors
    /// Complete definitions,active occurrences,key-byte quota,then actual operation mismatch.
    pub fn bind_edge_roles<'a>(
        &'a self,
        catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
        limits: EdgeOutputRoleCatalogLimits,
    ) -> Result<BoundJoinEdgeRoles<'a>, JoinEdgeRoleBindingError<'a>>
    where
        's: 'a,
    {
        use JoinEdgeRoleBindingRefusal as R;
        let refuse = |refusal| JoinEdgeRoleBindingError {
            banks: self,
            catalog,
            limits,
            refusal,
        };
        let definitions = catalog.catalog().definitions().len();
        let active = catalog.catalog().active_tags().len();
        if definitions > limits.definitions() {
            return Err(refuse(R::Definitions {
                required: definitions,
                allowed: limits.definitions(),
            }));
        }
        if active > limits.active() {
            return Err(refuse(R::Active {
                required: active,
                allowed: limits.active(),
            }));
        }
        if catalog.key_bytes() > limits.key_bytes() {
            return Err(refuse(R::KeyBytes {
                required: catalog.key_bytes(),
                allowed: limits.key_bytes(),
            }));
        }
        if self
            .sources()
            .population()
            .metadata()
            .splice()
            .source()
            .edit()
            .operation()
            != catalog.catalog().operation()
        {
            return Err(refuse(R::Operation));
        }
        Ok(BoundJoinEdgeRoles {
            banks: self,
            catalog,
            limits,
        })
    }
}
/// Authentic complete actual operation/Edge catalogue association,not an output or issued identity.
/// ```compile_fail
/// fn notch<'a>(banks:&'a sc_core::construction::BoundJoinCutBanks<'a>,catalog:&'a sc_core::construction::AdmittedNotchOutputRoleCatalog<'a>) {
///     banks.bind_edge_roles(catalog,sc_core::construction::EdgeOutputRoleCatalogLimits::new(0,0,0));
/// }
/// ```
/// ```compile_fail
/// fn point<'a>(banks:&'a sc_core::construction::BoundJoinCutBanks<'a>,catalog:&'a sc_core::construction::AdmittedPointOutputRoleCatalog<'a>) {
///     banks.bind_edge_roles(catalog,sc_core::construction::EdgeOutputRoleCatalogLimits::new(0,0,0));
/// }
/// ```
/// ```compile_fail
/// fn forge<'a>(mut roles:sc_core::construction::BoundJoinEdgeRoles<'a>,other:&'a sc_core::construction::BoundJoinCutBanks<'a>) {roles.banks=other;}
/// ```
/// ```compile_fail
/// fn escape(banks:sc_core::construction::BoundJoinCutBanks<'_>,catalog:sc_core::construction::AdmittedEdgeOutputRoleCatalog<'_>)
///     ->sc_core::construction::BoundJoinEdgeRoles<'static> {
///     banks.bind_edge_roles(&catalog,sc_core::construction::EdgeOutputRoleCatalogLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct BoundJoinEdgeRoles<'a> {
    banks: &'a BoundJoinCutBanks<'a>,
    catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits: EdgeOutputRoleCatalogLimits,
}
impl<'a> BoundJoinEdgeRoles<'a> {
    /// Actual original complete A/B banks and their source/population/metadata parents.
    #[must_use]
    pub const fn banks(&self) -> &'a BoundJoinCutBanks<'a> {
        self.banks
    }
    /// Actual complete admitted distinct Edge catalogue and its authentic membership views.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedEdgeOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicit complete association policy.
    #[must_use]
    pub const fn limits(&self) -> EdgeOutputRoleCatalogLimits {
        self.limits
    }
}
