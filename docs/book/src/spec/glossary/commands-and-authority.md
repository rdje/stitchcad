# Glossary: commands, front-ends and agent authority

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection. Every term here is specified by
> the [command layer](../command-layer.md) chapter.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| actor | who initiated a mutating command — a named human, or an agent acting for one | [command layer §5](../command-layer.md) | initiator, principal, caller | `actor` |
| agent authority level ⚠ | one of five scoped permissions: inspect, propose, commit, generate, approve | [command layer §7](../command-layer.md) | permission level, capability scope | → `inspect` · `propose` · `commit` · `generate` · `approve` |
| approve (authority) ⚠ | the authority level only a human holds; no graph mutation can manufacture it | [command layer §7](../command-layer.md) | signoff authority, release right | `approve` |
| atomic group | a set of commands that commits together or not at all, and undoes as one step | [command layer §3](../command-layer.md) | transaction, batch, unit of work | — |
| audit trail | the recorded sequence of mutating commands with their actors | [command layer §5](../command-layer.md) | command log, history log | — |
| command | one typed, validated mutation or evaluation; the only path by which anything changes | [command layer §2](../command-layer.md) | operation, action, request | typed command name |
| command bus | the single surface UI, CLI and MCP all sit on, so behaviour cannot diverge per front-end | [command layer §1](../command-layer.md) | command layer, core API | — |
| command class | one of five partitions of the command set, which fixes a command's authority, its reversibility and its undo granularity | [command layer §1](../command-layer.md) | command kind, category | — |
| commit (authority) | the authority to apply a proposed change to the design, short of releasing it | [command layer §7](../command-layer.md) | apply right, write access | `commit` |
| front-end adapter | a UI, CLI or MCP surface that translates user intent into commands and nothing else | [command layer §1](../command-layer.md) | shell, client, surface | — |
| generate (authority) | the authority to produce artifacts from an already-committed design | [command layer §7](../command-layer.md) | export right, build access | `generate` |
| idempotency | applying the same command twice has the same effect as applying it once | [command layer §5](../command-layer.md) | replay safety, once-semantics | — |
| idempotency key | the token a mutating command carries so a transport retry applies once and a replay is reported as a replay | [command layer §5](../command-layer.md) | request key, dedup token | — |
| inspect (authority) | the authority to read anything, which no other authority implies | [command layer §7](../command-layer.md) | read access, query right | `inspect` |
| MCP | the agent-facing façade over the same commands, stdio-only until a threat model says otherwise | [command layer §7](../command-layer.md) | Model Context Protocol, agent surface | — |
| preview / commit ⚠ | the two-phase shape of a mutation: see the result, then apply it or drop it | [command layer §4](../command-layer.md) | dry run then apply, trial and commit | — |
| progress | a reported position in a long operation, so a caller is never left guessing | [command layer §6](../command-layer.md) | progress report, percent done | — |
| propose (authority) | the authority to prepare a change for a human to accept, without applying it | [command layer §7](../command-layer.md) | suggest right, draft access | `propose` |
| revision precondition ⚠ | a command's declaration of the revision it is valid against; a stale revision is refused | [command layer §5](../command-layer.md) | optimistic lock, version guard | → `revision` |
| undo/redo granularity | what one undo reverses: a command group, never a half-applied group | [command layer §3](../command-layer.md) | undo step, history granularity | — |
| vacant seat ⚠ | a role nobody holds, where the authority is competence rather than office, so no acting holder may exercise it | [governance §8.1](../../governance.md) | unfilled role, reviewer not named, *unbesetzt* | `vacant` |
| workflow parity ⚠ | the invariant that UI, API and MCP can each complete the same workflows, proved by a table | [command layer §8](../command-layer.md) | front-end parity, feature parity | — |
| workflow registry | the declared list of named command sequences the parity table is generated from, so no adapter's coverage is written by hand | [command layer §8](../command-layer.md) | workflow list, parity source | — |
