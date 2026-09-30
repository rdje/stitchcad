# Glossary: interchange and the supported envelope

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection. Most format terms are specified by
> `G0-CONTRACT.10` (interchange dialects) and `G0-CONTRACT.4` (the feature matrix).

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| AAMA ⚠ | the interchange convention using **named** layers — CUT, DRAW, INTCUT, NOTCH, DRILL, TEXT, REF | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | AAMA/ASTM DXF, named-layer DXF | `dxf-aama-named` |
| all-contours-graded | an export mode carrying every size's contour instead of a base plus rules | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | fully graded, GradedNest-style | — |
| artifact | a file StitchCAD emits for a receiver: DXF, PLT, PDF, tech pack | roadmap §7.5 | output, export file, deliverable | — |
| ASTM D6673 ⚠ | the **withdrawn** (Jan 2019) specification whose numbered-layer convention cutting rooms still enforce | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | ASTM DXF, numbered-layer DXF | `dxf-astm-num-r13` |
| BLOCK (DXF) | a DXF container holding one piece, with no nested inserts, named `[piece]_[size]`; nothing to do with a pattern block | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | DXF block, piece block | `piece_block_required` |
| cut-as-1 ⚠ | an export mode whose layer 1 carries the **cut** line, so the knife follows the allowance | roadmap §7.5 · specified by `G0-CONTRACT.10` | cut on layer 1, allowance-out | `cut_sew_swap` |
| deferred | a construction the envelope names as coming later, with the gate that owns it | roadmap §3.2 · specified by `G0-CONTRACT.4` | postponed, later release | — |
| DXF | the drawing-interchange format every cutting room accepts, in several incompatible dialects | roadmap §7.5 · specified by `G0-CONTRACT.10` | drawing exchange format | `dxf` |
| DXF layer ⚠ | a numbered or named bucket separating cut, sew, notch, drill, grain and text geometry | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | layer, *Ebene* | layer number or name |
| feature matrix | the supported / rejected / deferred table that bounds the v1 release claim | roadmap §3.2 · specified by `G0-CONTRACT.4` | envelope matrix, capability table | — |
| HPGL/PLT | the plotter language, scaled in plotter units, with pens mapped to functions | roadmap §7.5 · specified by `G0-CONTRACT.10` | HP-GL, plot file, `.plt` | `plt` |
| interchange dialect ⚠ | one named, separately validated export target — never a claim of a universal package | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | export mode, format profile | — |
| loss report | the honest record of an import: what was preserved, approximated, omitted or unsupported | roadmap §7.5 · specified by `G0-CONTRACT.10` | import report, fidelity report | — |
| marker | a nested layout of pieces for cutting — an explicit **non-goal** of this product | roadmap §1.3 | nesting layout, cut plan, *Markerbild* | — |
| nesting | packing pieces onto fabric for yield — an explicit **non-goal** of this product | roadmap §1.3 | marker making, yield optimization | — |
| non-goal | a capability the roadmap refuses, so a partner cannot reopen it by asking | roadmap §1.3 | out of scope, refused feature | — |
| pathology corpus | the collection of hostile offset inputs the offset engine must survive | roadmap §4.2 · the corpus is a `G2-2D` gate input | offset corpus, nasty-case set | — |
| PDF | the vector review and print path, tiled when a piece does not fit the page | roadmap §7.5 | print file, plot PDF | `pdf-a0` |
| pen map | the assignment of plotter pens to functions: cut, draw, notch, drill | roadmap §7.5 · specified by `G0-CONTRACT.10` | pen table, tool map | `hpgl_pens` |
| plotter unit | the HPGL coordinate step: 1016 units per inch, so one unit is exactly 25 µm | [units §8](../units-and-tolerances.md) | plotter step, HPGL unit | — |
| PST ⚠ | Piece System Text — the per-piece metadata block the ASTM path requires | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | piece system text | `PST` |
| R12 / R13 | AutoCAD DXF releases; many importers accept only these, and only POLYLINE entities | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | DXF version, legacy DXF | `dxf_version_target` |
| registration mark | a printed mark that lets tiled pages be aligned and taped without guessing | roadmap §7.5 | alignment mark, crop mark, *Passermarke* | — |
| rejected ⚠ | a construction outside the envelope, which must produce a diagnostic and never an approximation | roadmap §3.2 · specified by `G0-CONTRACT.4` | unsupported, refused | — |
| `.rul` | the grade-rule table format of the AAMA era: base-size geometry plus point deltas | roadmap §3.3 · specified by `G0-CONTRACT.5` | rule file, grade table, *Gradierungsdatei* | `.rul` |
| scale square ⚠ | a printed square of declared size that proves, with a ruler, that a print is not rescaled | roadmap §7.5 · a `G2-2D` exit check | calibration square, test square | — |
| sew-as-1 ⚠ | an export mode whose layer 1 carries the **sew** line — the swap that rejects patterns | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | sew on layer 1, net-out | → `cut_sew_swap` |
| SST ⚠ | Style System Text — the case-sensitive metadata block on ASTM DXF layer 1 | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | style system text | `SST` |
| supported | a construction inside the envelope, backed by a named test and a fixture | roadmap §3.2 · specified by `G0-CONTRACT.4` | in scope, shipped | — |
| supported envelope | the declared family of garments v1 covers, bounding the release claim | roadmap §3.2 · specified by `G0-CONTRACT.4` | product envelope, v1 boundary | — |
| tech pack | the human-authored construction document, generated from canonical structured content | roadmap §7.5 | techpack, specification sheet, *Technikpaket* | — |
| tiled export | splitting one piece across pages, with overlap and ordering, instead of rescaling it | roadmap §7.5 | tiling, A3/A4 tiling, *Kachelung* | — |
