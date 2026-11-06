#![doc = include_str!("../../../docs/book/src/spec/project-store.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod digest;
#[cfg(not(target_arch = "wasm32"))]
mod native;
mod project;
mod repository;
mod store;
mod text;

pub use digest::sha256_hex;
#[cfg(not(target_arch = "wasm32"))]
pub use native::DirectoryRepository;
pub use project::{
    Autosave, Loaded, Migration, Project, ProjectError, ProjectStore, Schema, SCHEMA,
};
pub use repository::{FilePath, MemoryRepository, PathError, Repository, RepositoryError};
pub use store::{ChangeSet, Commit, Recovery, Store, StoreError};
pub use text::{parse, TextError, Value};
