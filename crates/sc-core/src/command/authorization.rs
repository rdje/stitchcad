//! Core permission decisions over one authoritative host snapshot, never over request claims alone.
use super::{
    CommandActorClaim, CommandAuthority, CommandClass, CommandGrant, CommandGrantStatus, CommandId,
    CommandRequest, CommandScopeRefusal,
};
use crate::ontology::EntityId;
use core::fmt;
use std::{collections::BTreeSet, time::SystemTime};

/// Explicit execution purpose; a mutation preview requires inspect and produces no committed effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandAuthorizationMode {
    /// Issue the command under its catalog permission.
    Execute,
    /// Inspect a mutation candidate; other command classes have no mutation-preview escape.
    Preview,
}

/// One consistent trusted-host observation, kept separate from serialized command input.
/// Only the trusted host may supply these facts, from authenticated session/delegation state.
/// Copying the submitted actor or time into this snapshot is not authentication.
#[derive(Clone, Copy, Debug)]
pub struct CommandAuthoritySnapshot<'a> {
    /// Verified session actor and human principal, or None if the session is unauthenticated.
    pub authenticated_actor: Option<CommandActorClaim>,
    /// Trusted observation time used for every grant in this decision.
    pub observed_at: SystemTime,
    /// Authoritative, current human-issued grants for this session, including revocation state.
    pub grants: &'a [CommandGrant],
}

/// Trusted application dependency, not an interface implemented by untrusted request data.
/// Adapters must authenticate sessions and validate grant issuance/delegation before supplying a
/// snapshot. Core enforces the checks below uniformly; no transport provider is implemented here.
pub trait CommandAuthoritySource {
    /// Return one internally consistent identity/time/current-grant observation.
    fn snapshot(&self) -> CommandAuthoritySnapshot<'_>;
}

/// One grant failed a required check. Permissions and scopes from different grants are never mixed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandGrantRefusal {
    /// A grant is bound to a different actor role, initiator or human principal.
    Actor {
        /// Exact grant binding.
        granted: CommandActorClaim,
        /// Verified session actor/principal.
        authenticated: CommandActorClaim,
    },
    /// The authoritative source reports this grant revoked.
    Revoked,
    /// This grant lacks the exact required permission; no hierarchy is inferred.
    Permission {
        /// Permission needed for this operation/purpose.
        required: CommandAuthority,
    },
    /// Design, exact command or observation time is outside this grant's scope.
    Scope(CommandScopeRefusal),
}
/// Actual rejected grant and its first failing condition, in source inventory order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandGrantRejection {
    /// Exact rejected grant identity.
    pub grant: EntityId,
    /// First failing condition for that grant.
    pub reason: CommandGrantRefusal,
}

/// Structured core refusal; transport diagnostics/localization remain adapter work.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandAuthorizationError {
    /// Trusted source could not authenticate the session.
    Unauthenticated,
    /// Submitted actor role, initiator or principal differs from the authenticated binding.
    ActorMismatch {
        /// Untrusted submitted claim.
        claimed: CommandActorClaim,
        /// Trusted session binding.
        authenticated: CommandActorClaim,
    },
    /// An agent cannot issue approval, irrespective of its other grants.
    HumanApprovalRequired {
        /// Verified agent/principal binding that attempted approval.
        actor: CommandActorClaim,
    },
    /// Only mutations have the inspect-only preview operation defined by the command contract.
    PreviewRequiresMutation {
        /// Actual submitted command identity.
        command: CommandId,
    },
    /// The source supplied multiple current records with the same grant identity.
    DuplicateGrant {
        /// Ambiguous identity; no record is selected by order.
        grant: EntityId,
    },
    /// No single authoritative grant covers every required dimension.
    NoMatchingGrant {
        /// Verified actor/principal binding.
        actor: CommandActorClaim,
        /// Actual requested Design.
        design: EntityId,
        /// Actual payload-derived identity.
        command: CommandId,
        /// Exact required permission for the requested purpose.
        required: CommandAuthority,
        /// Each available grant's first refusal; empty if the source supplies no grants.
        rejections: Vec<CommandGrantRejection>,
    },
}
impl fmt::Display for CommandAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command authorization refused: {self:?}")
    }
}
impl std::error::Error for CommandAuthorizationError {}

