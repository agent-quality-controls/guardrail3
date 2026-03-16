use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

#[allow(clippy::too_many_lines)] // reason: license validation
pub fn check_licenses(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let Some(licenses) = table.get("licenses") else {
        results.push(CheckResult {
            id: "R14".to_owned(),
            severity: Severity::Error,
            title: "[licenses] section missing".to_owned(),
            message: "deny.toml has no [licenses] section".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
        });
        return;
    };

    // R14: license allow-list
    match licenses.get("allow").and_then(|a| a.as_array()) {
        Some(arr) if !arr.is_empty() => {
            let license_list: Vec<&str> = arr.iter().filter_map(|v| v.as_str()).collect();
            results.push(CheckResult {
                id: "R14".to_owned(),
                severity: Severity::Info,
                title: "License allow list present".to_owned(),
                message: format!("Allowed: {}", license_list.join(", ")),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        _ => {
            results.push(CheckResult {
                id: "R14".to_owned(),
                severity: Severity::Error,
                title: "No license allow list".to_owned(),
                message: "[licenses].allow is missing or empty".to_owned(),
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
        .and_then(toml::Value::as_bool);

    match private_ignore {
        Some(true) => {
            // Correct
        }
        Some(false) => {
            results.push(CheckResult {
                id: "R14".to_owned(),
                severity: Severity::Error,
                title: "[licenses.private] ignore not true".to_owned(),
                message: "Expected [licenses.private] ignore = true".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        None => {
            results.push(CheckResult {
                id: "R14".to_owned(),
                severity: Severity::Error,
                title: "[licenses.private] missing".to_owned(),
                message: "Expected [licenses.private] ignore = true".to_owned(),
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
                    id: "R15".to_owned(),
                    severity: Severity::Info,
                    title: "confidence-threshold differs".to_owned(),
                    message: format!("confidence-threshold = {v} (expected 0.8)"),
                    file: Some(file_path.display().to_string()),
                    line: None,
                });
            }
        }
        None => {
            results.push(CheckResult {
                id: "R15".to_owned(),
                severity: Severity::Info,
                title: "confidence-threshold not set".to_owned(),
                message: "Expected confidence-threshold = 0.8".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        _ => {}
    }
}

#[allow(clippy::too_many_lines)] // reason: source validation
#[allow(clippy::branches_sharing_code)] // reason: separate branches for readability
pub fn check_sources(table: &toml::Value, file_path: &Path, results: &mut Vec<CheckResult>) {
    let Some(sources) = table.get("sources") else {
        results.push(CheckResult {
            id: "R16".to_owned(),
            severity: Severity::Error,
            title: "[sources] section missing".to_owned(),
            message: "deny.toml has no [sources] section".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
        });
        return;
    };

    for key in &["unknown-registry", "unknown-git"] {
        match sources.get(key).and_then(|v| v.as_str()) {
            Some("deny") => {
                results.push(CheckResult {
                    id: "R16".to_owned(),
                    severity: Severity::Info,
                    title: format!("{key} correct"),
                    message: format!("{key} = \"deny\""),
                    file: Some(file_path.display().to_string()),
                    line: None,
                });
            }
            Some(other) => {
                results.push(CheckResult {
                    id: "R16".to_owned(),
                    severity: Severity::Error,
                    title: format!("{key} wrong"),
                    message: format!("Expected \"deny\", got \"{other}\""),
                    file: Some(file_path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "R16".to_owned(),
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
        let has_non_cratesio = registries.iter().any(|r| !r.contains("crates.io"));
        if has_non_cratesio {
            results.push(CheckResult {
                id: "R16".to_owned(),
                severity: Severity::Error,
                title: "Non-crates.io registry allowed".to_owned(),
                message: format!("allow-registry contains: {}", registries.join(", ")),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }

    // Check allow-git = [] (empty)
    match sources.get("allow-git").and_then(|v| v.as_array()) {
        Some(arr) if !arr.is_empty() => {
            let git_sources: Vec<&str> = arr.iter().filter_map(|v| v.as_str()).collect();
            results.push(CheckResult {
                id: "R16".to_owned(),
                severity: Severity::Error,
                title: "allow-git not empty".to_owned(),
                message: format!("allow-git contains: {}", git_sources.join(", ")),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
        Some(_) | None => {
            // Empty array or not set — both acceptable
        }
    }
}
