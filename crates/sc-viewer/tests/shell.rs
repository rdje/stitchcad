//! The dev shell on the reference skirt: every edit is a bus step, the recording replays to the same result, and looking changes nothing.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use sc_cli::{replay, STEPS};
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use sc_viewer::shell::{Shell, COVERAGE, PIXEL_UNITS};
use std::collections::BTreeMap;

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");
const CHAPTER: &str = include_str!("../../../docs/book/src/spec/dev-shell.md");
const SIZE: (i64, i64) = (1400, 900);

fn open() -> Shell {
    Shell::open(SKIRT, SIZE).expect("the skirt opens")
}

fn act(shell: &mut Shell, action: &str) -> BTreeMap<String, Value> {
    shell.act(action).as_object().expect("a state").clone()
}

fn status(state: &BTreeMap<String, Value>) -> &str {
    state["status"].as_text().expect("text")
}

fn failed(state: &BTreeMap<String, Value>) -> bool {
    state["failed"] == Value::Bool(true)
}

fn input(state: &BTreeMap<String, Value>, token: &str) -> i64 {
    state["inputs"]
        .as_list()
        .expect("inputs")
        .iter()
        .map(|item| item.as_object().expect("an input"))
        .find(|item| item["token"].as_text() == Some(token))
        .expect("the input")["um"]
        .as_int()
        .expect("a number")
}

// Each cut line, in pixels, as the page would draw it.
fn cuts(state: &BTreeMap<String, Value>) -> Vec<Vec<(f64, f64)>> {
    state["lines"]
        .as_list()
        .expect("lines")
        .iter()
        .map(|item| item.as_object().expect("a line"))
        .filter(|item| item["kind"].as_text() == Some("cut"))
        .map(|item| {
            item["points"]
                .as_list()
                .expect("points")
                .chunks(2)
                .map(|pair| {
                    (
                        pair[0].as_int().expect("x") as f64 / PIXEL_UNITS as f64,
                        pair[1].as_int().expect("y") as f64 / PIXEL_UNITS as f64,
                    )
                })
                .collect()
        })
        .collect()
}

fn file_of(disk: &MemoryRepository, name: &str) -> Vec<u8> {
    disk.read(&FilePath::new(name).expect("a path"))
        .expect("reads")
        .unwrap_or_else(|| panic!("{name} is missing"))
}

fn hex(text: &str) -> Vec<u8> {
    text.as_bytes()
        .chunks(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("text"), 16).expect("hex"))
        .collect()
}

#[test]
fn the_book_and_the_shell_name_every_step_a_session_can_take() {
    let mut named: Vec<&str> = COVERAGE.iter().map(|(step, _)| *step).collect();
    let mut steps: Vec<&str> = STEPS.to_vec();
    named.sort_unstable();
    steps.sort_unstable();
    assert_eq!(named, steps, "a new step must say how the shell reaches it");
    for (step, how) in COVERAGE {
        let row = format!("| `{step}` | {how} |");
        assert!(CHAPTER.contains(&row), "the chapter has no row {row}");
    }
}

#[test]
fn every_step_the_list_names_is_one_the_session_knows_and_no_other() {
    for step in STEPS {
        let text = format!("{{\"format\":1,\"design\":\"01ARZ3NDEKTSV4RRFFQ69G5FAV\",\"inputs\":[],\"recipe\":\"\",\"steps\":[{{\"do\":\"{step}\"}}]}}");
        if let Err(failure) = replay(&parse(&text).expect("valid"), MemoryRepository::new()) {
            let value = failure.to_value();
            let message = value.as_object().expect("a diagnostic")["message"]
                .as_text()
                .expect("text")
                .to_owned();
            assert!(!message.contains("no step is named"), "{step}: {message}");
        }
    }
    let unknown = r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV","inputs":[],"recipe":"","steps":[{"do":"nonsense"}]}"#;
    let failure = replay(&parse(unknown).expect("valid"), MemoryRepository::new())
        .expect_err("refused")
        .to_value();
    assert!(failure.as_object().expect("a diagnostic")["message"]
        .as_text()
        .expect("text")
        .contains("no step is named"));
}

#[test]
fn opening_draws_all_five_pieces_inside_the_canvas() {
    let shell = open();
    let state = shell.state();
    let state = state.as_object().expect("a state");
    let pieces = cuts(state);
    assert_eq!(pieces.len(), 5);
    for piece in &pieces {
        for (x, y) in piece {
            assert!(
                (0.0..=SIZE.0 as f64).contains(x) && (0.0..=SIZE.1 as f64).contains(y),
                "({x}, {y})"
            );
        }
    }
    assert!(state["labels"].as_list().expect("labels").len() >= 5);
    let kinds: Vec<&str> = state["lines"]
        .as_list()
        .expect("lines")
        .iter()
        .map(|line| {
            line.as_object().expect("a line")["kind"]
                .as_text()
                .expect("text")
        })
        .collect();
    for kind in ["cut", "sew", "grain", "internal", "notch"] {
        assert!(kinds.contains(&kind), "{kind}");
    }
}

