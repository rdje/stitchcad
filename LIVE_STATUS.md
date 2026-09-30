# LIVE_STATUS.md — authoritative live progress tracker

Rows use ONLY these four states: **Done · Mostly Done · In Progress · Not Started**.
Review and update before every commit whenever actual closure or remaining scope changes;
summarize the snapshot in every commit-workflow completion message.

Detail lives elsewhere on purpose: leaf-level state in `docs/tasks/`, the defect census in
`docs/tasks/PLANNING.md`, the execution order in `docs/TASK_TREE.md`, history in `CHANGELOG.md`.
Notes cells here stay short — this is a bounded snapshot, not a journal.

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (bedrock 0.6.1) | Done | memory · task-trees · commit workflow · 12 universal + 2 project doctrine gates (`make gate`) · 12 probe suites (`make probes`) · mdBook |
| Roadmap → task-trees (`PLANNING`) | Done | All 10 roadmap lanes owned — 13 trees, 2 evidence siblings, `0 unowned / 0 orphan(s) / 0 dead link(s)`, derived by `run_tree_coverage_census.sh` and watched by its probe suite |
| Repo identity & policy (`SPINE`) | In Progress | containment (its targets now derived per table shape), the acceptance gates and the push cadence are enforced and derived. Open: `.5`, `.13`, `.15`, `.19` — none blocks product work |
| Adopted policy set | Done | README policy, claim verification and the containment doctrine are in-repo, with containment **enforced** by the `LIVE-DOC-SIZE` project doctrine; its debt baselines are revision-aware and its table-shape targets are derived |
| Defect census | In Progress | 41 logged, 36 closed — hand-kept, which is D38. Open: D22 (`SPINE.15`), D34 + D38 (`PLANNING.5`), D35 (`G1-SLICE.3`), D40 (`SPINE.19`) |
| G0 — product & semantic contract | In Progress | **Active lane.** `.1`–`.8`, `.13`/`.13b`/`.13c`/`.13d`, `.4b`, `.4c`, `.14`, `.14b`, `.18` done: glossary, units, ontology, envelope (proved by `ROADMAP.md` v0.3), paths, sizes, standards, ADR-0001, fixture, governance, `sc-units`. Next `.9` |
| G1 — executable architecture slice | Not Started | 16 leaves; three runtime profiles, command bus, persistence, CSP, API/MCP, spikes |
| G2 — correct 2D slice | Not Started | 14 leaves; offsets + pathology corpus, canonicalizer, DXF/PDF, print check, agent gate |
| G3 — construction & grading | Not Started | 14 leaves; bodice + set-in sleeve, both instantiation paths, `.rul` interchange |
| G4 — profiles & uncertainty | Not Started | 14 leaves; CSP + oracle, evidence store, policy matrix, HPGL, minimal Profile Editor |
| G5 — shells & validated 2D UX | Not Started | 14 leaves: Tauri + WASM shells, UX panels, parity table, native UI suite, i18n pack, tech pack |
| G6 — conformance lab & reliability | Not Started | 10 leaves: DXF import + loss report, receiver validation, plotter, pilot loop, reliability matrix, fuzzing |
| G7 — scoped production declaration | Not Started | 7 leaves: independent review, envelope statement, semver policy, upgrade/rollback, channels, governance |
| V1 — assembly visualization | Not Started | parallel, never blocks a G-gate; 7 leaves: mesh, ease-aware stitching, net-line binding, arrangement, viewport, blinded validation |
| V2 — physically validated simulation | Not Started | parallel, uncapped; 6 leaves: `sc-sim` out of the default build, XPBD research, labelled approximation, calibration + observables, evidence-gated exit |
| Product code (`crates/`) | In Progress | `sc-units` implements the numerical contract (30 tests, no deps, WASM build green); `sc-core` skeleton; starter crate retired |
