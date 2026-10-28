//! Sequential actual metadata candidates; no model, reserve, revision or replay history is published.
use super::{
    group_difference, CommandBus, CommandDifferenceError, CommandGroupDifference, CommandPreview,
    CommandPreviewError, PreparationBase,
};
use crate::{
    command::{
        authorize_mutation_group, CommandAuthoritySource, CommandAuthorizationMode,
        CommandGroupAuthorization, CommandGroupAuthorizationError, CommandGroupId, CommandId,
        CommandIdempotencyKey, CommandMutationGroup, DesignRevision,
    },
    design::DesignGarment,
    ontology::EntityId,
};
use core::fmt;
use std::collections::BTreeMap;

// Actual final validated ownership stays inside the bus, together with its derived report and the
// concrete cursor's consumed prefix. Public preview discards all publication-side ownership.
pub(super) struct PreparedGroup<H> {
    state: DesignGarment<H>,
    report: CommandGroupPreview,
    identities_consumed: usize,
}
impl<H> PreparedGroup<H> {
    pub(super) fn into_parts(self) -> (DesignGarment<H>, CommandGroupPreview, usize) {
        (self.state, self.report, self.identities_consumed)
    }
    fn into_report(self) -> CommandGroupPreview {
        let (state, report, _identities_consumed) = self.into_parts();
        drop(state);
        report
    }
}

/// One original member's actual first report, or an explicit alias of that first report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroupPreviewStep {
    command: CommandId,
    key: CommandIdempotencyKey,
    replay_of: Option<usize>,
    report: CommandPreview,
}
impl CommandGroupPreviewStep {
    pub(super) fn resource_report(&self) -> &CommandPreview {
        let Self {
            command,
            key,
            replay_of,
            report,
        } = self;
        crate::command::resource::fixed(command);
        crate::command::resource::fixed(key);
        crate::command::resource::fixed(replay_of);
        report
    }
    /// Original payload-derived command identity.
    #[must_use]
    pub const fn command(&self) -> CommandId {
        self.command
    }
    /// Original submitted retry identity.
    #[must_use]
    pub const fn key(&self) -> CommandIdempotencyKey {
        self.key
    }
    /// Original zero-based first occurrence for an exact within-group retry, otherwise None.
    #[must_use]
    pub const fn replay_of(&self) -> Option<usize> {
        self.replay_of
    }
    /// Actual original report; an alias does not reapply the edit or consume another identity.
    #[must_use]
    pub const fn report(&self) -> &CommandPreview {
        &self.report
    }
}

/// Base-revision metadata report; carries no candidate model or publication capability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroupPreview {
    group: CommandGroupId,
    design: EntityId,
    revision: DesignRevision,
    steps: Vec<CommandGroupPreviewStep>,
    difference: CommandGroupDifference,
}
impl CommandGroupPreview {
    pub(super) fn resource_parts(
        &self,
    ) -> (
        CommandGroupId,
        &DesignRevision,
        &[CommandGroupPreviewStep],
        &CommandGroupDifference,
    ) {
        let Self {
            group,
            design,
            revision,
            steps,
            difference,
        } = self;
        crate::command::resource::fixed(design);
        (*group, revision, steps, difference)
    }
    /// Original explicit group identity.
    #[must_use]
    pub const fn group(&self) -> CommandGroupId {
        self.group
    }
    /// Actual bound Design.
    #[must_use]
    pub const fn design(&self) -> EntityId {
        self.design
    }
    /// Exact unchanged original revision.
    #[must_use]
    pub const fn revision(&self) -> &DesignRevision {
        &self.revision
    }
    /// Reports in submitted order, including explicit retry aliases.
    #[must_use]
    pub fn steps(&self) -> &[CommandGroupPreviewStep] {
        &self.steps
    }
    /// Net semantic difference between the actual original and final validated candidate.
    #[must_use]
    pub const fn difference(&self) -> &CommandGroupDifference {
        &self.difference
    }
}

