//! The two paths compared, per quantity, with the class each is judged against. Neither wins, and the report does not reconcile them.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::cast_precision_loss
)]
use sc_cli::replay;
use sc_grading::GradeTable;
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
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

struct Report {
    whole: BTreeMap<String, Value>,
    disk: MemoryRepository,
}

fn report_of(plan: &Value) -> Report {
    let mut disk = MemoryRepository::new();
    let transcript = replay(plan, &mut disk).unwrap_or_else(|failure| panic!("{failure:?}"));
    let steps = transcript.as_list().expect("a transcript");
    let graded = fields_of(&steps[steps.len() - 2])["result"].clone();
    let report = fields_of(&fields_of(&graded)["equivalence"]).clone();
    Report {
        whole: report,
        disk,
    }
}

fn graded(session: &str) -> Report {
    report_of(&with_steps(
        session,
        r#"[{"do":"grade","equivalence_path":"exports/e.json"},{"do":"save"}]"#,
    ))
}

fn written(report: &Report) -> Vec<BTreeMap<String, Value>> {
    let text = String::from_utf8(
        report
            .disk
            .read(&FilePath::new("exports/e.json").expect("a path"))
            .expect("reads")
            .expect("written"),
    )
    .expect("text");
    let whole = parse(&text).expect("valid");
    fields_of(&whole)["quantities"]
        .as_list()
        .expect("quantities")
        .iter()
        .map(|item| fields_of(item).clone())
        .collect()
}

// The most any quantity of this kind differs, in micrometres.
fn most(quantities: &[BTreeMap<String, Value>], starts: &str) -> i64 {
    quantities
        .iter()
        .filter(|item| {
            item["quantity"]
                .as_text()
                .expect("text")
                .starts_with(starts)
        })
        .map(|item| item["difference_um"].as_int().expect("a number").abs())
        .max()
        .unwrap_or(0)
}

#[test]
fn every_quantity_says_both_values_the_difference_the_class_and_the_verdict() {
    for session in [SKIRT, BODICE, TROUSERS] {
        let report = graded(session);
        assert_eq!(report.whole["provenance"].as_text(), Some("regenerations"));
        let quantities = written(&report);
        assert_eq!(
            report.whole["compared"].as_int(),
            Some(i64::try_from(quantities.len()).expect("small"))
        );
        assert!(quantities.len() > 100, "{}", quantities.len());
        for item in &quantities {
            for field in [
                "quantity",
                "size",
                "by_recipe",
                "by_rule",
                "difference_um",
                "class",
                "verdict",
            ] {
                assert!(item.contains_key(field), "{field}: {item:?}");
            }
            let class = item["class"].as_text().expect("text");
            assert!(["T1", "T2", "seam"].contains(&class), "{class}");
            if item["verdict"].as_text() == Some("finding") {
                assert_eq!(
                    item["owner"].as_text(),
                    Some("the rule table"),
                    "a finding has an owner: {item:?}"
                );
            } else {
                assert!(!item.contains_key("owner"));
            }
        }
        // The findings are the quantities that did not hold, and no others.
        let findings = report.whole["findings"].as_list().expect("findings");
        let counted = quantities
            .iter()
            .filter(|item| item["verdict"].as_text() == Some("finding"))
            .count();
        assert_eq!(findings.len(), counted);
    }
}

#[test]
fn at_the_sample_there_is_nothing_to_find() {
    for session in [SKIRT, BODICE, TROUSERS] {
        let findings = graded(session).whole["findings"]
            .as_list()
            .expect("findings")
            .to_vec();
        assert!(
            !findings
                .iter()
                .any(|item| fields_of(item)["size"].as_text() == Some("M")),
            "{:?}",
            findings
                .iter()
                .find(|item| fields_of(item)["size"].as_text() == Some("M"))
        );
    }
}

#[test]
fn every_point_a_rule_moves_agrees_at_the_numerical_class() {
    for session in [SKIRT, BODICE, TROUSERS] {
        for item in written(&graded(session)) {
            if item["quantity"]
                .as_text()
                .expect("text")
                .starts_with("point ")
            {
                assert_eq!(item["class"].as_text(), Some("T1"), "{item:?}");
                assert_eq!(item["verdict"].as_text(), Some("holds"), "{item:?}");
            }
        }
    }
}

fn mm(micrometres: i64) -> String {
    format!("{:.1} mm", micrometres as f64 / 1000.0)
}

fn row<'a>(garment: &str) -> &'a str {
    BOOK.lines()
        .find(|line| line.starts_with(&format!("| Reference {garment} |")))
        .unwrap_or_else(|| panic!("the book has no row for {garment}"))
}

// What the book says, against what the report measures. A table of numbers in a chapter is a claim, and this is what stops it going stale.
#[test]
fn the_differences_the_book_states_are_the_ones_measured() {
    let skirt = written(&graded(SKIRT));
    assert_eq!(
        most(&skirt, "sew line "),
        0,
        "the skirt's sew lines match outright"
    );
    let line = row("skirt");
    assert!(line.contains("Not at all"), "{line}");
    assert!(line.contains(&mm(most(&skirt, "seam waist_"))), "{line}");

    let trousers = written(&graded(TROUSERS));
    let line = row("trousers");
    assert!(line.contains(&mm(most(&trousers, "sew line "))), "{line}");
    assert!(line.contains(&mm(most(&trousers, "seam band_"))), "{line}");
    assert!(
        line.contains(&mm(most(&trousers, "seam pocket_"))),
        "{line}"
    );

    let bodice = written(&graded(BODICE));
    let line = row("bodice");
    assert!(line.contains(&mm(most(&bodice, "sew line "))), "{line}");
    assert!(line.contains(&mm(most(&bodice, "seam cap_"))), "{line}");
    assert!(line.contains(&mm(most(&bodice, "seam collar_"))), "{line}");
    assert!(
        line.contains(&mm(most(&bodice, "seam underarm_"))),
        "{line}"
    );
}

