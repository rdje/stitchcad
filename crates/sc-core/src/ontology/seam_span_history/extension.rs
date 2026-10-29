//! One bounded immutable transition over already validated chronological history.
use super::{
    EntityId, SeamSpanHistory, SeamSpanHistoryEntry, SeamSpanHistoryError, SeamSpanHistoryLimits,
    SeamSpanHistoryState,
};

impl SeamSpanHistory {
    /// Append one supplied split to a new snapshot without rewriting this history.
    /// Admit the complete next entry and issued-identity populations before parent/child checks
    /// or any vector allocation. A missing/retired parent precedes either child; child nonreuse
    /// includes every retired ancestor and unrelated birth. The accepted prefix stays exact.
    ///
    /// Child order is supplied A order, not numeric identity order. This generates or reserves
    /// no IDs, grants no global freshness and proves no actual graph edit or command authority.
    /// Production allocation still belongs to the trusted bus reserve. Selected limits remain
    /// runtime inputs and are not retained as canonical history content.
    /// ```
    /// use core::num::NonZeroUsize;
    /// use sc_core::ontology::{EntityId, SeamSpanHistory, SeamSpanHistoryEntry as E,
    ///     SeamSpanHistoryLimits, SeamSpanHistoryState as S};
    /// let parent = EntityId::from_bits(7);
    /// let children = [EntityId::from_bits(9), EntityId::from_bits(3)];
    /// let limits = SeamSpanHistoryLimits::new(NonZeroUsize::new(2).unwrap(),
    ///     NonZeroUsize::new(3).unwrap());
    /// let original = SeamSpanHistory::new(vec![E::Root(parent)], limits).unwrap();
    /// let candidate = original.with_split(parent, children, limits).unwrap();
    /// assert_eq!(original.state(parent), Some(S::Live { birth_entry: 0 }));
    /// assert!(matches!(candidate.state(parent), Some(S::RetiredSplit { children: ids, .. })
    ///     if *ids == children));
    /// assert_eq!(candidate.entries().first(), original.entries().first());
    /// ```
    /// # Errors
    /// Preserves actual complete counts, original missing/retired parent evidence and original
    /// reused birth locations. Neither a partial new history nor a changed original can escape.
    pub fn with_split(
        &self,
        parent: EntityId,
        children: [EntityId; 2],
        limits: SeamSpanHistoryLimits,
    ) -> Result<Self, SeamSpanHistoryError> {
        use SeamSpanHistoryError as Error;
        let entry = self.entries.len();
        let required_entries = entry.checked_add(1).ok_or(Error::EntryCountOverflow)?;
        if required_entries > limits.entries.get() {
            return Err(Error::Entries {
                required: required_entries,
                limit: limits.entries.get(),
            });
        }
        let identities = self
            .identities
            .checked_add(2)
            .ok_or(Error::IdentityCountOverflow { entry })?;
        if identities > limits.identities.get() {
            return Err(Error::Identities {
                required: identities,
                limit: limits.identities.get(),
            });
        }
        match self.state(parent) {
            None => return Err(Error::MissingParent { entry, parent }),
            Some(SeamSpanHistoryState::RetiredSplit { split_entry, .. }) => {
                return Err(Error::RetiredParent {
                    entry,
                    parent,
                    split_entry,
                });
            }
            Some(SeamSpanHistoryState::Live { .. }) => {}
        }
        for identity in children {
            if let Some((_, first_birth, _)) = self.births().find(|(id, ..)| *id == identity) {
                return Err(Error::ReusedIdentity {
                    entry,
                    identity,
                    first_birth,
                });
            }
        }
        let [first, second] = children;
        if first == second {
            return Err(Error::ReusedIdentity {
                entry,
                identity: second,
                first_birth: entry,
            });
        }
        // Existing history is immutable and valid. The only new record has now passed every
        // chronological rule, so no second validation index or caller-provided clone is needed.
        let mut entries = Vec::with_capacity(required_entries);
        entries.extend_from_slice(&self.entries);
        entries.push(SeamSpanHistoryEntry::Split { parent, children });
        Ok(Self {
            entries,
            identities,
        })
    }
}
