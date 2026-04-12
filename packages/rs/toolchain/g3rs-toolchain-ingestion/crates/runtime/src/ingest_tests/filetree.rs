use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed in test fixture setup");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .expect("should create parent directories for test fixture files");
    }
    fs::write(path, content).expect("should write test fixture file to disk");
}

fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed on valid test workspace")
}

#[test]
fn filetree_ingests_root_toolchain_paths() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write(root.join("rust-toolchain"), "stable\n");

    let output = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed");

    assert_eq!(
        output.toolchain_toml_rel_path.as_deref(),
        Some("rust-toolchain.toml")
    );
    assert_eq!(
        output.legacy_toolchain_rel_path.as_deref(),
        Some("rust-toolchain")
    );
}

#[test]
fn filetree_ignores_descendant_toolchain_files() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("nested/rust-toolchain.toml"),
        "[toolchain]\nchannel = \"beta\"\n",
    );
    write(root.join("nested/rust-toolchain"), "stable\n");

    let output = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed");

    assert_eq!(output.toolchain_toml_rel_path, None);
    assert_eq!(output.legacy_toolchain_rel_path, None);
}

#[test]
fn pipeline_reports_missing_modern_toolchain_file() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed without toolchain files");
    let results = g3rs_toolchain_filetree_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TOOLCHAIN-FILETREE-01"
                && result.title() == "rust-toolchain.toml missing"
                && result.file() == Some("rust-toolchain.toml")
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_legacy_only_as_warn_plus_missing_modern() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rust-toolchain"), "stable\n");

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed with legacy toolchain only");
    let results = g3rs_toolchain_filetree_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TOOLCHAIN-FILETREE-01"
                && result.title() == "rust-toolchain.toml missing"
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TOOLCHAIN-FILETREE-04"
                && result.title() == "legacy rust-toolchain file present"
                && result.file() == Some("rust-toolchain")
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_both_toolchain_files_conflict() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write(root.join("rust-toolchain"), "stable\n");

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed when both files exist");
    let results = g3rs_toolchain_filetree_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TOOLCHAIN-FILETREE-01"
                && result.title() == "rust-toolchain.toml exists"
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TOOLCHAIN-FILETREE-04"
                && result.title() == "both rust-toolchain files present"
                && result.file() == Some("rust-toolchain")
        }),
        "{results:#?}"
    );
}
