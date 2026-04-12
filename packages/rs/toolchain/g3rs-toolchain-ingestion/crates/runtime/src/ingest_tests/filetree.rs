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

    assert_eq!(
        findings(&results),
        vec![Finding {
            id: "RS-TOOLCHAIN-FILETREE-01".to_owned(),
            severity: G3Severity::Error,
            title: "rust-toolchain.toml missing".to_owned(),
            message: "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.".to_owned(),
            file: Some("rust-toolchain.toml".to_owned()),
            inventory: false,
        }]
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

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-01".to_owned(),
                severity: G3Severity::Error,
                title: "rust-toolchain.toml missing".to_owned(),
                message: "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.".to_owned(),
                file: Some("rust-toolchain.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-04".to_owned(),
                severity: G3Severity::Warn,
                title: "legacy rust-toolchain file present".to_owned(),
                message: "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly.".to_owned(),
                file: Some("rust-toolchain".to_owned()),
                inventory: false,
            },
        ]
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

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-01".to_owned(),
                severity: G3Severity::Info,
                title: "rust-toolchain.toml exists".to_owned(),
                message: "Found rust-toolchain.toml at workspace root.".to_owned(),
                file: Some("rust-toolchain.toml".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-04".to_owned(),
                severity: G3Severity::Error,
                title: "both rust-toolchain files present".to_owned(),
                message: "Remove the legacy `rust-toolchain` file. rustup reads it instead of `rust-toolchain.toml` when both exist, so your modern config is ignored.".to_owned(),
                file: Some("rust-toolchain".to_owned()),
                inventory: false,
            },
        ]
    );
}

#[test]
fn pipeline_reports_root_files_even_when_contents_are_malformed() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rust-toolchain.toml"), "{{{{bad toml");
    write(root.join("rust-toolchain"), "not parsed anyway\n");

    let input = crate::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should ignore malformed file contents");
    let results = g3rs_toolchain_filetree_checks::check(&input);

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-01".to_owned(),
                severity: G3Severity::Info,
                title: "rust-toolchain.toml exists".to_owned(),
                message: "Found rust-toolchain.toml at workspace root.".to_owned(),
                file: Some("rust-toolchain.toml".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-04".to_owned(),
                severity: G3Severity::Error,
                title: "both rust-toolchain files present".to_owned(),
                message: "Remove the legacy `rust-toolchain` file. rustup reads it instead of `rust-toolchain.toml` when both exist, so your modern config is ignored.".to_owned(),
                file: Some("rust-toolchain".to_owned()),
                inventory: false,
            },
        ]
    );
}

#[test]
fn filetree_uses_unreadable_root_entries_without_reading_them() {
    use g3rs_workspace_crawl::{
        G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
        G3RsWorkspacePath,
    };

    let crawl = G3RsWorkspaceCrawl {
        root_abs_path: std::path::PathBuf::from("/synthetic/workspace"),
        entries: vec![
            G3RsWorkspaceEntry {
                path: G3RsWorkspacePath {
                    rel_path: "rust-toolchain.toml".to_owned(),
                    abs_path: std::path::PathBuf::from("/synthetic/workspace/rust-toolchain.toml"),
                },
                kind: G3RsWorkspaceEntryKind::File,
                ignore_state: G3RsWorkspaceIgnoreState::Included,
                readable: false,
            },
            G3RsWorkspaceEntry {
                path: G3RsWorkspacePath {
                    rel_path: "rust-toolchain".to_owned(),
                    abs_path: std::path::PathBuf::from("/synthetic/workspace/rust-toolchain"),
                },
                kind: G3RsWorkspaceEntryKind::File,
                ignore_state: G3RsWorkspaceIgnoreState::Included,
                readable: false,
            },
        ],
    };

    let input = crate::ingest_for_file_tree_checks(&crawl)
        .expect("filetree ingestion should not fail on unreadable entries");
    let results = g3rs_toolchain_filetree_checks::check(&input);

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-01".to_owned(),
                severity: G3Severity::Info,
                title: "rust-toolchain.toml exists".to_owned(),
                message: "Found rust-toolchain.toml at workspace root.".to_owned(),
                file: Some("rust-toolchain.toml".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-04".to_owned(),
                severity: G3Severity::Error,
                title: "both rust-toolchain files present".to_owned(),
                message: "Remove the legacy `rust-toolchain` file. rustup reads it instead of `rust-toolchain.toml` when both exist, so your modern config is ignored.".to_owned(),
                file: Some("rust-toolchain".to_owned()),
                inventory: false,
            },
        ]
    );
}

#[test]
fn filetree_still_reports_root_file_after_delete_after_crawl() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );

    let crawl = crawl(root);
    fs::remove_file(root.join("rust-toolchain.toml"))
        .expect("should delete rust-toolchain.toml after crawl");

    let input = crate::ingest_for_file_tree_checks(&crawl)
        .expect("filetree ingestion should not reread the filesystem");
    let results = g3rs_toolchain_filetree_checks::check(&input);

    assert_eq!(
        findings(&results),
        vec![Finding {
            id: "RS-TOOLCHAIN-FILETREE-01".to_owned(),
            severity: G3Severity::Info,
            title: "rust-toolchain.toml exists".to_owned(),
            message: "Found rust-toolchain.toml at workspace root.".to_owned(),
            file: Some("rust-toolchain.toml".to_owned()),
            inventory: true,
        }]
    );
}
