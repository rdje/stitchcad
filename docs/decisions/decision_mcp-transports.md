# MCP connections are admitted by deployment evidence from G1

- **Type:** `decision`
- **Date:** `2026-10-24` (UTC)
- **Status:** `active` — applied as roadmap v0.4; runtime integrations remain unimplemented/unapproved.
- **Owner / source:** `G1-SLICE.9.transport.policy`; the director requested amending the stdio-only restriction
  after the connection-options discussion. The engineer authored and applied the amendment under that
  instruction. The engineer does not approve the runtime evidence it will require.

answers: "which MCP transports may StitchCAD support?" · "must HTTP or a private bridge wait for G5?" · "what admits a connection profile?"

## Context and decision

Roadmap v0.3 §7.8 restricted MCP to stdio until a G5+ threat model, while §10 permitted more after a
documented model. The phase restriction delayed useful connectivity independently of its evidence.
The director explicitly requested amending that part of the roadmap. This instruction supersedes
the earlier roadmap-edit reservation for this transport amendment only.

Roadmap v0.4 is stdio-first and names `local-stdio`, `local-http`, `remote-http` and `private-bridge`.
Local stdio remains the G1 baseline; every connection profile is eligible from G1 when its own threat
model and executable admission evidence pass. G5 owns host UI/workflow parity; it is no prerequisite
for an otherwise ready transport. G6 still hardens security and reliability.

Admission covers caller identity/authorization, project and filesystem scopes, credentials and
revocation where applicable, binding/origin rules, resource bounds, injected data, artifact overwrites,
disconnect/retry and command-state parity. Core authority, actor trace, revision/idempotency/undo
semantics and human-only approval bind every adapter. Network/relay access is explicit opt-in.
A relay's inner stdio hop does not remove its remote caller from the trust model.

## Consequences and verification

- The command-layer chapter specifies the profiles and their named implementation/evidence owners.
  Its census compares profile declarations with the roadmap, checks ownership and refuses retired
  normative timing rules; copied fixtures and actual checker-body faults calibrate the guards.
- The amendment author applied the policy and may not certify its implementation. Every runtime
  profile remains unapproved until its owner's evidence exists and the applicable reviewer accepts it.
- Host protocol support is distinct from StitchCAD implementation and directory eligibility. Private
  tunnel access and public plugin distribution require separate compatibility/access checks.
- Supersedes the G5+ transport timing in decision_command-layer-contract-and-undo-granularity.md;
  the other command, undo, authority and parity decisions remain in force.
- Reopen when a required host cannot use these profiles, deployment evidence exposes an unmet trust
  boundary, or a protocol change requires another profile. Update the roadmap disposition and linked
  contracts together. A failed admission test keeps its profile disabled, not silently permitted.

## External observations

Official documentation fetched on 2026-10-24 describes local stdio/Streamable HTTP clients, private
bridging to stdio/HTTP, and separate public-submission requirements. These motivate the options;
they certify no StitchCAD connection. Reverify host/version and access before implementation:

- https://learn.chatgpt.com/docs/extend/mcp?surface=cli
- https://developers.openai.com/api/docs/guides/secure-mcp-tunnels
- https://developers.openai.com/plugins/deploy/desktop-only

Normative homes: ROADMAP.md §7.8/§10; docs/book/src/spec/command-layer.md §7.1.
