//! Explicit local coverage of a grant scope; authentication and actor-held permissions are separate.
use super::CommandId;
use crate::ontology::EntityId;
use core::fmt;
use std::time::SystemTime;

/// Immutable Design, command inventory and finite validity window supplied by a trusted grant owner.
/// This value alone authenticates nobody and grants no permission. Empty commands deny everything.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandScope {
    design: EntityId,
    commands: Vec<CommandId>,
    valid_from: SystemTime,
    valid_until: SystemTime,
}

/// Invalid finite window, preserving both original endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandScopeWindowError {
    /// Supplied inclusive start.
    pub valid_from: SystemTime,
    /// Supplied exclusive end, which must be strictly later than the start.
    pub valid_until: SystemTime,
}
impl fmt::Display for CommandScopeWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "command scope requires start before end: {:?}..{:?}",
            self.valid_from, self.valid_until
        )
    }
}
impl std::error::Error for CommandScopeWindowError {}

/// The first uncovered dimension of one requested operation; not an authentication verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandScopeRefusal {
    /// The request names a different Design.
    Design {
        /// Scope's exact Design.
        scoped: EntityId,
        /// Request's exact Design.
        requested: EntityId,
    },
    /// The exact command is absent, even if another command of the same class is present.
    Command {
        /// Uncovered command identity.
        requested: CommandId,
    },
    /// The supplied trusted observation precedes the inclusive start.
    NotYetValid {
        /// Observation supplied by the caller, never read from an ambient clock here.
        at: SystemTime,
        /// Scope start.
        valid_from: SystemTime,
    },
    /// The supplied observation is at or after the exclusive end.
    Expired {
        /// Supplied observation.
        at: SystemTime,
        /// Scope end.
        valid_until: SystemTime,
    },
}
impl fmt::Display for CommandScopeRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command scope does not cover request: {self:?}")
    }
}
impl std::error::Error for CommandScopeRefusal {}

/// An attempted delegation expands at least one dimension of the parent scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandScopeNarrowingError {
    /// A narrowed scope must retain the parent's Design.
    Design {
        /// Parent Design.
        parent: EntityId,
        /// Requested Design.
        requested: EntityId,
    },
    /// The first requested command absent from the parent's explicit inventory.
    Command {
        /// Expanded command identity.
        requested: CommandId,
    },
    /// Requested start/end are not contained within the parent's interval.
    Validity {
        /// Parent inclusive start.
        parent_from: SystemTime,
        /// Parent exclusive end.
        parent_until: SystemTime,
        /// Requested inclusive start.
        requested_from: SystemTime,
        /// Requested exclusive end.
        requested_until: SystemTime,
    },
}
impl fmt::Display for CommandScopeNarrowingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command scope narrowing would expand coverage: {self:?}")
    }
}
impl std::error::Error for CommandScopeNarrowingError {}

impl CommandScope {
    /// Preserve exact authored Design, command order/repetition and explicit finite endpoints.
    /// No class-wide expansion, wildcard, ambient time or default duration is introduced.
    /// # Errors
    /// Returns both original endpoints if the interval is empty or reversed.
    pub fn new(
        design: EntityId,
        commands: Vec<CommandId>,
        valid_from: SystemTime,
        valid_until: SystemTime,
    ) -> Result<Self, CommandScopeWindowError> {
        if valid_from >= valid_until {
            return Err(CommandScopeWindowError {
                valid_from,
                valid_until,
            });
        }
        Ok(Self {
            design,
            commands,
            valid_from,
            valid_until,
        })
    }
    /// Exact covered Design identity.
    #[must_use]
    pub const fn design(&self) -> EntityId {
        self.design
    }
    /// Authored command inventory, without normalization or inferred siblings.
    #[must_use]
    pub fn commands(&self) -> &[CommandId] {
        &self.commands
    }
    /// Inclusive validity start, without reading a clock.
    #[must_use]
    pub const fn valid_from(&self) -> SystemTime {
        self.valid_from
    }
    /// Exclusive validity end.
    #[must_use]
    pub const fn valid_until(&self) -> SystemTime {
        self.valid_until
    }

    /// Check only the scope's Design, exact command and half-open validity window.
    /// Authentication, actor/principal binding, permissions and revocation must be checked separately.
    /// # Errors
    /// Refuses the first mismatch in Design, command, start, end order, retaining actual values.
    pub fn check(
        &self,
        design: EntityId,
        command: CommandId,
        at: SystemTime,
    ) -> Result<(), CommandScopeRefusal> {
        if design != self.design {
            return Err(CommandScopeRefusal::Design {
                scoped: self.design,
                requested: design,
            });
        }
        if !self.commands.contains(&command) {
            return Err(CommandScopeRefusal::Command { requested: command });
        }
        if at < self.valid_from {
            return Err(CommandScopeRefusal::NotYetValid {
                at,
                valid_from: self.valid_from,
            });
        }
        if at >= self.valid_until {
            return Err(CommandScopeRefusal::Expired {
                at,
                valid_until: self.valid_until,
            });
        }
        Ok(())
    }

    /// Validate an explicitly requested child scope as a subset, returning it unchanged on success.
    /// Equal scope is allowed; an empty command inventory remains a valid deny-all child.
    /// No clipping or intersection silently repairs a requested expansion.
    /// # Errors
    /// Refuses a different Design, first added command or enlarged validity interval, in that order.
    pub fn narrow_to(&self, requested: Self) -> Result<Self, CommandScopeNarrowingError> {
        if requested.design != self.design {
            return Err(CommandScopeNarrowingError::Design {
                parent: self.design,
                requested: requested.design,
            });
        }
        if let Some(&command) = requested
            .commands
            .iter()
            .find(|command| !self.commands.contains(command))
        {
            return Err(CommandScopeNarrowingError::Command { requested: command });
        }
        if requested.valid_from < self.valid_from || requested.valid_until > self.valid_until {
            return Err(CommandScopeNarrowingError::Validity {
                parent_from: self.valid_from,
                parent_until: self.valid_until,
                requested_from: requested.valid_from,
                requested_until: requested.valid_until,
            });
        }
        Ok(requested)
    }
}
