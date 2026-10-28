//! Builtin native workflow declarations; metadata never grants authority or runtime availability.
use super::{
    group_authorization::mutation_group_authority, CommandAuthority, CommandAuthorizationMode,
    CommandClass, CommandId,
};

const METADATA_COMMANDS: &[CommandId] = &[
    CommandId::AssignMaterial,
    CommandId::ChangeMeasurement,
    CommandId::AddNotch,
];

/// One declared operation in a builtin metadata workflow, without arguments or execution capability.
/// Each group operation refers to the same original three-command metadata group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CommandWorkflowStep {
    /// Authorized inspection through GetPieceList at the actual current represented revision.
    GetPieceList,
    /// Nonpublishing preview of the original metadata group.
    PreviewGroup,
    /// New atomic publication of the original metadata group.
    CommitGroup,
    /// Exact authorized replay of that accepted group, without another publication.
    ReplayGroup,
    /// Authorized restoration of the accepted group's actual predecessor snapshot.
    UndoGroup,
    /// Authorized restoration of the accepted group's actual successor snapshot.
    RedoGroup,
}
impl CommandWorkflowStep {
    /// Stable operation spelling; these are workflow stages, not additional command identities.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::GetPieceList => "get_piece_list",
            Self::PreviewGroup => "preview_group",
            Self::CommitGroup => "commit_group",
            Self::ReplayGroup => "replay_group",
            Self::UndoGroup => "undo_group",
            Self::RedoGroup => "redo_group",
        }
    }

    /// Actual catalog identities whose scopes the operation needs, in original group order.
    /// History and replay keep the original group scope; they invent no Undo/Redo command ids.
    #[must_use]
    pub const fn commands(self) -> &'static [CommandId] {
        match self {
            Self::GetPieceList => &[CommandId::GetPieceList],
            Self::PreviewGroup
            | Self::CommitGroup
            | Self::ReplayGroup
            | Self::UndoGroup
            | Self::RedoGroup => METADATA_COMMANDS,
        }
    }

    /// Exact purpose permission from the actual command catalog or shared group authorization rule.
    /// This describes one prerequisite; grants, complete scopes and current state still need checking.
    #[must_use]
    pub const fn required_authority(self) -> CommandAuthority {
        match self {
            Self::GetPieceList => CommandId::GetPieceList.required_authority(),
            Self::PreviewGroup => mutation_group_authority(CommandAuthorizationMode::Preview),
            Self::CommitGroup | Self::ReplayGroup | Self::UndoGroup | Self::RedoGroup => {
                mutation_group_authority(CommandAuthorizationMode::Execute)
            }
        }
    }
}

/// Builtin command workflows with actual native acceptance; adapter presence remains separate.
///
/// Declarations contain immutable stages and catalog identities only. Discovery reads no model,
/// session, grant, policy or quota and cannot make a handler available. Each actual bus operation
/// retains its own fresh authorization and admission. Multiple stages are separate operations,
/// not one atomic transaction across the whole workflow. UI/API/MCP parity needs adapter evidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CommandWorkflowId {
    /// Inspect the actual Piece list without editing the Design.
    InspectPieces,
    /// Inspect, preview and publish metadata, retrieve the exact replay, then inspect the result.
    EditMetadata,
    /// Publish metadata, inspect, Undo and inspect, then Redo and inspect the original successor.
    UndoRedoMetadata,
}
impl CommandWorkflowId {
    /// All current builtin declarations in stable discovery order.
    pub const ALL: [Self; 3] = [
        Self::InspectPieces,
        Self::EditMetadata,
        Self::UndoRedoMetadata,
    ];

    /// Stable discovery identity, independent of localization and transport entry-point names.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::InspectPieces => "inspect_pieces",
            Self::EditMetadata => "edit_metadata",
            Self::UndoRedoMetadata => "undo_redo_metadata",
        }
    }

    /// Exact discovery lookup; unknown spelling stays absent without trimming, aliases or a default.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|workflow| workflow.token() == token)
    }

    /// Borrow the single declared operation sequence; no execution or live-state inspection occurs.
    #[must_use]
    pub const fn steps(self) -> &'static [CommandWorkflowStep] {
        use CommandWorkflowStep as Step;
        match self {
            Self::InspectPieces => &[Step::GetPieceList],
            Self::EditMetadata => &[
                Step::GetPieceList,
                Step::PreviewGroup,
                Step::CommitGroup,
                Step::ReplayGroup,
                Step::GetPieceList,
            ],
            Self::UndoRedoMetadata => &[
                Step::GetPieceList,
                Step::CommitGroup,
                Step::GetPieceList,
                Step::UndoGroup,
                Step::GetPieceList,
                Step::RedoGroup,
                Step::GetPieceList,
            ],
        }
    }

    /// Unique required catalog identities in first-use order, derived from the declared stages.
    #[must_use]
    pub fn commands(self) -> Vec<CommandId> {
        unique(
            self.steps()
                .iter()
                .flat_map(|step| step.commands().iter().copied()),
        )
    }

    /// Actual command classes in first-use order, derived from catalog metadata without a shadow table.
    #[must_use]
    pub fn command_classes(self) -> Vec<CommandClass> {
        unique(
            self.steps()
                .iter()
                .flat_map(|step| step.commands().iter().map(|command| command.class())),
        )
    }

    /// Complete distinct purpose-permission set in first-use order; permissions imply no hierarchy.
    /// This list does not combine grant coverage or constitute a complete authorized decision.
    #[must_use]
    pub fn required_authorities(self) -> Vec<CommandAuthority> {
        unique(self.steps().iter().map(|step| step.required_authority()))
    }
}

fn unique<T: PartialEq>(items: impl IntoIterator<Item = T>) -> Vec<T> {
    let mut values = Vec::new();
    for item in items {
        if !values.contains(&item) {
            values.push(item);
        }
    }
    values
}
