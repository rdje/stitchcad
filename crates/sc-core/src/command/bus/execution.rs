//! Atomic in-memory metadata publication; replay/audit are private to this owner's lifetime.
use super::{
    difference, CommandBus, CommandGroupPreview, CommandGroupPreviewError, CommandPreviewError,
};
use crate::{
    command::{
        CommandActorClaim, CommandAuthoritySource, CommandAuthorizationMode, CommandGroupId,
        CommandId, CommandIdempotencyKey, CommandMutationGroup, DesignRevision,
    },
    design::{DesignGarment, DesignInputs, DesignReferenceHost, DesignSewing, DesignStructure},
    ontology::EntityId,
};
use core::{fmt, num::NonZeroUsize};
use std::{collections::TryReserveError, time::SystemTime};

/// Actual accepted metadata changes at one exact successor revision; no geometry/release verdict.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroupCommit {
    changes: CommandGroupPreview,
    revision: DesignRevision,
}
impl CommandGroupCommit {
    /// Original base-revision step/net report from actual validated preparation.
    #[must_use]
    pub const fn changes(&self) -> &CommandGroupPreview {
        &self.changes
    }
    /// Actual represented revision after this complete group's publication.
    #[must_use]
    pub const fn revision(&self) -> &DesignRevision {
        &self.revision
    }
}

/// Immutable append-only record for one accepted authoring group, never an undo-history record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandAuditEntry {
    request: CommandMutationGroup,
    commit: CommandGroupCommit,
    grant: EntityId,
    checked_at: SystemTime,
}
impl CommandAuditEntry {
    /// Exact original contexts, arguments, keys and order, including within-group aliases.
    #[must_use]
    pub const fn request(&self) -> &CommandMutationGroup {
        &self.request
    }
    /// Original initiating actor/principal, freshly authenticated at the actual effect boundary.
    #[must_use]
    pub const fn actor(&self) -> CommandActorClaim {
        self.request.context().actor()
    }
    /// Actual complete publication result and diagnostics for this authoring act.
    #[must_use]
    pub const fn commit(&self) -> &CommandGroupCommit {
        &self.commit
    }
    /// Original complete Commit grant selected immediately before publication.
    #[must_use]
    pub const fn grant(&self) -> EntityId {
        self.grant
    }
    /// Actual final trusted authorization observation, never a caller timestamp.
    #[must_use]
    pub const fn checked_at(&self) -> SystemTime {
        self.checked_at
    }
}

/// Authorized publication or explicit exact replay of the first immutable accepted result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroupCommitResult {
    entry: CommandAuditEntry,
    replayed: bool,
}
impl CommandGroupCommitResult {
    /// Original accepted record; an exact replay neither rewrites it nor adds another entry.
    #[must_use]
    pub const fn entry(&self) -> &CommandAuditEntry {
        &self.entry
    }
    /// True only when the exact original group was already committed by this bus owner.
    #[must_use]
    pub const fn is_replay(&self) -> bool {
        self.replayed
    }
}

pub(super) struct CommandJournal {
    limit: NonZeroUsize,
    initial_revision: DesignRevision,
    entries: Vec<CommandAuditEntry>,
}

/// Actual authoring group that advanced this owner's represented revision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandRevisionChange {
    /// Original accepted containing group.
    pub group: CommandGroupId,
    /// Actual resulting revision from that group.
    pub revision: DesignRevision,
    /// First-occurrence commands in submitted order; retry aliases did not edit again.
    pub commands: Vec<CommandId>,
}

