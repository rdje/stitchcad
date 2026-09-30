# Interchange dialects

> **Status:** normative specification, gate **G0** (roadmap ADR-0004, §7.5, and §11's G0 exit clause:
> "ADR-0004 (dialects) recorded"). Implemented by `sc-artifacts` from gate G2 — one named mode first —
> and validated receiver by receiver at G6. Terms are defined in the [glossary](glossary.md); every
> external claim carries a status from the [standards chapter](standards.md) §1.

A cutting room does not accept "a DXF". It accepts a DXF whose layers are named or numbered the way
its own software expects, whose layer 1 carries the line its knife follows, whose entities are of a
kind its importer reads, and whose size information arrives in the form its grading table does. Those
choices are not one standard with options; they are **dialects**, and the disagreement between them is
what rejects patterns.

So this chapter does two things. It fixes the axes a dialect is made of, so a target is a named point
in a declared space rather than a bundle of writer flags. And it refuses the thing the roadmap's
disposition log refuses: **there is no universal package.** An artifact that claims to serve every
receiver serves the ones nobody tested, and the refusal is a diagnostic
([envelope §10](feature-matrix.md), `env_universal_package`), not a caveat in a manual.

## 1. The axes of an export target

An export target is a tuple over six axes. Each axis has a closed value set, a token, and a party that
resolves it — because an axis nobody owns is an axis whose default was chosen by an implementation.

| Token | Axis | Values | Resolved by | In the registry as |
| --- | --- | --- | --- | --- |
| `layer_naming` | layer naming | AAMA named layers, ASTM numbered layers | the target | Layer naming |
| `dxf_version_target` | DXF release | R12, R13 | the target | Release |
| `cut_sew_swap` | line semantics on layer 1 | cut-as-1, sew-as-1 | the Factory Profile (§4) | — |
| `seam_allowance_policy` | allowance inclusion | included in the contour, generated downstream | the Factory Profile (ontology §4.4) | — |
| `grading_mode` | grading carriage | single size, base plus rules, embedded rules, all contours | the target (§8) | Grading carriage |
| `entity_policy` | entity policy | polyline only | the release (§6) | Entity policy |

The last column is what makes the registry checkable in both directions: an axis the target resolves
must be a column of §2, and a column of §2 must be an axis declared here, so neither a silent axis nor
an undeclared one can appear.

Two rules make the space honest rather than merely large:

- **A tuple nobody has validated is not a target.** The registry in §2 is the list of targets; a
  request for a combination it does not carry is refused with `dialect_unregistered`, naming the
  nearest registered target and the axis that differs. Combinatorial possibility is not a capability.
- **Every axis value is recorded in the artifact's receiver-config record (§10).** A receiver's
  expectation and our output are then comparable as data, which is what makes a G6 finding scoped to
  that receiver and version instead of an argument.

## 2. The target registry

| Target | Layer naming | Release | Grading carriage | Entity policy | Proven at |
| --- | --- | --- | --- | --- | --- |
| `dxf-aama-named` | AAMA named | R12 | single size, then base plus rules | polyline only | G2 (the one mode its exit names), G6 receivers |
| `dxf-astm-num-r13` | ASTM numbered | R13 | single size, then embedded rules | polyline only | G2, G6 receivers |
| `plt` | not applicable — a pen map (§9) | not applicable | single size | polyline only | G3 writer, G4 real-plotter fixture |
| `pdf-a0` | not applicable — pages (§9) | not applicable | single size | curve preserving | G2 print check |

The two DXF targets differ on more than layer names, and the difference is the reason both exist: the
AAMA-named target is the one a plotter-and-marker workflow expects, and the ASTM-numbered target is
the one a cutting room enforcing D6673's numbered convention expects (§6). Neither is a superset of
the other, so neither is a default.

