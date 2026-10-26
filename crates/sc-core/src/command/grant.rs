//! Authoritative host-supplied grant records, distinct from request claims and catalog metadata.
use super::{CommandActorClaim, CommandAuthority, CommandScope};
use crate::ontology::EntityId;
use core::fmt;

/// Current authoritative status; revocation is checked afresh during authorization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandGrantStatus {
    /// Grant may be considered, subject to every other identity/permission/scope check.
    Active,
    /// Grant must not authorize an operation.
    Revoked,
}

/// Explicit grant data supplied by the trusted host's human-delegation registry.
/// Construction does not authenticate an issuer, persist a grant or make request data authoritative.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGrant {
    identity: EntityId,
    actor: CommandActorClaim,
    permissions: Vec<CommandAuthority>,
    scope: CommandScope,
    status: CommandGrantStatus,
}

/// An agent cannot hold the human-only approval permission, even in an otherwise unused grant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AgentApprovalGrantError {
    /// Original grant identity.
    pub grant: EntityId,
    /// Original actor/principal pair declared for that grant.
    pub actor: CommandActorClaim,
}
impl fmt::Display for AgentApprovalGrantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "agent cannot hold approval permission in grant {}",
            self.grant
        )
    }
}
impl std::error::Error for AgentApprovalGrantError {}

impl CommandGrant {
    /// Preserve exact grant fields; permissions are explicit and imply no other permission.
    /// Empty permissions authorize nothing. Order/repetition is retained, never sorted or expanded.
    /// The trusted host must validate human issuance, delegation and registry currentness.
    /// # Errors
    /// An agent subject with any Approve entry is refused, regardless of status or command scope.
    pub fn new(
        identity: EntityId,
        actor: CommandActorClaim,
        permissions: Vec<CommandAuthority>,
        scope: CommandScope,
        status: CommandGrantStatus,
    ) -> Result<Self, AgentApprovalGrantError> {
        if matches!(actor, CommandActorClaim::Agent { .. })
            && permissions.contains(&CommandAuthority::Approve)
        {
            return Err(AgentApprovalGrantError {
                grant: identity,
                actor,
            });
        }
        Ok(Self {
            identity,
            actor,
            permissions,
            scope,
            status,
        })
    }
    /// Exact grant identity for attribution, not a token a caller can redeem independently.
    #[must_use]
    pub const fn identity(&self) -> EntityId {
        self.identity
    }
    /// Exact bound authenticated actor/principal expected by this grant.
    #[must_use]
    pub const fn actor(&self) -> CommandActorClaim {
        self.actor
    }
    /// Explicit original permission inventory.
    #[must_use]
    pub fn permissions(&self) -> &[CommandAuthority] {
        &self.permissions
    }
    /// Original Design/command/time scope.
    #[must_use]
    pub const fn scope(&self) -> &CommandScope {
        &self.scope
    }
    /// Host-supplied current status.
    #[must_use]
    pub const fn status(&self) -> CommandGrantStatus {
        self.status
    }
}
