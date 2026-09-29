# G5-SHELLS: application shells & validated 2D UX (roadmap gate G5)

## Metadata

- Tree ID: `G5-SHELLS`
- Status: `proposed`
- Roadmap lane: `ROADMAP.md` §11 gate **G5 — Application shells & validated 2D UX** (sources: §7.3
  runtime profiles, §7.5 tech pack, §7.6 i18n shipping, §7.7 viewport, §7.8 parity, §2.8 no-code,
  ADR-0002)
- Created: `2026-09-29`
- Owner: repo-local workflow

## Goal

The tool becomes an **application**: Tauri native on Windows/macOS/Linux and a WASM web app, both per
the declared runtime profiles, implementing the full UX spec over the same command bus — and the
UI↔API↔MCP parity invariant is proven by a coverage table plus agent end-to-end runs on every target
and a real native UI test suite, because MCP tests are not UI tests.

## Entry criteria

- `G4-PROFILES` closed (the Profile Editor has a profile model to edit) and `G2-2D`'s dev-shell viewer
  replaced, not duplicated.
- ADR-0002 settled by the `G1-SLICE.13` canvas-hosting spike; the shell implements that verdict.

## Non-Goals

- The conformance lab, real import-filter validation and the factory pilot loop (G6).
- The production declaration, semver commitment and release channels (G7).
- Domain logic in TypeScript: prohibited by convention and CI (ADR-0002), and this tree keeps it that
  way — the shell is a view layer over `sc-api`.

## Acceptance Criteria (gate G5 exit, clause by clause)

| Roadmap G5 exit clause | Leaf |
| --- | --- |
| Tauri native (Win/macOS/Linux) per runtime profiles | `.1` |
| WASM web per runtime profiles | `.2` |
| full UX spec: 2D drafting canvas | `.3` |
| full UX spec: piece manager | `.4` |
| full UX spec: grading panel | `.5` |
| full UX spec: full Profile Editor | `.6` |
| full UX spec: uncertainty dashboard | `.7` |
| full UX spec: export wizard | `.8` |
| UI↔API↔MCP parity table proven by agent E2E on all targets | `.9` |
| real native UI test suite (pointer/keyboard/focus/screen-reader) | `.10` |
| supported OS/browser matrix + installer/update behavior | `.11` |
| i18n first complete language pack (reviewed, thresholded) + RTL verified | `.12` |
| minimum tech pack contents complete enough for a factory quote | `.13` |

## Task Tree

- ID: `G5-SHELLS`
  Status: `proposed`
  Goal: shipped shells with a validated 2D UX and proven front-end parity.
  Children: `.1` … `.14`

