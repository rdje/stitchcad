# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-10-02)_ — lexical source is borrowed and expression authority stays separate

- FormulaLexer borrows exact machine source and produces immutable text/span/kind tokens. One shared
  private spelling/keyword classifier preserves MachineToken public errors. ASCII preflight occurs
  before any token; token/error iteration fuses. Debug/errors omit source, while lexeme text is explicit.
- Successful lexing is deliberately weaker than valid expression/recipe: adjacent atoms, comments,
  unsupported calls/units/powers and type/name/numeric bounds stay with the parser/checker/evaluator.
  Leading zeroes/precision and whitespace gaps are retained for later exact literal/canonical work.
- The gap contract found D74: Rust's ASCII-whitespace helper omits vertical tab. Explicit handling and
  a real guard mutation verify the fix. D73 stale ADR clause references now target actual examples and
  exclusions. The mutation runner initially rejected custom assertion text lacking its output marker;
  named assertion messages correct the evidence. Strict lint exposed test indexing/helper expect and
  iterator style; safe access and a borrowing iterator correct the harness without loosening lints.
- Thirteen lexical contracts/two privacy-lifetime docs, nine actual production assertion reds and
  restored native/WASM checks pass. Book source/rendered/status/index checks cover 48 chapters/15 APIs.
  Checks use repository-local Cargo/scratch storage; installed toolchains remain read-only dependencies.
- Completed publication evidence and oldest live ledger payloads retain predecessor bytes; D73/D74
  seal with logged descriptions unchanged. No cap changes, remote-CI or recipe-execution claim.
- promotion: declined (routine syntax subleaf; canonical language/identity decisions remain ADR-0003).

## _(2026-10-02)_ — teach progressively while verifying public implementation scope

- The director requires roadmap/code/book lockstep, incremental teaching and direct expert routes.
  Five learning chapters precede detailed annexes; the existing glossary and new complete topic index
  remain independent navigation surfaces. Original chapter URLs/anchors and normative rules stay intact.
- Fourteen public-API rows bind book chapters, code, roadmap clauses and task owners. Source/rendered
  navigation and chapter/index coverage are checked, with eight copied-fixture refusal mutations.
  These are scoped structural/status proofs; prose semantics and later execution/approval still need
  their canonical contracts and evidence. D71 corrects old G0-only/Ease-future statements.
- The full milestone exposed D72: history growth made a fixed resident-limit fixture exceed the decoded
  limit first. Production validation was correct. A minimal independent archive, matched positive
  control and resident overflow arm fix the test; 28 archive arms and all 24 suites pass.
- Roadmap growth is contained by retaining D32's exact older explanation in its canonical decision;
  951 lines/53129 bytes remain within the original 951/53153 baseline. Map input is compacted rather
  than raising its cap. Completed MTM and sealed payloads preserve exact predecessor bytes.
- Browser local-file policy blocks screenshot review; source/rendered HTML links and table cells are
  inspected without a policy bypass. No visual browser or new runtime/remote-CI verdict is claimed.
- promotion: promoted by `decision_book-progression.md` (fresh answers/director publication requirement).

## _(2026-10-02)_ — MTM charts preserve distinct body inputs and garment targets

- A custom sole-member chart pins its canonical Ease-set reference snapshot, not numeric/fit/state
  copies. Current sources, uncertainty and mapping intent are borrowed; retargeting requires explicit
  chart replacement. Full coverage is measured against the current Design garment inventory.
- Body, signed Ease amount and garment POM metadata remain different roles. Unknown/derived inputs
  refuse numeric queries; a chart supplies no body-plus-Ease result and no grade-rule path. Structural
  completeness does not establish numeric, physical, fit or release readiness.
- Fifteen contracts/two privacy-role docs and twelve actual source guard/fallback mutations pass;
  strict 439 tests/WASM/book and focused checks are green. A fixture initially expected an extra
  terminal error source; exact typed wrappers correct it. Explicit fixture panic is replaced with a
  typed assertion under the unchanged strict lint. Production diagnostics were already correct.
