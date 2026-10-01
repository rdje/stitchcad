# Garment ontology

> **Status:** normative specification, gate **G0** (roadmap §3.1, §4.1). Specified here; implemented by
> `sc-core` and `sc-measure` at gate G1 (leaves `G1-SLICE.3`, `G1-SLICE.4`). Pieces are now structurally
> implemented with whole-interval repair queries (§10); other object types follow. Terms used below are
> defined in the [glossary](glossary.md).

This chapter defines every first-class object in a StitchCAD design: what it is, what it must carry,
what must hold for it to be valid, and how it keeps its identity when the geometry around it changes.

Three principles govern the whole model:

1. **The design is a recipe, not a drawing.** The authoritative content is the measurement table, the
   formula graph and the ordered drafting operations. Contours, meshes and render data are *derived*
   and are never authoritative (§2.2 of the roadmap). Imported geometry that has no history is stored
   explicitly as primitives and is always distinguishable from drafted geometry.
2. **Identity is stable and topological.** Notches, grade points and seam spans reference *entities*
   (edges, points), never array indices or tessellation vertices (§4.1).
3. **Uncertainty is part of the object, not a comment about it.** Every parameter carries a state —
   known fact, unknown fact, selectable choice, overridable preference, or derived value — and unknown
   facts are never assigned a value by the solver.

## 1. Identity

Every semantic entity carries an **entity id**: a ULID, assigned once at creation, never reused, never
re-derived from content. Ids are stable across saves, grades and exports, which is what makes evidence,
approvals and manifests able to name a specific object.

Geometric sub-entities are addressed through **stable topological references**, not indices:

- `EdgeRef` — an edge of a piece boundary, of a hole, or of an internal construction line, identified
  by the entity id of the operation that created it plus a local, persistent tag.
- `PointRef` — a constructed point (an intersection, a notch anchor, a grade point), identified the
  same way.

A reference is **parameterized** where the consumer needs a position along an edge: the parameter is a
rational in `[0, 1]` of that edge's own length, so it survives a change of tessellation and a change of
units. It is never a vertex index.

### 1.1 The persistent-identity contract

When an edit changes the topology a reference points into, exactly one of two things happens:

| Edit | Required behaviour |
| --- | --- |
| split an edge | references to the original edge resolve to the fragment that still contains their parameter; references at the split point resolve to both, and the consumer states which it wants |
| merge edges | references survive onto the merged edge with their parameter recomputed by arc length |
| reverse an edge | references survive; parameters become `1 − t`; directed consumers are told |
| delete an edge | every reference to it becomes **unresolved** and produces a repair task |
| fragment by offsetting | references are mapped to the offset fragments where the mapping is unambiguous, otherwise unresolved |

**No silent reassignment.** An unresolved reference is a first-class, visible object — a *repair task*
— that names the reference, the edit that orphaned it, and the candidate resolutions. A design with
unresolved references can be saved and inspected; it cannot be released (§8 of the release contract).

## 2. Measurement and fit

### 2.1 `MeasurementTable`

A named set of scalar measurements. Each entry carries:

- **name** and **machine token** (the token is stable and never rendered raw to a user);
- **value** in internal units, plus the **unit** it was entered in;
- **kind**: `body` (an ISO 8559 / ASTM D5219 style body measurement) or `garment` (a point of measure
  of the garment itself, a POM);
- **landmark** — the anatomical or garment landmarks the measurement runs between, as references, not
  prose;
- **procedure** — how it is taken, as a reference to a documented procedure, not free text;
- **source** and **state** (§5).

A measurement without a landmark or a procedure is **rejected**: an unrepeatable measurement cannot be
evidence for anything, and a factory dispute about "the chest" is a dispute about landmarks.

Body measurements and garment POMs are distinct kinds and are never silently interchanged. The
relationship between them is `Ease`.

### 2.2 `Ease`

Ease is a first-class mapping from a body measurement to the corresponding garment POM, per POM:

