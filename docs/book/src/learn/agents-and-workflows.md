# Working with agents

An external agent should be able to inspect a StitchCAD instance, understand its capabilities, author
a design, inspect diagnostics and prepare checked outputs through the application API and MCP server.
That is a project requirement, and [the API and MCP server](../spec/api-and-mcp.md) now do this for a drafted garment, over local stdio, and [two independent agent sessions](../spec/api-and-mcp.md) have drafted the reference skirt that way. Network connections are not built.

## One workflow, several front-ends

The planned native app, browser app, CLI and agents use the same semantic commands. A command states
what operation is intended, which revision it expects and which authority permits it. This lets a
caller reason about changes without interpreting screenshots or relying on hidden UI state.

For example, an agent preparing a skirt should be able to inspect its measurement inputs, identify
an unresolved hip measurement, explain what observation is missing, preview a proposed change and
commit that change under its permitted authority. It should then inspect the resulting diagnostics.
[`preview_change` and `change_measurement`](../spec/api-and-mcp.md) do exactly this today. What an unresolved measurement means is richer in the model than these tools show yet.

## Control needs observable results

Capabilities, uncertainty, errors, current references and repair work must be discoverable through
structured data. Revision checks, atomic operations, recovery and undo matter because an agent can
be interrupted or act on stale information. The command/API contracts and later parity tests own
these behaviors; fluent sewing advice alone does not establish technical control or domain expertise.

## Approval remains deliberate

Inspection, proposing, committing and generating are separate permissions. Human approval is a
separate capability: an agent can prepare evidence but cannot approve its own garment, factory
compatibility claim or production release. [Core authorization](../spec/command-scopes.md#core-authorization)
rejects agent approval, and [the server](../spec/api-and-mcp.md) cannot be started otherwise. Authenticated network adapters and actual release handlers remain planned.

There is no current MCP connection to configure. Check [availability](../availability.md) for the
implementation frontier. Experts can use the [command-layer contract](../spec/command-layer.md),
[release contract](../spec/release-contract.md) and [governance annex](../governance.md) directly.
The [topic index](../topic-index.md) is the shortest route to any reference in this book.