- ID: `G5-SHELLS.1`
  Status: `pending`
  Goal: the Tauri native shell for Windows, macOS and Linux under the `native-full` profile, hosting
  the canvas topology ADR-0002 settled, with the domain strictly behind `sc-api`.
  Acceptance: one build per OS in CI; the shell contains no domain logic (enforced by a lint/CI rule,
  not a convention); start-up, cancellation and resource bounds behave per §10.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.2`
  Status: `pending`
  Goal: the WASM web app under the `wasm-viewer` profile — real browser execution, files + IndexedDB
  persistence, the published capability matrix honoured rather than assumed.
  Acceptance: the browser matrix (WebGPU baseline, fallback strategy, COOP/COEP, startup size,
  cancellation) is measured and published; a project loads, evaluates, renders and exports in the
  browser; anything the profile excludes is visibly excluded, not silently degraded.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.3`
  Status: `pending`
  Goal: the 2D drafting canvas as product UX — zoom/pan at pattern scale, snapping, picking,
  annotations, measurement feedback in the user's units, over the dev shell's command coverage.
  Acceptance: the UX spec chapter in the book is implemented feature-for-feature; every interaction is
  a bus command; precision at micron scale is demonstrated, not asserted (ADR-0002's coordinate concern).
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.4`
  Status: `pending`
  Goal: the piece manager — list, select, rename, multiplicity, mirrored pairs, cut-on-fold,
  face/wrong-side, material assignment, layer index, printed label data.
  Acceptance: every field the ontology declares for a Piece is editable or visibly read-only with a
  reason; label data preview matches what export writes.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.5`
  Status: `pending`
  Goal: the grading panel — size sets, base size, both instantiation paths, the equivalence report and
  the per-mode grading interchange options.
  Acceptance: a user can see which path produced a size and what the declared tolerance was; extreme
  sizes are checkable from the UI; a divergence is presented as a bounded equivalence, not hidden.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.6`
  Status: `pending`
  Goal: the full Profile Editor (beyond G4's minimal guided editor) — no-code authoring of parameters,
  constraints, evidence and artifact effects, in plain language, with per-claim evidence scope visible.
  Acceptance: a domain expert never writes JSON or a constraint expression; every question maps to a
  parameter and a diagnostic template; the editor writes through `sc-api` commands only.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.7`
  Status: `pending`
  Goal: the uncertainty dashboard — known / assumed / unknown states per parameter, evidence and its
  scope, artifact effects, and the dependency closure that explains why an export is blocked.
  Acceptance: a blocked export can be traced from the message to the unknown, its field and its resolve
  policy; badges are derived views of evidence records, never independent claims.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.8`
  Status: `pending`
  Goal: the export wizard — artifact selection per profile `accepts`, mode selection (dialect, grading
  mode, cut/sew mapping), the policy-matrix verdict per artifact, and the release package preview.
  Acceptance: the wizard cannot produce an artifact the policy matrix blocks; every choice it makes is
  recorded in the manifest; a preview shows exactly what will be written.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.9`
  Status: `pending`
  Goal: prove the UI↔API↔MCP **workflow parity** invariant — the coverage table generated from the
  command list, plus agent end-to-end runs on every target.
  Acceptance: the table is generated, not hand-written triplication; each workflow row names its UI
  path, API command and MCP tool; an agent completes each workflow on native and web; a gap is a defect
  with an owner, not a note.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.10`
  Status: `pending`
  Goal: a real native UI test suite — pointer, keyboard, focus and screen-reader behaviour — because
  MCP tests are not UI tests.
  Acceptance: the suite runs in CI on at least one native target; accessibility failures are defects
  with owners; the suite drives the shipped shell, not a mock of it.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.11`
  Status: `pending`
  Goal: the supported OS/browser matrix plus installer and update behaviour (install, upgrade,
  rollback, first-run).
  Acceptance: the matrix is published with tested versions; installer and update paths are exercised on
  each supported OS; an unsupported platform fails with a diagnostic, not a crash.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.12`
  Status: `pending`
  Goal: the first complete language pack — reviewed, with review-coverage thresholds by importance
  (units, construction and approval warnings strictest) — and RTL verified: layout mirrored, geometry
  untouched.
  Acceptance: coverage thresholds are measured per category; a RTL locale mirrors layout while grain,
  winding and orientation are provably unchanged (a geometry-diff test, not a screenshot); machine
  tokens never render raw.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.13`
  Status: `pending`
  Goal: minimum tech-pack contents complete enough for a factory quote — piece list with cut quantity
  and fabric, POM with grade, stitch type and SPI per seam, seam allowance per edge, notions,
  version/date/style id — generated from canonical structured content, never hand-edited.
  Acceptance: each content item traces to a domain object; a missing item blocks the tech pack with a
  diagnostic naming the object; size-run quantities stay in the Order object, not the design.
  Verification: `pending`
  Commit: `pending`

- ID: `G5-SHELLS.14`
  Status: `pending`
  Goal: G5 exit review — every clause cited against evidence, the parity table and OS/browser matrix
  published, the frontier handed to `G6-CONFORMANCE`.
  Acceptance: each clause `met` with a re-runnable check or `not met` with a named blocker.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `G5-SHELLS.1` | `pending` | gated on `G4-PROFILES` and the ADR-0002 verdict from `G1-SLICE.13` |

## Decisions

- `2026-09-29`: parity (`.9`) and the native UI suite (`.10`) are separate leaves with separate
  oracles. The roadmap is explicit that agent/MCP tests are not UI tests; merging them would let an
  agent pass prove a keyboard path that nobody exercised.
- `2026-09-29`: the UX panels (`.3`–`.8`) are one leaf each so a panel can land, be reviewed in the
  book, and be exercised by the parity table independently — a single "implement the UX" leaf would be
  unfalsifiable until the end.

## Open Questions

- Whether the web app ships an editor or a viewer-plus-export at G5 (the `wasm-viewer` profile name
  suggests the latter) — decided at gate entry against the measured capability matrix from `.2`.
- Which language is the first complete pack (`.12`) — a director/community decision; the leaf records
  the criteria (reviewer availability, safety-term coverage) so the choice is argued, not incidental.

## Blockers

- None intrinsic; entry depends on `G4-PROFILES` and `G1-SLICE.13`.

## Acceptance Checklist

Filled per leaf, in a `### <leaf-id>` subsection added by the same commit as the work, and mechanically
required to be fresh in that commit by `FRESH-ACCEPTANCE-EVIDENCE`; this file carries no unticked
placeholder boxes (defect D15).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-29` | tree seeded | `scripts/check_doctrines.sh` | `=== all doctrines green ===`, `rc=0` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| tree seed | `STITCHCAD-PLANNING-0003 (leaf PLANNING.3)` | created by the seeding leaf |
| `.1` … `.14` | `pending` | — |

## Changelog

- `2026-09-29`: Tree created by `PLANNING.3` with 14 leaves mapped to the G5 exit clauses.
