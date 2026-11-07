#!/usr/bin/env bash
# docs/tasks/artifacts/printed_square/run_printed_square.sh
# G2-2D.7 — measures the reference skirt's PDF with renderers nobody in this repository wrote.
#
# WHY: the reader in crates/sc-artifacts/tests/pdf.rs parses what the writer wrote, so it shares the writer's idea of what a PDF
# means. A PDF is dimensionally correct when a real engine draws it at the right size. So this renders each file with what is
# installed, finds the drawn lines in the pixels, and measures between them. It writes no number from the file.
#
# WHAT IT MEASURES: the 100 mm scale square on A0, A3 and A4, each across two rows and two columns so a skew would show;
# on A0, the waistband and the front's full length, 800, 100 and 660 mm, because a scale error grows with distance; and across
# two tiled A4 sheets, that the overlap shows the same drawing, because that is what someone tapes together.
#
# WHAT IT DOES NOT MEASURE: paper. A printer has its own error, which is what a person with a ruler is for. This is not a T5 check
# and must not be recorded as one (units chapter §3). The limit here is the renderer's pixel, not a factory's tolerance.
#
# FALSIFIED ON EVERY RUN: it also draws squares of 99 mm and 100.2 mm in a small PDF it writes itself, and requires both to fail.
#
# Usage:  bash docs/tasks/artifacts/printed_square/run_printed_square.sh
# Output: target/printed_square/measured.tsv. Exit 0 when at least one renderer ran and every measurement is within a pixel.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
if ! python3 -I -B scripts/local_environment.py --verify >/dev/null 2>&1; then
  exec python3 -I -B scripts/local_environment.py -- bash docs/tasks/artifacts/printed_square/run_printed_square.sh "$@"
fi
STORE=target/printed_square
rm -rf "$STORE"
mkdir -p "$STORE"
cargo build -q -p sc-cli > "$STORE/build.log" 2>&1 || { echo "printed square: the program does not build; see $STORE/build.log" >&2; exit 1; }
target/debug/stitchcad replay conformance/reference-skirt/session.json "$STORE/project" > "$STORE/transcript.json" \
  || { echo "printed square: the session failed; see $STORE/transcript.json" >&2; exit 1; }
if command -v swiftc >/dev/null 2>&1; then
  cat > "$STORE/render.swift" <<'SWIFT'
import CoreGraphics
import Foundation

let arguments = CommandLine.arguments
guard let document = CGPDFDocument(URL(fileURLWithPath: arguments[1]) as CFURL),
      let page = document.page(at: Int(arguments[2]) ?? 1) else { exit(2) }
let scale = (Double(arguments[3]) ?? 72) / 72
let box = page.getBoxRect(.mediaBox)
let (width, height) = (Int((box.width * scale).rounded()), Int((box.height * scale).rounded()))
guard let context = CGContext(data: nil, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width,
                              space: CGColorSpaceCreateDeviceGray(), bitmapInfo: CGImageAlphaInfo.none.rawValue) else { exit(3) }
context.setFillColor(gray: 1, alpha: 1)
context.fill(CGRect(x: 0, y: 0, width: width, height: height))
context.scaleBy(x: scale, y: scale)
context.drawPDFPage(page)
guard let data = context.data else { exit(4) }
var out = Data("P5\n\(width) \(height)\n255\n".utf8)
out.append(Data(bytes: data, count: width * height))
try? out.write(to: URL(fileURLWithPath: arguments[4]))
SWIFT
  swiftc -O "$STORE/render.swift" -o "$STORE/render" > "$STORE/swift.log" 2>&1 || echo "printed square: CoreGraphics renderer did not build; see $STORE/swift.log" >&2
fi

exec python3 -I -B - <<'PY'
import json
import pathlib
import re
import shutil
import subprocess
import sys

STORE = pathlib.Path("target/printed_square")
EXPORTS = STORE / "project" / "exports"
MILLIMETRE = 25.4
SHEETS = {"a0": (841.0, 1189.0), "a3": (297.0, 420.0), "a4": (210.0, 297.0)}
MARGIN, GAP = 10.0, 10.0


def fail(message):
    print("printed square: " + message, file=sys.stderr)
    sys.exit(1)


