//! The reference trousers: a crotch curve, an inseam two pieces share, a pocket that is an object, and a hole with no length of its own.
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

const SESSION: &str = include_str!("../../../conformance/reference-trousers/session.json");
const BOOK: &str = include_str!("../../../docs/book/src/spec/reference-trousers.md");
const GOLDEN_CANONICAL: &str = include_str!(
    "../../../conformance/goldens/reference-trousers/reference-trousers.canonical.json"
);
const GOLDEN_DXF: &str =
    include_str!("../../../conformance/goldens/reference-trousers/reference-trousers.dxf");
// Twice T2: two lines each within T2 of one curve can differ by this much.
const WALK_TOLERANCE: i64 = 20;
const PIECES: [(&str, u32); 7] = [
    ("trouser_front_right", 1),
    ("trouser_front_left", 1),
    ("trouser_back_right", 1),
    ("trouser_back_left", 1),
    ("waistband", 1),
    ("pocket_bag_right", 2),
    ("pocket_bag_left", 2),
];

fn fields_of(value: &Value) -> &BTreeMap<String, Value> {
    value.as_object().expect("an object")
}

fn session() -> Value {
    parse(SESSION).expect("valid")
}

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

fn refused_by(changed: BTreeMap<String, Value>) -> String {
    let (bare, _) = opening();
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&bare, &mut disk).expect("opens");
    let failure = running.step(&Value::Object(changed)).expect_err("refused");
    fields_of(&failure.to_value())["message"]
        .as_text()
        .expect("text")
        .to_owned()
}

struct Walked {
    b: i64,
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
                    b: report["b_length"].as_int().expect("a length"),
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

fn assert_all_walk(running: &mut Running<&mut MemoryRepository>, case: &str) {
    for (seam, report) in &walked(running) {
        assert_eq!(
            report.finding, "matches",
            "{case}: {seam} out by {}",
            report.unexplained
        );
    }
    let reports = result(&step(running, r#"{"do":"closures"}"#).expect("closures"));
    for report in reports.as_list().expect("reports").iter().map(fields_of) {
        // A dart on a slanted waist has its two ends rounded to the micrometre, so its legs can differ by that.
        assert!(
            report["gap_after_closing"]
                .as_int()
                .expect("a length")
                .abs()
                <= 1,
            "{case}: {report:?}"
        );
        assert!(
            matches!(report["conserved"], Value::Bool(true)),
            "{case}: {report:?}"
        );
    }
}

#[test]
fn every_piece_the_book_lists_is_drafted_with_its_label_and_nothing_else() {
    let mut disk = MemoryRepository::new();
    let running = drawn(&mut disk);
    let pattern = running.pattern().expect("drawn");
    let names: Vec<&str> = pattern
        .pieces
        .iter()
        .map(|piece| piece.name.as_str())
        .collect();
    let wanted: Vec<&str> = PIECES.iter().map(|(name, _)| *name).collect();
    assert_eq!(names, wanted);
    for (name, quantity) in PIECES {
        assert!(
            BOOK.contains(&format!("| `{name}` | {quantity} |")),
            "the book has no row for {name}"
        );
        let drawn = piece(pattern, name);
        assert_eq!(drawn.quantity, quantity, "{name}");
        assert!(
            drawn.labels.iter().any(|label| label
                .text
                .starts_with(&format!("{name}, size M, cut {quantity}"))),
            "{name}: {:?}",
            drawn.labels
        );
    }
}

#[test]
fn every_seam_meets_with_the_darts_sewn_away() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    assert_all_walk(&mut running, "as drawn");
    assert_eq!(
        walked(&mut running).len(),
        14,
        "four on each side, two rises and four at the band"
    );
    let known = bindings(&mut running);
    let walks = walked(&mut running);
    for seam in [
        "band_back_left",
        "band_front_left",
        "band_front_right",
        "band_back_right",
    ] {
        assert!(
            (walks[seam].b - known["quarter_waist"]).abs() <= WALK_TOLERANCE,
            "{seam}: {}",
            walks[seam].b
        );
    }
    // The four quarters of the band, measured on the drawn line, come to the garment's waist.
    let total: i64 = [
        "band_back_left",
        "band_front_left",
        "band_front_right",
        "band_back_right",
    ]
    .iter()
    .map(|seam| walks[*seam].b)
    .sum();
    assert!(
        (total - known["garment_waist"]).abs() <= 4 * WALK_TOLERANCE,
        "{total}"
    );
}

#[test]
fn the_inseam_is_fitted_and_left_unfitted_it_is_a_defect() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let fit = running.pattern().expect("drawn").fits[0].clone();
    assert_eq!(fit.vary, "back_crotch_drop");
    assert!(
        fit.value > 0,
        "the back's crotch point is below the front's"
    );
    assert_ne!(
        fit.value,
        input("back_crotch_drop"),
        "what the session stores is not the answer"
    );
    let known = bindings(&mut running);
    assert_eq!(known["back_crotch_y"], input("crotch_depth") + fit.value);

    let (bare, draft) = opening();
    let mut changed = fields_of(&draft).clone();
    changed.remove("fits");
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&bare, &mut disk).expect("opens");
    running.step(&Value::Object(changed)).expect("draws");
    let report = &walked(&mut running)["inseam_right"];
    assert_eq!(report.finding, "defect");
}

