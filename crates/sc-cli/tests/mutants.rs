//! Mutation tests: each fault is injected on purpose, and each must be caught by the check that names it. A fault caught by something else
//! nearby is a failure here, because a harness that passes for the wrong reason would pass again when the right check is gone.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_artifacts::{
    from_canonical_text, judge_dxf, judge_inputs, judge_pattern, judge_pdf_square, Expectations,
    Finding,
};
use sc_cli::replay;
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::{BTreeMap, BTreeSet};

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");
const DECLARED: &str = include_str!("../../../conformance/reference-skirt/expectations.json");
const CHAPTER: &str = include_str!("../../../docs/book/src/spec/mutation-tests.md");

// What the run produced that a mutant may then damage.
struct Files {
    dxf: String,
    pdf: Vec<u8>,
}

type SessionFault = fn(&mut Value);
type DeclaredFault = fn(&mut Value);
type FileFault = fn(&mut Files);

struct Mutant {
    name: &'static str,
    family: &'static str,
    session: Option<SessionFault>,
    declared: Option<DeclaredFault>,
    files: Option<FileFault>,
    // The code the first finding must carry, and a word it must name.
    code: &'static str,
    names: &'static str,
}

fn fields_of(value: &mut Value) -> &mut BTreeMap<String, Value> {
    match value {
        Value::Object(fields) => fields,
        other => panic!("{other:?} is not an object"),
    }
}

fn list_of<'a>(fields: &'a mut BTreeMap<String, Value>, name: &str) -> &'a mut Vec<Value> {
    match fields.get_mut(name) {
        Some(Value::List(items)) => items,
        other => panic!("{name}: {other:?} is not a list"),
    }
}

fn set_input(session: &mut Value, token: &str, um: i64) {
    for input in list_of(fields_of(session), "inputs") {
        let input = fields_of(input);
        if input["token"].as_text() == Some(token) {
            input.insert("um".to_owned(), Value::Int(um));
            return;
        }
    }
    panic!("no input {token}");
}

fn replace_recipe(session: &mut Value, from: &str, to: &str) {
    let fields = fields_of(session);
    let recipe = fields["recipe"].as_text().expect("text").to_owned();
    assert!(recipe.contains(from), "the recipe has no {from}");
    fields.insert("recipe".to_owned(), Value::Text(recipe.replace(from, to)));
}

fn pieces(session: &mut Value) -> &mut Vec<Value> {
    for step in list_of(fields_of(session), "steps") {
        if step.as_object().expect("a step")["do"].as_text() == Some("draft") {
            return list_of(fields_of(step), "pieces");
        }
    }
    panic!("no draft");
}

fn piece<'a>(session: &'a mut Value, name: &str) -> &'a mut BTreeMap<String, Value> {
    for item in pieces(session) {
        if item.as_object().expect("a piece")["name"].as_text() == Some(name) {
            return fields_of(item);
        }
    }
    panic!("no piece {name}");
}

fn drop_piece(session: &mut Value, name: &str) {
    pieces(session)
        .retain(|item| item.as_object().expect("a piece")["name"].as_text() != Some(name));
}

fn points(text: &str) -> Value {
    parse(text).expect("valid points")
}

fn set_band_boundary(session: &mut Value, start: &str, corners: &[&str]) {
    let band = piece(session, "waistband");
    band.insert("start".to_owned(), points(start));
    let edges = list_of(band, "edges");
    for (edge, corner) in edges.iter_mut().zip(corners) {
        fields_of(edge).insert("to".to_owned(), points(corner));
    }
}

fn span<'a>(declared: &'a mut Value, name: &str) -> &'a mut BTreeMap<String, Value> {
    for item in list_of(fields_of(declared), "spans") {
        if item.as_object().expect("a span")["name"].as_text() == Some(name) {
            return fields_of(item);
        }
    }
    panic!("no span {name}");
}

fn flip_side(side: &mut BTreeMap<String, Value>) {
    // The same stretch, walked the other way: its ends swap, and it now runs against or with the line's order.
    let (from, to) = (side["from"].clone(), side["to"].clone());
    side.insert("from".to_owned(), to);
    side.insert("to".to_owned(), from);
    let against = matches!(side.get("against"), Some(Value::Bool(true)));
    side.insert("against".to_owned(), Value::Bool(!against));
}

const fn on_session(
    name: &'static str,
    family: &'static str,
    fault: SessionFault,
    code: &'static str,
    names: &'static str,
) -> Mutant {
    Mutant {
        name,
        family,
        session: Some(fault),
        declared: None,
        files: None,
        code,
        names,
    }
}

