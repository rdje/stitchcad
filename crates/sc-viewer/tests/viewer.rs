//! The browser entry point, run natively: it gives the same bytes as the native program's library, and a project
//! image carries a project from one visit to the next.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_store::{parse, FilePath, MemoryRepository, Repository};
use sc_viewer::{exports, image_from_text, image_to_text, run};

const FIRST: &str = include_str!("first-visit.session.json");
const SECOND: &str = include_str!("second-visit.session.json");

#[test]
fn two_visits_carry_a_project_through_its_image_and_match_the_native_program() {
    let first = run(FIRST, "");
    let fields = first.as_object().expect("a result");
    let image = fields["image"].as_text().expect("an image");

    let second = run(SECOND, image);
    let second_fields = second.as_object().expect("a result");
    let transcript = second_fields["transcript"].as_list().expect("a transcript");
    let evaluated = transcript[1].as_object().expect("a record")["result"]
        .as_list()
        .expect("bindings");
    let waist = evaluated[0].as_object().expect("a binding");
    assert_eq!(waist["name"].as_text(), Some("garment_waist"));
    assert_eq!(
        waist["value"].as_text(),
        Some("780000"),
        "the first visit's edit was saved and reopened"
    );
    assert_eq!(
        transcript[1].as_object().expect("a record")["revision"].as_text(),
        Some("1")
    );

    // The same two sessions through the native library, on one repository.
    let mut native = MemoryRepository::new();
    let first_native = sc_cli::replay(&parse(FIRST).expect("valid"), &mut native).expect("runs");
    assert_eq!(first_native, fields["transcript"]);
    assert_eq!(image_to_text(&native), image);
    let second_native = sc_cli::replay(&parse(SECOND).expect("valid"), &mut native).expect("runs");
    assert_eq!(second_native, second_fields["transcript"]);
    assert_eq!(
        image_to_text(&native),
        second_fields["image"].as_text().expect("an image")
    );
    assert!(native
        .read(&FilePath::new("exports/front-quadrant.svg").expect("a path"))
        .expect("reads")
        .is_some());
}

#[test]
fn an_image_round_trips_any_bytes() {
    let mut repository = MemoryRepository::new();
    repository
        .write(
            &FilePath::new("a/b.bin").expect("a path"),
            &[0, 1, 127, 128, 255],
        )
        .expect("writes");
    repository
        .write(&FilePath::new("empty").expect("a path"), &[])
        .expect("writes");
    let back = image_from_text(&image_to_text(&repository)).expect("reads");
    assert_eq!(back, repository);
    assert_eq!(
        image_from_text("  ").expect("an empty project"),
        MemoryRepository::new()
    );
}

#[test]
fn a_bad_image_or_session_is_a_structured_failure() {
    for (session, image, words) in [
        ("not text", "", "the session"),
        (FIRST, "not text", "the project image"),
        (FIRST, "[]", "must be an object"),
        (FIRST, "{\"../x\": \"00\"}", "may not contain"),
        (FIRST, "{\"a\": 1}", "hexadecimal text"),
        (FIRST, "{\"a\": \"0\"}", "not hexadecimal"),
        (FIRST, "{\"a\": \"zz\"}", "not hexadecimal"),
    ] {
        let failure = run(session, image);
        let fields = failure.as_object().expect("a diagnostic");
        assert_eq!(fields["failed"].as_text(), Some("usage"), "{image}");
        let message = fields["message"].as_text().expect("text");
        assert!(message.contains(words), "{image}: {message}");
    }
    let failure = run("{\"format\":2}", "");
    assert_eq!(
        failure.as_object().expect("a diagnostic")["failed"].as_text(),
        Some("session")
    );
}

#[test]
fn the_exports_carry_the_same_bytes_as_the_function() {
    let feed = |slot: u32, text: &str| {
        exports::sc_input_clear(slot);
        for byte in text.bytes() {
            exports::sc_input_push(slot, u32::from(byte));
        }
    };
    feed(0, FIRST);
    feed(1, "");
    exports::sc_run();
    let length = exports::sc_output_len();
    let bytes: Vec<u8> = (0..length)
        .map(|at| u8::try_from(exports::sc_output_byte(at)).expect("a byte"))
        .collect();
    assert_eq!(
        String::from_utf8(bytes).expect("text"),
        run(FIRST, "").to_text()
    );
    assert_eq!(
        exports::sc_output_byte(length + 5),
        0,
        "past the end reads as zero"
    );
}
