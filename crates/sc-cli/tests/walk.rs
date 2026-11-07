//! Walking and truing: a seam reports what differs and attributes it to declared ease or a defect, and truing is a deliberate edit with an undo.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss
)]
use sc_artifacts::{from_canonical_text, Pattern};
use sc_cli::{replay, Caller, CommandAuthority, Running};
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");

// Two panels, each a rectangle, whose top edges are sewn together: side A is 42 cm and side B is `b_width`, with a `cap_ease` of ease declared.
fn opening(cap_ease: i64, b_width: i64) -> String {
    format!(
        r#"{{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV",
        "inputs":[{{"token":"a_width","um":420000}},{{"token":"b_width","um":{b_width}}},{{"token":"tall","um":100000}},{{"token":"cap_ease","um":{cap_ease}}}],
        "recipe":"let quarter: length = a_width / 4\nlet notch_depth: length = 5 mm\nlet half_a: length = a_width / 2\nlet dart_near: length = half_a - 10 mm\nlet dart_far: length = half_a + 10 mm\nlet apex_y: length = tall - 30 mm\n",
        "steps":[]}}"#
    )
}

fn panel(name: &str, width: &str) -> String {
    format!(
        r#"{{"name":"{name}","quantity":1,"start":["zero","zero"],
        "edges":[{{"to":["{width}","zero"],"width":"zero","corner":"miter"}},{{"to":["{width}","tall"],"width":"zero","corner":"miter"}},
                 {{"to":["zero","tall"],"width":"zero","corner":"miter"}},{{"to":["zero","zero"],"width":"zero","corner":"miter"}}],
        "grain":[[["zero","zero"],["zero","tall"]]]}}"#
    )
}

fn draft_step(ease: bool) -> String {
    let declared = if ease { r#","ease":"cap_ease""# } else { "" };
    format!(
        r#"{{"do":"draft","style":"two","size":"M","pieces":[{},{}],
        "seams":[{{"name":"top","a":{{"piece":"a","from":["zero","tall"],"to":["a_width","tall"]}},"b":{{"piece":"b","from":["zero","tall"],"to":["b_width","tall"]}}{declared}}}]}}"#,
        panel("a", "a_width"),
        panel("b", "b_width")
    )
}

fn step(
    running: &mut Running<&mut MemoryRepository>,
    text: &str,
) -> Result<Value, (String, String)> {
    running
        .step(&parse(text).expect("a step"))
        .map_err(|failure| {
            let value = failure.to_value();
            let fields = value.as_object().expect("a diagnostic");
            (
                fields["failed"].as_text().expect("text").to_owned(),
                fields["message"].as_text().expect("text").to_owned(),
            )
        })
}

fn result(record: &Value) -> Value {
    record.as_object().expect("a record")["result"].clone()
}

fn revision(record: &Value) -> String {
    record.as_object().expect("a record")["revision"]
        .as_text()
        .expect("text")
        .to_owned()
}

fn walked(running: &mut Running<&mut MemoryRepository>) -> Vec<Value> {
    result(&step(running, r#"{"do":"walk"}"#).expect("walks"))
        .as_list()
        .expect("reports")
        .to_vec()
}

fn field<'a>(report: &'a Value, name: &str) -> &'a Value {
    &report.as_object().expect("a report")[name]
}

#[test]
fn a_seam_that_matches_says_so() {
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(0, 420_000)).expect("valid"), &mut disk).expect("opens");
    step(&mut running, &draft_step(false)).expect("draws");
    let report = &walked(&mut running)[0];
    assert_eq!(field(report, "finding").as_text(), Some("matches"));
    assert_eq!(
        (
            field(report, "a_length").as_int(),
            field(report, "b_length").as_int()
        ),
        (Some(420_000), Some(420_000))
    );
}