class Image:
    def __init__(self, path):
        data = path.read_bytes()
        found = re.match(rb"P5\s+(\d+)\s+(\d+)\s+(\d+)\s", data)
        if not found or found.group(3) != b"255":
            fail("%s is not an 8-bit PGM" % path)
        self.width, self.height = int(found.group(1)), int(found.group(2))
        self.data, self.start = data, found.end()
        path.unlink()

    def row(self, y):
        at = self.start + y * self.width
        return self.data[at:at + self.width]

    def column(self, x, low, high):
        return self.data[self.start + low * self.width + x:self.start + high * self.width:self.width]


# The centre of each dark stretch along a line of pixels, by how dark each pixel is, so a line drawn over two pixels still has one centre.
def lines_along(pixels, offset=0):
    found, total, moment = [], 0, 0
    for index, value in enumerate(pixels):
        dark = 255 - value
        if dark > 40:
            total += dark
            moment += dark * (index + 0.5)
        elif total:
            found.append(offset + moment / total)
            total, moment = 0, 0
    if total:
        found.append(offset + moment / total)
    return found


class Sheet:
    def __init__(self, image, dpi, height_mm):
        self.image, self.dpi, self.height_mm = image, dpi, height_mm

    def px(self, millimetres):
        return millimetres * self.dpi / MILLIMETRE

    # Across at a height above the page's bottom edge: the distance between the first and last line from x0 to x1.
    def across(self, y_mm, x0, x1):
        y = int(self.px(self.height_mm - y_mm))
        low, high = max(int(self.px(x0)), 0), min(int(self.px(x1)), self.image.width)
        found = lines_along(self.image.row(y)[low:high], low)
        return (found[-1] - found[0]) * MILLIMETRE / self.dpi if len(found) >= 2 else None

    # Up at a distance from the page's left edge: the distance between the first and last line from y0 to y1 above the bottom edge.
    def up(self, x_mm, y0, y1):
        low = max(int(self.px(self.height_mm - y1)), 0)
        high = min(int(self.px(self.height_mm - y0)), self.image.height)
        found = lines_along(self.image.column(int(self.px(x_mm)), low, high), low)
        return (found[-1] - found[0]) * MILLIMETRE / self.dpi if len(found) >= 2 else None


def pdftoppm(path, page, dpi, root):
    root = STORE / root
    subprocess.run(["pdftoppm", "-r", str(dpi), "-gray", "-f", str(page), "-l", str(page), str(path), str(root)],
                   check=True, capture_output=True)
    written = sorted(STORE.glob(root.name + "-*.pgm"))
    if len(written) != 1:
        fail("pdftoppm wrote %d files for one page" % len(written))
    return written[0]


def coregraphics(path, page, dpi, root):
    target = STORE / (root + ".pgm")
    subprocess.run([str(STORE / "render"), str(path), str(page), str(dpi), str(target)], check=True, capture_output=True)
    return target


def renderers():
    listed = []
    if shutil.which("pdftoppm"):
        done = subprocess.run(["pdftoppm", "-v"], capture_output=True, text=True)
        listed.append((((done.stderr or done.stdout).splitlines() or ["pdftoppm"])[0].strip(), pdftoppm))
    if (STORE / "render").is_file():
        listed.append(("macOS CoreGraphics", coregraphics))
    return listed


ROWS = []
FAILED = []


def record(renderer, what, dpi, wanted, measured):
    limit = MILLIMETRE / dpi
    error = None if measured is None else measured - wanted
    within = error is not None and abs(error) <= limit
    ROWS.append("\t".join([renderer, what, str(dpi), "%.3f" % wanted, "-" if measured is None else "%.3f" % measured,
                           "-" if error is None else "%+.3f" % error, "%.3f" % limit, "yes" if within else "no"]))
    if not within:
        FAILED.append("%s: %s" % (renderer, what))


def square(sheet):
    return [("square across, 50 mm up", sheet.across(MARGIN + 50, MARGIN - 5, MARGIN + 105)),
            ("square across, 75 mm up", sheet.across(MARGIN + 75, MARGIN - 5, MARGIN + 105)),
            ("square up, 25 mm in", sheet.up(MARGIN + 25, MARGIN - 5, MARGIN + 105)),
            ("square up, 75 mm in", sheet.up(MARGIN + 75, MARGIN - 5, MARGIN + 105))]


