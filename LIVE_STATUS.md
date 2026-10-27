# LIVE_STATUS.md — authoritative live progress tracker

Rows use ONLY these four states: **Done · Mostly Done · In Progress · Not Started**.
Review and update before every commit whenever actual closure or remaining scope changes;
summarize the snapshot in every commit-workflow completion message.

Detail lives elsewhere on purpose: leaf-level state in `docs/tasks/`, the defect census in
`docs/tasks/PLANNING.md`, the execution order in `docs/TASK_TREE.md`, history in `CHANGELOG.md`.
Notes cells here stay short — this is a bounded snapshot, not a journal.

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (bedrock 0.6.1) | Done | memory · task-trees · commit workflow · 12 universal + 4 project doctrine gates (`make gate`) · 28 probe suites (`make probes`) · mdBook |
| Roadmap → task-trees (`PLANNING`) | Done | All 10 lanes owned — 13 trees and linked evidence siblings, `0 unowned / 0 orphan(s) / 0 dead link(s)`, derived by `run_tree_coverage_census.sh` and watched by its probes |
| Repo identity & policy (`SPINE`) | In Progress | containment, the acceptance gates, the push cadence and the table convention are enforced or written where authors look. Open: `.5`, `.13`, `.19`, `.22`; handoff .23/.23v verified; .23r captured missing OS type; strict refusal verified |
| Adopted policy set | Done | README policy, claim verification and containment are in-repo; containment is **enforced** by `LIVE-DOC-SIZE`, with revision-aware baselines and table-shape targets |
| Defect census | In Progress | D156/D223/D263 audits and D250 vocabulary owned; finite generation component acceptance complete. D224/D225/D227/D229-D236/D245-D249/D251-D272 corrected; D70/D226/D228 ruled and implemented within their scopes. Canonical review owners remain tracked |
| G0 — product & semantic contract | Mostly Done | Contract reviewed; D239 detached-approval boundary adopted and checked. Evaluation-seat exception remains; human closure **unapproved**. Earlier exit census18 met/1 not met |
| G1 — executable architecture slice | In Progress | 6/18 leaves done. Structural inputs, numeric/read foundations, all17 command schemas and trusted-host core authorization available. Full evaluator/bus/storage/app pending; focused checks, integration/release proof later |
| G2 — correct 2D slice | Not Started | 14 leaves; offsets + pathology corpus, canonicalizer, DXF/PDF, print check, agent gate |
| G3 — construction & grading | Not Started | 14 leaves; bodice + set-in sleeve, both instantiation paths, `.rul` interchange |
| G4 — profiles & uncertainty | Not Started | 14 leaves; CSP + oracle, evidence store, policy matrix, HPGL, minimal Profile Editor |
| G5 — shells & validated 2D UX | Not Started | 14 leaves: Tauri + WASM shells, UX panels, parity table, native UI suite, i18n pack, tech pack |
| G6 — conformance lab & reliability | Not Started | 10 leaves: DXF import + loss report, receiver validation, plotter, pilot loop, reliability matrix, fuzzing |
| G7 — scoped production declaration | Not Started | 7 leaves: independent review, envelope statement, semver policy, upgrade/rollback, channels, governance |
| V1 — assembly visualization | Not Started | parallel, never blocks a G-gate; 7 leaves: mesh, ease-aware stitching, net-line binding, arrangement, viewport, blinded validation |
| V2 — physically validated simulation | Not Started | parallel, uncapped; 6 leaves: `sc-sim` out of the default build, XPBD research, labelled approximation, calibration + observables, evidence-gated exit |
| Product code (`crates/`) | In Progress | Units, ontology, measurements, formula/numeric APIs, command schemas, Design input/topology/sewing components and Pocket/notion/Closure/layer/Hem collections and borrowed offset intent available. Full execution/app/physical/release proof pending; focused native proof |
