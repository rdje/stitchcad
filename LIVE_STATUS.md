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
| Roadmap → task-trees (`PLANNING`) | In Progress | `.1`, `.2` done: 7 trees, 87 leaves. `.3` owes G5–G7, V1, V2 + coverage census |
| Repo identity & policy (`SPINE`) | In Progress | 11 of 17 leaves done. `.4.2` landed the containment data plane (17 surfaces, 15 routes, derived README caps). Next `.4.3` checker, `.5` toolbox/map, `.13`–`.15` containment debt |
| Adopted policy set | Mostly Done | README policy, claim verification, containment doctrine + data plane in-repo; enforcement owed by `.4.3` |
| Defect census | In Progress | 21 logged, 16 closed. Open: D7/D9 (`SPINE.5`), D13 (`SPINE.4.3`), D10 (`G1-SLICE.1`), D22 (`SPINE.15`) |
| G0 — product & semantic contract | Not Started | 18 leaves mapped to the gate's exit clauses; spec only, except `.18` (skeleton crates + CI) |
| G1 — executable architecture slice | Not Started | 16 leaves; three runtime profiles, command bus, persistence, CSP, API/MCP, spikes |
| G2 — correct 2D slice | Not Started | 14 leaves; offsets + pathology corpus, canonicalizer, DXF/PDF, print check, agent gate |
| G3 — construction & grading | Not Started | 14 leaves; bodice + set-in sleeve, both instantiation paths, `.rul` interchange |
| G4 — profiles & uncertainty | Not Started | 14 leaves; CSP + oracle, evidence store, policy matrix, HPGL, minimal Profile Editor |
| G5 — shells & validated 2D UX | Not Started | tree owed by `PLANNING.3` |
| G6 — conformance lab & reliability | Not Started | tree owed by `PLANNING.3` |
| G7 — scoped production declaration | Not Started | tree owed by `PLANNING.3` |
| V1 — assembly visualization | Not Started | parallel track; tree owed by `PLANNING.3` |
| V2 — physically validated simulation | Not Started | parallel, uncapped; tree owed by `PLANNING.3` |
| Product code (`crates/`) | Not Started | bedrock starter crate only; roadmap §4.3 crates appear at G0/G1 |
