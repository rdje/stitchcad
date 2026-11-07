# Project store

A project is a directory of text files you can read, diff and keep in version control. This store keeps those
files safe: if the machine crashes in the middle of a save, the project holds exactly its old files or exactly
its new ones, never a mix. It is the `sc-store` crate.

**What works today.** You can save a project as one commit and reopen it after a crash, on a real directory or in
memory. Older projects migrate all or nothing, newer ones are refused untouched, unknown fields and files survive,
and autosave keeps unsaved work beside the project. In a browser, the project is `MemoryRepository`'s one image, saved
to IndexedDB in one transaction, as [the browser run](browser-capabilities.md) shows. A drafted garment's project holds what it is made
from, below. **What does not exist yet:** the parts of a `Design` no command edits yet, such as the sewing graph, which arrive with
G3; undo across a reopen; and the SQLite catalog of roadmap §4.5. Schema 1 is the first schema, so no real migration exists yet, and the
migration tests use their own.

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
| `.sc-store` | The directory inside a project that holds a commit in progress, the waiting autosave and migration backups |
| `project.json` | A project's manifest: its `schema` field, plus any other fields |
| `schema` | The manifest field holding the schema a project is written in |
| `Project` | A project in memory: manifest fields and files, including any this version does not understand |
| `ProjectStore` | A project's manifest and files on any backend, with migrations and autosave |
| `ProjectError` | Why a project could not be loaded or saved. Nothing is half-written |
| `Schema` | The schema a store reads and writes, and how older projects reach it |
| `Migration` | One step from a schema to the next |
| `Loaded` | A loaded project, the schema it was migrated from, and whether an autosave is waiting |
| `Autosave` | Whether an autosave is waiting when a project loads |
| `Current` | An autosave made since the project's last save |
| `Stale` | An autosave made from a different saved state, so restoring it would lose that state's changes |

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

## A project

A project is its manifest, `project.json`, plus every other file in the directory. Loading then saving keeps what
this version does not understand: an unknown field, or a file written by another tool.

```rust
use sc_store::{Commit, FilePath, MemoryRepository, Project, ProjectStore, Repository, Value};

let path = |text: &str| FilePath::new(text).expect("a valid path");
let mut project = Project::new();
project.set_field("name", Value::Text("skirt".to_owned()));
project.set_file(path("recipes/skirt.txt"), "waist = 74 cm\n").expect("a project file");

let (mut store, _) = ProjectStore::open(MemoryRepository::new()).expect("opens");
assert_eq!(store.save(&project).expect("saves"), Commit::Applied(2));

// Saving the same state again writes nothing, and reopening gives an equal project.
assert_eq!(store.save(&project).expect("saves"), Commit::Unchanged);
assert_eq!(store.load().expect("loads").project, project);

let mut disk = store.into_repository();
assert_eq!(disk.read(&path("project.json")).expect("reads"),
    Some(b"{\n  \"name\": \"skirt\",\n  \"schema\": 1\n}\n".to_vec()));

// Another tool adds a field and a file. Both survive this version's load and save.
disk.write(&path("project.json"), br#"{"name": "skirt", "other_tool": [1], "schema": 1}"#).expect("writes");
disk.write(&path("extensions/other.bin"), &[0, 255]).expect("writes");
let (mut store, _) = ProjectStore::open(disk).expect("opens");
let mut project = store.load().expect("loads").project;
project.set_field("name", Value::Text("renamed".to_owned()));
store.save(&project).expect("saves");
let disk = store.into_repository();
assert!(String::from_utf8_lossy(&disk.read(&path("project.json")).expect("reads").expect("exists")).contains("other_tool"));
assert_eq!(disk.read(&path("extensions/other.bin")).expect("reads"), Some(vec![0, 255]));
```

## What a drafted garment saves

| File | What it holds |
| --- | --- |
| `inputs.json` | Every measurement and allowance, in whole micrometres |
| `recipe.formula` | The construction recipe, in its own text |
| `draft.json` | How to draw each piece, by the names of those lengths |

**The drawing itself is not saved,** because derived data is never authoritative: reopening draws it again, and it comes out to the same bytes. So two designs whose pieces have identical contours and different recipes stay two designs. One whose width follows an input moves when that input changes, and one that states the same number does not. A file a factory receives cannot show that difference, and the project can.

Saving what was loaded writes nothing and leaves every byte, in any locale and time zone. Revisions carry on across a reopen, and none is issued twice. Undo history does not cross one, and an `undo` right after a reopen says there is nothing to undo.

