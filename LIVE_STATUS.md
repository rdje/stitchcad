# LIVE_STATUS.md — authoritative live progress tracker

Rows use ONLY these four states: **Done · Mostly Done · In Progress · Not Started**.
Review and update before every commit whenever actual closure or remaining scope changes;
summarize the snapshot in every commit-workflow completion message.

Detail lives elsewhere on purpose: leaf-level state in `docs/tasks/`, the defect census in
`docs/tasks/PLANNING.md`, the execution order in `docs/TASK_TREE.md`, history in `CHANGELOG.md`.
Notes cells here stay short — this is a bounded snapshot, not a journal.

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (bedrock 0.6.1) | Done | memory · task-trees · commit workflow · 12 universal + 4 project doctrine gates (`make gate`) · 29 probe suites (`make probes`) · mdBook |
| Roadmap → task-trees (`PLANNING`) | Done | All 10 lanes owned — 13 trees and linked evidence siblings, `0 unowned / 0 orphan(s) / 0 dead link(s)`, derived by `run_tree_coverage_census.sh` and watched by its probes |
| Repo identity & policy (`SPINE`) | In Progress | containment, the acceptance gates, the push cadence and the table convention are enforced or written where authors look. Open: `.5`, `.13`, `.19`, `.22`; handoff .23/.23v verified; .23r captured missing OS type; strict refusal verified |
| Adopted policy set | Done | README/claim policies and non-book containment enforced. Director2026-11-04 removes all book size/growth limits;structural checks remain |
| Defect census | In Progress | D156 audits deferred;D223/D263 owned;D285 reopen owned;D250/D293–D299/D303–D333/D335–D386 fixed;D334 P2 scheduled;D300 scope corrected;D302 recurrence fixed0767;D387 ruled2026-11-06:input checks on hold;D390 fixed0980;D391 fixed0981;D301 paired P1 owned |
| G0 — product & semantic contract | Mostly Done | Contract reviewed; D239 detached-approval boundary adopted and checked. Evaluation-seat exception remains; human closure **unapproved**. Exit census18 met/1 not met |
| G1 — executable architecture slice | Mostly Done | Exit reviewed7/7 clauses,each with stated limits;human closure unapproved. Canvas verdict scoped to macOS. Inputs/numeric/schemas;audit/undo/progress/cancel;scalar execution reviewed. Local0645/0654 and hosted224a9805 scoped |
| G2 — correct 2D slice | In Progress | Exit1/8 clauses. Kernel,offsets,corpus,canonicalizer done;DXF next, then PDF, print check, agent gate |
| G3 — construction & grading | Not Started | 14 leaves; bodice + set-in sleeve, both instantiation paths, `.rul` interchange |
| G4 — profiles & uncertainty | Not Started | 14 activities; input admission, CSP/oracle, evidence, policy matrix, HPGL, minimal Profile Editor |
| G5 — shells & validated 2D UX | Not Started | 14 leaves: Tauri + WASM shells, UX panels, parity table, native UI suite, i18n pack, tech pack |
| G6 — conformance lab & reliability | Not Started | 10 leaves: DXF import + loss report, receiver validation, plotter, pilot loop, reliability matrix, fuzzing |
| G7 — scoped production declaration | Not Started | 7 leaves: independent review, envelope statement, semver policy, upgrade/rollback, channels, governance |
| V1 — assembly visualization | Not Started | parallel, never blocks a G-gate; 7 leaves: mesh, ease-aware stitching, net-line binding, arrangement, viewport, blinded validation |
| V2 — physically validated simulation | Not Started | parallel, uncapped; 6 leaves: `sc-sim` out of the default build, XPBD research, labelled approximation, calibration + observables, evidence-gated exit |
| Product code (`crates/`) | In Progress | Scalar/Design execution,static geometry,garment inputs,Pocket/Closure operation and placement member/copy/source joins pass focused checks. Placement current references,native/cohort,physical and app/storage/release proof remain |