#[test]
fn a_pocket_is_an_object_on_the_side_seam_with_its_pieces() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let pattern = running.pattern().expect("drawn").clone();
    assert_eq!(pattern.pockets.len(), 2);
    for (name, side, bag) in [
        (
            "side_pocket_right",
            "trouser_front_right",
            "pocket_bag_right",
        ),
        ("side_pocket_left", "trouser_front_left", "pocket_bag_left"),
    ] {
        let pocket = pattern
            .pockets
            .iter()
            .find(|pocket| pocket.name == name)
            .expect("the pocket");
        assert_eq!(pocket.piece, side);
        assert_eq!(pocket.opening, "in_seam");
        assert_eq!(pocket.pieces, [bag]);
        // Its orientation: it opens downward from near the waist, on the sew line of the piece it serves.
        assert!(pocket.from.y() > pocket.to.y(), "{name}");
        assert!(
            pocket.from.y() < 0 && pocket.to.y() > -input("waist_to_hip") - 1000,
            "{name}: {pocket:?}"
        );
        let opened = sc_artifacts::stretch_length(
            &sc_artifacts::stretch(&piece(&pattern, side).sew, pocket.from, pocket.to)
                .expect("on the sew line"),
        );
        assert!(
            (opened - input("pocket_opening") as f64).abs() <= WALK_TOLERANCE as f64,
            "{name}: {opened}"
        );
    }
    // Mirrored, on the other side of the body.
    assert_eq!(pattern.pockets[0].from.x(), -pattern.pockets[1].from.x());
    let again = from_canonical_text(&to_canonical_text(&pattern)).expect("reads");
    assert_eq!(again.pockets, pattern.pockets);
    // Each layer of the bag is sewn to the seam along exactly the opening.
    let walks = walked(&mut running);
    for seam in [
        "pocket_front_right",
        "pocket_back_right",
        "pocket_front_left",
        "pocket_back_left",
    ] {
        assert_eq!(walks[seam].finding, "matches", "{seam}");
    }
}

#[test]
fn a_pocket_that_cannot_be_built_is_refused_by_name() {
    let (_, draft) = opening();
    for (pocket, says) in [
        (
            r#"{"name":"p","piece":"trouser_front_right","edge":"side_upper","along":"pocket_from_waist","length":"pocket_opening","opening":"welt","pieces":["pocket_bag_right"]}"#,
            "in_seam is the opening",
        ),
        (
            r#"{"name":"p","piece":"trouser_front_right","edge":"side_upper","along":"pocket_from_waist","length":"pocket_opening","opening":"in_seam","pieces":["nothing"]}"#,
            "no piece has that name",
        ),
        (
            r#"{"name":"p","piece":"trouser_front_right","edge":"side_upper","along":"pocket_from_waist","length":"pocket_opening","opening":"in_seam","pieces":[]}"#,
            "made of nothing",
        ),
        (
            r#"{"name":"p","piece":"nothing","edge":"side_upper","along":"pocket_from_waist","length":"pocket_opening","opening":"in_seam","pieces":["pocket_bag_right"]}"#,
            "no piece has that name",
        ),
        (
            r#"{"name":"p","piece":"trouser_front_right","edge":"nothing","along":"pocket_from_waist","length":"pocket_opening","opening":"in_seam","pieces":["pocket_bag_right"]}"#,
            "no edge is named",
        ),
        (
            r#"{"name":"p","piece":"trouser_front_right","edge":"side_upper","along":"pocket_from_waist","length":"waist_to_hem","opening":"in_seam","pieces":["pocket_bag_right"]}"#,
            "reaches",
        ),
    ] {
        let mut changed = fields_of(&draft).clone();
        changed.insert(
            "pockets".to_owned(),
            Value::List(vec![parse(pocket).expect("valid")]),
        );
        let said = refused_by(changed);
        assert!(said.contains(says), "{pocket}: {said}");
    }
}

