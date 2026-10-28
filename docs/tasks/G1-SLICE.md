# G1-SLICE: executable architecture slice (roadmap gate G1)

## Metadata

- Tree ID: `G1-SLICE`
- Status: `active`
- Roadmap lane: `ROADMAP.md` §11 gate **G1 — Executable architecture slice** (sources: §4
  architecture, §5 ADR-0001/0002, §6 constraint machinery, §7.3 runtime profiles, §7.8 API+MCP,
  §10 security)
- Created: `2026-09-29`
- Owner: repo-local workflow

## Current leaf acceptance —0448

- [x] ROOT CAUSE: python3 -I -B native sewing schema assertions print spans/weighted regions/stops
  and Notch/Turn fixed logical/profile payloads rc=0; precharge actual dynamic fields without resolution.
- [x] ADDRESSED: cargo test -p sc-core --test design_resource_contract:28 passed rc=0; four actual
  sewing cases verify13/0/0, all ease variants, exact graph/span/index witnesses and held refs after deletion.
- [x] NO REGRESSION: strict selected Clippy/fmt finished rc=0; git diff --quiet HEAD --
  crates/sc-core/src/command.rs crates/sc-core/src/command rc=0. Prior24 resource cases remain passing.

## Goal

The architecture exists as **running code on all three runtime profiles**: the ontology and the
construction recipe evaluate, one CSP constraint resolves, a project persists and reopens, the
command bus mediates every mutation with revision checks and undo, a real browser executes the
slice end to end, and the canvas-hosting question is settled by measured evidence rather than
argument.

G1 is the first lane that lands product code, so it owns the workspace shape: the bedrock
starter crate is retired and the roadmap §4.3 crate layout appears (defect D10).

## Entry criteria

- `G0-CONTRACT` closed (or each leaf below cites the G0 spec chapter it implements; a leaf may
  not start before its spec exists — code without a contract is a guess with a build step).
- `SPINE.8` landed: the acceptance-evidence gate is sound for multi-leaf tree files, because
  every code commit from here on is judged by it (defect D15).

## Non-Goals

- The 2D correctness proof (G2), grading (G3), Factory Profiles (G4), shipped UX (G5).
- The production Profile Editor, exporters beyond the one needed to prove the browser spike's
  `export` step, or any 3D/mesh work (V1).
- Claiming WASM parity: `wasm-viewer` is a documented capability subset (§7.3), and the spike's
  output is a capability matrix with tested versions and limitations, not a parity claim.

## Acceptance Criteria (gate G1 exit, clause by clause)

| Roadmap G1 exit clause | Leaf |
| --- | --- |
| three runtime profiles compile | `.11` |
| intended browser target runs a real (not compile-only) spike: load → evaluate recipe → one CSP constraint → persist/reopen → render → export | `.12` |
| canvas-hosting spike evidence for ADR-0002 | `.13` |
| command bus live with revision checks + undo | `.6` |
| crash-consistent save/recovery demonstrated | `.7` |
| license-compatible solver builds on all targets | `.15` |
| WASM evidence: capability matrix with tested versions + limitations | `.12` |

## Task Tree

- ID: `G1-SLICE`
  Status: `active`
  Goal: the architecture slice runs on native and in a browser, with the command bus and
  persistence as the only mutation path.
  Children: `.1`, `.2`, `.3a`/`.3b`/`.3c`, `.4` … `.16` (18 leaves; `.3` was decomposed `2026-09-30`)

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

- ID: `G1-SLICE.2`
  Status: `done`
  Goal: sc-units0002/G0.18; typed exact units/tolerances and focused acceptance; [whole scope](../history/stitchcad-devnotes-part553.md).