#[test]
fn a_difference_is_declared_ease_only_when_somebody_declared_it() {
    // A cap 2 cm longer than its armscye, with 2 cm declared: intentional, and named as such.
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(20_000, 400_000)).expect("valid"), &mut disk).expect("opens");
    step(&mut running, &draft_step(true)).expect("draws");
    let report = &walked(&mut running)[0];
    assert_eq!(field(report, "finding").as_text(), Some("declared_ease"));
    assert_eq!(field(report, "differential").as_int(), Some(20_000));
    assert_eq!(field(report, "unexplained").as_int(), Some(0));

    // The same 2 cm with nothing declared is a defect, and the report says how much.
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(0, 400_000)).expect("valid"), &mut disk).expect("opens");
    step(&mut running, &draft_step(false)).expect("draws");
    let report = &walked(&mut running)[0];
    assert_eq!(field(report, "finding").as_text(), Some("defect"));
    assert_eq!(field(report, "unexplained").as_int(), Some(20_000));
}

#[test]
fn a_seam_with_the_wrong_ease_declared_is_a_defect_too() {
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(30_000, 400_000)).expect("valid"), &mut disk).expect("opens");
    step(&mut running, &draft_step(true)).expect("draws");
    let report = &walked(&mut running)[0];
    assert_eq!(field(report, "finding").as_text(), Some("defect"));
    assert_eq!(field(report, "unexplained").as_int(), Some(-10_000));
}

#[test]
fn a_dart_on_a_seam_is_sewn_away_before_the_seam_is_measured() {
    // Side A is 42 cm with a 2 cm dart on it, so it sews to 40, and side B is 40.
    let dart = r#","darts":[{"name":"d","near":["dart_near","tall"],"far":["dart_far","tall"],"apex":["half_a","apex_y"]}]"#;
    let with_dart = panel("a", "a_width");
    let with_dart = format!("{}{dart}}}", &with_dart[..with_dart.len() - 1]);
    let text = format!(
        r#"{{"do":"draft","style":"two","size":"M","pieces":[{with_dart},{}],
        "seams":[{{"name":"top","a":{{"piece":"a","from":["zero","tall"],"to":["a_width","tall"]}},"b":{{"piece":"b","from":["zero","tall"],"to":["b_width","tall"]}}}}]}}"#,
        panel("b", "b_width")
    );
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(0, 400_000)).expect("valid"), &mut disk).expect("opens");
    step(&mut running, &text).expect("draws");
    let report = &walked(&mut running)[0];
    assert_eq!(field(report, "a_length").as_int(), Some(400_000));
    assert_eq!(field(report, "finding").as_text(), Some("matches"));
}

