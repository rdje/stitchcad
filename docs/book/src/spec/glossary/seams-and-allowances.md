# Glossary: seams, sewing and allowances

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| allowance width | the distance a seam allowance stands off its net line, per edge — never one global number | [ontology §4.4](../ontology.md) | SA, seam allowance value, *Nahtzugabe* | `sa_*` |
| armscye | the armhole: the body-side edge a sleeve is set into | roadmap §3.1 · exercised at `G3-GRADING` | armhole, sleeve hole, *Armloch* | — |
| assembly order | the sequence the sewing graph's spans are joined in, which 3D assembly also reads | [ontology §4.2](../ontology.md) | sewing order, construction sequence, *Nähreihenfolge* | → `SewingGraph` |
| balance notch ⚠ | a notch planted by `walk` so two edges of different length meet at a declared place, not by luck | [ontology §3.2](../ontology.md) | match notch, alignment notch, *Passzeichen* | → `Notch` |
| cap ease | the declared differential between a sleeve cap and its armscye — intentional, never an invariant violation | roadmap §11 G3 · [ontology §4.2](../ontology.md) | sleeve ease, cap fullness, *Kugelweite* | — |
| corner treatment | how two allowances of any width are joined at a corner | [ontology §4.4](../ontology.md) | corner join, corner finish, *Eckenausführung* | `corner` |
| differential | the length difference between the two sides of a seam span | [ontology §4.2](../ontology.md) | delta, length mismatch, "the gap" | — |
| ease distribution | how a declared differential is spread along a span: uniform, weighted, or anchored between notches | [ontology §4.2](../ontology.md) | ease allocation, gathering plan, *Weitenverteilung* | `ease_distribution` |
| envelope (corner) | a corner treatment that wraps one allowance around the other — the hem-and-side-seam case | [ontology §4.4](../ontology.md) | wrap corner, folded corner, *Umschlag* | `envelope` |
| inclusion policy ⚠ | whether an allowance is stored inside the contour or generated downstream, resolved per Factory Profile | [ontology §4.4](../ontology.md) | SA policy, net-or-cut export, allowance ownership | `seam_allowance_policy` |
| miter (corner) | a corner treatment that joins two allowances at their bisector, so both edges meet square | [ontology §4.4](../ontology.md) | mitre, Gehrungsecke | `miter` |
| offset | the geometric operation that produces an allowance: a contour pushed out by a width | [units §6](../units-and-tolerances.md) | parallel curve, contour offset, *Versatz* | `offset` |
| one-to-many span | a seam span whose one side is sewn to several pieces, or several ranges to one | [ontology §4.2](../ontology.md) | multi-span, compound seam | → `SeamSpan` |
| partial span | a seam span covering a sub-range of an edge rather than the whole edge | [ontology §4.2](../ontology.md) | sub-seam, span segment | → `SeamSpan` |
| seam | the joint where two piece edges are stitched; in StitchCAD always a `SeamSpan`, never drawn lines | [ontology §4.2](../ontology.md) | join, stitch line, *Naht* | → `SeamSpan` |
| seam allowance ⚠ | the strip of fabric beyond the net line that a seam consumes when it is stitched | [ontology §4.4](../ontology.md) | SA, turnings, seam turn, *Nahtzugabe*, *rentré* | `SeamAllowance` |
| seam length | the length of a seam span along its own edge, measured on the net line | [ontology §4.2](../ontology.md) | sewn length, stitch length (⚠ not SPI) | — |
| seam span | one oriented correspondence between two edge ranges, with its ease and its stop landmarks | [ontology §4.2](../ontology.md) | span, seam correspondence, *Nahtzuordnung* | `SeamSpan` |
| self-intersection | an offset contour that crosses itself — a cusp or a too-tight inside corner, never silently emitted | [units §6](../units-and-tolerances.md) | loop, inversion, *Selbstüberschneidung* | — |
| sewing graph | the first-class object holding every seam span; it lives in the design, not in a renderer | [ontology §4.2](../ontology.md) | assembly graph, stitch graph, *Nähgraph* | `SewingGraph` |
| slant (corner) | a corner treatment that bevels one allowance across to the other's direction | [ontology §4.4](../ontology.md) | bevel corner, angled corner, *schräge Ecke* | `slant` |
| sleeve cap | the curved top edge of a sleeve, longer than the armscye it is set into by the cap ease | roadmap §3.1 · exercised at `G3-GRADING` | cap, sleeve head, *Kugel* | — |
| step (corner) | a corner treatment that leaves two allowance ends square and apart — a hem step | [ontology §4.4](../ontology.md) | square corner, offset corner, *Stufe* | `step` |
| stitches per inch (SPI) | a seam's stitch density: a tech-pack field, and not a seam length however much it looks like one | roadmap §7.5 · specified by `G0-CONTRACT.12` | SPI, stitch density, *Stiche pro cm* | `spi` |
| stop landmark ⚠ | the notch or turn point where sewing stops or changes direction; a zipper stop is one | [ontology §4.2](../ontology.md) | stop point, sewing stop, backtack point | `stop` |
| trim (corner) | a corner treatment that cuts one allowance back so the corner lies flat when turned | [ontology §4.4](../ontology.md) | clipped corner, reduced corner, *beschneiden* | `trim` |
| true (operation) ⚠ | adjust a seam so its two sides match within tolerance, attributing what is left to declared ease | [ontology §3.2](../ontology.md) | truing, trueing, "make it match" | `true` |
| turn point | a point where a cut or sew path changes direction sharply; its own layer in ASTM DXF | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | corner point, direction change | layer `2` |
| walk (operation) ⚠ | traverse a seam correspondence reporting the differential per span and planting balance notches | [ontology §3.2](../ontology.md) | walking the seam, seam walking, *Abrollen* | `walk` |
