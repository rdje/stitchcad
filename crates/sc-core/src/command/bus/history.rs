//! Bounded actual model alternatives and independently append-only history-action records.
use super::{CommandAuditSequence, CommandRevisionChange};
use super::{CommandBus, CommandGroupPreviewError, CommandPreviewError};
use crate::command::CommandId;
use crate::command::{CommandProgressObserver, CommandProgressPhase as Phase};
use crate::{
    command::{
        CommandActorClaim, CommandAuthoritySource, CommandAuthorizationMode, CommandGroupId,
        CommandHistoryDirection, CommandHistoryRequest, CommandIdempotencyKey, CommandMutation,
        CommandMutationContext, CommandMutationGroup, CommandMutationGroupError, DesignRevision,
    },
    design::{DesignGarment, DesignReferenceHost},
    ontology::EntityId,
};
use core::{fmt, num::NonZeroUsize};
use std::{
    collections::{TryReserveError, VecDeque},
    time::SystemTime,
};

/// Explicit loss of reversible history on a new authoring branch; audit/replay records remain intact.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandHistoryChange {
    dropped_undo: Option<CommandGroupId>,
    discarded_redo: Vec<CommandGroupId>,
}
impl CommandHistoryChange {
    pub(super) fn resource_discarded(&self) -> &[CommandGroupId] {
        let Self {
            dropped_undo,
            discarded_redo,
        } = self;
        crate::command::resource::fixed(dropped_undo);
        discarded_redo
    }
    /// Oldest undo group evicted at the configured depth, if any.
    #[must_use]
    pub const fn dropped_undo(&self) -> Option<CommandGroupId> {
        self.dropped_undo
    }
    /// Former redo branch discarded in its original redo application order.
    #[must_use]
    pub fn discarded_redo(&self) -> &[CommandGroupId] {
        &self.discarded_redo
    }
}

/// One immutable accepted history action; original author and current operator remain distinct.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandHistoryAuditEntry {
    sequence: CommandAuditSequence,
    commands: Vec<CommandId>,
    request: CommandHistoryRequest,
    before: DesignRevision,
    revision: DesignRevision,
    grant: EntityId,
    checked_at: SystemTime,
}
impl CommandHistoryAuditEntry {
    pub(super) fn resource_parts(
        &self,
    ) -> (
        &CommandAuditSequence,
        &[CommandId],
        &CommandHistoryRequest,
        &DesignRevision,
        &DesignRevision,
    ) {
        let Self {
            sequence,
            commands,
            request,
            before,
            revision,
            grant,
            checked_at,
        } = self;
        crate::command::resource::fixed(grant);
        crate::command::resource::fixed(checked_at);
        (sequence, commands, request, before, revision)
    }
    /// Actual common authoring/history chronological effect identity.
    #[must_use]
    pub const fn sequence(&self) -> &CommandAuditSequence {
        &self.sequence
    }
    /// Effective original command identities, with authoring retry aliases omitted.
    #[must_use]
    pub fn commands(&self) -> &[CommandId] {
        &self.commands
    }
    /// Exact accepted operation context, direction and original authoring group.
    #[must_use]
    pub const fn request(&self) -> &CommandHistoryRequest {
        &self.request
    }
    /// Actual authenticated operator of this history action.
    #[must_use]
    pub const fn operator(&self) -> CommandActorClaim {
        self.request.context().actor()
    }
    /// Original accepted author, unchanged by another operator's undo or redo.
    #[must_use]
    pub const fn original_actor(&self) -> CommandActorClaim {
        self.request.original_actor()
    }
    /// Actual represented revision before this action.
    #[must_use]
    pub const fn before_revision(&self) -> &DesignRevision {
        &self.before
    }
    /// Exact restored original snapshot revision after this action.
    #[must_use]
    pub const fn revision(&self) -> &DesignRevision {
        &self.revision
    }
    /// Complete current-operator Commit grant selected at the final effect boundary.
    #[must_use]
    pub const fn grant(&self) -> EntityId {
        self.grant
    }
    /// Actual trusted final authority observation.
    #[must_use]
    pub const fn checked_at(&self) -> SystemTime {
        self.checked_at
    }
}