Adding a target is adding a row here **and** a validation at G6 with a named receiver and version. It
is never a writer flag, and a target whose validation lapses (a receiver upgrade changes its
importer) is a finding that removes the row rather than a warning nobody reads.

## 3. The layer table, in both naming modes

The numbered convention below is the one the roadmap's ADR-0004 records as corrected; its status is
`cited-from-roadmap` (§12), which permits naming the layers and stating their role and forbids quoting
a clause of a standard nobody has read here. The **StitchCAD object** column is this project's own
mapping — a decision, not a claim about either convention.

| ASTM layer | Carries | StitchCAD object | AAMA name |
| --- | --- | --- | --- |
| 1 | the cut boundary, plus SST on the ASTM path (§5) | `Piece` outer boundary, allowance-resolved per §4 | CUT |
| 2 | turn points | a `POLYLINE` vertex where the boundary changes direction | CUT |
| 3 | curve points | the tessellation vertices of an approximated curve (§7) | CUT |
| 4 | V and slit notches | `Notch` of type V or slit | NOTCH |
| 5 | grade reference | `grade_point` entities (ontology §1) | REF |
| 6 | mirror line | the axis a mirrored pair was mirrored about | REF |
| 7 | grainline | `Grainline`, directed | DRAW |
| 8 | internal lines | internal construction lines: fold, dart leg, placement | INTCUT |
| 9 | stripe reference | the stripe half of the dual grainline reference (ontology §4.6) | REF |
| 10 | plaid reference | the plaid half of the same | REF |
| 11 | internal cutouts | a `Piece` hole, oriented opposite to the boundary | INTCUT |
| 12 | intentionally blank | nothing — and writing anything here is a defect | — |
| 13 | drill holes | `Notch` of type drill | DRILL |
| 14 | the sew line | the net line, allowance excluded | DRAW |
| 15 | annotation text | label data (ontology §4.1) | TEXT |
| 80–83 | T, castle, check and U notches | `Notch` of the matching type, staged by gate (§11) | NOTCH |
| 84–87 | quality-validation curves | **nothing in v1** — the model has no such object, and inventing one would put geometry in an artifact that no design authored | — |

Three rules follow, and each is checkable:

- **One layer, one meaning.** Every layer above is claimed by exactly one object family, and layer 12
  is claimed by none. A writer that puts two kinds of geometry on one layer has made the receiver's
  reading ambiguous, which is the D27 defect class in a file format.
- **The AAMA column is not a translation of the ASTM column.** Seven names cover seventeen numbered
  layers, so the named mode carries strictly less separation: turn points, curve points and the cut
  boundary share CUT, and a receiver reading CUT cannot tell them apart. That loss is declared here
  and appears in the loss report of an import (§7), rather than being discovered by a partner.
- **A layer nothing writes is written as absent, never as empty geometry.** Layer 12 and layers
  84–87 are absent from the entity list; a receiver that requires them is a G6 finding with a name,
  not a reason to emit placeholders.

## 4. Cut-as-1 and sew-as-1

Layer 1 carries either the **cut** line (the allowance is outside it, the knife follows it) or the
**sew** line (the net line, the allowance is generated downstream). Which one is a Factory Profile
parameter with an artifact effect, resolved per receiver, recorded in the manifest, and never inferred
from the design.

The hazard is the reason §4 exists as its own clause: a receiver that expects cut-as-1 and gets
sew-as-1 cuts a piece one allowance smaller than intended, and the file looks correct to both sides.
The roadmap records the swap as a top rejection cause, so three rules bind:

1. **It is a mapping, not an inversion.** The geometry keeps its meaning — a `SeamAllowance` is still
   attached to its edge, a net line is still a net line (ontology §4.4). The profile decides which of
   the two the target's layer 1 carries, and the other goes to the layer §3 gives it.
2. **The choice is recorded where a reader can see it without opening the file**: the receiver-config
   record (§10), the release manifest, and the printed label of a piece where the target's convention
   permits text.
