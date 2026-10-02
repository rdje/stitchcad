# StitchCAD — Roadmap v0.3 (functionality-first, deadline-free)

> Status: REVISED after peer review (GPT/Codex, Gemini, Grok — 2026-09-27).
> Revision policy: this is a LIVING document. Additional reviews produce
> v0.3, v0.4, … under the same rules: already-locked decisions are not
> reopened implicitly (only by explicit, argued challenge), and every
> change is appended to the Disposition Log in Appendix A with its source.
> Version history is preserved in git; earlier revisions remain available.
> The roadmap remains DRAFT until the G0 exit criteria are met.
> v0.1 was directionally validated by all three reviewers; v0.2 integrates
> their findings. v0.3 amends one gate exit (§11 G3) on the engineer's proposal
> under the director's delegation, closing defect D32 — envelope garments no
> exit criterion proved; it is logged in Appendix A with its source. Changes are
> marked conceptually via the Disposition Log
> in the Appendix; no timestamps or deadlines exist anywhere in this
> document — gates are functionality-defined only.
>
> Language: this document is English-only, as decided.

---

## 1. Vision

A sewing CAD where:

- The **design lives in an abstract parametric space** (measurement tables,
  construction recipes, style choices), never in any vendor file format.
- A **Factory Profile** (versioned parameter bundle + typed constraints +
  per-claim evidence) turns an abstract design into a concrete **Instance**
  (graded pattern set) via a documented, deterministic pipeline.
- An **Artifact Generator** serializes Instances to industry formats
  (DXF AAMA/ASTM, HPGL/PLT, PDF, tech pack) — a complete, immutable,
>  evidence-bearing **release package**.
- Unknown/unverified facts are **first-class citizens**, strictly separated
  from selectable design choices; they never silently become geometry.
- Every semantic object is introspectable through a **stable application
  command API**, and the tool is fully drivable by an AI/LLM agent via MCP
  (stdio-first; see §10).
- Runs as **native applications** (Windows / macOS / Linux) and as a
  **web app (WASM)** — via explicitly defined runtime profiles (§7.3), not
  by assuming dependency parity.

### 1.1 Prior art and differentiators (named, not implied)

Existing: Lectra Modaris, Gerber AccuMark, Optitex, Grafis, CLO/Browzwear,
StyleCAD, Seamly2D/Valentina (GPLv3). Seamly2D already ships parametric
measurement-driven drafting, DXF/HPGL/PDF export, and cross-platform desktop.

StitchCAD's differentiators:

1. **Factory Profiles**: versioned, evidence-bearing adapters between an
   abstract design and a specific cutting room — the signoff loop is the
   product.
2. **Explicit uncertainty + provenance** as semantic data, with an
   artifact policy matrix (§8.3) governing how unknowns affect exports.
3. **Industrial signoff loop**: release manifests, scoped approvals,
   rejection-reason taxonomy, factory feedback calibration.
4. **Agent-first control**: MCP as a first-class front-end from Stage 1.
5. A **headless, UI-agnostic core** not embedded in a legacy widget tree.

### 1.2 Primary persona (beachhead)

**A patternmaker producing a signoff package for a named factory**, with an
AI agent as first-class co-user, and a sewing/factory expert as the profile
author and reviewer. PLM administrators, casual web users, and MTM/body-scan
workflows are explicitly later personas.

### 1.3 Non-goals (written down so partners cannot reopen them)

- Marker making / nesting / yield / consumption (factories do this after
  accepting our file)
- Costing
- Paper-pattern digitization / raster tracing
- Direct cutter / CAM drivers
- PLM / ERP integration
- Body-scan MTM in v1
- Photorealistic or quantitatively validated drape (staged track, §12 V2)
- Spreading, colorway management, print/artwork placement

---

## 2. Guiding principles

1. **Headless-first.** The core is pure Rust libraries with no UI dependency;
   UI, CLI, web, and agents are interchangeable front-ends over the same
   command API.
2. **Construction-recipe semantics, not drawing semantics.** The canonical
   design is a replayable construction history: a measurement table + formula
   graph + ordered drafting operations producing pieces, seams, darts, and
   ease relationships. Geometry is derived; authoritative imported geometry
   is stored explicitly as primitives with no fabricated history (§4.1).
3. **Uncertainty is explicit and categorized.** Every parameter is one of:
   known fact (with scoped evidence) / unknown fact (requires observation) /
   selectable design choice / overridable preference / derived value. Unknown
   facts are never assigned a value by the solver and never silently exported
   (§8.3).
4. **Agent-first dogfooding.** The MCP surface is built early so AI agents
   help build and test the tool; but CLI determinism — not LLM behavior — is
   the hard correctness gate (§11, G2).
5. **Everything is versioned and evidence-backed.** Designs, profiles,
   schemas, golden files, and every compatibility claim carries provenance.
6. **No silent failure.** Infeasible configurations produce explained,
   field-linked diagnostics (unsat cores mapped to profile fields), and the
   supported constraint fragment is explicitly bounded (§6.2). Solver
   outcomes distinguish: satisfied / proven-unsatisfiable / unknown /
   numerically-failed.
7. **Deterministic serialization.** All artifacts pass through a controlled
   canonicalizer (stable entity IDs, no wall-clock, fixed float policy) before
   byte-level golden comparison; semantic/geometric comparison with declared
   tolerances is the primary correctness check (§11).
8. **No-code by design for domain experts.** Factory Profiles are edited
   through a guided, plain-language UI — never by writing JSON or constraint
   expressions. The JSON schema is for machines and diffs; the UX is for
   humans. The barrier to sharing field knowledge must be the knowledge
   itself.
9. **Localizable from day one.** All user-facing strings externalized from
   the start (CI-enforced); machine tokens never rendered raw; localization
   of knowledge content (descriptions, explanations) goes through the same
   governed pipeline as code strings (§7.6). RTL layouts never mirror
   geometry.
10. **Documentation in lockstep.** Roadmap, code and mdBook share verified
    scope in each task-owned commit. Teach students/newcomers incrementally;
    give experts a glossary, topic index and direct annexes. API, numerical,
    format and verification detail belongs in annexes. Distinguish specified
    future workflows from available behavior.

---

## 3. Domain model (the garment ontology)

The v0.1 object list (Piece/Seam/Notch/Grainline/Measurement) expressed a
drawing, not a garment. v0.2 first-class objects:

