//! A project keeps what it does not understand, migrates all or nothing, and survives a crash anywhere.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
mod common;

use common::{everything, path, Crashing};
use sc_store::{
    Autosave, Commit, MemoryRepository, Migration, Project, ProjectError, ProjectStore, Repository,
    Schema, Value,
};
use std::collections::BTreeMap;

fn sample() -> Project {
    let mut project = Project::new();
    project.set_field("name", Value::Text("skirt".to_owned()));
    project
        .set_file(path("recipes/skirt.txt"), "waist = 74 cm\n")
        .expect("a project file");
    project
        .set_file(path("measurements.json"), "{}\n")
        .expect("a project file");
    project
}

fn saved(project: &Project) -> MemoryRepository {
    let (mut store, _) = ProjectStore::open(MemoryRepository::new()).expect("opens");
    store.save(project).expect("saves");
    store.into_repository()
}

fn project_files(repository: &MemoryRepository) -> BTreeMap<String, Vec<u8>> {
    everything(repository)
        .into_iter()
        .filter(|(name, _)| !name.starts_with(".sc-store/"))
        .collect()
}

#[test]
fn a_saved_project_reopens_equal_and_saving_again_writes_nothing() {
    let project = sample();
    let disk = saved(&project);
    assert_eq!(
        String::from_utf8_lossy(
            &disk
                .read(&path("project.json"))
                .expect("reads")
                .expect("exists")
        ),
        "{\n  \"name\": \"skirt\",\n  \"schema\": 1\n}\n"
    );
    let first = everything(&disk);
    let (mut store, _) = ProjectStore::open(disk).expect("opens");
    let loaded = store.load().expect("loads");
    assert_eq!(loaded.project, project);
    assert_eq!(loaded.migrated_from, None);
    assert_eq!(loaded.autosave, Autosave::None);
    assert_eq!(
        store.save(&loaded.project).expect("saves"),
        Commit::Unchanged
    );
    assert_eq!(everything(&store.into_repository()), first);
}

#[test]
fn removing_a_file_removes_it_from_the_project() {
    let mut project = sample();
    let (mut store, _) = ProjectStore::open(saved(&project)).expect("opens");
    assert!(project.remove_file(&path("measurements.json")).is_some());
    store.save(&project).expect("saves");
    let names: Vec<String> = project_files(&store.into_repository())
        .into_keys()
        .collect();
    assert_eq!(names, ["project.json", "recipes/skirt.txt"]);
}

#[test]
fn fields_and_files_this_version_does_not_understand_survive_a_save() {
    let mut disk = saved(&sample());
    disk.write(
        &path("project.json"),
        b"{\"from_a_newer_tool\": {\"keep\": [1, 2]}, \"name\": \"skirt\", \"schema\": 1}",
    )
    .expect("memory writes");
    disk.write(&path("extensions/other-tool.bin"), &[0, 159, 255])
        .expect("memory writes");
    let (mut store, _) = ProjectStore::open(disk).expect("opens");
    let mut project = store.load().expect("loads").project;
    project.set_field("name", Value::Text("renamed".to_owned()));
    store.save(&project).expect("saves");
    let disk = store.into_repository();
    assert_eq!(
        String::from_utf8_lossy(&disk.read(&path("project.json")).expect("reads").expect("exists")),
        "{\n  \"from_a_newer_tool\": {\n    \"keep\": [\n      1,\n      2\n    ]\n  },\n  \"name\": \"renamed\",\n  \"schema\": 1\n}\n"
    );
    assert_eq!(
        disk.read(&path("extensions/other-tool.bin"))
            .expect("reads"),
        Some(vec![0, 159, 255])
    );
}

#[test]
fn a_newer_project_is_refused_and_left_untouched() {
    let mut disk = saved(&sample());
    disk.write(
        &path("project.json"),
        b"{\"schema\": 2, \"new_field\": true}",
    )
    .expect("memory writes");
    let before = everything(&disk);
    let (mut store, _) = ProjectStore::open(disk).expect("opens");
    let error = store.load().expect_err("a newer schema");
    assert_eq!(
        error.to_string(),
        "a newer StitchCAD wrote this project (schema 2); this version reads up to schema 1. Nothing was changed"
    );
    assert_eq!(everything(&store.into_repository()), before);
}