3. **An unresolved expectation is a refusal.** A profile that names no line semantics for a target
   produces `dialect_line_semantics_unresolved` rather than a writer default, because a default here is
   a wrong cut at somebody else's factory.

## 5. Blocks and metadata

- **One BLOCK per piece**, named `[piece]_[size]`, with **no nested INSERTs**. A piece is the unit a
  cutter, a marker and a label all agree on, so it is the unit the file carries; nesting would make a
  receiver's piece count depend on its traversal.
- **SST (Style System Text) on layer 1**, with a case-sensitive syntax, and **PST (Piece System
  Text) per piece**. Both are mandatory on the ASTM path and are metadata, not annotation: they are
  what lets a receiver bind geometry to a style and a piece without a human re-keying either.
- **Golden files validate the metadata blocks, not only the geometry.** A byte-stable contour with a
  missing PST is a rejected artifact, so the G2 golden harness compares the blocks as part of the
  artifact and the G6 receiver validation reads them back.
- The exact field content of SST and PST is **not specified here**: the syntax is case-sensitive and
  receiver-specific, so its fields are a receiver-config matter recorded per validated receiver (§10)
  and confirmed at G6, not a table this chapter could invent.

## 6. Release profiles: R12 and R13

**ASTM D6673-10 is withdrawn** (January 2019, with no replacement), and this repository implements its
numbered-layer convention as a **de-facto convention** because cutting rooms still enforce it. Nothing
here claims conformance to it, to a withdrawn standard or to a current one
([standards chapter](standards.md) §1 and §6). The specification it names as its carrier is AutoCAD
R13 DXF.

| Entity | Written | What it carries, or why it is refused |
| --- | --- | --- |
| `POLYLINE` | both releases | every boundary, internal line, grainline and mirror axis — one entity family for all of it |
| `VERTEX` | both releases | a `POLYLINE`'s vertex, with a **bulge** where an arc ends there, so a circular arc is carried exactly and never tessellated (units §4) |
| `LINE` | both releases | a single straight segment where a receiver expects one: a grainline, a mirror axis |
| `TEXT` | both releases | SST, PST and label data — single-line, because a multi-line entity's layout is the receiver's to decide |
| `INSERT` | both releases | one per piece, referencing that piece's `BLOCK` (§5) |
| `BLOCK` | both releases | the piece container (§5), never nested |
| `ARC` | never | a standalone arc is refused although its geometry is exact as a bulge: a receiver accepting only `POLYLINE` is the case the entity policy exists for, and two families for one boundary is two ways to disagree |
| `CIRCLE` | never | the same reason; a notch hole is a closed `POLYLINE` of bulged vertices |
| `LWPOLYLINE` | never | a later release's lightweight form, and a receiver's acceptance of it is not established (§12) |
| `SPLINE` | never | a NURBS-class entity, which the envelope defers ([envelope §3](feature-matrix.md), `env_nurbs`) |
| `MTEXT` | never | multi-line text layout is receiver-dependent, so a label is one `TEXT` per line |
| `HATCH` | never | a fill is presentation, and an artifact carries cut and sew geometry |

The written set is the same for both releases, so one geometry writer serves both targets and the
difference between them is the layer naming and the metadata blocks, not the entities.

- **The entity policy is polyline-only**, which is an axis value (§1) and not an implementation
  convenience: a Bézier is tessellated at the declared bound (§7), an arc is carried exactly as a
  bulge, and the file carries no entity a legacy importer has to guess at.
- **DXF carries no intrinsic unit.** A length in a DXF file is a number whose meaning is an agreement
  between writer and reader, so the unit is **declared in the receiver-config record and agreed with
  the receiver**, never inferred from the file, and a receiver whose expectation is unknown is refused
  rather than assumed to be millimetres or inches.
- **The revival work the roadmap names is watched by nobody here yet.** A committee's activity is not
  observable without access this repository does not have, so the claim stays
  `unverified-with-owner` with the procurement seat as its owner (§12).

