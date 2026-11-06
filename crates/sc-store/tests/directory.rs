//! The same guarantees on a real directory, including a real process killed in the middle of a save.
#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
mod common;

use common::{everything, path, Crashing};
use sc_store::{ChangeSet, Commit, DirectoryRepository, Recovery, Repository, Store};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

const FILES: usize = 12;
const CHILD: &str = "SC_STORE_KILL_CHILD";

fn fresh(name: &str) -> PathBuf {
    let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("sc-store")
        .join(name);
    if directory.exists() {
        fs::remove_dir_all(&directory).expect("clears the last run");
    }
    directory
}

fn all_files(tag: u64) -> ChangeSet {
    let mut set = ChangeSet::new();
    for at in 0..FILES {
        let body = format!("tag {tag}\n{}\n", "x".repeat(2_000 + at));
        set.write(path(&format!("group{}/file{at}.txt", at % 3)), body);
    }
    set
}

fn the_one_tag(repository: &DirectoryRepository) -> String {
    let files = everything(repository);
    assert_eq!(files.len(), FILES, "{:?}", files.keys());
    let tags: Vec<String> = files
        .values()
        .map(|bytes| {
            String::from_utf8_lossy(bytes)
                .lines()
                .next()
                .unwrap_or("")
                .to_owned()
        })
        .collect();
    assert!(
        tags.iter().all(|tag| *tag == tags[0]),
        "files disagree: {tags:?}"
    );
    tags[0].clone()
}

#[test]
fn a_killed_process_leaves_old_or_new_on_a_real_disk() {
    if let Ok(directory) = std::env::var(CHILD) {
        // The child saves as fast as it can until the parent kills it.
        let repository = DirectoryRepository::new(directory.clone()).expect("the directory opens");
        let (mut store, _) = Store::open(repository).expect("recovery works");
        fs::write(PathBuf::from(&directory).join(".started"), b"").expect("marks the start");
        let mut tag = std::process::id().into();
        loop {
            tag += 1;
            store.commit(&all_files(tag)).expect("the save works");
        }
    }
    let directory = fresh("kill");
    let (mut store, _) =
        Store::open(DirectoryRepository::new(&directory).expect("opens")).expect("opens");
    store.commit(&all_files(0)).expect("the first save works");
    drop(store);

    let (mut interrupted, mut settled) = (0, 0);
    for round in 0..30u64 {
        let started = directory.join(".started");
        let _ = fs::remove_file(&started);
        let mut child = Command::new(std::env::current_exe().expect("this test binary"))
            .args([
                "a_killed_process_leaves_old_or_new_on_a_real_disk",
                "--exact",
                "--nocapture",
            ])
            .env(CHILD, &directory)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("the child starts");
        let begun = Instant::now();
        while !started.exists() {
            assert!(
                begun.elapsed() < Duration::from_secs(120),
                "the child never started"
            );
            std::thread::sleep(Duration::from_millis(1));
        }
        // The moment varies by round so it lands on different steps of a save.
        std::thread::sleep(Duration::from_millis(1 + (round * 7) % 23));
        child.kill().expect("the kill works");
        let _ = child.wait();
        let _ = fs::remove_file(&started);

        let repository = DirectoryRepository::new(&directory).expect("opens");
        let (store, recovery) = Store::open(repository).expect("recovery works");
        if recovery == Recovery::Clean {
            settled += 1;
        } else {
            interrupted += 1;
        }
        let tag = the_one_tag(&store.into_repository());
        assert!(tag.starts_with("tag "), "round {round}: {tag}");
    }
    // If no kill ever landed inside a save, this run proved nothing.
    assert!(interrupted > 0, "{settled} kills all landed between saves");
    println!("30 kills: {interrupted} interrupted a save, {settled} landed between saves");
}

