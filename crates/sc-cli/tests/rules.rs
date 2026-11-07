//! Path 2: the sample moved by a table, and rebuilt with the recipe left out of it.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use sc_artifacts::{from_canonical_text, stretch, stretch_length, Pattern};
use sc_cli::{replay, Caller, CommandAuthority, Running};
use sc_geometry::Point;
use sc_grading::{from_rul, GradeTable, Provenance};
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::BTreeMap;

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");
const BODICE: &str = include_str!("../../../conformance/reference-bodice/session.json");
const TROUSERS: &str = include_str!("../../../conformance/reference-trousers/session.json");
const SIZES: [&str; 5] = ["XS", "S", "M", "L", "XL"];

fn fields_of(value: &Value) -> &BTreeMap<String, Value> {
    value.as_object().expect("an object")
}

fn result(record: &Value) -> Value {
    fields_of(record)["result"].clone()
}

fn rewritten(session: &str, change: &dyn Fn(&mut BTreeMap<String, Value>)) -> String {
    let whole = parse(session).expect("valid");
    let mut fields = fields_of(&whole).clone();
    let mut steps = fields["steps"].as_list().expect("steps").to_vec();
    let draft = steps
        .iter_mut()
        .find(|step| fields_of(step)["do"].as_text() == Some("draft"))
        .expect("a draft");
    let mut described = fields_of(draft).clone();
    change(&mut described);
    *draft = Value::Object(described);
    fields.insert("steps".to_owned(), Value::List(steps));
    Value::Object(fields).to_text()
}

fn with_steps(session: &str, extra: &str) -> Value {
    let whole = parse(session).expect("valid");
    let mut fields = fields_of(&whole).clone();
    let mut steps: Vec<Value> = fields["steps"]
        .as_list()
        .expect("steps")
        .iter()
        .filter(|step| fields_of(step)["do"].as_text() != Some("save"))
        .cloned()
        .collect();
    steps.extend(
        parse(extra)
            .expect("valid")
            .as_list()
            .expect("steps")
            .iter()
            .cloned(),
    );
    fields.insert("steps".to_owned(), Value::List(steps));
    Value::Object(fields)
}

struct Graded {
    report: Value,
    disk: MemoryRepository,
}

fn run(session: &str, step: &str) -> Graded {
    let mut disk = MemoryRepository::new();
    let plan = with_steps(session, &format!("[{step},{{\"do\":\"save\"}}]"));
    let transcript = replay(&plan, &mut disk).unwrap_or_else(|failure| panic!("{failure:?}"));
    let steps = transcript.as_list().expect("a transcript");
    Graded {
        report: result(&steps[steps.len() - 2]),
        disk,
    }
}

fn refused(session: &str, step: &str) -> String {
    let plan = with_steps(session, &format!("[{step}]"));
    let failure = replay(&plan, MemoryRepository::new())
        .expect_err(step)
        .to_value();
    fields_of(&failure)["message"]
        .as_text()
        .expect("text")
        .to_owned()
}

fn file(disk: &MemoryRepository, name: &str) -> String {
    String::from_utf8(
        disk.read(&FilePath::new(name).expect("a path"))
            .expect("reads")
            .unwrap_or_else(|| panic!("{name} was not written")),
    )
    .expect("text")
}

fn members(text: &str) -> BTreeMap<String, Pattern> {
    let whole = parse(text).expect("valid");
    fields_of(&whole)["sizes"]
        .as_list()
        .expect("sizes")
        .iter()
        .map(|member| {
            let member = fields_of(member);
            (
                member["size"].as_text().expect("a size").to_owned(),
                from_canonical_text(&member["pattern"].to_text()).expect("reads"),
            )
        })
        .collect()
}

fn piece<'a>(pattern: &'a Pattern, name: &str) -> &'a sc_artifacts::DrawnPiece {
    pattern
        .pieces
        .iter()
        .find(|piece| piece.name == name)
        .unwrap_or_else(|| panic!("no piece is named {name}"))
}

fn lowest_left(line: &[Point]) -> (i64, i64) {
    (
        line.iter().map(|at| at.x()).min().expect("a line"),
        line.iter().map(|at| at.y()).min().expect("a line"),
    )
}

