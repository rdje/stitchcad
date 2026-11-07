//! The Factory Profile file: the roadmap's own example, and what a mistake in one is told.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_profiles::{
    check_succession, decimal_text, parse_decimal, ContentVersion, ParamValue, Profile, Refusal,
    State,
};
use sc_store::{parse, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_text() -> String {
    fs::read_to_string(root().join("conformance/profiles/acme-cmt-pt/profile.json"))
        .expect("the fixture is there")
}

fn fixture() -> Value {
    parse(&fixture_text()).expect("the fixture parses")
}

fn evidence() -> BTreeSet<String> {
    ["ev:2026-07-14:acceptance#acme", "ev:2026-07-14"]
        .map(str::to_owned)
        .into()
}

/// The example block of roadmap §8 with its comments gone and each fractional literal as exact text.
fn roadmap_example() -> Value {
    let roadmap = fs::read_to_string(root().join("ROADMAP.md")).expect("the roadmap is there");
    let section = &roadmap[roadmap.find("## 8. Factory Profiles").expect("section 8")..];
    let block = &section[section.find("```jsonc\n").expect("its example") + 9..];
    let block = &block[..block.find("```").expect("its end")];
    let (mut out, mut rest, mut in_text) = (String::new(), block, false);
    while let Some(next) = rest.chars().next() {
        if in_text {
            in_text = next != '"';
        } else if rest.starts_with("//") {
            rest = &rest[rest.find('\n').expect("a line end")..];
            continue;
        } else if rest.starts_with("/*") {
            rest = &rest[rest.find("*/").expect("a comment's end") + 2..];
            continue;
        } else if next.is_ascii_digit() {
            let end = rest
                .find(|c: char| !c.is_ascii_digit() && c != '.')
                .expect("a number's end");
            let number = &rest[..end];
            if number.contains('.') {
                out.push_str(&format!("\"{number}\""));
            } else {
                out.push_str(number);
            }
            rest = &rest[end..];
            continue;
        } else {
            in_text = next == '"';
        }
        out.push(next);
        rest = &rest[next.len_utf8()..];
    }
    parse(&out).expect("the example is a value")
}

#[test]
fn the_roadmaps_example_validates_and_is_the_fixture_byte_for_byte() {
    let profile = Profile::read(&roadmap_example(), &evidence()).expect("it validates");
    assert_eq!(profile.to_value().to_text(), fixture_text());
    assert_eq!(profile.profile_id, "acme-cmt-pt");
    assert_eq!(profile.content_version, ContentVersion(1, 3, 0));
    assert_eq!(profile.parameters.len(), 10);
}

#[test]
fn a_profile_written_and_read_again_is_the_same_bytes() {
    let profile = Profile::read(&fixture(), &evidence()).expect("it validates");
    let again = Profile::read(&profile.to_value(), &evidence()).expect("it validates again");
    assert_eq!(profile, again);
    assert_eq!(again.to_value().to_text(), fixture_text());
}

fn parameter<'v>(file: &'v mut Value, name: &str) -> &'v mut BTreeMap<String, Value> {
    let Value::Object(top) = file else {
        panic!("a file is an object")
    };
    let Some(Value::List(items)) = top.get_mut("parameters") else {
        panic!("a file has parameters")
    };
    items
        .iter_mut()
        .find_map(|item| match item {
            Value::Object(fields) if fields.get("name").and_then(Value::as_text) == Some(name) => {
                Some(fields)
            }
            _ => None,
        })
        .expect("the parameter is there")
}

fn top(file: &mut Value) -> &mut BTreeMap<String, Value> {
    let Value::Object(fields) = file else {
        panic!("a file is an object")
    };
    fields
}

fn text(value: &str) -> Value {
    Value::Text(value.to_owned())
}

type Mutation = fn(&mut Value);

