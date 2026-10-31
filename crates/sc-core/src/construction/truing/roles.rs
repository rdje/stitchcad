//! Actual complete truing copy/pattern context and typed edge-role association;not output issuance.
use super::BoundTruingCopyInventory;
use crate::construction::{AdmittedEdgeOutputRoleCatalog, EdgeOutputRoleCatalogLimits};
use core::fmt;
/// Whole catalogue quota or actual operation-association refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingEdgeRoleBindingRefusal {
    /// Whole supplied definition population exceeds this association policy.
    Definitions {
        /// Complete actual count.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Whole supplied active occurrences exceed this association policy.
    Active {
        /// Complete actual count.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Whole actual machine-key bytes exceed this association policy.
    KeyBytes {
        /// Complete actual bytes.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Catalogue operation differs from this actual original truing operation.
    Operation,
}
/// Actual whole copy context,typed edge catalogue,policy and first refusal.
pub struct TruingEdgeRoleBindingError<'a> {
    copies: &'a BoundTruingCopyInventory<'a>,
    catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits: EdgeOutputRoleCatalogLimits,
    refusal: TruingEdgeRoleBindingRefusal,
}
impl<'a> TruingEdgeRoleBindingError<'a> {
    /// Actual complete copy/pattern/source graph/span/quantitative-provider context.
    #[must_use]
    pub const fn copies(&self) -> &'a BoundTruingCopyInventory<'a> {
        self.copies
    }
    /// Actual whole admitted edge metadata,including inactive definitions.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedEdgeOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicit complete association policy.
    #[must_use]
    pub const fn limits(&self) -> EdgeOutputRoleCatalogLimits {
        self.limits
    }
    /// First complete quota or actual operation cause.
    #[must_use]
    pub const fn refusal(&self) -> TruingEdgeRoleBindingRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingEdgeRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingEdgeRoleBindingError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingEdgeRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing edge-role association: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingEdgeRoleBindingError<'_> {}
impl<'s> BoundTruingCopyInventory<'s> {
    /// Bind this actual truing operation/copy/pattern context to distinct admitted edge-role metadata.
    /// Whole D/A/key-byte quotas precede operation equality. Cached original counts make association
    /// constant work/space with no scans or allocation. Empty/inactive metadata may bind;complete
    /// output manifest/reference/fitting policies remain required. No actual history,
    /// currentness,provider/endpoint-stop/correspondence/geometry/effect/global issuance or authority follows.
    /// # Errors
    /// Whole definition,active occurrence,key-byte quota,then actual truing/catalogue operation mismatch.
    pub fn bind_edge_roles<'a>(
        &'a self,
        catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
        limits: EdgeOutputRoleCatalogLimits,
    ) -> Result<BoundTruingEdgeRoles<'a>, TruingEdgeRoleBindingError<'a>>
    where
        's: 'a,
    {
        use TruingEdgeRoleBindingRefusal as R;
        let refuse = |refusal| TruingEdgeRoleBindingError {
            copies: self,
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
        if self.source().edit().operation() != catalog.catalog().operation() {
            return Err(refuse(R::Operation));
        }
        Ok(BoundTruingEdgeRoles {
            copies: self,
            catalog,
            limits,
        })
    }
}
/// Authentic actual truing/edge-role association,not a complete refit manifest or an issued edge.
/// ```compile_fail
/// fn notch<'a>(copies:&'a sc_core::construction::BoundTruingCopyInventory<'a>,
///     catalog:&'a sc_core::construction::AdmittedNotchOutputRoleCatalog<'a>) {
///     copies.bind_edge_roles(catalog,sc_core::construction::EdgeOutputRoleCatalogLimits::new(0,0,0));
/// }
/// ```
/// ```compile_fail
/// fn point<'a>(copies:&'a sc_core::construction::BoundTruingCopyInventory<'a>,
///     catalog:&'a sc_core::construction::AdmittedPointOutputRoleCatalog<'a>) {
///     copies.bind_edge_roles(catalog,sc_core::construction::EdgeOutputRoleCatalogLimits::new(0,0,0));
/// }
/// ```
/// ```compile_fail
/// fn reparent<'a>(mut roles:sc_core::construction::BoundTruingEdgeRoles<'a>,
///     other:&'a sc_core::construction::BoundTruingCopyInventory<'a>) {roles.copies=other;}
/// ```
/// ```compile_fail
/// fn escape(copies:sc_core::construction::BoundTruingCopyInventory<'_>,
///     catalog:sc_core::construction::AdmittedEdgeOutputRoleCatalog<'_>)
///     ->sc_core::construction::BoundTruingEdgeRoles<'static> {
///     copies.bind_edge_roles(&catalog,sc_core::construction::EdgeOutputRoleCatalogLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundTruingEdgeRoles<'a> {
    copies: &'a BoundTruingCopyInventory<'a>,
    catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits: EdgeOutputRoleCatalogLimits,
}
impl<'a> BoundTruingEdgeRoles<'a> {
    /// Actual complete physical-copy/pattern/source context.
    #[must_use]
    pub const fn copies(&self) -> &'a BoundTruingCopyInventory<'a> {
        self.copies
    }
    /// Actual complete distinct edge catalogue and original membership fields.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedEdgeOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicitly selected whole association policy.
    #[must_use]
    pub const fn limits(&self) -> EdgeOutputRoleCatalogLimits {
        self.limits
    }
}