/// Original group refusal; no partial accepted report or private candidate escapes.
#[derive(Debug)]
pub enum CommandGroupPreviewError<E> {
    /// Original host policy/admission/cancellation refusal for this complete group.
    Control {
        /// Original explicit group identity.
        group: CommandGroupId,
        /// Original typed control cause and exact checkpoint when cancelled.
        issue: Box<super::CommandControlError>,
    },
    /// Original whole-group authority cause, checked before model inspection.
    Authorization(CommandGroupAuthorizationError),
    /// Common Design claim differs from the actual private owner.
    DesignMismatch {
        /// Original explicit group identity.
        group: CommandGroupId,
        /// Common submitted Design identity.
        requested: EntityId,
        /// Actual private owner's Design identity.
        current: EntityId,
    },
    /// Common exact revision differs from the actual private owner.
    RevisionMismatch {
        /// Original explicit group identity.
        group: CommandGroupId,
        /// Common exact submitted revision.
        requested: DesignRevision,
        /// Actual private owner's exact revision.
        current: DesignRevision,
    },
    /// Same group-local retry key names a different original request.
    RetryConflict {
        /// Original explicit group identity.
        group: CommandGroupId,
        /// Original reused retry identity.
        key: CommandIdempotencyKey,
        /// Zero-based first occurrence of that key.
        first: usize,
        /// Zero-based conflicting occurrence in submitted order.
        index: usize,
        /// Conflicting member's actual payload-derived command.
        command: CommandId,
    },
    /// Original member preparation cause, with its exact submitted position.
    Mutation {
        /// Original explicit group identity.
        group: CommandGroupId,
        /// Zero-based failing member position in submitted order.
        index: usize,
        /// Failing member's actual payload-derived command.
        command: CommandId,
        /// Original complete preparation cause, retained in the standard source chain.
        issue: Box<CommandPreviewError<E>>,
    },
    /// Actual final candidate changed content outside the successful members' semantic footprint.
    Difference {
        /// Original explicit group identity.
        group: CommandGroupId,
        /// Original final-model semantic integrity cause.
        issue: CommandDifferenceError,
    },
}
impl<E: fmt::Debug> fmt::Display for CommandGroupPreviewError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "metadata group preparation refused: {self:?}")
    }
}
impl<E> CommandGroupPreviewError<E> {
    pub(super) fn control(group: CommandGroupId, issue: super::CommandControlError) -> Self {
        Self::Control {
            group,
            issue: Box::new(issue),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for CommandGroupPreviewError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Control { issue, .. } => Some(issue.as_ref()),
            Self::Authorization(issue) => Some(issue),
            Self::Mutation { issue, .. } => Some(issue.as_ref()),
            Self::Difference { issue, .. } => Some(issue),
            Self::DesignMismatch { .. }
            | Self::RevisionMismatch { .. }
            | Self::RetryConflict { .. } => None,
        }
    }
}
impl<H: crate::design::DesignResourceSource + Clone + PartialEq, A: CommandAuthoritySource>
    CommandBus<H, A>
{
    /// Preview original ordered metadata mutations against successive actual validated candidates.
    /// One fresh trusted snapshot and one complete Inspect grant cover the entire group. Exact retries
    /// within this group return the first report explicitly; conflicting reuse refuses. This ephemeral
    /// alias table is discarded with the preview, and supplies no persisted replay or commit approval.
    /// # Errors
    /// Installed policy and pure input admission precede authority and actual Design/revision binding.
    /// Then the first member retry/preparation
    /// failure retains its original index/command/cause; final semantic comparison may also refuse.
    pub fn preview_group(
        &self,
        group: &CommandMutationGroup,
    ) -> Result<CommandGroupPreview, CommandGroupPreviewError<H::Error>> {
        self.prepare_group(group, CommandAuthorizationMode::Preview)
            .map(PreparedGroup::into_report)
    }

    pub(super) fn prepare_group(
        &self,
        group: &CommandMutationGroup,
        mode: CommandAuthorizationMode,
    ) -> Result<PreparedGroup<H>, CommandGroupPreviewError<H::Error>> {
        self.prepare_group_with_progress(group, mode, &())
    }

    /// Preview actual ordered members with data-only pre-effect progress and cancellation.
    /// # Errors
    /// Preserves original preparation causes and trusted policy/admission/cancellation refusals.
    pub fn preview_group_with_progress(
        &self,
        group: &CommandMutationGroup,
        observer: &dyn crate::command::CommandProgressObserver,
    ) -> Result<CommandGroupPreview, CommandGroupPreviewError<H::Error>> {
        self.prepare_group_with_progress(group, CommandAuthorizationMode::Preview, observer)
            .map(PreparedGroup::into_report)
    }

    fn prepare_group_with_progress(
        &self,
        group: &CommandMutationGroup,
        mode: CommandAuthorizationMode,
        observer: &dyn crate::command::CommandProgressObserver,
    ) -> Result<PreparedGroup<H>, CommandGroupPreviewError<H::Error>> {
        self.admit_group(group)
            .map_err(|issue| CommandGroupPreviewError::control(group.id(), issue))?;
        self.authorize_group(group, mode)?;
        use crate::command::CommandProgressPhase as Phase;
        super::operation::group_checkpoint(group, observer, Phase::Authorized, 1, 1, None)?;
        self.require_group_revision(group)?;
        super::operation::group_checkpoint(group, observer, Phase::SnapshotBound, 1, 1, None)?;
        self.stage_group_with_progress(group, observer)
    }

    pub(super) fn authorize_group<'a>(
        &self,
        group: &'a CommandMutationGroup,
        mode: CommandAuthorizationMode,
    ) -> Result<CommandGroupAuthorization<'a>, CommandGroupPreviewError<H::Error>> {
        let checked = authorize_mutation_group(group, mode, &self.authority)
            .map_err(CommandGroupPreviewError::Authorization)?;
        let inputs = self.state.inputs();
        let context = group.context();
        if context.design() != inputs.id() {
            return Err(CommandGroupPreviewError::DesignMismatch {
                group: group.id(),
                requested: context.design(),
                current: inputs.id(),
            });
        }
        Ok(checked)
    }

    pub(super) fn require_group_revision(
        &self,
        group: &CommandMutationGroup,
    ) -> Result<(), CommandGroupPreviewError<H::Error>> {
        let inputs = self.state.inputs();
        let context = group.context();
        if context.expected_revision() != inputs.revision() {
            return Err(CommandGroupPreviewError::RevisionMismatch {
                group: group.id(),
                requested: context.expected_revision().clone(),
                current: inputs.revision().clone(),
            });
        }
        Ok(())
    }

    pub(super) fn stage_group_with_progress(
        &self,
        group: &CommandMutationGroup,
        observer: &dyn crate::command::CommandProgressObserver,
    ) -> Result<PreparedGroup<H>, CommandGroupPreviewError<H::Error>> {
        let inputs = self.state.inputs();
        let mut staged = None;
        let mut identities = self.identities.stage();
        let mut firsts = BTreeMap::new();
        let mut steps = Vec::new();
        use crate::command::CommandProgressPhase as Phase;
        for (index, mutation) in group.mutations().iter().enumerate() {
            let member = Some((index, mutation.command_id()));
            super::operation::group_checkpoint(
                group,
                observer,
                Phase::Preparing,
                index,
                group.mutations().len(),
                member,
            )?;
            let key = mutation.context().idempotency_key();
            if let Some((first, original, report)) = firsts.get(&key) {
                if mutation != *original {
                    return Err(CommandGroupPreviewError::RetryConflict {
                        group: group.id(),
                        key,
                        first: *first,
                        index,
                        command: mutation.command_id(),
                    });
                }
                steps.push(CommandGroupPreviewStep {
                    command: mutation.command_id(),
                    key,
                    replay_of: Some(*first),
                    report: CommandPreview::clone(report),
                });
                super::operation::group_checkpoint(
                    group,
                    observer,
                    Phase::Preparing,
                    index + 1,
                    group.mutations().len(),
                    member,
                )?;
                continue;
            }
            let base = PreparationBase {
                state: staged.as_ref().unwrap_or(&self.state),
                limits: self
                    .require_model_limits()
                    .map_err(|issue| CommandGroupPreviewError::control(group.id(), issue))?,
            };
            let prepared = base
                .prepare_payload(mutation.payload(), &mut identities, observer, member)
                .map_err(|issue| match issue {
                    CommandPreviewError::Control(issue) => CommandGroupPreviewError::Control {
                        group: group.id(),
                        issue,
                    },
                    issue => CommandGroupPreviewError::Mutation {
                        group: group.id(),
                        index,
                        command: mutation.command_id(),
                        issue: Box::new(issue),
                    },
                })?;
            let (state, report) = prepared.into_parts();
            staged = Some(state);
            firsts.insert(key, (index, mutation, report.clone()));
            steps.push(CommandGroupPreviewStep {
                command: mutation.command_id(),
                key,
                replay_of: None,
                report,
            });
            super::operation::group_checkpoint(
                group,
                observer,
                Phase::Preparing,
                index + 1,
                group.mutations().len(),
                member,
            )?;
        }
        let final_state = staged.as_ref().unwrap_or(&self.state);
        let difference =
            group_difference::between(&self.state, final_state, &steps).map_err(|issue| {
                CommandGroupPreviewError::Difference {
                    group: group.id(),
                    issue,
                }
            })?;
        let report = CommandGroupPreview {
            group: group.id(),
            design: inputs.id(),
            revision: inputs.revision().clone(),
            steps,
            difference,
        };
        // A validated group is nonempty, and the first member cannot be a retry. Successful
        // preparation therefore always owns a candidate; the fallback preserves the original
        // owner if that schema invariant is ever broadened to permit an empty no-op group.
        let state = staged.unwrap_or_else(|| self.state.clone());
        Ok(PreparedGroup {
            state,
            report,
            identities_consumed: identities.consumed(),
        })
    }
}