#[test]
fn a_seam_naming_what_does_not_exist_is_refused_by_name() {
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(0, 420_000)).expect("valid"), &mut disk).expect("opens");
    let bad = draft_step(false).replace(r#""piece":"b""#, r#""piece":"nobody""#);
    step(&mut running, &bad).expect("a seam is checked when it is walked");
    let (code, message) = step(&mut running, r#"{"do":"walk"}"#).expect_err("no such piece");
    assert_eq!(code, "walk");
    assert!(
        message.contains("top") && message.contains("nobody"),
        "{message}"
    );
}

#[test]
fn a_balance_notch_is_a_bus_command_with_an_undo_and_sits_at_the_same_share_of_each_side() {
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(20_000, 400_000)).expect("valid"), &mut disk).expect("opens");
    let drawn = step(&mut running, &draft_step(true)).expect("draws");
    assert_eq!(
        revision(&drawn),
        "0",
        "the first drawing is part of creating the design"
    );
    let planted = step(
        &mut running,
        r#"{"do":"plant_balance_notch","seam":"top","at":"quarter","depth":"notch_depth"}"#,
    )
    .expect("plants");
    assert_eq!(revision(&planted), "1", "an edit takes a revision");
    // It needs drawing again, like any edit.
    let (code, _) = step(&mut running, r#"{"do":"walk"}"#).expect_err("nothing is drawn");
    assert_eq!(code, "walk");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    let pattern = running.pattern().expect("a drawing");
    let mark = |name: &str| {
        pattern
            .pieces
            .iter()
            .find(|piece| piece.name == name)
            .expect(name)
            .marks
            .clone()
    };
    let (on_a, on_b) = (mark("a"), mark("b"));
    assert_eq!((on_a.len(), on_b.len()), (1, 1));
    // A quarter of side A is a quarter of side B, so the ease between the notches is even.
    assert_eq!(on_a[0].at.x(), 105_000);
    assert_eq!(on_b[0].at.x(), 100_000);
    assert!(
        pattern.pieces[0].cut.contains(&on_a[0].at),
        "on the cut line"
    );
    let toward = on_a[0].toward;
    assert!(toward.y() < 100_000, "it aims into the piece: {toward:?}");

    // Undone through the bus, the notch is gone and so is the drawing.
    let undone = step(&mut running, r#"{"do":"undo"}"#).expect("undoes");
    assert_eq!(revision(&undone), "0");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    assert!(running.pattern().expect("a drawing").pieces[0]
        .marks
        .is_empty());
    let redone = step(&mut running, r#"{"do":"redo"}"#).expect("redoes");
    assert_eq!(revision(&redone), "1");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    assert_eq!(
        running.pattern().expect("a drawing").pieces[0].marks.len(),
        1
    );
}

#[test]
fn truing_makes_the_seam_match_and_reports_what_it_moved() {
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(0, 400_000)).expect("valid"), &mut disk).expect("opens");
    step(&mut running, &draft_step(false)).expect("draws");
    let trued = step(
        &mut running,
        r#"{"do":"true_seam","seam":"top","side":"b","limit_um":25000}"#,
    )
    .expect("trues");
    assert_eq!(revision(&trued), "1");
    let outcome = result(&trued);
    let outcome = outcome.as_object().expect("an outcome");
    assert_eq!(outcome["moved_um"].as_int(), Some(20_000));
    assert_eq!(
        field(&outcome["before"], "finding").as_text(),
        Some("defect")
    );
    assert_eq!(
        field(&outcome["after"], "finding").as_text(),
        Some("matches")
    );
    // The piece really changed, and the seam's end follows it.
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    let report = &walked(&mut running)[0];
    assert_eq!(field(report, "finding").as_text(), Some("matches"));
    assert_eq!(field(report, "b_length").as_int(), Some(420_000));
}

#[test]
fn truing_leaves_what_the_seam_declares_as_ease() {
    // Side A is 42 and side B 39, with 2 declared. Truing takes up the 1 cm nobody declared, and leaves the 2.
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(20_000, 390_000)).expect("valid"), &mut disk).expect("opens");
    step(&mut running, &draft_step(true)).expect("draws");
    let trued = step(
        &mut running,
        r#"{"do":"true_seam","seam":"top","side":"b","limit_um":15000}"#,
    )
    .expect("trues");
    let outcome = result(&trued);
    let outcome = outcome.as_object().expect("an outcome");
    assert_eq!(outcome["moved_um"].as_int(), Some(10_000));
    assert_eq!(
        field(&outcome["after"], "finding").as_text(),
        Some("declared_ease")
    );
    assert_eq!(
        field(&outcome["after"], "differential").as_int(),
        Some(20_000)
    );
}

#[test]
fn truing_past_its_limit_is_refused_and_changes_nothing() {
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(0, 400_000)).expect("valid"), &mut disk).expect("opens");
    let drawn = step(&mut running, &draft_step(false)).expect("draws");
    let (code, message) = step(
        &mut running,
        r#"{"do":"true_seam","seam":"top","side":"b","limit_um":1000}"#,
    )
    .expect_err("too far");
    assert_eq!(code, "true_seam");
    assert!(
        message.contains("20000") && message.contains("1000"),
        "{message}"
    );
    assert_eq!(running.revision(), revision(&drawn));
    assert!(running.pattern().is_some(), "the drawing still stands");
    // A limit is not optional: truing never moves a thing it was not asked to.
    let (_, message) = step(
        &mut running,
        r#"{"do":"true_seam","seam":"top","side":"b"}"#,
    )
    .expect_err("no limit");
    assert!(message.contains("limit_um"), "{message}");
    let (_, message) = step(
        &mut running,
        r#"{"do":"true_seam","seam":"top","side":"c","limit_um":25000}"#,
    )
    .expect_err("no such side");
    assert!(message.contains("no side c"), "{message}");
}