- the **body measurement** it derives from;
- the **garment POM** it produces;
- the **ease value** (a length, signed — negative ease is compression and is legal only where declared);
- the **fit intent** it belongs to: `close`, `semi`, `loose` (a named, ordered vocabulary, so a fit
  intent can be compared, filtered and validated against a size chart);
- its **state** and **provenance**.

Ease is what makes measurement-driven regeneration and factory grade rules reconcilable: without it, a
graded size and a re-drafted size differ by an amount nobody can explain.

### 2.3 `SizeSet`

The set of sizes a design is instantiated in: labels, their order, the base size, and the mapping to a
size system (EN 13402, ASTM D5585, alphanumeric, numeric, or custom). Ownership — whether the size set
belongs to the design, the factory profile, or an order object — is decided in the [size-sets chapter](size-sets.md).

## 3. The design

### 3.1 `Design`

A `Design` is the construction recipe plus its semantic content:

- **identity and revision** — an entity id and a monotonically increasing revision counter, which is
  what a command's revision precondition and an approval bind to;
- **parameters** — named values exposed to formulas, each with a state (§5);
- **a `MeasurementTable`** reference and an **`Ease`** set;
- **a `SizeSet`** reference;
- **the formula graph** — an acyclic graph of expressions over measurements, parameters, profile
  parameters and previously constructed points/lengths/angles, in the language the
  [formula language](formula-language.md) chapter specifies;
- **the ordered operation list** — the drafting history, replayable from scratch;
- **the `SewingGraph`**;
- **materials** and their assignment to pieces;
- **imported geometry**, if any, stored as explicit primitives with `origin: imported` and no
  fabricated history.

Evaluation is deterministic and single-pass over the acyclic recipe graph (§6.1 of the roadmap): given
the same inputs, the same design produces the same geometry on every platform, byte for byte after
canonicalization.

### 3.2 Drafting operations

Operations are typed, ordered and replayable. The v1 set covers the supported envelope: construct
points from formulas; draw lines, arcs and Béziers; offset an edge by a seam allowance; slash and
spread; close a dart; add a notch; place a grainline; walk and true a seam; mirror; join pieces at a
seam. Every operation is invoked through the command bus, which is the only mutation path.

Two operations are first-class editing operations rather than validation checks, because a patternmaker
performs them deliberately:

- **`walk`** — traverse a seam correspondence (a sleeve cap around an armscye), reporting the length
  differential per span and planting balance notches at declared positions.
- **`true`** — adjust a seam so that its two sides match within the declared tolerance, attributing any
  remaining differential to declared ease rather than hiding it.

## 4. Geometry-bearing objects

### 4.1 `Piece`

A pattern piece. Required content:

- **outer boundary** — a closed loop of edges, oriented **counter-clockwise** in the piece's own frame;
- **holes** — zero or more closed internal loops (cutouts), each oriented opposite to the boundary;
- **internal construction lines** — non-cutting geometry (fold lines, placement lines, dart legs);
- **multiplicity** — how many of this piece are cut;
- **mirroring** — whether the piece is cut as a pair with one mirrored, or as a single asymmetric piece;
- **cut-on-fold** — whether the boundary includes a fold edge, and which edge it is;
- **face / wrong side** — which side is up when cutting;
- **material** — an assignment, or an explicit unresolved state;
- **layer index** — the ordering used by 3D assembly (see the V1 track);
- **identity and label data** — entity id plus the printed label content: name, size, cut quantity,
  pair L/R, fold indicator, fabric and colorway.

Invariants: the boundary is closed and simple; the winding is CCW; holes lie strictly inside the
boundary and do not intersect each other; a piece marked cut-on-fold has exactly one fold edge; label
data is complete enough to print without consulting anything else.

### 4.2 `SeamSpan` and `SewingGraph`

A `SeamSpan` is an oriented correspondence between two edge ranges:

- the **two sides** — `(piece, EdgeRef, parameter range)` each;
- **direction** — which end of side A meets which end of side B;
- **declared ease distribution** — how a length differential is spread along the span (uniform,
  weighted to a region, anchored between notches);
