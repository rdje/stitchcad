# LIVE_STATUS.md — authoritative live progress tracker

Rows use ONLY these four states: **Done · Mostly Done · In Progress · Not Started**.
Review and update before every commit whenever actual closure or remaining scope changes;
summarize the snapshot in every commit-workflow completion message.

Detail lives elsewhere on purpose: leaf-level state in `docs/tasks/`, the defect census in
`docs/tasks/PLANNING.md`, the execution order in `docs/TASK_TREE.md`, history in `CHANGELOG.md`.
Notes cells here stay short — this is a bounded snapshot, not a journal.

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (bedrock 0.6.1) | Done | memory · task-trees · commit workflow · 12 universal + 4 project doctrine gates (`make gate`) · 25 probe suites (`make probes`) · mdBook |
| Roadmap → task-trees (`PLANNING`) | Done | All 10 lanes owned — 13 trees, 8 evidence siblings, `0 unowned / 0 orphan(s) / 0 dead link(s)`, derived by `run_tree_coverage_census.sh` and watched by its probes |
| Repo identity & policy (`SPINE`) | In Progress | containment, the acceptance gates, the push cadence and the table convention are enforced or written where authors look. Open: `.5`, `.13`, `.19`, `.22` |
| Adopted policy set | Done | README policy, claim verification and containment are in-repo; containment is **enforced** by `LIVE-DOC-SIZE`, with revision-aware baselines and table-shape targets |
| Defect census | In Progress | 12 open, 87 sealed. Open: D34 + D38 (`PLANNING.5`), D40 + D46 + D51 (`SPINE.19`), D49 (`SPINE.5`), D53 + D54 (`SPINE.4`), D67 (`SPINE.22`), D70 (`G1-SLICE.4c.2`), D84 + D100 (`G1-SLICE.5a.3b.3c`). Counts derive from live census and reconstructed history |
| G0 — product & semantic contract | Mostly Done | Every clause met and derived except evaluation-seat procurement, accepted open by ruling; closure **unapproved** (governance §6.1). `run_g0_exit_review.sh` → `18 met / 1 not met` |
| G1 — executable architecture slice | In Progress | 5 of 18 top-level leaves done; 4 structural families done; geometry/recipe/profile/release pending; expression syntax done; numeric reference review done; signed-angle/book-replay controls verified; next `.5a.3b.3c.3` review; axes D70 pending |
| G2 — correct 2D slice | Not Started | 14 leaves; offsets + pathology corpus, canonicalizer, DXF/PDF, print check, agent gate |
| G3 — construction & grading | Not Started | 14 leaves; bodice + set-in sleeve, both instantiation paths, `.rul` interchange |
| G4 — profiles & uncertainty | Not Started | 14 leaves; CSP + oracle, evidence store, policy matrix, HPGL, minimal Profile Editor |
| G5 — shells & validated 2D UX | Not Started | 14 leaves: Tauri + WASM shells, UX panels, parity table, native UI suite, i18n pack, tech pack |
| G6 — conformance lab & reliability | Not Started | 10 leaves: DXF import + loss report, receiver validation, plotter, pilot loop, reliability matrix, fuzzing |
| G7 — scoped production declaration | Not Started | 7 leaves: independent review, envelope statement, semver policy, upgrade/rollback, channels, governance |
| V1 — assembly visualization | Not Started | parallel, never blocks a G-gate; 7 leaves: mesh, ease-aware stitching, net-line binding, arrangement, viewport, blinded validation |
| V2 — physically validated simulation | Not Started | parallel, uncapped; 6 leaves: `sc-sim` out of the default build, XPBD research, labelled approximation, calibration + observables, evidence-gated exit |
| Product code (`crates/`) | In Progress | `sc-units`: 40 tests; `sc-core`: ontology/canonical inputs/borrowed lexing/expression syntax; `sc-measure`: metadata/tables/Ease/size members/charts/MTM. Native/WASM green locally; older metadata CI observed; truth/physical/release proof deferred |