#[test]
fn an_edit_keeps_a_pocket_on_the_same_cloth_and_a_delete_makes_it_a_repair() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let before = running.pattern().expect("drawn").pockets.clone();
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"trouser_front_right","op":"split","edge":"side_upper","at":"3/7","into":["side_a","side_b"]}"#,
    )
    .expect("splits");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    let after = running.pattern().expect("drawn").pockets.clone();
    assert_eq!(before.len(), after.len());
    for (was, now) in before.iter().zip(&after) {
        let away = |a: sc_geometry::Point, b: sc_geometry::Point| {
            (a.x() - b.x()).abs() + (a.y() - b.y()).abs()
        };
        assert!(
            away(was.from, now.from) <= 2 * WALK_TOLERANCE,
            "{was:?} to {now:?}"
        );
        assert!(
            away(was.to, now.to) <= 2 * WALK_TOLERANCE,
            "{was:?} to {now:?}"
        );
    }
}

#[test]
fn a_deleted_side_seam_leaves_the_pocket_as_a_repair_and_blocks_a_cutting_file() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    step(
        &mut running,
        r#"{"do":"edit_edge","piece":"trouser_front_right","op":"delete","edge":"side_upper","bridge":"patch"}"#,
    )
    .expect("deletes");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    let pattern = running.pattern().expect("drawn").clone();
    assert!(
        pattern
            .repairs
            .iter()
            .any(|repair| repair.carrier == "pocket side_pocket_right, from"),
        "{:?}",
        pattern.repairs
    );
    assert!(!pattern
        .pockets
        .iter()
        .any(|pocket| pocket.name == "side_pocket_right"));
    let refused = step(
        &mut running,
        r#"{"do":"export_pattern","format":"pdf","layout":"a0_tiled","path":"exports/t.pdf"}"#,
    )
    .expect_err("not released");
    assert!(refused.contains("need repair"), "{refused}");
}

