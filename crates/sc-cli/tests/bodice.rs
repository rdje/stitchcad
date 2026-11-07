//! The reference bodice and set-in sleeve: a garment whose seams do not all match, on purpose, and which says which.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use sc_artifacts::{
    from_canonical_text, judge_dxf, judge_pdf_square, to_canonical_text, MarkKind, Pattern,
};
use sc_cli::{replay, Running};
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::BTreeMap;

const SESSION: &str = include_str!("../../../conformance/reference-bodice/session.json");
const BOOK: &str = include_str!("../../../docs/book/src/spec/reference-bodice.md");
const GOLDEN_CANONICAL: &str =
    include_str!("../../../conformance/goldens/reference-bodice/reference-bodice.canonical.json");
const GOLDEN_DXF: &str =
    include_str!("../../../conformance/goldens/reference-bodice/reference-bodice.dxf");
// Twice T2: two lines each within T2 of one curve can differ by this much.
const WALK_TOLERANCE: i64 = 20;
const PIECES: [&str; 7] = [
    "bodice_front",
    "bodice_back_right",
    "bodice_back_left",
    "sleeve_right",
    "sleeve_left",
    "collar_right",
    "collar_left",
];

fn fields_of(value: &Value) -> &BTreeMap<String, Value> {
    value.as_object().expect("an object")
}

fn session() -> Value {
    parse(SESSION).expect("valid")
}

// The session up to its steps, then its draft, so that a test can change what it draws.
fn opening() -> (Value, Value) {
    let whole = session();
    let draft = fields_of(&whole)["steps"]
        .as_list()
        .expect("steps")
        .iter()
        .find(|step| fields_of(step)["do"].as_text() == Some("draft"))
        .expect("a draft")
        .clone();
    let mut bare = fields_of(&whole).clone();
    bare.insert("steps".to_owned(), Value::List(Vec::new()));
    (Value::Object(bare), draft)
}

fn drawn(disk: &mut MemoryRepository) -> Running<&mut MemoryRepository> {
    let (bare, draft) = opening();
    let mut running = Running::open(&bare, disk).expect("opens");
    running.step(&draft).expect("draws");
    running
}

fn step(running: &mut Running<&mut MemoryRepository>, text: &str) -> Result<Value, String> {
    running
        .step(&parse(text).expect("a step"))
        .map_err(|failure| {
            fields_of(&failure.to_value())["message"]
                .as_text()
                .expect("text")
                .to_owned()
        })
}

fn result(record: &Value) -> Value {
    fields_of(record)["result"].clone()
}

struct Walked {
    a: i64,
    b: i64,
    ease: i64,
    unexplained: i64,
    finding: String,
}

fn walked(running: &mut Running<&mut MemoryRepository>) -> BTreeMap<String, Walked> {
    result(&step(running, r#"{"do":"walk"}"#).expect("walks"))
        .as_list()
        .expect("reports")
        .iter()
        .map(|report| {
            let report = fields_of(report);
            (
                report["seam"].as_text().expect("a name").to_owned(),
                Walked {
                    a: report["a_length"].as_int().expect("a length"),
                    b: report["b_length"].as_int().expect("a length"),
                    ease: report["ease"].as_int().expect("a length"),
                    unexplained: report["unexplained"].as_int().expect("a length"),
                    finding: report["finding"].as_text().expect("text").to_owned(),
                },
            )
        })
        .collect()
}

fn bindings(running: &mut Running<&mut MemoryRepository>) -> BTreeMap<String, i64> {
    result(&step(running, r#"{"do":"evaluate"}"#).expect("evaluates"))
        .as_list()
        .expect("bindings")
        .iter()
        .filter_map(|binding| {
            let binding = fields_of(binding);
            if binding["kind"].as_text() != Some("length") {
                return None;
            }
            Some((
                binding["name"].as_text()?.to_owned(),
                binding["value"].as_text()?.parse().ok()?,
            ))
        })
        .collect()
}

fn fitted(running: &Running<&mut MemoryRepository>, vary: &str) -> i64 {
    running
        .pattern()
        .expect("drawn")
        .fits
        .iter()
        .find(|fit| fit.vary == vary)
        .unwrap_or_else(|| panic!("nothing fitted {vary}"))
        .value
}

fn input(token: &str) -> i64 {
    fields_of(&session())["inputs"]
        .as_list()
        .expect("inputs")
        .iter()
        .find(|item| fields_of(item)["token"].as_text() == Some(token))
        .and_then(|item| fields_of(item)["um"].as_int())
        .unwrap_or_else(|| panic!("no input is named {token}"))
}

fn piece<'a>(pattern: &'a Pattern, name: &str) -> &'a sc_artifacts::DrawnPiece {
    pattern
        .pieces
        .iter()
        .find(|piece| piece.name == name)
        .unwrap_or_else(|| panic!("no piece is named {name}"))
}

const SEAMS_THAT_MATCH: [&str; 7] = [
    "side_right",
    "side_left",
    "under_right",
    "under_left",
    "underarm_front_right",
    "underarm_back_left",
    "cb",
];

#[test]
fn every_piece_the_book_lists_is_drafted_and_nothing_else() {
    let mut disk = MemoryRepository::new();
    let running = drawn(&mut disk);
    let pattern = running.pattern().expect("drawn");
    let names: Vec<&str> = pattern
        .pieces
        .iter()
        .map(|piece| piece.name.as_str())
        .collect();
    assert_eq!(names, PIECES);
    for name in PIECES {
        assert!(
            BOOK.contains(&format!("| `{name}` |")),
            "the book's piece table has no row for {name}"
        );
        let upper_and_under = u32::from(name.starts_with("collar")) + 1;
        assert_eq!(piece(pattern, name).quantity, upper_and_under, "{name}");
    }
}

#[test]
fn a_sleeve_cap_walks_to_its_declared_ease_and_the_finding_says_that_is_intended() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let walks = walked(&mut running);
    for hand in ["right", "left"] {
        for (seam, ease) in [
            (format!("cap_front_{hand}"), input("cap_ease_front")),
            (format!("cap_back_{hand}"), input("cap_ease_back")),
            (format!("shoulder_{hand}"), input("shoulder_ease")),
        ] {
            let report = &walks[&seam];
            assert_eq!(report.finding, "declared_ease", "{seam}");
            assert_eq!(report.ease, ease, "{seam}");
            assert!(
                report.unexplained.abs() <= WALK_TOLERANCE,
                "{seam}: {}",
                report.unexplained
            );
            assert!(
                (report.a - report.b - ease).abs() <= WALK_TOLERANCE,
                "{seam}"
            );
        }
    }
    for seam in SEAMS_THAT_MATCH {
        assert_eq!(walks[seam].finding, "matches", "{seam}");
    }
    assert_eq!(
        walks.len(),
        19,
        "nine seams on each side and the centre back"
    );
}

// The same cloth with nothing declared is a defect, which is the whole of the difference between ease and a mistake.
#[test]
fn the_same_difference_with_nothing_declared_is_a_defect_by_exactly_that_amount() {
    let (bare, draft) = opening();
    let mut changed = fields_of(&draft).clone();
    let seams: Vec<Value> = changed["seams"]
        .as_list()
        .expect("seams")
        .iter()
        .map(|seam| {
            let mut seam = fields_of(seam).clone();
            if seam["name"].as_text() == Some("cap_front_left") {
                seam.remove("ease");
            }
            Value::Object(seam)
        })
        .collect();
    changed.insert("seams".to_owned(), Value::List(seams));
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&bare, &mut disk).expect("opens");
    running.step(&Value::Object(changed)).expect("draws");
    let walks = walked(&mut running);
    let report = &walks["cap_front_left"];
    assert_eq!(report.finding, "defect");
    assert_eq!(report.ease, 0);
    assert!((report.unexplained - input("cap_ease_front")).abs() <= WALK_TOLERANCE);
    assert_eq!(walks["cap_front_right"].finding, "declared_ease");
}

