//! A project: its manifest with a schema version, its files, migrations, and autosave.
use crate::digest::sha256_hex;
use crate::repository::{FilePath, PathError, Repository};
use crate::store::{ChangeSet, Commit, Recovery, Store, StoreError};
use crate::text::{parse, TextError, Value};
use core::fmt;
use std::collections::BTreeMap;

/// The schema this version of StitchCAD writes.
pub const SCHEMA: i64 = 1;

const MANIFEST: &str = "project.json";
const SCHEMA_FIELD: &str = "schema";
const AUTOSAVE_FILES: &str = ".sc-store/autosave/files/";
const AUTOSAVE_BASE: &str = ".sc-store/autosave/base";
const BACKUPS: &str = ".sc-store/backups/";

/// One step from a schema to the next.
#[derive(Clone, Copy, Debug)]
pub struct Migration {
    /// The schema this step reads. It writes the next one.
    pub from: i64,
    /// Changes the project in memory. An error stops the migration with nothing written.
    pub apply: fn(&mut Project) -> Result<(), String>,
}

/// The schema a store reads and writes, and how older projects reach it.
#[derive(Clone, Debug)]
pub struct Schema {
    current: i64,
    migrations: Vec<Migration>,
}

impl Schema {
    /// This version of StitchCAD's schema.
    #[must_use]
    pub const fn latest() -> Self {
        Self {
            current: SCHEMA,
            migrations: Vec::new(),
        }
    }

    /// A schema with its migrations, each listed by the schema it reads.
    #[must_use]
    pub const fn new(current: i64, migrations: Vec<Migration>) -> Self {
        Self {
            current,
            migrations,
        }
    }

    /// The schema this store writes.
    #[must_use]
    pub const fn current(&self) -> i64 {
        self.current
    }
}

/// A project in memory: manifest fields and files, including any this version does not understand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Project {
    fields: BTreeMap<String, Value>,
    files: BTreeMap<FilePath, Vec<u8>>,
}

impl Project {
    /// An empty project at this version's schema.
    #[must_use]
    pub fn new() -> Self {
        Self::with_schema(SCHEMA)
    }

    /// An empty project at a stated schema.
    #[must_use]
    pub fn with_schema(schema: i64) -> Self {
        let mut fields = BTreeMap::new();
        fields.insert(SCHEMA_FIELD.to_owned(), Value::Int(schema));
        Self {
            fields,
            files: BTreeMap::new(),
        }
    }

    /// The schema this project is written in.
    #[must_use]
    pub fn schema(&self) -> i64 {
        self.fields
            .get(SCHEMA_FIELD)
            .and_then(Value::as_int)
            .unwrap_or(0)
    }

    /// A manifest field.
    #[must_use]
    pub fn field(&self, key: &str) -> Option<&Value> {
        self.fields.get(key)
    }

    /// Set a manifest field. The schema field belongs to the store, so setting it is ignored.
    pub fn set_field(&mut self, key: &str, value: Value) {
        if key != SCHEMA_FIELD {
            self.fields.insert(key.to_owned(), value);
        }
    }

    /// Remove a manifest field, which a migration may need. The schema field stays.
    pub fn remove_field(&mut self, key: &str) -> Option<Value> {
        if key == SCHEMA_FIELD {
            return None;
        }
        self.fields.remove(key)
    }

    /// A file's bytes.
    #[must_use]
    pub fn file(&self, path: &FilePath) -> Option<&[u8]> {
        self.files.get(path).map(Vec::as_slice)
    }

    /// Every file in path order.
    pub fn files(&self) -> impl Iterator<Item = (&FilePath, &[u8])> {
        self.files
            .iter()
            .map(|(path, bytes)| (path, bytes.as_slice()))
    }

