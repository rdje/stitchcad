<!-- knowledge-map/subsystems.md — the ONE hand-curated input to the derived Knowledge Map.
     Edit this to give a fast orientation to the project's key subsystems / entry points.
     gen_knowledge_map.sh embeds this section verbatim; the task-tree and decision sections
     are generated automatically. -->

Entries are orientation-sized — path, what it is, where to enter, owner — because the map is a projection
sharing its ceiling with a line per record and per tree: `decision_knowledge-map-entries-are-orientation-sized.md`.

- `crates/sc-units/` — **the numerical contract, implemented**: fixed-point micrometres and microdegrees,
  exact conversion ratios, five tolerance classes, typed diagnostics; no dependencies, so every runtime
  profile including `wasm-viewer` can use it. Entry `crates/sc-units/src/lib.rs`, conformance
  `crates/sc-units/tests/property.rs`, spec `docs/book/src/spec/units-and-tolerances.md`. Owner
  `G0-CONTRACT.2` / `.18`.
- `crates/sc-core/` — **skeleton**: the future ontology, recipe, command bus and uncertainty model, present
  at G0 only because the roadmap's CI clause needs a real `wasm32-unknown-unknown` build. Entry
  `crates/sc-core/src/lib.rs`, which names the module each future leaf owns. Owner `G0-CONTRACT.3` (spec),
  `G1-SLICE.3` (code).
- `docs/book/src/spec/` — the normative specification the director reviews: overview, glossary, units,
  ontology, formula language, envelope, instantiation paths, size sets, standards, interchange dialects,
  release and approval, reference skirt. Owner the `G0-CONTRACT` leaves.- `docs/book/src/spec/formula-language.md` — **the recipe's expression language, in three parts** (contract,
  `formula-language/grammar.md`, `formula-language/examples.md`). Its numbers are computed, not typed:
  `docs/tasks/artifacts/formula_language/run_formula_language_census.sh` reads the chapter's own tables.
  Owner `G0-CONTRACT.9`, implemented by `G1-SLICE.5`.
- `docs/book/src/spec/interchange-dialects.md` — **dialects, not a format**: six axes, a closed registry of
  four targets, the seventeen-layer table in both naming modes, one polyline-only entity set, three grading
  carriages, the receiver-config record. Derived against `ROADMAP.md` by
  `docs/tasks/artifacts/interchange/run_interchange_census.sh`. Owner `G0-CONTRACT.10`, written from `G2-2D`.
- `docs/book/src/spec/feature-matrix.md` — **the boundary of the release claim**: 105 dispositioned rows, 29
  declared diagnostics, coverage derived by
  `docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh`. Owner `G0-CONTRACT.4`.
- `docs/book/src/spec/glossary/` — **the vocabulary, partitioned** into eight domain parts behind
  `glossary.md`'s derived A–Z index: one meaning per term, one owner per machine token, derived by
  `docs/tasks/artifacts/glossary/run_glossary_census.sh`. Owner `G0-CONTRACT.1` and every later chapter.
- `.doctrine/live_document_size/` — the containment data plane (`surfaces.tsv`, `routes.tsv`), enforced by
  `scripts/check_live_doc_size.sh`. Owner `SPINE.4`.
- `docs/tasks/artifacts/` — the diagnostic probe suites, one directory per instrument; `make probes` runs
  them all with scratch pinned to this volume. Owner the leaf that needed the instrument.