#[test]
fn every_crash_point_holds_on_a_real_directory_too() {
    let mut budget = 0;
    loop {
        let directory = fresh(&format!("crash{budget}"));
        let (mut store, _) =
            Store::open(DirectoryRepository::new(&directory).expect("opens")).expect("opens");
        store.commit(&all_files(1)).expect("the first save works");
        drop(store);
        let (mut store, _) = Store::open(Crashing::new(
            DirectoryRepository::new(&directory).expect("opens"),
            budget,
        ))
        .expect("opens");
        let finished = store.commit(&all_files(2)).is_ok();
        drop(store);
        let (store, _) = Store::open(DirectoryRepository::new(&directory).expect("opens"))
            .expect("recovery works");
        let tag = the_one_tag(&store.into_repository());
        fs::remove_dir_all(&directory).expect("cleans up");
        if finished {
            assert_eq!(tag, "tag 2");
            break;
        }
        assert!(tag == "tag 1" || tag == "tag 2", "budget {budget}: {tag}");
        budget += 1;
    }
    assert!(budget > 2 * FILES, "only {budget} crash points");
}

#[test]
fn saving_the_same_files_twice_is_byte_identical_and_leaves_nothing_behind() {
    let directory = fresh("twice");
    let (mut store, _) =
        Store::open(DirectoryRepository::new(&directory).expect("opens")).expect("opens");
    assert_eq!(
        store.commit(&all_files(9)).expect("saves"),
        Commit::Applied(FILES)
    );
    let first = everything(&DirectoryRepository::new(&directory).expect("opens"));
    assert_eq!(
        store.commit(&all_files(9)).expect("saves"),
        Commit::Unchanged
    );
    assert_eq!(
        everything(&DirectoryRepository::new(&directory).expect("opens")),
        first
    );
    assert!(
        first.keys().all(|name| !name.starts_with(".sc-store")),
        "{:?}",
        first.keys()
    );
    assert!(
        !directory.join(".sc-store").exists(),
        "empty store directories are removed"
    );
}

#[cfg(unix)]
#[test]
fn links_are_refused_so_a_project_cannot_write_outside_itself() {
    let directory = fresh("links");
    let outside = fresh("links-outside");
    fs::create_dir_all(&outside).expect("makes the outside directory");
    let mut repository = DirectoryRepository::new(&directory).expect("opens");
    std::os::unix::fs::symlink(&outside, directory.join("escape")).expect("plants a link");
    let error = repository
        .write(&path("escape/stolen.txt"), b"x")
        .expect_err("a linked directory");
    assert!(error.message.contains("not a directory"), "{error}");
    assert!(!outside.join("stolen.txt").exists());

    std::os::unix::fs::symlink(outside.join("target.txt"), directory.join("link.txt"))
        .expect("plants a link");
    let error = repository
        .write(&path("link.txt"), b"x")
        .expect_err("a linked file");
    assert!(error.message.contains("not a regular file"), "{error}");
    assert!(!outside.join("target.txt").exists());
    let error = repository
        .read(&path("link.txt"))
        .expect_err("a linked file");
    assert!(error.message.contains("not a regular file"), "{error}");
    let error = repository.list().expect_err("a project holding a link");
    assert!(
        error.message.contains("only hold files and directories"),
        "{error}"
    );
}

#[test]
fn a_file_with_no_valid_name_is_refused_when_listing() {
    let directory = fresh("names");
    fs::create_dir_all(&directory).expect("makes the directory");
    fs::write(directory.join("what?.txt"), b"x").expect("plants a file");
    let repository = DirectoryRepository::new(&directory).expect("opens");
    let error = repository.list().expect_err("an invalid name");
    assert!(error.message.contains("character"), "{error}");
}

#[test]
fn reading_and_removing_missing_files_is_quiet() {
    let mut repository = DirectoryRepository::new(fresh("missing")).expect("opens");
    assert_eq!(
        repository.read(&path("no/such/file.txt")).expect("reads"),
        None
    );
    repository
        .remove(&path("no/such/file.txt"))
        .expect("removes");
    assert!(repository
        .rename(&path("no/such/file.txt"), &path("x.txt"))
        .is_err());
}