- ID: `G1-SLICE.3a`
  Status: `done`
  Goal:closed scope; [whole proof](../history/window21.md#stitchcad-devnotes-part526md).
- ID: `G1-SLICE.3b`
  Status: `done`
  Goal:closed scope; [whole proof](../history/window21.md#stitchcad-devnotes-part526md).
- ID: `G1-SLICE.3c`
  Status: `done`
  Goal:closed structural ontology scope; [whole proof](../history/window21.md#stitchcad-devnotes-part531md).
  Children: `.3c.1`, `.3c.2`, `.3c.3`, `.3c.4`; geometric proof remains G2.

- ID: `G1-SLICE.3c.1`
  Status: `done`
  Goal: Structural Piece contract complete; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part485md).
- ID: `G1-SLICE.3c.1a`
  Status: `done`
  Goal:original D56 pair-member scope; [whole proof](../history/window21.md#stitchcad-devnotes-part534md).

- ID: `G1-SLICE.3c.2`
  Status: `done`
  Goal:original sewing scope; [whole proof](../history/stitchcad-devnotes-part538.md).

- ID: `G1-SLICE.3c.2a`
  Status: `done`
  Goal:original range/sewing scope; [whole proof](../history/stitchcad-devnotes-part538.md).

- ID: `G1-SLICE.3c.2b`
  Status: `done`
  Goal:original range/sewing scope; [whole proof](../history/stitchcad-devnotes-part538.md).

- ID: `G1-SLICE.3c.2b.1`
  Status: `done`
  Goal: Structural sewing-side primitives complete; whole source/contracts/proofs [retained](../history/window20.md#stitchcad-devnotes-part484md).
- ID: `G1-SLICE.3c.2b.2`
  Status: `done`
  Goal: completed copy-addressed sewing contract; [whole scope/proofs](../history/window15.md#stitchcad-devnotes-part430md).

- ID: `G1-SLICE.3c.3`
  Status: `done`
  Goal:closed scope; [whole proof](../history/window12.md#stitchcad-devnotes-part328md).
- ID: `G1-SLICE.3c.3a`
  Status: `done`
  Goal:closed scope; [whole proof](../history/window12.md#stitchcad-devnotes-part328md).
- ID: `G1-SLICE.3c.3b`
  Status: `done`
  Goal:closed scope; [whole proof](../history/window12.md#stitchcad-devnotes-part328md).
- ID: `G1-SLICE.3c.3c`
  Status: `done`
  Goal:closed scope; [whole proof](../history/window12.md#stitchcad-devnotes-part328md).
- ID: `G1-SLICE.3c.4`
  Status: `done`
  Goal:original construction scope; [whole proof](../history/stitchcad-devnotes-part541.md).
  Children: .3c.4a/.3c.4b/.3c.4c/.3c.4d remain in the linked sibling below.

[Completed construction child contracts and acceptance evidence](G1-SLICE-constructions.md)
are preserved unchanged in a bounded sibling; .3c.4 remains closed by STITCHCAD-G1-0023.

- ID: `G1-SLICE.4`
  Status: `done`
  Goal: sc-measure0321 structural library; runtime/registry proof remains G1/G3/G4; [whole scope](../history/stitchcad-devnotes-part553.md).
  Children: .4a (measurement declarations/tables), .4b (per-POM ease), .4c (SizeSet), .4d (signoff).
- ID: `G1-SLICE.4a`
  Status: `done`
  Goal: complete preserved structural contracts/proofs/navigation in [part475](../history/window20.md#stitchcad-devnotes-part475md).
  Commit: .4a children0024–0028/.4b0031; whole originals retained.

- ID: `G1-SLICE.4a.2`
  Status: `done`
  Goal: complete preserved structural contracts/proofs/navigation in [part475](../history/window20.md#stitchcad-devnotes-part475md).
  Commit: .4a children0024–0028/.4b0031; whole originals retained.

- ID: `G1-SLICE.4a.3`
  Status: `done`
  Goal: named tables/current bindings and .4a structural signoff.
  Verification: [complete preserved contract and checklist](G1-SLICE-measurements.md#named-table-contract-and-evidence--preserved-from-f19982d).
  Commit: `STITCHCAD-G1-0028`

- ID: `G1-SLICE.4b`
  Status: `done`
  Goal: complete preserved structural contracts/proofs/navigation in [part475](../history/window20.md#stitchcad-devnotes-part475md).
  Commit: .4a children0024–0028/.4b0031; whole originals retained.

- ID: `G1-SLICE.4b.1`
  Status: `done`
  Goal: individual canonical Ease intent and explicit signed permission.
  Verification: [complete preserved contract/checklist](G1-SLICE-measurements.md#individual-ease-contract-and-evidence--preserved-from-0088969).
  Commit: `STITCHCAD-G1-0029`

- ID: `G1-SLICE.4b.2`
  Status: `done`
  Goal: unique current per-POM sets with named table membership.
  Verification: [complete preserved contract/checklist](G1-SLICE-measurements.md#per-pom-set-contract-and-evidence--preserved-from-b4e0bc7).
  Commit: `STITCHCAD-G1-0030`

- ID: `G1-SLICE.4b.3`
  Status: `done`
  Goal: structural Ease field/currentness review and milestone.
  Verification: [preserved review contract](G1-SLICE-measurements.md#structural-ease-review-contract--preserved-from-71aaf19)
  and [preserved evidence](G1-SLICE-measurements.md#structural-ease-review--preserved-from-b6bd985).
  Commit: `STITCHCAD-G1-0031`

- ID: `G1-SLICE.4c`
  Status: `done`
  Goal: SizeSet structural/intention children complete; execution/approval later; [whole scope](../history/stitchcad-devnotes-part553.md).
  Children: .4c.1 (membership), .4c.2 (axes), .4c.3 (chart/breaks), .4c.4 (resolution), .4c.5 (review).
- ID: `G1-SLICE.4c.1`
  Status: `done`
  Goal: stable authored size membership and checked revision successors.
  Verification: [preserved full contract/checklist](G1-SLICE-measurements.md#size-membership-contract-and-evidence--preserved-from-0b77235).
  Commit: `STITCHCAD-G1-0032`

- ID: `G1-SLICE.4c.2`
  Status: `done`
  Goal: axes with ordered values and member coordinate references; incomplete grids remain explicit.
  Director2026-10-26 resolves D70: every complete set has >=1 authored axis; never infer names/values.
  Acceptance: current axis/value identities and complete member points validated, no grid interpolation;
  no label-derived body measurements or silent axis defaults. Preserve source/state for numeric axes.
  Verification: individual5/0 + aggregate5/0; selected strict lint/fmt/book0.
  Commit: STITCHCAD-G1-0297/0298.

- ID: `G1-SLICE.4c.2.1`
  Status: `done`
  Goal:0297 individual axes verified5/0/lint/fmt/book0.
  Acceptance: [whole scope/proofs](../history/window15.md#stitchcad-devnotes-part422md).
  Commit: STITCHCAD-G1-0297.

- ID: `G1-SLICE.4c.2.2`
  Status: `done`
  Goal:0298 complete axis/member points verified5+5/0/lint/fmt/book0.
  Acceptance: [whole scope/proofs](../history/window15.md#stitchcad-devnotes-part424md).
  Commit: STITCHCAD-G1-0298.

- ID: `G1-SLICE.4c.3`
  Status: `done`
  Children: .4c.3a garment charts, .4c.3b collection, .4c.3c MTM, .4c.3d composite, .4c.3e review.
  Goal:complete SizeSet chart/break scope/proofs; [whole parent](../history/stitchcad-devnotes-part546.md).

- ID: `G1-SLICE.4c.3a`
  Status: `done`
  Goal: canonical current garment chart observations with stable member/POM references.
  Verification: [preserved full contract/checklist](G1-SLICE-measurements.md#garment-chart-observation-contract-and-evidence--preserved-from-71aaf19).
  Commit: `STITCHCAD-G1-0033`

- ID: `G1-SLICE.4c.3b`
  Status: `done`
  Goal: immutable current Design/member/POM garment-chart coverage; full contract/evidence preserved
  in [measurement evidence](G1-SLICE-measurements.md#garment-chart-collection-evidence--preserved-from-e299771).
  Verification: 18 contracts/privacy, fourteen real reds; 422 strict tests/WASM/book; staged doctrines.
  Commit: `STITCHCAD-G1-0034` (`e299771`).

- ID: `G1-SLICE.4c.3c`
  Status: `done`
  Goal: custom-member body/Ease inputs; full contract/evidence retained in
  [measurement evidence](G1-SLICE-measurements.md#mtm-contract-and-evidence--preserved-from-285e238).
  Verification: 15 contracts/two privacy-role docs, twelve real reds; 439 strict tests/WASM/book.
  Commit: `STITCHCAD-G1-0035` (`285e238`).

- ID: `G1-SLICE.4c.3d`
  Status: `done`
  Goal: signed canonical adjacent breaks and composite SizeSet over membership/axes/chart with source/
  state/provenance; per-axis/dimension coverage and typed path readiness under the confirmed D70 rule.
  Acceptance: authored adjacency, zero/uneven breaks, current references, missing dimension/pair and
  MTM path-2 refusal; no rounded values, generated equivalence or profile execution claimed.
  Verification: focused declaration/coverage/composite children0303/0304/0306–0308 pass0; structural scope only.
  Commit: STITCHCAD-G1-0303/0304/0306/0307/0308.

- ID: `G1-SLICE.4c.3d.0`
  Status: `done`
  Goal:0302 D228 key approved; [whole ruling/evidence](../history/window15.md#stitchcad-devnotes-part433md).
  Commit: STITCHCAD-G1-0302.
- ID: `G1-SLICE.4c.3d.1`
  Status: `done`
  Goal:0303 individual signed breaks verified5+5/0/lint/fmt/book0; [whole scope/proofs](../history/window15.md#stitchcad-devnotes-part435md).
  Commit: STITCHCAD-G1-0303.
- ID: `G1-SLICE.4c.3d.2`
  Status: `done`
  Goal:0304 required break coverage verified5+5/0/lint/fmt/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part437md).
  Commit: STITCHCAD-G1-0304.
- ID: `G1-SLICE.4c.3d.3`
  Status: `done`
  Goal: composite membership/axes/garment-or-MTM chart/break inputs and typed structural path readiness;
  no generated equivalence, profile execution or physical/release certification.
  Children .3.1 garment current-input view, .3.2 MTM body/Ease input view, .3.3 shared input/path review.
- ID: `G1-SLICE.4c.3d.3.1`
  Status: `done`
  Goal:0306 garment inputs verified5+5/0/lint/fmt/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part442md).
  Commit: STITCHCAD-G1-0306.
- ID: `G1-SLICE.4c.3d.3.2`
  Status: `done`
  Goal:0307 MTM inputs verified5+15+5/0/lint/fmt/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part444md).
  Commit: STITCHCAD-G1-0307.
- ID: `G1-SLICE.4c.3d.3.3`
  Status: `done`
  Goal:0308 unified inputs verified6+6+5/0/lint/fmt/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part446md).
  Commit: STITCHCAD-G1-0308.

- ID: `G1-SLICE.4c.3d.3.h15`
  Status: `done`
  Goal:0305 window15/source/public/materialization/prior14/retention/book verified; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part440md).
  Commit: STITCHCAD-G1-0305.

- ID: `G1-SLICE.4c.3e`
  Status: `done`
  Goal: chart/body/break/currentness requirement review and milestone checks before closing .3;
  remaining profile transformation .4 and full SizeSet .5/.4d stay owned.
  Verification: D230/D231 source/producer repairs0309, unchanged-code/book/targeted-anchor review0310.
  Commit: STITCHCAD-G1-0309/0310.

- ID: `G1-SLICE.4c.3e.1`
  Status: `done`
  Goal:0309 D230/D231 repairs verified13-child capture/5 refusals/omission-red/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part448md).
  Commit: STITCHCAD-G1-0309.
- ID: `G1-SLICE.4c.3e.2`
  Status: `done`
  Goal:0310 chart/body/break scope closes; unchanged-code/book/anchor checks0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part450md).
  Commit: STITCHCAD-G1-0310.

- ID: `G1-SLICE.4c.4`
  Status: `done`
  Goal: resolved profile intent0311–0318; G4 execution remains separate; [whole scope](../history/stitchcad-devnotes-part553.md).
- ID: `G1-SLICE.4c.4.1`
  Status: `done`
  Goal:0311 profile pins/origin; focused18/lint/fmt/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part452md).
  Commit: STITCHCAD-G1-0311.

- ID: `G1-SLICE.4c.4.2`
  Status: `done`
  Goal:0312 label/base intent; focused21/lint/fmt/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part454md).
  Commit: STITCHCAD-G1-0312.
- ID: `G1-SLICE.4c.4.3`
  Status: `done`
  Goal: explicit break-scaling and chart-replacement intent with canonical numeric/input bindings and original states.
  Children: .3.1 canonical signed break/factor bindings; .3.2 explicit current replacement chart bindings.
  Verification: current canonical break/chart bindings and original state/path contracts pass focused checks0313/0314.
  Commit: STITCHCAD-G1-0313/0314.
- ID: `G1-SLICE.4c.4.3.1`
  Status: `done`
  Goal:0313 canonical scaling; focused23/lint/fmt/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part457md).
  Commit: STITCHCAD-G1-0313.
- ID: `G1-SLICE.4c.4.3.2`
  Status: `done`
  Goal:0314 replacement charts; focused27/lint/fmt/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part461md).
  Commit: STITCHCAD-G1-0314.
- ID: `G1-SLICE.4c.4.cleanup5`
  Status: `done`
  Goal:0316 due cleanup verified residue/preservation/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part466md).
  Commit: STITCHCAD-G1-0316.
- ID: `G1-SLICE.4c.4.cleanup5.a`
  Status: `done`
  Goal:0315 D232 attestation repair; focused controls/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part464md).
  Commit: STITCHCAD-G1-0315.
- ID: `G1-SLICE.4c.4.4`
  Status: `done`
  Goal:0317 immutable resolution intent; focused31/lint/fmt/book0; [whole scope/proofs](../history/window16.md#stitchcad-devnotes-part468md).
  Commit: STITCHCAD-G1-0317.
- ID: `G1-SLICE.4c.4.5`
  Status: `done`
  Goal:0318 profile intent scope/D233 verified book/anchors0; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part469md).
  Commit: STITCHCAD-G1-0318.
- ID: `G1-SLICE.4c.5.h16`
  Status: `done`
  Goal:0319 window16 whole48/source/read/materialization/prior15/book0; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part470md).
  Commit: STITCHCAD-G1-0319.
- ID: `G1-SLICE.RETENTION-CAPACITY`
  Status: `done`
  Goal: [whole completed subtree/0344 proof](../history/window21.md#stitchcad-devnotes-part496md).
- ID: `G1-SLICE.RETENTION-CAPACITY.1`
  Status: `done`
  Goal: complete scope/proof retained496.
- ID: `G1-SLICE.RETENTION-CAPACITY.2`
  Status: `done`
  Goal: complete scope/proof retained496.
- ID: `G1-SLICE.RETENTION-CAPACITY.2.a`
  Status: `done`
  Goal: complete scope/proof retained496.
- ID: `G1-SLICE.RETENTION-CAPACITY.2.b`
  Status: `done`
  Goal: complete scope/proof retained496.
- ID: `G1-SLICE.RETENTION-CAPACITY.2.c`
  Status: `done`
  Goal: complete scope/proof retained496.
- ID: `G1-SLICE.RETENTION-CAPACITY.2.c.1`
  Status: `done`
  Goal: complete scope/proof retained496.
- ID: `G1-SLICE.RETENTION-CAPACITY.2.c.2`
  Status: `done`
  Goal: complete scope/proof retained496.
- ID: `G1-SLICE.RETENTION-CAPACITY.2.c.2.a`
  Status: `done`
  Goal: complete scope/proof retained496.
- ID: `G1-SLICE.RETENTION-CAPACITY.2.c.2.b`
  Status: `done`
  Goal: complete scope/proof retained496.
- ID: `G1-SLICE.4c.5`
  Status: `done`
  Goal:0320 SizeSet scope/current25/unchanged31/book0; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part472md).
  Commit: STITCHCAD-G1-0320.
- ID: `G1-SLICE.4d`
  Status: `done`
  Children: `.4d.1` book publication/alignment, `.4d.2` combined structural exit review.
  Goal: re-derive measurement/ease/size structural requirements, current reference contracts and book
  evidence; focused structural acceptance under the director ruling, close .4 only after all child acceptance.
  Full integration/runtime/CI retains G1 .11/.12/.16 and pre-push ownership.
  Verification:0321 focused native acceptance and scoped publication74 chapters/87 entries pass0; no fresh platform claim.
  Commit: STITCHCAD-G1-0321; full runtime/integration/CI retains later owners.

- ID: `G1-SLICE.4d.1`
  Status: `done`
  Contract/checklist: [preserved publication evidence](G1-SLICE-measurements.md#publication-contract-and-evidence--preserved-from-9ef9602).
  Verification: 47 chapters/14 APIs; publication nine, archive 28, full 24 suites green.
  Commit: `STITCHCAD-G1-0036`.

- ID: `G1-SLICE.4d.2`
  Status: `done`
  Goal:0321 combined structural acceptance/native46/publication74-87/progress6-18; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part476md).
  Commit: STITCHCAD-G1-0321.
- ID: `G1-SLICE.5`
  Status: `in_progress`
  Children: .5a syntax done; .5b static validation, .5c exact arithmetic, .5d irrational functions,
  .5e execution/replay, .5f operations and .5g full acceptance pending.
  Dependency boundary: actual measurement/Ease/SizeSet inputs and immutable profile intent are now implemented.
  Full AST .5e.2.4 and remaining functions/geometry retain .5 owners; the earlier incomplete-axis exception is retired.
  Goal: formula graph + construction-recipe evaluation — acyclic dependency graph, single
  deterministic pass, name binding (measurements, prior points/lengths/angles, profile
  parameters), conditionals, units inside expressions (implements the `G0-CONTRACT.9` language).
  Acceptance: evaluation is byte-reproducible across runs and platforms; a cycle or unbound name
  is a structured diagnostic naming the formula; every spec example in the formula chapter is a
  test.
  Pocket openings: validate logical declaration kind/domain/state and component recipe dependencies;
  G3 executes physical construction and determines supported opening scope without approximation.
  Buttonhole derivation: validate typed operation/dependency on the canonical button-size declaration;
  G3 executes it and validates positive physical length without a separately authored hole length.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5a`
  Status: `done`
  Children: .5a.1 lexical, .5a.2 expression, .5a.3 canonical, .5a.4 review.
  Goal:complete syntax scope/proofs; [whole parent](../history/stitchcad-devnotes-part546.md).
[Recipe subtree](G1-SLICE-recipes.md#completed-syntax-node-graph--preserved-during-g1-0072).
[Proof](G1-SLICE-checked-recipes.md) · [Retention](G1-SLICE-retention-proof.md).

- ID: `G1-SLICE.5a.4`
  Status: `done`
  Goal: review all syntax/canonical-form requirements and deferred name/type/evaluation owners,
  full milestone gate before closing .5a; exact evaluator/DAG and operations remain .5 children.
  Verification: full obligation/deferred map; native591/WASM3/probes25, seven actual reds/exact restore.
  Commit: `STITCHCAD-G1-0072`.

- ID: `G1-SLICE.5b`
  Status: `in_progress`
  Goal: namespace, origin and static kind validation before any value is computed (contract2/3/5).
  Children: .5b.1 complete independent obligation/oracle review; .2 typed declarations/context;
  .3 all operator/function/selector signatures; .4 whole ordered static graph and atomic refusal.
  Acceptance: all names/kinds in both branches validated, every static refusal contextual and no
  numeric/geometry callback invoked. Detailed child contracts in G1-SLICE-recipes.md.
  Verification: `pending`; Commit: `pending`.

- ID: `G1-SLICE.5c`
  Status: `pending`
  Goal: exact bounded rational arithmetic and binding storage (contract4.2/4.3; D84/D95).
  Children: .5c.1 reduced rational primitive; .2 complete rational operators/domains;
  .3 explicit quantization and numeric/Boolean binding; .4 coupled numerical boundary review.
  Acceptance: no intermediate rounding, reduced width128, per-result scalar domains, signed turns,
  full i64 bindings and typed contextual refusals. Detailed child contracts in G1-SLICE-recipes.md.
  Verification: `pending`; Commit: `pending`.

- ID: `G1-SLICE.5d`
  Status: `pending`
  Goal: correctly rounded sqrt/hypot/trigonometry/arc_length (contract4.2; grammar6).
  Children: .5d.1 algorithm/independent proof contract; .2 sqrt/hypot; .3 trig/inverse/sweeps;
  .4 complete rounding/domain/provenance review. Curated reference accuracy is insufficient alone.
  Acceptance: true-value nearest quantum/ties away, arbitrary admitted inputs, no libm-dependent
  answer; original width/domain limits preserved. Child contracts in G1-SLICE-recipes.md.
  Verification: `pending`; Commit: `pending`.

- ID: `G1-SLICE.5e`
  Status: `pending`
  Goal: atomic single-pass execution, state/tolerance handling and replay (contract3/4/5/9).
  Children: .5e.1 canonical input/state adapters; .2 ordered lazy execution;
  .3 within/assertions/irrational provenance; .4 atomic deterministic replay/DAG review.
  Acceptance: prior values only, unknown reads never default, static checks before execution,
  no partial accepted result and same canonical bound output on replay.
  [Execution children and receipts](G1-SLICE-execution.md) retain the current frontier.
  Verification: `pending`; Commit: `pending`.

- ID: `G1-SLICE.5f`
  Status: `pending`
  Goal: typed construction operations and geometry dependencies (ontology3.1/3.2; roadmap6.1).
  Children: .5f.1 complete operation/interface protocol; .2 ordered typed dependencies;
  .3 operation/selector execution bridge; .4 coupled construction integration review.
  Acceptance: full v1 set owned without implicit solver or fabricated geometry; operation arguments
  evaluate at their authored position. G2/G3 physical owners stay explicit. See G1-SLICE-recipes.md.
  Verification: `pending`; Commit: `pending`.

- ID: `G1-SLICE.5g`
  Status: `pending`
  Goal: full formula/construction acceptance, example parity and two-platform reproducibility.
  Children: .5g.1 worked/refusal/fixture whole population; .2 independent platform/replay proof;
  .3 full milestone and remaining-owner audit before closing .5.
  Acceptance: every contract9 requirement proven at product scope; actual platform evidence,
  retained diagnostics and public-interface fault controls. See G1-SLICE-recipes.md.
  Verification: `pending`; Commit: `pending`.

- ID: `G1-SLICE.6`
  Status: `in_progress`
  Goal: command bus (§4.4) — typed validated commands, atomic groups, preview/commit, revision
  preconditions, idempotency keys, structured errors, progress for long operations, and the
  undo/redo granularity decided at G0.
  Acceptance: no mutation path bypasses the bus (enforced by module privacy and a test that the
  domain types expose no public mutators); a stale revision is rejected; undo/redo restores
  semantics, not just geometry. Design-level reference validation consumes BOTH point registrations
  and whole-range repair evidence (`.3c.2a`); a lost interior blocks readiness even with live endpoints.
  Pair-member metadata (`.3c.1a`) is checked against the design collection: companions exist, name
  each other, have opposite handedness and equal quantities. Geometric mirroring stays G2's check.
  Sewing amount references (`.3c.2b.2`) also resolve to existing length-valued formula/profile
  declarations before design acceptance; symbolic identities are not valid parameter values.
  Verification: `pending`
  Commit: `pending`

  Pockets: validate current copy/source bindings, position/orientation and opening declarations;
  inspect every component Piece contour repair before execution/release. A borrowed target or
  successful Pocket placement check does not certify component geometry or opening scope.

  Closures: validate every current placement/instance, declaration and notion count before execution;
  unsupported fly requests retain named env_fly with requested Closure/trousers gap/G7.

  Notion placements: validate current plan, original source Piece and all held anchor/direction
  references with validate_current(); raw evidence alone is not a registry/release certificate.

  Hem composition: validate current Facing identities/served owners/sources and depth/fold declarations
  before execution; independent edge/target queries alone do not grant release readiness.

  Layer execution scope: `.3c.4b.1` models lining but its `require_in_scope()` returns env_lining.
  Apply envelope refusals before requested construction execution; inspection preserves modelled content.

- ID: `G1-SLICE.6.1`
  Status: `done`
  Goal:0291 typed17-command catalog/book2/0/lint0 and exact window14 packing verified.
  Paths: crates/sc-core/src/command.rs;crates/sc-core/tests/command_catalog_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window15.md#stitchcad-devnotes-part405md).
- ID: `G1-SLICE.6.2`
  Status: `done`
  Goal: typed payload/envelope, actor/revision/key/group/effect identities and actual canonical argument contracts.
- ID: `G1-SLICE.6.2.1`
  Status: `done`
  Goal:0292 D225 terminal allocation/atomic ledger refusal verified3/10/compile/lint0.
  Paths: crates/sc-core/src/ontology/id.rs;crates/sc-core/tests/id_exhaustion_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window15.md#stitchcad-devnotes-part409md).
- ID: `G1-SLICE.6.2.2`
  Status: `done`
  Goal: bounded command envelope/principal/identity contract and actual typed payload schema, after D225.
- ID: `G1-SLICE.6.2.2.a`
  Status: `done`
  Goal:0293 actor/revision/retry/group context verified native2/0/lint/census/book0.
  Paths: crates/sc-core/src/command/context.rs;crates/sc-core/tests/command_context_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/window15.md#stitchcad-devnotes-part411md).
- ID: `G1-SLICE.6.2.2.b`
  Status: `done`
  Goal: concrete typed command payload schemas from actual canonical interfaces, no caller placeholders.
- ID: `G1-SLICE.6.2.2.b.1`
  Status: `done`
  Goal:0294 measurement/material schemas verified native3/0/lint/census/book0.
  Acceptance: [whole scope/six proof boxes](../history/window15.md#stitchcad-devnotes-part414md).
- ID: `G1-SLICE.6.2.2.b.2`
  Status: `done`
  Goal: remaining SplitSeamSpan/CloseDart/AddNotch schemas from published canonical references and creation rules.
- ID: `G1-SLICE.6.2.2.b.2.a`
  Status: `done`
  Goal:0295 construction intents and D70/D226 rulings verified2+3/0/lint/book0 in1184214.
  Acceptance: [whole scope/six proof boxes](../history/window15.md#stitchcad-devnotes-part417md).
- ID: `G1-SLICE.6.2.2.b.2.b`
  Status: `done`
  Goal:0299 paired sewing split schema verified4+3/0/lint/fmt/census/book0.
  Acceptance: [whole scope/proofs](../history/window15.md#stitchcad-devnotes-part426md).
  Commit: STITCHCAD-G1-0299.
- ID: `G1-SLICE.6.2.2.b.2.a.receipt`
  Status: `done`
  Goal:0296 unique metadata receipt preserves1184214; same-invocation commit/clear/check guard.
  Acceptance: [whole scope/proofs](../history/window15.md#stitchcad-devnotes-part419md).
- ID: `G1-SLICE.6.2.2.c`
  Status: `done`
  Goal: remaining typed input families complete; [whole goals](../history/window20.md#stitchcad-devnotes-part491md).
- ID: `G1-SLICE.6.2.2.c.1`
  Status: `done`
  Goal:0300 pinned piece/seam queries verified2+2/0/lint/fmt/census/book0.
  Acceptance: [whole scope/proofs](../history/window15.md#stitchcad-devnotes-part428md).
  Commit: STITCHCAD-G1-0300.
- ID: `G1-SLICE.6.2.2.c.2`
  Status: `done`
  Goal: concrete evaluation/artifact/release and exact closure inputs complete; [whole goals](../history/window20.md#stitchcad-devnotes-part491md).
- ID: `G1-SLICE.6.2.2.c.2.1`
  Status: `done`
  Goal:0322 shared exact core profile pin; [whole implementation/proofs](../history/window20.md#stitchcad-devnotes-part477md).
- ID: `G1-SLICE.6.2.2.c.2.2`
  Status: `done`
  Goal:0323 shared snapshot/distinct instantiation inputs; [whole implementation/proofs](../history/window20.md#stitchcad-devnotes-part478md).
- ID: `G1-SLICE.6.2.2.c.2.3`
  Status: `done`
  Goal:0324 exact selected-profile resolution request; [whole implementation/proofs](../history/window20.md#stitchcad-devnotes-part479md).
- ID: `G1-SLICE.6.2.2.c.2.4`
  Status: `done`
  Goal: concrete DXF/PLT/PDF/tech-pack requests and artifact-dependent closure binding; no guessed global closure.
  Children: .1 single-instance DXF/exact closure target; .2 explicit graded carriages; .3 PLT/PDF;
  .4 tech-pack canonical sources; .5 combined artifact schema review before release inputs.
- ID: `G1-SLICE.6.2.2.c.2.4.1`
  Status: `done`
  Goal:0325 exact single-instance DXF/closure and D237 correction; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part480md).
- ID: `G1-SLICE.6.2.2.c.2.4.2`
  Status: `done`
  Goal:0326 explicit registered graded DXF inputs; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part481md).
- ID: `G1-SLICE.6.2.2.c.2.4.3`
  Status: `done`
  Goal:0327 exact PLT/PDF and print settings, D238 wording; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part482md).
- ID: `G1-SLICE.6.2.2.c.2.4.4`
  Status: `done`
  Goal:0328 canonical tech-pack source request; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part483md).
- ID: `G1-SLICE.6.2.2.c.2.4.5`
  Status: `done`
  Goal: 0329 combined artifact input-schema scope closes; whole review/proofs [retained](../history/window20.md#stitchcad-devnotes-part484md).
- ID: `G1-SLICE.6.2.2.c.2.5`
  Status: `done`
  Goal: exact release candidate preparation and human approval target schemas; no authority granted by actor claims.
  Children: .1 D239 identity-boundary diagnosis/proposal; .2 ruling adoption; .3 candidate inputs;
  .4 human approval inputs; .5 combined release-schema review before .2.6.
- ID: `G1-SLICE.6.2.2.c.2.5.1`
  Status: `done`
  Goal: 0330 D239 diagnosed/director detached-record ruling recorded; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part485md).
- ID: `G1-SLICE.6.2.2.c.2.5.2`
  Status: `done`
  Goal:0331 detached-record boundary adopted, D239/D240 fixed; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part486md).
- ID: `G1-SLICE.6.2.2.c.2.5.3`
  Status: `done`
  Goal:0332 explicit candidate request inputs; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part487md).
- ID: `G1-SLICE.6.2.2.c.2.5.4`
  Status: `done`
  Goal:0333 exact candidate/scope approval intent; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part488md).
- ID: `G1-SLICE.6.2.2.c.2.5.5`
  Status: `done`
  Goal:0334 combined release schema review; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part489md).
- ID: `G1-SLICE.6.2.2.c.2.6`
  Status: `done`
  Goal:0335 shared typed core request/input scope complete; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part490md).
- ID: `G1-SLICE.6.3`
  Status: `done`
  Goal: core-scoped authority/grant validation, human-only approval, actual actor/principal and denial tests.
  Children .1 exact command/Design/time scopes and narrowing; .2 trusted-source identity/grant checks;
  .3 coupled authority acceptance before aggregate execution. No transport authentication claim without its provider.
- ID: `G1-SLICE.6.3.1`
  Status: `done`
  Goal:0336 exact command/Design/time scope and narrowing; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part491md).
- ID: `G1-SLICE.6.3.2`
  Status: `done`
  Goal:0337 scoped trusted-host authorization/D242-D243 fixed; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part492md).
- ID: `G1-SLICE.6.3.3`
  Status: `done`
  Goal:0338 native authority scope/freshness reviewed; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part493md).
Completed structural/input/garment node graph: [canonical sibling](G1-SLICE-garment.md).
All98 completed ids/statuses and original proof pointers remain live there.

- ID: `G1-SLICE.6.vocabulary`
  Status: `pending`
  Goal: P2 D250 repair preexisting public API/variant/local token declarations by canonical meaning;
  schedule after .6.9 implementation, before G1 .11/integration/pre-push. Current .6.4.1 adds no C1 rows.
  Scope includes separate P3 advisory awk/head pipe-close diagnostic cleanup after C1 closure.
- ID: `G1-SLICE.6.5`
  Status: `in_progress`
  Goal: nonmutating preview/atomic commit and semantic group rollback with actual failing-command context.
  Children .1 transaction staging/ownership design and D277 scope wording; .book maintained-reference
  containment before new bus documentation; .2 private bus/candidates; .3 actual mutation handlers;
  .4 atomic preview/commit/groups; .5 scoped feature acceptance. Revision/replay/audit .6.6 follows.
- ID: `G1-SLICE.6.5.1`
  Status: `done`
  Goal:0415 bus staging/book design/D277 correction; [whole proof](../history/stitchcad-devnotes-part565.md).
- ID: `G1-SLICE.6.5.book`
  Status: `done`
  Goal: completed maintained-book scope; [whole proof](../history/stitchcad-devnotes-part568.md).
- ID: `G1-SLICE.6.5.book.1`
  Status: `done`
  Goal: completed maintained-book scope; [whole proof](../history/stitchcad-devnotes-part566.md).
- ID: `G1-SLICE.6.5.book.2`
  Status: `done`
  Goal: completed maintained-book scope; [whole proof](../history/stitchcad-devnotes-part567.md).
- ID: `G1-SLICE.6.5.book.3`
  Status: `done`
  Goal: completed maintained-book scope; [whole proof](../history/stitchcad-devnotes-part568.md).
- ID: `G1-SLICE.6.5.2`
  Status: `done`
  Goal: private actual bus/candidates, isolated explicit identities and actual semantic differences
  complete for implemented Material/Notch subset. Other mutation semantics remain .6.5.3.
- ID: `G1-SLICE.6.5.2.a`
  Status: `done`
  Goal:0419 private owner/authorized Piece-list read; [whole proof](../history/stitchcad-devnotes-part569.md).
- ID: `G1-SLICE.6.5.2.b`
  Status: `done`
  Goal: private Material/Notch candidates and explicit owned identity-reserve staging complete;
  exact aggregate semantic-change derivation .c follows; no publication or persistent issuance claim.
- ID: `G1-SLICE.6.5.2.b.1`
  Status: `done`
  Goal:0420 held-reference-preserving Material replacement; [whole proof](../history/stitchcad-devnotes-part570.md).
- ID: `G1-SLICE.6.5.2.b.sources`
  Status: `done`
  Goal:0421 source/task guards; [whole scope/proofs](../history/stitchcad-devnotes-part589.md).
- ID: `G1-SLICE.6.5.2.b.2`
  Status: `done`
  Goal:0422 actual Material preview; [whole scope/proofs](../history/stitchcad-devnotes-part589.md).
- ID: `G1-SLICE.6.5.2.b.3`
  Status: `done`
  Goal:0423 private reserve/actual Notch preview; [whole scope/proofs](../history/stitchcad-devnotes-part575.md).
- ID: `G1-SLICE.6.5.2.c`
  Status: `done`
  Goal:0424 exact actual candidate differences; [whole scope/proofs](../history/stitchcad-devnotes-part576.md).
- ID: `G1-SLICE.6.5.3`
  Status: `in_progress`
  Goal: actual canonical mutation handlers against real source/recipe/geometry interfaces.
  Children: .1 ChangeMeasurement metadata candidate; later paired-seam and Dart operations must
  retain actual recipe/geometry dependencies. Metadata preparation confers no evaluator/physical proof.
- ID: `G1-SLICE.6.5.3.1`
  Status: `done`
  Goal: actual ChangeMeasurement metadata binding/replacement/candidate and semantic diff complete.
  Recipe reevaluation remains .5e/.5f; no publication or complete evaluation proof follows.
- ID: `G1-SLICE.6.5.3.1.1`
  Status: `done`
  Goal:0425 stable detached canonical replacement; [whole scope/proofs](../history/stitchcad-devnotes-part577.md).
- ID: `G1-SLICE.6.5.3.1.2`
  Status: `done`
  Goal:0426 actual bound Measurement metadata/D283; [whole scope/proofs](../history/stitchcad-devnotes-part579.md).
- ID: `G1-SLICE.6.5.3.2`
  Status: `pending`
  Goal: actual paired SplitSeamSpan handler: recover native range/ease/stop/lineage interfaces and
  explicit A/B-cut contract first; own any real recipe/geometry dependencies without fabricated checks.
- ID: `G1-SLICE.6.5.4`
  Status: `done`
  Goal: authorized candidate preview, effect-boundary revalidation and whole-group atomic publication.
  Implemented metadata handlers can share staging now; paired split/Dart retain real recipe/geometry
  dependencies in .3. Native sewing EaseValidation explicitly defers realization to G2/G3.
- ID: `G1-SLICE.6.5.4.1`
  Status: `done`
  Goal:0427 private validated model/report preparation; [whole scope/proofs](../history/stitchcad-devnotes-part580.md).
- ID: `G1-SLICE.6.5.4.2`
  Status: `done`
  Goal: exact atomic group input/context ownership and sequential private staging with independent
  original reserve cursor and actual failing-command context, no partial accepted result.
- ID: `G1-SLICE.6.5.4.2.1`
  Status: `done`
  Goal:0428 exact typed group metadata; [whole scope/proofs](../history/stitchcad-devnotes-part581.md).
- ID: `G1-SLICE.6.5.4.2.2`
  Status: `done`
  Goal: sequential real private candidates with one independent staged reserve cursor; all-or-error
  group report preserving actual failing index/command and no publication/partial accepted result.
- ID: `G1-SLICE.6.5.4.2.2.1`
  Status: `done`
  Goal:0430 whole-group single-grant authority; [whole scope/proofs](../history/stitchcad-devnotes-part582.md).
- ID: `G1-SLICE.6.5.4.2.2.2`
  Status: `done`
  Goal:0431 private actual base/cursor; [whole scope/proofs](../history/stitchcad-devnotes-part583.md).
- ID: `G1-SLICE.6.5.4.2.2.3`
  Status: `done`
  Goal:0432 sequential metadata-group preview; [whole scope/proofs](../history/stitchcad-devnotes-part584.md).
- ID: `G1-SLICE.6.5.4.3`
  Status: `done`
  Goal:0433 private prepared-group ownership; [whole scope/proofs](../history/stitchcad-devnotes-part585.md).
- ID: `G1-SLICE.6.5.4.retention1`
  Status: `done`
  Goal:0429 D284 count-scope correction, no packing; [whole scope/proofs](../history/stitchcad-devnotes-part581.md).
- ID: `G1-SLICE.6.5.5`
  Status: `pending`
  Goal: scoped actual transaction/failure-context/rollback acceptance; full bus review .6.9 later.
- ID: `G1-SLICE.6.6`
  Status: `done`
  Goal: actual revision preconditions, idempotency replay and append-only actor audit trail.
- ID: `G1-SLICE.6.6.1`
  Status: `done`
  Goal:0434 atomic metadata commit/replay/audit; [whole scope/proofs](../history/stitchcad-devnotes-part586.md).
- ID: `G1-SLICE.6.7`
  Status: `done`
  Goal: bounded semantic undo/redo, original actor/revision restoration and explicit dropped-history report.
- ID: `G1-SLICE.6.7.1`
  Status: `done`
  Goal: explicit bounded metadata-group undo/redo with original semantic identities/revisions/actor,
  declared dropped-history reporting, fresh effect authority/current-source checks and atomic failure.
  Keep append-only original audit/replay independent; never rewind/reuse issued entity identities.
  Own D285: counter restoration must not let a new branch reuse an earlier snapshot revision.
  Reproduce the contract counterexample first; preserve separate maximum-issued revision through undo
  and allocate fresh edits above it. Native branch/stale-request proof is required before closure.
- ID: `G1-SLICE.6.7.1.1`
  Status: `done`
  Goal:0435 D285 diagnosis; [whole scope/proofs](../history/stitchcad-devnotes-part587.md); runtime verified0437, reopen pending.
- ID: `G1-SLICE.6.7.1.2`
  Status: `done`
  Goal:0436 typed history input; [whole scope/proofs](../history/stitchcad-devnotes-part588.md).
- ID: `G1-SLICE.6.7.1.3`
  Status: `done`
  Goal:0437 actual bounded undo/redo and D285 runtime fix/D286; [whole scope/proofs](../history/stitchcad-devnotes-part589.md).
- ID: `G1-SLICE.6.8`
  Status: `in_progress`
  Goal: typed progress/cancellation with no partial state/artifact; explicit resource bounds.
- ID: `G1-SLICE.6.8.1`
  Status: `done`
  Goal: explicit metadata input policy and actual pre-effect progress/cancellation; three declared
  input units are bounded, not total model memory/CPU or future numeric/file/geometry/solver work.
- ID: `G1-SLICE.6.8.1.1`
  Status: `done`
  Goal:0438 admission/progress data; [whole scope/proofs](../history/stitchcad-devnotes-part590.md).
- ID: `G1-SLICE.6.8.1.2`
  Status: `done`
  Goal:0439 runtime limits/progress/cancellation; [whole scope/proofs](../history/stitchcad-devnotes-part591.md).
- ID: `G1-SLICE.6.8.2`
  Status: `in_progress`
  Goal: derive and implement additional actual resource bounds from roadmap10/public host contracts;
  submitted input counts do not bound whole-model validation/cloning, retained journal/history bytes
  or future numeric/file/geometry work. Keep deeper/full verification in .6.5.5/.6.9 until feature closure.
- ID: `G1-SLICE.6.8.2.1`
  Status: `done`
  Goal:0440 evidence-reference admission/D287/D288; [whole scope/proofs](../history/stitchcad-devnotes-part592.md).
- ID: `G1-SLICE.6.8.2.2`
  Status: `in_progress`
  Goal: bound remaining concrete model/retained-state work through public immutable host contracts;
  do not label metadata input bounds total memory/CPU. First remove repeated source-host cloning.
- ID: `G1-SLICE.6.8.2.2.1`
  Status: `done`
  Goal:0441 bounded source-host copies/D289; [whole scope/proofs](../history/stitchcad-devnotes-part593.md).
- ID: `G1-SLICE.6.8.2.2.2`
  Status: `in_progress`
  Goal: remaining model/retained-state resource dimensions through actual public contracts;
  source-copy count does not bound per-copy size, validation/allocation work or retained journal bytes.
- ID: `G1-SLICE.6.8.2.2.2.1`
  Status: `done`
  Goal:0442 shared reserve/D290/D291; [whole scope/proofs](../history/stitchcad-devnotes-part594.md).
- ID: `G1-SLICE.6.8.2.2.2.2`
  Status: `in_progress`
  Goal: explicit actual dynamic-model/retained-state admission from public immutable contracts:
  cumulative collection entries, logical UTF-8 field bytes and unbounded counter bits. Preserve original
  model/authority failures, refuse before costly preparation, and distinguish logical content from RSS/time.
- ID: `G1-SLICE.6.8.2.2.2.2.1`
  Status: `done`
  Goal:0443 first-refusal logical resource contract; [whole scope/proofs](../history/stitchcad-devnotes-part595.md).
- ID: `G1-SLICE.6.8.2.2.2.2.2`
  Status: `in_progress`
  Goal: exhaustive actual core-model dynamic-content visitor with original typed locations and bounded
  precharge before nested traversal; cover every owned collection/text/counter field without inferred data.
- ID: `G1-SLICE.6.8.2.2.2.2.2.1`
  Status: `done`
  Goal:0444 actual canonical resource producer; [whole scope/proofs](../history/stitchcad-devnotes-part596.md).
- ID: `G1-SLICE.6.8.2.2.2.2.2.2`
  Status: `done`
  Goal: actual Piece/copy-plan/topology component accounting; exhaustive held views and bounded
  precharge retain authored fields/history. Whole-root/source/runtime admission remains separate.
- ID: `G1-SLICE.6.8.2.2.2.2.2.2.1`
  Status: `done`
  Goal:0445 actual Piece/copy resources/owner/D292; [whole scope/proofs](../history/stitchcad-devnotes-part598.md).
- ID: `G1-SLICE.6.8.2.2.2.2.2.2.2`
  Status: `done`
  Goal:0446 actual topology held resources; [whole scope/proofs](../history/stitchcad-devnotes-part599.md).
- ID: `G1-SLICE.6.8.2.2.2.2.2.3`
  Status: `done`
  Goal:0447 actual retained input resources; [whole scope/proofs](../history/stitchcad-devnotes-part600.md).
- ID: `G1-SLICE.6.8.2.2.2.2.2.4`
  Status: `in_progress`
  Goal: actual sewing/garment-family resource accounting with all nested held fields and original locations.
- ID: `G1-SLICE.6.8.2.2.2.2.2.4.1`
  Status: `done`
  Goal:0448 actual sewing graph/span/weighted-region/stop and semantic landmark resource producers.
  Exhaustive held views and fixed Copy guards retain graph/span owner/index, symbolic/profile intent
  and current anchors without copy/landmark lookup, resolution/rebirth, value read or ease realization.
  - [x] REPRODUCE / ISSUE: python3 -I -B native sewing schema assertions print span/weighted-region/
    stop vectors and fixed Notch/Turn logical/profile payloads rc=0. The original compile check stopped
    with E0432 because root public re-exports were missing; corrected exact export before accepted checks.
  - [x] ROOT CAUSE (WHY + WHERE): transition_receipts.py --record docs/history/stitchcad-devnotes-part600.md:
    3 exact Git spans rc=0; prior whole0447 scope/lesson/ledger retains its sewing/garment-pending boundary.
    Graph/spans/regions/stops and registry are actual independent held collections; fixed metadata adds none.
  - [x] FIX: cargo clippy -p sc-core --test design_resource_contract -- -D warnings:finished rc=0;
    exhaustive graph/span/ease/landmark private views, precharge before traversal and fixed payload
    guards count actual content without semantic/profile/physical work; root API exports are reachable.
  - [x] ADDRESSED: cargo test -p sc-core --test design_resource_contract:28 passed rc=0. Four actual
    cases verify3 spans/2 regions/5 stops/3 landmarks=13/0/0, all ease sources/distributions, Notch/Turn,
    spans0+3, stops3+1, regions4+2 and registry10+3 first owner/index refusals, actual deletion-held anchors
    without rebirth, unchanged graph/landmarks/ledger, empty scope and original prior-error preservation.
  - [x] NO REGRESSION: selected cargo fmt check/strict Clippy rc=0; git diff --quiet HEAD --
    crates/sc-core/src/command.rs crates/sc-core/src/command rc=0. Prior24 resource cases still pass;
    no source/authority/metadata bus execution path or physical/profile validation behavior changed.
  - [x] LOCKSTEP: whole0447 scope/lesson/ledger600 retained unchanged with3 exact Git spans rc=0;
    pointer/frontier/book/lesson/indexed decision aligned. Garment .2–.4, root .2.5/source .3/runtime .4,
    deeper .6.5.5 and full .6.9 remain owned; no full sewing/garment currentness or whole-model/RSS/time proof.
  Cargo invocations use scripts/local_environment.py. Promotion: indexed command-layer decision.
  Book build/native publication rc=0:81 chapters/159 scoped API rows/1479 source links/2481 rendered
  links; maintained reference82 files/18378 lines/1282263 bytes. Retention1054 records/84 working Markdown
  rc=0; Knowledge Map and git diff --check rc=0.
  Logical profile refs supply no selected style/encoding/dimensions, and Formula/Profile ease supplies
  no numeric result. Actual component counts supply no binding, geometry/physical/release authority.
- ID: `G1-SLICE.6.8.2.2.2.2.2.4.2`
  Status: `pending`
  Goal: actual Pocket/component, notion placement and Closure/instance resource accounting, with all
  held private fields and original source/copy intent, without rebirth/repair or physical inference.
- ID: `G1-SLICE.6.8.2.2.2.2.2.4.3`
  Status: `pending`
  Goal: actual Facing/modelled-Lining/Interfacing and Hem-family resource accounting; preserve deferred
  envelope/profile/Material intent and all held relationships without executing transformations.
- ID: `G1-SLICE.6.8.2.2.2.2.2.4.4`
  Status: `pending`
  Goal: actual Dart/Tuck/Pleat/Gather/Grainline/SeamAllowance-family resource accounting with exhaustive
  owner/definition/nested-field views and original field/owner/index refusals, without geometry realization.
- ID: `G1-SLICE.6.8.2.2.2.2.2.5`
  Status: `pending`
  Goal: compose actual core root accounting, including exact unbounded represented revision bits, under
  one cumulative meter; preserve scope-specific original first refusal and no mutation/large preparatory copy.
- ID: `G1-SLICE.6.8.2.2.2.2.3`
  Status: `pending`
  Goal: actual immutable source-host resource port and sc-measure producer through published readonly
  contracts; include all owned source collection/text/counter fields, no dishonest-host security claim.
- ID: `G1-SLICE.6.8.2.2.2.2.4`
  Status: `pending`
  Goal: install explicit immutable model policy and admit actual current/prospective metadata shapes
  before costly validation/cloning; retain original authority/cause/cancel/replay/undo and finite-state bounds.
  Bound logical model content and related retained-state composition; separate raw ingress, allocator peaks,
  per-host algorithm time and future numeric/file/decompression/geometry work with exact owners.

- ID: `G1-SLICE.6.9`
  Status: `pending`
  Goal: coupled actual bus acceptance/private-mutator compile failures; workflow registry feeds G1.9/G5 parity.

- ID: `G1-SLICE.6.9.publication`
  Status: `pending`
  Goal: after agreed command-feature implementation, run the complete existing publication probes,
  including named-result fixture compatibility, before push; current focused native formatter oracle
  does not claim this broader suite passed. COMMIT.md full pre-push gates remain mandatory.

- ID: `G1-SLICE.7`
  Status: `pending`
  Goal: `sc-store` (§4.5) — canonical-text project directory (stable key order, declared float
  formatting, ULID ids, no wall-clock), schema version + migrations, recoverable-unknown-extension
  preservation, crash-consistent atomic multi-file commits, autosave/recovery, and the repository
  trait with the named WASM (files + IndexedDB) backend.
  D285: persist/restore the maximum-issued revision separately from restored undo state; a reopened
  Design must never reissue an earlier snapshot revision. Restore common audit sequence too; native
  reopen/branch acceptance required. In-memory branch proof passes0437; persisted proof remains open.
  Typed formula expression/statement/recipe identity fields and digest domains must stay framed;
  .3f.1a decision requires cross-kind bind/recipe call-collision controls before storage or hashing.
  Acceptance: forced-interruption and failed-migration recovery are tested, not asserted; a
  saved project reopens to a semantically equal instance; two saves of the same state are
  byte-identical.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.8`
  Status: `pending`
  Goal: `sc-constraints` kernel (§6.1/§6.2) — typed constraint AST, finite-domain propagation +
  search with declared variable/value ordering, the four distinguished outcomes (satisfied /
  proven-unsatisfiable / unknown / numerically-failed), and the non-default `csp-z3` differential
  oracle feature wired but not shipped.
  Acceptance: determinism proven by repeated runs over the same input (fixed search order, no
  randomness, no wall-clock); `cargo build --features csp-z3` is a CI-only path and the default
  release artifact contains no Z3 dependency (verified by a dependency census).
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.9`
  Status: `pending`
  Goal: `sc-api` + `sc-mcp` (§7.8, §10) — versioned experimental command API over jsonrpsee
  (HTTP/WebSocket native, stdio for MCP), in-process Rust API distinct from the wire format,
  curated MCP tool set (workflows, not one tool per getter), scoped agent authority
  (inspect / propose / commit / generate / approve) enforced in core logic, actor trace on every
  mutating command, imported files treated as data.
  Acceptance: authority is enforced by tests that a scoped token cannot exceed (an `approve`
  attempt from a `propose` token fails); local stdio baseline and admission evidence for each
  enabled connection profile; the parity table's API column comes from the bus command list.
  Director 2026-10-01 reaffirmed comprehensive external-agent control through MCP/API. Curated
  workflows must expose every supported user operation with typed discovery, units, stable identities,
  current-state/provenance and actionable diagnostics. Evaluate discovery, invalid-input recovery,
  interruption/resume and parity using independent artifact checks (§7.8); API/MCP control is not a
  measured stitching-expertise claim. Existing human-only release approval remains §10's contract.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.9.doc`
  Status: `proposed`
  Goal: director2026-10-24 DBIQNP rustdoc adoption and mdBook-link requirements; no pivot.
  Plan: Cargo-generated public API reference; local artifacts; matching code/book revisions;
  mdBook links in local/published layouts and backlinks to semantic contracts.
  Official entry: https://doc.rust-lang.org/cargo/commands/cargo-doc.html.
  Acceptance: documented public interfaces/runnable examples/reference and link checks;
  deep semantic introspection through the clean API and MCP workflows covers
  identities, types/units, uncertainty, provenance, dependencies, diagnostics and authority.
  Runtime introspection/wire discovery retain separate typed contracts/tests under .9.
  Schedule: after D156 closure, before .9 signoff; discussion does not revise roadmap scope.

- ID: `G1-SLICE.9.plugin`
  Status: `proposed`
  Goal: director2026-10-24 DBIQNP StitchCAD plugin with chat and interactive UI control.
  Plan: API/MCP chat and UI share the bus, identities/revisions, preview/commit/undo and
  authority; human-only approval. .9 discovery; G5 host UI, conflict/recovery and parity tests.
  Host options/sources: docs/book/src/spec/command-layer.md#host-connection-options-under-review.
  Local/HTTP/bridge profiles follow §7.8/§10 admission evidence from G1, with host UI proof at G5.
  Director DBIQNP2026-10-25: all chosen UI surfaces shall be sleek, smooth, beautiful, efficient,
  engaging and precise, state-of-the-art and production-grade. Treat as observable acceptance:
  coherent visual system, readable units/hierarchy/accessibility, responsive navigation/picking,
  efficient keyboard/direct manipulation/undo/recovery and explicit precision feedback on realistic
  project/hardware fixtures. G1.13 validates hosting; G5.3 owns complete 2D UX; V1 owns staged3D.
  Roadmap already plans Tauri/TypeScript/React native-full and WASM web, with canvas topology open.
  Director accepted the recommendation2026-10-25: full standalone product UI is the baseline; chat UIs share the core/
  commands and can add embedded views. Host limits must not silently remove required workflows.
  Official OpenAI UI pages establish iframe web components/fullscreen canvases; Anthropic lists a
  Three.js3D connector. These are host examples, not measured StitchCAD GPU/performance/parity.
  Sources/qualification: docs/book/src/spec/command-layer.md#ui-surfaces-and-presentation-quality.
  Critical director requirement2026-10-25: desktop, web and2D/3D graphics are fully controllable by
  AI coding agents. Typed discoverable API/MCP workflows include view/camera/pan/zoom, selection,
  picking, layers/overlays, tools, measurements/editing, files/export, undo/recovery and result checks.
  Inspect document plus presentation state and stable object/view identities; observe completion,
  diagnostics and visual/artifact results. Verify end-to-end agent workflow parity on each admitted
  surface with declared authority; UI-only unreachable supported operations do not satisfy this bar.
  G1.9/.13 and G5 own control contracts/state visibility/real UI-agent tests; V1 owns staged3D coverage.
  Quality/baseline/full-control direction is adopted in decision_stitchcad-ui-baseline-and-quality.md;
  stack selection, implementation order and numerical frontier remain unchanged.


- ID: `G1-SLICE.9.plugin.t1`
  Status: `done`
  Goal: STITCHCAD-G1-0230 owns DBIQNP proposal/book/CH/DEV transport clarification.
  Acceptance: fetched official contracts, bounded docs and focused checks; no implementation,
  credentials, connection, roadmap or blocked-frontier change.
  Verified: book/publication45, command census0 failures, size OK; rc=0, protected transport-*.txt.
  Commit: STITCHCAD-G1-0230. LIVE categories and MEMORY next action unchanged.
  promotion: declined (scoped external compatibility research; reverify before adoption).

- ID: `G1-SLICE.9.transport`
  Status: `in_progress`
  Goal: admission policy and four implemented connection profiles; no runtime profile is certified yet.

- ID: `G1-SLICE.9.transport.policy`
  Status: `done`
  Goal: v0.4 MCP admission policy and D218 verified in0231; runtime profiles remain unapproved.
  Paths: ROADMAP.md;docs/book/src/spec/command-layer.md;docs/tasks/artifacts/command_layer/run_command_layer_census.sh;docs/tasks/artifacts/command_layer/run_command_layer_probes.sh
  Additional paths: docs/tasks/artifacts/glossary/run_glossary_probes.sh;docs/book/src/build-and-checks.md;docs/book/src/annexes/ledger-fixtures.md;docs/book/src/annexes/formula-static-signatures.md
  Acceptance: complete scope/six proof boxes retained in [part317](../history/window12.md#stitchcad-devnotes-part317md).

- ID: `G1-SLICE.9.transport.stdio`
  Status: `pending`
  Goal: G1 local-stdio baseline; owned process/identity/permissions/lifecycle and §7.1 admission tests.

- ID: `G1-SLICE.9.transport.http`
  Status: `pending`
  Goal: local-http; explicit loopback binding, origin/caller checks and §7.1 admission tests.

- ID: `G1-SLICE.9.transport.remote`
  Status: `pending`
  Goal: remote-http; TLS/authentication/revocation/isolation and §7.1 admission tests before enablement.

- ID: `G1-SLICE.9.transport.bridge`
  Status: `pending`
  Goal: private-bridge; end-to-end caller/relay/target scope, lifecycle and §7.1 admission tests.

- ID: `G1-SLICE.10`
  Status: `pending`
  Goal: `sc-cli` — headless command replay that is deterministic: the same command script produces
  the same project state and the same serialized bytes (§4.4; prerequisite for the G2 exit).
  Acceptance: a recorded session replays byte-identically on a second run; failures exit nonzero
  with structured diagnostics; no wall-clock or locale leaks into output.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.11`
  Status: `pending`
  Goal: the three runtime profiles build (§7.3): `native-full`, `native-headless`, `wasm-viewer`,
  each an explicit cargo feature/target combination with a published capability matrix, plus the
  CI WASM smoketest.
  Acceptance: one command per profile in `Makefile`, all green in CI; the capability matrix lists
  what each profile excludes and why; browser execution is tested for real, not `cargo check`.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.12`
  Status: `pending`
  Goal: the real browser spike (G1 exit) — load project → evaluate recipe → resolve one CSP
  constraint → persist/reopen → render → export, in the intended browser target, with the WASM
  capability matrix (tested versions + limitations) as its output.
  Acceptance: a recorded run (commands + observed output) demonstrates all six steps in a
  browser; limitations are written down (startup size, COOP/COEP, WebGPU baseline) and routed to
  G5 where they belong.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.13`
  Status: `pending`
  Goal: the canvas-hosting spike that settles ADR-0002 against the protocol written at
  `G0-CONTRACT.11` — zoom/pan, snapping, picking, annotations on a realistic pattern set, native
  AND browser, across the three topologies.
  Acceptance: the measurements the protocol named are recorded in
  `docs/tasks/artifacts/canvas_spike/results.tsv` — one row per applicable (topology, profile) pair, every
  gated metric filled, with the reference hardware, the OS versions and the corpus script's identity in the
  file's comment block, because a verdict is scoped to them; the corpus is the declared one (16 pieces,
  400 boundary vertices each, 200 pick probes, 200 snap probes, 1 000 fidelity round trips) or the
  deviation is recorded with its reason; the decision is whatever
  `bash docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh` prints, rule trace included, and a human
  overruling it does so by a recorded decision naming the rows it overrules (R6); ADR-0002's canvas half
  moves from `proposed` to `active` citing that output; and where R5 escalates, the fallback named in
  `spike_rule.tsv` is what ships until a re-spike.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.14`
  Status: `pending`
  Goal: the egui/iced dev shell for Stages 1–4 internal tools (ADR-0002's dev-shell ruling) —
  the harness the G2 viewer/editor and the conformance lab are built in, explicitly not the
  shipped product UX.
  Acceptance: the shell hosts a canvas and the command bus on both native and (where possible)
  browser; it is labeled a dev shell in the book so nobody mistakes it for the product.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.15`
  Status: `pending`
  Goal: the license census (G1 exit: "license-compatible solver builds on all targets") — every
  dependency's license recorded, copyleft excluded from the shipped tree per ADR-0001, solver
  builds on all three profiles.
  Acceptance: a re-runnable census command (dependency list + license fields) whose output is
  recorded in the leaf; any GPL/LGPL dependency is either absent or quarantined behind a
  non-default, non-shipped feature with the reason recorded.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.16`
  Status: `pending`
  Goal: G1 exit review — every exit clause cited against its artifact, the capability matrix
  published, the frontier handed to `G2-2D`.
  Acceptance: each clause is `met` with a re-runnable check or `not met` with a named blocker.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5b.4c.h2.b.p.o.i.k`
  Status: `done`
  Goal: completed immutable bounds proof; [whole scope/evidence](../history/window16.md#stitchcad-devnotes-part439md).

- ID: `G1-SLICE.5e.1a.h`
  Status: `done`
  Goal: completed handoff evidence/journal retained [whole in513](../history/window21.md#stitchcad-devnotes-part513md).
  Original first-capture identity remains unknown; current OS handoff contract remains in CLAUDE.md.
