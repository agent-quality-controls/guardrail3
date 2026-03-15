use std::collections::BTreeSet;
use std::path::Path;

use crate::report::types::{CheckResult, Severity};

const EXPECTED_METHOD_BANS: &[&str] = &[
    "std::env::var",
    "std::env::var_os",
    "std::env::vars",
    "std::env::set_var",
    "std::env::remove_var",
    "std::process::exit",
    "std::process::Command::new",
    "std::thread::sleep",
    "std::fs::read_to_string",
    "std::fs::read",
    "std::fs::read_dir",
    "std::fs::read_link",
    "std::fs::write",
    "std::fs::remove_file",
    "std::fs::remove_dir_all",
    "std::fs::create_dir_all",
    "std::fs::rename",
    "std::fs::copy",
    "std::fs::metadata",
    "std::fs::symlink_metadata",
    "std::fs::canonicalize",
    "std::fs::set_permissions",
    "std::fs::hard_link",
    "reqwest::Client::new",
    "reqwest::Client::builder",
];

const EXPECTED_TYPE_BANS: &[&str] = &[
    "std::collections::HashMap",
    "std::collections::HashSet",
    "std::sync::Mutex",
    "std::sync::RwLock",
    "std::fs::File",
];

const MINIMAL_EXPECTED_METHOD_BANS: &[&str] = &[
    "std::env::var",
    "std::env::var_os",
    "std::env::vars",
    "std::env::set_var",
    "std::env::remove_var",
    "std::process::exit",
    "std::process::Command::new",
    "std::thread::sleep",
];

const MINIMAL_EXPECTED_TYPE_BANS: &[&str] = &[
    "std::collections::HashMap",
    "std::collections::HashSet",
    "std::sync::Mutex",
    "std::sync::RwLock",
];

pub fn check(workspace_root: &Path, profile: Option<&str>) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let clippy_path = workspace_root.join("clippy.toml");

    if !clippy_path.exists() {
        results.push(CheckResult {
            id: "R4".to_owned(),
            severity: Severity::Error,
            title: "Cannot check clippy bans".to_owned(),
            message: "clippy.toml not found".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
        return results;
    }

    let content = match std::fs::read_to_string(&clippy_path) {
        Ok(content) => content,
        Err(e) => {
            results.push(CheckResult {
                id: "R4".to_owned(),
                severity: Severity::Error,
                title: "clippy.toml unreadable".to_owned(),
                message: format!("Failed to read: {e}"),
                file: Some(clippy_path.display().to_string()),
                line: None,
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
            });
            return results;
        }
    };

    let expected_methods = match profile {
        Some("minimal") => MINIMAL_EXPECTED_METHOD_BANS,
        _ => EXPECTED_METHOD_BANS,
    };
    let expected_types = match profile {
        Some("minimal") => MINIMAL_EXPECTED_TYPE_BANS,
        _ => EXPECTED_TYPE_BANS,
    };

    // Check disallowed-methods: R4=missing, R6=extras
    check_ban_list(
        &table,
        "disallowed-methods",
        expected_methods,
        "R4",
        "R6",
        "method ban",
        &clippy_path,
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
        &clippy_path,
        &mut results,
    );

    results
}

#[allow(clippy::too_many_arguments)] // reason: validation function needs all context parameters
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
        });
        return;
    };

    // Extract paths from the ban entries
    let mut found_paths: BTreeSet<String> = BTreeSet::new();
    for ban in bans {
        if let Some(path) = ban.get("path").and_then(|p| p.as_str()) {
            let _ = found_paths.insert(path.to_owned());
        } else if let Some(path) = ban.as_str() {
            // Simple string format
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
                message: exp.clone(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        } else {
            results.push(CheckResult {
                id: missing_id.to_owned(),
                severity: Severity::Error,
                title: format!("Missing {label}"),
                message: format!("Expected ban for {exp}"),
                file: Some(file_path.display().to_string()),
                line: None,
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
                message: format!("extra ban: {found}"),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }
}
