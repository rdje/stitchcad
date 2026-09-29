# LIVE_STATUS.md — authoritative live progress tracker

Rows use ONLY these four states: **Done · Mostly Done · In Progress · Not Started**.
Review and update before every commit whenever actual closure or remaining scope changes;
summarize the snapshot in every commit-workflow completion message.

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (bedrock 0.6.1) | Done | memory architecture · task-trees · commit workflow · doctrine enforcement · mdBook skeleton |
| Roadmap → task-trees (`PLANNING`) | In Progress | `.1`, `.2` done (index repaired; `PLANNING`/`SPINE`/`G0-CONTRACT`/`G1-SLICE`/`G2-2D`/`G3-GRADING`/`G4-PROFILES` seeded — 87 leaves across 7 trees; 15 defects logged and owned). `.3` owes `G5`–`G7`, `V1`, `V2` + the coverage map |
| Repo identity & adopted policy (`SPINE`) | In Progress | `SPINE.6` done (lockfile tracked). Next: `.7`/`.8` — prove and locally close defect D15 in the inherited acceptance gate; then README/book de-template, cleanup cadence, policy adoptions, containment, toolbox |
| G0 — product & semantic contract (`G0-CONTRACT`) | Not Started | 18 leaves mapped clause-by-clause to the gate's exit criteria; specification only, except `.18` (skeleton crates + G0 CI) |
| G1 — executable architecture slice (`G1-SLICE`) | Not Started | 16 leaves; owns the starter-crate replacement (defect D10) and the three runtime profiles |
| G2 — correct 2D slice (`G2-2D`) | Not Started | 14 leaves; offset pathology corpus, canonicalizer, DXF/PDF, printed scale square, agent gate |
| G3 — construction & grading (`G3-GRADING`) | Not Started | 14 leaves; bodice + set-in sleeve, both instantiation paths, `.rul` interchange |
| G4 — profiles & uncertainty (`G4-PROFILES`) | Not Started | 14 leaves; CSP + differential oracle, evidence store, policy matrix, HPGL, minimal Profile Editor |
| G5 — application shells & validated 2D UX | Not Started | tree seeded by `PLANNING.3` |
| G6 — conformance lab & reliability | Not Started | tree seeded by `PLANNING.3` |
| G7 — scoped production declaration | Not Started | tree seeded by `PLANNING.3` |
| V1 — assembly visualization (parallel) | Not Started | tree seeded by `PLANNING.3`; never blocks a G-gate |
| V2 — physically validated simulation (parallel) | Not Started | tree seeded by `PLANNING.3`; uncapped, evidence-gated |
| Product code (`crates/`) | Not Started | only the bedrock starter crate exists; roadmap §4.3 crates appear when their stage starts |
