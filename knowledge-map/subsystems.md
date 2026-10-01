<!-- knowledge-map/subsystems.md — the ONE hand-curated input to the derived Knowledge Map.
     Edit this to give a fast orientation to the project's key subsystems / entry points.
     gen_knowledge_map.sh embeds this section verbatim; the task-tree and decision sections
     are generated automatically. -->

Entries are orientation-sized — path, what it is, where to enter, owner — because the map is a projection
sharing its ceiling with a line per record and per tree: `decision_knowledge-map-entries-are-orientation-sized.md`.

- `crates/sc-units/` — fixed-point lengths/angles, exact conversions, five tolerance classes and typed
  errors; dependency-free and wasm-safe. Entry `crates/sc-units/src/lib.rs`, tests
  `crates/sc-units/tests/property.rs`, spec `docs/book/src/spec/units-and-tolerances.md`.
  Owner `G0-CONTRACT.2` / `.18`.
- `crates/sc-core/` — **ontology in progress**: identity, topology-journal resolution and repairs, immutable
  pieces/copy plans, notches and sewing graphs. Other objects, recipe and bus follow. Entry
  `crates/sc-core/src/ontology/`; tests `crates/sc-core/tests/`. Owner `G0-CONTRACT.3` / `G1-SLICE.3`.
- `docs/book/src/spec/` — normative contracts reviewed by the director. Entry
  `docs/book/src/SUMMARY.md`; owner `G0-CONTRACT` and later implementation gates.
- `docs/book/src/spec/formula-language.md` — expression contract, with linked grammar and examples.
  Oracle `docs/tasks/artifacts/formula_language/run_formula_language_census.sh`; owner
  `G0-CONTRACT.9` / `G1-SLICE.5`.
- `docs/book/src/spec/interchange-dialects.md` — six-axis target registry, layers, entities and grading.
  Checked by `docs/tasks/artifacts/interchange/run_interchange_census.sh`. Owner `G0-CONTRACT.10` / `G2-2D`.
- `docs/book/src/spec/feature-matrix.md` — **the boundary of the release claim**: 105 dispositioned rows, 29
  declared diagnostics, coverage derived by
  `docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh`. Owner `G0-CONTRACT.4`.
- `docs/book/src/spec/glossary/` — nine vocabulary parts; A–Z entry `glossary.md`. Checked by
  `docs/tasks/artifacts/glossary/run_glossary_census.sh`; owner `G0-CONTRACT.1` and later gates.
- `.doctrine/live_document_size/` — the containment data plane (`surfaces.tsv`, `routes.tsv`), enforced by
  `scripts/check_live_doc_size.sh`. Owner `SPINE.4`.
- `docs/tasks/artifacts/` — the diagnostic probe suites, one directory per instrument; `make probes` runs
  them all with scratch pinned to this volume. `g0_exit/run_g0_exit_review.sh` derives gate G0's verdict from
  `ROADMAP.md` §11 itself. Owner the leaf that needed the instrument.
