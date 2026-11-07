//! The reference skirt, drafted through the command line: the book's §12 obligations, and its frozen goldens.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use sc_artifacts::{compare, from_canonical_text, Comparison, DrawnPiece, MarkKind, Pattern};
use sc_cli::replay;
use sc_geometry::{segment_crossing, Point, SegmentCrossing};
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::BTreeMap;
use std::process::Command;

const SESSION: &str = include_str!("../../../conformance/reference-skirt/session.json");
const GOLDEN_CANONICAL: &str =
    include_str!("../../../conformance/goldens/reference-skirt/reference-skirt.canonical.json");
const GOLDEN_DXF: &str =
    include_str!("../../../conformance/goldens/reference-skirt/reference-skirt.dxf");
const GOLDEN_DXF_RECORD: &str =
    include_str!("../../../conformance/goldens/reference-skirt/reference-skirt.dxf.record.json");
const GOLDEN_PDF_RECORD: &str =
    include_str!("../../../conformance/goldens/reference-skirt/reference-skirt.pdf.record.json");

fn run() -> MemoryRepository {
    let mut disk = MemoryRepository::new();
    replay(&parse(SESSION).expect("valid"), &mut disk)
        .unwrap_or_else(|failure| panic!("{failure:?}"));
    disk
}

fn file(disk: &MemoryRepository, name: &str) -> Vec<u8> {
    disk.read(&FilePath::new(name).expect("a path"))
        .expect("reads")
        .unwrap_or_else(|| panic!("{name} was not exported"))
}

fn pattern() -> Pattern {
    let disk = run();
    from_canonical_text(
        &String::from_utf8(file(&disk, "exports/reference-skirt.canonical.json")).expect("text"),
    )
    .expect("reads")
}

fn piece<'a>(pattern: &'a Pattern, name: &str) -> &'a DrawnPiece {
    pattern
        .pieces
        .iter()
        .find(|item| item.name == name)
        .unwrap_or_else(|| panic!("no piece {name}"))
}

fn extent(points: &[Point]) -> (i64, i64, i64, i64) {
    let xs = points.iter().map(|at| at.x());
    let ys = points.iter().map(|at| at.y());
    (
        xs.clone().min().expect("x"),
        xs.max().expect("x"),
        ys.clone().min().expect("y"),
        ys.max().expect("y"),
    )
}

fn assert_clean(points: &[Point], what: &str) {
    let count = points.len();
    for i in 0..count {
        for j in (i + 1)..count {
            let neighbours = j == i + 1 || (i == 0 && j + 1 == count);
            match segment_crossing(
                points[i],
                points[(i + 1) % count],
                points[j],
                points[(j + 1) % count],
            ) {
                SegmentCrossing::Apart => {}
                SegmentCrossing::Overlap => panic!("{what}: segments {i} and {j} overlap"),
                SegmentCrossing::At { .. } => {
                    assert!(neighbours, "{what}: segments {i} and {j} cross")
                }
            }
        }
    }
}

#[test]
fn every_piece_the_book_lists_is_drafted_and_nothing_else() {
    let drafted = pattern();
    let mut names: Vec<&str> = drafted
        .pieces
        .iter()
        .map(|item| item.name.as_str())
        .collect();
    names.sort_unstable();
    assert_eq!(
        names,
        [
            "skirt_back_left",
            "skirt_back_right",
            "skirt_front",
            "waistband",
            "waistband_interfacing"
        ]
    );
    assert!(drafted
        .pieces
        .iter()
        .all(|item| item.size == "M" && item.quantity == 1));
}

