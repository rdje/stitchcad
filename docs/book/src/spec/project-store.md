# Project store

A project is a directory of text files you can read, diff and keep in version control. This store keeps those
files safe: if the machine crashes in the middle of a save, the project holds exactly its old files or exactly
its new ones, never a mix. It is the `sc-store` crate.

**What works today.** You can save several files as one commit and reopen after a crash, on a real directory or
in memory, and read and write the canonical text form. **What does not exist yet:** the project layer with its
schema version, migrations and autosave; saving a `Design`; IndexedDB in the browser; the SQLite catalog of
roadmap §4.5. The browser run, leaf `.12`, connects the browser side.

## The vocabulary

| Name | What it is |
| --- | --- |
| `sc-store` | The crate. Pure Rust, no dependencies |
| `Store` | A project's files, reachable only through recovery and atomic commits |
| `ChangeSet` | The files one commit changes, all or none of which take effect |
| `Commit` | What a commit did: nothing, because the bytes already matched, or changed this many files |
| `Unchanged` | The commit found the files already held these bytes, so it wrote nothing |
| `Applied` | The commit changed this many files |
| `Recovery` | What opening a project found after the last run |
| `Clean` | The last commit finished |
| `RolledBack` | A commit stopped before its commit point, so the project is as it was |
| `RolledForward` | A commit stopped after its commit point, and it has now finished |
| `StoreError` | Why the store refused, with the file involved |
| `Damaged` | A file the journal needs is not what the journal recorded. Nothing further is changed |
| `ReservedPath` | A commit named a file inside `.sc-store`, which the store owns |
| `Repository` | The durable primitives a backend supplies: read, write, rename, remove and list |
| `DirectoryRepository` | A project as a directory on disk. Each file reaches the disk before its call returns |
| `MemoryRepository` | A project held in memory, for tests and for the browser build |
| `FilePath` | A file's place in a project: relative, `/`-separated, and valid on every supported system |
| `Value` | One value in a canonical text file |
| `.sc-store` | The directory inside a project that holds a commit in progress. Empty after a finished commit |

## A save is all or nothing

```rust
use sc_store::{ChangeSet, Commit, FilePath, MemoryRepository, Recovery, Store};

let path = |text: &str| FilePath::new(text).expect("a valid path");
let (mut store, recovery) = Store::open(MemoryRepository::new()).expect("opens");
assert_eq!(recovery, Recovery::Clean);

let mut save = ChangeSet::new();
save.write(path("project.json"), "{}\n");
save.write(path("recipes/skirt.txt"), "waist = 74 cm\n");
assert_eq!(store.commit(&save).expect("saves"), Commit::Applied(2));

// The same files again write nothing, so saving twice leaves identical bytes.
assert_eq!(store.commit(&save).expect("saves"), Commit::Unchanged);
assert_eq!(store.read(&path("recipes/skirt.txt")).expect("reads"), Some(b"waist = 74 cm\n".to_vec()));
```

A commit writes each new file inside `.sc-store`, then moves one journal into place. **That move is the commit
point.** Before it, a crash discards the leftovers and the project is untouched. After it, the journal lets the
next open finish the commit, however many times that is itself interrupted.

| Where the crash lands | What the next open does | What you then have |
| --- | --- | --- |
| Before the commit point | Discards the unfinished files and reports `RolledBack` | The old files |
| After the commit point | Finishes the commit and reports `RolledForward` | The new files |
| While recovering from an earlier crash | Finishes the same recovery | The same files |

## Reopening after a crash

This uses a backend that stops partway through a save, the way a crash would. A real directory behaves the same,
and the tests kill a real process to show it.

