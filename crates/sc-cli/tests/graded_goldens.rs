//! The graded families frozen through the canonical form: every size of every garment, by both paths, and the table between them. A one micrometre change anywhere turns this red.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_cli::replay;
use sc_grading::GradeTable;
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::BTreeMap;

const GARMENTS: [(&str, &str); 3] = [
    (
        "skirt",
        include_str!("../../../conformance/reference-skirt/session.json"),
    ),
    (
        "bodice",
        include_str!("../../../conformance/reference-bodice/session.json"),
    ),
    (
        "trousers",
        include_str!("../../../conformance/reference-trousers/session.json"),
    ),
];
const FROZEN: &str = include_str!("../../../conformance/goldens/graded/families.tsv");

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

// Every digest a garment comes to, by name.
fn digests(name: &str, session: &str, table: Option<&str>) -> BTreeMap<String, String> {
    let given = table.map_or(String::new(), |text| {
        format!(
            r#","table_text":{}"#,
            Value::Text(text.to_owned()).to_text()
        )
    });
    let plan = with_steps(
        session,
        &format!(
            r#"[{{"do":"regenerate"}},{{"do":"grade","rules_path":"exports/rules.json"{given}}},{{"do":"save"}}]"#
        ),
    );
    let mut disk = MemoryRepository::new();
    let transcript =
        replay(&plan, &mut disk).unwrap_or_else(|failure| panic!("{name}: {failure:?}"));
    let steps = transcript.as_list().expect("a transcript");
    let mut out = BTreeMap::new();
    for (path, record) in [
        ("recipe", &steps[steps.len() - 3]),
        ("rule", &steps[steps.len() - 2]),
    ] {
        for report in fields_of(&fields_of(record)["result"])["sizes"]
            .as_list()
            .expect("sizes")
        {
            let report = fields_of(report);
            out.insert(
                format!(
                    "{name}/{path}/{}",
                    report["size"].as_text().expect("a size")
                ),
                report["canonical_sha256"]
                    .as_text()
                    .expect("a digest")
                    .to_owned(),
            );
        }
    }
    let rules = disk
        .read(&FilePath::new("exports/rules.json").expect("a path"))
        .expect("reads")
        .expect("written");
    out.insert(format!("{name}/table"), sc_store::sha256_hex(&rules));
    out
}

fn frozen() -> BTreeMap<String, String> {
    FROZEN
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_once('\t'))
        .map(|(key, digest)| (key.to_owned(), digest.to_owned()))
        .collect()
}

#[test]
fn every_size_by_both_paths_and_every_table_is_what_was_frozen() {
    let wanted = frozen();
    let mut found = BTreeMap::new();
    for (name, session) in GARMENTS {
        found.extend(digests(name, session, None));
    }
    // A line for each: this prints what is there, so a refreeze is reviewed rather than guessed at.
    let mismatched: Vec<String> = found
        .iter()
        .filter(|(key, digest)| wanted.get(*key) != Some(*digest))
        .map(|(key, digest)| format!("{key}\t{digest}"))
        .collect();
    assert!(
        mismatched.is_empty(),
        "not what was frozen:\n{}",
        mismatched.join("\n")
    );
    assert_eq!(
        found.len(),
        wanted.len(),
        "nothing frozen that nothing checks"
    );
    assert_eq!(found.len(), 3 * (5 + 5 + 1));
}

// The suite is only worth what it catches. So a rule is moved by one micrometre at one size, and the suite has to say which size of which garment, and no other.
#[test]
fn a_single_rule_changed_by_a_micrometre_turns_the_suite_red_at_that_size_and_no_other() {
    let (name, session) = GARMENTS[0];
    let plan = with_steps(
        session,
        r#"[{"do":"grade","rules_path":"exports/rules.json"},{"do":"save"}]"#,
    );
    let mut disk = MemoryRepository::new();
    replay(&plan, &mut disk).expect("replays");
    let text = String::from_utf8(
        disk.read(&FilePath::new("exports/rules.json").expect("a path"))
            .expect("reads")
            .expect("written"),
    )
    .expect("text");
    let ours = parse(&text).expect("valid");
    let mut fields = fields_of(&ours).clone();
    let rules: Vec<Value> = fields["rules"]
        .as_list()
        .expect("rules")
        .iter()
        .map(|rule| {
            let mut rule = fields_of(rule).clone();
            if rule["point"].as_text() == Some("skirt_front/hem/to") {
                let mut deltas = rule["deltas"].as_list().expect("deltas").to_vec();
                let was = deltas[3].as_list().expect("a pair").to_vec();
                deltas[3] = Value::List(vec![
                    Value::Int(was[0].as_int().expect("x") + 1),
                    was[1].clone(),
                ]);
                rule.insert("deltas".to_owned(), Value::List(deltas));
            }
            Value::Object(rule)
        })
        .collect();
    fields.insert("rules".to_owned(), Value::List(rules));
    let mutated = GradeTable::from_value(&Value::Object(fields))
        .expect("reads")
        .to_value()
        .to_text();
    let found = digests(name, session, Some(&mutated));
    let wanted = frozen();
    let red: Vec<&String> = found
        .iter()
        .filter(|(key, digest)| wanted.get(*key) != Some(*digest))
        .map(|(key, _)| key)
        .collect();
    assert!(red.contains(&&"skirt/rule/L".to_owned()), "{red:?}");
    assert!(!red.contains(&&"skirt/rule/M".to_owned()), "{red:?}");
    assert!(
        !red.iter().any(|key| key.contains("/recipe/")),
        "the recipe's path did not move: {red:?}"
    );
}
