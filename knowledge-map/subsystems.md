<!-- knowledge-map/subsystems.md — the ONE hand-curated input to the derived Knowledge Map.
     Edit this to give a fast orientation to the project's key subsystems / entry points.
     gen_knowledge_map.sh embeds this section verbatim; the task-tree and decision sections
     are generated automatically. -->

- `crates/sc-units/` — **the numerical contract, implemented.** Fixed-point micrometre lengths,
  microdegree angles, areas, ratios and counts as distinct types; exact integer conversion ratios with
  a single rounding step (half away from zero); the five tolerance classes, each requiring a derivation
  to be constructed; typed diagnostics instead of panics. No dependencies, so it serves every runtime
  profile including `wasm-viewer`. Entry point: `crates/sc-units/src/lib.rs`; conformance tests:
  `crates/sc-units/tests/property.rs`. Spec: `docs/book/src/spec/units-and-tolerances.md`.
  Owner: `G0-CONTRACT.2` (spec) and `G0-CONTRACT.18` (code).
- `crates/sc-core/` — **skeleton.** The future garment ontology, construction recipe, command bus and
  uncertainty model. Exists at G0 because the roadmap's CI clause requires a real
  `wasm32-unknown-unknown` build of `sc-core` + `sc-units`. Entry point: `crates/sc-core/src/lib.rs`,
  which names the module each future leaf owns. Owner: `G0-CONTRACT.3` (spec) and `G1-SLICE.3` (code).
- `docs/book/src/spec/` — the normative specification (the director-facing contract). Chapters so far:
  the G0 contract overview and units & tolerances. Owner: the `G0-CONTRACT` leaves.
- `.doctrine/live_document_size/` — the containment data plane: `surfaces.tsv` (every live document with
  its lifecycle, owner, health target and ceiling) and `routes.tsv` (every routing destination).
  Enforced by `scripts/check_live_doc_size.sh`. Owner: `SPINE.4`.
- `docs/tasks/artifacts/` — the diagnostic probe suites, one directory per instrument. Run all of them
  with `make probes`. Owner: the leaf that needed the instrument.
