use std::collections::BTreeSet;
use std::path::Path;

use crate::report::types::{CheckResult, Severity};

const EXPECTED_BANS: &[&str] = &[
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
            id: "R8".to_string(),
            severity: Severity::Error,
            title: "deny.toml missing".to_string(),
            message: "No deny.toml found at workspace root".to_string(),
            file: Some(workspace_root.display().to_string()),
            line: None,
        });
        return results;
    }

    results.push(CheckResult {
        id: "R8".to_string(),
        severity: Severity::Info,
        title: "deny.toml exists".to_string(),
        message: "Found at workspace root".to_string(),
        file: Some(deny_path.display().to_string()),
        line: None,
    });

    let content = match std::fs::read_to_string(&deny_path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "R8".to_string(),
                severity: Severity::Error,
                title: "deny.toml unreadable".to_string(),
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
                id: "R8".to_string(),
                severity: Severity::Error,
                title: "deny.toml parse error".to_string(),
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
                    id: "R9".to_string(),
                    severity: Severity::Warn,
                    title: format!("Deprecated field: {deprecated}"),
                    message: format!(
                        "[advisories].{deprecated} is deprecated in deny.toml 0.19+"
                    ),
                    file: Some(deny_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    // R10-R11: Check unmaintained and yanked values
    check_advisory_values(&table, &deny_path, &mut results);

    // R12-R13: Check bans deny list
    check_ban_list(&table, &deny_path, profile, &mut results);

    // R14-R15: Check licenses allow list
    check_licenses(&table, &deny_path, &mut results);

    // R16: Check sources
    check_sources(&table, &deny_path, &mut results);

    // R17-R18: Check tokio full ban
    check_tokio_feature_ban(&table, &deny_path, &mut results);

    // R19: Inventory skip entries
    check_skip_entries(&table, &deny_path, &mut results);

    // R20: Inventory advisory ignore entries
    check_advisory_ignores(&table, &deny_path, &mut results);

    results
}

fn check_advisory_values(
    table: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let advisories = match table.get("advisories") {
        Some(a) => a,
        None => {
            results.push(CheckResult {
                id: "R10".to_string(),
                severity: Severity::Error,
                title: "[advisories] section missing".to_string(),
                message: "deny.toml has no [advisories] section".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    // unmaintained
    match advisories.get("unmaintained").and_then(|v| v.as_str()) {
        Some("workspace") => {
            results.push(CheckResult {
                id: "R10".to_string(),
                severity: Severity::Info,
                title: "unmaintained correct".to_string(),
                message: "unmaintained = \"workspace\"".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        Some("deny") => {
            results.push(CheckResult {
                id: "R11".to_string(),
                severity: Severity::Info,
                title: "unmaintained stricter than expected".to_string(),
                message: "unmaintained = \"deny\" (expected \"workspace\")".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R10".to_string(),
                severity: Severity::Error,
                title: "unmaintained wrong value".to_string(),
                message: format!("Expected \"workspace\", got \"{other}\""),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R10".to_string(),
                severity: Severity::Error,
                title: "unmaintained missing".to_string(),
                message: "Expected unmaintained = \"workspace\"".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }

    // yanked
    match advisories.get("yanked").and_then(|v| v.as_str()) {
        Some("warn") => {
            results.push(CheckResult {
                id: "R10".to_string(),
                severity: Severity::Info,
                title: "yanked correct".to_string(),
                message: "yanked = \"warn\"".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        Some("deny") => {
            results.push(CheckResult {
                id: "R11".to_string(),
                severity: Severity::Info,
                title: "yanked stricter than expected".to_string(),
                message: "yanked = \"deny\" (expected \"warn\")".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R10".to_string(),
                severity: Severity::Error,
                title: "yanked wrong value".to_string(),
                message: format!("Expected \"warn\", got \"{other}\""),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R10".to_string(),
                severity: Severity::Error,
                title: "yanked not set".to_string(),
                message: "Expected yanked = \"warn\"".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }
}

fn check_ban_list(table: &toml::Value, file_path: &Path, profile: Option<&str>, results: &mut Vec<CheckResult>) {
    let bans = match table.get("bans") {
        Some(b) => b,
        None => {
            results.push(CheckResult {
                id: "R12".to_string(),
                severity: Severity::Error,
                title: "[bans] section missing".to_string(),
                message: "deny.toml has no [bans] section".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    // Check multiple-versions = "deny"
    match bans.get("multiple-versions").and_then(|v| v.as_str()) {
        Some("deny") => {
            // Correct
        }
        Some(other) => {
            results.push(CheckResult {
                id: "R12".to_string(),
                severity: Severity::Error,
                title: "multiple-versions wrong".to_string(),
                message: format!("Expected \"deny\", got \"{other}\""),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R12".to_string(),
                severity: Severity::Error,
                title: "multiple-versions missing".to_string(),
                message: "Expected multiple-versions = \"deny\"".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
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
                id: "R13".to_string(),
                severity: Severity::Info,
                title: "highlight not \"all\"".to_string(),
                message: format!("highlight = \"{other}\" (expected \"all\")"),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R13".to_string(),
                severity: Severity::Info,
                title: "highlight not set".to_string(),
                message: "Expected highlight = \"all\"".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }

    let deny_list = match bans.get("deny").and_then(|d| d.as_array()) {
        Some(arr) => arr,
        None => {
            results.push(CheckResult {
                id: "R12".to_string(),
                severity: Severity::Error,
                title: "No [bans] deny list".to_string(),
                message: "[bans].deny array missing".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    // Extract crate names from deny list
    let mut found_bans: BTreeSet<String> = BTreeSet::new();
    for entry in deny_list {
        if let Some(name) = entry.get("name").and_then(|n| n.as_str()) {
            found_bans.insert(name.to_string());
        } else if let Some(name) = entry.as_str() {
            found_bans.insert(name.to_string());
        }
    }

    let expected_bans: &[&str] = match profile {
        Some("minimal") => &[],
        _ => EXPECTED_BANS,
    };
    let expected_set: BTreeSet<String> =
        expected_bans.iter().map(|s| (*s).to_string()).collect();

    for exp in &expected_set {
        if !found_bans.contains(exp) {
            results.push(CheckResult {
                id: "R12".to_string(),
                severity: Severity::Error,
                title: "Missing ban".to_string(),
                message: format!("Expected ban for {exp}"),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }

    for found in &found_bans {
        if !expected_set.contains(found) {
            results.push(CheckResult {
                id: "R13".to_string(),
                severity: Severity::Info,
                title: "Extra ban".to_string(),
                message: format!("extra ban: {found}"),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }
}

fn check_licenses(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let licenses = match table.get("licenses") {
        Some(l) => l,
        None => {
            results.push(CheckResult {
                id: "R14".to_string(),
                severity: Severity::Error,
                title: "[licenses] section missing".to_string(),
                message: "deny.toml has no [licenses] section".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    // R14: license allow-list
    match licenses.get("allow").and_then(|a| a.as_array()) {
        Some(arr) if !arr.is_empty() => {
            let license_list: Vec<&str> =
                arr.iter().filter_map(|v| v.as_str()).collect();
            results.push(CheckResult {
                id: "R14".to_string(),
                severity: Severity::Info,
                title: "License allow list present".to_string(),
                message: format!("Allowed: {}", license_list.join(", ")),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        _ => {
            results.push(CheckResult {
                id: "R14".to_string(),
                severity: Severity::Error,
                title: "No license allow list".to_string(),
                message: "[licenses].allow is missing or empty".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }

    // R15: license extras — report any non-standard licenses as Info
    // (We don't know the "standard" set, so just report the full list)

    // Check [licenses.private] ignore = true
    let private_ignore = licenses
        .get("private")
        .and_then(|p| p.get("ignore"))
        .and_then(|i| i.as_bool());

    match private_ignore {
        Some(true) => {
            // Correct
        }
        Some(false) => {
            results.push(CheckResult {
                id: "R14".to_string(),
                severity: Severity::Error,
                title: "[licenses.private] ignore not true".to_string(),
                message: "Expected [licenses.private] ignore = true".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R14".to_string(),
                severity: Severity::Error,
                title: "[licenses.private] missing".to_string(),
                message: "Expected [licenses.private] ignore = true".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }

    // Check confidence-threshold
    match licenses.get("confidence-threshold") {
        Some(toml::Value::Float(v)) => {
            // Compare with tolerance
            let diff = (*v - 0.8_f64).abs();
            if diff > 0.001 {
                results.push(CheckResult {
                    id: "R15".to_string(),
                    severity: Severity::Info,
                    title: "confidence-threshold differs".to_string(),
                    message: format!("confidence-threshold = {v} (expected 0.8)"),
                    file: Some(file_path.display().to_string()),
                    line: None,
                });
            }
        }
        None => {
            results.push(CheckResult {
                id: "R15".to_string(),
                severity: Severity::Info,
                title: "confidence-threshold not set".to_string(),
                message: "Expected confidence-threshold = 0.8".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        _ => {}
    }
}

fn check_sources(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let sources = match table.get("sources") {
        Some(s) => s,
        None => {
            results.push(CheckResult {
                id: "R16".to_string(),
                severity: Severity::Error,
                title: "[sources] section missing".to_string(),
                message: "deny.toml has no [sources] section".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    for key in &["unknown-registry", "unknown-git"] {
        match sources.get(key).and_then(|v| v.as_str()) {
            Some("deny") => {
                results.push(CheckResult {
                    id: "R16".to_string(),
                    severity: Severity::Info,
                    title: format!("{key} correct"),
                    message: format!("{key} = \"deny\""),
                    file: Some(file_path.display().to_string()),
                    line: None,
                });
            }
            Some(other) => {
                results.push(CheckResult {
                    id: "R16".to_string(),
                    severity: Severity::Error,
                    title: format!("{key} wrong"),
                    message: format!("Expected \"deny\", got \"{other}\""),
                    file: Some(file_path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R16".to_string(),
                    severity: Severity::Error,
                    title: format!("{key} not set"),
                    message: format!("Expected {key} = \"deny\""),
                    file: Some(file_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    // Check allow-registry contains only crates.io
    if let Some(allow_reg) = sources.get("allow-registry").and_then(|v| v.as_array()) {
        let registries: Vec<&str> = allow_reg.iter().filter_map(|v| v.as_str()).collect();
        let has_non_cratesio = registries
            .iter()
            .any(|r| !r.contains("crates.io"));
        if has_non_cratesio {
            results.push(CheckResult {
                id: "R16".to_string(),
                severity: Severity::Error,
                title: "Non-crates.io registry allowed".to_string(),
                message: format!("allow-registry contains: {}", registries.join(", ")),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }

    // Check allow-git = [] (empty)
    match sources.get("allow-git").and_then(|v| v.as_array()) {
        Some(arr) if !arr.is_empty() => {
            let git_sources: Vec<&str> =
                arr.iter().filter_map(|v| v.as_str()).collect();
            results.push(CheckResult {
                id: "R16".to_string(),
                severity: Severity::Error,
                title: "allow-git not empty".to_string(),
                message: format!("allow-git contains: {}", git_sources.join(", ")),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        Some(_) => {
            // Empty array — correct
        }
        None => {
            // Not set — could be acceptable if unknown-git = "deny"
        }
    }
}

fn check_tokio_feature_ban(
    table: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let bans = match table.get("bans") {
        Some(b) => b,
        None => return,
    };

    let features = match bans.get("features").and_then(|f| f.as_array()) {
        Some(arr) => arr,
        None => {
            results.push(CheckResult {
                id: "R17".to_string(),
                severity: Severity::Warn,
                title: "No [[bans.features]]".to_string(),
                message: "No feature bans configured".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let mut tokio_full_banned = false;
    let mut extra_feature_bans = Vec::new();

    for feature_entry in features {
        let name = feature_entry
            .get("name")
            .or_else(|| feature_entry.get("crate"))
            .and_then(|n| n.as_str());

        if name == Some("tokio") {
            if let Some(deny_arr) =
                feature_entry.get("deny").and_then(|d| d.as_array())
            {
                let denied: Vec<&str> =
                    deny_arr.iter().filter_map(|v| v.as_str()).collect();
                if denied.contains(&"full") {
                    tokio_full_banned = true;
                }
            }
        } else if let Some(n) = name {
            extra_feature_bans.push(n.to_string());
        }
    }

    if tokio_full_banned {
        results.push(CheckResult {
            id: "R17".to_string(),
            severity: Severity::Info,
            title: "tokio full feature banned".to_string(),
            message: "[[bans.features]] denies tokio/full".to_string(),
            file: Some(file_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "R17".to_string(),
            severity: Severity::Warn,
            title: "tokio full not banned".to_string(),
            message: "Expected [[bans.features]] to deny tokio/full".to_string(),
            file: Some(file_path.display().to_string()),
            line: None,
        });
    }

    // R18: extra feature bans
    for extra in &extra_feature_bans {
        results.push(CheckResult {
            id: "R18".to_string(),
            severity: Severity::Info,
            title: "Extra feature ban".to_string(),
            message: format!("Feature ban for: {extra}"),
            file: Some(file_path.display().to_string()),
            line: None,
        });
    }
}

fn check_skip_entries(
    table: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let bans = match table.get("bans") {
        Some(b) => b,
        None => return,
    };

    if let Some(skip) = bans.get("skip").and_then(|s| s.as_array()) {
        for entry in skip {
            // Try 0.19+ format: { crate = "name@version" }
            let (name, version) =
                if let Some(crate_field) = entry.get("crate").and_then(|c| c.as_str()) {
                    let parts: Vec<&str> = crate_field.splitn(2, '@').collect();
                    let n = parts[0];
                    let v = parts.get(1).copied().unwrap_or("*");
                    (n.to_string(), v.to_string())
                } else if let Some(s) = entry.as_str() {
                    // Plain string entry
                    (s.to_string(), "*".to_string())
                } else {
                    // Fall back to older format: { name = "...", version = "..." }
                    let n = entry
                        .get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("unknown");
                    let v = entry
                        .get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("*");
                    (n.to_string(), v.to_string())
                };

            let reason = entry
                .get("reason")
                .and_then(|r| r.as_str())
                .unwrap_or("");
            let message = if reason.is_empty() {
                format!("{name} {version}")
            } else {
                format!("{name} {version} — {reason}")
            };

            results.push(CheckResult {
                id: "R19".to_string(),
                severity: Severity::Info,
                title: "Skip entry".to_string(),
                message,
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }
}

fn check_advisory_ignores(
    table: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let advisories = match table.get("advisories") {
        Some(a) => a,
        None => return,
    };

    if let Some(ignore) = advisories.get("ignore").and_then(|i| i.as_array()) {
        for entry in ignore {
            let id = entry.as_str().unwrap_or("unknown");
            results.push(CheckResult {
                id: "R20".to_string(),
                severity: Severity::Info,
                title: "Advisory ignore".to_string(),
                message: id.to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }
}