#[test]
fn changing_the_declared_ease_changes_the_sleeve_and_not_the_bodice() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let before = running.pattern().expect("drawn").clone();
    let more = input("cap_ease_front") + 5000;
    step(
        &mut running,
        &format!(r#"{{"do":"change","token":"cap_ease_front","um":{more}}}"#),
    )
    .expect("changes");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    let after = running.pattern().expect("drawn").clone();
    for name in ["bodice_front", "bodice_back_right", "bodice_back_left"] {
        assert_eq!(piece(&before, name).sew, piece(&after, name).sew, "{name}");
    }
    assert_ne!(
        piece(&before, "sleeve_right").sew,
        piece(&after, "sleeve_right").sew
    );
    let walks = walked(&mut running);
    assert_eq!(walks["cap_front_right"].ease, more);
    assert_eq!(walks["cap_front_right"].finding, "declared_ease");
    assert_eq!(walks["cap_back_right"].finding, "declared_ease");
    assert_eq!(walks["underarm_front_right"].finding, "matches");
}

#[test]
fn a_fit_nothing_can_meet_is_refused_by_name_and_with_how_far_out_it_is() {
    let (bare, draft) = opening();
    let mut changed = fields_of(&draft).clone();
    changed.insert(
        "fits".to_owned(),
        parse(r#"[{"vary":"sleeve_front_width","seam":"cap_front_right","between":["notch_depth","double_spacing"]}]"#)
            .expect("valid"),
    );
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&bare, &mut disk).expect("opens");
    let refused = running
        .step(&Value::Object(changed.clone()))
        .expect_err("no such width");
    let said = fields_of(&refused.to_value())["message"]
        .as_text()
        .expect("text")
        .to_owned();
    assert!(
        said.contains("sleeve_front_width") && said.contains("cap_front_right"),
        "{said}"
    );

    for (fits, says) in [
        (
            r#"[{"vary":"nothing","seam":"cap_front_right","between":["sleeve_width_low","sleeve_width_high"]}]"#,
            "no input has that name",
        ),
        (
            r#"[{"vary":"sleeve_front_width","seam":"nothing","between":["sleeve_width_low","sleeve_width_high"]}]"#,
            "no seam has that name",
        ),
        (
            r#"[{"vary":"sleeve_front_width","seam":"cap_front_right","between":["sleeve_width_high","sleeve_width_low"]}]"#,
            "no range",
        ),
        (
            r#"[{"vary":"sleeve_front_width","seam":"cap_front_right"}]"#,
            "between which two lengths",
        ),
    ] {
        changed.insert("fits".to_owned(), parse(fits).expect("valid"));
        let mut disk = MemoryRepository::new();
        let mut running = Running::open(&bare, &mut disk).expect("opens");
        let refused = running
            .step(&Value::Object(changed.clone()))
            .expect_err(says);
        let said = fields_of(&refused.to_value())["message"]
            .as_text()
            .expect("text")
            .to_owned();
        assert!(said.contains(says), "{fits}: {said}");
    }
}