#[test]
fn a_manifest_that_cannot_be_read_is_refused_with_the_reason() {
    for (bytes, words) in [
        (
            &b"{\"name\": 1.5, \"schema\": 1}"[..],
            "numbers here are whole",
        ),
        (&b"[]"[..], "not an object"),
        (&[0xff, 0xfe][..], "not UTF-8"),
        (&b"{\"name\": \"x\"}"[..], "no schema field"),
        (&b"{\"schema\": 0}"[..], "no schema field"),
        (&b"{\"schema\": \"1\"}"[..], "no schema field"),
    ] {
        let mut disk = saved(&sample());
        disk.write(&path("project.json"), bytes)
            .expect("memory writes");
        let (mut store, _) = ProjectStore::open(disk).expect("opens");
        let error = store.load().expect_err("a bad manifest");
        assert!(error.to_string().contains(words), "{bytes:?}: {error}");
    }
    let (mut store, _) = ProjectStore::open(MemoryRepository::new()).expect("opens");
    assert_eq!(
        store.load().expect_err("no manifest"),
        ProjectError::NoProject
    );
}

#[test]
fn the_manifest_and_the_stores_directory_cannot_be_written_as_files() {
    let mut project = Project::new();
    assert!(matches!(
        project.set_file(path("project.json"), "x"),
        Err(ProjectError::NotAFile(_))
    ));
    assert!(matches!(
        project.set_file(path(".sc-store/journal"), "x"),
        Err(ProjectError::NotAFile(_))
    ));
    project.set_field("schema", Value::Int(99));
    assert_eq!(project.schema(), 1);
    assert_eq!(project.remove_field("schema"), None);
}

fn rename_field(project: &mut Project) -> Result<(), String> {
    let name = project.remove_field("name").ok_or("no name to rename")?;
    project.set_field("title", name);
    Ok(())
}

fn add_unit_file(project: &mut Project) -> Result<(), String> {
    project
        .set_file(path("units.txt"), "cm\n")
        .map_err(|error| error.to_string())
}

fn refuses(_project: &mut Project) -> Result<(), String> {
    Err("the recipe uses a removed operation".to_owned())
}

fn two_steps() -> Schema {
    Schema::new(
        3,
        vec![
            Migration {
                from: 1,
                apply: rename_field,
            },
            Migration {
                from: 2,
                apply: add_unit_file,
            },
        ],
    )
}

#[test]
fn a_migration_reaches_the_current_schema_and_keeps_the_originals() {
    let old = sample();
    let (mut store, _) = ProjectStore::open_with(saved(&old), two_steps()).expect("opens");
    let loaded = store.load().expect("migrates");
    assert_eq!(loaded.migrated_from, Some(1));
    assert_eq!(loaded.project.schema(), 3);
    assert_eq!(
        loaded.project.field("title"),
        Some(&Value::Text("skirt".to_owned()))
    );
    assert_eq!(loaded.project.file(&path("units.txt")), Some(&b"cm\n"[..]));

    let again = store.load().expect("loads");
    assert_eq!(again.migrated_from, None, "the migrated project was saved");
    assert_eq!(again.project, loaded.project);

    let disk = store.into_repository();
    assert_eq!(
        disk.read(&path(".sc-store/backups/schema-1/project.json"))
            .expect("reads"),
        Some(b"{\n  \"name\": \"skirt\",\n  \"schema\": 1\n}\n".to_vec())
    );
    assert_eq!(
        disk.read(&path(".sc-store/backups/schema-1/recipes/skirt.txt"))
            .expect("reads"),
        Some(b"waist = 74 cm\n".to_vec())
    );
}

#[test]
fn a_failed_migration_leaves_the_project_exactly_as_it_was() {
    let schema = Schema::new(
        3,
        vec![
            Migration {
                from: 1,
                apply: rename_field,
            },
            Migration {
                from: 2,
                apply: refuses,
            },
        ],
    );
    let disk = saved(&sample());
    let before = everything(&disk);
    let (mut store, _) = ProjectStore::open_with(disk, schema).expect("opens");
    let error = store.load().expect_err("the second step fails");
    assert_eq!(
        error.to_string(),
        "migrating from schema 2 to 3 failed: the recipe uses a removed operation. Nothing was changed"
    );
    let disk = store.into_repository();
    assert_eq!(everything(&disk), before);

    // The project still opens once a corrected version is installed.
    let (mut store, _) = ProjectStore::open_with(disk, two_steps()).expect("opens");
    assert_eq!(store.load().expect("migrates").migrated_from, Some(1));
}

#[test]
fn a_missing_step_is_refused_and_changes_nothing() {
    let disk = saved(&sample());
    let before = everything(&disk);
    let schema = Schema::new(2, vec![]);
    let (mut store, _) = ProjectStore::open_with(disk, schema).expect("opens");
    assert_eq!(
        store.load().expect_err("no step"),
        ProjectError::NoMigration(1)
    );
    assert_eq!(everything(&store.into_repository()), before);
}