- **stop landmarks** — notches or turn points where the sewing stops or changes direction.

Spans may be **partial** (a sub-range of an edge) and **one-to-many** (one edge range sewn to several),
which is how a sleeve cap meets an armscye with a shoulder notch in between.

The `SewingGraph` is the first-class object holding the spans. It lives in the `Design`, not inside the
meshing code: assembly order, walk/true operations and 3D stitching all read it, and a garment whose
sewing graph is an implementation detail of its renderer cannot be validated.

Invariant: every span references edges that exist; a differential beyond the declared ease distribution
is a *finding* (reported by `walk`), not a silent stretch.

### 4.3 `Dart`, `Tuck`, `Pleat`, `Gather`

Closures with semantics, not line art. Each carries its **intake** (the length it removes or absorbs),
its **apex** or fold line, its **direction**, and the **operation** that closes it. Slash-and-spread is
a drafting operation on the recipe, not a property of these objects.

Invariant: intake is conserved by the closing operation — closing a dart removes exactly its intake
from the boundary length, within the numerical tolerance class.

### 4.4 `SeamAllowance`

A **derived object attached to an edge**, not a global switch:

- **width** per edge (a length, from a formula, a profile parameter, or an explicit value);
- **corner treatment** — miter, slant, envelope, trim, or step;
- **inclusion policy** — whether the allowance is included in the stored contour or generated
  downstream. This is resolved **per Factory Profile**, because it is exactly the kind of thing two
  cutting rooms disagree about; it is never a project-wide boolean.

Because allowances are derived by offsetting, they inherit the offset engine's error budget: an
allowance that cannot be produced within the declared bound fails explicitly rather than emitting
self-intersecting geometry.

### 4.5 `Notch`

A semantic matching mark whose physical representation is resolved at export:

- **position** — an `EdgeRef` plus a parameter, so it survives edits to the edge;
- **type** — single, double, V, I, T, U, castle, slit, drill;
- **geometry** — depth and width, with sample-room and production values where they differ;
- **encoding** — whether the target receives a coded point (position + direction + depth + type) or
  drawn geometry.

All of type, geometry and encoding are **Factory Profile parameters**; the ontology stores the
semantics, the profile decides the bytes.

### 4.6 `Grainline`

A **directed** entity: it has a direction, so "grain parallel to centre back" is expressible and
checkable. It supports bias and off-grain by an angle, and supports **dual references** — a stripe or
plaid reference in addition to the garment grain — because a striped fabric constrains placement
independently of the weave direction.

### 4.7 `Hem`, `Facing`, `Lining`, `Interfacing`, `Closure`, `Pocket`

Each is an object with its own parameters rather than a convention about how lines are drawn:

- **Hem** — depth, fold type, the edge it finishes, and whether it is turned or faced.
- **Facing / Lining / Interfacing** — the piece they serve, their offset relationship, and their
  material assignment.
- **Closure** — button/buttonhole pairs and zippers, with placement positions, counts and sizes; a
  buttonhole's length is derived from its button, not entered twice.
- **Pocket** — position, orientation, opening type, and the piece(s) it is composed of.

A construction outside the supported envelope produces an explicit diagnostic (see the feature matrix),
never an approximation.

## 5. Uncertainty states

Every parameter in the model — a measurement, an ease value, an allowance width, a notch depth, a
profile parameter — carries one state:

| State | Meaning | May it be exported? |
| --- | --- | --- |
| `known` | a fact with scoped evidence | yes |
| `assumed` | a value taken on a recorded human assumption | yes, with the assumption in the manifest |
| `unknown` | requires observation; **no value is invented** | governed by the artifact policy matrix |
| `preference` | an overridable default with provenance | yes, marked as a preference |
| `derived` | computed from others; its state follows its inputs | follows its inputs |

An `unknown` is never silently defaulted. Defaults exist only as `preference` values with provenance,
and the artifact policy matrix — not the object — decides what an unresolved unknown blocks.

