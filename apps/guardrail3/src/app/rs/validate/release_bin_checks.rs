use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

use super::release_checks::CrateInfo;
use super::release_repo_checks::read_workflow_files;
use crate::ports::outbound::FileSystem;
/// Run binary-specific checks (R-BIN-01 through R-BIN-03).
pub fn check_binary(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    krate: &CrateInfo,
    results: &mut Vec<CheckResult>,
) {
    check_binary_release_workflow(fs, workspace_root, &krate.name, results);
    check_binary_linux_target(fs, workspace_root, &krate.name, results);
    check_binstall_metadata(krate, results);
}

// --- R-BIN-01: binary release workflow ---

pub fn check_binary_release_workflow(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    name: &str,
    results: &mut Vec<CheckResult>,
) {
    let workflows = read_workflow_files(fs, workspace_root);
    let has_release_build = workflows
        .iter()
        .any(|(_, content)| content.contains("--release") && content.contains("action-gh-release"));

    if has_release_build {
        results.push(
            CheckResult {
                id: "R-BIN-01".to_owned(),
                severity: Severity::Info,
                title: format!("{name}: binary release workflow found"),
                message: "Workflow builds --release with action-gh-release".to_owned(),
                file: None,
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "R-BIN-01".to_owned(),
            severity: Severity::Info,
            title: format!("{name}: no binary release workflow"),
            message: "No workflow building --release with action-gh-release".to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

// --- R-BIN-02: linux target ---

pub fn check_binary_linux_target(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    name: &str,
    results: &mut Vec<CheckResult>,
) {
    let workflows = read_workflow_files(fs, workspace_root);
    let has_linux = workflows.iter().any(|(_, content)| {
        let lower = content.to_lowercase();
        lower.contains("linux")
            || lower.contains("x86_64")
            || lower.contains("amd64")
            || lower.contains("ubuntu")
    });

    if has_linux {
        results.push(
            CheckResult {
                id: "R-BIN-02".to_owned(),
                severity: Severity::Info,
                title: format!("{name}: linux target in workflow"),
                message: "Workflow references linux/x86_64/amd64/ubuntu".to_owned(),
                file: None,
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(
            CheckResult {
                id: "R-BIN-02".to_owned(),
                severity: Severity::Info,
                title: format!("{name}: no linux target in workflow"),
                message: "No workflow references linux/x86_64/amd64/ubuntu".to_owned(),
                file: None,
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

// --- R-BIN-03: binstall metadata ---

pub fn check_binstall_metadata(krate: &CrateInfo, results: &mut Vec<CheckResult>) {
    let has_binstall = krate
        .table
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("binstall"))
        .is_some();

    if has_binstall {
        results.push(CheckResult {
            id: "R-BIN-03".to_owned(),
            severity: Severity::Info,
            title: format!("{}: binstall metadata present", krate.name),
            message: "[package.metadata.binstall] found in Cargo.toml. cargo-binstall can download pre-built binaries instead of compiling from source. No action needed.".to_owned(),
            file: Some(krate.cargo_toml_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "R-BIN-03".to_owned(),
            severity: Severity::Warn,
            title: format!("{}: no binstall metadata", krate.name),
            message: "No [package.metadata.binstall] in Cargo.toml. Without this, `cargo binstall` cannot find pre-built binaries and falls back to compiling from source (slow). Add a `[package.metadata.binstall]` section with `pkg-url` and `bin-dir` fields.".to_owned(),
            file: Some(krate.cargo_toml_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}