/// Accepted traversal or exact replay of its original immutable action/result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandHistoryResult {
    entry: CommandHistoryAuditEntry,
    replayed: bool,
}
impl CommandHistoryResult {
    pub(super) fn resource_entry(&self) -> &CommandHistoryAuditEntry {
        let Self { entry, replayed } = self;
        crate::command::resource::fixed(replayed);
        entry
    }
    /// Original accepted action record, without a model or replay/reset capability.
    #[must_use]
    pub const fn entry(&self) -> &CommandHistoryAuditEntry {
        &self.entry
    }
    /// True when this exact operation was previously accepted by this owner.
    #[must_use]
    pub const fn is_replay(&self) -> bool {
        self.replayed
    }
}

#[derive(Clone)]
struct PendingHistoryEntry {
    sequence: CommandAuditSequence,
    commands: Vec<CommandId>,
    request: CommandHistoryRequest,
    before: DesignRevision,
    revision: DesignRevision,
}
impl PendingHistoryEntry {
    fn finish(self, grant: EntityId, checked_at: SystemTime) -> CommandHistoryAuditEntry {
        CommandHistoryAuditEntry {
            sequence: self.sequence,
            commands: self.commands,
            request: self.request,
            before: self.before,
            revision: self.revision,
            grant,
            checked_at,
        }
    }
}

struct HistoryStep<H> {
    group: CommandGroupId,
    alternative: DesignGarment<H>,
}
pub(super) struct History<H> {
    depth: NonZeroUsize,
    action_limit: NonZeroUsize,
    undo: VecDeque<HistoryStep<H>>,
    redo: Vec<HistoryStep<H>>,
    actions: Vec<CommandHistoryAuditEntry>,
}
impl<H> History<H> {
    pub(super) fn charge_authoring_change_resources(
        &self,
        owner: EntityId,
        sink: &mut dyn crate::design::DesignResourceSink,
    ) -> Result<(), crate::design::DesignResourceLimitError> {
        let Self {
            depth: _,
            action_limit: _,
            undo: _,
            redo,
            actions: _,
        } = self;
        sink.charge_entries(
            redo.len(),
            super::retained::location(owner, "records.commit.discarded_redo"),
        )?;
        for step in redo.iter().rev() {
            let HistoryStep {
                group,
                alternative: _,
            } = step;
            crate::command::resource::fixed(group);
        }
        Ok(())
    }
    pub(super) fn changes_after<'a>(
        &'a self,
        sequence: Option<&'a CommandAuditSequence>,
    ) -> impl Iterator<Item = CommandRevisionChange> + 'a {
        self.actions
            .iter()
            .filter(move |entry| sequence.is_some_and(|sequence| &entry.sequence > sequence))
            .map(|entry| CommandRevisionChange {
                sequence: entry.sequence.clone(),
                direction: Some(entry.request.direction()),
                group: entry.request.original().id(),
                revision: entry.revision.clone(),
                commands: entry.commands.clone(),
            })
    }
    pub(super) fn action_with_key(
        &self,
        actor: CommandActorClaim,
        key: CommandIdempotencyKey,
    ) -> Option<&CommandHistoryAuditEntry> {
        self.actions.iter().find(|entry| {
            entry.operator() == actor && entry.request.context().idempotency_key() == key
        })
    }
    pub(super) fn prepare_commit(&mut self) -> Result<CommandHistoryChange, TryReserveError> {
        self.undo.try_reserve(1)?;
        Ok(CommandHistoryChange {
            dropped_undo: if self.undo.len() >= self.depth.get() {
                self.undo.front().map(|step| step.group)
            } else {
                None
            },
            discarded_redo: self.redo.iter().rev().map(|step| step.group).collect(),
        })
    }
    pub(super) fn record_commit(&mut self, previous: DesignGarment<H>, group: CommandGroupId) {
        let dropped = if self.undo.len() >= self.depth.get() {
            self.undo.pop_front()
        } else {
            None
        };
        let discarded = core::mem::take(&mut self.redo);
        self.undo.push_back(HistoryStep {
            group,
            alternative: previous,
        });
        // Model/reserve/authoring journal and both stacks are coherent before any old host drops.
        drop(dropped);
        drop(discarded);
    }
    fn top(&self, direction: CommandHistoryDirection) -> Option<&HistoryStep<H>> {
        match direction {
            CommandHistoryDirection::Undo => self.undo.back(),
            CommandHistoryDirection::Redo => self.redo.last(),
        }
    }
}

