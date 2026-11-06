//! A real session through the real command bus: the reference skirt's recipe evaluates to the book's values,
//! edits change them, undo brings them back, a stale revision is refused, and a replay is byte-identical.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_cli::replay;
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::BTreeMap;
use std::process::Command;

const SKIRT: &str = include_str!("reference-skirt.session.json");

fn session(steps: &str) -> Value {
    let text = format!(
        r#"{{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV","inputs":[{{"token":"waist_girth","um":740000}}],
        "recipe":"let doubled: length = 2 * waist_girth\n","steps":{steps}}}"#
    );
    parse(&text).expect("a valid session")
}

fn run(value: &Value) -> Result<Value, Value> {
    replay(value, MemoryRepository::new()).map_err(|failure| failure.to_value())
}

/// The values one evaluate step printed, by name.
fn values(record: &Value) -> BTreeMap<String, String> {
    let fields = record.as_object().expect("a step record");
    assert_eq!(fields["do"].as_text(), Some("evaluate"));
    fields["result"]
        .as_list()
        .expect("bindings")
        .iter()
        .map(|binding| {
            let binding = binding.as_object().expect("a binding");
            let typed = format!(
                "{}:{}",
                binding["kind"].as_text().unwrap_or(""),
                binding["value"].as_text().unwrap_or("")
            );
            (binding["name"].as_text().unwrap_or("").to_owned(), typed)
        })
        .collect()
}

#[test]
fn the_reference_skirt_evaluates_to_the_books_values() {
    let transcript = run(&parse(SKIRT).expect("valid")).expect("the session runs");
    let steps = transcript.as_list().expect("a transcript");
    let first = values(&steps[0]);
    // In micrometres, from the reference skirt chapter §4 and the formula examples §2.
    for (name, wanted) in [
        ("garment_waist", "length:740000"),
        ("garment_hip", "length:1020000"),
        ("quarter_waist", "length:185000"),
        ("quarter_hip", "length:255000"),
        ("suppression", "length:70000"),
        ("dart_intake", "length:40000"),
        ("side_seam_total", "length:120000"),
        ("dart_total", "length:160000"),
        ("front_dart_centre", "length:112500"),
        ("back_dart_centre", "length:112500"),
        ("quarter_hem_width", "length:285000"),
        ("garment_hem", "length:1140000"),
        ("hip_to_hem_drop", "length:420000"),
        ("side_seam_length", "length:421070"),
        ("waistband_pattern_length", "length:800000"),
        ("waistband_cut_width", "length:100000"),
        ("waistband_fold_position", "length:50000"),
        ("waistband_finished_length", "length:770000"),
        ("wb_interfacing_width", "length:40000"),
        ("wb_interfacing_length", "length:770000"),
        ("dart_leg_angle", "angle:9462322"),
        ("dart_intake_share", "ratio:571429"),
        ("dart_count", "count:1"),
    ] {
        assert_eq!(first.get(name).map(String::as_str), Some(wanted), "{name}");
    }
}

#[test]
fn an_edit_changes_the_result_undo_restores_it_and_redo_returns_it() {
    let transcript = run(&parse(SKIRT).expect("valid")).expect("the session runs");
    let steps = transcript.as_list().expect("a transcript");
    let revision = |at: usize| {
        steps[at].as_object().expect("a record")["revision"]
            .as_text()
            .expect("text")
            .to_owned()
    };
    let before = values(&steps[0]);
    let edited = values(&steps[3]);
    let undone = values(&steps[5]);
    assert_eq!(before["garment_waist"], "length:740000");
    assert_eq!(edited["garment_waist"], "length:780000");
    assert_eq!(undone, before, "undo restores every value");
    assert_ne!(revision(2), revision(1), "the edit moved the revision");
    assert_eq!(
        steps[1].as_object().expect("a record")["result"]
            .as_object()
            .expect("a verdict")["outcome"]
            .as_text(),
        Some("satisfied")
    );

    // The save after redo holds the edit, and it survives reopening.
    let reopened = values(&steps[10]);
    assert_eq!(reopened, edited);
    let saves: Vec<&str> = [7, 8]
        .iter()
        .map(|at| {
            steps[*at].as_object().expect("a record")["result"]
                .as_text()
                .expect("text")
        })
        .collect();
    assert!(saves[0].starts_with("applied"), "{saves:?}");
    assert_eq!(
        saves[1], "unchanged",
        "saving the same state again writes nothing"
    );
}

