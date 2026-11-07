//! A table's two conventions, its promises, and its file, against a file somebody else wrote down.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_grading::{
    from_rul, to_rul, Attributes, Convention, GradeTable, Provenance, Rule, ZeroState,
};
use sc_store::{parse, Value};

const TRIALS: u64 = 200;

// A small random sequence with a stated seed, so a failure names the case that made it.
struct Dice(u64);

impl Dice {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) ^ self.0
    }

    fn below(&mut self, bound: u64) -> i64 {
        i64::try_from(self.next() % bound).expect("small")
    }

    fn delta(&mut self) -> (i64, i64) {
        (self.below(200_001) - 100_000, self.below(200_001) - 100_000)
    }
}

fn rule(point: &str, id: i64, deltas: Vec<(i64, i64)>) -> Rule {
    Rule {
        point: point.to_owned(),
        id,
        deltas,
        zero: ZeroState::Declared,
    }
}

fn stacked(stack: &str) -> Attributes {
    Attributes {
        stack: vec![stack.to_owned()],
        ..Attributes::default()
    }
}

fn sizes() -> (String, Vec<String>) {
    (
        "M".to_owned(),
        ["XS", "S", "M", "L", "XL"]
            .iter()
            .map(|size| (*size).to_owned())
            .collect(),
    )
}

fn table(rules: Vec<Rule>, attributes: Attributes) -> Result<GradeTable, String> {
    GradeTable::new(
        "t".to_owned(),
        sizes(),
        Convention::Cumulative,
        Provenance::Regenerations,
        rules,
        attributes,
    )
    .map_err(|error| error.to_string())
}

fn at_rest() -> Rule {
    rule("p/still", 1, vec![(0, 0); 5])
}

#[test]
fn both_conventions_say_the_same_thing_to_the_micrometre() {
    let mut dice = Dice(11);
    for trial in 0..TRIALS {
        let mut deltas: Vec<(i64, i64)> = (0..5).map(|_| dice.delta()).collect();
        deltas[2] = (0, 0);
        let cumulative = table(
            vec![at_rest(), rule("p/a", 2, deltas.clone())],
            stacked("p/still"),
        )
        .expect("a table");
        let incremental = cumulative
            .in_convention(Convention::Incremental)
            .expect("converts");
        assert_eq!(incremental.convention(), Convention::Incremental);
        let moved = incremental.rule("p/a").expect("the rule");
        for (at, size) in ["XS", "S", "M", "L", "XL"].iter().enumerate() {
            assert_eq!(
                incremental.delta_at(moved, size).expect("a size"),
                deltas[at],
                "trial {trial}: {size}"
            );
        }
        // Each increment is from the next size toward the sample.
        assert_eq!(moved.deltas[3], (deltas[3].0, deltas[3].1), "trial {trial}");
        assert_eq!(
            moved.deltas[4],
            (deltas[4].0 - deltas[3].0, deltas[4].1 - deltas[3].1),
            "trial {trial}"
        );
        assert_eq!(
            moved.deltas[0],
            (deltas[0].0 - deltas[1].0, deltas[0].1 - deltas[1].1),
            "trial {trial}"
        );
        let back = incremental
            .in_convention(Convention::Cumulative)
            .expect("converts");
        assert_eq!(back, cumulative, "trial {trial}");
    }
}

#[test]
fn a_table_written_and_read_back_is_the_same_table() {
    let mut dice = Dice(12);
    for trial in 0..TRIALS {
        let mut deltas: Vec<(i64, i64)> = (0..5).map(|_| dice.delta()).collect();
        deltas[2] = (0, 0);
        let convention = if trial % 2 == 0 {
            Convention::Cumulative
        } else {
            Convention::Incremental
        };
        let original = table(vec![at_rest(), rule("p/a", 2, deltas)], stacked("p/still"))
            .expect("a table")
            .in_convention(convention)
            .expect("converts");
        let text = original.to_value().to_text();
        let again = GradeTable::from_value(&parse(&text).expect("valid")).expect("reads");
        assert_eq!(again, original, "trial {trial}");
    }
}

