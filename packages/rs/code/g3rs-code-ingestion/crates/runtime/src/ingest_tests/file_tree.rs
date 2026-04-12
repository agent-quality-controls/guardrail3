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

fn run_file_tree_input(root: &Path) -> g3rs_code_ingestion_types::G3RsCodeFileTreeChecksInput {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingestion should succeed")
}

fn assert_structural_cap_result(
    result: &G3CheckResult,
    file: &str,
    message: &str,
) {
    assert_eq!(result.id(), "RS-CODE-FILETREE-35");
    assert_eq!(result.severity(), guardrail3_check_types::G3Severity::Error);
    assert_eq!(result.title(), "crate source tree exceeds structural caps");
    assert_eq!(result.file(), Some(file));
    assert_eq!(result.line(), None);
    assert!(!result.inventory(), "{result:#?}");
    assert_eq!(result.message(), message);
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
    assert_structural_cap_result(
        &results[0],
        "Cargo.toml",
        "Rust root `` exceeds structural caps: module depth 8 > 6, sibling source directories 14 > 12, sibling .rs files 22 > 20. Restructure the crate into smaller modules or sub-crates.",
    );
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
    assert_structural_cap_result(
        &results[0],
        "crates/api/Cargo.toml",
        "Rust root `crates/api` exceeds structural caps: sibling source directories 13 > 12. Restructure the crate into smaller modules or sub-crates.",
    );
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
    assert_structural_cap_result(
        &results[0],
        "crates/api/Cargo.toml",
        "Rust root `crates/api` exceeds structural caps: sibling source directories 13 > 12. Restructure the crate into smaller modules or sub-crates.",
    );
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

#[test]
fn pipeline_excludes_fixture_files_from_structural_caps() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "");
    for index in 0..13 {
        write(root.join(format!("tests/fixtures/generated/dir{index}/mod.rs")), "");
    }
    for index in 0..21 {
        write(root.join(format!("tests/fixtures/generated/file{index}.rs")), "");
    }
    write(root.join("tests/fixtures/generated/a/b/c/d/e/f/leaf.rs"), "");

    let results = run_file_tree_pipeline(root);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn pipeline_stays_quiet_for_member_at_exact_thresholds() {
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
    for index in 0..11 {
        write(root.join(format!("crates/api/src/dir{index}/mod.rs")), "");
    }
    for index in 0..19 {
        write(root.join(format!("crates/api/src/file{index}.rs")), "");
    }
    write(root.join("crates/api/src/a/b/c/d/e/mod.rs"), "");

    let results = run_file_tree_pipeline(root);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn pipeline_reports_root_and_member_exactly_once_each() {
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
    for index in 0..13 {
        write(root.join(format!("src/rootdir{index}/mod.rs")), "");
    }

    write(
        root.join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/api/src/lib.rs"), "");
    for index in 0..21 {
        write(root.join(format!("crates/api/src/file{index}.rs")), "");
    }

    let mut results = run_file_tree_pipeline(root);
    results.sort_by(|left, right| left.file().cmp(&right.file()));

    assert_eq!(results.len(), 2, "{results:#?}");
    assert_structural_cap_result(
        &results[0],
        "Cargo.toml",
        "Rust root `` exceeds structural caps: sibling source directories 13 > 12. Restructure the crate into smaller modules or sub-crates.",
    );
    assert_structural_cap_result(
        &results[1],
        "crates/api/Cargo.toml",
        "Rust root `crates/api` exceeds structural caps: sibling .rs files 22 > 20. Restructure the crate into smaller modules or sub-crates.",
    );
}

#[test]
fn pipeline_stays_quiet_when_root_and_member_are_both_at_thresholds() {
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
    for index in 0..11 {
        write(root.join(format!("src/rootdir{index}/mod.rs")), "");
    }
    for index in 0..19 {
        write(root.join(format!("src/rootfile{index}.rs")), "");
    }
    write(root.join("src/a/b/c/d/e/mod.rs"), "");

    write(
        root.join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/api/src/lib.rs"), "");
    for index in 0..11 {
        write(root.join(format!("crates/api/src/dir{index}/mod.rs")), "");
    }
    for index in 0..19 {
        write(root.join(format!("crates/api/src/file{index}.rs")), "");
    }
    write(root.join("crates/api/src/a/b/c/d/e/mod.rs"), "");

    let results = run_file_tree_pipeline(root);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn file_tree_input_supports_glob_members_and_excludes() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "\
[workspace]\n\
members = [\"crates/*\"]\n\
exclude = [\"crates/skip\"]\n\
resolver = \"2\"\n",
    );
    write(
        root.join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/api/src/lib.rs"), "");
    write(
        root.join("crates/skip/Cargo.toml"),
        "[package]\nname = \"skip\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/skip/src/lib.rs"), "");

    let input = run_file_tree_input(root);
    let roots = input
        .roots
        .into_iter()
        .map(|root| root.cargo_rel_path)
        .collect::<Vec<_>>();

    assert_eq!(roots, vec!["crates/api/Cargo.toml".to_owned()]);
}

#[test]
fn file_tree_input_supports_owned_roots_with_no_rust_files() {
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

    let input = run_file_tree_input(root);

    assert_eq!(input.roots.len(), 1, "{input:#?}");
    assert_eq!(input.roots[0].cargo_rel_path, "crates/api/Cargo.toml");
    assert_eq!(input.roots[0].max_module_depth, 0);
    assert_eq!(input.roots[0].max_sibling_dirs, 0);
    assert_eq!(input.roots[0].max_sibling_rs_files, 0);
}
