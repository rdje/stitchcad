//! Whole-group authorization over actual trusted snapshot facts, with no grant union or execution.
use super::{
    authorization::{authenticated_actor, grant_refusal, unique_grants},
    CommandActorClaim, CommandAuthority, CommandAuthoritySource, CommandAuthorizationError,
    CommandAuthorizationMode, CommandGrantRefusal, CommandGroupId, CommandId, CommandMutationGroup,
};
use crate::ontology::EntityId;
use core::fmt;
use std::time::SystemTime;

/// One actual source grant's first failing original group member, in source-grant order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandGroupGrantRejection {
    /// Original source grant identity.
    pub grant: EntityId,
    /// Zero-based original mutation position.
    pub index: usize,
    /// Actual payload-derived command at that position.
    pub command: CommandId,
    /// Original actor/status/permission/complete-scope refusal, without a substitute error.
    pub reason: CommandGrantRefusal,
}

/// Original group authority refusal; no candidate or partial operation is accepted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandGroupAuthorizationError {
    /// Original session/actor/duplicate-grant cause, before selecting any grant/member result.
    Source {
        /// Original requested group identity.
        group: CommandGroupId,
        /// Original core identity/current-inventory cause.
        issue: Box<CommandAuthorizationError>,
    },
    /// No single authoritative grant covers every actual member of this group.
    NoMatchingGrant {
        /// Original containing group identity.
        group: CommandGroupId,
        /// Verified source actor/principal binding.
        actor: CommandActorClaim,
        /// Original requested Design; existence/currentness remain separate checks.
        design: EntityId,
        /// Exact required permission: Inspect for preview, Commit for mutation execution.
        required: CommandAuthority,
        /// Each original grant's first failing command/refusal, in source order.
        rejections: Vec<CommandGroupGrantRejection>,
    },
}
impl fmt::Display for CommandGroupAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mutation-group authorization refused: {self:?}")
    }
}
impl std::error::Error for CommandGroupAuthorizationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Source { issue, .. } => Some(issue.as_ref()),
            Self::NoMatchingGrant { .. } => None,
        }
    }
}

/// Descriptive decision borrowing the exact original group; never a cached execution capability.
/// ```compile_fail
/// use sc_core::command::{authorize_mutation_group, CommandAuthoritySource,
///     CommandAuthorizationMode, CommandGroupAuthorization, CommandGroupAuthorizationError,
///     CommandMutationGroup};
/// fn escape(group: CommandMutationGroup, source: &dyn CommandAuthoritySource)
///     -> Result<CommandGroupAuthorization<'static>, CommandGroupAuthorizationError> {
///     authorize_mutation_group(&group, CommandAuthorizationMode::Preview, source)
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct CommandGroupAuthorization<'a> {
    group: &'a CommandMutationGroup,
    mode: CommandAuthorizationMode,
    required: CommandAuthority,
    grant: EntityId,
    checked_at: SystemTime,
}
impl<'a> CommandGroupAuthorization<'a> {
    /// Exact original immutable group, contexts, payloads and retry keys.
    #[must_use]
    pub const fn group(&self) -> &'a CommandMutationGroup {
        self.group
    }
    /// Preview versus execution purpose actually checked.
    #[must_use]
    pub const fn mode(&self) -> CommandAuthorizationMode {
        self.mode
    }
    /// Explicit required permission, without a hierarchy or a newly issued grant.
    #[must_use]
    pub const fn required_authority(&self) -> CommandAuthority {
        self.required
    }
    /// Original one grant covering the whole group.
    #[must_use]
    pub const fn grant(&self) -> EntityId {
        self.grant
    }
    /// Exact original trusted snapshot time for all grant/member checks.
    #[must_use]
    pub const fn checked_at(&self) -> SystemTime {
        self.checked_at
    }
}

/// Check one fresh trusted observation and one complete grant over every typed mutation member.
/// Shared original grant checks retain their actor/status/permission/scope order; each grant reports
/// its first failing member. Other grants' permissions/scopes cannot fill missing coverage.
/// No model lookup, revision-currentness check, identity allocation, replay or execution occurs.
/// Effects must independently reauthorize and validate actual subjects/revisions/sources.
/// # Errors
/// Preserves original unauthenticated/actor/duplicate-grant causes, then complete source-ordered
/// per-grant refusals when no one grant covers the whole original group.
pub fn authorize_mutation_group<'a>(
    group: &'a CommandMutationGroup,
    mode: CommandAuthorizationMode,
    source: &dyn CommandAuthoritySource,
) -> Result<CommandGroupAuthorization<'a>, CommandGroupAuthorizationError> {
    let snapshot = source.snapshot();
    let actor = authenticated_actor(group.context().actor(), &snapshot).map_err(|issue| {
        CommandGroupAuthorizationError::Source {
            group: group.id(),
            issue: Box::new(issue),
        }
    })?;
    unique_grants(snapshot.grants).map_err(|issue| CommandGroupAuthorizationError::Source {
        group: group.id(),
        issue: Box::new(issue),
    })?;
    let required = match mode {
        CommandAuthorizationMode::Preview => CommandAuthority::Inspect,
        CommandAuthorizationMode::Execute => CommandAuthority::Commit,
    };
    let mut rejections = Vec::new();
    for grant in snapshot.grants {
        let failure = group
            .mutations()
            .iter()
            .enumerate()
            .find_map(|(index, mutation)| {
                let command = mutation.command_id();
                grant_refusal(
                    grant,
                    actor,
                    required,
                    group.context().design(),
                    command,
                    snapshot.observed_at,
                )
                .map(|reason| CommandGroupGrantRejection {
                    grant: grant.identity(),
                    index,
                    command,
                    reason,
                })
            });
        if let Some(failure) = failure {
            rejections.push(failure);
        } else {
            return Ok(CommandGroupAuthorization {
                group,
                mode,
                required,
                grant: grant.identity(),
                checked_at: snapshot.observed_at,
            });
        }
    }
    Err(CommandGroupAuthorizationError::NoMatchingGrant {
        group: group.id(),
        actor,
        design: group.context().design(),
        required,
        rejections,
    })
}
