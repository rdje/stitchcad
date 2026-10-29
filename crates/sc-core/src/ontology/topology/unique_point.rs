//! Explicit bounded current-point uniqueness without constructing the full choice tree.

use core::num::NonZeroUsize;

use super::{
    point_step::{self, PointStep},
    Direction, EdgeRef, EntityId, IdentityLedger, OffsetFragment, OrphaningEdit, Param,
    ResolvedRef, TopologyEdit,
};

/// Explicit positive input limits for one current-point uniqueness query;no default is supplied.
/// Journal entries and nested declared edges/offset fragments are admitted before source lookup or
/// point mapping. Output is fixed storage;the query never constructs a reference-choice tree.
/// These counts do not certify geometry,whole Design readiness,allocator overhead or CPU time.
///
/// ```compile_fail
/// use sc_core::ontology::UniquePointLimits;
/// let limits: UniquePointLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniquePointLimits {
    journal_entries: NonZeroUsize,
    nested_items: NonZeroUsize,
}
impl UniquePointLimits {
    // Complete input admission shared with interval coverage;no source lookup or mapping here.
    pub(crate) fn admit_journal(
        self,
        ledger: &IdentityLedger,
    ) -> Result<(), PointJournalAdmissionIssue> {
        let journal = ledger.journal();
        if journal.len() > self.journal_entries.get() {
            return Err(PointJournalAdmissionIssue::JournalEntries {
                required: journal.len(),
                limit: self.journal_entries.get(),
            });
        }
        let mut consumed = 0usize;
        for entry in journal {
            if let Some((collection, requested)) = nested_population(entry.edit()) {
                let Some(total) = consumed.checked_add(requested) else {
                    return Err(PointJournalAdmissionIssue::CountOverflow {
                        operation: entry.operation(),
                        collection,
                        consumed,
                        requested,
                    });
                };
                if total > self.nested_items.get() {
                    return Err(PointJournalAdmissionIssue::NestedItems {
                        operation: entry.operation(),
                        collection,
                        consumed,
                        requested,
                        limit: self.nested_items.get(),
                    });
                }
                consumed = total;
            }
        }
        Ok(())
    }
    /// Choose both limits explicitly,including a positive nested budget for an empty journal.
    #[must_use]
    pub const fn new(journal_entries: NonZeroUsize, nested_items: NonZeroUsize) -> Self {
        Self {
            journal_entries,
            nested_items,
        }
    }
    /// Maximum complete journal entry population,including unrelated edits.
    #[must_use]
    pub const fn journal_entries(self) -> NonZeroUsize {
        self.journal_entries
    }
    /// Maximum aggregate declared-edge and offset-fragment population across the complete journal.
    #[must_use]
    pub const fn nested_items(self) -> NonZeroUsize {
        self.nested_items
    }
}

/// Variable-length journal population charged by a uniqueness query.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointJournalCollection {
    /// Edges recorded by a declaration.
    DeclaredEdges,
    /// Source intervals and edge identities recorded by an offset.
    OffsetFragments,
}

/// Complete journal admission refusal,before source lookup or point/interval mapping.
/// This fixed evidence excludes point choices and repair candidates,which are mapping outcomes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointJournalAdmissionIssue {
    /// Complete journal population exceeds the caller's limit,before any nested processing.
    JournalEntries {
        /// Complete population.
        required: usize,
        /// Selected limit.
        limit: usize,
    },
    /// Actual variable-length collection would exceed the aggregate nested limit.
    NestedItems {
        /// Owning journal operation.
        operation: EntityId,
        /// Actual collection.
        collection: PointJournalCollection,
        /// Previously admitted nested population.
        consumed: usize,
        /// Whole collection requested next.
        requested: usize,
        /// Selected aggregate limit.
        limit: usize,
    },
    /// Aggregate nested count cannot be represented;no source lookup or mapping has occurred.
    CountOverflow {
        /// Owning journal operation.
        operation: EntityId,
        /// Actual collection.
        collection: PointJournalCollection,
        /// Previously admitted nested population.
        consumed: usize,
        /// Whole collection requested next.
        requested: usize,
    },
}
impl From<PointJournalAdmissionIssue> for UniquePointOutcome {
    fn from(issue: PointJournalAdmissionIssue) -> Self {
        match issue {
            PointJournalAdmissionIssue::JournalEntries { required, limit } => {
                Self::JournalEntries { required, limit }
            }
            PointJournalAdmissionIssue::NestedItems {
                operation,
                collection,
                consumed,
                requested,
                limit,
            } => Self::NestedItems {
                operation,
                collection,
                consumed,
                requested,
                limit,
            },
            PointJournalAdmissionIssue::CountOverflow {
                operation,
                collection,
                consumed,
                requested,
            } => Self::CountOverflow {
                operation,
                collection,
                consumed,
                requested,
            },
        }
    }
}

