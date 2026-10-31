//! Actual slash intent and shared admitted edge-role catalogue association;no geometry or issuance.
use super::SlashSpreadOutputIntent;
use crate::construction::{AdmittedEdgeOutputRoleCatalog, EdgeOutputRoleCatalogLimits};
use core::fmt;

/// Complete catalogue quota or actual operation association refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadRoleBindingRefusal {
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
    /// Catalogue operation differs from the actual owned slash input's operation.
    Operation,
}
/// Actual whole intent/catalogue and selected limits survive refusal without cloning.
pub struct SlashSpreadRoleBindingError<'a> {
    intent: &'a SlashSpreadOutputIntent,
    catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits: EdgeOutputRoleCatalogLimits,
    refusal: SlashSpreadRoleBindingRefusal,
}
impl<'a> SlashSpreadRoleBindingError<'a> {
    /// Actual original complete slash intent.
    #[must_use]
    pub const fn intent(&self) -> &'a SlashSpreadOutputIntent {
        self.intent
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
    pub const fn refusal(&self) -> SlashSpreadRoleBindingRefusal {
        self.refusal
    }
}
impl fmt::Debug for SlashSpreadRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadRoleBindingError")
            .field("operation", &self.intent.input().operation())
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadRoleBindingError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "slash/spread edge-role association: {:?}", self.refusal)
    }
}
impl std::error::Error for SlashSpreadRoleBindingError<'_> {}

impl SlashSpreadOutputIntent {
    /// Bind actual slash operation identity to an actual admitted shared edge-role catalogue.
    /// Check complete D definitions,A active ids and key bytes before owner association. This stage
    /// allocates no workspace;the earlier catalogue admission/owned storage keep their own limits.
    /// Empty/inactive metadata remains valid here and proves no executable or complete output result.
    /// # Errors
    /// Definition,active-id,key-byte quota,then operation mismatch. No accepted prefix or mutation.
    pub fn bind_edge_roles<'a>(
        &'a self,
        catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
        limits: EdgeOutputRoleCatalogLimits,
    ) -> Result<BoundSlashSpreadEdgeRoles<'a>, SlashSpreadRoleBindingError<'a>> {
        let refuse = |refusal| SlashSpreadRoleBindingError {
            intent: self,
            catalog,
            limits,
            refusal,
        };
        let definitions = catalog.catalog().definitions().len();
        let active = catalog.catalog().active_tags().len();
        if definitions > limits.definitions() {
            return Err(refuse(SlashSpreadRoleBindingRefusal::Definitions {
                required: definitions,
                allowed: limits.definitions(),
            }));
        }
        if active > limits.active() {
            return Err(refuse(SlashSpreadRoleBindingRefusal::Active {
                required: active,
                allowed: limits.active(),
            }));
        }
        if catalog.key_bytes() > limits.key_bytes() {
            return Err(refuse(SlashSpreadRoleBindingRefusal::KeyBytes {
                required: catalog.key_bytes(),
                allowed: limits.key_bytes(),
            }));
        }
        if self.input().operation() != catalog.catalog().operation() {
            return Err(refuse(SlashSpreadRoleBindingRefusal::Operation));
        }
        Ok(BoundSlashSpreadEdgeRoles {
            intent: self,
            catalog,
            limits,
        })
    }
}
/// Authentic operation/catalogue association,not geometric role matching,issuance or currentness.
/// ```compile_fail
/// fn escape(catalog:sc_core::construction::AdmittedEdgeOutputRoleCatalog<'_>,intent:sc_core::construction::SlashSpreadOutputIntent)
///     ->sc_core::construction::BoundSlashSpreadEdgeRoles<'static> {
///     intent.bind_edge_roles(&catalog,sc_core::construction::EdgeOutputRoleCatalogLimits::new(0,0,0)).unwrap()
/// }
/// ```
/// ```compile_fail
/// fn reparent<'a>(mut binding:sc_core::construction::BoundSlashSpreadEdgeRoles<'a>,other:&'a sc_core::construction::SlashSpreadOutputIntent) {
///     binding.intent=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundSlashSpreadEdgeRoles<'a> {
    intent: &'a SlashSpreadOutputIntent,
    catalog: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits: EdgeOutputRoleCatalogLimits,
}
impl<'a> BoundSlashSpreadEdgeRoles<'a> {
    /// Actual original slash intent supplying this operation and all its owned fields.
    #[must_use]
    pub const fn intent(self) -> &'a SlashSpreadOutputIntent {
        self.intent
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