/// Read-only decision about one exact request at one source snapshot; not a cached execution token.
/// Execution must authorize at its effect boundary and check current sources/revisions separately.
/// Approval subject/evidence/scope validation and creation of a detached human record remain handlers.
#[derive(Debug, PartialEq, Eq)]
pub struct CommandAuthorization<'a> {
    request: &'a CommandRequest,
    mode: CommandAuthorizationMode,
    required: CommandAuthority,
    grant: EntityId,
    checked_at: SystemTime,
}
impl<'a> CommandAuthorization<'a> {
    /// Exact original request, retaining all typed content and actor/revision claims.
    #[must_use]
    pub const fn request(&self) -> &'a CommandRequest {
        self.request
    }
    /// Purpose checked, keeping a preview distinct from execution.
    #[must_use]
    pub const fn mode(&self) -> CommandAuthorizationMode {
        self.mode
    }
    /// Required permission, not a newly issued grant.
    #[must_use]
    pub const fn required_authority(&self) -> CommandAuthority {
        self.required
    }
    /// Identity of the one source grant satisfying all checks.
    #[must_use]
    pub const fn grant(&self) -> EntityId {
        self.grant
    }
    /// Exact trusted time used for this decision.
    #[must_use]
    pub const fn checked_at(&self) -> SystemTime {
        self.checked_at
    }
}

/// Authorize against one trusted identity/time/grant snapshot with no mutation or generated approval.
/// A request never supplies its own authority source. Permissions are explicit and remain distinct.
/// # Errors
/// Refuses unauthenticated or mismatched identity, agent approval, invalid preview purpose, then
/// ambiguous current grant identities, then absence of a single matching live grant.
/// Per-grant checks run actor, status, permission, scope.
pub fn authorize_command<'a>(
    request: &'a CommandRequest,
    mode: CommandAuthorizationMode,
    source: &dyn CommandAuthoritySource,
) -> Result<CommandAuthorization<'a>, CommandAuthorizationError> {
    let snapshot = source.snapshot();
    let actor = snapshot
        .authenticated_actor
        .ok_or(CommandAuthorizationError::Unauthenticated)?;
    if request.actor() != actor {
        return Err(CommandAuthorizationError::ActorMismatch {
            claimed: request.actor(),
            authenticated: actor,
        });
    }
    let command = request.command_id();
    if command == CommandId::ApprovePackage && matches!(actor, CommandActorClaim::Agent { .. }) {
        return Err(CommandAuthorizationError::HumanApprovalRequired { actor });
    }
    let required = match mode {
        CommandAuthorizationMode::Execute => command.required_authority(),
        CommandAuthorizationMode::Preview => {
            if command.class() != CommandClass::Mutation {
                return Err(CommandAuthorizationError::PreviewRequiresMutation { command });
            }
            CommandAuthority::Inspect
        }
    };
    let mut identities = BTreeSet::new();
    for grant in snapshot.grants {
        if !identities.insert(grant.identity()) {
            return Err(CommandAuthorizationError::DuplicateGrant {
                grant: grant.identity(),
            });
        }
    }
    let mut rejections = Vec::new();
    for grant in snapshot.grants {
        let reason = if grant.actor() != actor {
            Some(CommandGrantRefusal::Actor {
                granted: grant.actor(),
                authenticated: actor,
            })
        } else if grant.status() == CommandGrantStatus::Revoked {
            Some(CommandGrantRefusal::Revoked)
        } else if !grant.permissions().contains(&required) {
            Some(CommandGrantRefusal::Permission { required })
        } else {
            grant
                .scope()
                .check(request.design(), command, snapshot.observed_at)
                .err()
                .map(CommandGrantRefusal::Scope)
        };
        if let Some(reason) = reason {
            rejections.push(CommandGrantRejection {
                grant: grant.identity(),
                reason,
            });
        } else {
            return Ok(CommandAuthorization {
                request,
                mode,
                required,
                grant: grant.identity(),
                checked_at: snapshot.observed_at,
            });
        }
    }
    Err(CommandAuthorizationError::NoMatchingGrant {
        actor,
        design: request.design(),
        command,
        required,
        rejections,
    })
}