const fn on_declared(
    name: &'static str,
    family: &'static str,
    fault: DeclaredFault,
    code: &'static str,
    names: &'static str,
) -> Mutant {
    Mutant {
        name,
        family,
        session: None,
        declared: Some(fault),
        files: None,
        code,
        names,
    }
}

const fn on_files(
    name: &'static str,
    family: &'static str,
    fault: FileFault,
    code: &'static str,
    names: &'static str,
) -> Mutant {
    Mutant {
        name,
        family,
        session: None,
        declared: None,
        files: Some(fault),
        code,
        names,
    }
}

fn scale_dxf_vertices(text: &str, divide: f64) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::with_capacity(lines.len());
    let mut kind = "";
    for pair in lines.chunks(2) {
        let (code, value) = (pair[0], pair[1]);
        if code.trim() == "0" {
            kind = value;
        }
        out.push(code.to_owned());
        if kind == "VERTEX" && (code.trim() == "10" || code.trim() == "20") {
            out.push(format!(
                "{:.4}",
                value.trim().parse::<f64>().expect("a number") / divide
            ));
        } else {
            out.push(value.to_owned());
        }
    }
    out.join("\n") + "\n"
}

fn swap_dxf_layers(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::with_capacity(lines.len());
    let mut kind = "";
    for pair in lines.chunks(2) {
        let (code, value) = (pair[0], pair[1]);
        if code.trim() == "0" {
            kind = value;
        }
        out.push(code);
        out.push(match (kind, code.trim(), value.trim()) {
            ("POLYLINE" | "VERTEX" | "SEQEND", "8", "CUT") => "DRAW",
            ("POLYLINE" | "VERTEX" | "SEQEND", "8", "DRAW") => "CUT",
            _ => value,
        });
    }
    out.join("\n") + "\n"
}

