//! Sequential actual metadata candidates; no model, reserve, revision or replay history is published.
use super::{
    group_difference, CommandBus, CommandDifferenceError, CommandGroupDifference, CommandPreview,
    CommandPreviewError, PreparationBase,
};
use crate::{
    command::{
        authorize_mutation_group, CommandAuthoritySource, CommandAuthorizationMode,
        CommandGroupAuthorizationError, CommandGroupId, CommandId, CommandIdempotencyKey,
        CommandMutationGroup, DesignRevision,
    },
    design::DesignReferenceHost,
    ontology::EntityId,
};
use core::fmt;
use std::collections::BTreeMap;

/// One original member's actual first report, or an explicit alias of that first report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroupPreviewStep {
    command: CommandId,
    key: CommandIdempotencyKey,
    replay_of: Option<usize>,
    report: CommandPreview,
}
impl CommandGroupPreviewStep {
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

/// Successful inspect-only group report; all private candidate models have been discarded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroupPreview {
    group: CommandGroupId,
    design: EntityId,
    revision: DesignRevision,
    steps: Vec<CommandGroupPreviewStep>,
    difference: CommandGroupDifference,
}
impl CommandGroupPreview {
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
        write!(f, "metadata group preview refused: {self:?}")
    }
}
impl<E: std::error::Error + 'static> std::error::Error for CommandGroupPreviewError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Authorization(issue) => Some(issue),
            Self::Mutation { issue, .. } => Some(issue.as_ref()),
            Self::Difference { issue, .. } => Some(issue),
            Self::DesignMismatch { .. }
            | Self::RevisionMismatch { .. }
            | Self::RetryConflict { .. } => None,
        }
    }
}
impl<H: DesignReferenceHost + Clone + PartialEq, A: CommandAuthoritySource> CommandBus<H, A> {
    /// Preview original ordered metadata mutations against successive actual validated candidates.
    /// One fresh trusted snapshot and one complete Inspect grant cover the entire group. Exact retries
    /// within this group return the first report explicitly; conflicting reuse refuses. This ephemeral
    /// alias table is discarded with the preview, and supplies no persisted replay or commit approval.
    /// # Errors
    /// Authority precedes actual Design/revision binding. Then the first member retry/preparation
    /// failure retains its original index/command/cause; final semantic comparison may also refuse.
    pub fn preview_group(
        &self,
        group: &CommandMutationGroup,
    ) -> Result<CommandGroupPreview, CommandGroupPreviewError<H::Error>> {
        authorize_mutation_group(group, CommandAuthorizationMode::Preview, &self.authority)
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
        if context.expected_revision() != inputs.revision() {
            return Err(CommandGroupPreviewError::RevisionMismatch {
                group: group.id(),
                requested: context.expected_revision().clone(),
                current: inputs.revision().clone(),
            });
        }
        let mut staged = None;
        let mut identities = self.identities.stage();
        let mut firsts = BTreeMap::new();
        let mut steps = Vec::new();
        for (index, mutation) in group.mutations().iter().enumerate() {
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
                continue;
            }
            let base = PreparationBase {
                state: staged.as_ref().unwrap_or(&self.state),
            };
            let prepared = base
                .prepare_payload(mutation.payload(), &mut identities)
                .map_err(|issue| CommandGroupPreviewError::Mutation {
                    group: group.id(),
                    index,
                    command: mutation.command_id(),
                    issue: Box::new(issue),
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
        }
        let final_state = staged.as_ref().unwrap_or(&self.state);
        let difference =
            group_difference::between(&self.state, final_state, &steps).map_err(|issue| {
                CommandGroupPreviewError::Difference {
                    group: group.id(),
                    issue,
                }
            })?;
        Ok(CommandGroupPreview {
            group: group.id(),
            design: inputs.id(),
            revision: inputs.revision().clone(),
            steps,
            difference,
        })
    }
}