    /// Create or replace a file.
    ///
    /// # Errors
    /// The manifest and the store's own directory cannot be written as files.
    pub fn set_file(
        &mut self,
        path: FilePath,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<(), ProjectError> {
        if path.as_str() == MANIFEST || path.as_str().starts_with(".sc-store/") {
            return Err(ProjectError::NotAFile(path.to_string()));
        }
        self.files.insert(path, bytes.into());
        Ok(())
    }

    /// Remove a file, returning its bytes.
    pub fn remove_file(&mut self, path: &FilePath) -> Option<Vec<u8>> {
        self.files.remove(path)
    }

    fn manifest_text(&self) -> String {
        Value::Object(self.fields.clone()).to_text()
    }

    /// A digest of everything saved, framed so two different projects cannot produce one input.
    #[must_use]
    pub fn digest(&self) -> String {
        let mut framed = Vec::new();
        let manifest = self.manifest_text();
        for (path, bytes) in core::iter::once((MANIFEST, manifest.as_bytes())).chain(
            self.files
                .iter()
                .map(|(path, bytes)| (path.as_str(), bytes.as_slice())),
        ) {
            framed.extend_from_slice(format!("{}:{}:", path.len(), bytes.len()).as_bytes());
            framed.extend_from_slice(path.as_bytes());
            framed.extend_from_slice(bytes);
        }
        sha256_hex(&framed)
    }

    fn set_schema(&mut self, schema: i64) {
        self.fields
            .insert(SCHEMA_FIELD.to_owned(), Value::Int(schema));
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

/// An autosave found when loading.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autosave {
    /// None is waiting.
    None,
    /// One is waiting, made since the project's last save.
    Current,
    /// One is waiting, made from a different saved state, so restoring it would lose that state's changes.
    Stale,
}

/// A loaded project and what loading found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loaded {
    /// The project.
    pub project: Project,
    /// The schema it was migrated from, if it was. The originals are kept under `.sc-store/backups`.
    pub migrated_from: Option<i64>,
    /// Whether an autosave is waiting.
    pub autosave: Autosave,
}

/// Why a project could not be loaded or saved. Nothing is half-written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectError {
    /// The store refused.
    Store(StoreError),
    /// A path is not valid.
    Path(PathError),
    /// No `project.json` here.
    NoProject,
    /// `project.json` is not canonical text, or not valid UTF-8.
    BadManifest(String),
    /// `project.json` has no whole-number schema of 1 or more.
    NoSchema,
    /// A newer StitchCAD wrote this project. This version will not touch it.
    NewerSchema {
        /// The project's schema.
        found: i64,
        /// The newest this version reads.
        supported: i64,
    },
    /// No migration reads this schema.
    NoMigration(i64),
    /// A migration step failed. The project on disk is unchanged.
    MigrationFailed {
        /// The schema that step reads.
        from: i64,
        /// What it said.
        reason: String,
    },
    /// A project built for a different schema than this store writes.
    WrongSchema {
        /// The project's schema.
        found: i64,
        /// The schema this store writes.
        current: i64,
    },
    /// The manifest and the store's own directory cannot be written as files.
    NotAFile(String),
    /// There is no autosave to restore.
    NoAutosave,
}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Store(error) => write!(f, "{error}"),
            Self::Path(error) => write!(f, "{error}"),
            Self::NoProject => write!(f, "there is no {MANIFEST} here, so this is not a project"),
            Self::BadManifest(reason) => write!(f, "{MANIFEST} cannot be read: {reason}"),
            Self::NoSchema => write!(f, "{MANIFEST} has no schema field holding a whole number of 1 or more"),
            Self::NewerSchema { found, supported } => write!(
                f,
                "a newer StitchCAD wrote this project (schema {found}); this version reads up to schema {supported}. Nothing was changed"
            ),
            Self::NoMigration(schema) => {
                write!(f, "no migration reads schema {schema}. Nothing was changed")
            }
            Self::MigrationFailed { from, reason } => write!(
                f,
                "migrating from schema {from} to {} failed: {reason}. Nothing was changed",
                from + 1
            ),
            Self::WrongSchema { found, current } => {
                write!(f, "this project is schema {found}, and this store writes schema {current}")
            }
            Self::NotAFile(path) => write!(f, "{path} belongs to the store and cannot be written as a project file"),
            Self::NoAutosave => write!(f, "there is no autosave to restore"),
        }
    }
}

impl std::error::Error for ProjectError {}

impl From<StoreError> for ProjectError {
    fn from(error: StoreError) -> Self {
        Self::Store(error)
    }
}

impl From<PathError> for ProjectError {
    fn from(error: PathError) -> Self {
        Self::Path(error)
    }
}

