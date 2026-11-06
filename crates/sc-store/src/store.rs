//! Crash-consistent multi-file commits over any repository: after a crash at any step, a project holds
//! exactly its old files or exactly its new ones.
//!
//! A commit writes each new file beside the project, then moves one journal into place. That move is the
//! commit point. The journal then lets any later open finish applying the changes, however many times that
//! itself is interrupted. Without a journal, leftovers are discarded and the project is untouched.
use crate::digest::sha256_hex;
use crate::repository::{FilePath, PathError, Repository, RepositoryError};
use crate::text::{parse, Value};
use core::fmt;
use std::collections::BTreeMap;

/// The directory a project's store owns. Project files may not use it.
const RESERVED: &str = ".sc-store";
const JOURNAL: &str = ".sc-store/journal";
const JOURNAL_NEW: &str = ".sc-store/journal.new";
const PENDING: &str = ".sc-store/pending/";
const JOURNAL_FORMAT: i64 = 1;

/// The files one commit changes, all or none of which take effect.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChangeSet {
    changes: BTreeMap<FilePath, Option<Vec<u8>>>,
}

impl ChangeSet {
    /// An empty change set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create or replace a file. The last change named for a path wins.
    pub fn write(&mut self, path: FilePath, bytes: impl Into<Vec<u8>>) {
        self.changes.insert(path, Some(bytes.into()));
    }

    /// Delete a file. Deleting an absent file changes nothing.
    pub fn remove(&mut self, path: FilePath) {
        self.changes.insert(path, None);
    }
}

/// What a commit did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commit {
    /// The files already held these bytes, so nothing was written.
    Unchanged,
    /// This many files changed.
    Applied(usize),
}

/// What opening a project found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recovery {
    /// The last commit finished.
    Clean,
    /// A commit stopped before its commit point. The project is as it was before that commit.
    RolledBack,
    /// A commit stopped after its commit point. It has now finished, changing this many files.
    RolledForward(usize),
}

/// Why the store could not do what was asked. Nothing is ever half-applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StoreError {
    /// A backend call failed.
    Repository(RepositoryError),
    /// A file name in a project or journal is not a valid path.
    Path(PathError),
    /// A file named in the journal is not what the journal recorded. Nothing further was changed.
    Damaged {
        /// The file.
        path: String,
        /// What was found.
        reason: String,
    },
    /// A commit named a file in the directory the store owns.
    ReservedPath(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Repository(error) => write!(f, "{error}"),
            Self::Path(error) => write!(f, "{error}"),
            Self::Damaged { path, reason } => {
                write!(
                    f,
                    "{path} is damaged, and nothing further was changed: {reason}"
                )
            }
            Self::ReservedPath(path) => {
                write!(
                    f,
                    "{path}: project files may not live in {RESERVED}, which the store owns"
                )
            }
        }
    }
}

impl std::error::Error for StoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Repository(error) => Some(error),
            Self::Path(error) => Some(error),
            Self::Damaged { .. } | Self::ReservedPath(_) => None,
        }
    }
}

impl From<RepositoryError> for StoreError {
    fn from(error: RepositoryError) -> Self {
        Self::Repository(error)
    }
}

impl From<PathError> for StoreError {
    fn from(error: PathError) -> Self {
        Self::Path(error)
    }
}

/// One change as the journal records it.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Entry {
    path: FilePath,
    // The digest of the new bytes, or `None` for a deletion.
    written: Option<String>,
}

fn internal(path: &str) -> Result<FilePath, StoreError> {
    Ok(FilePath::new(path)?)
}

fn pending_of(path: &FilePath) -> Result<FilePath, StoreError> {
    internal(&format!("{PENDING}{path}"))
}

fn entries_value(entries: &[Entry]) -> Value {
    let items = entries.iter().map(|entry| {
        let mut fields = BTreeMap::new();
        fields.insert("path".to_owned(), Value::Text(entry.path.to_string()));
        match &entry.written {
            Some(digest) => fields.insert("sha256".to_owned(), Value::Text(digest.clone())),
            None => fields.insert("removed".to_owned(), Value::Bool(true)),
        };
        Value::Object(fields)
    });
    Value::List(items.collect())
}