fn mutants() -> Vec<Mutant> {
    vec![
        on_session(
            "hip_girth_as_millimetres",
            "wrong units",
            |s| set_input(s, "hip_girth", 98_000),
            "wrong_units",
            "hip_girth",
        ),
        on_session(
            "hip_girth_ten_times",
            "wrong units",
            |s| set_input(s, "hip_girth", 9_800_000),
            "wrong_units",
            "hip_girth",
        ),
        on_session(
            "waist_to_hem_in_inches",
            "wrong units",
            |s| set_input(s, "waist_to_hem", 24_410),
            "wrong_units",
            "waist_to_hem",
        ),
        on_session(
            "notch_literal_in_centimetres",
            "wrong units",
            |s| {
                replace_recipe(
                    s,
                    "let notch_depth: length = 5 mm",
                    "let notch_depth: length = 5 cm",
                )
            },
            "wrong_notch_depth",
            "50000",
        ),
        on_files(
            "dxf_written_in_inches",
            "wrong units",
            |f| f.dxf = scale_dxf_vertices(&f.dxf, 25.4),
            "wrong_units",
            "times the pattern",
        ),
        on_files(
            "pdf_square_shrunk",
            "wrong units",
            |f| {
                let text = String::from_utf8_lossy(&f.pdf).into_owned();
                let at = "283.4646 283.4646 re S";
                assert!(text.contains(at), "the file has no square as written");
                f.pdf = text.replacen(at, "280.6299 280.6299 re S", 1).into_bytes();
            },
            "scale_square_wrong",
            "99.000",
        ),
        on_session(
            "front_hip_notch_missing",
            "missing notch",
            |s| {
                list_of(piece(s, "skirt_front"), "notches").clear();
            },
            "missing_notch",
            "skirt_front",
        ),
        on_session(
            "zipper_notch_missing",
            "missing notch",
            |s| {
                list_of(piece(s, "skirt_back_right"), "notches").remove(1);
            },
            "missing_notch",
            "skirt_back_right",
        ),
        on_session(
            "band_quarter_notch_missing",
            "missing notch",
            |s| {
                list_of(piece(s, "waistband"), "notches").remove(2);
            },
            "missing_notch",
            "waistband",
        ),
        on_session(
            "hip_notch_ten_millimetres_off",
            "missing notch",
            |s| {
                replace_recipe(
                    s,
                    "let notch_depth: length = 5 mm",
                    "let notch_depth: length = 5 mm\nlet moved_hip: length = waist_to_hip - 10 mm",
                );
                let back = piece(s, "skirt_back_right");
                let notches = list_of(back, "notches");
                notches[0] =
                    points(r#"[["hip_notch_x","-moved_hip"],["hip_notch_end","-moved_hip"]]"#);
            },
            "notch_misaligned",
            "ss_right",
        ),
        on_session(
            "boundary_does_not_close",
            "invalid boundary",
            |s| {
                let band = piece(s, "waistband");
                let edges = list_of(band, "edges");
                let last = edges.last_mut().expect("edges");
                fields_of(last).insert("to".to_owned(), points(r#"["sa_cb","band_top"]"#));
            },
            "draft",
            "waistband",
        ),
        on_session(
            "boundary_runs_clockwise",
            "invalid boundary",
            |s| {
                set_band_boundary(
                    s,
                    r#"["sa_cb","sa_waist"]"#,
                    &[
                        r#"["sa_cb","band_top"]"#,
                        r#"["band_right","band_top"]"#,
                        r#"["band_right","sa_waist"]"#,
                        r#"["sa_cb","sa_waist"]"#,
                    ],
                );
            },
            "draft",
            "counterclockwise",
        ),
        on_session(
            "boundary_crosses_itself",
            "invalid boundary",
            |s| {
                set_band_boundary(
                    s,
                    r#"["sa_cb","sa_waist"]"#,
                    &[
                        r#"["band_right","band_top"]"#,
                        r#"["band_right","sa_waist"]"#,
                        r#"["sa_cb","band_top"]"#,
                        r#"["sa_cb","sa_waist"]"#,
                    ],
                );
            },
            "draft",
            "cross",
        ),
        on_session(
            "allowance_is_negative",
            "invalid boundary",
            |s| set_input(s, "sa_side", -10_000),
            "implausible_input",
            "sa_side",
        ),
        on_session(
            "waist_construction_wrong",
            "invalid boundary",
            |s| {
                replace_recipe(
                    s,
                    "let waist_side: length = quarter_hip - ss_suppress",
                    "let waist_side: length = quarter_waist - ss_suppress",
                );
            },
            "recipe",
            "waist_closure",
        ),
        on_declared(
            "side_seam_sewn_end_for_end",
            "reversed seam",
            |d| flip_side(fields_of(span(d, "ss_right").get_mut("a").expect("a side"))),
            "reversed_correspondence",
            "ss_right",
        ),
        on_declared(
            "centre_back_sewn_end_for_end",
            "reversed seam",
            |d| flip_side(fields_of(span(d, "cb").get_mut("b").expect("a side"))),
            "reversed_correspondence",
            "cb",
        ),
        on_declared(
            "side_seam_sewn_to_the_wrong_edge",
            "reversed seam",
            |d| {
                let side = fields_of(span(d, "ss_left").get_mut("b").expect("a side"));
                side.insert("from".to_owned(), points("[0, 0]"));
                side.insert("to".to_owned(), points("[0, -620000]"));
            },
            "seam_length",
            "ss_left",
        ),
        on_session(
            "interfacing_omitted",
            "omitted piece",
            |s| drop_piece(s, "waistband_interfacing"),
            "omitted_piece",
            "waistband_interfacing",
        ),
        on_session(
            "back_left_omitted",
            "omitted piece",
            |s| drop_piece(s, "skirt_back_left"),
            "omitted_piece",
            "skirt_back_left",
        ),
        on_session(
            "front_drafted_twice",
            "omitted piece",
            |s| {
                let copy = pieces(s)[0].clone();
                pieces(s).push(copy);
            },
            "duplicate_piece",
            "skirt_",
        ),
        on_session(
            "a_piece_nobody_asked_for",
            "omitted piece",
            |s| {
                let mut copy = pieces(s)[0].clone();
                fields_of(&mut copy).insert("name".to_owned(), Value::Text("pocket".to_owned()));
                pieces(s).push(copy);
            },
            "unexpected_piece",
            "pocket",
        ),
        on_declared(
            "a_piece_nobody_sews",
            "omitted piece",
            |d| {
                list_of(fields_of(d), "spans").retain(|item| {
                    !item.as_object().expect("a span")["name"]
                        .as_text()
                        .expect("text")
                        .starts_with("waist_")
                });
            },
            "unaccounted_piece",
            "waistband",
        ),
        on_session(
            "band_label_says_fold",
            "wrong marking",
            |s| {
                let labels = list_of(piece(s, "waistband"), "labels");
                fields_of(&mut labels[0]).insert(
                    "text".to_owned(),
                    Value::Text("waistband, size M, cut 1, shell, place on fold".to_owned()),
                );
            },
            "label_forbidden",
            "waistband",
        ),
        on_session(
            "front_label_loses_fold",
            "wrong marking",
            |s| {
                list_of(piece(s, "skirt_front"), "labels").retain(|label| {
                    !label.as_object().expect("a label")["text"]
                        .as_text()
                        .expect("text")
                        .contains("fold")
                });
            },
            "label_missing",
            "place on fold",
        ),
        on_session(
            "band_fold_missing",
            "wrong marking",
            |s| {
                list_of(piece(s, "waistband"), "internal").clear();
            },
            "fold_not_internal",
            "waistband",
        ),
        on_session(
            "grain_turned_across",
            "wrong marking",
            |s| {
                let grain = points(r#"[[["zero","label_y"],["grain_x","label_y"]]]"#);
                piece(s, "skirt_front").insert("grain".to_owned(), grain);
            },
            "grain_off",
            "skirt_front",
        ),
        on_session(
            "cut_quantity_wrong",
            "wrong marking",
            |s| {
                piece(s, "waistband").insert("quantity".to_owned(), Value::Int(2));
            },
            "wrong_quantity",
            "waistband",
        ),
        on_session(
            "hem_allowance_a_millimetre_off",
            "wrong dimension",
            |s| set_input(s, "sa_hem", 31_000),
            "wrong_dimension",
            "skirt_front",
        ),
        on_session(
            "allowance_on_the_fused_piece",
            "wrong dimension",
            |s| {
                let band = piece(s, "waistband_interfacing");
                for edge in list_of(band, "edges") {
                    fields_of(edge).insert("width".to_owned(), Value::Text("sa_side".to_owned()));
                }
            },
            "allowance_on_fused_piece",
            "waistband_interfacing",
        ),
        on_files(
            "cut_and_sew_swapped_in_the_file",
            "wrong dimension",
            |f| f.dxf = swap_dxf_layers(&f.dxf),
            "cut_and_sew_swapped",
            "skirt_",
        ),
    ]
}

// A refused step, as what it said. Anything but a step that exists to refuse is the harness's own fault.
fn refused(failure: &sc_cli::Failure) -> Vec<Finding> {
    let value = failure.to_value();
    let fields = value.as_object().expect("a diagnostic");
    let code = match fields["failed"].as_text().expect("text") {
        "draft" => "draft",
        "recipe" => "recipe",
        "export_pattern" => "export_pattern",
        other => panic!("refused for an unrelated reason, {other}: {value:?}"),
    };
    vec![Finding {
        code,
        detail: fields["message"].as_text().expect("text").to_owned(),
    }]
}

// Runs the whole pipeline in order and returns the first thing anything said: a finding about the inputs, a refused step, a finding about the drawing, then about the files.
fn judged(mutant: Option<&Mutant>) -> Vec<Finding> {
    let mut session = parse(SKIRT).expect("valid");
    let mut declared = parse(DECLARED).expect("valid");
    if let Some(fault) = mutant.and_then(|m| m.session) {
        fault(&mut session);
    }
    if let Some(fault) = mutant.and_then(|m| m.declared) {
        fault(&mut declared);
    }
    let expected = Expectations::from_text(&declared.to_text()).expect("a declaration");
    let inputs: Vec<(String, i64)> = session.as_object().expect("a session")["inputs"]
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
    let found = judge_inputs(&inputs, &expected);
    if !found.is_empty() {
        return found;
    }
    // The drawing first, on a session that writes only the canonical form, so that a piece a writer would refuse is still named for what it is.
    let mut drawing = session.clone();
    list_of(fields_of(&mut drawing), "steps").retain(|step| {
        let step = step.as_object().expect("a step");
        step["do"].as_text() != Some("export_pattern")
            || step.get("format").and_then(Value::as_text) == Some("canonical")
    });
    let mut disk = MemoryRepository::new();
    if let Err(failure) = replay(&drawing, &mut disk) {
        return refused(&failure);
    }
    let bytes = |disk: &MemoryRepository, name: &str| {
        disk.read(&FilePath::new(name).expect("a path"))
            .expect("reads")
            .unwrap_or_else(|| panic!("no {name}"))
    };
    let pattern = from_canonical_text(
        &String::from_utf8(bytes(&disk, "exports/reference-skirt.canonical.json")).expect("text"),
    )
    .expect("reads");
    let found = judge_pattern(&pattern, &expected);
    if !found.is_empty() {
        return found;
    }
    let mut disk = MemoryRepository::new();
    if let Err(failure) = replay(&session, &mut disk) {
        return refused(&failure);
    }
    let read = |name: &str| bytes(&disk, name);
    let mut files = Files {
        dxf: String::from_utf8(read("exports/reference-skirt.dxf")).expect("text"),
        pdf: read("exports/reference-skirt.pdf"),
    };
    if let Some(fault) = mutant.and_then(|m| m.files) {
        fault(&mut files);
    }
    let mut found = judge_dxf(&files.dxf, &pattern, true);
    found.extend(judge_pdf_square(&files.pdf));
    found
}

#[test]
fn the_fixture_itself_raises_nothing() {
    assert_eq!(judged(None), Vec::new());
}

#[test]
fn every_mutant_is_caught_by_the_check_that_names_it() {
    let mut wrong = Vec::new();
    for mutant in mutants() {
        let found = judged(Some(&mutant));
        match found.first() {
            None => wrong.push(format!("{}: nothing caught it", mutant.name)),
            Some(first) if first.code != mutant.code => wrong.push(format!(
                "{}: caught as {}, and it should be {}: {}",
                mutant.name, first.code, mutant.code, first.detail
            )),
            Some(first) if !first.detail.contains(mutant.names) => wrong.push(format!(
                "{}: {} does not name {}: {}",
                mutant.name, first.code, mutant.names, first.detail
            )),
            Some(_) => {}
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn the_book_lists_every_mutant_and_no_other() {
    let listed: BTreeSet<&str> = CHAPTER
        .lines()
        .filter_map(|line| {
            let mut cells = line.split('|').map(str::trim);
            cells.next()?;
            let name = cells.next()?.strip_prefix('`')?.strip_suffix('`')?;
            let family = cells.next()?;
            [
                "wrong units",
                "missing notch",
                "invalid boundary",
                "reversed seam",
                "omitted piece",
                "wrong marking",
                "wrong dimension",
            ]
            .contains(&family)
            .then_some(name)
        })
        .collect();
    let all = mutants();
    let names: BTreeSet<&str> = all.iter().map(|mutant| mutant.name).collect();
    assert_eq!(listed, names);
    for mutant in &all {
        let row = format!(
            "| `{}` | {} | `{}` |",
            mutant.name, mutant.family, mutant.code
        );
        assert!(CHAPTER.contains(&row), "the chapter has no row {row}");
    }
}

#[test]
fn every_family_the_roadmap_names_has_mutants() {
    let families: BTreeSet<&str> = mutants().iter().map(|mutant| mutant.family).collect();
    for family in [
        "wrong units",
        "missing notch",
        "invalid boundary",
        "reversed seam",
        "omitted piece",
    ] {
        assert!(families.contains(family), "{family}");
    }
}

#[test]
fn a_seam_with_equal_lengths_sewn_end_for_end_passes_a_length_check_alone() {
    // This is why the notch is the check. Without one declared, the reversed seam below is invisible, which a fixture must not rely on.
    let mut declared = parse(DECLARED).expect("valid");
    flip_side(fields_of(
        span(&mut declared, "ss_right")
            .get_mut("a")
            .expect("a side"),
    ));
    list_of(span(&mut declared, "ss_right"), "notches_at").clear();
    let expected = Expectations::from_text(&declared.to_text()).expect("a declaration");
    let mut disk = MemoryRepository::new();
    replay(&parse(SKIRT).expect("valid"), &mut disk)
        .unwrap_or_else(|failure| panic!("{failure:?}"));
    let bytes = disk
        .read(&FilePath::new("exports/reference-skirt.canonical.json").expect("a path"))
        .expect("reads")
        .expect("exported");
    let pattern = from_canonical_text(&String::from_utf8(bytes).expect("text")).expect("reads");
    assert_eq!(judge_pattern(&pattern, &expected), Vec::new());
}

#[test]
fn a_drawing_too_large_for_the_sheet_is_a_refusal_with_its_own_reason() {
    // Not a mutant of the design: the design is fine and the paper is small, so it must be told apart from every fault above.
    let mut session = parse(SKIRT).expect("valid");
    set_input(&mut session, "waist_to_hem", 1_000_000);
    let failure = replay(&session, MemoryRepository::new()).expect_err("too long for one sheet");
    let found = refused(&failure);
    assert_eq!(found[0].code, "export_pattern");
    assert!(found[0].detail.contains("never rescaled"), "{found:?}");
}
