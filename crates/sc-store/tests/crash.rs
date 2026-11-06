//! Crash a commit at every step, then crash its recovery at every step. The project always ends up
//! holding exactly its old files or exactly its new ones.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
mod common;

use common::{everything, path, Crashing};
use sc_store::{ChangeSet, Commit, MemoryRepository, Recovery, Repository, Store, StoreError};
use std::collections::BTreeMap;

fn before() -> MemoryRepository {
    let mut repository = MemoryRepository::new();
    for (file, bytes) in [
        ("a.txt", "a1"),
        ("dir/b.txt", "b1"),
        ("gone.txt", "gone"),
        ("same.txt", "same"),
    ] {
        repository
            .write(&path(file), bytes.as_bytes())
            .expect("memory writes");
    }
    repository
}

fn changes() -> ChangeSet {
    let mut set = ChangeSet::new();
    set.write(path("a.txt"), "a2");
    set.write(path("dir/b.txt"), "b2");
    set.write(path("dir/deep/c.txt"), "c2");
    set.write(path("same.txt"), "same");
    set.remove(path("gone.txt"));
    set
}

fn state(pairs: &[(&str, &str)]) -> BTreeMap<String, Vec<u8>> {
    pairs
        .iter()
        .map(|(file, bytes)| ((*file).to_owned(), bytes.as_bytes().to_vec()))
        .collect()
}

fn old() -> BTreeMap<String, Vec<u8>> {
    state(&[
        ("a.txt", "a1"),
        ("dir/b.txt", "b1"),
        ("gone.txt", "gone"),
        ("same.txt", "same"),
    ])
}

fn new() -> BTreeMap<String, Vec<u8>> {
    state(&[
        ("a.txt", "a2"),
        ("dir/b.txt", "b2"),
        ("dir/deep/c.txt", "c2"),
        ("same.txt", "same"),
    ])
}

/// Run one commit that crashes after `budget` changing calls, and return what the disk then holds.
fn crash_at(budget: usize) -> (MemoryRepository, bool) {
    let (mut store, recovery) = Store::open(Crashing::new(before(), budget))
        .expect("opening a clean project changes nothing");
    assert_eq!(recovery, Recovery::Clean);
    let result = store.commit(&changes());
    let crashing = store.into_repository();
    let crashed = crashing.crashed();
    assert_eq!(result.is_err(), crashed, "budget {budget}: {result:?}");
    (crashing.inner, crashed)
}

#[test]
fn every_crash_point_leaves_old_or_new() {
    let (mut saw_old, mut saw_new, mut budget) = (0, 0, 0);
    loop {
        let (disk, crashed) = crash_at(budget);
        if !crashed {
            assert_eq!(everything(&disk), new(), "a finished commit");
            break;
        }
        let committed = disk
            .read(&path(".sc-store/journal"))
            .expect("memory reads")
            .is_some();
        let (mut store, recovery) = Store::open(disk).expect("recovery works");
        let expected = if committed { new() } else { old() };
        let repository = {
            assert!(store.read(&path("a.txt")).expect("reads").is_some());
            store.into_repository()
        };
        assert_eq!(
            everything(&repository),
            expected,
            "budget {budget}, nothing of the store's own stays"
        );
        if committed {
            saw_new += 1;
            assert!(
                matches!(recovery, Recovery::RolledForward(4)),
                "budget {budget}: {recovery:?}"
            );
        } else {
            saw_old += 1;
        }
        budget += 1;
    }
    // A run that only ever saw one side would prove little.
    assert!(saw_old >= 3 && saw_new >= 3, "old {saw_old}, new {saw_new}");
    println!("{budget} crash points: {saw_old} rolled back, {saw_new} rolled forward");
}

#[test]
fn recovery_itself_may_crash_at_every_step() {
    let mut pairs = 0;
    let mut budget = 0;
    loop {
        let (disk, crashed) = crash_at(budget);
        if !crashed {
            break;
        }
        let committed = disk
            .read(&path(".sc-store/journal"))
            .expect("memory reads")
            .is_some();
        let expected = if committed { new() } else { old() };
        let mut second = 0;
        loop {
            match Store::open(Crashing::new(disk.clone(), second)) {
                Ok(_) => break,
                Err(_) => {
                    // Build the same crashed disk again, then crash recovery at this step, and recover for real.
                    let mut crashing = Crashing::new(disk.clone(), second);
                    assert!(Store::open(&mut crashing).is_err());
                    let (store, _) =
                        Store::open(crashing.inner).expect("a later open finishes the work");
                    assert_eq!(
                        everything(&store.into_repository()),
                        expected,
                        "budgets {budget},{second}"
                    );
                    pairs += 1;
                    second += 1;
                }
            }
        }
        budget += 1;
    }
    assert!(pairs >= 10, "only {pairs} double crashes");
    println!("{pairs} crashes during recovery");
}