fn journal_text(entries: &[Entry]) -> String {
    let list = entries_value(entries);
    let mut fields = BTreeMap::new();
    fields.insert("format".to_owned(), Value::Int(JOURNAL_FORMAT));
    fields.insert(
        "sha256".to_owned(),
        Value::Text(sha256_hex(list.to_text().as_bytes())),
    );
    fields.insert("entries".to_owned(), list);
    Value::Object(fields).to_text()
}

fn read_journal(bytes: &[u8]) -> Result<Vec<Entry>, StoreError> {
    let damaged = |reason: &str| StoreError::Damaged {
        path: JOURNAL.to_owned(),
        reason: reason.to_owned(),
    };
    let text = core::str::from_utf8(bytes).map_err(|_| damaged("it is not text"))?;
    let value = parse(text).map_err(|error| damaged(&error.to_string()))?;
    let fields = value
        .as_object()
        .ok_or_else(|| damaged("it is not an object"))?;
    if fields.get("format").and_then(Value::as_int) != Some(JOURNAL_FORMAT) {
        return Err(damaged("it declares a format this version does not read"));
    }
    let list = fields
        .get("entries")
        .ok_or_else(|| damaged("it has no entries"))?;
    let recorded = fields.get("sha256").and_then(Value::as_text);
    if recorded != Some(sha256_hex(list.to_text().as_bytes()).as_str()) {
        return Err(damaged("its own digest does not match"));
    }
    let mut entries = Vec::new();
    for item in list
        .as_list()
        .ok_or_else(|| damaged("its entries are not a list"))?
    {
        let item = item
            .as_object()
            .ok_or_else(|| damaged("an entry is not an object"))?;
        let path = item
            .get("path")
            .and_then(Value::as_text)
            .ok_or_else(|| damaged("an entry has no path"))?;
        entries.push(Entry {
            path: FilePath::new(path)?,
            written: item
                .get("sha256")
                .and_then(Value::as_text)
                .map(str::to_owned),
        });
    }
    Ok(entries)
}

/// A project's files, reachable only through recovery and atomic commits.
///
/// One writer at a time: nothing here stops two processes from opening the same project together.
#[derive(Debug)]
pub struct Store<R> {
    repository: R,
    // A commit stopped after its commit point, so reads must finish it first.
    unfinished: bool,
}

impl<R: Repository> Store<R> {
    /// Open a project, finishing or discarding whatever a crash left behind.
    ///
    /// # Errors
    /// A backend call failed, or a file the journal needs is damaged. Opening again retries.
    pub fn open(mut repository: R) -> Result<(Self, Recovery), StoreError> {
        let recovery = recover(&mut repository)?;
        Ok((
            Self {
                repository,
                unfinished: false,
            },
            recovery,
        ))
    }

    /// Change the files together: after a crash at any step, all of the changes hold or none do.
    ///
    /// # Errors
    /// A file is in the store's own directory, or a backend call failed. If that failure came before the
    /// commit point, the project is unchanged. After it, the next call or the next open finishes the commit.
    pub fn commit(&mut self, changes: &ChangeSet) -> Result<Commit, StoreError> {
        for path in changes.changes.keys() {
            let text = path.as_str();
            if text == RESERVED || text.starts_with(".sc-store/") {
                return Err(StoreError::ReservedPath(path.to_string()));
            }
        }
        self.commit_any(changes)
    }

