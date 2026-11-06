//! A real process saving and autosaving a real project directory is killed again and again. The project,
//! and any autosave, must always reopen whole.
#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
mod common;

use common::path;
use sc_store::{Autosave, DirectoryRepository, Project, ProjectStore, Recovery, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const FILES: usize = 6;
const CHILD: &str = "SC_STORE_PROJECT_KILL_CHILD";
const TEST: &str = "a_killed_process_leaves_a_whole_project_on_a_real_disk";

fn tagged(tag: u64) -> Project {
    let mut project = Project::new();
    project.set_field("tag", Value::Text(tag.to_string()));
    for at in 0..FILES {
        let body = format!("{tag}\n{}\n", "y".repeat(1_500 + at));
        project
            .set_file(path(&format!("part{}/file{at}.txt", at % 3)), body)
            .expect("a project file");
    }
    project
}

/// Every file carries the manifest's tag, so a mix of two saves cannot pass.
fn assert_whole(project: &Project, what: &str) {
    let tag = project
        .field("tag")
        .and_then(Value::as_text)
        .expect("a tag")
        .to_owned();
    assert_eq!(project.files().count(), FILES, "{what}");
    for (name, bytes) in project.files() {
        let first = String::from_utf8_lossy(bytes)
            .lines()
            .next()
            .unwrap_or("")
            .to_owned();
        assert_eq!(first, tag, "{what}: {name} disagrees with the manifest");
    }
}

// Beside the project, so it is not a project file. The child counts finished commits here, so the parent can
// choose when to kill it however fast this disk is.
fn progress_file(directory: &Path) -> PathBuf {
    directory.with_extension("progress")
}

fn child(directory: &str) -> ! {
    let directory = PathBuf::from(directory);
    let repository = DirectoryRepository::new(&directory).expect("the directory opens");
    let (mut store, _) = ProjectStore::open(repository).expect("recovery works");
    let mut tag = u64::from(std::process::id()) * 1_000_000;
    let mut finished = 0u64;
    loop {
        fs::write(progress_file(&directory), finished.to_string()).expect("records progress");
        tag += 1;
        // A save, then an autosave, so about half of the kills find an autosave waiting.
        if finished % 2 == 0 {
            store.save(&tagged(tag)).expect("the save works");
        } else {
            store.autosave(&tagged(tag)).expect("the autosave works");
        }
        finished += 1;
    }
}

#[test]
fn a_killed_process_leaves_a_whole_project_on_a_real_disk() {
    if let Ok(directory) = std::env::var(CHILD) {
        child(&directory);
    }
    let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("sc-store")
        .join("project-kill");
    if directory.exists() {
        fs::remove_dir_all(&directory).expect("clears the last run");
    }
    let repository = DirectoryRepository::new(&directory).expect("opens");
    let (mut store, _) = ProjectStore::open(repository).expect("opens");
    store.save(&tagged(0)).expect("the first save works");
    drop(store);

    let (mut interrupted, mut autosaves) = (0, 0);
    for round in 0..30u64 {
        let progress = progress_file(&directory);
        let _ = fs::remove_file(&progress);
        let mut running = Command::new(std::env::current_exe().expect("this test binary"))
            .args([TEST, "--exact", "--nocapture"])
            .env(CHILD, &directory)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("the child starts");
        let begun = Instant::now();
        let wanted = round % 4;
        loop {
            let done = fs::read_to_string(&progress)
                .ok()
                .and_then(|text| text.parse::<u64>().ok());
            if done.is_some_and(|done| done >= wanted) {
                break;
            }
            assert!(
                begun.elapsed() < Duration::from_secs(120),
                "the child made no progress"
            );
            std::thread::sleep(Duration::from_millis(1));
        }
        // The moment varies by round so it lands on different steps of the next commit.
        std::thread::sleep(Duration::from_millis((round * 7) % 29));
        running.kill().expect("the kill works");
        let _ = running.wait();

        let repository = DirectoryRepository::new(&directory).expect("opens");
        let (mut store, recovery) = ProjectStore::open(repository).expect("recovery works");
        if recovery != Recovery::Clean {
            interrupted += 1;
        }
        let loaded = store
            .load()
            .unwrap_or_else(|error| panic!("round {round}: {error}"));
        assert_whole(&loaded.project, "the project");
        if loaded.autosave != Autosave::None {
            autosaves += 1;
            let restored = store
                .restore_autosave()
                .unwrap_or_else(|error| panic!("round {round}: {error}"));
            assert_whole(&restored, "the autosave");
        }
    }
    // If no kill ever landed inside a save, this run proved nothing.
    assert!(interrupted > 0, "every kill landed between saves");
    assert!(
        autosaves > 0,
        "none of the kills left an autosave to check, with {interrupted} interrupted"
    );
    println!("30 kills: {interrupted} interrupted a save, {autosaves} left an autosave, all whole");
}