#[test]
fn a_replay_is_byte_identical_and_so_is_the_project_it_saves() {
    let value = parse(SKIRT).expect("valid");
    let (mut first_disk, mut second_disk) = (MemoryRepository::new(), MemoryRepository::new());
    let first = replay(&value, &mut first_disk).expect("runs").to_text();
    let second = replay(&value, &mut second_disk).expect("runs").to_text();
    assert_eq!(first, second);
    assert_eq!(first_disk, second_disk);
    let manifest = first_disk
        .read(&FilePath::new("project.json").expect("a path"))
        .expect("reads")
        .expect("saved");
    assert!(String::from_utf8_lossy(&manifest).contains("\"schema\": 1"));
}

#[test]
fn a_stale_revision_is_refused_by_the_bus_and_changes_nothing() {
    let steps = r#"[{"do":"change","token":"waist_girth","um":750000},
        {"do":"change","token":"waist_girth","um":760000,"expect_revision":"0"}]"#;
    let failure = run(&session(steps)).expect_err("a stale revision");
    let fields = failure.as_object().expect("a diagnostic");
    assert_eq!(fields["failed"].as_text(), Some("refused"));
    assert_eq!(fields["step"].as_int(), Some(1));
    let message = fields["message"].as_text().expect("text");
    assert!(message.to_lowercase().contains("revision"), "{message}");
}

#[test]
fn a_reopened_design_never_reissues_a_revision_it_used_before() {
    // Two edits reach revision 2, undo returns to 1, and that state is saved and reopened.
    let steps = r#"[{"do":"change","token":"waist_girth","um":750000},
        {"do":"change","token":"waist_girth","um":760000},
        {"do":"undo"},
        {"do":"save"},
        {"do":"reopen"},
        {"do":"evaluate"},
        {"do":"change","token":"waist_girth","um":770000},
        {"do":"evaluate"},
        {"do":"undo"},
        {"do":"evaluate"}]"#;
    let transcript = run(&session(steps)).expect("runs");
    let records = transcript.as_list().expect("a transcript");
    let revision = |at: usize| {
        records[at].as_object().expect("a record")["revision"]
            .as_text()
            .expect("text")
            .to_owned()
    };
    assert_eq!(revision(1), "2");
    assert_eq!(revision(2), "1", "undo moved back");
    assert_eq!(
        revision(4),
        "1",
        "the reopened Design is at its saved revision"
    );
    assert_eq!(
        values(&records[5])["doubled"],
        "length:1500000",
        "and holds the saved value"
    );
    assert_eq!(
        revision(6),
        "3",
        "revision 2 was issued before the reopen, so it is not reissued"
    );
    assert_eq!(values(&records[7])["doubled"], "length:1540000");
    // History made after the reopen still works; history from before it does not cross.
    assert_eq!(revision(8), "1");
    assert_eq!(values(&records[9])["doubled"], "length:1500000");

    let steps = r#"[{"do":"change","token":"waist_girth","um":750000},{"do":"save"},{"do":"reopen"},{"do":"undo"}]"#;
    let failure = run(&session(steps)).expect_err("undo does not cross a reopen");
    assert_eq!(
        failure.as_object().expect("a diagnostic")["failed"].as_text(),
        Some("nothing_to_change")
    );
}

#[test]
fn the_constraint_step_reports_a_conflict_in_sentences() {
    let transcript = run(&session(
        r#"[{"do":"solve","token":"waist_girth","at_most_um":700000}]"#,
    ))
    .expect("runs");
    let steps = transcript.as_list().expect("a transcript");
    let verdict = steps[0].as_object().expect("a record")["result"]
        .as_object()
        .expect("a verdict");
    assert_eq!(verdict["outcome"].as_text(), Some("proven_unsatisfiable"));
    let sentences = verdict["explanation"].as_list().expect("sentences");
    assert!(
        sentences.iter().any(|line| line
            .as_text()
            .is_some_and(|text| text.contains("must be at most 700000"))),
        "{sentences:?}"
    );
}

