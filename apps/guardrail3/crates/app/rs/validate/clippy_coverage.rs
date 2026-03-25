use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::domain::report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

pub use guardrail3_app_rs_family_clippy::{EXPECTED_METHOD_BANS, EXPECTED_TYPE_BANS};

pub fn check(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    _profile: Option<&str>,
    clippy_tomls: &[PathBuf],
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // Find the root clippy.toml from crawler data
    let root_clippy = clippy_tomls
        .iter()
        .find(|p| p.parent() == Some(workspace_root));

    let Some(clippy_path) = root_clippy else {
        results.push(CheckResult {
            id: "R4".to_owned(),
            severity: Severity::Error,
            title: "Cannot check clippy bans".to_owned(),
            message: "clippy.toml not found".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
            inventory: false,
        });
        return results;
    };

    let content = match fs.read_file_err(clippy_path) {
        Ok(content) => content,
        Err(e) => {
            results.push(CheckResult {
                id: "R4".to_owned(),
                severity: Severity::Error,
                title: "clippy.toml unreadable".to_owned(),
                message: format!("Failed to read: {e}"),
                file: Some(clippy_path.display().to_string()),
                line: None,
                inventory: false,
            });
            return results;
        }
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "R4".to_owned(),
                severity: Severity::Error,
                title: "clippy.toml parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(clippy_path.display().to_string()),
                line: None,
                inventory: false,
            });
            return results;
        }
    };

    // All profiles (service, library) use the same expected bans.
    // Unknown/missing profiles default to service (the most comprehensive set).
    let expected_methods = EXPECTED_METHOD_BANS;
    let expected_types = EXPECTED_TYPE_BANS;

    // Check disallowed-methods: R4=missing, R6=extras
    check_ban_list(
        &table,
        "disallowed-methods",
        expected_methods,
        "R4",
        "R6",
        "method ban",
        clippy_path,
        &mut results,
    );

    // Check disallowed-types: R5=missing, R7=extras
    check_ban_list(
        &table,
        "disallowed-types",
        expected_types,
        "R5",
        "R7",
        "type ban",
        clippy_path,
        &mut results,
    );

    results
}

#[allow(clippy::too_many_arguments)] // reason: validation function needs all context parameters
#[allow(clippy::too_many_lines)] // reason: ban list check with reason validation is inherently sequential
fn check_ban_list(
    table: &toml::Value,
    key: &str,
    expected: &[&str],
    missing_id: &str,
    extra_id: &str,
    label: &str,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let Some(bans) = table.get(key).and_then(|v| v.as_array()) else {
        results.push(CheckResult {
            id: missing_id.to_owned(),
            severity: Severity::Error,
            title: format!("No {key} section"),
            message: format!("{key} array missing from clippy.toml"),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    };

    // Extract paths from the ban entries and check for reason fields
    let mut found_paths: BTreeSet<String> = BTreeSet::new();
    for ban in bans {
        if let Some(path) = ban.get("path").and_then(|p| p.as_str()) {
            // Table entry — check for reason field
            if ban.get("reason").and_then(|r| r.as_str()).is_none() {
                results.push(CheckResult {
                    id: missing_id.to_owned(),
                    severity: Severity::Warn,
                    title: format!("{label} without reason"),
                    message: format!("`{path}` is banned in clippy.toml {key} but has no `reason` field. Add `reason = \"...\"` so developers understand why this API is banned and what alternative to use."),
                    file: Some(file_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
            let _ = found_paths.insert(path.to_owned());
        } else if let Some(path) = ban.as_str() {
            // Simple string format — inherently has no reason
            results.push(CheckResult {
                id: missing_id.to_owned(),
                severity: Severity::Warn,
                title: format!("{label} without reason"),
                message: format!("`{path}` is banned in clippy.toml {key} as a plain string with no `reason` field. Use table format `{{ path = \"{path}\", reason = \"...\" }}` so developers understand why this API is banned."),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
            let _ = found_paths.insert(path.to_owned());
        }
    }

    let expected_set: BTreeSet<String> = expected.iter().map(|s| (*s).to_owned()).collect();

    // Check for missing expected bans
    for exp in &expected_set {
        if found_paths.contains(exp) {
            results.push(CheckResult {
                id: missing_id.to_owned(),
                severity: Severity::Info,
                title: format!("{label} present"),
                message: format!("`{exp}` is banned in clippy.toml {key}. Calls to this API are flagged at compile time. No action needed."),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        } else {
            results.push(CheckResult {
                id: missing_id.to_owned(),
                severity: Severity::Error,
                title: format!("Missing {label}"),
                message: format!("`{exp}` is not banned in clippy.toml {key}. Without this ban, code can use this API unchecked, bypassing guardrail enforcement. Add it to the `{key}` array in clippy.toml or run `guardrail3 generate` to regenerate."),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    // Report extra bans as Info
    for found in &found_paths {
        if !expected_set.contains(found) {
            results.push(CheckResult {
                id: extra_id.to_owned(),
                severity: Severity::Info,
                title: format!("Extra {label}"),
                message: format!("Additional ban `{found}` in clippy.toml {key} beyond the guardrail3 baseline. Project-specific ban — no action needed."),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        }
    }
}
