//! Holds the conformance matrix chapter to the repository, so a row cannot name a check that does not exist and a clause cannot be left out.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use std::collections::BTreeSet;
use std::path::Path;

const CHAPTER: &str = include_str!("../../../docs/book/src/spec/conformance-matrix.md");
const CLAUSES: &str = include_str!("../../../docs/tasks/artifacts/g0_exit/gate_exit_clauses.tsv");

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the repository")
}

// The rows of the table under a heading, split into cells, with its header gone.
fn rows(heading: &str) -> Vec<Vec<String>> {
    let start = CHAPTER
        .find(heading)
        .unwrap_or_else(|| panic!("no heading {heading}"));
    let after = &CHAPTER[start + heading.len()..];
    let end = after.find("\n## ").unwrap_or(after.len());
    after[..end]
        .lines()
        .filter(|line| line.starts_with('|') && !line.starts_with("| ---"))
        .skip(1)
        .map(|line| {
            line.trim_matches('|')
                .split(" | ")
                .map(|cell| cell.trim().to_owned())
                .collect()
        })
        .collect()
}

fn code(cell: &str) -> &str {
    cell.trim_matches('`')
}

#[test]
fn every_check_the_matrix_names_exists() {
    let claimed = rows("## What is claimed");
    assert!(claimed.len() >= 15, "{}", claimed.len());
    for row in &claimed {
        assert_eq!(row.len(), 5, "{row:?}");
        assert!(
            ["1", "2", "3", "4", "5"].contains(&row[2].as_str()),
            "{row:?}"
        );
        assert!(!row[3].is_empty(), "a claim needs an oracle: {row:?}");
        let check = code(&row[4]);
        let words: Vec<&str> = check.split_whitespace().collect();
        match words.as_slice() {
            ["bash", path, ..] => assert!(root().join(path).is_file(), "{path} does not exist"),
            ["cargo", "test", "-p", package, "--test", test] => {
                let file = root()
                    .join("crates")
                    .join(package)
                    .join("tests")
                    .join(format!("{test}.rs"));
                assert!(file.is_file(), "{} does not exist", file.display());
            }
            other => panic!("not a check this page can verify: {other:?}"),
        }
    }
}

#[test]
fn every_clause_through_g2_is_claimed_or_listed_as_unclaimed() {
    let claimed: BTreeSet<String> = rows("## What is claimed")
        .iter()
        .map(|row| code(&row[1]).to_owned())
        .collect();
    let unclaimed: BTreeSet<String> = rows("## What is unclaimed")
        .iter()
        .map(|row| code(&row[1]).to_owned())
        .collect();
    for line in CLAUSES.lines() {
        let cells: Vec<&str> = line.split('\t').collect();
        let (id, authority) = (cells[0], cells.get(5).copied().unwrap_or(""));
        if !(id.starts_with("G1-") || id.starts_with("G2-")) {
            continue;
        }
        if authority == "instrument" {
            assert!(
                claimed.contains(id),
                "{id} is met and the matrix does not claim it"
            );
            assert!(
                !unclaimed.contains(id),
                "{id} is met and the matrix calls it unclaimed"
            );
        } else {
            assert!(
                unclaimed.contains(id),
                "{id} is open and the matrix does not list it as unclaimed"
            );
            assert!(
                !claimed.contains(id),
                "{id} is open and the matrix claims it"
            );
        }
    }
}

#[test]
fn nothing_reaches_a_layer_this_project_has_not() {
    // Layers 4 and 5 need a real receiver and real cloth. Saying so is the point of this page, and a row claiming either must change that sentence.
    assert!(CHAPTER.contains("Nothing in this project has reached layer 4 or 5 yet."));
    for row in rows("## What is claimed") {
        assert!(
            row[2] != "4" && row[2] != "5",
            "{row:?}: the sentence above is now false"
        );
    }
}

#[test]
fn the_determinism_job_runs_on_three_operating_systems_and_names_these_tests() {
    let workflow = std::fs::read_to_string(root().join(".github/workflows/determinism.yml"))
        .expect("the workflow");
    for os in ["ubuntu-latest", "macos-latest", "windows-latest"] {
        assert!(workflow.contains(os), "{os}");
    }
    for test in [
        "determinism",
        "skirt",
        "save_load",
        "mutants",
        "metamorphic",
        "offset_corpus",
    ] {
        assert!(workflow.contains(&format!("--test {test}")), "{test}");
    }
    let attributes =
        std::fs::read_to_string(root().join(".gitattributes")).expect("the attributes");
    assert!(attributes.contains("conformance/** -text"));
}