// The closures hold at the answer, not on the way to it: too narrow a sleeve is the recipe's own refusal.
#[test]
fn a_sleeve_too_narrow_for_the_arm_is_refused_by_the_closure_that_says_so() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    step(
        &mut running,
        r#"{"do":"change","token":"upper_arm_girth","um":400000}"#,
    )
    .expect("changes");
    let refused = step(&mut running, r#"{"do":"draft"}"#).expect_err("it cannot clear that arm");
    assert!(refused.contains("sleeve_clears_arm"), "{refused}");
}

#[derive(Clone, Copy)]
struct Size {
    name: &'static str,
    steps: i64,
}

// The chapter's size table: each size's change from M, per step.
const STEP_CHANGE: [(&str, i64); 15] = [
    ("bust_girth", 40_000),
    ("waist_girth", 40_000),
    ("nape_to_waist", 5_000),
    ("neck_girth", 10_000),
    ("shoulder_length", 3_000),
    ("across_back", 10_000),
    ("across_chest", 10_000),
    ("scye_depth", 5_000),
    ("waist_to_apex", 5_000),
    ("apex_from_cf", 2_500),
    ("upper_arm_girth", 15_000),
    ("wrist_girth", 5_000),
    ("sleeve_length", 5_000),
    ("collar_stand", 1_000),
    ("collar_fall", 2_000),
];
const SIZES: [Size; 5] = [
    Size {
        name: "XS",
        steps: -2,
    },
    Size {
        name: "S",
        steps: -1,
    },
    Size {
        name: "M",
        steps: 0,
    },
    Size {
        name: "L",
        steps: 1,
    },
    Size {
        name: "XL",
        steps: 2,
    },
];

fn assert_all_walk(running: &mut Running<&mut MemoryRepository>, case: &str) {
    for (seam, report) in &walked(running) {
        assert_ne!(
            report.finding, "defect",
            "{case}: {seam} out by {}",
            report.unexplained
        );
    }
}

fn crossing(line: &[sc_geometry::Point]) -> bool {
    let at = |index: usize| {
        let point = line[index % line.len()];
        (point.x() as f64, point.y() as f64)
    };
    for first in 0..line.len() {
        for second in first + 2..line.len() {
            if first == 0 && second == line.len() - 1 {
                continue;
            }
            let ((ax, ay), (bx, by)) = (at(first), at(first + 1));
            let ((cx, cy), (dx, dy)) = (at(second), at(second + 1));
            let side = |px: f64, py: f64, qx: f64, qy: f64, rx: f64, ry: f64| {
                (qx - px) * (ry - py) - (qy - py) * (rx - px)
            };
            if side(ax, ay, bx, by, cx, cy) * side(ax, ay, bx, by, dx, dy) < 0.0
                && side(cx, cy, dx, dy, ax, ay) * side(cx, cy, dx, dy, bx, by) < 0.0
            {
                return true;
            }
        }
    }
    false
}