#[test]
fn saving_a_project_of_another_schema_is_refused() {
    let (mut store, _) =
        ProjectStore::open_with(MemoryRepository::new(), two_steps()).expect("opens");
    let error = store
        .save(&sample())
        .expect_err("schema 1 to a schema 3 store");
    assert_eq!(
        error,
        ProjectError::WrongSchema {
            found: 1,
            current: 3
        }
    );
    assert_eq!(store.into_repository(), MemoryRepository::new());
}

#[test]
fn a_crash_at_every_step_of_a_migration_leaves_the_old_project_or_the_migrated_one() {
    let (old_disk, old_files) = {
        let disk = saved(&sample());
        let files = project_files(&disk);
        (disk, files)
    };
    let (mut migrated_store, _) =
        ProjectStore::open_with(old_disk.clone(), two_steps()).expect("opens");
    migrated_store.load().expect("migrates");
    let migrated_files = project_files(&migrated_store.into_repository());

    let (mut saw_old, mut saw_new, mut budget) = (0, 0, 0);
    loop {
        let (mut store, _) =
            ProjectStore::open_with(Crashing::new(old_disk.clone(), budget), two_steps())
                .expect("opens");
        let finished = store.load().is_ok();
        let disk = store.into_repository().inner;
        if finished {
            break;
        }
        let (store, _) = ProjectStore::open_with(disk, two_steps()).expect("recovery works");
        let recovered = store.into_repository();
        let files = project_files(&recovered);
        if files == old_files {
            saw_old += 1;
            assert!(recovered
                .read(&path(".sc-store/backups/schema-1/project.json"))
                .expect("reads")
                .is_none());
        } else {
            assert_eq!(files, migrated_files, "budget {budget}");
            saw_new += 1;
            assert!(recovered
                .read(&path(".sc-store/backups/schema-1/project.json"))
                .expect("reads")
                .is_some());
        }
        // Either way the next load ends in the same migrated project.
        let (mut store, _) = ProjectStore::open_with(recovered, two_steps()).expect("opens");
        store.load().expect("loads");
        assert_eq!(
            project_files(&store.into_repository()),
            migrated_files,
            "budget {budget}"
        );
        budget += 1;
    }
    assert!(saw_old >= 3 && saw_new >= 3, "old {saw_old}, new {saw_new}");
}

#[test]
fn a_crash_at_every_step_of_a_save_leaves_the_old_project_or_the_new_one() {
    let old = sample();
    let mut new = old.clone();
    new.set_field("name", Value::Text("renamed".to_owned()));
    new.set_file(path("recipes/skirt.txt"), "waist = 76 cm\n")
        .expect("a project file");
    new.remove_file(&path("measurements.json"));
    let disk = saved(&old);
    let (mut saw_old, mut saw_new, mut budget) = (0, 0, 0);
    loop {
        let (mut store, _) =
            ProjectStore::open(Crashing::new(disk.clone(), budget)).expect("opens");
        let finished = store.save(&new).is_ok();
        let crashed_disk = store.into_repository().inner;
        let (mut store, _) = ProjectStore::open(crashed_disk).expect("recovery works");
        let project = store.load().expect("loads").project;
        if finished {
            assert_eq!(project, new);
            break;
        }
        if project == old {
            saw_old += 1;
        } else {
            assert_eq!(project, new, "budget {budget}");
            saw_new += 1;
        }
        budget += 1;
    }
    assert!(saw_old >= 2 && saw_new >= 2, "old {saw_old}, new {saw_new}");
}

#[test]
fn an_autosave_leaves_the_project_alone_and_can_be_restored() {
    let saved_state = sample();
    let mut work = saved_state.clone();
    work.set_file(path("recipes/skirt.txt"), "waist = 80 cm\n")
        .expect("a project file");
    work.set_file(path("notes.txt"), "in progress\n")
        .expect("a project file");

    let disk = saved(&saved_state);
    let before = project_files(&disk);
    let (mut store, _) = ProjectStore::open(disk).expect("opens");
    store.autosave(&work).expect("autosaves");
    let disk = store.into_repository();
    assert_eq!(
        project_files(&disk),
        before,
        "the project itself did not change"
    );

    let (mut store, _) = ProjectStore::open(disk).expect("opens");
    let loaded = store.load().expect("loads");
    assert_eq!(loaded.project, saved_state);
    assert_eq!(loaded.autosave, Autosave::Current);
    let restored = store.restore_autosave().expect("restores");
    assert_eq!(restored, work);

    store.save(&restored).expect("saves");
    let loaded = store.load().expect("loads");
    assert_eq!(loaded.project, work);
    assert_eq!(
        loaded.autosave,
        Autosave::None,
        "a save clears the autosave"
    );
    assert_eq!(
        store.restore_autosave().expect_err("cleared"),
        ProjectError::NoAutosave
    );
}

