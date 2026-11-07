//! Path 1: a garment at every size of its set, each a fresh evaluation of its recipe, with nothing inherited and nothing edited by hand.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use sc_artifacts::{from_canonical_text, Pattern};
use sc_cli::{replay, Caller, CommandAuthority, Running};
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::BTreeMap;

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");
const BODICE: &str = include_str!("../../../conformance/reference-bodice/session.json");
const TROUSERS: &str = include_str!("../../../conformance/reference-trousers/session.json");
const GOLDEN_SKIRT: &str =
    include_str!("../../../conformance/goldens/reference-skirt/reference-skirt.canonical.json");
const GOLDEN_BODICE: &str =
    include_str!("../../../conformance/goldens/reference-bodice/reference-bodice.canonical.json");
const GOLDEN_TROUSERS: &str = include_str!(
    "../../../conformance/goldens/reference-trousers/reference-trousers.canonical.json"
);
const BOOK_GRADING: &str = include_str!("../../../docs/book/src/spec/grading.md");
const BOOK_BODICE: &str = include_str!("../../../docs/book/src/spec/reference-bodice.md");
const BOOK_TROUSERS: &str = include_str!("../../../docs/book/src/spec/reference-trousers.md");
const WALK_TOLERANCE: i64 = 20;
const SIZES: [(&str, i64); 5] = [("XS", -2), ("S", -1), ("M", 0), ("L", 1), ("XL", 2)];

fn fields_of(value: &Value) -> &BTreeMap<String, Value> {
    value.as_object().expect("an object")
}

fn result(record: &Value) -> Value {
    fields_of(record)["result"].clone()
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

fn input(session: &str, token: &str) -> i64 {
    fields_of(&parse(session).expect("valid"))["inputs"]
        .as_list()
        .expect("inputs")
        .iter()
        .find(|item| fields_of(item)["token"].as_text() == Some(token))
        .and_then(|item| fields_of(item)["um"].as_int())
        .unwrap_or_else(|| panic!("no input is named {token}"))
}

struct Family {
    report: Value,
    text: String,
}

fn family(session: &str) -> Family {
    let mut disk = MemoryRepository::new();
    let run = with_steps(
        session,
        r#"[{"do":"regenerate","path":"exports/family.json"},{"do":"save"}]"#,
    );
    let transcript = replay(&run, &mut disk).unwrap_or_else(|failure| panic!("{failure:?}"));
    let steps = transcript.as_list().expect("a transcript");
    let report = result(&steps[steps.len() - 2]);
    let text = String::from_utf8(
        disk.read(&FilePath::new("exports/family.json").expect("a path"))
            .expect("reads")
            .expect("written"),
    )
    .expect("text");
    Family { report, text }
}

fn members(text: &str) -> Vec<(String, Pattern)> {
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

#[test]
fn every_garment_evaluates_to_a_family_that_walks_and_closes_with_no_hand_edits() {
    for (name, session, golden) in [
        ("skirt", SKIRT, GOLDEN_SKIRT),
        ("bodice", BODICE, GOLDEN_BODICE),
        ("trousers", TROUSERS, GOLDEN_TROUSERS),
    ] {
        let family = family(session);
        assert_eq!(
            fields_of(&family.report)["sample"].as_text(),
            Some("M"),
            "{name}"
        );
        let drawn = members(&family.text);
        let names: Vec<&str> = drawn.iter().map(|(size, _)| size.as_str()).collect();
        assert_eq!(names, ["XS", "S", "M", "L", "XL"], "{name}");
        for (size, pattern) in &drawn {
            for member in &pattern.pieces {
                assert_eq!(&member.size, size, "{name}: {}", member.name);
                assert!(
                    member
                        .labels
                        .iter()
                        .all(|label| !label.text.contains("size ")
                            || label.text.contains(&format!("size {size},"))),
                    "{name} {size}: {:?}",
                    member.labels
                );
            }
            // The sample is the garment as it stands, to the byte.
            if size == "M" {
                assert_eq!(sc_artifacts::to_canonical_text(pattern), golden, "{name}");
            }
        }
        for report in fields_of(&family.report)["sizes"]
            .as_list()
            .expect("reports")
            .iter()
            .map(fields_of)
        {
            let size = report["size"].as_text().expect("a size");
            for seam in report["walk"]
                .as_list()
                .expect("a walk")
                .iter()
                .map(fields_of)
            {
                assert_ne!(
                    seam["finding"].as_text(),
                    Some("defect"),
                    "{name} {size}: {seam:?}"
                );
            }
            for closure in report["closures"]
                .as_list()
                .expect("closures")
                .iter()
                .map(fields_of)
            {
                assert!(
                    closure["gap_after_closing"]
                        .as_int()
                        .expect("a length")
                        .abs()
                        <= 1,
                    "{name} {size}: {closure:?}"
                );
            }
        }
        // Each size is different. Nothing was inherited.
        let mut digests: Vec<&str> = fields_of(&family.report)["sizes"]
            .as_list()
            .expect("reports")
            .iter()
            .map(|report| {
                fields_of(report)["canonical_sha256"]
                    .as_text()
                    .expect("a digest")
            })
            .collect();
        digests.sort_unstable();
        digests.dedup();
        assert_eq!(digests.len(), 5, "{name}");
    }
}

#[test]
fn regeneration_gives_the_same_bytes_every_time() {
    for session in [SKIRT, BODICE, TROUSERS] {
        assert_eq!(family(session).text, family(session).text);
    }
}

fn point<'a>(
    report: &'a BTreeMap<String, Value>,
    piece: &str,
    edge: &str,
    which: &str,
) -> &'a BTreeMap<String, Value> {
    report["points"]
        .as_list()
        .expect("points")
        .iter()
        .map(fields_of)
        .find(|item| {
            item["piece"].as_text() == Some(piece)
                && item["edge"].as_text() == Some(edge)
                && item["point"].as_text() == Some(which)
        })
        .unwrap_or_else(|| panic!("{piece} {edge} {which}"))
}