#[test]
fn a_table_that_breaks_its_own_promises_is_refused_by_name() {
    let moving = rule("p/a", 2, vec![(1, 0), (2, 0), (0, 0), (3, 0), (4, 0)]);
    for (rules, attributes, says) in [
        (
            vec![at_rest()],
            Attributes::default(),
            "at least one stack point",
        ),
        (vec![at_rest()], stacked("p/nothing"), "has no rule"),
        (
            vec![at_rest(), moving.clone()],
            stacked("p/a"),
            "a stack point does not",
        ),
        (
            vec![at_rest(), rule("p/still", 2, vec![(0, 0); 5])],
            stacked("p/still"),
            "two rules are for the point",
        ),
        (
            vec![at_rest(), rule("p/b", 1, vec![(0, 0); 5])],
            stacked("p/still"),
            "two rules have the number",
        ),
        (
            vec![at_rest(), rule("p/b", 2, vec![(0, 0); 3])],
            stacked("p/still"),
            "has 3 deltas",
        ),
        (
            vec![at_rest(), rule("p/b", 2, vec![(9, 0); 5])],
            stacked("p/still"),
            "moves the sample size",
        ),
        (
            vec![at_rest()],
            Attributes {
                stack: vec!["p/still".to_owned()],
                smoothing: vec!["p/armhole".to_owned()],
                ..Attributes::default()
            },
            "refused rather than ignored",
        ),
    ] {
        let said = table(rules, attributes).expect_err(says);
        assert!(said.contains(says), "{says}: {said}");
    }
    let said = GradeTable::new(
        "t".to_owned(),
        ("Q".to_owned(), vec!["M".to_owned()]),
        Convention::Cumulative,
        Provenance::Received,
        vec![rule("p/still", 1, vec![(0, 0)])],
        stacked("p/still"),
    )
    .expect_err("no such sample")
    .to_string();
    assert!(said.contains("not in the table's sizes"), "{said}");
    assert!(Convention::named("sideways").is_err());
}

#[test]
fn a_file_is_what_the_format_says() {
    let tested = table(
        vec![
            at_rest(),
            rule(
                "p/a",
                2,
                vec![
                    (-20_000, 1_500),
                    (-10_000, 750),
                    (0, 0),
                    (10_000, -750),
                    (20_004, -1_506),
                ],
            ),
        ],
        stacked("p/still"),
    )
    .expect("a table");
    let artifact = to_rul(&tested).expect("writes");
    assert_eq!(
        artifact.text,
        "ASTM/D13 Proposal 1 Version: D 6673-10\n\
         AUTHOR: STITCHCAD\n\
         PRODUCT: STITCHCAD\n\
         UNITS: METRIC\n\
         GRADE RULE TABLE: t\n\
         NUMBER OF SIZES: 5\n\
         SIZE LIST: XS S M L XL\n\
         SAMPLE SIZE: M\n\
         RULE: DELTA 1\n\
         0.00,0.00 0.00,0.00 0.00,0.00 0.00,0.00\n\
         0.00,0.00\n\
         RULE: DELTA 2\n\
         -20.00,1.50 -10.00,0.75 0.00,0.00 10.00,-0.75\n\
         20.00,-1.51\n\
         END\n"
    );
    let record = parse(&artifact.record).expect("valid");
    let record = record.as_object().expect("a record");
    assert_eq!(record["largest_rounding_um"].as_int(), Some(4));
    let carried: Vec<&str> = record["not_carried"]
        .as_list()
        .expect("a list")
        .iter()
        .filter_map(Value::as_text)
        .collect();
    assert!(
        carried.contains(&"stack_points") && carried.contains(&"fixed_contours"),
        "{carried:?}"
    );
}

#[test]
fn a_file_read_back_is_within_what_two_decimals_can_hold_and_says_a_zero_is_unknown() {
    let mut dice = Dice(13);
    for trial in 0..TRIALS {
        let mut deltas: Vec<(i64, i64)> = (0..5).map(|_| dice.delta()).collect();
        deltas[2] = (0, 0);
        let tested = table(
            vec![at_rest(), rule("p/a", 2, deltas.clone())],
            stacked("p/still"),
        )
        .expect("a table");
        let file = from_rul(&to_rul(&tested).expect("writes").text).expect("reads");
        assert_eq!(file.sizes, ["XS", "S", "M", "L", "XL"]);
        assert_eq!(file.sample, "M");
        assert_eq!(file.name, "t");
        let read = file
            .rules
            .iter()
            .find(|rule| rule.id == 2)
            .expect("the rule");
        for (written, back) in deltas.iter().zip(&read.deltas) {
            assert!(
                (written.0 - back.0).abs() <= 5 && (written.1 - back.1).abs() <= 5,
                "trial {trial}: {written:?} and {back:?}"
            );
        }
        assert_eq!(
            file.rules[0].zero,
            ZeroState::Unknown,
            "a file cannot say a zero was meant"
        );
        // A point no rule mentions stays absent: the two are told apart by there being a rule at all.
        assert!(!file.rules.iter().any(|rule| rule.id == 3));
    }
}