/// Atomic refusal before model/reserve/journal publication; original preparation causes survive.
#[derive(Debug)]
pub enum CommandGroupCommitError<E> {
    /// Original authority, binding, member or final semantic-integrity cause.
    Preparation(Box<CommandGroupPreviewError<E>>),
    /// Exact revision mismatch with actual intervening authoring groups available in this owner.
    Precondition {
        /// Original group/revision mismatch retaining exact requested/current counters.
        issue: Box<CommandGroupPreviewError<E>>,
        /// Earliest revision for which this in-memory journal records subsequent changes.
        history_from: DesignRevision,
        /// Actual accepted advances after the requested revision, in original commit order.
        changes: Vec<CommandRevisionChange>,
    },
    /// This owner lacks an explicit finite audit capacity.
    EditsNotConfigured {
        /// Original requested group.
        group: CommandGroupId,
    },
    /// Original group id already names a different accepted request.
    GroupConflict {
        /// Reused original group identity.
        group: CommandGroupId,
    },
    /// An actor's key already belongs to another accepted group; partial-group replay is refused.
    KeyConflict {
        /// Requested containing group.
        group: CommandGroupId,
        /// Original conflicting member position.
        index: usize,
        /// Original submitted key.
        key: CommandIdempotencyKey,
        /// First accepted containing group.
        previous: CommandGroupId,
    },
    /// No new authoring record fits; exact authorized replay still works at this bound.
    JournalFull {
        /// Requested group.
        group: CommandGroupId,
        /// Explicit host-configured maximum accepted groups.
        limit: usize,
    },
    /// Actual journal allocation refusal before publication.
    JournalAllocation {
        /// Requested group.
        group: CommandGroupId,
        /// Original allocation cause.
        issue: TryReserveError,
    },
    /// Concrete prepared cursor consumption exceeds the original private reserve.
    ReserveInvariant {
        /// Requested group.
        group: CommandGroupId,
        /// Actual prepared consumption count.
        consumed: usize,
    },
    /// Original actual successor-revision model validation/integrity cause.
    Revision {
        /// Requested group.
        group: CommandGroupId,
        /// Complete original immutable-model cause.
        issue: Box<CommandPreviewError<E>>,
    },
}
impl<E: fmt::Debug> fmt::Display for CommandGroupCommitError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "metadata group commit refused: {self:?}")
    }
}
impl<E: std::error::Error + 'static> std::error::Error for CommandGroupCommitError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Preparation(issue) => Some(issue.as_ref()),
            Self::Precondition { issue, .. } => Some(issue.as_ref()),
            Self::Revision { issue, .. } => Some(issue.as_ref()),
            Self::JournalAllocation { issue, .. } => Some(issue),
            _ => None,
        }
    }
}
impl<E> From<CommandGroupPreviewError<E>> for CommandGroupCommitError<E> {
    fn from(issue: CommandGroupPreviewError<E>) -> Self {
        Self::Preparation(Box::new(issue))
    }
}

impl<H: DesignReferenceHost, A: CommandAuthoritySource> CommandBus<H, A> {
    /// Install an edit-capable in-memory owner with an explicit finite accepted-group count bound.
    /// The journal starts empty and never discards accepted audit/replay entries. A full journal
    /// refuses new groups. Persistence, byte/work limits and undo remain separate owners.
    /// Construction confers no authority; each effect requires fresh complete Commit permission.
    /// # Errors
    /// Returns the original identity-reserve configuration cause before installing the owner.
    pub fn with_edit_capacity(
        state: DesignGarment<H>,
        authority: A,
        identities: Vec<EntityId>,
        maximum_groups: NonZeroUsize,
    ) -> Result<Self, super::CommandIdentityError> {
        let mut bus = Self::with_identities(state, authority, identities)?;
        bus.journal = Some(CommandJournal {
            limit: maximum_groups,
            initial_revision: bus.state.inputs().revision().clone(),
            entries: Vec::new(),
        });
        Ok(bus)
    }
}
impl<H: DesignReferenceHost + Clone + PartialEq, A: CommandAuthoritySource> CommandBus<H, A> {
    /// Whether this owner has the implemented metadata commit handler and finite edit capacity.
    /// This does not certify current authority, revision, journal room or candidate validity.
    #[must_use]
    pub fn is_commit_available(&self, command: crate::command::CommandId) -> bool {
        self.journal.is_some() && super::preparation::handler_available(command)
    }

