//! Bounded supplied span identity history; graph membership and command authority are separate.
use super::EntityId;
use core::{fmt, num::NonZeroUsize};
use std::collections::BTreeMap;

/// One ordered birth or retirement record; child order follows the original side-A partition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeamSpanHistoryEntry {
    /// An independently introduced span identity, not a child of an earlier split.
    Root(EntityId),
    /// Retire one live parent and introduce two fresh child identities in A order.
    Split {
        /// Original live parent; its identity is never transferred to either child.
        parent: EntityId,
        /// Immediate first/second children, retained even if either is later retired.
        children: [EntityId; 2],
    },
}

/// Explicit logical entry and total-issued-identity allowances, including retired identities.
/// These bound supplied records and the temporary validation map, not allocator bytes or CPU.
/// ```compile_fail
/// let limits: sc_core::ontology::SeamSpanHistoryLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeamSpanHistoryLimits {
    entries: NonZeroUsize,
    identities: NonZeroUsize,
}
impl SeamSpanHistoryLimits {
    /// Select both positive allowances explicitly; there is no inferred default.
    #[must_use]
    pub const fn new(entries: NonZeroUsize, identities: NonZeroUsize) -> Self {
        Self {
            entries,
            identities,
        }
    }
    /// Maximum supplied records, including unrelated births and splits.
    #[must_use]
    pub const fn entries(self) -> NonZeroUsize {
        self.entries
    }
    /// Maximum identities ever introduced, not merely the remaining live leaves.
    #[must_use]
    pub const fn identities(self) -> NonZeroUsize {
        self.identities
    }
}

/// Original input-limit or chronological identity refusal; no partial accepted history escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeamSpanHistoryError {
    /// Complete entry population refuses before examining semantic content or allocating a map.
    Entries {
        /// Actual complete supplied population.
        required: usize,
        /// Explicit selected allowance.
        limit: usize,
    },
    /// Counting the introduced identities cannot be represented.
    IdentityCountOverflow {
        /// Actual record whose count could not be added.
        entry: usize,
    },
    /// Complete issued population refuses before allocating or validating the identity map.
    Identities {
        /// Actual total births, counting both children and every retired identity.
        required: usize,
        /// Explicit selected allowance.
        limit: usize,
    },
    /// A split names an identity not introduced by an earlier record.
    MissingParent {
        /// Actual split record.
        entry: usize,
        /// Original requested parent.
        parent: EntityId,
    },
    /// A split attempts to retire the same parent again.
    RetiredParent {
        /// Actual refused split record.
        entry: usize,
        /// Original requested parent.
        parent: EntityId,
        /// Original prior split, without selecting one of its children.
        split_entry: usize,
    },
    /// A root or child attempts to reuse any earlier birth, including a retired identity.
    ReusedIdentity {
        /// Actual refused birth record.
        entry: usize,
        /// Original repeated identity.
        identity: EntityId,
        /// Actual original birth; equal siblings name this same split record.
        first_birth: usize,
    },
}
impl fmt::Display for SeamSpanHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "seam span history: {self:?}")
    }
}
impl std::error::Error for SeamSpanHistoryError {}

/// State only within the supplied validated history; this does not certify current graph sources.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeamSpanHistoryState<'a> {
    /// Introduced and not subsequently retired in this history.
    Live {
        /// Actual root or split record that introduced the identity.
        birth_entry: usize,
    },
    /// Explicit old target requiring a consumer decision; never an automatic child reassignment.
    RetiredSplit {
        /// Actual root or split record that introduced the identity.
        birth_entry: usize,
        /// Actual record that retired the parent.
        split_entry: usize,
        /// Original immediate children, borrowed from that exact record in A order.
        children: &'a [EntityId; 2],
    },
}

