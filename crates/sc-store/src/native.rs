//! A project as a directory on disk.
use crate::repository::{FilePath, Repository, RepositoryError};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};

/// A project directory. Each file reaches the disk before its call returns, and links are refused, so a
/// project cannot write outside its own directory.
#[derive(Clone, Debug)]
pub struct DirectoryRepository {
    root: PathBuf,
}

fn failed(path: impl ToString, error: &io::Error) -> RepositoryError {
    RepositoryError {
        path: path.to_string(),
        message: error.to_string(),
    }
}

// A rename or create is only durable once its directory reaches the disk. Windows has no such call.
#[cfg(unix)]
fn sync_directory(path: &Path) -> io::Result<()> {
    File::open(path)?.sync_all()
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> io::Result<()> {
    Ok(())
}

impl DirectoryRepository {
    /// Use `root` as a project directory, creating it if needed.
    ///
    /// # Errors
    /// The directory could not be created.
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, RepositoryError> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(|error| failed(root.display(), &error))?;
        Ok(Self { root })
    }

    fn on_disk(&self, path: &FilePath) -> PathBuf {
        self.root.join(path.as_str())
    }

    // Every existing component must be a real directory, so a planted link cannot redirect a write.
    // `None` means a parent is missing and `create` is off, so the file cannot exist either.
    fn checked_parent(
        &self,
        path: &FilePath,
        create: bool,
    ) -> Result<Option<PathBuf>, RepositoryError> {
        let mut current = self.root.clone();
        let mut parts: Vec<&str> = path.as_str().split('/').collect();
        parts.pop();
        for part in parts {
            current.push(part);
            match fs::symlink_metadata(&current) {
                Ok(metadata) if metadata.file_type().is_dir() => {}
                Ok(_) => {
                    return Err(RepositoryError {
                        path: path.to_string(),
                        message: format!("{part} is not a directory"),
                    });
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => {
                    if !create {
                        return Ok(None);
                    }
                    fs::create_dir(&current).map_err(|error| failed(path, &error))?;
                    if let Some(parent) = current.parent() {
                        sync_directory(parent).map_err(|error| failed(path, &error))?;
                    }
                }
                Err(error) => return Err(failed(path, &error)),
            }
        }
        Ok(Some(current))
    }

    fn refuse_link(&self, path: &FilePath) -> Result<(), RepositoryError> {
        match fs::symlink_metadata(self.on_disk(path)) {
            Ok(metadata) if !metadata.file_type().is_file() => Err(RepositoryError {
                path: path.to_string(),
                message: "this is not a regular file".to_owned(),
            }),
            _ => Ok(()),
        }
    }

    fn prune_empty_parents(&self, path: &FilePath) {
        let mut current = self.on_disk(path);
        while current.pop() && current != self.root {
            if fs::remove_dir(&current).is_err() {
                break;
            }
        }
    }

    fn walk(&self, directory: &Path, found: &mut Vec<FilePath>) -> Result<(), RepositoryError> {
        let entries =
            fs::read_dir(directory).map_err(|error| failed(directory.display(), &error))?;
        for entry in entries {
            let entry = entry.map_err(|error| failed(directory.display(), &error))?;
            let path = entry.path();
            let kind = entry
                .file_type()
                .map_err(|error| failed(path.display(), &error))?;
            if kind.is_dir() {
                self.walk(&path, found)?;
            } else if kind.is_file() {
                let relative = path.strip_prefix(&self.root).unwrap_or(&path);
                let text = relative.to_string_lossy().replace('\\', "/");
                let file = FilePath::new(&text).map_err(|error| RepositoryError {
                    path: text.clone(),
                    message: error.reason.to_owned(),
                })?;
                found.push(file);
            } else {
                return Err(RepositoryError {
                    path: path.display().to_string(),
                    message: "a project may only hold files and directories".to_owned(),
                });
            }
        }
        Ok(())
    }
}

impl Repository for DirectoryRepository {
    fn read(&self, path: &FilePath) -> Result<Option<Vec<u8>>, RepositoryError> {
        if self.checked_parent(path, false)?.is_none() {
            return Ok(None);
        }
        self.refuse_link(path)?;
        match fs::read(self.on_disk(path)) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(failed(path, &error)),
        }
    }

    fn write(&mut self, path: &FilePath, bytes: &[u8]) -> Result<(), RepositoryError> {
        let parent = self
            .checked_parent(path, true)?
            .unwrap_or_else(|| self.root.clone());
        self.refuse_link(path)?;
        let target = self.on_disk(path);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&target)
            .map_err(|error| failed(path, &error))?;
        file.write_all(bytes)
            .map_err(|error| failed(path, &error))?;
        file.sync_all().map_err(|error| failed(path, &error))?;
        sync_directory(&parent).map_err(|error| failed(path, &error))
    }

    fn rename(&mut self, from: &FilePath, to: &FilePath) -> Result<(), RepositoryError> {
        let from_parent = self
            .checked_parent(from, false)?
            .ok_or_else(|| RepositoryError {
                path: from.to_string(),
                message: "the file does not exist".to_owned(),
            })?;
        let to_parent = self
            .checked_parent(to, true)?
            .unwrap_or_else(|| self.root.clone());
        self.refuse_link(from)?;
        self.refuse_link(to)?;
        fs::rename(self.on_disk(from), self.on_disk(to)).map_err(|error| failed(from, &error))?;
        sync_directory(&to_parent).map_err(|error| failed(to, &error))?;
        if from_parent != to_parent {
            sync_directory(&from_parent).map_err(|error| failed(from, &error))?;
        }
        self.prune_empty_parents(from);
        Ok(())
    }

    fn remove(&mut self, path: &FilePath) -> Result<(), RepositoryError> {
        let Some(parent) = self.checked_parent(path, false)? else {
            return Ok(());
        };
        self.refuse_link(path)?;
        match fs::remove_file(self.on_disk(path)) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(failed(path, &error)),
        }
        sync_directory(&parent).map_err(|error| failed(path, &error))?;
        self.prune_empty_parents(path);
        Ok(())
    }

    fn list(&self) -> Result<Vec<FilePath>, RepositoryError> {
        let mut found = Vec::new();
        self.walk(&self.root, &mut found)?;
        found.sort();
        Ok(found)
    }
}