#[test]
fn failures_are_structured_and_name_the_step() {
    for (steps, code, step) in [
        (r#"[{"do":"undo"}]"#, "nothing_to_change", 0),
        (r#"[{"do":"redo"}]"#, "nothing_to_change", 0),
        (r#"[{"do":"evaluate"},{"do":"dance"}]"#, "session", 1),
        (
            r#"[{"do":"change","token":"nope","um":1}]"#,
            "unknown_input",
            0,
        ),
        (r#"[{"do":"change","token":"waist_girth"}]"#, "session", 0),
        (
            r#"[{"do":"solve","token":"nope","at_most_um":1}]"#,
            "unknown_input",
            0,
        ),
        (r#"[{"do":"reopen"}]"#, "reopen", 0),
        (r#"[{}]"#, "session", 0),
    ] {
        let failure = run(&session(steps)).expect_err(steps);
        let fields = failure.as_object().expect("a diagnostic");
        assert_eq!(
            fields["failed"].as_text(),
            Some(code),
            "{steps}: {failure:?}"
        );
        assert_eq!(fields["step"].as_int(), Some(step), "{steps}");
        assert!(
            !fields["message"].as_text().expect("text").is_empty(),
            "{steps}"
        );
    }
}

#[test]
fn a_recipe_that_does_not_check_is_a_structured_failure_at_its_step() {
    let text = r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV","inputs":[{"token":"a","um":1}],
        "recipe":"let b: length = a + 1\n","steps":[{"do":"evaluate"}]}"#;
    let failure = run(&parse(text).expect("valid")).expect_err("length plus ratio");
    let fields = failure.as_object().expect("a diagnostic");
    assert_eq!(fields["failed"].as_text(), Some("recipe"));
    assert_eq!(fields["step"].as_int(), Some(0));
}

#[test]
fn a_session_with_bad_fields_is_refused_before_any_step() {
    for (text, words) in [
        ("[]", "must be an object"),
        (r#"{"format":2}"#, "session format 1"),
        (r#"{"format":1}"#, "no design field"),
        (r#"{"format":1,"design":"not an id"}"#, "design:"),
        (
            r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV"}"#,
            "no inputs field",
        ),
        (
            r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV","inputs":[{"token":"a","um":"x"}]}"#,
            "whole number",
        ),
        (
            r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV","inputs":[]}"#,
            "no recipe field",
        ),
        (
            r#"{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV","inputs":[],"recipe":""}"#,
            "no steps field",
        ),
    ] {
        let failure = run(&parse(text).expect("valid text")).expect_err(text);
        let message = failure.as_object().expect("a diagnostic")["message"]
            .as_text()
            .expect("text")
            .to_owned();
        assert!(message.contains(words), "{text}: {message}");
    }
}

fn program(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_stitchcad"))
        .args(arguments)
        // Locale and time must not reach the output.
        .env("LC_ALL", "fr_FR.UTF-8")
        .env("TZ", "Asia/Tokyo")
        .output()
        .expect("the program runs")
}

#[test]
fn the_program_replays_a_recorded_session_byte_identically_and_saves_a_project() {
    let directory = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("sc-cli");
    let (first_dir, second_dir) = (directory.join("first"), directory.join("second"));
    for path in [&first_dir, &second_dir] {
        if path.exists() {
            std::fs::remove_dir_all(path).expect("clears the last run");
        }
    }
    let session = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/reference-skirt.session.json"
    );
    let first = program(&["replay", session, first_dir.to_str().expect("a path")]);
    let second = program(&["replay", session, second_dir.to_str().expect("a path")]);
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stdout)
    );
    assert_eq!(first.stdout, second.stdout);
    parse(&String::from_utf8_lossy(&first.stdout)).expect("the transcript is canonical text");
    for name in ["project.json", "inputs.json", "recipe.formula"] {
        let (a, b) = (
            std::fs::read(first_dir.join(name)).expect(name),
            std::fs::read(second_dir.join(name)).expect(name),
        );
        assert_eq!(a, b, "{name}");
    }
}

#[test]
fn the_program_exits_nonzero_with_a_structured_diagnostic() {
    let failure = program(&["dance"]);
    assert!(!failure.status.success());
    let value = parse(&String::from_utf8_lossy(&failure.stdout)).expect("canonical text");
    assert_eq!(
        value.as_object().expect("a diagnostic")["failed"].as_text(),
        Some("usage")
    );

    let missing = program(&["replay", "no-such-file.json", "no-such-dir"]);
    assert!(!missing.status.success());
    let value = parse(&String::from_utf8_lossy(&missing.stdout)).expect("canonical text");
    assert_eq!(
        value.as_object().expect("a diagnostic")["failed"].as_text(),
        Some("usage")
    );
}