    // Also used for the store's own files, such as autosaves and backups.
    pub(crate) fn commit_any(&mut self, changes: &ChangeSet) -> Result<Commit, StoreError> {
        self.finish_unfinished()?;
        let mut entries = Vec::new();
        let mut writes = Vec::new();
        for (path, change) in &changes.changes {
            let current = self.repository.read(path)?;
            if current.as_deref() == change.as_deref() {
                continue;
            }
            entries.push(Entry {
                path: path.clone(),
                written: change.as_deref().map(sha256_hex),
            });
            if let Some(bytes) = change {
                writes.push((path, bytes));
            }
        }
        if entries.is_empty() {
            return Ok(Commit::Unchanged);
        }
        for (path, bytes) in writes {
            self.repository.write(&pending_of(path)?, bytes)?;
        }
        let journal_new = internal(JOURNAL_NEW)?;
        self.repository
            .write(&journal_new, journal_text(&entries).as_bytes())?;
        // The commit point.
        self.repository.rename(&journal_new, &internal(JOURNAL)?)?;
        self.unfinished = true;
        let changed = apply(&mut self.repository, &entries)?;
        self.unfinished = false;
        Ok(Commit::Applied(changed))
    }

    fn finish_unfinished(&mut self) -> Result<(), StoreError> {
        if self.unfinished {
            recover(&mut self.repository)?;
            self.unfinished = false;
        }
        Ok(())
    }

    /// A project file's bytes, or `None` when it does not exist.
    ///
    /// # Errors
    /// A backend call failed, or an interrupted commit could not finish.
    pub fn read(&mut self, path: &FilePath) -> Result<Option<Vec<u8>>, StoreError> {
        self.finish_unfinished()?;
        Ok(self.repository.read(path)?)
    }

    /// Every project file, sorted by path, leaving out the store's own directory.
    ///
    /// # Errors
    /// A backend call failed, or an interrupted commit could not finish.
    pub fn files(&mut self) -> Result<Vec<FilePath>, StoreError> {
        self.finish_unfinished()?;
        Ok(self
            .repository
            .list()?
            .into_iter()
            .filter(|path| !path.as_str().starts_with(".sc-store/"))
            .collect())
    }

    // Every file, the store's own included.
    pub(crate) fn all_files(&mut self) -> Result<Vec<FilePath>, StoreError> {
        self.finish_unfinished()?;
        Ok(self.repository.list()?)
    }

    /// The backend, to flush or close it.
    #[must_use]
    pub fn into_repository(self) -> R {
        self.repository
    }
}

// Moves each pending file into place and removes the journal. Safe to repeat from any point.
fn apply<R: Repository>(repository: &mut R, entries: &[Entry]) -> Result<usize, StoreError> {
    for entry in entries {
        match &entry.written {
            None => repository.remove(&entry.path)?,
            Some(digest) => {
                let pending = pending_of(&entry.path)?;
                if let Some(bytes) = repository.read(&pending)? {
                    if sha256_hex(&bytes) != *digest {
                        return Err(StoreError::Damaged {
                            path: pending.to_string(),
                            reason: "its bytes do not match the journal".to_owned(),
                        });
                    }
                    repository.rename(&pending, &entry.path)?;
                } else {
                    // An earlier run already moved it, so the live file must now hold those bytes.
                    let live = repository.read(&entry.path)?;
                    if live.map(|bytes| sha256_hex(&bytes)).as_ref() != Some(digest) {
                        return Err(StoreError::Damaged {
                            path: entry.path.to_string(),
                            reason: "neither it nor its pending copy holds the journal's bytes"
                                .to_owned(),
                        });
                    }
                }
            }
        }
    }
    repository.remove(&internal(JOURNAL)?)?;
    Ok(entries.len())
}

fn recover<R: Repository>(repository: &mut R) -> Result<Recovery, StoreError> {
    let journal = internal(JOURNAL)?;
    let mut recovery = Recovery::Clean;
    if let Some(bytes) = repository.read(&journal)? {
        let entries = read_journal(&bytes)?;
        recovery = Recovery::RolledForward(apply(repository, &entries)?);
    }
    // Anything left under pending, or a journal never moved into place, belongs to a commit that did not
    // reach its commit point.
    let journal_new = internal(JOURNAL_NEW)?;
    let leftovers: Vec<FilePath> = repository
        .list()?
        .into_iter()
        .filter(|path| path == &journal_new || path.as_str().starts_with(PENDING))
        .collect();
    if !leftovers.is_empty() && recovery == Recovery::Clean {
        recovery = Recovery::RolledBack;
    }
    for path in leftovers {
        repository.remove(&path)?;
    }
    Ok(recovery)
}
