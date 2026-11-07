//! Closures with their meaning: a dart's intake is queryable and conserved by closing it, and slash and spread is done again from the recipe, not stored as geometry.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss
)]
use sc_artifacts::{closure_reports, from_canonical_text, ClosureKind, Pattern};
use sc_cli::replay;
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");

fn drawn(session: &Value) -> Result<Pattern, (String, String)> {
    let mut disk = MemoryRepository::new();
    replay(session, &mut disk).map_err(|failure| {
        let value = failure.to_value();
        let fields = value.as_object().expect("a diagnostic");
        (
            fields["failed"].as_text().expect("text").to_owned(),
            fields["message"].as_text().expect("text").to_owned(),
        )
    })?;
    let bytes = disk
        .read(&FilePath::new("out.json").expect("a path"))
        .expect("reads")
        .expect("exported");
    Ok(from_canonical_text(&String::from_utf8(bytes).expect("text")).expect("reads"))
}

// A plain rectangle 40 cm wide and 60 cm tall with no allowance, and whatever else a test puts on it.
fn panel(extra: &str, lengths: &str) -> Value {
    parse(&format!(
        r#"{{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV",
        "inputs":[{{"token":"width","um":400000}},{{"token":"height","um":600000}}{lengths}],
        "recipe":"let half: length = width / 2\nlet mid: length = height / 2\nlet near_x: length = half - 20 mm\nlet far_x: length = half + 20 mm\nlet apex_y: length = height - 120 mm\nlet quarter: length = width / 4\nlet three_quarters: length = 3 * width / 4\nlet gathered: length = 150 mm\nlet four_cm: length = 40 mm\nlet ten_cm: length = 100 mm\n",
        "steps":[{{"do":"draft","style":"panel","size":"M","pieces":[{{
          "name":"panel","quantity":1,"start":["zero","zero"],
          "edges":[{{"to":["width","zero"],"width":"zero","corner":"miter"}},
                   {{"to":["width","height"],"width":"zero","corner":"miter"}},
                   {{"to":["zero","height"],"width":"zero","corner":"miter"}},
                   {{"to":["zero","zero"],"width":"zero","corner":"miter"}}],
          "grain":[[["half","zero"],["half","height"]]]{extra}
        }}]}},
        {{"do":"closures"}},
        {{"do":"export_pattern","format":"canonical","path":"out.json"}},
        {{"do":"save"}}]}}"#
    ))
    .expect("valid")
}

const DART: &str = r#","darts":[{"name":"top","near":["near_x","height"],"far":["far_x","height"],"apex":["half","apex_y"],"intake":"four_cm"}]"#;

#[test]
fn a_dart_says_what_it_takes_and_closing_it_removes_exactly_that() {
    let pattern = drawn(&panel(DART, "")).expect("draws");
    let piece = &pattern.pieces[0];
    let closure = &piece.closures[0];
    assert_eq!((closure.kind, closure.intake), (ClosureKind::Dart, 40_000));
    assert_eq!(piece.internal.len(), 2, "its legs are drawn");
    let reports = closure_reports(&pattern);
    assert_eq!(reports.len(), 1);
    let report = &reports[0];
    assert!(report.conserved, "{report:?}");
    assert_eq!(report.open_length, 2_000_000);
    assert_eq!(
        report.closed_length,
        2_000_000 - 40_000,
        "the perimeter loses exactly the intake"
    );
    assert_eq!(
        report.gap_after_closing, 0,
        "turned about the apex, the far leg lands on the near one"
    );
}

#[test]
fn intake_is_conserved_when_a_measurement_changes() {
    // The dart's legs follow the recipe, so its intake and the closed length follow them, and the balance still holds at every width tried.
    for width in [300_000, 400_000, 523_417] {
        let text = panel(DART, "")
            .to_text()
            .replace("\"um\": 400000", &format!("\"um\": {width}"));
        let pattern = drawn(&parse(&text).expect("valid")).expect("draws");
        let report = &closure_reports(&pattern)[0];
        assert!(report.conserved, "{width}: {report:?}");
        assert_eq!(
            report.open_length - report.closed_length,
            report.intake,
            "{width}"
        );
    }
}

#[test]
fn a_dart_whose_declared_intake_disagrees_with_its_legs_is_refused_by_name() {
    let wrong = DART.replace("four_cm", "ten_cm");
    let (code, message) =
        drawn(&panel(&wrong, "")).expect_err("the legs take 4 cm and it declares 10");
    assert_eq!(code, "draft");
    assert!(
        message.contains("dart top") && message.contains("100000") && message.contains("40000"),
        "{message}"
    );
}