def falsify(render):
    # A small file of this script's own, whose square is wrong by a stated amount. If these pass, nothing above means anything.
    caught = 0
    for side_mm in (99.0, 100.2):
        side = side_mm * 72 / MILLIMETRE
        at = MARGIN * 72 / MILLIMETRE
        content = ("0.25 w %.4f %.4f %.4f %.4f re S\n" % (at, at, side, side)).encode()
        width, height = SHEETS["a4"][0] * 72 / MILLIMETRE, SHEETS["a4"][1] * 72 / MILLIMETRE
        objects = [b"<< /Type /Catalog /Pages 2 0 R >>", b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
                   ("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 %.4f %.4f] /Contents 4 0 R >>" % (width, height)).encode(),
                   b"<< /Length %d >>\nstream\n" % len(content) + content + b"endstream"]
        body, offsets = b"%PDF-1.4\n", []
        for number, item in enumerate(objects, 1):
            offsets.append(len(body))
            body += b"%d 0 obj\n" % number + item + b"\nendobj\n"
        table = len(body)
        body += b"xref\n0 %d\n0000000000 65535 f \n" % (len(objects) + 1)
        for offset in offsets:
            body += b"%010d 00000 n \n" % offset
        body += b"trailer\n<< /Size %d /Root 1 0 R >>\nstartxref\n%d\n%%%%EOF\n" % (len(objects) + 1, table)
        path = STORE / ("wrong-%s.pdf" % side_mm)
        path.write_bytes(body)
        sheet = Sheet(Image(render(path, 1, 600, "wrong-%s" % side_mm)), 600, SHEETS["a4"][1])
        limit = MILLIMETRE / 600
        measured = [value for _, value in square(sheet)]
        if all(value is not None for value in measured) and any(abs(value - 100.0) > limit for value in measured):
            caught += 1
    return caught == 2


def page_facts(path):
    done = subprocess.run(["pdfinfo", str(path)], capture_output=True, text=True)
    if done.returncode:
        fail("pdfinfo refused %s: %s" % (path, done.stderr))
    pages = re.search(r"Pages:\s+(\d+)", done.stdout)
    size = re.search(r"Page size:\s+([\d.]+) x ([\d.]+) pts", done.stdout)
    return int(pages.group(1)), float(size.group(1)), float(size.group(2))