### 3.1 Core objects

- **MeasurementTable**: named, unit-carrying scalars with landmarks, source,
  and procedure. Distinguishes *body* measurements (ISO 8559 / ASTM D5219
  semantics) from *garment* points of measure (POMs).
- **Ease**: first-class mapping body → garment per POM, with fit intent
  (close / semi / loose). Without Ease, measurement-driven regeneration and
  factory grade-rules cannot be reconciled.
- **Design**: the construction recipe — formula graph over the measurement
  table + ordered operations. This is the primary authoring paradigm
  (ADR-0003); geometric sketch constraints are optional local annotations,
  not the authoring model.
- **Piece**: oriented outer boundary (CCW winding), holes (internal cutouts),
  internal construction lines; multiplicity, mirrored pairs, cut-on-fold,
  face/wrong-side, material assignment, **layer index** (for 3D assembly
  ordering), stable identity (ULID-based) with printed label data
  (name, size, cut qty, pair L/R, fold, fabric/colorway).
- **SeamSpan**: oriented seam correspondences (piece X edge A ↔ piece Y edge
  edge B) with declared ease distribution, direction, stop-at-notch/turn
  landmarks. **SewingGraph / Assembly** is a first-class object living in the
  Design — not an implementation detail of meshing. One-to-many and partial
  spans supported.
- **Dart / Tuck / Pleat / Gather**: closure semantics, not just line art.
  (Slash-and-spread is a drafting operation on the recipe.)
- **SeamAllowance**: a *derived object attached to each edge*, with per-edge
  width, corner treatment (miter / slant / envelope / trim / step), and an
  explicit included-in-contour vs generated-downstream policy resolved per
  Factory Profile. Not a profile-level boolean.
- **Notch**: semantic matching mark with a physical representation resolved
  at export: type (single/double + V, I, T, U, castle, slit, drill),
  depth/width (sample-room vs production values), encoding (coded point +
  direction/depth/type vs drawn geometry) — all Factory Profile parameters.
- **Grainline**: directed entity; supports bias/off-grain and dual references
  (stripe + garment grain).
- **Hem / Facing / Lining / Interfacing**, **Closure** (button/buttonhole,
  zipper), **Pocket** — or explicit rejection with diagnostics when outside
  the supported envelope.
- **Corner / Truing / Walking**: `walk` and `true` are first-class editing
  operations on the SewingGraph (a patternmaker walks a sleeve cap around an
  armscye and plants balance notches) — not late-added validation checks.

### 3.2 Supported product envelope (v1 boundary)

The release claim is bounded. v1 supports: a declared family of woven
garments with documented seam operations (A-line skirt with waist dart and CB
zipper; darted bodice; set-in sleeve; classic collar; trousers). Unsupported
constructions (e.g., knit stretch blocks, leather, fully bespoke structures)
produce explicit diagnostics, never approximate silently. A
supported/rejected/deferred feature matrix is a G0 deliverable.

### 3.3 Two instantiation paths (both required)

1. **Measurement-driven regeneration**: re-evaluate the construction recipe
   per size / per body — the parametric vision (MTM-ready).
2. **Grade-rule instantiation**: base size + size breaks + per-point X/Y
   delta rule tables (`.rul` serialization; incremental or cumulative;
   stack-point / fixed-perimeter / smoothing attributes) — what RTW
   factories' systems expect and reconstruct.

These are semantically different (independently drafted sizes are NOT exactly
reconstructible from base+rules — known information loss, cf. GRAFIS docs).
Both paths exist; equivalence is validated within declared tolerances, and
extreme sizes are always checked after target-system reconstruction, not just
the base size.

### 3.4 Size systems

Size labels vs order, base size, multi-dimensional size systems, custom
charts: EN 13402, ASTM D5585, alphanumeric/numeric, custom. The SizeSet is a
first-class object; whether it lives in the Design, the Factory Profile, or a
third Order object is decided at G0 (default: SizeSet referenced by Design,
overridable per Factory Profile with a recorded transformation).

---

## 4. Architecture

### 4.1 Canonical data model and identity

- The canonical project = authored semantic inputs (measurement tables,
  formulas, operations, sewing graph, materials) **plus explicit geometric
  primitives where history does not exist** (imported contours). Derived
  contours, meshes, render buffers, and caches are never authoritative.
- **Persistent identity contract**: notches, grade points, and seam spans
  reference stable topological entities (edge IDs + parameterization), never
  array indices or tessellation vertices. Referenced-edge split/merge/reverse/
  delete/offset-fragmentation either preserves references or produces a
  visible unresolved-reference repair task. No silent reassignment.
- Drafting dependency graph (acyclic recipe evaluation) and geometric
  constraint solving (possibly simultaneous) are distinct structures with an
  explicit integration contract.

### 4.2 Numerical contract (replaces "zoom to micron")

- Single internal unit: **fixed-point micrometers (i64)** recommended;
  floating point only at format boundaries with an explicit policy.
- Separate tolerance classes: numerical computation, geometric approximation,
  format quantization (chordal tessellation, e.g. ≤ 0.1 mm for legacy
  polyline-only importers), importer comparison, physical acceptance. Derived
  from downstream requirements; no single global epsilon.
- Curve representation: line, circular arc, cubic Bézier (AAMA/ASTM diet).
  NURBS deferred. Robust predicates for intersections (Shewchuk-style /
  `robust` crate); non-finite and dimensionally invalid expressions rejected.
- **Offset engine is a first-class risk**: variable seam allowances, concave
  corners, cusps, near-tangencies, self-intersections, corner joins, hem
  allowances. Exact Bézier offsets are not Béziers — a declared
  approximation/error policy with topology repair is specified, and exports
  fail explicitly when the error bound cannot be met. Offset pathology corpus
  is a G2 gate input.

### 4.3 Crate layout (revised; crates appear when their stage starts)

