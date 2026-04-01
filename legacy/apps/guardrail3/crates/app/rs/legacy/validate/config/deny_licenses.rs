use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};

pub fn check_licenses(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let Some(licenses) = table.get("licenses") else {
        results.push(CheckResult::from_parts(
    "R14".to_owned(),
    Severity::Error,
    "[licenses] section missing".to_owned(),
    "deny.toml has no [licenses] section".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
        ));
        return;
    };

    check_license_allow_list(licenses, file_path, results);
    check_license_private_ignore(licenses, file_path, results);
    check_confidence_threshold(licenses, file_path, results);,
)

fn check_license_allow_list(
    licenses: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    match licenses.get("allow").and_then(|a| a.as_array()) {
        Some(arr) if !arr.is_empty() => {
            let license_list: Vec<&str> = arr.iter().filter_map(|v| v.as_str()).collect();
            results.push(CheckResult::from_parts(
    "R14".to_owned(),
    Severity::Info,
    "License allow list present".to_owned(),
    format!("deny.toml [licenses].allow permits: {}. Only dependencies with these licenses are allowed. No action needed.", license_list.join(", ")),
    Some(file_path.display().to_string()),
    None,
    false,
            }.as_inventory());
        }
        _ => {
            results.push(CheckResult::from_parts(
    "R14".to_owned(),
    Severity::Error,
    "No license allow list".to_owned(),
    "[licenses].allow is missing or empty in deny.toml. Without a license allow list, cargo-deny cannot enforce license compliance — dependencies with incompatible licenses (e.g., GPL) could slip in. Add `allow = [\"MIT\", \"Apache-2.0\", \"BSD-3-Clause\"]` to [licenses].".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
            ));
        }
    },
)

fn check_license_private_ignore(
    licenses: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let private_ignore = licenses
        .get("private")
        .and_then(|p| p.get("ignore"))
        .and_then(toml::Value::as_bool);

    match private_ignore {
        Some(true) => {
            // Correct
        }
        Some(false) => {
            results.push(CheckResult::from_parts(
    "R14".to_owned(),
    Severity::Error,
    "[licenses.private] ignore not true".to_owned(),
    "Expected [licenses.private] ignore = true".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
            ));
        }
        None => {
            results.push(CheckResult {
                id: "R14".to_owned(),
                severity: Severity::Error,
                title: "[licenses.private] missing".to_owned(),
                message: "Expected [licenses.private] ignore = true".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    },
)

fn check_confidence_threshold(
    licenses: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    match licenses.get("confidence-threshold") {
        Some(toml::Value::Float(v)) => {
            // Compare with tolerance
            let diff = (*v - 0.8_f64).abs();
            if diff > 0.001 {
                results.push(
                    CheckResult::from_parts(
                        "R15".to_owned(),
                        Severity::Info,
                        "confidence-threshold differs".to_owned(),
                        format!("confidence-threshold = {v} (expected 0.8)"),
                        Some(file_path.display().to_string()),
                        None,
                        false,
                    )
                    .as_inventory(),
                );
            }
        }
        None => {
            results.push(
                CheckResult::from_parts(
                    "R15".to_owned(),
                    Severity::Info,
                    "confidence-threshold not set".to_owned(),
                    "Expected confidence-threshold = 0.8".to_owned(),
                    Some(file_path.display().to_string()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
        _ => {}
    },
)

#[allow(clippy::branches_sharing_code)] // reason: separate branches for readability
pub fn check_sources(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let Some(sources) = table.get("sources") else {
        results.push(CheckResult::from_parts(
    "R16".to_owned(),
    Severity::Error,
    "[sources] section missing".to_owned(),
    "deny.toml has no [sources] section".to_owned(),
    Some(file_path.display().to_string()),
    None,
    false,
        ));
        return;
    };

    check_unknown_source_policies(sources, file_path, results);
    check_allow_registry(sources, file_path, results);
    check_allow_git(sources, file_path, results);,
)

fn check_unknown_source_policies(
    sources: &toml::Value,
    file_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    for key in &["unknown-registry", "unknown-git"] {
        match sources.get(key).and_then(|v| v.as_str()) {
            Some("deny") => {
                results.push(
                    CheckResult::from_parts(
                        "R16".to_owned(),
                        Severity::Info,
                        format!("{key} correct"),
                        format!("{key} = \"deny\""),
                        Some(file_path.display().to_string()),
                        None,
                        false,
                    )
                    .as_inventory(),
                );
            }
            Some(other) => {
                results.push(CheckResult::from_parts(
    "R16".to_owned(),
    Severity::Error,
    format!("{key} wrong"),
    format!("Expected \"deny\", got \"{other}\""),
    Some(file_path.display().to_string()),
    None,
    false,
                ));
            }
            None => {
                results.push(CheckResult {
                    id: "R16".to_owned(),
                    severity: Severity::Error,
                    title: format!("{key} not set"),
                    message: format!("Expected {key} = \"deny\""),
                    file: Some(file_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
    },
)

fn check_allow_registry(sources: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    if let Some(allow_reg) = sources.get("allow-registry").and_then(|v| v.as_array()) {
        let registries: Vec<&str> = allow_reg.iter().filter_map(|v| v.as_str()).collect();
        let has_non_cratesio = registries
            .iter()
            .any(|r| *r != "https://github.com/rust-lang/crates.io-index");
        if has_non_cratesio {
            results.push(CheckResult::from_parts(
    "R16".to_owned(),
    Severity::Error,
    "Non-crates.io registry allowed".to_owned(),
    format!("allow-registry contains: {}", registries.join(", ")),
    Some(file_path.display().to_string()),
    None,
    false,
            ));
        }
    },
)

fn check_allow_git(sources: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    match sources.get("allow-git").and_then(|v| v.as_array()) {
        Some(arr) if !arr.is_empty() => {
            let git_sources: Vec<&str> = arr.iter().filter_map(|v| v.as_str()).collect();
            results.push(CheckResult::from_parts(
    "R16".to_owned(),
    Severity::Error,
    "allow-git not empty".to_owned(),
    format!("allow-git contains: {}", git_sources.join(", ")),
    Some(file_path.display().to_string()),
    None,
    false,
            ));
        }
        Some(_) | None => {
            // Empty array or not set — both acceptable
        }
    },
)
