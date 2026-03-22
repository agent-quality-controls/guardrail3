use std::collections::BTreeMap;
use std::path::PathBuf;

use super::{DirEntry, ProjectTree};

fn sample_tree() -> ProjectTree {
    ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([
            (
                "".to_owned(),
                DirEntry {
                    dirs: vec!["crates".to_owned(), "apps".to_owned()],
                    files: vec!["Cargo.toml".to_owned(), "rust-toolchain.toml".to_owned()],
                },
            ),
            (
                "crates".to_owned(),
                DirEntry {
                    dirs: vec!["api".to_owned(), "missing".to_owned()],
                    files: vec![],
                },
            ),
            (
                "crates/api".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned(), ".rustfmt.toml".to_owned()],
                },
            ),
            (
                "crates/missing".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec![],
                },
            ),
            (
                "apps".to_owned(),
                DirEntry {
                    dirs: vec!["cli".to_owned()],
                    files: vec![],
                },
            ),
            (
                "apps/cli".to_owned(),
                DirEntry {
                    dirs: vec![],
                    files: vec!["Cargo.toml".to_owned()],
                },
            ),
        ]),
        content: BTreeMap::new(),
    }
}

#[test]
fn file_exists_checks_root_and_nested_files() {
    let tree = sample_tree();

    assert!(tree.file_exists("Cargo.toml"));
    assert!(tree.file_exists("rust-toolchain.toml"));
    assert!(tree.file_exists("crates/api/Cargo.toml"));
    assert!(tree.file_exists("crates/api/.rustfmt.toml"));
    assert!(!tree.file_exists("crates/missing/Cargo.toml"));
    assert!(!tree.file_exists("missing.toml"));
}

#[test]
fn all_dir_rels_excludes_root() {
    let tree = sample_tree();

    assert_eq!(
        tree.all_dir_rels(),
        vec![
            "apps".to_owned(),
            "apps/cli".to_owned(),
            "crates".to_owned(),
            "crates/api".to_owned(),
            "crates/missing".to_owned(),
        ]
    );
}

#[test]
fn dirs_with_file_returns_only_matching_dirs() {
    let tree = sample_tree();

    assert_eq!(
        tree.dirs_with_file("Cargo.toml"),
        vec!["apps/cli".to_owned(), "crates/api".to_owned()]
    );
    assert_eq!(tree.dirs_with_file(".rustfmt.toml"), vec!["crates/api".to_owned()]);
}

#[test]
fn matching_dir_rels_matches_actual_dirs_only() {
    let tree = sample_tree();

    assert_eq!(
        tree.matching_dir_rels("crates/*"),
        vec!["crates/api".to_owned(), "crates/missing".to_owned()]
    );
    assert_eq!(tree.matching_dir_rels("apps/*"), vec!["apps/cli".to_owned()]);
    assert!(tree.matching_dir_rels("does/not/exist/*").is_empty());
    assert!(tree.matching_dir_rels("[invalid").is_empty());
}
