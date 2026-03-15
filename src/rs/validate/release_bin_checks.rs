use std::path::Path;

use crate::report::types::{CheckResult, Severity};

use super::release_checks::CrateInfo;
use super::release_repo_checks::read_workflow_files;

/// Run binary-specific checks (R-BIN-01 through R-BIN-03).
pub fn check_binary(workspace_root: &Path, krate: &CrateInfo, results: &mut Vec<CheckResult>) {
    check_binary_release_workflow(workspace_root, &krate.name, results);
    check_binary_linux_target(workspace_root, &krate.name, results);
    check_binstall_metadata(krate, results);
}

// --- R-BIN-01: binary release workflow ---

fn check_binary_release_workflow(
    workspace_root: &Path,
    name: &str,
    results: &mut Vec<CheckResult>,
) {
    let workflows = read_workflow_files(workspace_root);
    let has_release_build = workflows
        .iter()
        .any(|(_, content)| content.contains("--release") && content.contains("action-gh-release"));

    if has_release_build {
        results.push(CheckResult {
            id: "R-BIN-01".to_owned(),
            severity: Severity::Info,
            title: format!("{name}: binary release workflow found"),
            message: "Workflow builds --release with action-gh-release".to_owned(),
            file: None,
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "R-BIN-01".to_owned(),
            severity: Severity::Info,
            title: format!("{name}: no binary release workflow"),
            message: "No workflow building --release with action-gh-release".to_owned(),
            file: None,
            line: None,
        });
    }
}

// --- R-BIN-02: linux target ---

fn check_binary_linux_target(workspace_root: &Path, name: &str, results: &mut Vec<CheckResult>) {
    let workflows = read_workflow_files(workspace_root);
    let has_linux = workflows.iter().any(|(_, content)| {
        let lower = content.to_lowercase();
        lower.contains("linux")
            || lower.contains("x86_64")
            || lower.contains("amd64")
            || lower.contains("ubuntu")
    });

    if has_linux {
        results.push(CheckResult {
            id: "R-BIN-02".to_owned(),
            severity: Severity::Info,
            title: format!("{name}: linux target in workflow"),
            message: "Workflow references linux/x86_64/amd64/ubuntu".to_owned(),
            file: None,
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "R-BIN-02".to_owned(),
            severity: Severity::Info,
            title: format!("{name}: no linux target in workflow"),
            message: "No workflow references linux/x86_64/amd64/ubuntu".to_owned(),
            file: None,
            line: None,
        });
    }
}

// --- R-BIN-03: binstall metadata ---

fn check_binstall_metadata(krate: &CrateInfo, results: &mut Vec<CheckResult>) {
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
            message: "[package.metadata.binstall] found in Cargo.toml".to_owned(),
            file: Some(krate.cargo_toml_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "R-BIN-03".to_owned(),
            severity: Severity::Info,
            title: format!("{}: no binstall metadata", krate.name),
            message: "No [package.metadata.binstall] in Cargo.toml".to_owned(),
            file: Some(krate.cargo_toml_path.display().to_string()),
            line: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- R-BIN-01 ---

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn bin01_neg_no_release_workflow() {
        let tmp = std::env::temp_dir().join("guardrail3_bin01_neg");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
        let _ = std::fs::write(tmp.join(".github/workflows/ci.yml"), "name: CI\n");

        let mut r = Vec::new();
        check_binary_release_workflow(&tmp, "mybin", &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-BIN-01" && c.title.contains("no binary release"))
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn bin01_pos_has_release_workflow() {
        let tmp = std::env::temp_dir().join("guardrail3_bin01_pos");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
        let _ = std::fs::write(
            tmp.join(".github/workflows/release.yml"),
            "cargo build --release\nuses: action-gh-release\n",
        );

        let mut r = Vec::new();
        check_binary_release_workflow(&tmp, "mybin", &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-BIN-01" && c.title.contains("binary release workflow found"))
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-BIN-02 ---

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn bin02_neg_no_linux() {
        let tmp = std::env::temp_dir().join("guardrail3_bin02_neg");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
        let _ = std::fs::write(
            tmp.join(".github/workflows/ci.yml"),
            "name: CI\nruns-on: macos-latest\n",
        );

        let mut r = Vec::new();
        check_binary_linux_target(&tmp, "mybin", &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-BIN-02" && c.title.contains("no linux"))
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn bin02_pos_has_linux() {
        let tmp = std::env::temp_dir().join("guardrail3_bin02_pos");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
        let _ = std::fs::write(
            tmp.join(".github/workflows/release.yml"),
            "runs-on: ubuntu-latest\ntarget: x86_64-unknown-linux-gnu\n",
        );

        let mut r = Vec::new();
        check_binary_linux_target(&tmp, "mybin", &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-BIN-02" && c.title.contains("linux target"))
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-BIN-03 ---

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn bin03_neg_no_binstall() {
        let t: toml::Value = "[package]\nname = \"x\"\nversion = \"0.1.0\""
            .parse()
            .expect("parse"); // reason: test
        let krate = CrateInfo {
            name: "x".to_owned(),
            cargo_toml_path: std::path::PathBuf::from("Cargo.toml"),
            dir: std::path::PathBuf::from("."),
            publishable: true,
            is_binary: true,
            table: t,
        };
        let mut r = Vec::new();
        check_binstall_metadata(&krate, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-BIN-03" && c.title.contains("no binstall"))
        );
    }

    #[test]
    #[allow(clippy::expect_used)] // reason: test assertion
    fn bin03_pos_has_binstall() {
        let t: toml::Value =
            "[package]\nname = \"x\"\nversion = \"0.1.0\"\n[package.metadata.binstall]\npkg-url = \"https://example.com\""
                .parse()
                .expect("parse"); // reason: test
        let krate = CrateInfo {
            name: "x".to_owned(),
            cargo_toml_path: std::path::PathBuf::from("Cargo.toml"),
            dir: std::path::PathBuf::from("."),
            publishable: true,
            is_binary: true,
            table: t,
        };
        let mut r = Vec::new();
        check_binstall_metadata(&krate, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-BIN-03" && c.title.contains("binstall metadata present"))
        );
    }
}