```text
$ cargo test -p sc-cli --test save_load
```

Beyond that, this runs the reference skirt through the real program: save, load and save to the same bytes on a real directory under another locale; a reopened project drawing the frozen goldens; and the same edit before and after a reopen giving the same pieces.

The [offset chapter](offsets.md) holds the transforms, and a written length means the same in any unit: rewriting the skirt's `5 mm` as `0.5 cm`, or `25.4 mm` as `1 in`, changes no byte.

## Schemas and migrations

A project written by a newer StitchCAD is refused and left untouched, because this version cannot know what it
would lose. An older one migrates when it loads. The steps run in memory, and the result is saved in one commit
that also keeps the originals under `.sc-store/backups`. So a failed step changes nothing, and a crash leaves
the old project or the migrated one.

```rust
use sc_store::{FilePath, MemoryRepository, Migration, Project, ProjectStore, Repository, Schema, Value};

let path = |text: &str| FilePath::new(text).expect("a valid path");
let mut project = Project::new();
project.set_field("name", Value::Text("skirt".to_owned()));
let (mut store, _) = ProjectStore::open(MemoryRepository::new()).expect("opens");
store.save(&project).expect("saves");
let disk = store.into_repository();

// Schema 2 renames a field.
fn rename(project: &mut Project) -> Result<(), String> {
    let name = project.remove_field("name").ok_or("no name to rename")?;
    project.set_field("title", name);
    Ok(())
}
let schema = Schema::new(2, vec![Migration { from: 1, apply: rename }]);
let (mut store, _) = ProjectStore::open_with(disk.clone(), schema).expect("opens");
let loaded = store.load().expect("migrates");
assert_eq!(loaded.migrated_from, Some(1));
assert_eq!(loaded.project.field("title"), Some(&Value::Text("skirt".to_owned())));
let migrated = store.into_repository();
assert!(migrated.read(&path(".sc-store/backups/schema-1/project.json")).expect("reads").is_some());

// A step that fails names itself, and the project stays exactly as it was.
fn refuse(_project: &mut Project) -> Result<(), String> {
    Err("the recipe uses a removed operation".to_owned())
}
let schema = Schema::new(2, vec![Migration { from: 1, apply: refuse }]);
let (mut store, _) = ProjectStore::open_with(disk.clone(), schema).expect("opens");
let error = store.load().expect_err("the step fails");
assert_eq!(error.to_string(),
    "migrating from schema 1 to 2 failed: the recipe uses a removed operation. Nothing was changed");
assert_eq!(store.into_repository(), disk);

// This version reads schema 1, so the migrated project is too new for it, and stays untouched.
let (mut store, _) = ProjectStore::open(migrated.clone()).expect("opens");
let error = store.load().expect_err("too new");
assert_eq!(error.to_string(),
    "a newer StitchCAD wrote this project (schema 2); this version reads up to schema 1. Nothing was changed");
assert_eq!(store.into_repository(), migrated);
```

## Autosave

An autosave keeps unsaved work beside the project and leaves the project itself as it was. It is written by the
same kind of commit, so it is always whole, and a later autosave replaces it whole. A save clears it in the same
commit. Nothing here reads a clock: an autosave records which saved state it came from, and `Stale` means the
project has since changed some other way.

```rust
use sc_store::{Autosave, FilePath, MemoryRepository, Project, ProjectStore};

let path = |text: &str| FilePath::new(text).expect("a valid path");
let saved = Project::new();
let (mut store, _) = ProjectStore::open(MemoryRepository::new()).expect("opens");
store.save(&saved).expect("saves");

let mut work = saved.clone();
work.set_file(path("notes.txt"), "unsaved work\n").expect("a project file");
store.autosave(&work).expect("autosaves");

// After a crash, the project is the last save, and the work is offered back.
let (mut store, _) = ProjectStore::open(store.into_repository()).expect("opens");
let loaded = store.load().expect("loads");
assert_eq!(loaded.project, saved);
assert_eq!(loaded.autosave, Autosave::Current);
let restored = store.restore_autosave().expect("restores");
assert_eq!(restored, work);

store.save(&restored).expect("saves");
assert_eq!(store.load().expect("loads").autosave, Autosave::None);
```

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
- do the same to a process saving and autosaving a whole project, then require the project and any autosave to reopen
  whole, and fail if no kill landed inside a save or left an autosave to check;
- crash every step of a migration, a save and an autosave;
- fail a migration step and require byte-identical files.
- round-trip 5,000 seeded random values through the text form, requiring a stable form.
