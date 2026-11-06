//! A repository that stops at a chosen step, the way a crash would.
#![allow(dead_code, clippy::expect_used)]
use sc_store::{FilePath, Repository, RepositoryError};
use std::cell::Cell;
use std::collections::BTreeMap;

/// Allows `budget` changing calls, then crashes: the next write is torn, and every call after it fails.
#[derive(Debug)]
pub struct Crashing<R> {
    pub inner: R,
    budget: Cell<usize>,
    crashed: Cell<bool>,
    pub steps: Cell<usize>,
    transient: bool,
}

impl<R> Crashing<R> {
    /// Crashes for good once the budget runs out.
    pub fn new(inner: R, budget: usize) -> Self {
        Self {
            inner,
            budget: Cell::new(budget),
            crashed: Cell::new(false),
            steps: Cell::new(0),
            transient: false,
        }
    }

    /// Fails one call once the budget runs out, then works again, like a full disk that clears.
    pub fn transient(inner: R, budget: usize) -> Self {
        Self {
            transient: true,
            ..Self::new(inner, budget)
        }
    }

    pub fn crashed(&self) -> bool {
        self.crashed.get()
    }

    fn dead(&self, path: &str) -> RepositoryError {
        RepositoryError {
            path: path.to_owned(),
            message: "the process has crashed".to_owned(),
        }
    }

    // True when this changing call may go ahead.
    fn spend(&self) -> bool {
        if self.crashed.get() {
            return false;
        }
        if self.budget.get() == 0 {
            if self.transient {
                self.budget.set(usize::MAX);
            } else {
                self.crashed.set(true);
            }
            return false;
        }
        self.budget.set(self.budget.get() - 1);
        self.steps.set(self.steps.get() + 1);
        true
    }
}

impl<R: Repository> Repository for Crashing<R> {
    fn read(&self, path: &FilePath) -> Result<Option<Vec<u8>>, RepositoryError> {
        if self.crashed.get() {
            return Err(self.dead(path.as_str()));
        }
        self.inner.read(path)
    }

    fn write(&mut self, path: &FilePath, bytes: &[u8]) -> Result<(), RepositoryError> {
        if self.spend() {
            return self.inner.write(path, bytes);
        }
        if !self.transient {
            // A crash mid-write leaves part of the file behind.
            let half = bytes.get(..bytes.len() / 2).unwrap_or(&[]);
            self.inner.write(path, half)?;
        }
        Err(self.dead(path.as_str()))
    }

    fn rename(&mut self, from: &FilePath, to: &FilePath) -> Result<(), RepositoryError> {
        if self.spend() {
            return self.inner.rename(from, to);
        }
        Err(self.dead(from.as_str()))
    }

    fn remove(&mut self, path: &FilePath) -> Result<(), RepositoryError> {
        if self.spend() {
            return self.inner.remove(path);
        }
        Err(self.dead(path.as_str()))
    }

    fn list(&self) -> Result<Vec<FilePath>, RepositoryError> {
        if self.crashed.get() {
            return Err(self.dead("(list)"));
        }
        self.inner.list()
    }
}

pub fn path(text: &str) -> FilePath {
    FilePath::new(text).expect("a test path is valid")
}

/// Every file a repository holds, the store's own included.
pub fn everything<R: Repository>(repository: &R) -> BTreeMap<String, Vec<u8>> {
    repository
        .list()
        .expect("the list works")
        .into_iter()
        .map(|file| {
            let bytes = repository
                .read(&file)
                .expect("the read works")
                .expect("a listed file exists");
            (file.to_string(), bytes)
        })
        .collect()
}