#[test]
fn a_failed_call_before_the_commit_point_changes_nothing_and_the_store_still_works() {
    let (mut store, _) = Store::open(Crashing::transient(before(), 1)).expect("opens");
    assert!(store.commit(&changes()).is_err());
    assert_eq!(
        store.read(&path("a.txt")).expect("reads"),
        Some(b"a1".to_vec())
    );
    assert_eq!(
        store.commit(&changes()).expect("a later commit works"),
        Commit::Applied(4)
    );
    assert_eq!(everything(&store.into_repository().inner), new());
}

#[test]
fn a_failed_call_after_the_commit_point_finishes_on_the_next_call() {
    // Three pending writes, the journal, and its move: the next call is the first one applying the commit.
    let (mut store, _) = Store::open(Crashing::transient(before(), 5)).expect("opens");
    assert!(store.commit(&changes()).is_err());
    assert_eq!(
        store.read(&path("a.txt")).expect("reads"),
        Some(b"a2".to_vec())
    );
    assert_eq!(everything(&store.into_repository().inner), new());
}

#[test]
fn committing_the_same_files_writes_nothing() {
    let mut set = ChangeSet::new();
    set.write(path("a.txt"), "a1");
    set.remove(path("not-there.txt"));
    let (mut store, _) = Store::open(Crashing::new(before(), 0)).expect("opens");
    assert_eq!(
        store.commit(&set).expect("nothing to write"),
        Commit::Unchanged
    );
    assert_eq!(everything(&store.into_repository().inner), old());
}

#[test]
fn the_stores_own_directory_is_refused() {
    let mut set = ChangeSet::new();
    set.write(path(".sc-store/journal"), "forged");
    let (mut store, _) = Store::open(before()).expect("opens");
    assert!(matches!(
        store.commit(&set),
        Err(StoreError::ReservedPath(_))
    ));
    assert_eq!(everything(&store.into_repository()), old());
}

#[test]
fn project_listing_leaves_out_the_stores_files() {
    let (mut disk, crashed) = crash_at(2);
    assert!(crashed);
    disk.write(&path("extra.txt"), b"x").expect("memory writes");
    let (mut store, recovery) = Store::open(disk).expect("opens");
    assert_eq!(recovery, Recovery::RolledBack);
    let names: Vec<String> = store
        .files()
        .expect("lists")
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(
        names,
        ["a.txt", "dir/b.txt", "extra.txt", "gone.txt", "same.txt"]
    );
}

/// Stop just after the commit point, so a journal and its pending files are on disk.
fn after_commit_point() -> MemoryRepository {
    let (disk, crashed) = crash_at(5);
    assert!(crashed);
    assert!(disk
        .read(&path(".sc-store/journal"))
        .expect("reads")
        .is_some());
    disk
}

#[test]
fn a_damaged_pending_file_stops_recovery_and_says_which() {
    let mut disk = after_commit_point();
    disk.write(&path(".sc-store/pending/dir/b.txt"), b"flipped")
        .expect("memory writes");
    let before_open = everything(&disk);
    let Err(error) = Store::open(disk.clone()) else {
        panic!("a damaged file must stop recovery")
    };
    assert!(
        matches!(&error, StoreError::Damaged { path, .. } if path == ".sc-store/pending/dir/b.txt"),
        "{error}"
    );
    assert_eq!(everything(&disk), before_open);
}

#[test]
fn a_damaged_journal_stops_recovery() {
    for damage in [
        "not a journal",
        "{}",
        "{\"format\":1,\"entries\":[],\"sha256\":\"wrong\"}",
        "{\"format\":9}",
    ] {
        let mut disk = after_commit_point();
        disk.write(&path(".sc-store/journal"), damage.as_bytes())
            .expect("memory writes");
        let Err(error) = Store::open(disk) else {
            panic!("{damage} must stop recovery")
        };
        assert!(
            matches!(error, StoreError::Damaged { .. }),
            "{damage}: {error}"
        );
    }
}

#[test]
fn a_missing_pending_file_with_wrong_live_bytes_is_damage_and_not_a_silent_loss() {
    let mut disk = after_commit_point();
    disk.remove(&path(".sc-store/pending/dir/b.txt"))
        .expect("memory removes");
    let Err(error) = Store::open(disk) else {
        panic!("a lost file must stop recovery")
    };
    assert!(
        matches!(&error, StoreError::Damaged { path, .. } if path == "dir/b.txt"),
        "{error}"
    );
}