#[test]
fn each_size_names_the_formula_that_drove_each_point() {
    let family = family(BODICE);
    let reports = fields_of(&family.report)["sizes"]
        .as_list()
        .expect("reports");
    let at = |size: &str| {
        reports
            .iter()
            .map(fields_of)
            .find(|report| report["size"].as_text() == Some(size))
            .expect("the size")
    };
    let underarm = |size: &str| point(at(size), "bodice_front", "side_upper", "to");
    let x = fields_of(&underarm("XS")["x"]);
    assert_eq!(x["name"].as_text(), Some("front_bust_x"));
    assert_eq!(x["formula"].as_text(), Some("quarter_bust + front_shift"));
    let inputs: Vec<&str> = x["inputs"]
        .as_list()
        .expect("inputs")
        .iter()
        .filter_map(Value::as_text)
        .collect();
    assert_eq!(inputs, ["bust_girth", "ease_bust", "front_shift"]);
    // A value is at that size: a quarter of 80 mm less bust, two steps down.
    assert_eq!(x["value"].as_int(), Some(240_000));
    assert_eq!(
        fields_of(&underarm("XL")["x"])["value"].as_int(),
        Some(280_000)
    );
    assert_eq!(
        fields_of(&underarm("M")["x"])["value"].as_int(),
        Some(260_000)
    );
    // What a fit solved is an input, and it is its own formula, at the value that size came to.
    for report in reports.iter().map(fields_of) {
        let size = report["size"].as_text().expect("a size");
        let solved = report["fits"]
            .as_list()
            .expect("fits")
            .iter()
            .map(fields_of)
            .find(|fit| fit["vary"].as_text() == Some("sleeve_front_width"))
            .expect("the fit")["value"]
            .as_int();
        let x = fields_of(&point(report, "sleeve_right", "under_front", "to")["x"]).clone();
        assert_eq!(x["formula"].as_text(), Some("sleeve_front_width"), "{size}");
        assert_eq!(x["value"].as_int(), solved, "{size}");
    }
    // A position below the origin keeps its sign.
    let wrist = family_point_y(&family, "XS");
    let wrist = fields_of(&wrist);
    assert_eq!(wrist["name"].as_text(), Some("wrist_y"));
    assert!(wrist["value"].as_int().expect("a value") < 0);
}

