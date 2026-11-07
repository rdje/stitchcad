# PDF output

`write_pdf` writes a drawn pattern at true scale for review and printing, on one A0 sheet or tiled across A3 or A4 sheets. Every file has a printed scale square of 100 mm, to check with a ruler. **A drawing is never
rescaled.** A sheet may turn on its side, which changes nothing about scale, and the record says so. A drawing that fits neither way is refused, and tiling it is the way out, because a rescaled print is a pattern nobody drafted, and it would still look right.

**What works today.** The writer, tiling, its record, a reader in the tests, and measurements by two renderers this project did not write. **What does not exist yet:** a print anyone has measured. That is [the printed-square check](interchange-dialects.md), G2's one step that needs a person with a ruler.

```rust
use sc_artifacts::{write_pdf, DrawnPiece, Line, PageLayout, Pattern, PdfError, Sheet};
use sc_geometry::Point;

let p = |x, y| Point::new(x, y).expect("in range");
let piece = |width: i64| DrawnPiece {
    name: "front".to_owned(),
    size: "M".to_owned(),
    quantity: 1,
    cut: vec![p(0, 0), p(width, 0), p(width, 300_000), p(0, 300_000)],
    sew: vec![p(10_000, 10_000), p(width - 10_000, 10_000), p(width - 10_000, 290_000), p(10_000, 290_000)],
    grain: Line { from: p(width / 2, 20_000), to: p(width / 2, 280_000) },
    internal: vec![],
    marks: vec![],
    labels: vec![],
};
let pattern = |pieces| Pattern { style: "skirt".to_owned(), design: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_owned(), revision: "1".to_owned(), pieces };

let fits = write_pdf(&pattern(vec![piece(200_000)]), PageLayout::Single(Sheet::A0)).expect("fits");
assert_eq!(fits.sheets, 1);

// Two metres does not fit A0. It is refused rather than shrunk, and tiling keeps it at true scale.
let long = pattern(vec![piece(2_000_000)]);
assert!(matches!(write_pdf(&long, PageLayout::Single(Sheet::A0)), Err(PdfError::DoesNotFit { .. })));
let tiled = write_pdf(&long, PageLayout::Tiled { sheet: Sheet::A4, overlap: 10_000 }).expect("tiles");
assert!(tiled.sheets > 5);
```

## What is on the paper

- **The scale square**, 100 mm on each side, labelled, at the drawing's corner. On tiled sheets it is on the first sheet.
- **Each piece**, in name order: the cut boundary in a heavier line, the sew line dashed, the grainline, internal lines, notches and drill marks, with its name, size and cut quantity.
- **On tiled sheets:** registration marks at the printable corners, the sheet's number and its row and column, and the overlap you asked for, so neighbours can be aligned and taped.

## Precision

Coordinates are written in points to four decimal places, about 0.035 micrometres, computed from whole micrometres without floating point. The record publishes that. A printer has its own error, far larger, which is what the scale square exists to catch.

## The file holds no date

There is no creation date, producer or file identifier, so two runs give one byte string. The record holds the target, the sheet, its orientation, the sheet count, the overlap, the drawing's size, the scale square, the Design and its revision, and the approval state.

## Why this writes its own PDF

Roadmap §7.5 asks to evaluate `pdf-writer` and typst-class libraries over `printpdf` and to record the choice with its measurements. **That evaluation was not measured.** The harness's safety check refused the build that would have fetched them, and that refusal was not worked around.

So this writes PDF directly, in about 300 lines with no dependency, because what it needs is small: lines, one built-in font, pages and a cross-reference table. It has two known costs: text uses Helvetica, whose accents are not supported, so a non-ASCII label prints as `?`; and no third-party PDF library has been compared with it. If a label needs more, this choice should be revisited.

## How it is checked

Run `cargo test -p sc-artifacts --test pdf`. A reader in the test, sharing no code with the writer, checks the cross-reference table points at each object exactly and each stream's stated length, then measures the scale square and a piece out of the file. It requires two runs to give one byte string, and tiled sheets to shift by exactly their usable width less the overlap.

## Measured by engines nobody here wrote

That reader shares the writer's idea of what a PDF means, so the file also has to come out right from an engine that does not. `bash docs/tasks/artifacts/printed_square/run_printed_square.sh` drafts the reference skirt, renders it with `pdftoppm` and with the CoreGraphics engine in macOS, finds the drawn lines in the pixels, and measures between them. `pdfinfo` reads the sheet counts and page sizes.

| Measured | Wanted | xpdf 4.06 | macOS CoreGraphics | One pixel |
| --- | --- | --- | --- | --- |
| Scale square on A0 | 100 mm | 99.991 | 99.966 | 0.085 |
| Scale square on A3 | 100 mm | 99.970 | 99.978 | 0.042 |
| Scale square on A4 | 100 mm | 99.970 to 99.991 | 99.978 | 0.042 |
| Waistband, cut length, on A0 | 800 mm | 799.973 | 799.972 | 0.085 |
| Waistband, cut width, on A0 | 100 mm | 100.034 | 100.000 | 0.085 |
| Front, hem to waist, on A0 | 660 mm | 659.966 | 659.967 | 0.085 |
| A4 neighbours, sideways over 564 crossings | 180 mm | 180.015 | 180.005 | 0.042 |
| A4 neighbours, upward drift over 240 crossings | 0 mm | 0.010 | 0.008 | 0.042 |

Every one is within a pixel, and the error does not grow with distance, so no scale error is hiding under it. The recorded table is `conformance/printed-square/renderer-measurements.tsv`. Each run also draws squares of 99 and 100.2 mm in a small file of its own and requires both to fail. Making the writer 0.1% too large, or shifting its sheets by 0.3 mm, made this check fail, and neither was left in.

**This measures pixels, not paper.** A printer has its own error, which only a ruler shows, and the units chapter keeps those two checks apart. [Physical evaluation](physical-evaluation.md) says how to measure it. **Hosted Linux has not run it yet.** It installs poppler for that, so it will be a third engine at the next push.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `write_pdf` | Writes a pattern at true scale, with its record |
| `pdf-writer` | A PDF library the roadmap names as a candidate |
| `printpdf` | A PDF library the roadmap names as a candidate |
| `pdftoppm` | A PDF renderer from xpdf or poppler |
| `pdfinfo` | A PDF reader from xpdf or poppler |