## 6. Materials

A `Material` carries the properties the product actually uses: name, composition where known, width,
whether it has a nap or a directional print, whether it is striped or checked (which activates the
dual grainline reference), and its shrinkage if measured. Shrinkage is applied as an explicit
transformation at a declared stage of the pipeline, never folded silently into a coordinate.

## 7. Serialization

The canonical project format is a directory of canonical-text files plus attachments, with:

- stable key order and declared float formatting;
- ULID entity ids;
- no wall-clock values in canonical content;
- a schema version and a migration path;
- preservation of unknown extensions — a file written by a newer version is not silently stripped by an
  older reader.

Two saves of the same state are byte-identical. Derived data (tessellations, meshes, caches) is not
part of the canonical project; it is regenerated.

## 8. Verification status of external claims

- **ISO 8559 / ASTM D5219 describe body measurement landmarks and procedures** — cited from the
  roadmap. The standards' texts have not been read in this repository; they are confirmed against the
  documents in the measurement-standards chapter, which owns that verification.
- **EN 13402 / ASTM D5585 define size designation systems** — cited from the roadmap; same status.
- **AAMA/ASTM notch encodings and layer conventions** — recorded in the [interchange
  dialects](interchange-dialects.md) chapter (§3 for the layers, §6 for the withdrawal) with its own
  verification status.
- **ULID as the identity scheme** — a project decision, not an external claim; recorded in the
  decisions index.

## 9. What must be true in tests

- **Identity stability:** references survive split, merge, reverse and offset-fragmentation, or become
  visible repair tasks — never silently reassigned (§1.1).
- **Invariants at construction:** an invalid piece (open boundary, wrong winding, hole outside the
  boundary) cannot be built; the diagnostic names the invariant.
- **Dart intake conservation** within the numerical tolerance class.
- **Ease attribution:** `walk` reports a differential and attributes it to declared ease or to a defect.
- **Recipe determinism:** re-evaluating a design twice, and on two platforms, yields identical
  canonical output.
- **Unknown facts stay unknown:** a parameter in the `unknown` state has no value in the serialized
  form, and any export that needs it is blocked or badged per the policy matrix.
- **Save/load preserves semantics**, including drafting intent: two designs with identical contours but
  different recipes remain distinct.

## 10. Executable pieces at G1

`sc_core::ontology::piece` implements §4.1's **structural** content (`G1-SLICE.3c.1`).
`Piece::new(PieceDefinition, &IdentityLedger)` returns an immutable `Piece` or a typed `PieceError`
naming the failed invariant. `PieceDefinition` is editable input; it is not a validated domain object.
A command replaces a piece through this constructor rather than modifying its validated fields.

The boundary and each hole are cyclic sequences of `DirectedEdge { edge, direction }`. The last edge
implicitly precedes the first; **do not repeat the first edge at the end**. Empty loops, repeated cut
edges (including an edge shared by two cut loops), and references absent from the current ledger are
rejected. Construction lines must also name live edges. A cyclic list does not prove that geometric
endpoints coincide: even a one-edge curve loop still requires G2's closure check.

The cut quantity is the **total number of physical copies**. `Mirroring::Single` accepts any positive
quantity; `Mirroring::MirroredPairs` requires an even quantity (two means one L/R pair within one
piece definition). Separate cut-once L/R members use `Mirroring::PairMember { handedness, companion }`:
quantity counts this member's copies, the handedness is explicitly left or right, and the companion is
another Piece identity. This is the reference skirt §6's back-piece form. A member cannot name itself
as its companion; companion existence, reciprocity, opposite hands and equal quantities are checked
at design-collection validation (`G1-SLICE.6`), not guessed by the standalone constructor. Actual
geometric mirroring stays G2's obligation. A cut-on-fold
piece declares exactly one fold edge on its outer boundary; other pieces declare none. The fabric
side and assembly layer index are explicit content. Material assignment is either a material id or
`Unresolved(reason)` with a nonblank explanation: an unresolved assignment is not defaulted to fabric.
The design will validate material identities against its material collection when that collection lands.

