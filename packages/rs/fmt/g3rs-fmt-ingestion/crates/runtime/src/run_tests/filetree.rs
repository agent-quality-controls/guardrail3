use g3rs_fmt_ingestion_assertions::run as assertions;
use tempfile::tempdir;

use super::helpers::{crawl, git_init, write};

#[test]
fn filetree_ingests_root_and_nested_fmt_configs() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), "edition = \"2024\"\n");
    write(root.join(".rustfmt.toml"), "edition = \"2021\"\n");
    write(root.join("crates/api/rustfmt.toml"), "edition = \"2024\"\n");
    write(
        root.join("crates/api/.rustfmt.toml"),
        "edition = \"2024\"\n",
    );

    let input = crate::run::ingest_for_file_tree_checks(&crawl(root));

    assert_eq!(
        input.root_rustfmt_toml_rel_path.as_deref(),
        Some("rustfmt.toml")
    );
    assert_eq!(
        input.root_dot_rustfmt_toml_rel_path.as_deref(),
        Some(".rustfmt.toml")
    );
    assert_eq!(
        input
            .nested_config_files
            .iter()
            .map(|file| file.rel_path.as_str())
            .collect::<Vec<_>>(),
        vec!["crates/api/.rustfmt.toml", "crates/api/rustfmt.toml"]
    );
    assert_eq!(
        input.dual_conflict_dirs,
        vec![String::new(), "crates/api".to_owned()]
    );
}

#[test]
fn filetree_ignores_fixture_and_target_configs() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("tests/fixtures/rustfmt.toml"),
        "edition = \"2024\"\n",
    );
    write(
        root.join("target/generated/.rustfmt.toml"),
        "edition = \"2024\"\n",
    );

    let input = crate::run::ingest_for_file_tree_checks(&crawl(root));

    assert!(input.nested_config_files.is_empty());
    assert!(input.dual_conflict_dirs.is_empty());
}

#[test]
fn filetree_ignores_claude_worktrees_configs() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join(".claude/worktrees/agent/rustfmt.toml"),
        "edition = \"2024\"\n",
    );
    write(
        root.join(".claude/worktrees/agent/.rustfmt.toml"),
        "edition = \"2024\"\n",
    );

    let input = crate::run::ingest_for_file_tree_checks(&crawl(root));

    assert!(input.nested_config_files.is_empty());
    assert!(input.dual_conflict_dirs.is_empty());
}

#[test]
fn filetree_ignores_snapshot_configs() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("tests/snapshots/case/rustfmt.toml"),
        "edition = \"2024\"\n",
    );
    write(
        root.join("tests/snapshots/case/.rustfmt.toml"),
        "edition = \"2024\"\n",
    );

    let input = crate::run::ingest_for_file_tree_checks(&crawl(root));

    assert!(input.nested_config_files.is_empty());
    assert!(input.dual_conflict_dirs.is_empty());
}

#[test]
fn pipeline_reports_missing_root_config() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let input = crate::run::ingest_for_file_tree_checks(&crawl(root));
    let results = g3rs_fmt_filetree_checks::check(&input);

    assertions::assert_missing_root(&results);
}

#[test]
fn pipeline_reports_nested_override_and_dual_conflict() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), "edition = \"2024\"\n");
    write(root.join("crates/api/rustfmt.toml"), "edition = \"2024\"\n");
    write(
        root.join("crates/api/.rustfmt.toml"),
        "edition = \"2024\"\n",
    );

    let input = crate::run::ingest_for_file_tree_checks(&crawl(root));
    let results = g3rs_fmt_filetree_checks::check(&input);

    assertions::assert_nested_override_and_dual_conflict(&results);
}
