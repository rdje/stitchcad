# A complete standalone UI is the baseline; every admitted surface meets the production quality bar

- **Type:** `decision`
- **Date:** `2026-10-25`
- **Status:** `active`; implementation remains planned
- **Owner / source:** director DBIQNP discussion and explicit agreement on2026-10-25;
  G1-SLICE.9.plugin, G1-SLICE.13, G5-SHELLS and V1-ASSEMBLY own implementation/evidence.

answers: "does StitchCAD need its own UI?" · "will chat UI limits restrict StitchCAD?" · "what UI quality is required?" · "which UI platforms are planned?" · "is the UI stack already final?" · "can AI coding agents control the desktop/web/graphics UI?"

## Adopted direction

A complete standalone StitchCAD interface is the full-feature product baseline. Desktop, web and
chat surfaces share the Rust domain core, command contracts, identities/revisions and authority.
Chat integrations may provide embedded previews and controls; host restrictions lead to a documented
fallback or opening the complete product view. They do not silently remove required workflows.

Every admitted platform shall be sleek, smooth, beautiful, efficient, engaging and precise,
state-of-the-art and production-grade. Demonstrate this through a coherent visual system,
typography/spacing/hierarchy, accessible and readable units/states, smooth direct manipulation,
efficient keyboard/snapping/selection/undo/recovery, and clear precision/commit feedback. Measure
interaction latency and frame pacing on representative CAD projects and declared hardware.
Use design review and actual workflows to establish the bar; no unmeasured performance claim is made.

## Full control by AI coding agents — critical

The director explicitly requires every desktop, web and2D/3D graphics environment to be fully
controllable by AI coding agents. Full product workflows must be discoverable, callable and
verifiable through supported API/MCP/control adapters. Include transient presentation/view state
as well as domain/project changes: camera, pan/zoom, picking/selection, layers/overlays, tool modes,
measurement/editing, files/export, undo/redo, cancellation and recovery.

Agents need inspectable document/view state, stable object and view identities, valid-action
constraints and completion/diagnostic events, plus visual/artifact evidence when relevant. A
supported operation reachable only through an unexposed GUI action does not meet this requirement.
Share the domain command contracts; expose presentation controls through the applicable typed
client adapters without duplicating domain logic in the shell. Existing authority, revision and
human-only approval contracts remain in effect.

G1-SLICE.9/.13, G5 and V1 own their applicable contracts and end-to-end agent workflow parity.
Verify real desktop/web/2D/3D operation, including error recovery and result inspection, for each
admitted surface. This requirement is adopted now; no current UI/control capability is claimed.

## Existing platform plan and admission

ROADMAP.md ADR-0002 recommends Tauri with TypeScript/React chrome and a shared Rust core. G5 plans
native Windows/macOS/Linux plus a WASM web application with an explicit capability matrix. The
canvas topology remains subject to the realistic native/browser G1 spike; no new framework or
renderer is selected by this decision. G5 owns full2D UX and V1 owns staged3D assembly.

Official documentation reviewed2026-10-25 establishes interactive host surfaces:

- OpenAI: [MCP web components](https://developers.openai.com/plugins/build/app-quickstart) and
  [fullscreen canvases/host UI policy](https://developers.openai.com/plugins/build/chatgpt-ui).
- Anthropic: [sandboxed interactive connectors](https://support.claude.com/en/articles/13454812-use-interactive-connectors-in-claude)
  and a listed [Three.js3D viewer](https://claude.com/marketplace/connectors/threejs-3d-viewer).

These examples are capability evidence for their documented hosts, not measured StitchCAD renderer,
GPU, input/focus, persistence, large-project performance or workflow-parity evidence. Re-verify the
actual client/version during the relevant host spike. Native/web capability profiles remain explicit;
the quality bar applies within every admitted profile.

## Scheduling

The director invoked DBIQNP: record and discuss without pivoting. Explicit agreement adopts this
architectural/quality direction; it does not start UI implementation, reorder the roadmap or interrupt
the current numerical feature work. The mdBook command-layer UI section presents this decision.