fn to_segment(at: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let reach = dx * dx + dy * dy;
    let t = if reach == 0.0 {
        0.0
    } else {
        (((at.0 - a.0) * dx + (at.1 - a.1) * dy) / reach).clamp(0.0, 1.0)
    };
    ((at.0 - a.0 - t * dx).powi(2) + (at.1 - a.1 - t * dy).powi(2)).sqrt()
}

// How far apart two closed lines get, once their lowest left corners are put together. A stack point moves a piece and does not change it.
fn apart(a: &[Point], b: &[Point]) -> f64 {
    let (ax, ay) = lowest_left(a);
    let (bx, by) = lowest_left(b);
    let one_way = |from: &[Point], fx: i64, fy: i64, onto: &[Point], ox: i64, oy: i64| {
        from.iter()
            .map(|at| {
                let at = ((at.x() - fx) as f64, (at.y() - fy) as f64);
                onto.iter()
                    .enumerate()
                    .map(|(index, start)| {
                        let end = onto[(index + 1) % onto.len()];
                        to_segment(
                            at,
                            ((start.x() - ox) as f64, (start.y() - oy) as f64),
                            ((end.x() - ox) as f64, (end.y() - oy) as f64),
                        )
                    })
                    .fold(f64::MAX, f64::min)
            })
            .fold(0.0, f64::max)
    };
    one_way(a, ax, ay, b, bx, by).max(one_way(b, bx, by, a, ax, ay))
}

