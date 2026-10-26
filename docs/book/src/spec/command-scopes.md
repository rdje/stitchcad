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

Actor/principal identity, human delegation, permissions/revocation and current-source/revision checks
remain required before effects. Transport authentication, preview/commit and human approval follow.