#[test]
fn a_second_autosave_replaces_the_first_whole_and_discarding_clears_it() {
    let saved_state = sample();
    let mut first = saved_state.clone();
    first
        .set_file(path("first-only.txt"), "1\n")
        .expect("a project file");
    let mut second = saved_state.clone();
    second
        .set_file(path("second-only.txt"), "2\n")
        .expect("a project file");

    let (mut store, _) = ProjectStore::open(saved(&saved_state)).expect("opens");
    store.autosave(&first).expect("autosaves");
    store.autosave(&second).expect("autosaves");
    assert_eq!(store.restore_autosave().expect("restores"), second);
    store.discard_autosave().expect("discards");
    assert_eq!(store.load().expect("loads").autosave, Autosave::None);
    let leftovers: Vec<String> = everything(&store.into_repository())
        .into_keys()
        .filter(|name| name.starts_with(".sc-store/"))
        .collect();
    assert!(leftovers.is_empty(), "{leftovers:?}");
}

#[test]
fn an_autosave_made_from_a_different_saved_state_is_reported_stale() {
    let saved_state = sample();
    let mut work = saved_state.clone();
    work.set_file(path("notes.txt"), "work\n")
        .expect("a project file");
    let (mut store, _) = ProjectStore::open(saved(&saved_state)).expect("opens");
    store.autosave(&work).expect("autosaves");
    let mut disk = store.into_repository();
    disk.write(&path("recipes/skirt.txt"), b"edited by hand\n")
        .expect("memory writes");
    let (mut store, _) = ProjectStore::open(disk).expect("opens");
    assert_eq!(store.load().expect("loads").autosave, Autosave::Stale);
    assert_eq!(store.restore_autosave().expect("still recoverable"), work);
}

#[test]
fn a_migration_keeps_a_waiting_autosave_and_migrates_it_on_restore() {
    let saved_state = sample();
    let mut work = saved_state.clone();
    work.set_file(path("notes.txt"), "work\n")
        .expect("a project file");
    let (mut store, _) = ProjectStore::open(saved(&saved_state)).expect("opens");
    store.autosave(&work).expect("autosaves");
    let (mut store, _) =
        ProjectStore::open_with(store.into_repository(), two_steps()).expect("opens");

    let loaded = store.load().expect("migrates");
    assert_eq!(loaded.autosave, Autosave::Current);
    let again = store.load().expect("loads");
    assert_eq!(
        again.autosave,
        Autosave::Current,
        "still based on the same work, after the migration"
    );

    let restored = store.restore_autosave().expect("restores");
    assert_eq!(restored.schema(), 3);
    assert_eq!(
        restored.field("title"),
        Some(&Value::Text("skirt".to_owned()))
    );
    assert_eq!(restored.file(&path("notes.txt")), Some(&b"work\n"[..]));
    store
        .save(&restored)
        .expect("a migrated autosave can be saved");
}

#[test]
fn a_crash_at_every_step_of_an_autosave_keeps_the_project_and_a_whole_autosave() {
    let saved_state = sample();
    let mut first = saved_state.clone();
    first
        .set_file(path("first-only.txt"), "1\n")
        .expect("a project file");
    let mut second = saved_state.clone();
    second
        .set_file(path("second-only.txt"), "2\n")
        .expect("a project file");

    let (mut store, _) = ProjectStore::open(saved(&saved_state)).expect("opens");
    store.autosave(&first).expect("autosaves");
    let disk = store.into_repository();
    let files = project_files(&disk);

    let mut budget = 0;
    loop {
        let (mut store, _) =
            ProjectStore::open(Crashing::new(disk.clone(), budget)).expect("opens");
        let finished = store.autosave(&second).is_ok();
        let crashed_disk = store.into_repository().inner;
        let (mut store, _) = ProjectStore::open(crashed_disk).expect("recovery works");
        let restored = store
            .restore_autosave()
            .expect("an autosave is always whole");
        assert!(restored == first || restored == second, "budget {budget}");
        assert_eq!(
            project_files(&store.into_repository()),
            files,
            "budget {budget}"
        );
        if finished {
            assert_eq!(restored, second);
            break;
        }
        budget += 1;
    }
    assert!(budget > 4, "only {budget} crash points");
}

#[test]
fn a_digest_tells_apart_projects_whose_bytes_only_move_between_fields() {
    let mut first = Project::new();
    first.set_file(path("a"), "bc").expect("a project file");
    let mut second = Project::new();
    second.set_file(path("ab"), "c").expect("a project file");
    assert_ne!(first.digest(), second.digest());
    assert_eq!(first.digest(), first.clone().digest());
}
