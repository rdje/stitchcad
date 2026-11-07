//! The smallest and largest sizes, rebuilt from a written file, each check failing with its own words rather than a clamped value.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_cli::{replay, Caller, CommandAuthority, Running};
use sc_store::{parse, MemoryRepository, Value};
use std::collections::BTreeMap;

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");
const BODICE: &str = include_str!("../../../conformance/reference-bodice/session.json");
const TROUSERS: &str = include_str!("../../../conformance/reference-trousers/session.json");
const BOOK: &str = include_str!("../../../docs/book/src/spec/grading.md");

fn fields_of(value: &Value) -> &BTreeMap<String, Value> {
    value.as_object().expect("an object")
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

fn result_of(session: &str) -> BTreeMap<String, Value> {
    let plan = with_steps(session, r#"[{"do":"check_extremes"}]"#);
    let transcript =
        replay(&plan, MemoryRepository::new()).unwrap_or_else(|failure| panic!("{failure:?}"));
    let steps = transcript.as_list().expect("a transcript");
    fields_of(&fields_of(&steps[steps.len() - 1])["result"]).clone()
}

fn checks(result: &BTreeMap<String, Value>) -> Vec<BTreeMap<String, Value>> {
    result["checks"]
        .as_list()
        .expect("checks")
        .iter()
        .map(|item| fields_of(item).clone())
        .collect()
}

fn verdict<'a>(
    found: &'a [BTreeMap<String, Value>],
    check: &str,
    size: &str,
) -> &'a BTreeMap<String, Value> {
    found
        .iter()
        .find(|item| item["check"].as_text() == Some(check) && item["size"].as_text() == Some(size))
        .unwrap_or_else(|| panic!("no {check} at {size}"))
}

const THE_CHECKS: [(&str, &str); 7] = [
    ("declared_domain", "numerical"),
    ("offset_validity", "geometric"),
    ("minimum_curvature_radius", "geometric"),
    ("bounding_box_against_fabric", "physical"),
    ("seam_differential_against_ease", "geometric"),
    ("notch_spacing", "physical"),
    ("dart_intake_against_maximum", "physical"),
];

#[test]
fn every_check_runs_at_the_smallest_and_the_largest_size_of_every_garment_through_a_file() {
    for session in [SKIRT, BODICE, TROUSERS] {
        let result = result_of(session);
        assert!(
            result["through"]
                .as_text()
                .expect("text")
                .contains("own engine"),
            "it says what read the file"
        );
        let found = checks(&result);
        for size in ["XS", "XL"] {
            for (check, class) in THE_CHECKS {
                let item = verdict(&found, check, size);
                assert_eq!(item["class"].as_text(), Some(class), "{check}");
                assert!(
                    !item["detail"].as_text().expect("text").is_empty(),
                    "{check} says why"
                );
                assert_ne!(
                    item["verdict"].as_text(),
                    Some("no_limit"),
                    "{check}: the sessions declare their limits"
                );
            }
            assert!(
                found
                    .iter()
                    .any(|item| item["check"].as_text() == Some("finished_dimension")
                        && item["size"].as_text() == Some(size)),
                "{size}"
            );
        }
        // Nothing here is the sample: it is the two ends that are the question.
        assert!(!found.iter().any(|item| item["size"].as_text() == Some("M")));
    }
}

#[test]
fn what_holds_at_the_extremes_and_what_does_not_is_what_the_book_says() {
    let bodice = checks(&result_of(BODICE));
    for check in [
        "declared_domain",
        "offset_validity",
        "minimum_curvature_radius",
        "bounding_box_against_fabric",
        "notch_spacing",
        "dart_intake_against_maximum",
    ] {
        for size in ["XS", "XL"] {
            assert_eq!(
                verdict(&bodice, check, size)["verdict"].as_text(),
                Some("holds"),
                "{check} at {size}: {:?}",
                verdict(&bodice, check, size)
            );
        }
    }
    for size in ["XS", "XL"] {
        let seams = verdict(&bodice, "seam_differential_against_ease", size);
        assert_eq!(seams["verdict"].as_text(), Some("fails"), "{size}");
        assert!(
            seams["detail"]
                .as_text()
                .expect("text")
                .contains("cap_front_right"),
            "{seams:?}"
        );
    }
    let dimensions = |found: &[BTreeMap<String, Value>]| {
        for item in found
            .iter()
            .filter(|item| item["check"].as_text() == Some("finished_dimension"))
        {
            assert_eq!(item["verdict"].as_text(), Some("holds"), "{item:?}");
        }
    };
    dimensions(&bodice);
    dimensions(&checks(&result_of(SKIRT)));
    dimensions(&checks(&result_of(TROUSERS)));
    // The book's table of which check is which is the list a test holds.
    for (check, _) in THE_CHECKS {
        assert!(
            BOOK.contains(&format!("| `{check}` |")),
            "the book has no row for {check}"
        );
    }
}

