# Glossary: commands, front-ends and agent authority

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection. Every term here is specified by
> `G0-CONTRACT.17` (the command layer); until that chapter lands, the roadmap clause is cited.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| actor | who initiated a mutating command — a named human, or an agent acting for one | roadmap §10 · specified by `G0-CONTRACT.17` | initiator, principal, caller | `actor` |
| agent authority level ⚠ | one of five scoped permissions: inspect, propose, commit, generate, approve | roadmap §7.8 · specified by `G0-CONTRACT.17` | permission level, capability scope | → `inspect` · `propose` · `commit` · `generate` · `approve` |
| approve (authority) ⚠ | the authority level only a human holds; no graph mutation can manufacture it | roadmap §7.8 · specified by `G0-CONTRACT.17` | signoff authority, release right | `approve` |
| atomic group | a set of commands that commits together or not at all, and undoes as one step | roadmap §4.4 · specified by `G0-CONTRACT.17` | transaction, batch, unit of work | — |
| audit trail | the recorded sequence of mutating commands with their actors | roadmap §10 · specified by `G0-CONTRACT.17` | command log, history log | — |
| command | one typed, validated mutation or evaluation; the only path by which anything changes | roadmap §4.4 · specified by `G0-CONTRACT.17` | operation, action, request | typed command name |
| command bus | the single surface UI, CLI and MCP all sit on, so behaviour cannot diverge per front-end | roadmap §4.4 · specified by `G0-CONTRACT.17` | command layer, core API | — |
| commit (authority) | the authority to apply a proposed change to the design, short of releasing it | roadmap §7.8 · specified by `G0-CONTRACT.17` | apply right, write access | `commit` |
| front-end adapter | a UI, CLI or MCP surface that translates user intent into commands and nothing else | roadmap §4.4 · specified by `G0-CONTRACT.17` | shell, client, surface | — |
| generate (authority) | the authority to produce artifacts from an already-committed design | roadmap §7.8 · specified by `G0-CONTRACT.17` | export right, build access | `generate` |
| idempotency | applying the same command twice has the same effect as applying it once | roadmap §4.4 · specified by `G0-CONTRACT.17` | replay safety, once-semantics | — |
| inspect (authority) | the authority to read anything, which no other authority implies | roadmap §7.8 · specified by `G0-CONTRACT.17` | read access, query right | `inspect` |
| MCP | the agent-facing façade over the same commands, stdio-only until a threat model says otherwise | roadmap §7.8 · specified by `G0-CONTRACT.17` | Model Context Protocol, agent surface | — |
| preview / commit ⚠ | the two-phase shape of a mutation: see the result, then apply it or drop it | roadmap §4.4 · specified by `G0-CONTRACT.17` | dry run then apply, trial and commit | — |
| progress | a reported position in a long operation, so a caller is never left guessing | roadmap §4.4 · specified by `G0-CONTRACT.17` | progress report, percent done | — |
| propose (authority) | the authority to prepare a change for a human to accept, without applying it | roadmap §7.8 · specified by `G0-CONTRACT.17` | suggest right, draft access | `propose` |
| revision precondition ⚠ | a command's declaration of the revision it is valid against; a stale revision is refused | roadmap §4.4 · specified by `G0-CONTRACT.17` | optimistic lock, version guard | → `revision` |
| undo/redo granularity | what one undo reverses: a command group, never a half-applied group | roadmap §4.4 · specified by `G0-CONTRACT.17` | undo step, history granularity | — |
| vacant seat ⚠ | a role nobody holds, where the authority is competence rather than office, so no acting holder may exercise it | [governance §8.1](../../governance.md) | unfilled role, reviewer not named, *unbesetzt* | `vacant` |
| workflow parity ⚠ | the invariant that UI, API and MCP can each complete the same workflows, proved by a table | roadmap §4.4 · specified by `G0-CONTRACT.17` | front-end parity, feature parity | — |
