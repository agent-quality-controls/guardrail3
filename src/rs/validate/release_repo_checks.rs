use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use crate::report::types::{CheckResult, Severity};

/// Run all repo-level release checks (R-REL-01 through R-REL-08).
pub fn check_repo_level(
    workspace_root: &Path,
    publishable_names: &BTreeSet<String>,
    results: &mut Vec<CheckResult>,
) {
    check_license_file(workspace_root, results);
    check_release_plz_toml(workspace_root, publishable_names, results);
    check_cliff_toml(workspace_root, results);
    check_workflow_contains(
        workspace_root,
        "release-plz",
        "R-REL-05",
        "Release workflow found",
        "A workflow references release-plz",
        "No release workflow",
        "No .github/workflows/*.yml containing \"release-plz\"",
        results,
    );
    check_workflow_contains(
        workspace_root,
        "cargo publish --dry-run",
        "R-REL-06",
        "Publish dry-run in CI",
        "A workflow contains \"cargo publish --dry-run\"",
        "No publish dry-run in CI",
        "No workflow with \"cargo publish --dry-run\"",
        results,
    );
    check_workflow_contains(
        workspace_root,
        "CARGO_REGISTRY_TOKEN",
        "R-REL-07",
        "CARGO_REGISTRY_TOKEN referenced",
        "A workflow references CARGO_REGISTRY_TOKEN",
        "No CARGO_REGISTRY_TOKEN in workflows",
        "No workflow references CARGO_REGISTRY_TOKEN",
        results,
    );
    check_semver_checks_installed(results);
}

// --- R-REL-01: LICENSE file at repo root ---