fn read_manifest(bytes: &[u8]) -> Result<BTreeMap<String, Value>, ProjectError> {
    let text = core::str::from_utf8(bytes)
        .map_err(|_| ProjectError::BadManifest("it is not UTF-8".to_owned()))?;
    let value =
        parse(text).map_err(|error: TextError| ProjectError::BadManifest(error.to_string()))?;
    match value {
        Value::Object(fields) => Ok(fields),
        _ => Err(ProjectError::BadManifest("it is not an object".to_owned())),
    }
}

/// Project files on any backend. Saves and migrations inherit the store's crash behavior.
#[derive(Debug)]
pub struct ProjectStore<R> {
    store: Store<R>,
    schema: Schema,
}

impl<R: Repository> ProjectStore<R> {
    /// Open a project directory at this version's schema, finishing whatever a crash left.
    ///
    /// # Errors
    /// See [`Store::open`].
    pub fn open(repository: R) -> Result<(Self, Recovery), ProjectError> {
        Self::open_with(repository, Schema::latest())
    }

    /// Open a project directory at a stated schema.
    ///
    /// # Errors
    /// See [`Store::open`].
    pub fn open_with(repository: R, schema: Schema) -> Result<(Self, Recovery), ProjectError> {
        let (store, recovery) = Store::open(repository)?;
        Ok((Self { store, schema }, recovery))
    }

    fn load_files(&mut self, prefix: &str) -> Result<Option<Project>, ProjectError> {
        let manifest_path = FilePath::new(&format!("{prefix}{MANIFEST}"))?;
        let Some(bytes) = self.store.read(&manifest_path)? else {
            return Ok(None);
        };
        let fields = read_manifest(&bytes)?;
        if fields
            .get(SCHEMA_FIELD)
            .and_then(Value::as_int)
            .is_none_or(|schema| schema < 1)
        {
            return Err(ProjectError::NoSchema);
        }
        let mut files = BTreeMap::new();
        for path in self.store.all_files()? {
            let Some(relative) = path.as_str().strip_prefix(prefix) else {
                continue;
            };
            if relative == MANIFEST || relative.starts_with(".sc-store/") {
                continue;
            }
            if let Some(bytes) = self.store.read(&path)? {
                files.insert(FilePath::new(relative)?, bytes);
            }
        }
        Ok(Some(Project { fields, files }))
    }

    fn migrate(&self, project: &mut Project) -> Result<Option<i64>, ProjectError> {
        let found = project.schema();
        if found > self.schema.current {
            return Err(ProjectError::NewerSchema {
                found,
                supported: self.schema.current,
            });
        }
        if found == self.schema.current {
            return Ok(None);
        }
        for from in found..self.schema.current {
            let step = self
                .schema
                .migrations
                .iter()
                .find(|migration| migration.from == from)
                .ok_or(ProjectError::NoMigration(from))?;
            (step.apply)(project)
                .map_err(|reason| ProjectError::MigrationFailed { from, reason })?;
            project.set_schema(from + 1);
        }
        Ok(Some(found))
    }

    /// Load the project, bringing an older schema up to date.
    ///
    /// A migrated project is saved at once, in one commit that also keeps the originals as a backup. So a
    /// crash leaves the old project or the migrated one, and a failed step leaves the old one.
    ///
    /// # Errors
    /// Refuses a project with no manifest, or a newer schema, and reports a failed migration. None changes
    /// anything on disk.
    pub fn load(&mut self) -> Result<Loaded, ProjectError> {
        let original = self.load_files("")?.ok_or(ProjectError::NoProject)?;
        let mut project = original.clone();
        let migrated_from = self.migrate(&mut project)?;
        let base_path = FilePath::new(AUTOSAVE_BASE)?;
        let base = self.store.read(&base_path)?;
        if let Some(from) = migrated_from {
            let mut changes = self.changes_for(&project, false)?;
            // A waiting autosave stays, still based on this project now that it has migrated.
            if base.as_deref() == Some(original.digest().as_bytes()) {
                changes.write(base_path, project.digest());
            }
            let backup = format!("{BACKUPS}schema-{from}/");
            changes.write(
                FilePath::new(&format!("{backup}{MANIFEST}"))?,
                original.manifest_text(),
            );
            for (path, bytes) in original.files() {
                changes.write(FilePath::new(&format!("{backup}{path}"))?, bytes.to_vec());
            }
            self.store.commit_any(&changes)?;
        }
        let autosave = match base {
            None => Autosave::None,
            Some(base)
                if base == original.digest().as_bytes() || base == project.digest().as_bytes() =>
            {
                Autosave::Current
            }
            Some(_) => Autosave::Stale,
        };
        Ok(Loaded {
            project,
            migrated_from,
            autosave,
        })
    }

