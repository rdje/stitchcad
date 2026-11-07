//! The agent gate's verdict, from each session's record and nothing the agent said about itself. Reading a record needs no model, so this runs anywhere, and a model's nondeterminism
//! can never block the command-line path: `G2-01` is that gate, and this is a separate verdict.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_artifacts::{
    compare, from_canonical_text, judge_pattern, to_canonical_text, Comparison, Expectations,
};
use sc_cli::replay;
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::path::{Path, PathBuf};

const DECLARED: &str = include_str!("../../../conformance/reference-skirt/expectations.json");
const GOLDEN: &str =
    include_str!("../../../conformance/goldens/reference-skirt/reference-skirt.canonical.json");
// T2 of the units chapter: 10 micrometres for internal geometry, the class of a re-flattened curve. It applies to every line the book pins, which is every line.
const TOLERANCE: i64 = 10;

fn runs() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/agent-gate");
    let mut found: Vec<PathBuf> = std::fs::read_dir(root)
        .expect("the record")
        .map(|entry| entry.expect("an entry").path())
        .filter(|path| path.is_dir())
        .collect();
    found.sort();
    found
}

fn name(run: &Path) -> String {
    run.file_name()
        .expect("a name")
        .to_string_lossy()
        .into_owned()
}

fn read(run: &Path, file: &str) -> String {
    std::fs::read_to_string(run.join(file)).unwrap_or_else(|_| panic!("{}: no {file}", name(run)))
}

fn facts(run: &Path) -> Value {
    parse(&read(run, "run.json")).expect("a record")
}

#[test]
fn there_are_two_independent_sessions_each_with_a_pinned_model_and_only_this_server() {
    let all = runs();
    assert!(all.len() >= 2, "{} sessions", all.len());
    let mut transcripts = Vec::new();
    for run in &all {
        let facts = facts(run);
        let fields = facts.as_object().expect("an object");
        let model = fields["model"].as_text().expect("a model");
        assert!(
            model.starts_with("claude-") && model.chars().any(|c| c.is_ascii_digit()),
            "{}: not a pinned id: {model}",
            name(run)
        );
        let tools = fields["tools"].as_list().expect("tools");
        assert!(!tools.is_empty(), "{}", name(run));
        for tool in tools {
            let tool = tool.as_text().expect("text");
            assert!(
                tool.starts_with("mcp__stitchcad__"),
                "{}: it also had {tool}",
                name(run)
            );
        }
        assert!(!read(run, "prompt.txt").is_empty());
        let transcript = read(run, "transcript.txt");
        assert!(
            !transcripts.contains(&transcript),
            "{}: the same transcript twice is one session",
            name(run)
        );
        transcripts.push(transcript);
    }
}

// What the command-line path writes, given only what the agent recorded.
fn replayed(run: &Path) -> String {
    let session = parse(&read(run, "session.json")).expect("a session");
    let mut disk = MemoryRepository::new();
    replay(&session, &mut disk).unwrap_or_else(|failure| panic!("{}: {failure:?}", name(run)));
    let bytes = disk
        .read(&FilePath::new("exports/agent.json").expect("a path"))
        .expect("reads")
        .unwrap_or_else(|| panic!("{}: the session wrote no exports/agent.json", name(run)));
    String::from_utf8(bytes).expect("text")
}

#[test]
fn the_command_line_replays_each_session_to_the_bytes_the_agent_was_handed() {
    for run in runs() {
        assert_eq!(
            replayed(&run),
            read(&run, "canonical.json"),
            "{}",
            name(&run)
        );
    }
}

#[test]
fn each_session_drew_what_the_fixture_declares() {
    let expected = Expectations::from_text(DECLARED).expect("a declaration");
    let mut wrong = Vec::new();
    for run in runs() {
        let pattern = from_canonical_text(&replayed(&run)).expect("reads");
        for finding in judge_pattern(&pattern, &expected) {
            wrong.push(format!(
                "{}: {}: {}",
                name(&run),
                finding.code,
                finding.detail
            ));
        }
    }
    assert!(wrong.is_empty(), "\n{}", wrong.join("\n"));
}

#[test]
fn each_session_golden_matches_within_the_declared_tolerance() {
    let mut wrong = Vec::new();
    for run in runs() {
        // A label's place and punctuation are free, and its words are the judge's above, so the comparison is of everything else.
        let without_labels = |text: &str| {
            let mut pattern = from_canonical_text(text).expect("reads");
            for piece in &mut pattern.pieces {
                piece.labels.clear();
                // Added at G3, after these sessions. A dart's legs are still compared as lines, which the sessions drew.
                piece.closures.clear();
            }
            pattern.style = String::new();
            // Added at G3 too. The judge above walks the fixture's own table of seams over these lines.
            pattern.seams.clear();
            to_canonical_text(&pattern)
        };
        match compare(
            &without_labels(GOLDEN),
            &without_labels(&replayed(&run)),
            TOLERANCE,
        )
        .expect("both read")
        {
            Comparison::Identical
            | Comparison::FormattingOnly
            | Comparison::WithinTolerance { .. } => {}
            Comparison::Different(found) => {
                let listed: Vec<String> = found
                    .iter()
                    .map(|item| format!("{} {}", item.piece, item.field))
                    .collect();
                wrong.push(format!("{}: {}", name(&run), listed.join(", ")));
            }
        }
    }
    assert!(wrong.is_empty(), "\n{}", wrong.join("\n"));
}