// Transcribed from the example grading table published in DH Patterns and Fit's guide to the standard (2011, 2012), which a Gerber AccuMark wrote. It is the one file here this project did not write.
const SOMEBODY_ELSES: &str = "ASTM/D13Proposal 1 Version: D 6673-04
AUTHOR: GERBER TECHNOLOGY
PRODUCT: ACCUMARK
VERSION: 8.4.1
CREATION DATE: 26-01-2011
CREATION TIME: 13:59
UNITS: METRIC
GRADE RULE TABLE: B9000
NUMBER OF SIZES: 8
SIZE LIST:  34 36 38 40 42 44 46 48
SAMPLE SIZE:  36
RULE: DELTA 1
0.00,     0.00      0.00,     0.00      0.00,     0.00      0.00,     0.00
0.00,     0.00      0.00,     0.00      0.00,     0.00      0.00,     0.00
RULE: DELTA 2
7.00,     0.00      0.00,     0.00     -7.00,     0.00    -14.00,     0.00
-20.00,     0.00    -25.98,     0.00    -31.98,     0.00    -37.98,     0.00
RULE: DELTA 3
-3.00,     0.00      0.00,     0.00      3.00,     0.00      6.00,     0.00
10.00,     0.00     14.00,     0.00     17.98,     0.00     21.98,     0.00
RULE: DELTA 3143
0.00,     0.00      0.00,     0.00      0.00,     0.00      0.00,     0.00
20.00,     0.00     20.00,     0.00     20.00,     0.00     20.00,     0.00
END
";

#[test]
fn a_file_this_project_did_not_write_reads_as_its_author_wrote_it() {
    let file = from_rul(SOMEBODY_ELSES).expect("reads");
    assert_eq!(file.name, "B9000");
    assert_eq!(file.sample, "36");
    assert_eq!(file.sizes, ["34", "36", "38", "40", "42", "44", "46", "48"]);
    let ids: Vec<i64> = file.rules.iter().map(|rule| rule.id).collect();
    assert_eq!(ids, [1, 2, 3, 3143]);
    assert_eq!(file.rules[1].deltas[0], (7_000, 0));
    assert_eq!(file.rules[1].deltas[5], (-25_980, 0));
    assert_eq!(file.rules[2].deltas[7], (21_980, 0));
    assert_eq!(file.rules[3].deltas[4], (20_000, 0));
}

#[test]
fn inches_are_inches() {
    let file = from_rul(
        "ASTM/D13 Proposal 1 Version: D 6673-10\nUNITS: ENGLISH\nNUMBER OF SIZES: 2\nSIZE LIST: A B\nSAMPLE SIZE: A\nRULE: DELTA 1\n0.0000,0.0000 0.2500,-1.0000\nEND\n",
    )
    .expect("reads");
    assert_eq!(file.rules[0].deltas[1], (6_350, -25_400));
}

#[test]
fn a_file_that_is_not_right_is_refused_by_name() {
    let good = "ASTM/D13 Proposal 1 Version: D 6673-10\nUNITS: METRIC\nNUMBER OF SIZES: 2\nSIZE LIST: A B\nSAMPLE SIZE: A\nRULE: DELTA 1\n0.00,0.00 1.00,2.00\nEND\n";
    assert!(from_rul(good).is_ok());
    for (from, to, says) in [
        (
            "ASTM/D13 Proposal 1 Version: D 6673-10\n",
            "",
            "does not say it is",
        ),
        ("UNITS: METRIC\n", "", "does not say its units"),
        ("UNITS: METRIC\n", "UNITS: FURLONGS\n", "not known"),
        (
            "NUMBER OF SIZES: 2\n",
            "NUMBER OF SIZES: 3\n",
            "says 3 sizes and lists 2",
        ),
        ("SIZE LIST: A B\n", "", "no size list"),
        ("SAMPLE SIZE: A\n", "", "does not say its sample size"),
        (
            "SAMPLE SIZE: A\n",
            "SAMPLE SIZE: Q\n",
            "not in the size list",
        ),
        ("RULE: DELTA 1\n", "RULE: ANGLE 1\n", "only DELTA exists"),
        (
            "RULE: DELTA 1\n",
            "RULE: DELTA\n",
            "no grade rule identifier",
        ),
        ("0.00,0.00 1.00,2.00\n", "0.00,0.00 1.00\n", "has 3 values"),
        (
            "0.00,0.00 1.00,2.00\n",
            "9.00,0.00 1.00,2.00\n",
            "moves the sample size",
        ),
        (
            "0.00,0.00 1.00,2.00\n",
            "0.00,0.00 one,2.00\n",
            "not a number",
        ),
        ("END\n", "", "stops before END"),
        ("END\n", "END\nRULE: DELTA 2\n", "more after END"),
    ] {
        assert!(good.contains(from), "{from}");
        let text = good.replacen(from, to, 1);
        let said = from_rul(&text).expect_err(says).to_string();
        assert!(said.contains(says), "{says}: {said}");
    }
    let rejected = table(vec![at_rest()], stacked("p/still")).expect("a table");
    let spaced = GradeTable::new(
        "t".to_owned(),
        (
            "M".to_owned(),
            vec!["M".to_owned(), "extra large".to_owned()],
        ),
        Convention::Cumulative,
        Provenance::Regenerations,
        vec![rule("p/still", 1, vec![(0, 0); 2])],
        stacked("p/still"),
    )
    .expect("a table");
    assert!(to_rul(&rejected).is_ok());
    assert!(to_rul(&spaced)
        .expect_err("no room for a space")
        .to_string()
        .contains("cannot go in a size list"));
}
