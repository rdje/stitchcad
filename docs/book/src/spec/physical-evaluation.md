# Physical evaluation

Everything else in this book is checked by software, and no software can say whether a pattern fits a body or a printer draws true. So this chapter is what to do with real paper and cloth, written so that whoever does it, with no sewing credential and no special knowledge of this project, produces a record the project can use without interpreting it.

**What this is.** Two protocols: a printed square, which takes five minutes and closes [G2's last clause](../availability.md), and a first toile of the reference skirt, which takes an afternoon. **Neither has been performed.** Every number below is what to measure, and none is a result.

## The printed square

Software has measured [the PDF](pdf-output.md) in pixels. What is left is the printer.

1. Make the file: `bash docs/tasks/artifacts/printed_square/run_printed_square.sh`. It writes `target/printed_square/project/exports/reference-skirt-a4.pdf`.
2. Print its first page on A4 at **100%**, which a print dialog may call "actual size". Turn off "fit to page" and "shrink oversized pages", because either one rescales it, and the page will still look right.
3. With a ruler that is known to be true, measure the labelled square across and up, between the centres of its lines. It should be 100 mm on each side.
4. Send one scan or photo of the square beside the ruler, the two measurements, the printer's model, and the settings used.

The result goes in `conformance/printed-square/paper-measurement.tsv`. **The project does not decide what is close enough.** The units chapter says T5 belongs to whoever receives the artifact, and no factory profile exists yet. So the record states what was measured, and the person who measured it says whether they accept it. If the square is off, say by how much: that is a finding about the printer or the settings, and if it is the same on every printer, it is a defect here.

## The first toile

The ruling of 2026-09-30 is that with no domain expert available, what the project lacks is **a measurement, not a credential**. A toile is a garment made in cheap cloth to test its fit. Anyone with a sewing machine and an afternoon can make this one. Sewing it well is not the point, and measuring it honestly is.

| You need | Why |
| --- | --- |
| The PDF, printed at 100% and taped, or plotted on A0 | The pattern at true scale. Check the square first |
| About 1.5 m of plain medium-weight woven cotton, such as calico, pressed, and its width noted | It behaves like the fixture's woven base, and it shows marks |
| Fusible woven interfacing, about 80 × 5 cm | For the band |
| A zipper of at least 18 cm, and a hook and bar | For the back and the band |
| Scissors, pins, chalk, a sewing machine, an iron and a flexible tape measure | To make and measure it |
| A person whose body measures about 74 cm waist and 98 cm hip, or a dress form of that size | The fixture's measurements. Record what yours actually are |

**In this order:**

1. Measure the person or form at waist, hip, waist to hip and waist to hem, and record them. A difference from §2 of [the skirt's chapter](reference-skirt.md) is not a failure, and it changes how the results are read.
2. Cut each piece on the cut line, on the grain, in the quantity its label says. Transfer the notches, darts and fold lines.
3. Assemble in §8's order: fuse the interfacing, then darts, side seams, CB with the zipper, the band to the waist, then the hem. Sew on the dashed line.
4. Measure the flat garment, then have it worn, then measure and photograph it from the front, side and back.
5. Fill the record. If something cannot be measured, say so, and do not estimate it.

## What each measurement can prove wrong

Each constant the chapter marks as an assumption has a measurement that would falsify it. Say what you see, even where the row does not expect it.

| Constant | The fixture says | Measure | What would show it is wrong |
| --- | --- | --- | --- |
| `ss_suppress` | 3.0 cm taken at each side seam | Worn: the side seam from waist to hip, and any drag lines or gap there | The seam pulls toward the front or back, or stands away from the body, so the side is taking too much or too little |
| `dart_intake` | 4.0 cm in each dart | Flat: the waist edge of each panel with its dart closed. Worn: any pucker at the dart's tip, or gap at the waist | The closed edge is not 18.5 cm, or the tip dimples because too much is being taken at once |
| `dart_intake_max` | 5.0 cm as the most one dart may take | Worn: the tip of each dart, in good light | A bubble at the tip that pressing does not remove |
| `dart_len_front` | 12.0 cm | Worn: the front dart's tip against the fullest part of the stomach or hip | The tip stops short of the shape it should release, or passes it |
| `dart_len_back` | 14.0 cm | Worn: the back dart's tip against the fullest part of the seat | The same, at the back |
| `a_line_flare` | 3.0 cm at each side, hip to hem | Worn: the side seam from hip to hem, and the hem's width | The side seam swings forward or back, or the skirt cannot take a stride |
| `wb_width` | 4.0 cm finished | Worn: the band sitting on the waist, seated and standing | It rolls or buckles when seated, which the sources disagree about at this height |

Two things a reader should also report, because the chapter holds them as an engineer's choice: the waist-to-hip curve, whether the side seam lies smooth over the hip, and the corner shapes, whether each allowance folds and sews without a lump.

## The record

Each measurement is one row of `conformance/reference-skirt/measurements.tsv`, whose columns are below, and a photo where one is named goes beside it.

| Column | What it holds |
| --- | --- |
| `constant` | A name from the table above, or `garment` for anything else |
| `measured` | What was measured, in that row's words |
| `value` | The number and its unit, or what was seen, in plain words |
| `verdict` | One of `holds`, `disagrees`, `could_not_measure` |
| `by` | Who measured it, and what they measured with |
| `evidence` | The photo's file name, or nothing |
| `date` | When, as a full date |

A test reads this file whenever it has rows, and it refuses a constant it does not know, a verdict outside the list, or a row with no one named. An empty file means nothing has been measured, which is true today.

**A measurement that disagrees is never discarded, and the drafting is never defended against it.** The constant is corrected in the fixture, every value is derived again, the four closures in §4.1 are checked again, the goldens are refrozen, and the row stays as the reason. A constant whose row holds moves from `assumed` to `known`, with that row as its evidence. Only the person who measured it decides what is close enough, as with the square.

## Verification status

Every constant in the table above is `assumed`, pending a reviewer: the domain seat in [governance](../governance.md) is **vacant**, so this is waiting on a measurement by whoever performs this protocol, or on a domain expert. This protocol itself is an engineer's, and no one who sews has reviewed it. Parts of it may turn out to be the wrong thing to ask, and whoever performs it should say so.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `bash docs/tasks/artifacts/printed_square/run_printed_square.sh` | Makes the files, and measures them on screen |
| `target/printed_square/project/exports/reference-skirt-a4.pdf` | The file to print |
| `conformance/printed-square/paper-measurement.tsv` | Where the printed square's result goes |
| `conformance/reference-skirt/measurements.tsv` | Where the toile's results go |
| `ss_suppress` | What each side seam takes out, §3 |
| `dart_intake` | What each dart takes out, §3 |
| `dart_intake_max` | The most one dart may take, from [the formula language](formula-language.md) |
| `dart_len_front` | The front dart's length, §3 |
| `dart_len_back` | The back dart's length, §3 |
| `a_line_flare` | The flare from hip to hem, §3 |
| `wb_width` | The finished band's height, §3 |
| `garment` | A row about the garment and no one constant |
| `holds` | The measurement agrees |
| `disagrees` | The measurement does not agree |
| `could_not_measure` | It could not be measured, and why goes in the value |
| `assumed` | A value nobody has confirmed: the ontology's state |
| `known` | A value with evidence: the ontology's state |
| `constant` | A column of the record |
| `measured` | A column of the record |
| `value` | A column of the record |
| `verdict` | A column of the record |
| `by` | A column of the record |
| `evidence` | A column of the record |
| `date` | A column of the record |
