# Glossary: measurements, ease and sizes

> One part of the [glossary](../glossary.md). Rows are alphabetical. The **canonical object** column
> names the chapter and clause that specifies the term normatively — this glossary defines the word,
> it never restates the rule. A ⚠ marks a **safety-relevant** term: one whose mistranslation or
> misreading causes a wrong cut or a factory rejection.

| Term | What it means | Canonical object | Also called | Machine token |
| --- | --- | --- | --- | --- |
| axis (of a size set) | one named body dimension a multi-dimensional size system steps in, with its own ordered values and its own breaks | [size sets §7](../size-sets.md) | dimension, grade axis, *Achse* | `axes` |
| base size | the single size a design is drafted in and that every other size is derived from | [ontology §2.3](../ontology.md) | base, block size, sample size, *Grundschnittgröße* | `base_size` |
| body measurement | a dimension of the **person**, taken between named landmarks by a documented procedure | [ontology §2.1](../ontology.md) | body size, net measure, "the measurement" | kind `body` |
| cumulative grading | a grade-rule table whose deltas are all measured **from the base size** | [instantiation paths §3](../instantiation-paths.md) | absolute grading, "from base" | `cumulative` |
| custom size chart | a size system that is neither EN 13402 nor ASTM D5585 — a factory's or a client's own | [ontology §2.3](../ontology.md) | house chart, bespoke chart, private label chart | `custom` |
| declared zero | a grade delta of zero the recipe states on purpose, which a round trip must not confuse with a point a table never mentioned | [instantiation paths §5](../instantiation-paths.md) | explicit zero, graded-by-nothing | — |
| depth | a vertical distance between two landmarks, e.g. waist-to-hip; not a girth and not a length along a seam | [ontology §2.1](../ontology.md) | drop, rise (on trousers), *Höhe* | — |
| design ease | ease added for style rather than for movement — the flare of an A-line, the volume of a coat | [ontology §2.2](../ontology.md) | style ease, fashion ease | → `Ease` |
| ease | the deliberate difference between a body measurement and the garment measurement at the same place | [ontology §2.2](../ontology.md) | wearing ease, slack, room, *aisance*, *Zugabe* | `Ease` |
| extreme size | the largest or smallest size of a set, always checked after the target system reconstructs it and never only at the base | [instantiation paths §7](../instantiation-paths.md) | end size, size-run edge, *Randgröße* | — |
| fit intent | the named class an ease value belongs to, so a fit can be compared, filtered and validated | [ontology §2.2](../ontology.md) | fit block, silhouette class, "the fit" | `close` · `semi` · `loose` |
| fixed perimeter | a grade-rule attribute that keeps a length constant across sizes instead of grading it | [instantiation paths §3](../instantiation-paths.md) | locked perimeter, non-graded edge | — |
| girth | a **circumference** taken around the body or the garment, not a flat width | [ontology §2.1](../ontology.md) | circumference, "the measure around", *Umfang* | `*_girth` |
| grade point | a point of a piece that a grade rule moves, addressed by identity and never by index | [ontology §1](../ontology.md) | grading point, nest point, rule point | `grade_point` |
| grade rule | a per-point X/Y delta that produces another size from the base size | roadmap §3.3 · specified by `G0-CONTRACT.5` | grading rule, nest rule, *Gradierung* | `grade_rule` |
| incremental grading | a grade-rule table whose deltas are measured **from the previous size** | [instantiation paths §3](../instantiation-paths.md) | relative grading, "size to size" | `incremental` |
| landmark ⚠ | the anatomical or garment point a measurement starts or ends at; without it the number is not repeatable | [ontology §2.1](../ontology.md) | reference point, anatomical point, measuring point | `landmark` |
| made-to-measure (MTM) | production from one person's measurements, which is what regeneration is for | roadmap §3.3 | bespoke, individual cut, custom fit | — |
| measurement | one named scalar with a unit, a landmark, a procedure and an uncertainty state | [ontology §2.1](../ontology.md) | dimension, value, "the number" | — |
| measurement table | the named set of scalars a design drafts from; the recipe's only body-shaped input | [ontology §2.1](../ontology.md) | measurement chart, spec sheet, measure sheet | `MeasurementTable` |
| negative ease | ease below zero: the garment is smaller than the body and must stretch onto it | [ontology §2.2](../ontology.md) | compression, stretch-to-fit | → `Ease` |
| order object | the not-yet-modelled home of size-run quantities and delivery data, which a reusable design must not carry | roadmap §7.5 · [size sets §1](../size-sets.md) | order, size run sheet, cutting order | — |
| point of measure (POM) ⚠ | a dimension **of the garment**, measured on the finished piece at a declared place | [ontology §2.1](../ontology.md) | POM, spec measurement, finished measure, *Maßstelle* | kind `garment` |
| procedure | the documented way a measurement is taken — tape position, posture, tension, which side | [ontology §2.1](../ontology.md) | measuring method, "how to measure", *Messanleitung* | `procedure` |
| ready-to-wear (RTW) | production in a graded size range, which is what grade-rule instantiation is for | roadmap §3.3 | off-the-rack, confectie, *Konfektion* | — |
| reconstruction | rebuilding the geometry between moved grade points — what a receiver's own system does with a `.rul` | [instantiation paths §3](../instantiation-paths.md) | regeneration (⚠ a different thing), rebuild | — |
| resolved size set | the size set a profile override produced: a new object naming the design's reference, the transformation and its evidence | [size sets §9](../size-sets.md) | effective size set, factory size set | — |
| shrinkage | a material's dimensional change after a declared treatment, applied as an explicit transformation | [ontology §6](../ontology.md) | contraction, fabric loss, *Einlaufwert* | `shrinkage_xy_pct` |
| size break | the interval between two adjacent sizes in one dimension — the "2 inch jump" of a grade | [instantiation paths §3](../instantiation-paths.md) | grade interval, jump, increment | `size_break` |
| size chart | a table of POMs across the sizes of a size set; the artifact a factory checks the pattern against | [ontology §2.3](../ontology.md) | spec chart, POM sheet, measurement grid | `chart` |
| size label ⚠ | the **name** of a size ("12", "M", "170/88A"); it is not its position in the range | [ontology §2.3](../ontology.md) | size name, size code, *Größe* | `size_label` |
| size order ⚠ | the **sequence** sizes are instantiated in, which a label does not determine | [ontology §2.3](../ontology.md) | size run order, grade order, sequence | `size_order` |
| size set | the sizes a design is instantiated in: labels, their order, the base size, and the size system | [ontology §2.3](../ontology.md) | size range, size run, "the ratio" | `SizeSet` |
| size system | which designation system a set's labels come from — and never a source of measurements by itself | [size sets §8](../size-sets.md) | size designation, sizing standard, *Größensystem* | `size_system` |
| size-set transformation | the typed change a Factory Profile applies to a design's size set: label mapping, break scaling, base substitution or chart replacement | [size sets §9](../size-sets.md) | size override, grade override, *Größenanpassung* | `transformation` |
| smoothing | a grade-rule attribute that fairings a graded contour instead of moving each point independently | [instantiation paths §3](../instantiation-paths.md) | fairing, curve smoothing, *Ausgleich* | — |
| stack point | a graded point treated as the anchor of its rule table, so sizes stack rather than drift | [instantiation paths §3](../instantiation-paths.md) | anchor point, fixed point, *Aufspringpunkt* | — |
| wearing ease | the minimum ease a garment needs to be wearable at all, before style is considered | [ontology §2.2](../ontology.md) | movement ease, minimum ease, *Bewegungsweite* | → `Ease` |
| width (flat) ⚠ | a distance across a garment laid flat — half a girth. Confusing the two halves a pattern | [ontology §2.1](../ontology.md) | flat measure, half girth, "across" | `*_width` |