def main():
    listed = renderers()
    if not listed:
        fail("no renderer is installed here: pdftoppm from xpdf or poppler, or swiftc on macOS")
    record_of = lambda name: json.loads((EXPORTS / (name + ".record.json")).read_text(encoding="utf-8"))
    # What a PDF reader this project did not write says about the files, against their records and the paper sizes.
    if shutil.which("pdfinfo"):
        for name, sheet, turned in (("reference-skirt.pdf", "a0", True), ("reference-skirt-a3.pdf", "a3", False), ("reference-skirt-a4.pdf", "a4", False)):
            pages, width, height = page_facts(EXPORTS / name)
            wide, tall = SHEETS[sheet]
            if turned:
                wide, tall = tall, wide
            if pages != record_of(name)["sheets"]:
                FAILED.append("%s: pdfinfo counts %d sheets" % (name, pages))
            if abs(width - wide * 72 / MILLIMETRE) > 0.01 or abs(height - tall * 72 / MILLIMETRE) > 0.01:
                FAILED.append("%s: pdfinfo reads %s x %s pts" % (name, width, height))
    else:
        print("printed square: pdfinfo is not installed, so sheet counts and page sizes were not read by an outside reader")
    for renderer, render in listed:
        if not falsify(render):
            FAILED.append("%s: a wrongly sized square passed" % renderer)
        # A0, on its side: the drawing's row positions come from the layout rule and the record's size.
        a0 = Sheet(Image(render(EXPORTS / "reference-skirt.pdf", 1, 300, "a0")), 300, SHEETS["a0"][0])
        for what, value in square(a0):
            record(renderer, "A0 " + what, 300, 100.0, value)
        band_bottom = MARGIN + 660.0 + GAP
        record(renderer, "A0 waistband, cut length", 300, 800.0, a0.across(band_bottom + 15, MARGIN - 5, MARGIN + 805))
        record(renderer, "A0 waistband, cut width", 300, 100.0, a0.up(MARGIN + 100, band_bottom - 5, band_bottom + 105))
        front_left = MARGIN + 100 + GAP + 310.026 + GAP + 310.026 + GAP
        record(renderer, "A0 front, hem to waist on the cut line", 300, 660.0, a0.up(front_left + 5, MARGIN - 5, MARGIN + 665))
        for name, sheet in (("reference-skirt-a3.pdf", "a3"), ("reference-skirt-a4.pdf", "a4")):
            first = Sheet(Image(render(EXPORTS / name, 1, 600, sheet)), 600, SHEETS[sheet][1])
            for what, value in square(first):
                record(renderer, sheet.upper() + " " + what, 600, 100.0, value)
        # Neighbouring A4 sheets: the same 10 mm of the drawing appears on both, which is what someone tapes together.
        wide, tall = SHEETS["a4"]
        details = record_of("reference-skirt-a4.pdf")
        step_x, step_y = wide - 2 * MARGIN - 10.0, tall - 2 * MARGIN - 10.0
        columns = 1 + (details["drawing_width_um"] - 10_000 - 1) // int(step_x * 1000)
        # Across: the third and fourth columns of the third row, where a back dart's leg crosses the join at a slope. Lines crossing a
        # row must sit `step_x` apart. Earlier sheets hold only the hem there, which runs along the join and says nothing about it.
        one = Sheet(Image(render(EXPORTS / "reference-skirt-a4.pdf", 2 * columns + 3, 600, "n1")), 600, tall)
        two = Sheet(Image(render(EXPORTS / "reference-skirt-a4.pdf", 2 * columns + 4, 600, "n2")), 600, tall)
        worst, compared = 0.0, 0
        for tenth in range(int(MARGIN + 1) * 10, int(tall - MARGIN - 1) * 10):
            y_mm = tenth / 10
            left = lines_along(one.image.row(int(one.px(tall - y_mm))), 0)
            right = lines_along(two.image.row(int(two.px(tall - y_mm))), 0)
            left = [at for at in left if one.px(wide - MARGIN - 9) <= at <= one.px(wide - MARGIN - 1)]
            right = [at for at in right if two.px(MARGIN + 1) <= at <= two.px(MARGIN + 9)]
            if len(left) == len(right):
                for a, b in zip(left, right):
                    worst = max(worst, abs((a - b) * MILLIMETRE / 600 - step_x))
                    compared += 1
        if compared < 20:
            FAILED.append("%s: the sideways overlap showed only %d lines to compare" % (renderer, compared))
        record(renderer, "A4 sideways overlap, worst of %d crossings" % compared, 600, step_x, step_x + worst)
        # Up: the second and third rows of the second column, which hold the back's dart legs. A sloped line's position along a row
        # depends on the height of that row, so a sheet out of place vertically moves it.
        lower = 1 * columns + 2
        upper = 2 * columns + 2
        one = Sheet(Image(render(EXPORTS / "reference-skirt-a4.pdf", lower, 600, "n3")), 600, tall)
        two = Sheet(Image(render(EXPORTS / "reference-skirt-a4.pdf", upper, 600, "n4")), 600, tall)
        worst, compared = 0.0, 0
        for tenth in range(0, 80):
            y_mm = 1 + tenth / 10
            # Inside the printable edge, because the registration marks are not the drawing.
            inner = lambda found: [at for at in found if one.px(MARGIN + 1) <= at <= one.px(wide - MARGIN - 1)]
            left = inner(lines_along(one.image.row(int(one.px(tall - (tall - MARGIN - 10 + y_mm)))), 0))
            right = inner(lines_along(two.image.row(int(two.px(tall - (MARGIN + y_mm)))), 0))
            if left and len(left) == len(right):
                for a, b in zip(left, right):
                    worst = max(worst, abs(a - b) * MILLIMETRE / 600)
                    compared += 1
        if compared < 20:
            FAILED.append("%s: the upward overlap showed only %d lines to compare" % (renderer, compared))
        record(renderer, "A4 upward overlap, worst sideways drift of %d crossings" % compared, 600, 0.0, worst)
    table = "\t".join(["renderer", "measurement", "dpi", "wanted_mm", "measured_mm", "error_mm", "pixel_mm", "within_a_pixel"])
    (STORE / "measured.tsv").write_text("\n".join([table] + ROWS) + "\n", encoding="utf-8")
    print("\n".join(ROWS))
    if FAILED:
        print("printed square: NOT MET\n  " + "\n  ".join(FAILED), file=sys.stderr)
        sys.exit(1)
    print("printed square: %d measurements by %d renderer(s), each within a pixel; two wrong squares caught" % (len(ROWS), len(listed)))


main()
PY
