# LIVE_STATUS.md — authoritative live progress tracker

Rows use ONLY these four states: **Done · Mostly Done · In Progress · Not Started**.
Review and update before every commit whenever actual closure or remaining scope changes;
summarize the snapshot in every commit-workflow completion message.

| Area | Status | Notes |
| --- | --- | --- |
| Discipline spine (bedrock 0.6.1) | Done | memory architecture · task-trees · commit workflow · doctrine enforcement · mdBook skeleton |
| Roadmap → task-trees (`PLANNING`) | In Progress | `.1`, `.2` done (index repaired; `PLANNING`/`SPINE`/`G0-CONTRACT`/`G1-SLICE`/`G2-2D`/`G3-GRADING`/`G4-PROFILES` seeded — 87 leaves across 7 trees; 19 defects logged, 15 closed, 4 open (D7/D9 → `SPINE.5`, D13 → `SPINE.4`, D10 → `G1-SLICE.1`)). `.3` owes `G5`–`G7`, `V1`, `V2` + the coverage map |
| Repo identity & adopted policy (`SPINE`) | In Progress | 10 of 14 leaves done (`.4` split into `.4.1`–`.4.3`): `.1` identity (README is the StitchCAD landing page — 103/300 lines, 6 063/16 384 bytes — the mdBook is named, builds, and has a `spec/` part; `make book` output ignored, D19), `.4.1` the live-document containment doctrine adopted in-repo with its deferrals named, `.11` the published signature measurement has a tracked, watched producer (D20), `.3` both external policy references adopted in-repo (`README_POLICY.md` refreshed, `CLAIM_VERIFICATION.md` added; D11, D12), `.12` the 400-commit push cadence recorded in `COMMIT.md`, `.6` lockfile tracked, `.2` first artifact cleanup + `docs/ARTIFACT_CLEANUP.md` cadence record (D8), `.7`–`.8` defect D15 measured, published and closed locally by the `FRESH-ACCEPTANCE-EVIDENCE` project doctrine, `.9` the scaffold updater guards project content (D17), `.10` `make probes` pins scratch to this volume (D16). Next `.4.2` surface inventory + derived ceilings + route registry, `.4.3` the containment checker, `.5` toolbox/knowledge map |
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
