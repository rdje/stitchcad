//! Holds the physical-evaluation protocol to the fixture and reads its records, so a measurement cannot be lost to a typo and a constant cannot be left with nothing to falsify it.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

const CHAPTER: &str = include_str!("../../../docs/book/src/spec/physical-evaluation.md");
const FIXTURE: &str = include_str!("../../../docs/book/src/spec/reference-skirt.md");
const MEASUREMENTS: &str = include_str!("../../../conformance/reference-skirt/measurements.tsv");
const PAPER: &str = include_str!("../../../conformance/printed-square/paper-measurement.tsv");
// The constants the leaf names, and the ones §11 of the fixture marks as assumed.
const ASSUMED: [&str; 7] = [
    "ss_suppress",
    "dart_intake",
    "dart_intake_max",
    "dart_len_front",
    "dart_len_back",
    "a_line_flare",
    "wb_width",
];

// The rows of the table whose first cell is a constant.
fn falsifiers() -> Vec<Vec<String>> {
    let start = CHAPTER
        .find("## What each measurement can prove wrong")
        .expect("the section");
    let end = CHAPTER[start..]
        .find("## The record")
        .expect("the next section");
    CHAPTER[start..start + end]
        .lines()
        .filter(|line| line.starts_with("| `"))
        .map(|line| {
            line.trim_matches('|')
                .split(" | ")
                .map(|cell| cell.trim().to_owned())
                .collect()
        })
        .collect()
}

#[test]
fn every_assumed_constant_has_a_measurement_that_would_falsify_it() {
    let rows = falsifiers();
    for constant in ASSUMED {
        let wanted = format!("`{constant}`");
        let row = rows
            .iter()
            .find(|row| row[0] == wanted)
            .unwrap_or_else(|| panic!("{constant} has no row"));
        assert_eq!(row.len(), 4, "{row:?}");
        assert!(row.iter().all(|cell| !cell.is_empty()), "{row:?}");
    }
    assert_eq!(
        rows.len(),
        ASSUMED.len(),
        "a row for something that is not one of the constants"
    );
}

#[test]
fn what_the_protocol_says_each_constant_is_what_the_fixture_says() {
    for row in falsifiers() {
        let name = row[0].trim_matches('`');
        // This one is the formula language's worked example, not the fixture's own table.
        if name == "dart_intake_max" {
            let formula = include_str!("../../../docs/book/src/spec/formula-language.md");
            assert!(
                formula.contains("`dart_intake_max` = 5.0 cm"),
                "the formula chapter changed"
            );
            assert!(row[1].starts_with("5.0 cm"), "{}", row[1]);
            continue;
        }
        // The fixture's own table, §3: `name` | 3.0 cm | ...
        let line = FIXTURE
            .lines()
            .find(|line| line.starts_with(&format!("| `{name}` | ")))
            .unwrap_or_else(|| panic!("{name} is not in the fixture's table"));
        let value = line.split(" | ").nth(1).expect("a value").trim();
        assert!(
            row[1].starts_with(value),
            "{name}: the protocol says {}, and the fixture says {value}",
            row[1]
        );
    }
}

#[test]
fn the_records_columns_are_the_ones_the_chapter_lists() {
    let header: Vec<&str> = MEASUREMENTS
        .lines()
        .next()
        .expect("a header")
        .split('\t')
        .collect();
    assert_eq!(
        header,
        ["constant", "measured", "value", "verdict", "by", "evidence", "date"]
    );
    let start = CHAPTER.find("## The record").expect("the section");
    for column in &header {
        assert!(
            CHAPTER[start..].contains(&format!("| `{column}` | ")),
            "{column}"
        );
    }
    assert!(PAPER.lines().next().expect("a header").contains("value_mm"));
}

#[test]
fn every_measurement_recorded_names_a_known_constant_a_known_verdict_and_who_took_it() {
    for (at, line) in MEASUREMENTS.lines().enumerate().skip(1) {
        let cells: Vec<&str> = line.split('\t').collect();
        assert_eq!(cells.len(), 7, "line {}: {line}", at + 1);
        assert!(
            cells[0] == "garment" || ASSUMED.contains(&cells[0]),
            "line {}: no constant is named {}",
            at + 1,
            cells[0]
        );
        assert!(
            ["holds", "disagrees", "could_not_measure"].contains(&cells[3]),
            "line {}: no verdict is named {}",
            at + 1,
            cells[3]
        );
        assert!(
            !cells[4].trim().is_empty(),
            "line {}: a measurement says who took it",
            at + 1
        );
        assert!(
            !cells[6].trim().is_empty(),
            "line {}: a measurement says when",
            at + 1
        );
    }
}

#[test]
fn a_record_with_measurements_cannot_leave_the_book_saying_none_were_made() {
    let measured = MEASUREMENTS.lines().count() > 1 || PAPER.lines().count() > 1;
    let claims_none = CHAPTER.contains("**Neither has been performed.**");
    assert_eq!(
        measured, !claims_none,
        "the chapter and the records disagree about whether anything was measured"
    );
}
