# Exact command scopes

`CommandScope` is immutable Design/command/time policy for [core authority](command-layer.md#7-agent-authority-as-a-command-layer-concept).
It grants no permission and authenticates nobody.

## API vocabulary

These are the existing core scope/authorization names, not transport capabilities.

| API token | Meaning |
| --- | --- |
| `CommandScope` | Exact Design, ordered commands and half-open time window |
| `CommandScopeWindowError` | Refused empty/reversed window with original endpoints |
| `CommandScopeNarrowingError` | Refused Design, command or time-window expansion |
| `SystemTime` | Standard-library time value used for explicit endpoints and observations |
| `CommandAuthoritySource` | Trusted host interface supplying one consistent snapshot |
| `CommandAuthoritySnapshot` | Authenticated binding, observation time and current grants |
| `CommandGrantStatus` | Active or revoked status of a host-supplied grant |
| `AgentApprovalGrantError` | Refusal to construct an agent grant containing approval permission |
| `CommandAuthorization` | Borrowed request and original successful authorization decision |
| `CommandAuthorizationError` | Typed single-command authentication/authorization refusal |
| `NoMatchingGrant` | Authorization refusal when no one grant covers all required dimensions |
| `Preview` | Mutation-only authorization mode requiring inspect permission |

## Explicit coverage

`CommandScope::new(design, commands, valid_from, valid_until)` preserves the Design, authored command
order/repetition and explicit `SystemTime` endpoints. The window includes its start and excludes its end. Empty/reversed windows return
`CommandScopeWindowError` with both endpoints.
No clock/default duration or wildcard is supplied. Empty commands deny all.

```rust
use sc_core::{command::{CommandId, CommandScope}, ontology::EntityId};
use std::time::{Duration, UNIX_EPOCH};
let start = UNIX_EPOCH + Duration::from_secs(10); // fixed example time, not a product default
let end = start + Duration::from_secs(20);
let scope = CommandScope::new(EntityId::from_bits(1), vec![CommandId::ExportPdf], start, end)?;
assert_eq!(scope.check(EntityId::from_bits(1), CommandId::ExportPdf, start), Ok(()));
assert!(scope.check(EntityId::from_bits(1), CommandId::ExportPlt, start).is_err());
assert!(scope.check(EntityId::from_bits(1), CommandId::ExportPdf, end).is_err());
# Ok::<(), Box<dyn std::error::Error>>(())
```

`check(design, command, at)` checks Design, exact command, start and end in order. Typed refusals retain
actual mismatches. Class membership adds no commands. Execution supplies trusted time/grant data.

## Narrow without silently repairing an expansion

`parent.narrow_to(requested)` returns the requested scope unchanged only if its Design matches,
every named command belongs to the parent and its full time window lies within the parent's window.
Equal coverage and deny-all children are allowed; command order/repetition remain authored.
A different Design, first added command or enlarged interval returns `CommandScopeNarrowingError`.
The parent stays unchanged. No clipping or intersection hides a wider request.

Actor/permission/revocation checks follow below. Current-source/revision checks, transport authentication,
preview/commit execution and human approval remain separate work.

## Core authorization

Repeated decisions re-read current host state. Native freshness controls refuse revoked or expired
grants after an earlier success for the same request; the earlier decision remains only its original snapshot.

`CommandAuthoritySource` is a trusted host dependency, separate from command input. Its one consistent
`CommandAuthoritySnapshot` supplies the authenticated actor/principal, trusted observation time and
current human-issued grants. Adapters must establish session identity and valid human delegation;
copying a request's actor into this source is not authentication. No transport provider ships yet.

`CommandGrant::new(identity, actor, permissions, scope, status)` retains exact fields. Permissions
are explicit: none implies another; empty permissions authorize nothing. Order/repetition survives.
`CommandGrantStatus` is Active or Revoked. An agent grant containing Approve returns
`AgentApprovalGrantError`, even when revoked or scoped to a different command.

`authorize_command(request, mode, trusted_source)` reads one snapshot. It requires a verified session
and an exact match to the submitted actor role, initiator and principal. An agent cannot approve,
even when all four ordinary permissions are present or the request names its human principal.
A verified human still needs an explicit approve grant for that Design, command and time.

Duplicate current grant identities refuse before selecting a successful grant; order cannot hide revocation.
For each grant, core checks actor binding, live status, required permission and complete scope in
that order. One grant must satisfy everything; permissions and scopes from separate grants cannot
be combined. `NoMatchingGrant` retains request identities, required permission and each grant's
first typed refusal in source order. Missing authentication and actor/preview/human-only failures
have distinct `CommandAuthorizationError` variants; wire diagnostics/localization remain adapters.

`CommandAuthorizationMode::Execute` requires the catalog permission.
`Preview` accepts only mutation requests and requires inspect; it cannot preview an approval or
turn inspect/propose/generate into commit.

```rust,ignore
let preview = authorize_command(&mutation_request, CommandAuthorizationMode::Preview, &trusted_host)?;
assert_eq!(preview.required_authority(), CommandAuthority::Inspect);
// Execute must obtain its own current decision with the command's required permission.
```

The returned `CommandAuthorization` borrows the exact request and records mode, permission, grant
identity and observation time. It is a snapshot decision. Execution must reauthorize at the effect
boundary and validate sources/revision, package identity, reviewed scope and evidence before effects.
No approval record, package hash or artifact is created. Native tests exercise controlled host
snapshots; they do not establish transport authentication or a completed command bus.

## Whole mutation-group authorization

authorize_mutation_group(group, mode, trusted_source) reads one fresh consistent snapshot for the
original typed CommandMutationGroup. Core authenticates its complete common actor claim and checks
all current grant identities for duplicates before selecting a grant. Each grant must cover every
original member's exact command, common Design and trusted observation time. The original
actor/status/permission/scope checks are shared with single-command authorization.

Preview requires Inspect and execution requires Commit for these typed mutations. Permissions remain
explicit: a Commit grant alone does not authorize an Inspect preview. A grant covering only one
member cannot combine with another partial grant. If an earlier partial grant fails but a later
grant covers the entire group, the complete later grant is selected.

```rust,ignore
let checked = authorize_mutation_group(&group, CommandAuthorizationMode::Preview, &trusted_host)?;
assert_eq!(checked.required_authority(), CommandAuthority::Inspect);
let original = checked.group();
let one_complete_grant = checked.grant();
```

CommandGroupAuthorization borrows the exact original group and records mode, required permission,
one grant id and the original trusted time. A compiler lifetime check rejects retaining it after its
group is destroyed. Repeated calls refresh revocation and expiry; an earlier result remains only its
old descriptive snapshot. It supplies no future execution capability or handler availability.

CommandGroupAuthorizationError::Source retains the original core unauthenticated, actor or duplicate-
grant error in its standard source chain. NoMatchingGrant preserves group, verified actor, requested
Design, required permission and each source grant's first failing zero-based member/command/refusal.
An empty grant inventory has an empty rejection vector. No candidate is staged during this check.
The [metadata group preview](command-execution.md#sequential-metadata-group-preview) separately binds
the actual model/revision, validates successive candidates and handles within-group retry aliases.
[Metadata commit](command-execution.md#atomic-in-memory-metadata-group-commit) refreshes complete Commit
authority initially and immediately before new effects. Exact result replay also refreshes authority.
Persisted replay and adapters remain their implementation owners. Controlled native source tests do
not establish transport identity.