const MISTAKES: [(&str, Mutation, &str, &str); 25] = [
    (
        "a misspelt parameter",
        |file| {
            parameter(file, "units").insert("name".to_owned(), text("unit"));
        },
        "profile_parameter_unknown",
        "parameters[2].name",
    ),
    (
        "a parameter given twice",
        |file| {
            parameter(file, "units").insert("name".to_owned(), text("cut_sew_swap"));
        },
        "profile_parameter_repeated",
        "parameters[8].name",
    ),
    (
        "a value on an unknown",
        |file| {
            parameter(file, "units").insert("value".to_owned(), text("cm"));
        },
        "profile_state_conflict",
        "parameters[units].value",
    ),
    (
        "an unknown with no way to an answer",
        |file| {
            parameter(file, "units").remove("resolve");
        },
        "profile_state_conflict",
        "parameters[units].resolve",
    ),
    (
        "a default on a known",
        |file| {
            parameter(file, "cut_sew_swap").insert("default".to_owned(), Value::Bool(true));
        },
        "profile_state_conflict",
        "parameters[cut_sew_swap].default",
    ),
    (
        "a known with no value",
        |file| {
            parameter(file, "cut_sew_swap").remove("value");
        },
        "profile_state_conflict",
        "parameters[cut_sew_swap].value",
    ),
    (
        "an effect on a known",
        |file| {
            parameter(file, "cut_sew_swap")
                .insert("artifact_effect".to_owned(), text("blocks_all"));
        },
        "profile_state_conflict",
        "parameters[cut_sew_swap].artifact_effect",
    ),
    (
        "a preference that says nothing of where it came from",
        |file| {
            parameter(file, "cut_sew_swap").insert("state".to_owned(), text("preference"));
        },
        "profile_state_conflict",
        "parameters[cut_sew_swap].provenance",
    ),
    (
        "a value on a choice",
        |file| {
            parameter(file, "cut_sew_swap").insert("state".to_owned(), text("choice"));
        },
        "profile_state_conflict",
        "parameters[cut_sew_swap].value",
    ),
    (
        "a derived value, with no formula to derive it",
        |file| {
            parameter(file, "cut_sew_swap").insert("state".to_owned(), text("derived"));
        },
        "profile_state_unavailable",
        "parameters[cut_sew_swap].state",
    ),
    (
        "a value the domain does not list",
        |file| {
            parameter(file, "spline_policy").insert("value".to_owned(), text("explode"));
        },
        "profile_value_outside_domain",
        "parameters[spline_policy].value",
    ),
    (
        "a domain nothing understands",
        |file| {
            parameter(file, "units").insert(
                "domain".to_owned(),
                Value::List(vec![text("mm"), text("furlong")]),
            );
        },
        "profile_token_unknown",
        "parameters[units].domain[1]",
    ),
    (
        "the wrong type for a parameter",
        |file| {
            parameter(file, "max_cutting_width_mm").insert("type".to_owned(), text("bool"));
        },
        "profile_type_mismatch",
        "parameters[max_cutting_width_mm].type",
    ),
    (
        "a transformation with no stage",
        |file| {
            parameter(file, "shrinkage_xy_pct").remove("transformation_stage");
        },
        "profile_stage_wrong",
        "parameters[shrinkage_xy_pct].transformation_stage",
    ),
    (
        "a transformation at a stage of its own",
        |file| {
            parameter(file, "shrinkage_xy_pct")
                .insert("transformation_stage".to_owned(), text("before_grading"));
        },
        "profile_stage_wrong",
        "parameters[shrinkage_xy_pct].transformation_stage",
    ),
    (
        "a stage on something that is not a transformation",
        |file| {
            parameter(file, "cut_sew_swap").insert(
                "transformation_stage".to_owned(),
                text("post_grade_pre_export"),
            );
        },
        "profile_stage_wrong",
        "parameters[cut_sew_swap].transformation_stage",
    ),
    (
        "a width that is not above zero",
        |file| {
            parameter(file, "max_cutting_width_mm").insert("value".to_owned(), Value::Int(0));
        },
        "profile_field_malformed",
        "parameters[max_cutting_width_mm].value",
    ),
    (
        "a notch shape nobody names",
        |file| {
            let mut notch = BTreeMap::new();
            notch.insert("code".to_owned(), text("Z"));
            notch.insert("depth_mm".to_owned(), Value::Int(5));
            notch.insert("sample_mm".to_owned(), Value::Int(3));
            parameter(file, "notch_geometry").insert("value".to_owned(), Value::Object(notch));
        },
        "profile_token_unknown",
        "parameters[notch_geometry].value.code",
    ),
    (
        "a pen for no function",
        |file| {
            let pens = BTreeMap::from([("sew".to_owned(), Value::Int(1))]);
            parameter(file, "hpgl_pens").insert("value".to_owned(), Value::Object(pens));
        },
        "profile_field_unknown",
        "parameters[hpgl_pens].value.sew",
    ),
    (
        "a field the schema does not have",
        |file| {
            top(file).insert("verified".to_owned(), Value::Bool(true));
        },
        "profile_field_unknown",
        "verified",
    ),
    (
        "a file of another schema",
        |file| {
            top(file).insert("schema_version".to_owned(), text("3.0"));
        },
        "profile_schema_version",
        "schema_version",
    ),
    (
        "a content version that is not three numbers",
        |file| {
            top(file).insert("content_version".to_owned(), text("1.3"));
        },
        "profile_field_malformed",
        "content_version",
    ),
    (
        "a target nothing registers",
        |file| {
            top(file).insert("accepts".to_owned(), Value::List(vec![text("dxf-r99")]));
        },
        "profile_token_unknown",
        "accepts[0]",
    ),
    (
        "defaults above hard restrictions",
        |file| {
            let order = [
                "default",
                "preference",
                "factory_override",
                "hard_restriction",
            ];
            let list = Value::List(order.map(text).to_vec());
            let composition = BTreeMap::from([("precedence".to_owned(), list)]);
            top(file).insert("composition".to_owned(), Value::Object(composition));
        },
        "profile_precedence_wrong",
        "composition.precedence",
    ),
    (
        "rules, which the next leaf reads",
        |file| {
            top(file).insert(
                "constraints_ast".to_owned(),
                Value::List(vec![text("a < b")]),
            );
        },
        "profile_constraints_unread",
        "constraints_ast",
    ),
];