#[test]
fn a_table_worked_out_here_rebuilds_a_garment_whose_points_are_affine_exactly() {
    // Every point of the skirt moves in proportion to its measurements, which is why the instantiation chapter says the two paths agree on it.
    let graded = run(SKIRT, r#"{"do":"grade","path":"exports/graded.json"}"#);
    let by_rule = members(&file(&graded.disk, "exports/graded.json"));
    let regenerated = run(SKIRT, r#"{"do":"regenerate","path":"exports/family.json"}"#);
    let by_recipe = members(&file(&regenerated.disk, "exports/family.json"));
    assert_eq!(
        fields_of(&graded.report)["provenance"].as_text(),
        Some("regenerations")
    );
    for size in SIZES {
        for drawn in &by_recipe[size].pieces {
            let rebuilt = piece(&by_rule[size], &drawn.name);
            let gap = apart(&drawn.sew, &rebuilt.sew);
            assert!(
                gap <= 2.0,
                "{size}: {}: the sew lines are {gap} apart",
                drawn.name
            );
            let gap = apart(&drawn.cut, &rebuilt.cut);
            assert!(
                gap <= 25.0,
                "{size}: {}: the cut lines are {gap} apart",
                drawn.name
            );
        }
    }
}

// The instantiation chapter's worked example: L, from M, with 4 cm more waist and hip and 1 cm more length.
#[test]
fn the_worked_example_in_the_book_is_what_a_rule_gives() {
    let graded = run(SKIRT, r#"{"do":"grade","path":"exports/graded.json"}"#);
    let family = members(&file(&graded.disk, "exports/graded.json"));
    let front = piece(&family["L"], "skirt_front");
    // Dart legs, from CF.
    let dart = &front.closures[0];
    assert_eq!(
        (
            dart.from.x().min(dart.to.x()),
            dart.from.x().max(dart.to.x())
        ),
        (97_500, 137_500)
    );
    assert_eq!(dart.intake, 40_000, "the dart does not grade");
    let widest_hem = front
        .sew
        .iter()
        .filter(|at| at.y() == -630_000)
        .map(|at| at.x())
        .max();
    assert_eq!(widest_hem, Some(295_000), "the quarter hem");
    // The side seam, hip to hem: 43.104524 cm by both routes.
    let at = |x: i64, y: i64| Point::new(x, y).expect("in range");
    let side = stretch_length(
        &stretch(&front.sew, at(295_000, -630_000), at(265_000, -200_000)).expect("on the line"),
    );
    assert!((side - 431_045.24).abs() <= 1.0, "{side}");
    // The waist it owes.
    let waist =
        stretch_length(&stretch(&front.sew, at(0, 0), at(235_000, 0)).expect("on the line"))
            - 40_000.0;
    assert!((waist - 195_000.0).abs() <= 1.0, "{waist}");
}

// Every corner a rule moves lands exactly where the recipe put it, once the piece is held to its stack point.
fn corners_are_exact(session: &str) {
    let graded = run(
        session,
        r#"{"do":"grade","path":"exports/graded.json","rules_path":"exports/r.json"}"#,
    );
    let by_rule = members(&file(&graded.disk, "exports/graded.json"));
    let table =
        GradeTable::from_value(&parse(&file(&graded.disk, "exports/r.json")).expect("valid"))
            .expect("reads");
    let regenerated = run(session, r#"{"do":"regenerate"}"#);
    for report in fields_of(&regenerated.report)["sizes"]
        .as_list()
        .expect("sizes")
        .iter()
        .map(fields_of)
    {
        let size = report["size"].as_text().expect("a size");
        let points: Vec<&BTreeMap<String, Value>> = report["points"]
            .as_list()
            .expect("points")
            .iter()
            .map(fields_of)
            .collect();
        let at = |piece_name: &str, edge: &str| -> (i64, i64) {
            let item = points
                .iter()
                .find(|item| {
                    item["piece"].as_text() == Some(piece_name)
                        && item["edge"].as_text() == Some(edge)
                        && item["point"].as_text() == Some("to")
                })
                .unwrap_or_else(|| panic!("{piece_name} {edge}"));
            (
                fields_of(&item["x"])["value"].as_int().expect("a value"),
                fields_of(&item["y"])["value"].as_int().expect("a value"),
            )
        };
        let sample = fields_of(&regenerated.report)["sizes"]
            .as_list()
            .expect("sizes")
            .iter()
            .map(fields_of)
            .find(|report| report["size"].as_text() == Some("M"))
            .expect("the sample");
        let at_sample = |piece_name: &str, edge: &str| -> (i64, i64) {
            let item = sample["points"]
                .as_list()
                .expect("points")
                .iter()
                .map(fields_of)
                .find(|item| {
                    item["piece"].as_text() == Some(piece_name)
                        && item["edge"].as_text() == Some(edge)
                        && item["point"].as_text() == Some("to")
                })
                .unwrap_or_else(|| panic!("{piece_name} {edge}"));
            (
                fields_of(&item["x"])["value"].as_int().expect("a value"),
                fields_of(&item["y"])["value"].as_int().expect("a value"),
            )
        };
        for item in &points {
            if item["point"].as_text() != Some("to") {
                continue;
            }
            let (name, edge) = (
                item["piece"].as_text().expect("a piece"),
                item["edge"].as_text().expect("an edge"),
            );
            let anchor = table
                .attributes()
                .stack
                .iter()
                .find(|stack| stack.starts_with(&format!("{name}/")))
                .unwrap_or_else(|| panic!("{name} has no stack point"));
            let anchor_edge = anchor.split('/').nth(1).expect("an edge");
            let (now, drift_now, drift_was) = (
                at(name, edge),
                at(name, anchor_edge),
                at_sample(name, anchor_edge),
            );
            let wanted = (
                now.0 - (drift_now.0 - drift_was.0),
                now.1 - (drift_now.1 - drift_was.1),
            );
            let rebuilt = piece(&by_rule[size], name);
            assert!(
                rebuilt.sew.iter().any(|at| (at.x(), at.y()) == wanted),
                "{size}: {name}/{edge}/to should be at {wanted:?}"
            );
        }
    }
}

// The bodice is the case the instantiation chapter says is the real test: the cap and the armhole are not affine, so the two paths differ. The corners agree exactly, what differs is between them, and it is measured here so that it cannot quietly grow.
#[test]
fn a_garment_with_solved_curves_has_its_corners_exact_and_differs_between_them_by_a_stated_amount()
{
    corners_are_exact(SKIRT);
    corners_are_exact(BODICE);
    corners_are_exact(TROUSERS);
    // The most any two lines differ, in micrometres, at the far ends of the set.
    for (session, most) in [(BODICE, 6_000.0), (TROUSERS, 3_000.0)] {
        let graded = run(session, r#"{"do":"grade","path":"exports/graded.json"}"#);
        let by_rule = members(&file(&graded.disk, "exports/graded.json"));
        let regenerated = run(
            session,
            r#"{"do":"regenerate","path":"exports/family.json"}"#,
        );
        let by_recipe = members(&file(&regenerated.disk, "exports/family.json"));
        for size in SIZES {
            for drawn in &by_recipe[size].pieces {
                let gap = apart(&drawn.sew, &piece(&by_rule[size], &drawn.name).sew);
                assert!(gap <= most, "{size}: {}: {gap}", drawn.name);
                if size == "M" {
                    assert!(
                        gap <= 2.0,
                        "the sample is the sample: {}: {gap}",
                        drawn.name
                    );
                }
            }
        }
        for report in fields_of(&graded.report)["sizes"]
            .as_list()
            .expect("sizes")
            .iter()
            .map(fields_of)
        {
            assert!(report["unmentioned"].as_list().expect("a list").is_empty());
            assert!(report["unmatched"].as_list().expect("a list").is_empty());
            assert!(report["conflicts"].as_list().expect("a list").is_empty());
        }
    }
}

#[test]
fn what_is_only_approximated_is_said() {
    let graded = run(BODICE, r#"{"do":"grade"}"#);
    let report = fields_of(&graded.report)["sizes"].as_list().expect("sizes")[0].clone();
    let notes: Vec<&str> = fields_of(&report)["notes"]
        .as_list()
        .expect("notes")
        .iter()
        .filter_map(Value::as_text)
        .collect();
    assert!(
        notes
            .iter()
            .any(|note| note.starts_with("bodice_front/armhole") && note.contains("interpolation")),
        "{notes:?}"
    );
}

#[test]
fn a_notch_keeps_its_place_on_its_edge_rather_than_its_distance() {
    let graded = run(SKIRT, r#"{"do":"grade","path":"exports/graded.json"}"#);
    let family = members(&file(&graded.disk, "exports/graded.json"));
    // Down CB from the waist: the same share at each size, which at a longer skirt is further down.
    let notch_y = |size: &str| {
        piece(&family[size], "skirt_back_right")
            .marks
            .iter()
            .map(|mark| mark.at)
            .filter(|at| at.x() < 0)
            .map(|at| at.y())
            .max()
            .expect("the zipper's notch")
    };
    let hem = |size: &str| lowest_left(&piece(&family[size], "skirt_back_right").sew).1;
    let share = |size: &str| notch_y(size) as f64 / hem(size) as f64;
    assert!(
        (share("XL") - share("XS")).abs() < 0.001,
        "{} and {}",
        share("XL"),
        share("XS")
    );
    assert_ne!(notch_y("XL"), notch_y("XS"));
}

#[test]
fn allowances_are_worked_out_again_and_the_net_lines_are_the_graded_ones() {
    let graded = |width: i64| {
        let whole = parse(SKIRT).expect("valid");
        let mut fields = fields_of(&whole).clone();
        let inputs: Vec<Value> = fields["inputs"]
            .as_list()
            .expect("inputs")
            .iter()
            .map(|item| {
                let mut item = fields_of(item).clone();
                if item["token"].as_text() == Some("sa_side") {
                    item.insert("um".to_owned(), Value::Int(width));
                }
                Value::Object(item)
            })
            .collect();
        fields.insert("inputs".to_owned(), Value::List(inputs));
        let done = run(
            &Value::Object(fields).to_text(),
            r#"{"do":"grade","path":"exports/graded.json"}"#,
        );
        members(&file(&done.disk, "exports/graded.json"))
    };
    let (narrow, wide) = (graded(10_000), graded(25_000));
    for size in SIZES {
        let (a, b) = (
            piece(&narrow[size], "skirt_front"),
            piece(&wide[size], "skirt_front"),
        );
        assert_eq!(a.sew, b.sew, "{size}");
        assert_ne!(a.cut, b.cut, "{size}");
    }
}

#[test]
fn a_contour_declared_fixed_that_grading_would_change_is_a_conflict_and_blocks_the_files() {
    let fixed = rewritten(SKIRT, &|draft| {
        let mut grading = fields_of(&draft["grading"]).clone();
        grading.insert(
            "fixed".to_owned(),
            parse(r#"[{"name":"zip_opening","piece":"skirt_back_right","from":{"edge":"cb","at":"0"},"to":{"edge":"cb","along":"zip_len"}}]"#).expect("valid"),
        );
        draft.insert("grading".to_owned(), Value::Object(grading));
    });
    let graded = run(&fixed, r#"{"do":"grade"}"#);
    let sizes = fields_of(&graded.report)["sizes"]
        .as_list()
        .expect("sizes")
        .to_vec();
    let at = |size: &str| {
        sizes
            .iter()
            .map(fields_of)
            .find(|report| report["size"].as_text() == Some(size))
            .expect("the size")["conflicts"]
            .as_list()
            .expect("a list")
            .to_vec()
    };
    assert!(at("M").is_empty(), "nothing has moved at the sample");
    let conflict = at("XL");
    let conflict = fields_of(&conflict[0]);
    assert_eq!(conflict["contour"].as_text(), Some("zip_opening"));
    assert_eq!(conflict["size"].as_text(), Some("XL"));
    assert_eq!(conflict["at_sample"].as_int(), Some(180_000));
    assert_ne!(conflict["here"].as_int(), Some(180_000));
    let said = refused(&fixed, r#"{"do":"grade","rul":"exports/x.rul"}"#);
    assert!(
        said.contains("zip_opening") && said.contains("not written"),
        "{said}"
    );
}

#[test]
fn smoothing_is_refused_rather_than_ignored() {
    let asked = rewritten(SKIRT, &|draft| {
        let mut grading = fields_of(&draft["grading"]).clone();
        grading.insert(
            "smoothing".to_owned(),
            parse(r#"["skirt_front/side_upper"]"#).expect("valid"),
        );
        draft.insert("grading".to_owned(), Value::Object(grading));
    });
    let said = refused(&asked, r#"{"do":"grade"}"#);
    assert!(said.contains("refused rather than ignored"), "{said}");
}

#[test]
fn a_piece_with_no_stack_point_and_a_draft_that_cannot_be_graded_are_refused_by_name() {
    let no_stack = rewritten(SKIRT, &|draft| {
        let mut grading = fields_of(&draft["grading"]).clone();
        let mut stack = fields_of(&grading["stack"]).clone();
        stack.remove("waistband");
        grading.insert("stack".to_owned(), Value::Object(stack));
        draft.insert("grading".to_owned(), Value::Object(grading));
    });
    let said = refused(&no_stack, r#"{"do":"grade"}"#);
    assert!(said.contains("no stack point for waistband"), "{said}");
    let no_grading = rewritten(SKIRT, &|draft| {
        draft.remove("grading");
    });
    let said = refused(&no_grading, r#"{"do":"grade"}"#);
    assert!(said.contains("does not say how it grades"), "{said}");
    let trued = rewritten(SKIRT, &|draft| {
        draft.insert(
            "trues".to_owned(),
            parse(r#"[{"seam":"cb","side":"a"}]"#).expect("valid"),
        );
    });
    let said = refused(&trued, r#"{"do":"grade"}"#);
    assert!(said.contains("a rule cannot follow it yet"), "{said}");
}

#[test]
fn a_table_is_written_in_either_convention_and_says_the_same_thing() {
    let written = |convention: &str| {
        let done = run(
            SKIRT,
            &format!(
                r#"{{"do":"grade","convention":"{convention}","rules_path":"exports/r.json","path":"exports/g.json"}}"#
            ),
        );
        (
            file(&done.disk, "exports/r.json"),
            file(&done.disk, "exports/g.json"),
        )
    };
    let (cumulative, a) = written("cumulative");
    let (incremental, b) = written("incremental");
    assert_ne!(cumulative, incremental);
    assert_eq!(a, b, "the same drawing either way");
    let read = GradeTable::from_value(&parse(&incremental).expect("valid")).expect("reads");
    assert_eq!(read.convention().token(), "incremental");
    let said = refused(SKIRT, r#"{"do":"grade","convention":"sideways"}"#);
    assert!(said.contains("no convention is named sideways"), "{said}");
}

#[test]
fn a_rul_file_holds_every_point_and_its_numbers_match_the_table() {
    let done = run(
        SKIRT,
        r#"{"do":"grade","rul":"exports/r.rul","rules_path":"exports/r.json"}"#,
    );
    let table = GradeTable::from_value(&parse(&file(&done.disk, "exports/r.json")).expect("valid"))
        .expect("reads");
    let read = from_rul(&file(&done.disk, "exports/r.rul")).expect("reads");
    assert_eq!(read.sizes, SIZES);
    assert_eq!(read.sample, "M");
    assert_eq!(read.name, "reference-skirt");
    assert_eq!(read.rules.len(), table.rules().len());
    for rule in table.rules() {
        let line = read
            .rules
            .iter()
            .find(|other| other.id == rule.id)
            .expect("the rule");
        for (written, back) in rule.deltas.iter().zip(&line.deltas) {
            assert!(
                (written.0 - back.0).abs() <= 5 && (written.1 - back.1).abs() <= 5,
                "{}",
                rule.point
            );
        }
    }
    let record = parse(&file(&done.disk, "exports/r.rul.record.json")).expect("valid");
    assert_eq!(
        fields_of(&record)["target"].as_text(),
        Some("rul-astm-d6673")
    );
    // A stack point is a zero, and a zero the table means.
    for point in &table.attributes().stack {
        assert!(table
            .rule(point)
            .expect("a rule")
            .deltas
            .iter()
            .all(|delta| *delta == (0, 0)));
    }
}

// A table from somewhere else: one that does not mention everything, and mentions something this design does not have.
#[test]
fn a_table_received_from_somewhere_else_says_what_it_did_not_mention_and_is_still_used() {
    let done = run(SKIRT, r#"{"do":"grade","rules_path":"exports/ours.json"}"#);
    let ours = parse(&file(&done.disk, "exports/ours.json")).expect("valid");
    let mut theirs = fields_of(&ours).clone();
    theirs.insert("provenance".to_owned(), Value::Text("received".to_owned()));
    let mut rules: Vec<Value> = theirs["rules"]
        .as_list()
        .expect("rules")
        .iter()
        .filter(|rule| fields_of(rule)["point"].as_text() != Some("skirt_front/side_lower/to"))
        .cloned()
        .collect();
    rules.push(parse(r#"{"point":"skirt_front/pocket/to","id":9001,"deltas":[[0,0],[0,0],[0,0],[0,0],[0,0]]}"#).expect("valid"));
    theirs.insert("rules".to_owned(), Value::List(rules));
    let table = GradeTable::from_value(&Value::Object(theirs)).expect("reads");
    assert_eq!(table.provenance(), Provenance::Received);

    let plan = with_steps(
        SKIRT,
        &format!(
            r#"[{{"do":"grade","table_text":{},"path":"exports/g.json"}},{{"do":"save"}}]"#,
            Value::Text(table.to_value().to_text()).to_text()
        ),
    );
    let mut disk = MemoryRepository::new();
    let transcript = replay(&plan, &mut disk).unwrap_or_else(|failure| panic!("{failure:?}"));
    let steps = transcript.as_list().expect("a transcript");
    let graded = result(&steps[steps.len() - 2]);
    assert_eq!(fields_of(&graded)["provenance"].as_text(), Some("received"));
    let reports = fields_of(&graded)["sizes"]
        .as_list()
        .expect("sizes")
        .to_vec();
    let first = fields_of(&reports[0]);
    let words = |name: &str| -> Vec<String> {
        first[name]
            .as_list()
            .expect("a list")
            .iter()
            .filter_map(Value::as_text)
            .map(str::to_owned)
            .collect()
    };
    assert_eq!(words("unmentioned"), ["skirt_front/side_lower/to"]);
    assert_eq!(words("unmatched"), ["skirt_front/pocket/to"]);
    // The corner it left out still moves, in proportion to the corners either side, and the seam it is on is a finding rather than a surprise.
    let family = members(&file(&disk, "exports/g.json"));
    let at_xl = lowest_left(&piece(&family["XL"], "skirt_front").sew);
    assert!(at_xl.1 < lowest_left(&piece(&family["M"], "skirt_front").sew).1);
}

#[test]
fn a_caller_who_cannot_generate_is_refused() {
    let whole = parse(SKIRT).expect("valid");
    let mut fields = fields_of(&whole).clone();
    fields.insert("steps".to_owned(), Value::List(Vec::new()));
    let caller = Caller {
        agent: true,
        authorities: vec![CommandAuthority::Inspect],
    };
    let mut disk = MemoryRepository::new();
    let mut running = Running::open_as(&Value::Object(fields), &mut disk, &caller).expect("opens");
    assert!(running
        .step(&parse(r#"{"do":"grade"}"#).expect("valid"))
        .is_err());
}

#[test]
fn grading_gives_the_same_bytes_every_time_and_changes_nothing_in_the_design() {
    let step = r#"{"do":"grade","path":"exports/g.json","rules_path":"exports/r.json","rul":"exports/r.rul"}"#;
    let (a, b) = (run(BODICE, step), run(BODICE, step));
    for name in [
        "exports/g.json",
        "exports/r.json",
        "exports/r.rul",
        "exports/r.rul.record.json",
    ] {
        assert_eq!(file(&a.disk, name), file(&b.disk, name), "{name}");
    }
    let whole = parse(SKIRT).expect("valid");
    let mut fields = fields_of(&whole).clone();
    let steps = fields["steps"].as_list().expect("steps").to_vec();
    fields.insert("steps".to_owned(), Value::List(Vec::new()));
    let draft = steps
        .iter()
        .find(|step| fields_of(step)["do"].as_text() == Some("draft"))
        .expect("a draft")
        .clone();
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&Value::Object(fields), &mut disk).expect("opens");
    running.step(&draft).expect("draws");
    let (revision, before) = (
        running.revision(),
        running.pattern().expect("drawn").clone(),
    );
    running
        .step(&parse(r#"{"do":"grade"}"#).expect("valid"))
        .expect("grades");
    assert_eq!(running.revision(), revision);
    assert_eq!(running.pattern().expect("still drawn"), &before);
}

#[test]
fn an_edit_to_the_sample_leaves_the_table_on_the_same_points() {
    let whole = parse(SKIRT).expect("valid");
    let mut fields = fields_of(&whole).clone();
    let steps = fields["steps"].as_list().expect("steps").to_vec();
    fields.insert("steps".to_owned(), Value::List(Vec::new()));
    let draft = steps
        .iter()
        .find(|step| fields_of(step)["do"].as_text() == Some("draft"))
        .expect("a draft")
        .clone();
    let mut disk = MemoryRepository::new();
    let mut running = Running::open(&Value::Object(fields), &mut disk).expect("opens");
    running.step(&draft).expect("draws");
    let findings = |graded: &Value| -> Vec<(String, String)> {
        fields_of(graded)["sizes"]
            .as_list()
            .expect("sizes")
            .iter()
            .flat_map(|report| {
                fields_of(report)["walk"]
                    .as_list()
                    .expect("a walk")
                    .to_vec()
            })
            .map(|seam| {
                let seam = fields_of(&seam).clone();
                (
                    seam["seam"].as_text().expect("a seam").to_owned(),
                    seam["finding"].as_text().expect("a finding").to_owned(),
                )
            })
            .collect()
    };
    let asked = parse(r#"{"do":"grade"}"#).expect("valid");
    let before = result(&running.step(&asked).expect("grades"));
    let edit = parse(r#"{"do":"edit_edge","piece":"skirt_front","op":"split","edge":"side_lower","at":"1/3","into":["low","high"]}"#).expect("valid");
    running.step(&edit).expect("splits");
    let graded = result(&running.step(&asked).expect("grades"));
    for report in fields_of(&graded)["sizes"]
        .as_list()
        .expect("sizes")
        .iter()
        .map(fields_of)
    {
        assert!(
            report["unmatched"].as_list().expect("a list").is_empty(),
            "{report:?}"
        );
        assert!(
            report["unmentioned"].as_list().expect("a list").is_empty(),
            "{report:?}"
        );
    }
    assert_eq!(
        findings(&before),
        findings(&graded),
        "the edit changed no seam's finding"
    );
}
