//! Actual complete walk copy/pattern context and typed notch-role association;not output issuance.
use super::BoundWalkCopyInventory;
use crate::construction::{AdmittedNotchOutputRoleCatalog, NotchOutputRoleCatalogLimits};
use core::fmt;
/// Whole catalogue quota or actual operation-association refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkNotchRoleBindingRefusal {
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
    /// Catalogue operation differs from this actual original walk operation.
    Operation,
}
/// Actual whole copy context,typed notch catalogue,policy and first refusal.
pub struct WalkNotchRoleBindingError<'a> {
    copies: &'a BoundWalkCopyInventory<'a>,
    catalog: &'a AdmittedNotchOutputRoleCatalog<'a>,
    limits: NotchOutputRoleCatalogLimits,
    refusal: WalkNotchRoleBindingRefusal,
}
impl<'a> WalkNotchRoleBindingError<'a> {
    /// Actual complete copy/pattern/source graph/span/pair context.
    #[must_use]
    pub const fn copies(&self) -> &'a BoundWalkCopyInventory<'a> {
        self.copies
    }
    /// Actual whole admitted notch metadata,including inactive definitions.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedNotchOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicit complete association policy.
    #[must_use]
    pub const fn limits(&self) -> NotchOutputRoleCatalogLimits {
        self.limits
    }
    /// First complete quota or actual operation cause.
    #[must_use]
    pub const fn refusal(&self) -> WalkNotchRoleBindingRefusal {
        self.refusal
    }
}
impl fmt::Debug for WalkNotchRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalkNotchRoleBindingError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for WalkNotchRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "walk notch-role association: {:?}", self.refusal)
    }
}
impl std::error::Error for WalkNotchRoleBindingError<'_> {}
impl<'s> BoundWalkCopyInventory<'s> {
    /// Bind this actual walk operation/copy/pattern context to distinct admitted notch-role metadata.
    /// Whole D/A/key-byte quotas precede operation equality. Cached original counts make association
    /// constant work/space with no scans or allocation. Empty/inactive metadata may bind;complete
    /// occurrence assignment and semantic identity/sharing policy remain required. No actual history,
    /// currentness,provider/profile/correspondence/geometry/effect/global issuance or authority follows.
    /// # Errors
    /// Whole definition,active occurrence,key-byte quota,then actual walk/catalogue operation mismatch.
    pub fn bind_notch_roles<'a>(
        &'a self,
        catalog: &'a AdmittedNotchOutputRoleCatalog<'a>,
        limits: NotchOutputRoleCatalogLimits,
    ) -> Result<BoundWalkNotchRoles<'a>, WalkNotchRoleBindingError<'a>>
    where
        's: 'a,
    {
        use WalkNotchRoleBindingRefusal as R;
        let refuse = |refusal| WalkNotchRoleBindingError {
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
        if self.source().admitted().edit().operation() != catalog.catalog().operation() {
            return Err(refuse(R::Operation));
        }
        Ok(BoundWalkNotchRoles {
            copies: self,
            catalog,
            limits,
        })
    }
}
/// Authentic actual walk/notch-role association,not semantic assignment or an issued Notch.
/// ```compile_fail
/// fn edge<'a>(copies:&'a sc_core::construction::BoundWalkCopyInventory<'a>,
///     catalog:&'a sc_core::construction::AdmittedEdgeOutputRoleCatalog<'a>) {
///     copies.bind_notch_roles(catalog,sc_core::construction::NotchOutputRoleCatalogLimits::new(0,0,0));
/// }
/// ```
/// ```compile_fail
/// fn point<'a>(copies:&'a sc_core::construction::BoundWalkCopyInventory<'a>,
///     catalog:&'a sc_core::construction::AdmittedPointOutputRoleCatalog<'a>) {
///     copies.bind_notch_roles(catalog,sc_core::construction::NotchOutputRoleCatalogLimits::new(0,0,0));
/// }
/// ```
/// ```compile_fail
/// fn reparent<'a>(mut roles:sc_core::construction::BoundWalkNotchRoles<'a>,
///     other:&'a sc_core::construction::BoundWalkCopyInventory<'a>) {roles.copies=other;}
/// ```
/// ```compile_fail
/// fn escape(copies:sc_core::construction::BoundWalkCopyInventory<'_>,
///     catalog:sc_core::construction::AdmittedNotchOutputRoleCatalog<'_>)
///     ->sc_core::construction::BoundWalkNotchRoles<'static> {
///     copies.bind_notch_roles(&catalog,sc_core::construction::NotchOutputRoleCatalogLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundWalkNotchRoles<'a> {
    copies: &'a BoundWalkCopyInventory<'a>,
    catalog: &'a AdmittedNotchOutputRoleCatalog<'a>,
    limits: NotchOutputRoleCatalogLimits,
}
impl<'a> BoundWalkNotchRoles<'a> {
    /// Actual complete physical-copy/pattern/source context.
    #[must_use]
    pub const fn copies(&self) -> &'a BoundWalkCopyInventory<'a> {
        self.copies
    }
    /// Actual complete distinct notch catalogue and original membership fields.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedNotchOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicitly selected whole association policy.
    #[must_use]
    pub const fn limits(&self) -> NotchOutputRoleCatalogLimits {
        self.limits
    }
}
