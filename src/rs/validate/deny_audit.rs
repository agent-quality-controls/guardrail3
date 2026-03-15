use std::path::Path;

use crate::report::types::{CheckResult, Severity};

use super::deny_bans;
use super::deny_inventory;
use super::deny_licenses;

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

pub fn check(workspace_root: &Path, profile: Option<&str>) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let deny_path = workspace_root.join("deny.toml");

    // R8: Check existence
    if !deny_path.exists() {
        results.push(CheckResult {
            id: "R8".to_owned(),
            severity: Severity::Error,
            title: "deny.toml missing".to_owned(),
            message: "No deny.toml found at workspace root".to_owned(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
        return results;
    }

    results.push(CheckResult {
        id: "R8".to_owned(),
        severity: Severity::Info,
        title: "deny.toml exists".to_owned(),
        message: "Found at workspace root".to_owned(),
        file: Some(deny_path.display().to_string()),
        line: None,
    });

    let content = match crate::fs::read_file_err(&deny_path) {
        Ok(content) => content,
        Err(e) => {
            results.push(CheckResult {
                id: "R8".to_owned(),
                severity: Severity::Error,
                title: "deny.toml unreadable".to_owned(),
                message: format!("Failed to read: {e}"),
                file: Some(deny_path.display().to_string()),
                line: None,
            });
            return results;
        }
    };

    let table: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "R8".to_owned(),
                severity: Severity::Error,
                title: "deny.toml parse error".to_owned(),
                message: format!("Invalid TOML: {e}"),
                file: Some(deny_path.display().to_string()),
                line: None,
            });
            return results;
        }
    };

    // R9: Check for deprecated fields in [advisories]
    if let Some(advisories) = table.get("advisories") {
        for deprecated in &["vulnerability", "notice", "unsound"] {
            if advisories.get(deprecated).is_some() {
                results.push(CheckResult {
                    id: "R9".to_owned(),
                    severity: Severity::Warn,
                    title: format!("Deprecated field: {deprecated}"),
                    message: format!("[advisories].{deprecated} is deprecated in deny.toml 0.19+"),
                    file: Some(deny_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    // R10-R11: Check unmaintained and yanked values
    check_advisory_values(&table, &deny_path, &mut results);

    // R12-R13: Check bans deny list
    deny_bans::check_ban_list(&table, &deny_path, profile, &mut results);

    // R14-R15: Check licenses allow list
    deny_licenses::check_licenses(&table, &deny_path, &mut results);

    // R16: Check sources
    deny_licenses::check_sources(&table, &deny_path, &mut results);

    // R17-R18: Check tokio full ban
    deny_bans::check_tokio_feature_ban(&table, &deny_path, &mut results);

    // R19: Inventory skip entries
    deny_inventory::check_skip_entries(&table, &deny_path, &mut results);

    // R20: Inventory advisory ignore entries
    deny_inventory::check_advisory_ignores(&table, &deny_path, &mut results);

    results
}

#[allow(clippy::too_many_lines)] // reason: advisory value checking
fn check_advisory_values(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let Some(advisories) = table.get("advisories") else {
        results.push(CheckResult {
            id: "R10".to_owned(),
            severity: Severity::Error,
            title: "[advisories] section missing".to_owned(),
            message: "deny.toml has no [advisories] section".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
        });
        return;
    };

    // unmaintained
    match advisories.get("unmaintained").and_then(|v| v.as_str()) {
        Some("workspace") => {
            results.push(CheckResult {
                id: "R10".to_owned(),
                severity: Severity::Info,
                title: "unmaintained correct".to_owned(),
                message: "unmaintained = \"workspace\"".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        Some("deny") => {
            results.push(CheckResult {
                id: "R11".to_owned(),
                severity: Severity::Info,
                title: "unmaintained stricter than expected".to_owned(),
                message: "unmaintained = \"deny\" (expected \"workspace\")".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R10".to_owned(),
                severity: Severity::Error,
                title: "unmaintained wrong value".to_owned(),
                message: format!("Expected \"workspace\", got \"{other}\""),
                file: Some(file_path.display().to_string()),
                line: None,
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
            });
        }
    }

    // yanked
    match advisories.get("yanked").and_then(|v| v.as_str()) {
        Some("warn") => {
            results.push(CheckResult {
                id: "R10".to_owned(),
                severity: Severity::Info,
                title: "yanked correct".to_owned(),
                message: "yanked = \"warn\"".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        Some("deny") => {
            results.push(CheckResult {
                id: "R11".to_owned(),
                severity: Severity::Info,
                title: "yanked stricter than expected".to_owned(),
                message: "yanked = \"deny\" (expected \"warn\")".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R10".to_owned(),
                severity: Severity::Error,
                title: "yanked wrong value".to_owned(),
                message: format!("Expected \"warn\", got \"{other}\""),
                file: Some(file_path.display().to_string()),
                line: None,
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
            });
        }
    }
}