- Main book prose introduces a waist example; API/currentness/verification moves to an annex. The
  director's publication requirement is owned next by .4d.1, including D71's stale G0 landing status.
  Completed .3b and oldest history payloads retain predecessor bytes; D70 axes remains pending.
- promotion: declined (routine MTM contract; publication requirement stays owned for .4d.1 adoption).

## _(2026-10-02)_ — chart coverage is measured against the current Design inventory

- A chart's explicit targets cannot certify their own completeness: full coverage requires every
  current Design garment POM and one valid canonical observation per authored member/POM cell.
  Draft validation checks existing targets; row queries check their declared cells. Complete validation
  also checks the full named Design table, including valid metadata for its non-chart body inputs.
- Saved observation reference targets refuse retargeting; current correspondence provenance and
  scalar source/state remain borrowed. Explicitly shared measurement inputs use separate member
  observations. Structural completeness accepts unresolved values but numeric queries refuse defaults.
- Eighteen contracts/privacy and fourteen real production mutation reds verify coverage/currentness,
  including reduced targets, valid duplicate cells and a zero fallback. Strict lint rejected a fixture
  integer truncation cast; checked conversion fixes it. Restored strict 422 tests/WASM/book and focused
  checks pass. Completed observations preserve predecessor evidence; MTM/axes/breaks remain owned.
- promotion: declined (routine current-reference/coverage implementation; book/task own the contract).

## _(2026-10-02)_ — chart correspondence retains authored and generated measurement roles

- An authored observation pins the member/set revision, named tables and two garment bindings.
  Tables validate current measurement metadata; observation bindings also refuse explicit table
  retargeting from transferring old intent. Selected-role queries do not certify the other role;
  declaration/numeric queries validate both. Same-label/member or same-content/table peers never substitute.
- Design input POMs identify logical quantities, not regenerated geometry results. Base authored
  inputs may serve both roles without duplicating scalar/state. G3 still creates distinct generated
  measurements and proves physical equivalence; mapping provenance alone cannot establish that truth.
- Sixteen contracts/privacy and eight actual assertion reds verify domains/currentness/identity;
  restored strict 403 tests, WASM/book and focused censuses pass. Unknown/derived values remain readable
  but refuse numeric defaults. Completed membership retains exact predecessor evidence; rolling seals
  preserve historical bytes. D70 axes ruling remains independent and pending; .3b coverage follows.
- promotion: promoted by `decision_size-set-ownership.md`'s chart observations section.

## _(2026-10-02)_ — size labels, member identity and sequence are separate data

- Membership stores pinned set id/revision, explicit system, ordered stable members and one base id.
  Human labels preserve exact Unicode/case/spacing; no token or numeric semantics are inferred. Missing
  ids never transfer to a same-label replacement. Explicit validated reconstruction preserves originals.
- Nonempty/unique membership and base existence are structural requirements. Count successor refuses
  overflow but does not replace current-registry/command revision checks. Single custom membership is
  not yet an MTM-ready body chart; quantities, chart and axes are separate owned contracts.
- Twelve contracts/three privacy-quantity docs and seven real guard/order mutations pass; exact restore,
  strict 386 tests/WASM/book verify the foundation. D70's source census pinpoints contradictory axes
  cardinality in size-sets .2/.7; director ruling requested before .4c.2, no representation defaulted.
- promotion: promoted by `decision_size-set-ownership.md`'s membership foundation section.

## _(2026-10-02)_ — structural Ease review retains independent proof boundaries

- Every ontology .2.2 field maps to current API/contracts; sets enforce per-POM identity and table
  membership. Canonical uncertainty/provenance and explicit compression remain distinct from truth.