#[test]
fn every_size_draws_closes_and_walks_to_what_it_declares() {
    for size in SIZES {
        let mut disk = MemoryRepository::new();
        let mut running = drawn(&mut disk);
        for (token, change) in STEP_CHANGE {
            let value = input(token) + change * size.steps;
            step(
                &mut running,
                &format!(r#"{{"do":"change","token":"{token}","um":{value}}}"#),
            )
            .unwrap_or_else(|message| panic!("{}: {token}: {message}", size.name));
        }
        step(&mut running, r#"{"do":"draft"}"#)
            .unwrap_or_else(|message| panic!("{}: {message}", size.name));
        assert_all_walk(&mut running, size.name);
        let known = bindings(&mut running);
        let walks = walked(&mut running);
        assert_eq!(
            walks["cap_front_right"].finding, "declared_ease",
            "{}",
            size.name
        );
        assert_eq!(
            walks["cap_back_right"].finding, "declared_ease",
            "{}",
            size.name
        );
        // The closures, read from the drawing rather than from the recipe that promised them.
        assert_eq!(
            fitted(&running, "sleeve_front_width") + fitted(&running, "sleeve_back_width"),
            known["sleeve_width"],
            "{}",
            size.name
        );
        assert!(
            known["sleeve_width"] >= known["garment_upper_arm"],
            "{}",
            size.name
        );
        let pattern = running.pattern().expect("drawn").clone();
        for name in PIECES {
            let drawn = piece(&pattern, name);
            assert!(
                !crossing(&drawn.cut),
                "{}: {name}: the cut line crosses itself",
                size.name
            );
            assert!(
                !crossing(&drawn.sew),
                "{}: {name}: the sew line crosses itself",
                size.name
            );
        }
        let front = piece(&pattern, "bodice_front");
        let closed: i64 = front
            .closures
            .iter()
            .filter(|closure| closure.name == "waist_dart")
            .map(|closure| closure.intake)
            .sum();
        assert_eq!(closed, known["waist_dart_front"], "{}", size.name);
        assert_eq!(
            known["front_waist_side"] - closed,
            known["quarter_waist"],
            "{}",
            size.name
        );
        // The stand and the fall are this size's, and the drawn collar is that deep at CB with its roll line that far up.
        let collar = piece(&pattern, "collar_right");
        assert!(
            collar
                .sew
                .iter()
                .any(|at| at.x() == 0 && at.y() == known["collar_depth"]),
            "{}",
            size.name
        );
        assert!(
            collar.internal.iter().any(|line| line.from.x() == 0
                && line.from.y() == input("collar_stand") + 1_000 * size.steps),
            "{}: the roll line starts at the stand's height",
            size.name
        );
        assert_eq!(
            known["collar_depth"],
            input("collar_stand") + input("collar_fall") + 3_000 * size.steps
        );
    }
}

// The same two lengths without the flattening that the walk uses: three hundred Gauss-Legendre panels over each control polygon, written here so that the check is not the thing it checks.
fn cubic_length(p: [(f64, f64); 4]) -> f64 {
    const NODES: [(f64, f64); 4] = [
        (-0.861_136_311_594_052_6, 0.347_854_845_137_453_85),
        (-0.339_981_043_584_856_26, 0.652_145_154_862_546_1),
        (0.339_981_043_584_856_26, 0.652_145_154_862_546_1),
        (0.861_136_311_594_052_6, 0.347_854_845_137_453_85),
    ];
    let mut total = 0.0;
    for panel in 0..300 {
        for (node, weight) in NODES {
            let t = (f64::from(panel) + (node + 1.0) / 2.0) / 300.0;
            let u = 1.0 - t;
            let d = |a: f64, b: f64, c: f64, e: f64| {
                3.0 * u * u * (b - a) + 6.0 * u * t * (c - b) + 3.0 * t * t * (e - c)
            };
            let dx = d(p[0].0, p[1].0, p[2].0, p[3].0);
            let dy = d(p[0].1, p[1].1, p[2].1, p[3].1);
            total += weight / 2.0 / 300.0 * (dx * dx + dy * dy).sqrt();
        }
    }
    total
}

#[test]
fn the_cap_is_longer_than_its_armhole_by_the_ease_measured_without_the_walk() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let known = bindings(&mut running);
    let v = |name: &str| known[name] as f64;
    let front_armhole = cubic_length([
        (v("front_bust_x"), v("front_underarm_y")),
        (v("arm_front_first_x"), v("front_underarm_y")),
        (v("arm_front_second_x"), v("arm_front_second_y")),
        (v("front_shoulder_x"), v("front_shoulder_y")),
    ]);
    let back_armhole = cubic_length([
        (v("back_bust_x"), v("back_underarm_y")),
        (v("arm_back_first_x"), v("back_underarm_y")),
        (v("arm_back_second_x"), v("arm_back_second_y")),
        (v("back_shoulder_x"), v("back_shoulder_y")),
    ]);
    let (front_width, back_width) = (
        fitted(&running, "sleeve_front_width") as f64,
        fitted(&running, "sleeve_back_width") as f64,
    );
    let front_cap = cubic_length([
        (front_width, 0.0),
        (v("cap_front_first_x"), 0.0),
        (v("cap_front_second_x"), v("cap_height")),
        (0.0, v("cap_height")),
    ]);
    let back_cap = cubic_length([
        (0.0, v("cap_height")),
        (-v("cap_back_first_x"), v("cap_height")),
        (-v("cap_back_second_x"), 0.0),
        (-back_width, 0.0),
    ]);
    // A chord is shorter than its arc by up to the flattening's bound on each side, which the walk's tolerance already allows.
    let tolerance = 2.0 * WALK_TOLERANCE as f64;
    assert!(
        (front_cap - front_armhole - input("cap_ease_front") as f64).abs() <= tolerance,
        "front: {front_cap} less {front_armhole}"
    );
    assert!(
        (back_cap - back_armhole - input("cap_ease_back") as f64).abs() <= tolerance,
        "back: {back_cap} less {back_armhole}"
    );
}

#[test]
fn the_fit_is_on_the_record_and_evaluate_shows_what_the_drawing_used() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let pattern = running.pattern().expect("drawn").clone();
    assert_eq!(pattern.fits.len(), 4);
    let known = bindings(&mut running);
    let sum: i64 = pattern
        .fits
        .iter()
        .filter(|fit| fit.vary.starts_with("sleeve"))
        .map(|fit| fit.value)
        .sum();
    assert_eq!(
        sum, known["sleeve_width"],
        "evaluate shows what the drawing used"
    );
    let again = from_canonical_text(&to_canonical_text(&pattern)).expect("reads");
    assert_eq!(again.fits, pattern.fits);
    // Asking what an unrelated change would do does not report the solved widths as changing.
    let preview = result(
        &step(
            &mut running,
            r#"{"do":"preview","token":"zip_len","um":340000}"#,
        )
        .expect("previews"),
    );
    let names: Vec<String> = fields_of(&preview)["changed"]
        .as_list()
        .expect("a list")
        .iter()
        .map(|binding| {
            fields_of(binding)["name"]
                .as_text()
                .expect("text")
                .to_owned()
        })
        .collect();
    assert!(
        !names.iter().any(|name| name.contains("sleeve")),
        "{names:?}"
    );
}

#[test]
fn the_notches_are_single_at_the_front_double_at_the_back_and_one_at_the_crown() {
    let mut disk = MemoryRepository::new();
    let running = drawn(&mut disk);
    let pattern = running.pattern().expect("drawn");
    let counted = |name: &str, kind: MarkKind| {
        piece(pattern, name)
            .marks
            .iter()
            .filter(|mark| mark.kind == kind)
            .count()
    };
    // side, armhole, one balance notch.
    assert_eq!(counted("bodice_front", MarkKind::VNotch), 3);
    assert_eq!(counted("bodice_front", MarkKind::Drill), 1);
    // side, the two of the double, zip, one balance notch.
    assert_eq!(counted("bodice_back_right", MarkKind::VNotch), 5);
    // front, the two of the double, crown, and a balance notch either side of it.
    assert_eq!(counted("sleeve_right", MarkKind::VNotch), 6);
    assert_eq!(counted("sleeve_left", MarkKind::VNotch), 6);
}

