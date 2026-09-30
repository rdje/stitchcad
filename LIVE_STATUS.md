# LIVE_STATUS.md — authoritative live progress tracker

Rows use ONLY these four states: **Done · Mostly Done · In Progress · Not Started**.
Review and update before every commit whenever actual closure or remaining scope changes;
summarize the snapshot in every commit-workflow completion message.

Detail lives elsewhere on purpose: leaf-level state in `docs/tasks/`, the defect census in
`docs/tasks/PLANNING.md`, the execution order in `docs/TASK_TREE.md`, history in `CHANGELOG.md`.
Notes cells here stay short — this is a bounded snapshot, not a journal.

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (bedrock 0.6.1) | Done | memory · task-trees · commit workflow · 13 universal + 2 project doctrine gates · 7 probe suites · mdBook |
| Roadmap → task-trees (`PLANNING`) | Done | All 10 roadmap lanes owned: 13 trees, 142 leaves, derived by `run_tree_coverage_census.sh` |
| Repo identity & policy (`SPINE`) | In Progress | 17 of 20 leaves done; containment, the acceptance gates and the push cadence are enforced and derived. Remaining `.5`/`.13`/`.15` block no product work |
| Adopted policy set | Done | README policy, claim verification and the containment doctrine are in-repo, with containment **enforced** by the `LIVE-DOC-SIZE` project doctrine |
| Defect census | In Progress | 27 logged, 23 closed. Open: D7/D9 (`SPINE.5`), D22 (`SPINE.15`), D27 (`G0-CONTRACT.14`), D10 (verify at `G1-SLICE.1`) |
| G0 — product & semantic contract | In Progress | **Active lane.** `.2`, `.3`, `.13`, `.13b`, `.18` done: contract, ontology, fixture (tokens declared, waistband disputed → D27). Next `.1` glossary |
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