#[test]
fn the_pieces_measure_what_the_book_says() {
    let drafted = pattern();
    // §5 step 8: one rectangle, the pattern length by the cut width, all four sides cut.
    assert_eq!(
        extent(&piece(&drafted, "waistband").cut),
        (0, 800_000, 0, 100_000)
    );
    assert!(
        piece(&drafted, "waistband")
            .cut
            .iter()
            .all(|at| at.x() == 0 || at.x() == 800_000 || at.y() == 0 || at.y() == 100_000),
        "one rectangle, whatever its notches add"
    );
    // §5 step 9: the band's finished dimensions, with no allowance on any edge.
    let interfacing = piece(&drafted, "waistband_interfacing");
    assert_eq!(extent(&interfacing.cut), (0, 770_000, 0, 40_000));
    assert_eq!(
        extent(&interfacing.sew),
        extent(&interfacing.cut),
        "no allowance anywhere"
    );
    // The front's net: 25.5 cm at the hip, 28.5 at the hem, 62 long.
    let front = piece(&drafted, "skirt_front");
    assert_eq!(extent(&front.sew), (0, 285_000, -620_000, 0));
    // The cut line adds 3 cm at the hem, 1 cm at the waist, and nothing past the fold at CF.
    let (low_x, _, low_y, high_y) = extent(&front.cut);
    assert_eq!((low_x, low_y, high_y), (0, -650_000, 10_000));
    // The back has 1.5 cm at CB where the front has a fold.
    assert_eq!(extent(&piece(&drafted, "skirt_back_right").cut).0, -15_000);
}

#[test]
fn the_front_side_seam_measures_true_and_the_seams_that_sew_together_are_equal() {
    let drafted = pattern();
    let seam = |item: &DrawnPiece| -> f64 {
        // The straight side seam runs from the hem corner to the hip point, on the sew line.
        let hip = item
            .sew
            .iter()
            .find(|at| at.y() == -200_000 && at.x() > 100_000)
            .expect("the hip point");
        let hem = item
            .sew
            .iter()
            .find(|at| at.y() == -620_000 && at.x() > 100_000)
            .expect("the hem corner");
        ((hip.x() - hem.x()) as f64).hypot((hip.y() - hem.y()) as f64)
    };
    let (front, back) = (
        seam(piece(&drafted, "skirt_front")),
        seam(piece(&drafted, "skirt_back_right")),
    );
    assert!(
        (front - 421_070.0).abs() < 1.0,
        "the book's 42.107 cm: {front}"
    );
    assert!(
        (front - back).abs() < 1e-9,
        "zero-ease seams sew together: {front} against {back}"
    );
}

#[test]
fn closing_each_dart_removes_exactly_its_intake_and_the_panel_owes_the_quarter_waist() {
    let drafted = pattern();
    for name in ["skirt_front", "skirt_back_right"] {
        let item = piece(&drafted, name);
        let legs: Vec<i64> = item.internal.iter().map(|line| line.from.x()).collect();
        assert_eq!(
            (legs[1] - legs[0]).abs(),
            40_000,
            "{name}: the dart's 4.0 cm"
        );
        assert_eq!(item.internal[0].to, item.internal[1].to, "{name}: one apex");
        // §5 step 2: the waist edge is 22.5 cm before the dart closes it, and 18.5 after.
        let waist = item
            .sew
            .iter()
            .filter(|at| at.y() == 0)
            .map(|at| at.x())
            .max()
            .expect("the waist");
        assert_eq!(waist, 225_000);
        assert_eq!(waist - 40_000, 185_000, "{name}: the quarter waist");
    }
    assert_eq!(piece(&drafted, "skirt_front").internal[0].to.y(), -120_000);
    assert_eq!(
        piece(&drafted, "skirt_back_right").internal[0].to.y(),
        -140_000
    );
}

#[test]
fn allowances_produce_no_crossing_on_any_piece() {
    for item in &pattern().pieces {
        assert_clean(&item.cut, &item.name);
        assert_clean(&item.sew, &item.name);
    }
}

#[test]
fn the_mirrored_piece_is_exactly_the_mirror_and_carries_its_own_label() {
    let drafted = pattern();
    let (right, left) = (
        piece(&drafted, "skirt_back_right"),
        piece(&drafted, "skirt_back_left"),
    );
    let mirrored = |points: &[Point]| {
        let mut out: Vec<(i64, i64)> = points.iter().map(|at| (-at.x(), at.y())).collect();
        out.sort_unstable();
        out
    };
    let sorted = |points: &[Point]| {
        let mut out: Vec<(i64, i64)> = points.iter().map(|at| (at.x(), at.y())).collect();
        out.sort_unstable();
        out
    };
    assert_eq!(mirrored(&right.cut), sorted(&left.cut));
    assert_eq!(mirrored(&right.sew), sorted(&left.sew));
    assert!(
        left.labels.iter().any(|label| label.text.ends_with(", L")),
        "{:?}",
        left.labels
    );
    assert!(
        right.labels.iter().any(|label| label.text.ends_with(", R")),
        "{:?}",
        right.labels
    );
}

