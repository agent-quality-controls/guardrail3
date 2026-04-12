use std::fs;
use std::path::Path;
use std::process::Command;

use guardrail3_check_types::G3CheckResult;
use tempfile::tempdir;

fn git_init(path: &Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
    assert!(status.success(), "git init should exit successfully");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}

fn run_file_tree_pipeline(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingestion should succeed");
    g3rs_code_file_tree_checks::check(&input)
}

#[test]
fn pipeline_reports_structural_cap_violation() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "");
    for index in 0..13 {
        write(root.join(format!("src/dir{index}/mod.rs")), "");
    }
    for index in 0..21 {
        write(root.join(format!("src/file{index}.rs")), "");
    }
    write(root.join("src/a/b/c/d/e/f/leaf.rs"), "");

    let results = run_file_tree_pipeline(root);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(results[0].id(), "RS-CODE-FILETREE-35");
    assert_eq!(results[0].file(), Some("Cargo.toml"));
}

#[test]
fn pipeline_stays_quiet_at_exact_thresholds() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "");
    for index in 0..11 {
        write(root.join(format!("src/dir{index}/mod.rs")), "");
    }
    for index in 0..19 {
        write(root.join(format!("src/file{index}.rs")), "");
    }
    write(root.join("src/a/b/c/d/e/mod.rs"), "");

    let results = run_file_tree_pipeline(root);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn pipeline_measures_workspace_member_separately() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/api/src/lib.rs"), "");
    for index in 0..13 {
        write(root.join(format!("crates/api/src/dir{index}/mod.rs")), "");
    }

    let results = run_file_tree_pipeline(root);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(results[0].id(), "RS-CODE-FILETREE-35");
    assert_eq!(results[0].file(), Some("crates/api/Cargo.toml"));
}

#[test]
fn pipeline_does_not_charge_member_structure_to_root_package() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[package]\n\
name = \"root\"\n\
version = \"0.1.0\"\n\
edition = \"2024\"\n\
\n\
[workspace]\n\
members = [\"crates/api\"]\n\
resolver = \"2\"\n",
    );
    write(root.join("src/lib.rs"), "");
    write(
        root.join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/api/src/lib.rs"), "");
    for index in 0..13 {
        write(root.join(format!("crates/api/src/dir{index}/mod.rs")), "");
    }

    let results = run_file_tree_pipeline(root);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(results[0].file(), Some("crates/api/Cargo.toml"));
}

#[test]
fn pipeline_excludes_target_files_from_structural_caps() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "");
    for index in 0..13 {
        write(root.join(format!("target/generated/dir{index}/mod.rs")), "");
    }
    for index in 0..21 {
        write(root.join(format!("target/generated/file{index}.rs")), "");
    }
    write(root.join("target/generated/a/b/c/d/e/f/leaf.rs"), "");

    let results = run_file_tree_pipeline(root);

    assert!(results.is_empty(), "{results:#?}");
}