fn check_license_file(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    let license_names = ["LICENSE", "LICENSE-MIT", "LICENSE-APACHE", "LICENSE.md"];

    let found = license_names
        .iter()
        .any(|name| workspace_root.join(name).exists());

    if found {
        results.push(CheckResult {
            id: "R-REL-01".to_owned(),
            severity: Severity::Info,
            title: "LICENSE file exists".to_owned(),
            message: "Found at repo root".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "R-REL-01".to_owned(),
            severity: Severity::Error,
            title: "LICENSE file missing".to_owned(),
            message: "No LICENSE, LICENSE-MIT, LICENSE-APACHE, or LICENSE.md at repo root"
                .to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
    }
}

// --- R-REL-02 + R-REL-03: release-plz.toml ---

#[allow(clippy::too_many_lines)] // reason: R-REL-02 + R-REL-03 combined with early returns
fn check_release_plz_toml(
    workspace_root: &Path,
    publishable_names: &BTreeSet<String>,
    results: &mut Vec<CheckResult>,
) {
    let plz_path = workspace_root.join("release-plz.toml");
    if !plz_path.exists() {
        results.push(CheckResult {
            id: "R-REL-02".to_owned(),
            severity: Severity::Warn,
            title: "release-plz.toml missing".to_owned(),
            message: "No release-plz.toml at repo root".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
        return;
    }

    results.push(CheckResult {
        id: "R-REL-02".to_owned(),
        severity: Severity::Info,
        title: "release-plz.toml exists".to_owned(),
        message: "Found at repo root".to_owned(),
        file: Some(plz_path.display().to_string()),
        line: None,
    });

    // R-REL-03: validate content
    let Some(content) = crate::fs::read_file(&plz_path) else {
        results.push(CheckResult {
            id: "R-REL-03".to_owned(),
            severity: Severity::Warn,
            title: "release-plz.toml unreadable".to_owned(),
            message: "Could not read file".to_owned(),
            file: Some(plz_path.display().to_string()),
            line: None,
        });
        return;
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "R-REL-03".to_owned(),
                severity: Severity::Warn,
                title: "release-plz.toml invalid TOML".to_owned(),
                message: format!("Parse error: {e}"),
                file: Some(plz_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    // Check [workspace] section
    if table.get("workspace").is_none() {
        results.push(CheckResult {
            id: "R-REL-03".to_owned(),
            severity: Severity::Warn,
            title: "release-plz.toml missing [workspace]".to_owned(),
            message: "No [workspace] section found".to_owned(),
            file: Some(plz_path.display().to_string()),
            line: None,
        });
        return;
    }

    // Check [[package]] entries cover all publishable crates
    let configured_names: BTreeSet<String> = table
        .get("package")
        .and_then(|p| p.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|entry| {
                    entry
                        .get("name")
                        .and_then(|n| n.as_str())
                        .map(std::borrow::ToOwned::to_owned)
                })
                .collect()
        })
        .unwrap_or_default();

    let missing: BTreeSet<_> = publishable_names.difference(&configured_names).collect();

    if missing.is_empty() {
        results.push(CheckResult {
            id: "R-REL-03".to_owned(),
            severity: Severity::Info,
            title: "release-plz.toml covers all crates".to_owned(),
            message: "All publishable crates have [[package]] entries".to_owned(),
            file: Some(plz_path.display().to_string()),
            line: None,
        });
    } else {
        for name in &missing {
            results.push(CheckResult {
                id: "R-REL-03".to_owned(),
                severity: Severity::Warn,
                title: format!("release-plz.toml missing package \"{name}\""),
                message: format!("Publishable crate \"{name}\" has no [[package]] entry"),
                file: Some(plz_path.display().to_string()),
                line: None,
            });
        }
    }
}

// --- R-REL-04: cliff.toml ---

fn check_cliff_toml(workspace_root: &Path, results: &mut Vec<CheckResult>) {
    super::code_quality_checks::check_file_exists_at_root(
        workspace_root,
        "cliff.toml",
        "R-REL-04",
        "cliff.toml exists",
        "cliff.toml missing",
        results,
    );
}

// --- R-REL-05 through R-REL-07: workflow checks ---

/// (filename, content) pairs from workflow YAML files.
pub type WorkflowFiles = Vec<(String, String)>;

/// Read all YAML files from .github/workflows/ and return their contents.
pub fn read_workflow_files(workspace_root: &Path) -> WorkflowFiles {
    let workflows_dir = workspace_root.join(".github").join("workflows");
    if !workflows_dir.exists() {
        return Vec::new();
    }

    let mut files = Vec::new();
    for entry in crate::fs::list_dir(&workflows_dir) {
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_owned();
        let is_yaml = Path::new(&name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("yml") || ext.eq_ignore_ascii_case("yaml"));
        if is_yaml {
            if let Some(content) = crate::fs::read_file(&path) {
                files.push((name, content));
            }
        }
    }
    files
}

/// Check if any workflow file contains a pattern, emitting an appropriate result.
#[allow(clippy::too_many_arguments)] // reason: dedup helper — all args are distinct semantic values
fn check_workflow_contains(
    workspace_root: &Path,
    pattern: &str,
    check_id: &str,
    found_title: &str,
    found_msg: &str,
    missing_title: &str,
    missing_msg: &str,
    results: &mut Vec<CheckResult>,
) {
    let workflows = read_workflow_files(workspace_root);
    let found = workflows
        .iter()
        .any(|(_, content)| content.contains(pattern));

    if found {
        results.push(CheckResult {
            id: check_id.to_owned(),
            severity: Severity::Info,
            title: found_title.to_owned(),
            message: found_msg.to_owned(),
            file: None,
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: check_id.to_owned(),
            severity: Severity::Warn,
            title: missing_title.to_owned(),
            message: missing_msg.to_owned(),
            file: None,
            line: None,
        });
    }
}

// --- R-REL-08: cargo-semver-checks installed ---

/// Check if a CLI tool is installed on PATH via `which`.
pub fn check_tool_installed(
    tool_name: &str,
    check_id: &str,
    install_cmd: &str,
    results: &mut Vec<CheckResult>,
) {
    #[allow(clippy::disallowed_methods)] // reason: tool installation check requires which command
    let cmd_result = Command::new("which").arg(tool_name).output();

    match cmd_result {
        Ok(output) if output.status.success() => {
            results.push(CheckResult {
                id: check_id.to_owned(),
                severity: Severity::Info,
                title: format!("{tool_name} installed"),
                message: format!("{tool_name} found on PATH"),
                file: None,
                line: None,
            });
        }
        _ => {
            results.push(CheckResult {
                id: check_id.to_owned(),
                severity: Severity::Warn,
                title: format!("{tool_name} not installed"),
                message: format!("Install with: {install_cmd}"),
                file: None,
                line: None,
            });
        }
    }
}

fn check_semver_checks_installed(results: &mut Vec<CheckResult>) {
    check_tool_installed(
        "cargo-semver-checks",
        "R-REL-08",
        "cargo install cargo-semver-checks",
        results,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- R-REL-01 ---

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel01_neg_no_license() {
        let tmp = std::env::temp_dir().join("guardrail3_rel01_neg");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);

        let mut r = Vec::new();
        check_license_file(&tmp, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-01" && c.severity == Severity::Error)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel01_pos_license_exists() {
        let tmp = std::env::temp_dir().join("guardrail3_rel01_pos");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(tmp.join("LICENSE"), "MIT License");

        let mut r = Vec::new();
        check_license_file(&tmp, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-01" && c.severity == Severity::Info)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-REL-02 ---

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel02_neg_no_release_plz() {
        let tmp = std::env::temp_dir().join("guardrail3_rel02_neg");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);

        let mut r = Vec::new();
        let names = BTreeSet::new();
        check_release_plz_toml(&tmp, &names, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-02" && c.severity == Severity::Warn)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel02_pos_release_plz_exists() {
        let tmp = std::env::temp_dir().join("guardrail3_rel02_pos");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(
            tmp.join("release-plz.toml"),
            "[workspace]\n[[package]]\nname = \"x\"\n",
        );

        let mut r = Vec::new();
        let names = BTreeSet::from(["x".to_owned()]);
        check_release_plz_toml(&tmp, &names, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-02" && c.severity == Severity::Info)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-REL-03 ---

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel03_neg_invalid_toml() {
        let tmp = std::env::temp_dir().join("guardrail3_rel03_neg");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(tmp.join("release-plz.toml"), "not valid toml [[[");

        let mut r = Vec::new();
        let names = BTreeSet::new();
        check_release_plz_toml(&tmp, &names, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-03" && c.severity == Severity::Warn)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel03_pos_valid_covers_crates() {
        let tmp = std::env::temp_dir().join("guardrail3_rel03_pos");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(
            tmp.join("release-plz.toml"),
            "[workspace]\n\n[[package]]\nname = \"a\"\n\n[[package]]\nname = \"b\"\n",
        );

        let mut r = Vec::new();
        let names = BTreeSet::from(["a".to_owned(), "b".to_owned()]);
        check_release_plz_toml(&tmp, &names, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-03" && c.severity == Severity::Info)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-REL-04 ---

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel04_neg_no_cliff() {
        let tmp = std::env::temp_dir().join("guardrail3_rel04_neg");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);

        let mut r = Vec::new();
        check_cliff_toml(&tmp, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-04" && c.severity == Severity::Warn)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel04_pos_cliff_exists() {
        let tmp = std::env::temp_dir().join("guardrail3_rel04_pos");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(tmp.join("cliff.toml"), "[changelog]\nheader = \"\"");

        let mut r = Vec::new();
        check_cliff_toml(&tmp, &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-04" && c.severity == Severity::Info)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-REL-05 ---

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel05_neg_no_release_workflow() {
        let tmp = std::env::temp_dir().join("guardrail3_rel05_neg");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
        let _ = std::fs::write(tmp.join(".github/workflows/ci.yml"), "name: CI\n");

        let mut r = Vec::new();
        check_workflow_contains(&tmp, "release-plz", "R-REL-05", "", "", "", "", &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-05" && c.severity == Severity::Warn)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel05_pos_has_release_workflow() {
        let tmp = std::env::temp_dir().join("guardrail3_rel05_pos");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
        let _ = std::fs::write(
            tmp.join(".github/workflows/release.yml"),
            "name: Release\nuses: release-plz/action@v0.5\n",
        );

        let mut r = Vec::new();
        check_workflow_contains(&tmp, "release-plz", "R-REL-05", "", "", "", "", &mut r);
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-05" && c.severity == Severity::Info)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-REL-06 ---

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel06_neg_no_dry_run() {
        let tmp = std::env::temp_dir().join("guardrail3_rel06_neg");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
        let _ = std::fs::write(
            tmp.join(".github/workflows/ci.yml"),
            "name: CI\ncargo test\n",
        );

        let mut r = Vec::new();
        check_workflow_contains(
            &tmp,
            "cargo publish --dry-run",
            "R-REL-06",
            "",
            "",
            "",
            "",
            &mut r,
        );
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-06" && c.severity == Severity::Warn)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel06_pos_has_dry_run() {
        let tmp = std::env::temp_dir().join("guardrail3_rel06_pos");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
        let _ = std::fs::write(
            tmp.join(".github/workflows/ci.yml"),
            "name: CI\nrun: cargo publish --dry-run\n",
        );

        let mut r = Vec::new();
        check_workflow_contains(
            &tmp,
            "cargo publish --dry-run",
            "R-REL-06",
            "",
            "",
            "",
            "",
            &mut r,
        );
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-06" && c.severity == Severity::Info)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-REL-07 ---

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel07_neg_no_token() {
        let tmp = std::env::temp_dir().join("guardrail3_rel07_neg");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
        let _ = std::fs::write(tmp.join(".github/workflows/ci.yml"), "name: CI\n");

        let mut r = Vec::new();
        check_workflow_contains(
            &tmp,
            "CARGO_REGISTRY_TOKEN",
            "R-REL-07",
            "",
            "",
            "",
            "",
            &mut r,
        );
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-07" && c.severity == Severity::Warn)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    #[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
    fn rel07_pos_has_token() {
        let tmp = std::env::temp_dir().join("guardrail3_rel07_pos");
        let _ = std::fs::remove_dir_all(&tmp);
        let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
        let _ = std::fs::write(
            tmp.join(".github/workflows/release.yml"),
            "env:\n  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}\n",
        );

        let mut r = Vec::new();
        check_workflow_contains(
            &tmp,
            "CARGO_REGISTRY_TOKEN",
            "R-REL-07",
            "",
            "",
            "",
            "",
            &mut r,
        );
        assert!(
            r.iter()
                .any(|c| c.id == "R-REL-07" && c.severity == Severity::Info)
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // --- R-REL-08: cargo-semver-checks installed (runtime check, test structure only) ---

    #[test]
    fn rel08_emits_result() {
        let mut r = Vec::new();
        check_semver_checks_installed(&mut r);
        // Should emit exactly one result with id R-REL-08
        assert_eq!(r.len(), 1);
        assert_eq!(r.first().map(|c| c.id.as_str()), Some("R-REL-08"));
    }
}