#[test]
fn a_dart_off_the_edge_or_with_its_apex_outside_is_refused_by_name() {
    let off = DART.replace(
        r#""near":["near_x","height"]"#,
        r#""near":["near_x","mid"]"#,
    );
    let (_, message) = drawn(&panel(&off, "")).expect_err("not on the sew line");
    assert!(message.contains("not on the sew line"), "{message}");
    let outside = DART.replace(r#""apex":["half","apex_y"]"#, r#""apex":["half","-mid"]"#);
    let (_, message) = drawn(&panel(&outside, "")).expect_err("outside");
    assert!(message.contains("outside the piece"), "{message}");
}

#[test]
fn tucks_pleats_and_gathers_each_carry_their_own_kind_and_intake() {
    let extra = r#","tucks":[{"name":"t","from":["quarter","height"],"to":["near_x","height"],"from_end":["quarter","apex_y"],"to_end":["near_x","apex_y"]}],
        "pleats":[{"name":"p","from":["far_x","height"],"to":["three_quarters","height"]}],
        "gathers":[{"name":"g","from":["zero","zero"],"to":["width","zero"],"gathered_to":"gathered"}]"#;
    let pattern = drawn(&panel(extra, "")).expect("draws");
    let piece = &pattern.pieces[0];
    let kind = |name: &str| {
        piece
            .closures
            .iter()
            .find(|closure| closure.name == name)
            .expect(name)
    };
    assert_eq!(
        (kind("t").kind, kind("t").intake),
        (ClosureKind::Tuck, 80_000)
    );
    assert_eq!(
        (kind("p").kind, kind("p").intake),
        (ClosureKind::Pleat, 80_000)
    );
    assert_eq!(
        (kind("g").kind, kind("g").intake),
        (ClosureKind::Gather, 250_000),
        "40 cm drawn up to 15"
    );
    assert_eq!(piece.internal.len(), 2, "a tuck's fold lines are drawn");
    for report in closure_reports(&pattern) {
        assert!(report.conserved, "{report:?}");
    }
}

#[test]
fn a_gather_can_only_make_a_range_shorter() {
    let extra = r#","gathers":[{"name":"g","from":["near_x","height"],"to":["far_x","height"],"gathered_to":"gathered"}]"#;
    let (_, message) = drawn(&panel(extra, "")).expect_err("4 cm cannot draw up to 15");
    assert!(message.contains("only make a range shorter"), "{message}");
}

#[test]
fn a_closures_form_reads_back_exactly_and_a_form_without_them_is_unchanged() {
    let pattern = drawn(&panel(DART, "")).expect("draws");
    let text = sc_artifacts::to_canonical_text(&pattern);
    assert!(text.contains("\"closures\""));
    assert_eq!(
        from_canonical_text(&text).expect("reads").pieces[0].closures,
        pattern.pieces[0].closures
    );
    let plain = sc_artifacts::to_canonical_text(&drawn(&panel("", "")).expect("draws"));
    assert!(
        !plain.contains("closures"),
        "a piece with none writes no field"
    );
}

#[test]
fn a_closure_written_differently_is_a_difference_in_meaning() {
    let (a, b) = (
        panel(DART, ""),
        panel(
            &DART
                .replace("near_x", "quarter")
                .replace(r#","intake":"four_cm""#, ""),
            "",
        ),
    );
    let (first, second) = (
        sc_artifacts::to_canonical_text(&drawn(&a).expect("draws")),
        sc_artifacts::to_canonical_text(&drawn(&b).expect("draws")),
    );
    let sc_artifacts::Comparison::Different(found) =
        sc_artifacts::compare(&first, &second, 0).expect("both read")
    else {
        panic!("a different dart must differ");
    };
    assert!(
        found.iter().any(|item| item.field == "closures"),
        "{found:?}"
    );
}

const SLASH: &str = r#","slash_spreads":[{"name":"flare","pivot":["half","height"],"slash_to":["half","zero"],"spread":"four_cm"}]"#;

#[test]
fn slash_and_spread_opens_the_hem_by_the_spread_and_leaves_the_pivot_alone() {
    let pattern = drawn(&panel(SLASH, "")).expect("draws");
    let line = &pattern.pieces[0].sew;
    let (low, high) = (
        line.iter().map(|at| at.x()).min().expect("x"),
        line.iter().map(|at| at.x()).max().expect("x"),
    );
    assert!(high - low > 400_000, "the piece grew: {low} to {high}");
    let top: Vec<_> = line.iter().filter(|at| at.y() == 600_000).collect();
    assert!(
        top.iter().any(|at| at.x() == 200_000),
        "the pivot has not moved: {top:?}"
    );
    // The opening at the slash's end is the spread: the new edge from the old end to the turned one, along the hem.
    let opening = line
        .iter()
        .enumerate()
        .map(|(index, a)| {
            let b = line[(index + 1) % line.len()];
            ((b.x() - a.x()) as f64).hypot((b.y() - a.y()) as f64)
        })
        .fold(f64::MAX, |least, length| {
            if (length - 40_000.0).abs() < (least - 40_000.0).abs() {
                length
            } else {
                least
            }
        });
    assert!((opening - 40_000.0).abs() <= 1.5, "{opening}");
}

#[test]
fn slash_and_spread_is_done_again_from_the_recipe_when_a_measurement_changes() {
    // Replayable, not baked: a different height changes the turn, and the same inputs give the same bytes.
    let first = drawn(&panel(SLASH, "")).expect("draws");
    let again = drawn(&panel(SLASH, "")).expect("draws");
    assert_eq!(first, again);
    let taller = panel(SLASH, "")
        .to_text()
        .replace("\"um\": 600000", "\"um\": 700000");
    let moved = drawn(&parse(&taller).expect("valid")).expect("draws");
    assert_ne!(first.pieces[0].sew, moved.pieces[0].sew);
    assert!(
        moved.pieces[0]
            .sew
            .iter()
            .any(|at| (at.x(), at.y()) == (200_000, 700_000)),
        "the pivot follows the recipe"
    );
}

#[test]
fn what_lay_in_the_spread_part_moves_with_it() {
    let extra = format!(
        "{SLASH},\"notches\":[[[\"width\",\"mid\"],[\"three_quarters\",\"mid\"]]],\"internal\":[[[\"three_quarters\",\"zero\"],[\"three_quarters\",\"mid\"]]]"
    );
    let plain = drawn(&panel(
        ",\"notches\":[[[\"width\",\"mid\"],[\"three_quarters\",\"mid\"]]]",
        "",
    ))
    .expect("draws");
    let spread = drawn(&panel(&extra, "")).expect("draws");
    assert_ne!(
        plain.pieces[0].marks[0].at, spread.pieces[0].marks[0].at,
        "the notch moved with the part it is on"
    );
    assert!(
        spread.pieces[0].cut.contains(&spread.pieces[0].marks[0].at),
        "and it is still on the cut line"
    );
}

#[test]
fn a_slash_that_cannot_open_is_refused_by_name() {
    let far = SLASH.replace("four_cm", "width").replace(
        r#""slash_to":["half","zero"]"#,
        r#""slash_to":["half","apex_y"]"#,
    );
    let (_, message) = drawn(&panel(&far, "")).expect_err("not on the boundary");
    assert!(message.contains("slash flare"), "{message}");
    let huge = SLASH
        .replace("four_cm", "height")
        .replace(r#""pivot":["half","height"]"#, r#""pivot":["half","mid"]"#);
    assert!(drawn(&panel(&huge, "")).is_err());
}

#[test]
fn the_reference_skirts_darts_are_darts_and_each_conserves_its_four_centimetres() {
    let mut session = parse(SKIRT).expect("valid");
    let Value::Object(fields) = &mut session else {
        panic!("an object")
    };
    let Some(Value::List(steps)) = fields.get_mut("steps") else {
        panic!("a list")
    };
    let keep: Vec<Value> = steps
        .iter()
        .filter(|step| {
            step.as_object().expect("a step")["do"]
                .as_text()
                .is_some_and(|what| what == "draft" || what == "save")
        })
        .cloned()
        .collect();
    let export =
        parse(r#"{"do":"export_pattern","format":"canonical","path":"out.json"}"#).expect("valid");
    *steps = vec![keep[0].clone(), export, keep[1].clone()];
    let pattern = drawn(&session).expect("draws");
    let reports = closure_reports(&pattern);
    assert_eq!(reports.len(), 3, "front, back right and back left");
    for report in &reports {
        assert_eq!(report.intake, 40_000, "{report:?}");
        assert!(report.conserved, "{report:?}");
    }
}