Printed labels carry nonblank name, size, fabric and colorway text. Their cut quantity, pair L/R and
fold indicator derive from the cut plan rather than being separately editable inputs. For example:

- One mirrored-pair definition with quantity `2` prints cut two, pair L/R.
- Separate `skirt_back_right` and `skirt_back_left` definitions with quantity `1` each carry opposite
  handedness and reciprocal companion ids, and print their own cut-one R / cut-one L labels.
- A single front on fold with quantity `1` and one boundary fold edge prints cut one on fold.
- A material awaiting selection may carry the explicit reason "awaiting fabric selection" and print
  "unassigned woven fabric" / "undetermined" for fabric and colorway. This is inspectable content,
  not evidence that a production release is acceptable.

`definition()` exposes authored content by shared reference; editing a clone does not change the
validated piece. `edges()` includes boundary, hole and construction-line references. After a ledger
edit, `endpoint_resolutions()` resolves the held traversal endpoints and exposes `Resolution` values,
including repair tasks; it never rewrites the authored references. `endpoint_references()` supplies
an endpoint inventory for the command bus's future atomic registration step. Endpoint queries **do not
certify complete-edge integrity**: deleting an interior split fragment can leave both original endpoints
resolvable. Whole-interval evidence now answers that separate question, as described below. Full
recipe/geometry validation remains required before release.

Every piece reports `GeometricValidation::DeferredToG2`. This is deliberately **not** a valid-geometry
badge. Endpoint closure, CCW outer winding, opposite hole winding, simplicity, holes strictly inside
and non-intersecting, and intake conservation remain `G2-2D.1`'s obligations. G1 has no constructor
that marks these checks passed; this implements the boundary recorded in
`decision_ontology-invariants-structural-g1-geometric-g2.md`.

**Whole-interval queries (`G1-SLICE.3c.2a`).** `EdgeRange::new(edge, from, to)` requires exact
ascending bounds `from < to`; `EdgeRange::whole(edge)` covers `[0,1]`. A zero-length anchor is a
point reference. `IdentityLedger::resolve_range` partitions the entire interval through each journal
edit, with no grid or endpoint sampling. Its `RangeResolution` retains every `RangePortion`, in the
held edge's original forward traversal: live portions carry the current edge, local bounds and
`Direction`; lost portions carry a `RangeRepairTask` naming the original held range, affected local
interval and the orphaning operation/cause. Existing causes have no candidate interval mapping;
endpoint ambiguity carries its candidates in the separate point-resolution answer.

For example, split a boundary edge into three equal fragments and delete the middle. Resolving the
original endpoints still succeeds, but resolving the entire edge returns live first fragment, a
repair for the deleted middle fragment, live last fragment. `has_full_coverage()` is false. The same
query on a sub-range entirely inside the first surviving third has full coverage. An offset gap,
however narrow, likewise becomes an explicit uncovered interval; exact-arithmetic overflow becomes a
recomputation repair, never an approximation. An edge never created in the ledger is `UnknownSource`.

`start()` and `end()` on the range result are the original endpoint **point** resolutions. A bound
exactly at a split point can still offer both sides; a bound on a shared offset boundary can still be
ambiguous. Full positive-length coverage does not resolve those choices. It also does not prove
continuity, winding or geometric validity, and grants no approval. A repair remains visible after
later edits: a surviving endpoint or new unrelated edge cannot silently reattach lost interval content.

`Piece::range_resolutions()` supplies this full-edge evidence for boundary, holes and construction
lines, alongside each authored `DirectedEdge`. Range portions are ordered forward in the stored edge's
frame; a reversed authored traversal consumes that sequence backwards and reverses the fragment
directions. The future command bus must consider range repairs **as well as** point repairs. The
existing point-only registration verdict keeps its narrower meaning until design-level registration
and validation land. The interval contract and its limits are recorded in
`decision_range-resolution-preserves-entire-interval.md`.