fn refused(file: &Value) -> Vec<Refusal> {
    Profile::read(file, &evidence()).expect_err("this is refused")
}

#[test]
fn each_mistake_is_refused_by_its_code_and_names_the_field_to_fix() {
    for (what, change, code, field) in MISTAKES {
        let mut file = fixture();
        change(&mut file);
        let refusals = refused(&file);
        assert!(
            refusals
                .iter()
                .any(|refusal| refusal.code == code && refusal.field == field),
            "{what}: {refusals:?}"
        );
    }
}

#[test]
fn a_dangling_evidence_reference_is_refused_and_names_the_field() {
    let refusals = Profile::read(&fixture(), &BTreeSet::from(["ev:2026-07-14".to_owned()]))
        .expect_err("one reference points at nothing");
    assert_eq!(refusals.len(), 1);
    assert_eq!(refusals[0].code, "profile_evidence_dangling");
    assert_eq!(
        refusals[0].field,
        "parameters[seam_allowance_policy].evidence[0]"
    );
    assert!(refusals[0]
        .message
        .contains("ev:2026-07-14:acceptance#acme"));
}

#[test]
fn every_mistake_in_a_file_is_reported_at_once_in_the_order_of_the_file() {
    let mut file = fixture();
    top(&mut file).insert("schema_version".to_owned(), text("3.0"));
    parameter(&mut file, "units").insert("name".to_owned(), text("unit"));
    parameter(&mut file, "cut_sew_swap").remove("value");
    let fields: Vec<String> = refused(&file)
        .into_iter()
        .map(|refusal| refusal.field)
        .collect();
    assert_eq!(
        fields,
        [
            "schema_version",
            "parameters[2].name",
            "parameters[cut_sew_swap].value"
        ]
    );
}

#[test]
fn an_unknown_holds_no_value_and_its_default_is_only_ever_offered() {
    let profile = Profile::read(&fixture(), &evidence()).expect("it validates");
    let units = profile
        .parameters
        .iter()
        .find(|parameter| parameter.name == "units")
        .expect("units");
    assert_eq!(units.state, State::Unknown);
    assert_eq!(units.value, None);
    assert_eq!(units.default, Some(ParamValue::Symbol("cm".to_owned())));
}

#[test]
fn an_unknown_may_have_no_default_because_nothing_should_be_invented() {
    let mut file = fixture();
    parameter(&mut file, "units").remove("default");
    assert!(Profile::read(&file, &evidence()).is_ok());
}

#[test]
fn a_known_with_no_evidence_is_read_and_listed_because_the_roadmaps_example_has_six() {
    let profile = Profile::read(&fixture(), &evidence()).expect("it validates");
    assert_eq!(
        profile.known_without_evidence(),
        [
            "dxf_version_target",
            "spline_policy",
            "shrinkage_xy_pct",
            "max_cutting_width_mm",
            "piece_block_required",
            "cut_sew_swap"
        ]
    );
}