#[test]
fn the_fold_has_no_allowance_and_its_label_says_so_and_the_band_fold_is_not_a_cut_line() {
    let drafted = pattern();
    let front = piece(&drafted, "skirt_front");
    // No allowance past the fold: the cut line's straight run down x = 0 spans the front, waist allowance to hem allowance.
    let on_fold: Vec<i64> = front
        .cut
        .iter()
        .filter(|at| at.x() == 0)
        .map(|at| at.y())
        .collect();
    assert_eq!(
        (on_fold.iter().min().copied(), on_fold.iter().max().copied()),
        (Some(-650_000), Some(10_000)),
        "the cut line runs along the fold"
    );
    assert!(
        front.cut.iter().all(|at| at.x() >= 0),
        "nothing crosses the fold"
    );
    assert!(front
        .labels
        .iter()
        .any(|label| label.text.contains("place on fold")));
    for name in ["skirt_back_right", "waistband", "waistband_interfacing"] {
        assert!(
            !piece(&drafted, name)
                .labels
                .iter()
                .any(|label| label.text.contains("place on fold")),
            "{name}"
        );
    }
    let band = piece(&drafted, "waistband");
    assert_eq!(band.internal.len(), 1);
    assert_eq!(
        (band.internal[0].from.y(), band.internal[0].to.y()),
        (50_000, 50_000),
        "the band's midline"
    );
    assert!(
        band.cut.iter().all(|at| at.y() != 50_000),
        "the fold never touches the cut line as a vertex"
    );
}

#[test]
fn notches_sit_where_the_book_puts_them() {
    let drafted = pattern();
    let kinds = |name: &str| {
        piece(&drafted, name)
            .marks
            .iter()
            .filter(|mark| mark.kind == MarkKind::VNotch)
            .count()
    };
    assert_eq!(kinds("skirt_front"), 1, "one single notch at hip level");
    assert_eq!(kinds("skirt_back_right"), 2, "the hip, and the zipper stop");
    assert_eq!(kinds("waistband"), 5, "each CB, and the quarter points");
    // Front and back carry the hip notch at the same parameter, so walking the seam aligns them.
    let hip = |name: &str| {
        piece(&drafted, name)
            .marks
            .iter()
            .find(|mark| (mark.at.y() + 200_000).abs() < 100)
            .expect("a hip notch")
            .at
    };
    assert_eq!(hip("skirt_front"), hip("skirt_back_right"));
    // The zipper stop is `zip_len` below the waist on the CB's cut edge.
    let stop = piece(&drafted, "skirt_back_right")
        .marks
        .iter()
        .find(|mark| mark.at.y() == -180_000)
        .expect("the zipper stop");
    assert_eq!(stop.at.x(), -15_000);
}

#[test]
fn the_export_equals_its_frozen_goldens_byte_for_byte() {
    let disk = run();
    let canonical =
        String::from_utf8(file(&disk, "exports/reference-skirt.canonical.json")).expect("text");
    // Semantic first, so a real change names its piece and field.
    assert_eq!(
        compare(GOLDEN_CANONICAL, &canonical, 0).expect("both read"),
        Comparison::Identical
    );
    assert_eq!(
        String::from_utf8(file(&disk, "exports/reference-skirt.dxf")).expect("text"),
        GOLDEN_DXF
    );
    assert_eq!(
        String::from_utf8(file(&disk, "exports/reference-skirt.dxf.record.json")).expect("text"),
        GOLDEN_DXF_RECORD
    );
    assert_eq!(
        String::from_utf8(file(&disk, "exports/reference-skirt.pdf.record.json")).expect("text"),
        GOLDEN_PDF_RECORD
    );
}