// A table from somewhere else, with one corner wrong: the report finds it, by name, at the size, by how much, and says where the table came from.
#[test]
fn a_received_table_with_a_wrong_corner_is_a_finding_at_the_numerical_class() {
    let done = report_of(&with_steps(
        SKIRT,
        r#"[{"do":"grade","rules_path":"exports/r.json"},{"do":"save"}]"#,
    ));
    let text = String::from_utf8(
        done.disk
            .read(&FilePath::new("exports/r.json").expect("a path"))
            .expect("reads")
            .expect("written"),
    )
    .expect("text");
    let ours = parse(&text).expect("valid");
    let mut theirs = fields_of(&ours).clone();
    theirs.insert("provenance".to_owned(), Value::Text("received".to_owned()));
    let rules: Vec<Value> = theirs["rules"]
        .as_list()
        .expect("rules")
        .iter()
        .map(|rule| {
            let mut rule = fields_of(rule).clone();
            if rule["point"].as_text() == Some("skirt_front/hem/to") {
                let mut deltas = rule["deltas"].as_list().expect("deltas").to_vec();
                let was = deltas[4].as_list().expect("a pair").to_vec();
                deltas[4] = Value::List(vec![
                    Value::Int(was[0].as_int().expect("x") + 3_000),
                    was[1].clone(),
                ]);
                rule.insert("deltas".to_owned(), Value::List(deltas));
            }
            Value::Object(rule)
        })
        .collect();
    theirs.insert("rules".to_owned(), Value::List(rules));
    let table = GradeTable::from_value(&Value::Object(theirs)).expect("reads");
    let report = report_of(&with_steps(
        SKIRT,
        &format!(
            r#"[{{"do":"grade","table_text":{}}},{{"do":"save"}}]"#,
            Value::Text(table.to_value().to_text()).to_text()
        ),
    ));
    assert_eq!(report.whole["provenance"].as_text(), Some("received"));
    let found = report.whole["findings"]
        .as_list()
        .expect("findings")
        .iter()
        .map(fields_of)
        .find(|item| item["quantity"].as_text() == Some("point skirt_front/hem/to"))
        .expect("the corner");
    assert_eq!(found["size"].as_text(), Some("XL"));
    assert_eq!(found["class"].as_text(), Some("T1"));
    assert_eq!(found["difference_um"].as_int(), Some(3_000));
}

#[test]
fn a_size_the_recipe_has_no_values_for_is_said_not_to_be_compared() {
    // A table with a size this draft's set does not have: the recipe cannot speak to it, so the report says so.
    let done = report_of(&with_steps(
        SKIRT,
        r#"[{"do":"grade","rules_path":"exports/r.json"},{"do":"save"}]"#,
    ));
    let text = String::from_utf8(
        done.disk
            .read(&FilePath::new("exports/r.json").expect("a path"))
            .expect("reads")
            .expect("written"),
    )
    .expect("text");
    let ours = parse(&text).expect("valid");
    let mut theirs = fields_of(&ours).clone();
    let sizes: Vec<Value> = theirs["sizes"]
        .as_list()
        .expect("sizes")
        .iter()
        .map(|size| {
            if size.as_text() == Some("XL") {
                Value::Text("3XL".to_owned())
            } else {
                size.clone()
            }
        })
        .collect();
    theirs.insert("sizes".to_owned(), Value::List(sizes));
    let report = report_of(&with_steps(
        SKIRT,
        &format!(
            r#"[{{"do":"grade","table_text":{}}},{{"do":"save"}}]"#,
            Value::Text(Value::Object(theirs).to_text()).to_text()
        ),
    ));
    assert_eq!(
        report.whole["not_compared"].as_list().expect("a list"),
        &[Value::Text("3XL".to_owned())]
    );
}

#[test]
fn the_report_is_the_same_bytes_every_time_and_names_where_each_class_comes_from() {
    let bytes = |report: &Report| {
        report
            .disk
            .read(&FilePath::new("exports/e.json").expect("a path"))
            .expect("reads")
            .expect("written")
    };
    assert_eq!(bytes(&graded(TROUSERS)), bytes(&graded(TROUSERS)));
    let report = graded(SKIRT);
    let classes: Vec<&BTreeMap<String, Value>> = report.whole["classes"]
        .as_list()
        .expect("classes")
        .iter()
        .map(fields_of)
        .collect();
    for class in &classes {
        assert!(!class["from"].as_text().expect("text").is_empty());
    }
    let named = |name: &str| {
        classes
            .iter()
            .find(|class| class["class"].as_text() == Some(name))
            .unwrap_or_else(|| panic!("no class {name}"))
    };
    assert_eq!(named("T1")["bound_um"].as_int(), Some(1));
    assert_eq!(named("T2")["bound_um"].as_int(), Some(10));
    assert_eq!(named("seam")["bound_um"].as_int(), Some(20));
    // A receiver's and a factory's tolerances are theirs. None has been declared, and none is made up here.
    assert!(!named("T4").contains_key("bound_um"));
    assert!(!named("T5").contains_key("bound_um"));
}