#[test]
fn a_trued_seam_is_undone_through_the_bus_and_is_trued_again_when_a_measurement_moves() {
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(0, 400_000)).expect("valid"), &mut disk).expect("opens");
    step(&mut running, &draft_step(false)).expect("draws");
    step(
        &mut running,
        r#"{"do":"true_seam","seam":"top","side":"b","limit_um":25000}"#,
    )
    .expect("trues");
    step(&mut running, r#"{"do":"undo"}"#).expect("undoes");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    assert_eq!(
        field(&walked(&mut running)[0], "finding").as_text(),
        Some("defect"),
        "back to before"
    );
    step(&mut running, r#"{"do":"redo"}"#).expect("redoes");
    // Truing is an operation in the document, so a new measurement is trued too.
    step(
        &mut running,
        r#"{"do":"change","token":"a_width","um":430000}"#,
    )
    .expect("changes");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    let report = &walked(&mut running)[0];
    assert_eq!(field(report, "finding").as_text(), Some("matches"));
    assert_eq!(field(report, "b_length").as_int(), Some(430_000));
}

#[test]
fn a_caller_without_commit_is_refused_by_the_bus_itself() {
    let caller = Caller {
        agent: true,
        authorities: vec![CommandAuthority::Inspect, CommandAuthority::Generate],
    };
    let mut disk = MemoryRepository::new();
    let mut running = Running::open_as(
        &parse(&opening(0, 400_000)).expect("valid"),
        &mut disk,
        &caller,
    )
    .expect("opens");
    step(&mut running, &draft_step(false)).expect("the first drawing needs generate");
    walked(&mut running);
    let (code, _) = step(
        &mut running,
        r#"{"do":"true_seam","seam":"top","side":"b","limit_um":25000}"#,
    )
    .expect_err("needs commit");
    assert_eq!(code, "refused");
    let (code, _) = step(
        &mut running,
        r#"{"do":"plant_balance_notch","seam":"top","at":"quarter","depth":"notch_depth"}"#,
    )
    .expect_err("needs commit");
    assert_eq!(code, "refused");
}

#[test]
fn a_seam_edit_saved_and_reopened_is_the_same_design() {
    let mut disk = MemoryRepository::new();
    let mut running =
        Running::open(&parse(&opening(0, 400_000)).expect("valid"), &mut disk).expect("opens");
    step(&mut running, &draft_step(false)).expect("draws");
    step(
        &mut running,
        r#"{"do":"true_seam","seam":"top","side":"b","limit_um":25000}"#,
    )
    .expect("trues");
    step(&mut running, r#"{"do":"save"}"#).expect("saves");
    step(&mut running, r#"{"do":"reopen"}"#).expect("reopens");
    step(&mut running, r#"{"do":"draft"}"#).expect("draws");
    assert_eq!(
        field(&walked(&mut running)[0], "finding").as_text(),
        Some("matches")
    );
}

fn skirt_pattern() -> Pattern {
    let mut disk = MemoryRepository::new();
    replay(&parse(SKIRT).expect("valid"), &mut disk)
        .unwrap_or_else(|failure| panic!("{failure:?}"));
    let bytes = disk
        .read(&FilePath::new("exports/reference-skirt.canonical.json").expect("a path"))
        .expect("reads")
        .expect("exported");
    from_canonical_text(&String::from_utf8(bytes).expect("text")).expect("reads")
}

#[test]
fn every_seam_of_the_reference_skirt_walks_to_a_match() {
    // §8 of the chapter: every span declares zero ease, so any differential past the class is a bug.
    let pattern = skirt_pattern();
    let reports = sc_artifacts::walk(&pattern).expect("walks");
    assert_eq!(
        reports.len(),
        7,
        "the side seams, the centre back, and the waist in four parts"
    );
    for report in &reports {
        assert_eq!(report.finding, "matches", "{report:?}");
    }
    // The waist is sewn after its dart is closed, which the walk sees for itself.
    let waist = reports
        .iter()
        .find(|report| report.seam == "waist_front_left")
        .expect("the front waist");
    assert_eq!((waist.a_length, waist.b_length), (185_000, 185_000));
}
