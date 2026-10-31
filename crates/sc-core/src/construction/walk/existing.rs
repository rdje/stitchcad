//! Bounded actual supplied Notch lookup;membership is not current Design registry completeness.
use super::AdmittedWalkNotchAssignments;
use crate::ontology::{EntityId, Notch};
use core::fmt;
use std::collections::BTreeMap;
/// Explicit whole supplied existing-Notch allowance before any identity scan/allocation.
/// ```compile_fail
/// let limits:sc_core::construction::WalkExistingNotchLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkExistingNotchLimits {
    notches: usize,
}
impl WalkExistingNotchLimits {
    /// Bound complete supplied K records and their K-entry identity index,not allocation bytes or
    /// prior owned Notch storage. This does not bound unprovided registries or whole execution work.
    #[must_use]
    pub const fn new(notches: usize) -> Self {
        Self { notches }
    }
    /// Complete actual supplied-record allowance,including unrelated Notches.
    #[must_use]
    pub const fn notches(self) -> usize {
        self.notches
    }
}
/// Complete quota or first repeated original semantic identity;no accepted prefix escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkExistingNotchRefusal {
    /// Whole supplied population exceeds explicit policy before any record inspection.
    Notches {
        /// Complete actual records.
        required: usize,
        /// Caller allowance.
        allowed: usize,
    },
    /// Two supplied original Notches have the same semantic id,even when all fields are equal.
    DuplicateId {
        /// Original first supplied position.
        first: usize,
        /// Original repeated supplied position.
        second: usize,
        /// Repeated semantic identity.
        id: EntityId,
    },
}
/// Whole actual assignments,provided records,policy and first cause remain inspectable.
pub struct WalkExistingNotchError<'a> {
    assignments: &'a AdmittedWalkNotchAssignments<'a>,
    notches: &'a [Notch],
    limits: WalkExistingNotchLimits,
    refusal: WalkExistingNotchRefusal,
}
impl<'a> WalkExistingNotchError<'a> {
    /// Actual complete occurrence/copy/role/source context.
    #[must_use]
    pub const fn assignments(&self) -> &'a AdmittedWalkNotchAssignments<'a> {
        self.assignments
    }
    /// Actual whole caller-supplied immutable collection,in original order.
    #[must_use]
    pub const fn notches(&self) -> &'a [Notch] {
        self.notches
    }
    /// Original explicit complete-record/index allowance.
    #[must_use]
    pub const fn limits(&self) -> WalkExistingNotchLimits {
        self.limits
    }
    /// Complete resource or first original repeated-id cause.
    #[must_use]
    pub const fn refusal(&self) -> WalkExistingNotchRefusal {
        self.refusal
    }
}
impl fmt::Debug for WalkExistingNotchError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalkExistingNotchError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for WalkExistingNotchError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "walk existing Notches: {:?}", self.refusal)
    }
}
impl std::error::Error for WalkExistingNotchError<'_> {}
impl<'s> AdmittedWalkNotchAssignments<'s> {
    /// Index complete caller-supplied existing canonical Notch objects without cloning their fields.
    /// Whole K quota precedes every id scan/allocation. Original duplicate-id order is retained even
    /// for equal objects. Work is O(K log(K+2)),with K new logical index entries. Other Notch fields,
    /// ownership/liveness,actual current registry completeness,namespace/history/global freshness,
    /// retirement,retention suitability,sharing/geometry and publication are separate obligations.
    /// # Errors
    /// Whole supplied-record quota,then first repeated original semantic identity.
    pub fn bind_existing_notches<'a>(
        &'a self,
        notches: &'a [Notch],
        limits: WalkExistingNotchLimits,
    ) -> Result<BoundWalkExistingNotches<'a>, WalkExistingNotchError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| WalkExistingNotchError {
            assignments: self,
            notches,
            limits,
            refusal,
        };
        if notches.len() > limits.notches {
            return Err(refuse(WalkExistingNotchRefusal::Notches {
                required: notches.len(),
                allowed: limits.notches,
            }));
        }
        let mut by_id = BTreeMap::new();
        for (index, notch) in notches.iter().enumerate() {
            if let Some((first, _)) = by_id.insert(notch.id(), (index, notch)) {
                return Err(refuse(WalkExistingNotchRefusal::DuplicateId {
                    first,
                    second: index,
                    id: notch.id(),
                }));
            }
        }
        Ok(BoundWalkExistingNotches {
            assignments: self,
            notches,
            limits,
            by_id,
        })
    }
}
/// Authentic complete supplied-object membership;not current registry,retention or new-id authority.
/// ```compile_fail
/// fn reparent<'a>(mut existing:sc_core::construction::BoundWalkExistingNotches<'a>,
///     other:&'a sc_core::construction::AdmittedWalkNotchAssignments<'a>) {existing.assignments=other;}
/// ```
/// ```compile_fail
/// fn escape(assignments:sc_core::construction::AdmittedWalkNotchAssignments<'_>)
///     ->sc_core::construction::BoundWalkExistingNotches<'static> {
///     let notches=vec![];assignments.bind_existing_notches(&notches,
///         sc_core::construction::WalkExistingNotchLimits::new(0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct BoundWalkExistingNotches<'a> {
    assignments: &'a AdmittedWalkNotchAssignments<'a>,
    notches: &'a [Notch],
    limits: WalkExistingNotchLimits,
    by_id: BTreeMap<EntityId, (usize, &'a Notch)>,
}
impl<'a> BoundWalkExistingNotches<'a> {
    /// Actual whole original assignment/copy/role/source context.
    #[must_use]
    pub const fn assignments(&self) -> &'a AdmittedWalkNotchAssignments<'a> {
        self.assignments
    }
    /// Actual complete supplied immutable Notch objects,in authored order.
    #[must_use]
    pub const fn notches(&self) -> &'a [Notch] {
        self.notches
    }
    /// Original explicit complete-record/index allowance.
    #[must_use]
    pub const fn limits(&self) -> WalkExistingNotchLimits {
        self.limits
    }
    /// Complete admitted K-entry logical index size,not allocation bytes or unprovided registries.
    #[must_use]
    pub fn indexed_count(&self) -> usize {
        self.by_id.len()
    }
    /// Actual provided object by stable semantic id,in O(log(K+2)) work. None means unlisted here,
    /// not globally fresh,retired or absent from the actual current Design. No owner/field is changed.
    #[must_use]
    pub fn notch(&self, id: EntityId) -> Option<&'a Notch> {
        self.by_id.get(&id).map(|(_, notch)| *notch)
    }
}