#[test]
fn a_session_with_no_draft_step_is_refused_by_name() {
    let reason = Shell::open(
        r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV","inputs":[],"recipe":"","steps":[]}"#,
        SIZE,
    )
    .expect_err("nothing to draw");
    assert!(reason.contains("draft"), "{reason}");
}

#[test]
fn what_the_shell_did_replays_through_the_command_line_to_identical_files() {
    let mut shell = open();
    for action in [
        r#"{"do":"change","token":"hip_girth","um":1000000}"#,
        r#"{"do":"change","token":"waist_to_hem","um":650000}"#,
        r#"{"do":"undo"}"#,
        r#"{"do":"redo"}"#,
        r#"{"do":"change","token":"sa_hem","um":40000}"#,
        r#"{"do":"undo"}"#,
    ] {
        let state = act(&mut shell, action);
        assert!(!failed(&state), "{action}: {}", status(&state));
    }
    // Looking around, between edits, adds nothing to what is replayed.
    let before = shell.recording();
    act(&mut shell, r#"{"do":"zoom","closer":true,"x":300,"y":200}"#);
    act(&mut shell, r#"{"do":"pan","x":40,"y":-30}"#);
    act(&mut shell, r#"{"do":"pick","x":300,"y":200}"#);
    act(&mut shell, r#"{"do":"fit"}"#);
    assert_eq!(shell.recording(), before);
    let canonical = act(
        &mut shell,
        r#"{"do":"export_pattern","format":"canonical","path":"exports/shell.json"}"#,
    );
    let dxf = act(
        &mut shell,
        r#"{"do":"export_pattern","format":"dxf","semantics":"cut_as_1","path":"exports/shell.dxf"}"#,
    );
    let pdf = act(
        &mut shell,
        r#"{"do":"export_pattern","format":"pdf","layout":"a4_tiled","path":"exports/shell.pdf"}"#,
    );

    let mut disk = MemoryRepository::new();
    let transcript =
        replay(&shell.recording(), &mut disk).unwrap_or_else(|failure| panic!("{failure:?}"));
    let last = transcript
        .as_list()
        .expect("a transcript")
        .last()
        .expect("a step")
        .as_object()
        .expect("a record")
        .clone();
    assert_eq!(
        last["revision"],
        shell.state().as_object().expect("a state")["revision"]
    );
    for (state, name) in [
        (&canonical, "exports/shell.json"),
        (&dxf, "exports/shell.dxf"),
        (&pdf, "exports/shell.pdf"),
    ] {
        let file = state["file"].as_object().expect("a file");
        assert_eq!(file["path"].as_text(), Some(name));
        assert_eq!(
            hex(file["hex"].as_text().expect("hex")),
            file_of(&disk, name),
            "{name}"
        );
    }
    let drafted = sc_artifacts::from_canonical_text(
        &String::from_utf8(file_of(&disk, "exports/shell.json")).expect("text"),
    )
    .expect("reads");
    let front = drafted
        .pieces
        .iter()
        .find(|piece| piece.name == "skirt_front")
        .expect("the front");
    let lowest = front.cut.iter().map(|at| at.y()).min().expect("points");
    assert_eq!(
        lowest, -680_000,
        "the length change was kept and the hem allowance change undone"
    );
}

#[test]
fn an_edit_the_draft_cannot_draw_is_taken_back_through_the_bus_and_says_why() {
    let mut shell = open();
    let start = shell.state();
    let state = act(
        &mut shell,
        r#"{"do":"change","token":"sa_side","um":-10000}"#,
    );
    assert!(failed(&state));
    assert!(
        status(&state).contains("negative allowance"),
        "{}",
        status(&state)
    );
    assert_eq!(input(&state, "sa_side"), 10_000);
    assert_eq!(
        state["lines"],
        start.as_object().expect("a state")["lines"],
        "the drawing is the one before"
    );
    // A bad edit taken back is still a session the program can replay, and it can be redone only as what it was.
    replay(&shell.recording(), MemoryRepository::new())
        .unwrap_or_else(|failure| panic!("{failure:?}"));
    // An unknown name is the bus's refusal, unrecorded.
    let steps = state["steps"].clone();
    let state = act(&mut shell, r#"{"do":"change","token":"nonsense","um":1}"#);
    assert!(failed(&state));
    assert_eq!(state["steps"], steps);
}

#[test]
fn undo_and_redo_say_when_there_is_nothing_to_do() {
    let mut shell = open();
    let state = act(&mut shell, r#"{"do":"undo"}"#);
    assert!(failed(&state));
    assert_eq!(state["can_undo"], Value::Bool(false));
    let state = act(
        &mut shell,
        r#"{"do":"change","token":"zip_len","um":190000}"#,
    );
    assert_eq!(state["can_undo"], Value::Bool(true));
    let state = act(&mut shell, r#"{"do":"undo"}"#);
    assert_eq!(state["can_redo"], Value::Bool(true));
    assert_eq!(input(&state, "zip_len"), 180_000);
    let state = act(&mut shell, r#"{"do":"redo"}"#);
    assert_eq!(input(&state, "zip_len"), 190_000);
}

#[test]
fn saving_and_reopening_returns_the_values_and_the_drawing() {
    let mut shell = open();
    act(
        &mut shell,
        r#"{"do":"change","token":"hip_girth","um":1000000}"#,
    );
    let saved = act(&mut shell, r#"{"do":"save"}"#);
    assert!(!failed(&saved), "{}", status(&saved));
    act(
        &mut shell,
        r#"{"do":"change","token":"hip_girth","um":900000}"#,
    );
    let state = act(&mut shell, r#"{"do":"reopen"}"#);
    assert!(!failed(&state), "{}", status(&state));
    assert_eq!(input(&state, "hip_girth"), 1_000_000);
    assert_eq!(state["lines"], saved["lines"]);
}

#[test]
fn zooming_keeps_the_point_under_the_cursor_and_zooming_back_returns_exactly() {
    let mut shell = open();
    let start = shell.state().as_object().expect("a state").clone();
    let (x, y) = (500, 300);
    // A point of the drawing near the cursor, which must stay put relative to it.
    let near = cuts(&start)[0][0];
    let scale_before = {
        let (a, b) = (cuts(&start)[0][0], cuts(&start)[0][1]);
        (b.0 - a.0).hypot(b.1 - a.1)
    };
    act(
        &mut shell,
        &format!(r#"{{"do":"zoom","closer":true,"x":{x},"y":{y}}}"#),
    );
    let closer = shell.state().as_object().expect("a state").clone();
    let moved = cuts(&closer)[0][0];
    let scale_after = {
        let (a, b) = (cuts(&closer)[0][0], cuts(&closer)[0][1]);
        (b.0 - a.0).hypot(b.1 - a.1)
    };
    let factor = scale_after / scale_before;
    assert!(factor > 1.0);
    let expected = (
        x as f64 + (near.0 - x as f64) * factor,
        y as f64 + (near.1 - y as f64) * factor,
    );
    assert!(
        (moved.0 - expected.0).abs() < 0.01 && (moved.1 - expected.1).abs() < 0.01,
        "{moved:?} against {expected:?}"
    );
    let back = act(
        &mut shell,
        &format!(r#"{{"do":"zoom","closer":false,"x":{x},"y":{y}}}"#),
    );
    assert_eq!(back["lines"], start["lines"], "exactly, with no drift");
    // Down to the bottom of the ladder and up the same way, from anywhere, returns too.
    for _ in 0..200 {
        act(&mut shell, r#"{"do":"zoom","closer":true,"x":37,"y":911}"#);
    }
    act(&mut shell, r#"{"do":"pan","x":12345,"y":-54321}"#);
    act(&mut shell, r#"{"do":"pan","x":-12345,"y":54321}"#);
    for _ in 0..26 {
        act(&mut shell, r#"{"do":"zoom","closer":false,"x":37,"y":911}"#);
    }
    let bottom = act(&mut shell, r#"{"do":"fit"}"#);
    assert_eq!(
        bottom["lines"], start["lines"],
        "fit is the same view however it got here"
    );
}

#[test]
fn panning_moves_every_line_by_exactly_the_drag() {
    let mut shell = open();
    let start = shell.state().as_object().expect("a state").clone();
    let moved = act(&mut shell, r#"{"do":"pan","x":-120,"y":75}"#);
    for (a, b) in cuts(&start)
        .iter()
        .flatten()
        .zip(cuts(&moved).iter().flatten())
    {
        assert!(
            (b.0 - a.0 - 120.0).abs() < 0.01 && (b.1 - a.1 + 75.0).abs() < 0.01,
            "{a:?} to {b:?}"
        );
    }
}

#[test]
fn a_click_on_an_edge_picks_that_edge_at_every_zoom_and_a_click_on_nothing_picks_nothing() {
    let mut shell = open();
    let names = [
        "skirt_back_left",
        "skirt_back_right",
        "skirt_front",
        "waistband",
        "waistband_interfacing",
    ];
    for zooms in 0..4 {
        let state = shell.state().as_object().expect("a state").clone();
        for (at, piece) in cuts(&state).iter().enumerate() {
            // The longest edge of each piece, so the middle of it is well clear of its neighbours.
            let longest = (0..piece.len() - 1)
                .max_by(|a, b| {
                    let length =
                        |i: usize| (piece[i + 1].0 - piece[i].0).hypot(piece[i + 1].1 - piece[i].1);
                    length(*a).total_cmp(&length(*b))
                })
                .expect("edges");
            let (x, y) = (
                (piece[longest].0 + piece[longest + 1].0) / 2.0,
                (piece[longest].1 + piece[longest + 1].1) / 2.0,
            );
            let picked = act(
                &mut shell,
                &format!(
                    r#"{{"do":"pick","x":{},"y":{}}}"#,
                    x.round() as i64,
                    y.round() as i64
                ),
            );
            let selected = picked["selected"]
                .as_object()
                .unwrap_or_else(|| panic!("zoom {zooms}, {}: nothing at ({x}, {y})", names[at]));
            assert_eq!(selected["piece"].as_text(), Some(names[at]));
            assert_eq!(
                selected["edge"].as_int(),
                Some(longest as i64),
                "zoom {zooms}, {}",
                names[at]
            );
        }
        act(&mut shell, r#"{"do":"zoom","closer":true,"x":100,"y":100}"#);
    }
    let mut shell = open();
    let far = act(&mut shell, r#"{"do":"pick","x":1399,"y":899}"#);
    assert_eq!(far["selected"], Value::Null);
    assert_eq!(status(&far), "nothing there");
}

#[test]
fn a_picked_edge_measures_true_at_any_zoom() {
    // The front's side seam from hem to hip is one straight edge: the book's 42.107 cm, on the net line, and on the cut line a little longer.
    let mut shell = open();
    for _ in 0..3 {
        let state = shell.state().as_object().expect("a state").clone();
        let front = &cuts(&state)[2];
        let longest = (0..front.len() - 1)
            .max_by(|a, b| {
                let length =
                    |i: usize| (front[i + 1].0 - front[i].0).hypot(front[i + 1].1 - front[i].1);
                length(*a).total_cmp(&length(*b))
            })
            .expect("edges");
        let (x, y) = (
            (front[longest].0 + front[longest + 1].0) / 2.0,
            (front[longest].1 + front[longest + 1].1) / 2.0,
        );
        let picked = act(
            &mut shell,
            &format!(
                r#"{{"do":"pick","x":{},"y":{}}}"#,
                x.round() as i64,
                y.round() as i64
            ),
        );
        let selected = picked["selected"].as_object().expect("an edge");
        // The longest cut edge is the fold on CF: hem allowance to waist allowance.
        assert_eq!(selected["length_um"].as_int(), Some(660_000));
        act(&mut shell, r#"{"do":"zoom","closer":true,"x":100,"y":100}"#);
    }
}

#[test]
fn a_session_cannot_export_a_drawing_the_design_has_moved_on_from() {
    let mut session = parse(SKIRT).expect("valid");
    let Value::Object(fields) = &mut session else {
        panic!("an object")
    };
    let Some(Value::List(steps)) = fields.get_mut("steps") else {
        panic!("a list")
    };
    let at = steps
        .iter()
        .position(|step| {
            step.as_object().expect("a step")["do"].as_text() == Some("export_pattern")
        })
        .expect("an export");
    let change = parse(r#"{"do":"change","token":"hip_girth","um":1000000}"#).expect("valid");
    steps.insert(at, change);
    let failure = replay(&session, MemoryRepository::new())
        .expect_err("the drawing is stale")
        .to_value();
    let message = failure.as_object().expect("a diagnostic")["message"]
        .as_text()
        .expect("text")
        .to_owned();
    assert!(message.contains("needs a draft first"), "{message}");
}

#[test]
fn nonsense_from_a_page_is_a_status_and_nothing_else() {
    let mut shell = open();
    let before = shell.recording();
    for action in [
        "not json",
        "[]",
        r#"{"do":"nonsense"}"#,
        r#"{"do":"pick"}"#,
        r#"{"do":"zoom"}"#,
    ] {
        let state = act(&mut shell, action);
        assert!(state.contains_key("lines"), "{action}");
    }
    assert_eq!(shell.recording(), before);
}
