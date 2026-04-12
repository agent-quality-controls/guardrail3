use std::fs;
use std::path::Path;
use std::process::Command;

use guardrail3_check_types::{G3CheckResult, G3Severity};
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Finding {
    id: String,
    severity: G3Severity,
    title: String,
    message: String,
    file: Option<String>,
    inventory: bool,
}

fn findings(results: &[G3CheckResult]) -> Vec<Finding> {
    let mut findings = results
        .iter()
        .map(|result| Finding {
            id: result.id().to_owned(),
            severity: result.severity(),
            title: result.title().to_owned(),
            message: result.message().to_owned(),
            file: result.file().map(str::to_owned),
            inventory: result.inventory(),
        })
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| {
        (
            left.id.as_str(),
            format!("{:?}", left.severity),
            left.title.as_str(),
            left.message.as_str(),
            left.file.as_deref(),
            left.inventory,
        )
            .cmp(&(
                right.id.as_str(),
                format!("{:?}", right.severity),
                right.title.as_str(),
                right.message.as_str(),
                right.file.as_deref(),
                right.inventory,
            ))
    });
    findings
}

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

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed");

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
        vec!["".to_owned(), "crates/api".to_owned()]
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

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed");

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

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed");

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

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed");

    assert!(input.nested_config_files.is_empty());
    assert!(input.dual_conflict_dirs.is_empty());
}

#[test]
fn pipeline_reports_missing_root_config() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed without config files");
    let results = g3rs_fmt_filetree_checks::check(&input);

    assert_eq!(
        findings(&results),
        vec![Finding {
            id: "RS-FMT-FILETREE-01".to_owned(),
            severity: G3Severity::Error,
            title: "rustfmt config missing".to_owned(),
            message: "Expected `rustfmt.toml` at workspace root. Create one with the required formatting settings.".to_owned(),
            file: Some("rustfmt.toml".to_owned()),
            inventory: false,
        }]
    );
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

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed");
    let results = g3rs_fmt_filetree_checks::check(&input);

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-FMT-FILETREE-05".to_owned(),
                severity: G3Severity::Error,
                title: "Illegal nested rustfmt config".to_owned(),
                message: "`.rustfmt.toml` below repository root is forbidden; rustfmt policy is root-only. Delete this file and ensure all formatting settings are in the root `rustfmt.toml`.".to_owned(),
                file: Some("crates/api/.rustfmt.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-FMT-FILETREE-05".to_owned(),
                severity: G3Severity::Error,
                title: "Illegal nested rustfmt config".to_owned(),
                message: "`rustfmt.toml` below repository root is forbidden; rustfmt policy is root-only. Delete this file and ensure all formatting settings are in the root `rustfmt.toml`.".to_owned(),
                file: Some("crates/api/rustfmt.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-FMT-FILETREE-08".to_owned(),
                severity: G3Severity::Warn,
                title: "Conflicting rustfmt config files".to_owned(),
                message: "Both `rustfmt.toml` and `.rustfmt.toml` exist in `crates/api`. Delete `.rustfmt.toml` and keep `rustfmt.toml`.".to_owned(),
                file: Some("crates/api/rustfmt.toml".to_owned()),
                inventory: false,
            },
        ]
    );
}