#[test]
fn a_version_range_says_what_it_covers() {
    let profile = Profile::read(&fixture(), &evidence()).expect("it validates");
    for covered in ["13", "13.0", "13.2", "13.2.1"] {
        assert!(profile.system.admits(covered), "{covered}");
    }
    for outside in ["14", "130", "1.13", "12.9"] {
        assert!(!profile.system.admits(outside), "{outside}");
    }
    let mut exact = profile.system.clone();
    exact.version_range = vec!["13.2".to_owned()];
    assert!(exact.admits("13.2"));
    assert!(!exact.admits("13.2.1"));
    assert!(!exact.admits("13"));

    for bad in ["x", "x.13", "13.x.2", "13.", "13 2"] {
        let mut file = fixture();
        let Some(Value::Object(system)) = top(&mut file).get_mut("system") else {
            panic!("a system")
        };
        system.insert("version_range".to_owned(), Value::List(vec![text(bad)]));
        let refusals = refused(&file);
        assert_eq!(refusals[0].field, "system.version_range[0]", "{bad}");
    }
}

#[test]
fn the_content_version_moves_when_the_content_does_and_only_then() {
    let earlier = Profile::read(&fixture(), &evidence()).expect("it validates");

    // Nothing changed, nothing moved.
    assert_eq!(check_succession(&earlier, &earlier), Ok(()));

    // Only the label moved.
    let mut label = fixture();
    top(&mut label).insert("content_version".to_owned(), text("1.3.1"));
    let label = Profile::read(&label, &evidence()).expect("it validates");
    assert_eq!(earlier.content_digest(), label.content_digest());
    let refusal = check_succession(&earlier, &label).expect_err("nothing changed");
    assert_eq!(refusal.code, "profile_version_without_change");

    // Only the content moved.
    let mut content = fixture();
    parameter(&mut content, "cut_sew_swap").insert("value".to_owned(), Value::Bool(true));
    let content = Profile::read(&content, &evidence()).expect("it validates");
    assert_ne!(earlier.content_digest(), content.content_digest());
    let refusal = check_succession(&earlier, &content).expect_err("no new version");
    assert_eq!(refusal.code, "profile_change_without_version");

    // The content moved and the version went down.
    let mut lower = fixture();
    parameter(&mut lower, "cut_sew_swap").insert("value".to_owned(), Value::Bool(true));
    top(&mut lower).insert("content_version".to_owned(), text("1.2.9"));
    let lower = Profile::read(&lower, &evidence()).expect("it validates");
    assert_eq!(
        check_succession(&earlier, &lower)
            .expect_err("it went down")
            .code,
        "profile_change_without_version"
    );

    // Both moved together.
    let mut both = fixture();
    parameter(&mut both, "cut_sew_swap").insert("value".to_owned(), Value::Bool(true));
    top(&mut both).insert("content_version".to_owned(), text("1.4.0"));
    let both = Profile::read(&both, &evidence()).expect("it validates");
    assert_eq!(check_succession(&earlier, &both), Ok(()));

    // Another profile is not a successor at all.
    let mut other = fixture();
    top(&mut other).insert("profile_id".to_owned(), text("another-factory"));
    let other = Profile::read(&other, &evidence()).expect("it validates");
    assert_eq!(
        check_succession(&earlier, &other)
            .expect_err("a different profile")
            .code,
        "profile_identity_changed"
    );
}

#[test]
fn two_spellings_of_one_number_are_one_content() {
    let earlier = Profile::read(&fixture(), &evidence()).expect("it validates");
    let mut spelt = fixture();
    parameter(&mut spelt, "max_cutting_width_mm").insert("value".to_owned(), text("1520.0"));
    let spelt = Profile::read(&spelt, &evidence()).expect("it validates");
    assert_eq!(earlier.content_digest(), spelt.content_digest());
    assert_eq!(earlier.to_value().to_text(), spelt.to_value().to_text());
}

#[test]
fn a_decimal_is_exact_and_has_one_spelling() {
    for (read, written) in [
        ("0.1", "0.1"),
        ("-0.25", "-0.25"),
        ("1520.50", "1520.5"),
        ("2.0", "2"),
        ("0.000001", "0.000001"),
        ("-3", "-3"),
    ] {
        let number = parse_decimal(read).expect(read);
        assert_eq!(decimal_text(number).as_deref(), Some(written), "{read}");
    }
    for refused in [
        "",
        ".5",
        "1.",
        "1.2.3",
        "1e3",
        "+1",
        "0x10",
        "1234567890123456789",
    ] {
        assert_eq!(parse_decimal(refused), None, "{refused}");
    }
}