```
stitchcad/
├── crates/
│   ├── sc-core/          # Garment ontology, construction recipe, formula
│   │                     #   graph, sewing graph, uncertainty model, command bus
│   ├── sc-units/         # Fixed-point units, tolerance classes, conversions
│   ├── sc-geometry/      # 2D kernel: curves, robust intersections, offsets
│   ├── sc-measure/       # Measurement tables, POM/ease model, size systems
│   ├── sc-grading/       # Dual instantiation: regeneration + grade rules (.rul)
│   ├── sc-constraints/   # Profile CSP — deterministic finite-domain solver
│   │                     #   (pure Rust, no deps); typed constraint AST
│   ├── sc-sketch/        # OPTIONAL local geometric constraints (solver TBD by
│   │                     #   license ADR; default: custom/least-squares, non-GPL)
│   ├── sc-profiles/      # Factory Profile schema, validation, evidence store
│   ├── sc-artifacts/     # Canonicalizer + DXF (AAMA/ASTM dialects), HPGL, PDF,
│   │                     #   tech pack — a product, not thin wrappers
│   ├── sc-mesh/          # Piece→3D mesh: arrangement surfaces, seam resampling
│   ├── sc-sim/           # Cloth simulation research track (out of default build)
│   ├── sc-store/         # Persistence: canonical text project format, SQLite
│   │                     #   (native), file/IndexedDB backend (WASM)
│   ├── sc-api/           # Versioned experimental command API (jsonrpsee)
│   ├── sc-mcp/           # MCP façade over sc-api (rmcp, stdio first)
│   ├── sc-cli/           # Headless CLI; deterministic command replay
│   ├── sc-app-tauri/     # Native shell (Stage 5)
│   └── sc-viewport/      # wgpu 2D/3D viewport (canvas hosting per ADR-0002)
├── conformance/          # Golden files, pathology corpus, layer/matrix suites,
│                         #   known-good foreign fixtures, mutation tests
├── docs/adr/             # ADR-0001 license×solver, ADR-0002 UI stack,
│                         #   ADR-0003 drafting paradigm, ADR-0004 DXF dialects
└── .github/workflows/    # CI grows with stages (G0: fmt/clippy/unit+property/
                          #   WASM smoketest; fuzzers start when parsers exist)
```

`sc-constraints` (discrete CSP) and `sc-sketch` (continuous geometry) are
separate crates with separate licenses and WASM stories.

### 4.4 Command bus (one surface, three front-ends)

Domain invariants live in a single **command layer**: typed, validated
commands (`SplitSeamSpan`, `ChangeMeasurement`, `AssignMaterial`,
`EvaluateInstance`, `PrepareRelease`) with atomic groups, preview/commit,
revision preconditions, idempotency, structured errors, and progress for long
operations. UI, CLI, and MCP are adapters over the same commands. MCP tool
count is curated (coherent workflows, batch operations, query resources) —
not one tool per getter. Undo/redo semantics are defined at G0 (granularity
per command group). UI↔API↔MCP **workflow parity** is the invariant; it is
verified by a coverage table, not by hand-written triplication.

### 4.5 Persistence authority

- One canonical serialization: a project directory of canonical-text files
  (stable key order, declared float formatting, ULID entity IDs, no
  wall-clock) + attachments. Schema-versioned with migrations and
  recoverable-unknown-extension preservation.
- SQLite (native) holds: profile library, feedback log, provenance — but the
  *authoritative* fact store is the canonical project; the DB is a cache for
  catalogs, never co-authoritative with the project directory. Reopening an
  old project pins the profile versions it was built against; updates are
  explicit.
- Crash-consistent saves, autosave/recovery, backups, atomic multi-file
  commits; forced-interruption and failed-migration recovery tested from G1.
- WASM backend = files + IndexedDB, named now (repository trait existed
  before first store call site). PostgreSQL deferred behind the same trait.
- **Data boundaries**: customer measurements, body scans, and commercially
  sensitive factory knowledge NEVER live in the public repo. Public repo =
  reference fixtures + deliberately contributed, sanitized profiles.
  Diagnostics/telemetry redact private data by default.

---

## 5. ADRs to record at G0 (coupled decisions locked together)

### ADR-0001 — License × solver (ONE decision)

The SolveSpace solver (`slvs`) is GPLv3; linking it into the core copylefts
the workspace. Z3 is MIT. Options:

- (a) Permissive core (MIT/Apache/BSL) → `slvs` rejected; `sc-sketch` uses a
  small custom kernel / numeric least-squares with diagnostics.
- (b) GPLv3 core → `slvs` allowed; note embedding implications for factories.
- BSL 1.1 is a distinct category (use-restricted, converts later) — never
  lumped with MIT/Apache.

Default recommendation: (a) unless a strong case for `slvs` emerges in the
G1 spike. Decision is recorded BEFORE any solver code exists — an external
contributor arriving at G1 must inherit a settled license, not a default.

### ADR-0002 — UI stack and canvas hosting

- Chrome: Tauri + TypeScript/React (recommended; Slint if "zero non-Rust"
  becomes a hard constraint). Flutter rejected: second language + FFI across
  the semantic graph, weaker CAD-viewport story.
- **Canvas hosting** (the actual decision): three topologies exist — WebGPU
  inside the webview; native wgpu surface composed with HTML chrome; DOM
  canvas. A G1 executable spike (zoom/pan, snapping, picking, annotations on
  a realistic pattern set, native AND browser) settles it with evidence.
  "Custom wgpu, not DOM canvas" is a hypothesis to test, not an axiom
  (coordinate precision is a renderer-design concern, not a DOM property).
- Business/domain logic in TypeScript: prohibited by convention and CI.
- An **egui/iced dev shell** is used for Stages 1–4 internal tools (golden
  diff viewer, uncertainty dashboard prototype, conformance lab) so the
  engine never blocks on the hardest integration in the repo. The dev shell
  is explicitly not the shipped product UX.

### ADR-0003 — Drafting paradigm

Primary = **construction recipe** (measurement table + formula language +
ordered operations; Seamly2D/Aldrich/Müller & Sohn lineage). The formula
language is specified HERE, not later: operators, units inside expressions,
conditionals (multi-size branching), name binding (measurements, prior
points/lengths/angles, profile parameters). Geometric sketch constraints =
optional local annotations. Grading = second instantiation path (§3.3), not
re-solving the sketch. A named drafting system ships as reference blocks
(v1: free drafting + one documented system, decided at G0).

### ADR-0004 — Interchange dialects

- **ASTM D6673-10 is withdrawn (Jan 2019, no replacement)** — record this.
  Implement it as de-facto convention ("ASTM DXF" is what cutting rooms
  enforce), watch D13.66 revival work, and define StitchCAD interchange
  profiles citing AAMA-292/D6673 semantics without claiming a current
  standard. D6673's specified carrier is AutoCAD R13 DXF; many importers
  accept only R12/R13 POLYLINE (no SPLINE/LWPOLYLINE).
