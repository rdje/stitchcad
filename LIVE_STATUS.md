# LIVE_STATUS.md — authoritative live progress tracker

Rows use ONLY these four states: **Done · Mostly Done · In Progress · Not Started**.
Review and update before every commit whenever actual closure or remaining scope changes;
summarize the snapshot in every commit-workflow completion message.

Detail lives elsewhere on purpose: leaf-level state in `docs/tasks/`, the defect census in
`docs/tasks/PLANNING.md`, the execution order in `docs/TASK_TREE.md`, history in `CHANGELOG.md`.
Notes cells here stay short — this is a bounded snapshot, not a journal.

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (bedrock 0.6.1) | Done | memory · task-trees · commit workflow · doctrine gates · mdBook |
| Roadmap → task-trees (`PLANNING`) | Mostly Done | **All 10 roadmap lanes owned** — 13 trees, 142 leaves, derived by `run_tree_coverage_census.sh` (`0 unowned / 0 orphans / 0 dead links`). `.4` owes the sequencing-rule record (D24) |
| Repo identity & policy (`SPINE`) | In Progress | 13 of 17 leaves done: identity, policies, containment data plane, changelog sealed to `docs/history/`. Containment is now **enforced** (`LIVE-DOC-SIZE`, 2nd project doctrine). `.5`/`.13`/`.15` remain and block no product work |
| Adopted policy set | Mostly Done | README policy, claim verification, containment doctrine + data plane in-repo; enforcement owed by `.4.3` |
| Defect census | In Progress | 22 logged, 18 closed. Open: D7/D9 (`SPINE.5`), D10 (`G1-SLICE.1`), D22 (`SPINE.15`) |
| G0 — product & semantic contract | Not Started | 18 leaves mapped to the gate's exit clauses; spec only, except `.18` (skeleton crates + CI) |
| G1 — executable architecture slice | Not Started | 16 leaves; three runtime profiles, command bus, persistence, CSP, API/MCP, spikes |
| G2 — correct 2D slice | Not Started | 14 leaves; offsets + pathology corpus, canonicalizer, DXF/PDF, print check, agent gate |
| G3 — construction & grading | Not Started | 14 leaves; bodice + set-in sleeve, both instantiation paths, `.rul` interchange |
| G4 — profiles & uncertainty | Not Started | 14 leaves; CSP + oracle, evidence store, policy matrix, HPGL, minimal Profile Editor |
| G5 — shells & validated 2D UX | Not Started | 14 leaves: Tauri + WASM shells, UX panels, parity table, native UI suite, i18n pack, tech pack |
| G6 — conformance lab & reliability | Not Started | 10 leaves: DXF import + loss report, receiver validation, plotter, pilot loop, reliability matrix, fuzzing |
| G7 — scoped production declaration | Not Started | 7 leaves: independent review, envelope statement, semver policy, upgrade/rollback, channels, governance |
| V1 — assembly visualization | Not Started | parallel, never blocks a G-gate; 7 leaves: mesh, ease-aware stitching, net-line binding, arrangement, viewport, blinded validation |
| V2 — physically validated simulation | Not Started | parallel, uncapped; 6 leaves: `sc-sim` out of the default build, XPBD research, labelled approximation, calibration + observables, evidence-gated exit |
| Product code (`crates/`) | Not Started | bedrock starter crate only; roadmap §4.3 crates appear at G0/G1 |
