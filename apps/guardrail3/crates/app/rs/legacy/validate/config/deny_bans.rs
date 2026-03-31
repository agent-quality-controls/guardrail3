use std::collections::BTreeSet;
use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};

pub fn check_ban_list(
    table: &toml::Value,
    file_path: &Path,
    profile: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let Some(bans) = table.get("bans") else {
        results.push(CheckResult::from_parts(
    "R12".to_owned(),
    Severity::Error,
    "[bans] section missing".to_owned(),
    "deny.toml has no [bans] section".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
        ));
        return;
    };

    check_bans_settings(bans, file_path, results);
    check_deny_list_coverage(bans, file_path, profile, results);,
)

fn check_bans_settings(bans: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    // Check multiple-versions = "deny"
    match bans.get("multiple-versions").and_then(|v| v.as_str()) {
        Some("deny") => {
            // Correct
        }
        Some(other) => {
            results.push(CheckResult::from_parts(
    "R12".to_owned(),
    Severity::Error,
    "multiple-versions wrong".to_owned(),
    format!("Expected \"deny\", got \"{other}\""),
    Some(file_path.display().to_string()),
    None,
    false,
            ));
        }
        None => {
            results.push(CheckResult::from_parts(
    "R12".to_owned(),
    Severity::Error,
    "multiple-versions missing".to_owned(),
    "Expected multiple-versions = \"deny\"".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
            });
        }
    }

    // Check highlight = "all" (Info if different)
    match bans.get("highlight").and_then(|v| v.as_str()) {
        Some("all") => {
            // Correct
        }
        Some(other) => {
            results.push(CheckResult::from_parts(
    "R13".to_owned(),
    Severity::Info,
    "highlight not \"all\"".to_owned(),
    format!("highlight = \"{other}\" (expected \"all\")"),
    Some(file_path.display().to_string()),
    None,
    false,
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
    },
)

fn check_deny_list_coverage(
    bans: &toml::Value,
    file_path: &Path,
    profile: Option<&str>,
    results: &mut Vec<CheckResult>,
) {
    let Some(deny_list) = bans.get("deny").and_then(|d| d.as_array()) else {
        results.push(CheckResult::from_parts(
    "R12".to_owned(),
    Severity::Error,
    "No [bans] deny list".to_owned(),
    "[bans].deny array missing".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
        ));
        return;
    };

    // Extract crate names from deny list
    let mut found_bans: BTreeSet<String> = BTreeSet::new();
    for entry in deny_list {
        if let Some(name) = entry
            .get("name")
            .and_then(|n| n.as_str())
            .map(str::to_owned)
            .or_else(|| {
                // cargo-deny 0.19+ format: { crate = "name@version" }
                entry
                    .get("crate")
                    .and_then(|c| c.as_str())
                    .map(|c| c.split('@').next().unwrap_or(c).to_owned())
            })
        {
            let _ = found_bans.insert(name);
        } else if let Some(name) = entry.as_str() {
            let _ = found_bans.insert(name.to_owned());
        }
    }

    let expected_set = guardrail3_app_rs_family_deny::expected_ban_names(profile);

    for exp in &expected_set {
        if !found_bans.contains(exp) {
            results.push(CheckResult {
                id: "R12".to_owned(),
                severity: Severity::Error,
                title: "Missing ban".to_owned(),
                message: format!("Crate `{exp}` is not in deny.toml [bans.deny]. This crate has a preferred alternative and should be banned to prevent accidental usage. Add `{{ name = \"{exp}\" }}` to the [bans].deny array in deny.toml or run `guardrail3 generate`."),
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
                message: format!("Crate `{found}` is banned in deny.toml beyond the guardrail3 baseline. Project-specific ban — no action needed."),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    },
)

pub fn check_tokio_feature_ban(
    table: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let Some(bans) = table.get("bans") else {
        return;
    };

    let Some(features) = bans.get("features").and_then(|f| f.as_array()) else {
        results.push(CheckResult::from_parts(
    "R17".to_owned(),
    Severity::Warn,
    "No [[bans.features]]".to_owned(),
    "No feature bans configured".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
        ));
        return;
    };

    let mut tokio_full_banned = false;
    let mut extra_feature_bans = Vec::new();

    for feature_entry in features {
        match feature_ban_name(feature_entry) {
            Some("tokio") => {
                if tokio_feature_is_banned(feature_entry, "full") {
                    tokio_full_banned = true;
                }
            }
            Some(name) => extra_feature_bans.push(name.to_owned()),
            None => {}
        }
    }

    if tokio_full_banned {
        results.push(CheckResult {
            id: "R17".to_owned(),
            severity: Severity::Info,
            title: "tokio full feature banned".to_owned(),
            message: "tokio's `full` feature is banned via [[bans.features]] in deny.toml. This forces explicit feature selection (e.g., `rt`, `macros`, `net`) instead of pulling in everything. No action needed.".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "R17".to_owned(),
            severity: Severity::Warn,
            title: "tokio full not banned".to_owned(),
            message: "tokio's `full` feature is not banned in deny.toml [[bans.features]]. The `full` feature enables every tokio subsystem, increasing compile time and binary size. Add `[[bans.features]]` with `name = \"tokio\"` and `deny = [\"full\"]`.".to_owned(),
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
        ));
    }

fn feature_ban_name(feature_entry: &toml::Value) -> Option<&str> {
    feature_entry
        .get("name")
        .or_else(|| feature_entry.get("crate"))
        .and_then(|name| name.as_str()),
)

fn tokio_feature_is_banned(feature_entry: &toml::Value, feature: &str) -> bool {
    feature_entry
        .get("deny")
        .and_then(|deny| deny.as_array())
        .is_some_and(|deny| {
            deny.iter()
                .filter_map(|value| value.as_str())
                .any(|name| name == feature)
        }),
)
