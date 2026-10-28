# Sealed engineering continuity — part625

- **Identity:** 16 lines, 1241 bytes, sha256:9613cfe74a10b1c4bdd37ed434b90b413495272e78c29b520ab380fdba457874
- **Coverage:** whole closed D292 ledger and G1 workspace scope from 71595c400486c84c9ca195d460cac251efcc2e76.
- Whole source interval: docs/tasks/PLANNING.md bytes 246–788, 542 bytes, sha256:7b5f0ac7b41b13db0f2ee96d33bf71b1443d163eb7f34939c4ba90de501d2eeb
- Whole source interval: docs/tasks/G1-SLICE.md bytes 3278–3977, 699 bytes, sha256:271952268d922b08773167704019e326f9c2563967fef4427f0f3b5edc962bbb
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

## D292 — ontology guide still defers implemented Material command metadata
P2 fixed0445, owner G1-SLICE.6.8.2.2.2.2.2.2.1; this is one observed clause, not a whole-book audit.
- [x] ROOT CAUSE: python3 -I -B scoped native ontology clause census1 rc=0; actual private Material prep/group commit exist.
- [x] ADDRESSED: scoped native prior/current stale clause1/0 rc=0; corrected metadata scope links actual execution.
- [x] NO REGRESSION: cargo test -p sc-core --test design_resource_contract:15 passed rc=0; strict lint/fmt/privacy4 rc=0.
- ID: `G1-SLICE.1`
  Status: `done`
  Goal: workspace shape per §4.3 — retire the bedrock starter crate (`crates/app`, defect D10),
  create `sc-units` and `sc-core` with the workspace lints inherited, and extend CI to the G0
  workflow shape (fmt / clippy / unit+property / WASM smoketest).
  Acceptance: `cargo metadata` lists the roadmap crates that exist so far; no crate prints the
  template message; CI green on the new layout; the Knowledge Map names the subsystems.
  Verification: recorded below — delivered by `G0-CONTRACT.18` (commit `eb83f01`) ahead of this
  leaf; every acceptance criterion re-derived by command in the `### G1-SLICE.1` checklist.
  Commit: `STITCHCAD-G1-0001`