/// Original atomic history refusal; no model/stack/action record is partially published.
#[derive(Debug)]
pub enum CommandHistoryError<E> {
    /// Original trusted operation-policy/admission/cancellation refusal.
    Control(Box<super::CommandControlError>),
    /// Internal authorization-only group could not preserve the validated submitted metadata.
    RequestInvariant(CommandMutationGroupError),
    /// Original fresh current-operator authority, Design or exact revision refusal.
    Preparation(Box<CommandGroupPreviewError<E>>),
    /// Exact operation revision mismatch with actual available ordered state-change witnesses.
    Precondition {
        /// Original exact requested/current revision cause.
        issue: Box<CommandGroupPreviewError<E>>,
        /// Original revision at the start of this in-memory journal.
        history_from: DesignRevision,
        /// Actual authoring/undo/redo events after the requested snapshot's original creation.
        changes: Vec<CommandRevisionChange>,
    },
    /// This owner has no explicitly bounded history/action journal configured.
    NotConfigured,
    /// Submitted original group has no accepted authoring record in this owner.
    GroupNotAccepted {
        /// Original submitted group identity.
        group: CommandGroupId,
    },
    /// Submitted group differs from the complete original accepted request.
    GroupConflict {
        /// Original reused group identity.
        group: CommandGroupId,
    },
    /// Operation key belongs to another accepted authoring/history request for this operator.
    KeyConflict {
        /// Exact conflicting operation key.
        key: CommandIdempotencyKey,
    },
    /// No reversible group remains in the selected direction.
    Empty {
        /// Requested traversal direction.
        direction: CommandHistoryDirection,
    },
    /// The requested group is not the next reversible group in this direction.
    NotCurrent {
        /// Exact selected original group.
        group: CommandGroupId,
        /// Requested traversal direction.
        direction: CommandHistoryDirection,
    },
    /// No new immutable action fits; exact authorized replay still works at this bound.
    ActionLimit {
        /// Explicit configured maximum accepted history actions.
        limit: usize,
    },
    /// Original allocation preflight refusal before any state/stack/action transition.
    Allocation(TryReserveError),
    /// Original current-model envelope/current-source refusal.
    Current(Box<CommandPreviewError<E>>),
    /// Original alternative-model envelope/current-source refusal.
    Alternative(Box<CommandPreviewError<E>>),
}
impl<E: fmt::Debug> fmt::Display for CommandHistoryError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "history operation refused: {self:?}")
    }
}
impl<E: std::error::Error + 'static> std::error::Error for CommandHistoryError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Control(issue) => Some(issue.as_ref()),
            Self::RequestInvariant(issue) => Some(issue),
            Self::Preparation(issue) => Some(issue.as_ref()),
            Self::Precondition { issue, .. } => Some(issue.as_ref()),
            Self::Allocation(issue) => Some(issue),
            Self::Current(issue) | Self::Alternative(issue) => Some(issue.as_ref()),
            _ => None,
        }
    }
}
impl<E> From<CommandGroupPreviewError<E>> for CommandHistoryError<E> {
    fn from(issue: CommandGroupPreviewError<E>) -> Self {
        Self::Preparation(Box::new(issue))
    }
}
impl<E> From<super::CommandControlError> for CommandHistoryError<E> {
    fn from(issue: super::CommandControlError) -> Self {
        Self::Control(Box::new(issue))
    }
}

