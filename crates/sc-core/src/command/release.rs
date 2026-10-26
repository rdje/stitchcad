//! Candidate preparation inputs; generated content and detached human approval remain separate effects.
use super::{ApprovePackage, ArtifactPayload, CommandId, CommandSnapshotContext};
use core::fmt;

/// Prepare one immutable content candidate from explicit canonical artifact requests.
/// Authored order and repetition remain intact for execution diagnostics, never silently deduplicated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrepareRelease {
    artifacts: Vec<ArtifactPayload>,
}
/// A candidate request must name actual requested content rather than silently selecting a Design default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrepareReleaseError {
    /// No artifact request was supplied.
    NoArtifacts,
}
impl fmt::Display for PrepareReleaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoArtifacts => {
                f.write_str("release preparation requires at least one artifact request")
            }
        }
    }
}
impl std::error::Error for PrepareReleaseError {}
impl PrepareRelease {
    /// Retain explicit requests for actual sources/configurations, including required companions.
    /// Execution must validate outputs, collisions/completeness, production policy and dispositions.
    /// It seals the fixed detached approval binding; this constructor creates no candidate or identity.
    /// # Errors
    /// Returns NoArtifacts for an empty inventory; no latest artifacts or default target are selected.
    pub fn new(artifacts: Vec<ArtifactPayload>) -> Result<Self, PrepareReleaseError> {
        if artifacts.is_empty() {
            return Err(PrepareReleaseError::NoArtifacts);
        }
        Ok(Self { artifacts })
    }
    /// Original ordered inputs, not generated files or a completeness/production-readiness certificate.
    #[must_use]
    pub fn artifacts(&self) -> &[ArtifactPayload] {
        &self.artifacts
    }
}

/// Concrete release input schemas; actual generation, human authorization and records remain separate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReleasePayload {
    /// Generate/check/seal a candidate; no approval metadata or actor-held grant is supplied here.
    PrepareRelease(PrepareRelease),
    /// Exact subject/scope approval intent; untrusted claims provide no human approval capability.
    ApprovePackage(ApprovePackage),
}
impl ReleasePayload {
    /// Command identity fixed by the actual typed request.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        match self {
            Self::PrepareRelease(_) => CommandId::PrepareRelease,
            Self::ApprovePackage(_) => CommandId::ApprovePackage,
        }
    }
}

/// Immutable release request against explicit Design/revision/actor claims, before authentication or effects.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandRelease {
    context: CommandSnapshotContext,
    payload: ReleasePayload,
}
impl CommandRelease {
    /// Bind exact context/inputs. No mutation retry key/group, package identity or approval is allocated.
    #[must_use]
    pub const fn new(context: CommandSnapshotContext, payload: ReleasePayload) -> Self {
        Self { context, payload }
    }
    /// Original snapshot and actor/principal claims, which grant no authority.
    #[must_use]
    pub const fn context(&self) -> &CommandSnapshotContext {
        &self.context
    }
    /// Canonical content requests; actual generation and readiness remain execution responsibilities.
    #[must_use]
    pub const fn payload(&self) -> &ReleasePayload {
        &self.payload
    }
    /// Actual command identity and its required generate or human-only approve metadata.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        self.payload.command_id()
    }
}