/// Immediate outcome of an admitted current-point query,with fixed evidence storage.
/// A split choice stops at its first occurrence;its branch identities have not been folded through
/// later edits. Coincident later locations cannot cancel that required reference choice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UniquePointOutcome {
    /// Known source reaches a unique current position with no required choice or repair.
    Resolved(ResolvedRef),
    /// Held edge was never created by this journal,unlike legacy point-fold pass-through.
    UnknownSource,
    /// Complete journal population exceeds the caller's limit,before any nested processing.
    JournalEntries {
        /// Complete population.
        required: usize,
        /// Selected limit.
        limit: usize,
    },
    /// Actual variable-length collection would exceed the aggregate nested limit.
    NestedItems {
        /// Owning journal operation.
        operation: EntityId,
        /// Actual collection.
        collection: PointJournalCollection,
        /// Previously admitted nested population.
        consumed: usize,
        /// Whole collection requested next.
        requested: usize,
        /// Selected aggregate limit.
        limit: usize,
    },
    /// Aggregate nested count cannot be represented;no source lookup or mapping has occurred.
    CountOverflow {
        /// Owning journal operation.
        operation: EntityId,
        /// Actual collection.
        collection: PointJournalCollection,
        /// Previously admitted nested population.
        consumed: usize,
        /// Whole collection requested next.
        requested: usize,
    },
    /// Required first split choice,without traversing either branch or inventing final candidates.
    SplitChoice {
        /// Actual split operation.
        operation: EntityId,
        /// Edge reached immediately before this split.
        source: EdgeRef,
        /// Exact split parameter in that edge's frame.
        at: Param,
        /// First branch identity,starting at its end parameter.
        first: EdgeRef,
        /// Second branch identity,starting at its start parameter.
        second: EdgeRef,
        /// Both branches' direction relative to the held reference frame.
        direction: Direction,
    },
    /// Concrete first repair cause. Offset candidates are immediate,as in the legacy point fold.
    Repair {
        /// Actual operation/source/numeric cause.
        cause: OrphaningEdit,
        /// Zero or two candidates in authored interval order;no candidate Vec is allocated.
        candidates: [Option<ResolvedRef>; 2],
    },
}

/// Held original reference and one bounded uniqueness outcome. No registration or choice is changed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniquePointResolution {
    reference: EdgeRef,
    param: Param,
    outcome: UniquePointOutcome,
}
impl UniquePointResolution {
    pub(crate) fn numeric_cause(&self) -> Option<&sc_units::UnitError> {
        match self.outcome() {
            UniquePointOutcome::Repair {
                cause,
                candidates: _,
            } => cause.numeric_cause(),
            UniquePointOutcome::Resolved(_)
            | UniquePointOutcome::UnknownSource
            | UniquePointOutcome::JournalEntries {
                required: _,
                limit: _,
            }
            | UniquePointOutcome::NestedItems {
                operation: _,
                collection: _,
                consumed: _,
                requested: _,
                limit: _,
            }
            | UniquePointOutcome::CountOverflow {
                operation: _,
                collection: _,
                consumed: _,
                requested: _,
            }
            | UniquePointOutcome::SplitChoice {
                operation: _,
                source: _,
                at: _,
                first: _,
                second: _,
                direction: _,
            } => None,
        }
    }
    /// Original held edge,including on all admission and mapping refusals.
    #[must_use]
    pub const fn reference(self) -> EdgeRef {
        self.reference
    }
    /// Original held parameter,including on all admission and mapping refusals.
    #[must_use]
    pub const fn param(self) -> Param {
        self.param
    }
    /// Immediate typed outcome;split branch identities are not final current resolutions.
    #[must_use]
    pub const fn outcome(&self) -> &UniquePointOutcome {
        &self.outcome
    }
    /// Unique known-source current position,only when no choice/repair/admission refusal occurred.
    #[must_use]
    pub const fn resolved(&self) -> Option<ResolvedRef> {
        match self.outcome {
            UniquePointOutcome::Resolved(point) => Some(point),
            _ => None,
        }
    }
}

