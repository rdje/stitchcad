<!-- knowledge-map/subsystems.md — the ONE hand-curated input to the derived Knowledge Map.
     Edit this to give a fast orientation to the project's key subsystems / entry points.
     gen_knowledge_map.sh embeds this section verbatim; the task-tree and decision sections
     are generated automatically. -->

- `crates/sc-units/src/lib.rs` — units/errors; tests
  `crates/sc-units/tests/property.rs`, spec `docs/book/src/spec/units-and-tolerances.md`.
  Owner `G0-CONTRACT.2` / `.18`.
- `crates/sc-core/src/lib.rs`, `crates/sc-measure/src/lib.rs` — ontology/measurement inputs;
  recipe syntax/literals/identity; crate tests.
  Owner `G1-SLICE.3` / `.4` / `.5`.
- `docs/book/src/SUMMARY.md` — learning/index/annexes; owner `G0-CONTRACT` / `G1-SLICE.4d.1`; feature leaves.
- `docs/book/src/spec/formula-language.md` — formula contract.
  Oracle `docs/tasks/artifacts/formula_language/run_formula_language_census.sh`; owner
  `G0-CONTRACT.9` / `G1-SLICE.5`.
- `docs/book/src/spec/interchange-dialects.md` — dialect registry.
  Tool `docs/tasks/artifacts/interchange/run_interchange_census.sh`. Owner `G0-CONTRACT.10` / `G2-2D`.
- `docs/book/src/spec/feature-matrix.md` — release scope;
  `docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh`. Owner `G0-CONTRACT.4`.
- `docs/book/src/spec/glossary/` — vocabulary; A–Z `glossary.md`;
  `docs/tasks/artifacts/glossary/run_glossary_census.sh`; owner `G0-CONTRACT.1` and later gates.
- `.doctrine/live_document_size/` — containment (`surfaces.tsv`, `routes.tsv`), enforced by
  `scripts/check_live_doc_size.sh`. Owner `SPINE.4`.
- `docs/tasks/artifacts/` — probes; `make probes`.
  `g0_exit/run_g0_exit_review.sh` derives G0 from `ROADMAP.md` §11. Owner: instrument leaf.
