# G1-SLICE: executable architecture slice (roadmap gate G1)

## Metadata

- Tree ID: `G1-SLICE`
- Status: `active`
- Roadmap lane: `ROADMAP.md` §11 gate **G1 — Executable architecture slice** (sources: §4
  architecture, §5 ADR-0001/0002, §6 constraint machinery, §7.3 runtime profiles, §7.8 API+MCP,
  §10 security)
- Created: `2026-09-29`
- Owner: repo-local workflow

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
  Goal: `sc-units` — fixed-point micrometre quantities (i64), the five tolerance classes as
  distinct types, unit conversions with explicit rounding, rejection of non-finite and
  dimensionally invalid expressions (implements the `G0-CONTRACT.2` spec).
  Acceptance: property tests for conversion round-trips and tolerance-class separation; a
  dimension error is a typed error, never a silent coercion; compiles for `wasm32-unknown-unknown`.
  Verification: recorded below — `sc-units` was delivered in full by `G0-CONTRACT.18` (commit
  `eb83f01`); every acceptance criterion re-derived by command in the `### G1-SLICE.2` checklist,
  and the property-test-framework choice recorded as a layer-C decision.
  Commit: `STITCHCAD-G1-0002`

- ID: `G1-SLICE.3a`
  Status: `done`
  Goal:closed scope; [whole proof](../history/stitchcad-devnotes-part526.md).
- ID: `G1-SLICE.3b`
  Status: `done`
  Goal:closed scope; [whole proof](../history/stitchcad-devnotes-part526.md).
- ID: `G1-SLICE.3c`
  Status: `done`
  Goal:closed structural ontology scope; [whole proof](../history/stitchcad-devnotes-part531.md).
  Children: `.3c.1`, `.3c.2`, `.3c.3`, `.3c.4`; geometric proof remains G2.

