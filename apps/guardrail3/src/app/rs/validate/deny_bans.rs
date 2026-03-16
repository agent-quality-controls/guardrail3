use std::collections::BTreeSet;
use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

pub fn check_ban_list(
    table: &toml::Value,
    file_path: &Path,
    _profile: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let Some(bans) = table.get("bans") else {
        results.push(CheckResult {
            id: "R12".to_owned(),
            severity: Severity::Error,
            title: "[bans] section missing".to_owned(),
            message: "deny.toml has no [bans] section".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    };

    check_bans_settings(bans, file_path, results);
    check_deny_list_coverage(bans, file_path, results);
}

fn check_bans_settings(
    bans: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    // Check multiple-versions = "deny"
    match bans.get("multiple-versions").and_then(|v| v.as_str()) {
        Some("deny") => {
            // Correct
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R12".to_owned(),
                severity: Severity::Error,
                title: "multiple-versions wrong".to_owned(),
                message: format!("Expected \"deny\", got \"{other}\""),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R12".to_owned(),
                severity: Severity::Error,
                title: "multiple-versions missing".to_owned(),
                message: "Expected multiple-versions = \"deny\"".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    // Check highlight = "all" (Info if different)
    match bans.get("highlight").and_then(|v| v.as_str()) {
        Some("all") => {
            // Correct
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R13".to_owned(),
                severity: Severity::Info,
                title: "highlight not \"all\"".to_owned(),
                message: format!("highlight = \"{other}\" (expected \"all\")"),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R13".to_owned(),
                severity: Severity::Info,
                title: "highlight not set".to_owned(),
                message: "Expected highlight = \"all\"".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

fn check_deny_list_coverage(
    bans: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let Some(deny_list) = bans.get("deny").and_then(|d| d.as_array()) else {
        results.push(CheckResult {
            id: "R12".to_owned(),
            severity: Severity::Error,
            title: "No [bans] deny list".to_owned(),
            message: "[bans].deny array missing".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    };

    // Extract crate names from deny list
    let mut found_bans: BTreeSet<String> = BTreeSet::new();
    for entry in deny_list {
        if let Some(name) = entry.get("name").and_then(|n| n.as_str()) {
            let _ = found_bans.insert(name.to_owned());
        } else if let Some(name) = entry.as_str() {
            let _ = found_bans.insert(name.to_owned());
        }
    }

    // All profiles use the same expected bans. Unknown/missing defaults to service.
    let expected_bans: &[&str] = super::deny_audit::EXPECTED_BANS;
    let expected_set: BTreeSet<String> = expected_bans.iter().map(|s| (*s).to_owned()).collect();

    for exp in &expected_set {
        if !found_bans.contains(exp) {
            results.push(CheckResult {
                id: "R12".to_owned(),
                severity: Severity::Error,
                title: "Missing ban".to_owned(),
                message: format!("Expected ban for {exp}"),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    for found in &found_bans {
        if !expected_set.contains(found) {
            results.push(CheckResult {
                id: "R13".to_owned(),
                severity: Severity::Info,
                title: "Extra ban".to_owned(),
                message: format!("extra ban: {found}"),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[allow(clippy::match_same_arms)] // reason: separate arms for readability of different tokio features
#[allow(clippy::wildcard_in_or_patterns)] // reason: explicit handling of all feature patterns
pub fn check_tokio_feature_ban(
    table: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let Some(bans) = table.get("bans") else {
        return;
    };

    let Some(features) = bans.get("features").and_then(|f| f.as_array()) else {
        results.push(CheckResult {
            id: "R17".to_owned(),
            severity: Severity::Warn,
            title: "No [[bans.features]]".to_owned(),
            message: "No feature bans configured".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    };

    let mut tokio_full_banned = false;
    let mut extra_feature_bans = Vec::new();

    for feature_entry in features {
        let name = feature_entry
            .get("name")
            .or_else(|| feature_entry.get("crate"))
            .and_then(|n| n.as_str());

        if name == Some("tokio") {
            if let Some(deny_arr) = feature_entry.get("deny").and_then(|d| d.as_array()) {
                if deny_arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .any(|x| x == "full")
                {
                    tokio_full_banned = true;
                }
            }
        } else if let Some(n) = name {
            extra_feature_bans.push(n.to_owned());
        }
    }

    if tokio_full_banned {
        results.push(CheckResult {
            id: "R17".to_owned(),
            severity: Severity::Info,
            title: "tokio full feature banned".to_owned(),
            message: "[[bans.features]] denies tokio/full".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "R17".to_owned(),
            severity: Severity::Warn,
            title: "tokio full not banned".to_owned(),
            message: "Expected [[bans.features]] to deny tokio/full".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // R18: extra feature bans
    for extra in &extra_feature_bans {
        results.push(CheckResult {
            id: "R18".to_owned(),
            severity: Severity::Info,
            title: "Extra feature ban".to_owned(),
            message: format!("Feature ban for: {extra}"),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}