/// Immutable ordered identity metadata with complete input admission before validation allocation.
/// This accepts supplied records, not physical cuts, fresh global IDs or executed commands.
/// Graph/Design ownership, retained-source aliases and actual split-candidate binding must follow.
/// Private fields prevent fabricating accepted history:
/// ```compile_fail
/// fn forge(limits: sc_core::ontology::SeamSpanHistoryLimits)
///     -> sc_core::ontology::SeamSpanHistory {
///     sc_core::ontology::SeamSpanHistory { entries: Vec::new(), identities: 0, limits }
/// }
/// ```
/// Borrowed retirement evidence cannot escape its actual history:
/// ```compile_fail
/// fn escape(history: &sc_core::ontology::SeamSpanHistory,
///     id: sc_core::ontology::EntityId) -> sc_core::ontology::SeamSpanHistoryState<'static> {
///     history.state(id).unwrap()
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct SeamSpanHistory {
    entries: Vec<SeamSpanHistoryEntry>,
    identities: usize,
    limits: SeamSpanHistoryLimits,
}
impl fmt::Debug for SeamSpanHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            entries,
            identities,
            limits,
        } = self;
        f.debug_struct("SeamSpanHistory")
            .field("entries", &entries.len())
            .field("identities", identities)
            .field("limits", limits)
            .finish()
    }
}
impl SeamSpanHistory {
    /// Admit complete populations, then validate records in original authored order.
    /// Input-entry quota precedes full issued-ID quota; parent existence/liveness precedes child
    /// nonreuse. The temporary map retains one fixed record per introduced identity and is dropped
    /// on success or refusal. The original supplied vector is retained without cloning or sorting.
    ///
    /// ```
    /// use core::num::NonZeroUsize;
    /// use sc_core::ontology::{EntityId, SeamSpanHistory, SeamSpanHistoryEntry as E,
    ///     SeamSpanHistoryLimits, SeamSpanHistoryState as S};
    /// let ids = [EntityId::from_bits(8), EntityId::from_bits(3), EntityId::from_bits(5)];
    /// let [parent, first, second] = ids;
    /// let limits = SeamSpanHistoryLimits::new(NonZeroUsize::new(2).unwrap(),
    ///     NonZeroUsize::new(3).unwrap());
    /// let history = SeamSpanHistory::new(vec![E::Root(parent),
    ///     E::Split { parent, children: [first, second] }], limits).unwrap();
    /// assert!(matches!(history.state(parent), Some(S::RetiredSplit { children, .. })
    ///     if *children == [first, second]));
    /// assert_eq!(history.state(first), Some(S::Live { birth_entry: 1 }));
    /// ```
    /// # Errors
    /// Returns complete quota/count refusal, then the first actual parent or birth refusal.
    pub fn new(
        entries: Vec<SeamSpanHistoryEntry>,
        limits: SeamSpanHistoryLimits,
    ) -> Result<Self, SeamSpanHistoryError> {
        use SeamSpanHistoryError as Error;
        if entries.len() > limits.entries.get() {
            return Err(Error::Entries {
                required: entries.len(),
                limit: limits.entries.get(),
            });
        }
        let mut identities = 0usize;
        for (entry, record) in entries.iter().enumerate() {
            let count = match record {
                SeamSpanHistoryEntry::Root(_) => 1,
                SeamSpanHistoryEntry::Split { .. } => 2,
            };
            identities = identities
                .checked_add(count)
                .ok_or(Error::IdentityCountOverflow { entry })?;
        }
        if identities > limits.identities.get() {
            return Err(Error::Identities {
                required: identities,
                limit: limits.identities.get(),
            });
        }
        let mut issued: BTreeMap<EntityId, (usize, Option<usize>)> = BTreeMap::new();
        for (entry, record) in entries.iter().enumerate() {
            match *record {
                SeamSpanHistoryEntry::Root(identity) => {
                    if let Some((first_birth, _)) = issued.get(&identity) {
                        return Err(Error::ReusedIdentity {
                            entry,
                            identity,
                            first_birth: *first_birth,
                        });
                    }
                    issued.insert(identity, (entry, None));
                }
                SeamSpanHistoryEntry::Split { parent, children } => {
                    let (birth, retired) = issued
                        .get(&parent)
                        .copied()
                        .ok_or(Error::MissingParent { entry, parent })?;
                    if let Some(split_entry) = retired {
                        return Err(Error::RetiredParent {
                            entry,
                            parent,
                            split_entry,
                        });
                    }
                    for identity in children {
                        if let Some((first_birth, _)) = issued.get(&identity) {
                            return Err(Error::ReusedIdentity {
                                entry,
                                identity,
                                first_birth: *first_birth,
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
                    issued.insert(parent, (birth, Some(entry)));
                    for identity in children {
                        issued.insert(identity, (entry, None));
                    }
                }
            }
        }
        Ok(Self {
            entries,
            identities,
            limits,
        })
    }
    /// Exact original supplied records, without reordering, rewriting or generated identities.
    #[must_use]
    pub fn entries(&self) -> &[SeamSpanHistoryEntry] {
        &self.entries
    }
    /// Actual complete issued population, including retired ancestors and unrelated roots.
    #[must_use]
    pub const fn identity_count(&self) -> usize {
        self.identities
    }
    /// Exact caller-selected admission limits.
    #[must_use]
    pub const fn limits(&self) -> SeamSpanHistoryLimits {
        self.limits
    }
    /// Allocation-free linear lookup retaining the immediate retirement record and both children.
    /// Unknown identities return none. A later split of a child never retargets an old parent or
    /// upgrades the result into current graph membership, physical correspondence or authority.
    #[must_use]
    pub fn state(&self, identity: EntityId) -> Option<SeamSpanHistoryState<'_>> {
        let mut birth = None;
        for (entry, record) in self.entries.iter().enumerate() {
            match record {
                SeamSpanHistoryEntry::Root(id) => {
                    if *id == identity {
                        birth = Some(entry);
                    }
                }
                SeamSpanHistoryEntry::Split { parent, children } => {
                    if *parent == identity {
                        return Some(SeamSpanHistoryState::RetiredSplit {
                            birth_entry: birth?,
                            split_entry: entry,
                            children,
                        });
                    }
                    if children.contains(&identity) {
                        birth = Some(entry);
                    }
                }
            }
        }
        birth.map(|birth_entry| SeamSpanHistoryState::Live { birth_entry })
    }
}