#[test]
fn the_golden_catches_a_changed_measurement_naming_the_piece() {
    let text = SESSION.replace("\"um\": 980000", "\"um\": 980100");
    let mut disk = MemoryRepository::new();
    replay(&parse(&text).expect("valid"), &mut disk)
        .unwrap_or_else(|failure| panic!("{failure:?}"));
    let canonical =
        String::from_utf8(file(&disk, "exports/reference-skirt.canonical.json")).expect("text");
    let Comparison::Different(found) = compare(GOLDEN_CANONICAL, &canonical, 0).expect("both read")
    else {
        panic!("a changed hip girth must change the pattern");
    };
    assert!(
        found
            .iter()
            .any(|item| item.piece == "skirt_front" && item.field == "cut"),
        "{found:?}"
    );
}

#[test]
fn every_closure_holds_after_a_measurement_change_and_a_broken_recipe_says_which() {
    // Different measurements still balance, because the dart intake is derived.
    let mut changed = parse(SESSION).expect("valid");
    if let Value::Object(fields) = &mut changed {
        if let Some(Value::List(inputs)) = fields.get_mut("inputs") {
            for input in inputs {
                if let Value::Object(entry) = input {
                    if entry.get("token").and_then(Value::as_text) == Some("hip_girth") {
                        entry.insert("um".to_owned(), Value::Int(1_000_000));
                    }
                }
            }
        }
    }
    replay(&changed, MemoryRepository::new()).unwrap_or_else(|failure| panic!("{failure:?}"));

    // A wrong construction is caught by the assertion that names it: the D33 defect class.
    let broken = SESSION.replace(
        "let waist_side: length = quarter_hip - ss_suppress",
        "let waist_side: length = quarter_waist - ss_suppress",
    );
    assert_ne!(broken, SESSION);
    let failure = replay(&parse(&broken).expect("valid"), MemoryRepository::new())
        .expect_err("the waist no longer closes")
        .to_value();
    let fields = failure.as_object().expect("a diagnostic");
    assert_eq!(fields["failed"].as_text(), Some("recipe"));
    assert!(
        fields["message"]
            .as_text()
            .expect("text")
            .contains("waist_closure"),
        "{failure:?}"
    );
}

#[test]
fn the_whole_skirt_fits_one_a0_sheet_on_its_side_and_a_longer_one_is_refused_rather_than_shrunk() {
    let disk = run();
    let record =
        |name: &str| parse(&String::from_utf8(file(&disk, name)).expect("text")).expect("a record");
    let single = record("exports/reference-skirt.pdf.record.json");
    let fields = single.as_object().expect("an object");
    assert_eq!(fields["sheets"].as_int(), Some(1));
    assert_eq!(fields["orientation"].as_text(), Some("landscape"));
    assert_eq!(fields["scale"].as_text(), Some("true_scale_never_rescaled"));
    let sheets = |name: &str| record(name).as_object().expect("an object")["sheets"].as_int();
    assert!(sheets("exports/reference-skirt-a3.pdf.record.json") > Some(1));
    assert!(
        sheets("exports/reference-skirt-a4.pdf.record.json")
            > sheets("exports/reference-skirt-a3.pdf.record.json")
    );
    // The file's own page box says the same, read from the file and not from its record.
    let bytes = file(&disk, "exports/reference-skirt.pdf");
    let text = String::from_utf8_lossy(&bytes);
    assert_eq!(text.matches("/Type /Page ").count(), 1);
    assert!(
        text.contains("/MediaBox [0 0 3370.3937 2383.9370]"),
        "{}",
        &text[..300]
    );

    let long = SESSION.replace("\"um\": 620000", "\"um\": 1300000");
    assert_ne!(long, SESSION);
    let failure = replay(&parse(&long).expect("valid"), MemoryRepository::new())
        .expect_err("it does not fit")
        .to_value();
    assert!(
        failure.as_object().expect("a diagnostic")["message"]
            .as_text()
            .expect("text")
            .contains("never rescaled"),
        "{failure:?}"
    );
    let tiled = long.replace("\"layout\": \"a0\"", "\"layout\": \"a0_tiled\"");
    assert_ne!(tiled, long);
    replay(&parse(&tiled).expect("valid"), MemoryRepository::new())
        .unwrap_or_else(|failure| panic!("{failure:?}"));
}