- Current measure tests/docs pass 65; full 23-suite milestone/book pass with product code unchanged
  from b4e0bc7 strict Rust/WASM. Completed set contract/evidence relocates unchanged. .4b closes
  structurally; SizeSet and combined review remain. Physical/evaluation/release proofs are still owned.
- promotion: declined (routine milestone review; results owned by book/task, canonical decisions unchanged).






# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/window1.md#stitchcad-devnotes-part1md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`devnotes-part2.md`](docs/history/window1.md#stitchcad-devnotes-part2md) | the two oldest `2026-09-30` lessons (enumeration, and the vocabulary census) | 62 lines, 5915 bytes, `sha256:edcd0808…` |
| [`devnotes-part3.md`](docs/history/window1.md#stitchcad-devnotes-part3md) | two `2026-09-30` lessons (two tables, one garment; a fixture internally right) | 50 lines, 4723 bytes, `sha256:fcca661d…` |
| [`devnotes-part4.md`](docs/history/window1.md#stitchcad-devnotes-part4md) | two `2026-09-30` lessons (a blocked leaf splits; permission is no criterion) | 42 lines, 3706 bytes, `sha256:c2ac5791…` |
| [`devnotes-part5.md`](docs/history/window1.md#stitchcad-devnotes-part5md) | two `2026-09-30` lessons (a rule whose only path is "don't"; a digest is about bytes) | 43 lines, 3977 bytes, `sha256:859ce981…` |
| [`devnotes-part6.md`](docs/history/window1.md#stitchcad-devnotes-part6md) | two `2026-09-30` lessons (a spec's tables are its test suite; settle it with the artifact) | 55 lines, 5359 bytes, `sha256:129d50d8…` |
| [`devnotes-part7.md`](docs/history/window1.md#stitchcad-devnotes-part7md) | three `2026-09-30` lessons (an arm that removes the rule; a synthetic input is a fixture; a RED arm asserts the refusal) | 57 lines, 5120 bytes, `sha256:13fd6c73…` |
| [`devnotes-part8.md`](docs/history/window1.md#stitchcad-devnotes-part8md) | two `2026-09-30` lessons (source layout; i18n population) | 35 lines, 3196 bytes, `sha256:04ab285c…` |
| [`devnotes-part9.md`](docs/history/window1.md#stitchcad-devnotes-part9md) | the `2026-09-30` certifying-artifact lesson | 15 lines, 1343 bytes, `sha256:bc7fae65…` |
| [`devnotes-part10.md`](docs/history/window1.md#stitchcad-devnotes-part10md) | the `2026-09-30` shipped-work reconciliation lesson | 15 lines, 1380 bytes, `sha256:701d33f2…` |
| [`devnotes-part11.md`](docs/history/window1.md#stitchcad-devnotes-part11md) | the `2026-09-30` property-test framework lesson | 15 lines, 1384 bytes, `sha256:ae04eadf…` |
| [`devnotes-part12.md`](docs/history/window1.md#stitchcad-devnotes-part12md) | ontology slice decomposition | 16 lines, 1570 bytes, `sha256:a2f04e3d…` |
| [`devnotes-part13.md`](docs/history/window1.md#stitchcad-devnotes-part13md) | injected identity lesson | 18 lines, 1612 bytes, `sha256:38e83349…` |
| [`devnotes-part14.md`](docs/history/window1.md#stitchcad-devnotes-part14md) | persistent-identity lesson | 24 lines, 2230 bytes, `sha256:2b6aebd3…` |
| [`devnotes-part15.md`](docs/history/window1.md#stitchcad-devnotes-part15md) | structural-piece lesson | 15 lines, 1334 bytes, `sha256:1b362d26…` |
| [`devnotes-part16.md`](docs/history/window1.md#stitchcad-devnotes-part16md) | interval-coverage lesson | 13 lines, 1183 bytes, `sha256:fcf7c475…` |
| [`devnotes-part17.md`](docs/history/window1.md#stitchcad-devnotes-part17md) | separate-pair-member lesson | 12 lines, 1049 bytes, `sha256:140c4c41…` |
| [`devnotes-part18.md`](docs/history/window1.md#stitchcad-devnotes-part18md) | semantic-anchor/profile-binding lesson | 12 lines, 1102 bytes, `sha256:61a13500…` |
| [`devnotes-part19.md`](docs/history/window1.md#stitchcad-devnotes-part19md) | physical-copy identity lesson | 18 lines, 1663 bytes, `sha256:c0e3c442…` |
| [`devnotes-part20.md`](docs/history/window1.md#stitchcad-devnotes-part20md) | physical sewing-interval lesson | 15 lines, 1375 bytes, `sha256:34867dc9…` |

| [`devnotes-part21.md`](docs/history/window1.md#stitchcad-devnotes-part21md) | directed-grainline lesson | 13 lines, 1212 bytes, `sha256:c31c3298…` |

| [`devnotes-part22.md`](docs/history/window1.md#stitchcad-devnotes-part22md) | per-edge allowance lesson | 13 lines, 1192 bytes, `sha256:1807ae98…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

| [`devnotes-part23.md`](docs/history/window1.md#stitchcad-devnotes-part23md) | tuck/pleat and dart lessons | 25 lines, 2172 bytes, `sha256:ba5ee2a7…` |

| [`devnotes-part24.md`](docs/history/window1.md#stitchcad-devnotes-part24md) | canonical gather lesson | 13 lines, 1160 bytes, `sha256:2a10a04e…` |

| [`devnotes-part25.md`](docs/history/window1.md#stitchcad-devnotes-part25md) | Hem and served-layer lessons | 28 lines, 2416 bytes, `sha256:a95d8c77…` |

| [`devnotes-part26.md`](docs/history/stitchcad-devnotes-part26.md) | closure and notion-placement lessons | 27 lines, 2385 bytes, `sha256:b4b58e1b…` |

| [`devnotes-part27.md`](docs/history/stitchcad-devnotes-part27.md) | canonical buttonhole derivation lesson | 13 lines, 1158 bytes, `sha256:319f11b3…` |

| [`stitchcad-devnotes-part28.md`](docs/history/stitchcad-devnotes-part28.md) | Pocket composition lesson | 14 lines, 1301 bytes, `sha256:764116ee…` |

| [`stitchcad-devnotes-part29.md`](docs/history/stitchcad-devnotes-part29.md) | coverage probe calibration lesson | 15 lines, 1383 bytes, `sha256:d6d266b6…` |

| [`stitchcad-devnotes-part30.md`](docs/history/stitchcad-devnotes-part30.md) | numeric availability/source-truth lesson | 15 lines, 1356 bytes, `sha256:7fa4db87…` |

| [`stitchcad-devnotes-part31.md`](docs/history/stitchcad-devnotes-part31.md) | identifier grammar/binding lesson | 14 lines, 1283 bytes, `sha256:317f385a…` |

| [`stitchcad-devnotes-part32.md`](docs/history/stitchcad-devnotes-part32.md) | canonical procedure metadata lesson | 20 lines, 1878 bytes, `sha256:d5d201dc…` |

| [`stitchcad-devnotes-part33.md`](docs/history/stitchcad-devnotes-part33.md) | archive capacity/retrieval lesson | 18 lines, 1591 bytes, `sha256:aad7494a…` |

| [`stitchcad-devnotes-part34.md`](docs/history/stitchcad-devnotes-part34.md) | measurement table binding lesson | 18 lines, 1699 bytes, `sha256:3dc0b619…` |

| [`devnotes-part35.md`](docs/history/stitchcad-devnotes-part35.md) | individual Ease mapping lesson | 13 lines, 1190 bytes, `sha256:de68d50d…` |

| [`devnotes-part36.md`](docs/history/stitchcad-devnotes-part36.md) | per-POM Ease query lesson | 13 lines, 1196 bytes, `sha256:f0b78fd7…` |