- ID: `G1-SLICE.3c.1`
  Status: `done`
  Goal: Structural Piece contract complete; [whole scope/proofs](../history/window20.md#stitchcad-devnotes-part485md).
- ID: `G1-SLICE.3c.1a`
  Status: `done`
  Goal: fix D56 — the shipped piece cut plan cannot represent the reference skirt's separate
  cut-once L/R back members. Add explicit handedness and companion-piece identity while retaining
  the existing total-quantity mirrored-pair mode. Printed labels derive the member's L/R information.
  Acceptance: cut-once left/right pieces can carry reciprocal companion identities; missing hand or
  companion is unrepresentable, self-companion is refused, existing even-quantity mirrored pairs
  remain valid and odd totals in that mode remain refused. No geometric mirror claim is invented;
  companion existence/reciprocity is a design-collection obligation at `.6`.
  Verification: 14 piece-contract tests, strict `make check`, wasm, book, fixture/feature censuses,
  ledger probes and staged doctrine gate green. D56 closed in defects-part3.
  Commit: `STITCHCAD-G1-0008`
  Design: `decision_piece-pair-members-have-explicit-handedness.md`.

- ID: `G1-SLICE.3c.2`
  Status: `done`
  Goal: first discharge D55 with full-range resolution/repair (endpoints cannot certify an interior);
  then `SeamSpan` and immutable `SewingGraph`, partial and one-to-many edge ranges, declared ease
  distribution, direction and stop landmarks; resolve D35's same-piece seam rule against the supported
  dart and trouser constructions before code.
  Acceptance: a deleted interior fragment blocks full-range integrity even when endpoints resolve;
  absent pieces/edges, empty or reversed parameter ranges, duplicate span identities and
  missing stop references are typed refusals; same-piece seams have an explicit tested contract;
  edits expose repairs without rewriting authored ranges. Geometric differential checks remain G2.
  Verification: 18 sewing contracts + existing range/copy/notch suites, strict check, wasm,
  book, fixture/feature/glossary censuses, ledger probes and staged doctrines green. D35/D57 closed.
  Commit: `STITCHCAD-G1-0011`

  Children: `.3c.2a` (full-range resolution), `.3c.2b` (sewing graph and D35).

- ID: `G1-SLICE.3c.2a`
  Status: `done`
  Goal: discharge D55 by folding the entire positive-length interval through the identity journal;
  expose every surviving fragment, deleted/trimmed interval and arithmetic refusal, preserve traversal
  order and never rewrite the authored range. Piece full-edge queries consume this contract.
  Acceptance: D55's middle deletion produces a visible range repair despite resolved endpoints;
  partial/full ranges survive split/merge/reverse exactly, offset gaps stay visible, every diagnostic
  names the held range and orphaning operation, and undeclared edges are refused. Endpoint point
  ambiguity stays governed by `.3b`; full interval coverage is not a geometric or release certification.
  Verification: 13 range-contract tests + existing suites green; strict clippy, wasm, book,
  feature census, ledger probes and staged doctrine gate green. D55 closed in defects-part2.
  Commit: `STITCHCAD-G1-0007`
  Design: `decision_range-resolution-preserves-entire-interval.md`, recorded before code.

- ID: `G1-SLICE.3c.2b`
  Status: `done`
  Goal: implement the sewing graph content and D35 after `.3c.2a` supplies its range contract.
  Director decision (D57), `2026-10-01`: every physical cut copy has a stable identity, so its
  seams can differ. Implement this explicit addressing contract before sewing spans; do not expand
  an underspecified pattern-only graph by renderer convention. D57 closes after verified code.
  Acceptance: all `.3c.2` sewing-graph criteria met; range and endpoint ambiguity visible; same-piece
  seams explicitly specified and tested. Then close the `.3c.2` parent.
  Verification: 18 sewing contracts + existing range/copy/notch suites, strict check, wasm,
  book, fixture/feature/glossary censuses, ledger probes and staged doctrines green. D35/D57 closed.
  Commit: `STITCHCAD-G1-0011`

  Children: `.3c.2b.1` (explicit cut-copy plan), `.3c.2b.2` (sewing spans and D35).

- ID: `G1-SLICE.3c.2b.1`
  Status: `done`
  Goal: Structural sewing-side primitives complete; whole source/contracts/proofs [retained](../history/window20.md#stitchcad-devnotes-part484md).
- ID: `G1-SLICE.3c.2b.2`
  Status: `done`
  Goal: completed copy-addressed sewing contract; [whole scope/proofs](../history/window15.md#stitchcad-devnotes-part430md).

- ID: `G1-SLICE.3c.3`
  Status: `done`
  Goal: `Notch`, directed and dual-reference `Grainline`, per-edge derived `SeamAllowance`; profile
  parameters are identity references rather than invented defaults until G4 supplies parameter states.
  Acceptance: [complete scope/verification](../history/window12.md#stitchcad-devnotes-part328md).
  Children: `.3c.3a` (notches), `.3c.3b` (grainlines), `.3c.3c` (allowances).

- ID: `G1-SLICE.3c.3a`
  Status: `done`
  Goal: semantic `Notch` with stable piece/edge/parameter anchoring and symbolic Factory Profile
  bindings for style, sample/production depth/width and encoding. Bindings carry no scalar values;
  their validation is visibly `DeferredToG4`, rather than treating an unread profile as resolved.
  Acceptance: [complete scope/verification](../history/window12.md#stitchcad-devnotes-part328md).

- ID: `G1-SLICE.3c.3b`
  Status: `done`
  Goal: directed `Grainline`, explicit parallel/off-grain angle binding and independent stripe/plaid
  references, with born-valid anchors and visible resolution/repair after topology edits.
  Acceptance: [complete scope/verification](../history/window12.md#stitchcad-devnotes-part328md).

- ID: `G1-SLICE.3c.3c`
  Status: `done`
  Goal: per-edge derived allowance descriptors, authored width-parameter origin, explicit corner choice
  and symbolic per-profile inclusion policy; then close the marks/allowances parent.
  Acceptance: [complete scope/verification](../history/window12.md#stitchcad-devnotes-part328md).


- ID: `G1-SLICE.3c.4`
  Status: `done`
  Goal: semantic `Dart`/`Tuck`/`Pleat`/`Gather`, `Hem`, `Facing`/`Lining`/`Interfacing`, `Closure` and
  `Pocket` with required content and validated structural references; close the object-type parent.
  Acceptance: required anchors, operation identities, composition and parameter references are carried;
  buttonhole size derives from its button rather than a second input; unsupported constructions are
  refused explicitly; intake conservation stays visibly deferred to G2. Every ontology §4 object has
  implementation evidence, synchronized book content and wasm-safe tests.
  Verification: object/support suites 149 contracts; sc-core 260 tests incl. 18 docs; full strict
  Rust/WASM/book, 22 probe suites, focused censuses/ledger and staged doctrine gates green.
  Commit: `STITCHCAD-G1-0023`

  Children: `.3c.4a` (intake constructions), `.3c.4b` (hem/layers), `.3c.4c` (closures),
  `.3c.4d` (pockets and family signoff). Safe child slices preserve the entire §4.3/§4.7 scope.

[Completed construction child contracts and acceptance evidence](G1-SLICE-constructions.md)
are preserved unchanged in a bounded sibling; .3c.4 remains closed by STITCHCAD-G1-0023.

- ID: `G1-SLICE.4`
  Status: `done`
  Goal: `sc-measure` — MeasurementTable (body vs garment POM, landmarks, source, procedure),
  Ease as a first-class body→garment mapping with fit intent, and SizeSet (implements
  `G0-CONTRACT.4`/`.6`).
  Acceptance: a POM without a landmark or procedure is rejected; ease is queryable per POM;
  size labels vs order vs base size are distinct fields with tested semantics.
  Children: .4a (measurement declarations/tables), .4b (per-POM ease), .4c (SizeSet), .4d (signoff).
  Verification: child contracts plus0321 measurement16/table16/Ease14,0320 model25 and unchanged composite31; publication74/87 pass0.
  Commit: STITCHCAD-G1-0321 (structural library); runtime/application/registry proof remains G1/G3/G4 work.

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
  Children: .4c.1 (membership), .4c.2 (axes), .4c.3 (chart/breaks), .4c.4 (resolution), .4c.5 (review).
  Goal: SizeSet identity/revision/system, authored ordered labels/base, chart/state/provenance, adjacent
  breaks and axes; immutable profile resolution/transformation intent per canonical SizeSet decision.
  Acceptance: label/order/base distinct; path-2 missing-break refusal, MTM of one, no quantities/defaults;
  profile transformations never mutate Design references; execution/equivalence/approval later owned.
  Split into safe slices before implementation; preserve every size-sets chapter requirement.
  Verification: `pending`
  Commit: `pending`

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
  Children: .4c.3a (garment chart observations), .3b (chart collection), .3c (MTM body inputs),
  .3d (breaks/composite SizeSet), .3e (review). .3a is independent of axis implementation.
  Goal: complete SizeSet chart/break object over membership/axes, canonical per-member POM scalars,
  landmarks/procedures/state/provenance and adjacent signed breaks; current references and path readiness.
  Acceptance: chart and generated POMs distinct; missing breaks name dimension/pair and refuse path 2;
  zero/uneven breaks legal; MTM of one uses body provenance and only path 1; no quantity/default values.
  Verification: chart/MTM originals and0303–0310 focused declaration/coverage/composite evidence; scope closure .3e.
  Commit: original0033–0035 and STITCHCAD-G1-0303–0310 structural scope.

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
  Goal: immutable resolved SizeSet/profile transformation intent, pinned Design reference/profile revision,
  typed label mapping/break scaling/base substitution/chart replacement with state/evidence provenance.
  Acceptance: new resolved object retains original reference and records transformation; no in-place
  mutation. Execution, precedence/hard restrictions, equivalence and release approval retain later owners.
  Verification:0311–0317 focused state/binding/aggregate contracts,0318 unchanged-code/book/anchor review pass0.
  Commit: STITCHCAD-G1-0311–0318 (includes due cleanup/D232 repair); actual G4 execution remains separate.

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
  Goal: [whole completed subtree/0344 proof](../history/stitchcad-devnotes-part496.md).
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
  Children: `.5a.1` lexical stream, `.5a.2` expression trees, `.5a.3` literal/canonical recipes,
  `.5a.4` syntax review. Finalize each protocol after reading its complete affected contracts.
  Goal: expression-language syntax from the full formula contract/grammar/examples. Read all three
  before finalizing a pre-code protocol and safe parser subleaves; introduce immutable syntax and
  precise source diagnostics without claiming evaluation, generated geometry or resolved SizeSet.
  Acceptance: syntax, keywords, literals/units, precedence/conditionals, structural bounds and
  rejected forms conform to the normative language; canonical/display forms and source spans are
  explicit. Type/name binding, exact evaluation/DAG and operation recipes remain .5's later children.
  Verification: full syntax/input/identity map; native591/WASM3/probes25/coupled7 actual reds, rc=0.
  Commit: `STITCHCAD-G1-0072`.

Completed syntax subtree .5a.1–.5a.3f.2 is preserved verbatim in
[G1-SLICE-recipes](G1-SLICE-recipes.md#completed-syntax-node-graph--preserved-during-g1-0072).
Whole proof evidence: [G1-SLICE-checked-recipes](G1-SLICE-checked-recipes.md).

Retention evidence: [G1-SLICE-retention-proof](G1-SLICE-retention-proof.md).

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
- ID: `G1-SLICE.6.4`
  Status: `in_progress`
  Goal: privately mutated design aggregate; actual reference/pair/sewing/pocket/closure/layer readiness.
  Children .1 complete Piece inventory; .2 owned canonical state; .3 point/range readiness;
  .4 sewing/current symbolic amounts; .5 garment families; .6 focused coupled aggregate/privacy acceptance.
- ID: `G1-SLICE.6.4.1`
  Status: `done`
  Goal:0345 complete Piece companion checks; [whole scope/proof](../history/stitchcad-devnotes-part497.md).
- ID: `G1-SLICE.6.4.2`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.a`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.b`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.c`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.1`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.2`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.2.a`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.2.b`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.3`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.3.1`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.3.2`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.3.3`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.3.4`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.1`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.2`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.3`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.3.a`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.3.b`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.3.c`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.3.c.1`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.3.c.2`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.3.c.3`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.3.c.3.a`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.3.c.3.b`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.3.c.3.c`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.4`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.4.a`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.4.b`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.4.c`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.2.d.4.4.d`
  Status: `done`
  Goal:0346–0368 input/source scope; [whole subtree/proof](../history/stitchcad-devnotes-part520.md).
- ID: `G1-SLICE.6.4.3`
  Status: `done`
  Goal:0369 combined structural topology readiness; [whole scope/proof](../history/stitchcad-devnotes-part521.md).
- ID: `G1-SLICE.6.4.4`
  Status: `done`
  Goal:closed scope; [whole proof](../history/stitchcad-devnotes-part526.md).
- ID: `G1-SLICE.6.4.4.1`
  Status: `done`
  Goal:closed scope; [whole proof](../history/stitchcad-devnotes-part526.md).
- ID: `G1-SLICE.6.4.4.2`
  Status: `done`
  Goal:closed scope; [whole proof](../history/stitchcad-devnotes-part526.md).
- ID: `G1-SLICE.6.4.4.3`
  Status: `done`
  Goal:closed scope; [whole proof](../history/stitchcad-devnotes-part526.md).
- ID: `G1-SLICE.6.4.4.3.a`
  Status: `done`
  Goal:closed scope; [whole proof](../history/stitchcad-devnotes-part526.md).
- ID: `G1-SLICE.6.4.4.3.b`
  Status: `done`
  Goal:closed scope; [whole proof](../history/stitchcad-devnotes-part526.md).
- ID: `G1-SLICE.6.4.4.4`
  Status: `done`
  Goal:closed scope; [whole proof](../history/stitchcad-devnotes-part526.md).
- ID: `G1-SLICE.6.4.5`
  Status: `in_progress`
  Goal: current pocket/closure/notion/layer/declaration sources with explicit geometric deferrals.
  Own garment child planning, original pocket/notion anchor error chains and continuity526.
  Children .1 owned/current Pockets; .2 notion/Closure instances; .3 layers/Hem; .4 other construction
  sources; .5 coupled root ownership/current declarations; .6 focused complete component acceptance.
  Keep existing G3/G4/typed-operation owners explicit; numeric ids cannot invent logical metadata.
- ID: `G1-SLICE.6.4.5.1`
  Status: `done`
  Goal:closed Pocket scope; [whole proof](../history/stitchcad-devnotes-part528.md).
- ID: `G1-SLICE.6.4.5.1.a`
  Status: `done`
  Goal:closed Pocket scope; [whole proof](../history/stitchcad-devnotes-part528.md).
- ID: `G1-SLICE.6.4.5.1.b`
  Status: `done`
  Goal:closed Pocket scope; [whole proof](../history/stitchcad-devnotes-part528.md).
- ID: `G1-SLICE.6.4.5.2`
  Status: `done`
  Goal: actual notion/Closure collection, every instance/placement and known declaration roles.
  Own Closure causes D260 and continuity528.
  Children .a original nested Closure errors; .b actual notion placements; .c actual Closure/instance
  inventory; .d known declaration roles with existing operation/domain/G3/G4 boundaries.
- ID: `G1-SLICE.6.4.5.2.a`
  Status: `done`
  Goal:0377 D260 original Closure causes; [whole scope/proof](../history/stitchcad-devnotes-part529.md).
- ID: `G1-SLICE.6.4.5.2.b`
  Status: `done`
  Goal:0378 actual notion collection; [whole scope/proof](../history/stitchcad-devnotes-part530.md).
- ID: `G1-SLICE.6.4.5.2.c`
  Status: `done`
  Goal:0379 actual Closure/instance collection; [whole scope/proof](../history/stitchcad-devnotes-part531.md).
- ID: `G1-SLICE.6.4.5.2.d`
  Status: `done`
  Goal: actual known Closure declaration roles; preserve existing logical domain/operation deferrals.
  Own borrowed zipper length, hook/bar/button size and canonical hole-source views over actual
  ClosureInventory and DesignInputs. Formula length requires actual parameter metadata/Length kind;
  explicit fields borrow original intent, logical size/operation/Profile roles stay G3/G4/recipe-owned.
  Complete root first; no numeric reads, defaults or geometry proof. Global identity joins remain .5.
  Focused actual cross-crate/native/privacy/lint/book; whole0379 continuity531.
  - [x] REPRODUCE / ISSUE: python3 Git-source check:9a4357c has no Closure declaration views rc=0.
  - [x] ROOT CAUSE (WHY + WHERE): same check rc=0; original descriptors retain intent without
    actual Design parameter/Length-kind joins. Logical size/operation schemas remain separate.
  - [x] FIX: cargo clippy -p sc-core --lib -- -D warnings and -p sc-measure
    --test design_inputs_contract -- -D warnings:rc=0 each; original borrowed field roles.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract closure_declarations_:
    4/0 rc=0; cargo test -p sc-core --doc design::closures:1/0 rc=0.
  - [x] NO REGRESSION: cargo fmt --all --check:rc=0; original Closure/inventory/sewing bytes
    identical rc=0; glossary132 failures rc=1, exact prior D250 rows compare rc=0.
  - [x] LOCKSTEP: bash scripts/run_make.sh book:rc=0; scoped publication79/148/1454/2405 rc=0;
    transition_receipts.py --record docs/history/stitchcad-devnotes-part531.md:7 exact spans rc=0.
    promotion: declined (settled source/physical boundaries). Completed STITCHCAD-G1-0380.
- ID: `G1-SLICE.6.4.5.3`
  Status: `pending`
  Goal: actual layers/Hem current sources, materials and known declaration roles; explicit lining refusal.
- ID: `G1-SLICE.6.4.5.4`
  Status: `pending`
  Goal: actual Dart/fold/Gather/grain/allowance references and known typed declarations.
- ID: `G1-SLICE.6.4.5.5`
  Status: `pending`
  Goal: couple actual garment ownership with single-root state and complete owned identity scope.
- ID: `G1-SLICE.6.4.5.6`
  Status: `pending`
  Goal: focused complete current garment/component acceptance before .6.4.6 aggregate review.
- ID: `G1-SLICE.6.4.6`
  Status: `pending`
  Goal: focused coupled aggregate and public-mutator privacy acceptance; full runtime integration later.
- ID: `G1-SLICE.6.vocabulary`
  Status: `pending`
  Goal: P2 D250 repair preexisting public API/variant/local token declarations by canonical meaning;
  schedule after .6.9 implementation, before G1 .11/integration/pre-push. Current .6.4.1 adds no C1 rows.
  Scope includes separate P3 advisory awk/head pipe-close diagnostic cleanup after C1 closure.
- ID: `G1-SLICE.6.5`
  Status: `pending`
  Goal: nonmutating preview/atomic commit and semantic group rollback with actual failing-command context.
- ID: `G1-SLICE.6.6`
  Status: `pending`
  Goal: actual revision preconditions, idempotency replay and append-only actor audit trail.
- ID: `G1-SLICE.6.7`
  Status: `pending`
  Goal: bounded semantic undo/redo, original actor/revision restoration and explicit dropped-history report.
- ID: `G1-SLICE.6.8`
  Status: `pending`
  Goal: typed progress/cancellation with no partial state/artifact; explicit resource bounds.
- ID: `G1-SLICE.6.9`
  Status: `pending`
  Goal: coupled actual bus acceptance/private-mutator compile failures; workflow registry feeds G1.9/G5 parity.

- ID: `G1-SLICE.7`
  Status: `pending`
  Goal: `sc-store` (§4.5) — canonical-text project directory (stable key order, declared float
  formatting, ULID ids, no wall-clock), schema version + migrations, recoverable-unknown-extension
  preservation, crash-consistent atomic multi-file commits, autosave/recovery, and the repository
  trait with the named WASM (files + IndexedDB) backend.
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
  Goal: completed handoff evidence/journal retained [whole in513](../history/stitchcad-devnotes-part513.md).
  Original first-capture identity remains unknown; current OS handoff contract remains in CLAUDE.md.