// One block of the written DXF, read with no help from the writer: its polylines by layer, and its text.
#[derive(Default)]
struct Block {
    lines: BTreeMap<String, Vec<Vec<(i64, i64)>>>,
    texts: Vec<String>,
}

fn micrometres(millimetres: &str) -> i64 {
    (millimetres.trim().parse::<f64>().expect("a number") * 1000.0).round() as i64
}

fn blocks(text: &str) -> BTreeMap<String, Block> {
    let mut out: BTreeMap<String, Block> = BTreeMap::new();
    let (mut name, mut kind, mut layer, mut x) = (String::new(), String::new(), String::new(), 0);
    let lines: Vec<&str> = text.lines().collect();
    for pair in lines.chunks(2) {
        let code: i32 = pair[0].trim().parse().expect("a group code");
        let value = pair[1].trim();
        let block = out.entry(name.clone()).or_default();
        match (code, kind.as_str()) {
            (0, _) => value.clone_into(&mut kind),
            (2, "BLOCK") => value.clone_into(&mut name),
            (8, "POLYLINE") => {
                value.clone_into(&mut layer);
                block
                    .lines
                    .entry(layer.clone())
                    .or_default()
                    .push(Vec::new());
            }
            (10, "VERTEX") => x = micrometres(value),
            (20, "VERTEX") => block
                .lines
                .get_mut(&layer)
                .and_then(|listed| listed.last_mut())
                .expect("a vertex belongs to a polyline")
                .push((x, micrometres(value))),
            (1, "TEXT") => block.texts.push(value.to_owned()),
            _ => {}
        }
    }
    out
}

#[test]
fn the_net_lines_are_identical_whichever_line_goes_on_the_cut_layer() {
    // §12's allowances included and excluded: which line a receiver cuts is a profile's choice, which is G4's. What G2 can hold
    // to is the property itself, on the file: the net line's coordinates do not move, and only the layer of each line does.
    let text = SESSION.replace("\"semantics\": \"cut_as_1\"", "\"semantics\": \"sew_as_1\"");
    assert_ne!(text, SESSION);
    let (mut a, mut b) = (MemoryRepository::new(), MemoryRepository::new());
    replay(&parse(SESSION).expect("valid"), &mut a).expect("runs");
    replay(&parse(&text).expect("valid"), &mut b).expect("runs");
    assert_eq!(
        file(&a, "exports/reference-skirt.canonical.json"),
        file(&b, "exports/reference-skirt.canonical.json"),
        "the pattern is the same"
    );
    let read = |disk: &MemoryRepository| {
        blocks(&String::from_utf8(file(disk, "exports/reference-skirt.dxf")).expect("text"))
    };
    let (first, second) = (read(&a), read(&b));
    let names = [
        "skirt_back_left_M",
        "skirt_back_right_M",
        "skirt_front_M",
        "waistband_M",
        "waistband_interfacing_M",
    ];
    for name in names {
        let (one, other) = (&first[name], &second[name]);
        assert_eq!(
            one.lines["CUT"], other.lines["DRAW"],
            "{name}: the cut line moved layer and nothing else"
        );
        assert_eq!(
            one.lines["DRAW"], other.lines["CUT"],
            "{name}: the net line moved layer and nothing else"
        );
        assert_eq!(one.lines["CUT"].len(), 1, "{name}");
        // The interfacing has no allowance anywhere, so its two lines are the same line. Every other piece has two.
        assert_eq!(
            one.lines["CUT"] == one.lines["DRAW"],
            name == "waistband_interfacing_M",
            "{name}"
        );
    }
}