```rust
use sc_store::{ChangeSet, FilePath, MemoryRepository, Recovery, Repository, RepositoryError, Store};

// Lets this many changing calls through, then fails every one after that.
struct Crashes { inner: MemoryRepository, left: usize }
impl Crashes {
    fn spend(&mut self, path: &FilePath) -> Result<(), RepositoryError> {
        if self.left == 0 {
            return Err(RepositoryError { path: path.to_string(), message: "crashed".to_owned() });
        }
        self.left -= 1;
        Ok(())
    }
}
impl Repository for Crashes {
    fn read(&self, path: &FilePath) -> Result<Option<Vec<u8>>, RepositoryError> { self.inner.read(path) }
    fn list(&self) -> Result<Vec<FilePath>, RepositoryError> { self.inner.list() }
    fn write(&mut self, path: &FilePath, bytes: &[u8]) -> Result<(), RepositoryError> {
        self.spend(path)?;
        self.inner.write(path, bytes)
    }
    fn rename(&mut self, from: &FilePath, to: &FilePath) -> Result<(), RepositoryError> {
        self.spend(from)?;
        self.inner.rename(from, to)
    }
    fn remove(&mut self, path: &FilePath) -> Result<(), RepositoryError> {
        self.spend(path)?;
        self.inner.remove(path)
    }
}

let path = |text: &str| FilePath::new(text).expect("a valid path");
let mut first = ChangeSet::new();
first.write(path("a.txt"), "old a");
first.write(path("b.txt"), "old b");
let (mut store, _) = Store::open(MemoryRepository::new()).expect("opens");
store.commit(&first).expect("saves");
let disk = store.into_repository();

let mut second = ChangeSet::new();
second.write(path("a.txt"), "new a");
second.write(path("b.txt"), "new b");

// Two pending writes, the journal, then its move: the crash comes just before the commit point.
let (mut store, _) = Store::open(Crashes { inner: disk.clone(), left: 3 }).expect("opens");
assert!(store.commit(&second).is_err());
let (mut reopened, recovery) = Store::open(store.into_repository().inner).expect("recovers");
assert_eq!(recovery, Recovery::RolledBack);
assert_eq!(reopened.read(&path("a.txt")).expect("reads"), Some(b"old a".to_vec()));
assert_eq!(reopened.read(&path("b.txt")).expect("reads"), Some(b"old b".to_vec()));

// One call later the journal is in place, so the commit has happened and recovery finishes it.
let (mut store, _) = Store::open(Crashes { inner: disk, left: 4 }).expect("opens");
assert!(store.commit(&second).is_err());
let (mut reopened, recovery) = Store::open(store.into_repository().inner).expect("recovers");
assert_eq!(recovery, Recovery::RolledForward(2));
assert_eq!(reopened.read(&path("a.txt")).expect("reads"), Some(b"new a".to_vec()));
assert_eq!(reopened.read(&path("b.txt")).expect("reads"), Some(b"new b".to_vec()));
```

## Canonical text

Files use a strict subset of JSON with exactly one way to write any value: keys in byte order, two-space
indentation, LF line ends and a final newline. So two saves of the same state are byte-identical, and a diff shows
only what changed.

```rust
use sc_store::parse;

let value = parse(r#"{"zipper": true, "name": "skirt", "darts": [1, 2]}"#).expect("valid");
assert_eq!(value.to_text(), "{\n  \"darts\": [\n    1,\n    2\n  ],\n  \"name\": \"skirt\",\n  \"zipper\": true\n}\n");

// Whitespace and key order may vary on the way in. The saved form cannot.
assert_eq!(parse(&value.to_text()).expect("valid"), value);

// A fraction is refused, because binary floating point cannot hold 0.1 exactly.
let error = parse("[0.1]").expect_err("a fraction");
assert_eq!(error.to_string(),
    "line 1, column 3: numbers here are whole; a fraction is stored as text in the format its field declares");
```

These are refused, each with its line and column:

- a fraction, an exponent, or a number that does not fit in 64 bits;
- a leading zero, or negative zero;
- a key repeated in one object;
- half a surrogate pair, or a raw control character in a string;
- lists and objects nested more than 64 levels;
- text after the value.

## What this does not protect against

- **Two writers.** Nothing stops two processes from opening the same project at once. Open each project once.
- **A damaged disk.** A digest check notices a damaged file during recovery and stops, naming it, and nothing
  further is changed. Damage to a file outside a commit in progress is not checked yet.
- **Power loss is not tested.** Each call asks the system to reach the disk before returning, and these tests kill
  a process. They cannot cut the power, so they rely on the system and its disk keeping that promise.

## How it is checked

Run `cargo test -p sc-store`. The tests:

- crash one save at every step, including a half-written file, and require exactly the old or the new files;
- crash recovery itself at every step of every one of those crashes;
- run the same crash points on a real directory;
- kill a real process 30 times during repeated saves to a real directory, and fail if no kill landed inside a save;
- round-trip 5,000 seeded random values through the text form, requiring a stable form.
