//! Where project files live. A backend supplies a few durable primitives, and the store builds atomic
//! multi-file commits on top, so every backend gets the same crash behavior.
use core::fmt;
use std::collections::BTreeMap;

/// Longest accepted path, in bytes.
const MAX_PATH_BYTES: usize = 1024;

/// A file's place in a project: relative, `/`-separated, and valid on every supported system.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FilePath(String);

/// Why a path is refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathError {
    /// The refused path.
    pub path: String,
    /// What is wrong with it.
    pub reason: &'static str,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "project file path {:?}: {}", self.path, self.reason)
    }
}

impl std::error::Error for PathError {}

impl FilePath {
    /// Check a path.
    ///
    /// # Errors
    /// Refuses an empty or over-long path, an absolute one, an empty, `.` or `..` component, and any character
    /// or trailing space or dot that some supported system cannot store.
    pub fn new(path: &str) -> Result<Self, PathError> {
        let refuse = |reason| {
            Err(PathError {
                path: path.to_owned(),
                reason,
            })
        };
        if path.is_empty() {
            return refuse("it is empty");
        }
        if path.len() > MAX_PATH_BYTES {
            return refuse("it is longer than 1024 bytes");
        }
        for component in path.split('/') {
            if component.is_empty() {
                return refuse("a component is empty, or it starts or ends with /");
            }
            if component == "." || component == ".." {
                return refuse("it may not contain . or ..");
            }
            if component.ends_with('.') || component.ends_with(' ') {
                return refuse("a component ends with a dot or a space");
            }
            let unsafe_character = |character: char| {
                u32::from(character) < 0x20 || "\\:*?\"<>|\u{7f}".contains(character)
            };
            if component.chars().any(unsafe_character) {
                return refuse("it holds a character some systems cannot store");
            }
        }
        Ok(Self(path.to_owned()))
    }

    /// The path as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A backend could not do what was asked. The message is the system's own.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepositoryError {
    /// The file involved.
    pub path: String,
    /// What the system said.
    pub message: String,
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for RepositoryError {}

/// The durable primitives a backend supplies.
///
/// Implementations must keep these promises, because the store's crash behavior rests on them.
pub trait Repository {
    /// The file's bytes, or `None` when it does not exist.
    ///
    /// # Errors
    /// The file exists and could not be read.
    fn read(&self, path: &FilePath) -> Result<Option<Vec<u8>>, RepositoryError>;

    /// Create or replace a file. When this returns, the bytes survive a crash. If it is interrupted, the file
    /// may hold anything, so the store only calls this on paths nobody reads as project content.
    ///
    /// # Errors
    /// The write failed.
    fn write(&mut self, path: &FilePath, bytes: &[u8]) -> Result<(), RepositoryError>;

    /// Move a file over any existing one, atomically: after a crash, `to` holds its old or its new bytes,
    /// whole. When this returns, the move survives a crash.
    ///
    /// # Errors
    /// The move failed, and both files are as they were.
    fn rename(&mut self, from: &FilePath, to: &FilePath) -> Result<(), RepositoryError>;

    /// Delete a file. Deleting an absent file succeeds. When this returns, the deletion survives a crash.
    ///
    /// # Errors
    /// The file exists and could not be deleted.
    fn remove(&mut self, path: &FilePath) -> Result<(), RepositoryError>;

    /// Every file, sorted by path.
    ///
    /// # Errors
    /// The project could not be listed, or it holds something that is not a file or directory.
    fn list(&self) -> Result<Vec<FilePath>, RepositoryError>;
}

impl<R: Repository + ?Sized> Repository for &mut R {
    fn read(&self, path: &FilePath) -> Result<Option<Vec<u8>>, RepositoryError> {
        (**self).read(path)
    }

    fn write(&mut self, path: &FilePath, bytes: &[u8]) -> Result<(), RepositoryError> {
        (**self).write(path, bytes)
    }

    fn rename(&mut self, from: &FilePath, to: &FilePath) -> Result<(), RepositoryError> {
        (**self).rename(from, to)
    }

    fn remove(&mut self, path: &FilePath) -> Result<(), RepositoryError> {
        (**self).remove(path)
    }

    fn list(&self) -> Result<Vec<FilePath>, RepositoryError> {
        (**self).list()
    }
}

/// A project held in memory. It backs tests and the browser build, which loads and flushes it as one image.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MemoryRepository {
    files: BTreeMap<FilePath, Vec<u8>>,
}

impl MemoryRepository {
    /// An empty project.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// A project from a saved image, such as the browser's stored copy.
    #[must_use]
    pub fn from_image(files: BTreeMap<FilePath, Vec<u8>>) -> Self {
        Self { files }
    }

    /// Every file as it stands, to store as one image.
    #[must_use]
    pub const fn image(&self) -> &BTreeMap<FilePath, Vec<u8>> {
        &self.files
    }
}

impl Repository for MemoryRepository {
    fn read(&self, path: &FilePath) -> Result<Option<Vec<u8>>, RepositoryError> {
        Ok(self.files.get(path).cloned())
    }

    fn write(&mut self, path: &FilePath, bytes: &[u8]) -> Result<(), RepositoryError> {
        self.files.insert(path.clone(), bytes.to_vec());
        Ok(())
    }

    fn rename(&mut self, from: &FilePath, to: &FilePath) -> Result<(), RepositoryError> {
        let bytes = self.files.remove(from).ok_or_else(|| RepositoryError {
            path: from.to_string(),
            message: "the file does not exist".to_owned(),
        })?;
        self.files.insert(to.clone(), bytes);
        Ok(())
    }

    fn remove(&mut self, path: &FilePath) -> Result<(), RepositoryError> {
        self.files.remove(path);
        Ok(())
    }

    fn list(&self) -> Result<Vec<FilePath>, RepositoryError> {
        Ok(self.files.keys().cloned().collect())
    }
}
