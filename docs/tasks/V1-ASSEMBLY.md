# V1-ASSEMBLY: assembly visualization (parallel V-track)

## Metadata

- Tree ID: `V1-ASSEMBLY`
- Status: `proposed`
- Roadmap lane: `ROADMAP.md` §11 track **V1 — Assembly visualization (parallel)** (sources: §7.7 4.1
  Mesh, §3.1 SewingGraph and layer index, §7.7 `sc-viewport`)
- Created: `2026-09-29`
- Owner: repo-local workflow

## Goal

A flat pattern set becomes an inspectable **3D assembly**: pieces triangulate, seam correspondences
resample with declared ease distribution (never welded 1:1 — a sleeve cap is intentionally longer than
its armscye), 3D binds to the **sew (net) line** rather than the cut boundary, arrangement surfaces and
layer index wrap flat pieces before any solver runs, and the viewport integrates natively and in the
browser.

The track's claim is bounded on purpose: assembly and topology inspection, plus a **qualitative** drape
preview. No fit-accuracy, pressure or strain claims — those are `V2-SIM`'s, and only with physical
evidence.

## Entry criteria

- SewingGraph specified at G0 (`G0-CONTRACT.3`) and exercised by `G3-GRADING` (walking/truing, ease).
- Mesh work starts post-G3, per the roadmap's prerequisite note.
- `sc-viewport` exists from the G1 canvas-hosting spike.

## Non-Goals

- Physically validated simulation, fit prediction or any quantitative claim (`V2-SIM`).
- Blocking any G-gate: §11 V1 states it does not, and this tree keeps that true — its leaves never sit
  on a G-gate's critical path.
- Photorealistic rendering; the viewport serves inspection, not marketing.

## Acceptance Criteria (track V1 exit, clause by clause)

| Roadmap V1 exit clause | Leaf |
| --- | --- |
| mesh + seam correspondence validated against supported constructions | `.1`, `.2`, `.6` |
| valid AND intentionally defective assemblies (blinded, with false-pos/neg reporting) | `.6` |
| arrangement surfaces + layer index | `.4` |
| viewport integration native + browser | `.5` |

## Task Tree

- ID: `V1-ASSEMBLY`
  Status: `proposed`
  Goal: an inspectable, honestly-labelled 3D assembly of the supported constructions.
  Children: `.1` … `.7`

- ID: `V1-ASSEMBLY.1`
  Status: `pending`
  Goal: `sc-mesh` triangulation — CDT via `spade`, with constraint splitting for notches and cutouts
  touching boundaries, as a budgeted preprocessing step.
  Acceptance: every supported construction triangulates within the declared budget; a boundary-touching
  notch or cutout is a constraint, not an approximation; failure to mesh produces a diagnostic naming
  the offending geometry.
  Verification: `pending`
  Commit: `pending`

- ID: `V1-ASSEMBLY.2`
  Status: `pending`
  Goal: seam stitching as parameterized domain correspondence `[0,1]` on the SewingGraph, with
  stop-at-notch/turn landmarks and **differential seam lengths distributed by a declared resampling
  rule** — never welded 1:1.
  Acceptance: cap ease is distributed by the declared rule and is visible as intentional; a seam whose
  differential exceeds the rule's envelope is reported, not silently stretched; one-to-many and partial
  spans stitch correctly.
  Verification: `pending`
  Commit: `pending`

- ID: `V1-ASSEMBLY.3`
  Status: `pending`
  Goal: net-line binding — 3D binds to the sew (net) line, never the cut boundary, with allowances
  stripped, folded or masked per a declared display policy.
  Acceptance: each display mode is implemented and labelled; a viewer cannot mistake a folded allowance
  for garment geometry; the policy is a per-view setting with a documented default.
  Verification: `pending`
  Commit: `pending`

- ID: `V1-ASSEMBLY.4`
  Status: `pending`
  Goal: arrangement surfaces — limb/torso proxy primitives plus the Piece layer index — wrapping flat
  pieces before any solver runs.
  Acceptance: the proxy set covers the supported garment families; layer index orders pieces for
  assembly; a piece with no plausible arrangement is reported rather than placed arbitrarily.
  Verification: `pending`
  Commit: `pending`

- ID: `V1-ASSEMBLY.5`
  Status: `pending`
  Goal: `sc-viewport` 3D integration, native and browser, with floating-origin / camera-relative
  precision handled as a design requirement (f32 jitter is a design problem, not a given).
  Acceptance: a full garment assembly renders without visible jitter at pattern scale in both targets;
  the browser path respects the `wasm-viewer` capability matrix; the WebGL2-class fallback decision
  from G5 is honoured.
  Verification: `pending`
  Commit: `pending`

- ID: `V1-ASSEMBLY.6`
  Status: `pending`
  Goal: blinded validation against the supported constructions — valid assemblies **and intentionally
  defective ones**, with false-positive and false-negative rates reported.
  Acceptance: the defect set is prepared by someone other than the evaluator where possible; both error
  rates are published with the sample size; a defect class the viewer cannot see is recorded as a named
  limitation, not omitted.
  Verification: `pending`
  Commit: `pending`

- ID: `V1-ASSEMBLY.7`
  Status: `pending`
  Goal: V1 exit review — every clause cited against evidence, the labelled-approximation statement
  published in the UI and on any output, the track closed or its remainder re-scoped.
  Acceptance: each clause `met` with a re-runnable check or `not met` with a named blocker; the
  approximation labels required by §7.7 4.2 are verified present in the shipped UI and exports.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `V1-ASSEMBLY.1` | `pending` | gated on `G3-GRADING` (post-G3 per the roadmap prerequisite) |

## Decisions

- `2026-09-29`: `.6` is a leaf with published error rates, not a demo. The roadmap asks for blinded
  defective assemblies with false-pos/neg reporting, which is a measurement; a walkthrough would prove
  only that the happy path renders.
- `2026-09-29`: the three capabilities in §7.7 4.2 stay separate in this tree — assembly/topology
  inspection and qualitative drape preview belong here; quantitatively validated fit prediction belongs
  to `V2-SIM` and is explicitly excluded.

## Open Questions

- Whether a qualitative drape preview ships inside V1 or waits for `V2-SIM`'s solver — the roadmap
  permits it here (labelled approximation) but does not require it; decided at track entry against the
  arrangement-surface results from `.4`.
- Which proxy primitive set is sufficient for the supported families (`.4`) — decided with the domain
  expert who reviews the G4 usability gate, so the vocabulary is theirs.

## Blockers

- None intrinsic; entry depends on `G3-GRADING`. This track never blocks a G-gate.

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
| `.1` … `.7` | `pending` | — |

## Changelog

- `2026-09-29`: Tree created by `PLANNING.3` with 7 leaves mapped to the V1 exit clauses.
