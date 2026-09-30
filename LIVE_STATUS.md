# LIVE_STATUS.md — authoritative live progress tracker

Rows use ONLY these four states: **Done · Mostly Done · In Progress · Not Started**.
Review and update before every commit whenever actual closure or remaining scope changes;
summarize the snapshot in every commit-workflow completion message.

Detail lives elsewhere on purpose: leaf-level state in `docs/tasks/`, the defect census in
`docs/tasks/PLANNING.md`, the execution order in `docs/TASK_TREE.md`, history in `CHANGELOG.md`.
Notes cells here stay short — this is a bounded snapshot, not a journal.

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (bedrock 0.6.1) | Done | memory · task-trees · commit workflow · 12 universal + 3 project doctrine gates (`make gate`) · 19 probe suites (`make probes`) · mdBook |
| Roadmap → task-trees (`PLANNING`) | Done | All 10 lanes owned — 13 trees, 3 evidence siblings, `0 unowned / 0 orphan(s) / 0 dead link(s)`, derived by `run_tree_coverage_census.sh` and watched by its probes |
| Repo identity & policy (`SPINE`) | In Progress | containment, the acceptance gates, the push cadence and the table convention are enforced or written where authors look. Open: `.5`, `.13`, `.19` |
| Adopted policy set | Done | README policy, claim verification and containment are in-repo; containment is **enforced** by `LIVE-DOC-SIZE`, with revision-aware baselines and table-shape targets |
| Defect census | In Progress | open: D34 + D38 (`PLANNING.5`), D35 (`G1-SLICE.3`), D40 + D46 + D51 (`SPINE.19`), D49 (`SPINE.5`). The logged/closed counts are hand-kept and underivable, which is D38 |
| G0 — product & semantic contract | In Progress | **Active lane.** ADR-0001, -0003, -0004 and §9's release contract land with the glossary, units, ontology, formula language, envelope, paths, sizes, standards and fixture. Next `.16` |
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