#[test]
fn the_written_file_says_place_on_fold_only_on_the_front_and_keeps_the_band_fold_off_the_cut_layer()
{
    let disk = run();
    let written =
        blocks(&String::from_utf8(file(&disk, "exports/reference-skirt.dxf")).expect("text"));
    let says = |name: &str| {
        written[name]
            .texts
            .iter()
            .any(|text| text.contains("place on fold"))
    };
    assert!(says("skirt_front_M"));
    for name in [
        "skirt_back_left_M",
        "skirt_back_right_M",
        "waistband_M",
        "waistband_interfacing_M",
    ] {
        assert!(!says(name), "{name}");
    }
    let band = &written["waistband_M"];
    assert_eq!(
        band.lines["INTCUT"],
        [vec![(0, 50_000), (800_000, 50_000)]],
        "the fold is an internal line"
    );
    assert!(
        band.lines["CUT"][0].iter().all(|at| at.1 != 50_000),
        "and it is on no cut line"
    );
    // The interfacing is fused: nothing folded, nothing allowed, one layer's worth of cloth.
    let interfacing = &written["waistband_interfacing_M"];
    assert!(!interfacing.lines.contains_key("INTCUT"));
    let drafted = pattern();
    let piece = piece(&drafted, "waistband_interfacing");
    assert_eq!(piece.cut, piece.sew, "no allowance on any edge");
}

#[test]
fn every_notch_is_a_vertex_of_the_cut_line_and_points_into_the_piece() {
    let drafted = pattern();
    let mut seen = 0;
    for item in &drafted.pieces {
        for mark in &item.marks {
            assert!(
                item.cut.contains(&mark.at),
                "{}: {:?} must be a vertex",
                item.name,
                mark.at
            );
            let (x, y) = (mark.toward.x() as f64, mark.toward.y() as f64);
            let mut inside = false;
            for (index, a) in item.cut.iter().enumerate() {
                let b = item.cut[(index + 1) % item.cut.len()];
                let (ay, by) = (a.y() as f64, b.y() as f64);
                if (ay > y) != (by > y)
                    && x < (b.x() - a.x()) as f64 * (y - ay) / (by - ay) + a.x() as f64
                {
                    inside = !inside;
                }
            }
            assert!(
                inside,
                "{}: the notch at {:?} points outside",
                item.name, mark.at
            );
            seen += 1;
        }
    }
    assert_eq!(seen, 10);
    // A notch named well off the cut line is a wrong notch, and it says so rather than moving a long way.
    let broken = SESSION
        .replace(
            "let hip_notch_x: length = quarter_hip + sa_side",
            "let hip_notch_x: length = quarter_hip + sa_side - 1 mm",
        )
        .replace(
            r#"{
              "aim": [
                "sa_side",
                "zero"
              ],
              "at": "1",
              "depth": "notch_depth",
              "edge": "side_lower",
              "name": "hip"
            }"#,
            r#"[["hip_notch_x", "-waist_to_hip"], ["hip_notch_end", "-waist_to_hip"]]"#,
        );
    assert!(
        broken.contains(r#"[["hip_notch_x""#),
        "the front's notch is written as two points"
    );
    let failure = replay(&parse(&broken).expect("valid"), MemoryRepository::new())
        .expect_err("off the line")
        .to_value();
    let message = failure.as_object().expect("a diagnostic")["message"]
        .as_text()
        .expect("text")
        .to_owned();
    assert!(
        message.contains("skirt_front") && message.contains("cut line"),
        "{message}"
    );
}

// The length along a net line from one corner to another, in its own running order.
fn walk(line: &[Point], from: (i64, i64), to: (i64, i64)) -> f64 {
    let at = |wanted: (i64, i64)| {
        line.iter()
            .position(|point| (point.x(), point.y()) == wanted)
            .unwrap_or_else(|| panic!("no corner at {wanted:?}"))
    };
    let (mut index, end, mut total) = (at(from), at(to), 0.0);
    while index != end {
        let (a, b) = (line[index], line[(index + 1) % line.len()]);
        total += ((b.x() - a.x()) as f64).hypot((b.y() - a.y()) as f64);
        index = (index + 1) % line.len();
    }
    total
}