## 7. Tessellation policy

A polyline-only target approximates the curves it cannot carry exactly. The policy is declared rather
than tuned:

- **an arc is not approximated.** It travels as a bulge on the vertex that ends it (§6), so the
  geometry is exact and the loss report classifies it as preserved;
- **a cubic Bézier is tessellated**, at the T2 class bound of 100 µm for polyline-only targets
  ([units §3](units-and-tolerances.md)), which is a property of the target and not of a piece;
- geometry is computed entirely in internal units and flattened once, at serialization, so a
  flattening never feeds precision back into the model (units §2.1);
- every approximated edge is **recorded in the loss report as approximated**, with the achieved
  deviation, and the bound is published with the artifact;
- a curve that cannot be flattened inside the bound **fails the export** rather than shipping a
  coarser approximation — the offset engine's rule (units §6) applies to tessellation as well;
- an **import** classifies each entity it reads as preserved, approximated, omitted or unsupported,
  and the two naming modes' losses of §3 appear there too.

## 8. Grading carriage

Three modes, separately validated, never combined into one file:

| Mode | Carries | Canonical use |
| --- | --- | --- |
| base plus rules | the base size's geometry, plus a `.rul` table of per-point deltas | the canonical interchange of the AAMA era, and this product's default at G3 |
| embedded rules | the base geometry and the grade rules inside the DXF | the ASTM path, where a receiver reads rules from the file |
| all contours | every size's finished contour, with no rules | a named option for receivers that grade nothing |

The `.rul` attributes — incremental against cumulative, stack point, fixed perimeter, smoothing — are
given StitchCAD semantics in the [instantiation paths](instantiation-paths.md) chapter, which also
states what is lost between that carriage and measurement-driven regeneration. This chapter fixes only
which mode a target carries, and the rule that **a mode is validated receiver by receiver**: an
all-contours file that one importer reads is evidence about that importer, and nothing else.

## 9. HPGL/PLT and PDF

- **`plt`** — the plotter path. Coordinates are in plotter units, one unit being exactly 25 µm from
  1 016 units per inch ([units §2](units-and-tolerances.md)); the **pen map** (`hpgl_pens`) assigns a
  pen to each function — cut, draw, notch, drill — and is part of the receiver-config record, because a
  pen number is a machine setting and not a property of the geometry. Scale is declared, never fitted.
- **`pdf-a0`** — the review and print path, curve preserving: an A0 page, or A3/A4 **tiled** with
  overlap, registration marks and page order, plus a printed **scale square** of declared size. A piece
  that does not fit is tiled or the export fails; it is **never rescaled**, because a rescaled print is
  a pattern nobody drafted and the scale square exists to prove it with a ruler (a T5 check, units §3).

## 10. What every export records

The **receiver-config record** travels with the artifact and is part of the release manifest's
identity, so a dispute is about data and not about memory:

| Field | Why it is there |
| --- | --- |
| target token (§2) | which named dialect this is |
| every axis value (§1) | so a receiver's expectation is comparable field by field |
| the declared unit | DXF has none intrinsically (§6) |
| tessellation bound and the achieved maximum (§7) | so an approximation is published, not discovered |
| pen map, where the target has one (§9) | a pen is a machine setting |
| writer identity and version | a G6 finding is scoped to a receiver *and* a writer version |
| the design and profile revisions, and the approval state | the release contract's binding |

