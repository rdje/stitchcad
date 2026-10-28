//! Owned closure/instance identities and complete current notion-placement sources.
use super::{
    Closure, ClosureError, CutPlan, EntityId, IdentityLedger, NotionPlacementInventory,
    NotionPlacementInventoryError, Piece,
};
use core::fmt;
use std::collections::BTreeMap;

/// Original owning position in the local Closure/instance identity census; targets remain uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosureInventoryPosition {
    /// First-class Closure in authored collection order.
    Closure {
        /// Zero-based collection position.
        index: usize,
    },
    /// Physical instance within its actual owning Closure.
    Instance {
        /// Zero-based owning Closure position.
        closure: usize,
        /// Zero-based authored instance position.
        index: usize,
    },
}
/// Original collection/current-source cause with authored context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClosureInventoryError {
    /// Actual Closure or instance objects claim one owned identity.
    DuplicateIdentity {
        /// Exact repeated identity.
        identity: EntityId,
        /// Original first owning position.
        first: ClosureInventoryPosition,
        /// Original first repeated owning position.
        duplicate: ClosureInventoryPosition,
    },
    /// Complete current plan and every supplied notion placement, including unused objects.
    Notions(Box<NotionPlacementInventoryError>),
    /// An actual Closure's original current instance/target refusal.
    Closure {
        /// Zero-based authored Closure position.
        index: usize,
        /// Actual Closure identity.
        closure: EntityId,
        /// Original concrete current target cause.
        issue: Box<ClosureError>,
    },
}
impl fmt::Display for ClosureInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f, "Closure/instance identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::Notions(issue) => write!(f, "Closure inventory current notions: {issue}"),
            Self::Closure { index, closure, issue } => write!(f, "Closure {closure} at {index}: {issue}"),
        }
    }
}
impl std::error::Error for ClosureInventoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Notions(issue) => Some(issue.as_ref()),
            Self::Closure { issue, .. } => Some(issue.as_ref()),
            Self::DuplicateIdentity { .. } => None,
        }
    }
}

/// Private immutable authored Closures; contexts are borrowed only for current validation.
///
/// Construction checks all local Closure/instance identities and retains the exact objects, including
/// targets held across edits. Current validation checks the complete supplied notion inventory first,
/// then every Closure in order. It does not add a placement-reuse restriction across Closures.
/// Global Design identity joins, logical declarations and G2/G3/G4 physical proof remain separate.
/// An explicit empty collection is a draft scope, not assembly or release readiness.
///
/// ```compile_fail
/// fn edit(value: &mut sc_core::ontology::ClosureInventory) { value.closures.clear(); }
/// ```
/// ```compile_fail
/// fn edit(value: &mut sc_core::ontology::ClosureInventory) { value.closures().get_mut(0); }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct ClosureInventory {
    closures: Vec<Closure>,
}
impl fmt::Debug for ClosureInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClosureInventory")
            .field("closure_count", &self.closures.len())
            .finish_non_exhaustive()
    }
}
impl ClosureInventory {
    pub(crate) fn resource_closures(&self) -> &[Closure] {
        let Self { closures } = self;
        closures
    }
    /// Own original ordered objects without resolving or rewriting held targets.
    ///
    /// # Errors
    /// Returns the first repeated Closure/instance identity with both original owning positions.
    pub fn new(closures: Vec<Closure>) -> Result<Self, ClosureInventoryError> {
        let mut seen = BTreeMap::new();
        for (index, closure) in closures.iter().enumerate() {
            let identities =
                core::iter::once((closure.id(), ClosureInventoryPosition::Closure { index }))
                    .chain(closure.definition().instances.iter().enumerate().map(
                        |(instance, value)| {
                            (
                                value.id,
                                ClosureInventoryPosition::Instance {
                                    closure: index,
                                    index: instance,
                                },
                            )
                        },
                    ));
            for (identity, duplicate) in identities {
                if let Some(first) = seen.insert(identity, duplicate) {
                    return Err(ClosureInventoryError::DuplicateIdentity {
                        identity,
                        first,
                        duplicate,
                    });
                }
            }
        }
        Ok(Self { closures })
    }
    /// Exact authored objects; shared inspection grants no mutator or current source proof.
    #[must_use]
    pub fn closures(&self) -> &[Closure] {
        &self.closures
    }
    /// Inspect an original Closure by exact identity without current validation or retargeting.
    #[must_use]
    pub fn closure(&self, identity: EntityId) -> Option<&Closure> {
        self.closures
            .iter()
            .find(|closure| closure.id() == identity)
    }
    /// Check all supplied notions/current copies first, then every Closure's current targets.
    ///
    /// Even an empty Closure collection checks unused placements and the complete supplied plan.
    /// Current contexts are never cached; raw repair/choice and physical scope rules are unchanged.
    ///
    /// # Errors
    /// Returns the original notion-inventory cause, then first Closure failure in authored order.
    pub fn validate_current(
        &self,
        placements: &NotionPlacementInventory,
        plan: &CutPlan,
        pieces: &[Piece],
        ledger: &IdentityLedger,
    ) -> Result<(), ClosureInventoryError> {
        placements
            .validate_current(plan, pieces, ledger)
            .map_err(|issue| ClosureInventoryError::Notions(Box::new(issue)))?;
        for (index, closure) in self.closures().iter().enumerate() {
            closure
                .validate_current(placements.placements(), plan, pieces, ledger)
                .map_err(|issue| ClosureInventoryError::Closure {
                    index,
                    closure: closure.id(),
                    issue: Box::new(issue),
                })?;
        }
        Ok(())
    }
}