#[test]
fn every_span_of_the_sewing_graph_walks_equal_because_they_all_declare_zero_ease() {
    // The `walk` command and the sewing graph as data are G3's. This walks §8's table by hand over the drawn lines, so the property
    // is held now and the command has something to equal when it exists. No span may differ by more than the numerical class.
    let class = 1.0;
    let drafted = pattern();
    let (front, right, left) = (
        &piece(&drafted, "skirt_front").sew,
        &piece(&drafted, "skirt_back_right").sew,
        &piece(&drafted, "skirt_back_left").sew,
    );
    let (hem, hip, waist) = ((285_000, -620_000), (255_000, -200_000), (225_000, 0));
    let mirror = |at: (i64, i64)| (-at.0, at.1);
    // ss_right: the whole seam, and each part of it, because the hip notch must align too.
    assert!((walk(front, hem, waist) - walk(right, hem, waist)).abs() <= class);
    assert!((walk(front, hem, hip) - walk(right, hem, hip)).abs() <= class);
    // ss_left: the left piece runs the other way along its mirrored seam.
    assert!((walk(front, hem, waist) - walk(left, mirror(waist), mirror(hem))).abs() <= class);
    assert!((walk(front, hem, hip) - walk(left, mirror(hip), mirror(hem))).abs() <= class);
    // cb: the two backs, and the zipper-stop notch on each.
    let (cb_top, cb_bottom) = ((0, 0), (0, -620_000));
    assert!((walk(right, cb_top, cb_bottom) - walk(left, cb_bottom, cb_top)).abs() <= class);
    let stop = |name: &str| {
        piece(&drafted, name)
            .marks
            .iter()
            .find(|mark| mark.at.x().abs() == 15_000)
            .expect("the stop")
            .at
            .y()
    };
    assert_eq!(stop("skirt_back_right"), stop("skirt_back_left"));
    // waist: each panel's edge less its dart, against the band between its quarter notches, which are the landmarks of this span.
    let band: Vec<i64> = piece(&drafted, "waistband")
        .marks
        .iter()
        .map(|mark| mark.at.x())
        .collect();
    for name in ["skirt_front", "skirt_back_right"] {
        let item = piece(&drafted, name);
        let dart = (item.internal[1].from.x() - item.internal[0].from.x()).abs();
        let edge = walk(&item.sew, waist, (0, 0)) - dart as f64;
        for pair in band.windows(2) {
            assert!(
                (edge - (pair[1] - pair[0]) as f64).abs() <= class,
                "{name}: {edge} against {pair:?}"
            );
        }
    }
}

#[test]
fn every_closure_holds_or_reports_for_a_change_to_every_measurement() {
    // §12: all four closures hold after any measurement change, or the recipe reports the imbalance. This re-derives the waist from the
    // drawing, not the session's arithmetic, because a balance over declared quantities is what hid D33.
    // One sheet is refused for size, which is a true answer about paper and hides the closures, so this sweep tiles it.
    let session =
        parse(&SESSION.replace("\"layout\": \"a0\"", "\"layout\": \"a0_tiled\"")).expect("valid");
    let Value::Object(fields) = &session else {
        panic!("an object")
    };
    let tokens: Vec<(String, i64)> = fields["inputs"]
        .as_list()
        .expect("inputs")
        .iter()
        .map(|input| {
            let input = input.as_object().expect("an input");
            (
                input["token"].as_text().expect("text").to_owned(),
                input["um"].as_int().expect("a number"),
            )
        })
        .collect();
    let (mut held, mut reported) = (0, 0);
    for (token, value) in &tokens {
        for delta in [-20_000, -4_000, 4_000, 20_000] {
            let mut changed = session.clone();
            if let Value::Object(fields) = &mut changed {
                if let Some(Value::List(inputs)) = fields.get_mut("inputs") {
                    for input in inputs {
                        if let Value::Object(entry) = input {
                            if entry.get("token").and_then(Value::as_text) == Some(token) {
                                entry.insert("um".to_owned(), Value::Int(value + delta));
                            }
                        }
                    }
                }
            }
            let mut disk = MemoryRepository::new();
            match replay(&changed, &mut disk) {
                Err(failure) => {
                    let failure = failure.to_value();
                    let fields = failure.as_object().expect("a diagnostic");
                    assert!(
                        fields.contains_key("failed")
                            && !fields["message"].as_text().expect("text").is_empty(),
                        "{token} {delta}: {failure:?}"
                    );
                    let message = fields["message"].as_text().expect("text");
                    assert!(
                        message.contains("negative allowance"),
                        "{token} {delta}: {message}"
                    );
                    reported += 1;
                }
                Ok(_) => {
                    let text =
                        String::from_utf8(file(&disk, "exports/reference-skirt.canonical.json"))
                            .expect("text");
                    let drafted = from_canonical_text(&text).expect("reads");
                    let input = |name: &str| {
                        let base = tokens
                            .iter()
                            .find(|(found, _)| found == name)
                            .expect("an input")
                            .1;
                        if name == token {
                            base + delta
                        } else {
                            base
                        }
                    };
                    let quarter = (input("waist_girth") + input("ease_waist")) / 4;
                    for name in ["skirt_front", "skirt_back_right"] {
                        let item = piece(&drafted, name);
                        let waist = item
                            .sew
                            .iter()
                            .filter(|at| at.y() == 0)
                            .map(|at| at.x())
                            .max()
                            .expect("a waist");
                        let dart = (item.internal[1].from.x() - item.internal[0].from.x()).abs();
                        assert_eq!(
                            waist - dart,
                            quarter,
                            "{token} {delta}: {name} drafts a waist it does not owe"
                        );
                    }
                    let band = piece(&drafted, "waistband");
                    let (start, end) = (band.marks[0].at.x(), band.marks[4].at.x());
                    assert_eq!(
                        end - start,
                        4 * quarter,
                        "{token} {delta}: the band does not fit the waist"
                    );
                    held += 1;
                }
            }
        }
    }
    // Taking 2 cm from a 1 cm allowance is the only change on this list that has no answer, and four inputs are that small.
    assert_eq!((held, reported), (tokens.len() * 4 - 4, 4));
}

