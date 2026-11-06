//! The `stitchcad` command.
use sc_cli::{replay, Failure};
use sc_store::{parse, DirectoryRepository};
use std::process::ExitCode;

const USAGE: &str = "usage: stitchcad replay <session.json> <project-directory>";

fn run(arguments: &[String]) -> Result<String, Failure> {
    let [command, session, project] = arguments else {
        return Err(Failure::usage(USAGE));
    };
    if command != "replay" {
        return Err(Failure::usage(USAGE));
    }
    let text = std::fs::read_to_string(session)
        .map_err(|error| Failure::usage(&format!("{session}: {error}")))?;
    let session_value =
        parse(&text).map_err(|error| Failure::usage(&format!("{session}: {error}")))?;
    let repository = DirectoryRepository::new(project)
        .map_err(|error| Failure::usage(&format!("{project}: {error}")))?;
    replay(&session_value, repository).map(|transcript| transcript.to_text())
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match run(&arguments) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(failure) => {
            print!("{}", failure.to_value().to_text());
            ExitCode::FAILURE
        }
    }
}