    fn changes_for(
        &mut self,
        project: &Project,
        clear_autosave: bool,
    ) -> Result<ChangeSet, ProjectError> {
        let mut changes = ChangeSet::new();
        changes.write(FilePath::new(MANIFEST)?, project.manifest_text());
        for (path, bytes) in project.files() {
            changes.write(path.clone(), bytes.to_vec());
        }
        for path in self.store.all_files()? {
            let text = path.as_str();
            if text == MANIFEST || text.starts_with(".sc-store/") {
                continue;
            }
            if project.file(&path).is_none() {
                changes.remove(path);
            }
        }
        if clear_autosave {
            // A save supersedes any autosave, in the same commit.
            for path in self.store.all_files()? {
                let text = path.as_str();
                if text == AUTOSAVE_BASE || text.starts_with(AUTOSAVE_FILES) {
                    changes.remove(path);
                }
            }
        }
        Ok(changes)
    }

    fn require_current(&self, project: &Project) -> Result<(), ProjectError> {
        if project.schema() == self.schema.current {
            Ok(())
        } else {
            Err(ProjectError::WrongSchema {
                found: project.schema(),
                current: self.schema.current,
            })
        }
    }

    /// Save the project as one commit, clearing any autosave. Saving the same state twice writes nothing.
    ///
    /// # Errors
    /// The project is another schema, or the store refused. The project on disk is then old or new, whole.
    pub fn save(&mut self, project: &Project) -> Result<Commit, ProjectError> {
        self.require_current(project)?;
        let changes = self.changes_for(project, true)?;
        Ok(self.store.commit_any(&changes)?)
    }

    /// Keep a copy of unsaved work beside the project, leaving the project itself as it is.
    ///
    /// # Errors
    /// The project is another schema, or the store refused. A previous autosave then stays whole.
    pub fn autosave(&mut self, project: &Project) -> Result<Commit, ProjectError> {
        self.require_current(project)?;
        let mut changes = ChangeSet::new();
        for path in self.store.all_files()? {
            if path.as_str().starts_with(AUTOSAVE_FILES) {
                changes.remove(path);
            }
        }
        changes.write(
            FilePath::new(&format!("{AUTOSAVE_FILES}{MANIFEST}"))?,
            project.manifest_text(),
        );
        for (path, bytes) in project.files() {
            changes.write(
                FilePath::new(&format!("{AUTOSAVE_FILES}{path}"))?,
                bytes.to_vec(),
            );
        }
        let base = match self.load_files("")? {
            Some(saved) => saved.digest(),
            None => String::new(),
        };
        changes.write(FilePath::new(AUTOSAVE_BASE)?, base);
        Ok(self.store.commit_any(&changes)?)
    }

    /// The waiting autosave as a project, migrated if an older version wrote it. Saving it makes it the project.
    ///
    /// # Errors
    /// There is no autosave, or it could not be read or migrated. Nothing on disk changes.
    pub fn restore_autosave(&mut self) -> Result<Project, ProjectError> {
        let mut project = self
            .load_files(AUTOSAVE_FILES)?
            .ok_or(ProjectError::NoAutosave)?;
        self.migrate(&mut project)?;
        Ok(project)
    }

    /// Drop the waiting autosave.
    ///
    /// # Errors
    /// The store refused. The autosave then stays whole.
    pub fn discard_autosave(&mut self) -> Result<Commit, ProjectError> {
        let mut changes = ChangeSet::new();
        for path in self.store.all_files()? {
            let text = path.as_str();
            if text == AUTOSAVE_BASE || text.starts_with(AUTOSAVE_FILES) {
                changes.remove(path);
            }
        }
        Ok(self.store.commit_any(&changes)?)
    }

    /// The backend, to flush or close it.
    #[must_use]
    pub fn into_repository(self) -> R {
        self.store.into_repository()
    }
}
