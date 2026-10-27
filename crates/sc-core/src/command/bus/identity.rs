//! Owned preallocated identities: staging cannot advance the original reserve or call a provider.
use crate::{
    design::{DesignGarment, DesignGarmentPosition, DesignReferenceHost},
    ontology::{EntityId, IdError, IdGenerator},
};
use core::fmt;
use std::{collections::BTreeMap, sync::Arc};

/// Invalid trusted application identity-reserve configuration, before constructing the bus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandIdentityError {
    /// Supplied generator order repeats or goes backwards; the bus never silently sorts it.
    NonIncreasing {
        /// Previous supplied identity.
        previous: EntityId,
        /// Refused supplied identity.
        identity: EntityId,
        /// Zero-based original position.
        index: usize,
    },
    /// A reserved identity already belongs to the actual garment, including retired operations.
    AlreadyOwned {
        /// Refused reserved identity.
        identity: EntityId,
        /// Zero-based original reserve position.
        index: usize,
        /// Exact original owning position.
        owner: DesignGarmentPosition,
    },
}
impl fmt::Display for CommandIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command identity reserve refused: {self:?}")
    }
}
impl std::error::Error for CommandIdentityError {}

#[derive(Default)]
pub(super) struct IdentityReserve {
    // One immutable backing remains for this bus lifetime. Publication advances only the prefix;
    // staging/replay/undo cannot rewind it, and publication never recopies unused identities.
    identities: Arc<[EntityId]>,
    consumed: usize,
}
impl IdentityReserve {
    pub(super) fn new<H: DesignReferenceHost>(
        identities: Vec<EntityId>,
        garment: &DesignGarment<H>,
    ) -> Result<Self, CommandIdentityError> {
        let mut previous = None;
        for (index, identity) in identities.iter().copied().enumerate() {
            if let Some(previous) = previous {
                if identity <= previous {
                    return Err(CommandIdentityError::NonIncreasing {
                        previous,
                        identity,
                        index,
                    });
                }
            }
            previous = Some(identity);
        }
        let owners: BTreeMap<_, _> = garment.identities().into_iter().collect();
        for (index, identity) in identities.iter().copied().enumerate() {
            if let Some(owner) = owners.get(&identity) {
                return Err(CommandIdentityError::AlreadyOwned {
                    identity,
                    index,
                    owner: *owner,
                });
            }
        }
        Ok(Self {
            identities: identities.into(),
            consumed: 0,
        })
    }
    pub(super) fn stage(&self) -> IdentityStage<'_> {
        let remaining = self.identities.iter().skip(self.consumed);
        let initial_len = remaining.len();
        IdentityStage {
            remaining,
            initial_len,
        }
    }
    // Build the publication reserve before changing the original owner. Refuse a bound failure.
    pub(super) fn after_consuming(&self, count: usize) -> Option<Self> {
        let consumed = self.consumed.checked_add(count)?;
        if consumed > self.identities.len() {
            return None;
        }
        Some(Self {
            identities: Arc::clone(&self.identities),
            consumed,
        })
    }
}

// An iterator owns its position and borrows immutable values. No Clone contract can alias a cursor.
pub(super) struct IdentityStage<'a> {
    remaining: core::iter::Skip<core::slice::Iter<'a, EntityId>>,
    initial_len: usize,
}
impl IdentityStage<'_> {
    // Only this concrete iterator advances the cursor, so remaining.len() cannot exceed initial_len.
    pub(super) fn consumed(&self) -> usize {
        self.initial_len - self.remaining.len()
    }
}
impl IdGenerator for IdentityStage<'_> {
    fn next_id(&mut self) -> Result<EntityId, IdError> {
        self.remaining.next().copied().ok_or(IdError::Exhausted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staged_cursor_exhausts_without_advancing_other_cursors_or_original_reserve() {
        let last = EntityId::from_bits(u128::MAX);
        let reserve = IdentityReserve {
            identities: vec![EntityId::from_bits(7), last].into(),
            consumed: 0,
        };
        let mut first = reserve.stage();
        let mut independent = reserve.stage();
        assert_eq!(first.consumed(), 0);
        assert_eq!(first.next_id(), Ok(EntityId::from_bits(7)));
        assert_eq!(first.consumed(), 1);
        assert_eq!(first.next_id(), Ok(last));
        assert_eq!(first.consumed(), 2);
        assert_eq!(first.next_id(), Err(IdError::Exhausted));
        assert_eq!(first.next_id(), Err(IdError::Exhausted));
        assert_eq!(first.consumed(), 2);
        assert_eq!(independent.consumed(), 0);
        assert_eq!(independent.next_id(), Ok(EntityId::from_bits(7)));
        assert_eq!(independent.consumed(), 1);
        assert_eq!(reserve.stage().consumed(), 0);
        assert_eq!(reserve.stage().next_id(), Ok(EntityId::from_bits(7)));
        assert_eq!(
            reserve
                .after_consuming(0)
                .map(|next| next.stage().next_id()),
            Some(Ok(EntityId::from_bits(7))),
            "zero creation preserves the next identity"
        );
        let suffix = reserve.after_consuming(1).map(|next| {
            let mut stage = next.stage();
            let before = stage.consumed();
            let identity = stage.next_id();
            let exhausted = stage.next_id();
            let consumed = stage.consumed();
            let empty = next.after_consuming(1).map(|empty| empty.stage().next_id());
            assert!(next.after_consuming(2).is_none());
            assert!(next.after_consuming(usize::MAX).is_none());
            (before, identity, exhausted, consumed, empty)
        });
        assert_eq!(
            suffix,
            Some((
                0,
                Ok(last),
                Err(IdError::Exhausted),
                1,
                Some(Err(IdError::Exhausted))
            ))
        );
        assert!(reserve.after_consuming(3).is_none());
        assert_eq!(
            reserve.stage().next_id(),
            Ok(EntityId::from_bits(7)),
            "preflight never advances the original reserve"
        );
    }
}
