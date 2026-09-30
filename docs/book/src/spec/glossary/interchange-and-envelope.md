# Glossary: interchange and the supported envelope

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection. Most format terms are specified by the
> [interchange dialects](../interchange-dialects.md) and [envelope](../feature-matrix.md) chapters.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| `.rul` | the grade-rule table format of the AAMA era: base-size geometry plus point deltas | roadmap §3.3 · specified by `G0-CONTRACT.5` | rule file, grade table, *Gradierungsdatei* | `.rul` |
| AAMA ⚠ | the interchange convention using **named** layers — CUT, DRAW, INTCUT, NOTCH, DRILL, TEXT, REF | roadmap ADR-0004 · [dialects §3](../interchange-dialects.md) | AAMA/ASTM DXF, named-layer DXF | `dxf-aama-named` |
| all-contours-graded | an export mode carrying every size's contour instead of a base plus rules | roadmap ADR-0004 · [dialects §8](../interchange-dialects.md) | fully graded, GradedNest-style | — |
| artifact | a file StitchCAD emits for a receiver: DXF, PLT, PDF, tech pack | roadmap §7.5 | output, export file, deliverable | — |
| ASTM D6673 ⚠ | the **withdrawn** (Jan 2019) specification whose numbered-layer convention cutting rooms still enforce | roadmap ADR-0004 · [dialects §6](../interchange-dialects.md) | ASTM DXF, numbered-layer DXF | `dxf-astm-num-r13` |
| BLOCK (DXF) | a DXF container holding one piece, with no nested inserts, named `[piece]_[size]`; nothing to do with a pattern block | roadmap ADR-0004 · [dialects §5](../interchange-dialects.md) | DXF block, piece block | `piece_block_required` |
| body scan | a three-dimensional measurement of a person; refused as a measurement source in v1 | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | 3D scan, *Bodyscan* | — |
| bulge | the DXF field that carries a circular arc exactly inside a polyline, so an arc is never tessellated | [dialects §6](../interchange-dialects.md) | bulge value, arc-in-polyline | `bulge` |
| CAM driver | software that drives a cutter directly; refused, because this product writes files a factory's own system drives | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | cutter driver, post-processor | — |
| colorway | one colour or print variant of a design; managing a family of them is refused in v1 | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | colourway, color variant, *Farbstellung* | — |
| consumption | how much fabric a layout uses; a marker-making concern, and refused with yield | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | fabric usage, *Verbrauch* | — |
| costing | pricing a garment from its materials and operations; refused, because it is a different product | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | price calculation, *Kalkulation* | — |
| cut-as-1 ⚠ | an export mode whose layer 1 carries the **cut** line, so the knife follows the allowance | roadmap §7.5 · [dialects §4](../interchange-dialects.md) | cut on layer 1, allowance-out | `cut_sew_swap` |
| deferred | a construction the envelope names as coming later, with the gate that owns it | roadmap §3.2 · specified by `G0-CONTRACT.4` | postponed, later release | — |
| de-facto convention | a convention a withdrawn standard left behind: implemented because receivers enforce it, with no conformance claim | [dialects §6](../interchange-dialects.md) | lapsed standard, house convention | — |
| dev shell | the egui-class tool for engine stages, ruled out as a product user interface by ADR-0002 | roadmap ADR-0002 · named by the [envelope](../feature-matrix.md) | engine shell, debug UI | — |
| digitizing | tracing a paper pattern into vectors; refused — an imported contour is data and never a recipe | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | pattern tracing, *Digitalisieren* | — |
| DXF | the drawing-interchange format every cutting room accepts, in several incompatible dialects | roadmap §7.5 · [dialects §2](../interchange-dialects.md) | drawing exchange format | `dxf` |
| DXF layer ⚠ | a numbered or named bucket separating cut, sew, notch, drill, grain and text geometry | roadmap ADR-0004 · [dialects §3](../interchange-dialects.md) | layer, *Ebene* | layer number or name |
| ERP | enterprise resource planning; integration is refused and its administrators are a later persona | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | enterprise system | — |
| entity policy | which DXF entity families a target writes, and which it refuses whatever a design contains | [dialects §6](../interchange-dialects.md) | entity set, written entities | `entity_policy` |
| export target | a named point in the axis space a dialect is made of; a tuple nobody has validated is not one | [dialects §2](../interchange-dialects.md) | dialect target, named mode | — |
| feature matrix | the supported / rejected / deferred table that bounds the v1 release claim | roadmap §3.2 · specified by `G0-CONTRACT.4` | envelope matrix, capability table | — |
| HPGL/PLT | the plotter language, scaled in plotter units, with pens mapped to functions | roadmap §7.5 · [dialects §9](../interchange-dialects.md) | HP-GL, plot file, `.plt` | `plt` |
| interchange dialect ⚠ | one named, separately validated export target — never a claim of a universal package | roadmap ADR-0004 · [dialects §1](../interchange-dialects.md) | export mode, format profile | — |
| loss report | the honest record of an import: what was preserved, approximated, omitted or unsupported | roadmap §7.5 · [dialects §7](../interchange-dialects.md) | import report, fidelity report | — |
| marker | a nested layout of pieces for cutting — an explicit **non-goal** of this product | roadmap §1.3 | nesting layout, cut plan, *Markerbild* | — |
| nesting | packing pieces onto fabric for yield — an explicit **non-goal** of this product | roadmap §1.3 | marker making, yield optimization | — |
| non-goal | a capability the roadmap refuses, so a partner cannot reopen it by asking | roadmap §1.3 | out of scope, refused feature | — |
| pathology corpus | the collection of hostile offset inputs the offset engine must survive | roadmap §4.2 · the corpus is a `G2-2D` gate input | offset corpus, nasty-case set | — |
| PDF | the vector review and print path, tiled when a piece does not fit the page | roadmap §7.5 | print file, plot PDF | `pdf-a0` |
| pen map | the assignment of plotter pens to functions: cut, draw, notch, drill | roadmap §7.5 · [dialects §9](../interchange-dialects.md) | pen table, tool map | `hpgl_pens` |
| PLM | product lifecycle management; integration is refused in v1 and its administrators are a later persona | roadmap §1.2, §1.3 · named by the [envelope](../feature-matrix.md) | product data management | — |
| plotter unit | the HPGL coordinate step: 1016 units per inch, so one unit is exactly 25 µm | [units §8](../units-and-tolerances.md) | plotter step, HPGL unit | — |
| print placement | positioning artwork on a piece; refused, although a stripe reference that constrains placement is supported | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | artwork placement, *Rapportplatzierung* | — |
| PST ⚠ | Piece System Text — the per-piece metadata block the ASTM path requires | roadmap ADR-0004 · [dialects §5](../interchange-dialects.md) | piece system text | `PST` |
| R12 / R13 | AutoCAD DXF releases; many importers accept only these, and only POLYLINE entities | roadmap ADR-0004 · [dialects §6](../interchange-dialects.md) | DXF version, legacy DXF | `dxf_version_target` |
| raster tracing | deriving vector geometry from a scanned image; refused with digitizing | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | image tracing, vectorizing | — |
| registration mark | a printed mark that lets tiled pages be aligned and taped without guessing | roadmap §7.5 | alignment mark, crop mark, *Passermarke* | — |
| receiver-config record | the data that travels with an artifact, so a dispute with a receiver is about fields and not about memory | [dialects §10](../interchange-dialects.md) | receiver record, export config | — |
| rejected ⚠ | a construction outside the envelope, which must produce a diagnostic and never an approximation | roadmap §3.2 · specified by `G0-CONTRACT.4` | unsupported, refused | — |
| scale square ⚠ | a printed square of declared size that proves, with a ruler, that a print is not rescaled | roadmap §7.5 · a `G2-2D` exit check | calibration square, test square | — |
| sew-as-1 ⚠ | an export mode whose layer 1 carries the **sew** line — the swap that rejects patterns | roadmap ADR-0004 · [dialects §4](../interchange-dialects.md) | sew on layer 1, net-out | → `cut_sew_swap` |
| spreading | laying fabric in plies for cutting; refused, because it belongs downstream of the artifact | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | ply laying, *Zuschnitt* | — |
| SST ⚠ | Style System Text — the case-sensitive metadata block on ASTM DXF layer 1 | roadmap ADR-0004 · [dialects §5](../interchange-dialects.md) | style system text | `SST` |
| supported | a construction inside the envelope, backed by a named test and a fixture | roadmap §3.2 · specified by `G0-CONTRACT.4` | in scope, shipped | — |
| supported envelope | the declared family of garments v1 covers, bounding the release claim | roadmap §3.2 · specified by `G0-CONTRACT.4` | product envelope, v1 boundary | — |
| tech pack | the human-authored construction document, generated from canonical structured content | roadmap §7.5 | techpack, specification sheet, *Technikpaket* | — |
| tiled export | splitting one piece across pages, with overlap and ordering, instead of rescaling it | roadmap §7.5 | tiling, A3/A4 tiling, *Kachelung* | — |
| universal package | one artifact claimed to serve every receiver; refused — modes are named and separately validated | roadmap ADR-0004 · named by the [envelope](../feature-matrix.md) | one-file-fits-all, generic export | — |
| yield | fabric utilization from a nesting layout; refused with marker making | roadmap §1.3 · named by the [envelope](../feature-matrix.md) | fabric yield, *Ausnutzungsgrad* | — |
