use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

use super::deny_bans;
use super::deny_inventory;
use super::deny_licenses;
use crate::ports::outbound::FileSystem;

pub const EXPECTED_BANS: &[&str] = &[
    "simd-json",
    "json5",
    "sonic-rs",
    "openssl",
    "openssl-sys",
    "ureq",
    "surf",
    "isahc",
    "log4rs",
    "env_logger",
    "simple_logger",
    "fern",
    "async-std",
    "smol",
    "anyhow",
    "actix-web",
    "rocket",
    "warp",
    "poem",
    "chrono",
    "diesel",
    "sea-orm",
    "bincode",
    "rmp-serde",
    "prost",
    "flatbuffers",
];

pub fn check(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    profile: Option<&str>,
) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let deny_path = workspace_root.join("deny.toml");

    let Some(table) = read_and_parse_deny_toml(fs, &deny_path, workspace_root, &mut results) else {
        return results;
    };

    check_deprecated_advisory_fields(&table, &deny_path, &mut results);
    check_advisory_values(&table, &deny_path, &mut results);
    deny_bans::check_ban_list(&table, &deny_path, profile, &mut results);
    deny_licenses::check_licenses(&table, &deny_path, &mut results);
    deny_licenses::check_sources(&table, &deny_path, &mut results);
    deny_bans::check_tokio_feature_ban(&table, &deny_path, &mut results);
    deny_inventory::check_skip_entries(&table, &deny_path, &mut results);
    deny_inventory::check_advisory_ignores(&table, &deny_path, &mut results);

    results
}

fn read_and_parse_deny_toml(
    fs: &dyn FileSystem,
    deny_path: &Path,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) -> Option<toml::Value> {
    if !deny_path.exists() {
        results.push(CheckResult {
            id: "R8".to_owned(),
            severity: Severity::Error,
            title: "deny.toml missing".to_owned(),
            message: "No deny.toml found at workspace root".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
            inventory: false,
        });
        return None;
    }

    results.push(CheckResult {
        id: "R8".to_owned(),
        severity: Severity::Info,
        title: "deny.toml exists".to_owned(),
        message: "Found at workspace root".to_owned(),
        file: Some(deny_path.display().to_string()),
        line: None,
        inventory: false,
    }.as_inventory());

    let content = match fs.read_file_err(deny_path) {
        Ok(content) => content,
        Err(e) => {
            results.push(CheckResult {
                id: "R8".to_owned(),
                severity: Severity::Error,
                title: "deny.toml unreadable".to_owned(),
                message: format!("Failed to read: {e}"),
                file: Some(deny_path.display().to_string()),
                line: None,
                inventory: false,
            });
            return None;
        }
    };

    match content.parse() {
        Ok(v) => Some(v),
        Err(e) => {
            results.push(CheckResult {
                id: "R8".to_owned(),
                severity: Severity::Error,
                title: "deny.toml parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(deny_path.display().to_string()),
                line: None,
                inventory: false,
            });
            None
        }
    }
}

fn check_deprecated_advisory_fields(
    table: &toml::Value,
    deny_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let Some(advisories) = table.get("advisories") else {
        return;
    };

    for deprecated in &["vulnerability", "notice", "unsound"] {
        if advisories.get(deprecated).is_some() {
            results.push(CheckResult {
                id: "R9".to_owned(),
                severity: Severity::Warn,
                title: format!("Deprecated field: {deprecated}"),
                message: format!("[advisories].{deprecated} is deprecated in deny.toml 0.19+"),
                file: Some(deny_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

fn check_advisory_values(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let Some(advisories) = table.get("advisories") else {
        results.push(CheckResult {
            id: "R10".to_owned(),
            severity: Severity::Error,
            title: "[advisories] section missing".to_owned(),
            message: "deny.toml has no [advisories] section".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    };

    check_unmaintained_value(advisories, file_path, results);
    check_yanked_value(advisories, file_path, results);
}

fn check_unmaintained_value(
    advisories: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    match advisories.get("unmaintained").and_then(|v| v.as_str()) {
        Some("workspace") => {
            results.push(CheckResult {
                id: "R10".to_owned(),
                severity: Severity::Info,
                title: "unmaintained correct".to_owned(),
                message: "unmaintained = \"workspace\"".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        }
        Some("deny") => {
            results.push(CheckResult {
                id: "R11".to_owned(),
                severity: Severity::Info,
                title: "unmaintained stricter than expected".to_owned(),
                message: "unmaintained = \"deny\" (expected \"workspace\")".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R10".to_owned(),
                severity: Severity::Error,
                title: "unmaintained wrong value".to_owned(),
                message: format!("Expected \"workspace\", got \"{other}\""),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R10".to_owned(),
                severity: Severity::Error,
                title: "unmaintained missing".to_owned(),
                message: "Expected unmaintained = \"workspace\"".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

fn check_yanked_value(
    advisories: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    match advisories.get("yanked").and_then(|v| v.as_str()) {
        Some("warn") => {
            results.push(CheckResult {
                id: "R10".to_owned(),
                severity: Severity::Info,
                title: "yanked correct".to_owned(),
                message: "yanked = \"warn\"".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        }
        Some("deny") => {
            results.push(CheckResult {
                id: "R11".to_owned(),
                severity: Severity::Info,
                title: "yanked stricter than expected".to_owned(),
                message: "yanked = \"deny\" (expected \"warn\")".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R10".to_owned(),
                severity: Severity::Error,
                title: "yanked wrong value".to_owned(),
                message: format!("Expected \"warn\", got \"{other}\""),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R10".to_owned(),
                severity: Severity::Error,
                title: "yanked not set".to_owned(),
                message: "Expected yanked = \"warn\"".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}