- **Layer table (corrected)**: 1 cut boundary (+SST), 2 turn points, 3 curve
  points, 4 V/slit notch, 5 grade reference, 6 mirror line, 7 grainline,
  8 internal lines, 9 stripe ref, 10 plaid ref, 11 internal cutouts, 12
  intentionally blank, 13 drill holes, 14 sew line, 15 annotation text,
  80–83 T/castle/check/U notches, 84–87 quality-validation curves. AAMA uses
  *named* layers (CUT/DRAW/INTCUT/NOTCH/DRILL/TEXT/REF) — close but not
  identical to ASTM's *numbered* layers; both are supported modes.
- Each piece SHALL be a separate DXF BLOCK (no nested INSERTs); block naming
  convention `[piece]_[size]`; Style System Text (SST, case-sensitive syntax)
  on layer 1 and Piece System Text (PST) per piece are mandatory for the
  ASTM path. Golden files validate metadata blocks, not just geometry.
- **Grading interchange**: canonical = base-size DXF + `.rul` table (AAMA
  era); ASTM mode embeds grade rules in DXF; "all-contours-graded" mode
  (GradedNest-style) as a named option. Modes are separate, validated
  receiver-by-receiver; we never claim a universal package.
- **Cut/sew line swap** (CLO's cut/sew default is a top rejection cause) is a
  Factory Profile parameter — a named export mapping, never an inversion of
  semantic meaning.

---

## 6. Constraint machinery (bounded, typed, explained)

### 6.1 Two solvers, three passes

1. **Discrete CSP — `sc-constraints` (custom, pure Rust, THE production
   solver, native and WASM)**: resolves profile parameters against typed
   constraints (enums, booleans, integer/rational fragments in v1) via
   finite-domain propagation + search with declared variable/value ordering —
   fully deterministic (fixed search order, no randomness, no wall-clock).
   Real-valued nonlinear constraints are out of the v1 fragment.
   **Differential oracle (dev-only):** Z3 (MIT) is wired behind the
   non-default `csp-z3` feature as a CI/dev differential oracle ONLY —
   randomized constraint sets are solved by both engines and results are
   compared; any divergence is a bug in `sc-constraints`. Z3 is never
   compiled into release artifacts, never shipped, never a runtime
   dependency. This gives SMT-grade confidence in the custom solver without
   carrying a ~250k-LOC C++ dependency into the product or the browser.
2. **Recipe evaluation**: the construction graph evaluates with the resolved
   concrete parameter set (deterministic, single pass).
3. **Topological verification**: geometric sanity checks (curve continuity,
   minimum curvature radius for knives, offset self-intersection, boundary
   boxes vs fabric width) — failures map back to parameter domains as
   structured diagnostics.

A discrete-feasible configuration can still fail geometric verification: the
contract says the *candidate* is rejected with explanation, not that the
whole space is infeasible, and bounded backtracking is defined.

### 6.2 Typed constraint language

Constraints are a **versioned, typed AST** (not free strings): stable rule
IDs, units, scope, hard/soft status, provenance, diagnostic templates. The
Profile Editor generates the AST; a pretty-printer renders explanations in
sentences ("Your factory uses Lectra Modaris 13.x — this profile requires
centimeters"). Outcomes distinguished: satisfied / proven-unsatisfiable
(explained, field-linked) / unknown (resource-limited) / numerically-failed.

### 6.3 SystemVerilog analogy — internal only

Constrained-random exploration is used in testing (seeds recorded), never
for production generation: production uses explicit selected values under a
documented deterministic preference policy. The analogy stays in ADRs, out
of user-facing docs.

---

## 7. Technology decisions (final or staged)

### 7.1 Language
Rust for all domain logic (Z3/SolveSpace are native *dependencies*, not
Rust — acknowledged in ADR-0001). TypeScript strictly view-layer.

### 7.2 Solvers
Production CSP: custom deterministic finite-domain solver (`sc-constraints`,
pure Rust — see §6.1). Z3 exists only as the non-shipped `csp-z3`
differential oracle in CI. Geometric sketch constraints per ADR-0001.
Solver boundary split per §6.1.

### 7.3 Runtime profiles (three, not one)

| Profile | Contents | Notes |
|---|---|---|
| `native-full` | everything: solvers, SQLite, wgpu viewport, Tauri | reference environment |
| `native-headless` | CLI / MCP / CI, no viewport | conformance & agents |
| `wasm-viewer` | core + geometry + artifacts + viewport + full CSP (pure Rust solver, zero heavy deps); persistence = files/IndexedDB | explicit capability matrix; browser execution tested for real (not `cargo check`) |

WASM CI at G0 = smoketest that `sc-core` + `sc-units` compile to
`wasm32-unknown-unknown`. Full browser matrix (WebGPU baseline, fallback
strategy, COOP/COEP headers, startup size, cancellation) is a G5 concern.
The former Z3-in-browser problem is eliminated by decision: the production
CSP solver is pure Rust with no heavy dependencies, so the full constraint
workflow runs identically native and in-browser. (Historical context: Z3 WASM
builds are heavy — MBs, slow init, SharedArrayBuffer requirements — which is
precisely why Z3 was removed from the production path.)

### 7.4 Storage
§4.5. Canonical text project format; repository trait with named WASM
backend; PostgreSQL deferred.

### 7.5 Artifacts (sc-artifacts as a product)

- **Canonicalizer** first: stable ordering, IDs, formatting — then DXF.
- **DXF**: AAMA named-layers × ASTM numbered-layers; cut-as-1 × sew-as-1;
  R12 × R13; grading modes per ADR-0004; adaptive chordal tessellation for
  polyline-only targets preserving notch intersection points; import produces
  an honest imported representation (no fabricated history) + a
  **loss report** (preserved / approximated / omitted / unsupported).
- **HPGL/PLT**: not trivial — plotter-unit scale (1016 units/inch), `IP`/`SC`
  calibration, pen-per-function mapping (cut/draw/drill/notch), `LB` labels
  with size/qty, path optimization, notch-as-geometry vs tool commands.
  Known-good real-plotter `.plt` fixture enters `conformance/` when the
  writer lands (G3); physical plotter test at G5/G6 — not deferred to the
  declaration stage.
- **PDF**: authoritative vector path (evaluate `pdf-writer`/typst-class over
  `printpdf` during G2); A0 + tiled A3/A4 with registration marks, overlap,
  page ordering, piece ID; a piece that doesn't fit A0 is tiled or errors —
  never silently rescaled. Printed scale-square verification is a G2 gate
  (measured with a ruler, on paper).
- **SVG**: demoted from crate blurb — promoted only when a named consumer
  exists (web preview interchange is the candidate).
- **Tech pack**: generated from canonical structured content that humans
  author (materials/trims, construction notes, stitch type + SPI per seam,
  POMs with landmarks/procedure/tolerances). "Generated, not hand-edited"
  governs the *export*, not the *knowledge*. Minimum contents grow with
  stages (§11). Size-run quantities belong to an Order object, not the
  reusable design.

### 7.6 i18n (architecture now, shipping staged)

- **One message system, chosen at G0** (Fluent OR ICU — not "Fluent or ICU";
  they are distinct systems; if both ends are needed, a designed bridge).
- Architecture in G0: externalization CI lint, glossary/termbase per language
  (safety-relevant terms: notch types, sew/cut line aliases factories use in
  rejection emails), pseudolocalization, locale-independent canonical files
  (decimal-comma input ≠ stored meaning).
- API returns stable diagnostic codes + typed arguments + units; localized
  prose is a presentation field — agents never parse translated sentences.
- RTL: mirrored layout, never mirrored geometry (grain, winding, orientation
  unchanged).
- **Staged**: in-app translation review queue, full RTL, and first shipped
  non-English pack are G5 (the shell), with review-coverage thresholds by
  importance (units/construction/approval warnings strictest). A GitHub-
  based review workflow may substitute for the custom portal early.
- MCP locale-awareness: nice-to-have, explicitly NOT a G5 gate.

### 7.7 3D visualization (staged, claims bounded)

- `sc-viewport`: wgpu; WebGL2-class fallback strategy decided at G5 with the
  WebGPU baseline; floating-origin/camera-relative precision (f32 jitter is
  a design problem, not a given).
- **4.1 Mesh**: pieces triangulate (CDT via `spade`, with constraint
  splitting for notches/cutouts touching boundaries — a budgeted
  preprocessing step). Seam stitching = parameterized domain correspondence
  `[0,1]` on the SewingGraph with stop-at-notch/turn — **differential seam
  lengths (ease) are distributed by a declared resampling rule, never welded
  1:1** (a sleeve cap is intentionally longer than the armscye). 3D binds to
  the **sew (net) line**, never the cut boundary; allowances are stripped,
  folded, or masked per display policy. **Arrangement surfaces** (limb/torso
  proxy primitives, layer index) wrap flat pieces before any solver.
- **4.2 Assembly/stitch-and-smooth**: labeled approximation in UI *and* on
  any output; no fit-accuracy, pressure, or strain claims. Three capabilities
  stay separate: assembly/topology inspection; qualitative drape preview;
  quantitatively validated fit prediction (only V2, only with physical
  evidence).
- **4.3 XPBD research track**: unbounded, out of the default workspace,
  progress reports not promises. Calibration against physical toiles.

### 7.8 API + MCP

- `sc-api`: versioned *experimental* contract from G1; semver commitment only
  at G7. Transports: jsonrpsee (HTTP/WebSocket native; stdio for MCP).
  In-process Rust API distinct from wire format.
- `sc-mcp`: stdio-only until a G5+ threat model exists for any HTTP surface
  (tool poisoning, injected "factory feedback", artifact overwrites). Agent
  authority is scoped from day one: inspect / propose / commit / generate /
  approve are distinct permissions; approval is a human-only capability that
  a graph mutation can never manufacture. Mutating commands carry actor
  trace. Imported files are data, never instructions.
- Agent evaluation: discovery, ambiguity resolution, invalid input, multi-
  step recovery, interruption, provenance explanation; independent checks
  evaluate artifacts (never the agent grading its own work).

---

## 8. Factory Profiles (v2 schema concept)

```jsonc
{
  "profile_id": "acme-cmt-pt",
  "schema_version": "2.0",            // schema vs content versions distinct
  "content_version": "1.3.0",
  "system": { "vendor": "lectra", "product": "modaris",
              "version_range": ["13.x"] },   // semantics of ranges defined
  "accepts": ["dxf-aama-named", "dxf-astm-num-r13", "plt", "pdf-a0"],
  "parameters": [
    { "name": "seam_allowance_policy", "type": "enum",
      "domain": ["included", "excluded"],
      "value": "included", "state": "known",
      "evidence": ["ev:2026-07-14:acceptance#acme"] },
    { "name": "notch_geometry", "type": "notch_spec",
      "value": { "code": "V", "depth_mm": 5, "sample_mm": 3 },
      "state": "known", "evidence": ["ev:2026-07-14"] },
    { "name": "units", "type": "enum", "domain": ["mm", "cm"],
      "state": "unknown", "default": "cm",
      "resolve": "ask_human",        // default can never bypass resolve policy
      "artifact_effect": "blocks_release" },
    { "name": "dxf_version_target", "type": "enum",
      "domain": ["r12", "r13", "r2000"], "value": "r13", "state": "known" },
    { "name": "spline_policy", "type": "enum",
      "domain": ["explode_bulge", "flatten_chord_0.1mm"], "value": "flatten_chord_0.1mm",
      "state": "known" },
    { "name": "shrinkage_xy_pct", "type": "vec2f",
      "value": [2.0, 1.0], "state": "known",
      "transformation_stage": "post_grade_pre_export" },
    { "name": "max_cutting_width_mm", "type": "f32",
      "value": 1520.0, "state": "known" },
    { "name": "piece_block_required", "type": "bool", "value": true, "state": "known" },
    { "name": "cut_sew_swap", "type": "bool", "value": false, "state": "known" },
    { "name": "hpgl_pens", "type": "pen_map", "value": {"cut": 1, "draw": 2, "notch": 3, "drill": 4},
      "state": "assumed", "evidence": [] }
  ],
  "constraints_ast": [ /* typed AST; no free strings; unresolved identifiers
                          rejected at profile validation */ ],
  "composition": { "precedence": ["hard_restriction", "factory_override",
                                   "preference", "default"] }
}
```

- Every parameter: type, domain, value OR default, state
  (known / assumed / unknown), evidence refs, artifact effect.
- **Evidence is scoped per claim**, never profile-wide: target system +
  version + import settings + artifact hashes + procedure + observer + date
  + result. Superseded/revoked evidence is preserved, not overwritten.
  Simple badges are derived views of this record.
- **Composition & precedence**: hard restrictions (safety/design) >
  factory overrides > preferences > defaults; conflicts are visible.
- **Profiles are private by default.** Community library = sanitized,
  consenting contributions; commercial secrets stay local while still
  participating in calibration. This is a product requirement, not a hope.

### 8.2 Unknowns and the artifact policy matrix

States × artifact effects are explicit. Example policy (tuned at G0/G4):

| Unknown affects | PDF preview | DXF/PLT draft | Production release |
|---|---|---|---|
| Cosmetic label | export with badge | export with badge | blocked until resolved or human disposition |
| Notch geometry | default + visible badge | default + provenance sidecar | blocked / recorded human assumption |
| Units | blocked | blocked | blocked |

Dependency closure computed per requested artifact: an irrelevant unknown
must not block an unrelated export.

### 8.3 No "conservative defaults"

A plausible default can still produce an unusable pattern (units, allowance
ownership, notch semantics). Defaults are overridable preferences with
provenance — never a substitute for observation (§2.3).

---

## 9. Signoff & release contract

- Package = immutable manifest: design revision, inputs (measurements,
  materials), resolved profile versions, concrete sizes/pieces, engine +
  exporter config, artifact hashes, validation results, unresolved
  assumptions + dispositions, approver + scope + date.
- Approval binds to package identity; any input/artifact change creates a
  new candidate and stale-ifies prior approval — never inherits it silently.
- Package completeness is checked against the declared construction (all
  pieces, multiplicities, sizes, cut instructions, material assignments,
  companion files). A geometrically valid DXF missing a piece is an invalid
  package.
- Human approval is deliberate and scoped; agents prepare evidence, never
  approve.
- Acceptance states are graduated: generated → internally checked →
  independently inspected → target-imported → physically evaluated →
  human-approved. One factory's acceptance is evidence for *that* envelope,
  never a general certificate. Production claims name the tested
  garment/material/target-system scope.

---

## 10. Security & agent authority (starts G1, hardens G6)

- Scoped permissions from the first exposed API; authorization enforced in
  core logic, not tool descriptions.
- Input validation and resource bounds (solver work, file sizes, decompression).
- MCP stdio-only until a documented threat model permits more.
- Private data (measurements, scans, factory IP) excluded from repo,
  diagnostics, and telemetry by default; profile sharing shows a preview of
  exactly what will be shared.
- Mutating command audit trail with initiating actor.

---

## 11. Stage gates (restructured: G0–G7 + parallel V-tracks)

Each gate lists entry criteria (prerequisites) and exit criteria (evidence).
No calendar. Gates are per-capability: V-tracks never block the 2D release.

### G0 — Product & semantic contract
- **Exit:** supported/rejected/deferred feature matrix; glossary of
  construction terms; units & tolerance policy; garment ontology v1
  (§3.1) specified; dual instantiation paths specified; size-set ownership
  decided; approval states & release contract (§9) specified; ADR-0001
  (license×solver), ADR-0003 (drafting paradigm + formula language v1),
  ADR-0004 (dialects) recorded; measurement/POM standards identified
  (ISO 8559, ASTM D5219, EN 13402, ASTM D5585); evaluation-seat procurement
  started (owner named — seats take months); governance model drafted
  (project owner named; sewist-vs-programmer review paths defined).
- Fixture: the **reference skirt** is fully specified with numbers — A-line,
  one waist dart/side, CB zipper, grain ∥ CB, SA 1 cm sides / 3 cm hem,
  single notches at side seams, cut-on-fold or paired front. This one garment
  exercises dart, grain, variable SA, notch, fold, and the included/excluded
  policy.

### G1 — Executable architecture slice
- **Exit:** three runtime profiles compile; intended browser target runs a
  real (not compile-only) spike: load project → evaluate recipe → one CSP
  constraint → persist/reopen → render → export; canvas-hosting spike
  evidence for ADR-0002; command bus live with revision checks + undo;
  crash-consistent save/recovery demonstrated; license-compatible solver
  builds on all targets.
- WASM evidence: capability matrix with tested versions + limitations.

### G2 — Correct 2D slice (the vertical proof)
- **Exit:** reference skirt drafted through CLI with deterministic command
  replay; minimal 2D viewer/editor (dev shell) over the same commands;
  canonical DXF (one named mode) + dimensionally correct PDF; printed
  scale-square verified on paper; offsets pass the pathology corpus
  (acute angles ≤ 45°, zero self-intersecting loops, bounded accumulated
  seam-length error); native save/load preserves semantics; mutation tests
  (wrong units, missing notch, invalid boundary) fail for the right reasons.
- **Agent gate (separate from CLI gate):** an LLM agent given only MCP +
  docs produces a *semantically equal* instance; the CLI serializer then
  golden-matches it. Two independent pinned-model sessions. LLM
  nondeterminism never blocks the CLI exit path.

### G3 — Construction & grading
- **Exit:** bodice + set-in sleeve with **declared ease** (cap ease is
  intentional, not an invariant violation); darts, folds, walking/truing
  operations; stable references survive edit/split/mirror (or visible repair
  tasks); both instantiation paths; `.rul` with grade-point identifiers
  re-imported into an independent engine; extreme sizes reconstructed and
  measured; grading modes (base+rules / embedded / all-contours) validated
  separately; offset + grading golden suites green.
- **Exit (envelope coverage):** every garment §3.2 names drafts, grades and
  exports at this gate or an earlier one — the A-line skirt at G2; the darted
  bodice and set-in sleeve above; a classic collar with a stand, a fall and a
  roll line; trousers carrying at least one pocket and one closure whose
  buttonhole length is derived from its button. A garment the envelope names
  and no exit criterion proves is a gate failure, not a scope note.
- Domain-complexity note: skirt → bodice+sleeve is a domain jump (armscye,
  cap ease, balance notches) and trousers are a second (crotch curve, inseam,
  waistband); an intermediate garment may be inserted where the domain evidence
  calls for one — gates slip on domain evidence, not on engineering — but an
  intermediate is a means, never a substitute for the coverage criterion.

### G4 — Profiles & uncertainty workflow
- **Exit:** typed constraint AST + CSP solving with explained unsat mapped
  to fields (custom solver; differential-oracle campaign green); minimal guided **Profile Editor** (CLI interview or single-page
  form over sc-api) — NOT the full app editor; artifact policy matrix (§8.2)
  enforced; per-claim evidence store with supersession; profile composition
  precedence; HPGL writer + known-good real-plotter fixture in conformance/;
  private-vs-public profile paths working; usability gate: a non-programmer
  sewing expert creates a profile unaided in one session (early expert
  feedback on terminology is a feature, not a delay).

### G5 — Application shells & validated 2D UX
- **Exit:** Tauri native (Win/macOS/Linux) + WASM web per runtime profiles;
  full UX spec implemented: 2D drafting canvas, piece manager, grading
  panel, full Profile Editor, uncertainty dashboard, export wizard;
  UI↔API↔MCP workflow parity table proven by agent E2E on all targets +
  real native UI test suite (pointer/keyboard/focus/screen-reader — MCP tests
  are NOT UI tests); supported OS/browser matrix + installer/update behavior;
  i18n first complete language pack (reviewed, thresholded) + RTL verified
  (layout mirrored, geometry untouched); minimum tech pack contents complete
  enough for a factory quote (piece list w/ cut qty & fabric, POM+grade,
  stitch/SPI per seam, SA per edge, notions, version/date/style id).

### G6 — Conformance lab & reliability
- **Exit:** real import-filter validation (receiver's product/version/import
  settings recorded; manual repair documented, never labeled automated
  acceptance); physical plotter + printed-pattern checks; factory pilot
  loop live with rejection-reason taxonomy feeding profile calibration +
  Profile Editor copy; foreign-DXF import-diff (factory returns modified
  files → semantic delta → candidate profile updates); recovery/migrations/
  malformed-input/performance/cancellation matrix at supported scale;
  cross-platform regression matrix; fuzzing at depth where parsers exist.

### G7 — Scoped production declaration
- **Exit:** independent evidence review; supported-envelope statement with
  named limitations; semver + schema/API policy committed; install/reopen/
  upgrade/rollback evidence; release channels; governance in force.

### V1 — Assembly visualization (parallel track)
- Prerequisites: SewingGraph (G0), mesh (post-G3). Exit: mesh + seam
  correspondence validated against supported constructions; valid AND
  intentionally defective assemblies (blinded, with false-pos/neg reporting);
  arrangement surfaces + layer index; viewport integration native + browser.
  Does not block any G-gate.

### V2 — Physically validated simulation (parallel track, uncapped)
- Exit only when quantitative comparison against physical garments/material
  tests supports specific claimed observables. No fit claims before that.

---

## 12. Community, governance & contribution

- **Public repo**: code, reference fixtures, sanitized contributed profiles,
  golden files. **Never**: customer data, body scans, factory trade secrets.
- **Profiles are private by default**; contribution = explicit share action
  with a preview of exactly what leaves the machine.
- **Domain review is not code review.** Profile changes that alter exported
  bytes need a named domain-expert approval path (two-step: expert +
  maintainer). Wrong notch defaults are cut-floor incidents; reviewers must
  be peers of the knowledge, not of the code.
- Governance written before the community arrives: sewist-vs-programmer
  conflict resolution (Valentina/Seamly2D history is the cautionary tale),
  golden-file approval ownership, funding/procurement owners (eval seats,
  physical plotter — named people, G0).
- Headless-first has a community cost: no sewist-facing UI until G5 is
  mitigated by the G4 minimal editor and the dev shell — the experts stay in
  the room from G4 on.
- i18n contributions follow the same no-code review-queue path (§7.6).

---

## 13. Testing & conformance strategy

- **Layers of truth (in order of authority):** (1) native semantic invariants
  + analytic geometry checks; (2) independently produced external fixtures
  with documented meaning; (3) differential inspection with separate readers;
  (4) import through actual target products with recorded options; (5)
  printed/cut/sewn physical checks. Each claimed feature maps to its oracle
  in a conformance matrix.
- **Golden files**: canonicalizer → frozen bytes for regression; semantic
  equality with tolerances for correctness. No timestamps; stable IDs;
  fixed float formatting; cross-platform determinism policy.
- **Metamorphic tests**: unit conversions, rigid transforms, mirror-twice,
  save/load — with per-invariant tolerances.
- **Mutation tests**: wrong units, missing marks, reversed seam
  correspondence, broken grade refs, omitted pieces — the harness must
  detect each deliberately injected fault.
- **Property tests (domain-corrected)**: seam-length equality holds only for
  declared correspondences without ease; grading monotonicity only for
  specified measurements; piece closure; offset validity; reference
  stability under boundary edits.
- **Round-trip honesty**: DXF projection `P(native)` is what's compared; the
  loss report classifies preserved/approximated/omitted/unsupported. Native
  save/load preserves full semantics. Two patterns with identical contours
  but different drafting intent remain distinct natively.
- **Fuzzing**: when parsers exist (DXF import is G6 scope; the writer is G2).
- **Agent suites**: conformance, task evaluation, and native UI tests are
  three separate suites (§7.8, §11 G5).

---

## 14. Risk register (v2)

| Risk | Gate / mitigation |
|---|---|
| License × solver coupling silently copylefts the core | ADR-0001 at G0; feature flags |
| WASM parity assumption (Z3/SQLite/wgpu/webview) | Three runtime profiles; G1 real-browser spike |
| Construction paradigm slides into mechanical-CAD sketching | ADR-0003; formula language specified at G0 |
| Offset/SA geometry sinks the 2D slice | Pathology corpus; G2 exit criterion |
| Stage-1 agent gate flakiness blocks progress | CLI golden ≠ agent gate; two-session pinned-model protocol |
| Dual instantiation paths treated as one | G3 exit requires both, extreme sizes validated |
| Byte-stable DXF treated as an LLM property | Canonicalizer + semantic equality |
| Factories never publish real profiles | Private-by-default profiles; sanitized public subset |
| Profile-wide "verified" from one acceptance | Per-claim evidence with scope |
| Headless-first loses sewist contributors | G4 minimal editor; dev shell |
| Community fork over governance | Governance doc at G0, while the room is empty |
| Golden-file cross-platform nondeterminism | Determinism policy §13 |
| Fit claims outrun simulation evidence | V2 track gate: physical evidence only |
| Eval-seat procurement slips | Named owner at G0; partner-run manual test as documented fallback |
| MCP write tools without threat model | Stdio-only until G5+ model; scoped authority §10 |

---

## 15. Decisions locked by this revision (was "open questions")

1. Drafting paradigm: construction recipe primary (ADR-0003). Formula
   language v1 specified at G0.
2. Runtime model: three profiles; WASM is a documented capability subset.
3. Canonical data: construction recipe + explicit primitives; DXF is a lossy
   projection with a loss report.
4. Unknowns: categorized states + artifact policy matrix; no universal
   conservative defaults.
5. Constraint fragment: typed AST, discrete v1, explained outcomes.
   Production solver: custom deterministic finite-domain CP engine (pure
   Rust, native + WASM); Z3 is a CI-only differential oracle, never shipped.
6. First validated interchange: one named DXF mode + `.rul`, receiver-config
   recorded, at G6; harness matrix built at G2.
7. Numerics: fixed-point µm internally; per-class tolerances; offset error
   budget declared.
8. Signoff: release manifest + scoped human approval; graduated acceptance
   states.
9. License: ADR-0001 single decision before solver code; default
   recommendation permissive core.
10. UI: Tauri+TS chrome; canvas-hosting settled by G1 spike; egui dev shell
    for engine stages; domain logic banned from TS.
11. Size sets: first-class object; default ownership per §3.4.
12. Notch types in domain model: full typed set at G3 (v1: single/double +
    drill; V/I/T/U/castle by G4 export).

---

## Appendix A — Disposition log (what the peer reviews changed)

**Adopted from all three reviewers (consensus):**
- Construction-recipe drafting paradigm as primary model (Grok §3.2 as
  sharpest formulation; GPT R02; Gemini §1.2).
- Garment ontology completion: sewing graph, ease, darts, per-edge seam
  allowance, notch types, piece identity/labels (Grok §4.4; GPT R02; Gemini §1.3).
- Two instantiation paths (regeneration + grade rules) (Grok §4.5; GPT R09).
- Units/tolerance/offset numerical contract (GPT R04; Grok §4.12; Gemini §2).
- ASTM D6673 withdrawal + corrected layer table + BLOCK/SST/PST + R12/R13
  (Grok §4.1–4.2; GPT R11).
- DXF non-canonicity: canonicalizer, semantic equality, agent gate split
  (Grok §4.3; GPT R12–R13).
- Three runtime profiles, WASM dependency caveats (Grok §3.4; GPT R15).
- License×solver single ADR (Grok §3.1; GPT R26).
- Tauri canvas-hosting spike; egui dev shell; TS domain-logic ban
  (Grok §3.5; GPT R16).
- Command bus over graph CRUD; UI parity as invariant (GPT R17–R18; Grok §5.2).
- Stage restructure G0–G7 + V-tracks (GPT §D adopted as backbone; Grok §6.1
  sequencing fixes: profile editor split G4/G5, HPGL fixture G3/G6, SVG demoted,
  principle dedup, section renumbering).
- Uncertainty categories + artifact policy matrix; no universal conservative
  defaults (GPT R06).
- Per-claim evidence vs profile-wide verification (GPT R07).
- Bounded constraint fragment with explained outcomes (GPT R08).
- Factory profile schema expansion (Grok §4.8; Gemini §1.3–1.4; GPT R05
  concern separation: design intent / material / process / target dialect).
- Shrinkage as explicit pre-export transformation (Gemini §1.4; GPT R05).
- 3D: arrangement surfaces, layer index, ease-aware resampling, net-line
  binding, downgraded claims (Gemini §1.1; GPT R21).
- Private-by-default profiles + governance before community (Grok §6.3, §7).
- Release manifest + scoped approvals (GPT R14).
- Tech pack staged minimum; order vs design separation (Grok §4.11; GPT R23).
- Persistence authority single-source; recovery early (GPT R19).
- i18n: one message system chosen at G0; locale-aware diagnostics; RTL
  geometry-invariance; shipping staged (GPT R24; Grok §6.4).
- Agent authority scoped from G1; stdio-first MCP (GPT R25; Grok §5.2/§7).
- Z3 disposition (post-review decision, owner-approved): custom deterministic
  finite-domain solver as sole production CSP (native + WASM); Z3 demoted to
  non-shipped CI differential oracle (`csp-z3`). Eliminates the WASM/heaviness
  concern raised by Grok §3.4 and GPT R15 for the CSP specifically.

**Deliberately NOT adopted (with reason):**
- Multi-size-in-one-DXF+".rul" as single universal package: split into named,
  separately validated modes (all reviewers; GRAFIS-documented divergence).
- "import/export DXF round-trip as THE signoff harness": kept as regression
  tool only; real importers + physical checks are the signoff oracle (GPT R12).
- Marker making: remains an explicit non-goal (all reviewers concurred).
- Full formula-language spec content: deferred to G0 execution (ADR-0003
  names its scope now).

**Amended after review (v0.3), with source:**
- §11 G3 adds whole-§3.2 envelope coverage, replacing the domain-complexity
  note as a substitute for proof. The engineer's proposal under the director's
  delegation closes D32: collar, trousers, buttons and pockets lacked proving
  exits. Four `(proposed)` matrix cells become gates; locked scope is unchanged.
  Exact prior disposition, proposal and reasoning are preserved in
  `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`.
  The proposal author applied it under governance §6.1; its evidence is derived
  by instruments and that author may not approve it (self-application decision).
**Director clarification applied within v0.3, with source:**
- §2 adds the director's explicit incremental-publication requirement: roadmap,
  code and book in lockstep, glossary and topic index, detailed expert annexes.
  G1-SLICE.4d.1 owns adoption and scoped status/navigation verification. This
  changes documentation obligations, not locked product scope or gate exits;
  `docs/decisions/decision_book-progression.md` records the source and application.

---

*End of roadmap v0.3. Functionality is paramount; gates exist to protect it.
"Implementing an exporter" and "proving its compatibility" are different
achievements — this document tracks the second.*