A record whose fields are prose is not a record: each field above is a stable token with a typed
value, and the localization of any human-readable summary is a presentation layer over it (the
[internationalization chapter](i18n-architecture.md) §3's rule).

## 11. Diagnostics

| Token | Raised when | Required arguments |
| --- | --- | --- |
| `dialect_unregistered` | a target combination the registry does not carry is requested | the axes asked for, the nearest registered target, the axis that differs |
| `dialect_line_semantics_unresolved` | a profile names no cut-as-1 or sew-as-1 for a target whose layer 1 needs one | the target, the profile, the two readings |
| `dialect_unit_undeclared` | a receiver's expected unit is not recorded and cannot be agreed | the target, the receiver, the candidates |
| `dialect_tessellation_bound` | a curve cannot be flattened inside the target's declared bound | the edge, the bound, the achieved deviation |
| `dialect_metadata_missing` | SST or PST is required by the target and the design cannot supply it | the block, the piece, the field |
| `env_universal_package` | one artifact is asked to serve every receiver | the receivers named, the registered targets instead |
| `env_notch_type` | a notch type staged past v1 is asked for on a target that would carry it | the requested type, the v1 set, the gate that owns it |

The last two are the [envelope](feature-matrix.md) §10's, raised here rather than duplicated: where a
refusal is about the supported envelope, the envelope's token is the one an agent sees, so two token
sets never compete for one refusal.

## 12. Verification status of the external claims in this chapter

- **The numbered layer table, the AAMA named set, the BLOCK/SST/PST requirement, the R12/R13 carrier
  and D6673-10's withdrawal** are all `cited-from-roadmap`: the roadmap records them with their own
  peer-review provenance, and [standards §2](standards.md) registers both designations with this leaf
  as their owner. Naming the layers and stating their role is permitted; quoting a clause is not, and
  nothing here does.
- **Two facts about DXF itself are `read-external`**, read on `2026-09-30` at
  `https://en.wikipedia.org/wiki/AutoCAD_DXF`: that Autodesk publishes the DXF specification
  *incompletely* ("Autodesk now publishes the incomplete DXF specifications online"), and that some
  object types are undocumented. Both support the position this chapter takes — a writer cannot conform
  its way to certainty in this format, so the oracle is a receiver reading the file back, which is what
  G6 does — and neither upgrades any claim about a standard.
- **Which entities each release defines, and which a given importer accepts, is
  `unverified-with-owner`.** The entity table in §6 states what this product writes and refuses; the
  claim that a receiver accepts it is established only by that receiver, so its owner is `G2-2D` for
  the writer and G6 for the validation, and until then no target is described as compatible with
  anything.
- **The mapping in §3's third column, the registry in §2, the axes in §1 and every rule in §4–§11 are
  project decisions**, recorded in
  `docs/decisions/decision_adr-0004-interchange-dialects.md`. Where a receiver disagrees, the receiver
  is right about itself and the row changes.
- **D6673's revival work is `unverified-with-owner`** with the procurement seat as owner
  ([governance §8.1](../governance.md), held acting): observing a committee needs access this
  repository does not have, and a watch nobody performs is not a control.

## 13. What must be true in tests

- Every registered target exports the reference skirt deterministically: two runs, one byte string
  (G2's exit names one mode; the others follow at G3 and G6).
- Layer content is exactly §3: one meaning per layer, layer 12 absent, layers 84–87 absent, and a
  writer that puts a second kind of geometry on a layer fails.
- The named-layer target's loss report states the separation CUT cannot carry (§3), and an import
  round-trip reports it rather than silently accepting it.
- cut-as-1 and sew-as-1 exports of one design differ only in which line layer 1 carries; the net lines
  of the two are identical, which is the fixture's own obligation ([reference skirt §12](reference-skirt.md)).
- SST and PST are present, case-correct and byte-stable on the ASTM path, and a golden comparison that
  ignores them fails.
- No entity outside §6's written set appears in either release profile, and a curve that cannot meet
  the §7 bound fails the export instead of shipping coarser geometry.
- Each grading mode is a separate artifact and a separate validation; a file carrying two modes does
  not exist.
- The receiver-config record is complete for every artifact: an export with an undeclared unit or an
  unresolved line semantics is refused, not written.
- A PDF whose piece does not fit the page is tiled or refused, and its scale square measures true with
  a ruler at T5.