fn family_point_y(family: &Family, size: &str) -> Value {
    let report = fields_of(&family.report)["sizes"]
        .as_list()
        .expect("reports")
        .iter()
        .map(fields_of)
        .find(|report| report["size"].as_text() == Some(size))
        .expect("the size");
    point(report, "sleeve_right", "wrist", "to")["y"].clone()
}

#[test]
fn a_size_that_breaks_a_closure_is_refused_naming_the_size_and_the_closure() {
    let whole = parse(BODICE).expect("valid");
    let mut fields = fields_of(&whole).clone();
    let mut steps = fields["steps"].as_list().expect("steps").to_vec();
    let draft = steps
        .iter_mut()
        .find(|step| fields_of(step)["do"].as_text() == Some("draft"))
        .expect("a draft");
    let mut described = fields_of(draft).clone();
    let mut sizes = fields_of(&described["sizes"]).clone();
    let mut list = sizes["list"].as_list().expect("a list").to_vec();
    let last = list.last_mut().expect("a size");
    let mut biggest = fields_of(last).clone();
    let mut inputs = fields_of(&biggest["inputs"]).clone();
    inputs.insert("upper_arm_girth".to_owned(), Value::Int(500_000));
    biggest.insert("inputs".to_owned(), Value::Object(inputs));
    *last = Value::Object(biggest);
    sizes.insert("list".to_owned(), Value::List(list));
    described.insert("sizes".to_owned(), Value::Object(sizes));
    *draft = Value::Object(described);
    fields.insert("steps".to_owned(), Value::List(steps));
    let mut disk = MemoryRepository::new();
    let run = with_steps(&Value::Object(fields).to_text(), r#"[{"do":"regenerate"}]"#);
    let failure = replay(&run, &mut disk)
        .expect_err("no sleeve clears that arm")
        .to_value();
    let said = fields_of(&failure)["message"].as_text().expect("text");
    assert!(
        said.contains("size XL") && said.contains("sleeve_clears_arm"),
        "{said}"
    );
}

#[test]
fn a_size_set_that_does_not_hang_together_is_refused_by_name() {
    for (sizes, says) in [
        (
            r#"{"sample":"Q","list":[{"name":"M"}]}"#,
            "the sample size is Q",
        ),
        (
            r#"{"sample":"M","list":[{"name":"S"}]}"#,
            "is not in the list",
        ),
        (
            r#"{"sample":"M","list":[{"name":"M","inputs":{"waist_girth":1}}]}"#,
            "the sample is the design as it stands",
        ),
        (
            r#"{"sample":"M","list":[{"name":"M"},{"name":"M"}]}"#,
            "listed twice",
        ),
        (
            r#"{"sample":"M","list":[{"name":"M"},{"name":"L","inputs":{"nothing":1}}]}"#,
            "no input has that name",
        ),
        (
            r#"{"sample":"M","list":[{"name":"M"},{"name":"L","inputs":{"waist_girth":"big"}}]}"#,
            "not micrometres",
        ),
    ] {
        let whole = parse(SKIRT).expect("valid");
        let mut fields = fields_of(&whole).clone();
        let mut steps = fields["steps"].as_list().expect("steps").to_vec();
        let draft = steps
            .iter_mut()
            .find(|step| fields_of(step)["do"].as_text() == Some("draft"))
            .expect("a draft");
        let mut described = fields_of(draft).clone();
        described.insert("sizes".to_owned(), parse(sizes).expect("valid"));
        *draft = Value::Object(described);
        fields.insert("steps".to_owned(), Value::List(steps));
        let run = with_steps(&Value::Object(fields).to_text(), r#"[{"do":"regenerate"}]"#);
        let failure = replay(&run, MemoryRepository::new())
            .expect_err(says)
            .to_value();
        let said = fields_of(&failure)["message"].as_text().expect("text");
        assert!(said.contains(says), "{sizes}: {said}");
    }
}

#[test]
fn regenerating_changes_nothing_in_the_design() {
    let whole = parse(BODICE).expect("valid");
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
    let revision = running.revision();
    let before = running.pattern().expect("drawn").clone();
    let asked = parse(r#"{"do":"evaluate"}"#).expect("valid");
    let evaluated = result(&running.step(&asked).expect("evaluates"));
    running
        .step(&parse(r#"{"do":"regenerate"}"#).expect("valid"))
        .expect("regenerates");
    assert_eq!(running.revision(), revision);
    assert_eq!(running.pattern().expect("still drawn"), &before);
    assert_eq!(result(&running.step(&asked).expect("evaluates")), evaluated);
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
        .step(&parse(r#"{"do":"regenerate"}"#).expect("valid"))
        .is_err());
}

fn x_at_level(line: &[sc_geometry::Point], y: i64) -> i64 {
    line.iter()
        .filter(|at| at.y() == y)
        .map(|at| at.x())
        .max()
        .unwrap_or_else(|| panic!("nothing at the level {y}"))
}

// The check that matters: what each size measures, read off its drawing, is what that size asks for.
#[test]
fn every_size_measures_what_it_asks_for_read_from_its_drawing() {
    let walked = |report: &BTreeMap<String, Value>, names: &[&str]| -> i64 {
        report["walk"]
            .as_list()
            .expect("a walk")
            .iter()
            .map(fields_of)
            .filter(|seam| names.contains(&seam["seam"].as_text().expect("a name")))
            .map(|seam| seam["b_length"].as_int().expect("a length"))
            .sum()
    };
    let for_size = |family: &Family, size: &str| {
        fields_of(&family.report)["sizes"]
            .as_list()
            .expect("reports")
            .iter()
            .map(fields_of)
            .find(|report| report["size"].as_text() == Some(size))
            .expect("the size")
            .clone()
    };

    let trousers = family(TROUSERS);
    for (size, pattern) in members(&trousers.text) {
        let steps = SIZES
            .iter()
            .find(|(name, _)| *name == size)
            .expect("a size")
            .1;
        let waist = input(TROUSERS, "waist_girth") + input(TROUSERS, "ease_waist") + 40_000 * steps;
        let hip = input(TROUSERS, "hip_girth") + input(TROUSERS, "ease_hip") + 40_000 * steps;
        let level = -(input(TROUSERS, "waist_to_hip") + 5_000 * steps);
        let report = for_size(&trousers, &size);
        let sewn = walked(
            &report,
            &[
                "band_back_left",
                "band_front_left",
                "band_front_right",
                "band_back_right",
            ],
        );
        assert!(
            (sewn - waist).abs() <= 4 * WALK_TOLERANCE,
            "{size}: the waist is {sewn}, and {waist} was asked for"
        );
        let across = x_at_level(&piece(&pattern, "trouser_front_right").sew, level)
            + x_at_level(&piece(&pattern, "trouser_back_right").sew, level);
        assert_eq!(2 * across, hip, "{size}: the hip");
        let long = piece(&pattern, "trouser_front_right")
            .sew
            .iter()
            .map(|at| at.y())
            .min()
            .expect("a line");
        assert_eq!(
            -long,
            input(TROUSERS, "waist_to_hem") + 10_000 * steps,
            "{size}: the length"
        );
    }

    let bodice = family(BODICE);
    for (size, pattern) in members(&bodice.text) {
        let steps = SIZES
            .iter()
            .find(|(name, _)| *name == size)
            .expect("a size")
            .1;
        let bust = input(BODICE, "bust_girth") + input(BODICE, "ease_bust") + 40_000 * steps;
        let widest = |name: &str| {
            piece(&pattern, name)
                .sew
                .iter()
                .map(|at| at.x())
                .max()
                .expect("a line")
        };
        assert_eq!(
            2 * (widest("bodice_front") + widest("bodice_back_right")),
            bust,
            "{size}: the bust"
        );
        let sleeve = &piece(&pattern, "sleeve_right").sew;
        let (low, high) = (
            sleeve.iter().map(|at| at.y()).min().expect("a line"),
            sleeve.iter().map(|at| at.y()).max().expect("a line"),
        );
        assert_eq!(
            high - low,
            input(BODICE, "sleeve_length") + 5_000 * steps,
            "{size}: the sleeve"
        );
        let across = sleeve.iter().map(|at| at.x()).max().expect("a line")
            - sleeve.iter().map(|at| at.x()).min().expect("a line");
        assert!(
            across
                >= input(BODICE, "upper_arm_girth")
                    + input(BODICE, "ease_upper_arm")
                    + 15_000 * steps,
            "{size}: {across}"
        );
    }

    let skirt = family(SKIRT);
    for (size, pattern) in members(&skirt.text) {
        let steps = SIZES
            .iter()
            .find(|(name, _)| *name == size)
            .expect("a size")
            .1;
        let hip = input(SKIRT, "hip_girth") + input(SKIRT, "ease_hip") + 40_000 * steps;
        let level = -input(SKIRT, "waist_to_hip");
        assert_eq!(
            4 * x_at_level(&piece(&pattern, "skirt_front").sew, level),
            hip,
            "{size}: the hip"
        );
        let long = piece(&pattern, "skirt_front")
            .sew
            .iter()
            .map(|at| at.y())
            .min()
            .expect("a line");
        assert_eq!(
            -long,
            input(SKIRT, "waist_to_hem") + 10_000 * steps,
            "{size}: the length"
        );
        let waist = input(SKIRT, "waist_girth") + input(SKIRT, "ease_waist") + 40_000 * steps;
        let report = for_size(&skirt, &size);
        let sewn = walked(
            &report,
            &[
                "waist_back_left",
                "waist_front_left",
                "waist_front_right",
                "waist_back_right",
            ],
        );
        assert!(
            (sewn - waist).abs() <= 4 * WALK_TOLERANCE,
            "{size}: the waist is {sewn}, and {waist} was asked for"
        );
    }
}

// A size set is data in the session and a table in the book, and the two are not allowed to drift.
#[test]
fn the_sizes_each_session_declares_are_the_steps_its_chapter_states() {
    let stated = |book: &str, session: &str| {
        let mut steps = BTreeMap::new();
        for line in book.lines() {
            let cells: Vec<&str> = line.split('|').map(str::trim).collect();
            let [_, token, change, ..] = cells.as_slice() else {
                continue;
            };
            if let (Some(token), Some(change)) = (
                token
                    .strip_prefix('`')
                    .and_then(|token| token.strip_suffix('`')),
                change.strip_suffix(" mm change"),
            ) {
                let change: f64 = change.parse().expect("a number");
                steps.insert(token.to_owned(), (change * 1000.0).round() as i64);
            }
        }
        assert!(!steps.is_empty());
        let whole = parse(session).expect("valid");
        let listed = fields_of(&whole)["steps"]
            .as_list()
            .expect("steps")
            .iter()
            .find(|step| fields_of(step)["do"].as_text() == Some("draft"))
            .map(|draft| {
                fields_of(&fields_of(draft)["sizes"])["list"]
                    .as_list()
                    .expect("a list")
                    .to_vec()
            })
            .expect("a draft");
        for (name, count) in SIZES {
            let size = listed
                .iter()
                .map(fields_of)
                .find(|size| size["name"].as_text() == Some(name))
                .expect("the size");
            if count == 0 {
                assert!(!size.contains_key("inputs"));
                continue;
            }
            let inputs = fields_of(&size["inputs"]);
            assert_eq!(inputs.len(), steps.len(), "{name}");
            for (token, change) in &steps {
                assert_eq!(
                    inputs[token].as_int(),
                    Some(input(session, token) + change * count),
                    "{name}: {token}"
                );
            }
        }
    };
    stated(BOOK_GRADING, SKIRT);
    stated(BOOK_BODICE, BODICE);
    stated(BOOK_TROUSERS, TROUSERS);
}
