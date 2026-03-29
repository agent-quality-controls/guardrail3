use std::collections::{BTreeMap, BTreeSet};

use super::super::{DirEntry, ProjectTree, run_tree};
use guardrail3_app_rs_family_code_assertions::rs_code_30_input_failures::{
    assert_files, assert_guardrail_policy_parse_failure, assert_message_fragment_failure,
    assert_source_parse_failure,
};
use test_support::{create_dir_all, create_temp_dir, write_path};

#[test]
fn family_surfaces_source_parse_failures_with_exact_owned_hit_set() {
    let root = create_temp_dir("rs-code-30-source-parse-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.path().join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.path()));
    write_path(&source_abs, "fn broken( {");

    let tree = ProjectTree {
        root: root.path().to_path_buf(),
        structure: BTreeMap::from([
            (
                String::new(),
                DirEntry {
                    dirs: vec!["src".to_owned()],
                    files: vec!["Cargo.toml".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
            (
                "src".to_owned(),
                DirEntry {
                    dirs: Vec::new(),
                    files: vec!["lib.rs".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
        ]),
        content: BTreeMap::from([(
            "Cargo.toml".to_owned(),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
        )]),
    };

    let results = run_tree(&tree);

    assert_files(&results, BTreeSet::from([source_rel.to_owned()]));
    assert_source_parse_failure(&results, source_rel);
}

#[test]
fn family_surfaces_guardrail_policy_parse_failures_with_exact_owned_hit_set() {
    let root = create_temp_dir("rs-code-30-guardrail-parse-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.path().join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.path()));
    write_path(
        &source_abs,
        "pub fn parse() -> Result<(), String> { Ok(()) }",
    );

    let tree = ProjectTree {
        root: root.path().to_path_buf(),
        structure: BTreeMap::from([
            (
                String::new(),
                DirEntry {
                    dirs: vec!["src".to_owned()],
                    files: vec!["Cargo.toml".to_owned(), "guardrail3.toml".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
            (
                "src".to_owned(),
                DirEntry {
                    dirs: Vec::new(),
                    files: vec!["lib.rs".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
        ]),
        content: BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "guardrail3.toml".to_owned(),
                "[rust.packages\n type = \"library\"\n".to_owned(),
            ),
        ]),
    };

    let results = run_tree(&tree);

    assert_files(&results, BTreeSet::from(["guardrail3.toml".to_owned()]));
    assert_guardrail_policy_parse_failure(&results, "guardrail3.toml");
}

#[test]
fn family_surfaces_unreadable_root_cargo_for_code_root_discovery() {
    let root = create_temp_dir("rs-code-30-cargo-read-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.path().join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.path()));
    write_path(&source_abs, "pub fn parse() {}\n");

    let tree = ProjectTree {
        root: root.path().to_path_buf(),
        structure: BTreeMap::from([
            (
                String::new(),
                DirEntry {
                    dirs: vec!["src".to_owned()],
                    files: vec!["Cargo.toml".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
            (
                "src".to_owned(),
                DirEntry {
                    dirs: Vec::new(),
                    files: vec!["lib.rs".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
        ]),
        content: BTreeMap::new(),
    };

    let results = run_tree(&tree);

    assert_files(&results, BTreeSet::from(["Cargo.toml".to_owned()]));
    assert_message_fragment_failure(
        &results,
        "Cargo.toml",
        "Failed to read Cargo.toml for code-family root discovery.",
    );
}

#[test]
fn family_surfaces_unreadable_guardrail_policy_input() {
    let root = create_temp_dir("rs-code-30-guardrail-read-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.path().join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.path()));
    write_path(&source_abs, "pub fn parse() {}\n");

    let tree = ProjectTree {
        root: root.path().to_path_buf(),
        structure: BTreeMap::from([
            (
                String::new(),
                DirEntry {
                    dirs: vec!["src".to_owned()],
                    files: vec!["Cargo.toml".to_owned(), "guardrail3.toml".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
            (
                "src".to_owned(),
                DirEntry {
                    dirs: Vec::new(),
                    files: vec!["lib.rs".to_owned()],
                    symlink_dirs: vec![],
                    symlink_files: vec![],
                },
            ),
        ]),
        content: BTreeMap::from([(
            "Cargo.toml".to_owned(),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
        )]),
    };

    let results = run_tree(&tree);

    assert_files(&results, BTreeSet::from(["guardrail3.toml".to_owned()]));
    assert_message_fragment_failure(
        &results,
        "guardrail3.toml",
        "Failed to read guardrail3.toml for code-family policy resolution.",
    );
}
