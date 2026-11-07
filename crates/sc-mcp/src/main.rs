//! `stitchcad-mcp`: the `local-stdio` MCP server. A host launches it and speaks over its pipes, and when the pipes close, it ends.
use sc_api::Api;
use sc_cli::{Caller, CommandAuthority};
use sc_mcp::{compact, handle};
use std::io::{BufRead, Write};
use std::path::PathBuf;

const USAGE: &str = "usage: stitchcad-mcp --project DIR [--book DIR] [--authority inspect,propose,commit,generate] [--human] [--recording FILE]";

fn authority(name: &str) -> Result<CommandAuthority, String> {
    match name {
        "inspect" => Ok(CommandAuthority::Inspect),
        "propose" => Ok(CommandAuthority::Propose),
        "commit" => Ok(CommandAuthority::Commit),
        "generate" => Ok(CommandAuthority::Generate),
        "approve" => Ok(CommandAuthority::Approve),
        other => Err(format!("no authority is named {other}")),
    }
}

fn run() -> Result<(), String> {
    let (mut project, mut book, mut recording, mut agent) = (None, None, None::<PathBuf>, true);
    let mut authorities = vec![
        CommandAuthority::Inspect,
        CommandAuthority::Propose,
        CommandAuthority::Commit,
        CommandAuthority::Generate,
    ];
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        let mut value = || arguments.next().ok_or_else(|| USAGE.to_owned());
        match argument.as_str() {
            "--project" => project = Some(PathBuf::from(value()?)),
            "--book" => book = Some(PathBuf::from(value()?)),
            "--recording" => recording = Some(PathBuf::from(value()?)),
            "--human" => agent = false,
            "--authority" => {
                authorities = value()?
                    .split(',')
                    .map(authority)
                    .collect::<Result<_, _>>()?
            }
            _ => return Err(USAGE.to_owned()),
        }
    }
    let project = project.ok_or_else(|| USAGE.to_owned())?;
    let caller = Caller { agent, authorities };
    let mut api = Api::new(caller, &project, book.as_deref()).map_err(|refusal| refusal.message)?;
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let line = line.map_err(|error| error.to_string())?;
        if line.trim().is_empty() {
            continue;
        }
        if let Some(answer) = handle(&mut api, &line) {
            writeln!(stdout, "{answer}").map_err(|error| error.to_string())?;
            stdout.flush().map_err(|error| error.to_string())?;
        }
        // Whatever the host does next, what was done so far is on disk for the replay.
        if let (Some(file), Some(session)) = (&recording, api.recording()) {
            std::fs::write(file, compact(&session)).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(message) = run() {
        eprintln!("stitchcad-mcp: {message}");
        std::process::exit(1);
    }
}
