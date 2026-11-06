//! A project file's path must be valid on every system a project can move to.
#![allow(clippy::expect_used)]
use sc_store::FilePath;

#[test]
fn ordinary_paths_are_kept() {
    for path in [
        "project.json",
        "recipes/skirt.txt",
        "a/b/c/d.json",
        ".hidden",
        "été/ñ.txt",
        "with space/x y.txt",
    ] {
        assert_eq!(FilePath::new(path).expect("valid").as_str(), path);
    }
}

#[test]
fn unportable_paths_are_refused_with_the_reason() {
    for (path, words) in [
        ("", "empty"),
        ("/etc/passwd", "starts or ends"),
        ("a/", "starts or ends"),
        ("a//b", "component is empty"),
        (".", "may not contain"),
        ("a/../b", "may not contain"),
        ("a\\b", "character"),
        ("c:/x", "character"),
        ("a?b", "character"),
        ("a\nb", "character"),
        ("name.", "dot or a space"),
        ("name ", "dot or a space"),
    ] {
        let error = FilePath::new(path).expect_err(path);
        assert!(error.reason.contains(words), "{path:?}: {error}");
    }
    assert!(FilePath::new(&"a".repeat(1025)).is_err());
    assert!(FilePath::new(&"a".repeat(1024)).is_ok());
}
