# StitchCAD — a sewing CAD whose design never lives in a vendor file format

StitchCAD is pattern engineering software for garment makers. The canonical design is an abstract
parametric space — measurement tables, formulas and an ordered **construction recipe** — and a
versioned, evidence-bearing **Factory Profile** turns that design into a concrete graded **Instance**.
An artifact generator serializes instances to industry formats (DXF in AAMA and ASTM dialects,
HPGL/PLT, PDF, tech pack) as an immutable, evidence-bearing release package.

Three properties define it:

- **Uncertainty is data.** Every parameter is a known fact (with scoped evidence), an unknown fact
  (requiring observation), a selectable design choice, an overridable preference or a derived value.
  Unknown facts are never silently defaulted into geometry, and they govern what may be exported.
- **Headless-first.** The core is pure Rust libraries with no UI dependency; the CLI, the native app,
  the WASM web app and AI agents are interchangeable front-ends over one command bus.
- **Agent-drivable.** Every semantic object is reachable through a stable command API and an MCP
  façade, with scoped authority — an agent may inspect, propose, commit and generate, but approval
  stays human.

**Status:** executable Rust foundations are available; no user-facing application exists yet.
The G0 semantic contract has been reviewed, with its human closure still unapproved. G1 implements
ontology, canonical inputs, measurement metadata/tables, per-POM Ease, size membership, garment/MTM
input charts and borrowed formula syntax, whole-recipe input normalization and owned expression/
statement/recipe identity. Formula metadata, sourced declarations, initial namespaces, exact reads,
ordered scopes and operator/built-in/selector signatures include typed wanted catalogs, sourced call
lookup, bounded initial-scope expression checks, actual scope-bound statement checks and complete immutable
recipe proofs with ordered source dependencies, independently reviewed through the whole factory. [`LIVE_STATUS.md`](LIVE_STATUS.md) tracks
verified progress; the mdBook offers progressive learning, a glossary/index and detailed annexes
with implemented behavior and remaining proof boundaries.

## Audience and scope

- **For:** a patternmaker producing a signoff package for a named factory, with an AI agent as
  co-user and a sewing/factory expert authoring and reviewing profiles.
- **Explicit non-goals:** marker making, nesting and yield; costing; digitizing paper patterns;
  cutter/CAM drivers; PLM/ERP integration; body-scan made-to-measure; photorealistic or quantitatively
  validated drape. `ROADMAP.md` §1.3 is the authoritative list, and §3.2 bounds the supported garment
  envelope.

## Architecture at a glance

Crates appear as their stage starts; nothing below exists before its gate.

| Layer | Crates |
| --- | --- |
| Domain | `sc-core` (ontology, construction recipe, sewing graph, uncertainty, command bus) · `sc-measure` · `sc-units` |
| Geometry | `sc-geometry` (curves, robust predicates, offsets) · `sc-mesh` · `sc-viewport` |
| Instantiation | `sc-grading` (regeneration + grade rules) · `sc-constraints` (deterministic finite-domain CSP) · `sc-sketch` (optional local constraints) |
| Product surface | `sc-profiles` · `sc-artifacts` (canonicalizer, DXF, HPGL, PDF, tech pack) · `sc-store` |
| Front-ends | `sc-api` · `sc-mcp` · `sc-cli` · `sc-app-tauri` |

## Quick start

Requires a stable Rust toolchain (`rust-toolchain.toml`), `mdbook` for the book, and Python 3.9+
(standard library only) for repository history verification/retrieval.

```bash
git config core.hooksPath .githooks   # activate the discipline gates (once per clone)
make check                            # cargo fmt --check + clippy -D warnings + cargo test
make wasm                             # compile sc-units, sc-core, sc-measure for the browser profile
make gate                             # the doctrine enforcer
make probes                           # every diagnostic probe suite
make book                             # build the mdBook (output: docs/book/book/, untracked)
```

These commands are verified by the leaf that last touched this page; if one fails, that is a
defect — log it in a task-tree and fix it (`TOOLBOX.md` explains how to diagnose).

## Where to read next

| Question | Canonical home |
| --- | --- |
| What is this product meant to become? | [`ROADMAP.md`](ROADMAP.md) |
| What is specified and agreed so far? | [`docs/book/`](docs/book/src/SUMMARY.md) — the mdBook, the public documentation surface |
| What is being worked on right now? | [`MEMORY.md`](MEMORY.md) → [`docs/TASK_TREE.md`](docs/TASK_TREE.md) → `docs/tasks/` |
| What is finished? | [`LIVE_STATUS.md`](LIVE_STATUS.md) and [`CHANGELOG.md`](CHANGELOG.md) |
| Why was something decided this way? | [`docs/decisions/`](docs/decisions/INDEX.md) |
| How do I commit work? | [`COMMIT.md`](COMMIT.md) |
| How do I diagnose a failure? | [`TOOLBOX.md`](TOOLBOX.md) — tools first, never a guessed root cause |
| What is enforced mechanically, and how? | [`DOCTRINE_ENFORCEMENT.md`](DOCTRINE_ENFORCEMENT.md) |
| How do I know a published number is earned? | [`CLAIM_VERIFICATION.md`](CLAIM_VERIFICATION.md) |
| How does durable memory survive a lost session? | [`MEMORY_ARCHITECTURE.md`](MEMORY_ARCHITECTURE.md) |
| Why does this README stay short? | [`README_POLICY.md`](README_POLICY.md) |

Agent harnesses enter through [`CLAUDE.md`](CLAUDE.md) / [`AGENTS.md`](AGENTS.md), which point at the
same documents.

## Repository layout

```
ROADMAP.md            the product roadmap (the one file a project replaces when forked from bedrock)
MEMORY.md             the bounded resume pointer: what is next
LIVE_STATUS.md        the authoritative progress tracker
CHANGELOG.md          completed work and how it was validated
DEV_NOTES.md          engineering continuity: root cause, implementation, validation per slice
TOOLBOX.md            the tools-first diagnostic doctrine and this project's instruments
docs/book/            the mdBook: specifications, guides, reference
docs/tasks/           task-trees — every change is owned by a leaf before it is made
docs/decisions/       durable facts and decisions, one record per file
scripts/              the doctrine enforcer, its checks, and the scaffold updater
crates/               the Rust workspace (grows from G0/G1)
conformance/          golden files, pathology corpus, fixtures (grows from G2)
```

## License

`MIT OR Apache-2.0`, declared once in the workspace `Cargo.toml`. The coupling between license and
solver choice is a recorded decision, not an accident: see `docs/decisions/` for ADR-0001.

This repository was generated from the `bedrock` discipline-spine template, which is why the memory,
task-tree, commit and doctrine-enforcement machinery exists before any product code. The spine is
kept current with `scripts/update_scaffold.sh`, which syncs neutral files and refuses to overwrite
project content.