#[test]
fn the_real_program_replays_it_byte_identically_under_a_foreign_locale() {
    let directory = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("sc-cli-skirt");
    let (first, second) = (directory.join("first"), directory.join("second"));
    for path in [&first, &second] {
        if path.exists() {
            std::fs::remove_dir_all(path).expect("clears the last run");
        }
    }
    let session = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../conformance/reference-skirt/session.json"
    );
    let run = |project: &std::path::Path, zone: &str| {
        Command::new(env!("CARGO_BIN_EXE_stitchcad"))
            .args(["replay", session, project.to_str().expect("a path")])
            .env("LC_ALL", "fr_FR.UTF-8")
            .env("TZ", zone)
            .output()
            .expect("the program runs")
    };
    let (a, b) = (run(&first, "Asia/Tokyo"), run(&second, "America/Lima"));
    assert!(a.status.success(), "{}", String::from_utf8_lossy(&a.stdout));
    assert_eq!(a.stdout, b.stdout);
    for name in [
        "reference-skirt.canonical.json",
        "reference-skirt.dxf",
        "reference-skirt.pdf",
        "reference-skirt-a3.pdf",
        "reference-skirt-a4.pdf",
    ] {
        let at = |project: &std::path::Path| {
            std::fs::read(project.join("exports").join(name)).expect(name)
        };
        assert_eq!(at(&first), at(&second), "{name}");
    }
}

#[test]
fn every_position_the_book_pins_is_where_the_drawing_has_it() {
    let chapter = include_str!("../../../docs/book/src/spec/reference-skirt.md");
    let drafted = pattern();
    let mut checked = 0;
    let at = chapter
        .find("### The drawing, pinned")
        .expect("the section");
    for line in chapter[at..].lines() {
        let cells: Vec<&str> = line.trim_matches('|').split(" | ").map(str::trim).collect();
        let (Some(name), Some(position)) = (cells.first(), cells.get(1)) else {
            continue;
        };
        if !name.starts_with("skirt_") && !name.starts_with("waistband") {
            continue;
        }
        let numbers: Vec<f64> = position
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .map(|number| number.parse().expect("a position in centimetres"))
            .collect();
        let wanted = (
            (numbers[0] * 10_000.0).round() as i64,
            (numbers[1] * 10_000.0).round() as i64,
        );
        let line = &piece(&drafted, name).sew;
        assert!(
            line.iter().any(|point| (point.x(), point.y()) == wanted),
            "{name}: the book says {position}, and the sew line has no such point"
        );
        checked += 1;
    }
    assert_eq!(checked, 22, "a row was lost");
}
