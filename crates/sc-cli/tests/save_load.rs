//! Native save and load preserve full semantics: the roadmap's §13 obligation, on the reference skirt, through the real program.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_artifacts::{compare, from_canonical_text, Comparison};
use sc_cli::replay;
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");
const GOLDEN: &str =
    include_str!("../../../conformance/goldens/reference-skirt/reference-skirt.canonical.json");
const DXF: &str = include_str!("../../../conformance/goldens/reference-skirt/reference-skirt.dxf");

// The skirt's own opening, with other steps. Its `draft` steps name nothing unless the test says so.
fn with_steps(steps: &str) -> Value {
    let mut session = parse(SKIRT).expect("valid");
    let Value::Object(fields) = &mut session else {
        panic!("an object")
    };
    fields.insert("steps".to_owned(), parse(steps).expect("valid steps"));
    session
}

fn first_save() -> MemoryRepository {
    let mut disk = MemoryRepository::new();
    replay(&parse(SKIRT).expect("valid"), &mut disk)
        .unwrap_or_else(|failure| panic!("{failure:?}"));
    disk
}

fn run(session: &Value, disk: &mut MemoryRepository) -> Vec<Value> {
    let transcript = replay(session, &mut *disk).unwrap_or_else(|failure| panic!("{failure:?}"));
    transcript.as_list().expect("a transcript").to_vec()
}

fn file(disk: &MemoryRepository, name: &str) -> String {
    let bytes = disk
        .read(&FilePath::new(name).expect("a path"))
        .expect("reads")
        .unwrap_or_else(|| panic!("no {name}"));
    String::from_utf8(bytes).expect("text")
}

fn saved_file(disk: &MemoryRepository, name: &str) -> String {
    // The store keeps a project's files under its own prefix, so a test asks by the path's end.
    let found: Vec<FilePath> = disk
        .list()
        .expect("lists")
        .into_iter()
        .filter(|path| path.as_str().ends_with(name))
        .collect();
    let [only] = found.as_slice() else {
        panic!("{name}: {found:?}")
    };
    file(disk, only.as_str())
}

fn result_of(record: &Value) -> Value {
    record.as_object().expect("a record")["result"].clone()
}

#[test]
fn a_project_holds_how_to_draw_the_pattern_and_never_the_drawing() {
    let disk = first_save();
    let description = parse(&saved_file(&disk, "draft.json")).expect("canonical text");
    let fields = description.as_object().expect("an object");
    assert_eq!(fields["style"].as_text(), Some("reference_skirt"));
    assert_eq!(fields["pieces"].as_list().expect("pieces").len(), 5);
    assert!(
        !fields.contains_key("do"),
        "a step's verb is not part of the design"
    );
    // Derived data is never authoritative, so nothing under the project's own files is a drawing. Exports stay what they are: files.
    let derived: Vec<String> = disk
        .list()
        .expect("lists")
        .iter()
        .map(|path| path.as_str().to_owned())
        .filter(|path| path.contains("pattern") || path.contains("drawn"))
        .collect();
    assert!(derived.is_empty(), "{derived:?}");
}