// The mutation the leaf asks for: change the button, and the hole changes with no other change anywhere.
#[test]
fn editing_the_button_changes_the_hole_and_nothing_can_set_it_separately() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let before = running.pattern().expect("drawn").buttons[0].clone();
    assert_eq!(
        before.hole_length,
        input("button_diameter") + input("button_thickness") + input("hole_clearance")
    );
    let bigger = input("button_diameter") + 3000;
    step(
        &mut running,
        &format!(r#"{{"do":"change","token":"button_diameter","um":{bigger}}}"#),
    )
    .expect("changes");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    let after = running.pattern().expect("drawn").buttons[0].clone();
    assert_eq!(after.size, bigger);
    assert_eq!(after.hole_length, before.hole_length + 3000);
    assert_eq!(after.hole_from, before.hole_from);
    assert_eq!(
        after.hole_to.x() - after.hole_from.x(),
        after.hole_length,
        "the drawn line is that long"
    );
    assert_eq!(after.button_at, before.button_at);
    let band = piece(running.pattern().expect("drawn"), "waistband");
    assert!(
        band.internal
            .iter()
            .any(|line| line.from == after.hole_from && line.to == after.hole_to),
        "the hole is drawn"
    );
    assert!(
        band.marks
            .iter()
            .any(|mark| mark.kind == MarkKind::Drill && mark.at == after.button_at),
        "the button is marked"
    );

    // A button too big for the band's extension is the recipe's own refusal.
    step(
        &mut running,
        r#"{"do":"change","token":"button_diameter","um":40000}"#,
    )
    .expect("changes");
    let refused = step(&mut running, r#"{"do":"draft"}"#).expect_err("no room");
    assert!(refused.contains("button_on_extension"), "{refused}");

    let (_, draft) = opening();
    for (button, says) in [
        (
            r#"{"name":"b","size":"button_diameter","policy":[],"hole_length":"zip_len","button":{"piece":"waistband","at":["button_x","hole_y"]},"hole":{"piece":"waistband","at":["hole_from_end","hole_y"],"toward":["band_length","hole_y"]}}"#,
            "comes from its button",
        ),
        (
            r#"{"name":"b","size":"button_diameter","policy":[],"button":{"piece":"waistband","at":["button_x","hole_y"]},"hole":{"piece":"waistband","length":"zip_len","at":["hole_from_end","hole_y"],"toward":["band_length","hole_y"]}}"#,
            "comes from its button",
        ),
        (
            r#"{"name":"b","size":"button_diameter","policy":[],"button":{"piece":"waistband","at":["button_x","waist_to_hip"]},"hole":{"piece":"waistband","at":["hole_from_end","hole_y"],"toward":["band_length","hole_y"]}}"#,
            "outside the sew line",
        ),
        (
            r#"{"name":"b","size":"zip_len","policy":[],"button":{"piece":"waistband","at":["button_x","hole_y"]},"hole":{"piece":"waistband","at":["hole_from_end","hole_y"],"toward":["zero","hole_y"]}}"#,
            "outside the sew line",
        ),
        (
            r#"{"name":"b","size":"zero","policy":[],"button":{"piece":"waistband","at":["button_x","hole_y"]},"hole":{"piece":"waistband","at":["hole_from_end","hole_y"],"toward":["band_length","hole_y"]}}"#,
            "comes to a size",
        ),
        (
            r#"{"name":"b","size":"button_diameter","policy":[],"button":{"piece":"nothing","at":["button_x","hole_y"]},"hole":{"piece":"waistband","at":["hole_from_end","hole_y"],"toward":["band_length","hole_y"]}}"#,
            "no piece has that name",
        ),
    ] {
        let mut changed = fields_of(&draft).clone();
        changed.insert(
            "buttons".to_owned(),
            Value::List(vec![parse(button).expect("valid")]),
        );
        let said = refused_by(changed);
        assert!(said.contains(says), "{button}: {said}");
    }
}

// The button has to land under the hole: the band's end overlaps its extension by exactly the waist.
#[test]
fn the_button_lands_at_the_end_of_the_hole_when_the_band_is_closed() {
    let mut disk = MemoryRepository::new();
    let mut running = drawn(&mut disk);
    let known = bindings(&mut running);
    let pair = running.pattern().expect("drawn").buttons[0].clone();
    assert_eq!(
        pair.button_at.x() - known["garment_waist"],
        pair.hole_from.x()
    );
    assert_eq!(pair.button_at.y(), pair.hole_from.y());
}

const STEP_CHANGE: [(&str, i64); 8] = [
    ("waist_girth", 40_000),
    ("hip_girth", 40_000),
    ("waist_to_hip", 5_000),
    ("crotch_depth", 5_000),
    ("waist_to_hem", 10_000),
    ("thigh_girth", 20_000),
    ("hem_front", 5_000),
    ("hem_back", 5_000),
];
const SIZES: [(&str, i64); 5] = [("XS", -2), ("S", -1), ("M", 0), ("L", 1), ("XL", 2)];

#[test]
fn every_size_draws_closes_and_walks_without_an_artifact() {
    let mut drops = Vec::new();
    for (size, steps) in SIZES {
        let mut disk = MemoryRepository::new();
        let mut running = drawn(&mut disk);
        for (token, change) in STEP_CHANGE {
            let value = input(token) + change * steps;
            step(
                &mut running,
                &format!(r#"{{"do":"change","token":"{token}","um":{value}}}"#),
            )
            .unwrap_or_else(|message| panic!("{size}: {token}: {message}"));
        }
        step(&mut running, r#"{"do":"draft"}"#)
            .unwrap_or_else(|message| panic!("{size}: {message}"));
        assert_all_walk(&mut running, size);
        let pattern = running.pattern().expect("drawn").clone();
        for (name, _) in PIECES {
            let drawn = piece(&pattern, name);
            assert!(
                !crossing(&drawn.cut),
                "{size}: {name}: the cut line crosses itself"
            );
            assert!(
                !crossing(&drawn.sew),
                "{size}: {name}: the sew line crosses itself"
            );
        }
        drops.push(pattern.fits[0].value);
        let known = bindings(&mut running);
        assert_eq!(
            2 * (known["front_hip_x"] + known["back_hip_x"]),
            input("hip_girth") + 40_000 * steps + input("ease_hip"),
            "{size}"
        );
    }
    assert!(drops.iter().all(|drop| *drop > 0), "{drops:?}");
}

#[test]
fn a_changed_measurement_closes_and_walks_or_is_refused_by_the_closure_it_breaks() {
    // Or by the place that stopped being on its edge, as a pocket's end does when the hip comes up past it.
    const CLOSURES: [&str; 3] = ["thigh_clears", "no value of", "along side_upper"];
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
                Ok(_) => assert_all_walk(&mut running, &case),
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
    assert!(refused < 3, "{refused} of these should still draw");
}

#[test]
fn every_edge_is_sewn_or_named_free() {
    const FREE: [(&str, &str); 7] = [
        ("trouser_front", "hem"),
        ("trouser_back", "hem"),
        ("waistband", "band_right"),
        ("waistband", "band_upper"),
        ("waistband", "band_left"),
        ("pocket_bag", "bag_lower"),
        ("pocket_bag", "bag_top"),
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
    let pieces = draft["pieces"].as_list().expect("pieces");
    for item in pieces {
        let item = fields_of(item);
        let name = item["name"].as_text().expect("a name");
        let source = item
            .get("mirror_of")
            .and_then(Value::as_text)
            .unwrap_or(name);
        let original = pieces
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
}

#[test]
fn the_export_equals_its_frozen_goldens_and_replays_the_same_and_a_receiver_reads_it() {
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
        "exports/reference-trousers.canonical.json",
        "exports/reference-trousers.dxf",
        "exports/reference-trousers.pdf",
    ] {
        assert_eq!(file(&first, name), file(&second, name), "{name}");
    }
    assert_eq!(
        file(&first, "exports/reference-trousers.canonical.json"),
        GOLDEN_CANONICAL.as_bytes()
    );
    assert_eq!(
        file(&first, "exports/reference-trousers.dxf"),
        GOLDEN_DXF.as_bytes()
    );
    let pattern = from_canonical_text(GOLDEN_CANONICAL).expect("reads");
    let found = judge_dxf(GOLDEN_DXF, &pattern, true);
    assert!(found.is_empty(), "{found:?}");
    let found = judge_pdf_square(&file(&first, "exports/reference-trousers.pdf"));
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn a_design_saved_and_opened_again_draws_the_same_bytes() {
    let mut disk = MemoryRepository::new();
    replay(&session(), &mut disk).expect("replays");
    let (bare, _) = opening();
    let mut running = Running::open(&bare, &mut disk).expect("opens");
    step(&mut running, r#"{"do":"reopen"}"#).expect("reopens");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws what was saved");
    assert_eq!(
        to_canonical_text(running.pattern().expect("drawn")),
        GOLDEN_CANONICAL
    );
}

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

#[test]
fn the_sizes_drawn_are_the_sizes_the_book_lists() {
    for (token, change) in STEP_CHANGE {
        let shown = format!("| `{token}` | {} mm change |", change / 1000);
        assert!(BOOK.contains(&shown), "the book has no row: {shown}");
    }
    for (size, steps) in SIZES {
        let steps = match steps {
            -2 => "−2".to_owned(),
            -1 => "−1".to_owned(),
            other => other.to_string(),
        };
        assert!(BOOK.contains(&format!("| {size} | {steps} |")), "{size}");
    }
}