#[test]
fn a_double_notch_is_its_spacing_apart_on_both_sides_of_the_seam() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let listed = result(&step(&mut running, r#"{"do":"references"}"#).expect("lists"));
    let place = |piece: &str, carrier: &str| {
        fields_of(&listed)["references"]
            .as_list()
            .expect("a list")
            .iter()
            .map(fields_of)
            .find(|item| {
                item["piece"].as_text() == Some(piece) && item["carrier"].as_text() == Some(carrier)
            })
            .and_then(|item| item["point"].as_list().map(<[Value]>::to_vec))
            .map(|point| (point[0].as_int().expect("x"), point[1].as_int().expect("y")))
            .unwrap_or_else(|| panic!("{piece}: {carrier}"))
    };
    for (piece, name) in [
        ("bodice_back_right", "armhole_back"),
        ("sleeve_right", "back"),
    ] {
        let (a, b) = (
            place(piece, &format!("notch {name}, first")),
            place(piece, &format!("notch {name}, second")),
        );
        let apart = (((a.0 - b.0) as f64).powi(2) + ((a.1 - b.1) as f64).powi(2)).sqrt();
        // A chord of a curved 8 mm is a little under 8 mm.
        assert!((apart - 8000.0).abs() <= 25.0, "{piece}: {apart}");
    }
}

// Every edge of every physical copy is sewn along its whole length or named as free, which is what makes a sewing graph complete rather than merely consistent.
#[test]
fn every_edge_is_sewn_along_its_whole_length_or_named_free() {
    const FREE: [(&str, &str); 7] = [
        ("bodice_front", "waist"),
        ("bodice_front", "cf"),
        ("bodice_back", "waist"),
        ("sleeve", "wrist"),
        ("collar", "front_edge"),
        ("collar", "outer"),
        ("collar", "cb"),
    ];
    let (_, draft) = opening();
    let draft = fields_of(&draft);
    let seams = draft["seams"].as_list().expect("seams");
    let sewn = |piece: &str, edge: &str| {
        seams.iter().map(fields_of).any(|seam| {
            ["a", "b"].iter().any(|side| {
                let side = fields_of(&seam[*side]);
                side["piece"].as_text() == Some(piece)
                    && [&side["from"], &side["to"]]
                        .iter()
                        .any(|end| fields_of(end)["edge"].as_text() == Some(edge))
            })
        })
    };
    for item in draft["pieces"].as_list().expect("pieces") {
        let item = fields_of(item);
        let name = item["name"].as_text().expect("a name");
        let source = item
            .get("mirror_of")
            .and_then(Value::as_text)
            .unwrap_or(name);
        let original = draft["pieces"]
            .as_list()
            .expect("pieces")
            .iter()
            .map(fields_of)
            .find(|piece| piece["name"].as_text() == Some(source))
            .expect("the piece it mirrors");
        for edge in original["edges"].as_list().expect("edges") {
            let edge = fields_of(edge)["name"].as_text().expect("a name");
            let family = name.trim_end_matches("_right").trim_end_matches("_left");
            let free = FREE.contains(&(family, edge));
            assert_eq!(sewn(name, edge), !free, "{name}: {edge}");
            assert!(
                !free || BOOK.contains(&format!("`{family}` | `{edge}`")),
                "the book does not say {family}'s {edge} is free"
            );
        }
    }
    // Along the whole length: the two stretches of each armhole add up to the armhole, and to the cap less its ease.
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let known = bindings(&mut running);
    let walks = walked(&mut running);
    let front_armhole = cubic_length([
        (
            known["front_bust_x"] as f64,
            known["front_underarm_y"] as f64,
        ),
        (
            known["arm_front_first_x"] as f64,
            known["front_underarm_y"] as f64,
        ),
        (
            known["arm_front_second_x"] as f64,
            known["arm_front_second_y"] as f64,
        ),
        (
            known["front_shoulder_x"] as f64,
            known["front_shoulder_y"] as f64,
        ),
    ]);
    let span = walks["underarm_front_right"].b + walks["cap_front_right"].b;
    assert!(
        (span as f64 - front_armhole).abs() <= 2.0 * WALK_TOLERANCE as f64,
        "{span} of {front_armhole}"
    );
    assert!((walks["underarm_front_right"].b - input("under_span_front")).abs() <= WALK_TOLERANCE);
}

#[test]
fn closing_the_bust_dart_is_what_makes_the_side_seams_meet() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let walks = walked(&mut running);
    assert_eq!(walks["side_right"].finding, "matches");
    let known = bindings(&mut running);
    assert_eq!(
        known["front_underarm_y"] - known["back_underarm_y"],
        input("bust_dart_intake"),
        "the front's side is longer by exactly the dart"
    );
    let reports = result(&step(&mut running, r#"{"do":"closures"}"#).expect("closures"));
    let bust = reports
        .as_list()
        .expect("reports")
        .iter()
        .map(fields_of)
        .find(|report| report["name"].as_text() == Some("bust_dart"))
        .expect("the bust dart");
    assert_eq!(bust["intake"].as_int(), Some(input("bust_dart_intake")));
    assert_eq!(bust["gap_after_closing"].as_int(), Some(0));
}

#[test]
fn an_edit_to_the_armhole_keeps_the_ease_and_a_delete_makes_the_fit_wait() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"bodice_front","op":"split","edge":"armhole","at":"7/9","into":["armhole_low","armhole_high"]}"#,
    )
    .expect("splits");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    let walks = walked(&mut running);
    assert_eq!(walks["cap_front_right"].finding, "declared_ease");
    assert_eq!(walks["underarm_front_right"].finding, "matches");

    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"bodice_back_right","op":"delete","edge":"armhole","bridge":"straight"}"#,
    )
    .expect("deletes");
    step(&mut running, r#"{"do":"draft"}"#).expect("still draws");
    let pattern = running.pattern().expect("drawn").clone();
    assert!(
        pattern
            .repairs
            .iter()
            .any(|repair| repair.carrier == "a fit on seam cap_back_right"),
        "{:?}",
        pattern.repairs
    );
    assert!(pattern
        .repairs
        .iter()
        .any(|repair| repair.carrier.starts_with("notch armhole_back, first")));
    let refused = step(
        &mut running,
        r#"{"do":"export_pattern","format":"dxf","semantics":"cut_as_1","path":"exports/b.dxf"}"#,
    )
    .expect_err("not released");
    assert!(refused.contains("need repair"), "{refused}");
    // A double notch is one thing to repair, though its two marks each answer for themselves.
    step(
        &mut running,
        r#"{"do":"repair_reference","piece":"bodice_back_right","notch":"armhole_back","edge":"straight","at":"1/2"}"#,
    )
    .expect("repairs");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    assert!(!running
        .pattern()
        .expect("drawn")
        .repairs
        .iter()
        .any(|repair| repair.carrier.starts_with("notch armhole_back")));
    step(&mut running, r#"{"do":"undo"}"#).expect("undoes the repair");
    step(&mut running, r#"{"do":"undo"}"#).expect("undoes the delete");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    assert!(running.pattern().expect("drawn").repairs.is_empty());
}

#[test]
fn a_seam_end_held_back_from_an_edges_end_can_be_repaired_to_somewhere_else() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"sleeve_right","op":"delete","edge":"cap_back","bridge":"patch"}"#,
    )
    .expect("deletes");
    step(
        &mut running,
        r#"{"do":"repair_reference","seam":"cap_back_right","side":"a","end":"from","edge":"patch","at":"1/2"}"#,
    )
    .expect("a before_end gives way to the place it is told");
    let listed = result(&step(&mut running, r#"{"do":"references"}"#).expect("lists"));
    let now = fields_of(&listed)["references"]
        .as_list()
        .expect("a list")
        .iter()
        .map(fields_of)
        .find(|item| item["carrier"].as_text() == Some("seam cap_back_right, side a, from"))
        .expect("the end")["now"]
        .as_text()
        .expect("text")
        .to_owned();
    assert!(now.starts_with("patch at"), "{now}");
}

#[test]
fn a_design_saved_and_opened_again_draws_the_same_bytes_by_solving_again() {
    let mut disk = MemoryRepository::new();
    replay(&session(), &mut disk).expect("replays");
    let (bare, _) = opening();
    let mut running = Running::open(&bare, &mut disk).expect("opens");
    step(&mut running, r#"{"do":"reopen"}"#).expect("reopens");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws what was saved");
    let pattern = running.pattern().expect("drawn");
    assert_eq!(to_canonical_text(pattern), GOLDEN_CANONICAL);
}

// Each measurement moved a long way on its own. It closes and walks, or the recipe refuses it by the name of the closure it broke, and nothing in between is a drawing with a seam that does not meet.
#[test]
fn a_changed_measurement_closes_and_walks_or_is_refused_by_the_closure_it_breaks() {
    const CLOSURES: [&str; 2] = ["sleeve_clears_arm", "no value of"];
    let mut refused = 0;
    for (token, _) in STEP_CHANGE {
        for change in [-20_000, 20_000] {
            let case = format!("{token} by {change}");
            let mut disk = MemoryRepository::new();
            let mut running = drawn(&mut disk);
            let value = input(token) + change;
            step(
                &mut running,
                &format!(r#"{{"do":"change","token":"{token}","um":{value}}}"#),
            )
            .unwrap_or_else(|message| panic!("{case}: {message}"));
            match step(&mut running, r#"{"do":"draft"}"#) {
                Ok(_) => {
                    assert_all_walk(&mut running, &case);
                    let reports =
                        result(&step(&mut running, r#"{"do":"closures"}"#).expect("closures"));
                    for report in reports.as_list().expect("reports").iter().map(fields_of) {
                        assert_eq!(
                            report["gap_after_closing"].as_int(),
                            Some(0),
                            "{case}: {report:?}"
                        );
                    }
                }
                Err(message) => {
                    assert!(
                        CLOSURES.iter().any(|closure| message.contains(closure)),
                        "{case}: {message}"
                    );
                    refused += 1;
                }
            }
        }
    }
    assert!(refused < 4, "{refused} of these should still draw");
}

#[test]
fn a_notch_type_that_comes_later_is_refused_by_name_and_a_drill_outside_the_piece_is_too() {
    let (bare, draft) = opening();
    for (replacement, says) in [
        (
            r#"{"name":"odd","type":"castle","edge":"cf","at":"1/2","depth":"notch_depth"}"#,
            "single, double or drill",
        ),
        (
            r#"{"name":"odd","type":"drill","edge":"cf","at":"1/2","inset":"nape_to_waist"}"#,
            "outside the piece",
        ),
        (
            r#"{"name":"odd","type":"double","edge":"cf","at":"1/2","depth":"notch_depth"}"#,
            "needs spacing",
        ),
        (
            r#"{"name":"odd","type":"double","spacing":"nape_to_waist","edge":"cf","at":"1/2","depth":"notch_depth"}"#,
            "reaches",
        ),
    ] {
        let mut changed = fields_of(&draft).clone();
        let mut pieces = changed["pieces"].as_list().expect("pieces").to_vec();
        let mut front = fields_of(&pieces[0]).clone();
        let mut notches = front["notches"].as_list().expect("notches").to_vec();
        notches.push(parse(replacement).expect("valid"));
        front.insert("notches".to_owned(), Value::List(notches));
        pieces[0] = Value::Object(front);
        changed.insert("pieces".to_owned(), Value::List(pieces));
        let mut disk = MemoryRepository::new();
        let mut running = Running::open(&bare, &mut disk).expect("opens");
        let refused = running.step(&Value::Object(changed)).expect_err(says);
        let said = fields_of(&refused.to_value())["message"]
            .as_text()
            .expect("text")
            .to_owned();
        assert!(said.contains(says), "{replacement}: {said}");
    }
}

#[test]
fn the_export_equals_its_frozen_goldens_byte_for_byte_and_replays_the_same() {
    let run = || {
        let mut disk = MemoryRepository::new();
        replay(&session(), &mut disk).expect("replays");
        disk
    };
    let (first, second) = (run(), run());
    let file = |disk: &MemoryRepository, name: &str| {
        disk.read(&FilePath::new(name).expect("a path"))
            .expect("reads")
            .unwrap_or_else(|| panic!("{name} was not exported"))
    };
    for name in [
        "exports/reference-bodice.canonical.json",
        "exports/reference-bodice.dxf",
        "exports/reference-bodice.pdf",
    ] {
        assert_eq!(file(&first, name), file(&second, name), "{name}");
    }
    assert_eq!(
        file(&first, "exports/reference-bodice.canonical.json"),
        GOLDEN_CANONICAL.as_bytes()
    );
    assert_eq!(
        file(&first, "exports/reference-bodice.dxf"),
        GOLDEN_DXF.as_bytes()
    );
    // The written files, read back by the same readers that read the skirt's.
    let pattern = from_canonical_text(GOLDEN_CANONICAL).expect("reads");
    let found = judge_dxf(GOLDEN_DXF, &pattern, true);
    assert!(found.is_empty(), "{found:?}");
    let found = judge_pdf_square(&file(&first, "exports/reference-bodice.pdf"));
    assert!(found.is_empty(), "{found:?}");
}

// Every value the chapter states, against what the recipe evaluates to: a table the test reads, not a copy of it.
#[test]
fn every_value_the_book_states_is_what_the_recipe_evaluates_to() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let mut known = bindings(&mut running);
    for item in fields_of(&session())["inputs"].as_list().expect("inputs") {
        let item = fields_of(item);
        known.insert(
            item["token"].as_text().expect("a token").to_owned(),
            item["um"].as_int().expect("a value"),
        );
    }
    for fit in &running.pattern().expect("drawn").fits {
        known.insert(fit.vary.clone(), fit.value);
    }
    let mut checked = 0;
    for line in BOOK.lines() {
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        let [_, token, value, ..] = cells.as_slice() else {
            continue;
        };
        let (Some(token), Some(value)) = (
            token
                .strip_prefix('`')
                .and_then(|token| token.strip_suffix('`')),
            value.strip_suffix(" mm"),
        ) else {
            continue;
        };
        let Ok(stated) = value.parse::<f64>() else {
            continue;
        };
        let actual = known
            .get(token)
            .copied()
            .unwrap_or_else(|| panic!("the book states {token}, and nothing has that name"));
        assert!(
            (stated * 1000.0 - actual as f64).abs() < 0.5,
            "{token}: the book says {value} mm, and it is {actual} um"
        );
        checked += 1;
    }
    assert!(checked >= 60, "only {checked} values were checked");
}

// The size table in the chapter is the table this test draws: the two are not allowed to drift.
#[test]
fn the_sizes_drawn_are_the_sizes_the_book_lists() {
    for (token, change) in STEP_CHANGE {
        let whole = change / 1000;
        let shown = if change % 1000 == 0 {
            format!("| `{token}` | {whole} mm change |")
        } else {
            format!(
                "| `{token}` | {}.{} mm change |",
                whole,
                (change % 1000) / 100
            )
        };
        assert!(BOOK.contains(&shown), "the book has no row: {shown}");
    }
    for size in SIZES {
        let steps = match size.steps {
            -2 => "−2".to_owned(),
            -1 => "−1".to_owned(),
            other => other.to_string(),
        };
        assert!(
            BOOK.contains(&format!("| {} | {steps} |", size.name)),
            "{}",
            size.name
        );
    }
}

fn sewn_along(line: &[sc_geometry::Point], from: (i64, i64), to: (i64, i64)) -> f64 {
    let at = |x: i64, y: i64| sc_geometry::Point::new(x, y).expect("in range");
    sc_artifacts::stretch_length(
        &sc_artifacts::stretch(line, at(from.0, from.1), at(to.0, to.1)).expect("on the line"),
    )
}

// The collar's two edges are concentric arcs, so that they differ by what the recipe says, at any size and any roll.
#[test]
fn a_collars_outer_edge_is_longer_than_its_neck_edge_by_the_declared_roll() {
    for (roll, case) in [
        (input("collar_roll"), "as declared"),
        (25_000, "a flatter collar"),
    ] {
        let mut disk = MemoryRepository::new();
        let mut running = drawn(&mut disk);
        step(
            &mut running,
            &format!(r#"{{"do":"change","token":"collar_roll","um":{roll}}}"#),
        )
        .expect("changes");
        step(&mut running, r#"{"do":"draft"}"#).expect("draws");
        let known = bindings(&mut running);
        let collar = piece(running.pattern().expect("drawn"), "collar_right").clone();
        let neck_end = (known["collar_neck_end_x"], known["collar_neck_end_y"]);
        let outer_end = (known["collar_outer_end_x"], known["collar_outer_end_y"]);
        let neck = sewn_along(&collar.sew, (0, 0), neck_end);
        let outer = sewn_along(&collar.sew, outer_end, (0, known["collar_depth"]));
        assert!(
            (outer - neck - roll as f64).abs() <= WALK_TOLERANCE as f64,
            "{case}: {outer} less {neck}, and {roll} declared"
        );
        let walks = walked(&mut running);
        for seam in [
            "collar_back_right",
            "collar_front_right",
            "collar_back_left",
            "collar_front_left",
        ] {
            assert_eq!(walks[seam].finding, "matches", "{case}: {seam}");
            assert!(
                walks[seam].unexplained.abs() <= WALK_TOLERANCE,
                "{case}: {seam}"
            );
        }
    }
}

// A roll line folded on and never cut is what the glossary marks as a hazard, so it is held in the written file, and on the arc it is meant to be.
#[test]
fn the_roll_line_is_on_the_internal_layer_and_never_on_the_cut_line() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let known = bindings(&mut running);
    let collar = piece(running.pattern().expect("drawn"), "collar_right").clone();
    assert!(collar.internal.len() >= 2, "an arc is drawn as short lines");
    let centre_y = -(known["collar_radius"] as f64);
    let radius = known["collar_roll_radius"] as f64;
    for line in &collar.internal {
        for at in [line.from, line.to] {
            let away = ((at.x() as f64).powi(2) + (at.y() as f64 - centre_y).powi(2)).sqrt();
            assert!(
                (away - radius).abs() <= 20.0,
                "{at:?} is {away} from the centre, not {radius}"
            );
            assert!(!collar.cut.contains(&at), "{at:?} is on the cut line");
        }
    }
    // Read from the file a receiver gets, by the layer each line is written on.
    let mut layer = String::new();
    let mut block = String::new();
    let mut entity = String::new();
    let mut on_cut = Vec::new();
    let mut on_internal = Vec::new();
    let mut x = 0.0;
    let lines: Vec<&str> = GOLDEN_DXF.lines().collect();
    for pair in lines.chunks(2) {
        let (code, value) = (pair[0].trim(), pair[1].trim());
        match (code, entity.as_str()) {
            ("0", _) => value.clone_into(&mut entity),
            ("2", "BLOCK") => value.clone_into(&mut block),
            ("8", "POLYLINE") => value.clone_into(&mut layer),
            ("10", "VERTEX") => x = value.parse().expect("a number"),
            ("20", "VERTEX") if block == "collar_right_M" => {
                let y: f64 = value.parse().expect("a number");
                let point = (
                    (x * 1000.0_f64).round() as i64,
                    (y * 1000.0_f64).round() as i64,
                );
                match layer.as_str() {
                    "CUT" => on_cut.push(point),
                    "INTCUT" => on_internal.push(point),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    assert!(!on_cut.is_empty(), "the block has a cut line");
    for line in &collar.internal {
        let start = (line.from.x(), line.from.y());
        assert!(on_internal.contains(&start), "{start:?} is not on INTCUT");
        assert!(!on_cut.contains(&start), "{start:?} is on CUT");
    }
}

#[test]
fn a_collar_that_was_not_fitted_is_a_defect_at_the_neck_by_how_much_it_is_out() {
    let (bare, draft) = opening();
    let mut changed = fields_of(&draft).clone();
    let kept: Vec<Value> = changed["fits"]
        .as_list()
        .expect("fits")
        .iter()
        .filter(|fit| {
            fields_of(fit)["vary"]
                .as_text()
                .is_some_and(|vary| vary.starts_with("sleeve"))
        })
        .cloned()
        .collect();
    changed.insert("fits".to_owned(), Value::List(kept));
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&bare, &mut disk).expect("opens");
    running.step(&Value::Object(changed)).expect("draws");
    let walks = walked(&mut running);
    // What the session stores for these two is only what they hold until a fit solves them, and it is not the neck's length.
    assert_eq!(walks["collar_back_right"].finding, "defect");
    assert_eq!(walks["collar_front_right"].finding, "defect");
    assert!(
        (walks["collar_back_right"].unexplained
            - (input("collar_back_span") - walks["collar_back_right"].b))
            .abs()
            <= WALK_TOLERANCE
    );
}

#[test]
fn a_collar_with_no_roll_is_refused_rather_than_drawn_straight() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    step(
        &mut running,
        r#"{"do":"change","token":"collar_roll","um":0}"#,
    )
    .expect("changes");
    let refused = step(&mut running, r#"{"do":"draft"}"#).expect_err("no such arc");
    assert!(refused.contains("collar_radius"), "{refused}");
}

#[test]
fn an_edit_to_the_neck_keeps_the_collar_on_it() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"bodice_front","op":"split","edge":"neck","at":"2/5","into":["neck_low","neck_high"]}"#,
    )
    .expect("splits");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    let walks = walked(&mut running);
    assert_eq!(walks["collar_front_right"].finding, "matches");
    assert_eq!(walks["collar_back_right"].finding, "matches");
}