#[test]
fn saving_what_was_loaded_writes_nothing_and_leaves_every_byte() {
    let mut disk = first_save();
    let before = disk.image().clone();
    let records = run(&with_steps(r#"[{"do":"reopen"},{"do":"save"}]"#), &mut disk);
    assert_eq!(disk.image(), &before, "save, load, save is byte-identical");
    assert_eq!(
        result_of(&records[1]).as_text(),
        Some("unchanged"),
        "and it knows it wrote nothing: {:?}",
        records[1]
    );
    // Twice over, and after drawing again: a draft changes no authored content.
    run(
        &with_steps(
            r#"[{"do":"reopen"},{"do":"draft"},{"do":"save"},{"do":"reopen"},{"do":"save"}]"#,
        ),
        &mut disk,
    );
    assert_eq!(disk.image(), &before);
}

#[test]
fn a_reopened_project_draws_the_same_bytes_with_no_pieces_named() {
    let mut disk = first_save();
    run(
        &with_steps(
            r#"[{"do":"reopen"},{"do":"draft"},
            {"do":"export_pattern","format":"canonical","path":"again/skirt.json"},
            {"do":"export_pattern","format":"dxf","semantics":"cut_as_1","path":"again/skirt.dxf"},
            {"do":"save"}]"#,
        ),
        &mut disk,
    );
    assert_eq!(saved_file(&disk, "again/skirt.json"), GOLDEN);
    assert_eq!(saved_file(&disk, "again/skirt.dxf"), DXF);
}

#[test]
fn reopening_ignores_what_the_session_file_says_and_keeps_what_the_project_saved() {
    // A session opening a project states an opening of its own, which must not leak past a reopen.
    let mut disk = first_save();
    let mut session = with_steps(
        r#"[{"do":"reopen"},{"do":"draft"},{"do":"export_pattern","format":"canonical","path":"again/skirt.json"},{"do":"save"}]"#,
    );
    let Value::Object(fields) = &mut session else {
        panic!("an object")
    };
    if let Some(Value::List(inputs)) = fields.get_mut("inputs") {
        for input in inputs {
            if let Value::Object(entry) = input {
                if entry.get("token").and_then(Value::as_text) == Some("hip_girth") {
                    entry.insert("um".to_owned(), Value::Int(1_500_000));
                }
            }
        }
    }
    run(&session, &mut disk);
    assert_eq!(saved_file(&disk, "again/skirt.json"), GOLDEN);
}

#[test]
fn an_edit_after_reopening_gives_exactly_what_the_same_edit_gives_before_saving() {
    let edit = r#"{"do":"change","token":"hip_girth","um":1000000},{"do":"draft"},{"do":"export_pattern","format":"canonical","path":"out.json"},{"do":"save"}"#;
    let mut straight = first_save();
    // Straight through, on one bus, with no reopen: the draft restates nothing because the session drew in the first run's own bus only.
    let mut through = MemoryRepository::new();
    let opening = parse(SKIRT).expect("valid");
    let steps = opening.as_object().expect("a session")["steps"]
        .as_list()
        .expect("steps")
        .to_vec();
    let first_draft = steps
        .iter()
        .find(|step| step.as_object().expect("a step")["do"].as_text() == Some("draft"))
        .expect("a draft")
        .to_text();
    run(
        &with_steps(&format!(r#"[{first_draft},{edit}]"#)),
        &mut through,
    );
    // Across a save and a reopen.
    run(
        &with_steps(&format!(r#"[{{"do":"reopen"}},{edit}]"#)),
        &mut straight,
    );
    let (one, two) = (
        saved_file(&through, "out.json"),
        saved_file(&straight, "out.json"),
    );
    let one = from_canonical_text(&one).expect("reads");
    let two = from_canonical_text(&two).expect("reads");
    assert_eq!(
        one.pieces, two.pieces,
        "the same geometry and data, every piece"
    );
}

// One design whose width follows an input, and another that states the same number. Same contours, different intent.
fn intent(recipe: &str) -> Value {
    parse(&format!(
        r#"{{"format":1,"design":"01ARZ3NDEKTSV4RRFFQ69G5FAV",
        "inputs":[{{"token":"width","um":200000}},{{"token":"height","um":100000}}],
        "recipe":"{recipe}\n",
        "steps":[{{"do":"draft","style":"panel","size":"M","pieces":[{{
          "name":"panel","quantity":1,"start":["zero","zero"],
          "edges":[{{"to":["panel_width","zero"],"width":"zero","corner":"miter"}},
                   {{"to":["panel_width","height"],"width":"zero","corner":"miter"}},
                   {{"to":["zero","height"],"width":"zero","corner":"miter"}},
                   {{"to":["zero","zero"],"width":"zero","corner":"miter"}}],
          "grain":[[["zero","zero"],["zero","height"]]]
        }}]}},
        {{"do":"export_pattern","format":"canonical","path":"out.json"}},
        {{"do":"save"}}]}}"#
    ))
    .expect("valid")
}

#[test]
fn two_designs_with_identical_contours_and_different_intent_stay_distinct() {
    let (mut follows, mut states) = (MemoryRepository::new(), MemoryRepository::new());
    run(&intent("let panel_width: length = width"), &mut follows);
    run(&intent("let panel_width: length = 200 mm"), &mut states);
    // What a file can show is the same.
    let (a, b) = (
        saved_file(&follows, "out.json"),
        saved_file(&states, "out.json"),
    );
    assert_eq!(
        compare(&a, &b, 0).expect("both read"),
        Comparison::Identical,
        "identical contours"
    );
    // What the project knows is not.
    assert_ne!(
        saved_file(&follows, "recipe.formula"),
        saved_file(&states, "recipe.formula")
    );
    // And the difference is real, after a save and a load: change the input, and only the one that follows it moves.
    let next = r#"[{"do":"reopen"},{"do":"change","token":"width","um":300000},{"do":"draft"},
        {"do":"export_pattern","format":"canonical","path":"out.json"},{"do":"save"}]"#;
    for disk in [&mut follows, &mut states] {
        let mut session = intent("");
        let Value::Object(fields) = &mut session else {
            panic!("an object")
        };
        fields.insert("steps".to_owned(), parse(next).expect("valid"));
        fields.insert(
            "recipe".to_owned(),
            Value::Text("let unused: length = 1 mm\n".to_owned()),
        );
        run(&session, disk);
    }
    let widest = |disk: &MemoryRepository| {
        let pattern = from_canonical_text(&saved_file(disk, "out.json")).expect("reads");
        pattern.pieces[0]
            .cut
            .iter()
            .map(|at| at.x())
            .max()
            .expect("points")
    };
    assert_eq!(widest(&follows), 300_000);
    assert_eq!(widest(&states), 200_000);
    assert!(matches!(
        compare(
            &saved_file(&follows, "out.json"),
            &saved_file(&states, "out.json"),
            0
        )
        .expect("both read"),
        Comparison::Different(_)
    ));
}

#[test]
fn undo_does_not_cross_a_reopen_and_says_so() {
    let mut disk = first_save();
    run(
        &with_steps(r#"[{"do":"change","token":"hip_girth","um":1000000},{"do":"save"}]"#),
        &mut disk,
    );
    let failure = replay(&with_steps(r#"[{"do":"reopen"},{"do":"undo"}]"#), &mut disk)
        .expect_err("no history")
        .to_value();
    assert_eq!(
        failure.as_object().expect("a diagnostic")["failed"].as_text(),
        Some("nothing_to_change")
    );
}

#[test]
fn revisions_carry_on_across_a_reopen_and_are_never_issued_twice() {
    let mut disk = first_save();
    let records = run(
        &with_steps(
            r#"[{"do":"change","token":"hip_girth","um":1000000},{"do":"save"},{"do":"reopen"},{"do":"change","token":"hip_girth","um":1010000}]"#,
        ),
        &mut disk,
    );
    let revisions: Vec<&str> = records
        .iter()
        .map(|record| {
            record.as_object().expect("a record")["revision"]
                .as_text()
                .expect("text")
        })
        .collect();
    assert_eq!(revisions, ["1", "1", "1", "2"]);
}

#[test]
fn a_project_saved_before_anything_was_drafted_reopens_and_a_draft_says_what_is_missing() {
    let mut disk = MemoryRepository::new();
    run(&with_steps(r#"[{"do":"save"},{"do":"reopen"}]"#), &mut disk);
    let failure = replay(
        &with_steps(r#"[{"do":"reopen"},{"do":"draft"}]"#),
        &mut disk,
    )
    .expect_err("nothing to draw")
    .to_value();
    let fields = failure.as_object().expect("a diagnostic");
    assert_eq!(fields["failed"].as_text(), Some("draft"));
    assert!(
        fields["message"]
            .as_text()
            .expect("text")
            .contains("saved none"),
        "{failure:?}"
    );
}

#[test]
fn a_description_that_cannot_be_read_is_refused_by_name_rather_than_ignored() {
    // A file edited on disk outside a commit is not detected yet, as the store's chapter says. One that cannot be read must not pass for no description.
    let mut disk = first_save();
    let found: Vec<FilePath> = disk
        .list()
        .expect("lists")
        .into_iter()
        .filter(|path| path.as_str().ends_with("draft.json"))
        .collect();
    let [path] = found.as_slice() else {
        panic!("{found:?}")
    };
    disk.write(path, b"not a description").expect("writes");
    let failure = replay(&with_steps(r#"[{"do":"reopen"}]"#), &mut disk)
        .expect_err("unreadable")
        .to_value();
    assert_eq!(
        failure.as_object().expect("a diagnostic")["failed"].as_text(),
        Some("reopen"),
        "{failure:?}"
    );
    disk.write(path, b"[]").expect("writes");
    let failure = replay(&with_steps(r#"[{"do":"reopen"}]"#), &mut disk)
        .expect_err("not an object")
        .to_value();
    assert!(
        failure.as_object().expect("a diagnostic")["message"]
            .as_text()
            .expect("text")
            .contains("must hold an object"),
        "{failure:?}"
    );
}

#[test]
fn a_literal_written_in_another_unit_changes_nothing() {
    // Unit conversion, metamorphic: the same length written differently is the same design, down to the byte.
    let rewritten = SKIRT
        .replace(
            "let notch_depth: length = 5 mm",
            "let notch_depth: length = 0.5 cm",
        )
        .replace(
            "let label_y: length = waist_to_hip + 40 mm",
            "let label_y: length = waist_to_hip + 4 cm",
        )
        .replace(
            "let label_lower_y: length = label_y + 16 mm",
            "let label_lower_y: length = label_y + 16000 um",
        );
    assert_ne!(rewritten, SKIRT);
    let mut disk = MemoryRepository::new();
    replay(&parse(&rewritten).expect("valid"), &mut disk)
        .unwrap_or_else(|failure| panic!("{failure:?}"));
    assert_eq!(
        file(&disk, "exports/reference-skirt.canonical.json"),
        GOLDEN
    );
    // An inch is exactly 25.4 mm, so a metric literal and an inch literal of one length are one design.
    let inches = SKIRT.replace(
        "let notch_depth: length = 5 mm",
        "let notch_depth: length = 25.4 mm",
    );
    let exact = SKIRT.replace(
        "let notch_depth: length = 5 mm",
        "let notch_depth: length = 1 in",
    );
    let (mut a, mut b) = (MemoryRepository::new(), MemoryRepository::new());
    replay(&parse(&inches).expect("valid"), &mut a).unwrap_or_else(|failure| panic!("{failure:?}"));
    replay(&parse(&exact).expect("valid"), &mut b).unwrap_or_else(|failure| panic!("{failure:?}"));
    assert_eq!(
        file(&a, "exports/reference-skirt.canonical.json"),
        file(&b, "exports/reference-skirt.canonical.json")
    );
}

fn tree(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut found = BTreeMap::new();
    let mut waiting = vec![root.to_path_buf()];
    while let Some(directory) = waiting.pop() {
        for entry in std::fs::read_dir(&directory).expect("reads") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                waiting.push(path);
            } else {
                found.insert(
                    path.strip_prefix(root).expect("inside").to_path_buf(),
                    std::fs::read(&path).expect("reads"),
                );
            }
        }
    }
    found
}

#[test]
fn the_real_program_saves_loads_and_saves_a_real_directory_to_the_same_bytes_in_another_locale() {
    let base = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("sc-cli-save-load");
    if base.exists() {
        std::fs::remove_dir_all(&base).expect("clears the last run");
    }
    std::fs::create_dir_all(&base).expect("creates");
    let project = base.join("project");
    let session = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../conformance/reference-skirt/session.json"
    );
    let program = |session: &Path, zone: &str, locale: &str| {
        let done = Command::new(env!("CARGO_BIN_EXE_stitchcad"))
            .args([
                "replay",
                session.to_str().expect("a path"),
                project.to_str().expect("a path"),
            ])
            .env("LC_ALL", locale)
            .env("TZ", zone)
            .output()
            .expect("the program runs");
        assert!(
            done.status.success(),
            "{}",
            String::from_utf8_lossy(&done.stdout)
        );
    };
    program(Path::new(session), "Asia/Tokyo", "C");
    let before = tree(&project);
    assert!(
        before.keys().any(|path| path.ends_with("draft.json")),
        "{:?}",
        before.keys()
    );
    let reopen = base.join("reopen.json");
    std::fs::write(
        &reopen,
        with_steps(r#"[{"do":"reopen"},{"do":"draft"},{"do":"save"}]"#).to_text(),
    )
    .expect("writes");
    program(&reopen, "America/Lima", "fr_FR.UTF-8");
    assert_eq!(tree(&project), before, "not one byte moved");
}
