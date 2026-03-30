use std::path::{Path, PathBuf};

use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_bans;
use super::deny_inventory;
use super::deny_licenses;
use guardrail3_outbound_traits::FileSystem;

pub fn check(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    profile: Option<&str>,
    deny_tomls: &[PathBuf],
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // Filter deny.tomls within this workspace
    let workspace_deny_tomls: Vec<&PathBuf> = deny_tomls
        .iter()
        .filter(|p| p.starts_with(workspace_root))
        .collect();

    if workspace_deny_tomls.is_empty() {
        results.push(CheckResult::from_parts(
    "R8".to_owned(),
    Severity::Error,
    "deny.toml missing".to_owned(),
    "No deny.toml at workspace root. Without it, cargo-deny cannot enforce crate bans, license compliance, or security advisory checks. Run `guardrail3 generate` to create it.".to_owned(),
    Some(workspace_root.display().to_string()),
    None,
    false,
        ));
        return results;
    }

    // Check each deny.toml found by the crawler within the workspace
    for deny_path in &workspace_deny_tomls {
        let Some(table) = read_and_parse_deny_toml(fs, deny_path, workspace_root, &mut results)
        else {
            continue;
        };

        check_deprecated_advisory_fields(&table, deny_path, &mut results);
        check_advisory_values(&table, deny_path, &mut results);
        deny_bans::check_ban_list(&table, deny_path, profile, &mut results);
        deny_licenses::check_licenses(&table, deny_path, &mut results);
        deny_licenses::check_sources(&table, deny_path, &mut results);
        deny_bans::check_tokio_feature_ban(&table, deny_path, &mut results);
        deny_inventory::check_skip_entries(&table, deny_path, &mut results);
        deny_inventory::check_advisory_ignores(&table, deny_path, &mut results);
    }

    results,
)

fn read_and_parse_deny_toml(
    fs: &dyn FileSystem,
    deny_path: &Path,
    workspace_root: &Path,
    results: &mut Vec<CheckResult>,
) -> Option<toml::Value> {
    if !deny_path.exists() {
        results.push(CheckResult::from_parts(
    "R8".to_owned(),
    Severity::Error,
    "deny.toml missing".to_owned(),
    "No deny.toml at workspace root. Without it, cargo-deny cannot enforce crate bans, license compliance, or security advisory checks. Run `guardrail3 generate` to create it.".to_owned(),
    Some(workspace_root.display().to_string()),
    None,
    false,
        ));
        return None;
    }

    results.push(CheckResult {
        id: "R8".to_owned(),
        severity: Severity::Info,
        title: "deny.toml exists".to_owned(),
        message: "deny.toml found at workspace root. This file configures cargo-deny to enforce crate bans, license compliance, and advisory checks. No action needed.".to_owned(),
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
    },
)

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
            results.push(CheckResult::from_parts(
    "R9".to_owned(),
    Severity::Warn,
    format!("Deprecated field: {deprecated}"),
    format!("[advisories].{deprecated} is deprecated in deny.toml 0.19+"),
    Some(deny_path.display().to_string()),
    None,
    false,
            ));
        }
    },
)

fn check_advisory_values(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let Some(advisories) = table.get("advisories") else {
        results.push(CheckResult::from_parts(
    "R10".to_owned(),
    Severity::Error,
    "[advisories] section missing".to_owned(),
    "deny.toml has no [advisories] section".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
        ));
        return;
    };

    check_unmaintained_value(advisories, file_path, results);
    check_yanked_value(advisories, file_path, results);,
)

fn check_unmaintained_value(
    advisories: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    match advisories.get("unmaintained").and_then(|v| v.as_str()) {
        Some("workspace") => {
            results.push(
                CheckResult::from_parts(
                    "R10".to_owned(),
                    Severity::Info,
                    "unmaintained correct".to_owned(),
                    "unmaintained = \"workspace\"".to_owned(),
                    Some(file_path.display().to_string()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
        Some("deny") => {
            results.push(
                CheckResult::from_parts(
                    "R11".to_owned(),
                    Severity::Info,
                    "unmaintained stricter than expected".to_owned(),
                    "unmaintained = \"deny\" (expected \"workspace\")".to_owned(),
                    Some(file_path.display().to_string()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
        Some(other) => {
            results.push(CheckResult::from_parts(
    "R10".to_owned(),
    Severity::Error,
    "unmaintained wrong value".to_owned(),
    format!("Expected \"workspace\", got \"{other}\""),
    Some(file_path.display().to_string()),
    None,
    false,
            ));
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
    },
)

fn check_yanked_value(advisories: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    match advisories.get("yanked").and_then(|v| v.as_str()) {
        Some("warn") => {
            results.push(
                CheckResult::from_parts(
                    "R10".to_owned(),
                    Severity::Info,
                    "yanked correct".to_owned(),
                    "yanked = \"warn\"".to_owned(),
                    Some(file_path.display().to_string()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
        Some("deny") => {
            results.push(
                CheckResult::from_parts(
                    "R11".to_owned(),
                    Severity::Info,
                    "yanked stricter than expected".to_owned(),
                    "yanked = \"deny\" (expected \"warn\")".to_owned(),
                    Some(file_path.display().to_string()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
        Some(other) => {
            results.push(CheckResult::from_parts(
    "R10".to_owned(),
    Severity::Error,
    "yanked wrong value".to_owned(),
    format!("Expected \"warn\", got \"{other}\""),
    Some(file_path.display().to_string()),
    None,
    false,
            ));
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
    },
)
