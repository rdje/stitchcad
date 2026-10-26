# Exact command scopes

`CommandScope` is immutable Design/command/time policy for [core authority](command-layer.md#7-agent-authority-as-a-command-layer-concept).
It grants no permission and authenticates nobody.

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
