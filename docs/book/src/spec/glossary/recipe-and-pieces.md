# Glossary: the recipe and the piece

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| A-line | a silhouette that widens steadily from waist or chest to hem, without a shaped hip curve | roadmap §3.2 · exercised by the [reference skirt](../reference-skirt.md) | flare skirt, "A" shape, *A-Linie* | — |
| bespoke structure | a garment with no construction recipe behind it, so there is nothing parametric to re-evaluate | roadmap §3.2 · named by the [envelope](../feature-matrix.md) | freeform, one-off draft, *Einzelanfertigung* | — |
| block (pattern) | a fitted base pattern, with no style, from which designs are developed by the recipe; nothing to do with a DXF `BLOCK` | roadmap ADR-0003 · the reference set is chosen by `G0-CONTRACT.9` | sloper, master pattern, foundation, *Grundschnitt* | — |
| bodice | the upper body section of a garment, shoulder to waist, shaped by darts or seams | roadmap §3.2 · proved at `G3-GRADING` | body, top block, corsage, *Leibchen* | — |
| bounding box | the smallest axis-aligned rectangle containing a piece; used against fabric width and page size | [units §1.1](../units-and-tolerances.md) | extents, "the piece's size", envelope | — |
| centre back (CB) ⚠ | the vertical centre line of the back; a seam edge here means the back is cut in two pieces | [reference skirt §5](../reference-skirt.md) | CB, back middle, *hintere Mitte* | `cb` |
| centre front (CF) ⚠ | the vertical centre line of the front; a fold edge here means the front is cut in one piece | [reference skirt §5](../reference-skirt.md) | CF, front middle, mid front, *vordere Mitte* | `cf` |
| classic collar | a collar with a stand and a fall that rolls at a declared line; the envelope names it and no gate proves it | [envelope §9](../feature-matrix.md) | shirt collar, two-piece collar, *Kragen* | — |
| collar fall | the part of a collar that folds over the stand and shows; its length sets the collar's roll | [envelope §9](../feature-matrix.md) | collar leaf, *Oberkragen* | — |
| collar stand | the band a collar's fall folds over, which decides how far the collar stands from the neck | [envelope §9](../feature-matrix.md) | collar band, *Steg*, *Unterlagen* | — |
| construction recipe | the ordered formula graph and drafting operations that *produce* a garment's geometry | [ontology §3.1](../ontology.md) | recipe, drafting history, construction, "the how" | → `Design` |
| cut line ⚠ | the boundary the knife follows: the net line pushed out by its seam allowance | [ontology §4.4](../ontology.md) | cutting line, contour, outer edge, *Schnittkante* | layer `CUT` · `1` |
| cut on fold ⚠ | a piece placed against a fabric fold so one cut yields a symmetric piece twice as wide | [ontology §4.1](../ontology.md) | place on fold, "cut 1 on fold", folded cut | `fold_edge` |
| cut quantity | how many of a piece are cut, in this size, from this material | [ontology §4.1](../ontology.md) | multiplicity, cut count, "cut 2" | `cut_qty` |
| design | the authored semantic content: measurements, ease, recipe, sewing graph, materials | [ontology §3.1](../ontology.md) | style, project, garment, pattern (⚠ ambiguous) | `Design` |
| drafting operation | one typed, ordered, replayable step of the recipe; every mutation goes through the command bus | [ontology §3.2](../ontology.md) | step, edit, command, action | typed command name |
| extension | the part of a waistband or fly that overlaps past the closure to carry it | [reference skirt §3](../reference-skirt.md) | overlap, underlap, fly extension | `wb_extension` |
| face side ⚠ | the side of the fabric that shows; a piece is stored with the face declared, never assumed | [ontology §4.1](../ontology.md) | right side, RS, good side, *rechte Warenseite* | `face_up` |
| formula graph | the acyclic graph of expressions over measurements, parameters and prior geometry | [ontology §3.1](../ontology.md) | dependency graph, expression graph, "the formulas" | `formula_graph` |
| hole | a closed internal loop that is cut out of a piece, oriented opposite to the outer boundary | [ontology §4.1](../ontology.md) | internal cutout, void, opening | `hole` |
| imported geometry | geometry with no history, stored as explicit primitives and always distinguishable from drafted | [ontology §3.1](../ontology.md) | dead geometry, flat import, "dumb" pattern | origin `imported` |
| instance | the sized result of evaluating a recipe: pieces, sewing graph, closures, labels and the provenance of how it was made | [instantiation paths §2](../instantiation-paths.md) | graded instance, size instance, *Instanz* | `Instance` |
| internal construction line | non-cutting geometry inside a piece: dart legs, fold lines, placement lines, guidelines | [ontology §4.1](../ontology.md) | internal line, guideline, construction line, *Hilfslinie* | layer `INTCUT` · `8` |
| knit or stretch block | a block drafted for a fabric that stretches, where the meaning of ease changes; refused in v1 | roadmap §3.2 · named by the [envelope](../feature-matrix.md) | jersey block, stretch sloper, *Strickgrundschnitt* | — |
| label data ⚠ | the text printed on a piece, complete without consulting anything else | [ontology §4.1](../ontology.md) | piece annotation, marker label, piece ticket | `label` |
| layer index | the ordering of pieces used by 3D assembly; not a cutting ply and not a DXF layer | [ontology §4.1](../ontology.md) | stacking order, assembly layer, z-order | `layer` |
| leather | a non-textile sheet material, whose allowance and offset behaviour the woven model does not describe | roadmap §3.2 · named by the [envelope](../feature-matrix.md) | hide, skin, *Leder* | — |
| material | the fabric or notion a piece is cut from, with width, nap, pattern and shrinkage | [ontology §6](../ontology.md) | fabric, cloth, substrate, *Stoff* | `Material` |
| mirror | reflect geometry about an axis, producing the left of a right-handed piece | [ontology §3.2](../ontology.md) | flip, reflect, *spiegeln* | `mirror` |
| mirrored pair ⚠ | two pieces that are mirror images of each other, labelled L and R and cut once each | [ontology §4.1](../ontology.md) | pair, L/R pair, left and right | `pair` |
| net line ⚠ | the stitched line: the garment's sewn boundary with **no** allowance on it | [ontology §4.4](../ontology.md) | sew line, seam line, stitching line, *Nahtlinie* | layer `DRAW` · `14` |
| outer boundary | the closed loop that is a piece's extent, wound counter-clockwise in the piece's own frame | [ontology §4.1](../ontology.md) | contour, perimeter, outline, *Kontur* | `boundary` |
| parameter | a named value exposed to formulas, carrying an uncertainty state like any other number | [ontology §3.1](../ontology.md) | variable, driver, input | `parameter` |
| pattern ⚠ | ambiguous in English: the **design**, one **piece**, or a **marker**. StitchCAD never uses it alone | — (deliberately undefined) | Schnitt (also ambiguous), pattern piece | — |
| piece | one pattern shape cut from one material, with identity, label data and geometry | [ontology §4.1](../ontology.md) | pattern piece, part, panel, *Schnittteil* | `Piece` |
| quadrant | one fourth of a symmetric garment — the unit the reference drafting works in | [reference skirt §3](../reference-skirt.md) | quarter, "1/4 of the body" | `quarter` |
| recipe replay | re-evaluating the recipe from scratch; the only way geometry is produced or reproduced | [ontology §3.1](../ontology.md) | regeneration, re-evaluation, rebuild | `EvaluateInstance` |
| reference drafting | the one named drafting system shipped as the v1 reference block set | roadmap ADR-0003 · chosen by `G0-CONTRACT.9` | reference block set, house drafting system | — |
| revision ⚠ | the design's monotonically increasing counter; a command's precondition and an approval bind to it | [ontology §3.1](../ontology.md) | version, edit level, *Stand* | `revision` |
| roll line | the line a collar, lapel or hem folds along; a construction line and never a cut edge | [envelope §9](../feature-matrix.md) | crease line, fold-over line, *Bruchkante* | — |
| shirt | a bodice with a front opening, a collar and a sleeve — the intermediate the roadmap permits at G3 | roadmap §11 G3 · permitted, not promised | top, *Hemd* | — |
| slash and spread | cut a shape along a line and open it to add fullness — or close it to remove fullness | [ontology §3.2](../ontology.md) | cut and spread, slash and overlap, fullness | `slash_spread` |
| trousers | a two-legged garment; inside the declared envelope and outside every gate's exit criteria | [envelope §9](../feature-matrix.md) | pants, slacks, *Hose* | — |
| waistband | the band that finishes a waist, cut separately or as an extension of the body | [reference skirt §5](../reference-skirt.md) | band, waist facing, *Bund* | `waistband` |
| winding ⚠ | the direction a boundary loop runs; outer boundaries are CCW, holes are CW | [ontology §4.1](../ontology.md) | orientation, contour direction, *Umlaufsinn* | `winding` |
| wrong side ⚠ | the side of the fabric that does not show; a piece cut face-down is a mirrored piece | [ontology §4.1](../ontology.md) | WS, back of fabric, *linke Warenseite* | → `face_up` |
