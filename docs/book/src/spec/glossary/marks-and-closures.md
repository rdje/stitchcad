# Glossary: marks, closures and finishes

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection. Notch types are the canonical example
> (roadmap §7.6), so every one of them carries the mark.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| apex | the point a dart closes to, where its two legs meet; a dart with no apex is a tuck | [ontology §4.3](../ontology.md) | dart point, dart tip, *Abnäherspitze* | `apex` |
| bias ⚠ | the 45° direction to the warp and weft, where woven fabric stretches most | [ontology §4.6](../ontology.md) | true bias, cross-grain, *schräger Fadenlauf* | `bias` |
| button | a closure part stitched to one side, whose size derives the buttonhole length | [ontology §4.7](../ontology.md) | knob, shank button, *Knopf* | → `Closure` |
| buttonhole | the slit a button passes through; its length is derived from the button, never entered twice | [ontology §4.7](../ontology.md) | eyelet (⚠ an eyelet is metal-bound), *Knopfloch* | → `Closure` |
| castle notch ⚠ | a notch shaped like a battlement outline, used where a single slit would not read on the fabric | [ontology §4.5](../ontology.md) | castle, battlement notch | type `castle` |
| check notch ⚠ | a notch on ASTM DXF layer 82 used to verify that a receiver's import scaled correctly | roadmap ADR-0004 · specified by `G0-CONTRACT.10` | validation notch, check mark | type `check` |
| closure | an object recording how a garment fastens: buttons, buttonholes, zips, hooks, with placements | [ontology §4.7](../ontology.md) | fastening, opening, *Verschluss* | `Closure` |
| dart ⚠ | a folded taper that removes fabric — its **intake** — to shape a flat piece to a curved body | [ontology §4.3](../ontology.md) | suppression dart, *Abnäher*, *pince*, *pinza* | `Dart` |
| dart intake ⚠ | the length a dart removes from the edge it starts at; closing the dart must remove exactly that | [ontology §4.3](../ontology.md) | dart width, suppression, *Abnäherinhalt* | `dart_intake` |
| dart leg | one of the two lines a dart folds along; both legs meet at the apex | [ontology §4.3](../ontology.md) | dart side, fold line of a dart, *Schenkel* | — |
| double notch ⚠ | two parallel marks, conventionally the back or the second of a matched pair | [ontology §4.5](../ontology.md) | twin notch, 2-notch | type `double` |
| drill hole ⚠ | a round mark punched inside a piece — a dart apex or a pocket corner, on ASTM DXF layer 13 | [ontology §4.5](../ontology.md) | drill, punch, *Bohrung* | type `drill` |
| facing | a piece that finishes an edge from the inside, cut from the same shape it faces | [ontology §4.7](../ontology.md) | edge facing, *Beleg* | `Facing` |
| gather | fullness absorbed by drawing one edge up to a shorter one, distributed along a span | [ontology §4.3](../ontology.md) | gathering, ruching, *Kräuseln* | `Gather` |
| grain ⚠ | the fabric's weave direction; the warp runs the length of the roll | [ontology §4.6](../ontology.md) | grainline direction, thread, *Fadenlauf* | → `Grainline` |
| grainline ⚠ | the directed line on a piece that must be laid parallel to the fabric's grain | [ontology §4.6](../ontology.md) | grain arrow, straight-of-grain, *Fadenlaufpfeil* | `Grainline` |
| hem | the finished lower edge, turned up by its depth; a hem allowance is not a seam allowance | [ontology §4.7](../ontology.md) | hemming, *Saum* | `Hem` |
| hook and bar | a two-part metal waist closure: a hook on one side, a bar or eye on the other | [reference skirt §9](../reference-skirt.md) | hook and eye, waist hook, *Haken und Öse* | → `Closure` |
| I-notch ⚠ | a single straight slit mark, the commonest production notch | [ontology §4.5](../ontology.md) | I notch, straight notch, slit | type `I` |
| interfacing | a stiffening layer fused or sewn inside a piece; it changes how the piece behaves | [ontology §4.7](../ontology.md) | interlining, fusing, *Einlage* | `Interfacing` |
| lining | a separate inner garment that finishes the inside, cut from its own pieces | [ontology §4.7](../ontology.md) | inner, *Futter* | `Lining` |
| nap ⚠ | a fabric's directional surface — velvet, corduroy — which forces every piece to be laid one way | [ontology §6](../ontology.md) | pile direction, one-way fabric, *Strich* | `nap` |
| notch ⚠ | a semantic matching mark whose physical form is resolved at export by the Factory Profile | [ontology §4.5](../ontology.md) | match mark, nick, *Knips*, *repère* | `Notch` |
| notch depth | how far a notch cuts in, with sample-room and production values where they differ | [ontology §4.5](../ontology.md) | notch size, nick depth | `notch_geometry` |
| notch encoding ⚠ | whether a receiver gets a coded point (position, direction, depth, type) or drawn geometry | [ontology §4.5](../ontology.md) | notch representation, notch mode | `notch_encoding` |
| notch type ⚠ | which shape a notch has; the vocabulary is single, double, V, I, T, U, castle, slit, drill | [ontology §4.5](../ontology.md) | notch shape, notch code | `notch_code` |
| notions | the non-fabric items a garment needs: zips, buttons, thread, elastics, hooks | [reference skirt §9](../reference-skirt.md) | trims, accessories, *Zutaten* | — |
| off-grain | a piece laid at a declared angle to the grain, deliberately and not by accident | [ontology §4.6](../ontology.md) | cross-grain, skewed, *schräg gelegt* | `grain_angle` |
| pleat | fullness folded back on itself and stitched or pressed down, keeping its fold | [ontology §4.3](../ontology.md) | fold pleat, knife pleat, box pleat, *Falte* | `Pleat` |
| pocket | an object with position, orientation, opening type and the pieces it is composed of | [ontology §4.7](../ontology.md) | patch pocket, welt, *Tasche* | `Pocket` |
| single notch ⚠ | one mark, conventionally the front or the first of a matched pair | [ontology §4.5](../ontology.md) | 1-notch, front notch | type `single` |
| slit notch ⚠ | a narrow open slot cut into an edge, distinct from a closed V or a punched drill | [ontology §4.5](../ontology.md) | slit, slot notch | type `slit` |
| stripe reference | a second, fabric-pattern direction a piece must align to, independent of the grain | [ontology §4.6](../ontology.md) | plaid reference, pattern match, *Rapport* | `stripe_ref` |
| T-notch ⚠ | a T-shaped mark on ASTM DXF layer 80, read by cutting rooms as a notch and not as a defect | [ontology §4.5](../ontology.md) | T notch | type `T` |
| tuck | a stitched fold that removes width but has **no apex**, so it stays open at the ends | [ontology §4.3](../ontology.md) | pintuck, *Fältchen* | `Tuck` |
| U-notch ⚠ | a U-shaped mark on ASTM DXF layer 83, used where a V would tear out | [ontology §4.5](../ontology.md) | U notch, round notch | type `U` |
| V-notch ⚠ | a V-shaped mark cut into an edge, on ASTM DXF layer 4 | [ontology §4.5](../ontology.md) | V notch, triangle notch | type `V` |
| zipper | a toothed closure with a declared length and a stop, housed in a seam or a fly | [ontology §4.7](../ontology.md) | zip, *Reißverschluss* | → `Closure` |
