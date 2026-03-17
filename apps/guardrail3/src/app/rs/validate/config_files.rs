use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

use super::rustfmt_check;
use super::toolchain_check;
use super::workspace_metadata;
use crate::ports::outbound::FileSystem;

type ExpectedInt<'a> = (&'a str, i64);
pub fn check(fs: &dyn FileSystem, workspace_root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // R1: clippy.toml exists at workspace root
    let clippy_path = workspace_root.join("clippy.toml");
    if clippy_path.exists() {
        results.push(CheckResult {
            id: "R1".to_owned(),
            severity: Severity::Info,
            title: "clippy.toml exists".to_owned(),
            message: "clippy.toml found at workspace root. This file configures clippy lint thresholds and method/type bans. No action needed.".to_owned(),
            file: Some(clippy_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());

        // R3: Thresholds
        check_clippy_thresholds(fs, &clippy_path, &mut results);
    } else {
        results.push(CheckResult {
            id: "R1".to_owned(),
            severity: Severity::Error,
            title: "clippy.toml missing".to_owned(),
            message: "No clippy.toml at workspace root. Without clippy.toml, method/type bans and lint thresholds are not enforced. Run `guardrail3 rs init` or `guardrail3 generate` to create it.".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // R21: rustfmt.toml exists
    let rustfmt_path = workspace_root.join("rustfmt.toml");
    if rustfmt_path.exists() {
        results.push(CheckResult {
            id: "R21".to_owned(),
            severity: Severity::Info,
            title: "rustfmt.toml exists".to_owned(),
            message: "rustfmt.toml found at workspace root. This file enforces consistent code formatting (max_width, edition, import ordering). No action needed.".to_owned(),
            file: Some(rustfmt_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());

        // R22: rustfmt.toml settings differ (Warn)
        // R23: rustfmt.toml extra settings (Info)
        rustfmt_check::check_rustfmt_settings(fs, &rustfmt_path, &mut results);
    } else {
        results.push(CheckResult {
            id: "R21".to_owned(),
            severity: Severity::Error,
            title: "rustfmt.toml missing".to_owned(),
            message: "No rustfmt.toml at workspace root. Without it, `cargo fmt` uses defaults which may not match project standards. Run `guardrail3 generate` to create it.".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // R24: rust-toolchain.toml exists — Error if missing
    let toolchain_path = workspace_root.join("rust-toolchain.toml");
    if toolchain_path.exists() {
        results.push(CheckResult {
            id: "R24".to_owned(),
            severity: Severity::Info,
            title: "rust-toolchain.toml exists".to_owned(),
            message: "rust-toolchain.toml found at workspace root. This pins the Rust toolchain channel and required components (clippy, rustfmt). No action needed.".to_owned(),
            file: Some(toolchain_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());

        // R25: rust-toolchain.toml settings
        toolchain_check::check_toolchain_settings(fs, &toolchain_path, &mut results);
    } else {
        results.push(CheckResult {
            id: "R24".to_owned(),
            severity: Severity::Error,
            title: "rust-toolchain.toml missing".to_owned(),
            message: "No rust-toolchain.toml at workspace root. Without it, each developer may use a different Rust version, causing inconsistent builds and CI failures. Create `rust-toolchain.toml` with `channel = \"stable\"` and `components = [\"clippy\", \"rustfmt\"]`.".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // R55-R57: workspace metadata & release profile
    workspace_metadata::check_workspace_metadata(fs, workspace_root, &mut results);

    results
}

pub fn check_per_crate_clippy(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    member_dirs: &[String],
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    for member in member_dirs {
        let crate_dir = workspace_root.join(member);
        let crate_clippy = crate_dir.join("clippy.toml");
        if crate_clippy.exists() {
            results.push(
                CheckResult {
                    id: "R2".to_owned(),
                    severity: Severity::Info,
                    title: "Per-crate clippy.toml".to_owned(),
                    message: format!("Found for {member}"),
                    file: Some(crate_clippy.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );

            // Check per-crate clippy.toml content for global-state type bans
            check_per_crate_clippy_content(fs, &crate_clippy, member, &mut results);
        } else {
            results.push(CheckResult {
                id: "R2".to_owned(),
                severity: Severity::Warn,
                title: "Per-crate clippy.toml missing".to_owned(),
                message: format!("No clippy.toml for {member}"),
                file: Some(crate_dir.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    results
}

fn check_per_crate_clippy_content(
    fs: &dyn FileSystem,
    path: &Path,
    member: &str,
    results: &mut Vec<CheckResult>,
) {
    let Some(content) = fs.read_file(path) else {
        return;
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let global_state_types = ["LazyLock", "OnceLock", "once_cell"];

    if let Some(types_arr) = table.get("disallowed-types").and_then(|v| v.as_array()) {
        let type_paths: Vec<String> = types_arr
            .iter()
            .filter_map(|v| {
                v.get("path")
                    .and_then(|p| p.as_str())
                    .or_else(|| v.as_str())
                    .map(std::borrow::ToOwned::to_owned)
            })
            .collect();

        let mut found_global_bans = Vec::new();
        for gs_type in &global_state_types {
            for tp in &type_paths {
                if tp.contains(gs_type) {
                    found_global_bans.push(tp.clone());
                }
            }
        }

        if found_global_bans.is_empty() {
            results.push(
                CheckResult {
                    id: "R2".to_owned(),
                    severity: Severity::Info,
                    title: format!("{member}: no global-state type bans"),
                    message: "No LazyLock/OnceLock/once_cell bans in per-crate clippy.toml"
                        .to_owned(),
                    file: Some(path.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        } else {
            results.push(
                CheckResult {
                    id: "R2".to_owned(),
                    severity: Severity::Info,
                    title: format!("{member}: global-state type bans present"),
                    message: format!("Bans: {}", found_global_bans.join(", ")),
                    file: Some(path.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        }
    }
}

fn check_clippy_thresholds(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let content = match fs.read_file_err(path) {
        Ok(content) => content,
        Err(e) => {
            results.push(CheckResult {
                id: "R3".to_owned(),
                severity: Severity::Error,
                title: "clippy.toml unreadable".to_owned(),
                message: format!("Failed to read: {e}"),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "R3".to_owned(),
                severity: Severity::Error,
                title: "clippy.toml parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
    };

    let expected: &[ExpectedInt<'_>] = &[
        ("too-many-lines-threshold", 75),
        ("cognitive-complexity-threshold", 15),
        ("too-many-arguments-threshold", 7),
        ("type-complexity-threshold", 75),
        ("max-struct-bools", 3),
    ];

    for (key, expected_val) in expected {
        check_clippy_int_threshold(&table, key, *expected_val, path, results);
    }
}

fn check_clippy_int_threshold(
    table: &toml::Value,
    key: &str,
    expected_val: i64,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
    match table.get(key) {
        Some(toml::Value::Integer(v)) if *v == expected_val => {
            results.push(
                CheckResult {
                    id: "R3".to_owned(),
                    severity: Severity::Info,
                    title: format!("{key} correct"),
                    message: format!("{key} = {v}"),
                    file: Some(path.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        }
        Some(toml::Value::Integer(v)) => {
            results.push(CheckResult {
                id: "R3".to_owned(),
                severity: Severity::Error,
                title: format!("{key} wrong value"),
                message: format!("Expected {expected_val}, got {v}"),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        Some(_) => {
            results.push(CheckResult {
                id: "R3".to_owned(),
                severity: Severity::Error,
                title: format!("{key} wrong type"),
                message: format!("Expected integer {expected_val}"),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R3".to_owned(),
                severity: Severity::Error,
                title: format!("{key} missing"),
                message: format!("Expected {key} = {expected_val}"),
                file: Some(path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}