impl IdentityLedger {
    /// Query unique current position after admitting the complete actual journal and nested data.
    /// Unlike [`Self::resolve`],this query rejects unknown source edges and stops at the first
    /// required split choice. It neither builds/follows that choice tree nor applies a registration's
    /// stated choice. Actual repair causes and immediate offset candidates retain their semantics.
    /// Current uniqueness alone does not certify a containing range,copy ownership or physical truth.
    /// The answer records this journal observation;query again after later edits.
    ///
    /// ```
    /// use core::num::NonZeroUsize;
    /// use sc_core::ontology::{DeterministicIdGenerator, IdentityLedger, Param, UniquePointLimits};
    /// let mut ledger = IdentityLedger::new();
    /// let mut ids = DeterministicIdGenerator::new();
    /// let edge = ledger.declare_edges(&mut ids, 1)?.remove(0);
    /// let one = NonZeroUsize::new(1).ok_or("positive limit")?;
    /// let answer = ledger.resolve_unique_point(edge, Param::START, UniquePointLimits::new(one, one));
    /// assert_eq!(answer.resolved().ok_or("known unique source")?.edge(), edge);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn resolve_unique_point(
        &self,
        reference: EdgeRef,
        param: Param,
        limits: UniquePointLimits,
    ) -> UniquePointResolution {
        let finish = |outcome| UniquePointResolution {
            reference,
            param,
            outcome,
        };
        if let Err(outcome) = limits.admit_journal(self) {
            return finish(outcome.into());
        }
        let journal = self.journal();
        if !journal.iter().any(|entry| creates(entry.edit(), reference)) {
            return finish(UniquePointOutcome::UnknownSource);
        }
        let mut current = ResolvedRef::new(reference, param, Direction::Original);
        for entry in journal {
            match point_step::step(current, entry) {
                PointStep::Current(next) => current = next,
                PointStep::Split { first, second } => {
                    return finish(UniquePointOutcome::SplitChoice {
                        operation: entry.operation(),
                        source: current.edge(),
                        at: current.param(),
                        first: first.edge(),
                        second: second.edge(),
                        direction: current.direction(),
                    });
                }
                PointStep::Repair { cause, candidates } => {
                    return finish(UniquePointOutcome::Repair { cause, candidates });
                }
            }
        }
        finish(UniquePointOutcome::Resolved(current))
    }
}

// Exhaustive typed edit views:future fields/variants require an explicit admission/source review.
fn nested_population(edit: &TopologyEdit) -> Option<(PointJournalCollection, usize)> {
    match edit {
        TopologyEdit::Declared { edges } => {
            Some((PointJournalCollection::DeclaredEdges, edges.len()))
        }
        TopologyEdit::Offset {
            source: _,
            fragments,
        } => Some((PointJournalCollection::OffsetFragments, fragments.len())),
        TopologyEdit::Split {
            source: _,
            at: _,
            first: _,
            second: _,
        }
        | TopologyEdit::Merge {
            first: _,
            first_length: _,
            second: _,
            second_length: _,
            merged: _,
        }
        | TopologyEdit::Reverse { edge: _ }
        | TopologyEdit::Delete { edge: _ } => None,
    }
}
pub(super) fn creates(edit: &TopologyEdit, reference: EdgeRef) -> bool {
    match edit {
        TopologyEdit::Declared { edges } => edges.contains(&reference),
        TopologyEdit::Split {
            source: _,
            at: _,
            first,
            second,
        } => *first == reference || *second == reference,
        TopologyEdit::Merge {
            first: _,
            first_length: _,
            second: _,
            second_length: _,
            merged,
        } => *merged == reference,
        TopologyEdit::Offset {
            source: _,
            fragments,
        } => fragments.iter().any(|fragment| {
            let OffsetFragment {
                edge,
                from: _,
                to: _,
            } = *fragment;
            edge == reference
        }),
        TopologyEdit::Reverse { edge: _ } | TopologyEdit::Delete { edge: _ } => false,
    }
}