impl<H: DesignReferenceHost, A: CommandAuthoritySource> CommandBus<H, A> {
    /// Install explicit finite authoring count, reversible depth and immutable action count bounds.
    /// Undo history starts empty and is noncanonical; original authoring/action audit entries are
    /// never dropped to make room. New authoring reports oldest undo/redo-branch losses separately.
    /// Install operation limits separately before use. This supplies no authority or persistence guarantee.
    /// # Errors
    /// Returns the original identity-reserve configuration cause.
    pub fn with_edit_history(
        state: DesignGarment<H>,
        authority: A,
        identities: Vec<EntityId>,
        maximum_groups: NonZeroUsize,
        undo_depth: NonZeroUsize,
        maximum_actions: NonZeroUsize,
    ) -> Result<Self, super::CommandIdentityError> {
        let mut bus = Self::with_edit_capacity(state, authority, identities, maximum_groups)?;
        bus.history = Some(History {
            depth: undo_depth,
            action_limit: maximum_actions,
            undo: VecDeque::new(),
            redo: Vec::new(),
            actions: Vec::new(),
        });
        Ok(bus)
    }
}
impl<H: crate::design::DesignResourceSource + Clone + PartialEq, A: CommandAuthoritySource>
    CommandBus<H, A>
{
    /// Traverse the selected original whole-group snapshot under fresh current-operator authority.
    /// Exact operation retry returns its first result before current revision/stack/capacity checks.
    /// Other reuse refuses, including a key already accepted for this operator's authoring mutation.
    /// Validate actual current/alternative models and preflight stack/action allocation, then refresh
    /// complete Commit authority immediately before exclusive model/stack/action moves. Original
    /// authoring audit, issued identities and maximum-issued authoring revision never rewind.
    /// # Errors
    /// Pure original-group admission precedes authority. Returns policy/admission, original authority/Design,
    /// missing configuration, key/original-group conflict, current
    /// revision, selected stack/bound, current/alternative validation, allocation or final authority cause.
    pub fn apply_history(
        &mut self,
        request: &CommandHistoryRequest,
    ) -> Result<CommandHistoryResult, CommandHistoryError<H::Error>> {
        self.apply_history_with_progress(request, &())
    }

    /// Traverse actual snapshots with typed pre-effect progress and cooperative atomic cancellation.
    /// Installed immutable operation policy applies to the complete original submitted group.
    /// # Errors
    /// Preserves original history causes and typed policy/admission/cancellation refusals.
    pub fn apply_history_with_progress(
        &mut self,
        request: &CommandHistoryRequest,
        observer: &dyn CommandProgressObserver,
    ) -> Result<CommandHistoryResult, CommandHistoryError<H::Error>> {
        self.admit_group(request.original())?;
        let authority_group =
            history_authority_group(request).map_err(CommandHistoryError::RequestInvariant)?;
        self.authorize_group(&authority_group, CommandAuthorizationMode::Execute)?;
        super::operation::checkpoint(observer, Phase::Authorized, 1, 1, None)?;
        let history = self
            .history
            .as_ref()
            .ok_or(CommandHistoryError::NotConfigured)?;
        let context = request.context();
        let key = context.idempotency_key();
        if let Some(first) = history.action_with_key(context.actor(), key) {
            if first.request != *request {
                return Err(CommandHistoryError::KeyConflict { key });
            }
            super::operation::checkpoint(observer, Phase::Replaying, 1, 1, None)?;
            return Ok(CommandHistoryResult {
                entry: first.clone(),
                replayed: true,
            });
        }
        let journal = self
            .journal
            .as_ref()
            .ok_or(CommandHistoryError::NotConfigured)?;
        if journal.entries.iter().any(|entry| {
            entry.actor() == context.actor()
                && entry
                    .request()
                    .mutations()
                    .iter()
                    .any(|member| member.context().idempotency_key() == key)
        }) {
            return Err(CommandHistoryError::KeyConflict { key });
        }
        let original = request.original();
        let accepted = journal
            .entries
            .iter()
            .find(|entry| entry.request().id() == original.id())
            .ok_or(CommandHistoryError::GroupNotAccepted {
                group: original.id(),
            })?;
        if accepted.request() != original {
            return Err(CommandHistoryError::GroupConflict {
                group: original.id(),
            });
        }
        self.require_group_revision(&authority_group)
            .map_err(|issue| {
                let (history_from, changes) =
                    self.revision_witness(journal, context.expected_revision());
                CommandHistoryError::Precondition {
                    issue: Box::new(issue),
                    history_from,
                    changes,
                }
            })?;
        super::operation::checkpoint(observer, Phase::SnapshotBound, 1, 1, None)?;
        let direction = request.direction();
        let target = history
            .top(direction)
            .ok_or(CommandHistoryError::Empty { direction })?;
        if target.group != original.id() {
            return Err(CommandHistoryError::NotCurrent {
                group: original.id(),
                direction,
            });
        }
        if history.actions.len() >= history.action_limit.get() {
            return Err(CommandHistoryError::ActionLimit {
                limit: history.action_limit.get(),
            });
        }
        self.admit_retained_current(observer)?;
        super::operation::checkpoint(observer, Phase::CurrentValidation, 0, 1, None)?;
        super::operation::admit_model(&self.state, self.require_model_limits()?)?;
        validate_snapshot(&self.state)
            .map_err(|issue| CommandHistoryError::Current(Box::new(issue)))?;
        super::operation::checkpoint(observer, Phase::CurrentValidation, 1, 1, None)?;
        super::operation::checkpoint(observer, Phase::CandidateValidation, 0, 1, None)?;
        super::operation::admit_model(&target.alternative, self.require_model_limits()?)?;
        validate_snapshot(&target.alternative)
            .map_err(|issue| CommandHistoryError::Alternative(Box::new(issue)))?;
        super::operation::checkpoint(observer, Phase::CandidateValidation, 1, 1, None)?;
        self.admit_retained_history(
            super::retained::HistoryProjection {
                target: &target.alternative,
                request,
                accepted,
                journal,
                history,
            },
            observer,
        )?;
        let before = self.state.inputs().revision().clone();
        let revision = target.alternative.inputs().revision().clone();
        let sequence = journal.sequence.successor();
        let commands = accepted
            .commit()
            .changes()
            .steps()
            .iter()
            .filter(|step| step.replay_of().is_none())
            .map(|step| step.command())
            .collect();
        super::operation::checkpoint(observer, Phase::AllocationPreflight, 0, 1, None)?;
        let history = self
            .history
            .as_mut()
            .ok_or(CommandHistoryError::NotConfigured)?;
        history
            .actions
            .try_reserve(1)
            .map_err(CommandHistoryError::Allocation)?;
        match direction {
            CommandHistoryDirection::Undo => history
                .redo
                .try_reserve(1)
                .map_err(CommandHistoryError::Allocation)?,
            CommandHistoryDirection::Redo => history
                .undo
                .try_reserve(1)
                .map_err(CommandHistoryError::Allocation)?,
        }
        let pending = PendingHistoryEntry {
            sequence: sequence.clone(),
            commands,
            request: request.clone(),
            before,
            revision,
        };
        let returned = pending.clone();
        super::operation::checkpoint(observer, Phase::AllocationPreflight, 1, 1, None)?;
        super::operation::checkpoint(observer, Phase::ReadyToPublish, 1, 1, None)?;
        let checked = self.authorize_group(&authority_group, CommandAuthorizationMode::Execute)?;
        let entry = pending.finish(checked.grant(), checked.checked_at());
        let result = CommandHistoryResult {
            entry: returned.finish(checked.grant(), checked.checked_at()),
            replayed: false,
        };
        let history = self
            .history
            .as_mut()
            .ok_or(CommandHistoryError::NotConfigured)?;
        let journal = self
            .journal
            .as_mut()
            .ok_or(CommandHistoryError::NotConfigured)?;
        let step = match direction {
            CommandHistoryDirection::Undo => history.undo.pop_back(),
            CommandHistoryDirection::Redo => history.redo.pop(),
        }
        .ok_or(CommandHistoryError::Empty { direction })?;
        let previous = core::mem::replace(&mut self.state, step.alternative);
        let reciprocal = HistoryStep {
            group: step.group,
            alternative: previous,
        };
        match direction {
            CommandHistoryDirection::Undo => history.redo.push(reciprocal),
            CommandHistoryDirection::Redo => history.undo.push_back(reciprocal),
        }
        history.actions.push(entry);
        journal.sequence = sequence;
        Ok(result)
    }
}

