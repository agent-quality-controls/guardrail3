use std::collections::BTreeMap;
use std::path::PathBuf;

use super::{DirEntry, ProjectTree};
use guardrail3_domain_project_tree_assertions::assert_string_vec_eq;

fn sample_tree() -> ProjectTree {
    ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec!["crates".to_owned(), "apps".to_owned()],
                    vec!["Cargo.toml".to_owned(), "rust-toolchain.toml".to_owned()],
                    vec![],
                    vec![],
                ),
            ),
            (
                "crates".to_owned(),
                DirEntry::new(
                    vec!["api".to_owned(), "missing".to_owned()],
                    vec![],
                    vec![],
                    vec![],
                ),
            ),
            (
                "crates/api".to_owned(),
                DirEntry::new(
                    vec![],
                    vec!["Cargo.toml".to_owned(), ".rustfmt.toml".to_owned()],
                    vec![],
                    vec![],
                ),
            ),
            (
                "crates/missing".to_owned(),
                DirEntry::new(vec![], vec![], vec![], vec![]),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(vec!["cli".to_owned()], vec![], vec![], vec![]),
            ),
            (
                "apps/cli".to_owned(),
                DirEntry::new(vec![], vec!["Cargo.toml".to_owned()], vec![], vec![]),
            ),
        ]),
        BTreeMap::new(),
    )
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

    assert_string_vec_eq(
        &tree.all_dir_rels(),
        &["apps", "apps/cli", "crates", "crates/api", "crates/missing"],
        "all_dir_rels should return every known non-root directory in sorted order",
    );
}

#[test]
fn dirs_with_file_returns_only_matching_dirs() {
    let tree = sample_tree();

    assert_string_vec_eq(
        &tree.dirs_with_file("Cargo.toml"),
        &["apps/cli", "crates/api"],
        "dirs_with_file should return only the directories containing Cargo.toml",
    );
    assert_string_vec_eq(
        &tree.dirs_with_file(".rustfmt.toml"),
        &["crates/api"],
        "dirs_with_file should return only the directories containing .rustfmt.toml",
    );
}

#[test]
fn matching_dir_rels_matches_actual_dirs_only() {
    let tree = sample_tree();

    assert_string_vec_eq(
        &tree.matching_dir_rels("crates/*"),
        &["crates/api", "crates/missing"],
        "matching_dir_rels should match the crate subtree only",
    );
    assert_string_vec_eq(
        &tree.matching_dir_rels("apps/*"),
        &["apps/cli"],
        "matching_dir_rels should match the app subtree only",
    );
    assert!(tree.matching_dir_rels("does/not/exist/*").is_empty());
    assert!(tree.matching_dir_rels("[invalid").is_empty());
}