    /// Commit the complete supported metadata group atomically at the exact successor revision.
    /// Fresh whole-group authority precedes Design binding and replay lookup. Exact original replay
    /// returns the first result even after the current revision advances; another request under the
    /// same group id or overlapping actor/key refuses. No partial-group replay or edit is accepted.
    /// A new group stages actual candidates, validates the successor-revision model and preflights
    /// reserve/journal allocation, then refreshes complete Commit authority at the effect boundary.
    /// No fallible operation or host callback lies between model/reserve/journal publication steps.
    /// This in-memory lifetime supplies no storage, physical/geometry, undo or transport certificate.
    /// # Errors
    /// Returns original authority/Design, missing edit configuration, replay conflict, exact revision,
    /// capacity, candidate/revision/allocation or final fresh authority refusal before publication.
    pub fn commit_group(
        &mut self,
        group: &CommandMutationGroup,
    ) -> Result<CommandGroupCommitResult, CommandGroupCommitError<H::Error>> {
        self.authorize_group(group, CommandAuthorizationMode::Execute)?;
        let journal = self
            .journal
            .as_ref()
            .ok_or(CommandGroupCommitError::EditsNotConfigured { group: group.id() })?;
        if let Some(first) = journal
            .entries
            .iter()
            .find(|entry| entry.request.id() == group.id())
        {
            if first.request != *group {
                return Err(CommandGroupCommitError::GroupConflict { group: group.id() });
            }
            return Ok(CommandGroupCommitResult {
                entry: first.clone(),
                replayed: true,
            });
        }
        for (index, mutation) in group.mutations().iter().enumerate() {
            let key = mutation.context().idempotency_key();
            if let Some(first) = journal.entries.iter().find(|entry| {
                entry.actor() == group.context().actor()
                    && entry
                        .request
                        .mutations()
                        .iter()
                        .any(|prior| prior.context().idempotency_key() == key)
            }) {
                return Err(CommandGroupCommitError::KeyConflict {
                    group: group.id(),
                    index,
                    key,
                    previous: first.request.id(),
                });
            }
        }
        self.require_group_revision(group).map_err(|issue| {
            let requested = group.context().expected_revision();
            let changes = journal
                .entries
                .iter()
                .filter(|entry| entry.commit.revision() > requested)
                .map(|entry| CommandRevisionChange {
                    group: entry.request.id(),
                    revision: entry.commit.revision().clone(),
                    commands: entry
                        .commit
                        .changes()
                        .steps()
                        .iter()
                        .filter(|step| step.replay_of().is_none())
                        .map(|step| step.command())
                        .collect(),
                })
                .collect();
            CommandGroupCommitError::Precondition {
                issue: Box::new(issue),
                history_from: journal.initial_revision.clone(),
                changes,
            }
        })?;
        if journal.entries.len() >= journal.limit.get() {
            return Err(CommandGroupCommitError::JournalFull {
                group: group.id(),
                limit: journal.limit.get(),
            });
        }
        let prepared = self.stage_group(group)?;
        let (candidate, changes, consumed) = prepared.into_parts();
        let revision = self.state.inputs().revision().successor();
        let state = at_revision(&candidate, revision.clone()).map_err(|issue| {
            CommandGroupCommitError::Revision {
                group: group.id(),
                issue: Box::new(issue),
            }
        })?;
        let identities = self.identities.after_consuming(consumed).ok_or(
            CommandGroupCommitError::ReserveInvariant {
                group: group.id(),
                consumed,
            },
        )?;
        self.journal
            .as_mut()
            .ok_or(CommandGroupCommitError::EditsNotConfigured { group: group.id() })?
            .entries
            .try_reserve(1)
            .map_err(|issue| CommandGroupCommitError::JournalAllocation {
                group: group.id(),
                issue,
            })?;
        let checked = self.authorize_group(group, CommandAuthorizationMode::Execute)?;
        let entry = CommandAuditEntry {
            request: group.clone(),
            commit: CommandGroupCommit { changes, revision },
            grant: checked.grant(),
            checked_at: checked.checked_at(),
        };
        let result = CommandGroupCommitResult {
            entry: entry.clone(),
            replayed: false,
        };
        let journal = self
            .journal
            .as_mut()
            .ok_or(CommandGroupCommitError::EditsNotConfigured { group: group.id() })?;
        // The sole &mut owner prevents observations between these infallible moves. Retain the old
        // host until all fields/record are published: even a host Drop cannot interrupt their update.
        let previous = core::mem::replace(&mut self.state, state);
        self.identities = identities;
        journal.entries.push(entry);
        drop(previous);
        Ok(result)
    }
}

fn at_revision<H: DesignReferenceHost + Clone + PartialEq>(
    before: &DesignGarment<H>,
    revision: DesignRevision,
) -> Result<DesignGarment<H>, CommandPreviewError<H::Error>> {
    let mut garment = before.definition().clone();
    let mut sewing = garment.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    structure.revision = revision.clone();
    inputs.structure =
        DesignStructure::new(structure).map_err(|e| CommandPreviewError::Structure(Box::new(e)))?;
    sewing.inputs =
        DesignInputs::new(inputs).map_err(|e| CommandPreviewError::Inputs(Box::new(e)))?;
    garment.sewing =
        DesignSewing::new(sewing).map_err(|e| CommandPreviewError::Sewing(Box::new(e)))?;
    let after =
        DesignGarment::new(garment).map_err(|e| CommandPreviewError::Candidate(Box::new(e)))?;
    after
        .validate_current()
        .map_err(|e| CommandPreviewError::Candidate(Box::new(e)))?;
    difference::unchanged_noncanonical_context_at_revision(before, &after, &revision)
        .map_err(CommandPreviewError::Difference)?;
    if before.inputs().inputs() != after.inputs().inputs() {
        return Err(CommandPreviewError::Difference(
            super::CommandDifferenceError::CanonicalInputs,
        ));
    }
    if before.inputs().structure().pieces() != after.inputs().structure().pieces() {
        return Err(CommandPreviewError::Difference(
            super::CommandDifferenceError::Pieces,
        ));
    }
    if before.sewing().landmarks() != after.sewing().landmarks() {
        return Err(CommandPreviewError::Difference(
            super::CommandDifferenceError::Landmarks,
        ));
    }
    Ok(after)
}