fn history_authority_group(
    request: &CommandHistoryRequest,
) -> Result<CommandMutationGroup, CommandMutationGroupError> {
    let operator = request.context();
    let original = request.original();
    let members = original
        .mutations()
        .iter()
        .map(|member| {
            CommandMutation::new(
                CommandMutationContext::new(
                    operator.design(),
                    operator.actor(),
                    operator.expected_revision().clone(),
                    member.context().idempotency_key(),
                    Some(original.id()),
                ),
                member.payload().clone(),
            )
        })
        .collect();
    CommandMutationGroup::new(original.id(), members)
}
fn validate_snapshot<H: DesignReferenceHost>(
    state: &DesignGarment<H>,
) -> Result<(), CommandPreviewError<H::Error>> {
    state
        .require_in_scope()
        .map_err(|issue| CommandPreviewError::Envelope(Box::new(issue)))?;
    state
        .validate_current()
        .map_err(|issue| CommandPreviewError::Current(Box::new(issue)))
}

impl<H: crate::design::DesignResourceSource> History<H> {
    pub(super) fn charge_after_history_resources(
        &self,
        owner: EntityId,
        previous: &DesignGarment<H>,
        projection: &super::retained::HistoryProjection<'_, H>,
        sink: &mut dyn crate::design::DesignResourceSink,
    ) -> Result<(), crate::design::DesignResourceLimitError> {
        let Self {
            depth,
            action_limit,
            undo,
            redo,
            actions,
        } = self;
        crate::command::resource::fixed(depth);
        crate::command::resource::fixed(action_limit);
        match projection.request.direction() {
            CommandHistoryDirection::Undo => {
                let retained = undo.iter().rev().skip(1).rev();
                sink.charge_entries(
                    retained.len(),
                    super::retained::location(owner, "retained.history.undo"),
                )?;
                for step in retained {
                    let HistoryStep { group, alternative } = step;
                    crate::command::resource::fixed(group);
                    crate::design::charge_design_garment_resources(alternative, sink)?;
                }
                super::retained::charge_appended_entries(
                    redo.len(),
                    sink,
                    super::retained::location(owner, "retained.history.redo"),
                )?;
                for step in redo {
                    let HistoryStep { group, alternative } = step;
                    crate::command::resource::fixed(group);
                    crate::design::charge_design_garment_resources(alternative, sink)?;
                }
                crate::design::charge_design_garment_resources(previous, sink)?;
            }
            CommandHistoryDirection::Redo => {
                super::retained::charge_appended_entries(
                    undo.len(),
                    sink,
                    super::retained::location(owner, "retained.history.undo"),
                )?;
                for step in undo {
                    let HistoryStep { group, alternative } = step;
                    crate::command::resource::fixed(group);
                    crate::design::charge_design_garment_resources(alternative, sink)?;
                }
                crate::design::charge_design_garment_resources(previous, sink)?;
                let retained = redo.iter().rev().skip(1).rev();
                sink.charge_entries(
                    retained.len(),
                    super::retained::location(owner, "retained.history.redo"),
                )?;
                for step in retained {
                    let HistoryStep { group, alternative } = step;
                    crate::command::resource::fixed(group);
                    crate::design::charge_design_garment_resources(alternative, sink)?;
                }
            }
        }
        super::retained::charge_appended_entries(
            actions.len(),
            sink,
            super::retained::location(owner, "retained.history.actions"),
        )?;
        for action in actions {
            super::resource::charge_command_history_audit_entry_resources(action, sink)?;
        }
        let group = projection.request.original().id().entity_id();
        crate::design::charge_revision_bits(
            projection.journal.sequence.successor_resource_bits(),
            sink,
            super::retained::location(group, "records.history.sequence"),
        )?;
        let steps = projection.accepted.commit().changes().steps();
        let effective = steps
            .iter()
            .filter(|step| step.replay_of().is_none())
            .count();
        sink.charge_entries(
            effective,
            super::retained::location(group, "records.history.commands"),
        )?;
        for step in steps.iter().filter(|step| step.replay_of().is_none()) {
            crate::command::resource::fixed(&step.command());
        }
        crate::command::charge_command_history_request_resources(projection.request, sink)?;
        sink.charge_counter_bits(
            previous.inputs().revision().resource_bits(),
            super::retained::location(group, "records.history.before_revision"),
        )?;
        sink.charge_counter_bits(
            projection.target.inputs().revision().resource_bits(),
            super::retained::location(group, "records.history.revision"),
        )
    }
    pub(super) fn charge_after_authoring_resources(
        &self,
        owner: EntityId,
        previous: &DesignGarment<H>,
        sink: &mut dyn crate::design::DesignResourceSink,
    ) -> Result<(), crate::design::DesignResourceLimitError> {
        let Self {
            depth,
            action_limit,
            undo,
            redo: _,
            actions,
        } = self;
        crate::command::resource::fixed(depth);
        crate::command::resource::fixed(action_limit);
        let retained = undo.iter().skip(usize::from(undo.len() >= depth.get()));
        super::retained::charge_appended_entries(
            retained.len(),
            sink,
            super::retained::location(owner, "retained.history.undo"),
        )?;
        for step in retained {
            let HistoryStep { group, alternative } = step;
            crate::command::resource::fixed(group);
            crate::design::charge_design_garment_resources(alternative, sink)?;
        }
        crate::design::charge_design_garment_resources(previous, sink)?;
        sink.charge_entries(0, super::retained::location(owner, "retained.history.redo"))?;
        sink.charge_entries(
            actions.len(),
            super::retained::location(owner, "retained.history.actions"),
        )?;
        for action in actions {
            super::resource::charge_command_history_audit_entry_resources(action, sink)?;
        }
        Ok(())
    }
    pub(super) fn charge_resources(
        &self,
        owner: EntityId,
        sink: &mut dyn crate::design::DesignResourceSink,
    ) -> Result<(), crate::design::DesignResourceLimitError> {
        let Self {
            depth,
            action_limit,
            undo,
            redo,
            actions,
        } = self;
        crate::command::resource::fixed(depth);
        crate::command::resource::fixed(action_limit);
        sink.charge_entries(
            undo.len(),
            super::retained::location(owner, "retained.history.undo"),
        )?;
        for step in undo {
            let HistoryStep { group, alternative } = step;
            crate::command::resource::fixed(group);
            crate::design::charge_design_garment_resources(alternative, sink)?;
        }
        sink.charge_entries(
            redo.len(),
            super::retained::location(owner, "retained.history.redo"),
        )?;
        for step in redo {
            let HistoryStep { group, alternative } = step;
            crate::command::resource::fixed(group);
            crate::design::charge_design_garment_resources(alternative, sink)?;
        }
        sink.charge_entries(
            actions.len(),
            super::retained::location(owner, "retained.history.actions"),
        )?;
        for action in actions {
            super::resource::charge_command_history_audit_entry_resources(action, sink)?;
        }
        Ok(())
    }
}