#[test]
fn a_limit_it_is_past_is_a_failure_in_its_own_words_and_nothing_is_clamped() {
    let tight = rewritten(SKIRT, &|draft| {
        let mut grading = fields_of(&draft["grading"]).clone();
        grading.insert(
            "limits".to_owned(),
            parse(r#"{"fabric_width":"zip_len","notch_spacing":"waist_to_hem","dart_intake":"sa_side","min_radius":"waist_to_hem"}"#).expect("valid"),
        );
        draft.insert("grading".to_owned(), Value::Object(grading));
    });
    let found = checks(&result_of(&tight));
    for (check, says) in [
        ("bounding_box_against_fabric", "past the limit"),
        ("notch_spacing", "under the limit"),
        ("dart_intake_against_maximum", "past the limit"),
        ("minimum_curvature_radius", "under the limit"),
    ] {
        let item = verdict(&found, check, "XL");
        assert_eq!(
            item["verdict"].as_text(),
            Some("fails"),
            "{check}: {item:?}"
        );
        assert!(
            item["detail"].as_text().expect("text").contains(says),
            "{check}: {item:?}"
        );
    }
}

#[test]
fn with_no_limit_declared_it_says_so_rather_than_making_one_up() {
    let none = rewritten(SKIRT, &|draft| {
        let mut grading = fields_of(&draft["grading"]).clone();
        grading.remove("limits");
        draft.insert("grading".to_owned(), Value::Object(grading));
    });
    let found = checks(&result_of(&none));
    for check in [
        "bounding_box_against_fabric",
        "notch_spacing",
        "dart_intake_against_maximum",
        "minimum_curvature_radius",
    ] {
        assert_eq!(
            verdict(&found, check, "XS")["verdict"].as_text(),
            Some("no_limit"),
            "{check}"
        );
    }
}

#[test]
fn a_size_too_far_to_draw_says_so_and_the_other_end_is_still_checked() {
    // A largest size so narrow at the waist that the darts cross the side.
    let far = rewritten(BODICE, &|draft| {
        let mut sizes = fields_of(&draft["sizes"]).clone();
        let mut list = sizes["list"].as_list().expect("a list").to_vec();
        list.push(
            parse(r#"{"name":"impossible","inputs":{"bust_girth":100000,"waist_girth":100000}}"#)
                .expect("valid"),
        );
        sizes.insert("list".to_owned(), Value::List(list));
        draft.insert("sizes".to_owned(), Value::Object(sizes));
    });
    let plan = with_steps(&far, r#"[{"do":"check_extremes"}]"#);
    // Regenerating it is what refuses first: its recipe cannot get there, by name.
    let failure = replay(&plan, MemoryRepository::new())
        .expect_err("no such garment")
        .to_value();
    let said = fields_of(&failure)["message"].as_text().expect("text");
    assert!(said.contains("size impossible"), "{said}");
}

#[test]
fn a_point_through_the_file_is_judged_at_the_files_own_quantum() {
    for session in [SKIRT, BODICE] {
        let result = result_of(session);
        let equivalence = fields_of(&result["equivalence"]);
        assert_eq!(equivalence["provenance"].as_text(), Some("received"));
        for finding in equivalence["findings"]
            .as_list()
            .expect("findings")
            .iter()
            .map(fields_of)
        {
            let quantity = finding["quantity"].as_text().expect("text");
            assert!(
                !quantity.starts_with("point "),
                "a point is within what two decimals can hold: {finding:?}"
            );
        }
        let classes = equivalence["classes"].as_list().expect("classes");
        assert!(classes
            .iter()
            .map(fields_of)
            .any(|class| class["class"].as_text() == Some("T3")));
    }
}

#[test]
fn a_caller_who_cannot_generate_is_refused_and_a_garment_with_no_sizes_is_told_so() {
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
        .step(&parse(r#"{"do":"check_extremes"}"#).expect("valid"))
        .is_err());

    let unsized_garment = rewritten(SKIRT, &|draft| {
        draft.remove("sizes");
    });
    let plan = with_steps(&unsized_garment, r#"[{"do":"check_extremes"}]"#);
    let failure = replay(&plan, MemoryRepository::new())
        .expect_err("nothing to check")
        .to_value();
    assert!(fields_of(&failure)["message"]
        .as_text()
        .expect("text")
        .contains("names no sizes"));
}
